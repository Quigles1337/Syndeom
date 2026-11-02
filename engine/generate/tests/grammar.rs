//! `s2_grammar` (ADR-0011 D1, D6): the grammar `ring_chain`, its control and FLAW-0001 —
//! the atom library, the presentations, the derivation, the plan and its two renderings, the
//! reference of a grammar instance's truth (D7), the commanded and achieved statistics (D8),
//! determinism (D9), FLAW-0001 `propagated` and `local` with their records and check K (D5),
//! and the refusals (D3, D6) — and, from 0.2.0, FLAW-0007 `unused` (irrelevant lemmas generated
//! from the atom library, cited by nowhere, each its own instance, check T) and FLAW-0013
//! (generated padding paragraphs with their unused declarations, `padding_ratio` measured) —
//! and, from 0.3.0, FLAW-0009 `axiom` (a cited lemma's declaration replaced by an axiom of the
//! same identity, or of a false one used ex falso at the citing step; check T on the axiom print).

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};
use syndeom_generate::grammar::{
    self, GrammarVersion, MAIN_DECLARATION, PERTURBATIONS, Perturbation, SEED_FORMS, Shape, Style,
    Term, apply, control_reference, derive, eval, mono_latex, mono_lean, normalize, render_ordered,
    targets, witness_point,
};
use syndeom_generate::harness::Observation;
use syndeom_generate::plan::{
    LemmaKind, PaddingKind, lemma_lines, own_declarations, padding_lines,
};
use syndeom_generate::rng::Stream;
use syndeom_generate::template::{Injection, Prepared, Producer, assemble};
use syndeom_generate::{prepare_any, prepare_for};
use syndeom_manifest::{
    LeanCatchable, Location, Manifest, ProofStatus, Severity, StatementMethod, Toolchain,
    parse_informal_md, parse_lean,
};

const COMMIT: &str = "0123456789abcdef0123456789abcdef01234567";

fn toolchain() -> Toolchain {
    Toolchain {
        lean: "leanprover/lean4:v4.34.1".into(),
        mathlib: "d13f23b723b8a846827a245b89c10fc7d3f11612".into(),
    }
}

fn config(n: u64, m: u64, target: &str, flaws: Value) -> Value {
    json!({
        "grammar": "ring_chain",
        "length": {"n_steps": n, "n_lemmas": m},
        "target": target,
        "flaws": flaws,
        "padding_ratio": 0,
    })
}

fn flaw(sub: &str, severity: &str, perturbation: &str, site: Value) -> Value {
    json!([{
        "class": "FLAW-0001", "sub_variant": sub, "severity": severity, "site": site,
        "dials": {"perturbation": perturbation}
    }])
}

fn propagated(perturbation: &str) -> Value {
    flaw("propagated", "fatal", perturbation, json!("auto"))
}

fn local(perturbation: &str) -> Value {
    flaw("local", "repairable", perturbation, json!("auto"))
}

/// A FLAW-0007 `unused` spec (0.2.0): `count` irrelevant lemmas, one identity each.
fn unused_spec(count: u64) -> Value {
    json!([{
        "class": "FLAW-0007", "sub_variant": "unused", "severity": "cosmetic", "site": "auto",
        "dials": {"count": count, "length_steps": 1}
    }])
}

/// A FLAW-0013 spec (0.2.0): the eligible kinds, how many paragraphs, and where they go.
fn padding_spec(kinds: Value, count: u64, placement: &str) -> Value {
    json!([{
        "class": "FLAW-0013", "sub_variant": null, "severity": "cosmetic", "site": "auto",
        "dials": {"kinds": kinds, "count": count, "placement": placement}
    }])
}

/// A padded config whose commanded `padding_ratio` is the ratio the paragraphs achieve at seed
/// 7 (the ratio is compared only in `measure`, ADR-0011 D4: a config commands what its count
/// yields, as the goldens do), read from a probe at ratio 1 — a FAIL verdict, not a refusal.
fn padded(n: u64, m: u64, target: &str, kinds: Value, count: u64, placement: &str) -> Value {
    let flaws = padding_spec(kinds, count, placement);
    let mut probe = config(n, m, target, flaws.clone());
    probe["padding_ratio"] = json!(1);
    let (_, pm) = build(&probe, 7);
    assert_eq!(
        verdict(&pm),
        json!("FAIL"),
        "a ratio the count cannot reach is a FAIL verdict, not a refusal"
    );
    let ratio = pm.achieved["padding_ratio"]
        .value
        .as_ref()
        .and_then(Value::as_f64)
        .unwrap();
    assert!(ratio > 0.0 && ratio < 1.0, "{ratio}");
    let mut c = config(n, m, target, flaws);
    c["padding_ratio"] = json!(ratio);
    c
}

/// A FLAW-0009 `axiom` spec (0.3.0): the cited lemma's declaration replaced by an axiom of the
/// same identity (`truth: true`, gap) or of a false one used ex falso (`truth: false`, fatal).
fn axiom_spec(truth: bool, site: Value) -> Value {
    json!([{
        "class": "FLAW-0009", "sub_variant": null,
        "severity": if truth { "gap" } else { "fatal" }, "site": site,
        "dials": {"mode": "axiom", "truth": truth}
    }])
}

/// The observation D5 predicts: a FLAW-0001 instance fails at the site's `calc` line (check K);
/// everything else builds with standard axioms — plus a FLAW-0009 axiom's name, which the print
/// lists (0.3.0) — every own declaration using nothing of the file's own, and the main
/// declaration using every cited lemma — never an unused lemma (0.2.0, FLAW-0007) or a padding
/// declaration (0.2.0, FLAW-0013), which check T reads.
fn predicted(p: &Prepared) -> Option<Observation> {
    let f = p.formal.as_ref()?;
    Some(match &p.injection {
        Some(inj) if inj.lean_catchable == LeanCatchable::YesKernel => Observation {
            exit_code: 1,
            first_error_line: f.fragment_lines[inj.site - 1].map(|r| r[0]),
            axioms: None,
            used_constants: None,
        },
        _ => {
            let unused: Vec<&str> = p
                .injection
                .iter()
                .chain(&p.further)
                .filter_map(|i| i.formal_dials["declaration"].as_str())
                .collect();
            let pads: Vec<&str> = p.padding.iter().map(|i| i.declaration.as_str()).collect();
            let own = own_declarations(f);
            let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for d in &own {
                map.insert(d.clone(), Vec::new());
            }
            let mut uses: Vec<String> = own
                .iter()
                .filter(|d| {
                    **d != p.plan.main_declaration
                        && !unused.contains(&d.as_str())
                        && !pads.contains(&d.as_str())
                })
                .cloned()
                .collect();
            uses.sort();
            map.insert(p.plan.main_declaration.clone(), uses);
            let mut axioms = vec!["propext".to_string()];
            axioms.extend(
                p.injection
                    .iter()
                    .filter(|i| i.class == "FLAW-0009")
                    .filter_map(|i| i.formal_dials["declaration"].as_str().map(str::to_string)),
            );
            axioms.push("Quot.sound".to_string());
            Observation {
                exit_code: 0,
                first_error_line: None,
                axioms: Some(axioms),
                used_constants: Some(map),
            }
        }
    })
}

fn build(c: &Value, seed: u64) -> (Prepared, Manifest) {
    let p = grammar::prepare(c, seed).unwrap_or_else(|e| panic!("{c}: {e}"));
    let obs = predicted(&p);
    let tc = p.formal.as_ref().map(|_| toolchain());
    let m = assemble(&p, COMMIT, tc, obs.as_ref()).unwrap_or_else(|e| panic!("{c}: {e}"));
    (p, m)
}

fn verdict(m: &Manifest) -> Value {
    json!(m.fidelity.verdict)
}

fn uses_x(p: &[Term]) -> bool {
    p.iter().any(|t| t.i >= 1)
}

fn uses_y(p: &[Term]) -> bool {
    p.iter().any(|t| t.j >= 1)
}

#[test]
fn d6_the_seed_forms_use_both_variables_and_expand_to_at_least_two_monomials() {
    assert_eq!(SEED_FORMS.len(), 14);
    for (k, form) in SEED_FORMS.iter().enumerate() {
        let p = form.poly();
        assert!(p.len() >= 2, "form {k}: {:?}", p);
        assert!(uses_x(&p) && uses_y(&p), "form {k} uses x and y");
        assert_eq!(normalize(&p), p, "form {k} is canonical");
        assert!(p.iter().all(|t| t.c != 0 && t.c.abs() <= 25));
        assert!(!form.lean().is_empty() && !form.latex().is_empty());
    }
    assert_eq!(SEED_FORMS[0].lean(), "(x + y) ^ 2");
    assert_eq!(SEED_FORMS[0].latex(), "(x + y)^2");
    assert_eq!(SEED_FORMS[2].lean(), "(x + y) * (x - y)");
    assert_eq!(SEED_FORMS[2].latex(), "(x + y)(x - y)");
    assert_eq!(SEED_FORMS[3].lean(), "(2 * x + y) ^ 2");
    assert_eq!(SEED_FORMS[3].latex(), "(2x + y)^2");
    assert_eq!(SEED_FORMS[10].lean(), "(x * y + 1) ^ 2");
    assert_eq!(SEED_FORMS[10].latex(), "(xy + 1)^2");
    assert_eq!(SEED_FORMS[11].lean(), "(x + y + 1) ^ 2");
    assert_eq!(SEED_FORMS[13].lean(), "(x - 2) * (y + 5)");
    assert_eq!(SEED_FORMS[13].latex(), "(x - 2)(y + 5)");
    // (x + y)^2 = x^2 + 2xy + y^2 in canonical order
    assert_eq!(
        SEED_FORMS[0].poly(),
        vec![Term::new(1, 2, 0), Term::new(2, 1, 1), Term::new(1, 0, 2)]
    );
    // (x + y)(x - y) = x^2 - y^2
    assert_eq!(
        SEED_FORMS[2].poly(),
        vec![Term::new(1, 2, 0), Term::new(-1, 0, 2)]
    );
    assert!(grammar::SOURCE.contains("outside the S1 corpus"));
}

