//! ADR-0010: the three preservation exceptions are exactly as narrow as ruled; any other byte
//! change in the excepted files, and any change elsewhere, still fails.
#[allow(dead_code)]
#[path = "../examples/support/effective.rs"]
mod effective;

use effective::*;
use serde_json::json;
use std::path::Path;
use std::sync::OnceLock;
use syndeom_manifest::content_hash;

const POLYFORM_LINE: &[u8] = b"\nlicense = \"PolyForm-Noncommercial-1.0.0\"\n";

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

fn replace_once(bytes: &[u8], from: &[u8], to: &[u8]) -> Vec<u8> {
    let hits: Vec<usize> = bytes
        .windows(from.len())
        .enumerate()
        .filter(|(_, w)| *w == from)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(hits.len(), 1, "fixture pattern must occur exactly once");
    let mut out = bytes[..hits[0]].to_vec();
    out.extend_from_slice(to);
    out.extend_from_slice(&bytes[hits[0] + from.len()..]);
    out
}

#[test]
fn working_tree_passes_with_exactly_the_three_exceptions() {
    let report = preserve(&fixture(), &frozen().unwrap()).unwrap();
    // 375 baseline files: 4 skipped, 4 append-only, 4 licence-line-only, 363 exact.
    assert_eq!(frozen().unwrap().files.len(), 375);
    assert_eq!(report["exact_files_checked"], 363);
    assert_eq!(
        report["append_only_prefixes_checked"],
        json!([
            "PROGRAM/DECISIONS/ADR-0003.md",
            "PROGRAM/HANDOFF.md",
            "PROGRAM/SOURCES.md",
            "groundtruth/README.md"
        ])
    );
    assert_eq!(
        report["license_line_only_files_checked"],
        json!(LICENSE_LINE_FILES)
    );
}

#[test]
fn licence_file_is_skipped_and_no_other_exact_file_is() {
    let mut blobs = fixture();
    blobs.insert("LICENSE".into(), b"any text at all\n".to_vec());
    preserve(&blobs, &frozen().unwrap()).unwrap();
    for path in [
        "PROGRAM/CLAIMS.md",
        "PROGRAM/DECISIONS/ADR-0000.md",
        "engine/Cargo.toml",
        "engine/Cargo.lock",
    ] {
        let mut blobs = fixture();
        blobs.get_mut(path).unwrap().push(b' ');
        assert!(
            refused(&blobs).contains("historical bytes changed"),
            "{path}"
        );
    }
}

#[test]
fn sources_register_accepts_appends_and_nothing_else() {
    let path = "PROGRAM/SOURCES.md";
    let mut blobs = fixture();
    blobs
        .get_mut(path)
        .unwrap()
        .extend_from_slice(b"\nappended register entry\n");
    preserve(&blobs, &frozen().unwrap()).unwrap();
    let pinned = frozen().unwrap().files[path].n_bytes;
    for at in [0, pinned / 2, pinned - 1] {
        let mut blobs = fixture();
        blobs.get_mut(path).unwrap()[at] ^= 1;
        assert!(refused(&blobs).contains("historical bytes changed"), "{at}");
    }
    let mut blobs = fixture();
    blobs.get_mut(path).unwrap().truncate(pinned - 1);
    assert!(refused(&blobs).contains("historical prefix truncated"));
}

#[test]
fn license_line_exception_covers_that_line_and_nothing_else() {
    let pins = frozen().unwrap().files;
    for path in LICENSE_LINE_FILES {
        let current = fixture()[path].clone();
        // Restoring the one PolyForm line yields exactly the pinned bytes.
        assert_eq!(
            content_hash(&license_line_restored(path, &current)),
            pins[path].sha256,
            "{path}"
        );
        let mutations = [
            [current.as_slice(), &b" "[..]].concat(),
            {
                let mut b = current.clone();
                b[0] ^= 1;
                b
            },
            replace_once(&current, POLYFORM_LINE, b"\nlicense = \"MIT\"\n"),
            replace_once(
                &current,
                POLYFORM_LINE,
                b"\nlicense = \"PolyForm-Noncommercial-1.0.0\" \n",
            ),
            replace_once(
                &current,
                POLYFORM_LINE,
                &[POLYFORM_LINE, &POLYFORM_LINE[1..]].concat(),
            ),
            replace_once(&current, b"\nversion = ", b"\nversion  = "),
        ];
        for (i, mutated) in mutations.into_iter().enumerate() {
            let mut blobs = fixture();
            blobs.insert(path.to_string(), mutated);
            assert!(
                refused(&blobs).contains("historical bytes changed"),
                "{path} mutation {i}"
            );
        }
    }
}

#[test]
fn license_line_exception_is_limited_to_the_four_manifests() {
    for path in [
        "engine/Cargo.toml",
        "engine/Cargo.lock",
        "PROGRAM/CLAIMS.md",
    ] {
        let with_line = [fixture()[path].as_slice(), POLYFORM_LINE].concat();
        assert_eq!(*license_line_restored(path, &with_line), with_line[..]);
        let mut blobs = fixture();
        blobs.insert(path.into(), with_line);
        assert!(
            refused(&blobs).contains("historical bytes changed"),
            "{path}"
        );
    }
}

#[test]
fn pair_input_check_allows_only_the_license_line_in_the_two_pair_manifests() {
    let blobs = fixture();
    let pins = frozen().unwrap().files;
    for path in ["engine/manifest/Cargo.toml", "engine/pair/Cargo.toml"] {
        // The producer-revision bytes are the pinned bytes (the reproduction checks both).
        let original = license_line_restored(path, &blobs[path]).into_owned();
        assert_eq!(content_hash(&original), pins[path].sha256);
        assert!(pair_input_unchanged(path, &blobs[path], &original));
        let trailing = [blobs[path].as_slice(), &b" "[..]].concat();
        assert!(!pair_input_unchanged(path, &trailing, &original));
        let other = replace_once(&blobs[path], POLYFORM_LINE, b"\nlicense = \"MIT\"\n");
        assert!(!pair_input_unchanged(path, &other, &original));
        let version = replace_once(&blobs[path], b"\nversion = ", b"\nversion  = ");
        assert!(!pair_input_unchanged(path, &version, &original));
    }
    // Every other pair input must still be byte-identical.
    let path = "engine/Cargo.toml";
    let original = blobs[path].clone();
    assert!(pair_input_unchanged(path, &original, &original));
    let with_line = [original.as_slice(), POLYFORM_LINE].concat();
    assert!(!pair_input_unchanged(path, &with_line, &original));
}
