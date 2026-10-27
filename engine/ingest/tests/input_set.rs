//! ADR-0012: the preservation check covers exactly the reproduction's input set. A byte change
//! inside it fails; a change, a removal or an addition outside it passes; an addition under
//! `groundtruth/` fails because the census enumerates that directory.
#[allow(dead_code)]
#[path = "../examples/support/effective.rs"]
mod effective;

use effective::*;
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::OnceLock;

/// The working tree's bytes for every file of the accepted preservation baseline.
fn fixture() -> Blobs {
    static INPUTS: OnceLock<Blobs> = OnceLock::new();
    INPUTS
        .get_or_init(|| {
            let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
            frozen()
                .unwrap()
                .files
                .into_keys()
                .map(|p| {
                    let b = std::fs::read(repo.join(&p)).unwrap_or_else(|e| panic!("{p}: {e}"));
                    (p, b)
                })
                .collect()
        })
        .clone()
}

fn refused(blobs: &Blobs) -> String {
    preserve(blobs, &frozen().unwrap()).unwrap_err()
}

fn passes(blobs: &Blobs) {
    preserve(blobs, &frozen().unwrap()).unwrap();
}

/// Baseline files the reproduction reads: one of each kind.
fn inside() -> Vec<String> {
    let f = frozen().unwrap();
    let first = |suffix: &str| {
        f.files
            .keys()
            .find(|p| p.starts_with("groundtruth/") && p.ends_with(suffix))
            .unwrap()
            .clone()
    };
    vec![
        "PROGRAM/FLAWS.md".into(),
        DISPOSITIONS.into(),
        WITNESS.into(),
        CANDIDATE_CONFIG.into(),
        format!("{}manifest.json", candidate_dir()),
        format!("{}original-0706.0357v70.tar.gz", candidate_dir()),
        manifest_path(OLD),
        format!("groundtruth/{OLD}/RECORD.md"),
        format!("groundtruth/{OLD}/original-0706.0357v70.tar.gz"),
        first("/source-map.json"),
        first("/Proof.lean"),
    ]
}

/// Baseline files the reproduction does not read: the ones NEXT named, the former exceptions,
/// the frozen packets' other files and the corpus directory's non-item files.
const OUTSIDE: [&str; 31] = [
    "PROGRAM/SYNDEOM-PROGRAM.md",
    "PROGRAM/DECISIONS/ADR-0005.md",
    "PROGRAM/SIBLING-MAP-2026-10.md",
    "scripts/check.sh",
    "lean/Audit.lean",
    "engine/Cargo.toml",
    "engine/Cargo.lock",
    "engine/derive/Cargo.toml",
    "engine/ingest/Cargo.toml",
    "engine/manifest/Cargo.toml",
    "engine/pair/Cargo.toml",
    "engine/manifest/src/lib.rs",
    "engine/pair/src/lib.rs",
    "engine/pair/input-files.txt",
    "rust-toolchain.toml",
    "lean/lean-toolchain",
    "LICENSE",
    "README.md",
    "PROGRAM/STATE.md",
    "PROGRAM/SOURCES.md",
    "PROGRAM/HANDOFF.md",
    "PROGRAM/DECISIONS/ADR-0003.md",
    "PROGRAM/CLAIMS.md",
    "groundtruth/README.md",
    "groundtruth/HARVEST-LOG.md",
    "groundtruth/derivations/1209.2022v2-proposition3.1.json",
    "analysis/s1-4/report/report.md",
    "analysis/s1-4/config/bins.json",
    "analysis/s1-first-parent-status/README.md",
    "analysis/s1-first-parent-status/Witness.lean",
    "analysis/s1-2/Proof.lean",
];

