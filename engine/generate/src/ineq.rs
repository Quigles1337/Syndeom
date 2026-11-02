//! `s2_grammar` 0.8.0 — the grammar `ineq_chain` (ADR-0011 D6, D7; STATE's NEXT, S2 ordering
//! step 4): an inequality over ℝ in two variables, proved as a chain of `length.n_steps`
//! inequalities inside a `calc` block. Each step adds one instance of a true atom and is closed by
//! `linarith` with the atom's Mathlib lemma as its one hint, or by one of `length.n_lemmas` lemma
//! steps (D2) that state it and are cited. The grammar opens with its control and FLAW-0001
//! (`propagated`, `local`), probes first (Al's rulings on PR 39; packet 15's probes).
//!
//! **The atom library** (D6: "a fixed library of true atoms (squares are non-negative; two-term
//! AM–GM; the triangle inequality)") is the table [`ATOMS`]: six squares of small linear forms
//! (`sq_nonneg`), three instances of the two-term AM–GM inequality (`two_mul_le_add_sq`, whose
//! Mathlib docstring names the AM–GM inequality) and the triangle inequality for `x` and `y`
//! (`abs_add_le`). An expression is a sum of monomials `c · x^i · y^j` and absolute values
//! `c · |x|`, `c · |y|`, `c · |x + y|`, merged and ordered by [`normalize`] — monomials by total
//! degree, then by the exponent of `x`, both descending; then `|x|`, `|y|`, `|x + y|` — so the lines
//! of a chain of any length stay short (the probes: a 256-step chain's longest line is 175
//! characters). Step `k` adds its atom's increment, `e_k = e_{k-1} + increment`, so every step is
//! true by construction, and `linarith` closes it from the hint alone (the probes; without the
//! hint `linarith` fails, and `positivity` does not take a goal `a ≤ b`). The statement's left side
//! `e_0` is the sum of what the AM–GM and triangle steps consume (`2xy`, `2x`, `2y`; `|x + y|`), so
//! each such step visibly turns its source into the atom's larger side. The library's `source` is
//! this module — executor-authored synthetic material outside the S1 corpus (ADR-0011 D6, Al's
//! ruling 1 of 2026-10-27) — and nothing here is derived from `groundtruth/`.
//!
//! **Imports.** `Mathlib.Basic.Real.Basic` and `Mathlib.Tactic.Linarith`; the probes show each is
//! needed, and the pinned Mathlib deprecates `Mathlib.Data.Real.Basic` in favour of the first (since
//! 2026-08-27), so a file importing it would build with a deprecation warning.
//!
//! **Ground truth (D7)**, as for `ring_chain`: `value: true`, `method: lean-kernel`, the reference
//! the formal control of the same grammar, length and seed, at `syndeom_s2_grammar_ineq_chain`.
//!
//! **FLAW-0001 (D5), read for inequalities.** The perturbation kinds are `ring_chain`'s operations
//! on the site's right side `e_k` (ADR-0011 D5; Al's rulings on PR 33): `sign` flips one term's
//! sign; `coefficient` raises one non-constant term's coefficient by one in magnitude (an
//! absolute-value term is non-constant); `constant` does the same to the constant term, or adds `+
//! 1` when there is none; `exponent` raises one exponent of `x` or `y` in a monomial by one where
//! the raised monomial is not another term's. A target is admitted only when it makes the step
//! false: a point of a fixed grid at which `e_{k-1}` exceeds the perturbed `e_k` is found and
//! recorded as the record's `witness`, so a perturbation that leaves the inequality true is no flaw
//! and never chosen (the probes: such a step builds, and the first error moves to the next line).
//! The site is a `linarith`-closed chain step (never lemma-cited, never a lemma; for `local`, never
//! the last) at which the kind has an admitted target; the site, then the target, are drawn on
//! `injector/FLAW-0001/1`. The site keeps its terms in place with the one term perturbed.
//! `propagated` (fatal) adds every later step's increment to the perturbed expression, restating
//! the lemmas those steps cite, so the chain ends away from the claim of record, which is the
//! faithful one; Lean reports `linarith`'s failure at the site, then the theorem-level `calc.step`
//! goal. `local` (repairable) changes the site alone, so the next step asserts the faithful
//! consequence; its hint certifies only the faithful increment, so Lean usually reports a second
//! error there (the probes), but not necessarily — a reading flagged for Al (packet 15).
//!
//! RNG labels new in 0.8.0: `grammar/ineq_chain/step/<k>` (the atom step k adds) and
//! `grammar/ineq_chain/lemma/<j>` (the step lemma j states, among the steps not yet taken);
//! FLAW-0001 draws on `injector/FLAW-0001/1`, as on `ring_chain`.

