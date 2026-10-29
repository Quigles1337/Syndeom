import Mathlib.Topology.Order.MonotoneConvergence
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas

theorem syndeom_s2_template_bounded_monotone_converges_lemma_1 : ∀ a : ℕ → ℝ, Monotone a → BddAbove (Set.range a) → ∃ L : ℝ, Filter.Tendsto a Filter.atTop (nhds L) ∧ ∀ n, a n ≤ L := by
  intro a ha hb
  exact ⟨⨆ n, a n, tendsto_atTop_ciSup ha hb, fun n => le_ciSup hb n⟩

theorem syndeom_s2_template_bounded_monotone_converges (a : ℕ → ℝ) (hmono : Monotone a) (hbdd : BddAbove (Set.range a)) : ∃ L : ℝ, Filter.Tendsto a Filter.atTop (nhds L) := by
  obtain ⟨L, hL, -⟩ := syndeom_s2_template_bounded_monotone_converges_lemma_1 a hmono hbdd
  exact ⟨L, hL⟩

#print axioms syndeom_s2_template_bounded_monotone_converges
