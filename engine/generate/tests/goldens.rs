//! The v0.1 goldens (ADR-0011 D10): for every seed, the control and the FLAWS.md example's flawed
//! artifact, informal and formal. Each case directory holds `case.json` (the config, the seed, the
//! engine commit, the toolchain and the harness observation measured at implementation) and the
//! exact bytes: `proof.md` or `Proof.lean`, and `manifest.json`. The test regenerates every case
//! without Lean, from the recorded observation, and compares bytes. If the engine ever disagrees
//! with a golden, the engine is wrong until proven otherwise.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use syndeom_generate::harness::Observation;
use syndeom_generate::template::{assemble, prepare};
use syndeom_manifest::{Toolchain, content_hash, to_disk_json};

fn dirs(path: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir())
        .collect();
    v.sort();
    v
}

#[test]
fn d10_goldens_regenerate_byte_for_byte() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens");
    let mut n = 0;
    for seed_dir in dirs(&root) {
        for case_dir in dirs(&seed_dir) {
            let case: Value =
                serde_json::from_slice(&fs::read(case_dir.join("case.json")).unwrap()).unwrap();
            let p = prepare(&case["config"], case["seed"].as_u64().unwrap()).unwrap();
            let toolchain: Option<Toolchain> =
                serde_json::from_value(case["toolchain"].clone()).unwrap();
            let observation: Option<Observation> =
                serde_json::from_value(case["observation"].clone()).unwrap();
            let manifest = assemble(
                &p,
                case["engine_commit"].as_str().unwrap(),
                toolchain,
                observation.as_ref(),
            )
            .unwrap();
            let expect = |name: &str, got: Option<Vec<u8>>| {
                let path = case_dir.join(name);
                match got {
                    Some(bytes) => assert_eq!(fs::read(&path).unwrap(), bytes, "{path:?}"),
                    None => assert!(!path.exists(), "{path:?} should not exist"),
                }
            };
            expect("proof.md", p.informal.clone());
            expect("Proof.lean", p.formal.as_ref().map(|f| f.bytes.clone()));
            expect(
                "manifest.json",
                Some(to_disk_json(&manifest).unwrap().into_bytes()),
            );
            // the manifest pins the golden proof bytes by content_hash
            if let Some(b) = &p.informal {
                let f = manifest.artifact.files.informal.as_ref().unwrap();
                assert_eq!(f.content_hash, content_hash(b));
            }
            n += 1;
        }
    }
    assert_eq!(
        n, 12,
        "each seed's control and FLAWS.md example, informal and formal"
    );
}
