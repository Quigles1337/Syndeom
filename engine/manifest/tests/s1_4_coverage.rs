//! Behavioral tests of the reporting rules, using independent small fixtures.
#[allow(dead_code)]
#[path = "../examples/support/coverage.rs"]
mod coverage;

use coverage::*;
use serde_json::json;
use std::collections::BTreeMap;
use syndeom_manifest::*;

fn fixture() -> Manifest {
    let adr = include_str!("../../../PROGRAM/DECISIONS/ADR-0001.md");
    let start = adr.find("```json\n").unwrap() + 8;
    let end = start + adr[start..].find("\n```").unwrap();
    let mut m = from_json_str(&adr[start..end]).unwrap();
    m.manifest_schema = MANIFEST_SCHEMA.into();
    m.origin = Origin::GroundTruth;
    m.generator.rng = RNG_NONE.into();
    m.seed = None;
    m.commanded.clear();
    m.achieved.clear();
    m.fidelity.per_statistic.clear();
    m.config = json!({"sources":["S-0007"],"item":"independent test fixture, never a corpus item","licence":"test-only","status":{"kind":"mathlib","reference":"fixture"}});
    m.ground_truth.as_mut().unwrap().derivation = Derivation::FormalizedInMathlib;
    identity(&mut m);
    m.check(FileBytes::default()).unwrap();
    m
}

fn identity(m: &mut Manifest) {
    m.config_hash = config_hash(&m.config).unwrap();
    m.artifact_id = artifact_id(
        &m.generator.id,
        &m.generator.version,
        &m.config_hash,
        m.seed,
    );
}
fn classes() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("FLAW-0001".into(), "sign".into()),
        ("FLAW-0005".into(), "hypothesis".into()),
    ])
}
fn flaw(instance: u64, class: &str, severity: Severity) -> Flaw {
    Flaw {
        instance,
        class: class.into(),
        sub_variant: None,
        location: Location::Informal(InformalLocation {
            artifact: InformalTag::Informal,
            step: 1,
            span: [0, 1],
        }),
        severity,
        lean_catchable: LeanCatchable::YesKernel,
        dials: json!({}),
    }
}
fn flawed() -> Manifest {
    let mut m = fixture();
    let gt = m.ground_truth.as_mut().unwrap();
    gt.proof_status = ProofStatus::Invalid;
    gt.correct = false;
    gt.flaws = vec![flaw(1, "FLAW-0005", Severity::Gap)];
    m
}
fn raw(m: &Manifest) -> Manifest {
    let mut p = m.clone();
    p.config["item"] = json!("raw fixture");
    identity(&mut p);
    let f = p.artifact.files.informal.as_mut().unwrap();
    f.file = "original.bin".into();
    f.format = FORMAT_RAW.into();
    f.n_tokens = None;
    f.n_steps = None;
    f.n_lemmas = None;
    for fl in &mut p.ground_truth.as_mut().unwrap().flaws {
        fl.location = Location::Raw(RawLocation {
            artifact: FileKey::Informal,
            locator: "test locator".into(),
        });
    }
    p
}
fn child(p: &Manifest) -> Manifest {
    let mut c = p.clone();
    c.config["derived_from"] = json!(p.artifact_id);
    c.config["method"] = json!("test/1");
    identity(&mut c);
    let f = c.artifact.files.informal.as_mut().unwrap();
    f.file = FILE_INFORMAL.into();
    f.format = FORMAT_INFORMAL_MD.into();
    f.n_tokens = Some(44);
    f.n_steps = Some(2);
    f.n_lemmas = Some(0);
    for fl in &mut c.ground_truth.as_mut().unwrap().flaws {
        fl.location = Location::Informal(InformalLocation {
            artifact: InformalTag::Informal,
            step: 1,
            span: [0, 1],
        });
    }
    c
}
fn item(m: Manifest) -> Item {
    let (unflagged, truth) = qualify(&[], m.ground_truth.as_ref().unwrap().statement_status.value);
    Item {
        root_id: m.artifact_id.clone(),
        representations: vec![m.artifact_id.clone()],
        sources: vec!["S-0007".into()],
        documentary_family: "fixture".into(),
        admission: "test only".into(),
        evidence: vec![],
        issues: vec![],
        unflagged_documented_subset: unflagged,
        truth_dimension_eligible: truth,
        scoring_ready: false,
        development_exposure: "test fixture".into(),
        lengths: lengths(&m),
        root_manifest: m,
    }
}
fn bins() -> Bins {
    Bins {
        schema: "s1-4-coverage/1".into(),
        population: "test".into(),
        edge_rule: "test".into(),
        role_rule: "test".into(),
        boundaries: BTreeMap::from([
            ("informal".into(), vec![7, 14, 28, 56, 112]),
            ("formal".into(), vec![14, 28]),
        ]),
    }
}

