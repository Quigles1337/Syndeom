# Goldens for `s2_template` (ADR-0011 D10)

There are 59 cases: 12 of `s2_template` 0.1.0 (Session 40), 14 of 0.2.0 (Session 41), 9 of
0.3.0 (Session 42), 8 of 0.4.0 (Session 43), 4 of 0.5.0 (Session 44) and 12 of 0.6.0
(Session 45). All use seed 7. Each case directory holds:
- `case.json`: the config, the seed, the engine commit, the toolchain, and the harness
  observation;
- the exact bytes: `proof.md` or `Proof.lean` (both for a pair), and `manifest.json`.

`tests/goldens.rs` regenerates each case without Lean, from the recorded observation, under the
producer version its `manifest.json` names (Al's ruling 5 on PR 26: the 0.1.0 goldens stay
regenerable under the version they name), and compares the bytes. No golden is rewritten except
under a ruling that says so.

## 0.1.0 (Session 40, 2026-10-28)

Each seed's control and, where FLAWS.md 0.1.0 gives an example, the flawed artifact, each as an
informal and as a formal artifact. The counts are per record (no case is a pair).

| Seed | Cases |
|---|---|
| `nat_le_sq` | control |
| `sq_plus_one_ge_two_x` | control; FLAW-0001 `propagated` (fatal); FLAW-0001 `local` (repairable) |
| `n_sq_mod_four` | control; FLAW-0014 gap (the odd case omitted; the claim holds there) |

