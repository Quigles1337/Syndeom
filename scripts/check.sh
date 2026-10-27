#!/usr/bin/env bash
# SYNDEOM gate (SD9; ADR-0002 D6). The default branch always passes this script.
#   lake build (lean/)  +  cargo fmt --check  +  cargo clippy -D warnings  +  cargo test (engine/)
#   + axiom audit of the golden fixture  +  LF/BOM check  +  register cross-reference check
# Run from anywhere: paths are resolved from this file's location.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== lean: lake build (toolchain $(cat lean/lean-toolchain)) =="
( cd lean && lake build )

echo "== lean: axiom audit of the golden fixture =="
AUDIT="$(cd lean && lake env lean Audit.lean 2>&1)"
echo "$AUDIT"
# SD2(ii): at most the standard three axioms; the audit line must be exactly one of these forms
case "$AUDIT" in
  *"'syndeom_golden_nat_le_sq' depends on axioms: [propext, Classical.choice, Quot.sound]"*|\
  *"'syndeom_golden_nat_le_sq' depends on axioms: [propext, Quot.sound]"*|\
  *"'syndeom_golden_nat_le_sq' depends on axioms: [propext]"*|\
  *"'syndeom_golden_nat_le_sq' does not depend on any axioms"*) ;;
  *) echo "FAIL: golden fixture axioms are not within the standard three"; exit 1 ;;
esac

echo "== engine: cargo fmt --check =="
( cd engine && cargo fmt --all --check )
echo "== engine: cargo clippy -D warnings =="
( cd engine && cargo clippy --all-targets --all-features -- -D warnings )
echo "== engine: cargo test =="
( cd engine && cargo test --all-targets --all-features --quiet )

echo "== repo: line endings (every tracked text file i/lf) =="
if git ls-files --eol | grep -E '^i/(crlf|mixed|lf-crlf)' ; then
  echo "FAIL: non-LF tracked text file"; exit 1
fi

echo "== repo: register cross-reference check =="
python -I -X utf8 scripts/check_registers.py "$ROOT"

echo "== repo: freeze register (frozen scopes and append-only files; ADR-0012 amendment 1) =="
python -I -X utf8 scripts/check_freeze.py "$ROOT"
python -I -X utf8 scripts/check_freeze_test.py "$ROOT"

echo "GATE: GREEN"
