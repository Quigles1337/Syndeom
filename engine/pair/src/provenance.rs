//! Compare the executable's build-time source hashes, recorded Git blobs and current inputs.
use crate::{LEAN, MATHLIB, Result, Sources, require};
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};
use syndeom_manifest::content_hash;

include!(concat!(env!("OUT_DIR"), "/inputs.rs"));

pub fn run_checked(
    program: &str,
    args: &[&str],
    cwd: &Path,
    log: &mut Vec<Value>,
) -> Result<Vec<u8>> {
    let mut command = Command::new(program);
    command.args(args).current_dir(cwd);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    let output = command.output().map_err(|e| format!("{program}: {e}"))?;
    log.push(json!({
        "program": program, "args": args, "cwd": cwd.to_string_lossy(),
        "exit_code": output.status.code(), "success": output.status.success(),
        "stdout_sha256": content_hash(&output.stdout),
        "stderr": String::from_utf8_lossy(&output.stderr),
    }));
    require(
        output.status.success(),
        &format!(
            "{program} subprocess failed ({:?}): {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ),
    )?;
    Ok(output.stdout)
}

pub fn verify_producer(root: &Path, commit: &str, commands: &mut Vec<Value>) -> Result<Value> {
    require(
        commit.len() == 40
            && commit
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "producer revision must be full lowercase 40-hex",
    )?;
    let resolved = run_checked(
        "git",
        &["rev-parse", "--verify", &format!("{commit}^{{commit}}")],
        root,
        commands,
    )?;
    require(
        String::from_utf8_lossy(&resolved).trim() == commit,
        "not an exact commit",
    )?;
    run_checked(
        "git",
        &["merge-base", "--is-ancestor", commit, "HEAD"],
        root,
        commands,
    )?;
    let mut inputs = Vec::new();
    for &(name, embedded_hash) in COMPILED_INPUTS {
        let blob = run_checked(
            "git",
            &["show", &format!("{commit}:{name}")],
            root,
            commands,
        )?;
        let current = fs::read(root.join(name)).map_err(|e| format!("{name}: {e}"))?;
        require(
            content_hash(&blob) == embedded_hash && blob == current,
            &format!("producer executable/Git/checkout input mismatch: {name}"),
        )?;
        inputs.push(json!({"file": name, "sha256": embedded_hash, "git_and_checkout_match": true}));
    }
    // Neither a new unlisted module nor a project-local compiler override is accepted.
    for directory in ["engine/pair/src", "engine/manifest/src"] {
        for entry in fs::read_dir(root.join(directory)).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            let name = format!(
                "{directory}/{}",
                path.file_name().unwrap().to_string_lossy()
            );
            require(
                path.is_file() && COMPILED_INPUTS.iter().any(|(n, _)| *n == name),
                "unlisted production source input",
            )?;
        }
    }
    for name in [
        ".cargo/config",
        ".cargo/config.toml",
        "engine/.cargo/config",
        "engine/.cargo/config.toml",
    ] {
        require(
            !root.join(name).exists(),
            "project-local Cargo overrides are unsupported",
        )?;
    }
    let version = run_checked("rustc", &["--version"], root, commands)?;
    require(
        String::from_utf8_lossy(&version).trim() == COMPILED_RUSTC,
        "runtime/compiler toolchain differs from built producer",
    )?;
    Ok(
        json!({"engine_commit": commit, "compiled_rustc": COMPILED_RUSTC,
              "compiled_input_closure": inputs, "executable_input_identity": "verified"}),
    )
}

pub fn verify_mathlib(root: &Path, sources: &Sources, commands: &mut Vec<Value>) -> Result<Value> {
    require(
        fs::read_to_string(root.join("lean/lean-toolchain"))
            .map_err(|e| e.to_string())?
            .trim()
            == LEAN,
        "unexpected Lean pin",
    )?;
    let package = root.join("lean/.lake/packages/mathlib");
    let head = run_checked("git", &["rev-parse", "HEAD"], &package, commands)?;
    require(
        String::from_utf8_lossy(&head).trim() == MATHLIB,
        "Mathlib checkout at wrong revision",
    )?;
    let mut comparisons = Vec::new();
    for (file, source) in [
        ("PrimeFin.lean", "Mathlib/Data/Nat/PrimeFin.lean"),
        ("Infinite.lean", "Mathlib/Data/Nat/Prime/Infinite.lean"),
        ("PrimeDefs.lean", "Mathlib/Data/Nat/Prime/Defs.lean"),
        ("mathlib-LICENSE", "LICENSE"),
    ] {
        let blob = run_checked(
            "git",
            &["show", &format!("{MATHLIB}:{source}")],
            &package,
            commands,
        )?;
        require(
            sources.get(file) == Some(&blob),
            "retained formal context differs from pinned Git blob",
        )?;
        let worktree = fs::read(package.join(source)).map_err(|e| e.to_string())?;
        let crlf: Vec<u8> = blob
            .iter()
            .flat_map(|&b| {
                if b == b'\n' {
                    vec![b'\r', b'\n']
                } else {
                    vec![b]
                }
            })
            .collect();
        require(
            worktree == blob || worktree == crlf,
            "Mathlib worktree differs beyond exact LF-to-CRLF conversion",
        )?;
        comparisons.push(json!({"file": source, "git_blob_sha256": content_hash(&blob),
            "worktree_sha256": content_hash(&worktree),
            "worktree_eol": if worktree == blob { "exact Git blob" } else { "exact CRLF checkout of LF blob" }}));
    }
    Ok(json!({"mathlib": MATHLIB, "lean": LEAN, "source_comparisons": comparisons}))
}
