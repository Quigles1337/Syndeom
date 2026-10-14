# RECORD — Route A item 10 (candidate 12, second documented result): arXiv:1107.0863 v3, Theorem 1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 9 (arXiv `1107.0863`, v3 → v4, Session 15); the same
  original bytes and the same S-entries (S-0038, S-0039); a different claim of record.
- **The claim of record (quoted from v3):** Theorem 1 (TeX label `theorem1`): "Let X be a
  2-dimensional CAT(0) cube complex such that the degrees of all its vertices are uniformly
  bounded by Δ. Then there exists M < ∞, independent of X, such that χ(Γ(X)) ≤ ε(Δ) = MΔ^26. In
  particular, [the 1-skeleton embeds isometrically into a product of at most ε(Δ) trees, and the
  event-structure labelling statement]" (the two further assertions as in the corrected Theorem
  A, which differs "only" by the added hypothesis). The paper's proof is in Section 7 and, per the
  appendix, colours Υ(U) through Proposition 12 of Section 6.4, whose proof applies Lemma 12.
- **Status document (SD12; D2.2):** the v4 comment ("… error in Lemma 12, affecting Theorem 1.
  Current version has appendix correcting Theorem 1 under additional hypothesis …") with v4's
  appendix: "Theorem 1 of [CH] is incorrect in the stated generality due to an error in [Lem. 12].
  We correct this under an extra hypothesis on X"; "The only difference between Theorem A and
  [Thm. 1] is the addition of the hypothesis about 5-cycles"; "The proof proceeds exactly as in
  [Sec. 7], with one difference."
- **Location (D3, `raw/1` locator):** "Theorem 1, statement; its proof in Section 7 through
  Proposition 12 of Section 6.4, which rests on Lemma 12".
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition), lean_catchable YES-KERNEL.** The authors state the theorem "is incorrect in the
  stated generality" and the corrected theorem differs only by an added hypothesis; the original
  proof is the corrected proof with the false Lemma 12 in the place of Proposition A — the entry's
  definition, with the hidden point being the appeal to Lemma 12 (recorded in the locator). The
  examples that show the failure are attributed by the authors to a forthcoming paper by two
  named mathematicians and are not in the document, so the method of record is the status
  document, not a counterexample.
  Other classes considered: FLAW-0005 — Lemma 12 is not applied outside its hypotheses; it is
  applied and is false. FLAW-0011 — no: the statement is documented false, not true.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0039`
  ("incorrect in the stated generality"). **Caveat:** the disproving examples are not in the
  document; if Al prefers to read "incorrect in the stated generality" as the authors' belief
  rather than an established falsity, the value would be `null` / `undetermined` — stated so the
  choice is visible.
- **Proof status:** `invalid`.
- **Human-authored, licence, provenance:** as item 9 (S-0038, S-0039; `external`).
- **What this item is and is not:** the same paper's main theorem, recorded as its own item
  because it is a distinct claim of record whose falsity the authors assert through the lemma's;
  the dependence between items 9 and 10 (one notice, one counterexample) is for the S3 calibration
  ADR's effective-N rule (ADR-0004 D6).
- **Derived canonical artifact (S1.3):** not yet produced. Note for S1.3: Theorem 1's proof is a
  whole section resting on Sections 5–6; the derived artifact's scope (the Section 7 argument
  with the cited lemmas as steps) is an S1.3 decision.
- **Written by:** the executor (Claude Fable 5.1), Session 15; a reading of public documents,
  not a verdict on the paper (SD3).
