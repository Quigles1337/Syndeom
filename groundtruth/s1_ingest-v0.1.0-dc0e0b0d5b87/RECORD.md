# RECORD — Route A item 24 (candidate 23, third documented result): arXiv:1210.6709 v1, Theorem 2.1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 22 (arXiv `1210.6709`, v1 → v2, Session 19); the same
  original bytes and the same S-entries (S-0056, S-0057); a different claim of record.
- **The claim of record (quoted from v1, Section 2, Theorem 2.1, TeX label `pseu`, headed
  "[Oppenheim]"):** "The group of all homeomorphisms of the pseudo-arc, H(P), has a dense
  conjugacy class." The paper's proof, whole: "We first see how Theorem 2.2 implies Theorem
  2.1. Proof of Theorem 2.1. The space Aut(ℙ) can be identified with a dense subspace of H(P).
  This follows from Lemma 4.8 in the Irwin–Solecki paper (take X = P, an arbitrary f_1 ∈ H(P),
  and take f_2 = id)." — a reduction to Theorem 2.2 (item 23) through the dense embedding of
  Aut(ℙ) in H(P).
- **Status document (SD12; D2.2, D2.3):** v2's Remark 2.3 — "We do not know if Aut(ℙ), has a
  dense conjugacy class or if H(P) has a dense conjugacy class. All existing "proofs" seem to
  have a flow." [sic] — names the result by its content and withdraws it as unknown; v2
  replaces the theorem by "The action H(P) ↷ I(P) has a dense orbit." (I(P) the involutions of
  the pseudo-arc) with a new proof through a new Lemma `denseI(p)`.
- **Location (D3, `raw/1` locator):** "Theorem 2.1, proof: the derivation from Theorem 2.2
  through the dense embedding Aut(ℙ) ⊂ H(P)".
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity gap,
  lean_catchable YES-KERNEL.** The proof's only substantive step invokes Theorem 2.2, which the
  author now says is unestablished (item 23); read as in items 13, 16 and 20, the invoked result
  itself is the failed premise. Gap, not fatal: the reduction itself is not faulted by the
  document or by the diff (v2's new proof of the weakened statement uses the same embedding plus
  a density lemma for involutions), and the invoked result is withdrawn as unknown rather than
  shown false — the item-16 shape.
  Other classes considered: FLAW-0011 — not recorded, the statement is not re-proved as
  written; FLAW-0012 — not recorded, no falsity is asserted.
- **Statement status (D3; ADR-0001 as amended):** `value: null`, `method: "undetermined"`,
  `reference: S-0057` (Remark 2.3). The seventh undetermined-truth item; `correct` is `false` by
  derivation.
- **Proof status:** `invalid` (S1.1 by construction; its premise is withdrawn).
- **Human-authored, licence, provenance:** as item 22 (S-0056, S-0057; `external`).
- **What this item is and is not:** the same paper's third documented result, recorded as its
  own item because it is a distinct claim of record about a different group, with its own
  (short) proof; the dependence between items 22, 23 and 24 is for the S3 calibration ADR's
  effective-N rule (ADR-0004 D6), and the S3 obligation for undetermined items applies.
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 19; a reading of public documents,
  not a verdict on the paper (SD3).
