# RECORD — Route A item 21 (candidate 22): arXiv:1209.4040 v3, Theorem 4.5 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1209.4040`; original = v3 (25 October 2016; v2 of
  2013 and v3 itself carry earlier corrections, outside the ADR-0003 D8 window and not
  harvested); correcting version = v4 (10 September 2025, inside the window); examined Session
  19, 2026-10-14 (access 20:14:48Z). One author; Symplectic Geometry (math.SG); a preprint with
  no journal reference on arXiv.
- **Status document (SD12; D2.2):** the v4 revision comment — "v4:The notion of "weakly
  regularizing" (in Def.4.3) was strengthened to fix a gap in the proof of the main theorem
  (Thm.4.5)" — with v4 itself, whose source carries commented-out edit notes at each change
  ("Here the notion of "weakly regularizing" was strengthened -- previously it was
  ker D_{E′}Φ(r,0) ⊂ E′_1"; "Here we clarified the use of the (strengthened) weak regularization
  property and added proof that D_{E′}Φ(r^ν,0) is uniformly bounded"; "Updated the weak
  regularization claim"). S-entries: S-0054 (the original, v3), S-0055 (the corrected version
  and its comment).
- **Provenance and digests:** abs v3 39,385 B `dcfc69f6…8854`; abs v4 39,270 B `80202a75…65b7`;
  e-print v3 43,630 B `6a907dc3…abd0` (one gzipped TeX file) — the
  original, **not committed**; e-print v4 46,286 B `5f59f9ad…13ed` (tar). Full digests in S-0054
  and S-0055.
- **The claim of record (quoted from v3, Section 4, Theorem 4.5, TeX label `thm`):** "Let
  Φ: 𝔼 → 𝔽 be a sc^∞ map that is sc-Fredholm at 0 with respect to a splitting 𝔼 ≅ ℝ^d × 𝔼′.
  Then it satisfies all conditions of Definition 4.1 except for the contraction (contraction)
  for m = 0. In particular, Φ|_{E_1}: (E_m)_{m∈ℕ} → (F_m)_{m∈ℕ} is sc-Fredholm at 0 w.r.t. the
  induced scale structures on E_1 ⊂ E_0 and F_1 ⊂ F_0." Here "sc-Fredholm at 0 with respect to
  a splitting" is Definition 4.3 (`def:scfred`), whose clause (iii) reads in v3: "The
  differential D_{E′}Φ(0,0): 𝔼′ → 𝔽 is sc-Fredholm as in Definition `fred op` of Section 3. Moreover,
  D_{E′}Φ(r,0): E′_0 → F_0 is Fredholm for all |r| < ε_0, with Fredholm index equal to that for
  r = 0, and weakly regularizing, i.e. ker D_{E′}Φ(r,0) ⊂ E′_1." The introduction calls the
  theorem the core of the paper (a simpler nonlinear Fredholm notion implying the polyfold one);
  Section 5 applies it to Floer's operator. The numbering: the counter is per section and
  shared — Definition 4.1, Remark 4.2, Definition 4.3, Remark 4.4, Theorem 4.5 — so the
  comment's numbers are v3's.
- **The notice's locating and describing sentences (quoted):** "The notion of "weakly
  regularizing" (in Def.4.3) was strengthened to fix a gap in the proof of the main theorem
  (Thm.4.5)".
