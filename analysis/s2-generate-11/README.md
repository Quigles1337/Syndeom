# S2 implementation packet 11 — `s2_grammar` 0.4.0 (ADR-0011, Session 50, 2026-11-02)

This packet records the eleventh implementation session of ADR-0011 (D11): `s2_grammar` 0.4.0 —
FLAW-0010 `a` on the grammar `ring_chain`, the fictional reference drawn from `s2_template`
0.6.0's lexicon (D5, D6), `claim: true` — with its goldens. Nothing here is a result: no CLAIM,
no fidelity sweep, no artifact under `corpora/`.

Branch `claude/s2-grammar-0.4.0`, from `main` `3462af39091ab9df637d6612e019c46cdbe634c6` (PR
35's merge):

| Commit | What |
|---|---|
| `a1d524ae644f44ac9887a8a7f4a2ab11d673e349` | the 0.4.0 producer: `src/grammar.rs` (`cite`, the version, `realizes`), `src/lib.rs`, `tests/grammar.rs`, the crate README |
| `38f1a076c39bc3e92b9a90b47f59840ab4fec47b` | the 4 goldens of `s2_grammar` 0.4.0, produced at `a1d524ae…`, the goldens README and the count (97 cases) |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 50 |

## What was built

- **FLAW-0010 `a` on `ring_chain` (D5, as `s2_template` 0.6.0 reads it).** The config's dials
  are `shape` (`journal`, `book` or `preprint`: which venue list the draw uses) and `claim`
  (`true`, gap; `false` is refused in 0.4.0 — reading 131); the sub-variant is `a` (`b` and `c`
  are deferred, Options 5); the target is `informal` or `pair` (`formal` is refused by D3: the
  class has no formal manifestation). The site is a **ring-closed chain step** — never a lemma
  step, never a lemma-cited chain step (reading 127) — drawn on `injector/FLAW-0010/1` as
  `eligible[below(len)]` when the spec's `site` is `auto`; an explicit `site` is such a step's
  plan index and skips the draw. The reference is drawn next on the same stream by the
  template's own `draw_citation` — the author, the year, the venue of the shape's list and the
  result, in that order (reading 128) — from the fixed lexicon of obviously fictional tokens
  (`Placeholder`, `Nonesuch`, `Fictitious`, `Hypothetical`; `19XX`, `20XX`; three venues per
  shape; `Theorem 4.2`, `Lemma 3.1`, `Proposition 2.5`, `Corollary 1.7`), and the step becomes
  `By [<author> <year>, <venue>, <result>]` followed by the bare claim `$e_{k-1} = e_k$.`; the
  chain's own justification before the comma (`Expanding the square`, `Expanding the square and
  factoring out $x$`, `Rearranging the terms`, `Factoring out $y$`, `Regrouping the terms`, …)
  is what the reference stands in for (reading 130). Every other step, step 0 and the formal
  file are unchanged: a pair's formal file is the formal control's byte for byte. The one record
  is informal-only, at the site, spanning the reference after `By ` (`[3, 3 + len)`), `N/A`,
  gap, confirmed by the FLAW-0010 check as 0.6.0 reads it (the span holds the reference; the
  reference equals the one the recorded indices name; every token is a lexicon token; the step
  is `By <reference>` and the bare claim; the faithful step held the justification exactly once
  and the step does not); its dials carry `shape`, `claim`, `site`, `author`, `year`, `venue`,
  `result` (the drawn indices), `reference`, `asserted` (the bare claim), `justification`,
  `inserted` (the reference), `replaced` (the justification) and `original_steps` (the faithful
  step). `proof_status` is `invalid` and `correct` false (D7); the counts are the control's; a
  pair commands `lean_build` `pass` and `lean_axioms_standard` `true`.
- **Refusals (D3, D6).** `claim: false` (reading 131); the target `formal`; a severity other
  than gap for `claim: true`; a sub-variant other than `a` (`null`, `b`, `c`); a `shape` outside
  the lexicon's three; a non-boolean `claim`; a missing, extra or reserved dial; a site that is a
  lemma step, a lemma-cited chain step, or past the chain; `n_lemmas = n_steps` (no ring-closed
  step); two specs. Under 0.1.0, 0.2.0 and 0.3.0 the class stays refused, so their goldens
  regenerate under the versions they name.
- **Versions (D1).** `GrammarVersion::V0_4_0` is current; `generate` emits 0.4.0;
  `Producer::parse` admits eleven producer versions. The bytes 0.4.0 emits for a 0.1.0, 0.2.0 or
  0.3.0 config are theirs: only the version, the id and the manifest differ (tested on a
  FLAW-0009 `truth: false` artifact and a FLAW-0001 `local` one; the 93 earlier goldens
  regenerate).

## Evidence that the material does what the records say

1. **No Lean probe** (STATE's NEXT, step 4): the class changes no formal byte. The pair's formal
   file is the formal control's — the 0.2.0 short control's bytes, which built in Session 48 and
   build again here (the goldens' observation: exit 0, `[propext, Quot.sound]`).
2. **The goldens.** 4 cases — on `ring_chain` at the short length (`n_steps` 4, `n_lemmas` 0),
   the control (informal, formal) and FLAW-0010 `a` with `claim: true` and `shape: journal`
   (informal, pair) — each produced by `generate --dry-run` with seed 7 at
   `a1d524ae644f44ac9887a8a7f4a2ab11d673e349` by two separate runs (`evidence/goldens-run11.py`;
   run A 2026-11-02T04:32:20Z–04:33:21Z, run B 04:33:21Z–04:34:21Z; every case exit 0, fidelity
   PASS; the provenance check passed against the commit before each run;
   `evidence/goldens-run11a-summary.txt`, `evidence/goldens-run11b-summary.txt`).
   `evidence/goldens-copy11.py` compared the runs file by file — every `proof.md`, `Proof.lean`
   and `manifest.json` byte-identical, every observation equal, one engine commit — checked
   every case's shape and placed run A's outputs under `engine/generate/tests/goldens/ring_chain/`
   (`v0.4.0-…`) with a `case.json` each (`evidence/goldens-production-report.json`, SHA-256
   `15cf4ee4483019fc367bf5acb58a9ad515689709a481278faa3e61431274c594`, 14,134 bytes):
   - the control's chain is the 0.1.0 and 0.2.0 short controls' (`(x - 2 * y) ^ 2` at seed 7);
     the formal control builds and prints `[propext, Quot.sound]`
     (`s2_grammar-v0.4.0-cc0c6a4daf03-s7`, 119 formal tokens; its proof bytes are the 0.2.0
     short control's); the informal control is `…-82300f31bd83-s7`, 93 tokens, `n_steps` 4,
     `n_lemmas` 0;
   - the FLAW-0010 cases draw plan step 2 on `injector/FLAW-0010/1` among the four ring-closed
     steps (`Rearranging the terms, $4 \cdot y \cdot y + x(x - 4y) = 4y^2 - 4yx + x \cdot
     x$.`) and then the reference `[Fictitious 19XX, Annals of Imaginary Results, Corollary
     1.7]` (the author index 2, the year 0, the venue 1, the result 3), so step 2 becomes `By
     [Fictitious 19XX, Annals of Imaginary Results, Corollary 1.7] $4 \cdot y \cdot y + x(x -
     4y) = 4y^2 - 4yx + x \cdot x$.` and every other step is the control's; one record,
     informal-only, step 2, span `[3, 64)`, `N/A`, gap; `flaws_confirmed` 1, `n_flaws_0010` 1,
     `proof_status: invalid`, `correct: false`; 99 informal tokens against the control's 93;
     the pair's formal file is the formal control's byte for byte, `lean_build` commanded and
     achieved `pass`, `lean_axioms_standard` `true`;
   - every case is a true statement whose reference is the 0.4.0 formal control at this length
     (`s2_grammar-v0.4.0-cc0c6a4daf03-s7:syndeom_s2_grammar_ring_chain`), which names itself;
   - the receipts, with the build and audit stdout and stderr of the two formal cases, are under
     the external root `C:\Users\LEET\Codex\Temp\syndeom-s2-grammar-040-20261102T0425Z\`
     (`goldens-11a\` and `goldens-11b\`), whose sorted `receipt.json` SHA-256 lists — one per
     line, LF-terminated — hash to
     `ff9be2b8c11f3b46de58d7549ea1af492474d96b1923f781723d29b12cafe06c` (A) and
     `5b43a05a656edb7ce8194b2de58f487662ecc0b2de17346b5cbe9a62193dbfa2` (B); the receipts
     differ only in their timings.
3. **The artifact ids** (`s2_grammar-v0.4.0-<config_hash[0:12]>-s7`): control `82300f31bd83` /
   `cc0c6a4daf03` (informal / formal); FLAW-0010 `a` `journal` `claim: true` `1004fe87f5aa` /
   `82f3f674cd2e` (informal / pair).
4. **Tests and lint.** The crate's 50 tests pass (goldens 1 over 97 cases, grammar 15,
   properties 24, provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features
   -- -D warnings` and `cargo fmt --check` are clean. New, in `tests/grammar.rs`: FLAW-0010 over
   seven configurations (lengths (4, 0) with each shape, (6, 2), (3, 2), (24, 2) and (1, 0)):
   the site and the reference re-derived from the stream; the plan the control's but for the
   site's text; the formal file unchanged byte for byte; the text `By <reference>` and the bare
   claim, the justification held once by the faithful step and not by the flawed one; the dials;
   the one informal record and its span; the counts and the commanded `lean_build` and
   `lean_axioms_standard`; the check's refusal of confirmation under tampered indices (FAIL,
   not a refusal); the informal-only target's bytes equal to the pair's; the formal target
   refused; determinism; explicit sites at every chain step of a (6, 2) instance (the
   lemma-cited ones refused) and at the lemma steps (refused); seventeen refusals; 0.1.0, 0.2.0
   and 0.3.0 refusing the class; the 0.3.0 bytes of a FLAW-0009 and of a FLAW-0001 artifact
   unchanged under 0.4.0 — and the version facts. The gate on the committed tree is recorded in
   the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-11-02T04:23:48Z, outside the gate (`cargo audit --file
engine/Cargo.lock`). It loaded 1,395 advisories (advisory-db commit
`2295466c4e9df26647fd67d8fa1c705ac2fc7f81`), scanned 35 crate dependencies, and reported no
vulnerability (exit 0; `evidence/cargo-audit.txt`). `engine/Cargo.lock` is unchanged by this
revision: no dependency was added.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `a1d524ae…` — `engine/generate/src/grammar.rs` (`cite`, the version gate
in `prepare_with`, `realizes`), `src/lib.rs`, `tests/grammar.rs`. The threat model is packet 1's:
an offline research CLI whose assets are the integrity of generated ground truth and the
operator's machine, and whose trust boundary is an artifact directory handed to `verify` and a
config handed to `generate`. New this revision: a `shape` string selects among three constant
lists; a boolean `claim`; an explicit `site` indexes the chain.

