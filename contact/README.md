# contact/ — real AI-generated corpora (S5 only)

**Empty until Phase S5.** This directory exists so that the SD8 rules are posted before anything
could land here.

House rules (constitution §1 SD8, §2 S5, §4.6):

1. Nothing is ingested here before Gate S4 is closed and the battery is frozen
   (`battery-freeze-v1` tag, SHA in `PROGRAM/CLAIMS.md`). Ingesting any real AI-generated proof
   earlier is an escalation to Al, never an executor decision.
2. Everything here is a **read-only test subject** for the frozen battery. Contact material
   **never informs design**: an ADR that cites contact-corpus behaviour as rationale before the
   freeze is a leakage violation, and every session's HANDOFF carries a LEAKAGE CHECK.
3. Contact results never modify the battery in place. A change is a new battery version; the old
   results stand as recorded under the old version.
4. Every item carries a `syndeom-manifest/1` manifest (ADR-0001) with `origin: "contact"` and
   **`ground_truth: null`** — the instrument carries no verdict about a real manuscript (SD3).
   Publisher-documented labels, if any are ever used, live in S5 analysis files, never here.
5. Items arrive as `raw/1` (bytes as received) and are scored through derived canonical artifacts
   (ADR-0001 D4 convention); the `raw/1` item remains the item of record. Each corpus's own Lean
   and Mathlib pins are recorded at ingest (S5.1); per-corpus toolchains may be needed.
6. Licence and provenance are recorded in `PROGRAM/SOURCES.md` before ingestion (SD16); a copy is
   committed only when its licence permits — this repository is public.
7. Outputs about any item are calibrated triage scores with uncertainty, relative to the frozen
   battery version and named flaw classes. The words "proven", "disproven", "correct", and
   "wrong" never appear in output about a specific real manuscript.
