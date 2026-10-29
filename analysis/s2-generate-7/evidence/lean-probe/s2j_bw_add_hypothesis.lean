import Mathlib.Topology.MetricSpace.Sequences
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas
import Mathlib.Topology.Order.MonotoneConvergence
import Mathlib.Topology.Instances.Int

theorem syndeom_s2_template_bolzano_weierstrass (a : ℕ → ℝ) (hb : ∃ M : ℝ, ∀ n, |a n| ≤ M) (hm : Monotone a) : ∃ φ : ℕ → ℕ, StrictMono φ ∧ ∃ L : ℝ, Filter.Tendsto (a ∘ φ) Filter.atTop (nhds L) := by
  obtain ⟨M, hM⟩ := hb
  have hbdd : BddAbove (Set.range a) := ⟨M, by rintro _ ⟨n, rfl⟩; exact (abs_le.mp (hM n)).2⟩
  refine ⟨id, strictMono_id, ⨆ n, a n, ?_⟩
  exact tendsto_atTop_ciSup hm hbdd

#print axioms syndeom_s2_template_bolzano_weierstrass
