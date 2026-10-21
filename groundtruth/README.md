# groundtruth/ — real proofs with documented status (S1)

Layout per ADR-0001 D2: `groundtruth/<artifact_id>/manifest.json` plus the stored files the
manifest names. Every item has `origin: "ground-truth"` and a derivation of
`documented-by-source` (errata, retractions) or `formalized-in-mathlib` (informal–formal pairs);
never `by-construction` (SD12). Each item records its source, licence, and the document that
establishes its status (`config.sources`, `config.licence`, `config.status`), all registered in
`PROGRAM/SOURCES.md` before ingestion (SD16).

Items arrive as `raw/1` and are scored through derived canonical artifacts whose location
re-expressions are argued in writing in the S1.3 record (ADR-0001 D4 convention); the `raw/1`
item remains the item of record. Nothing synthetic is ever placed here.

Raw-parent procedure: ADR-0003 (the errata-and-retraction harvest). Each raw item is written by the
deterministic producer `syndeom-ingest` (`engine/ingest/`, `generator.id` `s1_ingest`; ADR-0003
D6) from a per-item config and the original's bytes — never by hand — and carries `RECORD.md`,
the written record of provenance, the notice's locating sentences, the diff summary and the
FLAWS.md class argument (ADR-0003 D4). `HARVEST-LOG.md` beside this file is the append-only log
of every candidate examined, with its verdict and the last candidate processed (ADR-0003 D7, D8).
Manifests carry `syndeom-manifest/2` (ADR-0001 as amended 2026-10-13). The current prefix
contains 31 raw parents; the log records the counts and resume point.

Canonical procedure: [ADR-0006](../PROGRAM/DECISIONS/ADR-0006.md) and
[`syndeom-derive`](../engine/derive/README.md), version 0.1.0. Reviewed recipes live under
`derivations/`; manifests and source maps live under their child IDs. Source text and the
complete required context remain external for this first example. A child is another
representation of its parent and must not be counted as another independent real item.
Promotion holds in parent records must be resolved before using those parents for derivation.
The S1.3 evidence and single-command reproduction are in `analysis/s1-3/`.


## Post-merge inventory — 2026-10-20

The Route A prefix above still holds 31 logical items. Owner merge of PR 4 added one
Judson/Mathlib positive pair through the separate ADR-0007 route: main now has 32 raw
parents and two canonical children, representing 32 logical items. The canonical pair
selects a zero-step informal statement and a formal theorem alias. The S1.4
[coverage report](../analysis/s1-4/report/report.md) distinguishes archive, qualified,
truth-dependent and length-bearing populations, with the first-parent hold retained.
Gate S1 remains open; [remaining obligations](../analysis/s1-4/OBLIGATIONS.md).


## Accepted first-parent amendment — 2026-10-20 America/New_York

The owner accepted the exact PR 6 ruling; see the [actual decision](../analysis/s1-first-parent-integration/OWNER-DECISION.md)
and appended ADR-0003 amendment. The one-case [effective map](amendment-map.json)
keeps `s1_ingest-v0.1.0-63e23a952084` as the logical key and selects
`s1_ingest-v0.1.0-15ab865466d6` for effective status after validation. The latter
is the retained candidate copied byte-for-byte, including its historical proposed
notes and original producing revision. The old raw object is preserved, so directory
counts are not logical-item counts. This relation is not a canonical-child edge.

Use the [Rust effective inventory](../analysis/s1-first-parent-integration/README.md)
for the amended tree: 35 archived representations (33 raw plus two canonical),
32 effective logical items. It refuses a missing or inconsistent map and validates
historical preservation before counting. Its effective statement support for this
lemma is the explicitly attributed, hash-bound written counterexample; the author's
error/locator evidence remains S-0025. The historical S1.4 consumer and outputs above
remain valid for their explicitly pinned pre-amendment corpus, with the old hold.

The status-support hold is discharged only on the verified integration tree. This
draft does not change main until reviewed and merged. Canonical selection, context
and locator review remain separate. No producer should select a raw parent by merely
walking directories; resolve the stable logical key using the validated effective
inventory first. This metadata check does not itself authorize canonical production.


## Default-branch activation — 2026-10-21 UTC / 2026-10-20 America/New_York

The owner merged PR 7 at **2026-10-21T03:33:35Z** (23:33:35 America/New_York),
creating main `1c56028532ea135603dbad5423d5555313bfd9ec`. Its tree
`14b451866491dc4a2b866ec0b1d726d792385916` equals the tested integration head
`a337375dbae49150a1a40fa46f6d1417835aadd2`. The prior proposed/draft wording above
is the record of earlier events. Acceptance, verified integration and this later
main activation are distinct; the ruling requires no second acceptance.

The unchanged Rust consumer passes on merged main and reproduces the retained
effective inventory exactly: 35 archived representations, 32 effective logical
items, unflagged subset 18. The old logical key resolves once to the accepted
replacement, and this case's status-support hold is discharged **on main**.
The [post-merge event](../analysis/s1-post-merge-readiness/README.md) retains fresh
receipt/provenance/checks; the [readiness assessment](../analysis/s1-post-merge-readiness/READINESS.md)
maps every S1 criterion and carries the existing exclusions. Canonical selection,
context, locator review and external-byte verification remain separate scoring
prerequisites. All prior content above, raw objects and evidence packets are preserved.

Gate S1 remains OPEN. The next action after this reconciliation reaches main is a
separate genuinely fresh examination under §2/SD15, launched with the constitution's
§6 Session-N resume prompt verbatim and alone. This informed session is not that exam.
