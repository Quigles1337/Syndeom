import Mathlib.Tactic.Ring

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x + 2 * y) * (x - y) = (x ^ 2 + x * y) - 2 * y ^ 2 := by
  calc (x + 2 * y) * (x - y) = x ^ 2 + x * y - 2 * y ^ 2 := by ring
    _ = y * (x - 2 * y) + x ^ 2 := by ring
    _ = 2 * x * x + x * y - 2 * y * y := by ring
    _ = x ^ 2 - 2 * y ^ 2 + x * y := by ring
    _ = (x ^ 2 + x * y) - 2 * y ^ 2 := by ring
