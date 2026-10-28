#!/usr/bin/env bash
# Compile each probe with `lake env lean`, one process at a time, from lean/ (ADR-0011 D8).
dir=/c/Users/LEET/Codex/Temp/syndeom-s2-template-040-20261028T2136Z/lean-probe
cd /c/Users/LEET/Syndeom/lean || exit 1
for f in "$@"; do
  start=$(date -u +%H:%M:%S)
  lake env lean "C:/Users/LEET/Codex/Temp/syndeom-s2-template-040-20261028T2136Z/lean-probe/$f.lean" > "$dir/$f.out" 2>&1
  rc=$?
  echo "$f exit=$rc start=$start end=$(date -u +%H:%M:%S)" >> "$dir/summary.txt"
done
echo "=== PROBE end" >> "$dir/summary.txt"
