#!/usr/bin/env bash
# Exercise scripts/check.sh's axiom loop on crafted audit outputs. The block under test is
# extracted from check.sh itself (from `EXPECTED=` through the closing `fi`).
set -uo pipefail
here="$(mktemp -d)"
mkdir -p "$here/lean"
sed -n '/^EXPECTED=/,/^fi$/p' /c/Users/LEET/Syndeom/scripts/check.sh > "$here/block.sh"
printf '#print axioms a\n#print axioms b\n' > "$here/lean/Audit.lean"
run() {
  ( cd "$here" && set -euo pipefail && AUDIT="$1" && source ./block.sh ) > "$here/out" 2>&1
  echo "$2 -> exit $? : $(tail -1 "$here/out")"
}
run "$(printf "'a' depends on axioms: [propext, Classical.choice, Quot.sound]\n'b' does not depend on any axioms")" "standard (expect 0)"
run "$(printf "'a' depends on axioms: [Quot.sound, propext]\n'b' depends on axioms: [Classical.choice]")" "reordered subsets (expect 0)"
run "$(printf "'a' depends on axioms: [propext, sorryAx]\n'b' depends on axioms: [propext]")" "sorryAx (expect 1)"
run "$(printf "'a' depends on axioms: [propext, Lean.ofReduceBool]\n'b' depends on axioms: [propext]")" "ofReduceBool (expect 1)"
run "$(printf "'a' depends on axioms: [propext]\nerror: unknown identifier 'b'")" "one line missing (expect 1)"
run "" "empty output (expect 1)"
rm -rf "$here"
