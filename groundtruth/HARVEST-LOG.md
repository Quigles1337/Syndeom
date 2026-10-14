# HARVEST-LOG — every candidate examined for `groundtruth/` (ADR-0003 D7)

Append-only. One line per candidate examined, in the order examined; a changed verdict is a new
line citing the old one. Verdict vocabulary (ADR-0003 D4): `admitted` / `rejected` /
`found-but-unclassifiable` / `pending` / `pending-truth` / `escalated`. The verdicts are the
executor's readings of public documents against ADR-0003 D2's seven criteria; nothing here is a
verdict on any paper's mathematics beyond what its own status document says (SD3).

## Route A — arXiv revision comments (ADR-0003 D1, D8)

- **Window (D8):** correcting version submitted 2023-01-01 through 2026-09-30, inclusive. The
  window end precedes the 6 October 2026 release (S-0001), so the stream cannot overlap the
  contact era; nothing AI-generated is examined (SD8; D2 criterion 6).
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

**Session 11 counts (Route A run 1, prefix through arXiv id `0707.3799`):** examined 2 —
admitted 2, rejected 0, found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0.
**Last candidate processed: `0707.3799` (candidate 2 of 2,472).** The next session resumes with
candidate 3, `0812.3181` (v3, comment "Corrects an error in statement of Theorem 9.2 …"), in
`candidates.tsv` order; the session stopped at its budget, not at a change in the stream.
