# S2 implementation packet 12 — `s2_grammar` 0.5.0 (ADR-0011, Session 51, 2026-11-02)

This packet records the twelfth implementation session of ADR-0011 (D11): `s2_grammar` 0.5.0 —
FLAW-0003 `cycle` on the grammar `ring_chain`, a cited lemma re-justified by a citation of the
theorem while the chain step that cites the lemma is unchanged (D5, D6) — with its goldens.
Nothing here is a result: no CLAIM, no fidelity sweep, no artifact under `corpora/`.

Branch `claude/s2-grammar-0.5.0`, from `main` `602a5962f7379e2a71de022bbd20003ab4e60247` (PR
36's merge):

| Commit | What |
|---|---|
| `c6b240942b56fb0b50f5c41a8f838aa77b7ce585` | the 0.5.0 producer: `src/grammar.rs` (`circulate`, the version, `realizes`, `LEMMA_NOTE`), `src/lib.rs`, `tests/grammar.rs`, the crate README |
| `2c3f2b0ed38985a3de1cb27c133d03eced6c810b` | the 4 goldens of `s2_grammar` 0.5.0, produced at `c6b24094…`, the goldens README and the count (101 cases) |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 51 |

## What was built

- **FLAW-0003 `cycle` on `ring_chain` (D5, as `s2_template` 0.6.0 reads it).** The config's one
  dial is `cycle_length` (2 in v0.1; any other value is refused); the sub-variant is `cycle`
  (`direct` is refused: its material — the goal or its contrapositive as a lemma proved by the
  theorem — is hand-authored and `s2_template`-only, D6; reading 147); the severity is fatal
  (FLAWS.md's range for the class); the target is `informal` or `pair` (`formal` is refused by
  D3: declaration order forbids a formal cycle, so a formal-only artifact has nothing to carry
  it); `length.n_lemmas` is at least 1. The site is a **lemma step** — drawn on
  `injector/FLAW-0003/1` as `1 + below(m)` among the `m` lemma steps when the spec's `site` is
  `auto`; an explicit `site` is a lemma step's plan index (1 to `m`) and skips the draw. The
  lemma step's informal closing note, `both sides expand to the same polynomial` (the text every
  faithful lemma step ends with, now the constant `LEMMA_NOTE`), is replaced by a citation of
  the theorem, `this follows from the theorem` (the constant `CYCLE_PHRASE`; reading 139), so
  that `Lemma j. For all integers $x$ and $y$, $e_{k-1} = e_k$; both sides expand to the same
  polynomial.` becomes `Lemma j. For all integers $x$ and $y$, $e_{k-1} = e_k$; this follows
  from the theorem.`, and the phrase is recorded as the step's one premise (`cites:
  "theorem"`). The chain step that cites the lemma (`By Lemma j, …`) is unchanged — its premise
  already names the lemma — so the plan's premise graph (D2, 0.6.0) contains the cycle
  `[syndeom_s2_grammar_ring_chain_lemma_<j>, syndeom_s2_grammar_ring_chain]` (reading 138).
  Every other step, step 0 and the formal file are unchanged: the lemma is proved by `intro x
  y; ring` and cited as a term, as the control has it, so a pair's formal file is the formal
  control's byte for byte. The one record is informal-only at the lemma step, spanning the
  citation phrase (`inserted`; the note is `replaced` — reading 142), `NO`, fatal, confirmed by
  the structural check as 0.6.0 reads it (the plan's premise graph, re-derived at assembly,
  contains the recorded cycle through the lemma at the site; each recorded citation phrase is
  at its recorded span in the rendered step and occurs there exactly once; reading 149); its
  dials carry `cycle_length`, `site` (the lemma step's plan index), `cycle` (the declarations
  visited from the lemma), `citations` (the two premise phrases of the cycle, each with its
  step and span — reading 141), `inserted`, `replaced` and `original_steps` (the faithful lemma
  step). `proof_status` is `invalid` and `correct` false (D7); the counts are the control's; a
  pair commands `lean_build` `pass` and `lean_axioms_standard` `true`.
- **Refusals (D3, D6).** `direct` (reading 147); a `null` or other sub-variant; a severity other
  than fatal; `cycle_length` other than 2, non-integer, missing, or beside an extra or reserved
  dial (`cycle`, `citations`, `site`, …); `length.n_lemmas` 0; the target `formal`; a site that
  is a chain step (lemma-cited or not), or 0, or past the plan; two specs. Under 0.1.0, 0.2.0,
  0.3.0 and 0.4.0 the class stays refused, so their goldens regenerate under the versions they
  name.
- **Versions (D1).** `GrammarVersion::V0_5_0` is current; `generate` emits 0.5.0;
  `Producer::parse` admits twelve producer versions. The bytes 0.5.0 emits for a 0.1.0 to 0.4.0
  config are theirs: only the version, the id and the manifest differ (tested on a FLAW-0010
  `claim: true` artifact, a FLAW-0009 `truth: false` one and the (4, 1) control; the 97 earlier
  goldens regenerate). `LEMMA_NOTE` names the note `lemma_step` already emitted (reading 148);
  no byte of any earlier version changes.

## Evidence that the material does what the records say

1. **No Lean probe** (STATE's NEXT, step 4): the class changes no formal byte. The pair's formal
   file is the (4, 1) formal control's — the 0.3.0 control's bytes, which built in Session 49
   and build again here (the goldens' observation: exit 0, `[propext, Quot.sound]`, the main
   declaration using Lemma 1).
2. **The goldens.** 4 cases — on `ring_chain` at the short length with one lemma (`n_steps` 4,
   `n_lemmas` 1), the control (informal, formal) and FLAW-0003 `cycle` with `cycle_length` 2
   (informal, pair) — each produced by `generate --dry-run` with seed 7 at
   `c6b240942b56fb0b50f5c41a8f838aa77b7ce585` by two separate runs (`evidence/goldens-run12.py`;
   run A 2026-11-02T07:05:22Z–07:06:19Z, run B 07:06:19Z–07:07:14Z; every case exit 0, fidelity
   PASS; the provenance check passed against the commit before each run;
   `evidence/goldens-run12a-summary.txt`, `evidence/goldens-run12b-summary.txt`).
   `evidence/goldens-copy12.py` compared the runs file by file — every `proof.md`, `Proof.lean`
   and `manifest.json` byte-identical, every observation equal, one engine commit — checked
   every case's shape and placed run A's outputs under `engine/generate/tests/goldens/ring_chain/`
   (`v0.5.0-…`) with a `case.json` each (`evidence/goldens-production-report.json`, SHA-256
   `6a4708f3856360267b8d8d9d717f31a9220b6547871c5324403e07d5298fae2a`, 21,035 bytes):
   - the control's chain is the 0.3.0 control's (`(x - 2 * y) ^ 2` at seed 7, its second step
     stated as Lemma 1 and cited at plan step 3); both files are the 0.3.0 control's byte for
     byte; the formal control builds and prints `[propext, Quot.sound]`
     (`s2_grammar-v0.5.0-9ffdfe550dc7-s7`, 163 formal tokens); the informal control is
     `…-1ef0cde05162-s7`, 125 tokens, `n_steps` 5, `n_lemmas` 1;
   - the FLAW-0003 cases draw Lemma 1 on `injector/FLAW-0003/1` (with one lemma the draw is
     `1 + below(1)`, Lemma 1), so plan step 1 becomes `Lemma 1. For all integers $x$ and $y$,
     $4 \cdot y \cdot y + x(x - 4y) = 4y^2 - 4yx + x \cdot x$; this follows from the theorem.`
     and every other step is the control's, step 3 (`By Lemma 1, …`) among them; one record,
     informal-only, step 1, span `[97, 126)`, `NO`, fatal, with `cycle`
     `[syndeom_s2_grammar_ring_chain_lemma_1, syndeom_s2_grammar_ring_chain]` and `citations`
     step 1 `this follows from the theorem` at `[97, 126)`, step 3 `Lemma 1` at `[3, 10)`;
     `flaws_confirmed` 1, `n_flaws_0003` 1, `proof_status: invalid`, `correct: false`; 123
     informal tokens against the control's 125 (reading 146); the pair's formal file is the
     formal control's byte for byte, `lean_build` commanded and achieved `pass`,
     `lean_axioms_standard` `true`;
   - every case is a true statement whose reference is the 0.5.0 formal control at this length
     (`s2_grammar-v0.5.0-9ffdfe550dc7-s7:syndeom_s2_grammar_ring_chain`), which names itself;
   - the receipts, with the build and audit stdout and stderr of the two formal cases, are under
     the external root `C:\Users\LEET\Codex\Temp\syndeom-s2-grammar-050-20261102T0655Z\`
     (`goldens-12a\` and `goldens-12b\`), whose sorted `receipt.json` SHA-256 lists — one per
     line, LF-terminated — hash to
     `b942bf69230899b422f1f16c58dc58be25819b10f407e6f6fece4e598e9028d2` (A) and
     `c68b1174e382dbc5d5b0515374f0ea47bf0155f516c65813c48e48604c874bc0` (B); the receipts
     differ only in their timings (the pair's build 6,755 ms and audit 6,653 ms in run A).
3. **The artifact ids** (`s2_grammar-v0.5.0-<config_hash[0:12]>-s7`): control `1ef0cde05162` /
   `9ffdfe550dc7` (informal / formal); FLAW-0003 `cycle` `a47a5cca3cb6` / `82ce0fab51e2`
   (informal / pair).
4. **Tests and lint.** The crate's 51 tests pass (goldens 1 over 101 cases, grammar 16,
   properties 24, provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features
   -- -D warnings` and `cargo fmt --check` are clean. New, in `tests/grammar.rs`: FLAW-0003
   `cycle` over six configurations (lengths (4, 1), (6, 2), (3, 3), (24, 2), (1, 1) and
   (12, 5)): the site re-derived from the stream; the plan the control's but for the lemma
   step's text and premises; the formal file unchanged byte for byte; the text (the stem, then
   the citation in place of the note; the note held once by the faithful step and not by the
   flawed one, the phrase once); the premise graph's edges (`m + 1`) and the cycle through the
   lemma (none in the control; none of length 3); the dials; the one informal record and its
   span; the counts and the commanded `lean_build` and `lean_axioms_standard`; the check's
   refusal of confirmation under a tampered `cycle` and under a tampered citation span (FAIL,
   not a refusal); the informal-only target's bytes equal to the pair's and the formal target
   refused; determinism; explicit sites at both lemma steps of a (6, 2) instance (the chain
   steps refused); sixteen refusals; 0.1.0 to 0.4.0 refusing the class; the 0.4.0 bytes of a
   FLAW-0010, a FLAW-0009 and a control artifact unchanged under 0.5.0 — and the version facts.
   The gate on the committed tree is recorded in the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-11-02T07:07:34Z, outside the gate (`cargo audit --file
engine/Cargo.lock`). It loaded 1,395 advisories (advisory-db commit
`2295466c4e9df26647fd67d8fa1c705ac2fc7f81`, as in Sessions 48–50), scanned 35 crate
dependencies, and reported no vulnerability (exit 0; `evidence/cargo-audit.txt`).
`engine/Cargo.lock` is unchanged by this revision: no dependency was added.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `c6b24094…` — `engine/generate/src/grammar.rs` (`circulate`, the
constants `CYCLE_LENGTH`, `LEMMA_NOTE` and `CYCLE_PHRASE`, `GrammarVersion::V0_5_0`, the
dispatch in `prepare_with`, `realizes`), `src/lib.rs`, `tests/grammar.rs`. The threat model is
packet 1's: an offline research CLI whose assets are the integrity of generated ground truth
and the operator's machine, and whose trust boundary is an artifact directory handed to
`verify` and a config handed to `generate`. New this revision: an integer `cycle_length`
compared to a constant; an explicit `site` indexing the lemma steps.

