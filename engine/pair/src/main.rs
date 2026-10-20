#![forbid(unsafe_code)]
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use syndeom_manifest::{artifact_id, content_hash};
use syndeom_pair::{self as pair, Mode, Result, Sources, output, provenance, require};

const USAGE: &str = "syndeom-pair --mode archive|canonical --config <reviewed.json> --root <producer repo> --external-dir <existing external dir> --engine-commit <40-hex> --receipt-dir <NEW external dir> [--artifact-root <existing replay root>] [--sources-dir <archive source dir>] [--dry-run]";

fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_millis()
}

fn absolute(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    Ok(if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(path)
    })
}

fn sources(dir: &Path) -> Result<Sources> {
    let mut files = Sources::new();
    for spec in pair::context_specs()? {
        let name = spec["file"].as_str().ok_or("invalid reviewed filename")?;
        let path = dir.join(name);
        output::safe_path(&path)?;
        files.insert(
            name.into(),
            fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?,
        );
    }
    Ok(files)
}

fn disjoint(a: &Path, b: &Path) -> bool {
    !a.starts_with(b) && !b.starts_with(a)
}

fn execute(
    args: &BTreeMap<String, String>,
    dry: bool,
    receipt: &mut Value,
    commands: &mut Vec<Value>,
) -> Result<()> {
    let get = |key: &str| args.get(key).map(String::as_str).ok_or(USAGE.to_string());
    let root = output::directory(&absolute(get("--root")?)?)?;
    let external = output::directory(&absolute(get("--external-dir")?)?)?;
    let artifact_root = output::directory(&absolute(
        args.get("--artifact-root")
            .map(String::as_str)
            .unwrap_or(get("--root")?),
    )?)?;
    let receipt_dir = output::directory(&absolute(get("--receipt-dir")?)?)?;
    require(
        disjoint(&root, &external)
            && disjoint(&artifact_root, &external)
            && (artifact_root == root || disjoint(&root, &artifact_root))
            && disjoint(&receipt_dir, &root)
            && disjoint(&receipt_dir, &artifact_root)
            && disjoint(&receipt_dir, &external),
        "overlapping/unsafe source, artifact, external or receipt roots",
    )?;
    let mode = match get("--mode")? {
        "archive" => Mode::Archive,
        "canonical" => Mode::Canonical,
        _ => return Err("unsupported mode".into()),
    };
    require(
        (mode == Mode::Archive) == args.contains_key("--sources-dir"),
        "sources-dir is required only in archive mode; canonical resolves the actual archived files",
    )?;
    let config_path = absolute(get("--config")?)?;
    output::safe_path(&config_path)?;
    let config_bytes = fs::read(&config_path).map_err(|e| e.to_string())?;
    let config = pair::strict_json::parse(&config_bytes)?;
    pair::validate_config(&config, mode)?;
    let commit = get("--engine-commit")?;
    receipt["provenance"] = provenance::verify_producer(&root, commit, commands)?;
    let parent_id = artifact_id("s1_pair_archive", pair::VERSION, pair::ARCHIVE_HASH, None);
    let source_dir = if mode == Mode::Archive {
        let dir = output::directory(&absolute(get("--sources-dir")?)?)?;
        require(
            disjoint(&dir, &external) && disjoint(&dir, &artifact_root),
            "archive input and output roots overlap",
        )?;
        dir
    } else {
        external.join(&parent_id)
    };
    let actual_sources = sources(&source_dir)?;
    pair::validate_sources(&actual_sources)?;
    receipt["mathlib_inputs"] = provenance::verify_mathlib(&root, &actual_sources, commands)?;
    receipt["source_bytes"] = json!(actual_sources.iter().map(|(name, bytes)|
        json!({"file": name, "n_bytes": bytes.len(), "sha256": content_hash(bytes)})).collect::<Vec<_>>());
    receipt["config_file_sha256"] = json!(content_hash(&config_bytes));
    let parent_path = artifact_root
        .join("groundtruth")
        .join(&parent_id)
        .join("manifest.json");
    let product = if mode == Mode::Archive {
        pair::archive(&config, &actual_sources, commit)?
    } else {
        output::safe_path(&parent_path)?;
        let expected_parent = pair::archive(
            &pair::reviewed_config(Mode::Archive)?,
            &actual_sources,
            commit,
        )?;
        let parent_plan = output::plan(&expected_parent, &artifact_root, &external)?;
        require(
            parent_plan.iter().all(|(path, _)| path.is_file()),
            "incomplete actual parent output set",
        )?;
        output::preflight(&parent_plan)?;
        let parent_bytes = fs::read(&parent_path).map_err(|e| e.to_string())?;
        pair::canonical(&config, &parent_bytes, &actual_sources, commit)?
    };
    let plan = output::plan(&product, &artifact_root, &external)?;
    receipt["artifact_id"] = json!(product.manifest.artifact_id);
    receipt["config_hash"] = json!(product.manifest.config_hash);
    receipt["parent_manifest_sha256"] = json!(product.parent_manifest_sha256);
    receipt["planned_outputs"] = json!(
        plan.iter()
            .map(|(path, bytes)| json!({
                "path": path, "n_bytes": bytes.len(), "sha256": content_hash(bytes)
            }))
            .collect::<Vec<_>>()
    );
    output::write(&plan, dry)?;
    if !dry {
        let metadata = artifact_root
            .join("groundtruth")
            .join(&product.manifest.artifact_id);
        let manifest = pair::parse_manifest(
            &fs::read(metadata.join("manifest.json")).map_err(|e| e.to_string())?,
        )?;
        if mode == Mode::Archive {
            pair::validate_archive(
                &manifest,
                &sources(&external.join(&manifest.artifact_id))?,
                commit,
            )?;
        } else {
            let parent = pair::parse_manifest(&fs::read(&parent_path).map_err(|e| e.to_string())?)?;
            pair::validate_archive(&parent, &sources(&source_dir)?, commit)?;
            pair::validate_child_record(
                &manifest,
                &parent,
                &fs::read(external.join(&manifest.artifact_id).join("proof.md"))
                    .map_err(|e| e.to_string())?,
                &fs::read(metadata.join("Proof.lean")).map_err(|e| e.to_string())?,
                commit,
            )?;
        }
    }
    receipt["checks"] = json!({
        "reviewed_contract": true, "raw_both_file_validation": true,
        "complete_actual_parent_output_set": mode == Mode::Canonical,
        "canonical_both_file_validation": mode == Mode::Canonical,
        "check_derived_and_stronger_invariants": mode == Mode::Canonical,
        "all_output_preflight_including_dry_run": true,
        "generated_outputs_read_back": !dry,
        "lean_audit": "separate required emitted-byte admission check, not performed by producer",
        "corpus_admission": false, "empty_statistics_fidelity": "PASS; vacuous for semantics"
    });
    println!(
        "{}\nreviewed contract and both-file checks: PASS\nall-output preflight: PASS\n{}",
        product.manifest.artifact_id,
        if dry {
            "dry run: no artifact files written"
        } else {
            "complete output set verified; admission review remains"
        }
    );
    Ok(())
}

