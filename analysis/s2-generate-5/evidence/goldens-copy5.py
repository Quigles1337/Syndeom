"""Confirm two production runs of the 0.5.0 goldens are byte-identical, compare each formal
output with the hand-written Lean probe that preceded the seed material (the hedged file with
the proof of `hp` omitted, `s2i_hedged_p1.lean`, which has no tail), then place run A's outputs
under engine/generate/tests/goldens/sqrt2_irrational/<case>/ with a case.json each. Usage:
python -I -X utf8 goldens-copy5.py <run A base> <run B base> <probe dir> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, PROBES, REPORT = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
SEEDS_0_5_0 = ["sqrt2_irrational"]


def sha(b):
    return hashlib.sha256(b).hexdigest()


def cases(base):
    out = []
    for theorem in sorted(d for d in os.listdir(base) if "." not in d):
        for name in sorted(os.listdir(f"{base}/{theorem}")):
            out.append((theorem, name))
    return out


probe = open(f"{PROBES}/s2i_hedged_p1.lean", "rb").read()
assert b"#print axioms" not in probe, "the hedged probe has no tail"

assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "cases": [], "all_byte_identical": True, "probes": {},
          "probe_files": {"s2i_hedged_p1.lean": sha(probe)}}
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
    row["observation"] = ra["observation"]
    row["verdict"] = ra["verdict"]
    row["artifact_id"] = ra["artifact_ids"][0]
    manifest = json.load(open(f"{A}/{theorem}/{name}/receipt/outputs/manifest.json", encoding="utf-8"))
    gt = manifest["ground_truth"]
    row["proof_status"] = gt["proof_status"]
    row["correct"] = gt["correct"]
    row["records"] = [{"instance": r["instance"], "class": r["class"], "severity": r["severity"],
                       "lean_catchable": r["lean_catchable"], "location": r["location"]} for r in gt["flaws"]]
    hedge = [r for r in gt["flaws"] if r["class"] == "FLAW-0004" and r["location"]["artifact"] == "informal"]
    if hedge:
        row["hedge"] = {"index": hedge[0]["dials"]["hedge"], "text": hedge[0]["dials"]["hedge_text"]}
    row["n_flaws_total"] = {"commanded": manifest["commanded"]["n_flaws_total"]["value"],
                            "achieved": manifest["achieved"]["n_flaws_total"]["value"]}
    row["flaws_confirmed"] = manifest["achieved"]["flaws_confirmed"]["value"]
    if "Proof.lean" in outs_a:
        golden = open(f"{A}/{theorem}/{name}/receipt/outputs/Proof.lean", "rb").read()
        row["probe"] = {"file": "s2i_hedged_p1.lean", "golden_is_probe": golden == probe}
        report["probes"][f"{theorem}/{name}"] = golden == probe
    report["cases"].append(row)
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert all(report["probes"].values()), report["probes"]

for theorem, name in cases(A):
    assert theorem in SEEDS_0_5_0
    assert not os.path.isdir(f"{DEST}/{theorem}/{name}"), f"{theorem}/{name} already exists"
for theorem, name in cases(A):
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
print("probes:", report["probes"])
for r in report["cases"]:
    obs = r["observation"] or {}
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "first_error_line",
          obs.get("first_error_line"), "hedge", r.get("hedge"), "n_flaws", r["n_flaws_total"],
          "confirmed", r["flaws_confirmed"], r["proof_status"], "correct", r["correct"])
