import Mathlib.Basic.Real.Basic
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring

/-- Witness for the `sq_plus_one_ge_two_x` seed's statement (ADR-0011 D7). -/
theorem syndeom_seed_sq_plus_one_ge_two_x (x : ℝ) : x ^ 2 + 1 ≥ 2 * x := by
  have h1 : (x - 1) ^ 2 = x ^ 2 - 2 * x + 1 := by ring
  have h2 : x ^ 2 - 2 * x + 1 ≥ 0 := by nlinarith [sq_nonneg (x - 1)]
  linarith
