# Integration verification record

Owner decision: 2026-10-20 America/New_York. Execution timestamps below use UTC.
The accepted baseline is `37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f`, tree
`aadce1d6332ad3a43f7f31b6779ee52083287215`. PRs 4, 5 and 6 were merged; PR 6
merged at 2026-10-21T01:55:53Z. The integration branch was created clean from that main.

## Commands actually completed before the implementation commit

Each `evidence/<name>.json` records exact argv, head/tree, working state, actual UTC
start/end, exit code and raw-log SHA-256. Exact raw log bytes are retained in
`<name>.log.gz`; readable `.log` copies obey LF/trailing-newline repository policy.
`evidence/transcript-copies.json` binds both hashes. Receipt hashes are unchanged.

| Evidence | Actual command | Outcome |
|---|---|---|
| `base-gate` | Git Bash `scripts/check.sh` | Clean accepted baseline; exit 0, GATE: GREEN at 2026-10-21T02:57:48.435698Z; Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 67 Rust tests and 375-file/register checks |
| `focused-01` | `cargo test --locked --manifest-path engine/Cargo.toml -p syndeom-ingest --test first_parent_amendment` | Failed development run: 12 passed, 3 failed; checker incorrectly compared the ingest input's top-level notes as part of hashed manifest config |
| `focused-02` | Same focused command after following the existing ingest notes/config split | Exit 0; all 15 tests pass |
| `clippy-01` | `cargo clippy --locked --manifest-path engine/Cargo.toml --all-targets --all-features -- -D warnings` | Failed development run: one collapsible nested `if`, reported for example and test targets |
| `clippy-02` | Same lint command after the indicated let-chain correction | Exit 0; all workspace targets pass |

`cargo fmt --all --manifest-path engine/Cargo.toml` and `git diff --check` also
completed after the initial diff check identified a trailing blank line in a new
transcript. Transcript packaging preserves raw bytes in gzip and records its readable
LF conversion; no earlier receipt is regenerated. No old artifact or evidence byte was changed to satisfy a check. Two
local editing guards rejected commands before writing (duplicate staged patch target;
formatted-source string mismatch); neither was a verification pass or repository edit.

The 15 focused tests use controlled in-memory mutations of real retained inputs,
never corpus admissions: exact 35/32 counting and stable key; unchanged older
issues/exclusions; missing, inconsistent, duplicate or multiple effective maps;
witness and owner-decision binding; ID/hash/producing revision binding; disagreeing
status fields; candidate-note rewrite refusal; source-byte mismatch; missing old/new
objects; duplicate raw representation refusal; historical object/packet preservation;
append-only prefix integrity; and acceptance without the ADR/history record.

The successful focused run is a development check, not an exact-final-tree gate.
The implementation is committed before the Rust CLI generates the retained inventory.
Subsequent integration/replay/preservation evidence is recorded below after execution.
The exact final commit receives its own full gate externally, published in the draft PR
without editing that checked tree merely to embed its own commit hash.


## Actual committed integration and preservation

Implementation commit: `c87c827a27ce40af7e08101d452ba5532704c9c5`, tree
`c874c8d7bd4af341d135e7d47db80258f1cfc9a2`. The clean checkout generated the retained
`report/inventory.json` and `report/provenance.json` before this recording commit.
`evidence/integration-01/receipt.json` records the exact reproduction command and
three subprocesses: report-a exit 0, report-b exit 0, existing-output refusal exit 1.
The independent outputs were byte-identical and refusal preserved the first outputs.
Retained inventory SHA-256: `b626d1b53f50a23ee5eae9e99fc6ea387bb83f51c807d843d68a29dd1651b803`.
Retained provenance SHA-256: `12f4706af2b4bdbf861bc917c6d50d7aabb78b3297bf556f4e4a77c3bed2f4bc`.

Executed from the repository root:

```powershell
python -I -X utf8 analysis/s1-first-parent-integration/reproduce.py --repo C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical --out C:\Users\LEET\Codex\Temp\syndeom-first-parent-integration-20261020\integration-01
python -I -X utf8 analysis/s1-4/reproduce.py --repo C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical --corpus e6a7cae4392dd1565ee57f9fb727cc01e453687d --tool 4e46564ac62bc2bd7a5bef54265e9b2dd26772bb --out C:\Users\LEET\Codex\Temp\syndeom-first-parent-integration-20261020\frozen-s1-4-replay-01
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

The final command here was run on the clean implementation commit by the external
capture helper: exit 0 / GATE: GREEN, finished **2026-10-21T03:18:43.794933+00:00**. Lean built
8,926 jobs; golden axioms `[propext]`; fmt/clippy and all 82 Rust tests passed
(67 previous + 15 focused integration tests); 403 tracked files and register checks
passed. Exact log/receipt: `evidence/implementation-gate/`. This green integration
satisfies the accepted status-support conditions on the branch, with no effect on
main until review and merge. The later recording commit is gated separately below.

Rust's actual current counts are **35 archived representations / 32 effective
logical items**: 33 raw, two canonical; 46 flaw instances, five observed classes,
nine zero classes; statement truth 5 true / 17 false / 10 undetermined, offered-proof
status 1 valid / 31 invalid. The unflagged subset is 18. Truth-dimension-qualified
counts are 5 true / 10 false, with seven truth-review items and ten undetermined
items excluded. These are metadata qualifications, not new scoring readiness.
Thirty logical items still lack canonical representations. The old logical key
has exactly one effective status, supplied by `s1_ingest-v0.1.0-15ab865466d6`.

Rust verified the full 375-file accepted-baseline list against actual Git objects,
then 370 exact historical files and three exact append-only prefixes (README and
STATE are the two allowed current summaries). This includes **all 35 S1.4 files,
all 47 accepted packet files**, all historical groundtruth objects, original
producing revisions and pins. All **20 frozen pair inputs** also match producer
`eb1b96d5b8084ceb1de734fffc90c9b480ec32bf`; their hashes are in provenance.
The new manifest hash is `2fbb965ffe92a8d0f102081adc43d84d56182fba5f4fee2bbd7c8dbfd080f8be`;
config hash `15ab865466d67dfd18ed487b4e51a3dc1719b07a00b89b3627e92dc111c95364`;
source hash `5987d8106ae69669b10c876a0915d5ad52ce44a814ea7fe63600c1737642cca8`.
Available committed artifact bytes were checked with `Manifest::check`; absent
external/regenerable originals are explicitly disclosed, not newly verified.

The unchanged historical S1.4 wrapper produced two matching fresh reports and
refused an existing output (expected exit 2). All four outputs also match the retained
S1.4 report byte-for-byte; `evidence/frozen-s1-4-replay-01/` records commands and
the comparison. No frozen report, bin or receipt was replaced. No optional pair
regeneration was requested; its input closure and existing full gate were checked.

## Final committed-tree evidence

After the recording commit, run the full gate and the new Rust reproduction again
on that exact clean commit, in fresh external destinations. The final inventory
must equal the retained inventory; provenance changes to identify the final checkout.
The external receipts and draft PR record the exact head/tree and actual results
without a self-referential new commit. The final gate is not replaced by the earlier
implementation gate. External evidence root:
`C:\Users\LEET\Codex\Temp\syndeom-first-parent-integration-20261020`.

The owner ruling is accepted; branch integration is verified; default-branch
activation awaits reviewed merge. Canonical production still needs its selection,
context and locator review. No schema, detector, harvest, witness reinvestigation,
new source retrieval, new mathematical label for another item, or cold-start exam
was performed. Gate S1 remains open in this informed session.
