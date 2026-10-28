"""Copy the dry-run outputs into engine/generate/tests/goldens/ with a case.json each, and compare
the formal outputs with the hand-authored Lean probes compiled before the seeds were written.
Usage: python -I -X utf8 goldens-copy.py"""
import hashlib
import json
import os
import re
import shutil

BASE = "C:/Users/LEET/Codex/Temp/syndeom-s2-template-020-20261028T1830Z"
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
PROBES = {
    "continuity_is_not_uniform/control-formal": ("cinu_faithful", "strip-axiom-tail"),
    "continuity_is_not_uniform/flaw-0002-gap-formal": ("cinu_flawed", "exact"),
    "continuity_is_not_uniform/flaw-0002-gap-pair": ("cinu_flawed", "exact"),
    "x_over_abs_x/flaw-0014-fatal-formal": ("xoa_convincing", "exact"),
    "x_over_abs_x/flaw-0014-fatal-pair": ("xoa_convincing", "exact"),
    "rolle_abs/flaw-0005-violated-formal": ("rolle_convincing", "exact"),
    "rolle_abs/flaw-0005-violated-pair": ("rolle_convincing", "exact"),
    "horses/flaw-0006-fatal-formal": ("horses_convincing", "exact"),
    "horses/flaw-0006-fatal-pair": ("horses_convincing", "exact"),
}


def sha(b):
    return hashlib.sha256(b).hexdigest()


report = []
for theorem in sorted(d for d in os.listdir(f"{BASE}/goldens") if "." not in d):
    for name in sorted(os.listdir(f"{BASE}/goldens/{theorem}")):
        src = f"{BASE}/goldens/{theorem}/{name}"
        with open(f"{src}/receipt/receipt.json", encoding="utf-8") as f:
            receipt = json.load(f)
        out = f"{DEST}/{theorem}/{name}"
        os.makedirs(out, exist_ok=False)
        files = sorted(os.listdir(f"{src}/receipt/outputs"))
        for fn in files:
            shutil.copyfile(f"{src}/receipt/outputs/{fn}", f"{out}/{fn}")
        with open(f"{src}/config.json", encoding="utf-8") as f:
            config = json.load(f)
        case = {
            "config": config,
            "seed": 7,
            "engine_commit": receipt["engine_commit"],
            "toolchain": receipt["toolchain"],
            "observation": receipt["observation"],
        }
        with open(f"{out}/case.json", "w", encoding="utf-8", newline="\n") as f:
            f.write(json.dumps(case, indent=2, ensure_ascii=False) + "\n")
        row = {"case": f"{theorem}/{name}", "files": files, "verdict": receipt["verdict"],
               "observation": receipt["observation"], "artifact_id": receipt["artifact_ids"][0]}
        probe = PROBES.get(f"{theorem}/{name}")
        if probe:
            pname, mode = probe
            with open(f"{BASE}/lean-probe/{pname}.lean", "rb") as f:
                p = f.read()
            if mode == "strip-axiom-tail":
                # the faithful probe ends with "\n#print axioms <main>\n"; the golden does not
                p2 = re.sub(rb"\n#print axioms [^\n]+\n$", b"", p)
                row["comparison"] = "probe minus its trailing #print axioms line"
                p = p2
            else:
                row["comparison"] = "byte-exact"
            with open(f"{out}/Proof.lean", "rb") as f:
                g = f.read()
            row["hand_authored_probe"] = pname
            row["equals_probe"] = p == g
            row["proof_sha256"] = sha(g)
        report.append(row)
with open(f"{BASE}/goldens-copy-report.json", "w", encoding="utf-8", newline="\n") as f:
    f.write(json.dumps(report, indent=1, ensure_ascii=False) + "\n")
print(json.dumps(report, indent=1, ensure_ascii=False))
