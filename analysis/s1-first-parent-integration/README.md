# Accepted first-parent integration event

This later event implements the owner's 2026-10-20 America/New_York acceptance of
the exact ruling at `37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f`. See the actual
[owner decision](OWNER-DECISION.md), appended ADR-0003 amendment and HANDOFF decision
event. The complete earlier `analysis/s1-first-parent-status/` packet, including its
proposed map and candidate notes, remains historical and unchanged.

The [effective map](../../groundtruth/amendment-map.json) is one explicit case:

| Role | Artifact ID |
|---|---|
| Stable logical-item key; retained historical representation | `s1_ingest-v0.1.0-63e23a952084` |
| Single effective status representation after validation | `s1_ingest-v0.1.0-15ab865466d6` |

This is a raw metadata amendment, not `derived_from` or a new independent example.
The accepted candidate is copied byte-for-byte to `groundtruth/`; its original
producing revision, notes, config hash and artifact ID are preserved. Its original
“PROPOSED ONLY” notes are dated producer output, not the current policy decision.
The new RECORD, accepted owner decision and effective map establish subsequent use.

Both status fields use the exact hash-bound executor-written counterexample. S-0025
still supplies the author-documented error/locator. The D3/SD12 exception applies
only here. Scoped Lean support does not formalize the complete analytic bridge from
the manuscript; the two written checks are same-executor. This event makes no PROVEN
transition and performs no new witness investigation.

## Current consumer and preservation

`engine/ingest/examples/s1_effective_inventory.rs` is a Rust metadata consumer,
automatically built by the existing workspace gate. It uses the existing manifest
validator and frozen S1.4 lineage/qualification functions without editing them.
It has one fixed mapping, no generalized amendment registry or schema change.
Missing/inconsistent mapping, changed witness/decision, identity or status ambiguity,
unexpected population and historical-byte changes are errors, never fallback to
directory counting. It emits one item with one status for the stable logical key.
The other canonical children remain representations of their original logical roots.

`preservation-inputs.json` records SHA-256 and byte length for every file at the
accepted baseline. The Rust CLI independently matches this entire list against the
actual Git objects at that commit before checking current preservation. All old
objects, the complete S1.4 and accepted candidate packets, original source contexts,
mathematics, receipts, first-write bins and dependency pins remain exact. Only README
and STATE are reconciled; HANDOFF, ADR-0003 and groundtruth README permit exact-prefix
appends. New evidence stays under this event. All 20 pair inputs are also compared
to the original producing revision `eb1b96d5b8084ceb1de734fffc90c9b480ec32bf`.

The amended inventory has 35 archived representations (33 raw, two canonical) and
32 effective logical items. Use `items[].logical_item_key` for counting and
`items[].effective_artifact_id` for status selection; never count raw directories or
union old and new status values. Canonical production still requires separate
selection, source-context and locator review after resolving the effective parent.
The checker refuses an unreviewed canonical child of this amended item.

The original S1.4 tool remains a historical consumer pinned to its original corpus
and tool revisions. Its 34/32 counts, 17-item unflagged subset, hold and first-write
bins remain true of that frozen snapshot. This event's effective inventory is a
separate output, not a regenerated S1.4 result. The seven older truth questions,
seven severity questions and unnamed-result question keep their original dispositions
and exclusions. Raw missing lengths are not zero lengths; 30 logical items still
lack canonical representations. No new scoring eligibility or detector result is
asserted by this metadata check.

## Reproduction

Use a clean checkout of the integration commit with the accepted baseline and original
pair producing commit available in Git history. The compiled source and all committed
inputs are bound to the reported checkout revision/tree. No network retrieval is
performed. Choose a fresh output directory outside the checkout with an existing
parent. From the repository root:

```powershell
python -I -X utf8 analysis/s1-first-parent-integration/reproduce.py --repo C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical --out C:\Users\LEET\Codex\Temp\syndeom-first-parent-integration-replay-new
```

This runs the Rust consumer twice into new destinations, compares both JSON outputs
byte-for-byte, and verifies an attempted overwrite fails without changing them.
Python records commands, exits, UTC times and hashes; all metadata and counts come
from Rust. The receipt and logs stay together in the new destination. Direct command:

```powershell
cargo run --locked --manifest-path engine/Cargo.toml -p syndeom-ingest --example s1_effective_inventory -- C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical C:\Users\LEET\Codex\Temp\syndeom-first-parent-effective-new
```

Focused checks and the full gate:

```powershell
cargo test --locked --manifest-path engine/Cargo.toml -p syndeom-ingest --test first_parent_amendment
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

The focused tests use controlled in-memory mutations of actual archived inputs; they
are negative fixtures, not newly generated evidence about the mathematics. CLI reports
read committed Git objects, validate available committed artifact bytes and disclose
external/regenerable files whose original bytes are absent. No pair generation, new
canonical production, harvest or witness reinvestigation occurs here.

## Decision, branch integration and main

Owner acceptance is already recorded. Verified integration discharges this case's
status-support hold only in the tree that passes the conditions. Main acquires the
effective replacement only through reviewed merge of this integration; the requested
PR remains draft and unmerged. Actual checks and retained report revisions are recorded
in `VERIFICATION.md` after execution. The final clean committed tree is gated separately;
its exact head/tree, actual log and receipt are retained outside the repository and
published in the draft PR without another commit to embed its own hash.

Gate S1 remains open in this informed session. After reviewed merge, assess readiness
for a separate genuine cold-start examination using constitution §6's Session-N
resume prompt verbatim and alone. No cold-start result is claimed here.
