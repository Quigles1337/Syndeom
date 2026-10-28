//! Property tests for `s2_template` (ADR-0011 D10). None of them runs Lean: where a formal file is
//! assembled, the harness observation is the one the D5 prediction implies, and the goldens test
//! pins the observations measured at implementation.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use syndeom_generate::harness::{
    Observation, audit_bytes, first_error_line, parse_axioms, parse_used_constants, used_closure,
};
use syndeom_generate::plan::{own_declarations, render_formal, render_informal};
use syndeom_generate::template::{
    Prepared, Version, assemble, check_seed, faithful_plan, manifests_in, prepare, prepare_with,
    seeds,
};
use syndeom_manifest::{
    LeanCatchable, Location, ProofStatus, Toolchain, Verdict, parse_informal_md, parse_lean,
    span_text,
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

/// The observation D5 predicts: a K-class instance fails at the site's first fragment line; a
/// T-class instance builds, and the audit lists the smuggled axiom (FLAW-0009) or shows the
/// inserted lemma used by nothing (FLAW-0007 `unused`); a control builds with standard axioms.
fn predicted(p: &Prepared) -> Option<Observation> {
    let f = p.formal.as_ref()?;
    let standard = || vec!["propext".to_string(), "Quot.sound".to_string()];
    // every own declaration uses nothing of the file's own, except the main declaration, which
    // uses every lemma but an inserted one
    let used = |except: Option<&str>| {
        let own = own_declarations(f);
        let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for d in &own {
            map.insert(d.clone(), Vec::new());
        }
        let mut uses: Vec<String> = own
            .iter()
            .filter(|d| **d != p.plan.main_declaration && Some(d.as_str()) != except)
            .cloned()
            .collect();
        uses.sort();
        map.insert(p.plan.main_declaration.clone(), uses);
        Some(map)
    };
    Some(match &p.injection {
        Some(inj) if inj.lean_catchable == LeanCatchable::YesKernel => Observation {
            exit_code: 1,
            first_error_line: f.fragment_lines[inj.site - 1].map(|r| r[0]),
            axioms: None,
            used_constants: None,
        },
        Some(inj) => {
            let declaration = inj.formal_dials["declaration"].as_str().map(str::to_string);
            let mut axioms = standard();
            if inj.class == "FLAW-0009" {
                axioms.push(
                    declaration
                        .clone()
                        .expect("a FLAW-0009 record names its axiom"),
                );
            }
            Observation {
                exit_code: 0,
                first_error_line: None,
                axioms: Some(axioms),
                used_constants: used(if inj.class == "FLAW-0007" {
                    declaration.as_deref()
                } else {
                    None
                }),
            }
        }
        None => Observation {
            exit_code: 0,
            first_error_line: None,
            axioms: Some(standard()),
            used_constants: used(None),
        },
    })
}

fn faithful_plan_of(id: &str) -> syndeom_generate::plan::Plan {
    let seed = seeds(Version::CURRENT)
        .into_iter()
        .find(|s| s.id == id)
        .unwrap();
    faithful_plan(&seed, &seed.styles[0])
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
                    // D3: a class the target cannot realize is refused, not a case
                    let class = flaws[0]["class"].as_str().unwrap_or("");
                    let refused = matches!(
                        (class, target),
                        ("FLAW-0009", "informal")
                            | ("FLAW-0008", "informal" | "formal")
                            | ("FLAW-0004" | "FLAW-0010", "formal")
                    );
                    if !refused {
                        out.push(config(&seed.id, &style.id, target, flaws.clone()));
                    }
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
            assert!(m1.artifact_id.starts_with("s2_template-v0.3.0-"));
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
                    let f = p.formal.as_ref().unwrap();
                    let text = std::str::from_utf8(&f.bytes).unwrap();
                    let lines: Vec<&str> = text.lines().collect();
                    let inserted = r.dials["inserted"].as_array().unwrap();
                    if loc.declaration == p.plan.main_declaration {
                        // a main-proof step: its fragment lines, indented by the writer
                        assert_eq!(
                            format!("  {}", inserted[0].as_str().unwrap()),
                            lines[loc.line as usize - 1]
                        );
                        assert!(r.dials.get("declaration").is_none(), "{c}");
                    } else {
                        // a lemma step (0.3.0): its declaration lines as the writer emits them
                        assert_eq!(r.dials["declaration"], json!(loc.declaration), "{c}");
                        for (j, l) in inserted.iter().enumerate() {
                            assert_eq!(
                                l.as_str().unwrap(),
                                lines[loc.line as usize - 1 + j],
                                "{c}"
                            );
                        }
                        assert!(lines[loc.line as usize - 1].contains(&loc.declaration));
                    }
                }
                Location::Raw(_) => panic!("synthetic records are canonical"),
            }
            if r.dials["replaced"].is_null() && r.dials.get("counterexample").is_some() {
                // intrinsic: a false statement's argument replaces nothing and names its
                // counterexample
                assert!(r.dials["counterexample"].is_string(), "{c}");
                assert!(r.dials.get("original_steps").is_none(), "{c}");
            } else if r.dials["replaced"].is_null() {
                // an inserted step (0.3.0): nothing replaced, the originals recorded
                assert_eq!(r.class, "FLAW-0007", "{c}");
                assert!(
                    r.dials["original_steps"].is_object()
                        || r.dials["original_fragments"].is_object()
                );
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
        // a record per file the class manifests in (FLAWS.md's column: FLAW-0009 is formal-only)
        let files_with_records = flaws.first().map_or(0, |f| {
            let (informal, formal) = manifests_in(&f.class, f.sub_variant.as_deref());
            let target = c["target"].as_str().unwrap();
            usize::from(informal && target != "formal")
                + usize::from(formal && target != "informal")
        });
        assert_eq!(flaws.len(), n * files_with_records, "{c}");
        if c["target"] == "pair" && files_with_records == 2 {
            assert_eq!(
                flaws.len(),
                2 * n,
                "{c}: a pair records each instance twice"
            );
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
        used_constants: None,
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
        used_constants: None,
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
        used_constants: None,
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
    assert_eq!(seeds(Version::V0_2_0).len(), 7);
    let all = seeds(Version::V0_3_0);
    assert_eq!(all.len(), 8);
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
            "n_lt_two_pow",
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
                let site_text = style.steps.get(m.site as usize - 1);
                if m.intrinsic {
                    assert!(m.steps.is_empty());
                    let site_text = site_text.expect("the site is a step");
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
                let froms: Vec<u64> = m.steps.iter().filter_map(|s| s.from).collect();
                assert!(
                    froms.windows(2).all(|w| w[0] < w[1]),
                    "from indices increase"
                );
                assert!(
                    froms
                        .iter()
                        .all(|k| *k >= 1 && *k as usize <= style.steps.len())
                );
                let site = if m.inserts {
                    // an inserted step (0.3.0) stands in for nothing and is a proved lemma
                    let s = &m.steps[m.site as usize - 1];
                    assert!(s.from.is_none() && s.lemma.is_some(), "{}", seed.id);
                    s
                } else {
                    m.steps
                        .iter()
                        .find(|s| s.from == Some(m.site))
                        .expect("site kept")
                };
                assert!(
                    !site.fragment.is_empty() || site.lemma.is_some(),
                    "the site carries a formal fragment or is a lemma declaration"
                );
                if let Some(a) = &m.anchor {
                    assert_eq!(site.text.matches(&a.inserted).count(), 1);
                    let replaced = a.replaced.as_ref().expect("material replaces text");
                    let site_text = site_text.expect("the site is a step");
                    assert_eq!(site_text.text.matches(replaced).count(), 1);
                }
            }
        }
    }
}

