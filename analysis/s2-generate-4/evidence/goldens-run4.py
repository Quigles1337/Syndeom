"""Produce the 0.4.0 golden cases with `syndeom-generate generate --dry-run`, one at a time
(ADR-0011 D8: one Lean process at a time). Usage:
python -I -X utf8 goldens-run4.py <engine commit> <new base directory> <ratios.json | probe>
With `probe`, the padded cases command padding_ratio 1 for the informal and formal targets only,
so that their achieved ratios can be read from the outputs (the ratio is measured, D4); with a
ratios file ({case: {target: ratio}}), every case is produced with the ratio it names. Run the
production twice into two base directories to confirm byte-identical outputs."""
import json
import os
import subprocess
import sys
from datetime import datetime, timezone

COMMIT = sys.argv[1]
BASE = sys.argv[2]
RATIOS_ARG = sys.argv[3]
BIN = "C:/Users/LEET/Syndeom/engine/target/debug/syndeom-generate.exe"
ROOT = "C:/Users/LEET/Syndeom"
SEED = "7"
PROBE = RATIOS_ARG == "probe"
RATIOS = {} if PROBE else json.load(open(RATIOS_ARG, encoding="utf-8"))


def padding(kinds, count, placement):
    return [{"class": "FLAW-0013", "sub_variant": None, "severity": "cosmetic", "site": "auto",
             "dials": {"kinds": kinds, "count": count, "placement": placement}}]


CASES = [
    ("sqrt2_irrational", "control", "descent", [], ["informal", "formal"]),
    ("sqrt2_irrational", "flaw-0013-drawn-1", "descent",
     padding(["re-derivation"], 1, "drawn"), ["informal", "formal", "pair"]),
    ("sqrt2_irrational", "flaw-0013-before-7", "descent",
     padding(["restated-definition", "re-derivation"], 7, "before"), ["informal", "formal", "pair"]),
]


def now():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


os.makedirs(BASE, exist_ok=False)
with open(f"{BASE}/summary.txt", "w", encoding="utf-8", newline="\n") as summary:
    for theorem, name, style, flaws, targets in CASES:
        for target in targets:
            if PROBE and (not flaws or target == "pair"):
                continue
            if flaws:
                ratio = 1 if PROBE else RATIOS[name][target]
            else:
                ratio = 0
            case_id = f"{theorem}/{name}-{target}"
            d = f"{BASE}/{theorem}/{name}-{target}"
            os.makedirs(d, exist_ok=True)
            config = {"theorem": theorem, "style": style, "target": target, "flaws": flaws, "padding_ratio": ratio}
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
            if PROBE and r.returncode == 0:
                m = json.load(open(f"{d}/receipt/outputs/manifest.json", encoding="utf-8"))
                summary.write(f"{case_id} achieved padding_ratio = {m['achieved']['padding_ratio']['value']!r} "
                              f"n_tokens = {m['achieved']['n_tokens']['value']}\n")
                summary.flush()
    summary.write("=== GOLDENS end\n")
print("done")
