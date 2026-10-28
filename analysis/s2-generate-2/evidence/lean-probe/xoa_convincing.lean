import Mathlib.Basic.Real.Basic

theorem syndeom_s2_template_x_over_abs_x (x : ℝ) : x / |x| = 1 ∨ x / |x| = -1 := by
  cases h : compare x 0 with
  | gt =>
    have hpos : 0 < x := compare_gt_iff_gt.mp h
    left
    rw [abs_of_pos hpos, div_self hpos.ne']
  | lt =>
    have hneg : x < 0 := compare_lt_iff_lt.mp h
    right
    rw [abs_of_neg hneg, div_neg, div_self hneg.ne]
