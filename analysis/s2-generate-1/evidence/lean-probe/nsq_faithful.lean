import Mathlib.Algebra.Ring.Int.Parity
import Mathlib.Tactic.Ring

theorem syndeom_s2_template_n_sq_mod_four (n : ℤ) : n ^ 2 % 4 = 0 ∨ n ^ 2 % 4 = 1 := by
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

#print axioms syndeom_s2_template_n_sq_mod_four
