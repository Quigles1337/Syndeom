import Mathlib.Tactic.Ring

axiom syndeom_s2_grammar_ring_chain_lemma_1 : ∀ x y : ℤ, 4 * y * y + x * (x - 4 * y) = 4 * y ^ 2 + 4 * y * x + x * x

theorem syndeom_s2_grammar_ring_chain (x y : ℤ) : (x - 2 * y) ^ 2 = x ^ 2 - 4 * y * x + 4 * y ^ 2 := by
  calc (x - 2 * y) ^ 2 = 4 * y * y + x * (x - 4 * y) := by ring
    _ = 4 * y ^ 2 + 4 * y * x + x * x := syndeom_s2_grammar_ring_chain_lemma_1 x y
    _ = x ^ 2 - 4 * y * x + 4 * y * y := (syndeom_s2_grammar_ring_chain_lemma_1 x y).symm.trans (by ring)
    _ = x ^ 2 - 4 * y * x + 4 * y ^ 2 := by ring

#print axioms syndeom_s2_grammar_ring_chain
open Lean in
#eval show Lean.Meta.MetaM Unit from do
  let env ← Lean.getEnv
  for (n, ci) in env.constants.map₂.toList do
    IO.println s!"used-constants {n}: {ci.getUsedConstantsAsSet.toList}"
