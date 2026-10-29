# S2 implementation packet 5 — `s2_template` 0.5.0 (ADR-0011, Session 44, 2026-10-28)

This packet records the fifth implementation session of ADR-0011 (D11): `s2_template` 0.5.0 —
FLAW-0004 (D5: the hedge lexicon, `hidden_step`, the aligned tactic step omitted) realized on
`sqrt2_irrational` as the FLAWS.md FLAW-0011 example's hedge, and FLAW-0011 as a commanded
composite (D5: `embedded`), both with goldens, together with the FINDING-014 seed rule. Nothing
here is a result: no CLAIM, no fidelity sweep, no artifact under `corpora/`.

Branch `claude/s2-template-0.5.0`, from `main` `fbcc6c4e682e6cbaaf14d8e42a28cfcd5d2f6cb0` (PR
29's merge):

| Commit | What |
|---|---|
| `7be41123c698a07b9e56cab2aa2a1e9d257944b8` | the 0.5.0 producer: FLAW-0004's material shape and injection, the composite, the seed material, the FINDING-014 rule, the tests, the crate README |
| `4edc3f73f43e32d615cd7ba01093215f133c3e08` | the 4 goldens of 0.5.0, produced at `7be41123…`, and the goldens README |
| `074e5775e9626b6925fa8ff8cb32a15ad2aec069` | the records: this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 44 |
| `1c0c5fcdf1ffd375b109f272461a63c49330e007` | Al's rulings 2 and 3 on PR 30 applied: a composite embeds one gap-or-fatal instance of FLAW-0001 through FLAW-0010; the cosmetic composite goldens removed |
| `5afc00d7809fa34a0c163e08c6d8e353d374bcae` | the 0.5.0 goldens regenerated at `1c0c5fcd…`: FLAW-0004 (proof bytes unchanged) and the FLAW-0002 gap composite |
| the record commit after it | the rulings recorded: this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 44's addendum |

## What was built

- **FLAW-0004 (D5).** A seed's FLAW-0004 material is replacement material at a main-proof step
  whose `hidden` names the step's bare claim, the justification the recipe deletes, and the
  aligned tactic lines it omits; its one dial is `hidden_step` (`routine-true` cosmetic,
  `nontrivial-true` gap, `false` fatal — the severity is fixed by the dial, and a spec that
  contradicts it is refused). The injector selects the material as it selects every other, then
  draws the hedge from the fixed lexicon — `Clearly`, `It is easy to see that`, `Obviously`, `By a
  standard argument`, `One checks that` — with the next draw of the instance's stream
  `injector/FLAW-0004/1`, and the site step becomes the hedge, a space and the claim. The formal
  fragment is the faithful fragment without the omitted block, never `sorry`: for the seed's
  step 3, `have hp : 2 ∣ p := by` and `obtain ⟨k, hk⟩ := hp` stay and the four tactic lines of
  the proof of `hp` go, so the kernel reports an unsolved goal at the `have` line. The records
  carry `hedge` (the index), `hedge_text`, and `justification` and `claim` (informal) or `omitted`
  (formal) beside `inserted`, `replaced` and the originals; those names are reserved. The target
  `formal` is refused (FLAWS.md: the class manifests in informal); for a pair the instance is
  `YES-KERNEL` with a record in each file, for an informal-only artifact it is `N/A`.
- **The informal check (D5, S2.2).** A FLAW-0004 informal record is confirmed when the site step
  equals the hedge entry, a space and the bare claim; the faithful text (`replaced`) held the
  justification exactly once; and the step does not hold it. The formal record is confirmed by
  check K. `check_seed` enforces the material's shape: the site step of the material is the
  bare claim, the faithful step holds the justification exactly once, the omitted block occurs
  exactly once in the faithful fragment and removing it yields the material's fragment.
- **FLAW-0011 (D5).** Commanded with the one dial `embedded`, a list of FlawSpecs parsed exactly
  as the config's are — one in 0.5.0, since the materials are whole-argument variants and two
  cannot be made disjoint — never FLAW-0011, FLAW-0012 or FLAW-0013, on a seed whose statement
  is true, with the spec's severity equal to the embedded instance's (it inherits). The embedded
  spec is injected as instance 1; the composite is instance 2 with a record in each file the
  artifact has: step 0 with span `[0, len)` in the informal file, the main declaration at its
  header line in the formal file (the formal rendering now records that line), `lean_catchable`
  inherited, dials `embedded` (as given), `embedded_instances`, `site: 0`, `inserted` (the
  statement, or the header line) and `replaced: null`. It is confirmed when the seed's truth is
  by a kernel-checked witness or Mathlib (D7) and every record of every embedded instance passes.
  Counts are per instance: `n_flaws_total` 2, `n_flaws_0004` 1, `n_flaws_0011` 1.
  **Amended by Al's rulings 2 and 3 on PR 30 (below):** the embedded instance must be of
  severity gap or fatal, so that every composite is an invalid proof, and of FLAW-0001 through
  FLAW-0010; the cosmetic FLAW-0004 composite — FLAWS.md's example — is refused, and the
  committed composite goldens embed `continuity_is_not_uniform`'s FLAW-0002 gap.
- **The seed (D6).** `sqrt2_irrational` gains the FLAW-0004 `routine-true` material at step 3:
  the FLAWS.md FLAW-0011 example — `2 | p² ⇒ 2 | p` replaced by a hedge and the parity argument
  dropped. Its faithful steps, padding material and witness are unchanged, so every 0.4.0 golden
  regenerates byte for byte.
- **FINDING-014 (packet 4), resolved.** `check_seed` refuses a seed whose own style step carries
  a padding header; a flawed step cannot carry one by its shape.
- **Versions (D1).** `Version::V0_5_0` is current; `generate` emits 0.5.0; FLAW-0004 and
  FLAW-0011 are refused by every earlier version; the 43 earlier goldens regenerate under the
  versions they name, with no golden rewritten.

## Evidence that the Lean material does what the records say

1. **Probes before the material** (`evidence/lean-probe/`, `lake env lean` from `lean/`, one
   process at a time, `run.sh`; 2026-10-28T23:26:56Z–23:27:11Z, about 8 s each):

   | Probe | Exit | What |
   |---|---|---|
   | `s2i_hedged_p1` | 1 | the faithful file with the whole proof of `have hp : 2 ∣ p := by` omitted: one error, `7:21: error: unsolved goals … ⊢ 2 ∣ p`, at the `by` of line 7; no cascade — `hp` stays bound, and every later line elaborates |
   | `s2i_hedged_p2` | 1 | the alternative, the case split kept and only the odd case omitted: `7:21: error: unsolved goals case inr … hp : p % 2 = 1 ⊢ 2 ∣ p` |

   Both realize "the aligned tactic step omitted, leaving an unsolved goal" at the site's first
   fragment line; `p1` is the one built (reading 40). The seed material was written after these
   probes compiled.
2. **The goldens, first production** (superseded for the composite cases by Al's ruling 2 on PR
   30, below; the FLAW-0004 cases were regenerated with unchanged proof bytes). 4 cases —
   FLAW-0004 `routine-true` (informal, pair) and FLAW-0011 embedding it (informal, pair) — each
   produced by `generate --dry-run` with seed 7 at
   `7be41123c698a07b9e56cab2aa2a1e9d257944b8` by two separate runs (`evidence/goldens-run5.py`;
   run A 2026-10-28T23:41:58Z–23:42:26Z, run B 23:42:26Z–23:42:55Z; every case exit 0, fidelity
   PASS, the provenance check passed against the commit before each run;
   `evidence/goldens-run5a-summary.txt`, `evidence/goldens-run5b-summary.txt`).
   `evidence/goldens-copy5.py` compared the runs file by file — every `proof.md`, `Proof.lean` and
   `manifest.json` byte-identical, every observation equal, one engine commit — compared each
   pair's `Proof.lean` with the probe, and placed run A's outputs under
   `engine/generate/tests/goldens/sqrt2_irrational/` with a `case.json` each
   (`evidence/goldens-production-report.json`, SHA-256
   `784b17625743069b6af730c3140e65c8a2d19dd2e33dbc0fc277f83a9a1dec39`, 6,751 bytes):
   - both pair goldens' `Proof.lean` are byte-identical to `s2i_hedged_p1.lean` (SHA-256
     `603c0f1b55fa2ff600f82729e666014ea1b9135beb6cb940cbafc5804bc427f2`, 715 bytes), which has
     no tail;
   - both pair cases fail to build (exit 1) with the first error at line 7, the site's first
     fragment line (check K); `lean_axioms_standard` is unmeasured (`null`, with its reason);
   - at seed 7 the injector draws hedge 1, `It is easy to see that`: step 3 reads `It is easy to
     see that $p$ is even. Write $p = 2k$.` (51 code points); the other steps are faithful;
   - every instance is confirmed (`flaws_confirmed` 1 and 2); the artifacts are
     `proof_status: valid`, `correct: true` (reading 43), with `lean_build` commanded `fail` for
     the pairs; the informal-only artifacts carry `N/A`.
   - The receipts, with build stdout and stderr, are under the external root
     `C:\Users\LEET\Codex\Temp\syndeom-s2-template-050-20261028T2330Z\goldens-5a\` and
     `goldens-5b\`; the sorted lists of the four `receipt.json` SHA-256s hash to
     `40c5d738ec78bcea6c8be9e8593f447beae8e858bae9a0763aef81bfbec28494` and
     `910877218b075ef61659ecc1fe510c390324edd833045e56bdc9cb264f07cc8a`.
3. **The artifact ids** (`s2_template-v0.5.0-<config_hash[0:12]>-s7`): FLAW-0004 `0abb0ceac4c5`
   / `a389934b677e`; FLAW-0011 `f0ead84420be` / `a77967460c1f` (informal / pair).
4. **Tests and lint.** The crate's 31 tests pass (goldens 1 over 47 cases, properties 20,
   provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features -- -D warnings`
   and `cargo fmt --check` are clean. New: `d5_flaw_0004_deletes_the_justification_and_prefixes_a_drawn_hedge`
   (the draw, the text, the omission, the records, N/A for informal-only, the FAIL verdicts when
   the build passes or fails elsewhere, forty seeds' hedges all in the lexicon, ten refusals)
   and `d5_flaw_0011_is_a_commanded_composite_of_one_embedded_instance` (the embedded instance
   equal to the standalone one, the four records and their locations, counts, the composite
   failing with its embedded instance, N/A for informal-only, a gap composite invalid, fifteen
   refusals); `every_case` now embeds every true seed's material in a composite (FLAW-0001,
   0002, 0004, 0007, 0009 and 0014), and the record-count property is per instance's class;
   `d6` checks the FLAW-0004 material's shape rules and the FINDING-014 rule. The gate on the
   committed tree is recorded in the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-10-28T23:41:45Z, outside the gate. It loaded 1,381
