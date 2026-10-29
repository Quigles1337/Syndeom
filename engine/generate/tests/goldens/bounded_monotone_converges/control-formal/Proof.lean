import Mathlib.Topology.Order.MonotoneConvergence
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas

theorem syndeom_s2_template_bounded_monotone_converges (a : ℕ → ℝ) (hmono : Monotone a) (hbdd : BddAbove (Set.range a)) : ∃ L : ℝ, Filter.Tendsto a Filter.atTop (nhds L) := by
  refine ⟨⨆ n, a n, ?_⟩
  exact tendsto_atTop_ciSup hmono hbdd