#[test]
fn d10_monomials_and_shapes_render_as_the_probes_were_written() {
    let d = Style::default();
    assert_eq!(mono_lean(Term::new(2, 1, 1), d), "2 * x * y");
    assert_eq!(mono_latex(Term::new(2, 1, 1), d), "2xy");
    assert_eq!(mono_lean(Term::new(1, 2, 0), d), "x ^ 2");
    assert_eq!(mono_latex(Term::new(1, 2, 0), d), "x^2");
    let split = Style { split_x: true, ..d };
    assert_eq!(mono_lean(Term::new(1, 2, 0), split), "x * x");
    assert_eq!(mono_latex(Term::new(1, 2, 0), split), "x \\cdot x");
    assert_eq!(mono_lean(Term::new(2, 2, 1), split), "2 * x * x * y");
    assert_eq!(
        mono_latex(Term::new(2, 2, 1), split),
        "2 \\cdot x \\cdot x \\cdot y"
    );
    let y_first = Style { y_first: true, ..d };
    assert_eq!(mono_lean(Term::new(2, 1, 1), y_first), "2 * y * x");
    assert_eq!(mono_latex(Term::new(2, 1, 1), y_first), "2yx");
    assert_eq!(mono_lean(Term::new(3, 0, 0), d), "3");
    assert_eq!(mono_lean(Term::new(-3, 0, 0), d), "3");
    assert_eq!(mono_lean(Term::new(1, 0, 3), d), "y ^ 3");
    assert_eq!(mono_latex(Term::new(1, 0, 3), d), "y^3");

    let sq = [Term::new(1, 2, 0), Term::new(2, 1, 1), Term::new(1, 0, 2)];
    let styles = vec![d; 3];
    let e = render_ordered(&Shape::Expanded, &styles, &sq);
    assert_eq!(e.lean, "x ^ 2 + 2 * x * y + y ^ 2");
    assert_eq!(e.latex, "x^2 + 2xy + y^2");
    let f = render_ordered(
        &Shape::Factored {
            var: 0,
            first: true,
        },
        &styles,
        &sq,
    );
    assert_eq!(f.lean, "x * (x + 2 * y) + y ^ 2");
    assert_eq!(f.latex, "x(x + 2y) + y^2");
    assert_eq!(
        f.shape,
        Shape::Factored {
            var: 0,
            first: true
        }
    );
    let f = render_ordered(
        &Shape::Factored {
            var: 1,
            first: false,
        },
        &styles,
        &sq,
    );
    assert_eq!(f.lean, "x ^ 2 + y * (2 * x + y)");
    assert_eq!(f.latex, "x^2 + y(2x + y)");
    let g = render_ordered(&Shape::Grouped { split: 2 }, &styles, &sq);
    assert_eq!(g.lean, "(x ^ 2 + 2 * x * y) + (y ^ 2)");
    assert_eq!(g.latex, "(x^2 + 2xy) + (y^2)");
    // the leading sign and the subtraction of negative terms
    let diff = [Term::new(-2, 0, 2), Term::new(1, 1, 1), Term::new(1, 2, 0)];
    let e = render_ordered(&Shape::Expanded, &styles, &diff);
    assert_eq!(e.lean, "-2 * y ^ 2 + x * y + x ^ 2");
    assert_eq!(e.latex, "-2y^2 + xy + x^2");
    // a factored variable dividing one term only, or a grouping of two terms, resolves to
    // expanded
    let two = [Term::new(1, 2, 0), Term::new(-1, 0, 2)];
    let f = render_ordered(
        &Shape::Factored {
            var: 0,
            first: true,
        },
        &styles,
        &two,
    );
    assert_eq!(f.shape, Shape::Expanded);
    assert_eq!(f.lean, "x ^ 2 - y ^ 2");
    let g = render_ordered(&Shape::Grouped { split: 1 }, &styles, &two);
    assert_eq!(g.shape, Shape::Expanded);
    // a factored variable with nothing left over
    let both = [Term::new(1, 2, 0), Term::new(1, 1, 1)];
    let f = render_ordered(
        &Shape::Factored {
            var: 0,
            first: false,
        },
        &styles,
        &both,
    );
    assert_eq!(f.lean, "x * (x + y)");
    // a style for a missing position is the default
    let e = render_ordered(&Shape::Expanded, &[], &two);
    assert_eq!(e.lean, "x ^ 2 - y ^ 2");
}

#[test]
fn d6_every_step_presents_the_polynomial_and_differs_from_the_step_before() {
    for seed in [0u64, 1, 7, 11, 42, 1_000_003, u64::MAX] {
        for (n, m) in [
            (1usize, 0usize),
            (4, 0),
            (6, 2),
            (24, 2),
            (33, 32),
            (256, 32),
        ] {
            let d = derive(n, m, seed);
            assert_eq!(d.n(), n);
            assert_eq!(d.e.len(), n + 1);
            assert_eq!(d.presentations.len(), n);
            assert_eq!(d.poly, SEED_FORMS[d.form].poly());
            assert_eq!(d.e[0].lean, SEED_FORMS[d.form].lean());
            for k in 1..=n {
                assert_eq!(normalize(&d.e[k].ordered), d.poly, "seed {seed} step {k}");
                assert_ne!(d.e[k].lean, d.e[k - 1].lean, "seed {seed} step {k}");
                assert_ne!(d.e[k].latex, d.e[k - 1].latex, "seed {seed} step {k}");
                assert!(!d.e[k].lean.contains("  "));
                assert!(!d.e[k].latex.contains("  "));
            }
            assert_eq!(d.sites.len(), m);
            assert!(d.sites.windows(2).all(|w| w[0] < w[1]));
            assert!(d.sites.iter().all(|&c| (1..=n).contains(&c)));
        }
    }
    // the seed form is drawn from grammar/plan: the 14 forms all occur over a few seeds
    let forms: std::collections::BTreeSet<usize> =
        (0u64..200).map(|s| derive(1, 0, s).form).collect();
    assert_eq!(forms.len(), SEED_FORMS.len());
}

#[test]
fn d2_the_plan_renders_the_lemmas_then_a_calc_chain_closed_by_ring_or_a_cited_lemma() {
    let c = config(6, 2, "pair", json!([]));
    let (p, m) = build(&c, 7);
    assert_eq!(p.producer, Producer::Grammar(GrammarVersion::V0_3_0));
    assert_eq!(m.generator.id, "s2_grammar");
    assert_eq!(m.generator.version, "0.3.0");
    assert_eq!(m.generator.rng, "syndeom-rng/1");
    assert!(m.artifact_id.starts_with("s2_grammar-v0.3.0-"));
    assert!(m.artifact_id.ends_with("-s7"));
    let plan = &p.plan;
    assert_eq!(plan.steps.len(), 8);
    assert_eq!(plan.main_declaration, MAIN_DECLARATION);
    assert_eq!(plan.binders, "(x y : ℤ)");
    assert_eq!(plan.imports, vec!["Mathlib.Tactic.Ring".to_string()]);
    assert!(
        plan.statement
            .starts_with("For all integers $x$ and $y$, $")
    );
    let d = derive(6, 2, 7);
    assert_eq!(plan.claim, format!("{} = {}", d.e[0].lean, d.e[6].lean));
    for j in 1..=2 {
        let s = &plan.steps[j - 1];
        let l = s.lemma.as_ref().expect("a lemma step");
        assert_eq!(l.name, format!("{MAIN_DECLARATION}_lemma_{j}"));
        assert!(
            s.text
                .starts_with(&format!("Lemma {j}. For all integers $x$ and $y$, $"))
        );
        assert!(l.claim.starts_with("∀ x y : ℤ, "));
        assert_eq!(
            s.fragment,
            vec!["intro x y".to_string(), "ring".to_string()]
        );
        let c = d.sites[j - 1];
        assert_eq!(
            l.claim,
            format!("∀ x y : ℤ, {} = {}", d.e[c - 1].lean, d.e[c].lean)
        );
    }
    let mut cited = 0;
    for c in 1..=6 {
        let s = &plan.steps[2 + c - 1];
        assert!(s.lemma.is_none() && s.padding.is_none());
        assert_eq!(s.fragment.len(), 1);
        let line = &s.fragment[0];
        if c == 1 {
            assert!(line.starts_with(&format!("calc {} = {} := ", d.e[0].lean, d.e[1].lean)));
        } else {
            assert!(line.starts_with(&format!("  _ = {} := ", d.e[c].lean)));
        }
        assert!(
            s.text
                .ends_with(&format!("${} = {}$.", d.e[c - 1].latex, d.e[c].latex))
        );
        match d.lemma_of(c) {
            Some(j) => {
                cited += 1;
                assert!(line.ends_with(&format!(":= {MAIN_DECLARATION}_lemma_{j} x y")));
                assert!(s.text.starts_with(&format!("By Lemma {j}, $")));
                assert_eq!(s.premises.len(), 1);
                assert_eq!(s.premises[0].cites, format!("{MAIN_DECLARATION}_lemma_{j}"));
                assert_eq!(s.premises[0].phrase, format!("Lemma {j}"));
                assert_eq!(s.text.matches(&s.premises[0].phrase).count(), 1);
            }
            None => {
                assert!(line.ends_with(":= by ring"));
                assert!(s.premises.is_empty());
                assert!(!s.text.starts_with("By Lemma"));
            }
        }
    }
    assert_eq!(cited, 2);
    // both files parse with the counts the manifest records
    let informal = parse_informal_md(p.informal.as_ref().unwrap()).unwrap();
    assert_eq!(informal.n_steps, 8);
    assert_eq!(informal.n_lemmas, 2);
    let formal = parse_lean(&p.formal.as_ref().unwrap().bytes).unwrap();
    assert_eq!(formal.n_declarations, 3);
    assert_eq!(formal.imports, vec!["Mathlib.Tactic.Ring".to_string()]);
    let text = String::from_utf8(p.formal.as_ref().unwrap().bytes.clone()).unwrap();
    assert!(text.contains(&format!("theorem {MAIN_DECLARATION}_lemma_1 : ∀ x y : ℤ, ")));
    assert!(text.contains(&format!("\ntheorem {MAIN_DECLARATION} (x y : ℤ) : ")));
    assert!(text.contains("\n  calc "));
    assert!(text.contains("\n    _ = "));
    assert_eq!(m.commanded["n_steps"].value, json!(8));
    assert_eq!(m.commanded["n_lemmas"].value, json!(2));
    assert_eq!(m.achieved["n_steps"].value, Some(json!(8)));
    assert_eq!(m.achieved["n_lemmas"].value, Some(json!(2)));
    assert_eq!(verdict(&m), json!("PASS"));
}

