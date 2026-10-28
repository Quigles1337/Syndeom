"""Produce the 0.3.0 golden cases with `syndeom-generate generate --dry-run`, one at a time
(ADR-0011 D8: one Lean process at a time). Usage:
python -I -X utf8 goldens-run3.py <engine commit> <new base directory>.
Run twice into two base directories to confirm byte-identical outputs (Al's rulings on PR 27)."""
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


def spec(cls, sub, sev, dials):
    return [{"class": cls, "sub_variant": sub, "severity": sev, "site": "auto", "dials": dials}]


CASES = [
    ("n_lt_two_pow", "control", "helper_lemma", [], ["informal", "formal"]),
    ("n_lt_two_pow", "flaw-0009-gap", "helper_lemma",
     spec("FLAW-0009", None, "gap", {"mode": "axiom", "truth": True}), ["formal", "pair"]),
    ("n_lt_two_pow", "flaw-0009-fatal", "helper_lemma",
     spec("FLAW-0009", None, "fatal", {"mode": "axiom", "truth": False}), ["formal", "pair"]),
    ("n_lt_two_pow", "flaw-0007-unused", "helper_lemma",
     spec("FLAW-0007", "unused", "cosmetic", {"count": 1, "length_steps": 1}),
     ["informal", "formal", "pair"]),
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
