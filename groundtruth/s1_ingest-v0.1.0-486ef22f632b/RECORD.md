# RECORD — Route A item 23 (candidate 23, second documented result): arXiv:1210.6709 v1, Theorem 2.2 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 22 (arXiv `1210.6709`, v1 → v2, Session 19); the same
  original bytes and the same S-entries (S-0056, S-0057); a different claim of record.
- **The claim of record (quoted from v1, Section 2, Theorem 2.2, TeX label `aut`, headed
  "[Oppenheim]"):** "The group of all automorphisms of ℙ, Aut(ℙ), has a dense conjugacy class."
  The section opens: "In this section we present a short and simple proof of an unpublished
  result of Oppenheim that the group of all automorphisms of ℙ has a dense conjugacy class …
  Both Oppenheim's proof and the proof presented here use the projective Fraïssé theory." The
  paper's proof: "We split the proof of Theorem 2.2 into two propositions" — Proposition 2.5
  (`jpp`: "The family 𝓕 has the JPP") and Proposition 2.6 (`dense`: "The property JPP for 𝓕
  implies Aut(ℙ) has a dense conjugacy class"). Proposition 2.5's proof opens "Take (A, s^A),
  (B, s^B) ∈ 𝓕. Then s^A and s^B are surjective and connected. Without loss of generality, s^A
  and s^B are antidiagonals of A and B, respectively." and builds the joint structure as an
  antidiagonal on [kn]; Lemma 2.13 (`pse`) characterises 𝓕 as the surjective connected
  relations, its "other direction" lifting any such (A, s^A) through an antidiagonal (B, s^B)
  obtained from Lemma 2.12 (`slawek`). So the result is attributed to Oppenheim but the offered
  proof is the paper's own (D2.1).
- **Status document (SD12; D2.2, D2.3):** v2's Remark 2.3 — "We do not know if Aut(ℙ), has a
  dense conjugacy class or if H(P) has a dense conjugacy class. All existing "proofs" seem to
  have a flow." [sic] — names the result by its content, withdraws it as unknown, and says its
  proofs (this paper's among them) are flawed; v2 replaces the theorem by "The action
  Aut(ℙ) ↷ I(ℙ) has a dense orbit." and drops the "[Oppenheim]" attribution. The comment names
  only Theorem 1.7; the remark is part of the corrected version, which D2.2 makes part of the
  status document.
- **Location (D3, `raw/1` locator):** "Theorem 2.2, proof via Propositions 2.5 and 2.6:
  Proposition 2.5's step 'Without loss of generality, s^A and s^B are antidiagonals' (Lemma
  2.12) and Lemma 2.13's 'other direction' (the lift through an antidiagonal from Lemma 2.12)".
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity fatal,
  lean_catchable YES-KERNEL** — the same application of Lemma 2.12 to members of v1's class 𝓕
  as in item 22, here at the JPP and at the characterisation of 𝓕. **Executor's reading, as in
  item 22 and labelled as such:** the image of an antidiagonal under an epimorphism is
  symmetric, so Lemma 2.12 cannot hold for a non-symmetric s^A, and 𝓕 as v1 defines it contains
  such relations; v2 adds "symmetric" to Lemmas 2.12 and 2.13 and lifts, through the new
  Proposition `lifttoinv`, to an involution rather than to an arbitrary automorphism. Fatal: the
  needed hypothesis is false for objects in the proof's scope, and the result is withdrawn.
  Other classes considered: as in item 22 (FLAW-0012 for the lemma, FLAW-0002 for the
  statement), not recorded for the same reasons.
- **Statement status (D3; ADR-0001 as amended):** `value: null`, `method: "undetermined"`,
  `reference: S-0057` (Remark 2.3). The sixth undetermined-truth item; `correct` is `false` by
  derivation.
- **Proof status:** `invalid` (S1.1 by construction; the author says the proofs are flawed and
  replaces the statement).
- **Human-authored, licence, provenance:** as item 22 (S-0056, S-0057; `external`).
- **What this item is and is not:** the same paper's second documented result, recorded as its
  own item because it is a distinct claim of record (density rather than comeagerness, and a
  claim the author attributes to Oppenheim while proving it herself) whose proof is a different
  argument; the dependence between items 22, 23 and 24 (one notice, one lemma) is for the S3
  calibration ADR's effective-N rule (ADR-0004 D6), and the S3 obligation for undetermined items
  applies. Nothing here assesses Oppenheim's unpublished proof, which the record has not seen
  (SD3).
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 19; a reading of public documents,
  not a verdict on the paper (SD3).
