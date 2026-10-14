# RECORD — Route A item 18 (candidate 19): arXiv:1206.5522 v5, Proposition 5.3 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1206.5522`; original = v5 (18 August 2015, the
  accepted version — J. Topol. 8 (2015), no. 4, 1045–1084, DOI 10.1112/jtopol/jtv028 per the
  abstract page; v1–v4 are 2012–2015 revisions); correcting version = v6 (23 June 2024, inside
  the ADR-0003 D8 window); examined Session 18, 2026-10-14 (access 19:45:32Z). Two authors;
  Algebraic Topology (math.AT).
- **Status document (SD12; D2.2):** the v6 revision comment — "37 pages. Corrected Proposition
  5.3" — with v6 itself, whose acknowledgement adds "We thank Amabel Wilson and Theo
  Johnson–Freyd for correcting the hypotheses in the statement of Proposition 5.3". S-entries:
  S-0050 (the original, v5), S-0051 (the corrected version and its comment).
- **Provenance and digests:** abs v5 43,810 B `64a931cf…4859`; abs v6 43,703 B `0ee83fd6…dbbf`;
  e-print v5 43,241 B `dc5828b3…6a4f` (one gzipped TeX file) — the original, **not committed**;
  e-print v6 43,658 B `7a5c8411…29c4`. Full digests in S-0050 and S-0051.
- **The claim of record (quoted from v5, Subsection 5.1, Proposition 5.3; unlabelled in v5,
  `prop.5.3` in v6):** "Let M be an n-manifold. Let X be a nilpotent space whose first n homotopy
  groups are finite, |π_i X| < ∞ for i ≤ n, and let R be any commutative ring. There is a natural
  equivalence of chain complexes ∫_M 𝒞*(X, R) ≃ 𝒞*(X^M, R) between the factorization homology of
  M with coefficient in the R-cohomology of X and the R-cohomology of the space of maps from M to
  X." The numbering: the theorem counter is per section and shared, and this is the third
  numbered environment of Section 5 (after Proposition 5.1 `tensor` and Corollary 5.2), so the
  comment's "Proposition 5.3" is this one. The proof (one page) reduces by induction over a
  handle decomposition to a homotopy pullback square of mapping spaces, asserts that the natural
  map from 𝒞_*(X^M, R) to the cotensor product is an equivalence "exactly if the homological
  Eilenberg–Moore … spectral sequence for this homotopy Cartesian diagram converges", secures
  convergence by "By Dwyer [Dw], the convergence of this Eilenberg–Moore spectral sequence is
  assured if the base X^{S^k×ℝ^{n−k}} and the fibrations in the diagram are nilpotent. However,
  by [HMRS], the space of maps X^K is nilpotent for any K if X is nilpotent, and likewise are the
  maps in (nilpotent-square)", and then dualises, using "Since the first n homotopy groups of X
  are finite, the mapping space X^K has finitely many components … Since they are additionally
  finite CW complexes, the homology groups of the mapping spaces ℋ_i(X^K, R) are finite rank over
  R".
- **The notice's locating and describing sentences (quoted):** "Corrected Proposition 5.3";
  "correcting the hypotheses in the statement of Proposition 5.3".
