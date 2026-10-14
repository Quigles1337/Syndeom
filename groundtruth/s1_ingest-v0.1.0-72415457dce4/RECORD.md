# RECORD — Route A item 6 (candidate 8, second documented result): arXiv:1005.2398 v2, the lemma following Proposition 3.3.5 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 5 (arXiv `1005.2398`, v2 → v3, Session 14); the same
  original bytes and the same S-entries (S-0032, S-0033); a different claim of record. The two
  items have different `artifact_id`s because their configs differ (`item`, `flaws`); ADR-0001 D2
  requires only that one id never holds two byte sequences.
- **The claim of record:** the lemma immediately following Proposition 3.3.5's proof in Section
  3.3 (its TeX environment carries no label; in the paper's numbering it is the next numbered
  statement of §3.3 after 3.3.5): "Let 𝒳 be an algebraic stack and let 𝒜 → ℬ be a homomorphism
  of quasi-coherent 𝒪_𝒳-algebras. Then the following are equivalent: (1) the ideal sheaf
  ker(𝒜 → ℬ) is locally nilpotent, ker(𝒜 → ℬ) ⊗ ℚ = 0 and 𝒜 → ℬ is universally adequate; (2)
  𝒜 → ℬ is an adequate homeomorphism; (3) Spec_𝒳(ℬ) → Spec_𝒳(𝒜) is an adequate homeomorphism",
  with the proof "This follows from the definitions and fpqc descent using Lemma
  lemma-adequate-algebra-bc and Proposition proposition-adequate-equiv."
- **Status document (SD12; D2.2):** the v3 revision comment ("several minor errors corrected,
  including …") with v3 itself, in which this lemma gains the hypothesis "finitely presented" and
  the footnote "As with Proposition 3.3.5, the published version did not assume that A → B is
  finitely presented and was incorrect." The footnote is part of the corrected version and
  therefore of the status document (ADR-0003 D1, Route A).
- **Diff summary:** the hypothesis "finitely presented" and the footnote are added; the proof is
  unchanged.
- **Location (D3, `raw/1` locators):** instance 1 — "the lemma following Proposition 3.3.5,
  statement"; instance 2 — "the same lemma, proof: the application of Proposition 3.3.5".
- **Class argument (D3):**
  - **Instance 1, FLAW-0012, severity fatal, lean_catchable YES-KERNEL.** The author states the
    version without the hypothesis "was incorrect"; the statement inherits Proposition 3.3.5's
    counterexample through its proof's reduction to it.
  - **Instance 2, FLAW-0005, severity fatal, lean_catchable YES-KERNEL.** The one-line proof
    applies Proposition 3.3.5 to a homomorphism not assumed finitely presented — the proposition's
    (corrected) hypothesis is not available.
  - Other classes considered: FLAW-0003 (circularity) — no; the reduction to the proposition is
    legitimate once the hypothesis is present.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0033`
  (the footnote says "was incorrect"; the counterexample of record belongs to the proposition's
  footnote, so the method here is the document, not a counterexample stated for this lemma).
- **Proof status:** `invalid`.
- **Human-authored, licence, provenance:** as item 5 (S-0032, S-0033; `external`).
- **What this item is and is not:** the same paper's second documented correction, recorded as
  its own item because it is a distinct claim of record; the S1.4 coverage table will count both,
  and the S3 calibration ADR's effective-N rule (ADR-0004 D6) is where their dependence is handled —
  they come from one notice and one paper.
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 14; a reading of public documents,
  not a verdict on the paper (SD3).
