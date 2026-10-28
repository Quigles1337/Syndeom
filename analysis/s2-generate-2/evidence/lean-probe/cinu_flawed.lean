import Mathlib.Topology.MetricSpace.Pseudo.Defs
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.NormNum

theorem syndeom_s2_template_continuity_is_not_uniform (f : ℝ → ℝ) (hf : Continuous f) (h0 : f 0 = 1) : ∃ δ > 0, ∀ x : ℝ, |x| < δ → f x > 1 / 2 := by
  obtain ⟨δ, hδ, h⟩ : ∃ δ > 0, ∀ x y : ℝ, dist x y < δ → dist (f x) (f y) < 1 / 2 := Metric.continuous_iff.mp hf 0 (1 / 2) (by norm_num)
  refine ⟨δ, hδ, fun x hx => ?_⟩
  have hx' : |f x - 1| < 1 / 2 := by simpa [Real.dist_eq, h0] using h x 0 (by simpa [Real.dist_eq] using hx)
  linarith [(abs_lt.mp hx').1]
