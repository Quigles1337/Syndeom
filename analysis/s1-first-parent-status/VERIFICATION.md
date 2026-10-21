# Verification and reproduction

**2026-10-20 UTC.** Scope: source correspondence, the explicitly bounded witness,
the proposed raw metadata amendment and preservation. No admission or full formal
refutation follows from a compiler/validator exit code.

## Actual checks

| Check | Result | Retained evidence |
|---|---|---|
| Default branch / PR 5 merge / clean base | main; PR 5 merged by Quigles1337 at 20:41:46Z; base `1a76d9a…`, tree `3b9028c0…`; no reviews recorded | `evidence/remote-state.json`, orientation receipts |
| Fresh v70/v71 archives and exact members | Both expected hashes/lengths; byte-identical to retained S1.3 archives | `retrieval.json`, `correspondence.json`; complete bytes external |
| Source mathematical review | Both witnesses checked; parity/Euler-product derivation and missing-factor/zeta cross-check | WITNESS.md, source locators, S-0024/S-0025 and S-0069–S-0072 |
| Scoped Lean file | Exit 0; all eight theorems audited with only the standard three axioms | `evidence/lean-packet.json` and `.txt` |
| Existing Rust ingest build | Exit 0, locked dependencies; actual producer inputs compared to base revision | `evidence/candidate-run-01/receipt.json` and `build.txt` |
| Two actual fresh generations | Both files byte-identical | Same receipt; `ingest-01.txt`, `ingest-02.txt` |
| Independent typed candidate check | Old and candidate validate with actual source bytes; exact config amendment, ID/hash, both status copies and one-item mapping checked | `validate_candidate.rs`, `validate-candidate.txt` |
| Negative controls | Changed artifact ID, changed source bytes, double-counted mapping and premature effective replacement rejected | Same Rust validator and its actual exit-0 transcript |
| Preservation against merged base | 323 old files unchanged; all 35 S1.4 files frozen; 3 exact historical prefixes; all 20 pair inputs equal their original producer revision | `evidence/preservation-development.json`; repeated after final documentation |
| Orientation gate | Two actual exit-0 runs on the clean base; Lean build, golden axioms `[propext]`, fmt/clippy, 67 Rust tests, 328 tracked files and registers passed | `evidence/gate-orientation*` |
| Final clean committed-tree gate | Recorded externally and in the draft PR after the final commit | See final-tree procedure below; orientation is not a substitute |

The original retrieval command was `python
C:\Users\LEET\Codex\Temp\syndeom-first-parent-20261020\retrieve.py` (exit 0).
Its exact script is retained as `evidence/retrieve.py.txt`; per-object URLs, HTTP
responses and hashes are in `retrieval.json`. No separate raw retrieval-console log
was captured. `source-byte-review.json` records a subsequent actual-byte comparison
of all eight downloaded objects, both old archives, committed v70 and the older versus
fresh HTML digests. This check changes none of the retained sources or receipts.

The scoped Lean command was run in `lean/` with **Lean v4.34.1**, Mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`, using the existing lockfiles. The exact
packet file compiled at 21:49:47Z. Its actual output is:

```text
'FirstParentWitness.character_unique' depends on axioms: [propext, Classical.choice, Quot.sound]
'FirstParentWitness.phase_one' depends on axioms: [propext, Classical.choice, Quot.sound]
'FirstParentWitness.phase_two' depends on axioms: [propext, Classical.choice, Quot.sound]
'FirstParentWitness.coefficient_value' depends on axioms: [propext, Classical.choice, Quot.sound]
'FirstParentWitness.finite_step_fails' depends on axioms: [propext, Classical.choice, Quot.sound]
'FirstParentWitness.geometric_tail' depends on axioms: [propext, Classical.choice, Quot.sound]
'FirstParentWitness.residual_series' depends on axioms: [propext, Classical.choice, Quot.sound]
'FirstParentWitness.residual_positive' depends on axioms: [propext, Classical.choice, Quot.sound]
```

There is no sorry/admit or assumed source identity in the accepted witness. The
finite source-sum correspondence and the M/L analytic bridge remain as specified
in WITNESS.md; no CLAIM promotion or complete kernel-refutation claim is made.

## Producer provenance and exact identities

The unchanged producer is `syndeom-ingest` / `s1_ingest` 0.1.0 at
`1a76d9ac21cd5d2e4311f9164178327c0ca128bb`. Before generation the helper compared
all tracked engine inputs and the Rust pin against that revision, refused untracked
engine inputs and local unrecorded Cargo configuration, built with `--locked`, and
retained compiler/Cargo versions, dependency artifacts and binary hash. The actual
binary SHA-256 is `50df014e2db4e49a5588338a0cd8bc1457cb4776d166997172ac4bc98fd1abe4`.
Its `--engine-commit` argument is independently supported by those byte comparisons.
The new validator is compiled separately against that library, outside the engine
source tree, with Rust 1.91.0, `-D warnings`; rustfmt also passes. No pair producer
input or S1.4 implementation changes.

| Object | SHA-256 |
|---|---|
| Old config (JCS) | `63e23a9520843809c8f6b1d538d2ab2f1c3609dd5c6889b3779ea417a3567013` |
| Proposed config (JCS, computed by Rust) | `15ab865466d67dfd18ed487b4e51a3dc1719b07a00b89b3627e92dc111c95364` |
| Generated proposed manifest bytes | `2fbb965ffe92a8d0f102081adc43d84d56182fba5f4fee2bbd7c8dbfd080f8be` |
| Referenced WITNESS.md bytes | `c88af9c0d8b1ac2a65e8bdb634d41fbf3a8bbc1f31c168f81e3caff9e8573917` |
| Original archive, old and proposed | `5987d8106ae69669b10c876a0915d5ad52ce44a814ea7fe63600c1737642cca8` |

The existing ingest command, inspected in its source, is:

```text
syndeom-ingest --config <candidate-config.json> --bytes <old-original.gz>
  --root <fresh-output-tree> --engine-commit 1a76d9ac21cd5d2e4311f9164178327c0ca128bb
