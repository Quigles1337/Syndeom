"""Confirm two regeneration runs of the 0.5.0 goldens (Al's ruling 2 on PR 30) are
byte-identical; confirm the FLAW-0004 proof bytes equal the goldens committed at 4edc3f73
(only manifest.json and case.json may change, by the engine commit); confirm the composite's
proof bytes equal the 0.2.0 FLAW-0002 gap goldens of continuity_is_not_uniform (the composite
changes no byte of the embedded instance's files) and the pair's Proof.lean fails at line 6;
then replace the 0.5.0 goldens under engine/generate/tests/goldens/ with run A's outputs and a
case.json each. Usage:
python -I -X utf8 goldens-copy5r.py <run A base> <run B base> <old goldens dir (git worktree or
export of 4edc3f73)> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, OLD, REPORT = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
OLD_COMPOSITE = ["sqrt2_irrational/flaw-0011-flaw-0004-informal", "sqrt2_irrational/flaw-0011-flaw-0004-pair"]
PROOF_FILES = ("proof.md", "Proof.lean")


def sha(b):
    return hashlib.sha256(b).hexdigest()


def rd(p):
    return open(p, "rb").read()


def cases(base):
    out = []
    for theorem in sorted(d for d in os.listdir(base) if "." not in d):
        for name in sorted(os.listdir(f"{base}/{theorem}")):
            out.append((theorem, name))
    return out


assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "old_goldens": OLD, "cases": [], "all_byte_identical": True,
          "flaw_0004_proof_bytes_unchanged": {}, "composite_proof_bytes_equal_0_2_0_golden": {}}
for theorem, name in cases(A):
    outs_a = sorted(os.listdir(f"{A}/{theorem}/{name}/receipt/outputs"))
    outs_b = sorted(os.listdir(f"{B}/{theorem}/{name}/receipt/outputs"))
    assert outs_a == outs_b, (theorem, name)
    row = {"case": f"{theorem}/{name}", "files": {}}
    for fn in outs_a:
        a = rd(f"{A}/{theorem}/{name}/receipt/outputs/{fn}")
        b = rd(f"{B}/{theorem}/{name}/receipt/outputs/{fn}")
        row["files"][fn] = {"sha256": sha(a), "bytes": len(a), "identical": a == b}
        if a != b:
            report["all_byte_identical"] = False
    ra = json.load(open(f"{A}/{theorem}/{name}/receipt/receipt.json", encoding="utf-8"))
    rb = json.load(open(f"{B}/{theorem}/{name}/receipt/receipt.json", encoding="utf-8"))
    row["engine_commit"] = ra["engine_commit"]
    row["same_engine_commit"] = ra["engine_commit"] == rb["engine_commit"]
    row["same_observation"] = ra["observation"] == rb["observation"]
    row["observation"] = ra["observation"]
    row["verdict"] = ra["verdict"]
    row["artifact_id"] = ra["artifact_ids"][0]
    manifest = json.load(open(f"{A}/{theorem}/{name}/receipt/outputs/manifest.json", encoding="utf-8"))
    gt = manifest["ground_truth"]
    row["proof_status"] = gt["proof_status"]
    row["correct"] = gt["correct"]
    row["records"] = [{"instance": r["instance"], "class": r["class"], "severity": r["severity"],
                       "lean_catchable": r["lean_catchable"], "location": r["location"]} for r in gt["flaws"]]
    row["n_flaws_total"] = {"commanded": manifest["commanded"]["n_flaws_total"]["value"],
                            "achieved": manifest["achieved"]["n_flaws_total"]["value"]}
    row["flaws_confirmed"] = manifest["achieved"]["flaws_confirmed"]["value"]
    if theorem == "sqrt2_irrational":
        # the FLAW-0004 proof bytes must not change (ruling 2): compare with the committed goldens
        same = all(rd(f"{A}/{theorem}/{name}/receipt/outputs/{fn}") == rd(f"{OLD}/{theorem}/{name}/{fn}")
                   for fn in outs_a if fn in PROOF_FILES)
        old_manifest = json.load(open(f"{OLD}/{theorem}/{name}/manifest.json", encoding="utf-8"))
        same_ids = old_manifest["artifact_id"] == manifest["artifact_id"]
        same_gt = old_manifest["ground_truth"] == gt
        row["flaw_0004"] = {"proof_bytes_unchanged": same, "same_artifact_id": same_ids, "same_ground_truth": same_gt,
                            "old_engine_commit": old_manifest["engine_commit"]}
        report["flaw_0004_proof_bytes_unchanged"][f"{theorem}/{name}"] = same and same_ids and same_gt
    else:
        # the composite changes no byte of the embedded instance's files: equal to the 0.2.0 golden
        target = name.rsplit("-", 1)[1]
        ref = f"{DEST}/continuity_is_not_uniform/flaw-0002-gap-{target}"
        same = all(rd(f"{A}/{theorem}/{name}/receipt/outputs/{fn}") == rd(f"{ref}/{fn}")
                   for fn in outs_a if fn in PROOF_FILES)
        row["composite"] = {"proof_bytes_equal_0_2_0_golden": same, "reference": ref}
        report["composite_proof_bytes_equal_0_2_0_golden"][f"{theorem}/{name}"] = same
    report["cases"].append(row)
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert all(report["flaw_0004_proof_bytes_unchanged"].values()), report["flaw_0004_proof_bytes_unchanged"]
assert all(report["composite_proof_bytes_equal_0_2_0_golden"].values()), report["composite_proof_bytes_equal_0_2_0_golden"]
assert len(report["cases"]) == 4

# replace: the obsolete composite goldens go, the four regenerated cases land
for rel in OLD_COMPOSITE:
    if os.path.isdir(f"{DEST}/{rel}"):
        shutil.rmtree(f"{DEST}/{rel}")
        report.setdefault("removed", []).append(rel)
for theorem, name in cases(A):
    src = f"{A}/{theorem}/{name}"
    out = f"{DEST}/{theorem}/{name}"
    if os.path.isdir(out):
        shutil.rmtree(out)
        report.setdefault("replaced", []).append(f"{theorem}/{name}")
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
print("FLAW-0004 unchanged:", report["flaw_0004_proof_bytes_unchanged"])
print("composite == 0.2.0 golden bytes:", report["composite_proof_bytes_equal_0_2_0_golden"])
for r in report["cases"]:
    obs = r["observation"] or {}
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "first_error_line",
          obs.get("first_error_line"), "n_flaws", r["n_flaws_total"], "confirmed", r["flaws_confirmed"],
          r["proof_status"], "correct", r["correct"])
