# S2 implementation packet 14 — `s2_grammar` 0.7.0 (ADR-0011, Session 53, 2026-11-02)

This packet records the fourteenth implementation session of ADR-0011 (D11): `s2_grammar` 0.7.0,
FLAW-0011 as the composite on the grammar `ring_chain` (D5, D6). The composite embeds one gap-or-fatal
instance of a class the grammar realizes, applied by that class's own injector, and records
itself at the statement. It is the last of the grammar's v0.1 classes. The packet also holds the
goldens. Nothing here is a result: no CLAIM, no fidelity sweep, no artifact under `corpora/`.

Branch `claude/s2-grammar-0.7.0`, from `main` `22aac6edc9e3c4f4fb913ba825bf744c2515724f` (PR
38's merge):

| Commit | What |
|---|---|
| `97c6cc6c4a65679e4cea62c3c0c778027b00d747` | the 0.7.0 producer: `src/grammar.rs` (`compose`, the dispatch, the version, `realizes`), `src/lib.rs`, `tests/grammar.rs`, the crate README |
| `141b9310f602edc084cf5f742a598fa893068175` | the 5 goldens of `s2_grammar` 0.7.0, produced at `97c6cc6c…`, the goldens README and the count (110 cases) |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 53 |

## What was built

- **FLAW-0011, the composite, on `ring_chain` (D5, as `s2_template` 0.5.0 reads it with Al's
  rulings 2 and 3 on PR 30).** The config's one dial is `embedded`, a list of exactly one
  FlawSpec (reading 166). There is no sub-variant, and the site is `auto`, because the composite
  addresses the statement. The severity is the embedded instance's.
  - **What may be embedded.** A class FLAW-0001 through FLAW-0010 that the grammar realizes, at
    severity gap or fatal (reading 165). On `ring_chain` that is FLAW-0001 `propagated` (fatal),
    FLAW-0003 `cycle` (fatal), FLAW-0009 `axiom` with `truth` true (gap) or false (fatal), and
    FLAW-0010 `a` with `claim: true` (gap).
  - **How it is applied.** The grammar's own injector for the embedded class applies the
    embedded spec (`inject`, `circulate`, `smuggle`, `cite`; `hedge`, whose refusals stand) as
    instance 1, unchanged: its site (explicit or drawn), its draws on its own label, its dials,
    its target rule and its refusals (reading 164). The proof files are therefore the
    standalone embedded artifact's byte for byte, and the embedded records are that artifact's
    records.
  - **The composite's own records.** The composite is instance 2 (`Composite`), with a record
    in each file the artifact has (reading 168): step 0 with span `[0, len)` in the informal
    file, and the main declaration at its header line in the formal file. Its severity and
    `lean_catchable` are the embedded instance's. Its dials are `embedded` (the config's list),
    `site` 0, `embedded_instances` `[1]`, `inserted` (the statement, or the header line) and
    `replaced` null.
  - **Writing and confirmation.** The existing `assemble` writes the records and confirms the
    composite when every record of the embedded instance passes and the statement's truth is
    kernel-checked by construction (D7: the formal control the reference names; reading 169).
  - **Ground truth and statistics.** Every composite is an invalid proof of a true statement
    (`proof_status: invalid`, `correct: false`). `n_flaws_total` is 2, one per class. Every
    other statistic, including `lean_build` and `lean_axioms_standard`, is the embedded
    artifact's (reading 170).
  - **The target** follows the embedded class's rule: a FLAW-0009 composite is formal or pair,
    a FLAW-0003 `cycle` or FLAW-0010 composite is informal or pair, and a FLAW-0001 composite
    may have any target.
