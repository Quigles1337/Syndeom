//! `s2_grammar` 0.8.0 (ADR-0011 D1, D6): the grammar `ineq_chain`, its control and FLAW-0001 —
//! the atom library (three families of true atoms), the expressions and their two renderings, the
//! derivation (one atom per step, the left side the sum of the steps' sources), the plan and its
//! two renderings, the reference of a grammar instance's truth (D7), the commanded and achieved
//! statistics (D8), determinism and the dispatch (D9), FLAW-0001 `propagated` and `local` read for
//! inequalities — a perturbation admitted only when a recorded grid point shows the perturbed
//! step false — with its records and check K (D5), the refusals (D3, D6), and `ring_chain`'s bytes
//! unchanged by 0.8.0. Lean is not run here: the observations are the ones packet 15's probes
//! showed (a control builds with the standard three axioms; a falsified step fails first at its
//! own `calc` line).

use std::collections::BTreeSet;

use serde_json::{Value, json};
use syndeom_generate::grammar::{self, GrammarVersion, PERTURBATIONS, control_reference};
use syndeom_generate::harness::Observation;
use syndeom_generate::ineq::{
    ATOMS, AbsForm, Base, Family, IMPORTS, INEQ_CHAIN, ITerm, MAIN_DECLARATION, WITNESS_HALF_BOUND,
    add, apply, derive, fraction, grid, latex, lean, lemma_label, normalize, site_candidates,
    step_label, targets, witness,
};
use syndeom_generate::rng::Stream;
use syndeom_generate::template::{Prepared, Producer, assemble};
use syndeom_generate::{prepare_any, prepare_for};
use syndeom_manifest::{
    LeanCatchable, Location, Manifest, PROOF_LINE, ProofStatus, Severity, StatementMethod,
    TERMINATOR, THEOREM_MARKER, Toolchain, parse_informal_md, span_text,
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
        "grammar": "ineq_chain",
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

/// The observation packet 15's probes show: a FLAW-0001 instance fails first at the site's `calc`
/// line (check K); a control builds, and `#print axioms` lists the standard three.
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
            axioms: Some(vec![
                "propext".into(),
                "Classical.choice".into(),
                "Quot.sound".into(),
            ]),
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

/// `8 · e(x2 / 2, y2 / 2)`, exactly — written apart from the crate's own evaluation, for an
/// independent check of the atoms and the witnesses (every degree here is at most 3).
fn eight(e: &[ITerm], x2: i128, y2: i128) -> i128 {
    e.iter()
        .map(|t| {
            let c = i128::from(t.c);
            match t.base {
                Base::Mono(i, j) => c * x2.pow(i) * y2.pow(j) * 2i128.pow(3 - i - j),
                Base::Abs(f) => {
                    let v = match f {
                        AbsForm::X => x2,
                        AbsForm::Y => y2,
                        AbsForm::XPlusY => x2 + y2,
                    };
                    c * 4 * v.abs()
                }
            }
        })
        .sum()
}

/// Twice a linear form of the square atoms at `(x2 / 2, y2 / 2)`.
fn linear2(form: &str, x2: i128, y2: i128) -> i128 {
    match form {
        "x - y" => x2 - y2,
        "x + y" => x2 + y2,
        "x - 1" => x2 - 2,
        "y - 1" => y2 - 2,
        "x + 1" => x2 + 2,
        "y + 1" => y2 + 2,
        other => panic!("no linear form {other}"),
    }
}

/// A recorded coordinate, `"k"` or `"k/2"`, doubled.
fn doubled(s: &str) -> i128 {
    match s.split_once('/') {
        Some((k, "2")) => k.parse().unwrap(),
        None => 2 * s.parse::<i128>().unwrap(),
        _ => panic!("not a half-integer: {s}"),
    }
}

/// A recorded value, `"p"` or `"p/q"`, times 8 (q divides 8 here).
fn eight_of(s: &str) -> i128 {
    match s.split_once('/') {
        Some((p, q)) => p.parse::<i128>().unwrap() * 8 / q.parse::<i128>().unwrap(),
        None => s.parse::<i128>().unwrap() * 8,
    }
}

fn half_grid() -> Vec<(i128, i128)> {
    let b = i128::from(WITNESS_HALF_BOUND);
    (-b..=b)
        .flat_map(|x| (-b..=b).map(move |y| (x, y)))
        .collect()
}

fn m(c: i64, i: u32, j: u32) -> ITerm {
    ITerm::mono(c, i, j)
}

/// A faithful chain step's reason, written apart from the crate's.
fn reason(atom: &syndeom_generate::ineq::Atom) -> String {
    match atom.family {
        Family::Square => format!("Since squares are non-negative, ${}$", atom.fact),
        Family::AmGm => format!("By the two-term AM-GM inequality, ${}$", atom.fact),
        Family::Triangle => format!("By the triangle inequality, ${}$", atom.fact),
    }
}

fn a(c: i64, f: AbsForm) -> ITerm {
    ITerm::abs(c, f)
}

#[test]
fn d6_the_atom_library_is_ten_true_atoms_of_the_three_families() {
    assert_eq!(ATOMS.len(), 10);
    let ids: BTreeSet<&str> = ATOMS.iter().map(|x| x.id).collect();
    assert_eq!(ids.len(), ATOMS.len());
    let count = |f: Family| ATOMS.iter().filter(|x| x.family == f).count();
    assert_eq!(
        (
            count(Family::Square),
            count(Family::AmGm),
            count(Family::Triangle)
        ),
        (6, 3, 1)
    );
    let pts: Vec<(i128, i128)> = (-6..=6)
        .flat_map(|x| (-6..=6).map(move |y| (x, y)))
        .collect();
    for atom in ATOMS {
        assert_eq!(
            normalize(atom.increment),
            atom.increment.to_vec(),
            "{}",
            atom.id
        );
        assert!(!atom.increment.is_empty(), "{}", atom.id);
        // every atom is true: no point of the grid at which e exceeds e + increment
        assert!(witness(&[], atom.increment).is_none(), "{}", atom.id);
        for &(x2, y2) in &pts {
            assert!(eight(atom.increment, x2, y2) >= 0, "{}", atom.id);
        }
        match atom.family {
            Family::Square => {
                // the increment is the square of the hint's linear form, at every point
                let form = atom
                    .hint
                    .strip_prefix("sq_nonneg (")
                    .and_then(|s| s.strip_suffix(')'))
                    .unwrap();
                assert_eq!(atom.fact, format!("({form})^2 \\ge 0"));
                assert!(atom.source.is_empty(), "a square consumes nothing");
                for &(x2, y2) in &pts {
                    let l = linear2(form, x2, y2);
                    assert_eq!(eight(atom.increment, x2, y2), 2 * l * l, "{}", atom.id);
                }
            }
            Family::AmGm => {
                // 2uv ≤ u^2 + v^2: the source is 2uv, the increment (u - v)^2
                let args = atom.hint.strip_prefix("two_mul_le_add_sq ").unwrap();
                let (u, v) = args.split_once(' ').unwrap();
                let form = format!("{u} - {v}");
                for &(x2, y2) in &pts {
                    let l = linear2(&form, x2, y2);
                    assert_eq!(eight(atom.increment, x2, y2), 2 * l * l, "{}", atom.id);
                }
                let product = match (u, v) {
                    ("x", "y") => m(2, 1, 1),
                    ("x", "1") => m(2, 1, 0),
                    ("y", "1") => m(2, 0, 1),
                    other => panic!("{other:?}"),
                };
                assert_eq!(atom.source, [product]);
                // what the step leaves: u^2 + v^2
                let both = add(atom.increment, atom.source);
                assert_eq!(both.len(), 2, "{}", atom.id);
                assert!(both.iter().all(|t| t.c == 1), "{}", atom.id);
            }
            Family::Triangle => {
                assert_eq!(atom.hint, "abs_add_le x y");
                assert_eq!(atom.fact, "|x + y| \\le |x| + |y|");
                assert_eq!(atom.source, [a(1, AbsForm::XPlusY)]);
                assert_eq!(
                    add(atom.increment, atom.source),
                    vec![a(1, AbsForm::X), a(1, AbsForm::Y)]
                );
            }
        }
    }
}

#[test]
fn d2_expressions_render_as_the_probes_wrote_them() {
    // packet 15's short probe: e_0 and e_4
    let e0 = normalize(&[a(1, AbsForm::XPlusY), m(4, 1, 1)]);
    assert_eq!(lean(&e0), "4 * x * y + |x + y|");
    assert_eq!(latex(&e0), "4xy + |x + y|");
    let e4 = normalize(&[
        a(1, AbsForm::Y),
        m(1, 0, 0),
        m(2, 0, 2),
        a(1, AbsForm::X),
        m(-2, 1, 0),
        m(3, 2, 0),
    ]);
    assert_eq!(lean(&e4), "3 * x ^ 2 + 2 * y ^ 2 - 2 * x + 1 + |x| + |y|");
    assert_eq!(latex(&e4), "3x^2 + 2y^2 - 2x + 1 + |x| + |y|");
    // the order: total degree, then the exponent of x, both descending; then |x|, |y|, |x + y|
    let all = normalize(&[
        a(1, AbsForm::XPlusY),
        m(1, 0, 0),
        a(1, AbsForm::X),
        m(1, 0, 2),
        m(1, 1, 1),
        m(1, 2, 0),
        a(1, AbsForm::Y),
        m(1, 1, 0),
        m(1, 0, 1),
    ]);
    assert_eq!(
        lean(&all),
        "x ^ 2 + x * y + y ^ 2 + x + y + 1 + |x| + |y| + |x + y|"
    );
    assert_eq!(
        latex(&all),
        "x^2 + xy + y^2 + x + y + 1 + |x| + |y| + |x + y|"
    );
    // coefficients on absolute values; a leading minus; merging and cancellation; zero
    assert_eq!(
        lean(&[a(-2, AbsForm::X), a(3, AbsForm::XPlusY)]),
        "-2 * |x| + 3 * |x + y|"
    );
    assert_eq!(
        latex(&[a(-2, AbsForm::X), a(3, AbsForm::XPlusY)]),
        "-2|x| + 3|x + y|"
    );
    assert!(normalize(&[m(2, 1, 1), m(-2, 1, 1)]).is_empty());
    assert_eq!(lean(&[]), "0");
    assert_eq!(latex(&[]), "0");
    assert_eq!(normalize(&[m(1, 2, 0), m(2, 2, 0)]), vec![m(3, 2, 0)]);
    // the witness grid and the fractions it is written in
    let g = grid();
    assert_eq!(g.len(), 81);
    assert_eq!(g[0], (0, 0));
    assert!(
        g.windows(2)
            .all(|w| w[0].0.abs() + w[0].1.abs() <= w[1].0.abs() + w[1].1.abs())
    );
    assert_eq!(fraction(3, 6), "1/2");
    assert_eq!(fraction(-4, 2), "-2");
    assert_eq!(fraction(0, 8), "0");
    assert_eq!(fraction(6, -4), "-3/2");
}

#[test]
fn d6_the_derivation_adds_one_atom_per_step_to_the_sum_of_the_sources() {
    for seed in 0..40u64 {
        for (n, m) in [(1usize, 0usize), (4, 0), (6, 2), (24, 2), (64, 0)] {
            let d = derive(n, m, seed);
            assert_eq!(d.atoms.len(), n);
            assert_eq!(d.e.len(), n + 1);
            for k in 1..=n {
                // one draw on the step's own label (D4)
                let drawn = Stream::new(seed, &step_label(k)).below(ATOMS.len() as u64) as usize;
                assert_eq!(d.atoms[k - 1], drawn);
            }
            let mut sources: Vec<ITerm> = Vec::new();
            for &at in &d.atoms {
                sources = add(&sources, ATOMS[at].source);
            }
            assert_eq!(d.e[0], sources, "e_0 is the sum of the sources");
            for k in 1..=n {
                assert_eq!(d.e[k], add(&d.e[k - 1], ATOMS[d.atoms[k - 1]].increment));
                assert_ne!(d.e[k], d.e[k - 1]);
                assert_eq!(normalize(&d.e[k]), d.e[k]);
                // every faithful step is true: no point of the grid falsifies it
                assert!(
                    witness(&d.e[k - 1], &d.e[k]).is_none(),
                    "seed {seed} step {k}"
                );
                for (x2, y2) in half_grid() {
                    assert!(eight(&d.e[k - 1], x2, y2) <= eight(&d.e[k], x2, y2));
                }
            }
            // the constants are never negative: atoms add +1, nothing subtracts
            for e in &d.e {
                assert!(e.iter().all(|t| !t.is_constant() || t.c > 0));
            }
            // the lemma sites: distinct chain steps, ascending, drawn on their own labels
            assert_eq!(d.sites.len(), m);
            assert!(d.sites.windows(2).all(|w| w[0] < w[1]));
            assert!(d.sites.iter().all(|&s| (1..=n).contains(&s)));
            if m >= 1 {
                let first = Stream::new(seed, &lemma_label(1)).below(n as u64) as usize + 1;
                assert!(d.sites.contains(&first), "lemma 1 takes the drawn step");
            }
            assert_eq!(d, derive(n, m, seed));
        }
    }
}

/// The formal file the plan of `d` renders to, written out from the derivation.
fn expected_formal(n: usize, m: usize, seed: u64) -> String {
    let d = derive(n, m, seed);
    let mut out = String::new();
    for i in IMPORTS {
        out.push_str(&format!("import {i}\n"));
    }
    out.push('\n');
    for (j, &c) in d.sites.iter().enumerate() {
        out.push_str(&format!(
            "theorem {MAIN_DECLARATION}_lemma_{} : ∀ x y : ℝ, {} ≤ {} := by\n  intro x y\n  linarith [{}]\n\n",
            j + 1,
            lean(&d.e[c - 1]),
            lean(&d.e[c]),
            ATOMS[d.atoms[c - 1]].hint
        ));
    }
    out.push_str(&format!(
        "theorem {MAIN_DECLARATION} (x y : ℝ) : {} ≤ {} := by\n",
        lean(&d.e[0]),
        lean(&d.e[n])
    ));
    for c in 1..=n {
        let proof = match d.sites.iter().position(|&s| s == c) {
            Some(j) => format!("{MAIN_DECLARATION}_lemma_{} x y", j + 1),
            None => format!("by linarith [{}]", ATOMS[d.atoms[c - 1]].hint),
        };
        if c == 1 {
            out.push_str(&format!(
                "  calc {} ≤ {} := {proof}\n",
                lean(&d.e[0]),
                lean(&d.e[1])
            ));
        } else {
            out.push_str(&format!("    _ ≤ {} := {proof}\n", lean(&d.e[c])));
        }
    }
    out
}

#[test]
fn d2_the_plan_renders_the_lemmas_then_a_calc_chain_closed_by_linarith_or_a_cited_lemma() {
    for (n, m) in [(4usize, 0usize), (6, 2), (24, 2)] {
        let c = config(n as u64, m as u64, "pair", json!([]));
        let (p, manifest) = build(&c, 7);
        assert_eq!(p.producer, Producer::Grammar(GrammarVersion::V0_8_0));
        assert_eq!(manifest.generator.id, "s2_grammar");
        assert_eq!(manifest.generator.version, "0.8.0");
        assert!(manifest.artifact_id.starts_with("s2_grammar-v0.8.0-"));
        let formal = p.formal.as_ref().unwrap();
        assert_eq!(
            String::from_utf8(formal.bytes.clone()).unwrap(),
            expected_formal(n, m, 7)
        );
        // the informal file: title, statement, the lemma steps, then the chain
        let d = derive(n, m, 7);
        let md = String::from_utf8(p.informal.clone().unwrap()).unwrap();
        let mut want = format!(
            "# An inequality in two real variables\n\n{THEOREM_MARKER}For all real numbers $x$ and $y$, ${} \\le {}$.\n\n{PROOF_LINE}\n\n",
            latex(&d.e[0]),
            latex(&d.e[n])
        );
        let mut k = 0;
        for (j, &s) in d.sites.iter().enumerate() {
            k += 1;
            let atom = &ATOMS[d.atoms[s - 1]];
            let note = match atom.family {
                Family::Square => format!(
                    "it follows from ${}$, since squares are non-negative",
                    atom.fact
                ),
                Family::AmGm => format!(
                    "it follows from the two-term AM-GM inequality ${}$",
                    atom.fact
                ),
                Family::Triangle => {
                    format!("it follows from the triangle inequality ${}$", atom.fact)
                }
            };
            want.push_str(&format!(
                "**Step {k}.** Lemma {}. For all real numbers $x$ and $y$, ${} \\le {}$; {note}.\n\n",
                j + 1,
                latex(&d.e[s - 1]),
                latex(&d.e[s])
            ));
        }
        for c in 1..=n {
            k += 1;
            let atom = &ATOMS[d.atoms[c - 1]];
            let text = match d.sites.iter().position(|&s| s == c) {
                Some(j) => format!(
                    "By Lemma {}, ${} \\le {}$.",
                    j + 1,
                    latex(&d.e[c - 1]),
                    latex(&d.e[c])
                ),
                None => format!(
                    "{}, so ${} \\le {}$.",
                    reason(atom),
                    latex(&d.e[c - 1]),
                    latex(&d.e[c])
                ),
            };
            want.push_str(&format!("**Step {k}.** {text}\n\n"));
        }
        want.push_str(TERMINATOR);
        want.push('\n');
        assert_eq!(md, want);
        // the citing step's premise names its lemma
        for (j, &s) in d.sites.iter().enumerate() {
            let step = &p.plan.steps[m + s - 1];
            assert_eq!(step.premises.len(), 1);
            assert_eq!(
                step.premises[0].cites,
                format!("{MAIN_DECLARATION}_lemma_{}", j + 1)
            );
            assert_eq!(step.text.matches(&step.premises[0].phrase).count(), 1);
        }
        // the counts
        assert_eq!(
            manifest.commanded["n_steps"].value,
            json!((n + m) as u64),
            "the plan's step count"
        );
        assert_eq!(manifest.commanded["n_lemmas"].value, json!(m as u64));
        assert_eq!(
            manifest.achieved["n_steps"].value,
            Some(json!((n + m) as u64))
        );
        assert_eq!(manifest.achieved["n_lemmas"].value, Some(json!(m as u64)));
    }
}

#[test]
fn d7_a_control_is_correct_and_its_truth_names_the_formal_control() {
    for (n, m) in [(4u64, 0u64), (24, 2), (1, 1)] {
        let (_, fm) = build(&config(n, m, "formal", json!([])), 7);
        let reference = format!("{}:{MAIN_DECLARATION}", fm.artifact_id);
        assert_eq!(
            reference,
            control_reference(GrammarVersion::CURRENT, INEQ_CHAIN, n, m, 7).unwrap(),
            "the formal control names itself"
        );
        for target in ["informal", "formal", "pair"] {
            let (_, manifest) = build(&config(n, m, target, json!([])), 7);
            let gt = manifest.ground_truth.as_ref().unwrap();
            assert!(gt.flaws.is_empty());
            assert_eq!(gt.proof_status, ProofStatus::Valid);
            assert!(gt.correct);
            assert_eq!(gt.statement_status.value, Some(true));
            assert_eq!(gt.statement_status.method, StatementMethod::LeanKernel);
            assert_eq!(gt.statement_status.reference, reference);
            assert_eq!(manifest.commanded["n_flaws_total"].value, json!(0));
            if target != "informal" {
                assert_eq!(manifest.commanded["lean_build"].value, json!("pass"));
                assert_eq!(
                    manifest.commanded["lean_axioms_standard"].value,
                    json!(true)
                );
                assert_eq!(
                    manifest.achieved["lean_axioms_standard"].value,
                    Some(json!(true))
                );
            }
            assert_eq!(verdict(&manifest), json!("PASS"));
        }
    }
    // ring_chain's reference keeps its own main declaration
    assert!(
        control_reference(GrammarVersion::CURRENT, "ring_chain", 4, 0, 7)
            .unwrap()
            .ends_with(":syndeom_s2_grammar_ring_chain")
    );
}

#[test]
fn d9_determinism_the_dispatch_and_the_version() {
    let c = config(6, 1, "pair", propagated("sign"));
    let (p1, m1) = build(&c, 7);
    let (p2, m2) = build(&c, 7);
    assert_eq!(p1, p2);
    assert_eq!(m1, m2);
    let (_, m3) = build(&c, 8);
    assert_ne!(m1.artifact_id, m3.artifact_id);
    assert_eq!(m1.config_hash, m3.config_hash);
    // generate's dispatch: a grammar config is an s2_grammar config at its current version
    let any = prepare_any(&c, 7).unwrap();
    assert_eq!(any.producer, Producer::Grammar(GrammarVersion::V0_8_0));
    assert_eq!(any.informal, p1.informal);
    // regenerate's path: the producer the manifest names rebuilds the same bytes
    let producer = Producer::parse(&m1.generator.id, &m1.generator.version).unwrap();
    assert_eq!(producer, Producer::Grammar(GrammarVersion::V0_8_0));
    let again = prepare_for(producer, &m1.config, 7).unwrap();
    assert_eq!(again.informal, p1.informal);
    assert_eq!(
        again.formal.as_ref().map(|f| &f.bytes),
        p1.formal.as_ref().map(|f| &f.bytes)
    );
}

#[test]
fn d5_the_admitted_perturbations_are_exactly_those_a_grid_point_falsifies() {
    for seed in 0..12u64 {
        let d = derive(12, 2, seed);
        for c in 1..=12usize {
            let left = &d.e[c - 1];
            for kind in PERTURBATIONS {
                let admitted = site_candidates(kind, &d, c);
                for target in targets(kind, &d.e[c]) {
                    let (terms, replaced, inserted) = apply(target, &d.e[c]);
                    let falsified = half_grid()
                        .into_iter()
                        .any(|(x2, y2)| eight(left, x2, y2) > eight(&terms, x2, y2));
                    let chosen = admitted.iter().find(|x| x.target == target);
                    if !falsified {
                        assert!(
                            chosen.is_none(),
                            "{kind} {target:?}: a true step is no flaw"
                        );
                        continue;
                    }
                    let Some(chosen) = chosen else {
                        // falsified but not admitted: its expression is not unique in the site's
                        // text, so an informal record could not locate it
                        let text = format!(
                            "{}, so ${} \\le {}$.",
                            reason(&ATOMS[d.atoms[c - 1]]),
                            latex(left),
                            latex(&terms)
                        );
                        assert_ne!(
                            text.matches(&latex(&terms)).count(),
                            1,
                            "{kind} {target:?} at {c}"
                        );
                        continue;
                    };
                    assert_eq!(chosen.terms, terms);
                    assert_eq!(chosen.replaced, replaced);
                    assert_eq!(chosen.inserted, inserted);
                    // the witness is a grid point at which the perturbed step is false and the
                    // faithful one is not
                    let w = &chosen.witness;
                    let (x2, y2) = (doubled(&w.x), doubled(&w.y));
                    assert!(x2.abs() <= 4 && y2.abs() <= 4);
                    assert_eq!(eight_of(&w.left), eight(left, x2, y2));
                    assert_eq!(eight_of(&w.right), eight(&terms, x2, y2));
                    assert!(eight(left, x2, y2) > eight(&terms, x2, y2));
                    assert!(eight(left, x2, y2) <= eight(&d.e[c], x2, y2));
                }
                if *kind == "constant" {
                    assert!(
                        admitted.is_empty(),
                        "a constant raised or added only enlarges"
                    );
                }
            }
        }
    }
}

#[test]
fn d5_flaw_0001_perturbs_one_term_so_the_step_is_false_and_the_calc_line_fails_there() {
    let mut built = 0;
    for seed in [7u64, 1, 2, 3, 11, 12] {
        for (n, m) in [(4u64, 0u64), (6, 1), (3, 2)] {
            let control = grammar::prepare(&config(n, m, "pair", json!([])), seed).unwrap();
            let d = derive(n as usize, m as usize, seed);
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
                    let p = match grammar::prepare(&c, seed) {
                        Ok(p) => p,
                        Err(e) => {
                            // a refusal names the missing perturbation: no step admits the kind
                            assert!(
                                e.contains(&format!("no eligible {kind} perturbation")),
                                "{e}"
                            );
                            let nn = n as usize;
                            for ch in 1..=nn {
                                if d.lemma_of(ch).is_none() && (sub == "propagated" || ch < nn) {
                                    assert!(
                                        site_candidates(kind, &d, ch).is_empty(),
                                        "{kind} {ch}"
                                    );
                                }
                            }
                            continue;
                        }
                    };
                    assert_ne!(*kind, "constant", "the constant kind is never admitted");
                    built += 1;
                    let obs = predicted(&p);
                    let manifest = assemble(&p, COMMIT, Some(toolchain()), obs.as_ref()).unwrap();
                    let inj = p.injection.as_ref().expect("an injected instance");
                    assert_eq!(inj.class, "FLAW-0001");
                    assert_eq!(inj.sub_variant.as_deref(), Some(sub));
                    assert_eq!(inj.severity, severity);
                    assert_eq!(inj.lean_catchable, LeanCatchable::YesKernel);
                    let (m, n) = (m as usize, n as usize);
                    let site = inj.site;
                    assert!(site > m && site <= m + n, "the site is a chain step");
                    let ch = site - m;
                    assert!(d.lemma_of(ch).is_none(), "never a lemma-cited step");
                    if sub == "local" {
                        assert!(ch < n, "local sits before the last step");
                    }
                    // step 0 is untouched
                    assert_eq!(p.plan.claim, control.plan.claim);
                    assert_eq!(p.plan.statement, control.plan.statement);
                    let s = &p.plan.steps[site - 1];
                    let f = &control.plan.steps[site - 1];
                    assert_ne!(s.text, f.text);
                    assert_ne!(s.fragment, f.fragment);
                    let dl = &inj.informal_dials;
                    assert_eq!(dl["perturbation"], json!(kind));
                    assert_eq!(dl["site"], json!(site));
                    assert!(dl["monomial"]["replaced"].is_string());
                    assert_ne!(dl["monomial"]["replaced"], dl["monomial"]["inserted"]);
                    assert_eq!(dl["replaced"], json!(latex(&d.e[ch])));
                    let inserted = dl["inserted"].as_str().unwrap();
                    assert_eq!(s.text.matches(inserted).count(), 1);
                    // the witness: the perturbed step is false there, the faithful one true
                    let w = &dl["witness"];
                    let (x2, y2) = (
                        doubled(w["x"].as_str().unwrap()),
                        doubled(w["y"].as_str().unwrap()),
                    );
                    let chosen = site_candidates(kind, &d, ch)
                        .into_iter()
                        .find(|x| latex(&x.terms) == inserted)
                        .expect("the recorded perturbation is an admitted one");
                    assert!(eight(&d.e[ch - 1], x2, y2) > eight(&chosen.terms, x2, y2));
                    assert!(eight(&d.e[ch - 1], x2, y2) <= eight(&d.e[ch], x2, y2));
                    assert_eq!(
                        eight_of(w["left"].as_str().unwrap()),
                        eight(&d.e[ch - 1], x2, y2)
                    );
                    assert_eq!(
                        eight_of(w["right"].as_str().unwrap()),
                        eight(&chosen.terms, x2, y2)
                    );
                    assert_eq!(inj.formal_dials["witness"], *w);
                    let fd = &inj.formal_dials;
                    assert_eq!(fd["inserted"], json!(s.fragment));
                    assert_eq!(fd["replaced"], json!(f.fragment));
                    let changed: Vec<usize> = (1..=m + n)
                        .filter(|&k| p.plan.steps[k - 1] != control.plan.steps[k - 1])
                        .collect();
                    if sub == "local" {
                        assert_eq!(changed, vec![site], "local changes the site alone");
                    } else {
                        // every later chain step carries the perturbation forward, each step
                        // after the site true again, and the chain ends away from the claim
                        for k in site..=m + n {
                            assert!(changed.contains(&k), "step {k} re-derived");
                        }
                        for j in 1..=m {
                            let cites = d.sites[j - 1];
                            assert_eq!(changed.contains(&j), cites > ch, "lemma {j}");
                        }
                        let mut cur = chosen.terms.clone();
                        for k in ch + 1..=n {
                            let next = add(&cur, ATOMS[d.atoms[k - 1]].increment);
                            assert!(witness(&cur, &next).is_none(), "step {k} after the site");
                            cur = next;
                        }
                        assert_ne!(
                            normalize(&cur),
                            d.e[n],
                            "the chain ends away from the claim"
                        );
                        assert!(p.plan.steps.last().unwrap().fragment[0].contains(&lean(&cur)));
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
                                assert_eq!(span_text(text, l.span).as_deref(), Some(inserted));
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
                    assert_eq!(manifest.commanded["n_flaws_0001"].value, json!(1));
                    assert_eq!(manifest.achieved["flaws_confirmed"].value, Some(json!(1)));
                    assert_eq!(verdict(&manifest), json!("PASS"));
                    // a build that passes, or fails first elsewhere, is a FAIL, not a refusal
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
                    }
                    // the informal-only and formal-only targets carry one record each, same site
                    let (pi, mi) =
                        build(&config(n as u64, m as u64, "informal", flaws.clone()), seed);
                    assert_eq!(mi.ground_truth.as_ref().unwrap().flaws.len(), 1);
                    assert_eq!(pi.injection.as_ref().unwrap().site, site);
                    assert_eq!(pi.informal, p.informal);
                    let (pf, mf) = build(&config(n as u64, m as u64, "formal", flaws), seed);
                    assert_eq!(mf.ground_truth.as_ref().unwrap().flaws.len(), 1);
                    assert_eq!(pf.formal.as_ref().unwrap().bytes, formal.bytes);
                    assert_eq!(verdict(&mf), json!("PASS"));
                }
            }
        }
    }
    assert!(built >= 40, "{built} flawed artifacts built");
}