use std::cmp::Reverse;

use serde_json::{Map, Value, json};
use syndeom_manifest::{LeanCatchable, Severity, StatementMethod, StatementStatus};

use crate::grammar::{
    Config, GrammarVersion, PERTURBATIONS, Perturbation, Style, Term, control_reference,
    mono_latex, mono_lean, sum_join,
};
use crate::plan::{
    LemmaHeader, LemmaKind, Plan, Premise, Step, lemma_lines, render_formal, render_informal,
};
use crate::rng::Stream;
use crate::template::{
    FlawSpec, Injection, Prepared, Producer, Target, check_spec, exact_keys, unique_span,
};

/// The grammar's name in a config (ADR-0011 D3, D6).
pub const INEQ_CHAIN: &str = "ineq_chain";
/// The main declaration's name (ADR-0011 D2: `syndeom_s2_<producer>_<theorem>`).
pub const MAIN_DECLARATION: &str = "syndeom_s2_grammar_ineq_chain";
/// The imports every `ineq_chain` file needs (D6: the minimal `Mathlib.*` modules; packet 15's
/// probes).
pub const IMPORTS: &[&str] = &["Mathlib.Basic.Real.Basic", "Mathlib.Tactic.Linarith"];
/// Where the material comes from (ADR-0011 D6, Al's ruling 1 of 2026-10-27).
pub const SOURCE: &str = "engine/generate/src/ineq.rs, s2_grammar 0.8.0: the ineq_chain atom library (ATOMS), executor-authored synthetic material outside the S1 corpus (ADR-0011 D6)";
/// The witness grid (FLAW-0001): `x = X / 2` and `y = Y / 2` for integers `X` and `Y` with
/// `|X|, |Y| ≤ WITNESS_HALF_BOUND`, searched in order of `|X| + |Y|`, then `X`, then `Y`.
pub const WITNESS_HALF_BOUND: i64 = 4;

// ---- expressions --------------------------------------------------------------------------------

/// The absolute values an expression may carry, in the order a normal form lists them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AbsForm {
    X,
    Y,
    XPlusY,
}

impl AbsForm {
    /// The absolute value as both files write it.
    pub fn text(self) -> &'static str {
        match self {
            AbsForm::X => "|x|",
            AbsForm::Y => "|y|",
            AbsForm::XPlusY => "|x + y|",
        }
    }

    /// Twice the absolute value at `(x2 / 2, y2 / 2)`.
    fn doubled(self, x2: i64, y2: i64) -> i64 {
        match self {
            AbsForm::X => x2,
            AbsForm::Y => y2,
            AbsForm::XPlusY => x2 + y2,
        }
        .abs()
    }
}

/// What a term multiplies: the monomial `x^i · y^j`, or an absolute value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Mono(u32, u32),
    Abs(AbsForm),
}

impl Base {
    fn order(self) -> (u8, Reverse<u32>, Reverse<u32>, Option<AbsForm>) {
        match self {
            Base::Mono(i, j) => (0, Reverse(i + j), Reverse(i), None),
            Base::Abs(f) => (1, Reverse(0), Reverse(0), Some(f)),
        }
    }

    /// The degree in `(x, y)`: an absolute value of a linear form has degree 1.
    pub fn degree(self) -> u32 {
        match self {
            Base::Mono(i, j) => i + j,
            Base::Abs(_) => 1,
        }
    }
}

/// One term `c · base` with `c ≠ 0` in a normal form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ITerm {
    pub c: i64,
    pub base: Base,
}

impl ITerm {
    pub const fn mono(c: i64, i: u32, j: u32) -> ITerm {
        ITerm {
            c,
            base: Base::Mono(i, j),
        }
    }

    pub const fn abs(c: i64, f: AbsForm) -> ITerm {
        ITerm {
            c,
            base: Base::Abs(f),
        }
    }

    pub fn is_constant(self) -> bool {
        self.base == Base::Mono(0, 0)
    }
}

/// An expression: its terms, in normal form unless a FLAW-0001 site keeps a perturbed term in
/// place.
pub type Expr = Vec<ITerm>;

/// Merge equal bases, drop zero coefficients, and order the terms (see the module docs).
pub fn normalize(terms: &[ITerm]) -> Expr {
    let mut out: Expr = Vec::new();
    for t in terms {
        match out.iter_mut().find(|u| u.base == t.base) {
            Some(u) => u.c += t.c,
            None => out.push(*t),
        }
    }
    out.retain(|t| t.c != 0);
    out.sort_by_key(|t| t.base.order());
    out
}

/// The normal form of `a + b`.
pub fn add(a: &[ITerm], b: &[ITerm]) -> Expr {
    let mut all = a.to_vec();
    all.extend_from_slice(b);
    normalize(&all)
}