#[test]
fn d1_the_earlier_seed_sets_are_closed() {
    // A 0.2.0 seed is not a seed of 0.1.0, and the 0.3.0 seed is a seed of neither: the earlier
    // versions refused them, and still do.
    let style_of = |id: &str| {
        seeds(Version::CURRENT)
            .into_iter()
            .find(|s| s.id == id)
            .unwrap()
            .styles[0]
            .id
            .clone()
    };
    for id in [
        "continuity_is_not_uniform",
        "x_over_abs_x",
        "rolle_abs",
        "horses",
        "n_lt_two_pow",
    ] {
        let style = style_of(id);
        assert!(
            prepare_with(
                Version::V0_1_0,
                &config(id, &style, "informal", json!([])),
                7
            )
            .is_err()
        );
    }
    let style = style_of("n_lt_two_pow");
    assert!(
        prepare_with(
            Version::V0_2_0,
            &config("n_lt_two_pow", &style, "informal", json!([])),
            7
        )
        .is_err()
    );
    assert!(Version::parse("0.4.0").is_none());
    assert_eq!(Version::CURRENT.as_str(), "0.3.0");
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
        // FLAW-0009 is formal-only, and the n_lt_two_pow materials are what the seed names
        config(
            "n_lt_two_pow",
            "helper_lemma",
            "informal",
            json!([spec(
                "FLAW-0009",
                None,
                "gap",
                json!({"mode": "axiom", "truth": true})
            )]),
        ),
        config(
            "n_lt_two_pow",
            "helper_lemma",
            "formal",
            json!([spec(
                "FLAW-0009",
                None,
                "gap",
                json!({"mode": "native_decide", "truth": true})
            )]),
        ),
        config(
            "n_lt_two_pow",
            "helper_lemma",
            "pair",
            json!([spec(
                "FLAW-0007",
                Some("unused"),
                "cosmetic",
                json!({"count": 2, "length_steps": 1})
            )]),
        ),
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
                json!({"mode": "step-above-base", "failing_case": "n + 1 = 2"})
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
    // the used-constants listing (0.3.0), sorted and deduplicated per constant
    let listing = "used-constants b._proof_1: [Nat, a, Nat]\nused-constants main: [b, Nat, a]\nused-constants a: [Nat]\nused-constants b: [b._proof_1, Nat]\nnoise\n";
    let used = parse_used_constants(listing).unwrap();
    assert_eq!(used["main"], vec!["Nat", "a", "b"]);
    assert_eq!(used["b._proof_1"], vec!["Nat", "a"]);
    assert_eq!(parse_used_constants("nothing\n"), None);
    // the closure over the file's own constants: main → b → b._proof_1 → a, never into Mathlib
    let closure = used_closure(&used, "main");
    assert!(
        ["main", "a", "b", "b._proof_1"]
            .iter()
            .all(|d| closure.contains(*d))
    );
    assert!(!closure.contains("Nat"));
    let mut without_b = used.clone();
    without_b.insert("main".into(), vec!["Nat".into(), "a".into()]);
    let closure = used_closure(&without_b, "main");
    assert!(closure.contains("a") && !closure.contains("b"));
    // the audit tail prints the main declaration's axioms and every own constant's uses
    let tail = String::from_utf8(audit_bytes(b"import Mathlib\n", "main")).unwrap();
    assert!(tail.starts_with("import Mathlib\n\n#print axioms main\n"));
    assert!(tail.contains("env.constants.map₂.toList") && tail.contains("getUsedConstantsAsSet"));
}

