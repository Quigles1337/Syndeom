use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let here = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = here.parent().unwrap().parent().unwrap();
    let names = fs::read_to_string(here.join("input-files.txt")).unwrap();
    let mut generated = String::from("pub const COMPILED_INPUTS: &[(&str, &str)] = &[\n");
    for name in names.lines() {
        let path = root.join(name);
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        println!("cargo:rerun-if-changed={}", path.display());
        generated.push_str(&format!(
            "({name:?}, {:?}),\n",
            syndeom_manifest::content_hash(&bytes)
        ));
    }
    generated.push_str("];\n");
    // Fail if a new source file is added without extending the reviewed closure.
    for directory in ["engine/pair/src", "engine/manifest/src"] {
        for entry in fs::read_dir(root.join(directory)).unwrap() {
            let path = entry.unwrap().path();
            assert!(
                path.is_file(),
                "source subdirectories require explicit closure support"
            );
            let relative = format!(
                "{directory}/{}",
                path.file_name().unwrap().to_str().unwrap()
            );
            assert!(
                names.lines().any(|n| n == relative),
                "unlisted source input: {relative}"
            );
        }
    }
    let rustc = Command::new(env::var_os("RUSTC").unwrap())
        .arg("--version")
        .output()
        .expect("run pinned rustc");
    assert!(rustc.status.success(), "rustc version subprocess failed");
    let version = String::from_utf8(rustc.stdout).unwrap();
    assert!(
        version.starts_with("rustc 1.91.0 "),
        "unexpected Rust toolchain"
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
