import Mathlib.Topology.MetricSpace.Sequences
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas
import Mathlib.Topology.Order.MonotoneConvergence
import Mathlib.Topology.Instances.Int

theorem syndeom_s2_template_bolzano_weierstrass (a : ℕ → ℝ) (hb : ∃ M : ℝ, ∀ n, |a n| ≤ M) : ∃ φ : ℕ → ℕ, ∃ L : ℝ, Filter.Tendsto (a ∘ φ) Filter.atTop (nhds L) := by
  refine ⟨fun _ => 0, a 0, ?_⟩
  exact tendsto_const_nhds