**Pre-scan (Phase 1):** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate
roots and in the manifest crate; no external dependency was added; clippy at `-D warnings` is
clean. `cargo geiger`, `cargo deny` and Miri are not installed on this machine and were not run;
there is no `unsafe` to map.

**Panic and index sweep of the diff (2C):** no `unwrap`, `expect` or `panic!` in the edited
sources outside tests. The indexes: `spec.dials["shape"]` and `["claim"]` sit behind
`exact_keys` on the dials; `eligible[below(len)]` on a list checked non-empty;
`faithful.steps[site - 1]` with `site = m + c` and `c` an eligible chain step (drawn from the
list, or checked against it); `d.e[c - 1]` and `d.e[c]` with `c` in `1..=n`;
`SEED_FORMS[d.form]` with `form` drawn below the table's length. The lexicon indices are drawn
below each list's length (`draw_citation`), and `citation_reference` returns `None`, which is a
refusal, on any out-of-range index. No arithmetic beyond `m + c` on small counts bounded by D6's
ranges (2I).

**Serde and the config (2F):** `shape` is compared to the three constants through
`citation_venues` and everything else refused; `claim` via `as_bool`; the site via the existing
`parse_spec` (a step index at least 1). No config text reaches the proof text: the reference is
lexicon text, the bare claim is the chain's own rendering, and `shape` only selects a list.

