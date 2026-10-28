# First later prefix — inventory and coverage (ADR-0013)

The inventory event for the first later prefix: harvest candidates 28–30, items 32–36, integrated
by PR 24 (merge `ef567ec`) and frozen at `d2569d6` (Al's ruling Q-H7 of 2026-10-28). It counts
the committed corpus and writes the ADR-0003 D9 coverage table for the prefix through arXiv id
`1404.4599`. It is this prefix's own record. It does not rewrite the Gate S1 table, the frozen S1.4
packet or the first-parent integration's retained report, and it changes no manifest, label or
register status.

## Contents

- `config/prefix-inputs.json` — the freeze point, the five item directories' ten files and the
  harvest log as they stood there (ADR-0013 D2).
- `config/dispositions.json` — the prefix's review overlay (ADR-0013 D4).
- `report/` — the outputs as the tool wrote them from the committed tree: `inventory.json`,
  `coverage.json`, `report.md` and `provenance.json`. The run's receipt is in the pull request and
  under the external root, not in the tree it describes.

## Reproduce

From a clean checkout, with the output directory outside the repository:

```
cd engine
cargo run --locked -p syndeom-ingest --example s1_later_prefix -- <repo> <new-output-directory>
```

`inventory.json`, `coverage.json` and `report.md` depend only on the tree's contents and must be
byte-identical to `report/`. `provenance.json` names the checkout revision, so it differs from the
retained copy whenever the checkout does. The gate checks the first three from the working tree
(`engine/ingest/tests/later_prefix.rs`).

## Bounds

The tool refuses any population other than 40 archived representations and 37 effective logical
items. A later prefix is its own event, with its own tool revision and packet.
