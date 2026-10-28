#!/usr/bin/env bash
# SYNDEOM gate (SD9; ADR-0002 D6). The default branch always passes this script.
#   lake build (lean/)  +  cargo fmt --check  +  cargo clippy -D warnings  +  cargo test (engine/)
#   + axiom audit of the golden fixture and the seed witnesses  +  LF/BOM check
#   + register cross-reference check  +  freeze register
# Run from anywhere: paths are resolved from this file's location.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== lean: lake build (toolchain $(cat lean/lean-toolchain)) =="
( cd lean && lake build )

echo "== lean: axiom audit of the golden fixture and the seed witnesses =="
AUDIT="$(cd lean && lake env lean Audit.lean 2>&1)"
echo "$AUDIT"
# SD2(ii): every audited declaration depends on at most the standard three axioms (ADR-0011 D10).
# Audit.lean prints one line per `#print axioms`; each must name only propext, Classical.choice
# and Quot.sound, and the number of axiom lines must equal the number of audited declarations.
EXPECTED="$(grep -c '^#print axioms ' lean/Audit.lean)"
SEEN=0
while IFS= read -r line; do
  case "$line" in
    "'"*"' does not depend on any axioms") SEEN=$((SEEN + 1)) ;;
    "'"*"' depends on axioms: ["*"]")
      list="${line#*depends on axioms: [}"
      IFS=',' read -ra axioms <<< "${list%]}"
      for ax in "${axioms[@]}"; do
        case "${ax# }" in
          propext|Classical.choice|Quot.sound) ;;
          *) echo "FAIL: non-standard axiom '${ax# }' in: $line"; exit 1 ;;
        esac
      done
      SEEN=$((SEEN + 1)) ;;
  esac
done <<< "$AUDIT"
if [ "$EXPECTED" -lt 1 ] || [ "$SEEN" -ne "$EXPECTED" ]; then
  echo "FAIL: $SEEN axiom lines for $EXPECTED audited declarations"; exit 1
fi

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