#[test]
fn parent_child_is_one_logical_root() {
    let p = raw(&flawed());
    let c = child(&p);
    let roots = lineage(&BTreeMap::from([
        (p.artifact_id.clone(), p.clone()),
        (c.artifact_id.clone(), c.clone()),
    ]))
    .unwrap();
    assert_eq!(roots[&c.artifact_id], p.artifact_id);
    assert_eq!(
        roots
            .values()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        1
    );
}

#[test]
fn missing_parent_is_rejected() {
    let p = raw(&fixture());
    let c = child(&p);
    assert!(
        lineage(&BTreeMap::from([(c.artifact_id.clone(), c)]))
            .unwrap_err()
            .contains("missing parent")
    );
}

#[test]
fn cycles_are_rejected_before_inheritance() {
    let mut a = fixture();
    a.artifact_id = "a".into();
    a.config["derived_from"] = json!("b");
    let mut b = a.clone();
    b.artifact_id = "b".into();
    b.config["derived_from"] = json!("a");
    assert!(
        lineage(&BTreeMap::from([("a".into(), a), ("b".into(), b)]))
            .unwrap_err()
            .contains("cycle")
    );
}

#[test]
fn inherited_status_and_source_changes_are_rejected() {
    let p = raw(&flawed());
    let c = child(&p);
    let mut altered = c.clone();
    altered
        .ground_truth
        .as_mut()
        .unwrap()
        .statement_status
        .value = Some(false);
    assert!(
        lineage(&BTreeMap::from([
            (p.artifact_id.clone(), p.clone()),
            (altered.artifact_id.clone(), altered)
        ]))
        .is_err()
    );
    let mut altered = c;
    altered.config["sources"] = json!(["S-0008"]);
    assert!(
        lineage(&BTreeMap::from([
            (p.artifact_id.clone(), p),
            (altered.artifact_id.clone(), altered)
        ]))
        .unwrap_err()
        .contains("sources")
    );
}

#[test]
fn unknown_class_is_not_silently_ignored() {
    let mut m = flawed();
    m.ground_truth.as_mut().unwrap().flaws[0].class = "FLAW-9999".into();
    assert!(
        validate_labels(&m, &classes())
            .unwrap_err()
            .contains("unknown class")
    );
}

#[test]
fn null_truth_is_separate_and_excluded_from_truth_cells() {
    let mut m = flawed();
    let gt = m.ground_truth.as_mut().unwrap();
    gt.statement_status.value = None;
    gt.statement_status.method = StatementMethod::Undetermined;
    validate_labels(&m, &classes()).unwrap();
    let rows = coverage(&[item(m)], &classes(), &bins());
    let total = rows.last().unwrap();
    assert_eq!(total.truth["undetermined"], 1);
    assert_eq!(total.truth["false"], 0);
    assert_eq!(total.truth_eligible["excluded"], 1);
}

#[test]
fn status_hold_preserves_label_and_qualifies_eligibility() {
    let issue = Issue {
        kind: "promotion-hold".into(),
        reason: "source support questioned".into(),
        evidence: vec!["fixture record".into()],
    };
    assert_eq!(qualify(&[issue], Some(false)), (false, false));
    let severity = Issue {
        kind: "severity-review".into(),
        reason: "classification question".into(),
        evidence: vec!["fixture record".into()],
    };
    assert_eq!(qualify(&[severity], Some(true)), (false, true));
}

#[test]
fn raw_missing_length_is_not_a_canonical_zero() {
    let p = raw(&fixture());
    assert!(lengths(&p).is_empty());
    let mut c = child(&p);
    c.artifact.files.informal.as_mut().unwrap().n_steps = Some(0);
    let l = lengths(&c);
    assert_eq!(l[0].n_steps, Some(0));
    assert_eq!(l[0].role, "statement-only");
    let rows = coverage(&[item(p), item(c)], &classes(), &bins());
    let total = rows.last().unwrap();
    assert_eq!(total.missing_canonical_length, 1);
    assert_eq!(total.steps["min"], 0);
    assert_eq!(total.steps["missing"], 1);
    assert_eq!(total.positive_length_informal_controls, 0);
    assert_eq!(total.statement_only_controls, 1);
    assert_eq!(total.proof_steps["missing"], 1);
    assert_eq!(total.proof_steps["excluded_statement_only"], 1);
}

#[test]
fn overlapping_classes_instances_and_control_totals_are_distinct() {
    let mut m = flawed();
    m.ground_truth.as_mut().unwrap().flaws.extend([
        flaw(2, "FLAW-0005", Severity::Fatal),
        flaw(3, "FLAW-0001", Severity::Repairable),
    ]);
    let rows = coverage(&[item(m), item(fixture())], &classes(), &bins());
    assert_eq!(rows[0].items, 1);
    assert_eq!(rows[1].items, 1);
    assert_eq!(rows[1].flaw_instances, 2);
    assert_eq!(rows[1].severity_items["fatal"], 1);
    assert_eq!(rows[1].severity_items["gap"], 1);
    let total = rows.last().unwrap();
    assert_eq!(
        (total.items, total.flaw_instances, total.correct_controls),
        (2, 3, 1)
    );
    assert_eq!(total.proof_status["invalid"], 1);
    assert_eq!(total.truth["true"], 2);
    assert_eq!(total.lean_catchable_items["YES-KERNEL"], 1);
    assert_eq!(total.lean_catchable_instances["YES-KERNEL"], 3);
}

