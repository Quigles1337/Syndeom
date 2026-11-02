# S2 implementation packet 10 — `s2_grammar` 0.3.0 (ADR-0011, Session 49, 2026-11-01)

This packet records the tenth implementation session of ADR-0011 (D11): `s2_grammar` 0.3.0 —
FLAW-0009 `axiom` on the grammar `ring_chain`, with material generated from the atom library
(D5, D6), `truth: true` and `truth: false` — with its goldens. Nothing here is a result: no CLAIM,
no fidelity sweep, no artifact under `corpora/`.

Branch `claude/s2-grammar-0.3.0`, from `main` `5909e446b5c4e0008781890e24ef859f131659c7` (PR
34's merge):

| Commit | What |
|---|---|
| `d3c3851bb992bb7376631b1868db8d0c06d001d5` | the 0.3.0 producer: `src/grammar.rs` (`smuggle`, `eval`, `witness_point`, `chain_line`, the version), `src/lib.rs`, `tests/grammar.rs`, the crate README |
| `c4fb405e6d7e224a31738a81667d1edabfd5ac5d` | the 6 goldens of `s2_grammar` 0.3.0, produced at `d3c3851b…`, the goldens README and the count (93 cases) |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 49 |

## What was built

- **FLAW-0009 `axiom` on `ring_chain` (D5).** The config's dials are `mode` (`axiom` only) and
  `truth` (`true`: gap; `false`: fatal; a severity other than the truth's is refused); the
  target is `formal` or `pair` (`informal` is refused by D3, as for the template);
  `length.n_lemmas` is at least 1, since the class replaces a cited lemma's declaration. The
  lemma is drawn on `injector/FLAW-0009/1` among the `n_lemmas` lemma steps (`1 + below(m)`)
  when the spec's `site` is `auto`; an explicit `site` is a lemma step's plan index (1 to `m`)
  and skips the draw. The lemma step keeps its informal text and becomes an `axiom` with no
  proof (`LemmaKind::Axiom`, as `s2_template` 0.3.0 renders it). **`truth: true`:** the axiom
  states the lemma's own identity; nothing else changes. **`truth: false`:** the axiom states a
  false identity — the lemma's right-hand side (the chain step it states, `e_{c-1} = e_c`, with
  `e_c` perturbed once as FLAW-0001 perturbs a step: the kind is the first of D5's four, from a
  start drawn on the same stream, with an eligible target at `e_c`'s ordered terms; the target
  is drawn next; the terms are re-rendered under `e_c`'s own presentation) — and the chain step
  that cites the lemma is justified **ex falso**: `_ = e_c := absurd (…_lemma_<j> x₀ y₀) (by
  norm_num)`, the axiom's instance at the witness point `(x₀, y₀)` — the first pair with `1 ≤
  x₀, y₀ ≤ 8`, in order of `x₀ + y₀` and then `x₀`, at which the true and the perturbed
  polynomial take different values (a fixed search, no draw) — refuted by `norm_num`. Every
  identity of the chain therefore stays the faithful one and the main theorem still follows
  (Al's ruling of 2026-11-01 on PR 34). The informal file is unchanged in both cases (D5: the
  class manifests in the formal file). The one record is formal-only, at the axiom's
  declaration (its first and only line), `YES-TOOLING`, confirmed by check T as `s2_template`
  0.3.0 reads it (`#print axioms` of the main declaration lists the axiom's name); its dials
  carry `mode`, `truth`, `site` (the lemma step's plan index), `declaration`, `inserted` (the
  axiom line), `replaced` (the faithful lemma's declaration lines), `original_fragments` (the
  lemma's lines and, for `false`, the citing step's faithful `calc` line) and, for `false`,
  `perturbation`, `monomial` (`replaced`, `inserted`), `counterexample` (`x`, `y`, `lhs`,
  `rhs`: the witness point and the two sides' values there) and `use_site` (the citing step's
  plan index). `lean_build` is commanded `pass`, `lean_axioms_standard` `false`;
  `proof_status` is `invalid` and `correct` false (D7); the counts are the control's.
- **Refusals (D3, D6).** `native_decide` (STATE's NEXT: it stays refused; the chain has no
  decidable goal to close that way) and any other mode; a non-boolean `truth`; a severity other
  than the truth's (gap for `true`, fatal for `false`); a sub-variant; a missing or extra dial;
  a reserved dial; `n_lemmas` 0; a site that is not a lemma step (0, or a chain step's index);
  the target `informal`; two specs. Under 0.1.0 and 0.2.0 the class stays refused, so their
  goldens regenerate under the versions they name.
- **Versions (D1).** `GrammarVersion::V0_3_0` is current; `generate` emits 0.3.0;
  `Producer::parse` admits ten producer versions. The bytes 0.3.0 emits for a 0.1.0 or 0.2.0
  config are theirs: only the version, the id and the manifest differ (tested on a control, a
  FLAW-0001, a FLAW-0007 and a padded artifact). `chain_line` factors the `calc` line out of
  `chain_step` and changes no byte (the 87 earlier goldens regenerate).

## Evidence that the Lean material does what the records say

1. **Probes before the module** (`evidence/lean-probe/`, hand-written on the 0.1.0 short
   control's chain with its second step stated as a lemma; `lake env lean` from `lean/`, one
   process at a time, `run.sh`; 2026-11-01T19:06:23Z–19:06:54Z, after the ORIENT gate had
   ended; the log `probes.log`):

   | Probe | Exit | What |
   |---|---|---|
   | `s2l_rc_axiom_true` | 0 | Lemma 1's declaration replaced by `axiom … : ∀ x y : ℤ, 4 * y * y + x * (x - 4 * y) = 4 * y ^ 2 - 4 * y * x + x * x` (the same identity), cited as before; `[propext, syndeom_s2_grammar_ring_chain_lemma_1, Quot.sound]` |
   | `s2l_rc_axiom_false_exfalso_norm_num` | 0 | the axiom states the false identity `… = 4 * y ^ 2 + 4 * y * x + x * x` (one sign flipped) and the citing step is `absurd (…_lemma_1 1 1) (by norm_num)`: the sides are 1 and 9 at (1, 1); the same axiom print |
   | `s2l_rc_axiom_false_exfalso_decide` | 0 | the same with `by decide` in place of `by norm_num`; the same axiom print (`instDecidableNot`, `Int.instDecidableEq` and `of_decide_eq_true` in the main declaration's used-constants) |
   | `s2l_rc_axiom_false_intermediate` | 0 | the first candidate shape of Al's ruling: the false axiom cited as `_ = 4 * y ^ 2 + 4 * y * x + x * x := …_lemma_1 x y` (a false intermediate presentation) and undone at the next step, `_ = … := (…_lemma_1 x y).symm.trans (by ring)`; the same axiom print |

   The probes fix what the generator may emit: an axiom of the lemma's identity cited as before
   (`true`), and the ex falso shape with `norm_num` (`false`), both building under the one
   import `Mathlib.Tactic.Ring` with the axiom listed by the print, which is what check T reads.
   `decide` also refutes the instance; `norm_num` is the one realized (reading 117). The
   false-intermediate shape builds too and is not realized (reading 115). The module was written
   after they compiled.
2. **The goldens.** 6 cases — on `ring_chain` at the short length with one lemma (`n_steps` 4,
   `n_lemmas` 1), the control (informal, formal), FLAW-0009 `axiom` `truth: true` (formal,
   pair) and `truth: false` (formal, pair) — each produced by `generate --dry-run` with seed 7
   at `d3c3851bb992bb7376631b1868db8d0c06d001d5` by two separate runs
   (`evidence/goldens-run10.py`; run A 2026-11-01T19:19:24Z–19:21:06Z, run B
   19:21:06Z–19:22:48Z; every case exit 0, fidelity PASS; the provenance check passed against
   the commit before each run; `evidence/goldens-run10a-summary.txt`,
   `evidence/goldens-run10b-summary.txt`). `evidence/goldens-copy10.py` compared the runs file
   by file — every `proof.md`, `Proof.lean` and `manifest.json` byte-identical, every
   observation equal, one engine commit — checked every case's shape and placed run A's outputs
   under `engine/generate/tests/goldens/ring_chain/` (`v0.3.0-…`) with a `case.json` each
   (`evidence/goldens-production-report.json`, SHA-256
   `f3eb6393ebdc8c6ff7e9f4f88e14fbdb41f49272ab571eea37a7c11757fb310e`, 35,921 bytes):
   - the control's chain is the 0.1.0 short control's (`(x - 2 * y) ^ 2` at seed 7) with its
     second step, `4 * y * y + x * (x - 4 * y) = 4 * y ^ 2 - 4 * y * x + x * x`, stated as
     Lemma 1 (plan step 1) and cited at plan step 3; the formal control builds and prints
     `[propext, Quot.sound]` (`s2_grammar-v0.3.0-9ffdfe550dc7-s7`, 163 formal tokens;
     informal `…-1ef0cde05162-s7`, 125 tokens, `n_steps` 5, `n_lemmas` 1);
   - the `truth: true` cases replace Lemma 1's declaration by the axiom of the same identity at
     line 3 and change nothing else; every formal file builds and prints `[propext,
     syndeom_s2_grammar_ring_chain_lemma_1, Quot.sound]`, so the instance is confirmed
     (`flaws_confirmed` 1), `lean_axioms_standard` is achieved `false` as commanded,
     `proof_status: invalid`, `correct: false`, severity gap; 157 formal tokens; the pair's
     informal file is the informal control's byte for byte;
   - the `truth: false` cases draw the `exponent` perturbation on `injector/FLAW-0009/1`:
     `-4 * x * y` of the lemma's right-hand side becomes `-4 * x * y ^ 2`, so the axiom states
     `4 * y * y + x * (x - 4 * y) = 4 * y ^ 2 - 4 * y ^ 2 * x + x * x` (rendered under the
     step's `y`-first style); the two sides agree at (1, 1) and differ at (1, 2) — 9 and 1 —
     so the citing step (plan step 3) is `absurd (…_lemma_1 1 2) (by norm_num)`; every formal
     file builds with the same axiom print, the instance is confirmed, severity fatal; 162
     formal tokens; the pair's informal file is the informal control's byte for byte;
   - every case is a true statement whose reference is the 0.3.0 formal control at this length
     (`s2_grammar-v0.3.0-9ffdfe550dc7-s7:syndeom_s2_grammar_ring_chain`), which names itself;
   - the receipts, with build and audit stdout and stderr, are under the external root
     `C:\Users\LEET\Codex\Temp\syndeom-s2-grammar-030-20261101T1919Z\goldens-10a\` and
     `goldens-10b\`; the SHA-256 of the sorted list of the six `receipt.json` SHA-256s (one per
     line, LF-terminated) is `563c5ab9e0814a8bf74d08b390e854b54c292e35cf7f9a6a03ed43b1d9975bbe`
     (A) and `6a6707941a40513f7eaa5a61243134a459f24b7b71d098e625f8790c6ced45c3` (B); the
     receipts differ only in their timings.
3. **The artifact ids** (`s2_grammar-v0.3.0-<config_hash[0:12]>-s7`): control with one lemma
   `1ef0cde05162` / `9ffdfe550dc7` (informal / formal); FLAW-0009 `truth: true`
   `3df844943ca3` / `988979be1294` (formal / pair); `truth: false` `6a92e3614dbc` /
   `c448f9add59d` (formal / pair).
4. **Tests and lint.** The crate's 49 tests pass (goldens 1 over 93 cases, grammar 14,
   properties 24, provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features
   -- -D warnings` and `cargo fmt --check` are clean. New, in `tests/grammar.rs`: FLAW-0009
   over eight configurations (lengths (4, 1), (6, 2), (3, 3), (24, 2) and (1, 1), both truths):
   the site drawn as the stream says; the plan the control's but for the lemma's declaration
   and, under `false`, the citing step; the informal file unchanged; the axiom line; the false
   identity and the witness point re-derived from the stream and the grid order (every earlier
   grid point agrees); the ex falso line; the dials; the one formal record at the axiom's line;
   the counts and the commanded `lean_build` and `lean_axioms_standard`; check T's refusal of
   confirmation when the print omits the axiom or is absent (FAIL, not a refusal); the formal
   target's bytes equal to the pair's; the informal target refused; determinism; explicit
   sites; eighteen refusals; 0.1.0 and 0.2.0 refusing the class; the 0.2.0 bytes of a
   FLAW-0007 and of a padded artifact unchanged under 0.3.0 — and the version facts. The gate
   on the committed tree is recorded in the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-11-01T19:24Z, outside the gate (`cargo audit --file
engine/Cargo.lock`). It loaded 1,395 advisories (advisory-db commit
`2295466c4e9df26647fd67d8fa1c705ac2fc7f81`), scanned 35 crate dependencies, and reported no
vulnerability (exit 0; `evidence/cargo-audit.txt`). `engine/Cargo.lock` is unchanged by this
revision: no dependency was added.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `d3c3851b…` — `engine/generate/src/grammar.rs` (`smuggle`, `eval`,
`witness_point`, `chain_line`, the version gate in `prepare_with`, `realizes`),
`src/lib.rs`, `tests/grammar.rs`. The threat model is packet 1's: an offline research CLI whose
assets are the integrity of generated ground truth and the operator's machine, and whose trust
boundary is an artifact directory handed to `verify` and a config handed to `generate`. New
this revision: a boolean dial (`truth`) and a mode name select among constants; an explicit
`site` indexes the lemma steps; integer arithmetic evaluates polynomials at small points.

**Pre-scan (Phase 1):** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate
roots and in the manifest crate; no external dependency was added; clippy at `-D warnings` is
clean. `cargo geiger`, `cargo deny` and Miri are not installed on this machine and were not run;
there is no `unsafe` to map.

**Panic and index sweep of the diff (2C):** no `unwrap`, `expect` or `panic!` in the edited
sources outside tests. The indexes: `spec.dials["mode"]` and `["truth"]` sit behind
`exact_keys` on the dials (as 0.2.0's dials do); `d.sites[j - 1]` and `faithful.steps[j - 1]`
with `j` drawn below `m` or checked against `1..=m`; `d.e[c]`, `d.e[c - 1]` and
`d.presentations[c - 1]` with `c` a chain step of the derivation; `candidates[below(len)]` on a
list `find_map` returned non-empty; `plan.steps[m + c - 1]` within the plan. Integer arithmetic
(2I): `eval` multiplies coefficients of the atom library's expansions (magnitude at most 25,
packet 8) by powers of `x, y ≤ 8` of degree at most 5 — far inside `i64`; the witness grid is
a constant; no input-controlled quantity enters an arithmetic operation. The ordering key of
the witness search is a pair of small integers.

**Serde and the config (2F):** `mode` is compared to two constants and everything else
refused; `truth` via `as_bool`; the site via the existing `parse_spec` (a step index at least
1). No config text reaches the proof text: the axiom line is the lemma's own header or a
rendering of the atom library's terms; the ex falso line interpolates the lemma's name (the
producer's) and two integers of the fixed grid.

**Subprocesses and `verify`:** unchanged. `verify` compiles only bytes the producer the
manifest names itself emits for the manifest's `(config, seed)`, and the audit tail names only
the producer's main declaration (FINDING-011's guard holds).

**Determinism:** `BTreeMap` and `Vec` only; every draw is labelled (`injector/FLAW-0009/1`: the
site, then the perturbation's start and target); the witness point is a pure function of the
two polynomials; no floating-point value enters a generation decision.

No new finding. **FINDING-018** (informational, backlog: `derive`, `Derivation::n` and
`render` trust their callers' preconditions) stands and its scope gains `smuggle`'s use of
`d.sites[j - 1]`, which trusts the derivation's invariant that `sites` has `n_lemmas` entries;
the recommendation is unchanged. FINDING-015 does not extend to the grammar (packet 8);
FINDING-016 and FINDING-017 (informational, backlog) stand; FINDING-006 (accepted risk),
FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012 and FINDING-013 (accepted) stand as
packets 1–9 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 9's numbering.

113. **The site is the lemma step's plan index.** D5 says "replace one lemma's proof"; the
     lemma is drawn as `1 + below(n_lemmas)` on `injector/FLAW-0009/1` when the spec says
     `auto`, and an explicit `site` is a lemma step's plan index (1 to `n_lemmas`), refused
     otherwise, so the record's `site` names the declaration's step as the template's does.
     `n_lemmas` 0 is refused: there is no lemma to replace.
114. **`truth: true` changes one declaration and nothing else:** the lemma's `theorem` becomes
     an `axiom` of the same claim, cited as before. The informal file is unchanged (D5: the class
     manifests in the formal file), so a pair's informal lemma is stated and proved while the
     formal one is assumed — the modality mismatch Al's ruling on reading 24 asks the record to
     make explicit, which `inserted` and `replaced` do.
115. **`truth: false` realizes the ex falso shape**, the second of the two candidate shapes
     Al's ruling of 2026-11-01 names, and not the first. Both were probed and both build with
     the axiom listed. The ex falso shape keeps every identity of the chain the faithful one, so
     the pair's steps stay aligned (D2's load-bearing property) and the informal file stays
     unchanged; it has one record; it is eligible at every lemma; and the false identity is
     refuted inside the artifact's own build (116, 118). The false-intermediate shape changes
     two chain steps' identities in the formal file while the informal file keeps the faithful
     ones, so a pair's steps `c` and `c + 1` no longer align; it needs a second record for the
     false intermediate step, whose class FLAWS.md 0.1.0 does not name (FLAW-0001 is a
     perturbation the kernel rejects; this one the kernel accepts by the axiom); it is
     ineligible when the cited step is the last (the chain would end away from the claim); and
     its negation witness would live outside the artifact. Listed, not taken; a later version
     may realize it as a composite in FLAW-0012's sense if a ruling asks.
116. **The false identity is the lemma's right-hand side perturbed once**, as FLAW-0001
     perturbs a step (D6: "`false` from a perturbed one"), rendered under the step's own
     presentation so the visible change is one monomial. The kind is not a config dial (D5
     lists `mode` and `truth` only): a start is drawn among D5's four, and the first kind from
     it with an eligible target at the step is taken (`sign` always has one), then the target
     is drawn — one more labelled draw sequence on the same stream (D4). The alternative, a
     `perturbation` config dial as FLAW-0001 has, is listed and not taken.
117. **The witness point is found, not drawn:** the first pair `(x, y)` with `1 ≤ x, y ≤ 8`, in
     order of `x + y` and then `x`, where the two polynomials differ (two distinct polynomials
     of degree at most 5 cannot agree on an 8 × 8 grid). The refutation is `by norm_num`,
     which Al's ruling names with `decide`; both compile under the one import (the probes).
     `norm_num` is taken because `ring` already rests on it (the main declaration's
     used-constants list `Mathlib.Meta.NormNum.*` in every control) and it evaluates the
     instance rather than reducing `Int` powers in the kernel; `decide` is listed, not taken.
118. **The negation witness is the artifact's own build.** Al's ruling on reading 24 asks, for
     the template's `truth: false`, for "a witness module proving its negation"; the grammar's
     axiom is generated, so no library module can state it in advance. Its negation is
     kernel-checked where the artifact uses it: `by norm_num` proves `¬ (lhs(x₀, y₀) = rhs(x₀,
     y₀))` in the artifact's build, which must pass for the instance to be confirmed, and the
     record carries the point and the two values (`counterexample`) so that a reviewer can
     check the refutation by hand. A separate witness file compiled by the harness is listed,
     not taken (a second Lean process per artifact for a fact the build already checks).
119. **One instance, one formal record, at the axiom's declaration;** the citing step's changed
     `calc` line is recorded as `original_fragments[use_site]` and the step named by
     `use_site`, not as a second record (the step's identity is the faithful one; only its
     justification changed). `original_fragments[site]` carries the lemma's declaration lines —
     the grammar's FLAW-0001 convention for a restated lemma (packet 8) — where the template's
     FLAW-0009 record carries the proof lines only.
120. **`counterexample` is an object** (`x`, `y`, `lhs`, `rhs`), machine-checkable, where the
     template's FLAW-0012 records free text.
121. **The severity is fixed by the truth** (gap for `true`, fatal for `false`, D5) and a
     mismatch is refused; `sub_variant` is null; `native_decide` stays refused (STATE's NEXT).
122. **The goldens include the control at (4, 1), informal and formal,** so that the flawed
     cases' truth reference names a golden (as 108 did at (4, 0)), and the `truth: false` cases
     (formal, pair), which NEXT's step 5 did not name: six cases where it named three.
123. **A pair's informal file is the informal control's byte for byte** (the goldens assert
     it), as FLAW-0008's is (0.7.0).
124. **`chain_line`** factors the `calc` line out of `chain_step` for the ex falso line's
     sake; no byte changes (the 87 earlier goldens regenerate).
125. **The counts are the control's** (`n_steps`, `n_lemmas`, `n_tokens` achieved only); the
     ex falso line is longer than the cited one, which the formal `n_tokens` shows (162 against
     157 for `true`).
126. **The witness grid's bound (8) and order are constants of the producer** (D4: part of its
     definition); a change bumps the version.
