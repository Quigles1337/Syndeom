"""Confirm two production runs of the 0.4.0 goldens are byte-identical, compare each formal
output with the hand-written Lean probes that preceded the seed record (the control is the
faithful probe without its tail; a padded file's declarations are blocks of the padded probe),
then place run A's outputs under engine/generate/tests/goldens/sqrt2_irrational/ with a
case.json each. Usage:
python -I -X utf8 goldens-copy4.py <run A base> <run B base> <probe dir> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, PROBES, REPORT = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
SEEDS_0_4_0 = ["sqrt2_irrational"]
MAIN = "syndeom_s2_template_sqrt2_irrational"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def blocks(text):
    """The declaration blocks of a Lean file: maximal runs of non-blank lines that begin with
    `theorem`, each as its exact text."""
    out, cur = [], []
    for line in text.split("\n"):
        if line == "":
            if cur:
                out.append("\n".join(cur))
                cur = []
        else:
            cur.append(line)
    if cur:
        out.append("\n".join(cur))
    return [b for b in out if b.startswith("theorem ")]


def cases(base):
    out = []
    for theorem in sorted(d for d in os.listdir(base) if "." not in d):
        for name in sorted(os.listdir(f"{base}/{theorem}")):
            out.append((theorem, name))
    return out


faithful = open(f"{PROBES}/s2i_faithful.lean", "rb").read()
padded = open(f"{PROBES}/s2i_padded.lean", "rb").read()
faithful_head = faithful[:faithful.find(b"\n#print axioms ")]
padded_blocks = blocks(padded[:padded.find(b"\n#print axioms ")].decode("utf-8"))
main_block = [b for b in padded_blocks if b.startswith(f"theorem {MAIN} ")][0]

assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "cases": [], "all_byte_identical": True, "probes": {},
          "probe_files": {"s2i_faithful.lean": sha(faithful), "s2i_padded.lean": sha(padded)}}
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
    row["padding_ratio"] = {"commanded": manifest["commanded"]["padding_ratio"]["value"],
                            "achieved": manifest["achieved"]["padding_ratio"]["value"]}
    row["n_flaws_total"] = manifest["achieved"]["n_flaws_total"]["value"]
    if "Proof.lean" in outs_a:
        golden = open(f"{A}/{theorem}/{name}/receipt/outputs/Proof.lean", "rb").read()
        if name.startswith("control-"):
            row["probe"] = {"file": "s2i_faithful.lean", "golden_is_probe_without_its_tail": faithful_head == golden}
            report["probes"][f"{theorem}/{name}"] = row["probe"]["golden_is_probe_without_its_tail"]
        else:
            gb = blocks(golden.decode("utf-8"))
            pads = [b for b in gb if "_pad_" in b.split("\n")[0]]
            ok = all(b in padded_blocks for b in pads) and gb[-1] == main_block and len(gb) == len(pads) + 1
            row["probe"] = {"file": "s2i_padded.lean", "declarations_are_probe_blocks_in_drawn_order": ok,
                            "padding_declarations": [b.split("\n")[0].split(" ")[1] for b in pads]}
            report["probes"][f"{theorem}/{name}"] = ok
    report["cases"].append(row)
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert all(report["probes"].values()), report["probes"]

for theorem in SEEDS_0_4_0:
    assert not os.path.isdir(f"{DEST}/{theorem}"), f"{theorem} goldens already exist"
for theorem, name in cases(A):
    assert theorem in SEEDS_0_4_0
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
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "axioms", obs.get("axioms"),
          "ratio", r["padding_ratio"], "n_flaws", r["n_flaws_total"])
