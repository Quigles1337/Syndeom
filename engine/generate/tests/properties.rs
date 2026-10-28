//! Property tests for `s2_template` (ADR-0011 D10). None of them runs Lean: where a formal file is
//! assembled, the harness observation is the one the D5 prediction implies, and the goldens test
//! pins the observations measured at implementation.

use serde_json::{Value, json};
use syndeom_generate::harness::{Observation, first_error_line, parse_axioms};
use syndeom_generate::plan::{render_formal, render_informal};
use syndeom_generate::template::{
    Prepared, Version, assemble, check_seed, faithful_plan, prepare, prepare_with, seeds,
};
use syndeom_manifest::{
    Location, ProofStatus, Toolchain, Verdict, parse_informal_md, parse_lean, span_text,
};

const COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";

fn toolchain() -> Toolchain {
    Toolchain {
        lean: "leanprover/lean4:v4.34.1".into(),
        mathlib: "d13f23b723b8a846827a245b89c10fc7d3f11612".into(),
    }
}

fn config(theorem: &str, style: &str, target: &str, flaws: Value) -> Value {
    json!({"theorem": theorem, "style": style, "target": target, "flaws": flaws, "padding_ratio": 0})
}

fn spec(class: &str, sub: Option<&str>, severity: &str, dials: Value) -> Value {
    json!({"class": class, "sub_variant": sub, "severity": severity, "site": "auto", "dials": dials})
}

/// The observation D5 predicts: K-class instances fail at the site's first fragment line.
fn predicted(p: &Prepared) -> Option<Observation> {
    let f = p.formal.as_ref()?;
    Some(match &p.injection {
        Some(inj) => Observation {
            exit_code: 1,
            first_error_line: f.fragment_lines[inj.site - 1].map(|r| r[0]),
            axioms: None,
        },
        None => Observation {
            exit_code: 0,
            first_error_line: None,
            axioms: Some(vec!["propext".into()]),
        },
    })
}

fn build_with(version: Version, c: &Value, seed: u64) -> (Prepared, syndeom_manifest::Manifest) {
    let p = prepare_with(version, c, seed).unwrap_or_else(|e| panic!("{c}: {e}"));
    let obs = predicted(&p);
    let tc = p.formal.as_ref().map(|_| toolchain());
    let m = assemble(&p, COMMIT, tc, obs.as_ref()).unwrap_or_else(|e| panic!("{c}: {e}"));
    (p, m)
}

fn build(c: &Value, seed: u64) -> (Prepared, syndeom_manifest::Manifest) {
    build_with(Version::CURRENT, c, seed)
}

/// Every (seed, style, material-or-control, target) the current seed set supports; a seed whose
/// statement is false has no control.
fn every_case() -> Vec<Value> {
    let mut out = Vec::new();
    for seed in seeds(Version::CURRENT) {
        for style in &seed.styles {
            let mut specs = Vec::new();
            if seed.truth.value == Some(true) {
                specs.push(json!([]));
            }
            for m in &style.flaws {
                let sev = serde_json::to_value(m.severity).unwrap();
                specs.push(json!([{
                    "class": m.class, "sub_variant": m.sub_variant, "severity": sev,
                    "site": m.site, "dials": m.dials
                }]));
            }
            for flaws in specs {
                for target in ["informal", "formal", "pair"] {
                    out.push(config(&seed.id, &style.id, target, flaws.clone()));
                }
            }
        }
    }
    out
}

#[test]
fn d9_determinism_and_idempotence() {
    for c in every_case() {
        for seed in [0, 7, u64::MAX] {
            let (p1, m1) = build(&c, seed);
            let (p2, m2) = build(&c, seed);
            assert_eq!(p1, p2);
            assert_eq!(
                syndeom_manifest::to_disk_json(&m1).unwrap(),
                syndeom_manifest::to_disk_json(&m2).unwrap()
            );
            assert_eq!(m1.artifact_id, p1.artifact_id());
            assert!(m1.artifact_id.starts_with("s2_template-v0.2.0-"));
        }
    }
}

#[test]
fn d10_rendered_files_parse_with_the_recorded_counts() {
    for c in every_case() {
        let (p, m) = build(&c, 7);
        if let Some(b) = &p.informal {
            let st = parse_informal_md(b).unwrap();
            let f = m.artifact.files.informal.as_ref().unwrap();
            assert_eq!(
                (Some(st.n_tokens), Some(st.n_steps), Some(st.n_lemmas)),
                (f.n_tokens, f.n_steps, f.n_lemmas)
            );
            assert!(!b.contains(&b'\r'));
        }
        if let Some(r) = &p.formal {
            let st = parse_lean(&r.bytes).unwrap();
            let f = m.artifact.files.formal.as_ref().unwrap();
            assert_eq!(Some(st.n_lines), f.n_lines);
            assert_eq!(st.declarations.last().unwrap().1, p.plan.main_declaration);
        }
    }
}

