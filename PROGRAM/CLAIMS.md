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
- **Status:** VERIFIED
- **Evidence:** S-0001 — the OpenAI post "Sharing AI progress in mathematics" (dated October 6,
  2026 on the page) and the public repository `github.com/openai/math` (created
  2026-10-06T21:47:02Z, licence Apache-2.0 from its own LICENSE file), both read first-hand on
  2026-10-12. **Verification counts, derived from the repository itself, with units:** at the
  initial commit `adc7f124` (2026-10-06) the catalogue states 722 manuscripts in 372 result
  families (722 preprint directories linked, 372 result headers counted); at the tip `1581d4cb`
  (after the 2026-10-08 update) 719 manuscripts in 372 families (719 linked, 372 headers);
  `history.md` (dated October 7, 2026) states "300 / 719 = ~42%" of top-line results formalized
  (numerator phrased as top-line results, denominator = manuscripts); the Lean catalogue
  `lean/formalization.yaml` (schema version v0.4) lists 173 source papers with a formalized main
  result and 231 related-formalization entries, and declares `review: status: unchecked`; 242
  families carry a Lean scope note under `lean/docs/`. The post itself states no counts; it states
  10 reasoning summaries (the repository holds 10 PDFs) and roughly three hours of ChatGPT Pro
  thinking per result. The repository's own update PR #1 states "Lean builds were not run for
  this synchronization." **S5 contact-corpus location established:** `https://github.com/openai/math`,
  to be pinned at a commit at ingest; its Lean toolchain is `leanprover/lean4:v4.34.1` with Mathlib
  at `d13f23b7…` (recorded now for the Q8 consequence; acted on only at S5).
- **Deps:** S-0001.
- **Axiom print / fidelity check:** n/a (not a formal claim)
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.
  - 2026-10-12 (Session 2) — retrieval hints supplied by Al at ratification, recorded in SOURCES.md
    as "named by Al"; status unchanged: UNKNOWN until the primary source is read (SD11).
  - 2026-10-12 (Session 2) — UNKNOWN → **VERIFIED** (S0.1; the post and the repository read
    first-hand; counts derived from the catalogue at two commits; S-0001).

---

## CLAIM-0002 — the AGMAI guidelines

- **Informal statement:** A document referred to in the genesis prompt as "the AGMAI guidelines"
  exists and bears on the AI-mathematics situation. The acronym is deliberately not expanded here:
  the executor does not know what it stands for and will not guess (SD7, SD11).
- **Status:** VERIFIED
- **Evidence:** S-0002 — `agmai.org` read first-hand 2026-10-12: "Advisory Group on Mathematics
  and Artificial Intelligence", nine named members, and the document "Responsible Release of
  AI-Generated Mathematics" dated September 29, 2026 (web page plus PDF). The group's page header
  names the Institute for Advanced Study and links its IAS project page; that IAS page returned
  HTTP 403 to both retrieval tools, so the IAS affiliation is verified from the group's own site,
  not from IAS's. The OpenAI post links both agmai.org pages. Al's expansion of the acronym is
  confirmed by the primary.
- **Deps:** S-0002.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.
  - 2026-10-12 (Session 2) — retrieval hints supplied by Al at ratification, recorded in SOURCES.md
    as "named by Al"; status unchanged: UNKNOWN until the primary source is read (SD11).
  - 2026-10-12 (Session 2) — UNKNOWN → **VERIFIED** (S0.1; agmai.org index and document read
    first-hand; S-0002).

---

## CLAIM-0003 — the AHM statement

- **Informal statement:** A document referred to in the genesis prompt as "the AHM statement" exists
  and bears on the AI-mathematics situation. Acronym deliberately not expanded (SD7, SD11).
- **Status:** VERIFIED
- **Evidence:** S-0003 — `www.ahmath.org/statements` read first-hand 2026-10-12: "AHM Statement
  on OpenAI's October 6 Release of Mathematical Documents", signed "Association for Human
  Mathematics / Communications Working Group". The primary page carries **no date**; its text
  opens "Yesterday, on October 6th, 2026". The 7 October date is carried by Terence Tao's repost
  (terrytao.wordpress.com, 7 October, 2026, "reposted from their statements page"). Al's expansion
  of the acronym is confirmed by the primary.
