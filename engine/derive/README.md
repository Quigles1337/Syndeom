# `syndeom-derive` 0.1.0

Deterministic reviewed extraction under [ADR-0006](../../PROGRAM/DECISIONS/ADR-0006.md).
The existing `syndeom-manifest/2` contract and `syndeom-informal-md/1` parser are reused.
This producer does not modify `syndeom-ingest`, decide source status or verify mathematics.

Supported input: one gzip-compressed TeX source, a unique labeled proposition/theorem/lemma
immediately followed by a proof, UTF-8 statement/proof slices, a reviewed recipe and an
already-admitted raw informal parent. Only ASCII whitespace is collapsed; source commands
and mathematics remain literal. The complete decoded source is required external context.
Some TeX constructs are deliberately refused; see the procedure for the precise boundaries.

From the repository root, after committing the producer:

```text
cargo run --locked -q --manifest-path engine/Cargo.toml -p syndeom-derive -- --config <recipe.json> --parent <parent/manifest.json> --bytes <original.gz> --root . --external-dir <existing-directory-outside-repo> --engine-commit <producer-commit>
```

`--dry-run` performs input, derivation and manifest validation without writing, but skips
the existing-output content-collision preflight in `write_outputs`. It does not certify
that a subsequent write will find no conflicting output. Both modes verify the commit against
the tracked producer/library/lock/toolchain inputs. Manifests and metadata go under
`groundtruth/<child-id>`; proof and full context go under `<external-dir>/<child-id>`.
Identical retry is accepted; changed existing output is refused before writing anything.
After a crash, identical complete files can be retried to add missing siblings; incomplete
files are conflicts requiring a new destination or separately reviewed recovery. Do not hand-edit the
manifest or overwrite retained evidence to make a run appear successful.

The recipe contains no proof text: pinned half-open source byte ranges, their SHA-256,
context roles, a title, and Unicode-scalar flaw spans with text hashes. Review those against
the original source and the status record first. A supported format alone does not authorize
promotion; in particular the known held parent cannot bypass its RECORD hold.

The single real example has an orchestration-only reproduction script:

```text
python -I -X utf8 scripts/reproduce_s1_3.py --work-dir <new-directory-outside-repo>
```

Once the evidence commit is present it reads the recorded producer commit, retrieves and
hash-checks the original, invokes Rust twice, compares exact bytes, and writes fresh logs
and `receipt.json` outside the repository. `--source <already-retrieved-original.gz>` skips
the download but still checks the pinned digest. An existing work directory is refused.
Python supplies no canonicalization, counts, IDs or manifest authority.

Focused mechanical checks:

```text
cargo test --locked --manifest-path engine/Cargo.toml -p syndeom-derive
```

The test source and labels are explicitly artificial. They exercise ground-truth *schema
shape* without being admitted corpus items. The fixed zero commit is mechanical test data;
real CLI execution requires an actual producer commit. No restricted real source is embedded
in tests.
