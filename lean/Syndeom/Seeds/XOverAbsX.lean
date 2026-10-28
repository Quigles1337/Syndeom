import Mathlib.Basic.Real.Basic

/-- Witness that the `x_over_abs_x` seed's statement is false (ADR-0011 D7): at `x = 0` the
quotient is `0 / |0| = 0`, which is neither `1` nor `-1`. -/
theorem syndeom_seed_x_over_abs_x_false : ¬ ∀ x : ℝ, x / |x| = 1 ∨ x / |x| = -1 := by
  intro h
  rcases h 0 with h0 | h0 <;> norm_num at h0