advisories (advisory-db commit `40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`), scanned 35 crate
dependencies, and reported no vulnerability (exit 0; `evidence/cargo-audit.txt`).
`engine/Cargo.lock` is unchanged by this revision.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `7be41123…` — `engine/generate/src/plan.rs` (`main_line`),
`src/template.rs` (`HiddenStep`, `check_hidden_material`, `parse_spec`, the FLAW-0004 path of
`inject`, `compose`, the composite records in `assemble`) and `seeds/sqrt2_irrational.json`. The
threat model is packet 1's: an offline research CLI whose assets are the integrity of generated
ground truth and the operator's machine, and whose trust boundary is an artifact directory
handed to `verify` and a config handed to `generate`. New this revision: a config may carry a
nested FlawSpec list (`embedded`), and a dial value drawn by the producer selects a lexicon
entry that is written into the proof text.

**Pre-scan:** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate roots and in
the manifest crate; no external dependency was added; clippy at `-D warnings` is clean.

**Panic and index sweep of the diff:** no `unwrap`, `expect` or `panic!` was added. The indexes:
`HEDGES[i]` with `i = stream.below(5)`; `style.steps.get(m.site - 1)` behind `m.site >= 1`;
`faithful.fragment.windows(n)` with `n ≥ 1` checked first, and the drain bounded by the window
position; `embedded[0]` behind the emptiness check; `spec.dials["embedded"]` behind `exact_keys`
(the map index would panic on an absent key, as the padding module's `["kinds"]` would — both
are guarded the same way); `st.steps[0]` is the statement every rendered file has;
`lines().nth(main_line - 1)` returns an error, not a panic, if absent. The nested list is bounded
to one spec and nesting is refused by class before any recursion, so a config cannot deepen the
call stack.

