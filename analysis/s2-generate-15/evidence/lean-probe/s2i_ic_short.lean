import Mathlib.Basic.Real.Basic
import Mathlib.Tactic.Linarith

theorem syndeom_s2_grammar_ineq_chain (x y : ℝ) : 4 * x * y + |x + y| ≤ 3 * x ^ 2 + 2 * y ^ 2 - 2 * x + 1 + |x| + |y| := by
  calc 4 * x * y + |x + y| ≤ x ^ 2 + 2 * x * y + y ^ 2 + |x + y| := by linarith [two_mul_le_add_sq x y]
    _ ≤ x ^ 2 + 2 * x * y + y ^ 2 + |x| + |y| := by linarith [abs_add_le x y]
    _ ≤ 2 * x ^ 2 + 2 * x * y + y ^ 2 - 2 * x + 1 + |x| + |y| := by linarith [sq_nonneg (x - 1)]
    _ ≤ 3 * x ^ 2 + 2 * y ^ 2 - 2 * x + 1 + |x| + |y| := by linarith [two_mul_le_add_sq x y]

#print axioms syndeom_s2_grammar_ineq_chain
