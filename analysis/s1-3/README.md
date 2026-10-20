# S1.3 first canonical derivation — reproduction and evidence

This is the ADR-0006 engineering slice, based on public main
`8e222a49a64273c9cefc35bca5ae15a0a05fb9ee`. It adds one deterministic Rust producer and
one external canonical representation of an existing parent. Gate S1 remains open;
S1.2 positive pairs and S1.4 coverage/power remain unimplemented.

Producer commit: **`49e47237924eddf0c39c24aefad0eb9aa02ee8b3`**, committed before the real
manifest was generated. Parent: `s1_ingest-v0.1.0-00f28e252823`.
Child: `s1_derive-v0.1.0-c08f9e3ec092`. The later artifact/evidence commit keeps this producer
commit in `engine_commit`; it does not substitute a helper hash or a later checkout revision.

The public manifest, source map and written source-fidelity/location review are in
`groundtruth/s1_derive-v0.1.0-c08f9e3ec092/`. Restricted proof/context text is external.
The new receipt in `real-result.json` is separate from historical evidence; no prior receipt,
manifest, harvest log, gate record or mathematical claim has been regenerated to match it.

## Reproduce the real example

From the repository root, in PowerShell, use a new local directory outside OneDrive and the
repository (choose a new suffix if it already exists):

```powershell
python -I -X utf8 scripts/reproduce_s1_3.py --work-dir C:\Users\LEET\Codex\Projects\Syndeom-S1.3-reproduction-01
```

The script reads the producer commit from `real-result.json`, retrieves the original v2 gzip,
checks the parent-pinned hash/size, invokes the Rust producer twice with independent external
output directories and compares exact output bytes. It leaves `receipt.json`, logs and source
bytes together in that new destination. Existing work destinations are refused. This repository's
identical public manifest/source map can be reused; conflicting bytes are refused.

For an offline replay, add `--source <path-to-original-1209.2022v2.tex.gz>`.
The source is still hash-checked; this option does not claim a fresh primary retrieval.
For an explicit producer commit, add
`--engine-commit 49e47237924eddf0c39c24aefad0eb9aa02ee8b3`.
The CLI rejects a commit whose tracked producer/library/lock/toolchain inputs differ from
the checkout. Pin Lean 4.34.1 / Mathlib and Rust 1.91.0 exactly as the repository does.
Python orchestrates retrieval and comparison only; canonical outputs come from Rust.

## Commands actually executed

All commands below ran in the isolated local checkout
`C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical`, on branch
`codex/s1-3-canonical-derivation`. Detailed real-run argument arrays and exit codes are in
`real-result.json`. Terminal transcripts in this directory are UTF-8/LF with terminal
CRLF and excess final blank lines normalized; original captures remain in the external work
directory `C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical-work`.