- **Refusals (D3, D6).**
  - The embedded list: empty, two specs, not a list, or an element that is not a FlawSpec.
  - The embedded class: one the grammar does not realize (FLAW-0002, FLAW-0005, FLAW-0006,
    FLAW-0008), or FLAW-0011 to FLAW-0014 (ruling 3; nesting included).
  - The embedded severity: cosmetic or repairable (ruling 2: FLAW-0001 `local`, FLAW-0004
    `routine-true`, FLAW-0007 `unused`).
  - The composite's severity: anything but the embedded instance's.
  - The composite's shape: a sub-variant, an explicit site, or a missing, extra or reserved
    dial; a reserved dial in the embedded spec.
  - The target, under the embedded class's rule.
  - The embedded injector's own refusals: FLAW-0010 `claim: false`, FLAW-0004
    `nontrivial-true` and `false`, FLAW-0003 `direct`, FLAW-0009 `native_decide`, a lemma-needing
    class at `n_lemmas` 0, an instance whose every chain step is lemma-cited. FLAW-0007
    `vacuous` is refused by the composite, as `s2_template`-only.
  - Padding with a composite: two specs, or a ratio without its FLAW-0013 spec.

  Under 0.1.0 to 0.6.0 the class stays refused, so their goldens regenerate under the versions
  they name.
- **Versions (D1).** `GrammarVersion::V0_7_0` is current and `generate` emits 0.7.0;
  `Producer::parse` admits fourteen producer versions. No RNG label is new, and no draw is made
  for the composite itself: 0.7.0's rng definition is 0.6.0's (reading 171). The bytes 0.7.0
  emits for a 0.1.0 to 0.6.0 config are theirs; only the version, the id and the manifest
  differ. This is tested on one artifact of every class 0.6.0 realizes but FLAW-0013, and on the
  (4, 0) and (4, 1) controls; the 105 earlier goldens regenerate.
- **Housekeeping (reading 173).** The crate README's "not yet" line still listed FLAW-0004, which
  0.6.0 realized, among `ring_chain`'s remaining classes. The goldens README's count line read
  101 cases without Session 52's four. Both are corrected.

## Evidence that the material does what the records say

1. **No probe.** STATE's NEXT expected none, and none was needed: the composite changes no byte
   beyond its embedded instance's, whose formal shapes 0.1.0, 0.3.0, 0.4.0 and 0.5.0 fixed by
   their own probes. No Lean process ran before the goldens.
