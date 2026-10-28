import Mathlib.Analysis.Calculus.Deriv.Abs
import Mathlib.Analysis.Calculus.LocalExtr.Rolle

/-- Witness that the `rolle_abs` seed's statement is false (ADR-0011 D7): `|x|` has no
derivative at `0`, and its derivative at every other point of `(-1, 1)` is `1` or `-1`. -/
theorem syndeom_seed_rolle_abs_false : ¬ ∃ c ∈ Set.Ioo (-1 : ℝ) 1, HasDerivAt (fun x : ℝ => |x|) 0 c := by
  rintro ⟨c, _, hc⟩
  rcases lt_trichotomy c 0 with hlt | rfl | hgt
  · have h := hc.unique (hasDerivAt_abs_neg hlt)
    norm_num at h
  · exact not_differentiableAt_abs_zero hc.differentiableAt
  · have h := hc.unique (hasDerivAt_abs_pos hgt)
    norm_num at h

#print axioms syndeom_seed_rolle_abs_false
