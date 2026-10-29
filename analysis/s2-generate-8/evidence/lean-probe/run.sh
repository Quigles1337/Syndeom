#!/usr/bin/env bash
# Compile every probe one Lean process at a time from lean/ (ADR-0011 D8; the machine hazard:
# each process loads Mathlib's oleans). Output and exit code per probe land beside it.
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../.." && pwd)"
cd "$ROOT/lean" || exit 99
for f in "$HERE"/*.lean; do
  name="$(basename "$f" .lean)"
  echo "=== $name start $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  lake env lean "$f" > "$HERE/$name.out" 2>&1
  rc=$?
  echo "=== $name end $(date -u +%Y-%m-%dT%H:%M:%SZ) exit=$rc"
done
echo "=== PROBES end"
