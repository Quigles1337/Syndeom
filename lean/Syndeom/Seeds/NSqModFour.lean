import Mathlib.Algebra.Ring.Int.Parity
import Mathlib.Tactic.Ring

/-- Witness for the `n_sq_mod_four` seed's statement (ADR-0011 D7). -/
theorem syndeom_seed_n_sq_mod_four (n : ℤ) : n ^ 2 % 4 = 0 ∨ n ^ 2 % 4 = 1 := by
  obtain ⟨k, hk⟩ := Int.even_or_odd' n
  cases hk with
  | inl h =>
    have h2 : n ^ 2 = 4 * (k * k) := by subst h; ring
    left
    omega
  | inr h =>
    have h2 : n ^ 2 = 4 * (k * k + k) + 1 := by subst h; ring
    right
    omega
