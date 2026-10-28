# RECORD — Route A item 32 (candidate 28): arXiv:1403.6317 v1, Theorem 1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1403.6317`; original = v1 (25 March 2014; no journal
  reference on the abstract page); v2 (28 March 2014) is a withdrawal stub; correcting version =
  v3 (15 October 2024, inside the window); examined Session 22, 2026-10-21 (access
  21:46:38Z–21:51:56Z). Three authors; Symplectic Geometry (math.SG).
- **Status document (SD12; D2.2):** the v3 revision comment — "We removed the wrong theorem from
  the first version, which was brought to our attention by R. Leclercq" — with v3 itself, whose
  introduction adds "We would like to thank Rémi Leclerq for pointing to us an error in the
  previous version of the paper.", and the v2 withdrawal comment — "This paper has been withdrawn
  by the authors due to an error in the assumptions of the Theorem 1". S-entries: S-0074 (the
  original, v1), S-0075 (the corrected version, its comment and the withdrawal comment).
- **Provenance and digests:** abs v1 39,781 B `1e5d5c96…28fd`; abs v2 39,708 B `81d0d559…f907`;
  abs v3 38,733 B `5b32eedd…a531`; e-print v1 126,877 B `97b28cf2…9bce` (a tar bundle, one TeX
  file) — the original, **not committed**; e-print v3 125,994 B `3e95b88f…f2ed`. Full digests in
  S-0074 and S-0075.
- **The claim of record (quoted from v1, Introduction, Theorem 1, TeX label `thm:HD`; the paper
  numbers every theorem-like environment on one shared counter, so Theorem 1 is the first):**
  "Let P be a closed symplectic manifold and L ⊂ P its closed Lagrangian submanifold such that
  ω|_{π₂(P,L)} = 0, μ|_{π₂(P,L)} = 0. Suppose that a Hamiltonian H is admissible, quasi-autonomous
  and homotopic to zero through admissible Hamiltonians. Then it holds: d(φ¹_H, Id) =
  Δ(L, φ¹_H(L))." Here d is the Hofer distance on Hamiltonian diffeomorphisms and Δ the Hofer
  distance on the Hamiltonian deformations of L (v1 Subsection 2.4); a Hamiltonian is
  quasi-autonomous when its maximum and minimum are attained on neighbourhoods of two points
  (Definition 5) and admissible when "it admits no non-constant contractible 1-periodic solution"
  (Definition 6). The proof (v1 Section 3) is a chain of seven (in)equalities, ρ(1_P) − ρ([P]) =
  ρ(i_*(1_L)) − ρ([P]) ≤ ρ(i_*(1_L)) − c(i_![P]) = ρ(i_*(1_L)) − c([L]) ≤ c(1_L) − c([L]) ≤
  Δ(L, φ¹_H(L)) ≤ d(φ¹_H, Id) ≤ ‖H‖ = ρ(1_P) − ρ([P]), "so all inequalities must be equalities",
  with (ii) and (iv) the paper's Theorems 10 and 11 (the comparison inequalities between the
  periodic-orbit and Lagrangian spectral invariants), (v) "the known inequality between spectral
  invariants and Hofer norm, proved in [L]", and (vii) Schwarz's theorem (Theorem 7, cited from
  [Sc2]).
- **The notice's locating and describing sentences (quoted):** "an error in the assumptions of
  the Theorem 1" (v2); "We removed the wrong theorem from the first version" (v3 comment); "an
  error in the previous version of the paper" (v3 text).
