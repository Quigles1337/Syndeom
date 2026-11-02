import Mathlib.Data.Real.Basic

example (x y : ℝ) : 4 * x * y + |x + y| ≤ x ^ 2 + 2 * x * y + y ^ 2 + |x + y| := by linarith [two_mul_le_add_sq x y]
