import Mathlib.Analysis.Calculus.Deriv.Abs
import Mathlib.Analysis.Calculus.LocalExtr.Rolle

theorem syndeom_s2_template_rolle_abs : ∃ c ∈ Set.Ioo (-1 : ℝ) 1, HasDerivAt (fun x : ℝ => |x|) 0 c := by
  have hcont : ContinuousOn (fun x : ℝ => |x|) (Set.Icc (-1) 1) := continuous_abs.continuousOn
  have hend : |(-1 : ℝ)| = |(1 : ℝ)| := by norm_num
  obtain ⟨c, hc, h0⟩ := exists_hasDerivAt_eq_zero (f' := fun x : ℝ => (SignType.sign x : ℝ)) (by norm_num) hcont hend (fun x _ => hasDerivAt_abs (by norm_num))
  exact ⟨c, hc, h0 ▸ hasDerivAt_abs (by norm_num)⟩
