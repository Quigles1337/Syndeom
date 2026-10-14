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
| 7 | `0912.0966` | v6 (2026-06-30) | **rejected** | D2.1 not met: the v6 comment "A small correction made to the statement of Theorem 8" and the diff (the index set of the correlation sum in equation `close` gains the condition "distinct"; the acknowledgement thanks a named reader for the correction) concern Theorem 8, which the paper introduces as a result Nagao and Wadati "established" and cites; the paper proves its extension (a corollary) but offers no proof of Theorem 8 itself, so there is no offered proof to label — a FLAW-0010(c)-shaped mis-citation, as candidate 3. Both versions arXiv non-exclusive; sources read (abs v5/v6 `804f67ec…`/`77711da5…`, e-prints `75cd4342…`/`c4e463f6…`), nothing stored |
| 8 | `1005.2398` | v3 (2026-03-22) | **admitted ×2** | D2.1–D2.7 met for two documented results in one notice: the v3 comment names Proposition 3.3.5 ("the addition of a finitely presented hypothesis") and v3's footnotes say the published statements of the proposition *and* of the following lemma "were incorrect", the proposition's footnote giving the counterexample ℤ → ℤ[{x_p}]/(p x_p, x_p^p) (D2.2, D2.3); classifiable from the diff — a dropped hypothesis with the proof of the true case transplanted: FLAW-0012 (statement, fatal) + FLAW-0005 (the (2) ⇒ (1) step, fatal) for the proposition, `statement_status` false by the author's counterexample; the lemma inherits both through its one-line reduction to the proposition, `statement_status` false by source document (D2.4, D3); v2 e-print retrievable, arXiv non-exclusive → `external` (D2.5, D5); human-authored, finder unstated (D2.6); mathematical errors (D2.7). Items 5 and 6: `s1_ingest-v0.1.0-be3b17e73425` (Proposition 3.3.5), `s1_ingest-v0.1.0-72415457dce4` (the following lemma); S-0032 (original v2), S-0033 (notice v3), shared. A `char(κ)` fix elsewhere in the diff is not named by the notice and not harvested |
| 9 | `1009.3998` | v5 (2026-04-23) | **admitted** | D2.1–D2.7 met: a published paper (Annals of Math. 176, per the comment; math.CO, three authors); the v5 comment "corrects a mistake in the proof of Lemma 13.2 (the wrong filtration was specified)" and v5's footnote are the status document (D2.2); located to the filtration definition in that proof (D2.3); classifiable from the diff — the wrong object specified, the consequences of the intended one asserted: FLAW-0001 local (repairable) inside FLAW-0011 (the lemma unchanged), `statement_status` true by source document; FLAW-0004 named as the alternative reading (D2.4, D3); v4 e-print retrievable, arXiv non-exclusive → `external` (D2.5, D5); human-authored — the scanner's "llm" hit is `\smallmatrix` (D2.6); a mathematical error (D2.7). `artifact_id` `s1_ingest-v0.1.0-e0c199d3c60a`; S-0034 (original v4), S-0035 (notice v5) |
| 10 | `1010.2391` | v10 (2023-10-05) | **admitted** | D2.1–D2.7 met: a published paper (DCDS-A 32(8), 2012, per the abstract page; math.DS, one author); the v10 comment "correcting mistakes in formulation of Lemma A.3 …: center x of neighborhood must be in C¹" with v10 is the status document (D2.2); located to Lemma A.3's statement and the opening of its proof (D2.3); classifiable from the diff — the statement quantified over D^{0,1} while the proof assumed x ∈ D¹: FLAW-0012 (statement, fatal) + FLAW-0005 (proof, fatal), `statement_status` false by source document with the stricter `undetermined` reading named in the record as the alternative (D2.4, D3); v9 e-print retrievable, arXiv non-exclusive → `external` (D2.5, D5); human-authored, finder unstated (D2.6); a mathematical error (D2.7). **v9's own 2020 corrections (Lemmas 2.3, A.5) are outside the window** (D8 precision). `artifact_id` `s1_ingest-v0.1.0-448d7aefa486`; S-0036 (original v9), S-0037 (notice v10) |
| 11 | `1102.1087` | v11 (2024-01-01), itself a withdrawal | **found-but-unclassifiable** | D2.4 not met: the correcting version is a withdrawal stub (versions 2, 5, 9 and 11 are marked withdrawn; v11 has no e-print, HTTP 404) whose comment lists problems without describing any error — "The algorithm in Section 5.3 does not appear to follow the Turing model. There were other unrelated errors. Theorem 29 was not stated properly. There should have been a reference attached to Theorem 31. The explanation and definition of degree for Section 2.2 should have been done more carefully. It is not clear if this list of errors is comprehensive" — and there is no corrected version to confirm against; "Theorem 29 was not stated properly" locates (D2.3) but cannot be classed. One author; cs.DM primary, math cross-list; v10's comment is an arXiv overlap note. The original v10 was read (abs `d1f9a247…`, e-print 52,584 B `eab03cea…`), nothing stored. Counted toward the class the notice suggests: none |
| 12 | `1107.0863` | v4 (2026-04-22; the appendix is also a journal correction) | **admitted ×2** | D2.1–D2.7 met for two documented results in one notice: the v4 comment "error in Lemma 12, affecting Theorem 1" and v4's appendix — "Theorem 1 of [CH] is incorrect in the stated generality due to an error in [Lem. 12]"; "[Lem. 12] … is false as stated; Figure A shows a counterexample" — are the status document (D2.2, D2.3); classifiable: Lemma 12 — FLAW-0012 (statement, fatal), `statement_status` false by the authors' counterexample figure, the failing proof step not named by the notice and not guessed; Theorem 1 — FLAW-0012 (statement, fatal) through its proof's reliance on Lemma 12, `statement_status` false by source document ("incorrect in the stated generality"; the disproving examples are attributed to a forthcoming paper), the `undetermined` reading named in the record as the alternative (D2.4, D3); v3 e-print retrievable (4.2 MB with figures), arXiv non-exclusive → `external` (D2.5, D5); human-authored — the scanner's "llm" hit is the citation key `BallmanSwiatkowski` (D2.6); mathematical errors (D2.7). Items 9 and 10: `s1_ingest-v0.1.0-1c4c9a7747d0` (Lemma 12), `s1_ingest-v0.1.0-2d93a266558a` (Theorem 1); S-0038 (original v3), S-0039 (notice v4 with the appendix), shared. Theorem 2 is unchanged per the comment |

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