fn term_lean(t: ITerm) -> String {
    let abs = t.c.unsigned_abs();
    match t.base {
        Base::Mono(i, j) => mono_lean(Term::new(t.c, i, j), Style::default()),
        Base::Abs(f) if abs == 1 => f.text().to_string(),
        Base::Abs(f) => format!("{abs} * {}", f.text()),
    }
}

fn term_latex(t: ITerm) -> String {
    let abs = t.c.unsigned_abs();
    match t.base {
        Base::Mono(i, j) => mono_latex(Term::new(t.c, i, j), Style::default()),
        Base::Abs(f) if abs == 1 => f.text().to_string(),
        Base::Abs(f) => format!("{abs}{}", f.text()),
    }
}

/// The expression in `syndeom-lean/1`: `3 * x ^ 2 - 2 * x + 1 + |x| + 2 * |x + y|`; `0` when
/// empty.
pub fn lean(e: &[ITerm]) -> String {
    if e.is_empty() {
        return "0".to_string();
    }
    let parts: Vec<(bool, String)> = e.iter().map(|&t| (t.c < 0, term_lean(t))).collect();
    sum_join(&parts)
}

/// The expression in LaTeX: `3x^2 - 2x + 1 + |x| + 2|x + y|`; `0` when empty.
pub fn latex(e: &[ITerm]) -> String {
    if e.is_empty() {
        return "0".to_string();
    }
    let parts: Vec<(bool, String)> = e.iter().map(|&t| (t.c < 0, term_latex(t))).collect();
    sum_join(&parts)
}

/// One term with its sign, in `syndeom-lean/1` (a FLAW-0001 record's `monomial` dial).
pub fn signed_lean(t: ITerm) -> String {
    sum_join(&[(t.c < 0, term_lean(t))])
}

// ---- the atom library ---------------------------------------------------------------------------

/// The three atom families of D6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// Squares are non-negative.
    Square,
    /// The two-term AM–GM inequality, `2uv ≤ u^2 + v^2`.
    AmGm,
    /// The triangle inequality, `|u + v| ≤ |u| + |v|`.
    Triangle,
}

/// One atom: a true inequality, how the informal file states it, the Mathlib lemma instance
/// `linarith` takes as its one hint, what it adds to a chain expression (its larger side minus its
/// smaller side, merged), and what the statement's left side carries for it to consume (empty for
/// a square, which adds a term without consuming one).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Atom {
    pub id: &'static str,
    pub family: Family,
    pub fact: &'static str,
    pub hint: &'static str,
    pub increment: &'static [ITerm],
    pub source: &'static [ITerm],
}

const fn m(c: i64, i: u32, j: u32) -> ITerm {
    ITerm::mono(c, i, j)
}

const fn a(c: i64, f: AbsForm) -> ITerm {
    ITerm::abs(c, f)
}

/// The atom library (D6).
pub const ATOMS: &[Atom] = &[
    Atom {
        id: "sq_x_sub_y",
        family: Family::Square,
        fact: "(x - y)^2 \\ge 0",
        hint: "sq_nonneg (x - y)",
        increment: &[m(1, 2, 0), m(-2, 1, 1), m(1, 0, 2)],
        source: &[],
    },
    Atom {
        id: "sq_x_add_y",
        family: Family::Square,
        fact: "(x + y)^2 \\ge 0",
        hint: "sq_nonneg (x + y)",
        increment: &[m(1, 2, 0), m(2, 1, 1), m(1, 0, 2)],
        source: &[],
    },
    Atom {
        id: "sq_x_sub_1",
        family: Family::Square,
        fact: "(x - 1)^2 \\ge 0",
        hint: "sq_nonneg (x - 1)",
        increment: &[m(1, 2, 0), m(-2, 1, 0), m(1, 0, 0)],
        source: &[],
    },
    Atom {
        id: "sq_y_sub_1",
        family: Family::Square,
        fact: "(y - 1)^2 \\ge 0",
        hint: "sq_nonneg (y - 1)",
        increment: &[m(1, 0, 2), m(-2, 0, 1), m(1, 0, 0)],
        source: &[],
    },
    Atom {
        id: "sq_x_add_1",
        family: Family::Square,
        fact: "(x + 1)^2 \\ge 0",
        hint: "sq_nonneg (x + 1)",
        increment: &[m(1, 2, 0), m(2, 1, 0), m(1, 0, 0)],
        source: &[],
    },
    Atom {
        id: "sq_y_add_1",
        family: Family::Square,
        fact: "(y + 1)^2 \\ge 0",
        hint: "sq_nonneg (y + 1)",
        increment: &[m(1, 0, 2), m(2, 0, 1), m(1, 0, 0)],
        source: &[],
    },
    Atom {
        id: "amgm_x_y",
        family: Family::AmGm,
        fact: "2xy \\le x^2 + y^2",
        hint: "two_mul_le_add_sq x y",
        increment: &[m(1, 2, 0), m(-2, 1, 1), m(1, 0, 2)],
        source: &[m(2, 1, 1)],
    },
    Atom {
        id: "amgm_x_1",
        family: Family::AmGm,
        fact: "2x \\le x^2 + 1",
        hint: "two_mul_le_add_sq x 1",
        increment: &[m(1, 2, 0), m(-2, 1, 0), m(1, 0, 0)],
        source: &[m(2, 1, 0)],
    },
    Atom {
        id: "amgm_y_1",
        family: Family::AmGm,
        fact: "2y \\le y^2 + 1",
        hint: "two_mul_le_add_sq y 1",
        increment: &[m(1, 0, 2), m(-2, 0, 1), m(1, 0, 0)],
        source: &[m(2, 0, 1)],
    },
    Atom {
        id: "tri_x_y",
        family: Family::Triangle,
        fact: "|x + y| \\le |x| + |y|",
        hint: "abs_add_le x y",
        increment: &[a(1, AbsForm::X), a(1, AbsForm::Y), a(-1, AbsForm::XPlusY)],
        source: &[a(1, AbsForm::XPlusY)],
    },
];

