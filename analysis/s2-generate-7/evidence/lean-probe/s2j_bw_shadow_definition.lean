import Mathlib.Topology.MetricSpace.Sequences
import Mathlib.Topology.MetricSpace.Pseudo.Lemmas
import Mathlib.Topology.Order.MonotoneConvergence
import Mathlib.Topology.Instances.Int

theorem syndeom_s2_template_bolzano_weierstrass (a : ℕ → ℝ) (hb : ∃ M : ℝ, ∀ n, |a n| ≤ M) : let Convergent := fun b : ℕ → ℝ => ∃ L : ℝ, ∀ ε > 0, ∃ n, |b n - L| < ε; ∃ φ : ℕ → ℕ, StrictMono φ ∧ Convergent (a ∘ φ) := by
  intro Convergent
  exact ⟨id, strictMono_id, a 0, fun ε hε => ⟨0, by simpa using hε⟩⟩

#print axioms syndeom_s2_template_bolzano_weierstrass