#[test]
fn d10_locations_resolve_to_the_dials_text() {
    for c in every_case() {
        let (p, m) = build(&c, 7);
        for r in &m.ground_truth.as_ref().unwrap().flaws {
            match &r.location {
                Location::Informal(loc) => {
                    let st = parse_informal_md(p.informal.as_ref().unwrap()).unwrap();
                    let text = span_text(&st.steps[loc.step as usize], loc.span).unwrap();
                    assert_eq!(json!(text), r.dials["inserted"], "{c}");
                    if !r.dials["replaced"].is_null() {
                        assert_ne!(r.dials["inserted"], r.dials["replaced"], "{c}");
                    }
                }
                Location::Formal(loc) => {
                    let bytes = &p.formal.as_ref().unwrap().bytes;
                    let line = std::str::from_utf8(bytes)
                        .unwrap()
                        .lines()
                        .nth(loc.line as usize - 1)
                        .unwrap()
                        .to_string();
                    assert_eq!(
                        format!("  {}", r.dials["inserted"][0].as_str().unwrap()),
                        line
                    );
                }
                Location::Raw(_) => panic!("synthetic records are canonical"),
            }
            if r.dials["replaced"].is_null() {
                // intrinsic: a false statement's argument replaces nothing and names its
                // counterexample
                assert!(r.dials["counterexample"].is_string(), "{c}");
                assert!(r.dials.get("original_steps").is_none(), "{c}");
            } else {
                assert!(
                    r.dials["original_steps"].is_object()
                        || r.dials["original_fragments"].is_object()
                );
            }
        }
    }
}

#[test]
fn d8_counts_are_per_instance_and_predictions_pass() {
    for c in every_case() {
        let (_, m) = build(&c, 7);
        let flaws = &m.ground_truth.as_ref().unwrap().flaws;
        let mut instances: Vec<u64> = flaws.iter().map(|f| f.instance).collect();
        instances.sort();
        instances.dedup();
        let n = instances.len();
        assert_eq!(m.commanded["n_flaws_total"].value, json!(n), "{c}");
        assert_eq!(m.commanded["flaws_confirmed"].value, json!(n), "{c}");
        assert_eq!(
            m.achieved["n_flaws_total"].value,
            Some(json!(n)),
            "{c}: every predicted check passes"
        );
        if c["target"] == "pair" {
            assert_eq!(
                flaws.len(),
                2 * n,
                "{c}: a pair records each instance twice"
            );
        } else {
            assert_eq!(flaws.len(), n, "{c}");
        }
        assert_eq!(m.fidelity.verdict, Verdict::Pass, "{c}");
    }
}

#[test]
fn d8_a_pair_instance_is_confirmed_only_when_both_records_pass() {
    // Al's ruling 3 on PR 26 and the owner note of 2026-10-28 (ADR-0011 Amendments).
    let c = config(
        "sq_plus_one_ge_two_x",
        "expand_square",
        "pair",
        json!([spec(
            "FLAW-0001",
            Some("propagated"),
            "fatal",
            json!({"perturbation": "sign"})
        )]),
    );
    let (p, m) = build(&c, 7);
    let flaws = &m.ground_truth.as_ref().unwrap().flaws;
    assert_eq!(flaws.len(), 2);
    assert!(flaws.iter().all(|f| f.instance == 1));
    assert_eq!(m.commanded["n_flaws_total"].value, json!(1));
    assert_eq!(m.commanded["n_flaws_0001"].value, json!(1));
    assert_eq!(m.commanded["flaws_confirmed"].value, json!(1));
    assert_eq!(m.achieved["n_flaws_total"].value, Some(json!(1)));
    assert_eq!(m.achieved["n_flaws_0001"].value, Some(json!(1)));
    assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(1)));
    assert_eq!(m.fidelity.verdict, Verdict::Pass);
    // the formal record fails its check K: the instance is not confirmed, although the informal
    // record still passes its structural check
    let built = Observation {
        exit_code: 0,
        first_error_line: None,
        axioms: Some(vec![]),
    };
    let m = assemble(&p, COMMIT, Some(toolchain()), Some(&built)).unwrap();
    assert_eq!(m.achieved["n_flaws_total"].value, Some(json!(0)));
    assert_eq!(m.achieved["n_flaws_0001"].value, Some(json!(0)));
    assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(0)));
    assert_eq!(m.fidelity.per_statistic["flaws_confirmed"], Verdict::Fail);
    assert_eq!(m.fidelity.per_statistic["n_flaws_total"], Verdict::Fail);
    assert_eq!(m.fidelity.verdict, Verdict::Fail);
    // 0.1.0 counted records: the same pair commands two, as that version emitted it
    let (_, m01) = build_with(Version::V0_1_0, &c, 7);
    assert_eq!(m01.commanded["n_flaws_total"].value, json!(2));
    assert_eq!(m01.commanded["flaws_confirmed"].value, json!(2));
    assert_eq!(m01.achieved["n_flaws_total"].value, Some(json!(2)));
    assert!(m01.artifact_id.starts_with("s2_template-v0.1.0-"));
    assert_eq!(m01.generator.version, "0.1.0");
}

