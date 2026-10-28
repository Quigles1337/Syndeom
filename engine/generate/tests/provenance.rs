//! The running binary's provenance (ADR-0011 D9 as amended by Al's ruling 5 on PR 26; FINDING-005
//! of `analysis/s2-generate-1/README.md`): the hashes compiled in by `build.rs` cover exactly the
//! input set on disk, and the compiled toolchain is the pinned one.

use std::collections::BTreeMap;
use std::path::Path;

use syndeom_generate::provenance::{compiled_inputs, compiled_rustc, input_set, walk};
use syndeom_manifest::content_hash;

fn root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

#[test]
fn compiled_inputs_cover_exactly_the_input_set_on_disk() {
    let root = root();
    let (dirs, files) = input_set();
    assert_eq!(
        dirs,
        vec!["engine/generate", "engine/manifest", "lean/Syndeom/Seeds"]
    );
    assert_eq!(files.len(), 6);
    let mut on_disk: BTreeMap<String, String> = BTreeMap::new();
    for dir in dirs {
        let mut found = Vec::new();
        walk(&root.join(dir), &mut found).unwrap();
        for path in found {
            let rel = path
                .strip_prefix(&root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            on_disk.insert(rel, content_hash(&std::fs::read(&path).unwrap()));
        }
    }
    for file in files {
        on_disk.insert(
            file.to_string(),
            content_hash(&std::fs::read(root.join(file)).unwrap()),
        );
    }
    let compiled: BTreeMap<String, String> = compiled_inputs()
        .iter()
        .map(|(n, h)| (n.to_string(), h.to_string()))
        .collect();
    // Cargo rebuilt the binary before this test ran (build.rs reruns on any input change), so the
    // compiled set names every input on disk with the hash it has now.
    let missing: Vec<&String> = on_disk
        .keys()
        .filter(|k| !compiled.contains_key(*k))
        .collect();
    let extra: Vec<&String> = compiled
        .keys()
        .filter(|k| !on_disk.contains_key(*k))
        .collect();
    assert!(
        missing.is_empty(),
        "inputs on disk the binary did not compile: {missing:?}"
    );
    assert!(
        extra.is_empty(),
        "compiled inputs absent on disk: {extra:?}"
    );
    for (name, hash) in &compiled {
        assert_eq!(&on_disk[name], hash, "{name}");
    }
    assert!(compiled.contains_key("engine/generate/build.rs"));
    assert!(compiled.contains_key("engine/generate/input-set.txt"));
    assert!(compiled.contains_key("engine/generate/seeds/horses.json"));
    assert!(compiled.contains_key("lean/Syndeom/Seeds/Horses.lean"));
    assert!(compiled.contains_key("engine/Cargo.lock"));
}

#[test]
fn compiled_rustc_is_the_pinned_toolchain() {
    assert!(
        compiled_rustc().starts_with("rustc 1.91.0 "),
        "{}",
        compiled_rustc()
    );
}