- **Deps:** S-0003.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.
  - 2026-10-12 (Session 2) — retrieval hints supplied by Al at ratification, recorded in SOURCES.md
    as "named by Al"; status unchanged: UNKNOWN until the primary source is read (SD11).
  - 2026-10-12 (Session 2) — UNKNOWN → **VERIFIED** (S0.1; the AHM statements page read
    first-hand, Tao's repost read for the date; S-0003).

---

## CLAIM-0004 — the Fields Medalist letter

- **Informal statement:** A letter signed by one or more Fields Medalists concerning AI-generated
  mathematics, referred to in the genesis prompt as "the Fields Medalist letter", exists. Signatories,
  date, venue, and content unestablished.
- **Status:** VERIFIED
- **Evidence:** S-0004 — `mathandai.org` read first-hand 2026-10-12: "A Severe Misalignment of AI
  in Mathematics", "Published 11 September 2026 · DOI 10.5281/zenodo.22737750". The Zenodo record
  (version v1 at 22737751, licence CC0 1.0) lists 25 authors. The page lists **28 signatories as
  of 2026-10-12**, each labelled with a Fields Medal year: the 25 deposited plus Drinfeld,
  Margulis, and Mumford. Tao's same-day post states "25 initial signatories — all Fields
  Medallists". The deposited PDF (29,254 bytes; MD5 matches the record) is stored under
  `sources/` under CC0.
- **Deps:** S-0004.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.
  - 2026-10-12 (Session 2) — retrieval hints supplied by Al at ratification, recorded in SOURCES.md
    as "named by Al"; status unchanged: UNKNOWN until the primary source is read (SD11).
  - 2026-10-12 (Session 2) — UNKNOWN → **VERIFIED** (S0.1; page, Zenodo record and deposited PDF
    read first-hand; signatory count recorded as of access date; S-0004).

---

## CLAIM-0005 — the Leiden Declaration

- **Informal statement:** A document referred to in the genesis prompt as "the Leiden Declaration"
  exists and bears on the AI-mathematics situation. Date, authors, and content unestablished.
- **Status:** VERIFIED
- **Evidence:** S-0005 — `leidendeclaration.ai` read first-hand 2026-10-12: "Leiden Declaration on
  Artificial Intelligence and Mathematics", "2 June 2026 · DOI: 10.5281/zenodo.20302944"; a
  16-member working group is named; "4322 Signatories" as of 2026-10-12. Zenodo record v1 (licence
  CC BY-NC 4.0, 16 creators, published June 2, 2026) and arXiv:2609.35334 (math.HO; v1 28 Sep 2026,
  v2 30 Sep 2026; arXiv non-exclusive distribution licence). The deposited PDF (199,338 bytes; MD5
  matches the record) is stored under `sources/` under CC BY-NC 4.0.
- **Deps:** S-0005.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN per §S0.1.
  - 2026-10-12 (Session 2) — retrieval hints supplied by Al at ratification, recorded in SOURCES.md
    as "named by Al"; status unchanged: UNKNOWN until the primary source is read (SD11).
  - 2026-10-12 (Session 2) — UNKNOWN → **VERIFIED** (S0.1; page, Zenodo record, arXiv abstract
    and deposited PDF read first-hand; S-0005).

---

## CLAIM-0006 — the motivating premise (absence of calibration data)

- **Informal statement (original, Session 1; superseded in operation by the narrowed form below,
  kept verbatim for the record):** As of October 2026, no published calibration exists that states
  the error rate of a proof-triage method as a function of proof length and flaw class, for
  AI-generated mathematical proofs. This is the genesis prompt's framing of the problem (§0.1) and
  a scoped novelty claim; it resolves through a founding sweep (map, not evidence — §S0.2) plus
  primary-source retrieval, and it remains falsifiable by a single counterexample (§0.6). Until it
  is examined, SYNDEOM makes no novelty claim.
- **Narrowed statement (Session 5, 2026-10-13, the narrowest form the retrieved sources leave
  standing):** *As of 2026-10-13, among the primary sources retrieved and read (S-0009..S-0018,
  with S-0002 and S-0006), no published work reports the detection performance of any proof-triage
  method — automated or human — on mathematical proofs, informal or formal, broken down **jointly**
  by flaw class and by proof length, for AI-generated proofs at any level above grade-school and
  GRE-level word problems; and no retrieved work reports such a joint breakdown at research level
  for any kind of proof. No retrieved work defines a proof length at which a detector falls to
  chance for a flaw class.* Bounding results, each axis separately: **by class alone** — S-0009
  (nine word-problem error types), S-0010 (nine sub-categories, MATH level), S-0012 (missed-error
  distribution over four groups, undergraduate/qualifying-exam level, AI-generated proofs), S-0014
  (five Lean injection strategies, olympiad level), S-0016 (per-category F1 for dataset-defect
  detection, not proof triage); **by length alone** — S-0013 (proof-checking accuracy as a function
  of proof length in formal natural deduction, partly AI-written proofs, with top models at or near
  chance on 720–987-unit proofs — no class breakdown); position-not-length — S-0010, S-0011,
  S-0018; **both axes, one class** — S-0009 (calculation error, 2–9 steps, word-problem solutions
  rewritten by GPT-4: "not significantly affected by the number of steps"); **research-level
  AI-generated proofs with labelled errors, no breakdown** — S-0015's FirstProofGradingBench (16
  proofs, 24 referee-identified errors) and, human-authored, its ArxivMathGradingBench (35
  papers). What this examination cannot establish: absence beyond the retrieved set; the sweep's
  runners-up were not retrieved, and the sweep itself (a map) found no further candidate.
