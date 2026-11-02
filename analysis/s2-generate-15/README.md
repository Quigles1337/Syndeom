# S2 implementation packet 15 — `s2_grammar` 0.8.0 (ADR-0011, Session 54, 2026-11-02)

This packet records the fifteenth implementation session of ADR-0011 (D11): `s2_grammar` 0.8.0,
the second grammar, `ineq_chain` (D6), with its control and FLAW-0001 `propagated` and `local`,
built probes first, as STATE's NEXT and Al's rulings on PR 39 ordered. It also holds the probes,
the goldens' production and the mutation checks. Nothing here is a result: no CLAIM, no fidelity
sweep, no artifact under `corpora/`.

Branch `claude/s2-grammar-0.8.0`, from `main` `009bb85e6dad266b47257b7a36d688a35ced5f52` (PR 39's
merge):

| Commit | What |
|---|---|
| `677c8165fa94260d1ac2842c2db2be57be5a6daa` | the 0.8.0 producer: `src/ineq.rs` (new), `src/grammar.rs` (the version, `grammars`, `realizes_on`, the dispatch, `control_reference`), `src/lib.rs`, `tests/ineq.rs` (new), `tests/grammar.rs`, the crate README |
| `d40788c7c5e747af8c7cc7acea3f13936b02e7ca` | the 10 goldens of `s2_grammar` 0.8.0, produced at `677c8165…`, the goldens README and the count (120 cases) |
| the record commit | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 54 |

## The probes (STATE's NEXT, step 3)

