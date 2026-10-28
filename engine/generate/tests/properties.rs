//! Property tests for `s2_template` (ADR-0011 D10). None of them runs Lean: where a formal file is
//! assembled, the harness observation is the one the D5 prediction implies, and the goldens test
//! pins the observations measured at implementation.

use serde_json::{Value, json};
use syndeom_generate::harness::{Observation, first_error_line, parse_axioms};
use syndeom_generate::plan::{render_formal, render_informal};
use syndeom_generate::template::{Prepared, assemble, faithful_plan, prepare, seeds};
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

fn build(c: &Value, seed: u64) -> (Prepared, syndeom_manifest::Manifest) {
    let p = prepare(c, seed).unwrap_or_else(|e| panic!("{c}: {e}"));
    let obs = predicted(&p);
    let tc = p.formal.as_ref().map(|_| toolchain());
    let m = assemble(&p, COMMIT, tc, obs.as_ref()).unwrap_or_else(|e| panic!("{c}: {e}"));
    (p, m)
}

/// Every (seed, style, material-or-control, target) the v0.1 seed set supports.
fn every_case() -> Vec<Value> {
    let mut out = Vec::new();
    for seed in seeds() {
        for style in &seed.styles {
            let mut specs = vec![json!([])];
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
                    assert_ne!(r.dials["inserted"], r.dials["replaced"], "{c}");
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
            assert!(
                r.dials["original_steps"].is_object() || r.dials["original_fragments"].is_object()
            );
        }
    }
}

#[test]
fn d8_counts_equal_the_records_and_predictions_pass() {
    for c in every_case() {
        let (_, m) = build(&c, 7);
        let flaws = &m.ground_truth.as_ref().unwrap().flaws;
        assert_eq!(m.commanded["n_flaws_total"].value, json!(flaws.len()));
        assert_eq!(m.commanded["flaws_confirmed"].value, json!(flaws.len()));
        assert_eq!(m.fidelity.verdict, Verdict::Pass, "{c}");
    }
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
fn d10_every_control_is_correct_and_unflawed() {
    for seed in seeds() {
        for style in &seed.styles {
            for target in ["informal", "formal", "pair"] {
                let (_, m) = build(&config(&seed.id, &style.id, target, json!([])), 7);
                let gt = m.ground_truth.unwrap();
                assert!(gt.flaws.is_empty());
                assert_eq!(gt.proof_status, ProofStatus::Valid);
                assert!(gt.correct);
            }
        }
    }
}

#[test]
fn d6_seed_records_are_sourced_outside_the_s1_corpus_and_well_formed() {
    let all = seeds();
    assert_eq!(all.len(), 3);
    for seed in &all {
        assert!(!seed.source.is_empty() && !seed.source.contains("groundtruth"));
        assert_eq!(seed.truth.value, Some(true));
        for style in &seed.styles {
            let plan = faithful_plan(seed, style);
            parse_informal_md(&render_informal(&plan).unwrap()).unwrap();
            parse_lean(&render_formal(&plan).unwrap().bytes).unwrap();
            for m in &style.flaws {
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
                    let orig = &style.steps[m.site as usize - 1].text;
                    assert_eq!(orig.matches(&a.replaced).count(), 1);
                }
            }
        }
    }
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
    ];
    for c in refusals {
        assert!(prepare(&c, 7).is_err(), "should refuse {c}");
    }
}

#[test]
fn d8_harness_parsers() {
    let out = "C:/x/Proof.lean:1:0: warning: deprecated\nTry this: ring_nf\nC:/x/Proof.lean:6:47: error: unsolved goals\nC:/x/Proof.lean:9:2: error: Type mismatch\n";
    assert_eq!(first_error_line(out), Some(6));
    assert_eq!(first_error_line("no errors here\n"), None);
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
fn d9_the_cli_refuses_an_engine_commit_that_is_not_an_ancestor() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("generate-cli-refusal");
    std::fs::create_dir_all(&dir).unwrap();
    let cfg = dir.join("config.json");
    std::fs::write(
        &cfg,
        config("nat_le_sq", "two_cases", "informal", json!([])).to_string(),
    )
    .unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for commit in ["0000000000000000000000000000000000000000", "not-a-commit"] {
        let receipt = dir.join(format!("receipt-{}", commit.len()));
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
