# RECORD — Route A item 3 (candidate 4): arXiv:0902.0680 v1, the dense-subspace lemma (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A (arXiv revision comments); arXiv `0902.0680`; original =
  v1 (4 February 2009); v2 (29 June 2011) is a withdrawal; correcting version = v3 (30 September
  2026, the last day of the ADR-0003 D8 window); examined Session 13, 2026-10-14 (access
  02:52:35Z–02:55:06Z). One author; Dynamical Systems (math.DS).
- **Status document (SD12; D2.2):** the author's withdrawal of v2 and the v3 revision comment
  ("Revised and corrected version of the paper withdrawn in June 2011. The gap in the
  dense-subspace argument for pointwise convergence has been repaired by a new proof. This
  version treats the Fourier-dimension theorem; the earlier results on general Rajchman measures
  and polynomial curves are not included") together with v3's section "Erratum on a previous
  version" (quoted in S-0029). S-entries: S-0028 (the original, v1), S-0029 (the corrected
  version, its comment and the withdrawal).
- **Provenance and digests:** abs v1 39,446 B `4d155584…b0a4`; abs v2 39,139 B `a6a765b2…83d7`
  (withdrawn; no e-print, HTTP 404); abs v3 39,907 B `ed5a9f43…c5ba`; e-print v1 11,058 B
  `06c7ac8d…dbac` (one gzipped TeX file) — the original, **not committed**; e-print v3 12,220 B
  `193b6443…0048`. Full digests in S-0028 and S-0029.
- **The notice's locating and describing sentences (quoted):** "the author found a gap in the
  dense-subspace argument for pointwise convergence"; "The present paper should be read as a
  correction to the Fourier-dimension theorem in that version and gives a new proof of that
  result. The broader statements there on Rajchman measures under homogeneous dilations and on
  polynomial curves are not addressed here."
- **What the located argument is (executor's reading of v1):** the section "Pointwise Ergodic
  Theorems" opens "We first establish Wiener's ergodic theorem for a *dense* subspace of
  L^p(X), and then prove a maximal inequality … the key result in this paper is the following
  lemma", and states the lemma (TeX label `density`): the linear span L of functions
  f(x) = ∫ f₀(T_s x) φ(s) ds (f₀ ∈ L^∞(X), φ ∈ L¹₀(ℝ^d)) is dense in L¹₀(X), and for a Rajchman
  measure ν the dilated averages A_λ f(x) → 0 almost everywhere for all f ∈ L. The almost-everywhere
  part of its proof reduces, for a fixed x, to the weak convergence to 0 of a family of measures
  h_λ on ℝ^d against bounded continuous functions, and asserts: "Since the variation norm of h_λ
  is uniformly bounded in λ, this is implied by the uniform convergence of the Fourier transforms
  of h_λ to 0 and the tightness of the sequence h_λ (see e.g. [BD])", then shows the Fourier
  transforms converge. The later pointwise theorem's proof begins "The almost sure convergence
  above holds for f in the dense subspace L … defined in lemma density".
- **Diff summary (v1 → v3):** not a local fix: v3 is a shorter paper (10 pages against 11) that
  proves only the Fourier-dimension theorem, with a *new* dense class ("localized away from
  frequency zero", v3's section "A dense class with pointwise convergence") and a new proof; the
  general-Rajchman lemma and the polynomial-curve results of v1 are absent, and the erratum says
  they are "not addressed".
- **Location (D3, `raw/1` locator):** "Lemma (TeX label density), Section 'Pointwise Ergodic
  Theorems', proof of the almost-everywhere convergence for f in the dense subspace L: the
  weak-convergence step". **The notice locates the gap to the argument; the step-level locator is
  the executor's reading**, stated as such in the locator itself.
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity gap,
  lean_catchable YES-KERNEL.** The weak-convergence step invokes the criterion "uniformly bounded
  variation + Fourier transforms → 0 uniformly + tightness ⇒ weak convergence to 0" and then
  establishes only the Fourier-transform part; the tightness of {h_λ} is asserted in passing and
  not established, and for dilated translates of a fixed function it is not a given. That is the
  entry's clause "A named theorem is invoked on an object for which one of its hypotheses is …
  not established (gap)". Gap, not fatal: the author calls it a gap, re-proves the
  Fourier-dimension case by other means, and leaves the general statement open. Lean-catchable:
  the tightness argument cannot be supplied for the step as written (`YES-KERNEL` per the entry).
  Alternative considered: FLAW-0004 (unjustified step) — the step does carry a justification (a
  cited criterion), so the shape is a hypothesis of that criterion left unestablished, FLAW-0005.
  **Caveat recorded plainly:** the author's notice names the argument, not the step; if the gap
  the author found lies elsewhere in the same argument (for example in the "conull subset" on
  which g is uniformly continuous), the class would still be a gap-level flaw of the same
  argument but the sub-location and possibly the class (FLAW-0004) would differ. The item's
  location of record is the argument the notice names; the step is the executor's best reading.
- **Statement status (D3; ADR-0001 as amended 2026-10-13):** `value: null`,
  `method: "undetermined"`, `reference: S-0029` — the status document re-proves only the
  Fourier-dimension special case and says the broader statements "are not addressed"; the claim
  of record (the lemma for general Rajchman measures) is neither re-proved nor retracted. This is
  the case Al's ruling (ii) admitted; `correct` is `false` by derivation.
- **Proof status:** `invalid` (S1.1 by construction; the author withdrew the version and documents
  the gap).
- **Human-authored (D2.6):** one human author; no mention of AI assistance in the v3 sources
  (helper scan: none) or in any comment; the gap was found by the author ("after the author found
  a gap").
- **Licence (D5, SD16):** v1 carries `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` →
  the original's storage is **`external`**: no copy committed; the manifest pins the bytes by hash
  (`original-0902.0680v1.tex.gz`, 11,058 B, `06c7ac8d…dbac`). v3 carries CC BY 4.0. Recorded in
  S-0028 and S-0029 before ingestion.
- **What this item is and is not:** a real, human-authored preprint whose author withdrew it for
  a gap and, fifteen years later, published a corrected version proving a special case by a new
  argument. Nothing here assesses the paper's other results or the corrected version (SD3); the
  claim of record is the v1 dense-subspace lemma alone.
- **Derived canonical artifact (S1.3):** not yet produced; the location re-expression argument is
  added here when it is. The lemma and its proof are about one page of a single TeX file.
- **Written by:** the executor (Claude Fable 5.1), Session 13; a reading of public documents, not
  a verdict on the paper (SD3).
