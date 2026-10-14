# RECORD — Route A item 25 (candidate 24): arXiv:1303.3650 v2, Theorem 4.6 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1303.3650`; original = v2 (19 March 2014, the
  version "to appear in Quarterly Journal of Mathematics" per its comment; v1 of 2013 used a
  different method); correcting version = v3 (6 January 2025, inside the ADR-0003 D8 window);
  examined Session 19, 2026-10-14 (access 20:15:25Z). Two authors; Algebraic Geometry
  (math.AG), cross-listed Number Theory. The journal version was not retrieved; the original of
  record is v2.
- **Status document (SD12; D2.2):** the v3 revision comment — "José Alves Oliveira and Lucas
  Reis point out an error in the old version of Theorem 4.6. The assumption that Tr(g) is not of
  the form r^q-r is corrected to that Tr(g) is not of the form r^p-r" — with v3 itself, whose
  title footnote adds "We would like to thank José Alves Oliveira and Lucas Reis for pointing
  out an error in the old version of Theorem 4.6". S-entries: S-0058 (the original, v2), S-0059
  (the corrected version and its comment).
- **Provenance and digests:** abs v2 38,292 B `a4452233…2729`; abs v3 38,113 B `f1dd52c5…cce3`;
  e-print v2 11,478 B `7361c002…f9b9` (one gzipped TeX file) — the original, **not committed**;
  e-print v3 11,527 B `56343607…3164`. Full digests in S-0058 and S-0059.
- **The claim of record (quoted from v2, Section 4 "Character Sums on Curves", Theorem 4.6):**
  "Let f(t), g(t) ∈ 𝔽_{q^d}(t) be rational functions. Write f(t) = ∏_{j=1}^k f_j(t)^{n_j}, where
  f_j(t) ∈ 𝔽_{q^d}[t] are irreducible polynomials and n_j are nonzero integers. Let
  D_1 = Σ deg(f_j), let D_2 = max(deg(g), 0), let D_3 be the degree of the denominator of g(t),
  and let D_4 be the sum of degrees of those irreducible polynomials dividing the denominator of
  g but distinct from f_j(t). Let χ: 𝔽_{q^d}^* → ℚ̄_ℓ^* be a multiplicative character of
  𝔽_{q^d}, and let ψ_q: 𝔽_q → ℚ̄_ℓ^* be a nontrivial additive character of 𝔽_q. Suppose
  Tr_{𝔽_{q^d}/𝔽_q}(g) = Σ_{i=0}^{d−1} σ^i(g) is not of the form r(t)^q − r(t) in 𝔽(t). Then we
  have the estimate |Σ_{a∈𝔽_q, f(a)≠0,∞, g(a)≠∞} χ(f(a)) ψ_q(Tr_{𝔽_{q^d}/𝔽_q}(g(a)))| ≤
  (d(D_1+D_3+D_4)+D_2−1)√q." (𝔽 the paper's fixed algebraic closure of 𝔽_p.) The numbering: the
  counter is per section and shared — Proposition 4.1, Theorem 4.2, Lemma 4.3, Remark 4.4,
  Theorem 4.5, Theorem 4.6 — and v3's numbering is the same. The proof (one page) realises the
  sum as a trace of Frobenius on the compactly supported cohomology of a tensor-induced sheaf and
  bounds the first cohomology by the Grothendieck–Ogg–Shafarevich formula once the sheaf is
  known to be geometrically nonconstant with wild ramification at the poles of Tr(g).
- **The notice's locating and describing sentences (quoted):** "an error in the old version of
  Theorem 4.6. The assumption that Tr(g) is not of the form r^q-r is corrected to that Tr(g) is
  not of the form r^p-r".
