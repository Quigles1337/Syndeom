//! `s2_grammar` 0.1.0 (ADR-0011 D1, D6): the grammar `ring_chain`, its control and FLAW-0001 —
//! the atom library, the presentations, the derivation, the plan and its two renderings, the
//! reference of a grammar instance's truth (D7), the commanded and achieved statistics (D8),
//! determinism (D9), FLAW-0001 `propagated` and `local` with their records and check K (D5),
//! and the refusals (D3, D6).

use serde_json::{Value, json};
use syndeom_generate::grammar::{
    self, GrammarVersion, MAIN_DECLARATION, PERTURBATIONS, Perturbation, SEED_FORMS, Shape, Style,
    Term, apply, control_reference, derive, mono_latex, mono_lean, normalize, render_ordered,
    targets,
};
use syndeom_generate::harness::Observation;
use syndeom_generate::template::{Prepared, Producer, assemble};
use syndeom_generate::{prepare_any, prepare_for};
use syndeom_manifest::{
    Location, Manifest, ProofStatus, Severity, StatementMethod, Toolchain, parse_informal_md,
    parse_lean,
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

/// The observation D5 predicts: a control builds with standard axioms; a FLAW-0001 instance
/// fails at the site's `calc` line (check K).
fn predicted(p: &Prepared) -> Option<Observation> {
    let f = p.formal.as_ref()?;
    Some(match &p.injection {
        Some(inj) => Observation {
            exit_code: 1,
            first_error_line: f.fragment_lines[inj.site - 1].map(|r| r[0]),
            axioms: None,
            used_constants: None,
        },
        None => Observation {
            exit_code: 0,
            first_error_line: None,
            axioms: Some(vec!["propext".to_string()]),
            used_constants: None,
        },
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
    assert_eq!(p.producer, Producer::Grammar(GrammarVersion::V0_1_0));
    assert_eq!(m.generator.id, "s2_grammar");
    assert_eq!(m.generator.version, "0.1.0");
    assert_eq!(m.generator.rng, "syndeom-rng/1");
    assert!(m.artifact_id.starts_with("s2_grammar-v0.1.0-"));
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
        let reference = control_reference(GrammarVersion::V0_1_0, "ring_chain", n, m, 7).unwrap();
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
    let a = control_reference(GrammarVersion::V0_1_0, "ring_chain", 4, 0, 7).unwrap();
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
    assert_eq!(any.producer, Producer::Grammar(GrammarVersion::V0_1_0));
    assert!(prepare_any(&json!({"theorem": "nat_le_sq", "grammar": "ring_chain"}), 7).is_err());
    assert!(prepare_any(&json!({"length": {"n_steps": 1, "n_lemmas": 0}}), 7).is_err());
    assert_eq!(
        Producer::parse("s2_grammar", "0.1.0"),
        Some(Producer::Grammar(GrammarVersion::V0_1_0))
    );
    assert_eq!(Producer::parse("s2_grammar", "0.2.0"), None);
    assert_eq!(Producer::parse("s2_grammars", "0.1.0"), None);
    assert_eq!(GrammarVersion::CURRENT.as_str(), "0.1.0");
    assert!(grammar::realizes(GrammarVersion::V0_1_0, "FLAW-0001"));
    assert!(!grammar::realizes(GrammarVersion::V0_1_0, "FLAW-0013"));
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
    assert!(refused(&c), "no padding material");
    assert!(refused(&config(
        4,
        0,
        "informal",
        json!([{
            "class": "FLAW-0013", "sub_variant": null, "severity": "cosmetic", "site": "auto",
            "dials": {"kinds": ["re-derivation"], "count": 1, "placement": "before"}
        }])
    )));
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
    assert!(GrammarVersion::parse("0.2.0").is_none());
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
}
