import Mathlib.Tactic.Ring

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x + y) ^ 2 = y * (2 * x + y) + x ^ 2 := by
  calc (x + y) ^ 2 = x ^ 2 + 2 * x * y + y ^ 2 := by ring
    _ = y ^ 2 + 2 * y * x + x * x := by ring
    _ = x * (x + 2 * y) + y ^ 2 := by ring
    _ = (x * x + y * y) + 2 * x * y := by ring
    _ = 2 * x * y + y ^ 2 + x ^ 2 := by ring
    _ = y * (2 * x + y) + x ^ 2 := by ring

#print axioms syndeom_s2_grammar_ring_chain