#[test]
fn d7_a_control_is_correct_and_unflawed_and_its_truth_names_the_formal_control() {
    for (n, m, target) in [
        (1, 0, "informal"),
        (4, 0, "formal"),
        (6, 2, "pair"),
        (24, 2, "formal"),
    ] {
        let c = config(n, m, target, json!([]));
        let (p, manifest) = build(&c, 7);
        let gt = manifest.ground_truth.as_ref().unwrap();
        assert!(gt.flaws.is_empty());
        assert_eq!(gt.proof_status, ProofStatus::Valid);
        assert!(gt.correct);
        assert_eq!(gt.statement_status.value, Some(true));
        assert_eq!(gt.statement_status.method, StatementMethod::LeanKernel);
        let reference = control_reference(GrammarVersion::CURRENT, "ring_chain", n, m, 7).unwrap();
        assert_eq!(gt.statement_status.reference, reference);
        // the reference is the formal control's own artifact id and the main declaration
        let (fc, fm) = build(&config(n, m, "formal", json!([])), 7);
        assert_eq!(
            reference,
            format!("{}:{MAIN_DECLARATION}", fc.artifact_id())
        );
        assert_eq!(fm.artifact_id, fc.artifact_id());
        if target == "formal" {
            assert_eq!(
                manifest.artifact_id,
                fc.artifact_id(),
                "the formal control names itself"
            );
            assert_eq!(manifest.commanded["lean_build"].value, json!("pass"));
            assert_eq!(
                manifest.commanded["lean_axioms_standard"].value,
                json!(true)
            );
            assert_eq!(manifest.achieved["lean_build"].value, Some(json!("pass")));
        }
        assert_eq!(manifest.commanded["n_flaws_total"].value, json!(0));
        assert_eq!(manifest.commanded["flaws_confirmed"].value, json!(0));
        assert_eq!(manifest.commanded["padding_ratio"].value, json!(0.0));
        assert_eq!(manifest.achieved["padding_ratio"].value, Some(json!(0.0)));
        assert_eq!(verdict(&manifest), json!("PASS"));
        assert_eq!(p.injection, None);
        assert!(p.padding.is_empty() && p.composite.is_none() && p.false_theorem.is_none());
    }
    // the reference does not depend on the number form of the control's config
    let a = control_reference(GrammarVersion::CURRENT, "ring_chain", 4, 0, 7).unwrap();
    // the 0.1.0 reference names the 0.1.0 control, which the 0.1.0 goldens hold
    assert_ne!(
        a,
        control_reference(GrammarVersion::V0_1_0, "ring_chain", 4, 0, 7).unwrap()
    );
    let (pa, _) = build(
        &json!({"grammar": "ring_chain", "length": {"n_steps": 4, "n_lemmas": 0}, "target": "formal", "flaws": [], "padding_ratio": 0.0}),
        7,
    );
    assert_eq!(a, format!("{}:{MAIN_DECLARATION}", pa.artifact_id()));
}

#[test]
fn d9_determinism_and_idempotence() {
    let c = config(6, 1, "pair", propagated("sign"));
    let (p1, m1) = build(&c, 7);
    let (p2, m2) = build(&c, 7);
    assert_eq!(p1, p2);
    assert_eq!(m1, m2);
    let (_, m3) = build(&c, 8);
    assert_ne!(m1.artifact_id, m3.artifact_id);
    assert_eq!(m1.config_hash, m3.config_hash);
    let (_, m4) = build(&config(6, 1, "pair", local("sign")), 7);
    assert_ne!(m1.artifact_id, m4.artifact_id);
    // regenerate's path: the producer the manifest names rebuilds the same bytes
    let producer = Producer::parse(&m1.generator.id, &m1.generator.version).unwrap();
    let again = prepare_for(producer, &m1.config, 7).unwrap();
    assert_eq!(again.informal, p1.informal);
    assert_eq!(
        again.formal.as_ref().map(|f| &f.bytes),
        p1.formal.as_ref().map(|f| &f.bytes)
    );
    // generate's dispatch: a grammar config is an s2_grammar config
    let any = prepare_any(&c, 7).unwrap();
    assert_eq!(any.producer, Producer::Grammar(GrammarVersion::V0_3_0));
    assert!(prepare_any(&json!({"theorem": "nat_le_sq", "grammar": "ring_chain"}), 7).is_err());
    assert!(prepare_any(&json!({"length": {"n_steps": 1, "n_lemmas": 0}}), 7).is_err());
    assert_eq!(
        Producer::parse("s2_grammar", "0.1.0"),
        Some(Producer::Grammar(GrammarVersion::V0_1_0))
    );
    assert_eq!(
        Producer::parse("s2_grammar", "0.2.0"),
        Some(Producer::Grammar(GrammarVersion::V0_2_0))
    );
    assert_eq!(
        Producer::parse("s2_grammar", "0.3.0"),
        Some(Producer::Grammar(GrammarVersion::V0_3_0))
    );
    assert_eq!(Producer::parse("s2_grammar", "0.4.0"), None);
    assert_eq!(Producer::parse("s2_grammars", "0.1.0"), None);
    assert_eq!(GrammarVersion::CURRENT.as_str(), "0.3.0");
    assert!(grammar::realizes(GrammarVersion::V0_1_0, "FLAW-0001"));
    assert!(!grammar::realizes(GrammarVersion::V0_1_0, "FLAW-0007"));
    assert!(!grammar::realizes(GrammarVersion::V0_1_0, "FLAW-0013"));
    assert!(grammar::realizes(GrammarVersion::V0_2_0, "FLAW-0001"));
    assert!(grammar::realizes(GrammarVersion::V0_2_0, "FLAW-0007"));
    assert!(grammar::realizes(GrammarVersion::V0_2_0, "FLAW-0013"));
    assert!(!grammar::realizes(GrammarVersion::V0_2_0, "FLAW-0009"));
    assert!(!grammar::realizes(GrammarVersion::V0_2_0, "FLAW-0004"));
    for class in ["FLAW-0001", "FLAW-0007", "FLAW-0009", "FLAW-0013"] {
        assert!(grammar::realizes(GrammarVersion::V0_3_0, class), "{class}");
    }
    assert!(!grammar::realizes(GrammarVersion::V0_3_0, "FLAW-0004"));
    // the 0.1.0 bytes of a control and of FLAW-0001 are unchanged by 0.3.0: only the version,
    // and with it the id and the manifest, differ
    let old = prepare_for(Producer::Grammar(GrammarVersion::V0_1_0), &c, 7).unwrap();
    assert_eq!(old.informal, p1.informal);
    assert_eq!(
        old.formal.as_ref().map(|f| &f.bytes),
        p1.formal.as_ref().map(|f| &f.bytes)
    );
    assert_ne!(old.artifact_id(), p1.artifact_id());
}

#[test]
fn d5_the_perturbation_targets_and_their_effect() {
    let sq = [Term::new(1, 2, 0), Term::new(2, 1, 1), Term::new(1, 0, 2)];
    assert_eq!(
        targets("sign", &sq),
        vec![
            Perturbation::Sign(0),
            Perturbation::Sign(1),
            Perturbation::Sign(2)
        ]
    );
    assert_eq!(
        targets("coefficient", &sq),
        vec![
            Perturbation::Coefficient(0),
            Perturbation::Coefficient(1),
            Perturbation::Coefficient(2)
        ]
    );
    assert_eq!(targets("constant", &sq), vec![Perturbation::ConstantAdd]);
    // raising an exponent is eligible where the variable is present and the raised monomial is
    // not another term's: x^2 in x only, 2xy in both, y^2 in y only
    assert_eq!(
        targets("exponent", &sq),
        vec![
            Perturbation::Exponent(0, 0),
            Perturbation::Exponent(1, 0),
            Perturbation::Exponent(1, 1),
            Perturbation::Exponent(2, 1),
        ]
    );
    let with_x = [Term::new(1, 2, 0), Term::new(1, 1, 0)];
    assert_eq!(
        targets("exponent", &with_x),
        vec![Perturbation::Exponent(0, 0)],
        "x raised is x^2, another term's"
    );
    let with_constant = [Term::new(1, 1, 1), Term::new(-2, 0, 0)];
    assert_eq!(
        targets("constant", &with_constant),
        vec![Perturbation::Constant(1)]
    );
    assert_eq!(
        targets("coefficient", &with_constant),
        vec![Perturbation::Coefficient(0)]
    );
    assert!(targets("nonsense", &sq).is_empty());

    let (t, r, i) = apply(Perturbation::Sign(1), &sq);
    assert_eq!(t[1], Term::new(-2, 1, 1));
    assert_eq!(
        (r.as_deref(), i.as_str()),
        (Some("2 * x * y"), "-2 * x * y")
    );
    let (t, r, i) = apply(Perturbation::Coefficient(0), &sq);
    assert_eq!(t[0], Term::new(2, 2, 0));
    assert_eq!((r.as_deref(), i.as_str()), (Some("x ^ 2"), "2 * x ^ 2"));
    let (t, r, i) = apply(Perturbation::Constant(1), &with_constant);
    assert_eq!(t[1], Term::new(-3, 0, 0));
    assert_eq!((r.as_deref(), i.as_str()), (Some("-2"), "-3"));
    let (t, r, i) = apply(Perturbation::ConstantAdd, &sq);
    assert_eq!(t.len(), 4);
    assert_eq!(t[3], Term::new(1, 0, 0));
    assert_eq!((r, i.as_str()), (None, "1"));
    let (t, r, i) = apply(Perturbation::Exponent(2, 1), &sq);
    assert_eq!(t[2], Term::new(1, 0, 3));
    assert_eq!((r.as_deref(), i.as_str()), (Some("y ^ 2"), "y ^ 3"));
    // every perturbation changes the polynomial
    for kind in PERTURBATIONS {
        for form in SEED_FORMS {
            let p = form.poly();
            for target in targets(kind, &p) {
                let (t, _, _) = apply(target, &p);
                assert_ne!(normalize(&t), p, "{kind} {target:?} on {:?}", p);
            }
        }
    }
}

