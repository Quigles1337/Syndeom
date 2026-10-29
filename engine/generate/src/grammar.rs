//! `s2_grammar` (ADR-0011 D1, D6): derivation grammars producing arguments of commanded length,
//! true by construction. 0.1.0 carries one grammar, `ring_chain` — a polynomial identity over ℤ
//! in two variables, proved as a chain of `length.n_steps` equalities inside a `calc` block, each
//! closed by `ring` or by one of `length.n_lemmas` auxiliary identities proved separately as
//! lemma steps (D2) and cited — with its control and FLAW-0001 (`propagated`, `local`;
//! `perturbation` ∈ {`sign`, `coefficient`, `constant`, `exponent`}). The plan is rendered by the
//! existing renderers and measured by the existing `assemble` (D2, D8); the producer differs
//! from `s2_template` in `plan` and in how the injector finds its material, which is generated
//! rather than hand-authored.
//!
//! **The atom library** (D6: "the monomials, coefficients and rewrites the chain is built from")
//! is the fixed table [`SEED_FORMS`] — products and powers of small binomials and trinomials in
//! `x` and `y` with coefficients in {1, 2, 3, 5} and constants in {1, 2, 3, 5} — and the
//! presentations of [`present`]: a polynomial (a canonical list of monomials) rendered as its
//! terms in a drawn order with drawn monomial styles (`x ^ 2` or `x * x`; `x * y` or `y * x`),
//! either expanded, or with one variable factored out of the terms it divides, or regrouped into
//! two parenthesized sums. Every presentation of one polynomial equals every other, so each
//! `calc` step is a true identity that `ring` closes; the chain's steps are drawn one labelled
//! stream each, so a step is a function of the seed alone (D4). Its `source` is this module —
//! executor-authored synthetic material outside the S1 corpus (ADR-0011 D6, Al's ruling 1 of
//! 2026-10-27) — and nothing here is derived from `groundtruth/`.
//!
//! **Ground truth (D7).** A grammar instance's `statement_status` is `value: true`,
//! `method: lean-kernel`, `reference: <artifact_id of the instance's control>:<main
//! declaration>` — the control with target `formal` of the same grammar, length and seed, whose
//! `lean_build: pass` is measured and recorded in its own manifest (D8).
//!
//! **FLAW-0001 (D5).** One ring-closed step's expression is perturbed once — one monomial's
//! sign flipped, one coefficient or the constant term raised by one (a constant term added
//! when there is none), or one exponent raised by one — so that the step's identity is false
//! and `ring` fails there (check K). `propagated` re-derives every later step, and every lemma
//! a later step cites, from the perturbed polynomial, so that the chain's final line differs
//! from the claim of record while every later step is again a true identity; `local` leaves
//! every other step faithful, so that the next step asserts the correct consequence. The
//! RNG labels this producer draws from are `grammar/plan` (the seed form), `grammar/step/<k>`
//! (step k's presentation), `grammar/lemma/<j>` (the step lemma j states) and
//! `injector/FLAW-0001/1` (the site and the perturbed monomial).

use std::cmp::Reverse;
use std::collections::BTreeMap;

use serde_json::{Map, Value, json};
use syndeom_manifest::{
    LeanCatchable, Severity, StatementMethod, StatementStatus, artifact_id, config_hash,
};

use crate::plan::{
    LemmaHeader, LemmaKind, Plan, Premise, Step, lemma_lines, render_formal, render_informal,
};
use crate::rng::Stream;
use crate::template::{
    FlawSpec, Injection, Prepared, Producer, Target, check_spec, exact_keys, parse_spec,
    unique_span,
};

pub const GENERATOR_ID: &str = "s2_grammar";
/// The producer version `generate` emits (ADR-0011 D1).
pub const GENERATOR_VERSION: &str = "0.1.0";
/// The one grammar of 0.1.0 (ADR-0011 D6).
pub const RING_CHAIN: &str = "ring_chain";
/// The main declaration's name (ADR-0011 D2: `syndeom_s2_<producer>_<theorem>`).
pub const MAIN_DECLARATION: &str = "syndeom_s2_grammar_ring_chain";
/// The one import every `ring_chain` file needs (D6: the minimal `Mathlib.*` modules).
pub const IMPORTS: &[&str] = &["Mathlib.Tactic.Ring"];
/// The length dials' ranges in v0.1 (ADR-0011 D6).
pub const N_STEPS_RANGE: (u64, u64) = (1, 256);
pub const N_LEMMAS_RANGE: (u64, u64) = (0, 32);
/// FLAW-0001's `perturbation` dial (ADR-0011 D5).
pub const PERTURBATIONS: &[&str] = &["sign", "coefficient", "constant", "exponent"];
/// Where the material comes from (ADR-0011 D6, Al's ruling 1 of 2026-10-27).
pub const SOURCE: &str = "engine/generate/src/grammar.rs, s2_grammar 0.1.0: the ring_chain atom library (SEED_FORMS and the presentations of `present`), executor-authored synthetic material outside the S1 corpus (ADR-0011 D6)";

/// A producer version this crate regenerates for `s2_grammar`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrammarVersion {
    V0_1_0,
}

impl GrammarVersion {
    pub const CURRENT: GrammarVersion = GrammarVersion::V0_1_0;

