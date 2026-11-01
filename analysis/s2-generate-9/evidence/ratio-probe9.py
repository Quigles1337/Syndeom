"""Read what the s2_grammar 0.2.0 FLAW-0013 golden configs achieve as `padding_ratio` at seed 7
(ADR-0011 D4, D8; Al's reading on packet 4's reading 30): the two padded configs are produced
once with `padding_ratio` 1 for the informal and the formal targets — FAIL-verdict probe
artifacts, kept under the external root as the record and never goldens — and the achieved
ratios are printed, to be commanded to two decimals by goldens-run9.py. One Lean process at a
time. Usage: python -I -X utf8 ratio-probe9.py <engine commit> <new base directory>"""
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
SHORT = {"n_steps": 4, "n_lemmas": 0}
PADDING = [{"class": "FLAW-0013", "sub_variant": None, "severity": "cosmetic", "site": "auto",
            "dials": {"kinds": ["re-derivation", "unused-lemma"], "count": 2, "placement": "drawn"}}]


def now():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


os.makedirs(BASE, exist_ok=False)
ratios = {}
with open(f"{BASE}/summary.txt", "w", encoding="utf-8", newline="\n") as summary:
    for target in ["informal", "formal"]:
        d = f"{BASE}/ratio-probe-{target}"
        os.makedirs(d, exist_ok=False)
        config = {"grammar": "ring_chain", "length": SHORT, "target": target, "flaws": PADDING, "padding_ratio": 1}
        with open(f"{d}/config.json", "w", encoding="utf-8", newline="\n") as f:
            f.write(json.dumps(config) + "\n")
        started = now()
        r = subprocess.run(
            [BIN, "generate", "--config", f"{d}/config.json", "--seed", SEED, "--root", ROOT,
             "--engine-commit", COMMIT, "--receipt-dir", f"{d}/receipt", "--dry-run"],
            capture_output=True, text=True, encoding="utf-8",
        )
        summary.write(f"ratio-probe-{target} exit={r.returncode} start={started} end={now()}\n{r.stdout}{r.stderr}\n")
        summary.flush()
        manifest = json.load(open(f"{d}/receipt/outputs/manifest.json", encoding="utf-8"))
        achieved = manifest["achieved"]["padding_ratio"]["value"]
        tokens = manifest["achieved"]["n_tokens"]["value"]
        ratios[target] = {"achieved": achieved, "n_tokens": tokens, "verdict": manifest["fidelity"]["verdict"],
                          "two_decimals": round(achieved, 2)}
    summary.write("=== RATIO PROBE end\n")
with open(f"{BASE}/ratios.json", "w", encoding="utf-8", newline="\n") as f:
    f.write(json.dumps(ratios, indent=2) + "\n")
print(json.dumps(ratios, indent=2))
