"""Confirm two regeneration runs are byte-identical, then replace the 0.2.0 golden cases under
engine/generate/tests/goldens/ with run A's outputs and a case.json each (Al's rulings on PR 27:
the 0.2.0 goldens share one engine_commit). Usage:
python -I -X utf8 goldens-copy2.py <run A base> <run B base> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
SEEDS_0_2_0 = ["continuity_is_not_uniform", "x_over_abs_x", "rolle_abs", "horses"]


def sha(b):
    return hashlib.sha256(b).hexdigest()


def cases(base):
    out = []
    for theorem in sorted(d for d in os.listdir(base) if "." not in d):
        for name in sorted(os.listdir(f"{base}/{theorem}")):
            out.append((theorem, name))
    return out


assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "cases": [], "all_byte_identical": True}
for theorem, name in cases(A):
    outs_a = sorted(os.listdir(f"{A}/{theorem}/{name}/receipt/outputs"))
    outs_b = sorted(os.listdir(f"{B}/{theorem}/{name}/receipt/outputs"))
    assert outs_a == outs_b, (theorem, name)
    row = {"case": f"{theorem}/{name}", "files": {}}
    for fn in outs_a:
        a = open(f"{A}/{theorem}/{name}/receipt/outputs/{fn}", "rb").read()
        b = open(f"{B}/{theorem}/{name}/receipt/outputs/{fn}", "rb").read()
        row["files"][fn] = {"sha256": sha(a), "bytes": len(a), "identical": a == b}
        if a != b:
            report["all_byte_identical"] = False
    ra = json.load(open(f"{A}/{theorem}/{name}/receipt/receipt.json", encoding="utf-8"))
    rb = json.load(open(f"{B}/{theorem}/{name}/receipt/receipt.json", encoding="utf-8"))
    row["engine_commit"] = ra["engine_commit"]
    row["same_engine_commit"] = ra["engine_commit"] == rb["engine_commit"]
    row["same_observation"] = ra["observation"] == rb["observation"]
    row["verdict"] = ra["verdict"]
    row["artifact_id"] = ra["artifact_ids"][0]
    report["cases"].append(row)
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])

# Replace the 0.2.0 cases (Al's ruling: regenerate so they share one engine_commit).
for theorem in SEEDS_0_2_0:
    if os.path.isdir(f"{DEST}/{theorem}"):
        shutil.rmtree(f"{DEST}/{theorem}")
for theorem, name in cases(A):
    assert theorem in SEEDS_0_2_0
    src = f"{A}/{theorem}/{name}"
    out = f"{DEST}/{theorem}/{name}"
    os.makedirs(out, exist_ok=False)
    for fn in sorted(os.listdir(f"{src}/receipt/outputs")):
        shutil.copyfile(f"{src}/receipt/outputs/{fn}", f"{out}/{fn}")
    receipt = json.load(open(f"{src}/receipt/receipt.json", encoding="utf-8"))
    config = json.load(open(f"{src}/config.json", encoding="utf-8"))
    case = {"config": config, "seed": 7, "engine_commit": receipt["engine_commit"],
            "toolchain": receipt["toolchain"], "observation": receipt["observation"]}
    with open(f"{out}/case.json", "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(case, indent=2, ensure_ascii=False) + "\n")
with open(REPORT, "w", encoding="utf-8", newline="\n") as f:
    f.write(json.dumps(report, indent=1, ensure_ascii=False) + "\n")
print("byte-identical runs:", report["all_byte_identical"], "engine_commit:", report["engine_commits"])
for r in report["cases"]:
    print(r["case"], r["verdict"], r["artifact_id"])
