"""Security-review probe (packet 14, 2F): FLAW-0011's class pre-filter parses the number after
`FLAW-` with u32's FromStr, which accepts `+1` and `001`; the exact-string `realizes` and
`check_spec` must still refuse such classes. Runs `generate --dry-run` on each config and expects
exit 2 (refused), with a control as the positive case. Usage:
python -I -X utf8 probe-class-forms.py <engine commit> <new base directory>"""
import json
import os
import subprocess
import sys

COMMIT, BASE = sys.argv[1], sys.argv[2]
BIN = "C:/Users/LEET/Syndeom/engine/target/debug/syndeom-generate.exe"
ROOT = "C:/Users/LEET/Syndeom"


def config(flaws):
    return {"grammar": "ring_chain", "length": {"n_steps": 4, "n_lemmas": 0}, "target": "informal",
            "flaws": flaws, "padding_ratio": 0}


def composite(cls):
    inner = {"class": cls, "sub_variant": "propagated", "severity": "fatal", "site": "auto",
             "dials": {"perturbation": "sign"}}
    return [{"class": "FLAW-0011", "sub_variant": None, "severity": "fatal", "site": "auto",
             "dials": {"embedded": [inner]}}]


PROBES = [("control", config([]), 0)] + [
    (f"embedded-{cls}", config(composite(cls)), 2)
    for cls in ["FLAW-+1", "FLAW-1", "FLAW-001", "FLAW-00001", "flaw-0001", "FLAW-0001 "]
] + [("embedded-FLAW-0001", config(composite("FLAW-0001")), 0)]

os.makedirs(BASE, exist_ok=False)
ok = True
for i, (name, cfg, expect) in enumerate(PROBES):
    d = f"{BASE}/{i:02d}"  # by index: NTFS names are case-insensitive
    os.makedirs(d)
    with open(f"{d}/config.json", "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(cfg) + "\n")
    r = subprocess.run([BIN, "generate", "--config", f"{d}/config.json", "--seed", "7", "--root", ROOT,
                        "--engine-commit", COMMIT, "--receipt-dir", f"{d}/receipt", "--dry-run"],
                       capture_output=True, text=True, encoding="utf-8")
    good = r.returncode == expect
    ok &= good
    print(f"{name!r}: exit {r.returncode} (expected {expect}) {'OK' if good else 'UNEXPECTED'}")
    print("   ", (r.stderr.strip() or r.stdout.strip()).splitlines()[0][:200])
print("ALL AS EXPECTED" if ok else "SOME UNEXPECTED")
sys.exit(0 if ok else 1)