2. **The goldens.** There are 5 cases on `ring_chain` at the short length (`n_steps` 4,
   `n_lemmas` 0): the control (informal, formal), and FLAW-0011, the composite embedding one
   FLAW-0001 `propagated` instance with the `sign` perturbation (fatal; informal, formal, pair).
   - **Production.** Each case was produced by `generate --dry-run` with seed 7 at
     `97c6cc6c4a65679e4cea62c3c0c778027b00d747`, by two separate runs
     (`evidence/goldens-run14.py`; run A 2026-11-02T19:48:24Z–19:49:27Z, run B
     19:49:27Z–19:50:30Z). Every case exited 0 with fidelity PASS, and the provenance check
     passed against the commit before each run (`evidence/goldens-run14a-summary.txt`,
     `evidence/goldens-run14b-summary.txt`).
   - **Comparison and placement.** `evidence/goldens-copy14.py` compared the runs file by file:
     every `proof.md`, `Proof.lean` and `manifest.json` byte-identical, every observation equal,
     one engine commit. It checked every case's shape and placed run A's outputs under
     `engine/generate/tests/goldens/ring_chain/` (`v0.7.0-…`) with a `case.json` each. Its report
     is `evidence/goldens-production-report.json` (SHA-256
     `b9182edd7cddceb50cecd0fd2b77e931ffd84dfe6fccbebecef7188633dc8148`, 14,866 bytes).
   - **The controls.** The chain is the 0.1.0 short control's (`(x - 2 * y) ^ 2` at seed 7, four
     ring-closed steps), and both files are the 0.1.0 short control's byte for byte. The formal
     control builds and prints `[propext, Quot.sound]` (`s2_grammar-v0.7.0-cc0c6a4daf03-s7`, 119
     formal tokens). The informal control is `…-82300f31bd83-s7`: 93 tokens, `n_steps` 4,
     `n_lemmas` 0.
   - **The composites: the embedded instance.** In each composite, FLAW-0001's injector draws,
     on `injector/FLAW-0001/1`, chain step 1 as the site and the `sign` of `4 * y ^ 2` as the
     perturbation. It re-derives steps 2–4, so the chain ends at `x ^ 2 - 4 * y * x - 4 * y ^ 2`,
     away from the claim. Each composite's proof files are the 0.1.0 FLAW-0001 `propagated`
     golden's byte for byte, and its embedded records equal that golden's records, compared as
     JSON: instance 1, fatal, `YES-KERNEL`, at step 1 span `[58, 88)` and at line 4.
   - **The composites: the composite's records.** The composite's records are instance 2,
     fatal, `YES-KERNEL` by inheritance. The informal record is at step 0, span `[0, 62)`:
     `For all integers $x$ and $y$, $(x - 2y)^2 = x^2 - 4yx + 4y^2$.`. The formal record is at
     the header line, `syndeom_s2_grammar_ring_chain:3`. Both carry `embedded` (the config's
     list), `site` 0, `embedded_instances` `[1]` and `replaced` null.
   - **The composites: ground truth and the build.** `n_flaws_total` 2, `n_flaws_0001` 1,
     `n_flaws_0011` 1, `flaws_confirmed` 2; `proof_status: invalid`, `correct: false`. Each case
     has 93 informal tokens, or 119 formal tokens for the formal-only case, the embedded
     artifact's. The formal and pair builds exit 1 with the first error at line 4, the embedded
     site's `calc` line (check K). `lean_build` is commanded and achieved `fail`, with no axiom
     print.
   - **Every case is a true statement** whose reference is the 0.7.0 formal control at this
     length (`s2_grammar-v0.7.0-cc0c6a4daf03-s7:syndeom_s2_grammar_ring_chain`), which names
     itself.
   - **The receipts**, with the build and audit stdout and stderr of the three formal cases, are
     under the external root `C:\Users\LEET\Codex\Temp\syndeom-s2-grammar-070-20261102T1950Z\`
     (`goldens-14a\` and `goldens-14b\`). Their sorted `receipt.json` SHA-256 lists (one per
     line, LF-terminated) hash to
     `57c2b0181f1100cf572e13ab8ab5bbdee0d3aacefe1ea24ec5351b3b908133be` (A) and
     `f591a2da56ee69dae3374a5cc4697d0ae578a788c709018abde8f9076599913c` (B). The receipts differ
     only in their timings. In run A, the formal control's build took 6,781 ms and its audit
     6,604 ms, and the composite's two failing builds took 6,581 ms and 6,580 ms.
3. **The artifact ids** (`s2_grammar-v0.7.0-<config_hash[0:12]>-s7`):
   - control: `82300f31bd83` (informal), `cc0c6a4daf03` (formal);
   - FLAW-0011 composite of FLAW-0001 `propagated` `sign`: `9dae22d9ec2e` (informal),
     `3cbe4712121a` (formal), `f2f709a41238` (pair).
4. **Tests and lint.** The crate's 53 tests pass: goldens 1 (over 110 cases), grammar 18,
   properties 24, provenance 2, rng 5, writers 3. `cargo clippy --all-targets --all-features --
   -D warnings` and `cargo fmt --check` are clean.
   - **The new test** in `tests/grammar.rs` covers FLAW-0011 over ten embedded specs and every
     target their classes allow: FLAW-0001 `propagated` at (4, 0), (6, 2), (24, 2) and (1, 0)
     with each perturbation kind; FLAW-0003 `cycle` at (4, 1) and (12, 5); FLAW-0009 `axiom`
     with each truth; FLAW-0010 `a` at (4, 0) and (8, 1). It checks:
     - the plan, the bytes and the embedded instance equal the standalone artifact's;
     - the `Composite`, and the records (the standalone ones, then the composite's at the
       statement in each file, with their locations and dials);
     - the ground truth: invalid, a true statement, the reference the formal control;
     - the commanded and achieved statistics: two instances, the rest the standalone
       artifact's;
     - the confirmation: a composite naming no recorded instance is unconfirmed while the
       embedded one stands, and an observation failing the embedded check leaves both
       unconfirmed — a FAIL verdict, not a refusal;
     - explicit embedded sites at every ring-closed step of a (6, 2) instance, and determinism;
     - thirty-seven refusals, 0.1.0 to 0.6.0 refusing the class, and the 0.6.0 bytes of eight
       artifacts unchanged.

     The version facts move to 0.7.0, with the existence assertion moved to `"0.8.0"` before the
     first run, as Sessions 49–52 learned to do.
   - **Mutation checks** (`evidence/mutations/`). Each of two mutations of `compose` fails the
     new test, and the source is restored byte for byte (SHA-256 `196d57e3…`). Shifting the
     composite's instance number to 3 fails the `Composite` assertion; removing the
     gap-or-fatal check fails "FLAW-0004 routine-true is cosmetic". The new test passed on its
     first run, and the mutations show that it fails when the behavior it pins changes.
   - The gate on the committed tree is recorded in the pull request and under the external
     root.

## `cargo audit` and `cargo deny check` (D10)

- `cargo audit`: `cargo-audit 0.22.2` at 2026-11-02T19:52:08Z, outside the gate (`cargo audit
  --file engine/Cargo.lock`). It loaded 1,402 advisories (advisory-db commit
  `263a4d951a0867d4bddc29dddd6d59427c78f9a7`, Session 52's), scanned 35 crate dependencies, and
  reported no vulnerability (exit 0; `evidence/cargo-audit.txt`). `engine/Cargo.lock` is
  unchanged by this revision: no dependency was added.
- `cargo deny check`: `cargo-deny 0.20.2` at 2026-11-02T19:52:11Z, with `engine/deny.toml` as
  committed: **advisories ok, bans ok, licenses ok, sources ok** (exit 0;
  `evidence/cargo-deny-check.txt`). It runs where `cargo audit` runs, never in the offline gate.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `97c6cc6c…`:
- `engine/generate/src/grammar.rs`: `compose`, the FLAW-0011 arm of `prepare_with`'s dispatch,
  `GrammarVersion::V0_7_0`, `GENERATOR_VERSION`, `realizes`, the module docs;
- `src/lib.rs` (docs) and `tests/grammar.rs`.

The threat model is packet 1's. The crate is an offline research CLI. Its assets are the
integrity of generated ground truth and the operator's machine. Its trust boundary is an artifact
directory handed to `verify` and a config handed to `generate`. New in this revision:
- a nested FlawSpec (FLAW-0011's `embedded`) parsed from the config;
- a class-number pre-filter (`strip_prefix("FLAW-")`, then `parse::<u32>`);
- a dispatch to existing injectors;
- a composite record whose dials carry the config's embedded list.

**Pre-scan (Phase 1):**
- `cargo audit` and `cargo deny check` are clean (above), and no external dependency was added.
- `#![forbid(unsafe_code)]` stands in `lib.rs` and `main.rs`. No other `unsafe` token exists in
  the crate's sources.
