# RECORD — Route A item 27 (candidate 25, second documented result): arXiv:1304.5623 v4, Theorem 3.6, Assertion (4) (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 26 (arXiv `1304.5623`, v4 → v5, Session 20); the same
  original bytes and the same S-entries (S-0060, S-0061); a different claim of record.
- **The claim of record (quoted from v4, Section 3, Theorem 3.6, TeX label `thm:moving torsors
  general`, Assertion (4)):** under the theorem's standing hypotheses (the notations and
  assumptions of Proposition 3.2, Br̂(X) not p-divisible, R := k[[t]], S := Spec R, a family of
  A-torsors Ā → Y ×_k S → S associated to a non-zero p-torsion element of Br̂(X)(R); "after
  possibly replacing S by a finite and flat cover") — "(4) There exist rational maps
  X̄_η̄^{(1/p)} ⇢ X ×_{Spec k} η̄ ⇢ X̄_η̄^{(p)}, both of which are generically finite and purely
  inseparable of degree p², that is, both maps are purely inseparable isogenies of height 2."
  Assertions (1)–(3) of the theorem (a smooth projective compactification X̄ → Y ×_k S with
  special fibre X; the Picard and formal Brauer behaviour under specialisation; a morphism and a
  rational map Y → X ×_k η and Y ⇢ X̄_η "both of which are generically finite and purely
  inseparable of degree p") are not withdrawn by the erratum and are not part of the claim of
  record. The proof of (4): "After possibly replacing S by a finite flat cover, there exists a
  purely inseparable degree-p multisection D_ξ ⊂ Ā_ξ by Proposition 3.5, and we denote by D its
  closure in X̄_η. Since D_ξ → ξ ×_k S is finite, flat and radicial of degree p, the same is true
  for D → Y ×_k η. … Let F: X̄_η^{(1/p)} → X̄_η be the relative Frobenius morphism over η and note
  that F factors through X̄_η ×_{(Y×_k η)} D … From this, we obtain a composition X̄_η^{(1/p)} →
  X̄_η ×_{(Y×_k η)} D ⇢ X ×_{Spec k} η, which is a rational map … purely inseparable of degree p²,
  that is, a purely inseparable isogeny of height 2. Since twice the Frobenius morphism factors
  through this isogeny, we obtain claim (4)."
- **Status document (SD12; D2.2, D2.3):** the erratum (S-0061): "Now, in order to show that
  unirationality is preserved in these one-dimensional families, one needs Assertion (4) of
  Theorem 3.6, whose proof requires that one can find degree p multisections in Proposition 3.5
  that are radicial over the base. Without this, one can only conclude that any two supersingular
  K3 surfaces in characteristic p ≥ 5 are connected by a sequence of very special degree p
  correspondences (as in Assertion (3) of Theorem 3.6), which is not sufficient for
  unirationality." The erratum names the assertion and the premise its proof needs.
- **Location (D3, `raw/1` locator):** "Theorem 3.6, proof of Assertion (4): the invocation of
  Proposition 3.5 for a *purely inseparable* degree-p multisection and the factorisation of the
  relative Frobenius through X̄_η ×_{(Y×η)} D that it licenses".
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity gap,
  lean_catchable YES-KERNEL.** The step invokes Proposition 3.5's claim (1) in its radicial form;
  by the author's own erratum that claim is not established (item 26) — read as in items 13, 16,
  20 and 24, the invoked result is the failed premise. Gap, not fatal: the premise is withdrawn
  as unproved rather than shown false, and the assertion itself is left open ("one can only
  conclude …"); D3's consequence clause ("weakened → fatal") is read as in the sibling items
  (the escalation of HANDOFF Session 19 stands).
  **Executor's observation, labelled as such and not the author's:** the proof of Assertion (3)
  also takes "the morphisms on the left and right are purely inseparable of degree p" from the
  radiciality of D; the erratum nevertheless treats (3) as surviving in the form of "very
  special degree p correspondences". (3) is therefore not recorded (the document does not
  withdraw it), and this reading is not the basis of any class.
  Other classes considered: FLAW-0012 — not recorded, no falsity is asserted; FLAW-0004 — the
  step's justification is named (Proposition 3.5), so FLAW-0005 fits.
- **Statement status (D3; ADR-0001 as amended):** `value: null`, `method: "undetermined"`,
  `reference: S-0061`. The ninth undetermined-truth item; `correct` is `false` by derivation.
- **Proof status:** `invalid` (S1.1 by construction; its premise is withdrawn).
- **Human-authored, licence, provenance:** as item 26 (S-0060, S-0061; `external`).
- **What this item is and is not:** the same paper's second documented result, recorded as its
  own item because the erratum names it and its proof separately from Proposition 3.5 and because
  it is the step at which the unirationality strategy breaks; the dependence between items 26,
  27, 28 and 29 (one erratum) is for the S3 calibration ADR's effective-N rule (ADR-0004 D6).
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 20; a reading of public documents,
  not a verdict on the paper (SD3).
