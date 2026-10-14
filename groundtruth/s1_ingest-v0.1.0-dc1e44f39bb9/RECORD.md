# RECORD — Route A item 14 (candidate 16): arXiv:1202.5531 v3, Theorem 6 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1202.5531`; original = v3 (27 March 2013; v1 and v2
  are 2012 revisions outside the window); correcting version = v4 (23 August 2026, inside the
  ADR-0003 D8 window); examined Session 17, 2026-10-14 (access 08:09:19Z). One author; Algebraic
  Geometry (math.AG); a preprint with no journal reference or DOI on arXiv.
- **Status document (SD12; D2.2):** the v4 revision comment — "Major revision. The main
  irreducibility theorem in the previous version is false in the stated generality. The error
  occurs in Step 4 of the proof of Theorem 6. The present version replaces the incorrect general
  statement by a valid result for quadratic hulls of osculating varieties of closed orbits under
  suitable hypotheses" — with v4 itself. S-entries: S-0044 (the original, v3), S-0045 (the
  corrected version and its comment).
- **Provenance and digests:** abs v3 38,272 B `b28e0061…0837`; abs v4 40,372 B `8ab470eb…2d99`;
  e-print v3 11,081 B `d99f8d19…2cf9` (one gzipped TeX file) — the original, **not committed**;
  e-print v4 16,787 B `b0b1ec31…6b3a`. Full digests in S-0044 and S-0045.
- **The claim of record (quoted from v3, Theorem 6, TeX label `irr-phi-cat`):** under the
  standing hypotheses "Let V be a finite dimensional representation of a complex semisimple Lie
  algebra 𝔤. Let G be a simply connected and connected Lie group with Lie algebra 𝔤 and let
  y ∈ V." — "The variety M_y is irreducible. Recall that M_y ⊆ ℙV is the zero locus of quadrics
  that contain the orbit G.y." The introduction states it as the paper's main result: "The zero
  locus of quadrics in I(G.y)₂ is an irreducible variety." The numbering: the paper's theorems,
  lemmas and remarks share one counter, and this is its sixth entry, so "Theorem 6" in the
  comment is this theorem.
- **The notice's locating and describing sentences (quoted):** "The main irreducibility theorem
  in the previous version is false in the stated generality. The error occurs in Step 4 of the
  proof of Theorem 6."
- **Diff summary (executor's reading of the TeX diff v3 → v4):** v4 is a different paper under a
  new title (2,003 source lines against 666; new abstract, sections and bibliography), so the
  diff cannot localise the error beyond the comment. v4's main theorem is the announced "valid
  result": for d > 4p, the quadratic hull of the p-osculating variety T^p X_{dλ} of a closed
  highest-weight orbit equals the osculating variety and is therefore irreducible. The general
  claim of v3 — irreducibility for every representation and every y — is not re-proved anywhere
  in v4, whose introduction says of quadratic hulls in general that they "need not be
  irreducible" and exhibits reducible ones (Böhning's Z₃ ⊂ ℙS⁸V with two components; the
  quadratic hull of T³c₁₀, which "has another irreducible component of dimension 4" because 28
  does not divide 64). v4's text never mentions the previous version's theorem or Step 4.
- **Location (D3, `raw/1` locator):** "Theorem 6, statement; Step 4 of its proof (the claim that
  the secant variety σ_{ℓ−1}(𝒟) is a hypersurface in ℙW²)" — the step the author names. What
  fails inside the step is not stated by the notice.
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition), lean_catchable YES-KERNEL.** The statement is false by the author's own account;
  the offered proof is a five-step construction (a surjective rational map from the tangent
  bundle of a secant variety of the variety of squares onto M_y, Terracini's lemma for the
  tangent spaces, duality for the irreducibility) — convincing in form, as the entry requires.
  The author locates the error to Step 4 and the record stops there.
  **Executor's reading, labelled as such and not the author's:** Step 4 computes the tangent
  space at a point u built from a special basis f₁, …, f_ℓ of W (chosen through points p₁, …,
  p_ℓ) by Terracini's lemma, finds codim(HW) = 1 there, and concludes that σ_{ℓ−1}(𝒟) is a
  hypersurface. Terracini's lemma describes the tangent space at a *general* point, and the
  dimension of HW is lower-semicontinuous in H, so a special H with codim(HW) = 1 does not exclude
  H′W = W² for general H′ — the secant variety filling ℙW², in which case the dual variety of Step
  5 is not what the proof needs. This is one way Step 4 can fail; the document does not say it is
  the way.
  Other classes considered: FLAW-0005 (a lemma applied where its hypothesis — generality of the
  point — may fail) is precisely the executor's reading and is not recorded as a second instance
  because the notice does not state it; FLAW-0001 — no sign or constant is involved.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0045`
  ("false in the stated generality"). **Caveat, as for item 10:** the same phrase could be read
  as "not established in this generality", which would give `null` / `undetermined`; the
  executor records `false` because the author says "false" and v4 proves only a restricted
  statement. A further reading the executor makes and the document does not: v4's reducible
  quadratic hull of T³c₁₀ is, with v3's definitions, M_y for y = s⁷t³ ∈ S¹⁰ℂ² (the osculating
  variety is the closure of the SL₂-orbit of that form), so an instance of Theorem 6's scope is
  reducible — a counterexample in substance, but v4 does not present it as one, so the
  `counterexample` method is not used.
- **Proof status:** `invalid` (S1.1 by construction; the author locates an error in the proof).
- **Human-authored (D2.6):** one human author; no AI-assistance mention (the helper's scanner and
  a word-boundary scan both report no hit on either version); the finder of the error is unstated.
- **Licence (D5, SD16):** v3 and v4 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1202.5531v3.tex.gz`, 11,081 B, `d99f8d19…2cf9`). Recorded in S-0044 and S-0045
  before ingestion.
- **What this item is and is not:** a real, human-authored preprint whose author, thirteen years
  on, replaces its main theorem by a restricted one and says in the revision comment that the
  general statement was false and where its proof failed. Nothing here assesses v4's results
  (SD3); the claim of record is Theorem 6 as stated in v3.
- **Derived canonical artifact (S1.3):** not yet produced; Theorem 6 and its proof are about 200
  source lines and depend on the paper's Theorem 4 (`prelim-teo`) and Lemma 5 (`irr-lin`).
- **Written by:** the executor (Claude Fable 5.1), Session 17; a reading of public documents,
  not a verdict on the paper (SD3).