**Subprocesses and `verify`:** unchanged. `verify` compiles only bytes the producer the
manifest names itself emits for the manifest's `(config, seed)`, and the audit tail names only
the producer's main declaration (FINDING-011's guard holds).

**Determinism:** `BTreeMap` and `Vec` only; every draw is labelled (`injector/FLAW-0010/1`: the
site, then the author, the year, the venue, the result); no floating-point value enters a
generation decision.

No new finding. **FINDING-018** (informational, backlog: `derive`, `Derivation::n` and `render`
trust their callers' preconditions) stands and its scope gains `cite`'s use of `d.lemma_of` and
`d.e[c]`, which trust the derivation's invariant that `sites` names chain steps and `e` has
`n + 1` entries; the recommendation is unchanged. FINDING-015 does not extend to the grammar
(packet 8); FINDING-016 and FINDING-017 (informational, backlog) stand; FINDING-006 (accepted
risk), FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012 and FINDING-013 (accepted)
stand as packets 1–10 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 10's numbering.

127. **The site is a ring-closed chain step only.** D5 says "a step's justification replaced by
     a fictional reference"; on `ring_chain` the sites are the chain steps closed by `ring`
     (every `n_lemmas` 0 instance offers all of them). A lemma-cited chain step is not eligible —
     listed, not taken: its justification is the citation `By Lemma j`, and replacing it would
     leave the informal Lemma j cited by nothing (an unrecorded FLAW-0007 `unused` shape in the
     informal file) and strand the step's `premises`, while in a pair the formal step would still
     cite the lemma, a modality mismatch beyond the class. A lemma step is not eligible either:
     its closing note ("both sides expand to the same polynomial") is the lemma's proof note, and
     the class names a step's justification. An instance whose every chain step is lemma-cited
     (`n_lemmas = n_steps`) is therefore refused for want of a site.
128. **The draw order on `injector/FLAW-0010/1`** is the site (when `auto`), then the author, the
     year, the venue and the result — one more labelled sequence (D4). The template draws its
     material first and then the same four, so at a given seed both producers make exactly one
     draw before the citation and draw the same reference: at seed 7 the grammar's is the
     `s2_template` 0.6.0 golden's, `[Fictitious 19XX, Annals of Imaginary Results, Corollary
     1.7]`. This is a property of the shared label and the seed, recorded so that S3 does not
     read a same-seed coincidence across producers as a generator fingerprint; the lexicon
     offers 96 references per shape, and the sweep's seed list, not this coincidence, fixes
     their distribution.
129. **The lexicon is shared, not copied.** `template::draw_citation`, `citation_reference`,
     `citation_tokens_in_lexicon` and `citation_venues` serve both producers, so one definition
     governs the draw and the check, and a change to the lexicon bumps both producers' versions
     (D1). A grammar-local copy is listed and not taken.
130. **The bare claim is `$e_{k-1} = e_k$.` with its period, and the justification is the
     chain's own phrase before the comma.** The producer asserts that the faithful step is
     exactly `<justification>, <claim>` before it replaces anything, so `asserted` and
     `justification` partition the faithful text and the 0.6.0 check's conditions (the
     justification held once there, not here) hold by construction. The comma goes with the
     justification; the template, whose material names its own claim and justification, keeps
     its connective — a difference of material, not of record.
131. **`claim: false` is refused in 0.4.0, and its design is recorded here rather than built**
     (STATE's NEXT asked for the reading before any code). The candidate: the step's identity
     perturbed as FLAW-0009's `false` material perturbs the lemma's right-hand side — one
     monomial, under the step's own presentation, the kind and the target drawn on the same
     stream — the step justified by the fictional reference, and the formal file unchanged, so
     that a pair's informal step is false while its formal step is the faithful one. The chain's
     later informal steps would then either re-derive from the perturbed polynomial
     (`propagated`: the informal chain ends away from the claim while the formal proof still
     reaches it) or assert the correct consequence (`local`: the next step's left-hand side the
     faithful `e_k`, so the informal argument silently repairs itself). Both keep the formal
     file unchanged, as D5 requires; but a pair whose informal and formal arguments reach
     different conclusions, or whose step k is false informally and true formally, strains the
     alignment D2 calls load-bearing, and the record's `N/A` then means that the kernel cannot
     see the flaw at all. Whether that is the class's intended fatal cell for a grammar, which
     variant, and whether a `propagated` / `local` dial belongs to FLAW-0010 or only to
     FLAW-0001, are Al's to rule; a ruling that realizes it is its own version.
132. **The formal file is unchanged, the instance `N/A` for every target, the target `formal`
     refused** (D3; `check_spec`), a pair's formal file byte-identical to the formal control's;
     the pair commands `lean_build` `pass` and `lean_axioms_standard` `true`. The formal dials
     carry the site's fragment as both `inserted` and `replaced` with empty
     `original_fragments`, as the template's FLAW-0010 does; `assemble` writes no formal record
     (`manifests_in`).
133. **No Lean probe** (NEXT's step 4): the class changes no formal byte, and the formal
     control's build is the pair's evidence.
134. **One informal-only record spanning the reference**, `[3, 3 + len)` after `By `, with the
     dials listed above; the site is the plan step index, as every grammar record's is.
135. **The goldens are the four NEXT's step 5 named:** at (4, 0), seed 7, the 0.4.0 controls
     (informal, formal — so that the flawed cases' truth reference names a golden) and
     FLAW-0010 `journal` `claim: true` (informal, pair). The 0.4.0 formal control's proof bytes
     are the 0.2.0 short control's, so a reader may compare the two across versions; only the
     version, the id and the manifest differ.
136. **The counts are the control's** (`n_steps`, `n_lemmas`; `n_tokens` achieved only): 99
     informal tokens against the control's 93 — the reference is longer than the justification
     it replaces, a documented-weak surface cue of the class in this grammar, as the hedge is
     shorter in the template's FLAW-0004.
137. **The severity is fixed by `claim`** (gap for `true`, D5) and a mismatch is refused;
     sub-variant `a` is required, `b` and `c` refused by `check_spec` (Options 5); `shape` must
     be one of the lexicon's three; a missing, extra or reserved dial is refused.
