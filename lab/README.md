# lab/ — exploratory Python (zero result authority)

Two-tier code (SD10): anything here is exploratory and may be wrong. Only `engine/` (Rust) and
`lean/` (Lean 4) are canonical, and only their output enters `PROGRAM/CLAIMS.md` as MEASURED,
REPRODUCED, or PROVEN. Promotion from lab to engine is a normal, expected step.

Conventions: standard library only unless an ADR records a dependency and its licence; scripts
take paths as arguments and never import from the data they read; run with `python -I -X utf8`.
Empty at S0.6.