- **Diff summary (executor's reading of the TeX diff v5 → v6, 61 lines, the whole of it):** three
  hunks. (i) The acknowledgement sentence above. (ii) The statement: the hypotheses become "let X
  be a nilpotent n-connective space of finite type over R such that π_n X is finite"; the
  conclusion is unchanged. (iii) The proof: the opening case becomes "M is homeomorphic to ℝ^n";
  the convergence step is rewritten — Dwyer's criterion is now stated as requiring the base to be
  connected and the action of π₁ of the base on π_* of the fibre to be nilpotent, n-connectivity
  gives "for k < n any map f: S^k → X is nullhomotopic, and therefore the base … is connected",
  the fibre is identified with Ω^{k+1}X, and a semi-direct-product description of π₁(X^{S^k})
  yields the nilpotent action; the finiteness step becomes "Since π_n X is finite … Since they are
  additionally finite CW complexes and X is finite type, the homology groups … are finite rank
  over R". Nothing else in the 37 pages changes.
- **Location (D3, `raw/1` locators):** instance 1 — "Proposition 5.3, statement: the hypotheses"
  (the part of the statement the authors say they corrected); instance 2 — "Proposition 5.3,
  proof, the convergence step citing Dwyer and [HMRS]" (the step the corrected proof replaces).
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition; the result is weakened), lean_catchable YES-KERNEL** for the statement: the
  authors replace the hypotheses by stronger ones and call that "correcting the hypotheses", the
  proof is a standard-looking handle-induction with a convergence theorem, and the corrected
  version proves only the restricted statement — the entry's shape ("the proof mirrors the
  argument for a nearby true statement, hiding the point where a dropped hypothesis is used").
  **FLAW-0005 (theorem applied without its hypotheses), severity fatal, lean_catchable
  YES-KERNEL** for the proof: Dwyer's convergence theorem is invoked, and the v6 proof shows what
  its hypotheses are (a connected base; a nilpotent π₁-action) and derives them from the new
  n-connectivity hypothesis, which the v5 statement does not grant — the use site at which the
  dropped hypothesis enters, as the FLAW-0012 entry predicts. The same pairing as item 8 (a
  statement quantified over too large a class, a proof that needs the smaller one).
  **Executor's reading, labelled as such and not the authors':** under v5's hypotheses the base
  X^{S^k×ℝ^{n−k}} ≃ X^{S^k} need not be connected (its components are [S^k, X], which v5 only
  requires to be finite), so the connected-base hypothesis is false for objects within the
  statement's scope — hence fatal rather than gap; and the finiteness step's "finite rank over R"
  needs X of finite type, which v5 also does not grant. The document does not say which of
  Dwyer's hypotheses fails or exhibit an X for which the v5 statement fails.
  Other classes considered: FLAW-0010(c) (Dwyer's theorem cited with altered hypotheses — "if
  the base and the fibrations are nilpotent" for "connected base and nilpotent action") is a
  defensible reading of the same sentence and is not recorded separately, because the notice
  says the hypotheses of the *statement* were corrected, not the citation; FLAW-0004 (the
  finiteness claim unjustified) — not recorded, the notice does not name it; FLAW-0011 — not
  recorded, the statement as written is not re-proved.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0051`
  ("Corrected Proposition 5.3"; "correcting the hypotheses in the statement"). **Caveat, as for
  items 8, 10, 11, 14 and 15:** the authors do not write "false" or give a counterexample; the
  stricter `undetermined` reading is the alternative. The executor records `false` because the
  authors replace the statement's hypotheses and prove only the restricted statement — the
  reading applied to those five records; the escalation in STATE now counts six.
- **Proof status:** `invalid` (S1.1 by construction; the corrected proof replaces the step).
- **Human-authored (D2.6):** two human authors; no AI-assistance mention (the scanner reports no
  hit on either version); the error was pointed out by two named mathematicians, per the
  acknowledgement.
- **Licence (D5, SD16):** v5 and v6 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1206.5522v5.tex.gz`, 43,241 B, `dc5828b3…6a4f`). Recorded in S-0050 and S-0051
  before ingestion.
- **What this item is and is not:** a real, human-authored, published paper whose authors, nine
  years after acceptance, strengthen the hypotheses of an auxiliary proposition and rewrite the
  step of its proof that needed them, naming the two readers who found the problem. The journal
  version was not retrieved; the claim of record is Proposition 5.3 as stated in v5. Nothing
  here assesses the paper's main theorems (SD3).
- **Derived canonical artifact (S1.3):** not yet produced; the proposition and its proof are
  about twenty source lines and depend on the paper's excision property for factorization homology over a
  handle decomposition, Dwyer's convergence theorem and the duality between tensor and cotensor products.
- **Written by:** the executor (Claude Fable 5.1), Session 18; a reading of public documents,
  not a verdict on the paper (SD3).
