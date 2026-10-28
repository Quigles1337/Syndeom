"""Confirm two production runs of the 0.3.0 goldens are byte-identical, compare each formal
output with the hand-written Lean probe that preceded the seed record, then place run A's
outputs under engine/generate/tests/goldens/n_lt_two_pow/ with a case.json each. Usage:
python -I -X utf8 goldens-copy3.py <run A base> <run B base> <probe dir> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, PROBES, REPORT = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
SEEDS_0_3_0 = ["n_lt_two_pow"]
# formal golden -> the probe whose bytes are the golden followed by the probe's own audit tail
PROBE_OF = {
    "n_lt_two_pow/control-formal": "nltp_faithful",
    "n_lt_two_pow/flaw-0009-gap-formal": "nltp_axiom_true",
    "n_lt_two_pow/flaw-0009-gap-pair": "nltp_axiom_true",
    "n_lt_two_pow/flaw-0009-fatal-formal": "nltp_axiom_false",
    "n_lt_two_pow/flaw-0009-fatal-pair": "nltp_axiom_false",
    "n_lt_two_pow/flaw-0007-unused-formal": "nltp_unused",
    "n_lt_two_pow/flaw-0007-unused-pair": "nltp_unused",
}


def sha(b):
    return hashlib.sha256(b).hexdigest()


def cases(base):
    out = []
    for theorem in sorted(d for d in os.listdir(base) if "." not in d):
        for name in sorted(os.listdir(f"{base}/{theorem}")):
            out.append((theorem, name))
    return out


assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "cases": [], "all_byte_identical": True, "probes": {}}
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
    probe = PROBE_OF.get(f"{theorem}/{name}")
    if probe:
        golden = open(f"{A}/{theorem}/{name}/receipt/outputs/Proof.lean", "rb").read()
        probe_bytes = open(f"{PROBES}/{probe}.lean", "rb").read()
        tail_at = probe_bytes.find(b"\n#print axioms ")
        assert tail_at > 0, probe
        head = probe_bytes[:tail_at]  # the probe's bytes before the blank line that precedes its tail
        row["probe"] = {"file": f"{probe}.lean", "golden_is_probe_without_its_tail": head == golden,
                        "probe_sha256": sha(probe_bytes)}
        report["probes"][f"{theorem}/{name}"] = row["probe"]["golden_is_probe_without_its_tail"]
    report["cases"].append(row)
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])

for theorem in SEEDS_0_3_0:
    assert not os.path.isdir(f"{DEST}/{theorem}"), f"{theorem} goldens already exist"
for theorem, name in cases(A):
    assert theorem in SEEDS_0_3_0
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
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "first_error",
          obs.get("first_error_line"), "axioms", obs.get("axioms"))