/// The reason a faithful chain step gives: its atom, named and stated.
pub fn reason(atom: &Atom) -> String {
    match atom.family {
        Family::Square => format!("Since squares are non-negative, ${}$", atom.fact),
        Family::AmGm => format!("By the two-term AM-GM inequality, ${}$", atom.fact),
        Family::Triangle => format!("By the triangle inequality, ${}$", atom.fact),
    }
}

/// A lemma step's closing note (D2): its atom, named and stated.
pub fn lemma_note(atom: &Atom) -> String {
    match atom.family {
        Family::Square => format!(
            "it follows from ${}$, since squares are non-negative",
            atom.fact
        ),
        Family::AmGm => format!(
            "it follows from the two-term AM-GM inequality ${}$",
            atom.fact
        ),
        Family::Triangle => format!("it follows from the triangle inequality ${}$", atom.fact),
    }
}

// ---- the derivation -----------------------------------------------------------------------------

/// An `ineq_chain` instance before it is a plan: the atom each chain step adds, the expressions
/// `e_0..e_n` (`e_0` the sum of the steps' sources), and the chain steps the lemmas state
/// (`sites[j - 1]` is the step lemma `j` states and the step that cites it; ascending).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derivation {
    pub atoms: Vec<usize>,
    pub e: Vec<Expr>,
    pub sites: Vec<usize>,
}

impl Derivation {
    pub fn n(&self) -> usize {
        self.e.len() - 1
    }

    /// The lemma number (1-based) a chain step cites, if any.
    pub fn lemma_of(&self, c: usize) -> Option<usize> {
        self.sites.iter().position(|&s| s == c).map(|j| j + 1)
    }
}

/// The label of the stream that draws chain step `k`'s atom.
pub fn step_label(k: usize) -> String {
    format!("grammar/ineq_chain/step/{k}")
}

/// The label of the stream that draws the step lemma `j` states.
pub fn lemma_label(j: usize) -> String {
    format!("grammar/ineq_chain/lemma/{j}")
}

/// Derive an `ineq_chain` instance of `n` steps and `m` lemmas from `seed` (D4: one draw on
/// `grammar/ineq_chain/step/<k>` for step k's atom; one on `grammar/ineq_chain/lemma/<j>` for the
/// step lemma j states, among the steps not yet taken).
pub fn derive(n: usize, m: usize, seed: u64) -> Derivation {
    let atoms: Vec<usize> = (1..=n)
        .map(|k| Stream::new(seed, &step_label(k)).below(ATOMS.len() as u64) as usize)
        .collect();
    let mut cur: Expr = Vec::new();
    for &at in &atoms {
        cur = add(&cur, ATOMS[at].source);
    }
    let mut e = vec![cur.clone()];
    for &at in &atoms {
        cur = add(&cur, ATOMS[at].increment);
        e.push(cur.clone());
    }
    let mut remaining: Vec<usize> = (1..=n).collect();
    let mut sites = Vec::new();
    for j in 1..=m {
        let mut s = Stream::new(seed, &lemma_label(j));
        let pick = s.below(remaining.len() as u64) as usize;
        sites.push(remaining.remove(pick));
    }
    sites.sort_unstable();
    Derivation { atoms, e, sites }
}

