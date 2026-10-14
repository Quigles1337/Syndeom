# RECORD — Route A item 30 (candidate 26): arXiv:1312.1045 v1, Theorem A.1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1312.1045`; original = v1 (4 December 2013; published
  as J. Differential Equations 258 (2015), 362–378, DOI 10.1016/j.jde.2014.09.015, per the
  abstract page); correcting version = v2 (5 April 2023, inside the ADR-0003 D8 window); examined
  Session 20, 2026-10-14 (access 21:28:15Z). Two authors; Analysis of PDEs (math.AP).
- **Status document (SD12; D2.2):** the v2 revision comment — "We have revised the Appendix by
  slightly changing the statement (and the proof) of Proposition A.2 (Comparison Principle). We
  needed Proposition A.2 in this modified form to fill a gap in the proof of Theorem A.1. We have
  also introduced some simplifications in the proof of Proposition A.2" — with v2 itself.
  S-entries: S-0062 (the original, v1), S-0063 (the corrected version and its comment).
- **Provenance and digests:** abs v1 39,895 B `44a2f461…95dc`; abs v2 40,329 B `2a72ecf0…5821`;
  e-print v1 17,242 B `99711e20…8483` (tar: `NonCommutingNov2013.tex`, `.bbl`) — the original,
  **not committed**; e-print v2 17,971 B `3cf8c975…04b3`. Full digests in S-0062 and S-0063.
