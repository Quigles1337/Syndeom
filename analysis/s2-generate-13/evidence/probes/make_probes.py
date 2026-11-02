"""FLAW-0004 probes on ring_chain (STATE's NEXT, step 4): the (4, 0) seed-7 formal control (the
0.1.0 golden control-short-formal, whose bytes every later version emits) with one calc step's
tactic altered. Each probe is one file; they are compiled one Lean process at a time."""
import os
BASE = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens/ring_chain/control-short-formal/Proof.lean"
OUT = "C:/Users/LEET/Codex/Temp/syndeom-s2-grammar-060-20261102T1840Z/probes"
lines = open(BASE, "rb").read().decode("utf-8").split("\n")
assert lines[3].startswith("  calc ") and lines[4].startswith("    _ = ") and lines[6].startswith("    _ = "), lines
def strip_by_ring(l):
    assert l.endswith(" := by ring"), l
    return l[: -len(" ring")]
probes = {
    # (i) the tactic omitted after `by`, the line ending with `by`, the next step following
    "p1-middle-by-omitted": (4, strip_by_ring(lines[4])),
    "p2-first-by-omitted": (3, strip_by_ring(lines[3])),
    "p3-last-by-omitted": (6, strip_by_ring(lines[6])),
    # (ii) a tactic block that leaves the goal open without naming the omission (a visible
    # placeholder tactic; measured so that it can be refused on the record)
    "p4-middle-by-skip": (4, strip_by_ring(lines[4]) + " skip"),
    # (iii) the justification replaced by a term-level hole
    "p5-middle-term-hole": (4, lines[4][: -len(" := by ring")] + " := _"),
    "p6-middle-synthetic-hole": (4, lines[4][: -len(" := by ring")] + " := ?_"),
}
for name, (idx, text) in probes.items():
    out = list(lines)
    out[idx] = text
    data = "\n".join(out)
    assert data.endswith("\n") and "\r" not in data
    with open(f"{OUT}/{name}.lean", "wb") as f:
        f.write(data.encode("utf-8"))
    print(name, "line", idx + 1, repr(text))
