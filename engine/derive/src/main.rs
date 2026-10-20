#![forbid(unsafe_code)]
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syndeom_derive::{Recipe, derive, write_outputs};
use syndeom_manifest::to_disk_json;

const USAGE: &str = "syndeom-derive --config <recipe.json> --parent <manifest.json> --bytes <source.gz> --root <repo> --external-dir <existing directory outside repo> --engine-commit <commit> [--dry-run]";

fn run() -> Result<(), String> {
    let mut args = BTreeMap::new();
    let mut it = std::env::args().skip(1);
    let mut dry_run = false;
    while let Some(key) = it.next() {
        if key == "--dry-run" {
            dry_run = true;
            continue;
        }
        if ![
            "--config",
            "--parent",
            "--bytes",
            "--root",
            "--external-dir",
            "--engine-commit",
        ]
        .contains(&key.as_str())
        {
            return Err(USAGE.into());
        }
        let value = it.next().ok_or(USAGE)?;
        if args.insert(key, value).is_some() {
            return Err("duplicate argument".into());
        }
    }
    let get = |key: &str| args.get(key).map(String::as_str).ok_or(USAGE.to_string());
    let root = fs::canonicalize(get("--root")?).map_err(|e| e.to_string())?;
    let external = fs::canonicalize(get("--external-dir")?).map_err(|e| e.to_string())?;
    if !external.is_dir() || external.starts_with(&root) {
        return Err("external-dir must be an existing directory outside the repository".into());
    }
    let commit = get("--engine-commit")?;
    // Confirm this is a producer-containing commit and its tracked implementation matches the
    // checkout. Tests use the library directly with explicitly mechanical provenance.
    for name in [
        "engine/derive/Cargo.toml",
        "engine/derive/src/lib.rs",
        "engine/derive/src/main.rs",
        "engine/manifest/Cargo.toml",
        "engine/manifest/src/lib.rs",
        "engine/Cargo.toml",
        "engine/Cargo.lock",
        "rust-toolchain.toml",
    ] {
        let output = Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(["show", &format!("{commit}:{name}")])
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success()
            || output.stdout != fs::read(root.join(name)).map_err(|e| e.to_string())?
        {
            return Err(format!(
                "engine_commit does not identify the checked-out producer inputs: {name}"
            ));
        }
    }
    let recipe: Recipe =
        serde_json::from_str(&fs::read_to_string(get("--config")?).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let parent_path = PathBuf::from(get("--parent")?);
    let parent = fs::read(&parent_path).map_err(|e| e.to_string())?;
    let raw = fs::read(get("--bytes")?).map_err(|e| e.to_string())?;
    let product = derive(&parent, &raw, &recipe, commit)?;
    let parent_id = product.manifest.config["derived_from"]
        .as_str()
        .ok_or("missing parent id")?;
    if fs::canonicalize(parent_path).map_err(|e| e.to_string())?
        != fs::canonicalize(
            root.join("groundtruth")
                .join(parent_id)
                .join("manifest.json"),
        )
        .map_err(|e| e.to_string())?
    {
        return Err("parent must be the repository's item of record".into());
    }
    let id = &product.manifest.artifact_id;
    let metadata = root.join("groundtruth").join(id);
    let bytes_dir = external.join(id);
    for dir in [&metadata, &bytes_dir] {
        if dir.exists() && fs::canonicalize(dir).map_err(|e| e.to_string())? != *dir {
            return Err("redirected output directory is not supported".into());
        }
    }
    let manifest = to_disk_json(&product.manifest).map_err(|e| e.to_string())?;
    let source_map =
        serde_json::to_string_pretty(&product.source_map).map_err(|e| e.to_string())? + "\n";
    if !dry_run {
        write_outputs(&[
            (&metadata.join("manifest.json"), manifest.as_bytes()),
            (&metadata.join("source-map.json"), source_map.as_bytes()),
            (&bytes_dir.join("proof.md"), &product.proof),
            (&bytes_dir.join("context.tex"), &product.context),
        ])?;
    }
    println!(
        "{id}\nbyte-aware check: PASS\nparent/child check: PASS\nproof SHA-256: {}\ncontext SHA-256: {}\n{}",
        syndeom_manifest::content_hash(&product.proof),
        syndeom_manifest::content_hash(&product.context),
        if dry_run {
            "dry run: no files written"
        } else {
            "metadata and external outputs verified/written"
        }
    );
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(2);
    }
}
