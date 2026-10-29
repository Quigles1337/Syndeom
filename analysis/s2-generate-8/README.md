# S2 implementation packet 8 — `s2_grammar` 0.1.0 (ADR-0011, Session 47, 2026-10-29)

This packet records the eighth implementation session of ADR-0011 (D11): the family's second
producer, `s2_grammar` 0.1.0 — the grammar `ring_chain` (D6: a polynomial identity over ℤ in two
variables, proved as a `calc` chain of `n_steps` equalities each closed by `ring`, with
`n_lemmas` of them stated as lemma steps and cited), its control, and FLAW-0001 (`propagated`,
`local`) with generated material — with its goldens and the producer dispatch that lets one
`assemble` serve both producers. Nothing here is a result: no CLAIM, no fidelity sweep, no
artifact under `corpora/`.

Branch `claude/s2-grammar-0.1.0`, from `main` `a2370a01a31e727b40253593d3e694f10031ff51` (PR
32's merge):

| Commit | What |
|---|---|
| `25ad34fe45c1eea8527b3aab47829b31c8685931` | the 0.1.0 producer: `src/grammar.rs` (the atom library, the presentations, the derivation, the plan, the truth reference, the config, FLAW-0001), `Producer` in `Prepared` and `assemble`, the dispatch (`prepare_any`, `prepare_for`), the CLI, `tests/grammar.rs`, `tests/goldens.rs` by producer, the crate README |
| `29f5f75879521e78f2486e5f8e1f4557b0f6f779` | the 10 goldens of `s2_grammar` 0.1.0, produced at `25ad34fe…`, the goldens README and the count (79 cases) |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 47 |

## What was built

- **The second producer (D1).** `s2_grammar` 0.1.0 lives in the same crate, module
  `grammar`, with `generator.id` `s2_grammar`, `version` `0.1.0`, `rng` `syndeom-rng/1`.
  `Prepared` — the structure every producer prepares — carries a `Producer`
  (`Template(Version)` or `Grammar(GrammarVersion)`), from which `assemble` writes the
  manifest's `generator`, forms the artifact id and decides whether flaws count per instance
  (every producer but `s2_template` 0.1.0). `generate` prepares the producer a config names — a
  `theorem` key is an `s2_template` config, a `grammar` key an `s2_grammar` config — at that
  producer's current version (`prepare_any`); `regenerate` and `verify` select the producer and
  version from the manifest's `generator.id` and `generator.version` (`Producer::parse`,
  `prepare_for`); `tests/goldens.rs` does the same. No byte of any `s2_template` artifact
  changes: the 69 earlier goldens regenerate under the versions they name.
- **The grammar `ring_chain` and its atom library (D6).** A config `{"grammar": "ring_chain",
  "length": {"n_steps", "n_lemmas"}, "target", "flaws", "padding_ratio"}` (D3; `n_steps` 1 to
  256, `n_lemmas` 0 to 32 and at most `n_steps`; `padding_ratio` 0) names an instance: a seed
  form `e_0`, drawn on `grammar/plan` from a fixed table of fourteen products and powers of small
  binomials and trinomials in `x` and `y` (`(x + y) ^ 2`, `(x + 2 * y) * (x - y)`,
  `(x + y + 1) ^ 2`, `(x * y + 1) ^ 2`, …; coefficients and constants in {1, 2, 3, 5}; every form
  uses both variables), and `n_steps` presentations `e_1 … e_n` of its expansion, each drawn on
  its own stream `grammar/step/<k>` and differing from the step before. A presentation of a
  polynomial — a canonical list of monomials — is a drawn permutation of the terms, a drawn
  style per term (`x ^ 2` or `x * x`; `x * y` or `y * x`) and a drawn shape: expanded; one
  variable factored out of the terms it divides (`x * (x - 4 * y) + 4 * y ^ 2`), first or last;
  or the terms regrouped as two parenthesized sums. A shape the terms cannot take resolves to
  expanded; a step equal to the one before is redrawn (at most 32 draws, then a fallback). Every
  draw is made whether or not the shape uses it, so a step's draw sequence depends on the term
  count alone (D4). The theorem is `e_0 = e_n` for all integers `x`, `y`; the main declaration
  is `syndeom_s2_grammar_ring_chain (x y : ℤ)`; the one import is `Mathlib.Tactic.Ring`. The
  informal file renders LaTeX by juxtaposition (`4yx`, `x(x - 4y)`; a split square `4 \cdot y
  \cdot y`), with a justification per step from the shape (`Expanding the square and factoring
  out $x$`, `Rearranging the terms`, `Regrouping the terms`, `Factoring out $y$`). The library's
  `source` is the module itself: executor-authored synthetic material outside the S1 corpus
  (ruling 1 of 2026-10-27).
