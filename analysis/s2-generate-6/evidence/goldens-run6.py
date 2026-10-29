"""Produce the 0.6.0 golden cases with `syndeom-generate generate --dry-run`, one at a time
(ADR-0011 D8: one Lean process at a time): FLAW-0010 `a` `journal` on sqrt2_irrational (informal,
pair) and, on the new seed bounded_monotone_converges, the control (informal, formal), FLAW-0003
`direct` `restated-goal` (informal, formal, pair), FLAW-0003 `direct` `contrapositive` (informal,
formal, pair) and FLAW-0003 `cycle` (informal, pair). Usage:
python -I -X utf8 goldens-run6.py <engine commit> <new base directory>
Run the production twice into two base directories to confirm byte-identical outputs."""
import json
import os
import subprocess
import sys
from datetime import datetime, timezone

COMMIT = sys.argv[1]
BASE = sys.argv[2]
BIN = "C:/Users/LEET/Syndeom/engine/target/debug/syndeom-generate.exe"
ROOT = "C:/Users/LEET/Syndeom"
SEED = "7"

CITE = [{"class": "FLAW-0010", "sub_variant": "a", "severity": "gap", "site": "auto",
         "dials": {"shape": "journal", "claim": True}}]
RESTATED = [{"class": "FLAW-0003", "sub_variant": "direct", "severity": "fatal", "site": "auto",
             "dials": {"form": "restated-goal"}}]
CONTRAPOSITIVE = [{"class": "FLAW-0003", "sub_variant": "direct", "severity": "fatal", "site": "auto",
                   "dials": {"form": "contrapositive"}}]
CYCLE = [{"class": "FLAW-0003", "sub_variant": "cycle", "severity": "fatal", "site": "auto",
          "dials": {"cycle_length": 2}}]

CASES = [
    ("sqrt2_irrational", "flaw-0010-journal", "descent", CITE, ["informal", "pair"]),
    ("bounded_monotone_converges", "control", "supremum", [], ["informal", "formal"]),
    ("bounded_monotone_converges", "flaw-0003-direct-restated-goal", "supremum", RESTATED, ["informal", "formal", "pair"]),
    ("bounded_monotone_converges", "flaw-0003-direct-contrapositive", "supremum", CONTRAPOSITIVE, ["informal", "formal", "pair"]),
    ("bounded_monotone_converges", "flaw-0003-cycle", "supremum", CYCLE, ["informal", "pair"]),
]


def now():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


os.makedirs(BASE, exist_ok=False)
with open(f"{BASE}/summary.txt", "w", encoding="utf-8", newline="\n") as summary:
    for theorem, name, style, flaws, targets in CASES:
        for target in targets:
            case_id = f"{theorem}/{name}-{target}"
            d = f"{BASE}/{theorem}/{name}-{target}"
            os.makedirs(d, exist_ok=True)
            config = {"theorem": theorem, "style": style, "target": target, "flaws": flaws, "padding_ratio": 0}
            with open(f"{d}/config.json", "w", encoding="utf-8", newline="\n") as f:
                f.write(json.dumps(config) + "\n")
            started = now()
            r = subprocess.run(
                [BIN, "generate", "--config", f"{d}/config.json", "--seed", SEED, "--root", ROOT,
                 "--engine-commit", COMMIT, "--receipt-dir", f"{d}/receipt", "--dry-run"],
                capture_output=True, text=True, encoding="utf-8",
            )
            summary.write(f"{case_id} exit={r.returncode} start={started} end={now()}\n{r.stdout}{r.stderr}\n")
            summary.flush()
    summary.write("=== GOLDENS end\n")
print("done")
