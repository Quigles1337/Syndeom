# Executed S1.2 implementation and proposed-admission checks

All commands below were actually executed. The original procedure rehearsal and Phase A
correction evidence remain separate and unchanged. This document concerns the real committed
producer and generated pair, followed by the explicit review/admission boundary.

## Producer checks before generation

| Run | Actual result |
|---|---|
| `producer-check-01` | `cargo check --offline -p syndeom-pair`, exit 0 |
| `producer-tests-01` | exit 101; 8 contract tests passed, 3 of 6 I/O tests failed; actual Windows extended-drive-prefix query bug |
| `producer-tests-02` | exit 101; 13 of 14 tests passed; fixture normalized `..` before testing the checker |
| `producer-tests-03` | exit 0; all 14 new tests passed after those fixes |
| `producer-clippy-01` | workspace all-target/all-feature clippy with `-D warnings`, exit 0 |
| `producer-gate-01` | full `bash scripts/check.sh`, exit 0 / `GATE: GREEN`, 51 tests |

The first failure was a production path-handling defect; the second was a test-fixture
defect. Neither was a failed mathematical audit. Both failed runs are retained in
[`evidence/producer-checks/`](evidence/producer-checks/). No producer-affecting edit
occurred after the passing gate or actual generation.

The pre-generation gate ran 2026-10-20T06:17:12.966030+00:00 to 2026-10-20T06:28:55.544784+00:00, with
the same staged tree before/after: `ad9cbee9a1939ddf3e0deb200a8afbe8db642d53`. The next commit,
`eb1b96d5b8084ceb1de734fffc90c9b480ec32bf`, has exactly that tree. The gate built 8,926 Lean jobs,
golden axioms `[propext]`, clean fmt/clippy, 51 tests (24 manifest, 5 ingest, 8 derive,
14 pair), 203 tracked files and coherent Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 68.

## Real generated objects and fresh audit

Production ran 2026-10-20T06:33:04.160227+00:00 to 2026-10-20T06:34:11.430260+00:00, exit 0.
Fresh HTTP retrieval verified all five fixed source objects. The executable SHA-256
was `a63de04b8d0aef958fbdcb3b3b7d2bb621217f6312e7e99b447184e74f3d1ba6`; the orchestration helper hash was
`a4ad574e215c8ee5c7798de2c48487db9cb3650e6168cfa30ff62e4740715c78`. The helper is copied exactly as `reproduce.py`.
Rust was `rustc 1.91.0 (f8297e351 2025-10-28)`; Lean was `Lean (version 4.34.1, x86_64-w64-windows-gnu, commit 5045d0056413266e57c625dcd7c365b10e377c52, Release)`.

- Both actual raw files passed `Manifest::check`; both actual canonical files passed
  `Manifest::check`; `check_derived` and all ADR-0007 stronger invariants passed.
- The actual parent identity/config/producer, full raw output set, shared
  source/item/licence/status/toolchain/ground-truth fields, exact recipe and mapped
  source lines were checked. Generated files were read back and checked again.
- The executable's 20 compiled input hashes matched both the producer Git blobs and
  current checkout. Pinned Mathlib Git blobs were checked; exact LF-to-CRLF checkout
  conversion was separately recorded, without changing raw archival bytes.
- Fresh generation replay produced all 11 deterministic files identically, including
  both manifests, both source maps, the alias and all external source/output objects.
  Run timestamps, destinations and command lines legitimately differ between receipts.
- Identical retries preserved artifact bytes and mtimes. Dry runs checked the whole
  output set; a fresh archive dry run wrote no artifact. Eighteen real-CLI mutation
  controls returned 2 with failure receipts and unchanged fixture bytes/mtimes;
  reusing a retained receipt destination also returned 2 without overwriting it.

The fresh actual-emitted-file compilation and subsequent complete-type/axiom audit
both exited 0. The alias and target have exactly the same complete type. The target,
alias and three principal dependencies use the standard three axioms; `Nat.prime_def`
uses only `propext`. Full printed dependency bodies and actual commands are retained.
The old candidate audit was not substituted for this run.

All 33 top-level commands, with detailed argv/cwd/timestamps/raw-log hashes in
[`verification-receipt.json`](evidence/production/verification-receipt.json):