- **Lemma steps (D2).** `n_lemmas` chain steps, drawn on `grammar/lemma/<j>` among the steps not
  yet taken and numbered in the ascending order of the steps they state, are stated as lemma
  steps — `theorem …_lemma_<j> : ∀ x y : ℤ, e_{k-1} = e_k := by` / `intro x y` / `ring`, first in
  document order — and cited by the chain step as a term (`_ = e_k := …_lemma_<j> x y`; its
  text `By Lemma j, …`; its `premises` name the lemma). Every equality of the chain is closed by
  `ring`, directly or in the lemma that states it. `n_steps` and `n_lemmas` are commanded
  `exact` on the plan's step count (`length.n_steps + length.n_lemmas`) and lemma count;
  `n_tokens` is achieved only (D8).
- **Ground truth (D7).** `statement_status` is `value: true`, `method: lean-kernel`,
  `reference: <artifact_id of the control with target formal of the same grammar, length and
  seed>:syndeom_s2_grammar_ring_chain` — for every target; the formal control's reference names
  itself. The identity is true by construction (every `e_k` presents one polynomial); the
  control's `lean_build: pass` is measured and recorded in its own manifest (D8).
- **FLAW-0001 (D5).** The material is generated at the site. The site is a ring-closed chain
  step — never a lemma-cited one or a lemma step; for `local`, never the last — drawn on
  `injector/FLAW-0001/1` when the spec says `auto` (an explicit `site` is a plan step index and
  must be eligible). The perturbed monomial is drawn next on the same stream among the kind's
  eligible targets at the site's ordered terms: `sign` flips one term's sign; `coefficient`
  raises one non-constant coefficient's magnitude by one; `constant` does the same to the
  constant term, or adds `+ 1` when there is none; `exponent` raises one exponent by one where
  the raised monomial is not another term's. The site is re-rendered under its own presentation
  with the perturbed terms — the visible change is one monomial — so its identity is false and
  `ring` fails at that `calc` line (check K). `propagated` (fatal) re-derives every later step
  from the perturbed polynomial on the step's own stream and restates every lemma a later step
  cites, so every later identity is true again and the chain ends away from the claim of
  record, which is the faithful one (step 0 is untouched); `local` (repairable) changes the
  site alone, so the next step asserts the correct consequence. A spec whose severity is not
  the sub-variant's is refused. The records (one instance; a record in each file the artifact
  has) carry `perturbation`, `site`, `monomial` (`replaced`, `inserted` as signed `syndeom-lean/1`
  monomials; `replaced` null for an added constant), the site's LaTeX expression or fragment
  lines as `inserted` and `replaced`, and `original_steps` / `original_fragments` for every
  step the variant changed (a restated lemma's entry its declaration lines).
- **Refusals (D3, D6).** `n_steps` or `n_lemmas` outside D6's ranges; `n_lemmas > n_steps`; a
  positive `padding_ratio` or a FLAW-0013 spec (no padding material); every class but
  FLAW-0001; two specs; a sub-variant other than `propagated` or `local`; a perturbation outside
  D5's four; a reserved dial; a site that is not an eligible chain step; a grammar other than
  `ring_chain`; a config of the other producer's shape.
- **Versions (D1).** `GrammarVersion::V0_1_0` is current; `Version::V0_7_0` stays
  `s2_template`'s. `Producer::parse` admits exactly the eight producer versions the crate
  carries.

## Evidence that the Lean material does what the records say

1. **Probes before the module** (`evidence/lean-probe/`, hand-written; `lake env lean` from
   `lean/`, one process at a time, `run.sh`; 2026-10-29T04:17:24Z–04:20:04Z; the first probe
   took two minutes on a cold olean cache, the rest 8–9 s each):

   | Probe | Exit | What |
   |---|---|---|
   | `s2k_rc_short` | 0 | six steps on `(x + y) ^ 2`: expanded, reordered with split squares, factored by `x`, regrouped, reordered, factored by `y`; `[propext]` |
   | `s2k_rc_long` | 0 | 32 steps on `(x + y + 1) ^ 2` with two lemmas (`∀ x y : ℤ, …`, `intro x y; ring`) cited at steps 7 and 22 as terms (`_ = … := lemma x y`); `[propext]` |
   | `s2k_rc_propagated_sign` | 1 | five steps on `(x + 2 * y) * (x - y)`, the sign of `x * y` flipped at step 3 and carried through; errors at line 6 (step 3's `by ring`: `unsolved goals`) and then at line 3 (the theorem's `by`: `case calc.step`, the chain's end against the claim), in that order |
   | `s2k_rc_propagated_exponent_last` | 1 | three steps on `(x - y) ^ 2`, `y ^ 2` → `y ^ 3` at the last step; errors at line 6 then line 3 (`calc.step`) |
   | `s2k_rc_local_coefficient` | 1 | five steps on `(x + 2 * y) * (x - y)`, `x * x` → `2 * x * x` at step 3, the rest faithful; errors at line 6 (the site) then line 7 (the next step) |
   | `s2k_rc_local_constant` | 1 | four steps on `(x + y) * (x - y)`, `+ 1` added at step 2; errors at line 5 (the site) then line 6 |

   The probes fix what the generator may emit: the `calc` layout (the first step on the `calc`
   line, the rest `_ = … := …` indented two more), the lemma form and its term-mode citation,
   the leading negative term (`-2 * y ^ 2 + …`), split squares, factored and regrouped shapes,
   and the four perturbation kinds — each a false identity that `ring` leaves as an `unsolved
   goals` error at the step's own line, the first error in Lean's output. The module was written
   after they compiled; nothing in them is `s2_template` material.
2. **The goldens.** 10 cases — on `ring_chain`, the control at the short length (`n_steps` 4,
   `n_lemmas` 0; informal, formal), the control at the long length (24, 2; informal, formal),
   FLAW-0001 `propagated` `sign` (informal, formal, pair) and FLAW-0001 `local` `sign` (informal,
   formal, pair) — each produced by `generate --dry-run` with seed 7 at
   `25ad34fe45c1eea8527b3aab47829b31c8685931` by two separate runs (`evidence/goldens-run8.py`;
   run A 2026-10-29T04:35:41Z–04:38:16Z, run B 04:38:16Z–04:40:18Z; every case exit 0, fidelity
   PASS; the provenance check passed against the commit before each run;
   `evidence/goldens-run8a-summary.txt`, `evidence/goldens-run8b-summary.txt`).
   `evidence/goldens-copy8.py` compared the runs file by file — every `proof.md`, `Proof.lean`
   and `manifest.json` byte-identical, every observation equal, one engine commit — checked
   every case's shape and placed run A's outputs under `engine/generate/tests/goldens/ring_chain/`
   with a `case.json` each (`evidence/goldens-production-report.json`, SHA-256
   `6142a93977642c5ba8acc0b09d1026556cccb761ebdf537586a67713b1645e15`, 29,303 bytes):
   - at seed 7 the seed form is `(x - 2 * y) ^ 2` at both lengths (`x ^ 2 - 4 * x * y + 4 * y ^ 2`);
     the short control's chain is `4 * y * y + x * (x - 4 * y)` → `4 * y ^ 2 - 4 * y * x + x * x`
     → `x ^ 2 - 4 * y * x + 4 * y * y` → `x ^ 2 - 4 * y * x + 4 * y ^ 2`; the long control has 26
     steps (24 chain steps, lemmas 1 and 2 stated and cited), 35 lines, 599 tokens;
   - the four controls' formal files build (exit 0) and print `[propext, Quot.sound]` — within
     the standard three, where the hand-written probes printed `propext` alone;
   - the flawed cases perturb step 1 (the first `calc` line; the site drawn on
     `injector/FLAW-0001/1`), flipping `4 * y ^ 2` (rendered `4 * y * y` there) to `-4 * y ^ 2`:
     `propagated` re-derives steps 2–4 from `x ^ 2 - 4 * x * y - 4 * y ^ 2` and ends at
     `x ^ 2 - 4 * y * x - 4 * y ^ 2`, away from the claim; `local` keeps steps 2–4 faithful. The
     formal and pair cases fail to build (exit 1) with the first error at line 4, the site's
     `calc` line (check K); `lean_axioms_standard` is unmeasured (`null`, with its reason); the
     informal record spans the perturbed expression `-4 \cdot y \cdot y + x(x - 4y)` at step 1,
     `[58, 88)`; every instance is confirmed (`flaws_confirmed` 1), `proof_status: invalid`,
     `correct: false`, `lean_build` commanded `fail`;
   - every case is a true statement whose reference is the formal control of its length
     (`s2_grammar-v0.1.0-cc0c6a4daf03-s7` for the short cases, `s2_grammar-v0.1.0-6589db356c9e-s7`
     for the long ones), each naming itself;
   - the receipts, with build stdout and stderr, are under the external root
     `C:\Users\LEET\Codex\Temp\syndeom-s2-grammar-010-20261029T0430Z\goldens-8a\` and
     `goldens-8b\`; the sorted lists of the ten `receipt.json` SHA-256s hash to
     `2ecbd51bff2b9e9d29dfa89dd72630526382e584aa9cb9c00278b2f6c884a13d` (A) and
     `251fd79c8cab80576aa32c5a8d38eda5cab836eadcd3722a0de70b5500406fb3` (B).
3. **The artifact ids** (`s2_grammar-v0.1.0-<config_hash[0:12]>-s7`): control short
   `82300f31bd83` / `cc0c6a4daf03` (informal / formal); control long `08a5df565ca1` /
   `6589db356c9e`; FLAW-0001 `propagated` `a1078e68fb54` / `323a85437720` / `e6af9218f235`
   (informal / formal / pair); FLAW-0001 `local` `24cd510eafb6` / `5fc890d89006` / `bb491aa2dbdd`.
4. **Tests and lint.** The crate's 46 tests pass (goldens 1 over 79 cases, grammar 11,
   properties 24, provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features --
   -D warnings` and `cargo fmt --check` are clean. New, in `tests/grammar.rs`: the seed forms
   (both variables, at least two monomials, canonical, the renderings of six forms, the
   expansions of two); the monomial and shape renderings as the probes were written (split
   squares, `y` first, constants, factored first and last, grouped, the leading negative term,
   the resolutions to expanded); the derivation's invariants over seven seeds and six lengths
   up to (256, 32) (every step presents the polynomial and differs from the step before; the
   lemma sites distinct, ascending, within the chain; every seed form drawn over 200 seeds);
   the plan (lemmas first with their names, claims and proofs; the chain's `calc` lines, the
   cited steps' terms, texts and premises; both files parse with the recorded counts); the
   control and its reference (valid, correct, unflawed; `lean-kernel`; the reference the formal
   control's id, which names itself; independent of the number form of the config); determinism,
   idempotence and the dispatch (`prepare_any`, `prepare_for`, `Producer::parse`); the
   perturbation targets and their effect (every target changes the polynomial over every seed
   form); FLAW-0001 over three lengths × four kinds × two sub-variants × three targets (the site
   a ring-closed chain step, `local` before the last; step 0 untouched; the dials; `local`
   changes the site alone, `propagated` every later step and the lemmas cited after the site;
   one instance, two locations, confirmed under the predicted K; a passing build or an error
   elsewhere a FAIL verdict); explicit sites (eligible ones accepted, a lemma step, a cited step,
   the last step for `local` and a step past the chain refused); the refusals (ranges, more
   lemmas than steps, another grammar, padding, other classes, two specs, severities,
   sub-variants, perturbations, reserved dials, the config's shape, an `s2_template` config);
   and a 256-step instance with 32 lemmas, rendered and measured, with FLAW-0001 on it. The
   gate on the committed tree is recorded in the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-10-29T04:35:09Z, outside the gate (`cargo audit --file
engine/Cargo.lock`). It loaded 1,381 advisories (advisory-db commit
`40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`), scanned 35 crate dependencies, and reported no
vulnerability (exit 0; `evidence/cargo-audit.txt`). `engine/Cargo.lock` is unchanged by this
revision: no dependency was added.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `25ad34fe…` — `engine/generate/src/grammar.rs` (new: the polynomial
arithmetic, the seed form table, the renderers, the presentations, the derivation, the plan, the
config, FLAW-0001), `src/template.rs` (`Producer`; `Prepared.producer`; `counts` by a flag;
`exact_keys`, `parse_spec`, `check_spec`, `unique_span` and `Target`'s helpers made
crate-visible), `src/lib.rs` (`prepare_any`, `prepare_for`), `src/main.rs` (`producer_of`, the
dispatch), `tests/goldens.rs`. The threat model is packet 1's: an offline research CLI whose
assets are the integrity of generated ground truth and the operator's machine, and whose trust
boundary is an artifact directory handed to `verify` and a config handed to `generate`. New this
revision: a config selects a producer by its keys and carries two integers (`n_steps`,
`n_lemmas`) that size a generated derivation; a free-text `grammar` and a `perturbation` dial
select among constants; the producer computes with small integers.

**Pre-scan (Phase 1):** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate
roots and in the manifest crate; no external dependency was added; clippy at `-D warnings` is
clean (two `type_complexity` lints were resolved with aliases during the session, before the
commit). `cargo geiger`, `cargo deny` and Miri are not installed on this machine and were not
run; there is no `unsafe` to map.

**Panic and index sweep of the diff (2C):** no `unwrap`, `expect` or `panic!` in `grammar.rs`
or the edited files outside tests. The indexes: `spec.dials["perturbation"]` sits behind
`exact_keys`; `obj["grammar"]`, `obj["length"]`, `length["n_steps"]`, `length["n_lemmas"]`,
`obj["target"]`, `obj["padding_ratio"]`, `obj["flaws"]` sit behind `exact_keys` on their
objects, and a `Value` index of a missing key yields `Null`, which the `as_*` conversions
refuse; `eligible[stream.below(len)]` and `candidates[stream.below(len)]` are below the length
of lists checked non-empty; `d.e[c]`, `d.e[c - 1]` and `d.presentations[c - 1]` take `c` from the
eligible list, within `1..=n`; `plan.steps[m + c - 1]` and `faithful.steps[site - 1]` lie within
a plan of `m + n` steps; `styles.get(p)` defaults; `(k as usize).saturating_sub(m)` cannot
underflow; `stream.below(m.saturating_sub(1).max(1))` never draws below 0; `pres.order` is a
permutation of `0..m` applied to `m` terms. Integer arithmetic (2I): coefficients come from the
fixed table (magnitude at most 5, products at most 25 after `poly_mul`/`poly_pow` on the table's
own forms) and change by ±1 under a perturbation; exponents are at most 4; `n_steps` and
`n_lemmas` are bounded to 256 and 32 before any use. No input-controlled quantity enters an
arithmetic operation.

**Serde and the config (2F):** the config is parsed by hand with exact key sets at both levels
(`exact_keys` on the config and on `length`), integers via `as_u64` (a negative, fractional or
non-numeric value is refused), the flaw specs via `parse_spec` with its reserved-dial check; a
config with both or neither of the `theorem` and `grammar` keys is refused by `prepare_any`.
`Producer::parse` admits exactly the eight producer versions the crate carries.

**Subprocesses and `verify`:** unchanged. `verify` compiles only bytes the producer the manifest
names itself emits for the manifest's `(config, seed)`, and the audit tail names only that
producer's main declaration (FINDING-011's guard holds for `syndeom_s2_grammar_ring_chain`, a
constant). No config text reaches the proof text: `grammar` is compared to a constant,
`perturbation` to four constants, the numbers are bounded, and every byte of both files comes
from the fixed table and the renderers.

**Determinism:** `BTreeMap` and `Vec` only; no floating-point value enters a decision
(`padding_ratio` is compared to 0 only to refuse it); every draw is labelled.

```
FINDING-018: `grammar::derive`, `plan_of` and `render` trust their callers' preconditions
Severity:    Informational
Category:    Panic
Location:    engine/generate/src/grammar.rs (derive, Derivation::n, render)
Status:      Open (backlog)
Description: `derive(n, m, seed)` panics when `m > n` (it removes from an emptied list after
             `below(0)`), `Derivation::n` underflows on an empty `e`, and `render` indexes
             `terms` through a permutation of another length. `prepare_with` validates every
             config before calling them and the CLI reaches them only through it; the tests call
             them with valid arguments. They are public for the tests' sake.
Recommendation: make the preconditions explicit — return `Result` from `derive` and `render`,
             or assert them with a message — when the next grammar version touches these
             functions.
```

FINDING-017 (informational, backlog: the FLAW-0008 drift rules are textual) stands; the grammar
generates no drifted statement in 0.1.0. FINDING-015 (a composite's truth witness taken on the
seed's word) does not extend to the grammar: a grammar instance's truth rests on the
construction (every step presents one polynomial) and on the control's measured build, recorded
in the control's own manifest, not on authored material. FINDING-016 (informational, backlog)
stands. FINDING-006 (accepted risk), FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012
and FINDING-013 (accepted) stand as packets 1–7 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 7's numbering.

85. **The length dials count the chain and the lemmas separately.** `length.n_steps` is the
    number of equalities in the `calc` chain and `length.n_lemmas` the number of lemma steps,
    so the plan has `n_steps + n_lemmas` steps and the manifest commands `n_steps` as that sum
    (D8's "the plan's step count", the template's rule applied unchanged) and `n_lemmas` as
    `length.n_lemmas`. The alternative — `length.n_steps` as the total, so that a chain of one
    step with one lemma is impossible — is listed, not taken; D6's independent ranges (1 to 256,
    0 to 32) read as independent counts.
86. **A lemma states one chain step and is cited as a term.** The chain step's `calc` line is
    `_ = e_k := …_lemma_<j> x y`, so the lemma is genuinely used and every equality of the chain
    is closed by `ring` — directly, or in the lemma that states it. The alternative — every step
    `by ring` with the lemma cited only in the informal text — would leave an unused declaration
    in a control, FLAW-0007's shape.
87. **Lemma steps come first in document order,** numbered by the ascending order of the chain
    steps they state (`grammar/lemma/<j>` draws among the steps not yet taken, then the sites
    are sorted), so Lemma 1 is the first cited.
88. **The presentations are the grammar's rewrites,** and repeats are allowed. A step is a
    fresh draw (a permutation, a style per term, a shape), not a local rewrite of the step
    before; consecutive steps must differ (redrawn, at most 32 times, then a deterministic
    fallback), earlier forms may recur — a long chain of a polynomial with few presentations
    cycles through them. The LaTeX rendering juxtaposes factors (`4yx`, `x(x - 4y)`) and writes a
    split square with `\cdot` (`4 \cdot y \cdot y`).
89. **The seed form table is the atom library's monomials and coefficients:** fourteen
    products and powers of binomials and trinomials with coefficients and constants in {1, 2, 3,
    5}, every form using both variables (so no binder is unused and no linter warning arises;
    Al's note on reading 73), every expansion of at least two monomials.
90. **The truth reference names the formal control for every target** (informal, formal,
    pair): the one control whose `lean_build` is measured. The formal control's reference names
    itself. The reference is an artifact id, so it resolves only where that control was
    generated (the sweep generates every control; among the goldens the short and long formal
    controls are present).
91. **The one import is `Mathlib.Tactic.Ring`:** the ℤ notation and `ring` arrive with it (the
    probes). D6's "minimal `Mathlib.*` modules" is read as that one module.
92. **FLAW-0001's site is a ring-closed chain step.** Never a lemma-cited step (a perturbed
    cited step would fail inside the lemma's declaration, before the main declaration, so the
    first error would not be at the site and K would not hold) and never a lemma step; for
    `local`, never the last step (no next step to assert the correct consequence). An explicit
    `site` is a plan step index (the lemma steps counted), as the template's sites are step
    indices of the argument.
93. **The perturbation acts on the site's ordered terms and the site is re-rendered under its
    own presentation,** so the visible change is one monomial. `coefficient` on a coefficient of
    1 renders `x ^ 2` as `2 * x ^ 2`; a negative coefficient grows in magnitude (`-2` → `-3`),
    never toward zero; `constant` on a polynomial without a constant term adds `+ 1` (the
    perturbation of a constant that is not there); `exponent` excludes a raise whose monomial is
    already another term's (a collision would merge terms and hide the slip; such a target is
    ineligible, and a site without any eligible target is refused).
94. **`propagated` re-derives, `local` leaves a second false step.** `propagated` presents the
    perturbed polynomial afresh at every later step on that step's own stream (not the faithful
    presentations re-edited), and restates every lemma a later step cites — a lemma is logically
    later than the site although it precedes it in document order — so every later identity is
    true again, the chain ends away from the claim of record, and Lean reports the ring failure
    at the site and then the theorem-level `calc.step` goal (the probes and the goldens). `local`
    changes the site alone: the next step's identity `e_s' = e_{s+1}` is then false too (or its
    citation mismatches), so Lean reports a second error at the next line; the first error is at
    the site, which is what check K reads, and "the next step asserts the correct consequence" is
    satisfied by the faithful next step. The template's FLAW-0001 `local` had one error because
    its next step was provable on its own; a `calc` chain's next step cannot be.
95. **The record's dials add `monomial`** (`replaced`, `inserted`: the signed monomial in
    `syndeom-lean/1` form; `replaced` null for an added constant) beside `perturbation`, `site`,
    `inserted`/`replaced` (the site's LaTeX expression; the site's fragment lines) and
    `original_steps`/`original_fragments`; a restated lemma's `original_fragments` entry is its
    declaration lines (`lemma_lines`), the changed formal text being its header, where the
    template records a changed step's fragment.
96. **Severity is fixed by the sub-variant** (`propagated` fatal, `local` repairable, D5's row)
    and a spec with another severity is refused rather than realized.
97. **The grammar's remaining v0.1 classes are later versions.** 0.1.0 refuses every class but
    FLAW-0001, a positive `padding_ratio` and FLAW-0013 (no padding material), `n_lemmas >
    n_steps`, and the ranges' complements; FLAW-0003 `cycle`, FLAW-0004, FLAW-0007 `unused`,
    FLAW-0009, FLAW-0010 `a`, FLAW-0011 and FLAW-0013 — D6's list for v0.1 grammars — come with
    later grammar versions, as the template's classes came version by version.
98. **`generate` selects the producer by the config's key** (`theorem` vs `grammar`; both or
    neither refused), the reading STATE's NEXT asked to flag; `regenerate` and `verify` by the
    manifest's `generator.id` and `generator.version`. `Prepared` carries a `Producer` and one
    `assemble` serves both producers (D2's "the two producers differ only in `plan`"); `counts`
    takes a flag in place of the template version.
99. **The goldens' lengths:** the controls at (4, 0) and (24, 2); FLAW-0001 at (4, 0) with the
    `sign` perturbation (FLAWS.md's FLAW-0001 example is a sign error), so the flawed goldens'
    truth reference names the short formal control golden; the lemma interplay of a flawed
    instance is covered by the tests, not by a golden.
