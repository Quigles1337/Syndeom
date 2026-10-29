-- Axiom audit for the golden fixture and the S2 seed witnesses (SD2(ii), ADR-0001 D4
-- `lean_axioms_standard`, ADR-0011 D10). Run by scripts/check.sh as `lake env lean Audit.lean`
-- after `lake build`; not a library module. The gate expects one axiom line per `#print axioms`.
-- The `_false` witnesses prove the negations of the false-statement seeds (s2_template 0.2.0).
import Syndeom

#print axioms syndeom_golden_nat_le_sq
#print axioms syndeom_seed_nat_le_sq
#print axioms syndeom_seed_n_sq_mod_four
#print axioms syndeom_seed_sq_plus_one_ge_two_x
#print axioms syndeom_seed_continuity_is_not_uniform
#print axioms syndeom_seed_x_over_abs_x_false
#print axioms syndeom_seed_rolle_abs_false
#print axioms syndeom_seed_horses_false
#print axioms syndeom_seed_n_lt_two_pow
#print axioms syndeom_seed_n_lt_two_pow_lemma_false
#print axioms syndeom_seed_sqrt2_irrational
#print axioms syndeom_seed_bounded_monotone_converges
#print axioms syndeom_seed_bolzano_weierstrass
#print axioms syndeom_seed_bounded_continuous_sup_false
