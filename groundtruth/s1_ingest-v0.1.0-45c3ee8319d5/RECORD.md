# RECORD — Route A item 22 (candidate 23, first documented result): arXiv:1210.6709 v1, Theorem 1.6 — "Theorem 1.7" of the v2 comment (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1210.6709`; original = v1 (25 October 2012);
  correcting version = v2 (20 April 2023, inside the ADR-0003 D8 window, under a new title);
  examined Session 19, 2026-10-14 (access 20:15:06Z). One author; Logic (math.LO), cross-listed
  General Topology; no journal reference on arXiv. The same candidate yields two further items
  (Theorems 2.2 and 2.1), recorded in their own directories.
- **Status document (SD12; D2.2):** the v2 revision comment — "Statement of Theorem 1.7 is
  corrected" — with v2 itself: the new title ("Large orbits …" for "Large conjugacy classes …"),
  the new abstract, Theorem 1.7 as restated, and Remark 2.3: "We do not know if Aut(ℙ), has a
  dense conjugacy class or if H(P) has a dense conjugacy class. All existing "proofs" seem to
  have a flow." [sic]. S-entries: S-0056 (the original, v1), S-0057 (the corrected version, its
  comment and remark).
- **Provenance and digests:** abs v1 38,596 B `a50b9233…f9e3`; abs v2 38,086 B `decce1a9…1556`;
  e-print v1 19,732 B `3c24927f…bb34` (one gzipped TeX file) — the original, **not committed**;
  e-print v2 20,021 B `a52fadc2…8288`. Full digests in S-0056 and S-0057.
- **Numbering (resolved by the executor):** v1 has no Theorem 1.7 — Section 1's numbered
  environments run from Theorem 1.1 (Irwin–Solecki) to Theorem 1.6, the main result; v2 inserts
  Proposition 1.3 (`B-CThm`, cited from Basso–Camerlo) and the main result becomes Theorem 1.7
  there. The comment's "Theorem 1.7" is therefore v1's Theorem 1.6 (TeX label `abcd`), which
  Section 3 restates as Theorem 3.1 (label `maint`) before proving it.
- **The claim of record (quoted from v1, Theorem 1.6 = Theorem 3.1):** "The group of all
  automorphisms of ℙ, Aut(ℙ), has a comeager conjugacy class." (ℙ the projective Fraïssé limit
  of the family 𝓖 of finite reflexive linear graphs, whose natural quotient is the pseudo-arc.)
  The introduction: "The main result of the present paper is the following theorem … This
  result strengthens the result of Oppenheim that Aut(ℙ) has a dense conjugacy class." The
  proof (Section 3, about eight pages) is split into Proposition 3.2 (`ap`: the class 𝓕 of
  finite structures (A, s^A) induced by automorphisms has the coinitial amalgamation property;
  "More precisely, the coinitial family 𝓓, defined in Lemma 3.4 below, has the AP") and
  Proposition 3.3 (`comeager`: CAP and JPP for 𝓕 imply a comeager conjugacy class), the JPP
  being Proposition 2.5 of Section 2. Lemma 3.4 opens: "We know already that 𝓓_0 = {(A, s^A) ∈
  𝓕: s^A is the antidiagonal of A} is coinitial in 𝓕 (Lemma 2.12)."
- **The notice's locating and describing sentences (quoted):** "Statement of Theorem 1.7 is
  corrected"; v2's Theorem 1.7: "The action Aut(ℙ) ↷ I(ℙ) has a comeager orbit." (I(ℙ) the set
  of involutions of ℙ, on which Aut(ℙ) acts by conjugation); Remark 2.3 as quoted above.