fn lemma_name(j: usize) -> String {
    format!("{MAIN_DECLARATION}_lemma_{j}")
}

/// A faithful chain step's text: the reason, then the step's inequality.
fn chain_text(atom: &Atom, a: &[ITerm], b: &[ITerm]) -> String {
    format!("{}, so ${} \\le {}$.", reason(atom), latex(a), latex(b))
}

/// The lemma step stating chain step `c`'s inequality (D2: a step whose text begins `Lemma j.`,
/// declared as a `theorem` whose claim is closed, proved by `intro x y` and the step's `linarith`).
fn lemma_step(j: usize, d: &Derivation, c: usize) -> Step {
    let (a, b) = (&d.e[c - 1], &d.e[c]);
    let atom = &ATOMS[d.atoms[c - 1]];
    Step {
        text: format!(
            "Lemma {j}. For all real numbers $x$ and $y$, ${} \\le {}$; {}.",
            latex(a),
            latex(b),
            lemma_note(atom)
        ),
        fragment: vec!["intro x y".to_string(), format!("linarith [{}]", atom.hint)],
        lemma: Some(LemmaHeader {
            kind: LemmaKind::Theorem,
            name: lemma_name(j),
            claim: format!("∀ x y : ℝ, {} ≤ {}", lean(a), lean(b)),
        }),
        padding: None,
        premises: Vec::new(),
    }
}

/// The `calc` line of chain step `c` — the first opens the block — with its proof term.
fn chain_line(c: usize, a: &[ITerm], b: &[ITerm], proof: &str) -> String {
    if c == 1 {
        format!("calc {} ≤ {} := {proof}", lean(a), lean(b))
    } else {
        format!("  _ ≤ {} := {proof}", lean(b))
    }
}

/// Chain step `c` of the main proof: one `calc` line closed by `linarith` with the atom's hint,
/// or by the lemma that states it, which it cites.
fn chain_step(d: &Derivation, c: usize) -> Step {
    let (a, b) = (&d.e[c - 1], &d.e[c]);
    let atom = &ATOMS[d.atoms[c - 1]];
    let (text, proof, premises) = match d.lemma_of(c) {
        Some(j) => (
            format!("By Lemma {j}, ${} \\le {}$.", latex(a), latex(b)),
            format!("{} x y", lemma_name(j)),
            vec![Premise {
                cites: lemma_name(j),
                phrase: format!("Lemma {j}"),
            }],
        ),
        None => (
            chain_text(atom, a, b),
            format!("by linarith [{}]", atom.hint),
            Vec::new(),
        ),
    };
    Step {
        text,
        fragment: vec![chain_line(c, a, b, &proof)],
        lemma: None,
        padding: None,
        premises,
    }
}

/// The plan of a derivation (D2): the lemma steps first, in their order, then the chain.
pub fn plan_of(d: &Derivation, truth: StatementStatus) -> Plan {
    let n = d.n();
    let mut steps: Vec<Step> = d
        .sites
        .iter()
        .enumerate()
        .map(|(j, &c)| lemma_step(j + 1, d, c))
        .collect();
    steps.extend((1..=n).map(|c| chain_step(d, c)));
    Plan {
        title: "An inequality in two real variables".to_string(),
        statement: format!(
            "For all real numbers $x$ and $y$, ${} \\le {}$.",
            latex(&d.e[0]),
            latex(&d.e[n])
        ),
        imports: IMPORTS.iter().map(|s| s.to_string()).collect(),
        doc: None,
        main_declaration: MAIN_DECLARATION.to_string(),
        binders: "(x y : ℝ)".to_string(),
        claim: format!("{} ≤ {}", lean(&d.e[0]), lean(&d.e[n])),
        truth,
        steps,
    }
}

/// `plan` → `inject` → `render` for one `ineq_chain` config under `version` (0.8.0 on), after
/// `grammar::prepare_with`'s shared checks; the one class realized is FLAW-0001.
pub(crate) fn prepare(
    version: GrammarVersion,
    cfg: &Config,
    config: &Value,
    config_hash: String,
    seed: u64,
) -> Result<Prepared, String> {
    let d = derive(cfg.n_steps as usize, cfg.n_lemmas as usize, seed);
    let truth = StatementStatus {
        value: Some(true),
        method: StatementMethod::LeanKernel,
        reference: control_reference(version, INEQ_CHAIN, cfg.n_steps, cfg.n_lemmas, seed)?,
    };
    let faithful = plan_of(&d, truth);
    let (plan, injection) = match cfg.flaws.first() {
        None => (faithful, None),
        Some(spec) if spec.class == "FLAW-0001" => {
            let (plan, inj) = inject(&d, &faithful, spec, cfg.target, seed)?;
            (plan, Some(inj))
        }
        Some(spec) => {
            return Err(format!(
                "{} is not realized on {INEQ_CHAIN} by s2_grammar {}",
                spec.class,
                version.as_str()
            ));
        }
    };
    let informal = cfg
        .target
        .informal()
        .then(|| render_informal(&plan))
        .transpose()?;
    let formal = cfg
        .target
        .formal()
        .then(|| render_formal(&plan))
        .transpose()?;
    Ok(Prepared {
        producer: Producer::Grammar(version),
        config: config.clone(),
        config_hash,
        seed,
        target: cfg.target,
        plan,
        informal,
        formal,
        injection,
        further: Vec::new(),
        padding: Vec::new(),
        composite: None,
        false_theorem: None,
    })
}

