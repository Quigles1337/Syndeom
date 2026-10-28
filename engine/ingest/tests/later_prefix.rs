//! ADR-0013: the first later prefix's inventory and coverage. The working tree yields the retained
//! report; a byte changed in a pinned input, a removed prefix file or an addition under
//! `groundtruth/` fails; a change outside the inputs, or a later append to the harvest log, passes;
//! the prefix overlay must list exactly the prefix's roots.
#[allow(dead_code)]
#[path = "../examples/support/later_prefix.rs"]
mod later_prefix;

use later_prefix::*;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The working tree's bytes for the accepted baseline, the prefix's files, and the files outside
/// the baseline that the event reads.
fn fixture() -> Blobs {
    static INPUTS: OnceLock<Blobs> = OnceLock::new();
    INPUTS
        .get_or_init(|| {
            let repo = repo();
            let mut paths: Vec<String> = frozen().unwrap().files.into_keys().collect();
            paths.extend(prefix().unwrap().files.into_keys());
            paths.extend(
                [
                    MAP,
                    DECISION,
                    PREFIX_INPUTS,
                    OVERLAY,
                    "analysis/s1-held-harvest-review/REVIEW.md",
                ]
                .map(String::from),
            );
            let effective = format!("groundtruth/{NEW}");
            for entry in std::fs::read_dir(repo.join(&effective)).unwrap() {
                let name = entry.unwrap().file_name().into_string().unwrap();
                paths.push(format!("{effective}/{name}"));
            }
            paths
                .into_iter()
                .map(|p| {
                    let b = std::fs::read(repo.join(&p)).unwrap_or_else(|e| panic!("{p}: {e}"));
                    (p, b)
                })
                .collect()
        })
        .clone()
}

fn refused(blobs: &Blobs) -> String {
    match inventory(blobs) {
        Ok(_) => panic!("expected a refusal"),
        Err(e) => e,
    }
}

fn prefix_file(name: &str) -> String {
    format!(
        "groundtruth/{}/{name}",
        prefix().unwrap().item_directories[0]
    )
}

fn retained(name: &str) -> Vec<u8> {
    std::fs::read(repo().join(format!("{PACKET}/report/{name}"))).unwrap()
}

fn overlay_with(edit: impl FnOnce(&mut Value)) -> Blobs {
    let mut blobs = fixture();
    let mut o: Value = serde_json::from_slice(&blobs[OVERLAY]).unwrap();
    edit(&mut o);
    blobs.insert(OVERLAY.into(), serde_json::to_vec(&o).unwrap());
    blobs
}

#[test]
fn working_tree_yields_the_retained_report() {
    let event = inventory(&fixture()).unwrap();
    assert_eq!(
        json_bytes(&event.inventory).unwrap(),
        retained("inventory.json")
    );
    assert_eq!(
        json_bytes(&event.coverage).unwrap(),
        retained("coverage.json")
    );
    assert_eq!(
        render(&event.coverage).unwrap().into_bytes(),
        retained("report.md")
    );
}

#[test]
fn population_is_forty_representations_and_thirty_seven_logical_items() {
    let event = inventory(&fixture()).unwrap();
    let counts = &event.inventory["counts"];
    assert_eq!(counts["archived_representations"], 40);
    assert_eq!(counts["archived_raw_representations"], 38);
    assert_eq!(counts["canonical_representations"], 2);
    assert_eq!(counts["effective_logical_items"], 37);
    assert_eq!(counts["prefix_logical_items"], 5);
    assert_eq!(event.coverage["harvest_candidates"], 30);
    assert_eq!(event.coverage["prefix_through_arxiv_id"], "1404.4599");
    let rows = event.coverage["archive_coverage"].as_array().unwrap();
    assert_eq!(rows.len(), 15);
    assert_eq!(rows[14]["class"], "TOTAL");
    assert_eq!(rows[14]["items"], 37);
}

