#![forbid(unsafe_code)]
//! `syndeom-generate` (ADR-0011 D9): `generate`, `regenerate` and `verify`. Exit codes: 0 written
//! or identical; 1 usage or I/O; 2 refused; 3 a different artifact under the same id. A failed
//! confirmation is not a refusal: the artifact is written with its FAIL verdict.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};
use syndeom_generate::harness::{self, LeanRun, Observation};
use syndeom_generate::template::{self, GENERATOR_ID, GENERATOR_VERSION, Prepared};
use syndeom_manifest::{
    Manifest, Toolchain, content_hash, from_json_str, sha256_hex, to_disk_json,
};

const USAGE: &str = "usage:
  syndeom-generate generate --config <config.json> --seed <u64> --root <repo> --engine-commit <40-hex> --receipt-dir <new directory outside the repo> [--dry-run]
  syndeom-generate regenerate --manifest <manifest.json> --root <repo>
  syndeom-generate verify --artifact <corpora/<id>> --root <repo> --receipt-dir <new directory outside the repo>";

/// The producer's tracked input set (ADR-0011 D9): directories and single files.
const INPUT_DIRS: &[&str] = &["engine/generate", "engine/manifest", "lean/Syndeom/Seeds"];
const INPUT_FILES: &[&str] = &[
    "engine/Cargo.toml",
    "engine/Cargo.lock",
    "rust-toolchain.toml",
    "lean/lean-toolchain",
    "lean/lakefile.toml",
    "lean/lake-manifest.json",
];

enum Fail {
    Usage(String),
    Refused(String),
    Conflict(String),
}

impl From<std::io::Error> for Fail {
    fn from(e: std::io::Error) -> Self {
        Fail::Usage(e.to_string())
    }
}

fn io<E: std::fmt::Display>(e: E) -> Fail {
    Fail::Usage(e.to_string())
}

fn parse_args(
    args: &[String],
    keys: &[&str],
    flags: &[&str],
) -> Result<(BTreeMap<String, String>, Vec<String>), Fail> {
    let mut values = BTreeMap::new();
    let mut set = Vec::new();
    let mut it = args.iter();
    while let Some(k) = it.next() {
        if flags.contains(&k.as_str()) {
            set.push(k.clone());
        } else if keys.contains(&k.as_str()) {
            let v = it.next().ok_or(Fail::Usage(USAGE.into()))?;
            if values.insert(k.clone(), v.clone()).is_some() {
                return Err(Fail::Usage(format!("duplicate argument {k}")));
            }
        } else {
            return Err(Fail::Usage(USAGE.into()));
        }
    }
    for k in keys {
        if !values.contains_key(*k) {
            return Err(Fail::Usage(format!("missing {k}\n{USAGE}")));
        }
    }
    Ok((values, set))
}

fn git(root: &Path, args: &[&str]) -> Result<std::process::Output, Fail> {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(io)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), Fail> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

/// ADR-0011 D9: the named commit is an ancestor of `HEAD` and its blobs for the tracked input
/// set equal the working tree's bytes, with no input file the commit lacks.
fn verify_engine_commit(root: &Path, commit: &str) -> Result<(), Fail> {
    if commit.len() != 40 || !commit.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')) {
        return Err(Fail::Refused(
            "--engine-commit is a 40-character lowercase hex commit id".into(),
        ));
    }
    if !git(root, &["merge-base", "--is-ancestor", commit, "HEAD"])?
        .status
        .success()
    {
        return Err(Fail::Refused(format!(
            "engine_commit {commit} is not an ancestor of HEAD"
        )));
    }
    let mut args = vec!["ls-tree", "-r", "--name-only", commit, "--"];
    args.extend(INPUT_DIRS);
    let listed = git(root, &args)?;
    if !listed.status.success() {
        return Err(Fail::Refused(format!("git ls-tree failed for {commit}")));
    }
    let mut names: Vec<String> = String::from_utf8_lossy(&listed.stdout)
        .lines()
        .map(str::to_string)
        .collect();
    names.extend(INPUT_FILES.iter().map(|s| s.to_string()));
    for name in &names {
        let blob = git(root, &["show", &format!("{commit}:{name}")])?;
        if !blob.status.success() || blob.stdout != fs::read(root.join(name)).unwrap_or_default() {
            return Err(Fail::Refused(format!(
                "engine_commit does not identify the checked-out producer input {name}"
            )));
        }
    }
    let mut present = Vec::new();
    for dir in INPUT_DIRS {
        walk(&root.join(dir), &mut present)?;
    }
    for path in present {
        let rel = path
            .strip_prefix(root)
            .map_err(io)?
            .to_string_lossy()
            .replace('\\', "/");
        if !names.contains(&rel) {
            return Err(Fail::Refused(format!(
                "producer input {rel} is not in engine_commit {commit}"
            )));
        }
    }
    Ok(())
}