#[test]
fn d5_flaw_0001_perturbs_one_step_and_the_calc_line_fails_there() {
    for (n, m) in [(6u64, 0u64), (6, 1), (3, 2)] {
        let control = grammar::prepare(&config(n, m, "pair", json!([])), 7).unwrap();
        for kind in PERTURBATIONS {
            for (sub, severity) in [
                ("propagated", Severity::Fatal),
                ("local", Severity::Repairable),
            ] {
                let flaws = if sub == "propagated" {
                    propagated(kind)
                } else {
                    local(kind)
                };
                let c = config(n, m, "pair", flaws.clone());
                let (p, manifest) = build(&c, 7);
                let inj = p.injection.as_ref().expect("an injected instance");
                assert_eq!(inj.class, "FLAW-0001");
                assert_eq!(inj.sub_variant.as_deref(), Some(sub));
                assert_eq!(inj.severity, severity);
                let (m, n) = (m as usize, n as usize);
                let site = inj.site;
                assert!(site > m && site <= m + n, "the site is a chain step");
                let chain = site - m;
                if sub == "local" {
                    assert!(chain < n, "local sits before the last step");
                }
                assert!(control.plan.steps[site - 1].lemma.is_none());
                assert!(!control.plan.steps[site - 1].fragment[0].contains("_lemma_"));
                // step 0 is untouched
                assert_eq!(p.plan.claim, control.plan.claim);
                assert_eq!(p.plan.statement, control.plan.statement);
                assert_eq!(p.plan.steps.len(), control.plan.steps.len());
                // the site's text and fragment changed; the dials say how
                let s = &p.plan.steps[site - 1];
                let f = &control.plan.steps[site - 1];
                assert_ne!(s.text, f.text);
                assert_ne!(s.fragment, f.fragment);
                let d = &inj.informal_dials;
                assert_eq!(d["perturbation"], json!(kind));
                assert_eq!(d["site"], json!(site));
                assert!(
                    d["monomial"]["inserted"]
                        .as_str()
                        .is_some_and(|s| !s.is_empty())
                );
                if *kind == "constant" {
                    assert!(
                        d["monomial"]["replaced"].is_null()
                            || d["monomial"]["replaced"].is_string()
                    );
                } else {
                    assert!(d["monomial"]["replaced"].is_string());
                }
                assert_ne!(d["monomial"]["replaced"], d["monomial"]["inserted"]);
                assert_eq!(d["replaced"], json!(derive(n, m, 7).e[chain].latex));
                assert_eq!(s.text.matches(d["inserted"].as_str().unwrap()).count(), 1);
                let originals = grammar::originals(d, "original_steps");
                assert_eq!(originals[&site], json!(f.text));
                let fd = &inj.formal_dials;
                assert_eq!(fd["inserted"], json!(s.fragment));
                assert_eq!(fd["replaced"], json!(f.fragment));
                let changed: Vec<usize> = (1..=m + n)
                    .filter(|&k| p.plan.steps[k - 1] != control.plan.steps[k - 1])
                    .collect();
                if sub == "local" {
                    assert_eq!(changed, vec![site], "local changes the site alone");
                    assert_eq!(originals.len(), 1);
                } else {
                    // every later chain step is re-derived from the perturbed polynomial, and
                    // the chain ends away from the claim
                    for k in site..=m + n {
                        assert!(changed.contains(&k), "step {k} re-derived");
                    }
                    let last = p.plan.steps.last().unwrap();
                    let faithful_last = control.plan.steps.last().unwrap();
                    assert_ne!(last.fragment, faithful_last.fragment);
                    for k in 1..=m {
                        let cites = derive(n, m, 7).sites[k - 1];
                        if cites > chain {
                            assert!(
                                changed.contains(&k),
                                "lemma {k}, cited after the site, is restated"
                            );
                        } else {
                            assert!(
                                !changed.contains(&k),
                                "lemma {k}, cited before the site, stands"
                            );
                        }
                    }
                }
                // the records: one instance, two locations, confirmed under the predicted K
                let gt = manifest.ground_truth.as_ref().unwrap();
                assert_eq!(gt.flaws.len(), 2);
                assert!(
                    gt.flaws
                        .iter()
                        .all(|r| r.instance == 1 && r.class == "FLAW-0001")
                );
                assert_eq!(gt.proof_status, ProofStatus::Invalid);
                assert!(!gt.correct);
                assert_eq!(gt.statement_status.value, Some(true));
                let formal = p.formal.as_ref().unwrap();
                let line = formal.fragment_lines[site - 1].unwrap()[0];
                for r in &gt.flaws {
                    match &r.location {
                        Location::Informal(l) => {
                            assert_eq!(l.step, site as u64);
                            let text = &parse_informal_md(p.informal.as_ref().unwrap())
                                .unwrap()
                                .steps[site];
                            assert_eq!(
                                syndeom_manifest::span_text(text, l.span).as_deref(),
                                d["inserted"].as_str()
                            );
                        }
                        Location::Formal(l) => {
                            assert_eq!(l.declaration, MAIN_DECLARATION);
                            assert_eq!(l.line, line);
                        }
                        other => panic!("{other:?}"),
                    }
                }
                assert_eq!(manifest.commanded["lean_build"].value, json!("fail"));
                assert!(!manifest.commanded.contains_key("lean_axioms_standard"));
                assert_eq!(manifest.commanded["n_flaws_total"].value, json!(1));
                assert_eq!(manifest.commanded["n_flaws_0001"].value, json!(1));
                assert_eq!(manifest.achieved["flaws_confirmed"].value, Some(json!(1)));
                assert_eq!(verdict(&manifest), json!("PASS"));
                // a build that passes, or fails elsewhere, is a FAIL verdict, not a refusal
                for obs in [
                    Observation {
                        exit_code: 0,
                        first_error_line: None,
                        axioms: Some(vec!["propext".into()]),
                        used_constants: None,
                    },
                    Observation {
                        exit_code: 1,
                        first_error_line: Some(line + 1),
                        axioms: None,
                        used_constants: None,
                    },
                ] {
                    let bad = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
                    assert_eq!(verdict(&bad), json!("FAIL"));
                    assert_eq!(bad.achieved["flaws_confirmed"].value, Some(json!(0)));
                }
                // the informal-only and formal-only targets carry one record each
                let (pi, mi) = build(&config(n as u64, m as u64, "informal", flaws.clone()), 7);
                assert_eq!(mi.ground_truth.as_ref().unwrap().flaws.len(), 1);
                assert_eq!(pi.injection.as_ref().unwrap().site, site);
                assert_eq!(verdict(&mi), json!("PASS"));
                let (_, mf) = build(&config(n as u64, m as u64, "formal", flaws), 7);
                assert_eq!(mf.ground_truth.as_ref().unwrap().flaws.len(), 1);
                assert_eq!(verdict(&mf), json!("PASS"));
            }
        }
    }
}

#[test]
fn d5_an_explicit_site_is_an_eligible_chain_step_and_draws_the_same_way() {
    let n = 6u64;
    let m = 1u64;
    let d = derive(6, 1, 7);
    let lemma_site = d.sites[0];
    let eligible: Vec<usize> = (1..=6).filter(|&c| c != lemma_site).collect();
    for &c in &eligible {
        let k = 1 + c; // the plan index: one lemma step first
        let (p, manifest) = build(
            &config(n, m, "pair", flaw("propagated", "fatal", "sign", json!(k))),
            7,
        );
        assert_eq!(p.injection.as_ref().unwrap().site, k);
        assert_eq!(verdict(&manifest), json!("PASS"));
        if c < 6 {
            let (p, _) = build(
                &config(n, m, "pair", flaw("local", "repairable", "sign", json!(k))),
                7,
            );
            assert_eq!(p.injection.as_ref().unwrap().site, k);
        } else {
            assert!(
                grammar::prepare(
                    &config(n, m, "pair", flaw("local", "repairable", "sign", json!(k))),
                    7
                )
                .is_err()
            );
        }
    }
    // the lemma step itself, a lemma-cited chain step and a step past the chain are refused
    for k in [1u64, (1 + lemma_site) as u64, 8, 0] {
        assert!(
            grammar::prepare(
                &config(n, m, "pair", flaw("propagated", "fatal", "sign", json!(k))),
                7
            )
            .is_err(),
            "site {k}"
        );
    }
}

#[test]
fn d3_unsupported_configs_and_specs_are_refused() {
    let ok = |c: &Value| grammar::prepare(c, 7).is_ok();
    let refused = |c: &Value| grammar::prepare(c, 7).is_err();
    assert!(ok(&config(1, 0, "informal", json!([]))));
    assert!(ok(&config(256, 32, "formal", json!([]))));
    assert!(refused(&config(0, 0, "informal", json!([]))));
    assert!(refused(&config(257, 0, "informal", json!([]))));
    assert!(refused(&config(40, 33, "informal", json!([]))));
    assert!(
        refused(&config(2, 3, "informal", json!([]))),
        "more lemmas than steps"
    );
    assert!(ok(&config(2, 2, "informal", json!([]))));
    let mut c = config(4, 0, "informal", json!([]));
    c["grammar"] = json!("ineq_chain");
    assert!(refused(&c), "a grammar of a later version");
    let mut c = config(4, 0, "informal", json!([]));
    c["padding_ratio"] = json!(0.2);
    assert!(refused(&c), "a ratio without the FLAW-0013 spec");
    assert!(
        refused(&config(
            4,
            0,
            "informal",
            json!([{
                "class": "FLAW-0013", "sub_variant": null, "severity": "cosmetic", "site": "auto",
                "dials": {"kinds": ["re-derivation"], "count": 1, "placement": "before"}
            }])
        )),
        "the FLAW-0013 spec without a positive ratio"
    );
    assert!(
        refused(&config(
            4,
            0,
            "pair",
            json!([{
                "class": "FLAW-0002", "sub_variant": null, "severity": "gap", "site": "auto",
                "dials": {"swap": "forall-exists"}
            }])
        )),
        "a class a later grammar version realizes"
    );
    assert!(refused(&config(
        4,
        0,
        "pair",
        json!([{
            "class": "FLAW-0099", "sub_variant": null, "severity": "gap", "site": "auto", "dials": {}
        }])
    )));
    let two = json!([propagated("sign")[0], local("sign")[0]]);
    assert!(
        refused(&config(4, 0, "pair", two)),
        "one instance per artifact"
    );
    assert!(refused(&config(
        4,
        0,
        "pair",
        flaw("propagated", "repairable", "sign", json!("auto"))
    )));
    assert!(refused(&config(
        4,
        0,
        "pair",
        flaw("propagated", "gap", "sign", json!("auto"))
    )));
    assert!(refused(&config(
        4,
        0,
        "pair",
        flaw("local", "fatal", "sign", json!("auto"))
    )));
    assert!(refused(&config(
        4,
        0,
        "pair",
        flaw("local", "cosmetic", "sign", json!("auto"))
    )));
    assert!(refused(&config(
        4,
        0,
        "pair",
        flaw("global", "fatal", "sign", json!("auto"))
    )));
    assert!(
        refused(&config(
            4,
            0,
            "pair",
            json!([{
                "class": "FLAW-0001", "sub_variant": null, "severity": "fatal", "site": "auto",
                "dials": {"perturbation": "sign"}
            }])
        )),
        "a sub-variant is required"
    );
    assert!(refused(&config(
        4,
        0,
        "pair",
        flaw("propagated", "fatal", "modulus", json!("auto"))
    )));
    assert!(
        refused(&config(
            4,
            0,
            "pair",
            json!([{
                "class": "FLAW-0001", "sub_variant": "propagated", "severity": "fatal", "site": "auto",
                "dials": {"perturbation": "sign", "site": 3}
            }])
        )),
        "a reserved dial"
    );
    assert!(
        refused(&config(
            4,
            0,
            "pair",
            json!([{
                "class": "FLAW-0001", "sub_variant": "propagated", "severity": "fatal", "site": "auto",
                "dials": {}
            }])
        )),
        "the perturbation dial is required"
    );
    assert!(
        refused(&config(1, 0, "pair", local("sign"))),
        "local needs a step after the site"
    );
    assert!(ok(&config(1, 0, "pair", propagated("sign"))));
    assert!(
        refused(&config(1, 1, "pair", propagated("sign"))),
        "the one step is lemma-cited"
    );
    // the config's shape: exactly the five keys, the length object's two
    assert!(refused(
        &json!({"grammar": "ring_chain", "length": {"n_steps": 4, "n_lemmas": 0}, "target": "informal", "flaws": []})
    ));
    assert!(refused(
        &json!({"grammar": "ring_chain", "length": {"n_steps": 4}, "target": "informal", "flaws": [], "padding_ratio": 0})
    ));
    assert!(refused(
        &json!({"grammar": "ring_chain", "length": {"n_steps": 4, "n_lemmas": 0, "n_tokens": 50}, "target": "informal", "flaws": [], "padding_ratio": 0})
    ));
    assert!(refused(
        &json!({"grammar": "ring_chain", "length": {"n_steps": -4, "n_lemmas": 0}, "target": "informal", "flaws": [], "padding_ratio": 0})
    ));
    assert!(refused(
        &json!({"grammar": "ring_chain", "length": {"n_steps": 4.5, "n_lemmas": 0}, "target": "informal", "flaws": [], "padding_ratio": 0})
    ));
    assert!(
        refused(
            &json!({"theorem": "nat_le_sq", "style": "two_cases", "target": "informal", "flaws": [], "padding_ratio": 0})
        ),
        "an s2_template config"
    );
    let mut c = config(4, 0, "informal", json!([]));
    c["target"] = json!("both");
    assert!(refused(&c));
    // the version a manifest names must exist
    assert!(GrammarVersion::parse("0.4.0").is_none());
}

