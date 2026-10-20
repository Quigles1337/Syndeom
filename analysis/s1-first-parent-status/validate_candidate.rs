//! Focused proposal validator using the existing authoritative manifest library.
//! Does not admit an item, change a label, or implement a supersession platform.
use serde_json::{Value, json};
use std::{env, fs, path::Path};
use syndeom_manifest::{
    FileBytes, Manifest, artifact_id, config_hash, content_hash, from_json_str,
};

const OLD: &str = "s1_ingest-v0.1.0-63e23a952084";
const BASE: &str = "1a76d9ac21cd5d2e4311f9164178327c0ca128bb";

fn read(path: &Path) -> (Manifest, Value, Vec<u8>) {
    let text = fs::read_to_string(path).unwrap();
    let m = from_json_str(&text).unwrap();
    let v: Value = serde_json::from_str(&text).unwrap();
    let bytes = fs::read(
        path.parent()
            .unwrap()
            .join(v["artifact"]["files"]["informal"]["file"].as_str().unwrap()),
    )
    .unwrap();
    m.check(FileBytes {
        informal: Some(&bytes),
        formal: None,
    })
    .unwrap();
    (m, v, bytes)
}

fn mapping_ok(v: &Value, new_id: &str) -> bool {
    *v == json!({
        "event_id": "first-parent-status-2026-10-20",
        "state": "proposed-not-effective",
        "relation": "raw-metadata-amendment",
        "logical_item_key": OLD,
        "old_artifact_id": OLD,
        "proposed_artifact_id": new_id,
        "effective_now": OLD,
        "effective_if_accepted_and_integrated": new_id,
        "effective_items_contributed": 1,
        "corpus_logical_items_now": 32,
        "corpus_logical_items_if_accepted": 32,
        "canonical_eligibility": "held-pending-policy-and-integration"
    })
}

fn main() {
    let a: Vec<_> = env::args().collect();
    assert_eq!(
        a.len(),
        6,
        "old-manifest candidate-manifest witness config mapping"
    );
    let (old, ov, old_bytes) = read(Path::new(&a[1]));
    let (new, nv, new_bytes) = read(Path::new(&a[2]));
    let witness = fs::read(&a[3]).unwrap();
    let mut config: Value = serde_json::from_slice(&fs::read(&a[4]).unwrap()).unwrap();
    let notes = config.as_object_mut().unwrap().remove("notes").unwrap();
    let mapping: Value = serde_json::from_slice(&fs::read(&a[5]).unwrap()).unwrap();
    assert_eq!(old.artifact_id, OLD);
    assert_eq!(old_bytes, new_bytes);
    assert_eq!(new_bytes.len(), 8236);
    assert_eq!(
        content_hash(&new_bytes),
        "5987d8106ae69669b10c876a0915d5ad52ce44a814ea7fe63600c1737642cca8"
    );
    assert_eq!(nv["config"], config);
    assert_eq!(nv["notes"], notes);
    assert_eq!(new.engine_commit, BASE);
    assert_eq!(new.config_hash, config_hash(&new.config).unwrap());
    assert_eq!(
        new.artifact_id,
        artifact_id("s1_ingest", "0.1.0", &new.config_hash, None)
    );
    assert_ne!(old.artifact_id, new.artifact_id);
    let reference = format!(
        "analysis/s1-first-parent-status/WITNESS.md; SHA-256 {}; sections 2-3; Codex GPT-6, 2026-10-20",
        content_hash(&witness)
    );
    let expected = json!({"value": false, "method": "counterexample", "reference": reference});
    assert_eq!(nv["config"]["statement_status"], expected);
    assert_eq!(nv["ground_truth"]["statement_status"], expected);
    assert_eq!(
        ov["config"]["statement_status"],
        json!({"value": false, "method": "source-document", "reference": "S-0025"})
    );
    // Exactly the status evidence differs inside config; all flaw/source/rights fields are retained.
    let mut amended = ov["config"].clone();
    amended["statement_status"] = expected.clone();
    assert_eq!(amended, nv["config"]);
    let mut amended_gt = ov["ground_truth"].clone();
    amended_gt["statement_status"] = expected;
    assert_eq!(amended_gt, nv["ground_truth"]);
    for key in [
        "achieved",
        "artifact",
        "commanded",
        "fidelity",
        "generator",
        "manifest_schema",
        "origin",
        "seed",
        "toolchain",
    ] {
        assert_eq!(ov[key], nv[key], "unexpected changed field {key}");
    }
    assert!(new.config.get("derived_from").is_none());
    assert!(mapping_ok(&mapping, &new.artifact_id));
    // Meaningful negative controls: the proposed old/new relation must not manufacture an item.
    let mut bad_mapping = mapping.clone();
    bad_mapping["effective_items_contributed"] = json!(2);
    assert!(!mapping_ok(&bad_mapping, &new.artifact_id));
    bad_mapping = mapping.clone();
    bad_mapping["effective_now"] = json!(new.artifact_id);
    assert!(!mapping_ok(&bad_mapping, &new.artifact_id));
    let mut bad = new.clone();
    bad.artifact_id = old.artifact_id;
    assert!(
        bad.check(FileBytes {
            informal: Some(&new_bytes),
            formal: None
        })
        .is_err()
    );
    let mut bad_bytes = new_bytes.clone();
    bad_bytes[0] ^= 1;
    assert!(
        new.check(FileBytes {
            informal: Some(&bad_bytes),
            formal: None
        })
        .is_err()
    );
    println!("PASS: old and proposed manifests validate against actual source bytes");
    println!(
        "PASS: duplicated status agrees; exact evidence-only config amendment; no derived_from"
    );
    println!("PASS: one effective item, pending owner decision; four negative controls rejected");
    println!("candidate_id: {}", new.artifact_id);
    println!("config_sha256: {}", new.config_hash);
    println!(
        "manifest_sha256: {}",
        content_hash(&fs::read(&a[2]).unwrap())
    );
    println!("witness_sha256: {}", content_hash(&witness));
}
