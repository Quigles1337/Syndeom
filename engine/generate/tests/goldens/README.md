# Goldens for `s2_template` (ADR-0011 D10)

There are 26 cases: 12 of `s2_template` 0.1.0 (Session 40) and 14 of 0.2.0 (Session 41). All use
seed 7. Each case directory holds:
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

The receipts are under the session's external evidence root. Their digests and the runs are
recorded in `analysis/s2-generate-2/README.md`.

The `engine_commit` in these manifests is the commit the producer ran from, and it is a fixed
input of the test. A later commit that changes the emitted bytes for some (config, seed) bumps
the producer's version (ADR-0011 D1). It does not rewrite these files.
