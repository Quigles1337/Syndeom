# Goldens for `s2_template` 0.1.0 (ADR-0011 D10)

There are 12 cases. Each seed has its control and, where FLAWS.md 0.1.0 gives an example, the
flawed artifact, each as an informal and as a formal artifact. All use seed 7.

| Seed | Cases |
|---|---|
| `nat_le_sq` | control |
| `sq_plus_one_ge_two_x` | control; FLAW-0001 `propagated` (fatal); FLAW-0001 `local` (repairable) |
| `n_sq_mod_four` | control; FLAW-0014 gap (the odd case omitted; the claim holds there) |

Each case directory holds:
- `case.json`: the config, the seed, the engine commit, the toolchain, and the harness
  observation;
- the exact bytes: `proof.md` or `Proof.lean`, and `manifest.json`.

`tests/goldens.rs` regenerates each case without Lean, from the recorded observation, and
compares the bytes.

**Provenance (Session 40, 2026-10-28):**
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

The receipts are under the session's external evidence root. Their digests and the runs are
recorded in `analysis/s2-generate-1/README.md`.

The `engine_commit` in these manifests is the commit the producer ran from, and it is a fixed
input of the test. A later commit that changes the emitted bytes for some (config, seed) bumps
the producer's version (ADR-0011 D1). It does not rewrite these files.
