import Mathlib.Data.Nat.Notation

axiom syndeom_s2_template_n_lt_two_pow_lemma_1 : ∀ n : ℕ, n + 1 < 2 ^ n

theorem syndeom_s2_template_n_lt_two_pow (n : ℕ) : n + 1 ≤ 2 ^ n := by
  exact Nat.le_of_lt (syndeom_s2_template_n_lt_two_pow_lemma_1 n)

#print axioms syndeom_s2_template_n_lt_two_pow
