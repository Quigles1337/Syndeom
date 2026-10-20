//! Preflight every output, including dry runs. Existing bytes are never overwritten.
use crate::{Product, Result, require};
use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
};

pub type Plan = Vec<(PathBuf, Vec<u8>)>;

fn linked(meta: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        meta.file_attributes() & 0x400 != 0 // FILE_ATTRIBUTE_REPARSE_POINT, including junctions.
    }
    #[cfg(not(windows))]
    {
        meta.file_type().is_symlink()
    }
}

pub fn safe_path(path: &Path) -> Result<()> {
    require(path.is_absolute(), "paths must be absolute")?;
    let mut prefix = PathBuf::new();
    for component in path.components() {
        require(
            !matches!(component, Component::ParentDir | Component::CurDir),
            "relative/dot path components are unsupported",
        )?;
        if let Component::Normal(name) = component {
            let name = name.to_str().ok_or("path must be Unicode")?;
            let stem = name.split('.').next().unwrap().to_ascii_uppercase();
            require(
                !name.contains(':')
                    && !name.ends_with([' ', '.'])
                    && !matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
                    && !(stem.len() == 4
                        && (stem.starts_with("COM") || stem.starts_with("LPT"))
                        && matches!(stem.as_bytes()[3], b'1'..=b'9')),
                "unsafe/aliased portable path component",
            )?;
        }
        prefix.push(component.as_os_str());
        // A Windows drive prefix (especially \\?\C:) is not a filesystem path until
        // its RootDir component is present; querying it alone returns ERROR_INVALID_FUNCTION.
        if matches!(component, Component::Prefix(_)) {
            continue;
        }
        match fs::symlink_metadata(&prefix) {
            Ok(meta) => {
                require(
                    !linked(&meta),
                    "symlink/junction/reparse output or ancestor is unsupported",
                )?;
                require(
                    prefix == path || meta.is_dir(),
                    "non-directory path ancestor",
                )?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(format!("{}: {e}", prefix.display())),
        }
    }
    Ok(())
}

pub fn directory(path: &Path) -> Result<PathBuf> {
    safe_path(path)?;
    require(path.is_dir(), "directory must already exist")?;
    fs::canonicalize(path).map_err(|e| e.to_string())
}

fn single_link(path: &Path, meta: &fs::Metadata) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let _ = path;
        require(meta.nlink() == 1, "hard-linked output file is unsupported")
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let _ = meta;
        let output = std::process::Command::new("fsutil")
            .args(["hardlink", "list"])
            .arg(path)
            .creation_flags(0x08000000)
            .output()
            .map_err(|e| e.to_string())?;
        require(
            output.status.success(),
            &format!(
                "fsutil hardlink list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        )?;
        let count = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|s| s.trim_start().starts_with('\\'))
            .count();
        require(
            count == 1,
            "hard-linked output file or unsupported alias query result",
        )
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (path, meta);
        Err("output link checks unsupported on this platform".into())
    }
}

pub fn preflight(plan: &Plan) -> Result<()> {
    let mut keys = BTreeSet::new();
    for (path, bytes) in plan {
        safe_path(path)?;
        require(
            keys.insert(path.to_string_lossy().to_lowercase()),
            "duplicate/case-aliased output",
        )?;
        if let Ok(meta) = fs::symlink_metadata(path) {
            require(meta.is_file(), "planned output is not a regular file")?;
            single_link(path, &meta)?;
            require(
                fs::read(path).map_err(|e| e.to_string())? == *bytes,
                &format!("same-ID/different-bytes conflict: {}", path.display()),
            )?;
        }
    }
    for (i, (a, _)) in plan.iter().enumerate() {
        for (b, _) in &plan[i + 1..] {
            require(
                !a.starts_with(b) && !b.starts_with(a),
                "output file/ancestor alias",
            )?;
        }
    }
    Ok(())
}

pub fn write(plan: &Plan, dry_run: bool) -> Result<()> {
    preflight(plan)?;
    if dry_run {
        return Ok(());
    }
    for (path, bytes) in plan {
        safe_path(path)?;
        fs::create_dir_all(path.parent().ok_or("output has no parent")?)
            .map_err(|e| e.to_string())?;
        safe_path(path)?;
        match OpenOptions::new().write(true).create_new(true).open(path) {
            Ok(mut file) => {
                file.write_all(bytes).map_err(|e| e.to_string())?;
                file.sync_all().map_err(|e| e.to_string())?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                preflight(&vec![(path.clone(), bytes.clone())])?;
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    preflight(plan)?;
    Ok(())
}

pub fn plan(product: &Product, artifact_root: &Path, external: &Path) -> Result<Plan> {
    let metadata = artifact_root
        .join("groundtruth")
        .join(&product.manifest.artifact_id);
    let bytes = external.join(&product.manifest.artifact_id);
    let mut plan = vec![
        (
            metadata.join("manifest.json"),
            syndeom_manifest::to_disk_json(&product.manifest)
                .map_err(|e| e.to_string())?
                .into_bytes(),
        ),
        (
            metadata.join("source-map.json"),
            (serde_json::to_string_pretty(&product.source_map).map_err(|e| e.to_string())? + "\n")
                .into_bytes(),
        ),
    ];
    for (name, data) in &product.committed {
        plan.push((metadata.join(name), data.clone()));
    }
    for (name, data) in &product.external {
        plan.push((bytes.join(name), data.clone()));
    }
    Ok(plan)
}
