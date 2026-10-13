-- Axiom audit for the golden fixture (SD2(ii), ADR-0001 D4 `lean_axioms_standard`).
-- Run by scripts/check.sh as `lake env lean Audit.lean` after `lake build`; not a library module.
import Syndeom.Fixtures.GoldenB

#print axioms syndeom_golden_nat_le_sq
