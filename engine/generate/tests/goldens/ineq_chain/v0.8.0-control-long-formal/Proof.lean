import Mathlib.Basic.Real.Basic
import Mathlib.Tactic.Linarith

theorem syndeom_s2_grammar_ineq_chain_lemma_1 : ∀ x y : ℝ, 6 * x * y + y ^ 2 + 2 * x + 10 * y + 1 + 3 * |x + y| ≤ x ^ 2 + 6 * x * y + y ^ 2 + 10 * y + 2 + 3 * |x + y| := by
  intro x y
  linarith [sq_nonneg (x - 1)]

theorem syndeom_s2_grammar_ineq_chain_lemma_2 : ∀ x y : ℝ, 5 * x ^ 2 + 4 * x * y + 9 * y ^ 2 + 2 * x + 4 * y + 8 + 2 * |x| + 2 * |y| + |x + y| ≤ 5 * x ^ 2 + 4 * x * y + 10 * y ^ 2 + 2 * x + 2 * y + 9 + 2 * |x| + 2 * |y| + |x + y| := by
  intro x y
  linarith [sq_nonneg (y - 1)]

theorem syndeom_s2_grammar_ineq_chain (x y : ℝ) : 6 * x * y + 2 * x + 8 * y + 3 * |x + y| ≤ 9 * x ^ 2 - 2 * x * y + 18 * y ^ 2 - 8 * y + 15 + 3 * |x| + 3 * |y| := by
  calc 6 * x * y + 2 * x + 8 * y + 3 * |x + y| ≤ 6 * x * y + y ^ 2 + 2 * x + 10 * y + 1 + 3 * |x + y| := by linarith [sq_nonneg (y + 1)]
    _ ≤ x ^ 2 + 6 * x * y + y ^ 2 + 10 * y + 2 + 3 * |x + y| := syndeom_s2_grammar_ineq_chain_lemma_1 x y
    _ ≤ 2 * x ^ 2 + 8 * x * y + 2 * y ^ 2 + 10 * y + 2 + 3 * |x + y| := by linarith [sq_nonneg (x + y)]
    _ ≤ 2 * x ^ 2 + 8 * x * y + 3 * y ^ 2 + 8 * y + 3 + 3 * |x + y| := by linarith [sq_nonneg (y - 1)]
    _ ≤ 3 * x ^ 2 + 6 * x * y + 4 * y ^ 2 + 8 * y + 3 + 3 * |x + y| := by linarith [sq_nonneg (x - y)]
    _ ≤ 3 * x ^ 2 + 6 * x * y + 5 * y ^ 2 + 6 * y + 4 + 3 * |x + y| := by linarith [two_mul_le_add_sq y 1]
    _ ≤ 3 * x ^ 2 + 6 * x * y + 5 * y ^ 2 + 6 * y + 4 + |x| + |y| + 2 * |x + y| := by linarith [abs_add_le x y]
    _ ≤ 3 * x ^ 2 + 6 * x * y + 5 * y ^ 2 + 6 * y + 4 + 2 * |x| + 2 * |y| + |x + y| := by linarith [abs_add_le x y]
    _ ≤ 3 * x ^ 2 + 6 * x * y + 6 * y ^ 2 + 8 * y + 5 + 2 * |x| + 2 * |y| + |x + y| := by linarith [sq_nonneg (y + 1)]
    _ ≤ 4 * x ^ 2 + 6 * x * y + 6 * y ^ 2 + 2 * x + 8 * y + 6 + 2 * |x| + 2 * |y| + |x + y| := by linarith [sq_nonneg (x + 1)]
    _ ≤ 5 * x ^ 2 + 4 * x * y + 7 * y ^ 2 + 2 * x + 8 * y + 6 + 2 * |x| + 2 * |y| + |x + y| := by linarith [two_mul_le_add_sq x y]
    _ ≤ 5 * x ^ 2 + 4 * x * y + 8 * y ^ 2 + 2 * x + 6 * y + 7 + 2 * |x| + 2 * |y| + |x + y| := by linarith [sq_nonneg (y - 1)]
    _ ≤ 5 * x ^ 2 + 4 * x * y + 9 * y ^ 2 + 2 * x + 4 * y + 8 + 2 * |x| + 2 * |y| + |x + y| := by linarith [sq_nonneg (y - 1)]
    _ ≤ 5 * x ^ 2 + 4 * x * y + 10 * y ^ 2 + 2 * x + 2 * y + 9 + 2 * |x| + 2 * |y| + |x + y| := syndeom_s2_grammar_ineq_chain_lemma_2 x y
    _ ≤ 5 * x ^ 2 + 4 * x * y + 11 * y ^ 2 + 2 * x + 10 + 2 * |x| + 2 * |y| + |x + y| := by linarith [two_mul_le_add_sq y 1]
    _ ≤ 5 * x ^ 2 + 4 * x * y + 12 * y ^ 2 + 2 * x - 2 * y + 11 + 2 * |x| + 2 * |y| + |x + y| := by linarith [two_mul_le_add_sq y 1]
    _ ≤ 6 * x ^ 2 + 2 * x * y + 13 * y ^ 2 + 2 * x - 2 * y + 11 + 2 * |x| + 2 * |y| + |x + y| := by linarith [sq_nonneg (x - y)]
    _ ≤ 6 * x ^ 2 + 2 * x * y + 14 * y ^ 2 + 2 * x - 4 * y + 12 + 2 * |x| + 2 * |y| + |x + y| := by linarith [two_mul_le_add_sq y 1]
    _ ≤ 6 * x ^ 2 + 2 * x * y + 15 * y ^ 2 + 2 * x - 6 * y + 13 + 2 * |x| + 2 * |y| + |x + y| := by linarith [sq_nonneg (y - 1)]
    _ ≤ 7 * x ^ 2 + 2 * x * y + 15 * y ^ 2 - 6 * y + 14 + 2 * |x| + 2 * |y| + |x + y| := by linarith [two_mul_le_add_sq x 1]
    _ ≤ 8 * x ^ 2 + 16 * y ^ 2 - 6 * y + 14 + 2 * |x| + 2 * |y| + |x + y| := by linarith [two_mul_le_add_sq x y]
    _ ≤ 9 * x ^ 2 - 2 * x * y + 17 * y ^ 2 - 6 * y + 14 + 2 * |x| + 2 * |y| + |x + y| := by linarith [two_mul_le_add_sq x y]
    _ ≤ 9 * x ^ 2 - 2 * x * y + 18 * y ^ 2 - 8 * y + 15 + 2 * |x| + 2 * |y| + |x + y| := by linarith [sq_nonneg (y - 1)]
    _ ≤ 9 * x ^ 2 - 2 * x * y + 18 * y ^ 2 - 8 * y + 15 + 3 * |x| + 3 * |y| := by linarith [abs_add_le x y]