**Provenance:**
- The proof text is hand-authored seed material (`seeds/*.json`).
- The formal files were first written by hand as Lean probes and compiled before the crate
  existed (`analysis/s2-generate-1/evidence/lean-probe/`).
  - The three flawed formal goldens are byte-identical to their probes.
  - The two faithful probes are their goldens followed by the probe's own
    `\n#print axioms <main>\n` tail (Al's ruling 6 on PR 26 corrected this record).
  - The `nat_le_sq` control is checked instead by the D8 writer test against Golden B.
- The files and manifests were produced by `syndeom-generate generate --dry-run` at engine
  commit `21f83594a22d514bb9cdb1ec0261f659e7d1f4b9`. The Lean harness ran then, one process at a
  time, and each `observation` is what it measured: exit code, first error line, and axioms.
- Every case's fidelity verdict is PASS. The three flawed formal cases fail at line 6, which is
  the site's fragment (check K).

## 0.2.0 (Session 41, 2026-10-28)

The true seed's control (informal, formal) and every new seed's FLAWS.md example as an informal,
a formal and a pair artifact. The counts are per instance: a pair's two records share
`instance: 1`, and `n_flaws_total` is 1 (Al's ruling 3 on PR 26 and the owner note of 2026-10-28).

| Seed | Cases | Statement |
|---|---|---|
| `continuity_is_not_uniform` | control (informal, formal); FLAW-0002 `forall-exists` gap (informal, formal, pair) | true |
| `x_over_abs_x` | FLAW-0014 `holds: false` fatal (informal, formal, pair) | false: no control |
| `rolle_abs` | FLAW-0005 `violated` fatal (informal, formal, pair) | false: no control |
| `horses` | FLAW-0006 `step-above-base` fatal (informal, formal, pair) | false: no control |

**Provenance:**
- The proof text is hand-authored seed material (`seeds/*.json`). A seed whose statement is false
  carries the convincing argument as its only argument, with its counterexample; the witness
  module proves the negation.
- The formal files were first written by hand as Lean probes and compiled before the seed records
  were written, one process at a time (`analysis/s2-generate-2/evidence/lean-probe/`):
  - the eight flawed formal and pair goldens are byte-identical to their probes;
  - the faithful probe of `continuity_is_not_uniform` is its control golden followed by the
    probe's own `\n#print axioms <main>\n` tail.
- The files and manifests were first produced at engine commit `af28cc96…` and then, by Al's
  rulings of 2026-10-28 on PR 27, **regenerated at engine commit
  `c899b0732db8b7a1a104822772a428f534a651cd`**, the commit that relabels the horses record's
  `mode` from `shift-index` to the new D5 value `step-above-base` (`failing_case: "n + 1 = 2"`
  kept). Two separate runs of `syndeom-generate generate --dry-run` at that commit produced
  byte-identical outputs for all 14 cases (`analysis/s2-generate-2/evidence/goldens-regeneration-report.json`),
  with the running binary's compiled input hashes verified against the commit before each run.
  The horses proof bytes did not change; its manifests, config hashes and artifact ids did. The
  other eleven cases differ from the first production only in the engine commit they name.
- Every case's fidelity verdict is PASS. The flawed formal and pair cases fail at the site's
  fragment (check K): line 6 (`continuity_is_not_uniform`, the swapped `obtain`), line 4
  (`x_over_abs_x`, the `cases` with the `eq` alternative missing), line 7 (`rolle_abs`, the Rolle
  application whose differentiability argument fails) and line 20 (`horses`, the overlap claimed
  non-empty, which `omega` refuses without `2 ≤ m`). The control's audit prints
  `[propext, Classical.choice, Quot.sound]`.

## 0.3.0 (Session 42, 2026-10-28)

The seed's control (informal, formal), FLAW-0009 `axiom` with `truth` true and false (formal and
pair each: FLAW-0009 is formal-only, so a pair carries one record and the informal file is the
faithful one), and FLAW-0007 `unused` (informal, formal, pair). The flawed formal files build:
these are the tooling-caught classes, confirmed by check T from the audit tail's used-constants
listing (FLAW-0007) and `#print axioms` (FLAW-0009).

| Seed | Cases | Statement |
|---|---|---|
| `n_lt_two_pow` | control (informal, formal); FLAW-0009 `axiom` `truth: true` gap (formal, pair); FLAW-0009 `axiom` `truth: false` fatal (formal, pair); FLAW-0007 `unused` cosmetic (informal, formal, pair) | true |

**Provenance:**
- The proof text is hand-authored seed material (`seeds/n_lt_two_pow.json`): Lemma 1 (`n < 2 ^ n`,
  by induction) and the conclusion; the FLAW-0009 materials replace Lemma 1's declaration by an
  `axiom` — of the same statement (`truth: true`), or of the false lemma `n + 1 < 2 ^ n` with the
  conclusion adapted (`truth: false`; the witness module proves the negation); the FLAW-0007
  material inserts Lemma 2 (`n ≤ n ^ 2`, proved, cited nowhere).
- The formal files were first written by hand as Lean probes and compiled before the seed record
  was written, one process at a time (`analysis/s2-generate-3/evidence/lean-probe/`): every one
  of the seven formal and pair goldens is byte-identical to its probe without the probe's own
  tail (`\n#print axioms …` and, for two probes, a used-constants `#eval`), as
  `analysis/s2-generate-3/evidence/goldens-production-report.json` records.
- The files and manifests were produced by `syndeom-generate generate --dry-run` at engine commit
  `5a313ddf8de6a870c2f41c9d65d8cb87a1595b2e`, by two separate runs compared byte for byte
  (every `proof.md`, `Proof.lean` and `manifest.json` identical, every observation equal), with
  the running binary's compiled input hashes verified against the commit before each run.
- Every case's fidelity verdict is PASS. Every formal and pair case builds (exit 0). The control
  and the FLAW-0007 cases print `[propext, Quot.sound]`; the FLAW-0009 cases print
  `[syndeom_s2_template_n_lt_two_pow_lemma_1]`, the smuggled axiom. The used-constants listing
  of the FLAW-0007 cases names the file's four constants (the two lemmas, Lean's auxiliary
  `…lemma_1._proof_1_1`, and the main declaration), and the main declaration uses Lemma 1 only.

## 0.4.0 (Session 43, 2026-10-28)