/// `toolchain`, read from `lean/lean-toolchain` and `lean/lake-manifest.json` (ADR-0001 D4).
fn read_toolchain(root: &Path) -> Result<Toolchain, Fail> {
    let lean = fs::read_to_string(root.join("lean/lean-toolchain"))?
        .trim_end()
        .to_string();
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(root.join("lean/lake-manifest.json"))?)
            .map_err(io)?;
    let mathlib = manifest["packages"]
        .as_array()
        .and_then(|ps| ps.iter().find(|p| p["name"] == "mathlib"))
        .and_then(|p| p["rev"].as_str())
        .ok_or(Fail::Usage(
            "lake-manifest.json names no mathlib rev".into(),
        ))?
        .to_string();
    Ok(Toolchain { lean, mathlib })
}

/// A new receipt directory outside the repository; never reused (ADR-0011 D9).
fn new_receipt_dir(root: &Path, dir: &str) -> Result<PathBuf, Fail> {
    let path = PathBuf::from(dir);
    if path.exists() {
        return Err(Fail::Usage(format!(
            "receipt directory {dir} exists; receipts are never overwritten"
        )));
    }
    fs::create_dir_all(&path)?;
    let path = fs::canonicalize(&path)?;
    if path.starts_with(root) {
        return Err(Fail::Usage(
            "the receipt directory must lie outside the repository".into(),
        ));
    }
    Ok(path)
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), Fail> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    f.write_all(bytes)?;
    Ok(())
}

fn run_record(run: &LeanRun, dir: &Path, stem: &str) -> Result<Value, Fail> {
    write_new(
        &dir.join(format!("{stem}.stdout.txt")),
        run.stdout.as_bytes(),
    )?;
    write_new(
        &dir.join(format!("{stem}.stderr.txt")),
        run.stderr.as_bytes(),
    )?;
    Ok(json!({
        "argv": run.argv,
        "cwd": run.cwd,
        "exit_code": run.exit_code,
        "duration_ms": run.duration_ms as u64,
        "stdout": {"file": format!("{stem}.stdout.txt"), "sha256": sha256_hex(run.stdout.as_bytes())},
        "stderr": {"file": format!("{stem}.stderr.txt"), "sha256": sha256_hex(run.stderr.as_bytes())},
    }))
}

/// The harness on `bytes`, compiled from a copy in the receipt directory: the build, then — if it
/// passed — the audit file. Returns the observation and the receipt records of both runs.
fn harness_on(
    root: &Path,
    dir: &Path,
    bytes: &[u8],
    main: &str,
) -> Result<(Observation, Vec<Value>), Fail> {
    let lean_dir = root.join("lean");
    let file = dir.join("Proof.lean");
    write_new(&file, bytes)?;
    let build = harness::run_lean(&lean_dir, &file)?;
    let mut runs = vec![run_record(&build, dir, "build")?];
    let audit = if build.exit_code == 0 {
        let audit_file = dir.join("Audit.lean");
        write_new(&audit_file, &harness::audit_bytes(bytes, main))?;
        let run = harness::run_lean(&lean_dir, &audit_file)?;
        runs.push(run_record(&run, dir, "audit")?);
        Some(run)
    } else {
        None
    };
    Ok((harness::observe(&build, audit.as_ref(), main), runs))
}

fn receipt(dir: &Path, body: Value) -> Result<(), Fail> {
    let text = serde_json::to_string_pretty(&body).map_err(io)? + "\n";
    write_new(&dir.join("receipt.json"), text.as_bytes())
}