```

It writes `<fresh-output-tree>/groundtruth/<new-id>/`; the complete two-file output
was copied byte-for-byte to this packet's `candidate/<new-id>/` for review. The
actual live `groundtruth/` has no new directory. `notes` is not config-hashed;
method/reference are. A raw-to-raw amendment is not encoded as `derived_from`.

## Reproduce into a fresh local destination

From the repository root, with a destination that does not exist:

```powershell
python analysis/s1-first-parent-status/reproduce.py `
  --repo C:\Users\LEET\Documents\ChatGPT\Syndeom `
  --work C:\Users\LEET\Codex\Temp\syndeom-first-parent-replay-NEW
```

The helper records actual commands, exit codes and raw output hashes; builds the
existing producer; generates twice; runs the Rust candidate/mapping validator;
and compares both files and the mapping to this retained candidate when present.
Python orchestrates and compares bytes; it does not compute a mathematical result,
assign effective labels or confer admission. Output paths/timestamps are not identity.

Compile the supporting witness explicitly from `lean/`:

```powershell
lake env lean C:\Users\LEET\Documents\ChatGPT\Syndeom\analysis\s1-first-parent-status\Witness.lean
```

Run preservation from the repository root, also using a fresh receipt:

```powershell
python analysis/s1-first-parent-status/check_preservation.py `
  --repo C:\Users\LEET\Documents\ChatGPT\Syndeom `
  --output C:\Users\LEET\Codex\Temp\syndeom-first-parent-preservation-NEW.json
```

## Development failures and skipped checks

- The first Lean prototype failed on implicit character simplifications and geometric
  normalization. The second fixed the substantive finite obligations but left a
  `1 * inverse` normalization mismatch in the weighted series. Both exit-1 logs and
  exact prototype files are retained; their `sorryAx` outputs are failed elaboration,
  never accepted proof evidence. After these two attempts, the approach switched from
  broad simplification to explicit character facts and an explicit ring equality.
  The third prototype and exact packet file passed; all eight actual audits are clean.
- A sandbox process query returned no visible matching processes, and CIM access was
  denied. An unnecessary second orientation gate was started. Elevated inspection
  showed the first was still running. Both completed on the same clean base; neither
  was interrupted, reported failed, or substituted for a final-tree gate.
- An initial read used `STATE.md` rather than `PROGRAM/STATE.md`; no root file exists.
  A guessed ingest README path also did not exist; actual CLI source was inspected.
  These were read/discovery misses, not compiler or evidence failures.
- No complete analytic-number-theory Lean bridge was attempted. No floating-point
  approximation, finite-partial-sum stand-in, human referee, blindness claim, detector,
  S2/S3 work, canonical derivation, new harvest or private Eigenverse work occurred.
- Historical S1.4 reports/bins and pair mutation/Lean suites were not regenerated.
  Their evidence remains historical; preservation and the standard gate are the checks
  relevant to unchanged implementations. No other truth/severity question is adjudicated.

External raw logs and retrievals live under
`C:\Users\LEET\Codex\Temp\syndeom-first-parent-20261020` and remain first-write files.
Committed transcript copies use LF for repository policy. `transcript-copies.json`
maps their SHA-256 to the raw transcripts; empty raw output has an explicitly marked
text representation. Raw receipt hashes are not silently recomputed for converted logs.

## Final-tree receipt without a self-reference loop

After all packet/doc changes are committed and `git status --porcelain` is empty,
`capture.py` runs `C:\Program Files\Git\bin\bash.exe scripts/check.sh` from the root
to a fresh external `gate-final.log/.json`. The receipt records the exact commit/tree,
start/end status, command, exit code and log hash. A final explicit witness compile,
candidate replay and preservation check also use fresh external destinations. Their
actual results and links are put in the draft PR, not committed into the tree being
certified. A changed final tree must be gated again. The draft PR stays unmerged.
