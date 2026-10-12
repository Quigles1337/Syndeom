# FLAWS — versioned flaw taxonomy register (§3.7)

- **Taxonomy version:** `0.1.0` — **ratified by Al 2026-10-12 (Session 2, S0.5)**: thirteen classes
  seeded in Session 1 from the genesis prompt, plus FLAW-0014 and one example fix applied as
  pre-ratification edits (changelog). From ratification: entries are append-only; a new entry bumps
  the minor version; a semantic change to an existing entry's definition or Lean-catchable status
  bumps the major version and requires an ADR. Every manifest pins `taxonomy_version` (SD1).
- **Examples are executor-authored and synthetic.** They are illustrations, not citations (SD11); the
  mathematics of each was checked by hand in Session 1 and each becomes a golden fixture at S2.4. No
  example reuses a real author's name for a fictional work.
- **Nothing here is a claim about any real proof.** Classes describe what the generator can inject
  and what the battery is calibrated to find (SD3).

## Entry fields

| Field | Meaning |
|---|---|
| ID, Name | `FLAW-NNNN`, short name |
| Definition | what the flaw is, including sub-variants the generator must label |
| Manifests in | `informal` / `formal` / `both` — where an instance can occur |
| Lean-catchable | vocabulary below; this is the "can a proof checker catch it" field of the manifest |
| Severity range | vocabulary below; the manifest records the instance's actual severity |
| Generator recipe | how an instance is injected, and which dials it exposes |
| Example | a minimal synthetic instance |
| Candidate detectors | S3.1 detectors expected to see this class; expectations are pre-registered predictions, not results |

## Vocabulary

**Lean-catchable**
- `YES-KERNEL` — a faithful formalization of the flawed step fails to elaborate or typecheck; `lake build` fails.
- `YES-TOOLING` — the formal artifact compiles; the flaw is caught by tooling (`#print axioms`, linters, a declaration-usage audit), never by compilation alone.
- `NO` — the formal artifact compiles and Lean has no view of the flaw; detection needs semantic comparison or human review.
- `N/A` — the flaw has no formal manifestation (informal-only); the formal analogue, if any, is named in the entry.

**Severity**
- `cosmetic` — no effect on validity or conclusion; affects verification cost only.
- `repairable` — a local error whose correction is mechanical and leaves the argument intact.
- `gap` — a missing or unjustified step; the conclusion may still be true; repair needs new mathematics of unknown size.
- `fatal` — the argument does not establish the stated conclusion, or the conclusion is false.

**Location** (to be fixed by the S0.4 manifest ADR): informal instances by proof step index and
character span under the segmentation the ADR decides; formal instances by `<declaration>:<line>`.

**Ground truth is two-dimensional.** `statement_status` (true / false, established how) is recorded
separately from `proof_status` (valid / invalid) and from the flaw list, because FLAW-0011 and
FLAW-0012 exist precisely to separate them. The S0.4 ADR fixes the field names.

---

## FLAW-0001 — Sign or constant error

- **Definition:** An arithmetic or algebraic step carries a wrong sign, coefficient, exponent, or
  constant (including a dropped integration constant or an off-by-one bound). Sub-variants:
  **propagated** (the error changes what follows, up to the conclusion) and **local** (the wrong
  expression is written but the correct consequence is asserted anyway, so the step is inconsistent
  with its own neighbours).
- **Manifests in:** both.
- **Lean-catchable:** `YES-KERNEL` — a faithful formalization of the erroneous step does not close
  (`ring`, `norm_num`, `linarith` fail) or the rewrite does not match.
- **Severity range:** repairable (local) → fatal (propagated to a changed conclusion).
- **Generator recipe:** select an algebraic step; perturb exactly one sign, coefficient, or constant;
  dial `propagated | local`; record location and the original expression.
- **Example:** Goal: `x² + 1 ≥ 2x` for real x. Propagated: "(x − 1)² = x² − 2x − 1 ≥ 0, hence
  x² − 1 ≥ 2x" — the conclusion changed. Local: the same wrong expansion followed by the correct
  "hence x² + 1 ≥ 2x".
- **Candidate detectors:** autoformalization attempt; step-dependency extraction (the local variant
  shows as a step not entailed by its premise).

## FLAW-0002 — Quantifier scope error

- **Definition:** The order or scope of quantifiers changes inside a step — typically ∀x ∃y read as
  ∃y ∀x (a dependence silently made uniform), or a quantifier moved across a negation or an
  implication without dualizing.