**Serde:** `HiddenStep` carries `deny_unknown_fields`; `Material.hidden` defaults to `None`, so
every earlier seed and every golden `case.json` still parses. The embedded spec is parsed by
`parse_spec` with exact keys and the reserved-dial check, like a top-level spec.

**Subprocesses and `verify`:** unchanged. `verify` still compiles only bytes the producer itself
emits for the manifest's (config, seed) and names only the producer's main declaration
(FINDING-011). The lexicon is a compiled-in constant; a drawn index selects among five fixed
strings, so no config value reaches the proof text except through the seed material and the
existing dial echo.

```
FINDING-014: a seed's own step may carry a padding header
Severity:    Informational
Category:    Serde / Logic
Location:    engine/generate/src/template.rs (check_seed)
Status:      Resolved (0.5.0, 7be41123) — a style step carrying a padding header is refused;
             a flawed step cannot carry one by its shape; tested in d6.

FINDING-015: a composite's truth witness is taken on the seed's word
Severity:    Informational
Category:    Logic (D7)
Location:    engine/generate/src/template.rs (assemble, the composite records)
Status:      Open (backlog)
Description: The FLAW-0011 record is confirmed when `statement_status.method` is `lean-kernel`
             or `mathlib` with a non-empty `reference` and every embedded record passes. The
             reference is the seed's compiled-in string; nothing in the producer checks that
             the named module and declaration exist, so a seed whose reference is misspelled
             would still confirm. The gate builds and audits every witness module and every
             seed is executor-authored (D6), so nothing attacker-controlled reaches this.
Recommendation: at `generate`, resolve a `lean-kernel` reference against `root`: the module
             file exists under `lean/Syndeom/Seeds/` and declares the named theorem; refuse
             (exit 2) otherwise. A later producer revision.
```