#[test]
fn d5_an_explicit_site_is_an_admitted_chain_step() {
    let (n, m) = (6usize, 1usize);
    let d = derive(n, m, 7);
    for (sub, severity) in [("propagated", "fatal"), ("local", "repairable")] {
        let mut admitted_sites = 0;
        for k in 1..=m + n + 1 {
            let c = config(
                n as u64,
                m as u64,
                "informal",
                flaw(sub, severity, "sign", json!(k)),
            );
            let res = grammar::prepare(&c, 7);
            let ch = k.saturating_sub(m);
            let eligible = k > m
                && k <= m + n
                && d.lemma_of(ch).is_none()
                && (sub == "propagated" || ch < n)
                && !site_candidates("sign", &d, ch).is_empty();
            assert_eq!(res.is_ok(), eligible, "{sub} site {k}: {:?}", res.err());
            if let Ok(p) = res {
                admitted_sites += 1;
                assert_eq!(p.injection.unwrap().site, k);
            }
        }
        assert!(admitted_sites >= 2, "{sub}");
    }
}

#[test]
fn d3_unsupported_configs_and_specs_are_refused_on_ineq_chain() {
    let refused = |c: &Value| grammar::prepare(c, 7).is_err();
    let ok = |c: &Value| grammar::prepare(c, 7).is_ok();
    assert!(ok(&config(1, 0, "informal", json!([]))));
    assert!(ok(&config(256, 32, "formal", json!([]))));
    assert!(refused(&config(0, 0, "informal", json!([]))));
    assert!(refused(&config(257, 0, "informal", json!([]))));
    assert!(refused(&config(40, 33, "informal", json!([]))));
    assert!(
        refused(&config(2, 3, "informal", json!([]))),
        "more lemmas than steps"
    );
    // ineq_chain arrives in 0.8.0
    for v in [GrammarVersion::V0_1_0, GrammarVersion::V0_7_0] {
        let e = prepare_for(
            Producer::Grammar(v),
            &config(4, 0, "informal", json!([])),
            7,
        )
        .unwrap_err();
        assert!(e.contains("no grammar \"ineq_chain\""), "{e}");
    }
    // every class but FLAW-0001 is a later version on ineq_chain
    for k in 2..=14 {
        let class = format!("FLAW-{k:04}");
        let spec = json!([{"class": class, "sub_variant": null, "severity": "fatal",
                           "site": "auto", "dials": {}}]);
        let e = grammar::prepare(&config(4, 1, "pair", spec), 7).unwrap_err();
        if class != "FLAW-0013" {
            assert!(
                e.contains("is not realized on ineq_chain by s2_grammar 0.8.0"),
                "{e}"
            );
        }
    }
    let mut c = config(4, 0, "informal", json!([]));
    c["padding_ratio"] = json!(0.2);
    assert!(refused(&c), "a ratio without the FLAW-0013 spec");
    let pad = json!([{"class": "FLAW-0013", "sub_variant": null, "severity": "cosmetic",
                      "site": "auto", "dials": {"kinds": ["re-derivation"], "count": 1,
                      "placement": "drawn"}}]);
    let mut c = config(4, 0, "informal", pad);
    c["padding_ratio"] = json!(0.2);
    let e = grammar::prepare(&c, 7).unwrap_err();
    assert!(e.contains("FLAW-0013 is not realized on ineq_chain"), "{e}");
    // two specs
    let two = json!([
        {"class": "FLAW-0001", "sub_variant": "local", "severity": "repairable", "site": "auto",
         "dials": {"perturbation": "sign"}},
        {"class": "FLAW-0001", "sub_variant": "local", "severity": "repairable", "site": "auto",
         "dials": {"perturbation": "sign"}}
    ]);
    assert!(refused(&config(6, 0, "informal", two)));
    // FLAW-0001's own refusals
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
        flaw("local", "fatal", "sign", json!("auto"))
    )));
    assert!(refused(&config(
        4,
        0,
        "pair",
        flaw("drift", "fatal", "sign", json!("auto"))
    )));
    assert!(refused(&config(
        4,
        0,
        "pair",
        flaw("local", "repairable", "swap", json!("auto"))
    )));
    let extra = json!([{"class": "FLAW-0001", "sub_variant": "local", "severity": "repairable",
                        "site": "auto", "dials": {"perturbation": "sign", "witness": "0"}}]);
    assert!(
        refused(&config(4, 0, "pair", extra)),
        "the injector records the witness"
    );
    // local needs a step before the last: one step has none
    assert!(refused(&config(1, 0, "pair", local("sign"))));
    // the constant kind is never admitted on ineq_chain: its constants are positive
    for seed in 0..20u64 {
        for (n, m) in [(1u64, 0u64), (4, 0), (12, 2)] {
            let e =
                grammar::prepare(&config(n, m, "pair", propagated("constant")), seed).unwrap_err();
            assert!(e.contains("no eligible constant perturbation"), "{e}");
        }
    }
    // an explicit site that is a lemma step, a lemma-cited step, or beyond the plan
    let d = derive(6, 2, 7);
    let cited = d.sites[0];
    for k in [1u64, 2, 2 + cited as u64, 9] {
        let e = grammar::prepare(
            &config(6, 2, "pair", flaw("propagated", "fatal", "sign", json!(k))),
            7,
        )
        .unwrap_err();
        assert!(e.contains("is not an eligible step"), "{k}: {e}");
    }
}

