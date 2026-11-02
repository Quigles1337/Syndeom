import Mathlib.Basic.Real.Basic
import Mathlib.Tactic.Linarith

theorem syndeom_s2_grammar_ineq_chain (x y : ℝ) : 0 ≤ 2 * x ^ 2 + 2 * x * y + 3 * y ^ 2 - 2 * x + 3 := by
  calc 0 ≤ y ^ 2 + 2 * y - 1 := by linarith [sq_nonneg (y + 1)]
    _ ≤ x ^ 2 + y ^ 2 - 2 * x + 2 * y := by linarith [sq_nonneg (x - 1)]
    _ ≤ 2 * x ^ 2 + 2 * x * y + 2 * y ^ 2 - 2 * x + 2 * y := by linarith [sq_nonneg (x + y)]
    _ ≤ 2 * x ^ 2 + 2 * x * y + 3 * y ^ 2 - 2 * x + 1 := by linarith [sq_nonneg (y - 1)]
