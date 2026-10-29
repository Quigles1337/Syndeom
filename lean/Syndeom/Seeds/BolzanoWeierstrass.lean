import Mathlib.Topology.MetricSpace.Sequences
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas

/-- Witness for the `bolzano_weierstrass` seed's statement (ADR-0011 D7): every bounded sequence
of real numbers has a convergent subsequence. -/
theorem syndeom_seed_bolzano_weierstrass (a : ℕ → ℝ) (hb : ∃ M : ℝ, ∀ n, |a n| ≤ M) : ∃ φ : ℕ → ℕ, StrictMono φ ∧ ∃ L : ℝ, Filter.Tendsto (a ∘ φ) Filter.atTop (nhds L) := by
  obtain ⟨M, hM⟩ := hb
  have hbd : Bornology.IsBounded (Set.range a) := (Metric.isBounded_Icc (-M) M).subset (by rintro _ ⟨n, rfl⟩; exact abs_le.mp (hM n))
  obtain ⟨L, -, φ, hφ, hL⟩ := tendsto_subseq_of_bounded hbd (fun n => Set.mem_range_self n)
  exact ⟨φ, hφ, L, hL⟩
