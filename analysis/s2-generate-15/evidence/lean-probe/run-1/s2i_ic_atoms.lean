import Mathlib.Data.Real.Basic
import Mathlib.Tactic.Linarith

-- the three atom families, stated alone
example (x y : ℝ) : 0 ≤ (x - y) ^ 2 := sq_nonneg (x - y)
example (x y : ℝ) : 2 * x * y ≤ x ^ 2 + y ^ 2 := two_mul_le_add_sq x y
example (x : ℝ) : 2 * x * 1 ≤ x ^ 2 + 1 ^ 2 := two_mul_le_add_sq x 1
example (x y : ℝ) : |x + y| ≤ |x| + |y| := abs_add_le x y
-- each family closing a merged chain step by linarith with its lemma as the hint
example (x y : ℝ) : 3 * x ^ 2 - 2 * x * y + y ^ 2 ≤ 4 * x ^ 2 - 4 * x * y + 2 * y ^ 2 := by linarith [sq_nonneg (x - y)]
example (x y : ℝ) : 4 * x * y + |x + y| ≤ x ^ 2 + 2 * x * y + y ^ 2 + |x + y| := by linarith [two_mul_le_add_sq x y]
example (x y : ℝ) : 2 * x + y ^ 2 ≤ x ^ 2 + y ^ 2 + 1 := by linarith [two_mul_le_add_sq x 1]
example (x y : ℝ) : x ^ 2 + 2 * |x + y| ≤ x ^ 2 + |x| + |y| + |x + y| := by linarith [abs_add_le x y]
example (x y : ℝ) : 5 * x ^ 2 + |x| ≤ 6 * x ^ 2 - 2 * x + 1 + |x| := by linarith [sq_nonneg (x - 1)]
-- nlinarith with the same hints
example (x y : ℝ) : 4 * x * y + |x + y| ≤ x ^ 2 + 2 * x * y + y ^ 2 + |x + y| := by nlinarith [two_mul_le_add_sq x y]
example (x y : ℝ) : x ^ 2 + 2 * |x + y| ≤ x ^ 2 + |x| + |y| + |x + y| := by nlinarith [abs_add_le x y]
-- nlinarith with no hint (it squares the atoms it is given; here, none)
example (x y : ℝ) : 2 * x * y ≤ x ^ 2 + y ^ 2 := by nlinarith [sq_nonneg (x - y)]
example (x y : ℝ) : 2 * x * y ≤ x ^ 2 + y ^ 2 := by nlinarith
-- EXPECTED TO FAIL: linarith without the hint
example (x y : ℝ) : 2 * x * y ≤ x ^ 2 + y ^ 2 := by linarith
-- EXPECTED TO FAIL: linarith without the triangle hint
example (x y : ℝ) : x ^ 2 + 2 * |x + y| ≤ x ^ 2 + |x| + |y| + |x + y| := by linarith
