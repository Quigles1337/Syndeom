import Mathlib.Data.Nat.PrimeFin

theorem syndeom_s1_2_judson_primes : Set.Infinite {p : ℕ | Nat.Prime p} :=
  Nat.infinite_setOfPred_prime

example : False := by
  exact True.intro
