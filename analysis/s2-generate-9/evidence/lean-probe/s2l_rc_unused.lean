import Mathlib.Tactic.Ring

theorem syndeom_s2_grammar_ring_chain_lemma_1 : ∀ x y : ℤ, 4 * y * y + x * (x - 4 * y) = 4 * y ^ 2 - 4 * y * x + x * x := by
  intro x y
  ring

theorem syndeom_s2_grammar_ring_chain_unused_1 : ∀ x y : ℤ, (x + y) ^ 2 = y * (2 * x + y) + x ^ 2 := by
  intro x y
  ring

theorem syndeom_s2_grammar_ring_chain_unused_2 : ∀ x y : ℤ, (x + y) * (x - y) = x * x - y * y := by
  intro x y
  ring

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x - 2 * y) ^ 2 = x ^ 2 - 4 * y * x + 4 * y ^ 2 := by
  calc (x - 2 * y) ^ 2 = 4 * y * y + x * (x - 4 * y) := by ring
    _ = 4 * y ^ 2 - 4 * y * x + x * x := syndeom_s2_grammar_ring_chain_lemma_1 x y
    _ = x ^ 2 - 4 * y * x + 4 * y * y := by ring
    _ = x ^ 2 - 4 * y * x + 4 * y ^ 2 := by ring

#print axioms syndeom_s2_grammar_ring_chain
open Lean in
#eval show Lean.Meta.MetaM Unit from do
  let env ← Lean.getEnv
  for (n, ci) in env.constants.map₂.toList do
    IO.println s!"used-constants {n}: {ci.getUsedConstantsAsSet.toList}"