fn run() -> Result<()> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut args = BTreeMap::new();
    let mut it = argv.iter();
    let mut dry = false;
    while let Some(key) = it.next() {
        if key == "--dry-run" {
            require(!dry, "duplicate dry-run flag")?;
            dry = true;
        } else {
            require(
                [
                    "--mode",
                    "--config",
                    "--root",
                    "--external-dir",
                    "--engine-commit",
                    "--receipt-dir",
                    "--artifact-root",
                    "--sources-dir",
                ]
                .contains(&key.as_str()),
                USAGE,
            )?;
            let value = it.next().ok_or(USAGE)?;
            require(
                args.insert(key.clone(), value.clone()).is_none(),
                "duplicate argument",
            )?;
        }
    }
    for required in [
        "--mode",
        "--config",
        "--root",
        "--external-dir",
        "--engine-commit",
        "--receipt-dir",
    ] {
        require(args.contains_key(required), USAGE)?;
    }
    let receipt_dir = absolute(&args["--receipt-dir"])?;
    output::safe_path(&receipt_dir)?;
    let root = output::directory(&absolute(&args["--root"])?)?;
    let external = output::directory(&absolute(&args["--external-dir"])?)?;
    let artifact_root = output::directory(&absolute(
        args.get("--artifact-root").unwrap_or(&args["--root"]),
    )?)?;
    require(
        disjoint(&receipt_dir, &root)
            && disjoint(&receipt_dir, &external)
            && disjoint(&receipt_dir, &artifact_root),
        "receipt destination must be external and separate",
    )?;
    // Reserve a new receipt directory before work. Existing evidence is always refused.
    fs::create_dir(&receipt_dir).map_err(|e| format!("new receipt directory required: {e}"))?;
    let binary =
        fs::read(std::env::current_exe().map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let mut receipt = json!({
        "kind": "ADR-0007 production run; admission remains a review decision",
        "main_baseline": pair::MAIN_BASELINE, "corrected_procedure_revision": pair::CORRECTION_REVISION,
        "argv": argv, "started_unix_ms": now(), "dry_run": dry, "result": "failed",
        "os": std::env::consts::OS, "arch": std::env::consts::ARCH,
        "executable_sha256": content_hash(&binary),
        "native_file_alias_check": if cfg!(windows) { "reparse attributes and fsutil hardlink list" } else { "symlink metadata and nlink" },
    });
    let mut commands = Vec::new();
    let result = execute(&args, dry, &mut receipt, &mut commands);
    receipt["commands"] = json!(commands);
    receipt["completed_unix_ms"] = json!(now());
    receipt["exit_code"] = json!(if result.is_ok() { 0 } else { 2 });
    receipt["result"] = json!(if result.is_ok() {
        "mechanical checks passed; not admission"
    } else {
        "failed"
    });
    if let Err(e) = &result {
        receipt["error"] = json!(e);
    }
    let data = serde_json::to_string_pretty(&receipt).map_err(|e| e.to_string())? + "\n";
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(receipt_dir.join("receipt.json"))
        .map_err(|e| e.to_string())?
        .write_all(data.as_bytes())
        .map_err(|e| e.to_string())?;
    result
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(2);
    }
}