#[test]
fn d2_lemma_steps_render_as_declarations_before_the_main_declaration() {
    let plan = faithful_plan_of("n_lt_two_pow");
    let r = render_formal(&plan).unwrap();
    let text = String::from_utf8(r.bytes.clone()).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "import Mathlib.Data.Nat.Notation");
    assert_eq!(lines[1], "");
    assert_eq!(
        lines[2],
        "theorem syndeom_s2_template_n_lt_two_pow_lemma_1 : ∀ n : ℕ, n < 2 ^ n := by"
    );
    assert_eq!(lines[3], "  intro n");
    assert_eq!(lines[7], "");
    assert_eq!(
        lines[8],
        "theorem syndeom_s2_template_n_lt_two_pow (n : ℕ) : n + 1 ≤ 2 ^ n := by"
    );
    assert_eq!(
        lines[9],
        "  exact syndeom_s2_template_n_lt_two_pow_lemma_1 n"
    );
    assert_eq!(lines.len(), 10);
    assert_eq!(r.fragment_lines, vec![Some([3, 7]), Some([10, 10])]);
    assert_eq!(
        r.declarations,
        vec![
            "syndeom_s2_template_n_lt_two_pow_lemma_1".to_string(),
            "syndeom_s2_template_n_lt_two_pow".to_string()
        ]
    );
    assert_eq!(own_declarations(&r), r.declarations);
    let st = parse_lean(&r.bytes).unwrap();
    assert_eq!(st.n_declarations, 2);
    let informal = parse_informal_md(&render_informal(&plan).unwrap()).unwrap();
    assert_eq!((informal.n_steps, informal.n_lemmas), (2, 1));
    // the shape rules: numbering, headers, proof lines
    let mut bad = plan.clone();
    bad.steps[0].text = bad.steps[0].text.replacen("Lemma 1.", "Lemma 2.", 1);
    assert!(render_formal(&bad).is_err());
    let mut bad = plan.clone();
    bad.steps[1].text = format!("Lemma 2. {}", bad.steps[1].text);
    assert!(render_informal(&bad).is_err());
    let mut bad = plan.clone();
    bad.steps[0].fragment.clear();
    assert!(render_formal(&bad).is_err());
}

