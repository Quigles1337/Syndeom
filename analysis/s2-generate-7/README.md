# S2 implementation packet 7 — `s2_template` 0.7.0 (ADR-0011, Session 46, 2026-10-29)

This packet records the seventh implementation session of ADR-0011 (D11): `s2_template` 0.7.0 —
FLAW-0008 (D5: the formal statement drifts by exactly one recorded edit for each of the five
transformations while the informal file stays faithful) on the new seed `bolzano_weierstrass`,
FLAWS.md's FLAW-0008 example, and FLAW-0012 (D5: a false statement's convincing argument, the
true-case proof transplanted and failing at the hidden use site, which carries a FLAW-0005
`violated` record) on the new seed `bounded_continuous_sup`, FLAWS.md's FLAW-0012 example, each
with its witness module and its goldens. Every FLAWS.md 0.1.0 example now has its seed. Nothing
here is a result: no CLAIM, no fidelity sweep, no artifact under `corpora/`.

Branch `claude/s2-template-0.7.0`, from `main` `89cdcf45baaf13834a8f178e0614b8b9b8420937` (PR
31's merge):

| Commit | What |
|---|---|
| `6dba3615a001151116632169bd93b25440878ac0` | the 0.7.0 producer: `Drift` and `HiddenUse` material, `check_drift` (one rule per transformation), the seed shape rules, the injector's statement path and `drop_hypothesis`, the two records and their checks, `plan::header_line`, the two seeds and their witness modules, the tests, the crate README |
| `624c22f68a39178dabad9a9c01b1e647059e0a24` | the 10 goldens of 0.7.0, produced at `6dba3615…`, the goldens README and the count (69 cases) |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 46 |

## What was built

- **FLAW-0008 (D5).** A seed's FLAW-0008 material is replacement material of the *statement*:
  `site` 0, no anchor, `NO`, severity gap or fatal as the seed marks it, with the one dial
  `transformation` (one of D5's five). Its `drift` names the drifted binders and claim, which
  replace the faithful ones in the formal file, and the one edit that carries the drift —
  `replaced`, the faithful text, and `inserted`, what stands in its place; its steps are the
  drifted statement's own correct proof, every informal text, lemma header and premise list
  faithful. The target is `pair` only (D3; FLAWS.md: the class manifests in the pair). The
  injector replaces the plan's binders and claim and its steps; the informal file is the
  control's, byte for byte, and the formal file states the drifted statement, which builds, so
  `lean_build` is commanded `pass` and `lean_axioms_standard` `true`; the pair is `invalid`
  (D7). The one record is formal-only (`manifests_in`), at the main declaration's header line,
  with dials `transformation`, `site: 0`, `inserted` and `replaced` (the drifted and the
  faithful header lines, as `plan::header_line` emits them), `faithful_statement` and
  `drifted_statement` (binders and claim each), `edit` (`replaced`, `inserted`),
  `original_fragments` (the faithful fragments the drifted proof changed) and `declaration`.
- **The FLAW-0008 check (D5, S2.2).** A FLAW-0008 record is confirmed when the file builds
  (exit 0); the faithful statement has its kernel-checked witness (`truth` is `lean-kernel` or
  `mathlib` with a reference — the seed's module, which the gate builds and audits); the
  recorded faithful and drifted statements differ by exactly the transformation's edit under
  `check_drift`; the plan states the drifted statement; and the file's header line is the
  recorded `inserted` line. `check_drift` is one textual rule per transformation —
  `add-hypothesis`: one binder `(<name> : <type>)` appended after the replaced binder text,
  the claim unchanged; `restrict-type`: every occurrence of one whitespace-free type token
  replaced by another, in the binders and the claim; `flip-strictness`: one occurrence replaced
  by the same text with exactly one relation symbol flipped between `≤` and `<` (or `≥` and
  `>`); `weaken-conclusion`: one occurrence inside the claim replaced, the binders unchanged;
  `shadow-definition`: one occurrence inside the claim replaced by `let <name> := <definition>;
  <body>` whose body uses the name, the binders unchanged — and every transformation must
  change the statement. `check_seed` holds the material to the same rule.
- **FLAW-0012 (D5).** A seed's FLAW-0012 material is intrinsic (the convincing argument of a
  false statement, as the example writes it), fatal, `YES-KERNEL`, without a sub-variant, with
  the one dial `dropped_hypothesis` and a `use` naming the theorem applied at the hidden use
  site (`site`) and the hypothesis the dropped one supplied. Commanding FLAW-0012
  (`drop_hypothesis`, on the stream `injector/FLAW-0012/1`) writes two instances: the FLAW-0005
  `violated` instance at the use site (instance 1; a record in each file the artifact has, with
  dials `theorem`, `hypothesis`, `mode: violated`, `site`, the site's text as written,
  `replaced: null` and the counterexample — 0.2.0's intrinsic shape) and the FLAW-0012 instance
  at the statement (instance 2; step 0 with span `[0, len)` in the informal file, the main
  declaration at its header line in the formal file, with dials `dropped_hypothesis`, `site:
  0`, `use_site`, `use_instance`, `counterexample`, `inserted` and `replaced: null`). The seed
  has no control and no FLAW-0005 material of its own, so the use-site instance exists only
  through FLAW-0012; a composite never embeds the class (Al's ruling 3 on PR 30); `lean_build`
  is commanded `fail`; counts are per instance (`n_flaws_total` 2, one per class).
- **The FLAW-0012 check (D5, S2.2).** A FLAW-0012 record is confirmed when every record of the
  use-site instance passes (check K in the formal file; the span check in the informal file)
  and the seed's negation witness is kernel-checked (`truth` is `value: false`, `lean-kernel`,
  with a reference — the module the gate builds and audits).
- **The seeds (D6).** `bolzano_weierstrass`: every bounded sequence of reals has a convergent
  subsequence — FLAWS.md 0.1.0's FLAW-0008 example, outside the S1 corpus (ruling 1 of
  2026-10-27) — in three aligned steps on the compact interval `[-M, M]`
  (`Metric.isBounded_Icc`, `tendsto_subseq_of_bounded`), with the witness module
  `lean/Syndeom/Seeds/BolzanoWeierstrass.lean` and five materials: `add-hypothesis` (`(hm :
  Monotone a)` appended; the proof via the supremum, `tendsto_atTop_ciSup`; gap),
  `restrict-type` (ℝ → ℤ throughout; the proof text unchanged; gap), `flip-strictness` (`|a n|
  ≤ M` → `|a n| < M`; gap), `weaken-conclusion` (`StrictMono φ ∧` dropped; the constant
  subsequence; fatal) and `shadow-definition` (`Convergent` redefined by a `let` in the claim
  as "comes within ε of L for some n"; fatal). `bounded_continuous_sup`: every bounded
  continuous `f : ℝ → ℝ` attains its supremum — FLAWS.md 0.1.0's FLAW-0012 example — false by
  `arctan`, the extreme-value argument of a compact interval transplanted in four aligned steps
  (the supremum, a maximizing sequence by `exists_seq_tendsto_sSup`, the Bolzano–Weierstrass
  step, the continuity step), the hidden use site step 3, with the witness module
  `lean/Syndeom/Seeds/BoundedContinuousSup.lean` proving the negation. `lean/Syndeom.lean`
  imports both; `lean/Audit.lean` now audits fourteen declarations.
- **Versions (D1).** `Version::V0_7_0` is current; `generate` emits 0.7.0; FLAW-0008 and
  FLAW-0012 are refused by every earlier version, and the new seeds are seeds of none of them;
  the 59 earlier goldens regenerate under the versions they name, with no golden rewritten.

## Evidence that the Lean material does what the records say

1. **Probes before the material** (`evidence/lean-probe/`, `lake env lean` from `lean/`, one
   process at a time, `run.sh`; 2026-10-29T02:00:23Z–02:02:45Z, 10–28 s each):

   | Probe | Exit | What |
   |---|---|---|
   | `s2j_bw_faithful` | 0 | the faithful argument: `[propext, Classical.choice, Quot.sound]` |
   | `s2j_bw_add_hypothesis` | 0 | `(hm : Monotone a)` appended; the proof via the supremum: standard axioms |
   | `s2j_bw_restrict_type` | 0 | ℝ → ℤ with the faithful proof text unchanged (`Metric.isBounded_Icc` and `tendsto_subseq_of_bounded` hold for ℤ): standard axioms |
   | `s2j_bw_restrict_type_b` | 0 | the same statement proved through `Set.finite_Icc` — compiled as a fallback, not used |
   | `s2j_bw_flip_strictness` | 0 | `|a n| < M`; the bound weakened with `.le`: standard axioms |
   | `s2j_bw_weaken_conclusion` | 0 | `StrictMono φ ∧` dropped; the constant subsequence: standard axioms; Lean's linter warns that `hb` is not referenced |
   | `s2j_bw_shadow_definition` | 0 | the `let` in the claim; `intro Convergent` and the constructor: standard axioms; the same linter warning |
   | `s2j_bw_shadow_definition_b` | 0 | the same with `simp only [Convergent]` first — compiled as a fallback, not used |
   | `s2j_bw_witness` | 0 | the witness module's content: standard axioms |
   | `s2j_bcs_convincing` | 1 | the convincing argument: one error, `11:126: error: Application type mismatch` (`hM (x n)` has type `|f (x n)| ≤ M`, expected `|x n| ≤ M`), at the `have hxb` line, the first fragment line of step 3; no cascade |
   | `s2j_bcs_rest` | 0 | the same with `hxb` admitted by `sorry`: every other line elaborates (a control, never emitted) |
   | `s2j_bcs_witness` | 0 | the negation witness by `arctan`: standard axioms |

   The seed records and the witness modules were written after these probes compiled. The
   library then built with the two modules (`lake build`, 8,938 jobs) and the audit printed
   fourteen axiom lines, every one within the standard three (`lake-build-1.log` under the
   external root, SHA-256 `b5643eb292b59602e2cc7c5517ef216efd4e8ac1646d599ce00824de5feffb2f`).
2. **The goldens.** 10 cases — on `bolzano_weierstrass`, the control (informal, formal) and
   FLAW-0008 `add-hypothesis`, `restrict-type`, `flip-strictness`, `weaken-conclusion` and
   `shadow-definition` (pair each); on `bounded_continuous_sup`, FLAW-0012 (informal, formal,
   pair) — each produced by `generate --dry-run` with seed 7 at
   `6dba3615a001151116632169bd93b25440878ac0` by two separate runs (`evidence/goldens-run7.py`;
   run A 2026-10-29T02:26:37Z–02:29:25Z, run C 02:32:45Z–02:35:35Z; every case exit 0, fidelity PASS,
   the provenance check passed against the commit before each run;
   `evidence/goldens-run7a-summary.txt`, `evidence/goldens-run7c-summary.txt`). A run B, started
   right after run A, was **refused from its third case on** (exit 2, `engine_commit does not
   identify the checked-out producer input engine/generate/tests/goldens.rs`;
   `evidence/goldens-run7b-summary.txt`): the executor had edited `tests/goldens.rs`, a file of
   the producer's input set, while the run was in progress, so the checkout no longer matched
   the engine commit and the provenance check (D9; FINDING-005 closed) stopped the run, as it
   must. Nothing of run B was used; the edit was stashed, the input set confirmed equal to the
   commit, and run C made in its place.
   `evidence/goldens-copy7.py` compared runs A and C file by file — every `proof.md`,
   `Proof.lean` and `manifest.json` byte-identical, every observation equal, one engine commit
   — compared each formal file with its probe (the probe's `#print axioms` tail dropped) and
   placed run A's outputs under `engine/generate/tests/goldens/` with a `case.json` each
   (`evidence/goldens-production-report.json`, SHA-256
   `be32cd939c5bc6be4d13c2e9723956936e22828dad725d7fcbd41985336ed796`, 27,110 bytes):
   - every one of the eight formal files equals its probe: the control
     (`s2j_bw_faithful`), the five drifted pairs (their probes), and the FLAW-0012 formal and
     pair files (`s2j_bcs_convincing`);
   - the control and the five drifted pairs build (exit 0) with the standard three axioms;
     every FLAW-0008 record is formal-only at line 6 (the main declaration's header line), its
     pair `proof_status: invalid`, `correct: false`, of a true statement, `lean_build`
     commanded `pass`, and every FLAW-0008 pair's `proof.md` is the control's;
   - the FLAW-0012 formal and pair cases fail to build (exit 1) with the first error at line
     11, the first fragment line of step 3 (lines 11–12; check K); `lean_axioms_standard` is
     unmeasured (`null`, with its reason); the three FLAW-0012 cases carry two instances —
     FLAW-0005 `violated` at step 3 / line 11 and FLAW-0012 at step 0 / line 5 — with
     `statement_status.value: false`, `proof_status: invalid`, `correct: false`;
   - every instance is confirmed (`flaws_confirmed` equals the instance count: 1, or 2 for
     FLAW-0012).
   - The receipts, with build stdout and stderr, are under the external root
     `C:\Users\LEET\Codex\Temp\syndeom-s2-template-070-20261029T0220Z\goldens-7a\` and
     `goldens-7c\` (the refused run under `goldens-7b\`); the sorted lists of the ten
     `receipt.json` SHA-256s hash to
     `3ae2f56a2e1a036d1a1fea6ed8e90f11cc289669b40777537d63318b776df221` (A) and
     `3e57f42f3fd30d08f6c00e1bd26117ec31ce60ffb1a6fa7c6edda272ddee23e5` (C).
3. **The artifact ids** (`s2_template-v0.7.0-<config_hash[0:12]>-s7`): `bolzano_weierstrass`
   control `a67602abb5dd` / `161fc51ffb45` (informal / formal); FLAW-0008 `add-hypothesis`
   `c607b52c1eac`, `restrict-type` `9dedc138a9e0`, `flip-strictness` `4be2b87571b6`,
   `weaken-conclusion` `e4c274e9a544`, `shadow-definition` `1f0d447ea8bb` (pair each);
   `bounded_continuous_sup` FLAW-0012 `35b526a2cabe` / `6f6417a96546` / `4a45be1bb211`
   (informal / formal / pair).
4. **Tests and lint.** The crate's 35 tests pass (goldens 1 over 69 cases, properties 24,
   provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features -- -D warnings`
   and `cargo fmt --check` are clean. New:
   `d5_flaw_0008_drifts_the_formal_statement_by_one_recorded_edit` (the add-hypothesis pair's
   files, header, dials, record and location; the faithful informal file; confirmation failing
   on a failed build, a tampered edit and a tampered statement; every transformation's header,
   line count and verdict; the restrict-type proof unchanged; a composite of a drift; the five
   rules of `check_drift` with their refusals; twelve config refusals; the earlier versions;
   the seed's shape rules) and
   `d5_flaw_0012_transplants_the_true_case_proof_and_records_the_hidden_use_site` (the
   convincing argument's files and lines, the use-site record's dials, the four records with
   their locations, the counts and the commanded build, K anywhere inside the use site's lines
   and its failures, the informal-only and formal-only targets, the draw, an explicit use site,
   thirteen refusals, the earlier versions, the seed's shape rules, `use` on FLAW-0012 only);
   `d6` checks both materials' shape rules; `every_case` names FLAW-0008's site `"auto"`; `d7`
   admits the two-instance false-statement artifact; `d10_locations` admits the two new
   statement records beside the composite's; `d1` closes the earlier seed sets and versions.
   The gate on the committed tree is recorded in the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-10-29T02:18:10Z, outside the gate. It loaded 1,381
advisories (advisory-db commit `40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`), scanned 35 crate
dependencies, and reported no vulnerability (exit 0; `evidence/cargo-audit.txt`).
`engine/Cargo.lock` is unchanged by this revision.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `6dba3615…` — `engine/generate/src/plan.rs` (`header_line`),
`src/template.rs` (`Drift`, `HiddenUse`, `TRANSFORMATIONS`, `check_drift`,
`check_drift_material`, `check_dropped_hypothesis_material`, the statement path of `inject`,
`drop_hypothesis`, `FalseTheorem`, the two records and their checks in `assemble`),
`seeds/bolzano_weierstrass.json` and `seeds/bounded_continuous_sup.json`. The threat model is
packet 1's: an offline research CLI whose assets are the integrity of generated ground truth and
the operator's machine, and whose trust boundary is an artifact directory handed to `verify` and
a config handed to `generate`. New this revision: a config dial (`transformation`) selects
among a seed's statement-level materials, a free-text dial (`dropped_hypothesis`) is matched
against a seed's material and copied into a record, and the producer compares statement texts
under per-transformation rules.

**Pre-scan:** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate roots and
in the manifest crate; no external dependency was added; clippy at `-D warnings` is clean.

**Panic and index sweep of the diff:** no `unwrap`, `expect` or `panic!` was added. The
indexes: `spec.dials["dropped_hypothesis"]`, a map index, sits behind `exact_keys` (the pattern
packets 4–6 recorded); `d[k]` on a `serde_json::Value` yields `Null` for an absent key, and the
record check treats a missing or malformed dial as unconfirmed, never as a panic;
`style.steps.get(site.wrapping_sub(1))` and `(m.site >= 1).then(...)` guard the use site;
`eligible[stream.below(len)]` is below the length of a non-empty list; `text.lines().nth(main_line
- 1)` is behind the writer's own count; `st.steps[0]` is the statement every parsed informal file
has (the composite's precedent); `check_drift` slices nothing by byte offset (it uses
`strip_prefix`, `split_once`, `replacen` and `chars`). `premise_cycle` is untouched
(FINDING-016).

**Serde:** `Drift` and `HiddenUse` carry `deny_unknown_fields`; `Material.drift` and
`Material.hidden_use` (JSON `use`) default to absent, so every earlier seed and every golden
`case.json` still parses. A material at site 0 is admitted only when it carries `drift`.

**Subprocesses and `verify`:** unchanged. No config value reaches the proof text: the drifted
statement, its proof and the convincing argument are compiled-in seed material; the config's
`transformation` and `dropped_hypothesis` select material and are copied into dials only.
`verify` still compiles only bytes the producer itself emits and names only the producer's main
declaration (FINDING-011).

```
FINDING-017: the FLAW-0008 drift rules are textual, not token-aware
Severity:    Informational
Category:    Logic
Location:    engine/generate/src/template.rs (check_drift)
Status:      Open (backlog)
Description: `restrict-type` replaces every occurrence of the type token, so a token that also
             occurred inside another identifier would be rewritten with it; `shadow-definition`
             splits the local definition at the first `; `, so a definition that itself held
             `; ` would be misread; `flip-strictness` compares the two texts character by
             character. In 0.7.0 the only materials are the seed's, held to the same rule at
             `check_seed`, and no config text enters a statement, so no input can exploit the
             looseness; the rule is a check on authored material, not a parser.
Recommendation: when a grammar generates drifted statements, replace the textual rules by
             edits over a tokenized statement, and refuse a token that is a substring of
             another token of the statement.
```

FINDING-015 (informational, backlog: a composite's truth witness is taken on the seed's word)
stands and now also covers the two new witness references — `BolzanoWeierstrass.lean`, which
the FLAW-0008 check relies on for "the faithful statement builds", and
`BoundedContinuousSup.lean`, which the FLAW-0012 check relies on for the negation — both built
and audited by the gate. FINDING-016 (informational, backlog) stands. FINDING-006 (accepted
risk), FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012 and FINDING-013 (accepted)
stand as packets 1–6 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 6's numbering.

66. **FLAW-0008's material is a replacement of the statement.** D5's recipe transforms the
    faithful formalization's statement, so the material sits at `site` 0 (the statement, as
    FLAW-0011's and FLAW-0012's records do) and its record is formal-only (FLAWS.md: manifests
    in formal — the pair), at the main declaration's header line (D5's note: the location is
    the changed text; for the formal file, the main declaration at the fragment's first line —
    the statement's line).
67. **The drift is one edit under one rule per transformation.** D5's check — "the drifted
    statement text differs from the faithful one by exactly the recorded transformation (both
    texts in `dials`)" — is read as a textual rule, `check_drift`, applied to the seed's
    material at `check_seed` and to the record at `assemble` alike; the record carries both
    statements (binders and claim each) and the edit (`replaced`, `inserted`), so the check
    can be re-run from the manifest alone.
68. **FLAWS.md's example applies two transformations at once** — an added hypothesis
    (monotone) and a changed conclusion (the sequence itself converges) — where D5 applies
    exactly one. The changed conclusion is false without the added hypothesis, so it cannot
    stand alone as a drift. STATE's NEXT asked that the example be realized as the seed's
    `add-hypothesis` and `weaken-conclusion` materials and the reading flagged: the example's
    hypothesis is the `add-hypothesis` material, whose proof takes the example's route (the
    supremum, `tendsto_atTop_ciSup`, the identity subsequence); `weaken-conclusion` is the
    executor's own — `StrictMono φ ∧` dropped, so the constant subsequence proves it — because
    no weakening of the example's conclusion is both provable and distinct from the faithful
    statement.
69. **`restrict-type` as ℝ → ℤ.** FLAWS.md says "a restricted type (ℝ to ℕ)"; ℕ has no
    `|·|`, so the restriction is to ℤ, applied to every occurrence (the sequence's values, the
    bound and the limit). The integer statement is a special case of the real one (integer
    sequences are real sequences and their convergence agrees), so the severity is gap. The
    faithful proof text proves it unchanged, which makes this drift a pure statement change.
70. **`flip-strictness` flips the hypothesis's bound** (`|a n| ≤ M` → `|a n| < M`): a stronger
    hypothesis, a strictly weaker statement, gap. The claim has no relation symbol to flip.
71. **`shadow-definition` is a `let` inside the claim.** "A locally redefined term" is read as
    a definition local to the statement: `Convergent` redefined as "comes within ε of L for
    some n" and the claim restated with it. The alternative — a `def` declared before the
    theorem — is not expressible under D2, which renders every declaration from a step both
    files share, while FLAW-0008 leaves the informal file faithful. The rule requires `let
    <name> := <definition>; <body>` with the body using the name.
72. **Severities by the seed's marking:** gap for `add-hypothesis`, `restrict-type` and
    `flip-strictness` (strictly weaker, the claim still plausible), fatal for
    `weaken-conclusion` and `shadow-definition` (trivialized) — FLAWS.md's range.
73. **The trivialized drifts leave `hb` unreferenced,** and Lean's `unusedVariables` linter
    warns (exit 0; the kernel accepts the statement it is given). The class stays `NO`: a
    linter warning is not a kernel rejection and not what this program's check T reads (the
    audit tail's axioms and used-constants). Binding `hb` as `_hb` would be a second edit to
    the statement; using it artificially would misrepresent the proof. Flagged: whether a
    later version should record the warning's presence as an observation.
74. **"The faithful and the drifted statements both build"** is discharged by the artifact's
    own build (the drifted one, observed) and the seed's witness reference (the faithful one,
    D7: a module the gate builds and audits) — the FLAW-0011 reading (45, accepted) applied
    again; the producer runs no Lean for the faithful statement.
75. **`original_fragments` without `original_steps`.** The record carries the faithful
    fragments the drifted proof changed (none for `restrict-type`); there are no original
    steps, the informal file being unchanged. The `Injection` shape still holds informal
    dials (a copy of the formal ones), which are never emitted.
76. **A composite may embed FLAW-0008** (class 8 lies within FLAW-0001 through FLAW-0010,
    Al's ruling 3 on PR 30; severity gap or fatal): an invalid proof of a true statement whose
    Lean file builds, the composite inheriting `NO` and `lean_build` `pass`.
77. **FLAW-0012 is two instances.** D5 writes "a FLAW-0005 `violated` record at the hidden use
    site" beside the class's own record; STATE's NEXT asked whether that record is a second
    instance or a second record of the FLAW-0012 instance. The reading: a second instance
    (instance 1, FLAW-0005 at the use site; instance 2, FLAW-0012 at the statement — the
    composite's shape, numbered the same way), because the records of one instance share a
    class and a counted instance of each class is what S3's cells need. The alternative — one
    FLAW-0012 instance whose use-site record is of class FLAW-0005 — is listed, not taken.
78. **The use-site record's dials come from the seed's `use`** (`theorem`, `hypothesis`) with
    `mode` fixed `violated` (D5's text), the site's text as written, `replaced: null` and the
    counterexample — 0.2.0's intrinsic shape. The seed has no control and no FLAW-0005
    material of its own: a FLAW-0005 spec on it is refused, so the use-site instance exists
    only through FLAW-0012.
79. **FLAW-0012's confirmation** — "K at the use site, and the seed's Lean witness of the
    negation builds (D7)" — is every record of the use-site instance passing (K in the formal
    file; the span check in the informal file) and `truth` being `value: false`,
    `lean-kernel`, with a reference: the negation witness, a module the gate audits.
80. **The transplanted proof fails exactly where the example says.** The
    Bolzano–Weierstrass step asks for the boundedness of `(x_n)`, which the compact domain
    supplied; the transplant bounds the points by the bound on `f`, an application type
    mismatch at the `have hxb` line (the step's first fragment line), and every other line
    elaborates (the `sorry` control probe) — no cascade.
81. **The seed's statement form:** "attains its supremum" as `∃ x₀, ∀ x, f x ≤ f x₀` (a
    maximum), bounded as `∃ M, ∀ x, |f x| ≤ M`; the negation witness by `arctan` (continuous,
    bounded by `π / 2`, strictly increasing). The informal statement names the maximizer
    `x^*`.
82. **`dropped_hypothesis` is a free-text dial** matched against the seed's material, as
    FLAW-0005's `theorem` and `hypothesis` are; the spec's `site` may be `"auto"` or the use
    site's index.
83. **The seed-level import list covers every material,** so the control's file imports
    `Mathlib.Topology.Instances.Int`, which only the `restrict-type` drift uses — the price of
    D6's one import list per seed.
84. **FLAW-0012's informal-only artifact keeps `YES-KERNEL` on both records,** as the 0.2.0
    FLAW-0005 artifacts do (no informal-only exception like FLAW-0004's `N/A`).
