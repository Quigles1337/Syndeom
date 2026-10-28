//! Build-time input hashes (ADR-0011 D9 as amended by Al's ruling 5 on PR 26; FINDING-005 of
//! `analysis/s2-generate-1/README.md`). Every file of the producer's input set (`input-set.txt`)
//! is hashed into the binary with the pinned `rustc --version`, so that `verify_engine_commit`
//! can require the running binary, the named commit and the checkout to agree — as
//! `syndeom-pair`'s `build.rs` does for its own closure.
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(&path, out);
        } else {
            out.push(path);
        }
    }
}

fn main() {
    let here = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = here.parent().unwrap().parent().unwrap().to_path_buf();
    let set_file = here.join("input-set.txt");
    println!("cargo:rerun-if-changed={}", set_file.display());
    let set = fs::read_to_string(&set_file).unwrap();
    let mut files: Vec<String> = Vec::new();
    for line in set.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(dir) = line.strip_suffix('/') {
            let path = root.join(dir);
            // A directory line reruns the build when anything under it changes.
            println!("cargo:rerun-if-changed={}", path.display());
            let mut found = Vec::new();
            walk(&path, &mut found);
            for p in found {
                files.push(
                    p.strip_prefix(&root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        } else {
            files.push(line.to_string());
        }
    }
    files.sort();
    files.dedup();
    let mut generated = String::from("pub const COMPILED_INPUTS: &[(&str, &str)] = &[\n");
    for name in &files {
        let path = root.join(name);
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        println!("cargo:rerun-if-changed={}", path.display());
        generated.push_str(&format!(
            "    ({name:?}, {:?}),\n",
            syndeom_manifest::content_hash(&bytes)
        ));
    }
    generated.push_str("];\n");
    let rustc = Command::new(env::var_os("RUSTC").unwrap())
        .arg("--version")
        .output()
        .expect("run pinned rustc");
    assert!(rustc.status.success(), "rustc version subprocess failed");
    let version = String::from_utf8(rustc.stdout).unwrap();
    assert!(
        version.starts_with("rustc 1.91.0 "),
        "unexpected Rust toolchain: {version}"
    );
    generated.push_str(&format!(
        "pub const COMPILED_RUSTC: &str = {:?};\n",
        version.trim()
    ));
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("inputs.rs"),
        generated,
    )
    .unwrap();
}
