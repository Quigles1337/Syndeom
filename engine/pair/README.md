# syndeom-pair — the reviewed first S1.2 pair

This crate implements ADR-0007 for one fixed candidate: Judson's AATA 2025 Theorem
2.2.6 and `Nat.infinite_setOfPred_prime` at the existing Mathlib pin. Archive mode
is `s1_pair_archive` 0.1.0; canonical mode is `s1_pair` 0.1.0. These are two
representations of one independently published statement and its Mathlib offered
argument. Generation does not itself accept a main-branch admission or promote a CLAIM.

The corrected procedure/configuration revision is
`d88fad2bb7b339c56bd9964167d3e0acddc8ee4e`, over merged PR 3 main
`1a8388b03017e4044cad01ac10e7e89f8fb5e38f`. The original evidence and separate correction
evidence remain in `analysis/s1-2/`. The source interval is **27–28, one-based inclusive**;
the alias declaration starts at line 3. The former 28–29 range is a regression control.

## Exact contract and actual bytes

The two corrected configs and alias are compile-time inputs. The complete config must
equal the reviewed object and its hardcoded Rust JCS digest, so changing input bytes
and caller-supplied expected hashes together cannot authorize a substitution. Unknown,
missing or duplicate JSON fields, alternate methods, sources, declarations, types,
statuses and recipes are rejected. Supporting a different candidate requires review
and a producer revision, not a new caller-supplied hash.

Archive mode requires all five context objects from the reviewed receipt:
`aata-20250801.pdf`, `PrimeFin.lean`, `Infinite.lean`, `PrimeDefs.lean`, and
`mathlib-LICENSE`. The book includes its own copyright page and full licence appendix.
The full PDF and full Mathlib module become the raw pair files. All five original
objects remain external under the archive ID with exact digests and original bytes.

Canonical mode resolves `groundtruth/<reviewed-archive-id>/manifest.json` under the
artifact root and the actual five files under the external archive directory. It
also requires the complete archive output set, including its deterministic source map.
It calls `Manifest::check` with both raw files, then checks the exact reviewed parent
manifest/config/producer identity. It emits only the fixed zero-step informal statement
and exact-type alias, checks both actual canonical files, `check_derived`, and the
additional source/item/licence/status/toolchain/ground-truth/config/recipe invariants.
The actual parent-manifest hash is in the child source map and the run receipt.

The caller cannot supply a replacement parent path or an alias as the raw Mathlib object.
Canonical `Proof.lean` is committed; `proof.md` remains external. Both representations'
manifests and source maps are committed. The source-map checker inspects the actual
declaration and proof-body lines; it is not a general Lean parser or semantic judge.
The independent statement correspondence review remains in the candidate packet.
Empty-statistics fidelity PASS is vacuous for semantics.

## Provenance before generation

Commit the tested producer before running either production mode. `build.rs` embeds
SHA-256 of every file in `input-files.txt` into the executable, including itself,
the new crate, manifest library, workspace/lockfiles, Rust/Lean pins, corrected configs,
alias and audit template. Unknown source files fail the build/provenance check.
At run time every recorded Git blob must equal both its embedded hash and current
checkout bytes. The named full commit must exist and be an ancestor of checkout HEAD;
project-local Cargo overrides are rejected. Receipts record the executable hash,
compiler version, input closure and actual Git subprocess outcomes.

The pinned Mathlib HEAD and four formal context blobs are verified separately.
An exact LF-to-CRLF dependency checkout conversion is allowed and recorded; original
raw output bytes are always the LF Git blobs. No arbitrary whitespace normalization
or substitution of working-file hashes occurs.

## Commands

Build from the committed checkout, using the pinned Rust toolchain:

```powershell
cargo build --locked --manifest-path engine/Cargo.toml -p syndeom-pair
```

Use the resulting `engine/target/debug/syndeom-pair.exe` on Windows (omit `.exe`
on Unix). All directory arguments except the new receipt directory must already exist.
The command below uses placeholders for explicit local paths and the actual producer SHA:

```text
syndeom-pair --mode archive --config <repo>/analysis/s1-2/archive-config.proposed.json --root <repo> --external-dir <external-output> --sources-dir <reviewed-source-directory> --engine-commit <producer-SHA> --receipt-dir <NEW-external-archive-receipt-directory>
syndeom-pair --mode canonical --config <repo>/analysis/s1-2/canonical-config.proposed.json --root <repo> --external-dir <same-external-output> --engine-commit <same-producer-SHA> --receipt-dir <NEW-external-canonical-receipt-directory>
```

`--artifact-root <existing-independent-directory>` is an explicit replay destination:
its `groundtruth/` layout holds the replay archive and child while `--root` continues
to name the producer checkout for provenance. It must be the repository root or fully
outside it. External bytes and receipts must be separate from the artifact/producer roots;
receipts must also be separate from the external bytes directory. Source inputs cannot
overlap archive output roots. No output is written under a different existing item ID.

`--dry-run` performs **all** input, provenance, contract, parent/child and complete
output-collision preflight checks, and writes only its new run receipt. It makes no
Lean-compilation or admission claim. A fresh receipt directory is required even for
an identical replay; historical receipts are never overwritten.

Before any artifact write the entire planned set is checked. Existing different bytes,
directories at file locations, duplicate/case-aliased paths, dot components, symlinks,
Windows junctions/reparse points, and hard-linked output files are refused.
Windows link-count inspection uses `fsutil hardlink list` and fails closed if that query
fails; Unix uses `nlink`. Writes use exclusive creation. Exact-byte retries leave
existing files/timestamps untouched and may fill missing siblings. A crash/partial file
is a failed output set, never an admission; a different partial file is a conflict.
The whole directory set is not a filesystem transaction, and a failed run's new receipt
records failure. These checks assume no hostile concurrent changes to filesystem ancestors.

Exit 0 means the specified mechanical checks passed (or dry-run preflight passed).
Exit 2 means refusal/failure. Valid invocations reserve a new external receipt directory;
failures after that reservation get `receipt.json` with actual command exits and error.
Invalid arguments, unsafe receipt paths and existing receipt destinations are rejected
before reservation. No Lean subprocess is part of this producer: admission verification
must compile/audit the actual emitted file separately.

## Verification boundary

`cargo test --locked -p syndeom-pair --all-targets` covers the exact canonical bytes,
zero-step syntax, frozen identities, contract substitutions, duplicate/unknown fields,
source-line regression, both canonical byte checks, parent/metadata changes, complete
output preflight/dry-run conflicts, idempotence, partial siblings, links, and executable
provenance. The Git subprocess refusal controls are explicitly development fixtures.

The unit tests do not redistribute the book or pretend synthetic bytes are its raw hash.
Actual full-source archive/canonical success, emitted Lean/type/axiom checking, fresh
generation replay and real-CLI refusal evidence run after the producer commit and are
recorded separately under `analysis/s1-2/implementation/`. The owner merged that
admission packet in PR 4 on 2026-10-20 at 18:10:13Z. The post-merge dispositions and
current coverage/qualifications are in `analysis/s1-4/`; no PROVEN promotion or separate
independent reviewer is inferred. The producer contract and all 20 inputs are unchanged.
