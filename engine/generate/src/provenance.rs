//! Provenance of the running binary (ADR-0011 D9 as amended by Al's ruling 5 on PR 26). For every
//! file of the producer's input set (`input-set.txt`) three things must agree: the hash compiled
//! into this binary at build time, the named engine commit's blob, and the checkout's bytes. A
//! binary built from other sources, a commit that does not hold what the binary compiled, or a
//! checkout that differs from the commit is refused. This closes FINDING-005 of
//! `analysis/s2-generate-1/README.md`, as `syndeom-pair`'s provenance check does for its closure.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{Value, json};
use syndeom_manifest::content_hash;

include!(concat!(env!("OUT_DIR"), "/inputs.rs"));

/// `input-set.txt`: the producer's tracked input set, one path per line — a directory (trailing
/// `/`, walked recursively) or a single file — relative to the repository root.
pub const INPUT_SET: &str = include_str!("../input-set.txt");

/// A failure of the check: an I/O or subprocess failure (exit 1 at the CLI), or a refusal (exit 2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceError {
    Io(String),
    Refused(String),
}

/// The input set as (directories, files).
pub fn input_set() -> (Vec<&'static str>, Vec<&'static str>) {
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for line in INPUT_SET.lines().map(str::trim).filter(|l| !l.is_empty()) {
        match line.strip_suffix('/') {
            Some(dir) => dirs.push(dir),
            None => files.push(line),
        }
    }
    (dirs, files)
}

/// The (name, SHA-256) pairs compiled into this binary, in name order.
pub fn compiled_inputs() -> &'static [(&'static str, &'static str)] {
    COMPILED_INPUTS
}

/// The `rustc --version` line this binary was built with.
pub fn compiled_rustc() -> &'static str {
    COMPILED_RUSTC
}

fn git(root: &Path, args: &[&str]) -> Result<Output, ProvenanceError> {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|e| ProvenanceError::Io(format!("git: {e}")))
}

/// Every file under `dir`, recursively, in sorted order (nothing if `dir` is absent).
pub fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), ProvenanceError> {
    if !dir.exists() {
        return Ok(());
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| ProvenanceError::Io(format!("{}: {e}", dir.display())))?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<_, _>>()
        .map_err(|e| ProvenanceError::Io(e.to_string()))?;
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

fn relative(root: &Path, path: &Path) -> Result<String, ProvenanceError> {
    Ok(path
        .strip_prefix(root)
        .map_err(|e| ProvenanceError::Io(e.to_string()))?
        .to_string_lossy()
        .replace('\\', "/"))
}

/// ADR-0011 D9, as amended: `commit` is an exact commit id and an ancestor of `HEAD`; for every
/// input the commit names, its blob equals the checkout's bytes and hashes to the value compiled
/// into this binary; the commit holds no input this binary did not compile, and this binary
/// compiled no input the commit lacks; the checkout holds no input file the commit lacks; the
/// running `rustc` is the compiled one; and no project-local Cargo override exists. Returns the
/// record of what was checked, for the receipt.
pub fn verify_engine_commit(root: &Path, commit: &str) -> Result<Value, ProvenanceError> {
    use ProvenanceError::Refused;
    if commit.len() != 40 || !commit.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')) {
        return Err(Refused(
            "--engine-commit is a 40-character lowercase hex commit id".into(),
        ));
    }
    let resolved = git(
        root,
        &["rev-parse", "--verify", &format!("{commit}^{{commit}}")],
    )?;
    if !resolved.status.success() || String::from_utf8_lossy(&resolved.stdout).trim() != commit {
        return Err(Refused(format!(
            "engine_commit {commit} is not an exact commit"
        )));
    }
    if !git(root, &["merge-base", "--is-ancestor", commit, "HEAD"])?
        .status
        .success()
    {
        return Err(Refused(format!(
            "engine_commit {commit} is not an ancestor of HEAD"
        )));
    }
    let (dirs, files) = input_set();
    let mut args = vec!["ls-tree", "-r", "--name-only", commit, "--"];
    args.extend(dirs.iter().copied());
    let listed = git(root, &args)?;
    if !listed.status.success() {
        return Err(Refused(format!("git ls-tree failed for {commit}")));
    }
    let mut names: Vec<String> = String::from_utf8_lossy(&listed.stdout)
        .lines()
        .map(str::to_string)
        .collect();
    names.extend(files.iter().map(|s| s.to_string()));
    names.sort();
    names.dedup();
    let mut checked = 0u64;
    for name in &names {
        let blob = git(root, &["show", &format!("{commit}:{name}")])?;
        if !blob.status.success() || blob.stdout != fs::read(root.join(name)).unwrap_or_default() {
            return Err(Refused(format!(
                "engine_commit does not identify the checked-out producer input {name}"
            )));
        }
        let Some((_, embedded)) = COMPILED_INPUTS.iter().find(|(n, _)| n == name) else {
            return Err(Refused(format!(
                "the running binary did not compile the producer input {name} that engine_commit {commit} holds; rebuild it"
            )));
        };
        if content_hash(&blob.stdout) != *embedded {
            return Err(Refused(format!(
                "the running binary was built from another version of the producer input {name} than engine_commit {commit} holds; rebuild it"
            )));
        }
        checked += 1;
    }
    for (name, _) in COMPILED_INPUTS {
        if !names.iter().any(|n| n == name) {
            return Err(Refused(format!(
                "the running binary compiled the producer input {name}, which engine_commit {commit} lacks"
            )));
        }
    }
    let mut present = Vec::new();
    for dir in &dirs {
        walk(&root.join(dir), &mut present)?;
    }
    for path in present {
        let rel = relative(root, &path)?;
        if !names.contains(&rel) {
            return Err(Refused(format!(
                "producer input {rel} is not in engine_commit {commit}"
            )));
        }
    }
    for name in [
        ".cargo/config",
        ".cargo/config.toml",
        "engine/.cargo/config",
        "engine/.cargo/config.toml",
    ] {
        if root.join(name).exists() {
            return Err(Refused(format!(
                "project-local Cargo overrides are unsupported: {name} exists"
            )));
        }
    }
    let rustc = Command::new("rustc")
        .arg("--version")
        .current_dir(root)
        .output()
        .map_err(|e| ProvenanceError::Io(format!("rustc: {e}")))?;
    let version = String::from_utf8_lossy(&rustc.stdout).trim().to_string();
    if !rustc.status.success() || version != COMPILED_RUSTC {
        return Err(Refused(format!(
            "the runtime toolchain ({version:?}) differs from the one this binary was built with ({COMPILED_RUSTC:?})"
        )));
    }
    Ok(json!({
        "engine_commit": commit,
        "compiled_rustc": COMPILED_RUSTC,
        "inputs_checked": checked,
        "executable_input_identity": "verified",
    }))
}
