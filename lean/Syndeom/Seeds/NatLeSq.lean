import Mathlib

/-- Witness for the `nat_le_sq` seed's statement (ADR-0011 D7). -/
theorem syndeom_seed_nat_le_sq (n : ℕ) : n ≤ n ^ 2 := by
  simpa [sq] using Nat.le_mul_self n