- **The claim of record (quoted from v1, Appendix, Theorem A.1, TeX label `teo appendix`):** "Let
  F: ℝⁿ × ℝⁿ → ℝ be a Hamiltonian satisfying (F1)-(F2) and let u_0 ∈ UC(ℝⁿ). Then there exists a
  unique function u ∈ UC([0,+∞)×ℝⁿ) with u(0,·) = u_0 that solves (A) in the viscosity sense. If
  u_0 is Lipschitz in ℝⁿ, then u is Lipschitz continuous in [0,+∞)×ℝⁿ and its Lipschitz constant
  only depends on ‖Du_0‖_∞ and F." — equation (A) being ∂_t u + F(x, D_x u) = 0 (v1 writes its
  domain "(0,+∞)×(0,+∞)×ℝⁿ", a typo v2 fixes to "(0,+∞)×ℝⁿ"). The introduction calls it "a
  folklore theorem on existence and uniqueness of viscosity solutions" and says "all the proofs
  we have found in literature are …" given under other hypotheses, hence the appendix. The proof
  (one page, Perron's method): uniqueness "comes from Proposition A.2"; for u_0 Lipschitz, u is
  the supremum of continuous subsolutions between u_0 − Mt and u_0 + Mt; "Then u* and u_* are,
  respectively, a sub and a supersolution of (A), see Theorem 2.14 in [bardi], and satisfy
  u*(0,·) = u_*(0,·) = u_0 on ℝⁿ. By applying the comparison principle stated in Proposition
  A.2, we infer u* ≤ u_* on [0,+∞)×ℝⁿ."; then "It is left to show the Lipschitz continuity of u.
  By the comparison principle and the definition of u we get ‖u(t+h,·) − u(t,·)‖_{L^∞} ≤
  ‖u(h,·) − u(0,·)‖_{L^∞} ≤ Mh."; the uniformly continuous case by approximation.
- **The notice's locating and describing sentences (quoted):** "We needed Proposition A.2 in this
  modified form to fill a gap in the proof of Theorem A.1."
- **Diff summary (executor's reading of the TeX diff v1 → v2, 467 lines):** Theorem A.1's
  statement is unchanged word for word. Proposition A.2 (v1: "Let v and u be, respectively, an
  upper and lower semicontinuous function on [0,+∞)×ℝⁿ. Let us assume that, for every α > 0,
  there exists a constant M_α > 0 such that, for every (t,x), (v_0) v(t,x) ≥ v(0,x) − (α+M_α t),
  (u_0) u(t,x) ≥ u(0,x) − (α+M_α t). If v is a subsolution and u is a supersolution of (A), we
  have sup(v−u) ≤ sup_{ℝⁿ}(v(0,·)−u(0,·))") becomes: both functions satisfy the growth condition
  (C) for *one* pair of constants, and either (a) the subsolution satisfies (C) for every α, or
  (b) the subsolution is continuous; its proof is reworked. The proof of Theorem A.1 gains, before
  the first application, the bounds "u_0(x) − Mt ≤ u_*(t,x) ≤ u*(t,x) ≤ u_0(x) + Mt on
  [0,+∞)×ℝⁿ" (so the hypotheses hold with α arbitrary and M_α = M), and, for the Lipschitz
  bound, replaces "By the comparison principle and the definition of u we get" by: "for every
  fixed h, the function u_h(t,x) := u(t+h,x) is a continuous solution to (A) with initial
  condition u_h(0,x) = u(h,x). From the fact that |u(t+h,x) − u(0,x)| ≤ M(t+h) … one easily see
  that u_h satisfies condition (C) with α := 2Mh and M_α := M. By applying Proposition A.2 to the
  continuous solutions u_h, u, we get …" — case (b). The rest of the paper is prose edits.
- **Location (D3, `raw/1` locators):** instance 1 — "Theorem A.1, proof" (the two applications
  of the comparison principle, re-argued in v2); instance 2 — "Theorem A.1, proof, the sentences
  'By applying the comparison principle stated in Proposition A.2, we infer u* ≤ u_*' and 'By the
  comparison principle and the definition of u we get ‖u(t+h,·) − u(t,·)‖ ≤ …'".
- **Class argument (D3):** **FLAW-0011 (true theorem with invalid proof), severity gap (a step
  the fix had to re-prove), lean_catchable YES-KERNEL (per the embedded flaw)** — the statement
  stands (the corrected version proves the same statement) and the offered proof carries a flaw:
  **FLAW-0005 (theorem applied without its hypotheses), severity gap, lean_catchable
  YES-KERNEL** — Proposition A.2 is applied to the envelopes u*, u_* and to the pair u(·+h,·), u
  without the growth conditions (v_0), (u_0) it requires being established for them; the entry's
  gap case ("hypothesis true but unshown"): v2 shows the bounds that give the conditions for the
  envelopes and, for the time-shift pair — where (v_0) for every α would amount to the Lipschitz
  bound being proved — weakens the proposition so that continuity of the subsolution suffices.
  **Executor's reading, labelled as such and not the authors':** the comment does not say which
  application was the gap; the diff touches both, and the second is the one v1's proposition
  could not have covered without circularity, which is why case (b) exists. The same shape as
  items 2, 20 and 30's predecessors (a true theorem whose proof invoked a result outside its
  hypotheses).
  Other classes considered: FLAW-0003 (circularity, for the Lipschitz step under v1's
  proposition) is the executor's reading of the mechanism and is not recorded, because the
  authors say "gap"; FLAW-0012 — not recorded, the statement is re-proved.
- **Statement status (D3):** `value: true`, `method: "source-document"`, `reference: S-0063` —
  the corrected version proves the same statement and the comment locates the gap in the proof,
  not the theorem; `correct` is `false` by derivation (the proof is invalid). The third
  true-statement item (after items 2 and 20).
- **Proof status:** `invalid` (S1.1 by construction; the authors fill a gap).
- **Human-authored (D2.6):** two human authors; no AI-assistance mention (the scanner reports no
  hit on either version); the finder of the gap is unstated.
- **Licence (D5, SD16):** v1 and v2 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1312.1045v1.tar.gz`, 17,242 B, `99711e20…8483`). Recorded in S-0062 and
  S-0063 before ingestion.
- **What this item is and is not:** a real, human-authored, published paper whose authors, ten
  years on, repair the proof of an appendix theorem they call folklore by restating the
  comparison principle it uses. Proposition A.2 itself is not an item: the comment reports no
  error in it, only a change "in this modified form" and simplifications, and v1's version is a
  special case of v2's. Nothing here assesses the paper's main results (SD3); the claim of record
  is Theorem A.1 as stated in v1.
- **Derived canonical artifact (S1.3):** not yet produced; the theorem and its proof are about
  forty source lines and depend on Proposition A.2, the Perron construction and the cited
  stability results.
- **Written by:** the executor (Claude Fable 5.1), Session 20; a reading of public documents,
  not a verdict on the paper (SD3).