- **Diff summary (executor's reading of the TeX diff v3 → v4, 1,183 lines):** most of the diff
  is reference updates (the polyfold book in place of the original papers), a new appendix
  "Brief Guide to Polyfold Theory", and prose edits. The substantive changes: (i) Definition
  4.3 (iii) now ends "and these operators are weakly regularizing in the sense that for all
  |r| < ε_0 and e ∈ E′_0 we have the implication D_{E′}Φ(r,0) e ∈ F_1 ⟹ e ∈ E′_1" — a
  strictly stronger condition than v3's kernel regularity; (ii) Theorem 4.5's statement gains
  "in the sense of Definition 4.3" and is otherwise word-for-word the same; (iii) in its proof,
  v3's sentence "Hence for the isomorphism property (eq:iso) is suffices to prove injectivity on
  W_0; which follows directly from the estimate (eq:est) for m=1 and the weak regularization
  property in Definition 4.3 (iii)" becomes a displayed implication — Π_C^⊥ D_{E′}Φ(r,0) w = 0
  ⟹ D_{E′}Φ(r,0) w ∈ C ⊂ F_∞ ⟹ w ∈ E′_1 ∩ W_0 — "and the estimate (eq:est) for m=1 on
  E′_1 ∩ W_0 = W_1", followed by a new half-page proving the uniform bound on
  ‖D_{E′}Φ(r^ν,0) w^ν‖_{F_{m−1}} that the subsequence argument then uses; (iv) Section 5's
  Lemma `le3` asserts the strengthened property for the linearized Floer operators and its
  proof is rewritten to establish it, so the paper's application still satisfies the new
  definition.
- **Location (D3, `raw/1` locator):** "Theorem 4.5, proof, the sentence 'Hence for the
  isomorphism property (eq:iso) is suffices to prove injectivity on W_0; which follows directly
  from the estimate (eq:est) for m=1 and the weak regularization property in Definition 4.3
  (iii)'" — the step the corrected proof replaces, as the source's own edit note says.
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity gap,
  lean_catchable YES-KERNEL.** The step applies the estimate (eq:est) for m = 1, which the proof
  has for w ∈ W_1, to an arbitrary w ∈ W_0 with Π_C^⊥ D_{E′}Φ(r,0) w = 0, and offers v3's
  clause (iii) as the bridge; but v3's (iii) places only the *kernel* of D_{E′}Φ(r,0) in E′_1,
  while such a w has D_{E′}Φ(r,0) w ∈ C, not necessarily 0 — the hypothesis w ∈ W_1 is not
  established (the entry's gap case: "hypothesis true but unshown"). The author's fix is to
  strengthen (iii) so that it supplies exactly that hypothesis, and to prove the uniform bound
  the following argument also needed. Gap, not fatal: the author calls it a gap, the theorem's
  text is unchanged, and the paper's application (Lemma `le3`) is shown to satisfy the stronger
  definition — the same shape as items 13 and 16 (a hypothesis added, the statement neither
  re-proved as it stood nor disproved).
  Other classes considered: FLAW-0004 (an unjustified step — "follows directly") is the
  alternative reading of the same sentence and is named here; the record uses FLAW-0005 because
  the sentence does name its justification (the estimate and clause (iii)) and the defect is
  that the named justification does not cover the object. FLAW-0012 — not recorded: the
  document does not say the v3 statement is false, and no counterexample is given.
- **Statement status (D3; ADR-0001 as amended 2026-10-13):** `value: null`,
  `method: "undetermined"`, `reference: S-0055` — the theorem as stated in v3 (with v3's
  Definition 4.3) is neither re-proved nor disproved; v4 proves the theorem under the
  strengthened definition. The fourth undetermined-truth item (after items 3, 13 and 16);
  `correct` is `false` by derivation.
- **Proof status:** `invalid` (S1.1 by construction; the author replaces the step).
- **Human-authored (D2.6):** one human author; no AI-assistance mention (the scanner reports no
  hit on either version); the finder of the gap is unstated.
- **Licence (D5, SD16):** v3 and v4 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1209.4040v3.tex.gz`, 43,630 B, `6a907dc3…abd0`). Recorded in S-0054 and
  S-0055 before ingestion.
- **What this item is and is not:** a real, human-authored preprint whose author, nine years
  after the previous version, strengthens a definition so that the main theorem's proof closes,
  and annotates every change in the source. Nothing here assesses the Floer application or the
  rest of the paper (SD3); the claim of record is Theorem 4.5 as stated in v3 under v3's
  Definition 4.3.
- **Derived canonical artifact (S1.3):** not yet produced; the theorem and its proof are about
  sixty source lines and depend on Definitions 4.1 and 4.3, Lemma `sc is hwz` and the paper's
  scale-calculus conventions.
- **Written by:** the executor (Claude Fable 5.1), Session 19; a reading of public documents,
  not a verdict on the paper (SD3).