#[test]
fn d8_a_failed_confirmation_is_a_fail_verdict_not_a_refusal() {
    let c = config(
        "sq_plus_one_ge_two_x",
        "expand_square",
        "formal",
        json!([spec(
            "FLAW-0001",
            Some("local"),
            "repairable",
            json!({"perturbation": "sign"})
        )]),
    );
    let p = prepare(&c, 7).unwrap();
    // the file built: the kernel did not reject it as predicted
    let built = Observation {
        exit_code: 0,
        first_error_line: None,
        axioms: Some(vec![]),
    };
    let m = assemble(&p, COMMIT, Some(toolchain()), Some(&built)).unwrap();
    assert_eq!(m.fidelity.verdict, Verdict::Fail);
    assert_eq!(m.fidelity.per_statistic["flaws_confirmed"], Verdict::Fail);
    assert_eq!(m.fidelity.per_statistic["lean_build"], Verdict::Fail);
    // an error outside the site's fragment is not a confirmation either
    let elsewhere = Observation {
        exit_code: 1,
        first_error_line: Some(8),
        axioms: None,
    };
    let m = assemble(&p, COMMIT, Some(toolchain()), Some(&elsewhere)).unwrap();
    assert_eq!(m.fidelity.per_statistic["flaws_confirmed"], Verdict::Fail);
    assert_eq!(m.fidelity.per_statistic["lean_build"], Verdict::Pass);
}

#[test]
fn d10_every_control_is_correct_and_unflawed_and_a_false_statement_has_none() {
    for seed in seeds(Version::CURRENT) {
        for style in &seed.styles {
            for target in ["informal", "formal", "pair"] {
                let c = config(&seed.id, &style.id, target, json!([]));
                if seed.truth.value == Some(true) {
                    let (_, m) = build(&c, 7);
                    let gt = m.ground_truth.unwrap();
                    assert!(gt.flaws.is_empty());
                    assert_eq!(gt.proof_status, ProofStatus::Valid);
                    assert!(gt.correct);
                } else {
                    assert!(
                        prepare(&c, 7).is_err(),
                        "{c}: a false statement has no control"
                    );
                }
            }
        }
    }
}

#[test]
fn d7_a_false_statement_seed_yields_an_invalid_incorrect_artifact_with_its_counterexample() {
    for seed in seeds(Version::CURRENT)
        .into_iter()
        .filter(|s| s.truth.value == Some(false))
    {
        assert!(seed.truth.reference.ends_with("_false"), "{}", seed.id);
        for style in &seed.styles {
            for m in &style.flaws {
                let sev = serde_json::to_value(m.severity).unwrap();
                let c = config(
                    &seed.id,
                    &style.id,
                    "pair",
                    json!([{"class": m.class, "sub_variant": m.sub_variant, "severity": sev,
                            "site": "auto", "dials": m.dials}]),
                );
                let (_, manifest) = build(&c, 7);
                let gt = manifest.ground_truth.unwrap();
                assert_eq!(gt.statement_status.value, Some(false));
                assert_eq!(gt.proof_status, ProofStatus::Invalid);
                assert!(!gt.correct);
                assert_eq!(gt.flaws.len(), 2);
                for r in &gt.flaws {
                    assert!(r.dials["replaced"].is_null());
                    assert_eq!(r.dials["counterexample"], json!(seed.counterexample));
                }
                assert_eq!(manifest.commanded["lean_build"].value, json!("fail"));
            }
        }
    }
}