**Pre-scan (Phase 1):** `cargo audit` is clean (above); `#![forbid(unsafe_code)]` stands in
both crate roots and in the manifest crate, and no `unsafe` block exists in the crate; no
external dependency was added; clippy at `-D warnings` is clean. Unlike packets 1–11,
`cargo-geiger` 0.13.0 and `cargo-deny` 0.20.2 are now installed on this machine; Miri is not
available for the pinned toolchain (`1.91.0-x86_64-pc-windows-msvc`). `cargo geiger` (from the
package directory, 2026-11-02T07:11:40Z) compiled the crate and then failed with an I/O error
naming a file that does not exist, `engine/ingest/tests/relicense_exceptions.rs` (not a target
of any crate's `Cargo.toml`; a stale reference in the build directory's metadata is the
executor's guess, not a finding), so it produced no unsafe map; there is no `unsafe` to map.
`cargo deny check` (the workspace has no `deny.toml`, so the tool's default configuration
applied): advisories ok, bans ok, sources ok; licenses FAILED because the default configuration
allows no licence explicitly, so every one of the 34 crates in the graph was rejected — 20
`MIT OR Apache-2.0`, 3 `MIT`, the five workspace crates' own `PolyForm-Noncommercial-1.0.0`,
and six other permissive expressions — which is the absence of an allow-list, not a licence
problem: the dependencies' licences are the ones S-0008 and `THIRD-PARTY-NOTICES.md` record.
Whether a `deny.toml` naming the permitted licences should enter the repository is noted for
Al as housekeeping, not as a finding. The outputs are retained under the external root
(`cargo-geiger.txt`, `cargo-deny-full.txt`).

**Panic and index sweep of the diff (2C):** no `unwrap`, `expect` or `panic!` in the edited
sources outside tests. The indexes: `spec.dials["cycle_length"]` sits behind `exact_keys` on
the dials; `d.sites[j - 1]` and `faithful.steps[j - 1]` with `j` in `1..=m` (drawn as `1 +
below(m)`, or checked against that range); `plan.steps[k - 1]` for `k` in `[j, citing]` with
`citing = plan_index(m, c)` and `c = d.sites[j - 1]`, a chain step in `1..=n`, so `k ≤ m + n`,
the plan's step count; `strip_suffix` and `unique_span` return `Option` and refuse on `None`.
No arithmetic beyond `1 + below(m)` and `m + c` on small counts bounded by D6's ranges (2I).

**Serde and the config (2F):** `cycle_length` via `as_u64` compared to the constant 2;
`sub_variant` compared to two string constants; the severity checked twice (the range in
`check_spec`, fatal in `circulate`); the site via the existing `parse_spec` (a step index at
least 1). No config text reaches the proof text: the citation phrase is a constant of the
producer, and the stem is the faithful lemma step's own text.

**Subprocesses and `verify`:** unchanged. `verify` compiles only bytes the producer the
manifest names itself emits for the manifest's `(config, seed)`, and the audit tail names only
the producer's main declaration (FINDING-011's guard holds).

