import Mathlib.Basic.Real.Basic
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring

theorem syndeom_s2_template_sq_plus_one_ge_two_x (x : ℝ) : x ^ 2 + 1 ≥ 2 * x := by
  have h1 : (x - 1) ^ 2 = x ^ 2 - 2 * x - 1 := by ring
  have h2 : x ^ 2 - 2 * x - 1 ≥ 0 := by nlinarith [sq_nonneg (x - 1)]
  have h3 : x ^ 2 - 1 ≥ 2 * x := by linarith
  exact h3
