"""Regenerate the 0.5.0 golden cases at the fix commit (Al's ruling 2 on PR 30) with
`syndeom-generate generate --dry-run`, one at a time (ADR-0011 D8: one Lean process at a time):
FLAW-0004 routine-true on sqrt2_irrational (informal, pair) and FLAW-0011 embedding the
FLAW-0002 forall-exists gap instance of continuity_is_not_uniform (informal, pair). Usage:
python -I -X utf8 goldens-run5r.py <engine commit> <new base directory>
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

HEDGE = [{"class": "FLAW-0004", "sub_variant": None, "severity": "cosmetic", "site": "auto",
          "dials": {"hidden_step": "routine-true"}}]
SWAP = [{"class": "FLAW-0002", "sub_variant": None, "severity": "gap", "site": "auto",
         "dials": {"swap": "forall-exists"}}]
COMPOSITE = [{"class": "FLAW-0011", "sub_variant": None, "severity": "gap", "site": "auto",
              "dials": {"embedded": SWAP}}]

CASES = [
    ("sqrt2_irrational", "flaw-0004-routine-true", "descent", HEDGE, ["informal", "pair"]),
    ("continuity_is_not_uniform", "flaw-0011-flaw-0002", "epsilon_delta", COMPOSITE, ["informal", "pair"]),
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
