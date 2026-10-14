# RECORD — Route A item 20 (candidate 21, second documented result): arXiv:1209.2022 v2, Theorem 3.2 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 19 (arXiv `1209.2022`, v2 → v3, Session 18); the same
  original bytes and the same S-entries (S-0052, S-0053); a different claim of record.
- **The claim of record (quoted from v2, Subsection 3.1, Theorem 3.2, TeX label `braids are
  unitary`):** after the definition "A braiding over a unitary fusion category 𝒞 is called
  unitary braiding if the morphism c_{X,Y} is unitary for any pair of objects X, Y ∈ 𝒞" — "Every
  braiding of a unitary fusion category is unitary." The proof, whole: "Let 𝒞 be a unitary fusion
  category and let c be a braiding. It is easy to see that the braiding c defines an inclusion
  functor 𝒞 ↪ 𝒵(𝒞), X ↦ (X, c_{X,−}). Proposition 3.1 implies that c_{X,W} is unitary for every
  W ∈ 𝒞." The theorem is the note's first main result (the abstract's "every braiding over a
  unitary fusion category is unitary"; the introduction's footnote says it answers a problem
  posed by Zhenghan Wang).
- **Status document (SD12; D2.2):** the v3 comment's last sentence, "The proof of Theorem 3.2 has
  been revised to avoid reliance on this proposition", with v3's new proof. The statement is
  word-for-word the same in v3 (less the final full stop). The new proof keeps the object
  (X, c_{−,X}) of 𝒵(𝒞), takes an isomorphic object (Y, τ_{−,Y}) of 𝒵*(𝒞) by the cited
  equivalence, shows τ_{−,Y} = c_{−,Y} "Using the naturality of c_{−,X}, c_{−,Y}, and τ_{−,Y}",
  applies Remark 2.1 to f: X → Y in 𝒞 to get a unitary u, and concludes
  c_{S,X} = (u ⊗ id_S)* c_{S,Y} (id_S ⊗ u) for every S, "Since c_{S,Y} is unitary and u is also
  unitary".
- **Location (D3, `raw/1` locators):** instance 1 — "Theorem 3.2, proof" (replaced whole);
  instance 2 — "Theorem 3.2, proof, the sentence 'Proposition 3.1 implies that c_{X,W} is unitary
  for every W ∈ 𝒞'".
- **Class argument (D3):** **FLAW-0011 (true theorem with invalid proof), severity gap (a step the
  fix had to re-prove), lean_catchable YES-KERNEL (per the embedded flaw)** — the statement
  stands (the corrected version proves the same statement) and the offered proof carries a flaw:
  **FLAW-0005 (theorem applied without its hypotheses), severity gap, lean_catchable
  YES-KERNEL** — the proof's only non-trivial step invokes Proposition 3.1, which the author
  states is false (item 19); read as in items 13 and 16, the invoked result itself is the failed
  premise, and the v3 proof supplies what it was used for (that the half-braiding c_{−,X} is
  unitary) by a direct argument inside the center. Gap, not fatal: the theorem is true and the
  fix re-proves the step. The same shape as item 2 (a true theorem whose proof rested on an
  identification the corrected proof establishes).
  Other classes considered: FLAW-0012 — not recorded, the statement is re-proved; FLAW-0010 —
  the cited proposition exists and says what the proof uses, it is simply false; FLAW-0003 — no
  circularity, the new proof uses only the cited external equivalence and naturality.
- **Statement status (D3):** `value: true`, `method: "source-document"`, `reference: S-0053` —
  the corrected version proves the same statement and the comment says the proof, not the
  theorem, was revised; `correct` is `false` by derivation (the proof is invalid).
- **Proof status:** `invalid` (S1.1 by construction; the author replaces the proof).
- **Human-authored, licence, provenance:** as item 19 (S-0052, S-0053; `external`).
- **What this item is and is not:** the same note's second documented result, recorded as its
  own item because it is a distinct claim of record whose status differs from Proposition 3.1's
  (true rather than false) and because it is the note's main theorem; the dependence between
  items 19 and 20 (one notice, one false proposition) is for the S3 calibration ADR's effective-N
  rule (ADR-0004 D6). Nothing here assesses Theorem 3.4 or the rest of the note (SD3).
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 18; a reading of public documents,
  not a verdict on the paper (SD3).