// ---- FLAW-0001 ---------------------------------------------------------------------------------

/// The candidate targets of a perturbation kind in a site's terms, in position order (the module
/// docs; `ring_chain`'s operations, ADR-0011 D5): every term for `sign`; every non-constant term
/// for `coefficient`; the constant term, or the addition of one, for `constant`; every (monomial,
/// variable) with a positive exponent whose raised monomial is not another term's, for `exponent`.
pub fn targets(kind: &str, terms: &[ITerm]) -> Vec<Perturbation> {
    let mut out = Vec::new();
    match kind {
        "sign" => out.extend((0..terms.len()).map(Perturbation::Sign)),
        "coefficient" => out.extend(
            terms
                .iter()
                .enumerate()
                .filter(|(_, t)| !t.is_constant())
                .map(|(p, _)| Perturbation::Coefficient(p)),
        ),
        "constant" => match terms.iter().position(|t| t.is_constant()) {
            Some(p) => out.push(Perturbation::Constant(p)),
            None => out.push(Perturbation::ConstantAdd),
        },
        "exponent" => {
            for (p, t) in terms.iter().enumerate() {
                let Base::Mono(i, j) = t.base else {
                    continue;
                };
                for var in 0..2u8 {
                    let (has, raised) = if var == 0 {
                        (i >= 1, Base::Mono(i + 1, j))
                    } else {
                        (j >= 1, Base::Mono(i, j + 1))
                    };
                    if has && !terms.iter().any(|u| u.base == raised) {
                        out.push(Perturbation::Exponent(p, var));
                    }
                }
            }
        }
        _ => {}
    }
    out
}

/// Apply a target in place: the perturbed terms, and the term replaced (none for an added
/// constant) and inserted, as signed `syndeom-lean/1` text.
pub fn apply(target: Perturbation, terms: &[ITerm]) -> (Vec<ITerm>, Option<String>, String) {
    let mut out = terms.to_vec();
    match target {
        Perturbation::Sign(p) => {
            let before = out[p];
            out[p].c = -out[p].c;
            let after = out[p];
            (out, Some(signed_lean(before)), signed_lean(after))
        }
        Perturbation::Coefficient(p) | Perturbation::Constant(p) => {
            let before = out[p];
            out[p].c += before.c.signum();
            let after = out[p];
            (out, Some(signed_lean(before)), signed_lean(after))
        }
        Perturbation::ConstantAdd => {
            let added = ITerm::mono(1, 0, 0);
            out.push(added);
            (out, None, signed_lean(added))
        }
        Perturbation::Exponent(p, var) => {
            let before = out[p];
            if let Base::Mono(i, j) = before.base {
                out[p].base = if var == 0 {
                    Base::Mono(i + 1, j)
                } else {
                    Base::Mono(i, j + 1)
                };
            }
            let after = out[p];
            (out, Some(signed_lean(before)), signed_lean(after))
        }
    }
}

/// A point of the witness grid at which a step's left side exceeds its right side, with the two
/// values; every number an exact fraction in lowest terms (`"1/2"`, `"-1"`, `"3/4"`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness {
    pub x: String,
    pub y: String,
    pub left: String,
    pub right: String,
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// `num / den` in lowest terms, `den > 0`: `"n"` or `"n/d"`.
pub fn fraction(num: i128, den: i128) -> String {
    let g = gcd(num.unsigned_abs(), den.unsigned_abs()).max(1);
    let (n, d) = (num / g as i128, den / g as i128);
    let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
    if d == 1 {
        n.to_string()
    } else {
        format!("{n}/{d}")
    }
}

fn degree(e: &[ITerm]) -> u32 {
    e.iter().map(|t| t.base.degree()).max().unwrap_or(0)
}

