# S2 implementation packet 13 — `s2_grammar` 0.6.0 (ADR-0011, Session 52, 2026-11-02)

This packet records the thirteenth implementation session of ADR-0011 (D11): `s2_grammar` 0.6.0 —
FLAW-0004 on the grammar `ring_chain`, a ring-closed chain step's justification replaced by a
drawn hedge before the bare claim and the aligned `calc` line's tactic omitted (D5, D6) — with
the Lean probes that fixed the formal shape, its goldens, and the `cargo-deny` allow-list Al's
rulings on PR 37 ordered. Nothing here is a result: no CLAIM, no fidelity sweep, no artifact
under `corpora/`.

Branch `claude/s2-grammar-0.6.0`, from `main` `1344357bc59a5219c564069259bc3e68b8a3b930` (PR
37's merge):

| Commit | What |
|---|---|
| `5036342f88c6309155facaad2900cea3298a6129` | `engine/deny.toml`, the licence allow-list read from `cargo deny list` (Al's ruling on PR 37; its own commit) |
| `bc093c4be3b76c0bb520e8b56778b86abdc0792c` | the 0.6.0 producer: `src/grammar.rs` (`hedge`, the version, `realizes`, `HIDDEN_STEP`, `OMITTED_TACTIC`), `src/lib.rs`, `tests/grammar.rs`, the crate README |
| the goldens commit after it | the 4 goldens of `s2_grammar` 0.6.0, produced at `bc093c4b…`, the goldens README and the count (105 cases) |
| the record commit after that | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 52 |

## What was built

- **FLAW-0004 on `ring_chain` (D5, as `s2_template` 0.5.0 reads it).** The config's one dial is
  `hidden_step`; there is no sub-variant; the severity is the one the dial fixes; the target is
  `informal` or `pair` (`formal` is refused by D3: FLAWS.md's *Manifests in* is informal). Only
  `routine-true` (cosmetic) has material on the chain: every step of `ring_chain` is a `ring`
  identity, which is routine by construction, so `nontrivial-true` (no step is non-trivially
  true) and `false` (no false-step material) are refused for lack of material (reading 153). The
  site is a **ring-closed chain step** — never a lemma step, never a lemma-cited chain step, the
  last step eligible (reading 150) — drawn on `injector/FLAW-0004/1` as `below(e)` among the `e`
  eligible steps when the spec's `site` is `auto`; an explicit `site` is such a step's plan index
  and skips that draw. The hedge is drawn next on the same stream, `below(5)`, from the
  template's fixed lexicon (`Clearly`, `It is easy to see that`, `Obviously`, `By a standard
  argument`, `One checks that`; reading 154), and the step becomes the hedge, a space and the
  bare claim `$e_{k-1} = e_k$.` (its period kept), the chain's own justification before the comma
  (`Expanding the square`, `Rearranging the terms`, `Factoring out $x$`, …) deleted and recorded
  (reading 155). The aligned fragment — the one `calc` line `_ = e_k := by ring`, or `calc e_0 =
  e_1 := by ring` at the first step — loses its tactic step and becomes `_ = e_k := by`: the `by`
  is kept, as the template keeps its `have` header, and `sorry` is never written, so that Lean
  reports exactly one `unsolved goals` error at that line and nothing else, at any position of
  the chain (the probes below; reading 151). A pair carries a record in each file, `YES-KERNEL`:
  the informal one over the whole hedged step (`inserted` the hedged step, `replaced` the
  faithful one; reading 156), confirmed by the FLAW-0004 check as 0.5.0 reads it (the step is
  the hedge entry, a space and the recorded claim; the faithful step held the justification
  exactly once; the step does not); the formal one at the `calc` line of the main declaration,
  confirmed by check K (the build's first error at that line). The pair commands `lean_build`
  `fail`; an informal-only artifact's one record is `N/A` (reading 157). The dials carry
  `hidden_step`, `site`, `hedge` (the index) and `hedge_text`, and `justification`, `claim`,
  `inserted`, `replaced` and `original_steps` (informal) or `omitted` (`["ring"]`, reading 152),
  `inserted`, `replaced` and `original_fragments` (formal). `proof_status` is `valid` and
  `correct` true (D7: the instance is cosmetic; reading 160); the counts are the control's
  (reading 158).
- **Refusals (D3, D6).** `nontrivial-true` and `false` (no material); any other `hidden_step`; a
  non-string `hidden_step`; a sub-variant; a severity other than cosmetic; a missing, extra or
  reserved dial (`hedge`, `hedge_text`, `claim`, `justification`, `omitted`, …); the target
  `formal`; a site that is a lemma step, a lemma-cited chain step, 0, or past the plan; an
  instance whose every chain step is lemma-cited (`n_lemmas = n_steps`); two specs. Under 0.1.0
  to 0.5.0 the class stays refused, so their goldens regenerate under the versions they name.
- **Versions (D1).** `GrammarVersion::V0_6_0` is current; `generate` emits 0.6.0;
  `Producer::parse` admits thirteen producer versions. The bytes 0.6.0 emits for a 0.1.0 to
  0.5.0 config are theirs: only the version, the id and the manifest differ (tested on a
  FLAW-0003 `cycle` artifact, a FLAW-0010 one, a FLAW-0009 `truth: false` one and the (4, 0)
  and (4, 1) controls; the 101 earlier goldens regenerate). No byte of any earlier version
  changes. The new label `injector/FLAW-0004/1` with its two draws is part of 0.6.0's rng
  definition (D4).
- **`engine/deny.toml` (D10, as Al ruled on PR 37).** `cargo deny list` on the current
  `engine/Cargo.lock` (35 packages: the five workspace crates and 30 external) prints eight
  licence identifiers — `0BSD`, `Apache-2.0`, `BSL-1.0`, `MIT`, `PolyForm-Noncommercial-1.0.0`,
  `Unicode-3.0`, `Unlicense`, `Zlib` (`evidence/cargo-deny-list.txt`) — and the allow-list is
  exactly those, in its own commit `5036342f…`, with nothing else configured (reading 163).

## Evidence that the material does what the records say

1. **The probes** (STATE's NEXT, step 4; `evidence/probes/`), compiled one Lean process at a
   time from `lean/` with `lake env lean`, before any design, from the (4, 0) seed-7 formal
   control (the 0.1.0 golden `control-short-formal`, whose bytes every later version emits) with
   one `calc` line altered (`make_probes.py`):

   | Probe | The altered line | Exit | First error (line:col) and message | Other errors |
   |---|---|---|---|---|
   | (i) `p1-middle-by-omitted` | line 5, `_ = e_2 := by` (`ring` omitted) | 1 | `5:41: error: unsolved goals` (the step's own goal) | none |
   | (i) `p2-first-by-omitted` | line 4, `calc e_0 = e_1 := by` | 1 | `4:56: error: unsolved goals` | none |
   | (i) `p3-last-by-omitted` | line 7, `_ = e_4 := by` (the last step, end of file) | 1 | `7:41: error: unsolved goals` | none |
   | (ii) `p4-middle-by-skip` | line 5, `_ = e_2 := by skip` | 1 | `5:41: error: unsolved goals` | none |
   | (iii) `p5-middle-term-hole` | line 5, `_ = e_2 := _` | 1 | `5:41: error: don't know how to synthesize placeholder` | `3:101: error: unsolved goals` at the theorem's header line (a cascade) |
   | (iii) `p6-middle-synthetic-hole` | line 5, `_ = e_2 := ?_` | 1 | `3:101: error: unsolved goals` (the step's goal, reported at the theorem's header line) | none |

   Shape (i) realizes "the aligned tactic step omitted (never `sorry`), leaving an unsolved
   goal" with check K at the site: one `unsolved goals` error at the altered line, at the first,
   a middle and the last step, no cascade. Shape (ii) produces the same message but names a
   tactic (`skip`) in the text — a visible placeholder, not an omission — and is refused on the
   record. Shape (iii) is a term-level hole, not an omitted tactic step: `_` is reported at the
   line but cascades to the theorem, and `?_` is reported at the theorem's line only, where
   check K would not hold at the site. The probe files, their stdout (every stderr was empty; the empty files stay
   under the external root, not here) and the run summary (each run 6 to 8 seconds) are under
   `evidence/probes/`.
2. **The goldens.** 4 cases — on `ring_chain` at the short length (`n_steps` 4, `n_lemmas` 0),
   the control (informal, formal) and FLAW-0004 `routine-true` (informal, pair) — each produced
   by `generate --dry-run` with seed 7 at `bc093c4be3b76c0bb520e8b56778b86abdc0792c` by two
   separate runs (`evidence/goldens-run13.py`; run A 2026-11-02T18:42:52Z–18:43:40Z, run B
   18:43:40Z–18:44:28Z; every case exit 0, fidelity PASS; the provenance check passed against the
   commit before each run; `evidence/goldens-run13a-summary.txt`,
   `evidence/goldens-run13b-summary.txt`). `evidence/goldens-copy13.py` compared the runs file
   by file — every `proof.md`, `Proof.lean` and `manifest.json` byte-identical, every
   observation equal, one engine commit — checked every case's shape and placed run A's outputs
   under `engine/generate/tests/goldens/ring_chain/` (`v0.6.0-…`) with a `case.json` each
   (`evidence/goldens-production-report.json`, SHA-256
   `d5059eb569918aa9ced9866cc248963cf2040e66dc46142d38db691ab0fe8f10`, 11,290 bytes):
   - the control's chain is the 0.1.0 short control's (`(x - 2 * y) ^ 2` at seed 7, four
     ring-closed steps); both files are the 0.1.0 short control's byte for byte; the formal
     control builds and prints `[propext, Quot.sound]` (`s2_grammar-v0.6.0-cc0c6a4daf03-s7`,
     119 formal tokens); the informal control is `…-82300f31bd83-s7`, 93 tokens, `n_steps` 4,
     `n_lemmas` 0;
   - the FLAW-0004 cases draw, on `injector/FLAW-0004/1`, the site `below(4)` = chain step 3 and
     then the hedge `below(5)` = 1, `It is easy to see that`, so plan step 3 becomes `It is easy
     to see that $4y^2 - 4yx + x \cdot x = x^2 - 4yx + 4 \cdot y \cdot y$.` in place of
     `Rearranging the terms, $4y^2 - 4yx + x \cdot x = x^2 - 4yx + 4 \cdot y \cdot y$.` and the
     `calc` line 6 becomes `_ = x ^ 2 - 4 * y * x + 4 * y * y := by`; every other step and line
     is the control's. The informal record is at step 3, span `[0, 80)` (the whole hedged step),
     `justification` `Rearranging the terms`, `claim` the bare identity with its period, `hedge`
     1; the pair's formal record is at `syndeom_s2_grammar_ring_chain:6`, `omitted` `["ring"]`,
     `inserted` the flawed line and `replaced` the faithful one; both `YES-KERNEL`, cosmetic,
     one instance (`N/A` in the informal-only case). `flaws_confirmed` 1, `n_flaws_0004` 1,
     `proof_status: valid`, `correct: true`; 96 informal tokens against the control's 93
     (reading 158); the pair's build exits 1 with its one error `6:41: error: unsolved goals`
     (`evidence/goldens-production-report.json`: `first_error_line` 6), `lean_build` commanded
     and achieved `fail`, no axiom print (`lean_axioms_standard` achieved `null` with its
     reason, not commanded);
   - every case is a true statement whose reference is the 0.6.0 formal control at this length
     (`s2_grammar-v0.6.0-cc0c6a4daf03-s7:syndeom_s2_grammar_ring_chain`), which names itself;
   - the receipts, with the build and audit stdout and stderr of the two formal cases, are under
     the external root `C:\Users\LEET\Codex\Temp\syndeom-s2-grammar-060-20261102T1840Z\`
     (`goldens-13a\` and `goldens-13b\`), whose sorted `receipt.json` SHA-256 lists — one per
     line, LF-terminated — hash to
     `d0e1ffd64f6b43eadde8315f5b8cb09604c7a4e90ec72e6742f6fff4db374259` (A) and
     `ba7211375fce08ddff570bb753e64e5b1dfcc002a3c271638970f510946d9bac` (B); the receipts
     differ only in their timings (in run A the pair's failing build 6,393 ms; the formal
     control's build 6,750 ms and audit 6,652 ms).
3. **The artifact ids** (`s2_grammar-v0.6.0-<config_hash[0:12]>-s7`): control `82300f31bd83` /
   `cc0c6a4daf03` (informal / formal); FLAW-0004 `routine-true` `c42a470c2f7c` / `2e435c3a4306`
   (informal / pair).
4. **Tests and lint.** The crate's 52 tests pass (goldens 1 over 105 cases, grammar 17,
   properties 24, provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features
   -- -D warnings` and `cargo fmt --check` are clean. New, in `tests/grammar.rs`: FLAW-0004 over
   six configurations (lengths (4, 0), (6, 2), (1, 0), (24, 2), (12, 5) and (8, 1)): the site and
   the hedge re-derived from the stream; the plan the control's but for the site's text and
   fragment, step 0 untouched; the text (the hedge, a space and the claim; the justification held
   once by the faithful step and not by the flawed one, nor by the claim); the fragment (the
   `calc` line without its tactic, the `by` kept, no `sorry`); the formal file differing from the
   control's in exactly that line, at the site's line, with the fragment lines and declarations
   unchanged; the dials of both records; the two records of a pair, their locations and the
   ground truth (valid, correct, the reference the formal control); the counts, `lean_build`
   commanded `fail` and `lean_axioms_standard` not commanded; the rendered informal file; check
   K's refusal of confirmation under a first error at another line and under a passing build
   (FAIL, not a refusal); the informal-only target's bytes equal to the pair's with one `N/A`
   record and no `lean_build`; the formal target refused; determinism; explicit sites at every
   ring-closed step of a (6, 2) instance with the lemma-cited ones refused, the hedge still drawn;
   sites 0, the lemma steps and past the plan refused; sixteen refusals; 0.1.0 to 0.5.0 refusing
   the class; the 0.5.0 bytes of five artifacts unchanged under 0.6.0 — and the version facts.
   The gate on the committed tree is recorded in the pull request and under the external root.

## `cargo audit` and `cargo deny check` (D10)

- `cargo audit`: `cargo-audit 0.22.2` at 2026-11-02T18:44:29Z, outside the gate (`cargo audit
  --file engine/Cargo.lock`). It loaded 1,402 advisories (advisory-db commit
  `263a4d951a0867d4bddc29dddd6d59427c78f9a7`, seven more than Sessions 48–51's `2295466c…`),
  scanned 35 crate dependencies, and reported no vulnerability (exit 0;
  `evidence/cargo-audit.txt`). `engine/Cargo.lock` is unchanged by this revision: no
  dependency was added.
- `cargo deny check`: `cargo-deny 0.20.2` at 2026-11-02T18:36:00Z, with `engine/deny.toml`
  as committed: **advisories ok, bans ok, licenses ok, sources ok** (exit 0;
  `evidence/cargo-deny-check.txt`). It runs where `cargo audit` runs — here, at S2.4 and
  before the freeze — never in the offline gate, which is unchanged.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `bc093c4b…` — `engine/generate/src/grammar.rs` (`hedge`, the constants
`HIDDEN_STEP` and `OMITTED_TACTIC`, `GrammarVersion::V0_6_0`, the dispatch in `prepare_with`,
`realizes`), `src/lib.rs`, `tests/grammar.rs` — and `engine/deny.toml` (`5036342f…`, a
configuration file the gate never reads). The threat model is packet 1's: an offline research
CLI whose assets are the integrity of generated ground truth and the operator's machine, and
whose trust boundary is an artifact directory handed to `verify` and a config handed to
`generate`. New this revision: a string `hidden_step` compared to three constants; an explicit
`site` indexing the chain steps; one more lexicon index drawn and used to index `HEDGES`.

**Pre-scan (Phase 1):** `cargo audit` and `cargo deny check` are clean (above);
`#![forbid(unsafe_code)]` stands in both crate roots and in the manifest crate, and no `unsafe`
block exists in the crate; no external dependency was added; clippy at `-D warnings` is clean.
`cargo-geiger` is not adopted (Al's ruling on PR 37; `#![forbid(unsafe_code)]` is the stronger
guarantee); Miri is not available for the pinned toolchain.

**Panic and index sweep of the diff (2C):** no `unwrap`, `expect` or `panic!` in the edited
sources outside tests. The indexes: `spec.dials["hidden_step"]` sits behind `exact_keys` on the
dials; `eligible[below(e)]` with `e = eligible.len() ≥ 1` (an empty list is refused first);
`HEDGES[below(5)]`; `faithful.steps[site - 1]` with `site = m + c` and `c` in `eligible ⊆
1..=n`, so `site ≤ m + n`, the plan's step count; `d.e[c - 1]` and `d.e[c]` with `c` in
`1..=n` and `d.e` of length `n + 1`. The faithful step's shape (its justification, a comma and
the claim; the one `calc` line closed by `ring`) is checked, not assumed, and a mismatch is a
refusal. No arithmetic beyond `m + c` and `k - m` (saturating) on counts bounded by D6's ranges
(2I).

**Serde and the config (2F):** `hidden_step` via `as_str` compared to constants; the severity
checked twice (the range in `check_spec`, cosmetic in `hedge`); the site via the existing
`parse_spec` (a step index at least 1). No config text reaches the proof text: the hedge is a
lexicon entry the index selects, the claim and the justification are the faithful step's own
text, and the flawed `calc` line is the faithful one truncated by a constant.

**Subprocesses and `verify`:** unchanged. `verify` compiles only bytes the producer the
manifest names itself emits for the manifest's `(config, seed)`, and the audit tail names only
the producer's main declaration (FINDING-011's guard holds). The probes were compiled from
files under the external root, written by the executor from a committed golden, never from a
config.

**Determinism:** `BTreeMap` and `Vec` only; two labelled draws (`injector/FLAW-0004/1`: the
site when `auto`, then the hedge); no floating-point value enters a generation decision.

No new finding. **FINDING-018** (informational, backlog: `derive`, `Derivation::n` and `render`
trust their callers' preconditions) stands and its scope gains `hedge`'s use of `d.sites` (via
`lemma_of`), `d.e` and `plan_index`, which trust the derivation's invariant that `e` has `n + 1`
entries and that the plan lists the `m` lemma steps first; the recommendation is unchanged.
FINDING-015 does not extend to the grammar (packet 8); FINDING-016 and FINDING-017
(informational, backlog) stand; FINDING-006 (accepted risk), FINDING-007 (backlog),
FINDING-009, FINDING-010, FINDING-012 and FINDING-013 (accepted) stand as packets 1–12 record
them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 12's numbering.

150. **The site is a ring-closed chain step only, the last step included.** FLAW-0004's recipe
     deletes a step's justification; on the chain a ring-closed step's justification is its
     own phrase before the comma, while a lemma-cited step's is the citation `By Lemma j` (a
     hedge in its place would strand the step's premise and leave the lemma cited by nothing —
     reading 127's rule, applied as 0.4.0 applied it) and a lemma step carries a closing note,
     not a justification. Unlike FLAW-0001 `local`, the last step is eligible: nothing after
     the site is needed.
151. **The formal shape is the tactic omitted with the `by` kept**, chosen by the probes: (i)
     `_ = e_k := by` yields one `unsolved goals` error at that line and no cascade at the
     first, a middle and the last step; (ii) `by skip` yields the same message but is a visible
     placeholder tactic and is refused on the record; (iii) a term-level hole is not an omitted
     tactic step — `_` cascades to the theorem's line, `?_` is reported there only, where check
     K would not hold at the site. This is the template's shape at the one-line scale: the
     template keeps the `have` header and omits the proof block; the chain keeps `:= by` and
     omits `ring`. `sorry` is never written (D5, Options 4).
152. **`omitted` records the tactic text, `["ring"]`, not whole lines.** The template's
     `omitted` lists the whole lines it removed from a multi-line fragment; the chain's
     fragment is one line and the omission is inside it, so the record carries the omitted
     tactic and `replaced` equals `inserted` followed by a space and that tactic — checked at
     injection. D5's "the aligned tactic step omitted" is read as the tactic, whatever its
     extent.
153. **`routine-true` is the one `hidden_step` with material; `nontrivial-true` and `false` are
     refused for lack of it.** Every step of the chain is a `ring` identity, routine by
     construction, so no step is non-trivially true; a `false` hedged step would need
     false-step material — the step's identity perturbed as FLAW-0009's `false` material
     perturbs the lemma's right-hand side, hedged, with the formal line's tactic omitted so
     that the kernel reports an unsolved (and unprovable) goal — which is a design in FLAW-0012's
     sense, listed and not taken: whether S3's cell design wants a fatal FLAW-0004 cell on the
     grammar is Al's to say, as its own version.
154. **The draw order on `injector/FLAW-0004/1`** is the site (`below(e)`, when `auto`), then
     the hedge (`below(5)`), the hedge drawn even when the site is explicit. The template draws
     its material first and the hedge second on the same label, so at a given seed both
     producers draw the same hedge (at seed 7, index 1, `It is easy to see that` — the same
     entry the template's `sqrt2_irrational` golden carries): a property of the shared label
     and seed, as reading 128 recorded for FLAW-0010's reference, not a defect.
155. **The bare claim keeps its period and the justification is the chain's own phrase**, as
     FLAW-0010's `asserted` and `justification` are (reading 130); the step is the hedge, a
     space and the claim, which the FLAW-0004 check requires. A hedge entry that is a full
     clause (`It is easy to see that`, `One checks that`) reads naturally before `$…$.`; one
     that is an adverb (`Clearly`, `Obviously`) reads as the template's goldens read it.
156. **The record spans the whole hedged step**, `[0, len)`, with `inserted` the hedged step
     and `replaced` the faithful one, as the template's FLAW-0004 record does (its span `[0,
     51)` on `sqrt2_irrational`) — not the hedge alone, because the recipe changes the step's
     justification and its text, not one phrase inside an otherwise faithful step.
157. **`lean_catchable` follows the target:** `YES-KERNEL` in both records of a pair, `N/A` for
     an informal-only artifact (no formal manifestation), the target `formal` refused — as the
     template's 0.5.0 injector does. The informal-only artifact's dials equal the pair's
     informal dials byte for byte.
158. **The counts are the control's** (`n_steps` 4, `n_lemmas` 0; `n_tokens` achieved only): 96
     informal tokens against the control's 93 — the hedge is three tokens longer than
     `Rearranging the terms`. The lexicon's entries run from one token (`Clearly`) to six, the
     chain's justifications from three to seven, so the sign of the difference varies with the
     draw: a documented-weak surface cue of the class in this grammar, in either direction.
159. **The goldens are the four NEXT's step 6 named:** at (4, 0), seed 7, the 0.6.0 controls
     (informal, formal — so that the flawed cases' truth reference names a golden; both files
     the 0.1.0 short control's bytes, as 0.2.0's and 0.4.0's short controls are) and FLAW-0004
     `routine-true` (informal, pair).
160. **A hedged pair is a valid proof of a true statement whose formal file does not build.**
     D7's rule (`valid` iff every record is cosmetic) makes `proof_status: valid` and
     `correct: true` while `lean_build` is `fail` — the same combination the template's
     FLAW-0004 `routine-true` pair golden carries since 0.5.0, and the reason the class is
     `YES-KERNEL` at severity cosmetic: the gap can be filled. Recorded as a consistency fact,
     not a question.
161. **The checks are the existing ones, unchanged:** the FLAW-0004 informal check of
     `assemble` (0.5.0) and check K of the harness. Tampering is tested: a first error at
     another line, or a passing build, yields a FAIL verdict with `flaws_confirmed` 0, not a
     refusal.
162. **The omission is visible in the formal text** — a `calc` line ending in `by` — as the
     template's omitted `have` block is, so the grammar's FLAW-0004 cell measures whether a
     detector reads the formal file at all, not concealment; the informal file's hedge is the
     concealed surface. Noted for the S3 cell design, as reading 115 noted FLAW-0009's.
163. **The allow-list names licence identifiers, not expressions.** `cargo deny list` splits
     each crate's SPDX expression into its identifiers (`adler2` is `0BSD OR MIT OR
     Apache-2.0`, `unicode-ident` is `(MIT OR Apache-2.0) AND Unicode-3.0`), and `cargo deny
     check` accepts an `OR` expression when any branch is allowed and an `AND` when every
     operand is; the eight identifiers the listing prints are therefore the allow-list, with
     nothing assumed about which branch a crate's expression resolves to. The five workspace
     crates' `PolyForm-Noncommercial-1.0.0` is among them, so the check does not flag the
     repository's own licence. Housekeeping, not a finding; recorded so that the next lock
     change knows what the list is.