#[test]
fn d8_a_long_instance_renders_and_measures_with_short_lines() {
    let mut longest = 0;
    for seed in [7u64, 8, 9] {
        let (p, manifest) = build(&config(256, 32, "pair", json!([])), seed);
        assert_eq!(manifest.commanded["n_steps"].value, json!(288));
        assert_eq!(manifest.commanded["n_lemmas"].value, json!(32));
        assert_eq!(verdict(&manifest), json!("PASS"));
        let text = String::from_utf8(p.formal.unwrap().bytes).unwrap();
        longest = longest.max(text.lines().map(|l| l.chars().count()).max().unwrap());
        let (pf, mf) = build(&config(256, 0, "formal", propagated("sign")), seed);
        assert_eq!(verdict(&mf), json!("PASS"));
        assert!(pf.injection.is_some());
    }
    // a chain of the dial's longest length keeps its lines short (packet 15's probe of 256 steps:
    // 175 characters)
    assert!(longest <= 400, "{longest}");
}

#[test]
fn d1_ring_chain_is_unchanged_by_0_8_0() {
    let ring = |n: u64, m: u64, flaws: Value| {
        json!({"grammar": "ring_chain", "length": {"n_steps": n, "n_lemmas": m},
               "target": "pair", "flaws": flaws, "padding_ratio": 0})
    };
    let composite = json!([{"class": "FLAW-0011", "sub_variant": null, "severity": "fatal",
        "site": "auto", "dials": {"embedded": [{"class": "FLAW-0001", "sub_variant": "propagated",
        "severity": "fatal", "site": "auto", "dials": {"perturbation": "sign"}}]}}]);
    for c in [
        ring(4, 0, json!([])),
        ring(24, 2, json!([])),
        ring(4, 0, propagated("sign")),
        ring(6, 1, local("exponent")),
        ring(4, 0, composite),
    ] {
        let now = grammar::prepare(&c, 7).unwrap();
        let old = prepare_for(Producer::Grammar(GrammarVersion::V0_7_0), &c, 7).unwrap();
        assert_eq!(now.producer, Producer::Grammar(GrammarVersion::V0_8_0));
        assert_eq!(old.informal, now.informal);
        assert_eq!(
            old.formal.as_ref().map(|f| &f.bytes),
            now.formal.as_ref().map(|f| &f.bytes)
        );
        assert_eq!(old.plan.truth.value, now.plan.truth.value);
        assert_ne!(old.artifact_id(), now.artifact_id());
    }
}