#[test]
fn d8_a_long_instance_renders_and_measures() {
    let (p, m) = build(&config(256, 32, "pair", json!([])), 7);
    assert_eq!(p.plan.steps.len(), 288);
    let informal = parse_informal_md(p.informal.as_ref().unwrap()).unwrap();
    assert_eq!(informal.n_steps, 288);
    assert_eq!(informal.n_lemmas, 32);
    let formal = parse_lean(&p.formal.as_ref().unwrap().bytes).unwrap();
    assert_eq!(formal.n_declarations, 33);
    assert_eq!(m.commanded["n_steps"].value, json!(288));
    assert_eq!(m.commanded["n_lemmas"].value, json!(32));
    assert_eq!(verdict(&m), json!("PASS"));
    // n_tokens is achieved only, for both producers (D8)
    assert!(!m.commanded.contains_key("n_tokens"));
    assert!(
        m.achieved["n_tokens"]
            .value
            .as_ref()
            .and_then(Value::as_u64)
            .unwrap()
            > 288
    );
    // FLAW-0001 on a long chain with lemmas: propagated restates the lemmas cited after the site
    let (p, m) = build(&config(256, 32, "pair", propagated("exponent")), 7);
    assert_eq!(verdict(&m), json!("PASS"));
    assert_eq!(m.ground_truth.as_ref().unwrap().flaws.len(), 2);
    assert!(p.injection.as_ref().unwrap().site > 32);
    // 0.2.0: the most unused lemmas, and the most padding, on the longest instance
    let (p, m) = build(&config(256, 32, "pair", unused_spec(13)), 7);
    assert_eq!(p.plan.steps.len(), 301);
    assert_eq!(m.commanded["n_lemmas"].value, json!(45));
    assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(13)));
    assert_eq!(verdict(&m), json!("PASS"));
    let c = padded(
        256,
        32,
        "pair",
        json!(["re-derivation", "unused-lemma"]),
        32,
        "drawn",
    );
    let (p, m) = build(&c, 7);
    assert_eq!(p.plan.steps.len(), 320);
    assert_eq!(p.padding.len(), 32);
    assert_eq!(m.achieved["flaws_confirmed"].value, Some(json!(32)));
    assert_eq!(verdict(&m), json!("PASS"));
}

#[test]
fn d5_flaw_0007_unused_inserts_irrelevant_lemmas_after_the_cited_ones_cited_nowhere() {
    for (n, m, count) in [(4u64, 0u64, 1u64), (4, 0, 2), (6, 2, 3), (3, 2, 13)] {
        let control = grammar::prepare(&config(n, m, "pair", json!([])), 7).unwrap();
        let c = config(n, m, "pair", unused_spec(count));
        let (p, manifest) = build(&c, 7);
        let (n, m, count) = (n as usize, m as usize, count as usize);
        let d = derive(n, m, 7);
        // the plan: the cited lemmas, then the unused ones, then the chain; no faithful step
        // changes, and step 0 is untouched
        assert_eq!(p.plan.steps.len(), n + m + count);
        assert_eq!(p.plan.claim, control.plan.claim);
        assert_eq!(p.plan.statement, control.plan.statement);
        for k in 0..m {
            assert_eq!(p.plan.steps[k], control.plan.steps[k]);
        }
        for k in 0..n {
            assert_eq!(p.plan.steps[m + count + k], control.plan.steps[m + k]);
        }
        assert!(p.padding.is_empty() && p.composite.is_none() && p.false_theorem.is_none());
        let instances: Vec<&Injection> = p.injection.iter().chain(&p.further).collect();
        assert_eq!(instances.len(), count);
        let mut forms: BTreeSet<usize> = BTreeSet::new();
        for (i, inj) in instances.iter().enumerate() {
            let number = m + i + 1;
            assert_eq!(inj.instance as usize, i + 1);
            assert_eq!(inj.class, "FLAW-0007");
            assert_eq!(inj.sub_variant.as_deref(), Some("unused"));
            assert_eq!(inj.severity, Severity::Cosmetic);
            assert_eq!(inj.lean_catchable, LeanCatchable::YesTooling);
            assert_eq!(inj.site, number);
            let s = &p.plan.steps[number - 1];
            let l = s.lemma.as_ref().expect("an unused lemma is a lemma step");
            assert_eq!(l.name, format!("{MAIN_DECLARATION}_unused_{}", i + 1));
            assert!(
                s.text
                    .starts_with(&format!("Lemma {number}. For all integers $x$ and $y$, $"))
            );
            assert!(
                s.text
                    .ends_with("$; both sides expand to the same polynomial.")
            );
            assert_eq!(
                s.fragment,
                vec!["intro x y".to_string(), "ring".to_string()]
            );
            assert!(s.premises.is_empty() && s.padding.is_none());
            // the identity: a seed form the instance does not use, drawn without replacement,
            // equated to a presentation of its expansion
            let form = inj.informal_dials["seed_form"].as_u64().unwrap() as usize;
            assert!((1..=SEED_FORMS.len()).contains(&form) && form - 1 != d.form);
            assert!(forms.insert(form), "forms are drawn without replacement");
            let sf = &SEED_FORMS[form - 1];
            let claim = l.claim.strip_prefix("∀ x y : ℤ, ").unwrap();
            let (a, b) = claim.split_once(" = ").unwrap();
            assert_eq!(a, sf.lean());
            assert_ne!(a, b);
            assert!(s.text.contains(&format!("${} = ", sf.latex())));
            // nothing cites the lemma
            assert!(p.plan.steps.iter().all(|t| {
                !t.fragment.iter().any(|f| f.contains(&l.name))
                    && t.premises.iter().all(|pr| pr.cites != l.name)
            }));
            // the dials: the spec's, the site, the form, the inserted text or lines, nothing
            // replaced, no originals
            let di = &inj.informal_dials;
            assert_eq!(di["count"], json!(count));
            assert_eq!(di["length_steps"], json!(1));
            assert_eq!(di["site"], json!(number));
            assert_eq!(di["inserted"], json!(s.text));
            assert!(di["replaced"].is_null());
            assert_eq!(di["original_steps"], json!({}));
            let df = &inj.formal_dials;
            assert_eq!(df["declaration"], json!(l.name));
            assert_eq!(df["inserted"], json!(lemma_lines(s).unwrap()));
            assert_eq!(df["seed_form"], json!(form));
            assert!(df["replaced"].is_null());
            assert_eq!(df["original_fragments"], json!({}));
        }
        // the records: two per instance, a valid proof of a true statement, confirmed under
        // the predicted T
        let gt = manifest.ground_truth.as_ref().unwrap();
        assert_eq!(gt.flaws.len(), 2 * count);
        assert_eq!(gt.proof_status, ProofStatus::Valid);
        assert!(gt.correct);
        assert_eq!(gt.statement_status.value, Some(true));
        let formal = p.formal.as_ref().unwrap();
        let informal = parse_informal_md(p.informal.as_ref().unwrap()).unwrap();
        for r in &gt.flaws {
            assert_eq!(r.class, "FLAW-0007");
            assert_eq!(r.sub_variant.as_deref(), Some("unused"));
            assert_eq!(r.severity, Severity::Cosmetic);
            assert_eq!(r.lean_catchable, LeanCatchable::YesTooling);
            let number = m + r.instance as usize;
            match &r.location {
                Location::Informal(l) => {
                    assert_eq!(l.step, number as u64);
                    assert_eq!(l.span, [0, informal.steps[number].chars().count() as u64]);
                }
                Location::Formal(l) => {
                    assert_eq!(
                        l.declaration,
                        format!("{MAIN_DECLARATION}_unused_{}", r.instance)
                    );
                    assert_eq!(l.line, formal.fragment_lines[number - 1].unwrap()[0]);
                }
                other => panic!("{other:?}"),
            }
        }
        assert_eq!(manifest.commanded["n_steps"].value, json!(n + m + count));
        assert_eq!(manifest.commanded["n_lemmas"].value, json!(m + count));
        assert_eq!(manifest.commanded["n_flaws_total"].value, json!(count));
        assert_eq!(manifest.commanded["n_flaws_0007"].value, json!(count));
        assert_eq!(manifest.commanded["lean_build"].value, json!("pass"));
        assert_eq!(
            manifest.commanded["lean_axioms_standard"].value,
            json!(true)
        );
        assert_eq!(
            manifest.achieved["flaws_confirmed"].value,
            Some(json!(count))
        );
        assert_eq!(manifest.achieved["n_lemmas"].value, Some(json!(m + count)));
        assert_eq!(informal.n_lemmas as usize, m + count);
        assert_eq!(verdict(&manifest), json!("PASS"));
        // the formal file: the cited lemmas, then the unused ones, then the main declaration
        let text = String::from_utf8(formal.bytes.clone()).unwrap();
        let pos = |needle: &str| text.find(needle).unwrap_or_else(|| panic!("{needle}"));
        let main_at = pos(&format!("\ntheorem {MAIN_DECLARATION} (x y : ℤ) : "));
        for i in 1..=count {
            let at = pos(&format!(
                "\ntheorem {MAIN_DECLARATION}_unused_{i} : ∀ x y : ℤ, "
            ));
            assert!(at < main_at);
            for j in 1..=m {
                assert!(pos(&format!("theorem {MAIN_DECLARATION}_lemma_{j} :")) < at);
            }
        }
        assert_eq!(
            parse_lean(&formal.bytes).unwrap().n_declarations as usize,
            1 + m + count
        );
        // check T refuses confirmation when the main declaration uses an unused lemma,
        // directly or through an auxiliary constant of the file, and when no listing exists;
        // the artifact's verdict is then FAIL, not a refusal
        let name = format!("{MAIN_DECLARATION}_unused_1");
        let mut obs = predicted(&p).unwrap();
        obs.used_constants
            .as_mut()
            .unwrap()
            .get_mut(MAIN_DECLARATION)
            .unwrap()
            .push(name.clone());
        let bad = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
        assert_eq!(
            bad.achieved["flaws_confirmed"].value,
            Some(json!(count - 1))
        );
        assert_eq!(verdict(&bad), json!("FAIL"));
        let mut obs = predicted(&p).unwrap();
        {
            let used = obs.used_constants.as_mut().unwrap();
            used.insert("aux._proof_1".into(), vec![name.clone()]);
            used.get_mut(MAIN_DECLARATION)
                .unwrap()
                .push("aux._proof_1".into());
        }
        let bad = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
        assert_eq!(
            bad.achieved["flaws_confirmed"].value,
            Some(json!(count - 1))
        );
        let mut obs = predicted(&p).unwrap();
        obs.used_constants = None;
        let bad = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
        assert_eq!(bad.achieved["flaws_confirmed"].value, Some(json!(0)));
        assert_eq!(verdict(&bad), json!("FAIL"));
        // the informal-only and formal-only targets carry one record per instance
        let (pi, mi) = build(
            &config(n as u64, m as u64, "informal", unused_spec(count as u64)),
            7,
        );
        assert_eq!(mi.ground_truth.as_ref().unwrap().flaws.len(), count);
        assert_eq!(pi.injection.iter().chain(&pi.further).count(), count);
        assert!(!mi.commanded.contains_key("lean_build"));
        assert_eq!(verdict(&mi), json!("PASS"));
        let (_, mf) = build(
            &config(n as u64, m as u64, "formal", unused_spec(count as u64)),
            7,
        );
        assert_eq!(mf.ground_truth.as_ref().unwrap().flaws.len(), count);
        assert_eq!(mf.commanded["lean_build"].value, json!("pass"));
        assert_eq!(verdict(&mf), json!("PASS"));
    }
    // determinism: two preparations agree; another seed draws other forms or presentations
    let c = config(6, 1, "pair", unused_spec(2));
    let (p1, m1) = build(&c, 7);
    let (p2, m2) = build(&c, 7);
    assert_eq!(p1, p2);
    assert_eq!(m1, m2);
    let (p3, _) = build(&c, 8);
    assert_ne!(p3.plan.steps, p1.plan.steps);
    // the refusals
    let refused = |flaws: Value| grammar::prepare(&config(4, 0, "pair", flaws), 7).is_err();
    let spec = |sub: Value, severity: &str, site: Value, dials: Value| {
        json!([{
            "class": "FLAW-0007", "sub_variant": sub, "severity": severity, "site": site,
            "dials": dials
        }])
    };
    let one = json!({"count": 1, "length_steps": 1});
    assert!(
        refused(spec(json!("vacuous"), "fatal", json!("auto"), one.clone())),
        "vacuous needs hand-authored unsatisfiable hypotheses"
    );
    assert!(refused(spec(
        json!(null),
        "cosmetic",
        json!("auto"),
        one.clone()
    )));
    assert!(
        refused(spec(json!("unused"), "gap", json!("auto"), one.clone())),
        "unused is cosmetic"
    );
    assert!(
        refused(spec(json!("unused"), "cosmetic", json!(1), one.clone())),
        "the site is auto"
    );
    assert!(refused(spec(
        json!("unused"),
        "cosmetic",
        json!("auto"),
        json!({"count": 0, "length_steps": 1})
    )));
    assert!(
        refused(spec(
            json!("unused"),
            "cosmetic",
            json!("auto"),
            json!({"count": 14, "length_steps": 1})
        )),
        "at most the other thirteen forms"
    );
    assert!(
        refused(spec(
            json!("unused"),
            "cosmetic",
            json!("auto"),
            json!({"count": 1, "length_steps": 2})
        )),
        "one identity per lemma"
    );
    assert!(
        refused(spec(
            json!("unused"),
            "cosmetic",
            json!("auto"),
            json!({"count": 1})
        )),
        "both dials"
    );
    assert!(
        refused(spec(
            json!("unused"),
            "cosmetic",
            json!("auto"),
            json!({"count": 1, "length_steps": 1, "seed_form": 3})
        )),
        "a reserved dial"
    );
    let two = json!([unused_spec(1)[0], propagated("sign")[0]]);
    assert!(refused(two), "one spec per artifact");
    // 0.1.0 never realized the class: its goldens regenerate under the version they name
    assert!(
        prepare_for(
            Producer::Grammar(GrammarVersion::V0_1_0),
            &config(4, 0, "pair", unused_spec(1)),
            7
        )
        .is_err()
    );
}