```powershell
# Orientation, using Git Bash (not the machine's WSL bash):
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
# Initial no-cache build was interrupted to prepare pinned dependencies:
# from lean/
lake exe cache get
# Then the orientation gate above was run to completion.

cargo fmt --manifest-path engine/Cargo.toml --all
cargo clippy --manifest-path engine/Cargo.toml --locked --all-targets --all-features -- -D warnings
cargo test --manifest-path engine/Cargo.toml --locked --all-targets --all-features
cargo metadata --manifest-path engine/Cargo.toml --locked --format-version 1
python -I -X utf8 scripts/check_registers.py .
git diff --cached --check

# Only after the tested producer commit existed:
python -I -X utf8 scripts/reproduce_s1_3.py --work-dir C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical-work\real-run --engine-commit 49e47237924eddf0c39c24aefad0eb9aa02ee8b3

# Final staged artifact/evidence tree:
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

`gate-orientation.txt` records baseline Lean/Rust code passing with 29 tests and axiom list
`[propext]`; documentation reconciliation was in progress. The new crate was enabled afterward.
`mathlib-cache.txt` records the successful pinned-cache preparation, not a verification gate.
`tests-producer.txt` records 37 passing tests: 24 manifest, 5 ingest, 8 derivation.
`clippy-producer.txt` records the clean warning-denying lint run. The fixed artificial goldens
were cross-checked using Python SHA-256 and sorted compact JSON (ASCII keys, integer-only
configuration; no second canonical authority); see `mechanical-golden.json`.

Development outcomes are not hidden: the initial golden-capture run had seven passing tests
and one intentional placeholder-hash assertion failure before independently checked constants
were installed. The first register pass exposed CRLF in two copied retrieval ledgers; they
were normalized, restaged and passed. A local proof-printing inspection hit Windows cp1252
output encoding; rerunning with `python -I -X utf8` completed the inspection. None of those
attempts is represented as a passing verification.

## Evidence and limits

- `retrieval.json` / `dependency-retrieval.json`: fresh primary accesses, URLs, timestamps,
  response hashes. Historical source entries remain, with an explicit dated rereading addendum.
- `dependencies.json`: all 26 original lockfile packages unchanged; six decompression
  dependencies plus the new workspace crate produce 33 entries. Toolchain pins unchanged.
- `real-result.json`, `real-first.txt`, `real-regenerated.txt`: two successful actual-byte
  validations and identical regenerated proof/context/manifest/source-map bytes.
- `baseline-preservation.json`: 54 protected baseline files compared byte-for-byte, including
  all 31 raw manifests, the committed raw archive, harvest log, existing Rust crates/goldens,
  Lean inputs, constitution, taxonomy, claims and Rust pin. Both changed parent RECORDs retain
  their complete historical contents as prefixes and add dated reviews.
- Child RECORD: written source-fidelity and location review. The proof is 770 bytes,
  75 format-defined tokens, one proof step, zero lemma steps. Instance 1's statement-wide
  FLAW-0012 maps to step 0 `[0,79)`, without inventing a proof-step attribution.

The known first raw item `s1_ingest-v0.1.0-63e23a952084` is held for canonical promotion.
Its historical source-document falsity attribution needs a reviewed evidentiary resolution;
no alternative truth label was inferred and its original manifest was preserved.
The supported Galindo example completes the engineering slice independently of that hold.
There is no new admission, extra independent sample, positive informal–formal pair, class
coverage conclusion, generator, detector, cold-start certification or PROVEN result here.


## Concurrent documentation update

Before publication, a fresh remote check found public main had advanced to
`be7d1445edf942821716b2d5c0d7b02c0d9f2257` (PR 1, institutional README). Its changes are
README, STATE and HANDOFF only; no engine or proof inputs changed. This branch preserves the
original implementation baseline and immutable producer commit. The draft PR identifies the
STATE/HANDOFF recording overlap; the owner's no-merge instruction is respected. The separately
published README/history must be retained when the owner integrates the branches.


The documented default-commit command was also actually executed, using
`--work-dir C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical-work\documented-command-check`.
It selected the producer commit from the retained receipt, fetched matching original bytes,
ran Rust twice successfully and matched the existing public metadata. Its separate receipt is
`documented-command.json`; the first real-run receipt was not changed.


Final gate: **GATE: GREEN, exit 0 at 2026-10-20 03:47:27Z** (`gate-final.txt`).
Lean: 8,926 jobs; existing golden axioms `[propext]`. Rust: fmt/clippy clean, all 37 tests
passed. Byte/register checks: 147 tracked files; Q 17, SD 17, FLAW 14, CLAIM 7, S- 66,
no undefined references. The new log and final recording edits were then checked with
`python -I -X utf8 scripts/check_registers.py .` and `git diff --cached --check`; no
producer or artifact input changed after the gate. This is an informed working-session gate,
not the Gate S1 cold-start exam and not a Lean verification of the source proof.

## PR 2 review correction and integration verification

On 2026-10-20 the owner explicitly requested the supplied chronology correction, a complete
gate, and merging [PR 2](https://github.com/Quigles1337/Syndeom/pull/2). This supersedes the
earlier no-merge boundary recorded above; those historical paragraphs remain unchanged.
The reviewed merge was `64ed84ef16df982468cd386a5b2803e1a022e8ef`, with main at
`be7d1445edf942821716b2d5c0d7b02c0d9f2257`.

Applied the supplied patch to put Session 21 before S1.3 and restore STATE's continuation
indentation. HANDOFF now begins with main's exact 235,705 bytes, followed by the original
9,269-byte S1.3 block; the last NEXT is S1.2. The new review note follows both blocks.
STATE's Session 20 label is explicitly historical. README now describes the derivation
producer, one existing-parent canonical child, reproduction instructions and current paths.
Its three diagrams are unchanged; 58 relative links and nine anchors resolve.

[`pr2-preservation.json`](pr2-preservation.json) records exact preservation of 144 existing
files against `d2a33062435b5fbc21cb1562b2a33b5a8c0ca20e`, including production/proof inputs,
dependency pins, source records, recipes and retained verification receipts/logs. This README's
original bytes are also preserved as a prefix. No real artifact was regenerated for this fix.

The correction-tree gate passed at **2026-10-20 04:26:20Z**, exit 0 / **GATE: GREEN**:
8,926 Lean jobs, golden axioms `[propext]`, clean fmt/clippy, 37 tests (24 + 5 + 8),
149 tracked files, and coherent Q/SD/FLAW/CLAIM/S- registers (17/17/14/7/66).
[`gate-pr2-review.txt`](gate-pr2-review.txt) records that run's checked index tree and transcript.
It is separate from the original `gate-final.txt`. Raw terminal CRLF is normalized to LF in
the committed copy; the original capture remains outside the repository.

Commands executed for the correction (from the repository root, PowerShell):

```powershell
$reviewWork = 'C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical-work\pr2-fix-2026-10-20'
git apply --check "$reviewWork\pr2-fix.patch"
git apply "$reviewWork\pr2-fix.patch"
python -I -X utf8 "$reviewWork\verify_pr2.py"
python -I -X utf8 scripts/check_registers.py .
git diff --cached --check
python -I -X utf8 "$reviewWork\run_gate.py" corrected
```

The local capture wrapper executes exactly
`C:\Program Files\Git\bin\bash.exe scripts/check.sh`, records the actual HEAD and staged tree,
and refuses an existing log destination. To reproduce the gate from any checkout:

```powershell
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

The final recording commit is checked once more with the same full gate before push and merge,
using `python -I -X utf8 "$reviewWork\run_gate.py" final-commit`. Its clean revision/tree,
actual completion time, exit code and raw-log SHA-256 are recorded in PR 2 and the integration
merge message. The separate local files are `gate-final-commit.log` and
`gate-final-commit.json` under `$reviewWork`. This keeps the final checked file tree fixed
while preserving its verification result. Gate S1 remains open; the next task remains S1.2.
