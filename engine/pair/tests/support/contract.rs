//! Development controls only. Real fixed source success is exercised after producer commit.
use super::*;

const FIXTURE_COMMIT: &str = "1111111111111111111111111111111111111111";

#[test]
fn reviewed_identities_and_canonical_shape_are_fixed() {
    let a = reviewed_config(Mode::Archive).unwrap();
    let c = reviewed_config(Mode::Canonical).unwrap();
    assert_eq!(config_hash(&a).unwrap(), ARCHIVE_HASH);
    assert_eq!(config_hash(&c).unwrap(), CHILD_HASH);
    assert_eq!(c["derived_from"], "s1_pair_archive-v0.1.0-094625fa10ff");
    let (i, f) = canonical_bytes().unwrap();
    assert_eq!((i.len(), f.len()), (118, 142));
    assert_eq!(
        content_hash(&i),
        "56983d03aa658d0315659b88147f6277e7cb6b883e3ce5a6db099094b5949b95"
    );
    assert_eq!(
        content_hash(&f),
        "038c70bf73cacf9eca975bd58ac6d64c23f3227474654d0c18ee2a93f5be09a2"
    );
    let parent = expected_manifest(Mode::Archive, FIXTURE_COMMIT).unwrap();
    let child = expected_manifest(Mode::Canonical, FIXTURE_COMMIT).unwrap();
    // Metadata/canonical success only: this test does not claim raw byte verification.
    validate_child_record(&child, &parent, &i, &f, FIXTURE_COMMIT).unwrap();
    assert!(child.commanded.is_empty());
    assert_eq!(child.artifact.files.informal.unwrap().n_steps, Some(0));
}

#[test]
fn changed_inputs_with_matching_caller_hashes_still_fail_reviewed_contract() {
    let mut a = reviewed_config(Mode::Archive).unwrap();
    a["files"]["informal"]["sha256"] = json!(content_hash(b"synthetic substitute"));
    a["files"]["informal"]["n_bytes"] = json!(20);
    assert!(validate_config(&a, Mode::Archive).is_err());
    let mut s = Sources::new();
    for spec in context_specs().unwrap() {
        s.insert(
            spec["file"].as_str().unwrap().into(),
            b"synthetic substitute".to_vec(),
        );
    }
    assert!(
        validate_sources(&s)
            .unwrap_err()
            .contains("changed/unreviewed")
    );
    s.remove("PrimeFin.lean");
    assert!(validate_sources(&s).unwrap_err().contains("missing"));
}

#[test]
fn unknown_missing_fields_and_substituted_recipe_type_declaration_fail() {
    let original = reviewed_config(Mode::Canonical).unwrap();
    for (pointer, value) in [
        ("/recipe/formal_declaration", json!("Nat.other")),
        ("/recipe/formal_type", json!("True")),
        ("/recipe/statement", json!("A different claim")),
        (
            "/recipe/informal_proof_steps",
            json!(["invented source proof"]),
        ),
        ("/method", json!("unreviewed/1")),
        ("/toolchain/mathlib", json!("0".repeat(40))),
        ("/derived_from", json!("wrong-parent")),
        ("/parent_config_sha256", json!("0".repeat(64))),
    ] {
        let mut c = original.clone();
        *c.pointer_mut(pointer).unwrap() = value;
        assert!(validate_config(&c, Mode::Canonical).is_err(), "{pointer}");
    }
    let mut c = original.clone();
    c["recipe"]["unknown"] = json!(true);
    assert!(validate_config(&c, Mode::Canonical).is_err());
    c = original;
    c["recipe"].as_object_mut().unwrap().remove("formal_type");
    assert!(validate_config(&c, Mode::Canonical).is_err());
}

#[test]
fn duplicate_json_keys_are_rejected_at_every_depth() {
    for text in [
        r#"{"method":1,"method":1}"#,
        r#"{"recipe":{"x":1,"x":2}}"#,
        r#"{"files":[{"file":"a","file":"a"}]}"#,
    ] {
        assert!(
            strict_json::parse(text.as_bytes())
                .unwrap_err()
                .contains("duplicate")
        );
    }
    assert!(strict_json::parse(b"{\"a\":1} trailing").is_err());
}

