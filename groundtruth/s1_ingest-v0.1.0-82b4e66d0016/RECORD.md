# RECORD — Route A item 28 (candidate 25, third documented result): arXiv:1304.5623 v4, Theorem 4.3 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 26 (arXiv `1304.5623`, v4 → v5, Session 20); the same
  original bytes and the same S-entries (S-0060, S-0061); a different claim of record.
- **The claim of record (quoted from v4, Section 4 "Moduli spaces", Theorem 4.3, TeX label `thm:
  bundle structure`):** "Let N and N_+ be the supersingular K3 lattices in odd characteristic p
  of Artin-invariants σ_0 and σ_0+1, respectively. Then, there exists a rank 20 lattice Λ, and
  orthogonal decompositions N ≅ U ⊕ Λ and N_+ ≅ U(p) ⊕ Λ, where U denotes the hyperbolic plane.
  These decompositions gives rise to a surjective morphism ϖ_N of moduli spaces of rigidified K3
  crystals with a section σ_N [𝓜_{N_+} → 𝓜_N with σ_N: 𝓜_N → 𝓜_{N_+}] which turns 𝓜_{N_+} into
  a ℙ¹-bundle over 𝓜_N." The introduction states it as an unnumbered theorem ("… together with a
  section, which turns 𝓜_{N_+} into a ℙ¹-bundle over 𝓜_N. In particular, 𝓜_N and 𝓜_{N_+} are
  iterated ℙ¹-bundles over Spec 𝔽_{p²}" — the second sentence dropped in v5), and Remark 4.4
  tabulates 𝓜_N for σ_0 = 1, 2, 3 as a point, ℙ¹ and ℙ¹ × ℙ¹ over 𝔽_{p²}. The proof runs in
  six steps on characteristic subspaces of the lattices' 𝔽_p-reductions; Step 6 is headed "ϖ_N
  defines a ℙ¹-bundle structure".
- **Status document (SD12; D2.2, D2.3):** the erratum's second part, "Fibration structure"
  (S-0061): "Moreover, the proof of Theorem 4.3 contains a mistake. In fact, the proof only shows
  the following: [Theorem: … This decomposition gives rise to a dominant rational map ϖ_N of
  moduli spaces of rigidified K3 crystals with a section σ_N … More precisely, (1) this map is
  defined on the open set of 𝓜_{N+1} of rigidified K3 crystals with Artin invariant σ_0+1 and
  maps surjectively onto the open set of rigidified K3 crystals of Artin invariant σ_0. (2) The
  normalization of the geometric generic fibre is ℙ¹. (In general, ϖ_N may not be generically
  smooth.)] In the proof, the map is only defined on geometric points and extension to a
  morphism would require more arguments. In particular, the computations in Step 6 of the proof
  only show that the normalisation of the geometric generic fibre is isomorphic to ℙ¹. In
  particular, the table Remark 4.4 has to be corrected as follows: for σ_0(N) = 3, the moduli
  space 𝓜_N consists geometrically of two copies of the Fermat hypersurface of degree (p+1) in
  ℙ³. In this case, ϖ_N arises geometrically by choosing a pencil of planes through a line of this
  surface and then, one obtains a fibration over ℙ¹, whose geometric generic fibre is a singular
  rational curve that is not smooth. This gives an example, where the geometric generic fibre of
  ϖ_N is not smooth, and in particular, not isomorphic to ℙ¹."
- **Diff summary:** as item 26 — Section 4's body is unchanged in v5; the erratum carries the
  location.
- **Location (D3, `raw/1` locators):** instance 1 — "Theorem 4.3, statement: the conclusion that
  ϖ_N is a surjective morphism turning 𝓜_{N_+} into a ℙ¹-bundle over 𝓜_N" (the lattice
  decompositions are not withdrawn); instance 2 — "Theorem 4.3, proof, Step 6 'ϖ_N defines a
  ℙ¹-bundle structure'"; instance 3 — "Theorem 4.3, proof, the definition of ϖ_N on
  characteristic subspaces (geometric points)".
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition), lean_catchable YES-KERNEL** for the statement: the erratum gives an example in
  which the constructed ϖ_N has a singular geometric generic fibre, so it is not a ℙ¹-bundle, and
  replaces the conclusion by a dominant rational map with ℙ¹ as the *normalisation* of the generic
  fibre; the six-step proof is a lattice-and-crystal computation of the standard kind — the
  entry's shape. **FLAW-0004 (unjustified step), severity fatal, lean_catchable YES-KERNEL** for
  Step 6: by the erratum, the computations there "only show that the normalisation of the
  geometric generic fibre is isomorphic to ℙ¹", while the step concludes a ℙ¹-bundle — a hidden
  step (smoothness of the fibre) that the example shows false (the entry's `false` dial).
  **FLAW-0004, severity gap, lean_catchable YES-KERNEL** for the definition of ϖ_N: "the map is
  only defined on geometric points and extension to a morphism would require more arguments" —
  a non-trivial step without justification whose truth the erratum leaves open (the corrected
  theorem asserts a dominant rational map). Three records for one erratum paragraph, each with its
  own location, as D3 allows.
  Other classes considered: FLAW-0002 (a quantifier slip between "for every geometric point" and
  "as a morphism of schemes") describes instance 3's mechanism and is not recorded separately;
  FLAW-0005 — no named theorem is misapplied.
- **Statement status (D3):** `value: false`, `method: "counterexample"`, `reference` = the
  erratum's σ_0(N) = 3 example (S-0061). **Caveat:** the example concerns the map ϖ_N that the
  proof constructs from the decompositions — which is what the statement's "These decompositions
  gives rise to …" asserts — so the record reads it as a counterexample to the statement; a
  reader who takes the statement as a bare existence claim for *some* ℙ¹-bundle structure would
  record `null`/`undetermined` instead. The third counterexample-method item (after items 9 and
  19).
- **Proof status:** `invalid` (S1.1 by construction; the author says the proof contains a
  mistake).
- **Human-authored, licence, provenance:** as item 26 (S-0060, S-0061; `external`); the finder
  of this second mistake is not named (the erratum attributes only the Proposition 3.5 mistake to
  Bragg and Lieblich).
- **What this item is and is not:** the same paper's third documented result, independent of the
  Proposition 3.5 chain (the erratum introduces it with "Moreover"), recorded as its own item
  because the erratum names it, weakens it and exhibits an example. Theorem 4.5 (the moduli
  interpretation), which keeps Theorem 4.3's notations, is not named by the erratum and is not
  recorded (SD3).
- **Derived canonical artifact (S1.3):** not yet produced; the theorem's proof is about two
  hundred source lines and depends on Ogus's crystalline Torelli theory as the paper cites it.
- **Written by:** the executor (Claude Fable 5.1), Session 20; a reading of public documents,
  not a verdict on the paper (SD3).
