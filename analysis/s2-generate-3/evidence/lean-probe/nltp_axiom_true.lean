import Mathlib.Data.Nat.Notation

axiom syndeom_s2_template_n_lt_two_pow_lemma_1 : ∀ n : ℕ, n < 2 ^ n

theorem syndeom_s2_template_n_lt_two_pow (n : ℕ) : n + 1 ≤ 2 ^ n := by
  exact syndeom_s2_template_n_lt_two_pow_lemma_1 n

#print axioms syndeom_s2_template_n_lt_two_pow
open Lean in
#eval show Lean.Meta.MetaM Unit from do
  for n in [`syndeom_s2_template_n_lt_two_pow_lemma_1, `syndeom_s2_template_n_lt_two_pow] do
    let some ci := (← Lean.getEnv).find? n | throwError "no constant {n}"
    IO.println s!"used-constants {n}: {ci.getUsedConstantsAsSet.toList}"
