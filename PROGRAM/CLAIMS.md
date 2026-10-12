# CLAIMS — append-only claim register

Entry schema (§3.3): `ID | informal statement | status | evidence | manifest/taxonomy/battery deps |
axiom print and fidelity check (if PROVEN) | transitions log`.

Statuses: `UNKNOWN → VERIFIED (primary source read) / PROVEN (SD2: kernel-checked, standard axioms,
fidelity-reviewed) / REPRODUCED (published result recovered by our engine) / MEASURED (our own
result: engine SHA + seed + config hash + taxonomy and battery versions + CI) / CONDITIONAL /
FALSIFIED / RETRACTED`.

Rules: append-only — never delete or rewrite an entry or a past transition; a correction is a new
transition line. Transitions require evidence. Downgrades are always permitted and never shameful
(SD11, §4.5 honesty check). The **Status** field always reflects the latest transition.

SD11 reminder: nothing enters this register from executor memory, from a sweep report, or from the
genesis prompt. A claim about a source leaves UNKNOWN only after the source is retrieved this-session
and read from a primary source, with the retrieval recorded in SOURCES.md. CLAIM-0001..0005 below are
*program memory carried in the genesis prompt* — the executor's training does not cover these events
and nothing about them beyond the prompt's wording is known or recorded.

---

## CLAIM-0001 — the 6 October 2026 OpenAI manuscript release and its verification counts

- **Informal statement:** On 6 October 2026 OpenAI publicly released a set of AI-generated
  mathematical manuscripts, and that release (or its accompanying material) states counts of how
  many were verified. Program memory from the genesis prompt (§S0.1). Content, public location,
  counts, licence, and format are all unestablished. The exact location of the S5 contact corpus is
  derived from this claim's resolution and from nowhere else.
- **Status:** UNKNOWN
- **Evidence:** none. S-0001 UNVERIFIED.
- **Deps:** S-0001.
- **Axiom print / fidelity check:** n/a (not a formal claim)
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.

---

## CLAIM-0002 — the AGMAI guidelines

- **Informal statement:** A document referred to in the genesis prompt as "the AGMAI guidelines"
  exists and bears on the AI-mathematics situation. The acronym is deliberately not expanded here:
  the executor does not know what it stands for and will not guess (SD7, SD11).
- **Status:** UNKNOWN
- **Evidence:** none. S-0002 UNVERIFIED.
- **Deps:** S-0002.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.

---

## CLAIM-0003 — the AHM statement

- **Informal statement:** A document referred to in the genesis prompt as "the AHM statement" exists
  and bears on the AI-mathematics situation. Acronym deliberately not expanded (SD7, SD11).
- **Status:** UNKNOWN
- **Evidence:** none. S-0003 UNVERIFIED.
- **Deps:** S-0003.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.

---

## CLAIM-0004 — the Fields Medalist letter

- **Informal statement:** A letter signed by one or more Fields Medalists concerning AI-generated
  mathematics, referred to in the genesis prompt as "the Fields Medalist letter", exists. Signatories,
  date, venue, and content unestablished.
- **Status:** UNKNOWN
- **Evidence:** none. S-0004 UNVERIFIED.
- **Deps:** S-0004.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.

---

## CLAIM-0005 — the Leiden Declaration

- **Informal statement:** A document referred to in the genesis prompt as "the Leiden Declaration"
  exists and bears on the AI-mathematics situation. Date, authors, and content unestablished.
- **Status:** UNKNOWN
- **Evidence:** none. S-0005 UNVERIFIED.
- **Deps:** S-0005.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.

---

## CLAIM-0006 — the motivating premise (absence of calibration data)

- **Informal statement:** As of October 2026, no published calibration exists that states the error
  rate of a proof-triage method as a function of proof length and flaw class, for AI-generated
  mathematical proofs. This is the genesis prompt's framing of the problem (§0.1) and a scoped
  novelty claim; it resolves through a founding sweep (map, not evidence — §S0.2) plus primary-source
  retrieval, and it remains falsifiable by a single counterexample (§0.6). Until it is examined,
  SYNDEOM makes no novelty claim.
- **Status:** UNKNOWN
- **Evidence:** none.
- **Deps:** any future `PROGRAM/SWEEP-*.md`; S-0001..S-0005.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN from the genesis prompt's problem statement.
