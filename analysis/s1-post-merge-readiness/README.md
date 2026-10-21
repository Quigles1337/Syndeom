# Post-merge activation and S1 readiness event

**Recorded:** 2026-10-21 UTC. **Activation local date:** 2026-10-20 America/New_York.
**Event:** `s1-post-merge-activation-2026-10-21`.
This is an informed working session. Gate S1 remains **OPEN**; no cold-start
examination is performed or claimed here. The [readiness assessment](READINESS.md)
maps each actual S1 requirement to its evidence, limits and remaining examination.

## Acceptance, integration and activation

| Event | Actual evidence |
|---|---|
| Owner acceptance, 2026-10-20 America/New_York | The [hash-bound decision](https://github.com/Quigles1337/Syndeom/blob/1c56028532ea135603dbad5423d5555313bfd9ec/analysis/s1-first-parent-integration/OWNER-DECISION.md) accepts the exact ruling at `37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f`. No second acceptance is needed. |
| Verified branch integration | Implementation `c87c827a27ce40af7e08101d452ba5532704c9c5`, followed by tested final head [a337375dbae49150a1a40fa46f6d1417835aadd2](https://github.com/Quigles1337/Syndeom/commit/a337375dbae49150a1a40fa46f6d1417835aadd2). The original integration packet, including failures, remains unchanged. PR 7 retains its exact-final-head local gate and reproduction receipts. |
| Default-branch activation | [PR 7](https://github.com/Quigles1337/Syndeom/pull/7) merged at **2026-10-21T03:33:35Z**, or **2026-10-20 23:33:35 America/New_York**, creating main [1c56028532ea135603dbad5423d5555313bfd9ec](https://github.com/Quigles1337/Syndeom/commit/1c56028532ea135603dbad5423d5555313bfd9ec). Parents are `37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f` and the tested head. Both head and merge have tree **14b451866491dc4a2b866ec0b1d726d792385916**; their content diff is empty. |
| Applicable checks | [Tested-head workflow 39538331617](https://github.com/Quigles1337/Syndeom/actions/runs/39538331617) and [main-push workflow 39539245635](https://github.com/Quigles1337/Syndeom/actions/runs/39539245635) both concluded success at their respective SHAs. They are advisory; this session also ran the full local gate on clean merged main. |

Fetched main was exactly that merge, with no intervening commits; the existing
checkout was clean. The reconciliation branch starts there. Actual Git commands,
parents/tree comparison and timestamped GitHub responses are retained in
[merge-evidence.json](evidence/merge-evidence.json), [pr7.json](evidence/pr7.json),
[tested-head-ci.json](evidence/tested-head-ci.json) and [main-ci.json](evidence/main-ci.json).
The repo is public; no visibility or policy change is part of this event.

The existing [effective map](../../groundtruth/amendment-map.json) is unchanged:

| Role | ID |
|---|---|
| Stable logical key and retained old representation | `s1_ingest-v0.1.0-63e23a952084` |
| Single effective status representation | `s1_ingest-v0.1.0-15ab865466d6` |

This case's status-support hold is discharged **on verified main**. The relation is
raw metadata amendment, not `derived_from` or a second independent lemma. The exact
executor-written witness supplies statement-truth support under the accepted narrow
D3/SD12 exception. S-0025 remains the author's error/locator evidence. The full
analytic source-to-Lean bridge is unformalized; no PROVEN transition or independent
human/blind validation follows. Canonical selection, context and locator review
remain separate.

Earlier proposed-state wording in the candidate, proposal, owner-decision record,
integration packet and historical S1.4 ledger is preserved as dated evidence.
The inventory's fixed `activation_scope` wording describes its validation rule;
the merge event above supplies the subsequent main activation. No output or helper
was changed merely to substitute current tense or a new producing revision.

## Actual merged-main reproduction

From a clean checkout at the merge, the unchanged Python wrapper invoked the
existing Rust consumer twice and attempted an overwrite:

```powershell
python -I -X utf8 analysis/s1-first-parent-integration/reproduce.py --repo C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical --out C:\Users\LEET\Codex\Temp\syndeom-post-merge-readiness-20261021\main-reproduction
```

Actual Python executable: `C:\Users\LEET\AppData\Local\Programs\Python\Python311\python.exe`.
Execution finished **2026-10-21T03:50:28.787156+00:00**: two successful Rust runs,
byte-identical outputs, then expected refusal exit 1 with outputs unchanged.
The [receipt](evidence/main-reproduction/receipt.json) records exact argv, times,
exits and raw log hashes. Fresh [provenance](evidence/main-reproduction/provenance.json)
identifies checkout `1c56028532ea135603dbad5423d5555313bfd9ec` and the shared tree.

The [comparison](evidence/main-comparison.json) verifies inventory bytes against the
[retained integration inventory](../s1-first-parent-integration/report/inventory.json):

- Inventory SHA-256: `b626d1b53f50a23ee5eae9e99fc6ea387bb83f51c807d843d68a29dd1651b803`.
- Fresh merged-main provenance SHA-256: `ec0d888eac1d9ad8b1187e0cfb754b529cbd7687ce793215a1960dd15972e708`.
- Retained implementation provenance SHA-256: `12f4706af2b4bdbf861bc917c6d50d7aabb78b3297bf556f4e4a77c3bed2f4bc`.

Provenance correctly differs across checkout revisions; inventory bytes remain
identical. The full two-run outputs remain in the fresh external destination above;
this event retains the new receipt, provenance and logs without duplicating the
unchanged inventory or replacing earlier evidence. All authoritative metadata and
counts come from Rust; Python only orchestrates or compares bytes.

| Rust inventory field | Verified merged-main value |
|---|---:|
| Archived representations | 35: 33 raw, two canonical |
| Effective logical items | 32 |
| Unflagged documented subset | 18 |
| Statement truth | 5 true / 17 false / 10 undetermined |
| Qualified truth dimension | 5 true / 10 false |
| Offered-proof status | 1 valid / 31 invalid |
| Flaw instances | 46 |
| Observed / zero-coverage classes | 5 / 9 |
| Logical items without canonical representations | 30 |

The stable key has one effective status, its hold is false, and the original
producing revisions remain. Seven truth questions, seven severity questions and
the unnamed-result question keep their existing dispositions/exclusions. Available
committed artifact bytes are checked. External/regenerable originals that are absent
are explicitly reported, not claimed newly verified; retrieve and verify them
before future scoring.

## Gate, preservation and evidence handling

Full local `bash scripts/check.sh` on clean merged main completed at
**2026-10-21T04:00:41.304180+00:00**, exit 0 / **GATE: GREEN**. Lean built
8,926 jobs; the golden fixture uses `[propext]`; fmt/clippy and all 82 Rust tests
passed; all 428 tracked files passed LF/BOM/trailing-newline and register checks
(Q 17, SD 17, FLAW 14, CLAIM 7, sources 72). HEAD/tree and clean state were unchanged
before/after. [Receipt](evidence/main-gate.json), [readable log](evidence/main-gate.log)
and [lossless raw log](evidence/main-gate.log.gz) are retained separately here.
Raw gate log SHA-256: `c86f844457d1680774ca3c79c7ddc75385247616e4d7aa538a6b8fb67c9a1fd0`.
Actual command: `C:\Program Files\Git\bin\bash.exe scripts/check.sh` from the
repository root, captured by the external helper with exact argv and UTC times.

The unchanged Rust consumer rechecks the 375-file accepted baseline: 370 exact
files and three append-only prefixes, all 35 frozen S1.4 files, all 47 accepted
witness/proposal packet files, and all 20 pair inputs against original producer
`eb1b96d5b8084ceb1de734fffc90c9b480ec32bf`. This includes both raw objects, candidate
manifest/source, owner decision and mapping, source records/licences/taxonomy and
dependency pins. The full frozen S1.4 packet, first-write bins, old receipts and
failed runs are not regenerated.

The reconciliation's separate preservation comparison uses merged main as its base:
only README and STATE may be reconciled; HANDOFF and groundtruth README must retain
their entire previous bytes as exact prefixes; every other baseline file must remain
byte-identical. New files are confined to this event. The original integration packet,
accepted decision, map and earlier proposal are all exact, not merely prefix-preserved.
The exact-final-commit comparison and Rust reproduction are retained externally
with the final full gate. No producer, detector, corpus, source, ADR or pin is edited.

New raw command logs are retained losslessly as `.log.gz`, alongside readable LF
copies; [transcript-copies.json](evidence/transcript-copies.json) binds both hashes.
The receipt's raw-log hash is never rewritten to match the readable copy. The command
capture helper is external and records actual start/end, exits, HEAD/tree and clean
state. All prior records stay untouched. No source retrieval, witness investigation,
canonical production or pair regeneration occurs in this session.

The final committed reconciliation tree must receive its own full gate and existing
Rust reproduction in fresh external destinations. Its exact head/tree, actual
commands, exits, logs, preservation comparison and receipts are recorded in the PR
and outside the tree being certified, following the project's convention. This
avoids a self-referential recording commit. This event's merged-main result never
substitutes for the changed final tree's result.

External session evidence root:
`C:\Users\LEET\Codex\Temp\syndeom-post-merge-readiness-20261021`.
The PR carries temporary draft/review status; current summaries describe completed
activation and the next examination, so merging reconciliation cannot leave another
“merge this PR” next action.

## Reproduce without touching retained evidence

Use a clean committed checkout, an existing external parent and a **new** destination:

```powershell
python -I -X utf8 analysis/s1-first-parent-integration/reproduce.py --repo C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical --out C:\Users\LEET\Codex\Temp\syndeom-post-merge-readiness-replay-new
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

The fresh inventory must match the retained inventory while inputs are unchanged.
Its provenance must identify the checkout actually used. Run the gate on the final
committed tree and retain its result externally; do not commit that result into the
tree it certifies. Once reconciliation reaches main, a **separate fresh session**
should attempt Gate S1's cold-start examination under §2/SD15 using §6's Session-N
resume prompt verbatim and alone. Readiness is not closure; Gate S1 remains OPEN.