`evidence/lean-probe/`. The shapes were written by hand before any design in the crate. A small
authoring aid, `make_probes.py` (no result authority, SD10), computes the merged normal forms of a
few hand-chosen chains and writes each probe. `run.sh` (byte-identical to packet 8's) compiles
every probe one Lean process at a time from `lean/`, and each output lands beside its probe.

- **Run 1** (2026-11-02T21:46:32Z–21:48:08Z; `run-1/`, with `run-1/probes-1.log`) imported
  `Mathlib.Data.Real.Basic`. Every file built or failed as below, but each printed a warning: the
  pinned Mathlib marks that module `deprecated_module (since := "2026-08-27")`, a shim that
  re-exports `Mathlib.Basic.Real.Basic`. The outputs name the probes at their first location;
  they were moved into `run-1/` afterwards, unchanged.
- **Run 2** (21:50:32Z–21:53:05Z; the files at the top level, with `probes-2.log`) imports
  `Mathlib.Basic.Real.Basic` and builds without the warning. It adds two long controls.

| Probe | What it shows | Run 2 |
|---|---|---|
| `s2i_ic_atoms` | each atom stated alone; each family closing a merged chain step by `linarith [<lemma>]`; `nlinarith` with the hint; `nlinarith` and `linarith` with no hint | the hinted steps all build; `nlinarith` with no hint fails (line 20), and `linarith` with no hint fails on the AM-GM and the triangle step (lines 22, 24), as marked |
| `s2i_ic_positivity` | `positivity` on `0 ≤ (x - y)^2` and `0 ≤ 2(x - 1)^2 + \|x + y\|`, and on a chain step | the two `0 ≤` goals build; the step fails, "not a positivity goal" |
| `s2i_ic_imports_real_only` | the reals without `Mathlib.Tactic.Linarith` | `linarith`: "unknown tactic" |
| `s2i_ic_imports_linarith_only` | `Mathlib.Tactic.Linarith` without the reals | no `HAdd ℝ`, `HPow ℝ ℕ ℝ` … instances |
| `s2i_ic_short` | a 4-step control (AM-GM, triangle, square, AM-GM) | builds; `[propext, Classical.choice, Quot.sound]` |
| `s2i_ic_long` | a 12-step control with two cited lemma steps | builds; the same three axioms |
| `s2i_ic_control_64`, `s2i_ic_control_256` | long controls, the atoms in a fixed stride through the table | build, in 11 s and 43 s; the 256-step file is 262 lines, its longest line 175 characters |
| `s2i_ic_propagated_sign` | step 2's `2xy` flipped, the later steps carrying it | errors at line 6 (the site), then line 4 (`calc.step`) |
| `s2i_ic_local_sign` | the same site, `local` | line 6, then line 7 (the next step, false) |
| `s2i_ic_local_sign_abs` | step 2's `\|x\|` flipped, `local`; the next step stays true | line 6, then line 7: the next step's hint certifies only its faithful increment |
| `s2i_ic_local_coefficient` | step 3's `-2x` raised to `-3x`, `local` | line 7 (the site), then line 8 |
| `s2i_ic_propagated_exponent_last` | the last step's `3x^2` raised to `3x^3` | line 8 (the site), then line 4 |
| `s2i_ic_local_constant_true` | step 3's constant raised from 1 to 2: the step stays true | **line 8 only**: the site builds and the first error moves to the next line |

What the probes decided:
- **Which tactic closes which atom:** `linarith` with the atom's lemma as the one hint closes
  every family — `sq_nonneg (L)` for a square, `two_mul_le_add_sq u v` for AM-GM (its `2 * x *
  1` and `1 ^ 2` normalize), `abs_add_le x y` for the triangle inequality. `nlinarith` adds
  nothing and fails without a hint; `positivity` does not take a step `a ≤ b`.
- **The imports:** `Mathlib.Basic.Real.Basic` and `Mathlib.Tactic.Linarith`; each is needed.
- **Check K:** in every falsifying shape the first error, in output order, is at the site's own
  `calc` line, with no cascade before it. Under `propagated` the theorem-level `calc.step` goal
  follows; under `local` a second error followed in every probe, because the next step's hint
  certifies only the faithful increment.
- **A perturbation that leaves the step true is no flaw:** its step builds and the first error
  moves (`local_constant_true`), so check K would fail. The design admits only perturbations
  whose step is false (below).

## What was built

- **The grammar `ineq_chain`** (`engine/generate/src/ineq.rs`; ADR-0011 D6, D7):
  - a config `{"grammar": "ineq_chain", "length": {"n_steps", "n_lemmas"}, "target", "flaws",
    "padding_ratio"}` with `ring_chain`'s ranges and checks, names an inequality `e_0 ≤ e_n` over
    ℝ in `x` and `y`;
  - chain step `k` adds one atom drawn on `grammar/ineq_chain/step/<k>` (`below(10)`), so `e_k =
    e_{k-1} + increment` and every step is true by construction;
  - the atom library `ATOMS` (reading 175): six squares (`sq_nonneg`), three two-term AM-GM
    instances (`two_mul_le_add_sq`), the triangle inequality for `x` and `y` (`abs_add_le`);
  - `e_0` is the sum of the sources the AM-GM and triangle steps consume (reading 176);
  - expressions are merged normal forms, monomials by total degree and then the exponent of
    `x`, then `|x|`, `|y|`, `|x + y|` (reading 174);
  - the plan: `n_lemmas` steps stated as lemma steps (`grammar/ineq_chain/lemma/<j>`) and cited
    as a term, as `ring_chain`'s; each other step `_ ≤ e_k := by linarith [<lemma>]` (reading
    177); the informal step names its atom ("Since squares are non-negative, $(x - 1)^2 \ge 0$,
    so $a \le b$.");
  - ground truth: true by construction, the reference the formal control of the same length and
    seed (D7).
- **FLAW-0001 `propagated` and `local`, read for inequalities** (D5; readings 179–184): the four
  kinds are `ring_chain`'s operations on the site's right side; a target is admitted only when a
  point of the witness grid shows the perturbed step false and its expression occurs once in the
  site's text; the site, then the target, are drawn on `injector/FLAW-0001/1`; the term is
  perturbed in place; `propagated` carries the change through every later increment, `local`
  changes the site alone. The record's `witness` dial holds the point and both sides' values as
  exact fractions. `constant` is never admitted, because every constant of a chain is positive
  (reading 180).
- **`src/grammar.rs`:** `GrammarVersion::V0_8_0` (`generate` emits 0.8.0); `grammars` (the
  grammars a version carries); `realizes_on` (`ring_chain`'s classes as `realizes`, unchanged in
  0.8.0; `ineq_chain` FLAW-0001 only); the dispatch to `ineq::prepare` after the shared checks;
  `sum_join` crate-visible; `control_reference` naming each grammar's main declaration (reading
  185). `ring_chain`'s bytes and draws under 0.8.0 are its 0.7.0 ones (reading 188).
