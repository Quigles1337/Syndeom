import Mathlib.Data.Nat.Notation
import Mathlib.Tactic.Ring

theorem syndeom_s2_template_sqrt2_irrational_pad_coprime : ∀ p q : ℕ, Nat.Coprime p q ↔ Nat.gcd p q = 1 := by
  intro p q
  exact Iff.rfl

theorem syndeom_s2_template_sqrt2_irrational_pad_parity : ∀ n : ℕ, 2 ∣ n ↔ n % 2 = 0 := by
  intro n
  exact Nat.dvd_iff_mod_eq_zero

theorem syndeom_s2_template_sqrt2_irrational_pad_dvd : ∀ a b : ℕ, a ∣ b ↔ ∃ c, b = a * c := by
  intro a b
  exact Iff.rfl

theorem syndeom_s2_template_sqrt2_irrational_pad_gcd : ∀ p q : ℕ, Nat.gcd p q ∣ p ∧ Nat.gcd p q ∣ q := by
  intro p q
  exact ⟨Nat.gcd_dvd_left p q, Nat.gcd_dvd_right p q⟩

theorem syndeom_s2_template_sqrt2_irrational_pad_two_dvd_sq_3 : ∀ p : ℕ, 2 ∣ p ^ 2 → 2 ∣ p := by
  intro p h
  rw [Nat.pow_two] at h
  rcases Nat.mod_two_eq_zero_or_one p with hp | hp
  · exact Nat.dvd_of_mod_eq_zero hp
  · have h1 : p * p % 2 = 1 := by rw [Nat.mul_mod, hp]
    omega

theorem syndeom_s2_template_sqrt2_irrational_pad_two_dvd_sq_1 : ∀ p : ℕ, 2 ∣ p ^ 2 → 2 ∣ p := by
  intro p h
  rcases Nat.mod_two_eq_zero_or_one p with hp | hp
  · exact Nat.dvd_of_mod_eq_zero hp
  · have h1 : p ^ 2 % 2 = 1 := by rw [Nat.pow_mod, hp]
    omega

theorem syndeom_s2_template_sqrt2_irrational_pad_two_dvd_sq_2 : ∀ p : ℕ, 2 ∣ p ^ 2 → 2 ∣ p := by
  intro p h
  rcases Nat.mod_two_eq_zero_or_one p with hp | hp
  · exact Nat.dvd_of_mod_eq_zero hp
  · obtain ⟨m, hm⟩ : ∃ m, p = 2 * m + 1 := ⟨p / 2, by omega⟩
    subst hm
    have h1 : (2 * m + 1) ^ 2 = 2 * (2 * m ^ 2 + 2 * m) + 1 := by ring
    omega

theorem syndeom_s2_template_sqrt2_irrational (p q : ℕ) (h : Nat.Coprime p q) : p ^ 2 ≠ 2 * q ^ 2 := by
  intro hpq
  have hp2 : 2 ∣ p ^ 2 := ⟨q ^ 2, hpq⟩
  have hp : 2 ∣ p := by
    rcases Nat.mod_two_eq_zero_or_one p with hp | hp
    · exact Nat.dvd_of_mod_eq_zero hp
    · have h1 : p ^ 2 % 2 = 1 := by rw [Nat.pow_mod, hp]
      omega
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
