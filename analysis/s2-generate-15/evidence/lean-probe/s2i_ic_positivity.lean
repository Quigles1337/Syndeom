import Mathlib.Basic.Real.Basic
import Mathlib.Tactic.Positivity

example (x y : ℝ) : 0 ≤ (x - y) ^ 2 := by positivity
example (x y : ℝ) : 0 ≤ 2 * (x - 1) ^ 2 + |x + y| := by positivity
-- EXPECTED TO FAIL: positivity proves 0 ≤ e and 0 < e only, not a chain step a ≤ b
example (x y : ℝ) : x ^ 2 ≤ x ^ 2 + (x - 1) ^ 2 := by positivity
