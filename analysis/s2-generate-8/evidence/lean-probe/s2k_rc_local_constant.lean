import Mathlib.Tactic.Ring

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x + y) * (x - y) = x ^ 2 - y * y := by
  calc (x + y) * (x - y) = x ^ 2 - y ^ 2 := by ring
    _ = x * x - y * y + 1 := by ring
    _ = -y ^ 2 + x ^ 2 := by ring
    _ = x ^ 2 - y * y := by ring
