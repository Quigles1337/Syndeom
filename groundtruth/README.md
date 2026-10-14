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

Procedure: ADR-0003 (the errata-and-retraction harvest). Every item directory is written by the
deterministic producer `syndeom-ingest` (`engine/ingest/`, `generator.id` `s1_ingest`; ADR-0003
D6) from a per-item config and the original's bytes — never by hand — and carries `RECORD.md`,
the written record of provenance, the notice's locating sentences, the diff summary and the
FLAWS.md class argument (ADR-0003 D4). `HARVEST-LOG.md` beside this file is the append-only log
of every candidate examined, with its verdict and the last candidate processed (ADR-0003 D7, D8).
Manifests carry `syndeom-manifest/2` (ADR-0001 as amended 2026-10-13). No item yet.