- **Refusals** on `ineq_chain`: every class but FLAW-0001, padding, two specs, the `constant` kind,
  a site with no admitted target, the existing FLAW-0001 refusals; the grammar under 0.1.0 to
  0.7.0.

## The goldens (STATE's NEXT, step 5; reading 186)

The controls at (4, 0) and (24, 2), informal and formal, and FLAW-0001 `propagated` and `local`
with the `sign` perturbation at (4, 0), informal, formal and pair, all at seed 7. They were
produced by `generate --dry-run` at `677c8165…` (`evidence/goldens-run15.py`):

| Run | Directory under the external root | When (UTC) | Result |
|---|---|---|---|
| A | `goldens-15a` | 22:05:29Z–22:07:56Z | 10 cases, exit 0, fidelity PASS (`evidence/goldens-run15a-summary.txt`) |
| B | `goldens-15b` | 22:07:56Z–22:08:35Z | **refused from the third case** (exit 2): "engine_commit does not identify the checked-out producer input engine/generate/tests/goldens.rs" — the file had been edited while the run ran (`evidence/goldens-run15b-summary.txt`; HANDOFF Session 54, CONFUSIONS) |
| B2 | `goldens-15b2` | 22:09:07Z–22:11:35Z | after the committed `goldens.rs` was restored (blob `78ab74bf…`, the edit set aside and restored afterwards): 10 cases, exit 0, fidelity PASS (`evidence/goldens-run15b2-summary.txt`) |

`evidence/goldens-copy15.py` compared A and B2 byte for byte: every file and observation is
identical, there is one engine commit, and every verdict is PASS. It checked each case's shape and
placed A's outputs under `engine/generate/tests/goldens/ineq_chain/`. Its report is
`evidence/goldens-production-report.json` (SHA-256
`5bf425fe2e6b0a9d884197954685b887571389462f3220d670d619c4f14f235f`, 40,944 bytes). The sorted
`receipt.json` concatenations of A and B2 hash to
`d133a95de4aee530df90a899c7dded73bdb809ba30836c7d8c0b4d4798b50aae` and
`1c0212cb819f051929c33e6fa5f6931afc9d75117e1b82b0f6137586b6e8b0d8`; the receipts carry durations,
so they differ.

| Case | Artifact id | What it holds |
|---|---|---|
| control short, informal / formal | `…-f539c4579bbc-s7` / `…-32dcf77f7873-s7` | four squares at seed 7: `0 \le 2x^2 + 2xy + 3y^2 - 2x + 3`; 122 informal tokens; the formal one builds with the standard three |
| control long, informal / formal | `…-a54a1ff89bfd-s7` / `…-5863c0b40ac8-s7` | 24 steps, 2 lemmas; 1,190 informal tokens; builds with the standard three |
| FLAW-0001 `propagated`, informal / formal / pair | `…-5b011e904f8e-s7` / `…-dcae41107195-s7` / `…-08aea335a7bc-s7` | the site step 1, its constant's sign flipped (`1` to `-1`; span `[61, 73)`, line 5), witness `(0, 0)` with left `0` and right `-1`; the chain ends at `2x^2 + 2xy + 3y^2 - 2x + 1`; builds fail at line 5, then line 4; 114 tokens |
| FLAW-0001 `local`, informal / formal / pair | `…-fdf47e446e6f-s7` / `…-68ede8a10a36-s7` / `…-5ef7b70d1089-s7` | the same site, term and witness; the build fails at line 5 only, because the next step (`y^2 + 2y - 1 \le x^2 + y^2 - 2x + 2y + 2`) is true and its hint proves it (reading 181); 122 tokens |