#[test]
fn d5_check_t_confirms_the_tooling_caught_classes_only_as_predicted() {
    // FLAW-0007 `unused`: confirmed iff the inserted lemma is listed and outside the main
    // declaration's closure; FLAW-0009: confirmed iff `#print axioms` lists the axiom.
    let unused = config(
        "n_lt_two_pow",
        "helper_lemma",
        "pair",
        json!([spec(
            "FLAW-0007",
            Some("unused"),
            "cosmetic",
            json!({"count": 1, "length_steps": 1})
        )]),
    );
    let (p, m) = build(&unused, 7);
    let gt = m.ground_truth.as_ref().unwrap();
    assert_eq!(gt.flaws.len(), 2);
    assert_eq!(gt.proof_status, ProofStatus::Valid);
    assert!(gt.correct);
    assert_eq!(m.commanded["n_lemmas"].value, json!(2));
    assert_eq!(m.commanded["n_steps"].value, json!(3));
    assert_eq!(m.commanded["lean_build"].value, json!("pass"));
    assert_eq!(m.commanded["lean_axioms_standard"].value, json!(true));
    assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(1)));
    assert_eq!(m.fidelity.verdict, Verdict::Pass);
    let lemma_2 = "syndeom_s2_template_n_lt_two_pow_lemma_2";
    let formal = gt
        .flaws
        .iter()
        .find(|f| matches!(f.location, Location::Formal(_)))
        .unwrap();
    assert_eq!(formal.dials["declaration"], json!(lemma_2));
    match &formal.location {
        Location::Formal(loc) => assert_eq!(loc.declaration, lemma_2),
        _ => unreachable!(),
    }
    // the main declaration uses the lemma after all: not confirmed
    let mut obs = predicted(&p).unwrap();
    let used = obs.used_constants.as_mut().unwrap();
    used.get_mut(&p.plan.main_declaration)
        .unwrap()
        .push(lemma_2.into());
    let m = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
    assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(0)));
    assert_eq!(m.fidelity.verdict, Verdict::Fail);
    // used through an auxiliary constant of the file: not confirmed either
    let mut obs = predicted(&p).unwrap();
    let used = obs.used_constants.as_mut().unwrap();
    used.insert("aux._proof_1".into(), vec![lemma_2.into()]);
    used.get_mut(&p.plan.main_declaration)
        .unwrap()
        .push("aux._proof_1".into());
    let m = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
    assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(0)));
    // no listing at all: not confirmed
    let mut obs = predicted(&p).unwrap();
    obs.used_constants = None;
    let m = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
    assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(0)));

    for (severity, truth) in [("gap", true), ("fatal", false)] {
        let smuggled = config(
            "n_lt_two_pow",
            "helper_lemma",
            "pair",
            json!([spec(
                "FLAW-0009",
                None,
                severity,
                json!({"mode": "axiom", "truth": truth})
            )]),
        );
        let (p, m) = build(&smuggled, 7);
        let gt = m.ground_truth.as_ref().unwrap();
        // formal-only: one record, one instance, the informal file unchanged
        assert_eq!(gt.flaws.len(), 1);
        assert!(matches!(gt.flaws[0].location, Location::Formal(_)));
        assert_eq!(gt.proof_status, ProofStatus::Invalid);
        assert_eq!(m.commanded["n_flaws_total"].value, json!(1));
        assert_eq!(m.commanded["lean_build"].value, json!("pass"));
        assert_eq!(m.commanded["lean_axioms_standard"].value, json!(false));
        assert_eq!(m.achieved["lean_axioms_standard"].value, Some(json!(false)));
        assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(1)));
        assert_eq!(m.fidelity.verdict, Verdict::Pass);
        let lemma_1 = "syndeom_s2_template_n_lt_two_pow_lemma_1";
        assert_eq!(gt.flaws[0].dials["declaration"], json!(lemma_1));
        assert_eq!(gt.flaws[0].dials["truth"], json!(truth));
        let text = std::str::from_utf8(&p.formal.as_ref().unwrap().bytes).unwrap();
        assert!(text.contains(&format!("\naxiom {lemma_1} : ")));
        assert_eq!(
            p.informal.as_ref().unwrap(),
            &render_informal(&faithful_plan_of("n_lt_two_pow")).unwrap()
        );
        // the axiom print without the axiom: not confirmed, and the axioms statistic fails too
        let mut obs = predicted(&p).unwrap();
        obs.axioms = Some(vec!["propext".into()]);
        let m = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
        assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(0)));
        assert_eq!(
            m.fidelity.per_statistic["lean_axioms_standard"],
            Verdict::Fail
        );
        assert_eq!(m.fidelity.verdict, Verdict::Fail);
    }
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