FINDING-006 (accepted risk), FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012 and
FINDING-013 (accepted) stand as packets 1–4 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 4's numbering. **Al ruled on 2026-10-28 (below): 39, 40, 41, 42, 44, 45, 47, 48, 49, 50
and 51 are accepted as read; 43 and 46 are replaced.**

39. **The hedge is drawn, not commanded.** D5 lists `hedge` among FLAW-0004's dials as "an index
    into the fixed hedge lexicon" and says a record's dials hold "every lexicon entry drawn", as
    FLAW-0010's lexicon indices are drawn. The reading: the spec's one dial is `hidden_step`;
    the injector draws the hedge from the instance's stream after the material and records
    `hedge` (the index) and `hedge_text`; a config that sets either is refused. The alternative,
    a commanded `hedge`, would make the entry a config choice rather than a draw.
40. **The formal realization is the whole proof block omitted.** "The aligned tactic step
    omitted (never `sorry`), leaving an unsolved goal" is read as: the tactic lines that justify
    the hidden step — for `sqrt2_irrational`'s step 3, the four lines proving `hp` — are removed,
    and the `have` header and the `obtain` stay. Lean then reports one `unsolved goals` error at
    the `by` of the `have` line and elaborates the rest (probe `p1`). The alternative that keeps
    the parity case split and omits only the odd case (probe `p2`) also fails at line 7 but
    keeps a case split the hedged informal text has dropped; it is recorded, not built.
41. **"The step text minus its recorded justification."** The faithful step is "Hence $p$ is
    even: *if $p$ were odd, $p^2$ would be odd, because the square of an odd number is odd*.
    Write $p = 2k$." Deleting the justification mechanically leaves the connective and the
    colon behind, so the material records the bare claim ("$p$ is even. Write $p = 2k$.") and
    the justification separately, and the check is: the step equals the hedge entry, a space and
    the claim; the faithful text held the justification exactly once; the step does not.
    `check_seed` holds the material to that shape.
42. **`N/A` for an informal-only target** is the instance's `lean_catchable` (D5's "informal-only
    target: `N/A`"), set by the injector; the seed's material stays `YES-KERNEL`. For a pair both
    records carry `YES-KERNEL`, as every pair's records carry the instance's value.