#[test]
fn a_changed_pinned_input_fails() {
    for path in [
        prefix_file("manifest.json"),
        prefix_file("RECORD.md"),
        "PROGRAM/FLAWS.md".into(),
        BINS.into(),
        DISPOSITIONS.into(),
        manifest_path(OLD),
    ] {
        let mut blobs = fixture();
        blobs.get_mut(&path).unwrap().push(b'\n');
        assert_eq!(refused(&blobs), format!("historical bytes changed: {path}"));
        let mut blobs = fixture();
        blobs.get_mut(&path).unwrap()[0] ^= 1;
        assert_eq!(refused(&blobs), format!("historical bytes changed: {path}"));
    }
}

#[test]
fn a_removed_prefix_file_fails() {
    let path = prefix_file("RECORD.md");
    let mut blobs = fixture();
    blobs.remove(&path);
    assert_eq!(refused(&blobs), format!("missing input {path}"));
}

#[test]
fn the_harvest_log_is_read_to_its_pinned_prefix() {
    let mut blobs = fixture();
    blobs
        .get_mut(HARVEST_LOG)
        .unwrap()
        .extend_from_slice(b"\n| 31 | `1406.7174` | v2 | **admitted** | a later harvest's row |\n");
    let event = inventory(&blobs).unwrap();
    assert_eq!(event.coverage["harvest_candidates"], 30);
    let mut blobs = fixture();
    blobs.get_mut(HARVEST_LOG).unwrap()[100] ^= 1;
    assert_eq!(
        refused(&blobs),
        format!("historical bytes changed: {HARVEST_LOG}")
    );
}

#[test]
fn an_addition_under_groundtruth_fails() {
    let mut blobs = fixture();
    let path = "groundtruth/s1_ingest-v0.1.0-000000000000/RECORD.md";
    blobs.insert(path.into(), b"later prefix".to_vec());
    assert_eq!(
        refused(&blobs),
        format!("unexpected addition in preserved scope: {path}")
    );
}

#[test]
fn changes_outside_the_inputs_pass() {
    let mut blobs = fixture();
    for path in [
        "PROGRAM/STATE.md",
        "PROGRAM/SOURCES.md",
        "PROGRAM/SYNDEOM-PROGRAM.md",
        "engine/Cargo.toml",
        "README.md",
    ] {
        blobs.get_mut(path).unwrap()[0] ^= 1;
    }
    blobs.insert("engine/generate/Cargo.toml".into(), b"[package]".to_vec());
    blobs.insert(format!("{PACKET}/report/receipt.json"), b"{}".to_vec());
    let event = inventory(&blobs).unwrap();
    assert_eq!(event.inventory["counts"]["effective_logical_items"], 37);
}

#[test]
fn the_overlay_lists_exactly_the_prefix_roots() {
    let missing = overlay_with(|o| {
        o["reviews"].as_array_mut().unwrap().pop();
    });
    let listed_base = overlay_with(|o| {
        o["reviews"][0]["root_id"] = OLD.into();
    });
    let unknown_kind = overlay_with(|o| {
        o["reviews"][0]["issues"] =
            serde_json::json!([{"kind":"verdict","reason":"r","evidence":[PREFIX_INPUTS]}]);
    });
    let no_basis = overlay_with(|o| {
        o["reviews"][0]["basis"] = serde_json::json!([]);
    });
    for blobs in [missing, listed_base] {
        assert_eq!(
            refused(&blobs),
            "the prefix overlay must list each prefix root once, and only those"
        );
    }
    for blobs in [unknown_kind, no_basis] {
        assert_eq!(refused(&blobs), "unknown or unsupported prefix review");
    }
}

#[test]
fn a_prefix_issue_leaves_the_unflagged_subset() {
    let base = inventory(&fixture()).unwrap();
    let flagged = overlay_with(|o| {
        o["reviews"][0]["issues"] = serde_json::json!([
            {"kind":"pending-review","reason":"test","evidence":[PREFIX_INPUTS]}
        ]);
    });
    let event = inventory(&flagged).unwrap();
    let before = base.inventory["counts"]["unflagged_documented_subset"]
        .as_u64()
        .unwrap();
    assert_eq!(
        event.inventory["counts"]["unflagged_documented_subset"],
        before - 1
    );
    assert_eq!(
        event.coverage["issues_by_kind"]["pending-review"],
        serde_json::json!([prefix().unwrap().item_directories[0]])
    );
}