- **Status:** CONDITIONAL — unfalsified as of 2026-10-13 in the narrowed form, conditional on the
  bounding sources listed; a claim of absence cannot become VERIFIED by retrieval (§0.6; Al,
  Session 5). FALSIFIED by any single published counterexample to the narrowed statement.
- **Evidence:** S-0009..S-0018 (each entry's "Bears on CLAIM-0006" field quotes the source on the
  two axes); none falsifies the original wording either, because the one source crossing class
  with length (S-0009) does so for one class on word-problem solutions, not proofs.
- **Scoped novelty statement (lifts §0.6's "no novelty claim" in exactly this scope):** SYNDEOM's
  deliverable — detector error rates jointly by flaw class × proof length with a power-floored
  fooling length (§2 S3.2), on manifested informal and formal proof artifacts — has, as far as the
  Session-5 retrieval found, no published precedent; precedents exist for each axis separately
  (cite S-0009, S-0010, S-0012, S-0013, S-0014) and for research-level labelled errors without
  either breakdown (S-0011, S-0015). Every use of this statement cites 2026-10-13 and this scope.
- **Deps:** `PROGRAM/SWEEP-2026-10.md` (map); S-0002, S-0006, S-0009..S-0018.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 1) — seeded UNKNOWN from the genesis prompt's problem statement.
  - 2026-10-12 (Session 2) — Al confirms no founding sweep exists (Q14); §0.6 holds; stays UNKNOWN.
  - 2026-10-12 (Session 3) — founding sweep ferried by Al and placed at `PROGRAM/SWEEP-2026-10.md`
    (SHA-256 `f4bfb2ad…b03e`, verified against Al's stated digest; SD11 banner). **Stays UNKNOWN:**
    the sweep is a map, not evidence. Its verdict (no falsifying source found; a narrowed form
    survives) is a hint for retrieval, not a transition. Al's ruling: CLAIM-0006 moves only after
    the ten sources the sweep ranks are retrieved and read first-hand — a later S0 action (STATE
    "Deferred S0 actions"). Deps now name the file.
  - 2026-10-13 (Session 5) — the ten ranked sources retrieved from primaries and read against the
    claim as worded (S-0009..S-0018; S-0002 and S-0006 cross-referenced). **No source falsifies the
    claim as worded.** Narrowed to the statement above (Al's instruction: the narrowest form the
    sources leave standing; the sweep's §(2) wording was the draft, not the answer — it is
    tightened to say what *was* found: a single-class length sweep at word-problem level, a
    length-axis near-chance regime in formal logic, and research-level labelled AI proofs with no
    breakdown). UNKNOWN → **CONDITIONAL** (unfalsified as of 2026-10-13, bounded by the sources
    listed). The scoped novelty statement is recorded; §0.6 lifts only in that scope.

---

## CLAIM-0007 — a reported correction or retraction within the 6 October 2026 release

- **Informal statement:** Shortly after the 6 October 2026 release, some of its manuscripts were
  corrected or retracted, as reported. Named by Al at ratification (Session 2). **Contact-phase item
  under SD8:** verifying that such a correction or retraction *exists* is S0 work; reading its
  *content* (which manuscripts, which flaws) is S5 work and never informs design before the freeze.
  It is never S1 ground truth (SD12: S1 holds human-authored proofs with documented status).
- **Status:** VERIFIED (existence only)
- **Evidence:** S-0006 — `history.md` in `openai/math`, dated "October 7, 2026", added by commit
  `30148886` (PR #1 "Update manuscripts and Lean formalizations", merged 2026-10-08T05:20:01Z;
  the file is absent at the initial commit). It records withdrawals (3 manuscripts), revisions
  (14), citation updates (13), and additional formalizations; the catalogue's manuscript count
  fell from 722 to 719 accordingly, and the README promises that previous versions remain
  accessible. **The content — which manuscripts, what gaps — is deliberately not transcribed
  here (SD8)** and is read only in S5.
- **Deps:** S-0006; S-0001.
- **Axiom print / fidelity check:** n/a
- **Transitions:**
  - 2026-10-12 (Session 2) — seeded UNKNOWN at ratification (pre-ratification edit 3).
  - 2026-10-12 (Session 2) — UNKNOWN → **VERIFIED (existence only)** (S0.1; the repository's own
    history file and PR read first-hand; content withheld under SD8; S-0006).