fn files_of(p: &Prepared) -> Vec<(&'static str, Vec<u8>)> {
    let mut v = Vec::new();
    if let Some(b) = &p.informal {
        v.push(("proof.md", b.clone()));
    }
    if let Some(f) = &p.formal {
        v.push(("Proof.lean", f.bytes.clone()));
    }
    v
}

fn generate(args: &[String]) -> Result<(), Fail> {
    let (a, flags) = parse_args(
        args,
        &[
            "--config",
            "--seed",
            "--root",
            "--engine-commit",
            "--receipt-dir",
        ],
        &["--dry-run"],
    )?;
    let dry_run = flags.iter().any(|f| f == "--dry-run");
    let root = fs::canonicalize(&a["--root"])?;
    let commit = a["--engine-commit"].as_str();
    let seed: u64 = a["--seed"]
        .parse()
        .map_err(|_| Fail::Usage("--seed is an unsigned 64-bit integer".into()))?;
    let config: Value = serde_json::from_str(&fs::read_to_string(&a["--config"])?).map_err(io)?;
    verify_engine_commit(&root, commit)?;
    let prepared = template::prepare(&config, seed).map_err(Fail::Refused)?;
    let id = prepared.artifact_id();
    let dir = new_receipt_dir(&root, &a["--receipt-dir"])?;

    let mut runs = Vec::new();
    let (toolchain, observation) = match &prepared.formal {
        None => (None, None),
        Some(f) => {
            let (obs, r) = harness_on(&root, &dir, &f.bytes, &prepared.plan.main_declaration)?;
            runs = r;
            (Some(read_toolchain(&root)?), Some(obs))
        }
    };
    let manifest = template::assemble(&prepared, commit, toolchain.clone(), observation.as_ref())
        .map_err(Fail::Refused)?;
    let disk = to_disk_json(&manifest).map_err(io)?;
    let mut outputs = files_of(&prepared);
    outputs.push(("manifest.json", disk.into_bytes()));

    let target = root.join("corpora").join(&id);
    let result = if dry_run {
        let out = dir.join("outputs");
        fs::create_dir(&out)?;
        for (name, bytes) in &outputs {
            write_new(&out.join(name), bytes)?;
        }
        "dry run: nothing written under corpora/; the outputs are in the receipt directory"
    } else if target.exists() {
        for (name, bytes) in &outputs {
            if fs::read(target.join(name)).ok().as_deref() != Some(bytes.as_slice()) {
                receipt(
                    &dir,
                    json!({"schema": "syndeom-generate-receipt/1", "command": args, "result": "conflict", "artifact_id": id}),
                )?;
                return Err(Fail::Conflict(format!(
                    "corpora/{id}/{name} differs from what this producer generates"
                )));
            }
        }
        "identical: corpora/<id>/ already holds these bytes"
    } else {
        fs::create_dir_all(&target)?;
        for (name, bytes) in &outputs {
            write_new(&target.join(name), bytes)?;
        }
        "written"
    };
    receipt(
        &dir,
        json!({
            "schema": "syndeom-generate-receipt/1",
            "command": std::iter::once("generate".to_string()).chain(args.iter().cloned()).collect::<Vec<_>>(),
            "producer": {"id": GENERATOR_ID, "version": GENERATOR_VERSION},
            "engine_commit": commit,
            "toolchain": toolchain.map(|t| json!({"lean": t.lean, "mathlib": t.mathlib})),
            "lean_commands": runs,
            "observation": observation,
            "artifact_ids": [id],
            "files": outputs.iter().map(|(n, b)| json!({"file": n, "sha256": content_hash(b), "bytes": b.len()})).collect::<Vec<_>>(),
            "verdict": manifest.fidelity.verdict,
            "result": result,
        }),
    )?;
    println!("{id}\nfidelity: {:?}\n{result}", manifest.fidelity.verdict);
    Ok(())
}

fn load_manifest(path: &Path) -> Result<Manifest, Fail> {
    from_json_str(&fs::read_to_string(path)?).map_err(|e| Fail::Refused(e.to_string()))
}