- **Manifests in:** both. If the change is in the *statement* rather than in a step, it is FLAW-0008.
- **Lean-catchable:** `YES-KERNEL` — the term's type does not match the goal; `intro`/`obtain` order
  is forced by the statement.
- **Severity range:** gap → fatal.
- **Generator recipe:** locate an adjacent ∀∃ pair in a step and swap it; or move ¬ across a
  quantifier without dualizing; record the original scoping.
- **Example:** Step: "f is continuous, so for every ε > 0 there is δ > 0 such that for all x, y with
  |x − y| < δ, |f(x) − f(y)| < ε." Continuity gives δ depending on x; the step asserts uniform
  continuity. Fatal when the argument then uses one δ for all x (e.g., f(x) = x² on ℝ).
- **Candidate detectors:** autoformalization attempt; hypothesis check at theorem applications
  (uniform continuity consumed where only continuity was available).

## FLAW-0003 — Circularity

- **Definition:** The argument uses the conclusion, or a statement equivalent to it, as a premise —
  directly (the goal restated as a lemma and "proved" by appeal to the theorem), or through a cycle
  of lemmas each cited in the other's proof.
- **Manifests in:** both for the direct form; the cyclic form is informal-only in practice (Lean's
  declaration order forbids forward reference, so a formal cycle cannot be written).
- **Lean-catchable:** `YES-KERNEL` for direct self-reference (non-well-founded) or a missing proof
  term; the cyclic form cannot be expressed formally. In informal text: `NO`; detectable only by
  dependency extraction.
- **Severity range:** fatal.
- **Generator recipe:** replace the justification of the key step with the goal (or its
  contrapositive) under a new lemma name; cycle variant: Lemma A cites Theorem T, and T's proof
  cites Lemma A.
- **Example:** "Every bounded monotone real sequence converges. Proof: let L = lim aₙ. For ε > 0
  choose N with |aₙ − L| < ε for n ≥ N; hence aₙ → L." The existence of L is the claim.
- **Candidate detectors:** step-dependency extraction (the goal appears among its own premises; the
  citation graph has a cycle); autoformalization attempt.

## FLAW-0004 — Unjustified step ("clearly")

