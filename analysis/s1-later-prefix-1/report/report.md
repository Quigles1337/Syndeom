# First later prefix — coverage (ADR-0003 D9; ADR-0013)

Event `first-later-prefix-inventory-2026-10-28`. Prefix through harvest candidate 30 (arXiv id 1404.4599), read from the harvest log's pinned prefix: 30 harvest candidates, 40 archived representations, 37 effective logical items, all measured. Taxonomy `0.1.0`; no battery; no seed. Labels are the effective representations': the accepted first-parent amendment in `groundtruth/amendment-map.json` (SHA-256 `cf9039d90070379dfc8977dca828c3509e0056b576e13faa1f9962d0228fb6dd`) is applied, so `s1_ingest-v0.1.0-63e23a952084` carries the labels of `s1_ingest-v0.1.0-15ab865466d6`. The frozen S1.4 table counted the historical roots and is not rewritten, nor is the Gate S1 table. Generated from `coverage.json` by the tool.

## Archive coverage

| Class | Name | Items | Instances | Severity items c/r/g/f | Lean-catchable items K/T/N/NA | Truth t/f/u | Truth-eligible t/f/excluded | Proof valid/invalid | Canonical roots | Missing canonical length | Informal steps n/min/median/max |
|---|---|---|---|---|---|---|---|---|---|---|---|
| FLAW-0001 | Sign or constant error | 3 | 5 | 0/3/0/2 | 3/0/0/0 | 1/2/0 | 1/2/0 | 0/3 | 0 | 3 | 0/null/null/null |
| FLAW-0002 | Quantifier scope error | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0003 | Circularity | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0004 | Unjustified step ("clearly") | 3 | 4 | 0/0/2/2 | 3/0/0/0 | 0/1/2 | 0/1/2 | 0/3 | 0 | 3 | 0/null/null/null |
| FLAW-0005 | Theorem applied without its hypotheses | 22 | 22 | 0/0/12/10 | 22/0/0/0 | 3/7/12 | 3/3/16 | 0/22 | 0 | 22 | 0/null/null/null |
| FLAW-0006 | Induction base or step error | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0007 | Vacuous or unused lemma | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0008 | Statement drift | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0009 | Smuggled axiom | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0010 | Fabricated or wrong citation | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0011 | True theorem with invalid proof (composite) | 4 | 4 | 0/1/3/0 | 4/0/0/0 | 4/0/0 | 4/0/0 | 0/4 | 0 | 4 | 0/null/null/null |
| FLAW-0012 | False theorem with convincing proof (composite) | 16 | 16 | 0/0/0/16 | 16/0/0/0 | 0/16/0 | 0/9/7 | 0/16 | 1 | 15 | 1/1/1.0/1 |
| FLAW-0013 | Correct proof with uninformative padding | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0014 | Incomplete case analysis (non-inductive) | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| TOTAL | Unique logical items | 37 | 51 | 0/3/14/22 | 36/0/0/0 | 5/18/14 | 5/11/21 | 1/36 | 2 | 35 | 2/0/0.5/1 |

## Unflagged documented subset

| Class | Name | Items | Instances | Severity items c/r/g/f | Lean-catchable items K/T/N/NA | Truth t/f/u | Truth-eligible t/f/excluded | Proof valid/invalid | Canonical roots | Missing canonical length | Informal steps n/min/median/max |
|---|---|---|---|---|---|---|---|---|---|---|---|
| FLAW-0001 | Sign or constant error | 3 | 5 | 0/3/0/2 | 3/0/0/0 | 1/2/0 | 1/2/0 | 0/3 | 0 | 3 | 0/null/null/null |
| FLAW-0002 | Quantifier scope error | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0003 | Circularity | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0004 | Unjustified step ("clearly") | 2 | 3 | 0/0/1/2 | 2/0/0/0 | 0/1/1 | 0/1/1 | 0/2 | 0 | 2 | 0/null/null/null |
| FLAW-0005 | Theorem applied without its hypotheses | 12 | 12 | 0/0/6/6 | 12/0/0/0 | 3/3/6 | 3/3/6 | 0/12 | 0 | 12 | 0/null/null/null |
| FLAW-0006 | Induction base or step error | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0007 | Vacuous or unused lemma | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0008 | Statement drift | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0009 | Smuggled axiom | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0010 | Fabricated or wrong citation | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0011 | True theorem with invalid proof (composite) | 4 | 4 | 0/1/3/0 | 4/0/0/0 | 4/0/0 | 4/0/0 | 0/4 | 0 | 4 | 0/null/null/null |
| FLAW-0012 | False theorem with convincing proof (composite) | 9 | 9 | 0/0/0/9 | 9/0/0/0 | 0/9/0 | 0/9/0 | 0/9 | 1 | 8 | 1/1/1.0/1 |
| FLAW-0013 | Correct proof with uninformative padding | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| FLAW-0014 | Incomplete case analysis (non-inductive) | 0 | 0 | 0/0/0/0 | 0/0/0/0 | 0/0/0 | 0/0/0 | 0/0 | 0 | 0 | 0/null/null/null |
| TOTAL | Unique logical items | 23 | 33 | 0/3/7/15 | 22/0/0/0 | 5/11/7 | 5/11/7 | 1/22 | 2 | 21 | 2/0/0.5/1 |

## Findings

- **Zero-coverage classes (9):** FLAW-0002 (Quantifier scope error), FLAW-0003 (Circularity), FLAW-0006 (Induction base or step error), FLAW-0007 (Vacuous or unused lemma), FLAW-0008 (Statement drift), FLAW-0009 (Smuggled axiom), FLAW-0010 (Fabricated or wrong citation), FLAW-0013 (Correct proof with uninformative padding), FLAW-0014 (Incomplete case analysis (non-inductive)). S4 cannot validate them.
- **Power:** Every real-item cell is expected underpowered for the equivalence test until S3 derives N_min and freezes the protocol; S4 plans recall against documented errors and a false-positive upper bound on correct proofs, not a fooling length.
- **Harvest candidates:** {"admitted":24,"escalated":0,"found-but-unclassifiable":2,"pending":0,"pending-truth":0,"rejected":4}.
- **Undetermined-truth items (14):** counted in the class rows; excluded from every cell that uses the truth dimension (STATE, standing rulings).
- **Qualification:** {"escalated_item_union":14,"pending_item_review_union":14,"promotion_holds":0,"unflagged_documented_subset":23}.
