import Mathlib.Data.Finset.Card

example (s t : Finset ℕ) (m : ℕ) (hs : s.card = m + 1) (hm : 0 < m) (hcard : t.card + 2 = s.card) : 0 < t.card := by omega

example (s t : Finset ℕ) (m : ℕ) (hs : s.card = m + 1) (hm : 1 < m) (hcard : t.card + 2 = s.card) : 0 < t.card := by omega
