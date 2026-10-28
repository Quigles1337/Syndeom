//! The goldens (ADR-0011 D10): for every seed, the control (where the statement is true) and the
//! FLAWS.md example's flawed artifact, informal and formal, and for the 0.2.0 seeds the pair too.
//! Each case directory holds `case.json` (the config, the seed, the engine commit, the toolchain
//! and the harness observation measured at implementation) and the exact bytes: `proof.md` or
//! `Proof.lean`, and `manifest.json`. The test regenerates every case without Lean, from the
//! recorded observation, under the producer version its manifest names (ADR-0011 *Amendments*,
//! Al's ruling 5 on PR 26: the 0.1.0 goldens stay regenerable under the version they name), and
//! compares bytes. If the engine ever disagrees with a golden, the engine is wrong until proven
//! otherwise.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use syndeom_generate::harness::Observation;
use syndeom_generate::template::{Version, assemble, prepare_with};
use syndeom_manifest::{Toolchain, content_hash, to_disk_json};

/// The 0.1.0 goldens: each seed's control and FLAWS.md example, informal and formal (Session 40).
const GOLDENS_0_1_0: u64 = 12;
/// The 0.2.0 goldens (Session 41): the true seed's control (informal, formal) and every new
/// seed's FLAWS.md example (informal, formal, pair), regenerated at `c899b073…` after Al's
/// rulings on PR 27 so that they share one engine commit.
const GOLDENS_0_2_0: u64 = 14;
/// The 0.3.0 goldens (Session 42): the `n_lt_two_pow` control (informal, formal), FLAW-0009
/// `axiom` with `truth` true and false (formal, pair each) and FLAW-0007 `unused` (informal,
/// formal, pair), produced at the implementation commit `5a313ddf…` from two byte-identical runs.
const GOLDENS_0_3_0: u64 = 9;
/// The 0.4.0 goldens (Session 43): the `sqrt2_irrational` control (informal, formal), the
/// lightly padded control (FLAW-0013, one re-derivation at a drawn position) and the FLAWS.md
/// FLAW-0013 example (every paragraph of the seed's material before the argument), each as
/// informal, formal and pair, produced at the implementation commit `b54e2f16…` from two
/// byte-identical runs.
const GOLDENS_0_4_0: u64 = 8;

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
fn d10_goldens_regenerate_byte_for_byte_under_the_version_they_name() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens");
    let mut by_version: BTreeMap<&'static str, u64> = BTreeMap::new();
    for seed_dir in dirs(&root) {
        for case_dir in dirs(&seed_dir) {
            let case: Value =
                serde_json::from_slice(&fs::read(case_dir.join("case.json")).unwrap()).unwrap();
            let recorded: Value =
                serde_json::from_slice(&fs::read(case_dir.join("manifest.json")).unwrap()).unwrap();
            let version = Version::parse(recorded["generator"]["version"].as_str().unwrap())
                .unwrap_or_else(|| panic!("{case_dir:?} names an unknown producer version"));
            let p = prepare_with(version, &case["config"], case["seed"].as_u64().unwrap())
                .unwrap_or_else(|e| panic!("{case_dir:?}: {e}"));
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
            .unwrap_or_else(|e| panic!("{case_dir:?}: {e}"));
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
            if let Some(f) = &p.formal {
                let r = manifest.artifact.files.formal.as_ref().unwrap();
                assert_eq!(r.content_hash, content_hash(&f.bytes));
            }
            *by_version.entry(version.as_str()).or_default() += 1;
        }
    }
    assert_eq!(
        by_version.get("0.1.0").copied().unwrap_or(0),
        GOLDENS_0_1_0,
        "the 0.1.0 goldens"
    );
    assert_eq!(
        by_version.get("0.2.0").copied().unwrap_or(0),
        GOLDENS_0_2_0,
        "the 0.2.0 goldens"
    );
    assert_eq!(
        by_version.get("0.3.0").copied().unwrap_or(0),
        GOLDENS_0_3_0,
        "the 0.3.0 goldens"
    );
    assert_eq!(
        by_version.get("0.4.0").copied().unwrap_or(0),
        GOLDENS_0_4_0,
        "the 0.4.0 goldens"
    );
}