#[test]
fn octave_boundaries_empty_bins_and_overflow_are_explicit() {
    let edges = octave_edges(&[7, 75]).unwrap();
    assert_eq!(edges, vec![7, 14, 28, 56, 112]);
    assert_eq!(bin(6, &edges), "underflow");
    assert_eq!(bin(7, &edges), "[7, 14)");
    assert_eq!(bin(14, &edges), "[14, 28)");
    assert_eq!(bin(112, &edges), "overflow");
    assert!(octave_edges(&[0, 7]).is_err());
    let rows = coverage(&[item(fixture())], &classes(), &bins());
    assert_eq!(rows.last().unwrap().informal_bins["[7, 14)"], 0);
}

#[test]
fn formal_alias_and_statement_roles_do_not_pool() {
    let mut m = fixture();
    m.config["method"] = json!("reviewed-statement-mathlib-alias/1");
    m.artifact.files.informal.as_mut().unwrap().n_steps = Some(0);
    m.artifact.files.informal.as_mut().unwrap().n_tokens = Some(7);
    m.artifact.files.formal = Some(FormalFile {
        file: "Proof.lean".into(),
        format: FORMAT_LEAN.into(),
        storage: Storage::External,
        content_hash: "a".repeat(64),
        n_bytes: 142,
        n_tokens: Some(14),
        n_lines: Some(4),
        n_declarations: Some(1),
        main_declaration: Some("fixture".into()),
    });
    let rows = coverage(&[item(m)], &classes(), &bins());
    let total = rows.last().unwrap();
    assert_eq!(total.informal_bins["[7, 14)"], 1);
    assert_eq!(total.informal_proof_bins.values().sum::<usize>(), 0);
    assert_eq!(total.informal_statement_bins.values().sum::<usize>(), 1);
    assert_eq!(total.formal_bins["[14, 28)"], 1);
    assert_eq!(total.steps["n"], 1);
}

#[test]
fn harvest_multiline_rows_and_unassigned_candidates() {
    let rows=harvest("| 1 | `0101.0001` | v2 | **admitted ×2** | source record\n`s1_x-v0.1.0-abc` and `s1_x-v0.1.0-def` |\n| 2 | `0101.0002` | v3 | **found-but-unclassifiable** | no supported class |\n").unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].artifact_ids.len(), 2);
    assert!(rows[1].suggested_classes.is_empty());
    assert!(harvest("| 2 | `0101.0002` | v3 | **rejected** | reason |\n").is_err());
}

#[test]
fn changed_added_and_missing_frozen_inputs_are_rejected() {
    let expected = BTreeMap::from([("a".into(), "hash".into())]);
    assert!(verify_frozen(&expected, &expected).is_ok());
    let mut changed = expected.clone();
    changed.insert("a".into(), "changed".into());
    assert!(verify_frozen(&changed, &expected).is_err());
    let mut extra = expected.clone();
    extra.insert("b".into(), "new".into());
    assert!(verify_frozen(&extra, &expected).is_err());
    assert!(verify_frozen(&BTreeMap::new(), &expected).is_err());
}

#[test]
fn serialization_is_deterministic_across_insertion_orders() {
    let a = vec![item(flawed()), item(fixture()), item(raw(&flawed()))];
    let b = a.iter().rev().cloned().collect::<Vec<_>>();
    let first = json_bytes(&coverage(&a, &classes(), &bins())).unwrap();
    let second = json_bytes(&coverage(&b, &classes(), &bins())).unwrap();
    assert_eq!(first, second);
    assert!(first.ends_with(b"\n"));
}

#[test]
fn existing_output_directory_is_refused_without_touching_files() {
    let path =
        std::env::temp_dir().join(format!("syndeom-coverage-refusal-{}", std::process::id()));
    let files = BTreeMap::from([("sentinel.json".into(), b"original\n".to_vec())]);
    output_dir(&path, files).unwrap();
    assert!(
        output_dir(
            &path,
            BTreeMap::from([("sentinel.json".into(), b"changed".to_vec())])
        )
        .is_err()
    );
    assert_eq!(
        std::fs::read(path.join("sentinel.json")).unwrap(),
        b"original\n"
    );
    std::fs::remove_file(path.join("sentinel.json")).unwrap();
    std::fs::remove_dir(path).unwrap();
}

#[test]
fn malformed_counts_and_truth_are_delegated_to_manifest_validator() {
    let mut m = raw(&flawed());
    m.artifact.files.informal.as_mut().unwrap().n_steps = Some(0);
    assert!(validate_labels(&m, &classes()).is_err());
    let mut m = fixture();
    m.ground_truth.as_mut().unwrap().statement_status.value = None;
    assert!(validate_labels(&m, &classes()).is_err());
}