- **Diff summary (executor's reading of the TeX diff v2 → v3, 40 lines, the whole of it):** the
  footnote above; in the statement, "r(t)^q−r(t)" becomes "r(t)^p−r(t)"; in the proof, the
  sentence "Since Tr_{𝔽_{q^d}/𝔽_q}(g) is not of the form r(t)^q−r(t) in 𝔽(t), the sheaf
  𝓛_{ψ,Tr(g)} is geometrically nonconstant, and it has wild ramification at poles of Tr(g)"
  changes in the same way; one notation change elsewhere ("tran" to "⊗-Ind") and a
  document-class option. Nothing else changes.
- **Location (D3, `raw/1` locators):** instance 1 — "Theorem 4.6, statement: the hypothesis on
  Tr(g)" (the assumption the authors say they corrected); instance 2 — "Theorem 4.6, proof, the
  sentence 'Since Tr(g) is not of the form r(t)^q−r(t) in 𝔽(t), the sheaf 𝓛_{ψ,Tr(g)} is
  geometrically nonconstant'" (the one sentence of the proof that uses the hypothesis).
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition; the result is weakened), lean_catchable YES-KERNEL** for the statement: the
  authors say the old assumption was in error and replace it by a stronger one, and the proof
  is the standard cohomological argument for the nearby true statement — the entry's shape.
  **FLAW-0005 (theorem applied without its hypotheses), severity fatal, lean_catchable
  YES-KERNEL** for the proof: the geometric nonconstancy of the Artin–Schreier sheaf is inferred
  from the weaker hypothesis — the use site at which the dropped hypothesis enters, as the
  FLAW-0012 entry predicts; the same pairing as items 8 and 18. **Executor's reading, labelled
  as such and not the authors':** every r^q − r is of the form s^p − s (take
  s = r + r^p + ⋯ + r^{q/p}), so "not of the form r^q − r" is the weaker condition, and the
  criterion for 𝓛_{ψ,h} to be geometrically nonconstant is stated in terms of p-th powers, not
  q-th; what the finders' examples are, and whether the v2 statement fails for every ψ_q or only
  for some, the document does not say.
  Other classes considered: FLAW-0001 (a "constant" slip, q for p, in an exponent) describes
  the mechanism and is not recorded, because the notice frames the error as a wrong assumption
  rather than a typographical one and the fix changes what the theorem asserts; FLAW-0011 — not
  recorded, the statement as written is not re-proved.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0059`
  ("an error in the old version of Theorem 4.6"; the assumption "is corrected"). **Caveat, as
  for items 8, 10, 11, 14, 15 and 18:** the authors do not write "false" and print no
  counterexample (the finders' examples are not in the document); the stricter `undetermined`
  reading is the alternative. The executor records `false` because the authors replace the
  statement's hypothesis and prove only the restricted statement — the reading applied to those
  six records; the escalation in STATE now counts seven.
- **Proof status:** `invalid` (S1.1 by construction; the proof's inference rests on the
  corrected assumption).
- **Human-authored (D2.6):** two human authors; no AI-assistance mention (the scanner reports no
  hit on either version); the error was pointed out by two named mathematicians, per the
  authors.
- **Licence (D5, SD16):** v2 and v3 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1303.3650v2.tex.gz`, 11,478 B, `7361c002…f9b9`). Recorded in S-0058 and
  S-0059 before ingestion.
- **What this item is and is not:** a real, human-authored paper (accepted by a journal per its
  own comment) whose authors, eleven years on, strengthen one hypothesis of a theorem after two
  named readers pointed out an error. Nothing here assesses the paper's other theorems or the
  journal version (SD3); the claim of record is Theorem 4.6 as stated in v2.
- **Derived canonical artifact (S1.3):** not yet produced; the theorem and its proof are about
  seventy source lines and depend on the paper's Sections 1–3 (tensor induction of Kummer and
  Artin–Schreier sheaves) and standard ℓ-adic cohomology.
- **Written by:** the executor (Claude Fable 5.1), Session 19; a reading of public documents,
  not a verdict on the paper (SD3).