#[test]
fn d5_flaw_0013_pads_with_generated_identities_and_their_unused_declarations() {
    for (n, m, kinds, count, placement) in [
        (4u64, 0u64, json!(["re-derivation"]), 1u64, "drawn"),
        (4, 0, json!(["unused-lemma"]), 2, "before"),
        (4, 0, json!(["re-derivation", "unused-lemma"]), 2, "drawn"),
        (6, 2, json!(["re-derivation", "unused-lemma"]), 5, "drawn"),
        (6, 2, json!(["unused-lemma", "re-derivation"]), 32, "before"),
    ] {
        let control = grammar::prepare(&config(n, m, "pair", json!([])), 7).unwrap();
        let c = padded(n, m, "pair", kinds.clone(), count, placement);
        let (p, manifest) = build(&c, 7);
        let (n, m, count) = (n as usize, m as usize, count as usize);
        let d = derive(n, m, 7);
        assert_eq!(p.plan.steps.len(), n + m + count);
        assert_eq!(p.padding.len(), count);
        assert!(p.injection.is_none() && p.further.is_empty());
        assert_eq!(p.plan.claim, control.plan.claim);
        // the argument's order is preserved: deleting the padding steps gives the control's
        // plan (ADR-0011 Options 8)
        let rest: Vec<_> = p
            .plan
            .steps
            .iter()
            .filter(|s| s.padding.is_none())
            .cloned()
            .collect();
        assert_eq!(rest, control.plan.steps);
        let named: Vec<String> = kinds
            .as_array()
            .unwrap()
            .iter()
            .map(|k| k.as_str().unwrap().to_string())
            .collect();
        for ins in &p.padding {
            let s = &p.plan.steps[ins.site - 1];
            let h = s.padding.as_ref().expect("a padding step");
            assert!(s.lemma.is_none() && s.premises.is_empty());
            assert!(!s.text.starts_with("Lemma"));
            assert_eq!(h.kind, ins.kind);
            assert!(named.contains(&h.kind.as_str().to_string()));
            assert_eq!(h.name, format!("{MAIN_DECLARATION}_pad_{}", ins.instance));
            assert_eq!(ins.declaration, h.name);
            assert_eq!(ins.paragraph, ins.instance);
            assert_eq!(ins.text, s.text);
            assert_eq!(
                s.fragment,
                vec!["intro x y".to_string(), "ring".to_string()]
            );
            let form = ins.informal_dials["seed_form"].as_u64().unwrap() as usize;
            let (a, b) = h
                .claim
                .strip_prefix("∀ x y : ℤ, ")
                .unwrap()
                .split_once(" = ")
                .unwrap();
            assert_ne!(a, b);
            match h.kind {
                PaddingKind::ReDerivation => {
                    assert_eq!(form - 1, d.form, "the instance's own polynomial");
                    assert!(s.text.starts_with("As a check, "));
                }
                PaddingKind::UnusedLemma => {
                    assert_ne!(form - 1, d.form, "a seed form the instance does not use");
                    assert_eq!(a, SEED_FORMS[form - 1].lean());
                    assert!(s.text.starts_with("Note the identity: "));
                }
                other => panic!("{other:?}"),
            }
            assert!(
                s.text
                    .ends_with("$; both sides expand to the same polynomial.")
            );
            assert!(
                p.plan
                    .steps
                    .iter()
                    .all(|t| !t.fragment.iter().any(|f| f.contains(&h.name)))
            );
            let di = &ins.informal_dials;
            assert_eq!(di["kinds"], kinds);
            assert_eq!(di["count"], json!(count));
            assert_eq!(di["placement"], json!(placement));
            assert_eq!(di["ratio"], c["padding_ratio"]);
            assert_eq!(di["site"], json!(ins.site));
            assert_eq!(di["kind"], json!(h.kind.as_str()));
            assert_eq!(di["inserted"], json!(s.text));
            assert!(di["replaced"].is_null());
            assert!(di.get("paragraph").is_none());
            let df = &ins.formal_dials;
            assert_eq!(df["declaration"], json!(h.name));
            assert_eq!(df["inserted"], json!(padding_lines(s).unwrap()));
            assert_eq!(df["seed_form"], json!(form));
            assert!(df["replaced"].is_null());
            if placement == "before" {
                assert_eq!(
                    ins.site, ins.instance as usize,
                    "before the first step, in drawn order"
                );
            }
        }
        // the records: one instance per paragraph, informal NO, formal YES-TOOLING, confirmed
        let gt = manifest.ground_truth.as_ref().unwrap();
        assert_eq!(gt.flaws.len(), 2 * count);
        assert_eq!(gt.proof_status, ProofStatus::Valid);
        assert!(gt.correct);
        let formal = p.formal.as_ref().unwrap();
        let informal = parse_informal_md(p.informal.as_ref().unwrap()).unwrap();
        for r in &gt.flaws {
            assert_eq!(r.class, "FLAW-0013");
            assert_eq!(r.sub_variant, None);
            assert_eq!(r.severity, Severity::Cosmetic);
            let ins = &p.padding[r.instance as usize - 1];
            match &r.location {
                Location::Informal(l) => {
                    assert_eq!(r.lean_catchable, LeanCatchable::No);
                    assert_eq!(l.step, ins.site as u64);
                    assert_eq!(l.span, [0, informal.steps[ins.site].chars().count() as u64]);
                }
                Location::Formal(l) => {
                    assert_eq!(r.lean_catchable, LeanCatchable::YesTooling);
                    assert_eq!(l.declaration, ins.declaration);
                    assert_eq!(l.line, formal.fragment_lines[ins.site - 1].unwrap()[0]);
                }
                other => panic!("{other:?}"),
            }
        }
        assert_eq!(manifest.commanded["n_steps"].value, json!(n + m + count));
        assert_eq!(manifest.commanded["n_lemmas"].value, json!(m));
        assert_eq!(manifest.commanded["n_flaws_total"].value, json!(count));
        assert_eq!(manifest.commanded["n_flaws_0013"].value, json!(count));
        assert_eq!(manifest.commanded["lean_build"].value, json!("pass"));
        assert_eq!(
            json!(manifest.commanded["padding_ratio"].tolerance),
            json!({"abs": 0.05})
        );
        assert_eq!(
            manifest.achieved["flaws_confirmed"].value,
            Some(json!(count))
        );
        assert_eq!(informal.n_lemmas as usize, m);
        assert_eq!(
            parse_lean(&formal.bytes).unwrap().n_declarations as usize,
            1 + m + count
        );
        assert_eq!(verdict(&manifest), json!("PASS"));
        // check T refuses confirmation when the main declaration uses a padding declaration
        let mut obs = predicted(&p).unwrap();
        obs.used_constants
            .as_mut()
            .unwrap()
            .get_mut(MAIN_DECLARATION)
            .unwrap()
            .push(p.padding[0].declaration.clone());
        let bad = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
        assert_eq!(
            bad.achieved["flaws_confirmed"].value,
            Some(json!(count - 1))
        );
        assert_eq!(verdict(&bad), json!("FAIL"));
        // the informal-only and formal-only targets: one record per paragraph, the ratio over
        // the file that `n_tokens` is measured on
        let (pi, mi) = build(
            &padded(
                n as u64,
                m as u64,
                "informal",
                kinds.clone(),
                count as u64,
                placement,
            ),
            7,
        );
        assert_eq!(mi.ground_truth.as_ref().unwrap().flaws.len(), count);
        assert_eq!(pi.padding.len(), count);
        assert_eq!(verdict(&mi), json!("PASS"));
        let (_, mf) = build(
            &padded(
                n as u64,
                m as u64,
                "formal",
                kinds.clone(),
                count as u64,
                placement,
            ),
            7,
        );
        assert_eq!(mf.ground_truth.as_ref().unwrap().flaws.len(), count);
        assert!(
            mf.achieved["padding_ratio"]
                .value
                .as_ref()
                .and_then(Value::as_f64)
                .unwrap()
                > 0.0
        );
        assert_eq!(verdict(&mf), json!("PASS"));
    }
    // both kinds occur when both are named, over a few seeds
    let mut seen: BTreeSet<PaddingKind> = BTreeSet::new();
    for seed in 0..8 {
        let mut c = config(
            4,
            0,
            "informal",
            padding_spec(json!(["re-derivation", "unused-lemma"]), 3, "drawn"),
        );
        c["padding_ratio"] = json!(1);
        for ins in &grammar::prepare(&c, seed).unwrap().padding {
            seen.insert(ins.kind);
        }
    }
    assert_eq!(seen.len(), 2);
    // determinism
    let c = padded(
        6,
        1,
        "pair",
        json!(["re-derivation", "unused-lemma"]),
        3,
        "drawn",
    );
    let (p1, m1) = build(&c, 7);
    let (p2, m2) = build(&c, 7);
    assert_eq!(p1, p2);
    assert_eq!(m1, m2);
    // the refusals
    let refused = |c: &Value| grammar::prepare(c, 7).is_err();
    let with_ratio = |flaws: Value, ratio: f64| {
        let mut c = config(4, 0, "pair", flaws);
        c["padding_ratio"] = json!(ratio);
        c
    };
    let one = |kinds: Value| padding_spec(kinds, 1, "drawn");
    assert!(
        refused(&with_ratio(one(json!(["re-derivation"])), 0.0)),
        "the spec needs a positive ratio"
    );
    assert!(
        refused(&with_ratio(json!([]), 0.2)),
        "a ratio needs the spec"
    );
    assert!(
        refused(&with_ratio(one(json!(["restated-definition"])), 0.2)),
        "no material"
    );
    assert!(refused(&with_ratio(one(json!(["verbose-rewrite"])), 0.2)));
    assert!(refused(&with_ratio(
        one(json!(["re-derivation", "restated-definition"])),
        0.2
    )));
    assert!(refused(&with_ratio(one(json!([])), 0.2)));
    assert!(refused(&with_ratio(
        padding_spec(json!(["re-derivation", "re-derivation"]), 2, "drawn"),
        0.2
    )));
    assert!(refused(&with_ratio(one(json!(["nonsense"])), 0.2)));
    assert!(refused(&with_ratio(
        padding_spec(json!(["re-derivation"]), 0, "drawn"),
        0.2
    )));
    assert!(refused(&with_ratio(
        padding_spec(json!(["re-derivation"]), 33, "drawn"),
        0.2
    )));
    assert!(refused(&with_ratio(
        padding_spec(json!(["re-derivation"]), 1, "after"),
        0.2
    )));
    let mut s = one(json!(["re-derivation"]));
    s[0]["sub_variant"] = json!("x");
    assert!(refused(&with_ratio(s, 0.2)));
    let mut s = one(json!(["re-derivation"]));
    s[0]["site"] = json!(1);
    assert!(refused(&with_ratio(s, 0.2)), "the site is auto");
    let mut s = one(json!(["re-derivation"]));
    s[0]["severity"] = json!("gap");
    assert!(refused(&with_ratio(s, 0.2)));
    let mut s = one(json!(["re-derivation"]));
    s[0]["dials"]["ratio"] = json!(0.2);
    assert!(refused(&with_ratio(s, 0.2)), "a reserved dial");
    let two = json!([one(json!(["re-derivation"]))[0], propagated("sign")[0]]);
    assert!(refused(&with_ratio(two, 0.2)), "one spec per artifact");
    let two = json!([one(json!(["re-derivation"]))[0], unused_spec(1)[0]]);
    assert!(refused(&with_ratio(two, 0.2)));
    // 0.1.0 refuses padding: its goldens regenerate under the version they name
    assert!(
        prepare_for(
            Producer::Grammar(GrammarVersion::V0_1_0),
            &with_ratio(one(json!(["re-derivation"])), 0.2),
            7
        )
        .is_err()
    );
}

