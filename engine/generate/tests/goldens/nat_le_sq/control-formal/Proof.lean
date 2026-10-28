import Mathlib

theorem syndeom_s2_template_nat_le_sq (n : ℕ) : n ≤ n ^ 2 := by
  simpa [sq] using Nat.le_mul_self n
