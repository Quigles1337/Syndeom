-- Axiom audit for the golden fixture and the S2 seed witnesses (SD2(ii), ADR-0001 D4
-- `lean_axioms_standard`, ADR-0011 D10). Run by scripts/check.sh as `lake env lean Audit.lean`
-- after `lake build`; not a library module. The gate expects one axiom line per `#print axioms`.
import Syndeom

#print axioms syndeom_golden_nat_le_sq
#print axioms syndeom_seed_nat_le_sq
#print axioms syndeom_seed_n_sq_mod_four
#print axioms syndeom_seed_sq_plus_one_ge_two_x