- **Diff summary (executor's reading of the TeX diff v1 → v2, 1,047 lines):** everything about
  conjugacy classes in Aut(ℙ) becomes the conjugacy action on I(ℙ). The class 𝓕 is redefined
  with f ∈ I(ℙ) in place of f ∈ Aut(ℙ). In Section 2, "symmetric" is added to the hypotheses of
  Lemma 2.12 (`slawek`: "For any (A, s^A) with A ∈ 𝓖 and s^A surjective and connected there is
  (B, s^B) such that: s^B is the antidiagonal of B and there exists an epimorphism from (B, s^B)
  onto (A, s^A)") and to the characterisation Lemma 2.13 (`pse`: "(A, s^A) ∈ 𝓕 if and only if
  s^A is surjective and connected"); a new Proposition `lifttoinv` (an antidiagonal structure
  lifts to an involution of ℙ, via a new Lemma `switch`) replaces v1's lifting argument in Lemma
  2.13 and is invoked in Proposition 2.5; a new Lemma `denseI(p)` and a new proof of Theorem
  2.1 are added. Section 3 keeps its propositions with Aut(ℙ) replaced by I(ℙ). No sentence
  names the failing step.
- **Location (D3, `raw/1` locator):** "Theorem 1.6 = Theorem 3.1, proof (Section 3): Lemma
  3.4's claim that 𝓓_0 is coinitial in 𝓕 '(Lemma 2.12)' and, through Proposition 2.5 (the
  JPP), the step 'Without loss of generality, s^A and s^B are antidiagonals'" — the uses of
  Lemma 2.12 on members of v1's class 𝓕.
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity fatal,
  lean_catchable YES-KERNEL.** The proof's reduction to antidiagonal structures rests on Lemma
  2.12 applied to every (A, s^A) ∈ 𝓕. **Executor's reading, labelled as such and not the
  author's:** an epimorphism carries a relation to its image, and the image of an antidiagonal
  — a symmetric relation — is symmetric; so Lemma 2.12's conclusion forces s^A symmetric, which
  the restriction of a non-involutive automorphism to a finite quotient need not be (v1's 𝓕
  contains such structures by definition). The lemma is thus applied to objects for which the
  hypothesis it needs — the one v2 adds — is false, which is the entry's fatal case, and the
  coinitiality of 𝓓_0 in 𝓕 and the JPP of 𝓕 as v1 argues them fail with it. v2's restriction
  of the whole paper to involutions, whose graphs are symmetric, is consistent with this reading;
  the document itself says only that the statement is corrected and that the existing proofs
  seem flawed. Severity fatal also by D3's consequence rule: the result is withdrawn and
  replaced by a different statement.
  Other classes considered: FLAW-0012 for Lemma 2.12 itself (a lemma false as stated, with a
  proof credited to a colleague) is the natural companion reading and is not recorded as an item
  because the notice does not name the lemma; FLAW-0012 for the theorem — not recorded, the
  author does not say the statement is false and gives no counterexample; FLAW-0002 (a
  quantifier over all automorphisms where only involutions were handled) describes the same
  defect at the statement's level and is not recorded separately.
- **Statement status (D3; ADR-0001 as amended 2026-10-13):** `value: null`,
  `method: "undetermined"`, `reference: S-0057` — the author replaces the statement by one about
  the action on involutions and writes that whether Aut(ℙ) has even a dense conjugacy class is
  not known; a comeager class would be dense, so the original claim is open. The fifth
  undetermined-truth item; `correct` is `false` by derivation.
- **Proof status:** `invalid` (S1.1 by construction; the author withdraws the statement and
  reworks the proof for a different one).
- **Human-authored (D2.6):** one human author; no AI-assistance mention (the scanner reports no
  hit on either version); the finder of the flaw is unstated.
- **Licence (D5, SD16):** v1 and v2 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1210.6709v1.tex.gz`, 19,732 B, `3c24927f…bb34`). Recorded in S-0056 and
  S-0057 before ingestion.
- **What this item is and is not:** a real, human-authored paper whose author, ten years on,
  replaces its main theorem by a weaker one under a new title and records that the stronger
  claims — her own and an unpublished one of Oppenheim's — are open. Nothing here assesses v2's
  results (SD3); the claim of record is Theorem 1.6 as stated in v1.
- **Derived canonical artifact (S1.3):** not yet produced; the theorem's proof spans Sections 2
  and 3 and depends on Irwin–Solecki's projective Fraïssé theory and the Kechris–Rosendal
  criterion (Proposition 3.5).
- **Written by:** the executor (Claude Fable 5.1), Session 19; a reading of public documents,
  not a verdict on the paper (SD3).