| Command label | Actual exit | Classification |
|---|---|---|
| `producer-build` | 0 | positive |
| `rust-version` | 0 | positive |
| `lean-version` | 0 | positive |
| `archive` | 0 | positive |
| `canonical` | 0 | positive |
| `emitted-file-compile` | 0 | positive |
| `emitted-type-axiom-audit` | 0 | positive |
| `controlled-lean-failure` | 1 | synthetic subprocess failure fixture; not admitted |
| `archive-identical-retry` | 0 | positive |
| `canonical-identical-retry` | 0 | positive |
| `canonical-existing-dry-run` | 0 | positive |
| `archive-new-dry-run` | 0 | positive |
| `replay-archive` | 0 | positive |
| `replay-canonical` | 0 | positive |
| `wrong-declaration` | 2 | controlled refusal |
| `wrong-type` | 2 | controlled refusal |
| `invented-informal-proof` | 2 | controlled refusal |
| `unknown-field` | 2 | controlled refusal |
| `unsupported-method` | 2 | controlled refusal |
| `wrong-parent-link` | 2 | controlled refusal |
| `wrong-parent-config-hash` | 2 | controlled refusal |
| `source-line-regression` | 2 | controlled refusal |
| `missing-source` | 2 | controlled refusal |
| `altered-formal-source` | 2 | controlled refusal |
| `substituted-source-with-matching-caller-hash` | 2 | controlled refusal |
| `changed-parent-metadata` | 2 | controlled refusal |
| `changed-parent-source-map` | 2 | controlled refusal |
| `missing-actual-parent-file` | 2 | controlled refusal |
| `changed-actual-parent-bytes` | 2 | controlled refusal |
| `late-output-collision` | 2 | controlled refusal |
| `late-output-collision-dry-run` | 2 | controlled refusal |
| `producer-before-implementation` | 2 | controlled refusal |
| `existing-receipt-refused` | 2 | controlled refusal |

The deliberately false Lean fixture is a **synthetic subprocess failure control**.
It is not the generated proof, an admitted corpus item or a failed theorem. Its exit 1
was expected, recorded, and followed by an unchanged-emitted-proof assertion.
Expected CLI refusals below are development mutation controls, not mathematical data.

| Refusal fixture | Actual reason |
|---|---|
| `wrong-declaration` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `wrong-type` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `invented-informal-proof` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `unknown-field` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `unsupported-method` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `wrong-parent-link` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `wrong-parent-config-hash` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `source-line-regression` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `missing-source` | `\\?\C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\controls\missing-source\sources\PrimeFin.lean: The system cannot find the file specified. (os error 2)` |
| `altered-formal-source` | `changed/unreviewed source bytes: PrimeFin.lean` |
| `substituted-source-with-matching-caller-hash` | `unsupported/unreviewed configuration, metadata, source, declaration, type or recipe` |
| `changed-parent-metadata` | `same-ID/different-bytes conflict: \\?\C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\controls\changed-parent-metadata\artifacts\groundtruth\s1_pair_archive-v0.1.0-094625fa10ff\manifest.json` |
| `changed-parent-source-map` | `same-ID/different-bytes conflict: \\?\C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\controls\changed-parent-source-map\artifacts\groundtruth\s1_pair_archive-v0.1.0-094625fa10ff\source-map.json` |
| `missing-actual-parent-file` | `\\?\C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\controls\missing-actual-parent-file\external\s1_pair_archive-v0.1.0-094625fa10ff\PrimeFin.lean: The system cannot find the file specified. (os error 2)` |
| `changed-actual-parent-bytes` | `changed/unreviewed source bytes: PrimeFin.lean` |
| `late-output-collision` | `same-ID/different-bytes conflict: \\?\C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\controls\late-output-collision\artifacts\groundtruth\s1_pair-v0.1.0-62fe0c816a6b\Proof.lean` |
| `late-output-collision-dry-run` | `same-ID/different-bytes conflict: \\?\C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\controls\late-output-collision-dry-run\artifacts\groundtruth\s1_pair-v0.1.0-62fe0c816a6b\Proof.lean` |
| `producer-before-implementation` | `git subprocess failed (Some(128)): fatal: path 'engine/pair/Cargo.toml' exists on disk, but not in 'd88fad2bb7b339c56bd9964167d3e0acddc8ee4e' ` |

The nineteenth refusal reuses the already retained archive receipt directory. It fails
before reserving a new receipt, so it has a command log and orchestration record rather
than a new CLI receipt; the old receipt and artifacts were checked unchanged.

## Evidence retention and final gate

Original raw logs, receipts, sources, replay objects and fixtures remain outside OneDrive at
`C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20`.
[`retention-ledger.json`](evidence/retention-ledger.json) maps raw digests to committed
copies. JSON/source receipts are exact bytes. Text logs use LF display copies; two copies omit terminal empty lines as listed in the
ledger. Empty stdout is explicitly displayed as `(no output)`. The originals are never overwritten.

The new scoped `check_preservation.py` checks historical prefixes, old corpus/evidence,
correction evidence, unchanged dependency entries, and the precisely bounded two-item
branch addition. Its executed result was **PASS**: 162 protected baseline files, 20 correction files,
all historical prefixes and existing dependency entries unchanged; exactly the two expected
new representations. Its receipt is `evidence/preservation.json`. The old procedure-only
31+1/no-pair preservation helper was neither changed nor run against the new inventory.

After the artifact/evidence commit, run the full gate on that exact clean commit and
retain its SHA/tree, start/end times, exit and log digest in the external
`artifact-final-gate-01.json` / `.log`. Record the actual result and hosted check in the
new draft PR without another edit of the checked tree. That final exact-commit result
is separate from the already completed pre-generation gate above.

Only owner/reviewer admission and merging remain deferred by instruction. No source
correspondence mismatch or mechanical validation blocker remains in this packet.
No S2 family, detector, contact work, Eigenverse integration, schema change, claim promotion
or harvest advancement was performed.