/// `2^d · e(x2 / 2, y2 / 2)`, exactly, for `d` at least the largest degree in `e` and at least 1.
fn scaled(e: &[ITerm], x2: i64, y2: i64, d: u32) -> i128 {
    e.iter()
        .map(|t| {
            let c = i128::from(t.c);
            match t.base {
                Base::Mono(i, j) => {
                    c * i128::from(x2).pow(i) * i128::from(y2).pow(j) * (1i128 << (d - i - j))
                }
                Base::Abs(f) => c * i128::from(f.doubled(x2, y2)) * (1i128 << (d - 1)),
            }
        })
        .sum()
}

/// The witness grid's points `(X, Y)` (`x = X / 2`, `y = Y / 2`), in search order.
pub fn grid() -> Vec<(i64, i64)> {
    let b = WITNESS_HALF_BOUND;
    let mut pts: Vec<(i64, i64)> = (-b..=b)
        .flat_map(|x| (-b..=b).map(move |y| (x, y)))
        .collect();
    pts.sort_by_key(|&(x, y)| (x.abs() + y.abs(), x, y));
    pts
}

/// The first grid point at which `left` exceeds `right` — so the inequality `left ≤ right` is
/// false there — or `None` when there is none on the grid.
pub fn witness(left: &[ITerm], right: &[ITerm]) -> Option<Witness> {
    let d = degree(left).max(degree(right)).max(1);
    grid().into_iter().find_map(|(x2, y2)| {
        let (l, r) = (scaled(left, x2, y2, d), scaled(right, x2, y2, d));
        (l > r).then(|| Witness {
            x: fraction(i128::from(x2), 2),
            y: fraction(i128::from(y2), 2),
            left: fraction(l, 1i128 << d),
            right: fraction(r, 1i128 << d),
        })
    })
}

/// An admitted perturbation at a site: the target, the perturbed terms in place, the term replaced
/// and inserted, and the witness that the perturbed step is false.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub target: Perturbation,
    pub terms: Vec<ITerm>,
    pub replaced: Option<String>,
    pub inserted: String,
    pub witness: Witness,
}

/// The admitted targets of `kind` at chain step `c`: those whose perturbed step `e_{c-1} ≤ e_c'`
/// is false at a grid point, and whose perturbed expression occurs exactly once in the site's text
/// (so that the informal record's span is unique), in position order.
pub fn site_candidates(kind: &str, d: &Derivation, c: usize) -> Vec<Candidate> {
    let left = &d.e[c - 1];
    let atom = &ATOMS[d.atoms[c - 1]];
    targets(kind, &d.e[c])
        .into_iter()
        .filter_map(|target| {
            let (terms, replaced, inserted) = apply(target, &d.e[c]);
            let witness = witness(left, &terms)?;
            unique_span(&chain_text(atom, left, &terms), &latex(&terms))?;
            Some(Candidate {
                target,
                terms,
                replaced,
                inserted,
                witness,
            })
        })
        .collect()
}

