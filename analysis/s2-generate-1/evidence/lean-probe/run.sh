#!/usr/bin/env bash
# Compile each probe file with `lake env lean`, one process at a time, from lean/.
dir=/c/Users/LEET/Codex/Temp/syndeom-generate-20261028T1706Z/lean-probe
cd /c/Users/LEET/Syndeom/lean || exit 1
for f in "$@"; do
  start=$(date -u +%H:%M:%S)
  lake env lean "C:/Users/LEET/Codex/Temp/syndeom-generate-20261028T1706Z/lean-probe/$f.lean" > "$dir/$f.out" 2>&1
  rc=$?
  echo "$f exit=$rc start=$start end=$(date -u +%H:%M:%S)" >> "$dir/summary.txt"
done
echo "=== PROBE end" >> "$dir/summary.txt"