#[test]
fn source_interval_regression_checks_declaration_and_body_not_just_hash() {
    // Synthetic line-position fixture; never raw source or a corpus item.
    let raw = format!("{}{}\n{}\n\n", "\n".repeat(26), DECL, BODY).into_bytes();
    let mut a = reviewed_config(Mode::Archive).unwrap();
    let mut c = reviewed_config(Mode::Canonical).unwrap();
    source_map_check(&raw, ALIAS, &a, &c).unwrap();
    c["recipe"]["source_map"]["formal"]["source_lines_one_based"] = json!([28, 29]);
    a["files"]["formal"]["locator"] = json!("Nat.infinite_setOfPred_prime, lines 28-29");
    assert!(
        source_map_check(&raw, ALIAS, &a, &c)
            .unwrap_err()
            .contains("declaration/proof")
    );
    c["recipe"]["source_map"]["formal"]["source_lines_one_based"] = json!([27, 27]);
    assert!(source_map_check(&raw, ALIAS, &a, &c).is_err());
    c = reviewed_config(Mode::Canonical).unwrap();
    a = reviewed_config(Mode::Archive).unwrap();
    c["recipe"]["source_map"]["formal"]["canonical_line"] = json!(4);
    assert!(source_map_check(&raw, ALIAS, &a, &c).is_err());
    c["recipe"]["source_map"]["formal"]["source_lines_one_based"] = json!([0, u64::MAX]);
    assert!(source_map_check(&raw, ALIAS, &a, &c).is_err());
}

#[test]
fn self_consistent_but_wrong_parent_config_and_metadata_are_rejected() {
    let parent = expected_manifest(Mode::Archive, FIXTURE_COMMIT).unwrap();
    for key in [
        "sources",
        "item",
        "licence",
        "status",
        "toolchain",
        "attribution",
    ] {
        let mut p = parent.clone();
        p.config[key] = json!("unreviewed");
        p.config_hash = config_hash(&p.config).unwrap();
        p.artifact_id = artifact_id("s1_pair_archive", VERSION, &p.config_hash, None);
        assert!(validate_parent_record(&p, FIXTURE_COMMIT).is_err(), "{key}");
    }
    let mut p = parent.clone();
    p.engine_commit = "2".repeat(40);
    assert!(validate_parent_record(&p, FIXTURE_COMMIT).is_err());
    p = parent;
    p.artifact.kind = Kind::Informal;
    p.artifact.files.formal = None;
    p.toolchain = None;
    assert!(validate_parent_record(&p, FIXTURE_COMMIT).is_err());
}

#[test]
fn canonical_both_byte_checks_and_stronger_inheritance_are_required() {
    let p = expected_manifest(Mode::Archive, FIXTURE_COMMIT).unwrap();
    let c = expected_manifest(Mode::Canonical, FIXTURE_COMMIT).unwrap();
    let (i, f) = canonical_bytes().unwrap();
    assert!(validate_child_record(&c, &p, b"", &f, FIXTURE_COMMIT).is_err());
    assert!(validate_child_record(&c, &p, &i, b"", FIXTURE_COMMIT).is_err());
    for key in [
        "sources",
        "item",
        "licence",
        "status",
        "parent_config_sha256",
        "derived_from",
    ] {
        let mut mutated = c.clone();
        mutated.config[key] = json!("changed");
        mutated.config_hash = config_hash(&mutated.config).unwrap();
        mutated.artifact_id = artifact_id("s1_pair", VERSION, &mutated.config_hash, None);
        assert!(validate_child_record(&mutated, &p, &i, &f, FIXTURE_COMMIT).is_err());
    }
    let mut changed = c.clone();
    changed
        .ground_truth
        .as_mut()
        .unwrap()
        .statement_status
        .reference = "unreviewed".into();
    assert!(validate_child_record(&changed, &p, &i, &f, FIXTURE_COMMIT).is_err());
    changed = c;
    changed.artifact.files.formal.as_mut().unwrap().storage = Storage::External;
    assert!(validate_child_record(&changed, &p, &i, &f, FIXTURE_COMMIT).is_err());
}

#[test]
fn manifest_unknown_fields_and_wrong_producer_are_refused() {
    let p = expected_manifest(Mode::Archive, FIXTURE_COMMIT).unwrap();
    let mut value = serde_json::to_value(&p).unwrap();
    value["unexpected"] = json!(true);
    assert!(parse_manifest(serde_json::to_string(&value).unwrap().as_bytes()).is_err());
    assert!(expected_manifest(Mode::Archive, "HEAD").is_err());
}
