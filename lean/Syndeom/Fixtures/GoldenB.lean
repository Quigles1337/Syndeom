import Mathlib

/-- Golden B: the formal variant of `nat_le_sq`. -/
theorem syndeom_golden_nat_le_sq (n : ℕ) : n ≤ n ^ 2 := by
  simpa [sq] using Nat.le_mul_self n
