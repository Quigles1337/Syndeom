# HARVEST-LOG — every candidate examined for `groundtruth/` (ADR-0003 D7)

Append-only. One line per candidate examined, in the order examined; a changed verdict is a new
line citing the old one. Verdict vocabulary (ADR-0003 D4): `admitted` / `rejected` /
`found-but-unclassifiable` / `pending` / `pending-truth` / `escalated`. The verdicts are the
executor's readings of public documents against ADR-0003 D2's seven criteria; nothing here is a
verdict on any paper's mathematics beyond what its own status document says (SD3).

## Route A — arXiv revision comments (ADR-0003 D1, D8)

- **Window (D8):** correcting version submitted 2023-01-01 through 2026-09-30, inclusive. The
  window end precedes the 6 October 2026 release (S-0001), so the stream cannot overlap the
  contact era; nothing AI-generated is examined (SD8; D2 criterion 6). **Precision (ADR-0003 D8,
  Al, 2026-10-14):** the window bounds the date of the correcting version, never the paper's
  original submission; ascending arXiv id therefore puts the oldest papers first — the head of
  this stream is 2007–2012 submissions corrected in the window, by design.
- **Ordering (D8):** ascending arXiv identifier of the paper, as a plain string sort
  (`YYMM.NNNNN` ids first, then any `archive/YYMMNNN` id). **No skipping:** every candidate the
  finder lists is examined in this order and logged.
- **Candidate finder:** `lab/route_a_candidates.py` (standard library; zero result authority,
  SD10). API query over `https://export.arxiv.org/api/query`:
  `(co:corrected OR co:correction OR co:corrections OR co:erratum OR co:errata OR co:error OR
  co:errors OR co:fix OR co:fixed OR co:fixes OR co:mistake OR co:mistakes OR co:bug OR co:flaw OR
  co:flawed OR co:wrong OR co:revised OR co:amendment) AND (co:lemma OR co:theorem OR
  co:proposition) AND cat:math.*`, `sortBy=submittedDate`, ascending, 500 per page, 3 s between
  pages, with backoff on HTTP 429. Post-filter in the script: latest version ≥ 2; the two S-0015
  stage-1 regexes, case-insensitive, on the latest version's comment —
  `correct|errat|error|fix|mistake|bug|flaw|wrong|revised|amendment` and
  `lemma|theorem|proposition`; latest update on or after the window start. Papers whose latest
  update falls after the window end are kept and marked `latest update after window`; whether
  the *correcting* version lies in the window is settled from the submission history at
  examination. The finder sees only the latest version's comment, so a correction announced in an
  earlier version's comment and no longer mentioned in the latest is invisible to Route A; that
  is a known blind spot of the route, recorded here, not a defect of the no-skipping rule.
- **Examination helper:** `lab/route_a_examine.py <id> <v> <dir>` fetches both versions'
  abstract pages (per-version comment, submission history, licence links) and e-print sources,
  records their digests, and prints the TeX diff and any AI-assistance mention; the executor
  reads and decides.
- **Run 1 (Session 11, 2026-10-14):** `python -I -X utf8 lab/route_a_candidates.py 2023-01-01
  2026-09-30 <scratchpad>/route-a`, 01:28:02Z–01:56:25Z. 57 original-submission-date ranges
  (1991–2019 yearly, 2020Q1–2026Q4 quarterly), 73 requests, 11 transient HTTP 429/503 answers
  retried with backoff, no range beyond the shallow-offset limit (largest range 462 hits, year
  2017). API hits 8,081; after the post-filter **2,472 candidates**, 71 of them marked `latest
  update after window`. `candidates.tsv` SHA-256
  `e164c0b1693c7a6cdf242dcca853db28517f8c210d17af7ba82d2446dd752d48` (an arXiv-metadata snapshot,
  not committed; regenerable by the script modulo later arXiv changes; the examined prefix is
  fully recorded below, so nothing a verdict rests on lives only in the snapshot). An earlier form
  of the finder (one range, 500 per page) failed twice at page offset ≥ 1,500 with 429/503 — the
  API refuses deep offsets — and was replaced by the range split (HANDOFF Session 11).
- **Last candidate processed:** see the final row below; the next session resumes with the next
  id in `candidates.tsv` order after it.

## Candidates examined (Route A, in order)

Columns: `#` | arXiv id | correcting version (date) | verdict | reason, with the criterion that
decided it (D2.1–D2.7) and, for admitted items, the `artifact_id` and S-entries.