- **Definition:** A non-trivial step carries no justification, only a hedge ("clearly", "it is easy
  to see", "by a standard argument", "obviously"). Dial: the hidden step is `routine-true`,
  `nontrivial-true`, or `false`.
- **Manifests in:** informal. The formal analogue is a missing proof term, which cannot compile
  without `sorry`.
- **Lean-catchable:** `YES-KERNEL` — the gap must be filled; when the hidden step is false it cannot
  be.
- **Severity range:** cosmetic (routine-true) → gap (nontrivial-true) → fatal (false).
- **Generator recipe:** select a step; delete its justification; substitute a phrase from a fixed
  hedge lexicon; the dial is set by which step is chosen; record the deleted justification.
- **Example:** False: "Clearly every continuous function on [0, 1] is differentiable almost
  everywhere" (nowhere-differentiable continuous functions exist). Nontrivial-true: "it is easy to
  see the series converges uniformly" where only pointwise convergence was shown and uniform
  convergence does in fact hold.
- **Candidate detectors:** step-dependency extraction (a step with no inbound justification);
  autoformalization attempt; hedge-word statistics (surface control — pre-registered expectation:
  fooled by LLM text that hedges fluently and by correct proofs that say "clearly" honestly).

## FLAW-0005 — Theorem applied without its hypotheses

- **Definition:** A named theorem is invoked on an object for which one of its hypotheses is either
  not established (gap) or false (fatal) — differentiability for the mean value theorem, a
  dominating function for interchanging limit and integral, a non-zero divisor for cancellation.
- **Manifests in:** both.
- **Lean-catchable:** `YES-KERNEL` — the hypothesis argument cannot be supplied.
- **Severity range:** gap (hypothesis true but unshown) → fatal (hypothesis false).
- **Generator recipe:** choose a theorem application; remove the derivation of one hypothesis (gap)
  or replace the object by one that violates it (fatal); record which hypothesis.
- **Example:** Fatal: "By Rolle's theorem applied to f(x) = |x| on [−1, 1], since f(−1) = f(1)
  there is c ∈ (−1, 1) with f′(c) = 0." f is not differentiable at 0; no such c exists. Gap:
  "since ab = ac, b = c" with a ≠ 0 true but never stated.
- **Candidate detectors:** hypothesis check at theorem applications (the priority detector for this
  class); autoformalization attempt.

## FLAW-0006 — Induction base or step error

- **Definition:** An induction is missing or misstates its base case, proves the step only for n
  above the base (so the first step is never justified), shifts the index, or confuses strong and
  weak induction.
- **Manifests in:** both.
- **Lean-catchable:** `YES-KERNEL` — `induction` forces both cases; a step argument valid only for
  n ≥ 2 leaves the case 1 → 2 open.
- **Severity range:** gap → fatal.
- **Generator recipe:** drop or shift the base case; or restrict the step's argument with a hidden
  assumption that fails at the base; record the failing case.
- **Example:** "In any set of n horses all have the same colour. Base n = 1: trivial. Step: in n + 1
  horses, the first n share a colour and the last n share a colour; the overlap forces all n + 1 to
  agree." The overlap is empty when n + 1 = 2.
- **Candidate detectors:** step-dependency extraction (incomplete case analysis); autoformalization
  attempt.

## FLAW-0007 — Vacuous or unused lemma

- **Definition:** (a) **Unused:** a lemma is stated and proved but never used by any later step.
  (b) **Vacuous:** a lemma's hypotheses are jointly unsatisfiable, so it has no instances; a later
  step that applies it to a concrete object is additionally FLAW-0005.
- **Manifests in:** both.
- **Lean-catchable:** unused — `YES-TOOLING` (compiles; a declaration-usage audit or linter finds
  it); vacuous — `NO` in general (compiles; unsatisfiability of hypotheses is not decided by
  elaboration).
- **Severity range:** cosmetic (unused; cost only) → gap or fatal (a vacuous lemma relied upon).
- **Generator recipe:** insert a true, proved, irrelevant lemma (dials: count, length); or a lemma
  with contradictory hypotheses plus a later application.
- **Example:** Unused: "Lemma 2. For all n ∈ ℕ, n² ≥ n." — proved, cited nowhere. Vacuous: "Lemma 3. If
  p is an even prime greater than 2 then …" (no such p), later "applying Lemma 3 to p = 2".
- **Candidate detectors:** lemma-usage consistency (the priority detector for this class);
  step-dependency extraction.

## FLAW-0008 — Statement drift

- **Definition:** The formal statement differs from the informal claim: an added hypothesis, a
  restricted type (ℝ to ℕ), strict vs non-strict inequality, a weakened conclusion, or a locally
  redefined term. The formal proof is correct *for its own statement*; it is not a proof of the
  claim.
- **Manifests in:** formal — the pair (informal claim, Lean statement).
- **Lean-catchable:** `NO` — by definition. Lean checks the statement it is given. (KOBER's E3 is
  this failure mode seen from the prover's side; SD2(iii) is this program's guard on its own output.)
- **Severity range:** gap (the drifted statement is strictly weaker and the claim still plausible)
  → fatal (incomparable, or trivialized).
- **Generator recipe:** from a faithful formalization, apply exactly one of: add a hypothesis;
  restrict a type; flip strict/non-strict; weaken the conclusion; shadow a definition. Record the
  transformation.
- **Example:** Informal: "every bounded sequence of reals has a convergent subsequence." Lean:
  `theorem bw (a : ℕ → ℝ) (hb : ∃ M, ∀ n, |a n| ≤ M) (hm : Monotone a) :
  ∃ L, Filter.Tendsto a Filter.atTop (nhds L)` — an added hypothesis (monotone) and a changed
  conclusion (the sequence itself converges). True, compiles against mathlib, and is not
  Bolzano–Weierstrass.
- **Candidate detectors:** statement-fidelity comparison — the only detector that can see this
  class; the S1.2 informal–formal pairs are its ground truth.

## FLAW-0009 — Smuggled axiom

- **Definition:** The formal proof is sorry-free and compiles, but depends on an axiom beyond the
  standard three — a user `axiom` declaration stating a needed lemma (true or false),
  `native_decide` (which adds `Lean.ofReduceBool`), or an imported declaration that itself depends
  on one.
- **Manifests in:** formal.
- **Lean-catchable:** `YES-TOOLING` — `#print axioms`; the kernel accepts the proof. "It compiles"
  is not a check.
- **Severity range:** gap (axiom true but unproved) → fatal (axiom false).
- **Generator recipe:** replace one lemma's proof with an `axiom` of the same statement; dial
  `true | false`; variant: close a decidable goal by `native_decide`. Record the axiom name.
- **Example:** `axiom helper : ∀ n : ℕ, n < 2 ^ n` followed by a sorry-free `theorem main` that
  uses `helper`. `lake build` succeeds; `#print axioms main` lists `helper`.
- **Candidate detectors:** axiom audit (`#print axioms` against the SD2 standard three) — mandatory
  before any "proven" label.

## FLAW-0010 — Fabricated or wrong citation

- **Definition:** A step is justified by a reference that (a) does not exist, (b) exists but does
  not contain the cited result, or (c) contains the result with altered hypotheses or conclusion.
- **Manifests in:** informal. In Lean a reference is a lemma name: a nonexistent name is
  `YES-KERNEL` (unknown identifier); a wrong lemma is `YES-KERNEL` on type mismatch unless its type
  coincidentally fits.
- **Lean-catchable:** `N/A` for the informal form; `YES-KERNEL` for the formal analogue.
- **Severity range:** gap (the cited result exists elsewhere and is true) → fatal (no such result).
- **Generator recipe:** replace a real citation with (a) a fictional reference of plausible shape,
  (b) a real reference to the wrong work, or (c) the real work with an altered statement. The
  generator never attaches a real author's name to a fictional work.
- **Example (shape only; fictional by construction):** "by [Placeholder 19XX, Theorem 4.2] the
  graph is connected", where no such work is indexed by DOI, Crossref, arXiv, or zbMATH.
- **Candidate detectors:** citation existence check (DOI / arXiv / Crossref / zbMATH lookup);
  citation content check (semantic or human).

## FLAW-0011 — True theorem with invalid proof (composite)

- **Definition:** The statement is known true (e.g., mathlib-formalized) and the offered proof
  carries at least one flaw from FLAW-0001..0010. Exists so triage must separate "is the statement
  true" from "does this argument prove it".
- **Manifests in:** both.
- **Lean-catchable:** for the proof, per the embedded flaw; the statement's truth is established
  independently (S1.2).
- **Severity range:** inherits the embedded flaw's severity; the conclusion is nonetheless true.
- **Generator recipe:** pair a known-true statement with a flawed proof; record
  `statement_status: true`, `proof_status: invalid`, and the embedded flaw list.
- **Example:** Statement: √2 is irrational. Proof: the standard descent argument with the step
  "2 | p² ⇒ 2 | p" replaced by "clearly" (FLAW-0004, routine-true) and the parity case split
  dropped.
- **Candidate detectors:** all; the class measures whether detectors are lulled by true conclusions.

## FLAW-0012 — False theorem with convincing proof (composite)

- **Definition:** The statement is false (a counterexample exists) and the proof mirrors the
  argument for a nearby true statement, hiding the point where a dropped hypothesis is used.
- **Manifests in:** both.
- **Lean-catchable:** `YES-KERNEL` — a sorry-free, standard-axiom proof of a false statement cannot
  exist (consistency assumed); the flaw surfaces at the hypothesis's use site, usually as FLAW-0005.
- **Severity range:** fatal by definition.
- **Generator recipe:** take a true theorem; drop or weaken one hypothesis to obtain a false
  statement with a known counterexample; transplant the true-case proof; record the counterexample
  and the hidden use site.
- **Example:** Claim: "every bounded continuous f : ℝ → ℝ attains its supremum" (false: arctan).
  Proof mirrors the extreme value theorem on a compact interval: "take xₙ with f(xₙ) → sup f; pass
  to a convergent subsequence xₙₖ → x*; by continuity f(x*) = sup f." The subsequence step needs
  (xₙ) bounded, which compactness supplied and ℝ does not.
- **Candidate detectors:** hypothesis check at theorem applications; autoformalization attempt;
  counterexample search (only if added to the battery by ADR).

## FLAW-0013 — Correct proof with uninformative padding

- **Definition:** The proof is valid and its statement true, but the text carries material that
  contributes nothing to the argument: restated definitions, re-derivations of established facts,
  proved-but-unused lemmas (overlaps FLAW-0007a), verbose rewriting. A cost flaw, not a correctness
  flaw (SD17: measured as verification cost, never as pedagogical harm).
- **Manifests in:** both.
- **Lean-catchable:** `NO` — it compiles and is correct. Unused declarations inside it are
  `YES-TOOLING`, as in FLAW-0007.
- **Severity range:** cosmetic; the manifest records `padding_ratio` as a cost weight.
- **Generator recipe:** inject true, irrelevant material at ratio r (dial) at recorded positions.
- **Example:** A correct five-step proof that √2 is irrational, preceded by forty lines restating
  the definitions of rational number, gcd, and parity, with three separate derivations of
  "2 | p² ⇒ 2 | p".
- **Candidate detectors:** lemma-usage consistency; step-dependency extraction (steps with no
  outbound edge to the conclusion); length statistics (surface control). Feeds the S3.4 cost model:
  padding raises expert-minutes per item without raising the flaw count.

## FLAW-0014 — Incomplete case analysis (non-inductive)

- **Definition:** A case split omits a case — equality, zero, the empty set, a boundary value — and
  the conclusion is asserted for all cases. Distinct from FLAW-0006 (induction): no inductive
  structure is involved. Dial: the omitted case is one where the claim still holds (gap) or one
  where it fails (fatal).
- **Manifests in:** both.
- **Lean-catchable:** `YES-KERNEL` — `cases`, `match`, and `rcases` force exhaustiveness; a missing
  branch is an error.
- **Severity range:** gap → fatal.
- **Generator recipe:** from a complete case split, delete one branch and every mention of it;
  record the deleted case and whether the claim holds there.
- **Example:** Gap: "Claim: for every integer n, n² ≡ 0 or 1 (mod 4). Proof: if n is even, n = 2k
  and n² = 4k² ≡ 0. Hence the claim." — the odd case is omitted; the claim holds there. Fatal:
  "Claim: for every real x, x/|x| = ±1. Case x > 0: it is 1. Case x < 0: it is −1. Hence for all
  x." — the case x = 0 is omitted, and there the expression is undefined.
- **Candidate detectors:** step-dependency extraction (a case analysis whose branches do not cover
  the domain); autoformalization attempt.

---

## Summary table (v0.1.0)

| ID | Name | Manifests in | Lean-catchable | Severity range |
|---|---|---|---|---|
| FLAW-0001 | Sign or constant error | both | YES-KERNEL | repairable → fatal |
| FLAW-0002 | Quantifier scope error | both | YES-KERNEL | gap → fatal |
| FLAW-0003 | Circularity | both (cycle: informal) | YES-KERNEL (direct) / NO (informal cycle) | fatal |
| FLAW-0004 | Unjustified step | informal | YES-KERNEL | cosmetic → fatal |
| FLAW-0005 | Theorem applied without hypotheses | both | YES-KERNEL | gap → fatal |
| FLAW-0006 | Induction base/step error | both | YES-KERNEL | gap → fatal |
| FLAW-0007 | Vacuous or unused lemma | both | YES-TOOLING (unused) / NO (vacuous) | cosmetic → fatal |
| FLAW-0008 | Statement drift | formal pair | NO | gap → fatal |
| FLAW-0009 | Smuggled axiom | formal | YES-TOOLING | gap → fatal |
| FLAW-0010 | Fabricated or wrong citation | informal | N/A (formal analogue YES-KERNEL) | gap → fatal |
| FLAW-0011 | True theorem, invalid proof | both | per embedded flaw | inherits |
| FLAW-0012 | False theorem, convincing proof | both | YES-KERNEL | fatal |
| FLAW-0013 | Correct proof with padding | both | NO | cosmetic |
| FLAW-0014 | Incomplete case analysis (non-inductive) | both | YES-KERNEL | gap → fatal |

**Pre-registered expectation (§0.7, SD5):** four of fourteen classes (0003-cycle, 0007-vacuous,
0008, 0013) are invisible to compilation, and one (0009) is invisible to compilation but visible to
tooling. A battery that relies on "it compiles" is blind to exactly the classes an adversarial or
careless generator finds cheapest to emit. This expectation is a prediction to be tested in S3, not
a result.

## Changelog

- `0.1.0` — 2026-10-12, Session 1: seeded FLAW-0001..0013 from the genesis prompt (draft).
- `0.1.0` — 2026-10-12, Session 2, pre-ratification edits ruled by Al: added FLAW-0014 (incomplete
  case analysis, non-inductive); FLAW-0007's unused-lemma example now states n ∈ ℕ. Version unchanged
  (pre-ratification). **Ratified by Al, Session 2 (S0.5, Q12).**
