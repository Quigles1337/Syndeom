# S1.4 — Existing-prefix coverage and interim power

This packet inventories the corpus after the owner merged PR 4 at 2026-10-20 18:10:13 UTC.
The corpus snapshot is `e6a7cae4392dd1565ee57f9fb727cc01e453687d`, tree
`2e4d454048ac916a0047bd908ae3051ba1b981af`. Its PR head had the same tree. The verified
merge metadata is in [config/merge-evidence.json](config/merge-evidence.json); the observed
main-push CI [run 39472403912](https://github.com/Quigles1337/Syndeom/actions/runs/39472403912)
completed successfully. Those historical checks are distinct from this packet's final gate.

Read the [generated report](report/report.md), [machine-readable coverage](report/coverage.json),
[per-representation and logical-item inventory](report/inventory.json),
[input provenance](report/provenance.json), and [remaining obligations](OBLIGATIONS.md).
Report outputs are descriptive analysis artifacts, not synthetic corpus artifacts and not
CLAIM promotions. Gate S1 remains open.

## Authority and counting

[`s1_4_coverage.rs`](../../engine/manifest/examples/s1_4_coverage.rs) is a versioned Rust example
target using the existing `syndeom-manifest` library and existing dependencies. It reads Git
blobs at the pinned corpus commit, parses typed manifests, invokes `Manifest::check` with
whatever committed bytes are actually available, validates lineage using `check_derived`,
and performs all counting and deterministic JSON/Markdown rendering in Rust. Python only
orchestrates commands and compares output bytes. No new crate, Cargo edit or library-source
edit is required; the historical pair producer's immediate-source-directory guard remains intact.

The [frozen corpus input list](config/corpus-inputs.json) includes all tracked groundtruth
files, source/taxonomy/decision/state/handoff evidence and retained S1.2/S1.3 packets. It is
verified against the complete selected Git-tree input set, not a mutable filesystem glob.
The separate [tool closure](tool-inputs.txt) includes implementation, integration tests,
configs, orchestrator and existing library/dependency pins. Every current tool input must
equal its declared committed blob; compiled core source bytes are checked too. Unexpected
example/config inputs, changed/missing hashes, unknown classes, ambiguous canonical
selection and invalid lineage are refused. The recorded corpus revision is not silently
advanced when a new branch changes documentation.

The [review overlay](config/dispositions.json) explicitly carries the first-parent hold,
the seven historical truth questions, and seven severity questions. Those are sourced
review inputs, not labels guessed by a Markdown keyword classifier. Historical admission,
stored truth/proof-validity and later analysis eligibility are separate. GitHub records
the owner merge but no separate reviewer approval; the existing pair's full checks and
written correspondence are retained evidence, not newly executed checks or PROVEN promotion.

## First-write frozen bins

Rust's `freeze` command produced [bins.json](config/bins.json) from the observed canonical
metadata before the implementation commit and before the final report. All canonical
informal files define one grid: **[7,14), [14,28), [28,56), [56,112)**, including empty bins.
Within it, statement-only and proof-bearing selections are counted separately. The independent
formal grid is **[14,28)**. Both use lower-inclusive/upper-exclusive edges and explicit
underflow/overflow buckets. These boundaries remain fixed, including if later work introduces
another prefix. Raw null counts are missing, never zero or estimates from archive bytes.
The formal theorem alias is a file-size measurement, not the transitive Mathlib proof size.

The one-time command, executed from `engine/`, was:

```powershell
cargo run --locked -p syndeom-manifest --example s1_4_coverage -- freeze C:\Users\LEET\Documents\ChatGPT\Syndeom e6a7cae4392dd1565ee57f9fb727cc01e453687d C:\Users\LEET\Codex\Temp\syndeom-s1-4-20261020\first-freeze
```

Do not run `freeze` to recenter this report's bins. Its outputs were committed as configuration.

## Reproduce

Use a checkout containing the reporting implementation. The actual tool revision is recorded
in `report/provenance.json` after it exists; implementation and fixed configs are committed
before the retained report is generated. From `engine/`, substitute that full revision and
a new output directory (its parent must exist):

```powershell
cargo run --locked -p syndeom-manifest --example s1_4_coverage -- report C:\Users\LEET\Documents\ChatGPT\Syndeom e6a7cae4392dd1565ee57f9fb727cc01e453687d 4e46564ac62bc2bd7a5bef54265e9b2dd26772bb C:\Users\LEET\Codex\Temp\syndeom-s1-4-report-new
```

For two independent destinations, byte comparison and an existing-output refusal check,
run from the repository root:

```powershell
python -I -X utf8 analysis/s1-4/reproduce.py --repo C:\Users\LEET\Documents\ChatGPT\Syndeom --corpus e6a7cae4392dd1565ee57f9fb727cc01e453687d --tool 4e46564ac62bc2bd7a5bef54265e9b2dd26772bb --out C:\Users\LEET\Codex\Temp\syndeom-s1-4-reproduction-new
```

The orchestrator records actual argv, exit codes, compiler version, run times, and log/output
hashes outside the deterministic artifacts. It refuses an existing output directory. No
network request or external proof-byte verification occurs in the inventory. Both committed
proof objects are checked anew; external metadata retains explicit missingness and links to
prior evidence. The optional compatibility replay below performs actual pair-byte checks
separately and does not retroactively change the inventory's declared verification scope.

To include a **fresh pair build** into an empty Cargo target directory and one historical
archive/canonical replay, append these options using retained exact bytes:

```powershell
--pair-sources C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\sources --pair-retained-external C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\external
```

The unchanged pair CLI verifies all source hashes, the original committed producer closure
and parent/child bytes; the orchestrator compares generated metadata against pinned Git blobs
and external outputs against the retained originals. No historical output is rewritten.
The original five source objects may be retrieved by the established S1.2 reproduction
procedure if they are unavailable locally; no new source or licence choice is authorized here.
This compatibility replay does not rerun the old mutation suite or the emitted-file Lean audit.

## Verification and final-tree convention

Focused integration tests are in `engine/manifest/tests/s1_4_coverage.rs`, so ordinary tests
exercise the report rules. The existing full gate uses `--all-targets --all-features` for
both tests and clippy and therefore also builds the example. No tests are hidden solely
inside an example target. Run from the repository root:

```powershell
cargo test --locked --manifest-path engine/Cargo.toml -p syndeom-manifest --test s1_4_coverage
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

Retained execution evidence belongs under `evidence/`; its index is written only after the
commands run. After the final report/documentation commit, run the complete gate on that
clean commit and retain the exact commit/tree, log, timestamps and exit outside the repository.
Publish that actual result in the draft PR without another edit to the checked tree merely
to embed its own hash. Historical HANDOFF/SOURCES remain exact byte prefixes; corpus labels,
manifests, recipes, proofs, licence decisions and old receipts are preserved.


## Retained results

[VERIFICATION.md](VERIFICATION.md) identifies actual commands, scope, failures and the
final-tree convention. [evidence/execution-final.json](evidence/execution-final.json) records the final two
fresh Rust reports, byte comparison and output refusal. The separate
[initial execution receipt](evidence/execution-receipt.json) retains the fresh historical
pair build/replay and first reporting revision. The committed report is an exact copy of
`production-02/report-a`, identified by the final execution receipt.
The preservation helper compares all baseline files, allowing only named current summaries
and exact-prefix appends, and checks the pair closure separately:

```powershell
python -I -X utf8 analysis/s1-4/check_preservation.py --repo C:\Users\LEET\Documents\ChatGPT\Syndeom --base e6a7cae4392dd1565ee57f9fb727cc01e453687d --output C:\Users\LEET\Codex\Temp\syndeom-s1-4-preservation-new.json
```