#[test]
fn working_tree_passes_and_reports_exactly_the_input_set() {
    let f = frozen().unwrap();
    let report = preserve(&fixture(), &f).unwrap();
    // 375 baseline files; 34 item directories hold 73 files; plus the taxonomy, the
    // dispositions overlay and four accepted-candidate packet files: 79 inputs, 296 unchecked.
    assert_eq!(f.files.len(), 375);
    assert_eq!(report["baseline_files"], 375);
    assert_eq!(report["exact_files_checked"], 79);
    assert_eq!(report["baseline_files_outside_input_set"], 296);
    assert_eq!(report["groundtruth_item_files_checked"], 73);
    assert_eq!(report["accepted_packet_files_checked"], 4);
    let listed: Vec<&str> = report["inputs_checked"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(listed, input_set(&f));
    assert_eq!(listed.len(), 79);
    let set: BTreeSet<&str> = listed.iter().copied().collect();
    assert_eq!(set.len(), listed.len());
    for path in &listed {
        assert!(f.files.contains_key(*path), "{path} is not a baseline file");
    }
    for path in inside() {
        assert!(set.contains(path.as_str()), "{path} must be inside");
    }
    for path in OUTSIDE {
        assert!(f.files.contains_key(path), "{path} is not a baseline file");
        assert!(!set.contains(path), "{path} must be outside");
    }
    // The dispositions overlay is the only frozen S1.4 file the reproduction reads.
    assert_eq!(
        listed
            .iter()
            .filter(|p| p.starts_with("analysis/s1-4/"))
            .collect::<Vec<_>>(),
        vec![&DISPOSITIONS]
    );
}

#[test]
fn item_directories_come_from_the_baseline_not_from_a_name_pattern() {
    let f = frozen().unwrap();
    let dirs: BTreeSet<&str> = f
        .files
        .keys()
        .filter_map(|p| {
            p.strip_prefix("groundtruth/")?
                .strip_suffix("/manifest.json")
        })
        .collect();
    assert_eq!(dirs.len(), 34);
    assert!(dirs.contains(OLD));
    assert!(!dirs.contains("derivations"));
    let items: Vec<&str> = input_set(&f)
        .into_iter()
        .filter(|p| p.starts_with("groundtruth/"))
        .collect();
    assert_eq!(items.len(), 73);
    for path in &items {
        let dir = path
            .strip_prefix("groundtruth/")
            .unwrap()
            .split('/')
            .next()
            .unwrap();
        assert!(dirs.contains(dir), "{path}");
    }
    // The effective representation's directory is read but is not a baseline file: it is bound
    // by `validate_case` to the accepted candidate packet's bytes and hashes.
    assert!(!dirs.contains(NEW));
    assert!(!f.files.contains_key(MAP));
    assert!(
        !f.files
            .contains_key("analysis/s1-first-parent-integration/OWNER-DECISION.md")
    );
}

#[test]
fn a_byte_change_inside_the_input_set_fails() {
    for path in inside() {
        let mut blobs = fixture();
        blobs.get_mut(&path).unwrap().push(b' ');
        assert!(
            refused(&blobs).contains("historical bytes changed"),
            "{path} append"
        );
        let mut blobs = fixture();
        blobs.get_mut(&path).unwrap()[0] ^= 1;
        assert!(
            refused(&blobs).contains("historical bytes changed"),
            "{path} flip"
        );
        let mut blobs = fixture();
        blobs.get_mut(&path).unwrap().pop();
        assert!(
            refused(&blobs).contains("historical bytes changed"),
            "{path} truncate"
        );
        let mut blobs = fixture();
        blobs.remove(&path);
        assert!(refused(&blobs).contains("missing input"), "{path} remove");
    }
}

#[test]
fn a_change_removal_or_rewrite_outside_the_input_set_passes() {
    for path in OUTSIDE {
        let mut blobs = fixture();
        blobs.get_mut(path).unwrap().push(b' ');
        passes(&blobs);
        let mut blobs = fixture();
        blobs.get_mut(path).unwrap()[0] ^= 1;
        passes(&blobs);
        let mut blobs = fixture();
        blobs.insert(path.into(), b"any text at all\n".to_vec());
        passes(&blobs);
        let mut blobs = fixture();
        blobs.remove(path);
        passes(&blobs);
    }
}

#[test]
fn the_former_exceptions_need_no_exception() {
    // ADR-0009 D5 (the constitution), ADR-0010 D6 (the licence file, the append-only source
    // register, the four crate manifests' licence line): all outside the input set.
    let mut blobs = fixture();
    blobs.insert("LICENSE".into(), b"any text at all\n".to_vec());
    blobs
        .get_mut("PROGRAM/SOURCES.md")
        .unwrap()
        .extend_from_slice(b"\nappended register entry\n");
    blobs.get_mut("PROGRAM/SYNDEOM-PROGRAM.md").unwrap()[0] ^= 1;
    for path in [
        "engine/derive/Cargo.toml",
        "engine/ingest/Cargo.toml",
        "engine/manifest/Cargo.toml",
        "engine/pair/Cargo.toml",
    ] {
        let current = String::from_utf8(blobs[path].clone()).unwrap();
        assert!(current.contains("\nlicense = \"PolyForm-Noncommercial-1.0.0\"\n"));
        let other = current.replace(
            "\nlicense = \"PolyForm-Noncommercial-1.0.0\"\n",
            "\nlicense = \"MIT\"\n",
        );
        blobs.insert(path.into(), other.into_bytes());
    }
    // S2's first code commit edits the workspace manifest and lock file (ADR-0010 D7).
    blobs
        .get_mut("engine/Cargo.toml")
        .unwrap()
        .extend_from_slice(b"# members gain \"generate\"\n");
    blobs
        .get_mut("engine/Cargo.lock")
        .unwrap()
        .extend_from_slice(b"\n[[package]]\nname = \"syndeom-generate\"\nversion = \"0.1.0\"\n");
    passes(&blobs);
}

#[test]
fn additions_under_groundtruth_fail_and_additions_elsewhere_pass() {
    for path in [
        "groundtruth/unreviewed-item/manifest.json",
        "groundtruth/unreviewed-item/RECORD.md",
        "groundtruth/new-note.md",
        "groundtruth/derivations/another.json",
    ] {
        let mut blobs = fixture();
        blobs.insert(path.into(), b"{}".to_vec());
        assert!(
            refused(&blobs).contains("unexpected addition in preserved scope"),
            "{path}"
        );
    }
    for path in [
        MAP,
        &format!("groundtruth/{NEW}/manifest.json"),
        &format!("groundtruth/{NEW}/extra-note.md"),
        "analysis/s1-4/new-receipt.json",
        "analysis/s1-first-parent-status/new-receipt.json",
        "analysis/s1-first-parent-integration/OWNER-DECISION.md",
        "PROGRAM/DECISIONS/ADR-0012.md",
        "engine/generate/src/lib.rs",
        "engine/manifest/src/generate.rs",
        "engine/pair/src/extra.rs",
        "lean/Syndeom/Seeds/Seed.lean",
    ] {
        let mut blobs = fixture();
        blobs.insert(path.into(), b"{}".to_vec());
        passes(&blobs);
    }
}

/// The working tree's bytes for every baseline file plus the files the one-case validation
/// reads that are not baseline files (as `first_parent_amendment.rs` loads them).
fn fixture_with_case() -> Blobs {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut blobs = fixture();
    for p in [
        MAP.to_string(),
        format!("{PACKET}/OWNER-DECISION.md"),
        manifest_path(NEW),
        format!("groundtruth/{NEW}/RECORD.md"),
        format!("groundtruth/{NEW}/original-0706.0357v70.tar.gz"),
    ] {
        let b = std::fs::read(repo.join(&p)).unwrap_or_else(|e| panic!("{p}: {e}"));
        blobs.insert(p, b);
    }
    blobs
}

const ADR_MARKER: &str =
    "### 2026-10-20 America/New_York — accepted first-parent D3/SD12 exception";
const HANDOFF_MARKER: &str =
    "OWNER DECISION EVENT: 2026-10-20 America/New_York — accepted first-parent amendment";

/// One in-memory mutation of one path of the working tree.
type Mutation<'a> = &'a dyn Fn(&mut Blobs, &str);

#[test]
fn inventory_is_byte_identical_under_every_outside_mutation() {
    // ADR-0012 amendment 1: the property that makes the narrowing safe. With every file outside
    // the input set mutated as the tests above mutate them — appended, flipped, rewritten,
    // removed — and files added outside `groundtruth/`, the whole report is byte-identical, the
    // `preservation` object included (it is derived from the baseline list, not from the
    // mutated files). The two acceptance-marker files are rewritten to their marker alone and are
    // not removed, because `validate_case` requires the markers: that is the acceptance binding,
    // not preservation.
    let expected = serde_json::to_vec_pretty(&inventory(&fixture_with_case()).unwrap()).unwrap();
    let marker_text = |path: &str| -> Option<String> {
        match path {
            "PROGRAM/DECISIONS/ADR-0003.md" => Some(format!("{ADR_MARKER}\n")),
            "PROGRAM/HANDOFF.md" => Some(format!("{HANDOFF_MARKER}\n")),
            _ => None,
        }
    };
    let worlds: [(&str, Mutation); 4] = [
        ("appended", &|b, p| b.get_mut(p).unwrap().push(b' ')),
        ("flipped", &|b, p| b.get_mut(p).unwrap()[0] ^= 1),
        ("rewritten", &|b, p| {
            let text = marker_text(p).unwrap_or_else(|| "any text at all\n".into());
            b.insert(p.into(), text.into_bytes());
        }),
        ("removed", &|b, p| {
            if marker_text(p).is_none() {
                b.remove(p);
            }
        }),
    ];
    for (label, mutate) in worlds {
        let mut blobs = fixture_with_case();
        for path in OUTSIDE {
            mutate(&mut blobs, path);
        }
        blobs.insert("analysis/s1-4/new-receipt.json".into(), b"{}".to_vec());
        blobs.insert("PROGRAM/DECISIONS/ADR-0013.md".into(), b"{}".to_vec());
        blobs.insert("engine/generate/src/lib.rs".into(), b"{}".to_vec());
        let actual = serde_json::to_vec_pretty(&inventory(&blobs).unwrap()).unwrap();
        assert!(
            actual == expected,
            "the report changed under the {label} world"
        );
    }
    // A byte inside the input set still stops the whole report.
    let mut blobs = fixture_with_case();
    blobs.get_mut("PROGRAM/FLAWS.md").unwrap().push(b' ');
    assert!(
        inventory(&blobs)
            .unwrap_err()
            .contains("historical bytes changed")
    );
}
