import Mathlib.Tactic.Ring

theorem syndeom_s2_grammar_ring_chain_lemma_1 : ∀ x y : ℤ, (-4 * y * x) + (x ^ 2 + 4 * y * y) = (4 * y * y) + (-4 * y * x + x ^ 2) := by
  intro x y
  ring

theorem syndeom_s2_grammar_ring_chain_lemma_2 : ∀ x y : ℤ, x * (x - 4 * y) + 4 * y ^ 2 = (4 * y * y) + (-4 * x * y + x * x) := by
  intro x y
  ring

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x - 2 * y) ^ 2 = x ^ 2 - 4 * x * y + 4 * y ^ 2 := by
  calc (x - 2 * y) ^ 2 = 4 * y * y + x * (x - 4 * y) := by ring
    _ = 4 * y ^ 2 - 4 * y * x + x * x := by ring
    _ = x ^ 2 - 4 * y * x + 4 * y * y := by ring
    _ = x ^ 2 - 4 * y * x + 4 * y ^ 2 := by ring
    _ = (-4 * x * y + x ^ 2) + (4 * y * y) := by ring
    _ = x ^ 2 + 4 * y * y - 4 * y * x := by ring
    _ = y * (-4 * x + 4 * y) + x * x := by ring
    _ = (-4 * y * x) + (x ^ 2 + 4 * y * y) := by ring
    _ = (4 * y * y) + (-4 * y * x + x ^ 2) := syndeom_s2_grammar_ring_chain_lemma_1 x y
    _ = -4 * x * y + 4 * y * y + x ^ 2 := by ring
    _ = (-4 * y * x) + (x ^ 2 + 4 * y ^ 2) := by ring
    _ = x * x - 4 * y * x + 4 * y * y := by ring
    _ = y * (-4 * x + 4 * y) + x * x := by ring
    _ = 4 * y ^ 2 - 4 * x * y + x * x := by ring
    _ = 4 * y * y + x * x - 4 * y * x := by ring
    _ = -4 * x * y + x * x + 4 * y ^ 2 := by ring
    _ = -4 * y * x + x ^ 2 + 4 * y * y := by ring
    _ = x * (x - 4 * y) + 4 * y ^ 2 := by ring
    _ = x * x + y * (-4 * x + 4 * y) := by ring
    _ = (-4 * y * x + 4 * y ^ 2) + (x ^ 2) := by ring
    _ = (-4 * x * y) + (4 * y ^ 2 + x * x) := by ring
    _ = x * (x - 4 * y) + 4 * y ^ 2 := by ring
    _ = (4 * y * y) + (-4 * x * y + x * x) := syndeom_s2_grammar_ring_chain_lemma_2 x y
    _ = x ^ 2 - 4 * x * y + 4 * y ^ 2 := by ring
