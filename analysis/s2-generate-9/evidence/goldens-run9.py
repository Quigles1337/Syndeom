"""Produce the s2_grammar 0.2.0 golden cases with `syndeom-generate generate --dry-run`, one at a
time (ADR-0011 D8: one Lean process at a time): on the grammar ring_chain at the short length
(n_steps 4, n_lemmas 0), the control (informal, formal), FLAW-0007 `unused` with two lemmas
(informal, formal, pair) and FLAW-0013 with two drawn paragraphs of the kinds re-derivation and
unused-lemma (informal, formal, pair), whose commanded `padding_ratio` is the ratio the count
achieves at seed 7 to two decimals (ratio-probe9.py; informal and pair over the informal file,
formal over the formal file). Usage:
python -I -X utf8 goldens-run9.py <engine commit> <new base directory> <ratio informal> <ratio formal>
Run the production twice into two base directories to confirm byte-identical outputs."""
import json
import os
import subprocess
import sys
from datetime import datetime, timezone

COMMIT = sys.argv[1]
BASE = sys.argv[2]
RATIO = {"informal": float(sys.argv[3]), "pair": float(sys.argv[3]), "formal": float(sys.argv[4])}
BIN = "C:/Users/LEET/Syndeom/engine/target/debug/syndeom-generate.exe"
ROOT = "C:/Users/LEET/Syndeom"
SEED = "7"
SHORT = {"n_steps": 4, "n_lemmas": 0}
UNUSED = [{"class": "FLAW-0007", "sub_variant": "unused", "severity": "cosmetic", "site": "auto",
           "dials": {"count": 2, "length_steps": 1}}]
PADDING = [{"class": "FLAW-0013", "sub_variant": None, "severity": "cosmetic", "site": "auto",
            "dials": {"kinds": ["re-derivation", "unused-lemma"], "count": 2, "placement": "drawn"}}]

CASES = [
    ("ring_chain", "v0.2.0-control-short", SHORT, [], ["informal", "formal"], False),
    ("ring_chain", "v0.2.0-flaw-0007-unused", SHORT, UNUSED, ["informal", "formal", "pair"], False),
    ("ring_chain", "v0.2.0-flaw-0013-drawn", SHORT, PADDING, ["informal", "formal", "pair"], True),
]


def now():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%f")[:-3] + "Z"


os.makedirs(BASE, exist_ok=False)
with open(f"{BASE}/summary.txt", "w", encoding="utf-8", newline="\n") as summary:
    for grammar, name, length, flaws, targets, padded in CASES:
        for target in targets:
            case_id = f"{grammar}/{name}-{target}"
            d = f"{BASE}/{grammar}/{name}-{target}"
            os.makedirs(d, exist_ok=True)
            ratio = RATIO[target] if padded else 0
            config = {"grammar": grammar, "length": length, "target": target, "flaws": flaws, "padding_ratio": ratio}
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
