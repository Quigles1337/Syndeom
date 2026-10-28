# Held harvest admission review — `claude/s1-harvest-candidates-28-30` @ `c3a3d36`

- **What this is:** S2 ordering step 3 (Al's order of 2026-10-26), STATE's NEXT-SINGLE-ACTION
  as of `main` `c48705c2d23183340eef5c01ff13059905513985` (PR 22's merge). It is a review of the
  held branch as the candidate first later prefix of the Route A stream (ADR-0003 D8, D9).
- **What it does:** it records and recommends.
- **What it does not do:** it integrates nothing, approves nothing, relabels nothing and writes no
  generator code. The branch's five labels stay unapproved (Al's decision 1 of 2026-10-21).
- **Who and when:** Session 37, the executor (Claude Opus 5.5), 2026-10-28 UTC
  (2026-10-27 America/New_York). This is a reading of public documents and of the branch, not a
  verdict on any paper (SD3).
- **Receipt:** `receipt.json`, beside this file.
- **Evidence re-verification:** `evidence-reverify.json`, produced by `verify_held_evidence.py`;
  both are in this directory.

## 1. What was read, and how

- **The branch.** The branch was read only through `git show c3a3d36:<path>` and
  `git diff origin/main...c3a3d36`. It was never checked out.
- **Branch facts:**
  - tip `c3a3d369689d56bab3678f519904087b313daf4e`, tree `7bb6600d…`;
  - one commit on base `8e222a49…`, authored 2026-10-21T22:26:57Z;
  - local branch, remote-tracking ref and live remote (`git ls-remote`) all at the same SHA;
  - 13 files, +1,223/−52: `PROGRAM/SOURCES.md`, `PROGRAM/STATE.md`, `groundtruth/HARVEST-LOG.md`,
    and five item directories, each holding `RECORD.md` and `manifest.json`.
- **The preserved evidence.** Session 22's retrieval evidence lives under
  `C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\held-evidence\`.
  - All **147 of 147** files re-hash to the committed manifest
    `analysis/s1-orientation-repair/evidence/held-evidence-manifest.json`
    (SHA-256 `d3ea7879…87ce`), 17,824,213 bytes.
  - The root's own `manifest.json` is byte-identical to the committed copy. No file on disk is
    unlisted. Result: `ALL MATCH`.
  - Ten abstract pages exist only as hash-and-size text receipts, as the manifest says. Six of
    them carry notice text read here: 1403.6317 v1/v3, 1403.8075 v9/v11 and 1404.4599 v4/v5. For
    these, the comments were checked against the receipts' transcriptions, not against page bytes.
    The 1403.6317 v2 and 1403.8075 v10 withdrawal comments were checked against the preserved
    page bytes.
- **The originals.** Each manifest's `artifact.files.informal.content_hash` and `n_bytes` equal the
  preserved e-print's full SHA-256 and length, for all five items:

  | Item | Pinned original | Bytes | SHA-256 prefix |
  |---|---|---|---|
  | 32 | 1403.6317 v1 | 126,877 | `97b28cf2…` |
  | 33–35 | 1403.8075 v9 | 136,883 | `c171e338…` |
  | 36 | 1404.4599 v4 | 48,382 | `2e326a33…` |

  Every `config_hash` begins with its `artifact_id` suffix, and all five use `syndeom-manifest/2`.
- **The stream (ADR-0003 D8).** The preserved candidate list `route-a/candidates.tsv` has 2,472
  rows, SHA-256 `e164c0b1…2d48`. That is the run-1 digest Session 11 recorded (HANDOFF line 1464
  at `c48705c`). Its rows 27–31 are:

  | Row | arXiv id |
  |---|---|
  | 27 | `1403.4485` |
  | 28 | `1403.6317` |
  | 29 | `1403.8075` |
  | 30 | `1404.4599` |
  | 31 | `1406.7174` |

  The order is ascending and nothing is skipped: main's last candidate is 27, and the branch
  examined 28–30.
- **The window.** Every correcting version falls inside 2023-01-01..2026-09-30:

  | arXiv id | Correcting version | Date |
  |---|---|---|
  | 1403.6317 | v3 | 2024-10-15 |
  | 1403.8075 | v10 | 2025-12-27 |
  | 1403.8075 | v11 | 2026-01-09 |
  | 1404.4599 | v5 | 2024-01-12 |

- **The criteria re-applied.**
  - ADR-0003 D2.1–D2.7 and the D3 mapping;
  - the FLAWS.md `0.1.0` definitions of FLAW-0004, FLAW-0005 and FLAW-0012;
  - each record's claim of record, located in the preserved TeX by label and by counting the
    numbered environments on the paper's own counter.

## 2. Per-item verdicts

| Item (branch numbering) | Candidate | Result | Branch labels | D2 | D3 | Verdict |
|---|---|---|---|---|---|---|
| 32 `…eeea00c52724` | 28, arXiv 1403.6317 v1 | Theorem 1 (`thm:HD`) | FLAW-0012 fatal, `YES-KERNEL`; false by source document (`undetermined` named) | agree | agree | **AGREE**, one coupled question (Q-H1) |
| 33 `…b77cf8e312d0` | 29, arXiv 1403.8075 v9 | Proposition 4.1 | FLAW-0004 dial `false`, fatal; `undetermined` | agree | agree | **AGREE** |
| 34 `…c38c8c65dd2c` | 29 | Theorem 4.3 | FLAW-0005 gap; `undetermined` | agree | agree under the entry-range practice | **AGREE**, severity question (Q-H2) |
| 35 `…e8075d1f1a0d` | 29 | Theorem 4.4 | FLAW-0005 gap; `undetermined` | agree | agree under the entry-range practice | **AGREE**, severity question (Q-H3) |
| 36 `…22b1b48f3565` | 30, arXiv 1404.4599 v4 | Proposition 2.22 (`Nacycgroupoidprop`) | FLAW-0005 fatal (FLAW-0004 named); `undetermined` (`true` named) | agree | agree, with a class-fit note | **AGREE**, wording question (Q-H5) |

**Five items reviewed. All five agree with the evidence, and there are no disagreements.** Every
"agree" is conditional on the questions in §3, which are Al's.

### Item 32: candidate 28, arXiv 1403.6317, Theorem 1

**D2.2 (the status document).** Verified against the preserved bytes:
- The v2 withdrawal comment "…due to an error in the assumptions of the Theorem 1" is in
  `abs-v2.html` (39,708 B, `81d0d559…`).
- The v3 comment "We removed the wrong theorem from the first version, which was brought to our
  attention by R. Leclercq" is in the `abs-only.txt` receipt.
- v3's "We would like to thank R\'emi Leclerq for pointing to us an error in the previous version
  of the paper." is in `v3/DKM_filomat_v3.tex`.

**D2.3 (the location).** The statement is quoted correctly from `v1/Djuretic_Katic_Milinkovic.tex`
line 168.
- Theorem 1 is the first `thrm`, and every theorem-like environment shares the `thrm` counter.
- The proof at lines 643–650 is the chain the record describes, ending "so all inequalities must
  be equalities". It has seven labelled relations, (i)–(vii), plus the unlabelled
  `d(φ, Id) ≤ ‖H‖`. The record's "chain of seven (in)equalities" counts the labels, which is a
  precision note only.
- `thm:HD` is absent from v3.

**D2.5 (licence).** Both v1 and v3 carry the arXiv non-exclusive licence, so the copy is
`external`.

**D2.6 (authorship).** Three human authors, and the finder is named.

**The class.** FLAW-0012's definition needs a false statement ("a counterexample exists"). The
record reads "the wrong theorem" as false by source document, which is the practice for main's
"wrong"/"incorrect" wording group. So the class depends on the truth reading. If Al rules for the
`undetermined` alternative, FLAW-0012 no longer applies, and the item's class must be argued again
(Q-H1).

**Observation, not a question.** `status.kind` is `erratum`. The status document joins a
withdrawal (v2) and a corrected version (v3); the kind follows the correcting version.

### Items 33–35: candidate 29, arXiv 1403.8075, Proposition 4.1 and Theorems 4.3 and 4.4

**D2.2 (the status document).** Both withdrawal comments match the records:
- v10's comment from `abs-v10.html` (39,877 B, `b01ae085…`), the page bytes;
- v11's comment from the `abs-only.txt` receipt.

**D2.3 (the location).** In `v9/rev_math1.tex` (595 lines), `thm` is numbered within section and
shared by every environment. Section 4, "Distribution of prime numbers" (line 409), holds:

| Number | Kind | Line |
|---|---|---|
| 4.1 | Proposition | 414 |
| 4.2 | Lemma | 438 |
| 4.3 | Theorem | 460 |
| 4.4 | Theorem | 513 |

The quoted sentences all sit where the records say:
- Proposition 4.1's statement, line 415;
- "…equivalent to verifying the first digit of the p_j-ary number is not 0 for certain p_js",
  line 425;
- the independence sentence, line 432;
- Theorem 4.4's "by applying Theorems 3.4 and 4.3", line 524;
- "Theorem 4.3 implies that", line 533.

**The class.**
- Proposition 4.1 is FLAW-0004 with dial `false`. The author now calls the asserted equivalence
  incorrect, and the entry's range makes a false hidden step fatal.
- Theorems 4.3 and 4.4 are FLAW-0005 `gap`. The independence hypothesis of Theorem 3.4 is
  "unjustified", that is, unshown rather than shown false.

The three records are internally consistent: each severity comes from its own entry's range. The
records say this is the practice of main's items 13, 16, 21, 24, 26, 27 and 29; this review did
not re-check those items. The D3 consequence clause would give the opposite reading, "the
derivation withdrawn → fatal". It is the two further severity readings (Q-H2, Q-H3).

**Truth.** All three are `undetermined`, the eleventh to thirteenth uses of ruling (ii). The
notice faults the argument, not the statements.

**Category.** math.GM was read as STATE asked: named results, offered proofs, and a withdrawal by
the author describing a mathematical error (D2.6, D2.7).

### Item 36: candidate 30, arXiv 1404.4599, Proposition 2.22

**D2.2 (the status document).** The v5 withdrawal comment matches the `examine.txt` receipt (the
v5 page receipt is `4f36fad4…`, 40,117 B). The supplementary sentences from the later paper,
arXiv:1806.08664 v6, are in `v6/main.tex`:
- "the proposal of a naive completion operation accounts for the major flaw", line 4043;
- "a serious flaw that was also carried in the arXiv preprints … finally found out in 2019 by
  Julian Bitterlich", lines 4993–4996.

**D2.3 (the location).** In `v4/main.tex` the counter `theo` is numbered within section and shared
by eight environments, which the paper opens through macros (`\bP`, `\bL`, …). Counting
Section 2's environments puts:
- Lemma 2.21 (`amalgamationlem`) at line 2217;
- Proposition 2.22 (`Nacycgroupoidprop`) at line 2304, with the statement at lines 2306–2310 as
  quoted.

The proof's "inductively apply Lemma~\ref{amalgamationlem}" is at line 2316. Lemma 2.21's "It will
be important later that" is at line 2239, and its "arise as substructures of the completions" at
line 2294.

**Class-fit note.**
- The failure sits in what the invoked Lemma 2.21 delivers. Its clause (i) rests on the
  compatibility of completion with reducts, which the notice says fails. The failure is not a
  hypothesis of Lemma 2.21 left unestablished.
- Reading that as FLAW-0005 "hypothesis false" follows main's "rests on a withdrawn result"
  practice, which the record cites. The record names FLAW-0004 (a false asserted stabilisation)
  as the alternative, and the two readings differ only in where the false claim is located.
- The review agrees with the record as written. It does not propose a relabeling.

**Truth.** `undetermined`, with `true` by the later paper named as the alternative (Q-H5).

## 3. Questions for Al (the branch's open readings; not relabelings)

These join the existing truth, severity and unnamed-result questions (STATE, "Older questions";
`analysis/s1-4/config/dispositions.json`). Until Al rules, the branch's labels stand as recorded,
and none is approved.

- **Q-H1 — the eighth truth reading (item 32).** Is "We removed the wrong theorem" plus "an error
  in the assumptions" enough for `false` by source document, or is the theorem withdrawn rather
  than disproved (`undetermined`)?
  - Coupled consequence: under `undetermined`, FLAW-0012 cannot apply, because its definition
    requires falsity.
  - The candidate classes would then be FLAW-0005 (the notice names no step, which D3 does not
    allow to be guessed) or **found-but-unclassifiable**.
- **Q-H2 / Q-H3 — two further severity readings (items 34, 35).** Should FLAW-0005 be `gap` by the
  entry's range (hypothesis unshown), as recorded, or `fatal` by D3's consequence clause (the
  derivation withdrawn)?
- **Q-H4 — further unnamed results.** These results are not named by their notice, so they are not
  items (D2.3); each is affected by the same error:
  - 1403.6317's Corollary 12 and the demoted Theorem 3;
  - 1403.8075's Lemma 4.2;
  - 1404.4599's Lemmas 2.9 and 2.21, where the record locates the flaw.

  They join the unnamed-result question.
- **Q-H5 — item 36's wording.** Is "This problem has been overcome in arXiv:1806.08664" a reason
  for `true`, or does it stay `undetermined` as recorded? The later paper reworks the concepts ("A
  major difference is the absence of a simple completion operation for I-graphs"), and the notice
  does not say Proposition 2.22 stands as stated.
- **Q-H6 — proceed to integration?** Should the five items be integrated as the first later prefix
  under their own record (option (a), the executor's recommendation since Session 22, repeated
  here)? Or should the review stand and the integration wait?

## 4. The integration plan this review implies (nothing here is done)

1. **A fresh branch from `main`, never the held branch.** No cherry-pick, no merge, no
   force-push. The held branch stays at `c3a3d36` as the provenance record.
2. **Sources renumbered after main's last entry.**
   - S-0066..S-0072 on the branch collide with main's S-0066..S-0073 (flate2, AATA, Mathlib,
     DLMF, PolyForm). They become **the seven numbers after S-0073 (0074–0080)**, in the branch's order, with each entry's
     text carried and its numbers changed.
   - The entries keep their Session-22 retrieval dates and digests. The six abstract pages held
     only as receipts stay receipts. An integration session that wants page bytes retrieves them
     under its own access time and records any digest change; arXiv's HTML is not byte-stable.
3. **Configs, hashes, ids and records regenerated.**
   - Every `config.sources`, `status.reference` and `statement_status.reference` changes with the
     renumbering, and so does every `config_hash` and `artifact_id`.
   - The five manifests are rewritten by the producer `s1_ingest` 0.1.0 at the integration
     commit, which becomes their `engine_commit`, and self-checked.
   - The records and log rows are regenerated with the new ids and S-numbers.
   - The originals' bytes stay pinned by the same SHA-256 (§1).
4. **The harvest log appended, not inserted.**
   - The branch inserts rows 28–30 inside the existing table. Its file diverges from the
     `gate-s1-registration` bytes at offset 41,721 (line 87).
   - Under the freeze register (Al's ruling 3 of 2026-10-27 on PR 22),
     `groundtruth/HARVEST-LOG.md` is append-only from the tag, so the branch's file **would fail
     the gate's freeze check as committed**.
   - The integration appends the rows and their counts paragraph after the file's current end,
     with the tag's bytes kept as an exact prefix. The continuation's form is Al's to approve
     (Q-H7).
5. **The session label corrected.** The branch's log, records and counts say "Session 21". On
   `main`, Session 21 is the README session (HANDOFF line 2781), and the harvest was run by
   Session 22 (HANDOFF line 3543). The integration writes "Session 22 (2026-10-21)".
6. **STATE written fresh.** The branch's STATE diff (147 lines, against Session 20's STATE) is not
   applied.
7. **A new inventory snapshot under its own event.**
   - The effective-inventory reproduction pins 35 archived representations and 32 logical items,
     and refuses additions under `groundtruth/` (ADR-0012 D7). A later prefix therefore needs its
     own snapshot and coverage run (ADR-0003 D8, D9).
   - The Gate S1 table and the frozen S1.4 packet are never rewritten.
   - Session 23's conditional projection (36 errata/retraction items; 37 effective logical items,
     40 archived representations) stays a projection until that run measures it.
8. **Freeze-register rows for the new item directories.** The register keeps one frozen row per
   accepted item directory "so later prefixes can add new item directories". Whether the five new
   directories get rows, and at which freeze point, is a register edit under a ruling (Q-H7).
9. **Candidate 31 (`1406.7174`) stays undecided.** The harvest stays paused after candidate 30
   once integrated, or after candidate 27 if it is not.

- **Q-H7 — the two register-facing forms in steps 4 and 8.**
  - The continuation form of the append-only harvest log.
  - Whether, and at which freeze point, the five new item directories join the freeze register.

## 5. Bounds honoured

- The held branch was not checked out, merged or cherry-picked. Its commits, labels and rows are
  unchanged.
- No register status, manifest, frozen packet, retained report, constitution byte, licence or tag
  changed. The accepted corpus (35 / 32) is unchanged, and no generator code was written.
- `contact/` was not opened.
- The seven truth questions, the seven severity questions and the unnamed-result question keep
  their dispositions. This review only adds Q-H1..Q-H7 beside them.
