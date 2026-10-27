//! Controlled mutations of the actual retained integration inputs; never corpus admissions.
#[allow(dead_code)]
#[path = "../examples/support/effective.rs"]
mod effective;

use effective::*;
use serde_json::{Value, json};
use std::path::Path;
use std::sync::OnceLock;

fn fixture() -> Blobs {
    static INPUTS: OnceLock<Blobs> = OnceLock::new();
    INPUTS
        .get_or_init(|| {
            let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
            let mut paths: Vec<_> = frozen().unwrap().files.into_keys().collect();
            paths.extend([
                MAP.to_string(),
                format!("{PACKET}/OWNER-DECISION.md"),
                manifest_path(NEW),
                format!("groundtruth/{NEW}/RECORD.md"),
                format!("groundtruth/{NEW}/original-0706.0357v70.tar.gz"),
            ]);
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

fn put(blobs: &mut Blobs, path: &str, value: &Value) {
    blobs.insert(path.into(), serde_json::to_vec_pretty(value).unwrap());
}

#[test]
fn integrated_counts_use_one_effective_status_and_keep_stable_key() {
    let report = inventory(&fixture()).unwrap();
    let counts = &report["counts"];
    assert_eq!(counts["archived_representations"], 35);
    assert_eq!(counts["archived_raw_representations"], 33);
    assert_eq!(counts["canonical_representations"], 2);
    assert_eq!(counts["effective_logical_items"], 32);
    assert_eq!(
        counts["statement_status"],
        json!({"true":5,"false":17,"undetermined":10})
    );
    assert_eq!(counts["unflagged_documented_subset"], 18);
    assert_eq!(
        counts["qualified_truth_dimension"],
        json!({"true":5,"false":10})
    );
    assert_eq!(counts["logical_items_without_canonical"], 30);
    assert_eq!(counts["flaw_instances"], 46);
    let items = report["items"].as_array().unwrap();
    let item = items.iter().find(|i| i["logical_item_key"] == OLD).unwrap();
    assert_eq!(item["effective_artifact_id"], NEW);
    assert_eq!(item["archived_representations"], json!([NEW, OLD]));
    assert_eq!(item["statement_status"]["method"], "counterexample");
    assert_eq!(item["author_status"]["reference"], "S-0025");
    assert_eq!(item["truth_dimension_eligible"], true);
    assert_eq!(item["status_support_hold"], false);
    assert_eq!(
        item["canonical_selection_context_locator_review_required"],
        true
    );
    assert!(
        !items
            .iter()
            .any(|i| i["logical_item_key"] == NEW || i["effective_artifact_id"] == OLD)
    );
}

#[test]
fn older_dispositions_and_scope_notes_survive_exactly() {
    let blobs = fixture();
    let old = json(&blobs, "analysis/s1-4/config/dispositions.json").unwrap();
    let report = inventory(&blobs).unwrap();
    for review in old["reviews"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["root_id"] != OLD)
    {
        let item = report["items"]
            .as_array()
            .unwrap()
            .iter()
            .find(|i| i["logical_item_key"] == review["root_id"])
            .unwrap();
        assert_eq!(item["issues"], review["issues"]);
        assert_eq!(item["unflagged_documented_subset"], false);
        if review["issues"][0]["kind"] == "truth-review" {
            assert_eq!(item["truth_dimension_eligible"], false);
        }
    }
    assert_eq!(report["scope_notes"], old["scope_notes"]);
}

#[test]
fn missing_mapping_never_falls_back_to_directory_counting() {
    let mut blobs = fixture();
    blobs.remove(MAP);
    assert!(
        inventory(&blobs)
            .unwrap_err()
            .contains("missing input groundtruth/amendment-map.json")
    );
}

#[test]
fn inconsistent_mapping_and_old_effective_status_are_refused() {
    for (field, value) in [
        ("effective_artifact_id", OLD),
        ("logical_item_key", NEW),
        ("relation", "derived_from"),
        ("accepted_packet_revision", "unreviewed"),
    ] {
        let mut blobs = fixture();
        let mut m = json(&blobs, MAP).unwrap();
        m[field] = value.into();
        put(&mut blobs, MAP, &m);
        assert!(mapping(&blobs).is_err(), "{field}");
    }
}

#[test]
fn duplicate_or_multiple_effective_status_mapping_is_refused() {
    let mut blobs = fixture();
    let original = String::from_utf8(blobs[MAP].clone()).unwrap();
    let duplicate = original.replacen('{', &format!("{{\"effective_artifact_id\":\"{OLD}\","), 1);
    blobs.insert(MAP.into(), duplicate.into_bytes());
    assert!(mapping(&blobs).err().unwrap().contains("duplicate field"));
    blobs.insert(MAP.into(), format!("[{original},{original}]").into_bytes());
    assert!(mapping(&blobs).is_err());
}

#[test]
fn witness_and_actual_owner_decision_are_hash_bound() {
    for path in [
        WITNESS,
        "analysis/s1-first-parent-integration/OWNER-DECISION.md",
    ] {
        let mut blobs = fixture();
        blobs.get_mut(path).unwrap().push(b' ');
        assert!(validate_case(&blobs).is_err(), "{path}");
    }
}

#[test]
fn candidate_identity_and_original_producer_revision_are_bound() {
    for (field, value) in [
        ("artifact_id", OLD),
        ("config_hash", "invented"),
        ("engine_commit", BASE),
    ] {
        let mut blobs = fixture();
        let mut m = json(&blobs, &manifest_path(NEW)).unwrap();
        m[field] = value.into();
        put(&mut blobs, &manifest_path(NEW), &m);
        assert!(
            validate_case(&blobs)
                .unwrap_err()
                .contains("identity/producing")
        );
    }
}

#[test]
fn both_status_fields_must_bind_the_same_exact_witness() {
    for field in ["config", "ground_truth"] {
        let mut blobs = fixture();
        let mut m = json(&blobs, &manifest_path(NEW)).unwrap();
        m[field]["statement_status"]["reference"] = "S-0025".into();
        put(&mut blobs, &manifest_path(NEW), &m);
        assert!(
            validate_case(&blobs)
                .unwrap_err()
                .contains("duplicated statement status")
        );
    }
}

#[test]
fn no_note_rewrite_under_existing_candidate_identity() {
    let mut blobs = fixture();
    let mut m = json(&blobs, &manifest_path(NEW)).unwrap();
    m["notes"] = "Activated by rewriting history".into();
    put(&mut blobs, &manifest_path(NEW), &m);
    assert!(
        validate_case(&blobs)
            .unwrap_err()
            .contains("exact accepted packet")
    );
}

#[test]
fn same_source_bytes_required_in_all_three_locations() {
    let mut blobs = fixture();
    blobs
        .get_mut(&format!("groundtruth/{NEW}/original-0706.0357v70.tar.gz"))
        .unwrap()[0] ^= 1;
    assert!(
        validate_case(&blobs)
            .unwrap_err()
            .contains("source bytes mismatch")
    );
}

#[test]
fn both_archived_representations_are_required() {
    for id in [OLD, NEW] {
        let mut blobs = fixture();
        blobs.remove(&manifest_path(id));
        assert!(inventory(&blobs).is_err());
    }
}

#[test]
fn extra_raw_item_cannot_silently_double_count_this_lemma() {
    let mut blobs = fixture();
    let extra = "groundtruth/unreviewed-duplicate/manifest.json";
    blobs.insert(extra.into(), blobs[&manifest_path(NEW)].clone());
    assert!(
        inventory(&blobs)
            .unwrap_err()
            .contains("unexpected addition in preserved scope")
    );
}

#[test]
fn accepted_inputs_in_every_scope_are_immutable() {
    // ADR-0012: the input set spans the corpus, the accepted candidate packet and the S1.4
    // dispositions overlay; a byte added to any read file fails in each scope.
    let f = frozen().unwrap();
    for path in [
        manifest_path(OLD),
        format!("groundtruth/{OLD}/RECORD.md"),
        WITNESS.into(),
        CANDIDATE_CONFIG.into(),
        format!("{}manifest.json", candidate_dir()),
        DISPOSITIONS.into(),
        "PROGRAM/FLAWS.md".into(),
    ] {
        let mut blobs = fixture();
        blobs.get_mut(&path).unwrap().push(b' ');
        assert!(
            preserve(&blobs, &f)
                .unwrap_err()
                .contains("historical bytes changed"),
            "{path}"
        );
    }
    let mut blobs = fixture();
    blobs.insert(
        "groundtruth/unreviewed-item/RECORD.md".into(),
        b"{}".to_vec(),
    );
    assert!(preserve(&blobs, &f).is_err());
}

#[test]
fn history_outside_the_acceptance_markers_is_not_pinned() {
    // ADR-0012: HANDOFF and ADR-0003 are read only for the acceptance markers, which
    // `validate_case` requires; their other bytes are outside the input set.
    let mut blobs = fixture();
    blobs.get_mut("PROGRAM/HANDOFF.md").unwrap()[0] ^= 1;
    blobs.get_mut("PROGRAM/DECISIONS/ADR-0003.md").unwrap()[0] ^= 1;
    preserve(&blobs, &frozen().unwrap()).unwrap();
    validate_case(&blobs).unwrap();
}

#[test]
fn acceptance_without_adr_or_history_cannot_release_status_support() {
    for path in ["PROGRAM/DECISIONS/ADR-0003.md", "PROGRAM/HANDOFF.md"] {
        let mut blobs = fixture();
        blobs
            .get_mut(path)
            .unwrap()
            .truncate(frozen().unwrap().files[path].n_bytes);
        assert!(
            validate_case(&blobs)
                .unwrap_err()
                .contains("ADR and append-only history")
        );
    }
}