**Determinism:** `BTreeMap` and `Vec` only; one labelled draw (`injector/FLAW-0003/1`: the
site, when `auto`); no floating-point value enters a generation decision.

No new finding. **FINDING-018** (informational, backlog: `derive`, `Derivation::n` and `render`
trust their callers' preconditions) stands and its scope gains `circulate`'s use of `d.sites`
and `plan_index`, which trust the derivation's invariant that `sites` has `m` entries naming
chain steps in `1..=n` and that the plan lists the `m` lemma steps first; the recommendation
is unchanged. FINDING-015 does not extend to the grammar (packet 8); FINDING-016 and
FINDING-017 (informational, backlog) stand; FINDING-006 (accepted risk), FINDING-007
(backlog), FINDING-009, FINDING-010, FINDING-012 and FINDING-013 (accepted) stand as packets
1–11 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 11's numbering.

138. **The cycle is lemma → theorem → lemma, with the citing chain step unchanged.** D5's
     recipe — "Lemma A cites Theorem T, and T's proof cites Lemma A" — and 0.6.0's reading
     (the lemma's proof cites the theorem while the key step cites the lemma) are realized on
     the grammar by changing one text only: the lemma step's closing note. The chain step that
     cites the lemma already carries the premise `Lemma j` (0.1.0, D2), so the theorem's side
     of the cycle exists in every control with a lemma and the injection adds the lemma's
     side. The site is therefore a lemma step only; `n_lemmas` 0 is refused for want of one.
     An alternative — re-justifying a ring-closed chain step by the theorem — makes no cycle
     through declarations (a main-proof step citing the main declaration is a self-loop of
     length 1, FLAWS.md's `direct` self-reference) and is listed, not taken.
139. **The citation phrase and its form.** The note `both sides expand to the same polynomial`
     is replaced by `this follows from the theorem`: a full clause in the note's position after
     the semicolon, where the template's 0.6.0 material wrote `by the theorem` inside a longer
     proof note of its own. The phrase is a constant of the producer (`CYCLE_PHRASE`), neither
     drawn nor a dial, so it is a fixed surface cue of the class in this grammar — as the
     template's hedge lexicon entries and the grammar's padding texts are documented-weak
     cues. Whether a drawn phrase lexicon (as FLAW-0004's hedges) is wanted for S3's cells is
     Al's to say; it would be a later version.
140. **The draw order on `injector/FLAW-0003/1`** is one draw, the site (`1 + below(m)`), when
     `auto`; an explicit site is a lemma step's plan index, as FLAW-0009's is. A new label and
     one draw bump the version (D1, D4).
141. **`citations` records the two phrases of the cycle, not every premise of the plan.** D5
     says "the two citation phrases are at the recorded spans"; an instance with `m ≥ 2` has
     other lemma citations outside the cycle, which are not recorded. The `cycle` dial is
     `premise_cycle`'s path from the lemma, `[…_lemma_<j>, syndeom_s2_grammar_ring_chain]`,
     as 0.6.0 records `[lemma_1, main]`.
142. **The record spans the citation phrase, not the whole lemma step.** The template's 0.6.0
     record spans the whole inserted lemma (`[0, len)`, `replaced: null`) because its material
     inserts the lemma; here the lemma exists in the control and only its note changes, so the
     record is a replacement — `inserted` the phrase, `replaced` the note — as FLAW-0010's
     reference replaces the justification, and the span is the phrase's (`[97, 126)` at the
     goldens' length).
143. **The formal file is unchanged, the instance `NO` for every target, the target `formal`
     refused** (D3; `check_spec`), a pair's formal file byte-identical to the formal control's;
     the pair commands `lean_build` `pass` and `lean_axioms_standard` `true`. The formal dials
     carry the lemma's declaration lines as both `inserted` and `replaced` with empty
     `original_fragments` and **no `declaration` key**, although the site is a lemma step:
     `assemble` writes no formal record (`manifests_in`), and the tests' predicted observation
     reads a `declaration` dial as an unused declaration, which the cited lemma is not. The
     formal dials are carried unused, as FLAW-0010's are.
144. **No Lean probe** (NEXT's step 4): the class changes no formal byte, and the formal
     control's build is the pair's evidence.
145. **The goldens are the four NEXT's step 5 named:** at (4, 1), seed 7, the 0.5.0 controls
     (informal, formal — so that the flawed cases' truth reference names a golden) and FLAW-0003
     `cycle` (informal, pair). Both control files are the 0.3.0 control's bytes, so a reader may
     compare the two across versions; only the version, the id and the manifest differ.
146. **The counts are the control's** (`n_steps` 5, `n_lemmas` 1; `n_tokens` achieved only): 123
     informal tokens against the control's 125 — the citation is two tokens shorter than the
     note, a documented-weak surface cue of the class in this grammar, in the opposite
     direction from FLAW-0010's longer reference.
147. **`direct` is refused, and a grammar `direct` is listed, not taken.** D6 names `cycle` as
     the grammar's FLAW-0003 form and `direct` as `s2_template`-only (hand-authored
     mathematics). A generated `direct` is conceivable — a lemma stating the theorem's own
     identity `e_0 = e_n`, proved by `exact syndeom_s2_grammar_ring_chain`, a forward reference
     the kernel rejects at the lemma's proof line (check K, `YES-KERNEL`), with the chain's key
     step citing it — and would give the grammar a formal manifestation of the class. Whether
     it is wanted is Al's to say; it would be its own version and a D6 amendment.
148. **`LEMMA_NOTE` is extracted as a constant** used by `lemma_step` (no byte changes: the 97
     earlier goldens regenerate); the unused-lemma and padding texts keep their own literal
     copies of the same sentence, because they are not sites of the class.
149. **The structural check is the template's, unchanged:** `assemble`'s FLAW-0003 `cycle`
     branch re-derives the cycle from the plan at assembly and compares it with the recorded
     dial, and holds each recorded citation phrase at its span, exactly once in its step; a
     tampered cycle or a tampered span yields a FAIL verdict, not a refusal. The severity is
     fixed (fatal; FLAWS.md's range) and `cycle_length` is 2 only; a spec naming the reserved
     dials `cycle` or `citations` is refused by `parse_spec`.