- clippy at `-D warnings` is clean. `clippy::pedantic` was read as information, not the gate's
  level: of its warnings on the library, two fall on lines this diff added, both `doc_markdown`
  (the word FlawSpec unbackticked, as the existing docs write it). None concerns behavior; the
  `must_use_candidate` on `realizes` predates the diff.
- `cargo-geiger` is not adopted (Al's ruling on PR 37). Miri is not available for the pinned
  toolchain.

**Panic and index sweep of the diff (2C).** The edited sources outside tests have no `unwrap`,
`expect`, `panic!` or `unreachable!`. The indexes:
- `spec.dials["embedded"]` (a `serde_json::Map` index, which panics on a missing key) sits behind
  `exact_keys` on the dials, at both of its uses;
- `embedded[0]` sits behind the empty-list and longer-list refusals;
- `inj.instance + 1` takes instance 1 from the injector.

The dispatch's arm for an unexpected class returns a refusal, not an `unreachable!`.

**Serde and the config (2F).**
- The embedded spec goes through the existing `parse_spec`: exact keys, reserved dials refused,
  `site` `auto` or at least 1, the severity through its enum.
- A list longer than one is refused before its second element is parsed. Nesting is refused by
  the class check, and `compose` never recurses; `serde_json`'s own recursion limit bounds the
  config's parse.
- The class pre-filter accepts forms like `FLAW-+1` or `FLAW-001` as 1, because u32's `FromStr`
  allows a sign and leading zeros. `realizes` and each injector's `check_spec` compare the exact
  class strings, so such a class is refused there. The probe `evidence/probe-class-forms-2.py`
  ran `generate --dry-run` at `141b9310…` on six non-canonical forms (`FLAW-+1`, `FLAW-1`,
  `FLAW-001`, `FLAW-00001`, `flaw-0001`, `FLAW-0001 `), each refused with exit 2. The control
  and the canonical composite exit 0, the latter with the informal golden's id
  (`evidence/probe-class-forms-2.txt`). The pre-filter cannot widen what is realized.
- The config's `embedded` list enters the manifest's composite records as the spec's dials, as
  the template's has since 0.5.0 (the whole config is in the manifest already). It never enters
  the proof text.

**Integer (2I):** `inj.instance + 1` only.

**Ground truth (logic, SD1):**
- A composite cannot be recorded valid or correct. Its embedded instance is gap or fatal (else
  refused), and D7 derives `invalid`.
- Its severity equals the embedded instance's (else refused).
- It is confirmed only when every embedded record passes (`assemble`, unchanged). This is tested
  with an orphaned composite and with failing observations.
- The earlier versions refuse the class (tested).

**Subprocesses and `verify`:** unchanged. `verify` compiles only bytes the producer the manifest
names itself emits for the manifest's `(config, seed)`, and a composite's bytes are its embedded
artifact's. The audit tail names only the producer's main declaration (FINDING-011's guard holds).

**Determinism:** `BTreeMap` and `Vec` only; no new label or draw; no floating-point value enters a
generation decision.

No new finding. **FINDING-018** (informational, backlog) does not gain scope: `compose` indexes
nothing of the derivation and passes it to injectors already within the finding. FINDING-015 does
not extend to the grammar (packet 8). FINDING-016 and FINDING-017 (informational, backlog) stand.
FINDING-006 (accepted risk), FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012 and
FINDING-013 (accepted) stand as packets 1–13 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 13's numbering.

164. **The composite on the grammar is `s2_template` 0.5.0's composite, with no new material.**
     STATE's NEXT reads D5 as "the embedded spec applied by the grammar's own injector for its
     class (its site, draws and dials as that class records them)". The implementation calls
     the same function a standalone spec of that class reaches. The embedded instance's site,
     its draws on its own label (`injector/FLAW-000N/1`), its dials, its target rule and its
     refusals are therefore that class's. The proof files and the embedded records are the
     standalone artifact's byte for byte: tested over ten embedded specs, and the goldens equal
     the 0.1.0 FLAW-0001 `propagated` goldens. The composite adds only its records.
165. **The grammar composes four embedded specs.** These are the ones at gap or fatal with
     material on `ring_chain`:
     - FLAW-0001 `propagated`, fatal, any perturbation;
     - FLAW-0003 `cycle`, fatal, `n_lemmas` at least 1;
     - FLAW-0009 `axiom` with `truth` true (gap) or false (fatal), `n_lemmas` at least 1;
     - FLAW-0010 `a` with `claim: true`, gap.

     Refused:
     - FLAW-0001 `local`, repairable: ruling 2 as 0.5.0's amendment reads it at one instance;
     - FLAW-0004 `routine-true` and FLAW-0007 `unused`, cosmetic;
     - FLAW-0004 `nontrivial-true` and `false`, and FLAW-0010 `claim: false`: no material, refused
       by the class's injector;
     - FLAW-0007 `vacuous`: `s2_template`-only, refused by the composite with that reason;
     - FLAW-0003 `direct` and FLAW-0009 `native_decide`: refused by their injectors;
     - the classes the grammar does not realize;
     - FLAW-0011 to FLAW-0014 (ruling 3).
166. **The one-spec rule.** `s2_grammar` takes one spec per artifact, so a composite's
     `embedded` list holds exactly one spec. The rule is the grammar's own; the template gives a
     different reason, that its materials are whole-argument variants. An empty list and a list
     of two are refused. Padding with a composite is refused too: two specs, or a ratio without
     its FLAW-0013 spec. A composite of several instances, or a padded one, is a later version
     if S3's cell design asks for it.
167. **The checks run in the template's order, before the embedded injector runs.**
     1. The composite's own shape: no sub-variant, site `auto`, the one dial.
     2. The list.
     3. The embedded spec's parse: exact keys, no reserved dial.
     4. Its class number in 1–10.
     5. Its severity gap or fatal.
     6. The version realizing its class.
     7. The severity equality.

     The class-number pre-filter accepts non-canonical forms, and the exact-string `realizes`
     refuses them (the probe above).
168. **The composite's `lean_catchable` is inherited in both of its records.** This includes the
     formal record of a FLAW-0003 `cycle` or FLAW-0010 composite pair, whose embedded instance
     has no formal record (`manifests_in`). That composite's formal record is `NO` or `N/A`, at
     the header line of a formal file whose bytes are the control's. The rule "a record in each
     file the artifact has" is 0.5.0's (reading 45, accepted), and the template's own FLAW-0010
     and FLAW-0003 `cycle` composites have recorded this way since 0.6.0. Recorded as a
     consistency fact for S3's cell design: such a formal record marks a file whose bytes carry
     no flaw.
169. **The statement's truth witness is the formal control** (D7 for a grammar instance). The
     composite's confirmation reads the truth's method (`lean-kernel`) and its non-empty
     reference, the formal control of the same version, length and seed. It does so as the
     template reads its witness library, without running Lean for it. The 0.7.0 goldens include
     that control, whose recorded build passes with the standard axioms. "The statement's truth
     witness builds" is discharged by construction, as reading 45 discharged it for the
     template.
170. **`lean_build`, `lean_axioms_standard` and the counts.** `assemble`'s rules are unchanged.
     - `lean_build` is `fail` iff some formal record is `YES-KERNEL`. The composite inherits
       `YES-KERNEL` exactly when its embedded instance is, so the command is the embedded
       artifact's.
     - `lean_axioms_standard` is `false` iff a FLAW-0009 record exists.
     - The counts: `n_flaws_total` 2 (per instance), `n_flaws_0011` 1 and the embedded class's
       count 1. `n_steps`, `n_lemmas` and `n_tokens` are the embedded artifact's.
171. **The version bumps although no proof byte changes.** 0.7.0 emits bytes for configs that
     0.6.0 refused, so the version bumps (D1). No proof byte of any earlier version changes and
     no RNG label is new: 0.7.0's rng definition is 0.6.0's.
172. **A composite cell and its standalone cell share their bytes.** Every grammar artifact with
     one gap-or-fatal instance is, in substance, an invalid proof of a true statement:
     `statement_status` true by construction, `proof_status` invalid. FLAW-0011 records that
     only when commanded, as D5's row says ("recorded only when commanded, so the class's cells
     are explicit"). So a FLAW-0011 cell on the grammar holds byte for byte the artifacts of its
     embedded class's cell, and differs only in the labels. A detector that reads the files
     scores the two cells identically. S3's cell design should know this before it pairs or
     compares them. Noted, as reading 162 noted FLAW-0004's visible omission.
173. **Housekeeping.** Two documentation lines lagged the code:
     - the crate README's "not yet" line still named FLAW-0004 among `ring_chain`'s remaining
       v0.1 classes after 0.6.0 realized it;
     - the goldens README's count line still read 101 cases, without Session 52's four.

     Both are corrected here. The goldens test pinned the counts correctly throughout, so nothing
     regenerated.