**Session 14 counts (Route A run 1, candidates 7–10; candidate list regenerated from the cached
pages, 0 requests, digest identical to run 1):** examined 4 — admitted 3 candidates yielding **4
items** (candidate 8 carries two documented results), rejected 1 (D2.1), found-but-unclassifiable
0, pending 0, pending-truth 0, escalated 0. **Cumulative: examined 10 — admitted 7 candidates (8
items), rejected 3 (two D2.1, one D8 window).** **Last candidate processed: `1010.2391`
(candidate 10 of 2,472).** The next session resumes with candidate 11, `1102.1087` (v11, cs.DM,
comment beginning "The algorithm in Section 5.3 does not appear to follow the Turing model …");
the session stopped at its budget.

**Session 15 counts (Route A run 1, candidates 11–12; candidate list regenerated from the cached
pages, 0 requests, digest identical to run 1):** examined 2 — admitted 1 candidate yielding **2
items**, found-but-unclassifiable 1, rejected 0, pending 0, pending-truth 0, escalated 0.
**Cumulative: examined 12 — admitted 8 candidates (10 items), rejected 3, found-but-unclassifiable
1.** **Last candidate processed: `1107.0863` (candidate 12 of 2,472).** The next session resumes
with candidate 13, `1109.1964` (v2, math.DG, comment "Crucial mistake in Proposition 2.1. Article
superseded by …"); the session stopped at its budget.