    pub fn parse(s: &str) -> Option<GrammarVersion> {
        match s {
            "0.1.0" => Some(GrammarVersion::V0_1_0),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            GrammarVersion::V0_1_0 => "0.1.0",
        }
    }
}

// ---- polynomials -------------------------------------------------------------------------------

/// A monomial `c · x^i · y^j` with `c ≠ 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Term {
    pub c: i64,
    pub i: u32,
    pub j: u32,
}

impl Term {
    pub const fn new(c: i64, i: u32, j: u32) -> Term {
        Term { c, i, j }
    }

    pub fn degree(self) -> u32 {
        self.i + self.j
    }

    fn same_monomial(self, o: Term) -> bool {
        self.i == o.i && self.j == o.j
    }
}

/// A polynomial in canonical form: monomials merged, zero coefficients dropped, sorted by total
/// degree descending, then by the exponent of `x` descending.
pub type Poly = Vec<Term>;

pub fn normalize(terms: &[Term]) -> Poly {
    let mut out: Poly = Vec::new();
    for t in terms {
        match out.iter_mut().find(|u| u.same_monomial(*t)) {
            Some(u) => u.c += t.c,
            None => out.push(*t),
        }
    }
    out.retain(|t| t.c != 0);
    out.sort_by_key(|t| (Reverse(t.degree()), Reverse(t.i)));
    out
}

pub fn poly_mul(a: &[Term], b: &[Term]) -> Poly {
    let mut out = Vec::new();
    for s in a {
        for t in b {
            out.push(Term::new(s.c * t.c, s.i + t.i, s.j + t.j));
        }
    }
    normalize(&out)
}

pub fn poly_pow(a: &[Term], k: u32) -> Poly {
    let mut out = vec![Term::new(1, 0, 0)];
    for _ in 0..k {
        out = poly_mul(&out, a);
    }
    out
}

// ---- the atom library: seed forms ---------------------------------------------------------------

/// The shape of a seed form: a power of one sum, or a product of two.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedKind {
    Pow(u32),
    Product,
}

/// A seed form `e_0`: the factored expression the chain starts from, whose expansion is the
/// polynomial every later step presents.
#[derive(Debug, Clone, Copy)]
pub struct SeedForm {
    pub kind: SeedKind,
    pub factors: &'static [&'static [Term]],
}

const fn t(c: i64, i: u32, j: u32) -> Term {
    Term::new(c, i, j)
}

