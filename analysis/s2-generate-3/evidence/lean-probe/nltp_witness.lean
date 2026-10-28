import Mathlib.Data.Nat.Notation

/-- Witness for the `n_lt_two_pow` seed's statement (ADR-0011 D7). -/
theorem syndeom_seed_n_lt_two_pow (n : ℕ) : n + 1 ≤ 2 ^ n := by
  induction n with
  | zero => decide
  | succ k ih => rw [Nat.pow_succ]; omega

/-- Witness that the lemma the seed's FLAW-0009 `truth: false` axiom states is false
(ADR-0011 D7): it fails at `n = 0`. -/
theorem syndeom_seed_n_lt_two_pow_lemma_false : ¬ ∀ n : ℕ, n + 1 < 2 ^ n := by
  intro h
  exact absurd (h 0) (by decide)

#print axioms syndeom_seed_n_lt_two_pow
#print axioms syndeom_seed_n_lt_two_pow_lemma_false
