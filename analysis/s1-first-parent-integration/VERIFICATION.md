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
