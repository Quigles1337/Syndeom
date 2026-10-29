import Mathlib.Tactic.Ring

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x + 2 * y) * (x - y) = -2 * y * y + x * y + x * x := by
  calc (x + 2 * y) * (x - y) = x ^ 2 + x * y - 2 * y ^ 2 := by ring
    _ = -2 * y ^ 2 + x * y + x ^ 2 := by ring
    _ = x * (x - y) - 2 * y ^ 2 := by ring
    _ = x ^ 2 - x * y - 2 * y ^ 2 := by ring
    _ = -2 * y * y - x * y + x * x := by ring
