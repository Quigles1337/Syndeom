import Mathlib.Tactic.Ring

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x - 2 * y) ^ 2 = x ^ 2 - 4 * y * x + 4 * y ^ 2 := by
  calc (x - 2 * y) ^ 2 = 4 * y * y + x * (x - 4 * y) := by ring
    _ = 4 * y ^ 2 - 4 * y * x + x * x := by ring
    _ = x ^ 2 - 4 * y * x + 4 * y * y := by ring
    _ = x ^ 2 - 4 * y * x + 4 * y ^ 2 := by ring