#[test]
fn d6_seed_records_are_sourced_outside_the_s1_corpus_and_well_formed() {
    assert_eq!(seeds(Version::V0_1_0).len(), 3);
    let all = seeds(Version::V0_2_0);
    assert_eq!(all.len(), 7);
    let ids: Vec<&str> = all.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(
        ids,
        [
            "nat_le_sq",
            "n_sq_mod_four",
            "sq_plus_one_ge_two_x",
            "continuity_is_not_uniform",
            "x_over_abs_x",
            "rolle_abs",
            "horses",
        ]
    );
    for seed in &all {
        check_seed(seed).unwrap_or_else(|e| panic!("{e}"));
        assert!(!seed.source.is_empty() && !seed.source.contains("groundtruth"));
        assert!(seed.source.contains("FLAWS.md") || seed.source.contains("ADR-0001"));
        assert_eq!(seed.licence, "PolyForm-Noncommercial-1.0.0");
        let false_statement = seed.truth.value == Some(false);
        assert_eq!(seed.counterexample.is_some(), false_statement);
        for style in &seed.styles {
            let plan = faithful_plan(seed, style);
            parse_informal_md(&render_informal(&plan).unwrap()).unwrap();
            parse_lean(&render_formal(&plan).unwrap().bytes).unwrap();
            assert!(!style.flaws.is_empty() || seed.id == "nat_le_sq");
            for m in &style.flaws {
                assert_eq!(m.intrinsic, false_statement, "{}", seed.id);
                let site_text = &style.steps[m.site as usize - 1];
                if m.intrinsic {
                    assert!(m.steps.is_empty());
                    assert!(
                        !site_text.fragment.is_empty(),
                        "the site carries a formal fragment"
                    );
                    if let Some(a) = &m.anchor {
                        assert!(a.replaced.is_none());
                        assert_eq!(site_text.text.matches(&a.inserted).count(), 1);
                    }
                    continue;
                }
                let froms: Vec<u64> = m.steps.iter().map(|s| s.from).collect();
                assert!(
                    froms.windows(2).all(|w| w[0] < w[1]),
                    "from indices increase"
                );
                assert!(
                    froms
                        .iter()
                        .all(|k| *k >= 1 && *k as usize <= style.steps.len())
                );
                let site = m
                    .steps
                    .iter()
                    .find(|s| s.from == m.site)
                    .expect("site kept");
                assert!(
                    !site.fragment.is_empty(),
                    "the site carries a formal fragment"
                );
                if let Some(a) = &m.anchor {
                    assert_eq!(site.text.matches(&a.inserted).count(), 1);
                    let replaced = a.replaced.as_ref().expect("material replaces text");
                    assert_eq!(site_text.text.matches(replaced).count(), 1);
                }
            }
        }
    }
}

#[test]
fn d1_the_0_1_0_seed_set_is_closed() {
    // A 0.2.0 seed is not a seed of 0.1.0: the earlier version refused it, and still does.
    for id in [
        "continuity_is_not_uniform",
        "x_over_abs_x",
        "rolle_abs",
        "horses",
    ] {
        let style = seeds(Version::V0_2_0)
            .into_iter()
            .find(|s| s.id == id)
            .unwrap()
            .styles[0]
            .id
            .clone();
        assert!(
            prepare_with(
                Version::V0_1_0,
                &config(id, &style, "informal", json!([])),
                7
            )
            .is_err()
        );
    }
    assert!(Version::parse("0.3.0").is_none());
    assert_eq!(Version::CURRENT.as_str(), "0.2.0");
}