| # | arXiv id | correcting version | verdict | reason |
|---|---|---|---|---|
| 1 | `0706.0357` | v71 (2026-06-07) | **admitted** | D2.1–D2.7 met: a preprint (math.GM, single author, 71 versions since 2007); the v71 comment "To fix an error in Lemma 1.1" with v71 itself is the status document (D2.2); located to Lemma 1.1 (D2.3); classifiable from the v70→v71 diff — the lemma's identity lost a family of terms because the character-orthogonality formula was applied outside its coprimality hypothesis: FLAW-0012 (statement, fatal) + FLAW-0005 (proof, fatal), `statement_status` false by source document (D2.4, D3); v70 e-print retrievable, CC BY 4.0 → `committed` (D2.5, D5); human-authored, no AI mention (D2.6); a mathematical error (D2.7). `artifact_id` `s1_ingest-v0.1.0-63e23a952084`; S-0024 (original), S-0025 (notice); `RECORD.md` in the item directory. The paper's category and revision count played no part (D8, no skipping) |
| 2 | `0707.3799` | v3 (2026-02-25) | **admitted** | D2.1–D2.7 met: a paper "to appear in Moscow Math J" (math.RT, two authors); the v3 comment "corrects a mistake in the proof of Theorem 1b) … All the statements are unchanged" and v3's footnote are the status document (D2.2); located to the proof of Theorem 1 b) (D2.3); classifiable from the v2→v3 diff — the proof took W-invariants of a product decomposition whose identifications were never shown W-equivariant, which the corrected proof constructs: FLAW-0011 (composite, gap) + FLAW-0005 (gap), `statement_status` true by source document (D2.4, D3); v2 e-print retrievable, licence "assumed-1991-2003" (equivalent to arXiv non-exclusive) → `external`, digest-only (D2.5, D5); human-authored, the error pointed out by a named third party in the authors' acknowledgement (D2.6); a mathematical error (D2.7). `artifact_id` `s1_ingest-v0.1.0-744652f4b95f`; S-0026 (original), S-0027 (notice); `RECORD.md` in the item directory. v4 (2026-09-09) corrects other things and is not this item's notice |
| 3 | `0812.3181` | v3 (2023-07-20) | **rejected** | D2.1 not met: the v3 comment "Corrects an error in statement of Theorem 9.2 kindly pointed out by Amir Vig" and the v2→v3 diff show the error is in the *statement* of a theorem the lecture notes quote from the literature (the Duistermaat–Guillemin trace formula: "the number of conjugate points" corrected to the Morse index of the periodic variational problem); the notes offer no proof of it, so there is no offered proof to label — a FLAW-0010(c)-shaped mis-citation in expository notes, not an S1.1 item. The diff also fixes several unannounced small errors elsewhere; none is documented by the notice (D2.2) and none is used. Both versions arXiv non-exclusive licence; sources read, nothing stored |
| 4 | `0902.0680` | v3 (2026-09-30; v2 of 2011-06-29 withdrawn) | **admitted** | D2.1–D2.7 met: a preprint (math.DS, one author); the author's withdrawal plus the v3 comment and its section "Erratum on a previous version" are the status document (D2.2); located to "the dense-subspace argument for pointwise convergence" = the v1 lemma `density` and its proof (D2.3); classifiable: the only non-mechanical step of that proof invokes a weak-convergence criterion whose tightness hypothesis is asserted, not established — FLAW-0005 (gap), with the step-level reading marked as the executor's in the record (D2.4, D3); **`statement_status` null / `undetermined`** — v3 re-proves only the Fourier-dimension case and says the general statements "are not addressed" (the first use of Al's ruling (ii)); v1 e-print retrievable, arXiv non-exclusive → `external` (D2.5, D5); human-authored, gap found by the author (D2.6); a mathematical error (D2.7). `artifact_id` `s1_ingest-v0.1.0-5ba6c0f07557`; S-0028 (original v1), S-0029 (notice: v3, its erratum, the v2 withdrawal); `RECORD.md` in the item directory |
| 5 | `0908.3213` | v5 (2024-07-29) | **admitted** | D2.1–D2.7 met: a classification paper (math.RA/DG, three authors); the v5 comment "The statement of Theorem 4.4 (assertions (1) and (2)) was fixed and the proof was modified accordingly" with v5 is the status document (D2.2); located to Theorem 4.4 (1), (2) and the proof's normalization (D2.3); classifiable from the diff and an executor's check that the v4 structures fail the abelian condition `[Jx,Jy]=[x,y]` on four pairs while v5's satisfy it: FLAW-0001 propagated (statement, fatal) + FLAW-0001 local (proof, repairable), `statement_status` false by source document (D2.4, D3); v4 e-print retrievable, arXiv non-exclusive → `external` (D2.5, D5); human-authored, finder unstated (D2.6); a mathematical error (D2.7). **v4's own 2012 correction (Theorem 3.2 proof, Theorem 3.4 statement) is outside the window and not harvested** (D8 precision). `artifact_id` `s1_ingest-v0.1.0-a8ac73896464`; S-0030 (original v4), S-0031 (notice v5); `RECORD.md` in the item directory |
| 6 | `0912.0308` | v3 (2026-10-02) | **rejected** | D8 window: the correcting version was submitted 2 October 2026, after the window end (2026-09-30); abstract pages only were read, no sources fetched. Its comment — "Corrected Lemma 6.4 which was false as stated. The correction leads to slight weakening of the final bound" — names a candidate for a later window, recorded here for that purpose only |

**Session 11 counts (Route A run 1, prefix through arXiv id `0707.3799`):** examined 2 —
admitted 2, rejected 0, found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0.
**Last candidate processed: `0707.3799` (candidate 2 of 2,472).** The next session resumes with
candidate 3, `0812.3181` (v3, comment "Corrects an error in statement of Theorem 9.2 …"), in
`candidates.tsv` order; the session stopped at its budget, not at a change in the stream.

**Session 13 counts (Route A run 1, candidates 3–6; the candidate list regenerated from the
cached pages, 0 requests, digest identical to run 1):** examined 4 — admitted 2, rejected 2 (one
D2.1, one D8 window), found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0.
**Cumulative: examined 6 — admitted 4, rejected 2.** **Last candidate processed: `0912.0308`
(candidate 6 of 2,472).** The next session resumes with candidate 7, `0912.0966` (v6, comment
"… A small correction made to the statement of Theorem 8"); the session stopped at its budget.