43. **A composite of a cosmetic instance is a valid proof.** FLAWS.md's FLAW-0011 example is the
    routine-true hedge, which FLAW-0004 makes cosmetic, and D7 makes `proof_status` valid iff
    every record is cosmetic; so the example's artifact is `valid` and `correct: true` with two
    instances recorded, while FLAW-0011's recipe says `proof_status: invalid`. The
    implementation follows D7 (ADR-0001's consistency rule, which the manifest's self-check
    enforces). The alternatives — force `invalid` for every composite, or refuse a composite whose
    embedded instances are all cosmetic — are Al's to choose; a composite of a gap or fatal
    instance is `invalid`, as the FLAW-0002 composite shows. **Replaced by Al's ruling 2 on PR
    30 (below): D7 stands, and a composite requires an embedded instance of severity gap or
    fatal; FLAWS.md's example realizes FLAW-0004 `routine-true`.**
44. **Instance order:** the embedded instance is 1 and the composite 2, so the composite's
    inherited fields and its confirmation derive from records already made; `embedded_instances`
    names them. The records are in instance order (ADR-0001 D4).
45. **"The statement's truth witness builds"** is discharged by construction: the seed's `truth`
    is `lean-kernel` with a reference into the witness library, which the gate builds and
    audits; the producer runs no Lean for it (the artifact's own formal file fails to build when
    the embedded instance is `YES-KERNEL`). FINDING-015 records what is not checked.
46. **One embedded instance per composite** in 0.5.0, by the whole-argument-variant rule of 0.3.0
    and 0.4.0; two, nesting, FLAW-0012 and FLAW-0013 embedded are refused, and padding with a
    composite by the one-spec rule. Whether a composite may embed FLAW-0013 is a reading STATE's
    NEXT named for Al, not a decision taken here. **Replaced by Al's ruling 3 on PR 30 (below):
    a composite embeds FLAW-0001 through FLAW-0010 only.**