#[test]
fn d3_unsupported_specs_are_refused() {
    let sq = |flaws: Value| config("sq_plus_one_ge_two_x", "expand_square", "pair", flaws);
    let sign = json!({"perturbation": "sign"});
    let refusals = [
        // shape: missing and extra keys, bad values
        json!({"theorem": "nat_le_sq", "style": "two_cases", "target": "informal", "flaws": []}),
        json!({"theorem": "nat_le_sq", "style": "two_cases", "target": "informal", "flaws": [], "padding_ratio": 0, "x": 1}),
        config("nat_le_sq", "two_cases", "both", json!([])),
        config("no_such_seed", "two_cases", "informal", json!([])),
        config("nat_le_sq", "no_such_style", "informal", json!([])),
        json!({"theorem": "nat_le_sq", "style": "two_cases", "target": "informal", "flaws": [], "padding_ratio": 0.25}),
        // classes, ranges and targets (D3)
        sq(json!([spec("FLAW-0015", None, "gap", json!({}))])),
        sq(json!([spec(
            "FLAW-0001",
            Some("local"),
            "cosmetic",
            sign.clone()
        )])),
        config(
            "sq_plus_one_ge_two_x",
            "expand_square",
            "informal",
            json!([spec("FLAW-0009", Some("axiom"), "gap", json!({}))]),
        ),
        config(
            "sq_plus_one_ge_two_x",
            "expand_square",
            "formal",
            json!([spec("FLAW-0004", None, "gap", json!({}))]),
        ),
        config(
            "sq_plus_one_ge_two_x",
            "expand_square",
            "formal",
            json!([spec("FLAW-0008", None, "gap", json!({}))]),
        ),
        // no material: other dials, another class, an ineligible explicit site, two instances
        sq(json!([spec(
            "FLAW-0001",
            Some("local"),
            "repairable",
            json!({"perturbation": "exponent"})
        )])),
        sq(json!([spec(
            "FLAW-0014",
            None,
            "gap",
            json!({"omitted_case": "odd", "holds": true})
        )])),
        sq(
            json!([{"class": "FLAW-0001", "sub_variant": "local", "severity": "repairable", "site": 2, "dials": sign}]),
        ),
        sq(json!([
            spec("FLAW-0001", Some("local"), "repairable", sign.clone()),
            spec("FLAW-0001", Some("propagated"), "fatal", sign.clone())
        ])),
        // the injector's own dial names are reserved
        sq(json!([spec(
            "FLAW-0001",
            Some("local"),
            "repairable",
            json!({"perturbation": "sign", "site": 1})
        )])),
        sq(json!([spec(
            "FLAW-0001",
            Some("local"),
            "repairable",
            json!({"perturbation": "sign", "counterexample": "none"})
        )])),
        // a false statement has no control, and its material is what the seed names
        config("horses", "induction", "pair", json!([])),
        config(
            "horses",
            "induction",
            "pair",
            json!([spec(
                "FLAW-0006",
                None,
                "gap",
                json!({"mode": "shift-index"})
            )]),
        ),
    ];
    for c in refusals {
        assert!(prepare(&c, 7).is_err(), "should refuse {c}");
    }
}

#[test]
fn d8_harness_parsers() {
    let file = "//?/C:/x/Proof.lean";
    let out = "//?/C:/x/Proof.lean:1:0: warning: deprecated\nTry this: ring_nf\n//?/C:/x/Proof.lean:6:47: error: unsolved goals\n//?/C:/x/Proof.lean:9:2: error: Type mismatch\n";
    assert_eq!(first_error_line(out, file), Some(6));
    assert_eq!(first_error_line("no errors here\n", file), None);
    // an error reported against another file is not this file's first error
    let other =
        "//?/C:/y/Other.lean:2:0: error: unknown\n//?/C:/x/Proof.lean:9:2: error: Type mismatch\n";
    assert_eq!(first_error_line(other, file), Some(9));
    assert_eq!(first_error_line(other, "/tmp/Proof.lean"), None);
    let ax = "'f' depends on axioms: [propext, Classical.choice, Quot.sound]\n";
    assert_eq!(
        parse_axioms(ax, "f"),
        Some(vec![
            "propext".into(),
            "Classical.choice".into(),
            "Quot.sound".into()
        ])
    );
    assert_eq!(
        parse_axioms("'f' does not depend on any axioms\n", "f"),
        Some(vec![])
    );
    assert_eq!(parse_axioms(ax, "g"), None);
}

#[test]
fn d9_the_cli_refuses_an_engine_commit_that_does_not_identify_this_binary() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("generate-cli-refusal");
    std::fs::create_dir_all(&dir).unwrap();
    let cfg = dir.join("config.json");
    std::fs::write(
        &cfg,
        config("nat_le_sq", "two_cases", "informal", json!([])).to_string(),
    )
    .unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (tag, commit) in [
        ("zeros", "0000000000000000000000000000000000000000"),
        ("malformed", "not-a-commit"),
        // PR 25's merge: an ancestor on main that holds no engine/generate/ at all, so the
        // inputs this binary compiled are not in it (FINDING-005 closed)
        (
            "ancestor-without-the-crate",
            "f9f973012cbe1a7d7f1d8cd4520db998dca0e550",
        ),
    ] {
        let receipt = dir.join(format!("receipt-{tag}"));
        let out = std::process::Command::new(env!("CARGO_BIN_EXE_syndeom-generate"))
            .args(["generate", "--config"])
            .arg(&cfg)
            .args(["--seed", "7", "--root"])
            .arg(&root)
            .args(["--engine-commit", commit, "--receipt-dir"])
            .arg(&receipt)
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(2), "{commit}");
        assert!(!receipt.exists(), "a refusal writes nothing");
    }
}
