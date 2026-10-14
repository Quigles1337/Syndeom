# RECORD — Route A item 12 (candidate 15, first documented result): arXiv:1201.3327 v2, the former Lemma 5.8 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1201.3327`; original = v2 (18 November 2013);
  correcting version = v3 (5 May 2023, inside the ADR-0003 D8 window); examined Session 16,
  2026-10-14 (access 04:53:37Z). One author; Number Theory (math.NT). The same candidate yields a
  second item (the former Theorem 5.9), recorded in its own directory.
- **Status document (SD12; D2.2):** the v3 revision comment — "There was an error in the proof
  of the former Lemma 5.8. This false lemma and the former Theorem 5.9 have been deleted in this
  version" — with v3 itself: the acknowledgement's "In the published version of this article
  there is an error in the proof of the former Lemma 5.8. This was noticed by Francesco Amoroso
  and Lea Terracini produced a counterexample to prove that the whole statement was false." and
  the in-text note "the claimed Lemma 5.8 in the previous version of this article is not true - a
  counterexample was found by Francesco Amoroso and Lea Terracini". S-entries: S-0042 (the
  original, v2), S-0043 (the corrected version, its comment and notes).
- **Provenance and digests:** abs v2 42,130 B `0fb78c5d…5168`; abs v3 41,990 B `0ae768a0…4c3d`;
  e-print v2 16,122 B `f15a4756…db33` (one gzipped TeX file) — the original, **not committed**;
  e-print v3 16,661 B `a23aec07…6e7e`. Full digests in S-0042 and S-0043.
- **The claim of record (quoted from v2, Section 5):** "Lemma 5.8. Let K be a number field with
  non-archimedian absolute value v. Let α be an algebraic number and let w₁, …, w_n be a complete
  set of extensions of v to K(α). Then we have K^{nr,v}(α) = ⋂_{i=1}^{n} K(α)^{nr,w_i}." The proof
  is one paragraph: the inclusion ⊆ from an earlier lemma on ramification, then an argument on a
  maximal unramified subextension L^w and the totally ramified extension K(α, β)_w / L^w.
- **The notice's locating and describing sentences (quoted):** "an error in the proof of the
  former Lemma 5.8"; "the whole statement was false"; "the claimed Lemma 5.8 … is not true - a
  counterexample was found".
- **Diff summary (executor's reading of the TeX diff v2 → v3):** the lemma, its proof, the
  sentence "If we combine this lemma with the preceding results we obtain" and Theorem 5.9 are
  removed; a capitalised note that the following is not equivalent to the published version, two
  sentences of explanation, and a weaker Proposition (TeX label `thm:finex`) replace them; the old
  proof of the theorem and a remark survive as TeX comments; two bibliography entries change.
- **Location (D3, `raw/1` locator):** "the former Lemma 5.8, statement and proof"; the notice
  locates the error to the proof without naming the step, and the executor does not supply one.
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition), lean_catchable YES-KERNEL.** The statement is false by the author's own account,
  resting on a counterexample produced by named mathematicians and accepted by the author; the
  offered proof is a short argument of the usual kind for such field-theoretic identities —
  the entry's definition. The failing step is not named by the notice and is not recorded; the
  counterexample is not reproduced in the document, so the method of record is the document.
  Other classes considered: FLAW-0005 (a hypothesis of an applied result unmet) — plausible for
  the step that concludes equality from an inequality of ramification indices, but a guess; not
  recorded.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0043`
  ("the whole statement was false"; "is not true"). The counterexample method is not used
  because the counterexample itself is not in the document.
- **Proof status:** `invalid` (S1.1 by construction; the author documents the error in the proof).
- **Human-authored (D2.6):** one human author; no AI-assistance mention (the scanner's "llm" hit
  is the surname Ullmo in the acknowledgement, checked); the error was noticed and the
  counterexample produced by two named mathematicians, per the author.
- **Licence (D5, SD16):** v2 and v3 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1201.3327v2.tex.gz`, 16,122 B, `f15a4756…db33`). Recorded in S-0042 and
  S-0043 before ingestion.
- **What this item is and is not:** a real, human-authored, published paper whose author removes
  a false lemma ten years after publication and says so in the text. Nothing here assesses the
  paper's other results (SD3); the claim of record is the former Lemma 5.8.
- **Derived canonical artifact (S1.3):** not yet produced; the lemma and its proof are one
  paragraph each.
- **Written by:** the executor (Claude Fable 5.1), Session 16; a reading of public documents,
  not a verdict on the paper (SD3).
