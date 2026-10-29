"""Produce the 0.7.0 golden cases with `syndeom-generate generate --dry-run`, one at a time
(ADR-0011 D8: one Lean process at a time): on the new seed bolzano_weierstrass, the control
(informal, formal) and FLAW-0008 for each of D5's five transformations (pair only: the class
manifests in the pair); on the new seed bounded_continuous_sup, FLAW-0012 (informal, formal,
pair). Usage:
python -I -X utf8 goldens-run7.py <engine commit> <new base directory>
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


def drift(transformation, severity):
    return [{"class": "FLAW-0008", "sub_variant": None, "severity": severity, "site": "auto",
             "dials": {"transformation": transformation}}]


DROPPED = [{"class": "FLAW-0012", "sub_variant": None, "severity": "fatal", "site": "auto",
            "dials": {"dropped_hypothesis": "the domain of $f$ is a compact interval"}}]

CASES = [
    ("bolzano_weierstrass", "control", "compactness", [], ["informal", "formal"]),
    ("bolzano_weierstrass", "flaw-0008-add-hypothesis", "compactness", drift("add-hypothesis", "gap"), ["pair"]),
    ("bolzano_weierstrass", "flaw-0008-restrict-type", "compactness", drift("restrict-type", "gap"), ["pair"]),
    ("bolzano_weierstrass", "flaw-0008-flip-strictness", "compactness", drift("flip-strictness", "gap"), ["pair"]),
    ("bolzano_weierstrass", "flaw-0008-weaken-conclusion", "compactness", drift("weaken-conclusion", "fatal"), ["pair"]),
    ("bolzano_weierstrass", "flaw-0008-shadow-definition", "compactness", drift("shadow-definition", "fatal"), ["pair"]),
    ("bounded_continuous_sup", "flaw-0012-fatal", "extreme_value", DROPPED, ["informal", "formal", "pair"]),
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