The ids are `s2_grammar-v0.8.0-<config_hash[0:12]>-s7`. Every flawed case names the short formal
control as its truth's reference.

## Evidence that the material does what the records say

- **Tests** (`tests/ineq.rs`, 12; `tests/grammar.rs`, 18, its version facts at 0.8.0; the crate's
  65 in all):
  - the atom library: each square's increment equals the square of its linear form at 169
    points, exactly; each AM-GM increment is `(u - v)^2` and its source `2uv`; the triangle
    atom; no grid point falsifies any atom;
  - the rendering of expressions as the probes wrote them;
  - the derivation over 40 seeds and five lengths, up to 64 steps: one draw per step on its own
    label; `e_0` the sum of the sources; every faithful step true at every grid point; every
    constant positive; the lemma sites;
  - the plan's two files written out from the derivation and compared byte for byte;
  - the controls' ground truth and the truth reference;
  - determinism, the dispatch and the version;
  - **admitted iff falsified:** over 12 seeds, at every step of a 12-step chain, every
    candidate target of every kind is evaluated by a separately written exact evaluator. A target
    is admitted if and only if a grid point falsifies its step, except where its expression is not
    unique in the site's text. Each admitted target's witness is verified, and `constant` is
    never admitted;
  - FLAW-0001 over six seeds, three lengths, the four kinds, both sub-variants and three
    targets: the records, the witness, the plan changes, the restated lemmas, check K under
    the predicted observation, and FAIL when the build passes or fails first elsewhere (at least
    40 flawed artifacts built);
  - explicit sites; the refusals (the other classes, padding, two specs, FLAW-0001's own, the
    `constant` kind over 20 seeds and three lengths, ineligible explicit sites); a 256-step chain
    with 32 lemmas, whose lines the test bounds at 400 characters; `ring_chain`'s 0.7.0 bytes
    under 0.8.0 for five configs, the composite among them.
- **Mutations** (`evidence/mutations/`, `mutate15.py` with three logs). Each mutation was applied
  to `ineq.rs` alone, the `ineq_chain` tests run, and the source restored byte for byte (SHA-256
  `430d339e…57e7` each time):
  - the witness filter removed: three tests fail;
  - the first square's increment changed (`-2xy` to `-3xy`): five tests fail;
  - `propagated` stopped carrying the perturbation forward: the FLAW-0001 test fails.
- **The 110 earlier goldens** regenerate byte for byte under the versions they name.

## `cargo audit` and `cargo deny check` (D10)

- `cargo audit` (0.22.2; 2026-11-02T22:07:48Z; advisory-db `263a4d951a0867d4bddc29dddd6d59427c78f9a7`,
  1,402 advisories; 35 crate dependencies) found no vulnerability (`evidence/cargo-audit.txt`).
- `cargo deny check` (0.20.2; 22:07:50Z): advisories, bans, licenses and sources ok
  (`evidence/cargo-deny-check.txt`).
- `engine/Cargo.lock` is unchanged since `main`, so `engine/deny.toml` is unchanged.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `677c8165…` against `009bb85…`: `engine/generate/src/ineq.rs` (new), and
`src/grammar.rs` and `src/lib.rs`. The threat model is packet 1's. The crate is an offline
research CLI, and its assets are the integrity of generated ground truth and the operator's
machine. New in this revision are a second grammar's derivation from the seed, a perturbation
filter evaluated in exact integer arithmetic, and a new dial carrying a witness.

