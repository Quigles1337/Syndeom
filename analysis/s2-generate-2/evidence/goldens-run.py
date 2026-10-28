"""Produce the 0.2.0 golden cases with `syndeom-generate generate --dry-run`, one at a time
(ADR-0011 D8: one Lean process at a time). Usage: python -I -X utf8 goldens-run.py <engine commit>."""
import json
import os
import subprocess
import sys
from datetime import datetime, timezone

BASE = "C:/Users/LEET/Codex/Temp/syndeom-s2-template-020-20261028T1830Z/goldens"
BIN = "C:/Users/LEET/Syndeom/engine/target/debug/syndeom-generate.exe"
ROOT = "C:/Users/LEET/Syndeom"
COMMIT = sys.argv[1]
SEED = "7"


def spec(cls, sub, sev, dials):
    return [{"class": cls, "sub_variant": sub, "severity": sev, "site": "auto", "dials": dials}]


CASES = [
    ("continuity_is_not_uniform", "control", "epsilon_delta", [], ["informal", "formal"]),
    ("continuity_is_not_uniform", "flaw-0002-gap", "epsilon_delta",
     spec("FLAW-0002", None, "gap", {"swap": "forall-exists"}), ["informal", "formal", "pair"]),
    ("x_over_abs_x", "flaw-0014-fatal", "two_cases",
     spec("FLAW-0014", None, "fatal", {"omitted_case": "x = 0", "holds": False}), ["informal", "formal", "pair"]),
    ("rolle_abs", "flaw-0005-violated", "rolle",
     spec("FLAW-0005", None, "fatal", {"theorem": "Rolle's theorem", "hypothesis": "f is differentiable on (-1, 1)", "mode": "violated"}),
     ["informal", "formal", "pair"]),
    ("horses", "flaw-0006-fatal", "induction",
     spec("FLAW-0006", None, "fatal", {"mode": "shift-index", "failing_case": "n + 1 = 2"}), ["informal", "formal", "pair"]),
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
