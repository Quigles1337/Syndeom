# RECORD — Route A item 34 (candidate 29): arXiv:1403.8075 v9, Theorem 4.3 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1403.8075`; original = v9 (23 March 2023); correcting
  versions = v10 (27 December 2025) and v11 (9 January 2026), both withdrawals, inside the
  window; examined Session 22, 2026-10-21 (access 21:48:05Z–21:52:06Z). One author; General
  Mathematics (math.GM). The paper, its versions and its status document are described in the
  record of item 33 (`s1_ingest-v0.1.0-915251f36fc5`); this record states what is specific to
  Theorem 4.3.
- **Status document (SD12; D2.2):** the v10 and v11 withdrawal comments (quoted in full in the
  item-33 record), whose closing sentences name this theorem: "These invalidate the probabilistic
  step leading to Theorems 4.3 -- 4.4" (v10); "This invalidates the derivation of Theorems
  4.3-4.4" (v11). S-entries: S-0076 (the original, v9), S-0077 (the withdrawal comments), shared
  with items 33 and 35.
- **Provenance and digests:** as item 33; the original's bytes are the same e-print v9
  (136,883 B, `c171e338…9270`), **not committed**.
- **The claim of record (quoted from v9, Section 4, Theorem 4.3 — the section's third numbered
  environment):** "Let m(n) = E(π(n)). Then, π(n) − Li(n) = o(M(m(n))√m(n)) is almost certainly
  true, where M : ℝ → ℝ is an arbitrary function such that lim_{x→∞} M(x) = ∞." — "almost
  certainly true" in the paper's sense (Definition 2.1: "holds with a probability of 1"), E the
  expectation on the paper's probability spaces (Ω, 𝔅_n, P) of Section 2, Li(n) = ∫₂ⁿ dx/log x.
  Its proof opens: "In this case, we can replace m → ∞ with n → ∞ because m → ∞ if and only if
  n → ∞. The assumptions of Theorem 3.4 are then satisfied if we consider n to be k." It supposes
  |π(n) − Li(n)| > C₀M(m)√m for arbitrarily large m, deduces "By applying Theorem 3.4, we deduce
  that π(n) − m(n) = o(M(m(n))√m(n)) a.c.", derives from Theorem 3.4 and the supposition a
  subsequence with π(n_i) > Li(n_i) a.c., and closes with Lemma 4.2 ("for any algorithm, there
  exists a sufficiently large i such that π(n_i) > Li(n_i) is not determined within a certain
  number of finite steps by Lemma 4.2. This result almost certainly leads to a contradiction").
  Theorem 3.4 reads "Suppose that m → ∞ when k → ∞. Thus, S_k − m = o(M(m)√m) is almost certainly
  true." for S_k = Σ X_i and m = Σ E(X_i), under Section 3's opening assumption "let X_n be a
  random variable. We assume that ω'₁ ∈ {ω₁, ω₁ᶜ}, …, ω'_n ∈ {ω_n, ω_nᶜ} are independent."
  The bridge from Theorem 3.4 to π(n) is the paragraph after Proposition 4.1: "We consider that
  f_p contains a random variable, and we can deduce from Proposition 4.1 and Remark 2.4 that
  lim ω_{p_i} ≠ ∅ and lim ω_{p_i} ≠ Ω when {p_i} is the prime number sequence. Furthermore,
  f_p(1), …, f_p(n) are independent, and f_p(n) is determined by n. [footnote: When n is finite,
  the probability of f_p(n) = 1 is 0 or 1, and it is independent of the condition of other
  integers because n does not change.] Accordingly, the prime counting function π(n) = Σ f_p(i)
  behaves as the sum of the independent random variables, although f_p(n) is decided without
  almost certainty when n is finite."
- **The notice's locating and describing sentences (quoted):** "the subsequent claim that
  fp(1),...,fp(n) are independent is unjustified. These invalidate the probabilistic step leading
  to Theorems 4.3 -- 4.4" (v10); "the subsequent independence assumption for fp(1),...,fp(n) is
  unjustified. This invalidates the derivation of Theorems 4.3-4.4" (v11).
- **Confirmation against the text (executor's reading):** the "probabilistic step" is the
  application of Theorem 3.4 to π(n) as a sum of independent random variables; the theorem's
  hypotheses are supplied only by the bridge paragraph — the independence sentence the notice
  calls unjustified, and the random-variable status drawn from Proposition 4.1 (item 33).
- **Location (D3, `raw/1` locator):** "Theorem 4.3, proof, 'The assumptions of Theorem 3.4 are
  then satisfied if we consider n to be k' and 'By applying Theorem 3.4, we deduce that …'".
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity gap,
  lean_catchable YES-KERNEL.** Theorem 3.4 is invoked on π(n) with its hypothesis — independent
  random summands — asserted and not established: the entry's "hypothesis … not established
  (gap)" clause, the notice's own word being "unjustified". Severity gap by the entry's range, as
  items 13, 16, 21, 24, 26, 27 and 29; the D3 severity-clause reading (the derivation withdrawn →
  fatal) is the escalation carried in STATE. Other classes considered: FLAW-0004 — the
  independence sentence is an unjustified assertion, but it serves as the hypothesis of a named
  theorem's application, so FLAW-0005 is the entry whose vocabulary fits; FLAW-0012 — the notice
  does not say the theorem is false.
- **Statement status (D3):** `value: null`, `method: "undetermined"`, `reference: S-0077` — the
  derivation is withdrawn; the statement is neither re-proved nor disproved (the twelfth use of
  ruling (ii)).
- **Proof status:** `invalid` (S1.1 by construction).
- **Human-authored (D2.6), licence (D5, SD16):** as item 33; `original-1403.8075v9.tar.gz`,
  `external`, digest-only.
- **What this item is and is not:** the first of the two theorems whose derivation the author
  withdraws; the record describes the step the notice names and nothing beyond it (SD3). The claim
  of record is Theorem 4.3 as stated in v9.
- **Derived canonical artifact (S1.3):** not yet produced; the theorem and its proof are about
  forty source lines and consume Theorem 3.4, Lemma 4.2 and the bridge paragraph.
- **Integrated:** Session 38 (2026-10-28), as the first later prefix (Al's rulings Q-H6 and
  Q-H7 of 2026-10-28), from the held branch `claude/s1-harvest-candidates-28-30` at `c3a3d36`,
  where this item was `s1_ingest-v0.1.0-c38c8c65dd2c` with sources S-0068 and S-0069. Per
  `analysis/s1-held-harvest-review/REVIEW.md` §4 the S-numbers are renumbered (S-0066..S-0072
  → S-0074..S-0080), the manifest is regenerated by `s1_ingest` 0.1.0 at `a793954` (its
  `engine_commit`) over the same original bytes, and the session label is Session 22, which
  examined the item. The reading is unchanged; Al's rulings let it stand (Q-H2: `gap`, as recorded, by the entry-range practice main's items follow;
  STATE, *Standing rulings*).
- **Written by:** the executor (Claude Fable 5.1), Session 22 (2026-10-21); a reading of public documents,
  not a verdict on the paper (SD3).