**Pre-scan (Phase 1):**
- `cargo audit` and `cargo deny check` are clean, with no dependency added.
- `#![forbid(unsafe_code)]` stands in `lib.rs`, and `ineq.rs` has no `unsafe` token.
- clippy at `-D warnings` is clean. `clippy::pedantic`, read as information, puts 43 warnings on
  `ineq.rs`, all of style: `must_use` candidates, 32-bit truncation notes on `u64` dials already
  bounded at 256, single-character names, a long function, `clone_from`, and `u128`-to-`i128`
  notes on a gcd no larger than its operands. None concerns behavior.
- `cargo-geiger` is not adopted (Al's ruling on PR 37). Miri is not available for the pinned
  toolchain.

**Panic and index sweep (2C).** `ineq.rs` has no `unwrap`, `expect`, `panic!`, `todo!`,
`unimplemented!` or `unreachable!`. Its indexes:
- `ATOMS[at]` takes `at` from `below(ATOMS.len())`;
- `d.e[c - 1]`, `d.e[c]` and `d.atoms[c - 1]` take `c` in `1..=n`, as validated by
  `prepare_with`;
- the chosen candidate's index is drawn below the length of a list `admits` has just found
  non-empty, and the same pure function recomputes it;
- `spec.dials["perturbation"]` (a `serde_json::Map` index, which panics on a missing key) sits
  behind `exact_keys`;
- `scaled` shifts by `d - i - j` and `d - 1`, with `d` at least every term's degree and at
  least 1 by construction.

**Integer arithmetic (2I).** The coefficients are small. Each step changes a coefficient by at
most 2 and adds at most 2 to `e_0`'s sources, so at 256 steps no coefficient exceeds 1,024 in
magnitude. Exact evaluation uses `i128` at `|x|, |y| ≤ 2` and degree at most 3. A sign flip
negates a small coefficient, never `i64::MIN`.

**Serde and the config (2F).** No new config key. FLAW-0001's dials are exact (`perturbation`
only). `witness` and `monomial` are written by the injector and refused as config dials by the
exact-keys check. No floating-point value enters a generation decision: the witness is exact
rationals.

**Ground truth (logic, SD1).** A faithful step is true because each increment is a square, `(u -
v)^2` or `|x| + |y| - |x + y|`; this is tested exactly at 169 points per atom. A FLAW-0001
instance is false by its recorded witness, so `linarith` cannot prove it. Lean's soundness makes
it `YES-KERNEL`, and its first error is at the site, because every earlier step and every
restated lemma is true with its hint. It is confirmed only by check K on the observed build.

**Determinism.** `Vec` and stable sorts on total keys only; one new label per step and per
lemma; the grid's order is total.

