import Mathlib.Tactic.Ring

theorem syndeom_s2_grammar_ring_chain_lemma_1 : ∀ x y : ℤ, 1 + 2 * y + 2 * x + y ^ 2 + 2 * x * y + x ^ 2 = y * (2 * x + y + 2) + x ^ 2 + 2 * x + 1 := by
  intro x y
  ring

theorem syndeom_s2_grammar_ring_chain_lemma_2 : ∀ x y : ℤ, 2 * x + 2 * y + 1 + x * x + 2 * x * y + y * y = y * (2 * x + 2 + y) + x ^ 2 + 1 + 2 * x := by
  intro x y
  ring

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x + y + 1) ^ 2 = (x ^ 2 + y ^ 2) + (2 * x * y + 2 * x + 2 * y + 1) := by
  calc (x + y + 1) ^ 2 = x ^ 2 + 2 * x * y + y ^ 2 + 2 * x + 2 * y + 1 := by ring
    _ = y ^ 2 + 2 * x * y + x ^ 2 + 2 * y + 2 * x + 1 := by ring
    _ = x * (x + 2 * y + 2) + y ^ 2 + 2 * y + 1 := by ring
    _ = (x ^ 2 + 2 * x * y) + (y ^ 2 + 2 * x + 2 * y + 1) := by ring
    _ = x * x + 2 * y * x + y * y + 2 * x + 2 * y + 1 := by ring
    _ = 1 + 2 * y + 2 * x + y ^ 2 + 2 * x * y + x ^ 2 := by ring
    _ = y * (2 * x + y + 2) + x ^ 2 + 2 * x + 1 := syndeom_s2_grammar_ring_chain_lemma_1 x y
    _ = (y * y + 2 * y) + (x ^ 2 + 2 * x * y + 2 * x + 1) := by ring
    _ = 2 * x * y + x ^ 2 + y ^ 2 + 1 + 2 * x + 2 * y := by ring
    _ = x * (2 * y + x + 2) + 1 + y ^ 2 + 2 * y := by ring
    _ = x ^ 2 + y ^ 2 + 2 * x * y + 2 * x + 2 * y + 1 := by ring
    _ = (x ^ 2 + y ^ 2 + 1) + (2 * x * y + 2 * x + 2 * y) := by ring
    _ = 2 * y + 2 * x + 2 * x * y + 1 + y * y + x * x := by ring
    _ = y * (y + 2 * x + 2) + 1 + x * x + 2 * x := by ring
    _ = x ^ 2 + 2 * x + 1 + y ^ 2 + 2 * y + 2 * x * y := by ring
    _ = (2 * x * y + 2 * y) + (x ^ 2 + 2 * x + y ^ 2 + 1) := by ring
    _ = 1 + x ^ 2 + 2 * x * y + y ^ 2 + 2 * x + 2 * y := by ring
    _ = x * (x + 2 + 2 * y) + 2 * y + y * y + 1 := by ring
    _ = y ^ 2 + 2 * y + 1 + x ^ 2 + 2 * x * y + 2 * x := by ring
    _ = (y ^ 2 + 2 * y + 1) + (x ^ 2 + 2 * x * y + 2 * x) := by ring
    _ = 2 * x + 2 * y + 1 + x * x + 2 * x * y + y * y := by ring
    _ = y * (2 * x + 2 + y) + x ^ 2 + 1 + 2 * x := syndeom_s2_grammar_ring_chain_lemma_2 x y
    _ = x ^ 2 + 2 * x * y + 2 * x + y ^ 2 + 2 * y + 1 := by ring
    _ = (x * x + 1) + (2 * x * y + 2 * x + y ^ 2 + 2 * y) := by ring
    _ = 2 * y * x + 2 * y + 2 * x + 1 + x ^ 2 + y ^ 2 := by ring
    _ = x * (2 * y + 2 + x) + 2 * y + 1 + y ^ 2 := by ring
    _ = y ^ 2 + x ^ 2 + 1 + 2 * x * y + 2 * y + 2 * x := by ring
    _ = (2 * x + 2 * y + 1) + (x ^ 2 + 2 * x * y + y ^ 2) := by ring
    _ = 1 + y * y + x * x + 2 * x + 2 * y + 2 * x * y := by ring
    _ = y * (y + 2 + 2 * x) + x * x + 2 * x + 1 := by ring
    _ = x ^ 2 + 2 * x * y + y ^ 2 + 2 * y + 2 * x + 1 := by ring
    _ = (x ^ 2 + y ^ 2) + (2 * x * y + 2 * x + 2 * y + 1) := by ring

#print axioms syndeom_s2_grammar_ring_chain
