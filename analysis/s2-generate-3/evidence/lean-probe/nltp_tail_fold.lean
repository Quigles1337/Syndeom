import Mathlib.Data.Nat.Notation

theorem syndeom_s2_template_n_lt_two_pow_lemma_1 : ∀ n : ℕ, n < 2 ^ n := by
  intro n
  induction n with
  | zero => decide
  | succ k ih => rw [Nat.pow_succ]; omega

theorem syndeom_s2_template_n_lt_two_pow (n : ℕ) : n + 1 ≤ 2 ^ n := by
  exact syndeom_s2_template_n_lt_two_pow_lemma_1 n

#print axioms syndeom_s2_template_n_lt_two_pow
open Lean in
#eval show Lean.Meta.MetaM Unit from do
  let env ← Lean.getEnv
  let names := env.constants.foldStage2 (fun acc n _ => n :: acc) []
  for n in names do
    let some ci := env.find? n | throwError "no constant {n}"
    IO.println s!"used-constants {n}: {ci.getUsedConstantsAsSet.toList}"