fn regenerate(args: &[String]) -> Result<(), Fail> {
    let (a, _) = parse_args(args, &["--manifest", "--root"], &[])?;
    let path = PathBuf::from(&a["--manifest"]);
    let manifest = load_manifest(&path)?;
    if (
        manifest.generator.id.as_str(),
        manifest.generator.version.as_str(),
    ) != (GENERATOR_ID, GENERATOR_VERSION)
    {
        return Err(Fail::Refused(format!(
            "{} {} is not this producer",
            manifest.generator.id, manifest.generator.version
        )));
    }
    let seed = manifest.seed.ok_or(Fail::Refused("no seed".into()))?;
    let prepared = template::prepare(&manifest.config, seed).map_err(Fail::Refused)?;
    if prepared.artifact_id() != manifest.artifact_id {
        return Err(Fail::Refused("the manifest's id does not derive".into()));
    }
    let dir = path.parent().ok_or(Fail::Usage("no directory".into()))?;
    let wanted = [
        (
            "proof.md",
            manifest
                .artifact
                .files
                .informal
                .as_ref()
                .map(|f| f.content_hash.clone()),
            prepared.informal.clone(),
        ),
        (
            "Proof.lean",
            manifest
                .artifact
                .files
                .formal
                .as_ref()
                .map(|f| f.content_hash.clone()),
            prepared.formal.as_ref().map(|f| f.bytes.clone()),
        ),
    ];
    for (name, hash, bytes) in wanted {
        match (hash, bytes) {
            (None, None) => {}
            (Some(h), Some(b)) if h == content_hash(&b) => {
                let file = dir.join(name);
                if file.exists() {
                    if fs::read(&file)? != b {
                        return Err(Fail::Conflict(format!("{name} on disk differs")));
                    }
                } else {
                    write_new(&file, &b)?;
                }
                println!("{name}: {h} reproduced");
            }
            _ => {
                return Err(Fail::Refused(format!(
                    "{name}: the regenerated bytes do not reproduce the recorded content_hash"
                )));
            }
        }
    }
    Ok(())
}

fn verify(args: &[String]) -> Result<(), Fail> {
    let (a, _) = parse_args(args, &["--artifact", "--root", "--receipt-dir"], &[])?;
    let root = fs::canonicalize(&a["--root"])?;
    let artifact = PathBuf::from(&a["--artifact"]);
    let manifest = load_manifest(&artifact.join("manifest.json"))?;
    let formal = manifest
        .artifact
        .files
        .formal
        .as_ref()
        .ok_or(Fail::Refused("the artifact has no formal file".into()))?;
    let bytes = fs::read(artifact.join("Proof.lean"))?;
    if content_hash(&bytes) != formal.content_hash {
        return Err(Fail::Refused(
            "Proof.lean does not match the manifest's content_hash".into(),
        ));
    }
    let main = formal
        .main_declaration
        .clone()
        .ok_or(Fail::Refused("no main declaration".into()))?;
    let dir = new_receipt_dir(&root, &a["--receipt-dir"])?;
    let (obs, runs) = harness_on(&root, &dir, &bytes, &main)?;
    let recorded_build = manifest
        .achieved
        .get("lean_build")
        .and_then(|x| x.value.clone());
    let observed_build = json!(if obs.exit_code == 0 { "pass" } else { "fail" });
    let agrees = recorded_build.as_ref() == Some(&observed_build);
    receipt(
        &dir,
        json!({
            "schema": "syndeom-generate-receipt/1",
            "command": std::iter::once("verify".to_string()).chain(args.iter().cloned()).collect::<Vec<_>>(),
            "producer": {"id": GENERATOR_ID, "version": GENERATOR_VERSION},
            "artifact_ids": [manifest.artifact_id],
            "content_hash": formal.content_hash,
            "lean_commands": runs,
            "observation": obs,
            "recorded_lean_build": recorded_build,
            "agrees_with_manifest": agrees,
        }),
    )?;
    println!(
        "{}\nobserved lean_build {observed_build}; manifest agrees: {agrees}",
        manifest.artifact_id
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("generate") => generate(&args[1..]),
        Some("regenerate") => regenerate(&args[1..]),
        Some("verify") => verify(&args[1..]),
        _ => Err(Fail::Usage(USAGE.into())),
    };
    if let Err(f) = result {
        let (code, msg) = match f {
            Fail::Usage(m) => (1, m),
            Fail::Refused(m) => (2, m),
            Fail::Conflict(m) => (3, m),
        };
        eprintln!("{msg}");
        std::process::exit(code);
    }
}