/// The injector for FLAW-0001 on `ineq_chain` (ADR-0011 D5; the module docs): the site and the
/// target drawn on `injector/FLAW-0001/1` (the site only when the spec says `auto`) among the
/// admitted ones; the site's term perturbed in place; `propagated` adding every later step's
/// increment to the perturbed expression, `local` changing the site alone. The record's dials
/// carry the perturbation, the site (a plan step index), the term replaced and inserted
/// (`monomial`, as on `ring_chain`), the witness, the site's text or fragment replaced and
/// inserted, and every original text or fragment the variant changed.
fn inject(
    d: &Derivation,
    faithful: &Plan,
    spec: &FlawSpec,
    target: Target,
    seed: u64,
) -> Result<(Plan, Injection), String> {
    check_spec(spec, target)?;
    let propagated = match spec.sub_variant.as_deref() {
        Some("propagated") => true,
        Some("local") => false,
        _ => return Err("FLAW-0001's sub_variant is \"propagated\" or \"local\"".into()),
    };
    let sub = spec.sub_variant.as_deref().unwrap_or_default();
    let expected = if propagated {
        Severity::Fatal
    } else {
        Severity::Repairable
    };
    if spec.severity != expected {
        return Err(format!(
            "FLAW-0001 {sub} is {expected:?} (ADR-0011 D5), not {:?}",
            spec.severity
        ));
    }
    exact_keys(&spec.dials, &["perturbation"], "FLAW-0001's dials")?;
    let kind = spec.dials["perturbation"]
        .as_str()
        .filter(|k| PERTURBATIONS.contains(k))
        .ok_or(format!(
            "FLAW-0001's perturbation is one of {PERTURBATIONS:?}"
        ))?;
    let n = d.n();
    let m = d.sites.len();
    let last = if propagated { "" } else { ", not the last" };
    let steps: Vec<usize> = (1..=n)
        .filter(|&c| d.lemma_of(c).is_none() && (propagated || c < n))
        .collect();
    if steps.is_empty() {
        return Err(format!(
            "no eligible site for FLAW-0001 {sub}: it needs a linarith-closed chain step{}",
            if propagated { "" } else { " before the last" }
        ));
    }
    let admits = |c: usize| !site_candidates(kind, d, c).is_empty();
    let mut stream = Stream::new(seed, "injector/FLAW-0001/1");
    let c = match spec.site {
        None => {
            let eligible: Vec<usize> = steps.iter().copied().filter(|&c| admits(c)).collect();
            if eligible.is_empty() {
                return Err(format!(
                    "no eligible {kind} perturbation for FLAW-0001 {sub}: at every linarith-closed chain step{last} each candidate leaves the step's inequality true"
                ));
            }
            eligible[stream.below(eligible.len() as u64) as usize]
        }
        Some(k) => {
            let c = (k as usize).saturating_sub(m);
            if !steps.contains(&c) {
                return Err(format!(
                    "site {k} is not an eligible step: FLAW-0001 {sub} sits at a linarith-closed chain step (plan steps {} to {}{last})",
                    m + 1,
                    m + n
                ));
            }
            if !admits(c) {
                return Err(format!(
                    "no eligible {kind} perturbation at step {k}: each candidate leaves the step's inequality true"
                ));
            }
            c
        }
    };
    let candidates = site_candidates(kind, d, c);
    let chosen = &candidates[stream.below(candidates.len() as u64) as usize];

    // The flawed derivation: the site's term perturbed in place; under `propagated`, every later
    // step's increment added to the perturbed expression.
    let mut flawed = d.clone();
    flawed.e[c] = chosen.terms.clone();
    if propagated {
        for k in c + 1..=n {
            flawed.e[k] = add(&flawed.e[k - 1], ATOMS[d.atoms[k - 1]].increment);
        }
    }
    let plan = if propagated {
        // step 0 is untouched (D5): the claim of record stays the faithful one, so the chain ends
        // away from it
        let mut plan = plan_of(&flawed, faithful.truth.clone());
        plan.claim = faithful.claim.clone();
        plan.statement = faithful.statement.clone();
        plan
    } else {
        let mut plan = faithful.clone();
        plan.steps[m + c - 1] = chain_step(&flawed, c);
        plan
    };
    let site = m + c;
    let site_step = &plan.steps[site - 1];
    let faithful_site = &faithful.steps[site - 1];
    let inserted = latex(&flawed.e[c]);
    if unique_span(&site_step.text, &inserted).is_none() {
        return Err("the perturbed expression occurs exactly once in the site step".into());
    }
    let mut original_steps = Map::new();
    let mut original_fragments = Map::new();
    for (k, (f, s)) in faithful.steps.iter().zip(&plan.steps).enumerate() {
        let key = (k + 1).to_string();
        if s.text != f.text {
            original_steps.insert(key.clone(), json!(f.text));
        }
        if s.fragment != f.fragment || s.lemma != f.lemma {
            let lines = match &f.lemma {
                Some(_) => lemma_lines(f)?,
                None => f.fragment.clone(),
            };
            original_fragments.insert(key, json!(lines));
        }
    }
    let w = &chosen.witness;
    let with = |extra: Vec<(&str, Value)>| {
        let mut dials = spec.dials.clone();
        dials.insert("site".into(), json!(site));
        dials.insert(
            "monomial".into(),
            json!({"replaced": chosen.replaced, "inserted": chosen.inserted}),
        );
        dials.insert(
            "witness".into(),
            json!({"x": w.x, "y": w.y, "left": w.left, "right": w.right}),
        );
        for (k, v) in extra {
            dials.insert(k.into(), v);
        }
        Value::Object(dials)
    };
    let informal_dials = with(vec![
        ("inserted", json!(inserted)),
        ("replaced", json!(latex(&d.e[c]))),
        ("original_steps", Value::Object(original_steps)),
    ]);
    let formal_dials = with(vec![
        ("inserted", json!(site_step.fragment)),
        ("replaced", json!(faithful_site.fragment)),
        ("original_fragments", Value::Object(original_fragments)),
    ]);
    let injection = Injection {
        instance: 1,
        class: "FLAW-0001".to_string(),
        sub_variant: spec.sub_variant.clone(),
        severity: spec.severity,
        lean_catchable: LeanCatchable::YesKernel,
        site,
        informal_inserted: inserted,
        informal_dials,
        formal_inserted: site_step.fragment.clone(),
        formal_dials,
    };
    Ok((plan, injection))
}

/// The plan step index (1-based) of chain step `c` in an instance with `m` lemmas.
pub fn plan_index(m: usize, c: usize) -> usize {
    m + c
}
