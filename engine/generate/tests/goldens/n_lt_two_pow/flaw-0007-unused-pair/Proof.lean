import Mathlib.Data.Nat.Notation

theorem syndeom_s2_template_n_lt_two_pow_lemma_1 : ∀ n : ℕ, n < 2 ^ n := by
  intro n
  induction n with
  | zero => decide
  | succ k ih => rw [Nat.pow_succ]; omega

theorem syndeom_s2_template_n_lt_two_pow_lemma_2 : ∀ n : ℕ, n ≤ n ^ 2 := by
  intro n
  rw [Nat.pow_two]
  exact Nat.le_mul_self n

theorem syndeom_s2_template_n_lt_two_pow (n : ℕ) : n + 1 ≤ 2 ^ n := by
  exact syndeom_s2_template_n_lt_two_pow_lemma_1 n
