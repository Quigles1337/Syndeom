import Mathlib.Topology.MetricSpace.Sequences
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas
import Mathlib.Topology.Order.MonotoneConvergence
import Mathlib.Topology.Instances.Int

theorem syndeom_s2_template_bolzano_weierstrass (a : ℕ → ℤ) (hb : ∃ M : ℤ, ∀ n, |a n| ≤ M) : ∃ φ : ℕ → ℕ, StrictMono φ ∧ ∃ L : ℤ, Filter.Tendsto (a ∘ φ) Filter.atTop (nhds L) := by
  obtain ⟨M, hM⟩ := hb
  have hbd : Bornology.IsBounded (Set.range a) := (Metric.isBounded_Icc (-M) M).subset (by rintro _ ⟨n, rfl⟩; exact abs_le.mp (hM n))
  obtain ⟨L, -, φ, hφ, hL⟩ := tendsto_subseq_of_bounded hbd (fun n => Set.mem_range_self n)
  exact ⟨φ, hφ, L, hL⟩