No new finding. **FINDING-018** (informational, backlog: `derive`, `Derivation::n` and `render`
trust their callers' preconditions) stands. Its scope gains the new module's
`ineq::derive` (`m > n` empties its list), `ineq::plan_of` and `ineq::site_candidates` (`c` in
`1..=n`), and `ineq::witness` (degrees at most 3, which no producer path exceeds). The
recommendation is unchanged, and if acted on it covers both grammars at once. FINDING-015 does not
extend to the grammar (packet 8). FINDING-016 and FINDING-017 (informational, backlog) stand.
FINDING-006 (accepted risk), FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012 and
FINDING-013 (accepted) stand as packets 1–14 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 14's numbering.

174. **The chain's design: each step adds one atom to a merged normal form.** `e_k = e_{k-1} +
     increment`, with like terms merged, so a line does not grow with the chain (175 characters
     at 256 steps). The alternative — each step rewriting one summand of a growing sum — makes
     the file quadratic in the length (about a megabyte at 256 steps) and was not built. The
     cost of the merge is that a reader sees each atom's effect as changed coefficients rather
     than as a rewritten term.
175. **The atom library.** Six squares, three two-term AM-GM instances and the triangle
     inequality for `x` and `y`. AM-GM's increment equals a square's (`2xy ≤ x^2 + y^2` adds
     `(x - y)^2`): two atoms with two justifications and one increment, so detectors cannot tell
     them apart by the arithmetic. The triangle inequality has one instance; more pairs (`|x -
     y|`, `|x + 1|`) need lemma forms whose sides `linarith` matches syntactically, a later
     version if S3 asks.
176. **`e_0` is the sum of the sources.** It shows how many AM-GM and triangle steps the chain
     takes. An all-squares chain states `0 ≤ …`, as seed 7's short chain does.
177. **`linarith` alone.** D6 names `linarith`, `nlinarith` or `positivity`. The probes show
     `linarith` with the atom's lemma closes every step, `nlinarith` adds nothing and fails
     without a hint, and `positivity` does not take a step `a ≤ b`. The hint is visible in the
     formal file, one lemma per step.
178. **The imports.** `Mathlib.Basic.Real.Basic`, not the deprecated `Mathlib.Data.Real.Basic`
     (both resolve at the pin; only the former builds without a warning), and
     `Mathlib.Tactic.Linarith`. Both are needed.
179. **The site is drawn among admitting steps.** `ring_chain` draws the site among its eligible
     steps and refuses afterwards when the kind has no target there. `ineq_chain` draws among the
     steps at which the kind has an admitted target, so `auto` refuses only when no step admits
     the kind. An explicit site without an admitted target is refused.
180. **`constant` is never admitted on `ineq_chain` 0.8.0.** `ring_chain`'s operation — raise the
     constant's magnitude, or add `+ 1` — only enlarges a right side whose constants are all
     positive, so the step stays true. The kind is refused rather than redefined. If Al wants a
     constant slip on inequalities, the readings available are lowering the constant (an
     off-by-one bound, FLAWS.md's own phrase) or atoms that carry negative constants; either is a
     later version.
181. **`local`'s second error is likely, not necessary.** Al's ruling on reading 94 records that
     in a `calc` chain of identities a local perturbation necessarily produces a second kernel
     error. On inequalities the next step asserts the faithful consequence, and Lean fails there
     only when that step's hint cannot certify it. The probes show a second error even when the
     next step is true. Seed 7's `local` golden shows none, because its perturbation lowered the
     right side by a constant the next step's hint absorbs. The instance is one either way, and
     its site is the first error. S3 reads a count of errors as a count of lines.
182. **`monomial` names the perturbed term**, which on `ineq_chain` may be an absolute value
     (`2 * |x|`) or the constant (`1`). The key is kept so that FLAW-0001's record has one shape
     across the grammars.
183. **The uniqueness filter.** A target whose perturbed expression occurs more than once in the
     site's text is not admitted, so the informal record's span is unique. `ring_chain` refuses
     such a draw after the fact. The test shows the filter excludes nothing else.
184. **The witness and its grid.** `x` and `y` take half-integers from -2 to 2 (81 points), in
     order of `|x| + |y|`, then `x`, then `y`, with exact evaluation. A falsifying perturbation the
     grid misses would not be admitted, which is conservative; the test found none. The
     witness is recorded in both records' dials as a check a reader can redo by hand.
185. **The truth reference names each grammar's own main declaration**
     (`syndeom_s2_grammar_<grammar>`). For `ring_chain` this is the old constant, so no byte
     changes.
186. **The goldens:** as `ring_chain` 0.1.0's — two controls and FLAW-0001 `propagated` and
     `local` with `sign` at the short length, informal, formal and pair. At seed 7 both flaws
     fall on the constant of step 1, so neither golden shows a monomial or an absolute value
     perturbed; the tests cover those.
187. **`Classical.choice` in the axiom print.** The reals bring it. It is one of the standard
     three (SD2(ii)), so `lean_axioms_standard` is commanded and achieved `true`.
188. **The version bump.** A new grammar is new bytes for new configs, so `s2_grammar` becomes
     0.8.0. `ring_chain`'s 0.8.0 bytes and draws are its 0.7.0 ones; only the version, the id
     and the manifest differ.