#[test]
fn d5_flaw_0009_axiom_replaces_a_cited_lemma_and_the_axiom_print_lists_it() {
    for (n, m, truth) in [
        (4u64, 1u64, true),
        (4, 1, false),
        (6, 2, true),
        (6, 2, false),
        (3, 3, true),
        (3, 3, false),
        (24, 2, false),
        (1, 1, false),
    ] {
        let control = grammar::prepare(&config(n, m, "pair", json!([])), 7).unwrap();
        let c = config(n, m, "pair", axiom_spec(truth, json!("auto")));
        let (p, manifest) = build(&c, 7);
        let (n, m) = (n as usize, m as usize);
        let d = derive(n, m, 7);
        let inj = p.injection.as_ref().expect("one instance");
        assert!(p.further.is_empty() && p.padding.is_empty());
        assert!(p.composite.is_none() && p.false_theorem.is_none());
        assert_eq!(inj.instance, 1);
        assert_eq!(inj.class, "FLAW-0009");
        assert_eq!(inj.sub_variant, None);
        let severity = if truth {
            Severity::Gap
        } else {
            Severity::Fatal
        };
        assert_eq!(inj.severity, severity);
        assert_eq!(inj.lean_catchable, LeanCatchable::YesTooling);
        // the site is a lemma step, drawn on the stream: lemma j states chain step c
        let j = inj.site;
        assert!((1..=m).contains(&j));
        let mut stream = Stream::new(7, "injector/FLAW-0009/1");
        assert_eq!(j, 1 + stream.below(m as u64) as usize);
        let chain = d.sites[j - 1];
        let name = format!("{MAIN_DECLARATION}_lemma_{j}");
        // the plan: every step the control's, except the lemma's declaration — an axiom with
        // no proof — and, under `false`, the citing step's justification; step 0 and every
        // informal text untouched
        assert_eq!(p.plan.steps.len(), n + m);
        assert_eq!(p.plan.claim, control.plan.claim);
        assert_eq!(p.plan.statement, control.plan.statement);
        assert_eq!(
            p.informal, control.informal,
            "the informal file is unchanged"
        );
        let axiom = &p.plan.steps[j - 1];
        let faithful_lemma = &control.plan.steps[j - 1];
        for k in 0..n + m {
            let (s, f) = (&p.plan.steps[k], &control.plan.steps[k]);
            assert_eq!(s.text, f.text);
            assert_eq!(s.premises, f.premises);
            assert!(s.padding.is_none());
            if k == j - 1 || (!truth && k == m + chain - 1) {
                continue;
            }
            assert_eq!(s, f);
        }
        let l = axiom.lemma.as_ref().unwrap();
        let fl = faithful_lemma.lemma.as_ref().unwrap();
        assert_eq!(l.kind, LemmaKind::Axiom);
        assert_eq!(fl.kind, LemmaKind::Theorem);
        assert_eq!(l.name, name);
        assert_eq!(fl.name, name);
        assert!(axiom.fragment.is_empty(), "an axiom has no proof lines");
        let axiom_line = format!("axiom {name} : {}", l.claim);
        assert_eq!(lemma_lines(axiom).unwrap(), vec![axiom_line.clone()]);
        let (lhs, rhs) = l
            .claim
            .strip_prefix("∀ x y : ℤ, ")
            .unwrap()
            .split_once(" = ")
            .unwrap();
        assert_eq!(
            lhs,
            d.e[chain - 1].lean,
            "the left-hand side is the lemma's"
        );
        let citing = &p.plan.steps[m + chain - 1];
        let faithful_citing = &control.plan.steps[m + chain - 1];
        if truth {
            assert_eq!(l.claim, fl.claim, "an axiom of the same identity");
            assert_eq!(citing, faithful_citing);
        } else {
            // the false identity: the lemma's right-hand side perturbed once — the kind the
            // first of D5's four, from a drawn start, with an eligible target; the target
            // drawn next — rendered under the step's own presentation (the draws, in this
            // order, after the site's: D4)
            let site_terms = &d.e[chain].ordered;
            let start = stream.below(PERTURBATIONS.len() as u64) as usize;
            let (kind, candidates) = (0..PERTURBATIONS.len())
                .find_map(|off| {
                    let kind = PERTURBATIONS[(start + off) % PERTURBATIONS.len()];
                    let cands = targets(kind, site_terms);
                    (!cands.is_empty()).then_some((kind, cands))
                })
                .unwrap();
            let chosen = candidates[stream.below(candidates.len() as u64) as usize];
            let (perturbed, mono_replaced, mono_inserted) = apply(chosen, site_terms);
            let pres = &d.presentations[chain - 1];
            let mut styles = pres.styles.clone();
            styles.resize(perturbed.len(), Style::default());
            let f = render_ordered(&pres.shape, &styles, &perturbed);
            assert_eq!(rhs, f.lean);
            assert_ne!(rhs, d.e[chain].lean);
            let perturbed_poly = normalize(&perturbed);
            assert_ne!(perturbed_poly, d.poly, "the identity is false");
            // the witness point: the first grid point, by x + y and then x, where the two
            // sides differ
            let (x, y) = witness_point(&d.poly, &perturbed_poly).unwrap();
            assert!((1..=8).contains(&x) && (1..=8).contains(&y));
            let (lhs_value, rhs_value) = (eval(&d.poly, x, y), eval(&perturbed_poly, x, y));
            assert_ne!(lhs_value, rhs_value);
            let earlier = |x2: i64, y2: i64| (x2 + y2, x2) < (x + y, x);
            for x2 in 1..=8 {
                for y2 in 1..=8 {
                    if earlier(x2, y2) {
                        assert_eq!(eval(&d.poly, x2, y2), eval(&perturbed_poly, x2, y2));
                    }
                }
            }
            // the citing step: the faithful identity, justified ex falso by the axiom's
            // instance at the witness point, refuted by norm_num
            assert_eq!(citing.text, faithful_citing.text);
            assert_eq!(citing.lemma, None);
            assert_eq!(citing.premises, faithful_citing.premises);
            let proof = format!("absurd ({name} {x} {y}) (by norm_num)");
            let faithful_line = &faithful_citing.fragment[0];
            let (identity, faithful_proof) = faithful_line.rsplit_once(" := ").unwrap();
            assert_eq!(faithful_proof, format!("{name} x y"));
            assert_eq!(citing.fragment, vec![format!("{identity} := {proof}")]);
            let df = &inj.formal_dials;
            assert_eq!(df["perturbation"], json!(kind));
            assert_eq!(
                df["monomial"],
                json!({"replaced": mono_replaced, "inserted": mono_inserted})
            );
            assert_eq!(
                df["counterexample"],
                json!({"x": x, "y": y, "lhs": lhs_value, "rhs": rhs_value})
            );
            assert_eq!(df["use_site"], json!(m + chain));
        }
        // the dials
        let df = &inj.formal_dials;
        assert_eq!(df["mode"], json!("axiom"));
        assert_eq!(df["truth"], json!(truth));
        assert_eq!(df["site"], json!(j));
        assert_eq!(df["declaration"], json!(name));
        assert_eq!(df["inserted"], json!([axiom_line]));
        assert_eq!(df["replaced"], json!(lemma_lines(faithful_lemma).unwrap()));
        let originals = grammar::originals(df, "original_fragments");
        let mut expected: BTreeMap<usize, Value> = BTreeMap::new();
        expected.insert(j, json!(lemma_lines(faithful_lemma).unwrap()));
        if !truth {
            expected.insert(m + chain, json!(faithful_citing.fragment));
        }
        assert_eq!(originals, expected);
        if truth {
            for key in ["perturbation", "monomial", "counterexample", "use_site"] {
                assert!(df.get(key).is_none(), "{key}");
            }
        }
        // the record: formal-only, at the axiom's declaration; an invalid proof of a true
        // statement whose reference is the formal control
        let gt = manifest.ground_truth.as_ref().unwrap();
        assert_eq!(
            gt.flaws.len(),
            1,
            "FLAW-0009 manifests in the formal file only"
        );
        let r = &gt.flaws[0];
        assert_eq!(r.instance, 1);
        assert_eq!(r.class, "FLAW-0009");
        assert_eq!(r.sub_variant, None);
        assert_eq!(r.severity, severity);
        assert_eq!(r.lean_catchable, LeanCatchable::YesTooling);
        let formal = p.formal.as_ref().unwrap();
        match &r.location {
            Location::Formal(l) => {
                assert_eq!(l.declaration, name);
                assert_eq!(formal.fragment_lines[j - 1].unwrap(), [l.line, l.line]);
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(r.dials, *df);
        assert_eq!(gt.proof_status, ProofStatus::Invalid);
        assert!(!gt.correct);
        assert_eq!(gt.statement_status.value, Some(true));
        assert_eq!(
            gt.statement_status.reference,
            control_reference(GrammarVersion::CURRENT, "ring_chain", n as u64, m as u64, 7)
                .unwrap()
        );
        // commanded and achieved (D8): the counts the control's, the build passing, the
        // axioms not standard; confirmed under the predicted print
        assert_eq!(manifest.commanded["n_steps"].value, json!(n + m));
        assert_eq!(manifest.commanded["n_lemmas"].value, json!(m));
        assert_eq!(manifest.commanded["n_flaws_total"].value, json!(1));
        assert_eq!(manifest.commanded["n_flaws_0009"].value, json!(1));
        assert_eq!(manifest.commanded["lean_build"].value, json!("pass"));
        assert_eq!(
            manifest.commanded["lean_axioms_standard"].value,
            json!(false)
        );
        assert_eq!(manifest.achieved["flaws_confirmed"].value, Some(json!(1)));
        assert_eq!(
            manifest.achieved["lean_axioms_standard"].value,
            Some(json!(false))
        );
        assert_eq!(manifest.achieved["n_lemmas"].value, Some(json!(m)));
        assert_eq!(verdict(&manifest), json!("PASS"));
        // the formal file: the axiom where the theorem was, nothing else moved
        let text = String::from_utf8(formal.bytes.clone()).unwrap();
        let faithful_text =
            String::from_utf8(control.formal.as_ref().unwrap().bytes.clone()).unwrap();
        assert!(text.contains(&format!("\n{axiom_line}\n\n")));
        assert!(!text.contains(&format!("theorem {name} ")));
        assert!(faithful_text.contains(&format!("theorem {name} :")));
        assert_eq!(
            parse_lean(&formal.bytes).unwrap().n_declarations as usize,
            1 + m
        );
        if truth {
            // the faithful file with the lemma's declaration replaced is this file
            let replaced = lemma_lines(faithful_lemma).unwrap().join("\n");
            assert_eq!(faithful_text.replacen(&replaced, &axiom_line, 1), text);
        }
        // check T refuses confirmation when the print omits the axiom, or when there is no
        // print; the verdict is then FAIL, not a refusal
        let mut obs = predicted(&p).unwrap();
        obs.axioms = Some(vec!["propext".to_string(), "Quot.sound".to_string()]);
        let bad = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
        assert_eq!(bad.achieved["flaws_confirmed"].value, Some(json!(0)));
        assert_eq!(
            bad.achieved["lean_axioms_standard"].value,
            Some(json!(true))
        );
        assert_eq!(verdict(&bad), json!("FAIL"));
        let mut obs = predicted(&p).unwrap();
        obs.axioms = None;
        let bad = assemble(&p, COMMIT, Some(toolchain()), Some(&obs)).unwrap();
        assert_eq!(bad.achieved["flaws_confirmed"].value, Some(json!(0)));
        assert_eq!(verdict(&bad), json!("FAIL"));
        // the formal-only target carries the same one record and the same bytes; informal is
        // refused (D3)
        let (pf, mf) = build(
            &config(
                n as u64,
                m as u64,
                "formal",
                axiom_spec(truth, json!("auto")),
            ),
            7,
        );
        assert_eq!(pf.formal.as_ref().map(|f| &f.bytes), Some(&formal.bytes));
        assert_eq!(mf.ground_truth.as_ref().unwrap().flaws.len(), 1);
        assert!(!mf.commanded.contains_key("n_steps"));
        assert_eq!(mf.commanded["lean_axioms_standard"].value, json!(false));
        assert_eq!(verdict(&mf), json!("PASS"));
        assert!(
            grammar::prepare(
                &config(
                    n as u64,
                    m as u64,
                    "informal",
                    axiom_spec(truth, json!("auto"))
                ),
                7
            )
            .is_err()
        );
    }
    // determinism: two preparations agree; another seed draws another instance
    let c = config(6, 2, "pair", axiom_spec(false, json!("auto")));
    let (p1, m1) = build(&c, 7);
    let (p2, m2) = build(&c, 7);
    assert_eq!(p1, p2);
    assert_eq!(m1, m2);
    let (p3, _) = build(&c, 8);
    assert_ne!(p3.plan.steps, p1.plan.steps);
    // an explicit site is a lemma step's plan index and skips the site draw
    for site in 1..=2usize {
        let (p, m) = build(&config(6, 2, "pair", axiom_spec(true, json!(site))), 7);
        assert_eq!(p.injection.as_ref().unwrap().site, site);
        let l = p.plan.steps[site - 1].lemma.as_ref().unwrap();
        assert_eq!(l.kind, LemmaKind::Axiom);
        assert_eq!(l.name, format!("{MAIN_DECLARATION}_lemma_{site}"));
        assert_eq!(verdict(&m), json!("PASS"));
    }
    // the refusals
    let refused = |n: u64, m: u64, target: &str, flaws: Value| {
        grammar::prepare(&config(n, m, target, flaws), 7).is_err()
    };
    let spec = |sub: Value, severity: &str, site: Value, dials: Value| {
        json!([{
            "class": "FLAW-0009", "sub_variant": sub, "severity": severity, "site": site,
            "dials": dials
        }])
    };
    let axiom = |truth: bool| json!({"mode": "axiom", "truth": truth});
    assert!(
        refused(4, 0, "pair", axiom_spec(true, json!("auto"))),
        "a lemma to replace"
    );
    assert!(
        refused(4, 1, "informal", axiom_spec(true, json!("auto"))),
        "formal or pair"
    );
    assert!(
        refused(
            4,
            1,
            "pair",
            spec(json!(null), "fatal", json!("auto"), axiom(true))
        ),
        "true is gap"
    );
    assert!(
        refused(
            4,
            1,
            "pair",
            spec(json!(null), "gap", json!("auto"), axiom(false))
        ),
        "false is fatal"
    );
    assert!(refused(
        4,
        1,
        "pair",
        spec(json!(null), "cosmetic", json!("auto"), axiom(true))
    ));
    assert!(
        refused(
            4,
            1,
            "pair",
            spec(json!("x"), "gap", json!("auto"), axiom(true))
        ),
        "no sub-variant"
    );
    assert!(
        refused(
            4,
            1,
            "pair",
            spec(
                json!(null),
                "gap",
                json!("auto"),
                json!({"mode": "native_decide", "truth": true})
            )
        ),
        "native_decide stays refused"
    );
    assert!(refused(
        4,
        1,
        "pair",
        spec(
            json!(null),
            "gap",
            json!("auto"),
            json!({"mode": "sorry", "truth": true})
        )
    ));
    assert!(
        refused(
            4,
            1,
            "pair",
            spec(
                json!(null),
                "gap",
                json!("auto"),
                json!({"mode": "axiom", "truth": "true"})
            )
        ),
        "truth is a boolean"
    );
    assert!(
        refused(
            4,
            1,
            "pair",
            spec(json!(null), "gap", json!("auto"), json!({"mode": "axiom"}))
        ),
        "both dials"
    );
    assert!(
        refused(
            4,
            1,
            "pair",
            spec(
                json!(null),
                "gap",
                json!("auto"),
                json!({"mode": "axiom", "truth": true, "perturbation": "sign"})
            )
        ),
        "no extra dial"
    );
    assert!(
        refused(
            4,
            1,
            "pair",
            spec(
                json!(null),
                "gap",
                json!("auto"),
                json!({"mode": "axiom", "truth": true, "declaration": "x"})
            )
        ),
        "a reserved dial"
    );
    assert!(
        refused(4, 1, "pair", axiom_spec(true, json!(2))),
        "site 2 is a chain step"
    );
    assert!(refused(4, 1, "pair", axiom_spec(true, json!(0))));
    assert!(refused(4, 2, "pair", axiom_spec(true, json!(3))));
    let two = json!([axiom_spec(true, json!("auto"))[0], unused_spec(1)[0]]);
    assert!(refused(4, 1, "pair", two), "one spec per artifact");
    // 0.1.0 and 0.2.0 never realized the class: their goldens regenerate under the versions
    // they name
    for v in [GrammarVersion::V0_1_0, GrammarVersion::V0_2_0] {
        assert!(
            prepare_for(
                Producer::Grammar(v),
                &config(4, 1, "pair", axiom_spec(true, json!("auto"))),
                7
            )
            .is_err()
        );
    }
    // the 0.2.0 bytes of a FLAW-0007 artifact and of a padded one are unchanged by 0.3.0: only
    // the version, and with it the id and the manifest, differ
    for c in [
        config(4, 0, "pair", unused_spec(2)),
        padded(
            4,
            0,
            "pair",
            json!(["re-derivation", "unused-lemma"]),
            2,
            "drawn",
        ),
    ] {
        let now = grammar::prepare(&c, 7).unwrap();
        let old = prepare_for(Producer::Grammar(GrammarVersion::V0_2_0), &c, 7).unwrap();
        assert_eq!(old.informal, now.informal);
        assert_eq!(
            old.formal.as_ref().map(|f| &f.bytes),
            now.formal.as_ref().map(|f| &f.bytes)
        );
        assert_ne!(old.artifact_id(), now.artifact_id());
    }
}