/// The seed forms (D6). Every form uses both `x` and `y`; every expansion has at least two
/// monomials.
pub const SEED_FORMS: &[SeedForm] = &[
    SeedForm {
        kind: SeedKind::Pow(2),
        factors: &[&[t(1, 1, 0), t(1, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Pow(2),
        factors: &[&[t(1, 1, 0), t(-1, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Product,
        factors: &[&[t(1, 1, 0), t(1, 0, 1)], &[t(1, 1, 0), t(-1, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Pow(2),
        factors: &[&[t(2, 1, 0), t(1, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Pow(2),
        factors: &[&[t(1, 1, 0), t(-2, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Product,
        factors: &[&[t(1, 1, 0), t(2, 0, 1)], &[t(1, 1, 0), t(-1, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Pow(3),
        factors: &[&[t(1, 1, 0), t(1, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Pow(3),
        factors: &[&[t(1, 1, 0), t(-1, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Product,
        factors: &[&[t(3, 1, 0), t(-1, 0, 1)], &[t(1, 1, 0), t(2, 0, 0)]],
    },
    SeedForm {
        kind: SeedKind::Product,
        factors: &[&[t(1, 1, 0), t(3, 0, 0)], &[t(1, 0, 1), t(-1, 0, 0)]],
    },
    SeedForm {
        kind: SeedKind::Pow(2),
        factors: &[&[t(1, 1, 1), t(1, 0, 0)]],
    },
    SeedForm {
        kind: SeedKind::Pow(2),
        factors: &[&[t(1, 1, 0), t(1, 0, 1), t(1, 0, 0)]],
    },
    SeedForm {
        kind: SeedKind::Product,
        factors: &[&[t(2, 1, 0), t(-3, 0, 1)], &[t(2, 1, 0), t(3, 0, 1)]],
    },
    SeedForm {
        kind: SeedKind::Product,
        factors: &[&[t(1, 1, 0), t(-2, 0, 0)], &[t(1, 0, 1), t(5, 0, 0)]],
    },
];

impl SeedForm {
    /// The expansion.
    pub fn poly(&self) -> Poly {
        match self.kind {
            SeedKind::Pow(k) => poly_pow(self.factors[0], k),
            SeedKind::Product => poly_mul(self.factors[0], self.factors[1]),
        }
    }

    fn sum_lean(terms: &[Term]) -> String {
        sum_join(
            &terms
                .iter()
                .map(|&u| (u.c < 0, mono_lean(u, Style::default())))
                .collect::<Vec<_>>(),
        )
    }

    fn sum_latex(terms: &[Term]) -> String {
        sum_join(
            &terms
                .iter()
                .map(|&u| (u.c < 0, mono_latex(u, Style::default())))
                .collect::<Vec<_>>(),
        )
    }

    /// `e_0` in `syndeom-lean/1`: `(x + y) ^ 2`, `(x + y) * (x - y)`.
    pub fn lean(&self) -> String {
        match self.kind {
            SeedKind::Pow(k) => format!("({}) ^ {k}", Self::sum_lean(self.factors[0])),
            SeedKind::Product => format!(
                "({}) * ({})",
                Self::sum_lean(self.factors[0]),
                Self::sum_lean(self.factors[1])
            ),
        }
    }

    /// `e_0` in LaTeX: `(x + y)^2`, `(x + y)(x - y)`.
    pub fn latex(&self) -> String {
        match self.kind {
            SeedKind::Pow(k) => format!("({})^{k}", Self::sum_latex(self.factors[0])),
            SeedKind::Product => format!(
                "({})({})",
                Self::sum_latex(self.factors[0]),
                Self::sum_latex(self.factors[1])
            ),
        }
    }

    fn expanding(&self) -> &'static str {
        match self.kind {
            SeedKind::Pow(2) => "Expanding the square",
            SeedKind::Pow(3) => "Expanding the cube",
            SeedKind::Pow(_) => "Expanding the power",
            SeedKind::Product => "Expanding the product",
        }
    }
}

// ---- rendering monomials and sums ---------------------------------------------------------------

/// A monomial's drawn style: a square written as a product (`x * x` for `x ^ 2`), and `y`
/// before `x`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Style {
    pub split_x: bool,
    pub split_y: bool,
    pub y_first: bool,
}

fn var_lean(name: &str, e: u32, split: bool) -> Option<String> {
    match e {
        0 => None,
        1 => Some(name.to_string()),
        2 if split => Some(format!("{name} * {name}")),
        k => Some(format!("{name} ^ {k}")),
    }
}

fn var_latex(name: &str, e: u32, split: bool) -> Option<String> {
    match e {
        0 => None,
        1 => Some(name.to_string()),
        2 if split => Some(format!("{name} \\cdot {name}")),
        k => Some(format!("{name}^{k}")),
    }
}

/// `|c| * x^i * y^j` in `syndeom-lean/1` (the sign is the sum's).
pub fn mono_lean(u: Term, s: Style) -> String {
    let abs = u.c.unsigned_abs();
    let mut parts: Vec<String> = Vec::new();
    if abs != 1 || u.degree() == 0 {
        parts.push(abs.to_string());
    }
    let x = var_lean("x", u.i, s.split_x);
    let y = var_lean("y", u.j, s.split_y);
    let (a, b) = if s.y_first { (y, x) } else { (x, y) };
    parts.extend(a);
    parts.extend(b);
    parts.join(" * ")
}

/// `|c| x^i y^j` in LaTeX: factors juxtaposed (`2x^2y`), or joined by `\cdot` when a square is
/// written as a product (`2 \cdot x \cdot x \cdot y`).
pub fn mono_latex(u: Term, s: Style) -> String {
    let abs = u.c.unsigned_abs();
    let mut parts: Vec<String> = Vec::new();
    if abs != 1 || u.degree() == 0 {
        parts.push(abs.to_string());
    }
    let x = var_latex("x", u.i, s.split_x);
    let y = var_latex("y", u.j, s.split_y);
    let (a, b) = if s.y_first { (y, x) } else { (x, y) };
    parts.extend(a);
    parts.extend(b);
    let split_used = (s.split_x && u.i == 2) || (s.split_y && u.j == 2);
    if split_used {
        parts.join(" \\cdot ")
    } else {
        parts.concat()
    }
}

/// A sum of signed parts: `a + b - c`, with a leading `-` for a negative first part.
fn sum_join(parts: &[(bool, String)]) -> String {
    let mut out = String::new();
    for (k, (neg, s)) in parts.iter().enumerate() {
        if k == 0 {
            if *neg {
                out.push('-');
            }
        } else {
            out.push_str(if *neg { " - " } else { " + " });
        }
        out.push_str(s);
    }
    out
}

// ---- presentations -----------------------------------------------------------------------------

/// How a polynomial's ordered terms are laid out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
    /// The terms as a sum.
    Expanded,
    /// One variable (`0` = `x`, `1` = `y`) factored out of the terms it divides — `x * (…)` —
    /// placed first or last among the remaining terms.
    Factored { var: u8, first: bool },
    /// The terms as two parenthesized sums, split before the term at `split`.
    Grouped { split: usize },
}

fn var_name(var: u8) -> &'static str {
    if var == 0 { "x" } else { "y" }
}

/// A drawn presentation: a permutation of the polynomial's canonical terms, a style per
/// position, and a shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Presentation {
    pub order: Vec<usize>,
    pub styles: Vec<Style>,
    pub shape: Shape,
}

/// One rendered expression: its `syndeom-lean/1` text, its LaTeX, the shape it resolved to, and
/// the ordered terms it presents (empty for a seed form).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rendered {
    pub lean: String,
    pub latex: String,
    pub shape: Shape,
    pub ordered: Vec<Term>,
}

/// The shape a drawn shape resolves to on these terms: a factored variable must divide at least
/// two terms (else the other variable, else expanded); a grouping needs three terms and a split
/// inside them.
fn resolve(shape: &Shape, ordered: &[Term]) -> Shape {
    let divides = |var: u8| {
        ordered
            .iter()
            .filter(|u| if var == 0 { u.i >= 1 } else { u.j >= 1 })
            .count()
            >= 2
    };
    match shape {
        Shape::Factored { var, first } => {
            if divides(*var) {
                Shape::Factored {
                    var: *var,
                    first: *first,
                }
            } else if divides(1 - *var) {
                Shape::Factored {
                    var: 1 - *var,
                    first: *first,
                }
            } else {
                Shape::Expanded
            }
        }
        Shape::Grouped { split } => {
            if ordered.len() >= 3 && *split >= 1 && *split < ordered.len() {
                Shape::Grouped { split: *split }
            } else {
                Shape::Expanded
            }
        }
        Shape::Expanded => Shape::Expanded,
    }
}

/// A rendered monomial with its sign.
type Signed = (bool, String);
/// A term with its position in the ordered terms.
type Indexed = (usize, Term);
/// The signed monomials of some terms, in Lean and in LaTeX.
type Parts = (Vec<Signed>, Vec<Signed>);

/// Render ordered terms under a shape with a style per position (a style for every position is
/// required; a missing one is the default).
pub fn render_ordered(shape: &Shape, styles: &[Style], ordered: &[Term]) -> Rendered {
    let style = |p: usize| styles.get(p).copied().unwrap_or_default();
    let parts = |terms: &[Indexed]| -> Parts {
        let lean = terms
            .iter()
            .map(|&(p, u)| (u.c < 0, mono_lean(u, style(p))))
            .collect();
        let latex = terms
            .iter()
            .map(|&(p, u)| (u.c < 0, mono_latex(u, style(p))))
            .collect();
        (lean, latex)
    };
    let indexed: Vec<Indexed> = ordered.iter().copied().enumerate().collect();
    let shape = resolve(shape, ordered);
    let (lean, latex) = match &shape {
        Shape::Expanded => {
            let (l, x) = parts(&indexed);
            (sum_join(&l), sum_join(&x))
        }
        Shape::Factored { var, first } => {
            let v = var_name(*var);
            let (group, rest): (Vec<Indexed>, Vec<Indexed>) = indexed
                .iter()
                .copied()
                .partition(|(_, u)| if *var == 0 { u.i >= 1 } else { u.j >= 1 });
            let quotient: Vec<Indexed> = group
                .iter()
                .map(|&(p, u)| {
                    let q = if *var == 0 {
                        Term::new(u.c, u.i - 1, u.j)
                    } else {
                        Term::new(u.c, u.i, u.j - 1)
                    };
                    (p, q)
                })
                .collect();
            let (ql, qx) = parts(&quotient);
            let factored_lean = format!("{v} * ({})", sum_join(&ql));
            let factored_latex = format!("{v}({})", sum_join(&qx));
            let (rl, rx) = parts(&rest);
            if rest.is_empty() {
                (factored_lean, factored_latex)
            } else if *first {
                let tail = |ps: &[Signed]| {
                    ps.iter()
                        .map(|(neg, s)| format!("{} {s}", if *neg { "-" } else { "+" }))
                        .collect::<Vec<_>>()
                        .join(" ")
                };
                (
                    format!("{factored_lean} {}", tail(&rl)),
                    format!("{factored_latex} {}", tail(&rx)),
                )
            } else {
                (
                    format!("{} + {factored_lean}", sum_join(&rl)),
                    format!("{} + {factored_latex}", sum_join(&rx)),
                )
            }
        }
        Shape::Grouped { split } => {
            let (al, ax) = parts(&indexed[..*split]);
            let (bl, bx) = parts(&indexed[*split..]);
            (
                format!("({}) + ({})", sum_join(&al), sum_join(&bl)),
                format!("({}) + ({})", sum_join(&ax), sum_join(&bx)),
            )
        }
    };
    Rendered {
        lean,
        latex,
        shape,
        ordered: ordered.to_vec(),
    }
}

/// Draw a presentation of `m` terms from `stream` (ADR-0011 D4: the draws, in this order, are
/// part of the producer's rng definition): the shape kind (`below(3)`), the factored variable
/// (`below(2)`), its placement (`below(2)`), the grouping split (`1 + below(m - 1)`), a
/// Fisher–Yates permutation (`below(i + 1)` for `i` from `m - 1` down to 1), then three style
/// bits per position (`below(2)` each: `split_x`, `split_y`, `y_first`). Every draw is made
/// whether or not the shape uses it, so the sequence depends on `m` alone.
pub fn draw_presentation(stream: &mut Stream, m: usize) -> Presentation {
    let kind = stream.below(3);
    let var = stream.below(2) as u8;
    let first = stream.below(2) == 0;
    let split = 1 + stream.below(m.saturating_sub(1).max(1) as u64) as usize;
    let mut order: Vec<usize> = (0..m).collect();
    for i in (1..m).rev() {
        let j = stream.below(i as u64 + 1) as usize;
        order.swap(i, j);
    }
    let styles: Vec<Style> = (0..m)
        .map(|_| {
            let split_x = stream.below(2) == 1;
            let split_y = stream.below(2) == 1;
            let y_first = stream.below(2) == 1;
            Style {
                split_x,
                split_y,
                y_first,
            }
        })
        .collect();
    let shape = match kind {
        1 => Shape::Factored { var, first },
        2 => Shape::Grouped { split },
        _ => Shape::Expanded,
    };
    Presentation {
        order,
        styles,
        shape,
    }
}

/// Render a polynomial under a presentation.
pub fn render(pres: &Presentation, terms: &[Term]) -> Rendered {
    let ordered: Vec<Term> = pres.order.iter().map(|&i| terms[i]).collect();
    render_ordered(&pres.shape, &pres.styles, &ordered)
}

/// Draw presentations of `terms` from `stream` until one renders differently from `avoid` (the
/// previous step's Lean text), at most 32 times; then fall back to the reversed canonical order,
/// then to the canonical order, in the default style — one of which differs from `avoid`.
pub fn present(stream: &mut Stream, terms: &[Term], avoid: &str) -> (Presentation, Rendered) {
    for _ in 0..32 {
        let p = draw_presentation(stream, terms.len());
        let r = render(&p, terms);
        if r.lean != avoid {
            return (p, r);
        }
    }
    let m = terms.len();
    let mut p = Presentation {
        order: (0..m).rev().collect(),
        styles: vec![Style::default(); m],
        shape: Shape::Expanded,
    };
    let r = render(&p, terms);
    if r.lean != avoid {
        return (p, r);
    }
    p.order = (0..m).collect();
    let r = render(&p, terms);
    (p, r)
}

// ---- the derivation -----------------------------------------------------------------------------

/// A `ring_chain` instance before it is a plan: the seed form, its polynomial, the rendered
/// expressions `e_0..e_n` (`e_0` the seed form), the presentation of each `e_k` (`k ≥ 1`), and
/// the chain steps the lemmas state (`sites[j - 1]` is the step lemma `j` states and the step
/// that cites it; ascending).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derivation {
    pub form: usize,
    pub poly: Poly,
    pub e: Vec<Rendered>,
    pub presentations: Vec<Presentation>,
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

/// The seed form's rendering as `e_0`.
fn seed_rendered(form: &SeedForm) -> Rendered {
    Rendered {
        lean: form.lean(),
        latex: form.latex(),
        shape: Shape::Expanded,
        ordered: Vec::new(),
    }
}

/// Derive a `ring_chain` instance of `n` steps and `m` lemmas from `seed` (D4's labels:
/// `grammar/plan` draws the seed form, `grammar/step/<k>` step k's presentation,
/// `grammar/lemma/<j>` the step lemma j states, among the steps not yet taken).
pub fn derive(n: usize, m: usize, seed: u64) -> Derivation {
    let mut plan_stream = Stream::new(seed, "grammar/plan");
    let form = plan_stream.below(SEED_FORMS.len() as u64) as usize;
    let sf = &SEED_FORMS[form];
    let poly = sf.poly();
    let mut e = vec![seed_rendered(sf)];
    let mut presentations = Vec::new();
    for k in 1..=n {
        let mut s = Stream::new(seed, &format!("grammar/step/{k}"));
        let (p, r) = present(&mut s, &poly, &e[k - 1].lean);
        e.push(r);
        presentations.push(p);
    }
    let mut remaining: Vec<usize> = (1..=n).collect();
    let mut sites = Vec::new();
    for j in 1..=m {
        let mut s = Stream::new(seed, &format!("grammar/lemma/{j}"));
        let pick = s.below(remaining.len() as u64) as usize;
        sites.push(remaining.remove(pick));
    }
    sites.sort_unstable();
    Derivation {
        form,
        poly,
        e,
        presentations,
        sites,
    }
}

fn lemma_name(j: usize) -> String {
    format!("{MAIN_DECLARATION}_lemma_{j}")
}

fn justification(k: usize, form: &SeedForm, shape: &Shape) -> String {
    let expand = form.expanding();
    match (k, shape) {
        (1, Shape::Expanded) => expand.to_string(),
        (1, Shape::Factored { var, .. }) => {
            format!("{expand} and factoring out ${}$", var_name(*var))
        }
        (1, Shape::Grouped { .. }) => format!("{expand} and regrouping"),
        (_, Shape::Expanded) => "Rearranging the terms".to_string(),
        (_, Shape::Factored { var, .. }) => format!("Factoring out ${}$", var_name(*var)),
        (_, Shape::Grouped { .. }) => "Regrouping the terms".to_string(),
    }
}

/// The lemma step stating chain step `c`'s identity (D2: a step whose text begins `Lemma j.`,
/// declared as a `theorem` whose claim is closed, proved by `intro x y` and `ring`).
fn lemma_step(j: usize, d: &Derivation, c: usize) -> Step {
    let (a, b) = (&d.e[c - 1], &d.e[c]);
    Step {
        text: format!(
            "Lemma {j}. For all integers $x$ and $y$, ${} = {}$; both sides expand to the same polynomial.",
            a.latex, b.latex
        ),
        fragment: vec!["intro x y".to_string(), "ring".to_string()],
        lemma: Some(LemmaHeader {
            kind: LemmaKind::Theorem,
            name: lemma_name(j),
            claim: format!("∀ x y : ℤ, {} = {}", a.lean, b.lean),
        }),
        padding: None,
        premises: Vec::new(),
    }
}

/// Chain step `c` of the main proof: one `calc` line — the first opens the block — closed by
/// `ring`, or by the lemma that states it, which it cites.
fn chain_step(d: &Derivation, c: usize) -> Step {
    let (a, b) = (&d.e[c - 1], &d.e[c]);
    let (justification, proof, premises) = match d.lemma_of(c) {
        Some(j) => (
            format!("By Lemma {j}"),
            format!("{} x y", lemma_name(j)),
            vec![Premise {
                cites: lemma_name(j),
                phrase: format!("Lemma {j}"),
            }],
        ),
        None => (
            justification(c, &SEED_FORMS[d.form], &b.shape),
            "by ring".to_string(),
            Vec::new(),
        ),
    };
    let line = if c == 1 {
        format!("calc {} = {} := {proof}", a.lean, b.lean)
    } else {
        format!("  _ = {} := {proof}", b.lean)
    };
    Step {
        text: format!("{justification}, ${} = {}$.", a.latex, b.latex),
        fragment: vec![line],
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
        title: "A polynomial identity in two integer variables".to_string(),
        statement: format!(
            "For all integers $x$ and $y$, ${} = {}$.",
            d.e[0].latex, d.e[n].latex
        ),
        imports: IMPORTS.iter().map(|s| s.to_string()).collect(),
        doc: None,
        main_declaration: MAIN_DECLARATION.to_string(),
        binders: "(x y : ℤ)".to_string(),
        claim: format!("{} = {}", d.e[0].lean, d.e[n].lean),
        truth,
        steps,
    }
}

/// D7: the reference of a grammar instance's `statement_status` — the artifact id of the
/// instance's control with target `formal` (the same grammar, length and seed; no flaw, no
/// padding) and the main declaration.
pub fn control_reference(
    version: GrammarVersion,
    grammar: &str,
    n_steps: u64,
    n_lemmas: u64,
    seed: u64,
) -> Result<String, String> {
    let control = json!({
        "grammar": grammar,
        "length": {"n_steps": n_steps, "n_lemmas": n_lemmas},
        "target": "formal",
        "flaws": [],
        "padding_ratio": 0,
    });
    let hash = config_hash(&control).map_err(|e| format!("the control config: {e}"))?;
    Ok(format!(
        "{}:{MAIN_DECLARATION}",
        artifact_id(GENERATOR_ID, version.as_str(), &hash, Some(seed))
    ))
}

// ---- configuration ------------------------------------------------------------------------------

/// A parsed `s2_grammar` config (ADR-0011 D3).
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub grammar: String,
    pub n_steps: u64,
    pub n_lemmas: u64,
    pub target: Target,
    pub flaws: Vec<FlawSpec>,
    pub padding_ratio: f64,
}

/// Parse and shape-check a config; every refusal is an exit-2 refusal.
pub fn parse_config(config: &Value) -> Result<Config, String> {
    let obj = config.as_object().ok_or("the config is a JSON object")?;
    exact_keys(
        obj,
        &["grammar", "length", "target", "flaws", "padding_ratio"],
        "an s2_grammar config",
    )?;
    let grammar = obj["grammar"]
        .as_str()
        .ok_or("config.grammar is a string")?
        .to_string();
    let length = obj["length"]
        .as_object()
        .ok_or("config.length is an object")?;
    exact_keys(length, &["n_steps", "n_lemmas"], "config.length")?;
    let n_steps = length["n_steps"]
        .as_u64()
        .ok_or("config.length.n_steps is a non-negative integer")?;
    let n_lemmas = length["n_lemmas"]
        .as_u64()
        .ok_or("config.length.n_lemmas is a non-negative integer")?;
    let target = match obj["target"].as_str() {
        Some("informal") => Target::Informal,
        Some("formal") => Target::Formal,
        Some("pair") => Target::Pair,
        _ => return Err("config.target is \"informal\", \"formal\" or \"pair\"".into()),
    };
    let padding_ratio = obj["padding_ratio"]
        .as_f64()
        .filter(|r| r.is_finite() && *r >= 0.0)
        .ok_or("config.padding_ratio is a non-negative number")?;
    let mut flaws = Vec::new();
    for (i, f) in obj["flaws"]
        .as_array()
        .ok_or("config.flaws is an array")?
        .iter()
        .enumerate()
    {
        flaws.push(parse_spec(f, &format!("config.flaws[{i}]"))?);
    }
    Ok(Config {
        grammar,
        n_steps,
        n_lemmas,
        target,
        flaws,
        padding_ratio,
    })
}

/// `plan` → `inject` → `render` for one (config, seed) under the current producer version.
pub fn prepare(config: &Value, seed_value: u64) -> Result<Prepared, String> {
    prepare_with(GrammarVersion::CURRENT, config, seed_value)
}

/// `plan` → `inject` → `render` for one (config, seed) under a named producer version.
pub fn prepare_with(
    version: GrammarVersion,
    config: &Value,
    seed_value: u64,
) -> Result<Prepared, String> {
    let cfg = parse_config(config)?;
    let hash =
        config_hash(config).map_err(|e| format!("the config cannot be canonicalized: {e}"))?;
    if cfg.grammar != RING_CHAIN {
        return Err(format!(
            "no grammar {:?} in s2_grammar {}",
            cfg.grammar,
            version.as_str()
        ));
    }
    if cfg.n_steps < N_STEPS_RANGE.0 || cfg.n_steps > N_STEPS_RANGE.1 {
        return Err(format!(
            "length.n_steps is {} to {} in s2_grammar {} (ADR-0011 D6)",
            N_STEPS_RANGE.0,
            N_STEPS_RANGE.1,
            version.as_str()
        ));
    }
    if cfg.n_lemmas > N_LEMMAS_RANGE.1 {
        return Err(format!(
            "length.n_lemmas is {} to {} in s2_grammar {} (ADR-0011 D6)",
            N_LEMMAS_RANGE.0,
            N_LEMMAS_RANGE.1,
            version.as_str()
        ));
    }
    if cfg.n_lemmas > cfg.n_steps {
        return Err(
            "length.n_lemmas is at most length.n_steps: every lemma states one chain step".into(),
        );
    }
    if cfg.padding_ratio != 0.0 || cfg.flaws.iter().any(|s| s.class == "FLAW-0013") {
        return Err(format!(
            "s2_grammar {} carries no FLAW-0013 padding material; padding_ratio is 0",
            version.as_str()
        ));
    }
    if cfg.flaws.len() > 1 {
        return Err(format!(
            "s2_grammar {} injects one instance per artifact (ADR-0011 D5)",
            version.as_str()
        ));
    }
    if let Some(spec) = cfg.flaws.first()
        && spec.class != "FLAW-0001"
    {
        return Err(format!(
            "{} is not realized by s2_grammar {}",
            spec.class,
            version.as_str()
        ));
    }
    let n = cfg.n_steps as usize;
    let m = cfg.n_lemmas as usize;
    let d = derive(n, m, seed_value);
    let truth = StatementStatus {
        value: Some(true),
        method: StatementMethod::LeanKernel,
        reference: control_reference(version, &cfg.grammar, cfg.n_steps, cfg.n_lemmas, seed_value)?,
    };
    let faithful = plan_of(&d, truth);
    let (plan, injection) = match cfg.flaws.first() {
        None => (faithful, None),
        Some(spec) => {
            let (plan, inj) = inject(&d, &faithful, spec, cfg.target, seed_value)?;
            (plan, Some(inj))
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
        config_hash: hash,
        seed: seed_value,
        target: cfg.target,
        plan,
        informal,
        formal,
        injection,
        padding: Vec::new(),
        composite: None,
        false_theorem: None,
    })
}

// ---- FLAW-0001 ---------------------------------------------------------------------------------

/// One perturbation target in a step's ordered terms (ADR-0011 D5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Perturbation {
    /// The sign of the term at this position flipped.
    Sign(usize),
    /// The coefficient of the term at this position raised by one in magnitude.
    Coefficient(usize),
    /// The constant term at this position raised by one in magnitude.
    Constant(usize),
    /// A constant term `+ 1` added, there being none.
    ConstantAdd,
    /// The exponent of `x` (`0`) or `y` (`1`) in the term at this position raised by one.
    Exponent(usize, u8),
}

/// The eligible targets of a perturbation kind in ordered terms, in position order: every term
/// for `sign`; every non-constant term for `coefficient`; the constant term, or the addition of
/// one, for `constant`; every (term, variable) with a positive exponent whose raised monomial is
/// not another term's, for `exponent`.
pub fn targets(kind: &str, ordered: &[Term]) -> Vec<Perturbation> {
    let mut out = Vec::new();
    match kind {
        "sign" => out.extend((0..ordered.len()).map(Perturbation::Sign)),
        "coefficient" => out.extend(
            ordered
                .iter()
                .enumerate()
                .filter(|(_, u)| u.degree() >= 1)
                .map(|(p, _)| Perturbation::Coefficient(p)),
        ),
        "constant" => {
            match ordered.iter().position(|u| u.degree() == 0) {
                Some(p) => out.push(Perturbation::Constant(p)),
                None => out.push(Perturbation::ConstantAdd),
            };
        }
        "exponent" => {
            for (p, u) in ordered.iter().enumerate() {
                for var in 0..2u8 {
                    let raised = if var == 0 {
                        Term::new(u.c, u.i + 1, u.j)
                    } else {
                        Term::new(u.c, u.i, u.j + 1)
                    };
                    let has = if var == 0 { u.i >= 1 } else { u.j >= 1 };
                    if has && !ordered.iter().any(|v| v.same_monomial(raised)) {
                        out.push(Perturbation::Exponent(p, var));
                    }
                }
            }
        }
        _ => {}
    }
    out
}

/// Apply a target: the perturbed ordered terms and the monomial replaced (none for an added
/// constant) and inserted, as signed `syndeom-lean/1` text.
pub fn apply(target: Perturbation, ordered: &[Term]) -> (Vec<Term>, Option<String>, String) {
    let signed = |u: Term| sum_join(&[(u.c < 0, mono_lean(u, Style::default()))]);
    let mut out = ordered.to_vec();
    match target {
        Perturbation::Sign(p) => {
            let before = out[p];
            out[p].c = -out[p].c;
            let after = out[p];
            (out, Some(signed(before)), signed(after))
        }
        Perturbation::Coefficient(p) | Perturbation::Constant(p) => {
            let before = out[p];
            out[p].c += before.c.signum();
            let after = out[p];
            (out, Some(signed(before)), signed(after))
        }
        Perturbation::ConstantAdd => {
            let added = Term::new(1, 0, 0);
            out.push(added);
            (out, None, signed(added))
        }
        Perturbation::Exponent(p, var) => {
            let before = out[p];
            if var == 0 {
                out[p].i += 1;
            } else {
                out[p].j += 1;
            }
            let after = out[p];
            (out, Some(signed(before)), signed(after))
        }
    }
}

/// The injector for FLAW-0001 on `ring_chain` (ADR-0011 D5): the material is generated — the
/// site is a ring-closed chain step (for `local`, not the last), drawn from
/// `injector/FLAW-0001/1` when the spec says `auto`; the perturbed monomial is drawn from the
/// same stream among the kind's eligible targets at the site; the site is re-rendered under its
/// own presentation with the perturbed terms. `propagated` re-derives every later step, and
/// every lemma a later step cites, from the perturbed polynomial (each later step on its own
/// `grammar/step/<k>` stream, as the faithful one was); `local` changes the site alone. The
/// record's dials carry the perturbation, the site (a plan step index), the monomial replaced
/// and inserted, the site's text or fragment replaced and inserted, and every original text or
/// fragment the variant changed.
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
    let expected = if propagated {
        Severity::Fatal
    } else {
        Severity::Repairable
    };
    if spec.severity != expected {
        return Err(format!(
            "FLAW-0001 {} is {expected:?} (ADR-0011 D5), not {:?}",
            spec.sub_variant.as_deref().unwrap_or_default(),
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
    let eligible: Vec<usize> = (1..=n)
        .filter(|&c| d.lemma_of(c).is_none() && (propagated || c < n))
        .collect();
    if eligible.is_empty() {
        return Err(format!(
            "no eligible site for FLAW-0001 {}: it needs a ring-closed chain step{}",
            spec.sub_variant.as_deref().unwrap_or_default(),
            if propagated { "" } else { " before the last" }
        ));
    }
    let mut stream = Stream::new(seed, "injector/FLAW-0001/1");
    let c = match spec.site {
        None => eligible[stream.below(eligible.len() as u64) as usize],
        Some(k) => {
            let c = (k as usize).saturating_sub(m);
            if !eligible.contains(&c) {
                return Err(format!(
                    "site {k} is not an eligible step: FLAW-0001 {} sits at a ring-closed chain step (plan steps {} to {}{})",
                    spec.sub_variant.as_deref().unwrap_or_default(),
                    m + 1,
                    m + n,
                    if propagated { "" } else { ", not the last" }
                ));
            }
            c
        }
    };
    let site_terms = &d.e[c].ordered;
    let candidates = targets(kind, site_terms);
    if candidates.is_empty() {
        return Err(format!(
            "no eligible {kind} perturbation at step {}: every raised monomial is another term's",
            m + c
        ));
    }
    let chosen = candidates[stream.below(candidates.len() as u64) as usize];
    let (perturbed_terms, mono_replaced, mono_inserted) = apply(chosen, site_terms);
    let perturbed_poly = normalize(&perturbed_terms);
    if perturbed_poly == d.poly {
        return Err("a perturbation changes the polynomial".into());
    }
    let pres = &d.presentations[c - 1];
    let mut styles = pres.styles.clone();
    styles.resize(perturbed_terms.len(), Style::default());
    let site_rendered = render_ordered(&pres.shape, &styles, &perturbed_terms);

    // The flawed derivation: the site re-rendered; under `propagated`, every later step a fresh
    // presentation of the perturbed polynomial on its own stream.
    let mut flawed = d.clone();
    flawed.e[c] = site_rendered;
    if propagated {
        flawed.poly = perturbed_poly;
        for k in c + 1..=n {
            let mut s = Stream::new(seed, &format!("grammar/step/{k}"));
            let (p, r) = present(&mut s, &flawed.poly, &flawed.e[k - 1].lean);
            flawed.e[k] = r;
            flawed.presentations[k - 1] = p;
        }
    }
    let plan = if propagated {
        // step 0 is untouched (D5): the claim of record stays the faithful one, so the
        // re-derived chain ends away from it
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
    let inserted = flawed.e[c].latex.clone();
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
    let with = |extra: Vec<(&str, Value)>| {
        let mut dials = spec.dials.clone();
        dials.insert("site".into(), json!(site));
        dials.insert(
            "monomial".into(),
            json!({"replaced": mono_replaced, "inserted": mono_inserted}),
        );
        for (k, v) in extra {
            dials.insert(k.into(), v);
        }
        Value::Object(dials)
    };
    let informal_dials = with(vec![
        ("inserted", json!(inserted)),
        ("replaced", json!(d.e[c].latex)),
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

/// The classes `s2_grammar` realizes, by version (ADR-0011 D6): FLAW-0001 in 0.1.0.
pub fn realizes(version: GrammarVersion, class: &str) -> bool {
    match version {
        GrammarVersion::V0_1_0 => class == "FLAW-0001",
    }
}

/// The plan step index (1-based) of chain step `c` in an instance with `m` lemmas.
pub fn plan_index(m: usize, c: usize) -> usize {
    m + c
}

/// For tests: the original texts a record carries, as a map from plan step index to text.
pub fn originals(dials: &Value, key: &str) -> BTreeMap<usize, Value> {
    dials[key]
        .as_object()
        .map(|o| {
            o.iter()
                .filter_map(|(k, v)| k.parse().ok().map(|k| (k, v.clone())))
                .collect()
        })
        .unwrap_or_default()
}
