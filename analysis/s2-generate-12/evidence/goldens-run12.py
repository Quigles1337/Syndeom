"""Produce the s2_grammar 0.5.0 golden cases with `syndeom-generate generate --dry-run`, one at a
time (ADR-0011 D8: one Lean process at a time): on the grammar ring_chain at the short length
with one lemma (n_steps 4, n_lemmas 1), the control (informal, formal — so that the flawed cases'
truth reference names a golden) and FLAW-0003 `cycle` with `cycle_length` 2 (fatal; informal,
pair — the formal target being refused). Usage:
python -I -X utf8 goldens-run12.py <engine commit> <new base directory>
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
LENGTH = {"n_steps": 4, "n_lemmas": 1}


def cycle():
    return [{"class": "FLAW-0003", "sub_variant": "cycle", "severity": "fatal", "site": "auto",
             "dials": {"cycle_length": 2}}]


CASES = [
    ("ring_chain", "v0.5.0-control-lemma", [], ["informal", "formal"]),
    ("ring_chain", "v0.5.0-flaw-0003-cycle", cycle(), ["informal", "pair"]),
]


def now():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


os.makedirs(BASE, exist_ok=False)
with open(f"{BASE}/summary.txt", "w", encoding="utf-8", newline="\n") as summary:
    for grammar, name, flaws, targets in CASES:
        for target in targets:
            case_id = f"{grammar}/{name}-{target}"
            d = f"{BASE}/{grammar}/{name}-{target}"
            os.makedirs(d, exist_ok=True)
            config = {"grammar": grammar, "length": LENGTH, "target": target, "flaws": flaws, "padding_ratio": 0}
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
