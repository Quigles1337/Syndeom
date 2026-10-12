# SOURCES — citations and data provenance (SD11, SD16)

An entry here records what the program *has been told exists* and what it *has actually retrieved*.
Every entry carries one of these states:

- **UNVERIFIED** — named (by the genesis prompt, a sweep, or Al) but not yet retrieved; nothing
  beyond the naming is recorded; a note for Al says what is needed.
- **VERIFIED** — retrieved this-session and read from a primary source, with full citation, URL,
  access date, licence terms, and SHA-256 of any stored copy.
- **SURVEYED** — existence, coverage, and licence noted; nothing ingested.
- **INGESTED** — data imported into the repo under the recorded licence, with audit trail.

Rules: a citation exists only after retrieval and verification this-session (SD11). Licence terms are
recorded *before* any data ingestion (SD16); ambiguity → escalate to Al, do not ingest. A stored copy
is committed to the repo only when its licence permits redistribution (this repo is public);
otherwise the entry records URL + access date + SHA-256 of the local copy and notes that the copy is
not committed (FRIEDMAN's storage rule, adopted). Stored copies live under `sources/`. If the
session has no web access, everything stays UNVERIFIED. Never fabricate a citation.

Any founding sweep report placed under `PROGRAM/` is a **map, not evidence**: it may suggest what to
retrieve; nothing enters this register from it.

---

## S-0001 — the 6 October 2026 OpenAI manuscript release and its verification counts (CLAIM-0001)

- **State:** UNVERIFIED
- **As named in the genesis prompt (2026-10-12):** "the Oct 6 2026 OpenAI manuscript release and its
  verification counts". Also the candidate S5 contact corpus: "the Oct 6 2026 public release — exact
  location verified in S0, not assumed".
- **Known to the executor beyond that wording:** nothing. No URL, title, author list, count, licence,
  or format is recorded because none has been read.
- **Needed (S0.1):** the primary announcement or release page; the manuscripts' public location and
  licence; the verification counts as stated by the primary source. If Al holds copies, a path plus
  size or SHA-256 lets the executor verify identity in one step.

## S-0002 — the AGMAI guidelines (CLAIM-0002)

- **State:** UNVERIFIED
- **As named in the genesis prompt:** "the AGMAI guidelines".
- **Known to the executor beyond that wording:** nothing. The acronym is not expanded here (SD7).
- **Needed (S0.1):** the primary document, its issuing body, date, URL, licence.

## S-0003 — the AHM statement (CLAIM-0003)

- **State:** UNVERIFIED
- **As named in the genesis prompt:** "the AHM statement".
- **Known to the executor beyond that wording:** nothing. The acronym is not expanded here (SD7).
- **Needed (S0.1):** the primary document, its issuing body, date, URL, licence.

## S-0004 — the Fields Medalist letter (CLAIM-0004)

- **State:** UNVERIFIED
- **As named in the genesis prompt:** "the Fields Medalist letter".
- **Known to the executor beyond that wording:** nothing.
- **Needed (S0.1):** the letter itself (primary), signatories, date, venue, URL, licence.

## S-0005 — the Leiden Declaration (CLAIM-0005)

- **State:** UNVERIFIED
- **As named in the genesis prompt:** "the Leiden Declaration".
- **Known to the executor beyond that wording:** nothing.
- **Needed (S0.1):** the declaration text (primary), authors or signatories, date, URL, licence.
