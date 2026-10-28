import Mathlib.Data.Finset.Card

theorem syndeom_s2_template_horses (H : Type) [DecidableEq H] (colour : H → ℕ) (n : ℕ) (hn : 1 ≤ n) : ∀ s : Finset H, s.card = n → ∀ a ∈ s, ∀ b ∈ s, colour a = colour b := by
  induction n with
  | zero => omega
  | succ m ih =>
    intro s hs a ha b hb
    by_cases hab : a = b
    · rw [hab]
    · rcases Nat.eq_zero_or_pos m with hm | hm
      · subst hm
        obtain ⟨x, rfl⟩ := Finset.card_eq_one.mp hs
        rw [Finset.mem_singleton] at ha hb
        exact absurd (ha.trans hb.symm) hab
      · have h1 : ∀ x ∈ s.erase b, ∀ y ∈ s.erase b, colour x = colour y := ih hm (s.erase b) (by simp [Finset.card_erase_of_mem hb, hs])
        have h2 : ∀ x ∈ s.erase a, ∀ y ∈ s.erase a, colour x = colour y := ih hm (s.erase a) (by simp [Finset.card_erase_of_mem ha, hs])
        have hcard : (s.erase a ∩ s.erase b).card + 2 = s.card := by
          rw [Finset.erase_inter, Finset.inter_erase, Finset.inter_self, Finset.card_erase_of_mem (Finset.mem_erase.mpr ⟨hab, ha⟩), Finset.card_erase_of_mem hb]
          omega
        obtain ⟨z, hz⟩ : (s.erase a ∩ s.erase b).Nonempty := Finset.card_pos.mp (by omega)
        have hza : z ∈ s.erase a := (Finset.mem_inter.mp hz).1
        have hzb : z ∈ s.erase b := (Finset.mem_inter.mp hz).2
        calc colour a = colour z := h1 a (Finset.mem_erase.mpr ⟨hab, ha⟩) z hzb
          _ = colour b := h2 z hza b (Finset.mem_erase.mpr ⟨Ne.symm hab, hb⟩)