The `sqrt2_irrational` seed's control (informal, formal); a lightly padded control — FLAW-0013
with one re-derivation paragraph at a drawn position (`kinds: ["re-derivation"]`, `count: 1`,
`placement: "drawn"`) — and the FLAWS.md FLAW-0013 example — every paragraph of the seed's
material, four restated definitions and three re-derivations of `2 | p² ⇒ 2 | p`, before the
five-step argument (`kinds: ["restated-definition", "re-derivation"]`, `count: 7`,
`placement: "before"`) — each as an informal, a formal and a pair artifact. Padding is
cosmetic, so every padded case is a correct proof (`proof_status: valid`, `correct: true`) with
its paragraphs recorded, one instance each. The commanded `padding_ratio` of a padded case is
the ratio its paragraphs achieve at seed 7, to two decimals (informal and pair: 0.28 and 0.71;
formal-only: 0.31 and 0.61, measured over the declarations' tokens), because the ratio is a
measured statistic (ADR-0011 D4) checked at tolerance `abs 0.05`.

| Seed | Cases | Statement |
|---|---|---|
| `sqrt2_irrational` | control (informal, formal); FLAW-0013 `drawn`, 1 paragraph, cosmetic (informal, formal, pair); FLAW-0013 `before`, 7 paragraphs, cosmetic (informal, formal, pair) | true |

**Provenance:**
- The proof text is hand-authored seed material (`seeds/sqrt2_irrational.json`): five aligned
  steps of the descent argument on `p ^ 2 ≠ 2 * q ^ 2` for coprime `p`, `q`, and seven padding
  paragraphs, each with its matching unused declaration.
- The formal files were first written by hand as Lean probes and compiled before the seed record
  was written, one process at a time (`analysis/s2-generate-4/evidence/lean-probe/`): the
  control golden is the faithful probe `s2i_faithful.lean` without its `#print axioms` tail, and
  every padded formal and pair golden's declarations are blocks of the padded probe
  `s2i_padded.lean`, in drawn order, followed by the probe's main declaration, as
  `analysis/s2-generate-4/evidence/goldens-production-report.json` records.
- The files and manifests were produced by `syndeom-generate generate --dry-run` at engine commit
  `b54e2f16aa3c153258bf6ece67798bd270993cbb`, by two separate runs compared byte for byte
  (every `proof.md`, `Proof.lean` and `manifest.json` identical, every observation equal), with
  the running binary's compiled input hashes verified against the commit before each run.
- Every case's fidelity verdict is PASS. Every formal and pair case builds (exit 0) and prints
  `[propext, Quot.sound]`. The used-constants listing of a padded case names its padding
  declarations, none of them reachable from the main declaration, which uses only its own
  auxiliary `_proof_N` constants (check T).

## 0.5.0 (Session 44, 2026-10-28)

FLAW-0004 `routine-true` on the `sqrt2_irrational` seed's step 3 — the FLAWS.md FLAW-0011
example's hedge: `2 | p² ⇒ 2 | p` asserted with a hedge and the parity argument dropped — as an
informal and a pair artifact (the formal target is refused for FLAW-0004: FLAWS.md says it
manifests in informal), and FLAW-0011 as a commanded composite of one gap instance,
`continuity_is_not_uniform`'s FLAW-0002 `forall-exists` gap (`embedded`), as an informal and a
pair artifact. At seed 7 the injector draws hedge 1 of the fixed lexicon, `It is easy to see
that`, so step 3 reads `It is easy to see that $p$ is even. Write $p = 2k$.`; in the formal file
the proof of `have hp : 2 ∣ p := by` is omitted (the `have` header and the `obtain` stay; no
`sorry`), and the kernel reports `unsolved goals` at line 7, the site's first fragment line
(check K). The hidden step is routine and true, so the instance is cosmetic and the artifact is
a valid proof of a true statement under ADR-0011 D7 (`proof_status: valid`, `correct: true`),
with `lean_build` commanded `fail`; the informal-only artifact carries `N/A`. The composite's
embedded instance is the 0.2.0 FLAW-0002 instance byte for byte (the same draw, the same files,
failing at line 6); the composite inherits `gap` and `YES-KERNEL`, its own records sit at step 0
(the statement) and at the main declaration's header line (line 5), and the artifact is an
invalid proof of a true statement (`proof_status: invalid`, `correct: false`), two instances.

| Seed | Cases | Statement |
|---|---|---|
| `sqrt2_irrational` | FLAW-0004 `routine-true` cosmetic (informal, pair) | true |
| `continuity_is_not_uniform` | FLAW-0011 embedding FLAW-0002 `forall-exists` gap (informal, pair) | true |

**Provenance:**
- The proof text is hand-authored seed material (`seeds/sqrt2_irrational.json`: the five
  faithful steps and the FLAW-0004 material's bare claim at step 3 with the justification and
  the four omitted tactic lines recorded; `seeds/continuity_is_not_uniform.json`: the 0.2.0
  material, unchanged).
- The hedged formal file was first written by hand as a Lean probe and compiled before the seed
  material was written, one process at a time (`analysis/s2-generate-5/evidence/lean-probe/`):
  the FLAW-0004 pair golden's `Proof.lean` is byte-identical to the probe `s2i_hedged_p1.lean`,
  which has no tail. The composite's `proof.md` and `Proof.lean` are byte-identical to the
  0.2.0 goldens `continuity_is_not_uniform/flaw-0002-gap-informal` and `-pair` (the composite
  changes no byte of the embedded instance's files), as
  `analysis/s2-generate-5/evidence/goldens-regeneration-report.json` records.
- The files and manifests were first produced at engine commit `7be41123…` (two FLAW-0004
  cases and two composites embedding that cosmetic instance) and then, by Al's ruling 2 of
  2026-10-28 on PR 30, **regenerated at engine commit
  `1c0c5fcdf1ffd375b109f272461a63c49330e007`**, the commit that refuses a composite whose
  embedded instances are all cosmetic and restricts `embedded` to FLAW-0001 through FLAW-0010:
  the two cosmetic composites were removed and replaced by the FLAW-0002 gap composite, and the
  FLAW-0004 cases were regenerated so that the four cases name one engine commit — their proof
  bytes, ids and ground truth unchanged, only `manifest.json` and `case.json` naming the new
  commit. Two separate runs of `syndeom-generate generate --dry-run` at that commit produced
  byte-identical outputs for all four cases, with the running binary's compiled input hashes
  verified against the commit before each run.
- Every case's fidelity verdict is PASS. Both pair cases fail to build (exit 1) with the first
  error at the site's first fragment line (7 for the hedge, 6 for the swapped `obtain`);
  `lean_axioms_standard` is therefore unmeasured (`null`, with its reason). Each instance is
  confirmed: the FLAW-0004 informal record by the D5 check (the hedge entry, a space and the
  bare claim; the faithful text held the justification exactly once), every formal record by
  check K, the FLAW-0002 informal record by its span check, and the composite by the seed's
  kernel-checked witness together with its embedded instance's records.

## 0.6.0 (Session 45, 2026-10-29)

FLAW-0010 `a` (`shape: journal`, `claim: true`) on the `sqrt2_irrational` seed's step 3 — the
FLAW-0004 material's bare claim, its justification replaced by a fictional reference drawn from
the fixed lexicon — as an informal and a pair artifact (the formal target is refused: FLAWS.md
says the class manifests in informal, and a pair's formal file is the faithful one); and, on the
new seed `bounded_monotone_converges` (FLAWS.md's FLAW-0003 example: a monotone real sequence
that is bounded above converges, in two aligned steps on the supremum), the control (informal,
formal), FLAW-0003 `direct` with `form: restated-goal` and with `form: contrapositive` (informal,
formal, pair each) and FLAW-0003 `cycle` (`cycle_length: 2`; informal and pair — the formal
target is refused, a cycle having no formal manifestation). At seed 7 the injector draws author
2, year 0, venue 1 and result 3 of the lexicon, so step 3 reads `By [Fictitious 19XX, Annals of
Imaginary Results, Corollary 1.7] $p$ is even. Write $p = 2k$.`, the record spanning the
reference; the formal file of the FLAW-0010 pair is byte-identical to the seed's control
`Proof.lean`, so the artifact is `N/A`, invalid (gap), with `lean_build` commanded `pass`. The
`direct` cases insert Lemma 1 (the goal, or its contrapositive) whose proof is `exact
syndeom_s2_template_bounded_monotone_converges`, a forward reference: the kernel reports an
unknown identifier at line 5, inside the lemma's declaration (lines 4–5; check K), and the key
step cites Lemma 1. The `cycle` cases insert Lemma 1 whose proof cites the theorem while the key
step cites Lemma 1; the record (informal-only, `NO`) carries the cycle `[lemma_1, main]` and
the two citation phrases' spans, and the pair's formal file keeps a faithful proof (the lemma
proved from Mathlib, the main declaration using it), which builds with the standard three axioms.

| Seed | Cases | Statement |
|---|---|---|
| `sqrt2_irrational` | FLAW-0010 `a` `journal` `claim: true` gap (informal, pair) | true |
| `bounded_monotone_converges` | control (informal, formal); FLAW-0003 `direct` `restated-goal` fatal (informal, formal, pair); FLAW-0003 `direct` `contrapositive` fatal (informal, formal, pair); FLAW-0003 `cycle` fatal (informal, pair) | true |

**Provenance:**
- The proof text is hand-authored seed material (`seeds/sqrt2_irrational.json`: the FLAW-0010
  material's bare claim at step 3 with the justification recorded, every fragment faithful;
  `seeds/bounded_monotone_converges.json`: the two faithful steps and the three FLAW-0003
  materials, each inserting Lemma 1).
- The formal files were first written by hand as Lean probes and compiled before the seed record
  was written, one process at a time (`analysis/s2-generate-6/evidence/lean-probe/`): the
  control golden is the faithful probe `s2i_faithful.lean` without its `#print axioms` tail, the
  `direct` formal and pair goldens are byte-identical to `s2i_direct_restated.lean` and
  `s2i_direct_contrapositive.lean` (which have no tail), the `cycle` pair golden is
  `s2i_cycle.lean` without its tail, and the FLAW-0010 pair golden's `Proof.lean` is the 0.4.0
  control golden's, as `analysis/s2-generate-6/evidence/goldens-production-report.json` records.
- The files and manifests were produced by `syndeom-generate generate --dry-run` at engine commit
  `ef41a10ce16db34c1ebe3ea66a456df97df83be1`, by two separate runs compared byte for byte
  (every `proof.md`, `Proof.lean` and `manifest.json` identical, every observation equal), with
  the running binary's compiled input hashes verified against the commit before each run.
- Every case's fidelity verdict is PASS. The four `direct` formal and pair cases fail to build
  (exit 1) with the first error at line 5; `lean_axioms_standard` is therefore unmeasured
  (`null`, with its reason). The control, the `cycle` pair and the FLAW-0010 pair build (exit 0)
  and print the standard axioms (`[propext, Classical.choice, Quot.sound]` for the new seed,
  `[propext, Quot.sound]` for the faithful `sqrt2_irrational` file); the `cycle` pair's
  used-constants listing shows the main declaration using Lemma 1, and Lemma 1 using
  `tendsto_atTop_ciSup` and `le_ciSup`.

The receipts are under the sessions' external evidence roots. Their digests and the runs are
recorded in `analysis/s2-generate-2/README.md`, `analysis/s2-generate-3/README.md`,
`analysis/s2-generate-4/README.md`, `analysis/s2-generate-5/README.md` and
`analysis/s2-generate-6/README.md`.

The `engine_commit` in these manifests is the commit the producer ran from, and it is a fixed
input of the test. A later commit that changes the emitted bytes for some (config, seed) bumps
the producer's version (ADR-0011 D1). It does not rewrite these files.
