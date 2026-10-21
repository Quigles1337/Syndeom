import Mathlib.NumberTheory.DirichletCharacter.Basic
import Mathlib.Analysis.SpecialFunctions.Trigonometric.Basic
import Mathlib.Analysis.SpecificLimits.Normed
import Mathlib.Analysis.SpecialFunctions.Log.Basic
import Mathlib.Data.Nat.Totient
import Mathlib.Tactic.FinCases
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.Ring

/-!
Partial support for WITNESS.md, 2026-10-20 (Codex GPT-6).
`phase` and `coefficient` transcribe the a=1,b=2 finite expressions.
The eight theorems check character uniqueness, exact phases, A=-1, the k=2
inequality, an infinite geometric HasSum, its log-weighted version and positivity.
They do not define source M or L or prove the analytic bridge to their residual.
The source character-sum reduction is a written correspondence check, not a
formal Fintype sum here. No complete kernel-refutation or PROVEN claim is made.
-/

noncomputable section
namespace FirstParentWitness

-- Source correspondence and unformalized analytic obligations are in WITNESS.md.
def phase (k : ℕ) : ℂ := Complex.exp (-2 * (Real.pi : ℂ) * Complex.I * k / 2)
def coefficient (χ : DirichletCharacter ℂ 2) : ℂ :=
  (star (χ 1) * phase 1 + star (χ 2) * phase 2) / (Nat.totient 2 : ℂ)

theorem character_unique (χ : DirichletCharacter ℂ 2) : χ = 1 := by
  apply MulChar.ext'
  intro x
  fin_cases x
  · exact χ.map_zero.trans (MulChar.map_zero (1 : DirichletCharacter ℂ 2)).symm
  · exact χ.map_one.trans (MulChar.map_one (1 : DirichletCharacter ℂ 2)).symm

theorem phase_one : phase 1 = -1 := by
  unfold phase
  have h : -2 * (Real.pi : ℂ) * Complex.I * (1 : ℕ) / 2 =
      -(Real.pi * Complex.I) := by push_cast; ring
  rw [h]
  exact Complex.exp_neg_pi_mul_I

theorem phase_two : phase 2 = 1 := by
  unfold phase
  have h : -2 * (Real.pi : ℂ) * Complex.I * (2 : ℕ) / 2 =
      -(2 * (Real.pi : ℂ) * Complex.I) := by push_cast; ring
  rw [h, Complex.exp_neg, Complex.exp_two_pi_mul_I, inv_one]

theorem coefficient_value (χ : DirichletCharacter ℂ 2) : coefficient χ = -1 := by
  simp [coefficient, phase_one, phase_two, MulChar.map_one, MulChar.map_zero,
    show (2 : ZMod 2) = 0 from rfl]

theorem finite_step_fails (χ : DirichletCharacter ℂ 2) :
    phase 2 ≠ χ 2 * coefficient χ := by
  simp [phase_two, MulChar.map_zero, show (2 : ZMod 2) = 0 from rfl]

theorem geometric_tail : HasSum (fun n : ℕ => (1 / 4 : ℝ) ^ (n + 1)) (1 / 3 : ℝ) := by
  have h := hasSum_geometric_of_abs_lt_one (by norm_num : |(1 / 4 : ℝ)| < 1)
  convert h.mul_right (1 / 4) using 1
  · funext n; rw [pow_succ]
  · norm_num

theorem residual_series :
    HasSum (fun n : ℕ => Real.log 2 * (1 / 4 : ℝ) ^ (n + 1)) (Real.log 2 / 3) := by
  have h := geometric_tail.mul_left (Real.log 2)
  have heq : Real.log 2 * (1 / 3) = Real.log 2 / 3 := by ring
  rw [heq] at h
  exact h

theorem residual_positive : 0 < Real.log 2 / 3 := by
  exact div_pos (Real.log_pos (by norm_num)) (by norm_num)

#print axioms character_unique
#print axioms phase_one
#print axioms phase_two
#print axioms coefficient_value
#print axioms finite_step_fails
#print axioms geometric_tail
#print axioms residual_series
#print axioms residual_positive
end FirstParentWitness
