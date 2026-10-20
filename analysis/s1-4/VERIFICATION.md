# S1.4 verification record — 2026-10-20 UTC

The retained report describes the fixed corpus at
`e6a7cae4392dd1565ee57f9fb727cc01e453687d`, tree
`2e4d454048ac916a0047bd908ae3051ba1b981af`. Remote inspection confirmed the owner's
PR 4 merge at 18:10:13Z, an initially clean worktree, no open PRs, and successful main
CI [39472403912](https://github.com/Quigles1337/Syndeom/actions/runs/39472403912).
The PR head `0b30d782163ed961262dbf70328dc1d16cf58746` has that same tree.

The reporting implementation and fixed configuration were committed before generation:
initial implementation `9375af283e748e915ffc6e31df12f1bb81122e1d`, followed by the
reporting correction `4e46564ac62bc2bd7a5bef54265e9b2dd26772bb`, tree
`03d9d65ad664b665cbfc4df23fce3645b0b624d9`. The latter is the **actual final reporting
tool revision**. Its own input closure is distinct from the unchanged pair producer
`eb1b96d5b8084ceb1de734fffc90c9b480ec32bf` and from the later report/documentation commit.

## Deterministic report execution

[execution-final.json](evidence/execution-final.json) records the two final Rust invocations
from `engine/`, actual arguments, Rust 1.91.0, exits and log hashes. The orchestration ran
from **20:25:24.283682Z to 20:25:44.186035Z**. Both fresh destinations succeeded and all
four deterministic artifacts were byte-identical. The committed `report/` files are exact
copies of `production-02/report-a`; timestamps and machine-local paths remain in the
separate execution receipt. Reusing `report-a` exited **2**, as required, without changing
its files.

| Artifact | SHA-256 in both fresh outputs and the retained report |
|---|---|
| `coverage.json` | `394003309e58e716c2b39882662a008936194420ac2a5d6bce5701961945fa02` |
| `inventory.json` | `882c05d0cb5207beebb9f94b583dbfa9080dca0a0d34832b75ad96be838e33ec` |
| `provenance.json` | `2364ffc77ccdf275b13300331348ce96033d5fa7f89695b2468ccc0277d162f9` |
| `report.md` | `38c02f7d411ec6e1d95049b94788cdfa10fe7ff68e24513973050fd4ba945492` |

The exact command, from the repository root, was:

```powershell
python -I -X utf8 analysis/s1-4/reproduce.py --repo C:\Users\LEET\Documents\ChatGPT\Syndeom --corpus e6a7cae4392dd1565ee57f9fb727cc01e453687d --tool 4e46564ac62bc2bd7a5bef54265e9b2dd26772bb --out C:\Users\LEET\Codex\Temp\syndeom-s1-4-20261020\production-02
```

This wrapper delegates all labels, counts, lineage, bins and rendering to the Rust example.
Rust verifies every frozen corpus hash, the selected input set, the committed tool closure,
compiled core sources and all 20 pair inputs. It uses the manifest library's typed validation
and parent/child API. Locally committed original/alias bytes are checked; externally stored
proof bytes are explicitly **not newly checked by the inventory**. Prior evidence remains
linked. Missing length is not substituted with zero.

The earlier [execution-receipt.json](evidence/execution-receipt.json) retains `production-01`
at tool `9375af2`. Its fresh report pair agreed, but subsequent review found that the
proof-only step summary counted the known statement-only selection as missing. Tool
`4e46564` now reports 30 missing lengths and one separate role exclusion, with an integration
assertion; the same correction also makes status/admission evidence explicit in the source
matrix. All final report files were regenerated after that commit. The earlier receipt,
logs and external output directories are preserved; their output hashes are not advertised
as the final report's hashes.

## Pair compatibility and historical preservation

`production-01` also executed a fresh pair build in an empty Cargo target directory and
one archive/canonical replay into new destinations. All commands exited **0**. Five
generated metadata files matched the pinned Git blobs; six external files matched the
retained originals: **11 files identical**. The source and original external directories
were unchanged. The wrapper supplies the original producer revision, and the unchanged
pair CLI performs its provenance and actual-byte checks. Receipts:
[archive](evidence/pair-archive.json), [canonical](evidence/pair-canonical.json), and
the [execution command/log index](evidence/execution-receipt.json).

The full wrapper run lasted **20:11:26.520573Z–20:12:26.012375Z**. Its retained inputs were
`C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\sources`
and the sibling `external` directory. No historical output was overwritten. The final
reporting correction changes neither the 20-file producer closure nor the reproduction
wrapper, so this compatibility evidence remains applicable. This is a compatibility replay,
not a new corpus admission or repeat of the full S1.2 mutation suite. No new emitted-pair
Lean audit was run; the original admitted pair's type/axiom audit remains historical evidence.

[preservation.json](evidence/preservation.json) records the baseline comparison performed
after report/document reconciliation. It verifies **279 unchanged baseline files**, five
exact-prefix append-only documents, and all **20 frozen pair inputs**. The only permitted
current-summary replacements are named in the helper and receipt. Historical HANDOFF and
SOURCES bytes, old manifests/proofs/configurations, source labels, receipts and logs are
preserved. The same helper must pass again at the final commit in a new external receipt.

The [retention ledger](evidence/retention-ledger.json) records source/destination hashes
and any display transformation. Original raw logs remain outside the repository; committed
text copies use LF, mark empty stdout when necessary, and ensure a terminal newline.

## Tests, full gates and failed development checks

The integration target contains **16 tests**, exercised by ordinary workspace testing.
They cover root/child deduplication, repeated flaw instances, missing/inconsistent parents,
cycles, unknown taxonomy IDs, null truth, held status, genuine zero versus missing length,
overlapping classes, controls, modality/role separation, bin edges and empty bins, input
mutation/refusal, actual deterministic aggregation, and existing-output preservation.
Existing manifest checks are delegated to the library. No assertion hardcodes the real
32-item inventory as a substitute for these cases.

The first full implementation gate used:

```powershell
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

It ran on `9375af283e748e915ffc6e31df12f1bb81122e1d`, tree
`96cf29f99656b039561170cc9356d0839f97a6ef`, from **20:11:20.569576Z to
20:23:14.733002Z**, with the tracked implementation unchanged. It exited **0 / GATE: GREEN**:
Lean 8,926 jobs; golden fixture axioms `[propext]`; fmt and all-target/all-feature clippy;
**67 Rust tests** (24 manifest, 16 coverage, five ingest, eight derive, 14 pair); LF and
register checks, then 301 tracked files. The example is built by `--all-targets`, and its
integration tests execute. The [receipt](evidence/gate-implementation.json) and
[log](evidence/gate-implementation.txt) retain the actual evidence; the original raw-log
SHA-256 is `8eaa2870f6e4a8bef6cfcea7720d2340039d746dd2403855a6cc556bc6a9890a`.
After the reporting correction, the 16 focused tests and all-target clippy passed again.
This earlier gate does not certify the final report/documentation tree.

The initial orientation attempt is retained as a **failure**, not a clean-base certificate:
19:56:54.626658Z–20:08:08.755887Z, exit 1. The Lean build/golden audit passed, then fmt
rejected new reporting drafts being edited during that run. See its
[receipt](evidence/gate-orientation.json) and [log](evidence/gate-orientation.txt).
A development compile also caught an E0382 moved-string borrow in the renderer; it was
fixed before the implementation commit and subsequent successful checks. No failed check
is silently relabelled successful.

The staged byte/register preflight found CRLF in two newly copied gate JSON receipts.
Only those copies were normalized to LF, with unchanged JSON values and original files
retained. Their distinct source/destination hashes and transformation are in the ledger.
The byte/register preflight was rerun after this correction.

## Final committed-tree convention

After committing this complete packet, run the full gate on that **clean final commit**,
with no subsequent source or documentation edits. Retain the exact commit, tree, clean
worktree/index status before and after, actual command, UTC start/end, exit and raw log
outside the repository under:

`C:\Users\LEET\Codex\Temp\syndeom-s1-4-20261020\gate-final.json` and `gate-final.log`.

The draft PR description and task completion record report that actual result and the
matching hosted CI. This follows the existing evidence convention: a file cannot name
its own future commit, and another evidence commit would create another tree to check.
Do not substitute an earlier gate or anticipated result. The final preservation check
and retained-report hash comparison are also recorded externally. Keep the PR draft and
unmerged. Gate S1 remains open for the [documented obligations](OBLIGATIONS.md), including
the independent later cold-start session.