47. **The composite's locations:** step 0 with span `[0, len)` in the informal file; the main
    declaration at its header line in the formal file ("the main declaration at its statement
    line", D5's note), with `inserted` that line and `replaced: null`.
48. **`lean_build` of a composite pair is commanded `fail`,** because the composite record is
    `YES-KERNEL` by inheritance beside the embedded one; `lean_axioms_standard` is not commanded.
49. **FLAW-0004 in a pair manifests in both files.** FLAWS.md says the class manifests in
    informal and names the formal analogue; D3 refuses the formal target; the pair's formal file
    carries the omission site, so a pair records the instance in both files (D5's "formal: K").
50. **`embedded` is general beyond the example:** every true seed's material embeds in a
    composite (FLAW-0001, 0002, 0004, 0007, 0009, 0014), tested without Lean in `every_case`;
    the goldens are the FLAWS.md example only.
51. **`nontrivial-true` and `false`** are parsed and mapped to gap and fatal, with no material in
    0.5.0, so a spec naming them is refused for lack of material, as `unused-lemma` was in 0.4.0.

## Al's rulings on PR 30, and their application

Al ruled on 2026-10-28 (received 2026-10-29 UTC; verbatim in ADR-0011 *Amendments*, STATE
*Standing rulings* and HANDOFF Session 44's addendum; copy `owner-rulings-pr30.txt` under the
external root, 1,248 bytes, SHA-256
`8907df8358260ff8da2ce8d2b7db9b30b63516816a122b0793063983ed3be618`):

1. Readings 39, 40, 41, 42, 44, 45, 47, 48, 49, 50 and 51 are accepted as read.
2. **Reading 43 is replaced.** D7 stands. FLAW-0011 requires at least one embedded instance of
   severity gap or fatal; a composite whose embedded instances are all cosmetic is not an
   invalid proof and is refused, with a test. Under D7, FLAWS.md's FLAW-0011 example embeds a
   cosmetic instance and therefore realizes FLAW-0004 `routine-true`, which the 0.5.0 goldens
   already hold — recorded as an **erratum note against FLAWS.md 0.1.0** in ADR-0011
   *Amendments*; FLAWS.md's text is left as it is. The two FLAW-0011 goldens are replaced by a
   composite embedding one gap-or-fatal instance from a seed's existing material, informal and
   pair, from two byte-identical runs, and all 0.5.0 goldens are regenerated at the fix commit so
   that they name one `engine_commit`; the FLAW-0004 proof bytes must not change.
3. **Reading 46 is replaced.** A composite embeds instances of FLAW-0001 through FLAW-0010 only,
   by FLAW-0011's definition; FLAW-0012 and FLAW-0013 are never embedded.

**Application**, in three further commits on the branch (no force-push):

- **Commit 4, the fix** `1c0c5fcdf1ffd375b109f272461a63c49330e007`: `compose` refuses an
  embedded instance whose severity is not gap or fatal (so a composite of cosmetic or
  repairable instances is refused) and any class outside FLAW-0001 through FLAW-0010 (so
  FLAW-0011, 0012, 0013 and 0014 are never embedded); the test
  `d5_flaw_0011_is_a_commanded_composite_of_one_embedded_instance` now takes
  `continuity_is_not_uniform`'s FLAW-0002 `forall-exists` gap composite as its subject and
  refuses the cosmetic FLAW-0004 composite (FLAWS.md's example), a cosmetic FLAW-0007 `unused`
  composite, a repairable FLAW-0001 `local` composite and a FLAW-0014 composite, while the
  FLAW-0004 instance alone stays accepted; `every_case` embeds only gap-or-fatal materials of
  FLAW-0001 through FLAW-0010 (FLAW-0001 `propagated`, FLAW-0002, FLAW-0009); the two cosmetic
  composite goldens are removed; the crate README follows. The 20 property tests, clippy and fmt
  stay clean.
- **Commit 5, the regeneration** `5afc00d7809fa34a0c163e08c6d8e353d374bcae`: the four 0.5.0 goldens produced at
  `1c0c5fcd…` by two separate runs (`evidence/goldens-run5r.py`; run A
  2026-10-29T00:11:35Z–00:12:07Z, run B 00:12:07Z–00:12:38Z; every case exit 0, fidelity PASS,
  the provenance check passed against the commit before each run;
  `evidence/goldens-run5ra-summary.txt`, `evidence/goldens-run5rb-summary.txt`).
  `evidence/goldens-copy5r.py` compared the runs file by file (every `proof.md`, `Proof.lean` and
  `manifest.json` byte-identical, every observation equal, one engine commit), checked the
  FLAW-0004 cases against the goldens committed at `4edc3f7` (exported under the external root:
  `proof.md` and `Proof.lean` byte-identical, the same artifact ids `0abb0ceac4c5` and
  `a389934b677e`, the same ground truth — only `manifest.json` and `case.json` name the new
  commit), checked the composite's `proof.md` and `Proof.lean` against the 0.2.0 goldens
  `continuity_is_not_uniform/flaw-0002-gap-informal` and `-pair` (byte-identical: the composite
  changes no byte of the embedded instance's files), removed the two obsolete composite
  directories and placed run A's outputs with a `case.json` each
  (`evidence/goldens-regeneration-report.json`, SHA-256
  `5e0d258b5cb3c6cb1cab78e7646ac28e2466e36b3336575afa16f8089a3239f6`, 7,397 bytes). The
  composite's artifact ids are `21d62f8f06d1` (informal) and `dac9acd07051` (pair); its pair
  fails at line 6, the swapped `obtain` (check K); its four records are the FLAW-0002 instance
  in both files and the composite at step 0 and at the main declaration's header line (line 5),
  all `gap`, `YES-KERNEL`; the artifact is `proof_status: invalid`, `correct: false`, two
  instances confirmed. The receipts are under `goldens-5ra\` and `goldens-5rb\` of the external
  root; the sorted lists of the four `receipt.json` SHA-256s hash to
  `bbe9fdb30289e0dfffedcd899a76701e94bb5190d298ec71e2c28d67c4d66aa8` and
  `5244afe6f4fe2abc1d7609b2f5058565bd2880a21001118e44cf066d4df8e652`. The goldens README and the
  count (47 cases) follow.
- **Commit 6, the records**: ADR-0011 *Amendments*, STATE, HANDOFF Session 44's addendum and this
  section. The gate on the committed tree, the step-7 recheck and the push are in the pull
  request and under the external root.

**The choice of the gap instance.** `continuity_is_not_uniform`'s FLAW-0002 `forall-exists`
gap: a true statement (its witness module is audited by the gate) whose argument swaps the
quantifiers at step 1, a gap the kernel catches at the site in a pair, with 0.2.0 goldens
already pinning the embedded instance's bytes. The composite therefore adds exactly its own two
records and changes no proof byte, which the comparison script asserts.