- **Diff summary (executor's reading of the TeX diff v1 → v3, 645 lines):** Theorem 1 and the
  sentence announcing its proof are removed from the introduction, together with the whole of
  Section 3 "Proof of Theorem 1", Subsection 2.4 "Hofer's distance" (Definitions 5 and 6,
  Theorem 7 and the distance Δ) and Corollary 12 (which combined Theorems 10 and 11); the
  abstract loses "From this result, we derive a corollary considering comparison of Hofer's
  distance in periodic orbits and Lagrangian case" and the keyword "Hofer's distance". Theorem 3
  (the Morse-homology product inequality) is restated as a remark with its derivation. The
  invariants are reduced from four (ρ, η for periodic orbits; c, σ for the Lagrangian case) to
  two (ρ, σ) normalised through the PSS isomorphisms; v1's "In [L], Leclercq showed that c(β)
  does not depend on H, but only on L and φ¹_H(L)" is replaced by a new theorem that
  σ(·, H) − σ(·, K) is a constant C(H, K) whenever φ¹_H(L) = φ¹_K(L), and the introduction now
  says "after a certain normalization, the spectral invariants also do not depend on any included
  choices". Theorems 10 and 11 are kept in the normalised form; Theorem 2 (the product) is now
  "The main result of the paper". The remaining hunks are notation (HF_*(H:L) for
  HF_*(L, φ¹_H(L))), a non-degeneracy assumption made explicit, an admissibility assumption "in
  the sense of [A]" for the chimney construction, a reference to Hu–Lalonde, figure renumbering
  and typography. The AI-assistance scan of v3: no hit.
- **Location (D3, `raw/1` locator):** "Theorem 1 (TeX label thm:HD), statement — the hypotheses
  on H", as the withdrawal comment locates it; the proof is removed with the theorem.
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal,
  lean_catchable YES-KERNEL.** The authors call the theorem "wrong" and locate the error in its
  assumptions: the statement, under the hypotheses it imposes on H, is withdrawn, and its proof
  is a chain of seven steps each borrowed from a known result — the entry's "proof mirrors the
  argument for a nearby true statement" shape. Which step the hypotheses fail to support is not
  stated by the notice and is not guessed here (as for items 9 and 12). Severity fatal: the result
  is withdrawn (D3). Other classes considered: FLAW-0005 — "error in the assumptions" suggests a
  step applying a cited result outside the hypotheses Theorem 1 provides, but the notice names no
  step, so no FLAW-0005 record is written; FLAW-0002 — no quantifier change is reported.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0075`
  ("the wrong theorem"; "an error in the assumptions"). The stricter `undetermined` reading — a
  theorem withdrawn without a counterexample and without a corrected statement, the notice giving
  neither — is the alternative; the record joins the list of "corrected"/"incorrect"/"wrong"-
  wording items (8, 10, 11, 14, 15, 18, 25) carried in STATE's escalations for Al's standing
  ruling.
- **Proof status:** `invalid` (S1.1 by construction; the proof is withdrawn with the theorem).
- **Human-authored (D2.6):** three human authors; no AI-assistance mention (scanner clean); the
  error pointed out by a named mathematician (R. Leclercq), per the comment and the
  acknowledgement.
- **Licence (D5, SD16):** v1 and v3 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1403.6317v1.tar.gz`, 126,877 B, `97b28cf2…9bce`). Recorded in S-0074 and
  S-0075 before ingestion.
- **What this item is and is not:** a real, human-authored preprint whose authors, ten years
  after a three-day withdrawal "due to an error in the assumptions of the Theorem 1", restore the
  paper without that theorem and re-found its invariants; Theorem 2 (the product) and the
  comparison inequalities stand in v3 in normalised form. Corollary 12 and the demoted Theorem 3
  are not named by the notice and are not items (D2.3). Nothing here assesses the rest of the
  paper (SD3); the claim of record is Theorem 1 as stated in v1.
- **Derived canonical artifact (S1.3):** not yet produced; the theorem's proof is about twenty
  source lines but cites Theorems 10 and 11 (proved in the paper), the inequality of [L] and
  Schwarz's theorem.
- **Integrated:** Session 38 (2026-10-28), as the first later prefix (Al's rulings Q-H6 and
  Q-H7 of 2026-10-28), from the held branch `claude/s1-harvest-candidates-28-30` at `c3a3d36`,
  where this item was `s1_ingest-v0.1.0-eeea00c52724` with sources S-0066 and S-0067. Per
  `analysis/s1-held-harvest-review/REVIEW.md` §4 the S-numbers are renumbered (S-0066..S-0072
  → S-0074..S-0080), the manifest is regenerated by `s1_ingest` 0.1.0 at `a793954` (its
  `engine_commit`) over the same original bytes, and the session label is Session 22, which
  examined the item. The reading is unchanged; Al's rulings let it stand (Q-H1: false by source document, as recorded, so FLAW-0012 stands; Q-H4: Corollary 12 and
  the demoted Theorem 3 stay out as items and join the unnamed-result question;
  STATE, *Standing rulings*).
- **Written by:** the executor (Claude Fable 5.1), Session 22 (2026-10-21); a reading of public documents,
  not a verdict on the paper (SD3).
