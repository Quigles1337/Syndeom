import Mathlib.Data.Real.Basic
import Mathlib.Tactic.Linarith

theorem syndeom_s2_grammar_ineq_chain_lemma_1 : ∀ x y : ℝ, 2 * x ^ 2 + 6 * x * y + y ^ 2 + 2 * x + 2 * y + 1 + |x| + |y| + |x + y| ≤ 3 * x ^ 2 + 4 * x * y + 2 * y ^ 2 + 2 * x + 2 * y + 1 + |x| + |y| + |x + y| := by
  intro x y
  linarith [two_mul_le_add_sq x y]

theorem syndeom_s2_grammar_ineq_chain_lemma_2 : ∀ x y : ℝ, 4 * x ^ 2 + 2 * x * y + 5 * y ^ 2 + 2 * x - 2 * y + 3 + 2 * |x| + 2 * |y| ≤ 5 * x ^ 2 + 6 * y ^ 2 + 2 * x - 2 * y + 3 + 2 * |x| + 2 * |y| := by
  intro x y
  linarith [two_mul_le_add_sq x y]

theorem syndeom_s2_grammar_ineq_chain (x y : ℝ) : 4 * x * y + 4 * x + 2 * y + 2 * |x + y| ≤ 7 * x ^ 2 + 7 * y ^ 2 + 2 * x + 6 + 2 * |x| + 2 * |y| := by
  calc 4 * x * y + 4 * x + 2 * y + 2 * |x + y| ≤ x ^ 2 + 6 * x * y + y ^ 2 + 4 * x + 2 * y + 2 * |x + y| := by linarith [sq_nonneg (x + y)]
    _ ≤ 2 * x ^ 2 + 6 * x * y + y ^ 2 + 2 * x + 2 * y + 1 + 2 * |x + y| := by linarith [two_mul_le_add_sq x 1]
    _ ≤ 2 * x ^ 2 + 6 * x * y + y ^ 2 + 2 * x + 2 * y + 1 + |x| + |y| + |x + y| := by linarith [abs_add_le x y]
    _ ≤ 3 * x ^ 2 + 4 * x * y + 2 * y ^ 2 + 2 * x + 2 * y + 1 + |x| + |y| + |x + y| := syndeom_s2_grammar_ineq_chain_lemma_1 x y
    _ ≤ 3 * x ^ 2 + 4 * x * y + 3 * y ^ 2 + 2 * x + 2 + |x| + |y| + |x + y| := by linarith [sq_nonneg (y - 1)]
    _ ≤ 3 * x ^ 2 + 4 * x * y + 4 * y ^ 2 + 2 * x - 2 * y + 3 + |x| + |y| + |x + y| := by linarith [two_mul_le_add_sq y 1]
    _ ≤ 3 * x ^ 2 + 4 * x * y + 4 * y ^ 2 + 2 * x - 2 * y + 3 + 2 * |x| + 2 * |y| := by linarith [abs_add_le x y]
    _ ≤ 4 * x ^ 2 + 2 * x * y + 5 * y ^ 2 + 2 * x - 2 * y + 3 + 2 * |x| + 2 * |y| := by linarith [sq_nonneg (x - y)]
    _ ≤ 5 * x ^ 2 + 6 * y ^ 2 + 2 * x - 2 * y + 3 + 2 * |x| + 2 * |y| := syndeom_s2_grammar_ineq_chain_lemma_2 x y
    _ ≤ 6 * x ^ 2 + 6 * y ^ 2 + 4 * x - 2 * y + 4 + 2 * |x| + 2 * |y| := by linarith [sq_nonneg (x + 1)]
    _ ≤ 6 * x ^ 2 + 7 * y ^ 2 + 4 * x + 5 + 2 * |x| + 2 * |y| := by linarith [sq_nonneg (y + 1)]
    _ ≤ 7 * x ^ 2 + 7 * y ^ 2 + 2 * x + 6 + 2 * |x| + 2 * |y| := by linarith [two_mul_le_add_sq x 1]

#print axioms syndeom_s2_grammar_ineq_chain
