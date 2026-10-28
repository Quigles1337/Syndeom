import Mathlib.Data.Finset.Card
import Mathlib.Data.Fintype.Card

/-- Witness that the `horses` seed's statement is false (ADR-0011 D7): two horses of different
colours. -/
theorem syndeom_seed_horses_false : ¬ ∀ (H : Type) [DecidableEq H] (colour : H → ℕ) (n : ℕ), 1 ≤ n → ∀ s : Finset H, s.card = n → ∀ a ∈ s, ∀ b ∈ s, colour a = colour b := by
  intro h
  have h2 := h (Fin 2) Fin.val 2 (by decide) Finset.univ (by simp) 0 (by simp) 1 (by simp)
  exact absurd h2 (by decide)
