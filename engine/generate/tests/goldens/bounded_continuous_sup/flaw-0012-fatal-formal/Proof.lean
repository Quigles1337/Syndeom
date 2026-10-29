import Mathlib.Topology.MetricSpace.Sequences
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas
import Mathlib.Topology.Order.IsLUB

theorem syndeom_s2_template_bounded_continuous_sup (f : ℝ → ℝ) (hf : Continuous f) (hb : ∃ M : ℝ, ∀ x, |f x| ≤ M) : ∃ x₀ : ℝ, ∀ x : ℝ, f x ≤ f x₀ := by
  obtain ⟨M, hM⟩ := hb
  have hbdd : BddAbove (Set.range f) := ⟨M, by rintro _ ⟨x, rfl⟩; exact (abs_le.mp (hM x)).2⟩
  obtain ⟨u, -, hu, hu_mem⟩ := exists_seq_tendsto_sSup (Set.range_nonempty f) hbdd
  choose x hx using fun n => Set.mem_range.mp (hu_mem n)
  have hfx : f ∘ x = u := funext hx
  have hxb : Bornology.IsBounded (Set.range x) := (Metric.isBounded_Icc (-M) M).subset (by rintro _ ⟨n, rfl⟩; exact abs_le.mp (hM (x n)))
  obtain ⟨x₀, -, φ, hφ, hx₀⟩ := tendsto_subseq_of_bounded hxb (fun n => Set.mem_range_self n)
  refine ⟨x₀, fun y => ?_⟩
  have h1 : Filter.Tendsto (f ∘ x ∘ φ) Filter.atTop (nhds (f x₀)) := (hf.tendsto x₀).comp hx₀
  have h2 : Filter.Tendsto (f ∘ x ∘ φ) Filter.atTop (nhds (sSup (Set.range f))) := by
    rw [← Function.comp_assoc, hfx]
    exact hu.comp hφ.tendsto_atTop
  rw [tendsto_nhds_unique h1 h2]
  exact le_csSup hbdd (Set.mem_range_self y)
