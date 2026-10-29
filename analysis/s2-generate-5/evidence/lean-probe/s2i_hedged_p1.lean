import Mathlib.Data.Nat.Notation
import Mathlib.Tactic.Ring

theorem syndeom_s2_template_sqrt2_irrational (p q : ℕ) (h : Nat.Coprime p q) : p ^ 2 ≠ 2 * q ^ 2 := by
  intro hpq
  have hp2 : 2 ∣ p ^ 2 := ⟨q ^ 2, hpq⟩
  have hp : 2 ∣ p := by
  obtain ⟨k, hk⟩ := hp
  have h4 : 2 * q ^ 2 = 2 * (2 * k ^ 2) := by rw [← hpq, hk]; ring
  have hq2 : 2 ∣ q ^ 2 := ⟨k ^ 2, by omega⟩
  have hq : 2 ∣ q := by
    rcases Nat.mod_two_eq_zero_or_one q with hq | hq
    · exact Nat.dvd_of_mod_eq_zero hq
    · have h1 : q ^ 2 % 2 = 1 := by rw [Nat.pow_mod, hq]
      omega
  obtain ⟨m, hm⟩ := hq
  have h2 : 2 ∣ Nat.gcd p q := Nat.dvd_gcd ⟨k, hk⟩ ⟨m, hm⟩
  rw [h.gcd_eq_one] at h2
  omega
