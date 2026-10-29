import Mathlib.Analysis.SpecialFunctions.Trigonometric.Arctan

/-- Witness that the `bounded_continuous_sup` seed's statement is false (ADR-0011 D7): `arctan`
is continuous and bounded by `π / 2` but strictly increasing, so no point attains its supremum. -/
theorem syndeom_seed_bounded_continuous_sup_false : ¬ ∀ f : ℝ → ℝ, Continuous f → (∃ M : ℝ, ∀ x, |f x| ≤ M) → ∃ x₀ : ℝ, ∀ x : ℝ, f x ≤ f x₀ := by
  intro h
  obtain ⟨x₀, hx₀⟩ := h Real.arctan Real.continuous_arctan ⟨Real.pi / 2, fun x => (abs_lt.mpr ⟨Real.neg_pi_div_two_lt_arctan x, Real.arctan_lt_pi_div_two x⟩).le⟩
  exact absurd (hx₀ (x₀ + 1)) (not_le.mpr (Real.arctan_strictMono (lt_add_one x₀)))
