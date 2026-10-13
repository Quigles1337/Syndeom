# groundtruth/ — real proofs with documented status (S1)

Layout per ADR-0001 D2: `groundtruth/<artifact_id>/manifest.json` plus the stored files the
manifest names. Every item has `origin: "ground-truth"` and a derivation of
`documented-by-source` (errata, retractions) or `formalized-in-mathlib` (informal–formal pairs);
never `by-construction` (SD12). Each item records its source, licence, and the document that
establishes its status (`config.sources`, `config.licence`, `config.status`), all registered in
`PROGRAM/SOURCES.md` before ingestion (SD16).

Items arrive as `raw/1` and are scored through derived canonical artifacts whose location
re-expressions are argued in writing in the S1.3 record (ADR-0001 D4 convention); the `raw/1`
item remains the item of record. Nothing synthetic is ever placed here. Empty until Phase S1.