#[test]
fn d9_verify_refuses_a_manifest_whose_main_declaration_is_not_the_producers() {
    // FINDING-011 (analysis/s2-generate-3): the audit tail interpolates the main declaration
    // into Lean source that `verify` then compiles, so nothing is compiled unless the manifest
    // names the producer's own declaration.
    let golden = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/goldens/continuity_is_not_uniform/control-formal");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (tag, name) in [
        ("other", "syndeom_s2_template_other"),
        ("injected", "x\n#eval (1 : Nat)"),
    ] {
        let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join(format!("verify-main-declaration-{tag}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::copy(golden.join("Proof.lean"), dir.join("Proof.lean")).unwrap();
        let mut manifest: Value =
            serde_json::from_slice(&std::fs::read(golden.join("manifest.json")).unwrap()).unwrap();
        manifest["artifact"]["files"]["formal"]["main_declaration"] = json!(name);
        std::fs::write(
            dir.join("manifest.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        let receipt = dir.join("receipt");
        let out = std::process::Command::new(env!("CARGO_BIN_EXE_syndeom-generate"))
            .args(["verify", "--artifact"])
            .arg(&dir)
            .arg("--root")
            .arg(&root)
            .arg("--receipt-dir")
            .arg(&receipt)
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(2),
            "{tag}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            !receipt.exists(),
            "{tag}: a refusal compiles nothing and writes no receipt"
        );
    }
}
