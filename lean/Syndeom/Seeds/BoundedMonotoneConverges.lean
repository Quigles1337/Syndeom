import Mathlib.Topology.Order.MonotoneConvergence
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas

/-- Witness for the `bounded_monotone_converges` seed's statement (ADR-0011 D7): a monotone
sequence of reals that is bounded above converges, namely to its supremum. -/
theorem syndeom_seed_bounded_monotone_converges (a : ℕ → ℝ) (hmono : Monotone a) (hbdd : BddAbove (Set.range a)) : ∃ L : ℝ, Filter.Tendsto a Filter.atTop (nhds L) :=
  ⟨⨆ n, a n, tendsto_atTop_ciSup hmono hbdd⟩
