# RECORD — Route A item 26 (candidate 25, first documented result): arXiv:1304.5623 v4, Proposition 3.5 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1304.5623`; original = v4 (16 September 2014, "final
  version"; published as Invent. Math. 200 (2015), 979–1014 per the abstract page); correcting
  version = v5 (8 February 2023, inside the ADR-0003 D8 window), whose appendix is the erratum
  also published as Invent. Math. 223 (2021), 1407–1409 (not retrieved); examined Session 20,
  2026-10-14 (access 21:27:57Z). One author; Algebraic Geometry (math.AG). The same candidate
  yields three further items (Theorem 3.6 Assertion (4), Theorem 4.3, Theorem 5.3), recorded in
  their own directories.
- **Status document (SD12; D2.2):** the v5 revision comment — "due to a mistake in Proposition
  3.5, the unirationality of supersingular K3 surfaces remains a conjecture: see the appendix for
  erratum" — with v5 itself: the new footnote on the introduction and the appendix "Erratum",
  quoted at length in S-0061. S-entries: S-0060 (the original, v4), S-0061 (the corrected version,
  its comment and erratum).
- **Provenance and digests:** abs v4 39,635 B `d67c3a08…bacc`; abs v5 39,394 B `97fabea8…ab4e`;
  e-print v4 33,391 B `86a989c4…4f25` (one gzipped TeX file) — the original, **not committed**;
  e-print v5 34,307 B `531f8bad…80b4`. Full digests in S-0060 and S-0061.
- **The claim of record (quoted from v4, Section 3, Proposition 3.5, TeX label `prop:inseparable
  multisection`):** "We keep the notations and assumptions of Proposition 3.2 and assume that
  Br̂(X) is not p-divisible. Set R := k[[t]] and S := Spec R, and let Ā → Y ×_k S → S be a family
  of A-torsors associated to a non-trivial p-torsion element of Br̂(X)(R) as in Proposition 3.2.
  Then, after possibly replacing S by some finite flat cover (1) There exists a degree p
  multisection D_ξ ⊂ Ā_ξ such that the induced morphism D_ξ → ξ ×_k S is finite, flat and
  radicial of degree p. (2) More precisely, D_ξ → ξ ×_k S is a family of A_ξ[F]-torsors. Thus,
  we obtain an isomorphism Ā_ξ ≅ (A_ξ ×_{(ξ×_k S)} D_ξ) / A_ξ[F], that is, a description of this
  family of A_ξ-torsors as A_ξ[F]-twist." The proof (one page) extends 𝒪_{X′}(pZ) to an
  invertible sheaf, realises degree-p relative Cartier divisors as sections of a ℙ^{p−1}-bundle,
  lets A[F] act on it, determines the fixed-point locus of the A_ξ[F]-action ("On geometric
  fibers, a fixed point is of the form pP"), takes an invariant section D_ξ specialising to pZ,
  and concludes: "By construction, D_ξ → ξ ×_k S is a family of A_ξ[F]-torsors, and in
  particular, finite, flat, and radicial of degree p over ξ ×_k S, which establishes claim (1)."
  The text that follows calls the next theorem, which uses this proposition, "the technical heart
  of this article".
- **The notice's locating and describing sentences (quoted):** "As was pointed out by Bragg and
  Lieblich in [BL], the proof of Proposition 3.5 contains a mistake"; "More precisely, the proof
  only shows the following: [Proposition: … (1) There exists a degree p multisection D_ξ ⊂ Ā_ξ
  such that the induced morphism D_ξ → ξ ×_k S is finite and flat of degree p. (2) More
  precisely, D_ξ → ξ ×_k S arises from a family of A_ξ[p]-torsors.]"; "In particular, contrary to
  what was claimed in Proposition 3.5, one may not be able to choose the degree p multisection
  D_ξ to be radicial, that is, purely inseparable, over ξ ×_k S."
- **Diff summary (executor's reading of the TeX diff v4 → v5, 732 lines):** the body of Section 3
  is unchanged; the changes are the footnote, the appendix, a rearranged introduction, a shorter
  acknowledgement and prose edits. The erratum therefore carries the whole location.
- **Location (D3, `raw/1` locator):** "Proposition 3.5, proof, the sentence 'By construction, D_ξ
  → ξ ×_k S is a family of A_ξ[F]-torsors, and in particular, finite, flat, and radicial of
  degree p …'" — the step whose conclusion the erratum weakens from A_ξ[F]-torsors (the Frobenius
  kernel, hence radicial) to A_ξ[p]-torsors (the p-torsion subgroup scheme, finite flat).
- **Class argument (D3):** **FLAW-0004 (unjustified step), severity gap, lean_catchable
  YES-KERNEL.** The sentence asserts, on the hedge "By construction", that the invariant section
  is a torsor under the infinitesimal group scheme A_ξ[F]; the erratum says the construction
  gives only a torsor family under A_ξ[p], so the Frobenius-kernel structure — and with it the
  radiciality that claim (1) states — is asserted without justification. Gap, not fatal: the
  erratum does not say the hidden step is false ("one may not be able to choose … radicial") and
  gives no example; the dial is unset for the same reason. The result is weakened, which D3's
  consequence clause reads as fatal; the record follows the entry's range as in items 13, 16 and
  21 (the escalation on this tension stands in HANDOFF Session 19).
  Other classes considered: FLAW-0005 (a torsor criterion applied without its hypothesis) — the
  sentence names no criterion, so FLAW-0004 fits better; FLAW-0012 — not recorded, the statement
  is not said to be false; FLAW-0008 (A_ξ[F] for A_ξ[p], a drift between two group schemes) is
  the formal-pair class and is named here only as a description of the slip.
- **Statement status (D3; ADR-0001 as amended 2026-10-13):** `value: null`,
  `method: "undetermined"`, `reference: S-0061` — the author replaces the proposition by a weaker
  one and says the stronger one "may not" hold; neither re-proved nor disproved. The eighth
  undetermined-truth item; `correct` is `false` by derivation.
- **Proof status:** `invalid` (S1.1 by construction; the author says the proof contains a mistake).
- **Human-authored (D2.6):** one human author; no AI-assistance mention (the scanner reports no
  hit on either version); the mistake was pointed out by two named mathematicians in a cited
  paper (Bragg and Lieblich, Compos. Math. 158 (2022)), not retrieved.
- **Licence (D5, SD16):** v4 and v5 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1304.5623v4.tex.gz`, 33,391 B, `86a989c4…4f25`). Recorded in S-0060 and
  S-0061 before ingestion.
- **What this item is and is not:** a real, human-authored, published paper whose author adds a
  published erratum nine years on, withdrawing the main theorem to a conjecture because an
  auxiliary proposition's proof shows less than it claims. Nothing here assesses Bragg and
  Lieblich's paper or the later literature on the conjecture (SD3); the claim of record is
  Proposition 3.5 as stated in v4.
- **Derived canonical artifact (S1.3):** not yet produced; the proposition and its proof are
  about seventy source lines and depend on Propositions 3.2 and 3.4 and on the Picard-scheme
  facts the proof cites.
- **Written by:** the executor (Claude Fable 5.1), Session 20; a reading of public documents,
  not a verdict on the paper (SD3).
