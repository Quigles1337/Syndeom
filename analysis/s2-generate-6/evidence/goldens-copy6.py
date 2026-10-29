"""Confirm two production runs of the 0.6.0 goldens are byte-identical, compare every formal file
with the hand-written Lean probe it realizes (the probes under evidence/lean-probe/, compiled one
process at a time before the seed record was written; a probe's trailing `#print axioms` line is
not part of the golden) or, for the FLAW-0010 pair, with the committed 0.4.0 control golden's
Proof.lean (the formal file is unchanged), then place run A's outputs under
engine/generate/tests/goldens/ with a case.json each. Usage:
python -I -X utf8 goldens-copy6.py <run A base> <run B base> <probe dir> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, PROBES, REPORT = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
PROOF_FILES = ("proof.md", "Proof.lean")
# which probe (or committed golden) each formal file must equal, after the probe's tail is dropped
FORMAL_REFERENCE = {
    "sqrt2_irrational/flaw-0010-journal-pair": ("golden", f"{DEST}/sqrt2_irrational/control-formal/Proof.lean"),
    "bounded_monotone_converges/control-formal": ("probe", f"{PROBES}/s2i_faithful.lean"),
    "bounded_monotone_converges/flaw-0003-direct-restated-goal-formal": ("probe", f"{PROBES}/s2i_direct_restated.lean"),
    "bounded_monotone_converges/flaw-0003-direct-restated-goal-pair": ("probe", f"{PROBES}/s2i_direct_restated.lean"),
    "bounded_monotone_converges/flaw-0003-direct-contrapositive-formal": ("probe", f"{PROBES}/s2i_direct_contrapositive.lean"),
    "bounded_monotone_converges/flaw-0003-direct-contrapositive-pair": ("probe", f"{PROBES}/s2i_direct_contrapositive.lean"),
    "bounded_monotone_converges/flaw-0003-cycle-pair": ("probe", f"{PROBES}/s2i_cycle.lean"),
}


def sha(b):
    return hashlib.sha256(b).hexdigest()


def rd(p):
    return open(p, "rb").read()


def without_tail(probe_bytes):
    """A probe is the golden followed by `\\n#print axioms <main>\\n`, or the golden itself."""
    text = probe_bytes.decode("utf-8")
    lines = text.split("\n")
    # drop a trailing `#print axioms …` line and the blank line before it
    if len(lines) >= 3 and lines[-1] == "" and lines[-2].startswith("#print axioms ") and lines[-3] == "":
        return ("\n".join(lines[:-3]) + "\n").encode("utf-8")
    return probe_bytes


def cases(base):
    out = []
    for theorem in sorted(d for d in os.listdir(base) if "." not in d):
        for name in sorted(os.listdir(f"{base}/{theorem}")):
            out.append((theorem, name))
    return out


assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "probes": PROBES, "cases": [], "all_byte_identical": True,
          "formal_equals_reference": {}}
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
    row["records"] = [{"instance": r["instance"], "class": r["class"], "sub_variant": r["sub_variant"],
                       "severity": r["severity"], "lean_catchable": r["lean_catchable"],
                       "location": r["location"]} for r in gt["flaws"]]
    row["n_flaws_total"] = {"commanded": manifest["commanded"]["n_flaws_total"]["value"],
                            "achieved": manifest["achieved"]["n_flaws_total"]["value"]}
    row["flaws_confirmed"] = manifest["achieved"]["flaws_confirmed"]["value"]
    row["commanded_lean_build"] = manifest["commanded"].get("lean_build", {}).get("value")
    key = f"{theorem}/{name}"
    if key in FORMAL_REFERENCE:
        kind, path = FORMAL_REFERENCE[key]
        ref = rd(path)
        if kind == "probe":
            ref = without_tail(ref)
        same = rd(f"{A}/{theorem}/{name}/receipt/outputs/Proof.lean") == ref
        row["formal_reference"] = {"kind": kind, "path": path, "equal": same, "sha256": sha(ref)}
        report["formal_equals_reference"][key] = same
    else:
        assert "Proof.lean" not in outs_a, key
    if theorem == "sqrt2_irrational":
        # the FLAW-0010 instance sits at step 3 after `By `, and the step text is the reference
        # followed by the bare claim; the formal file is the faithful one (pair)
        rec = gt["flaws"][0]
        assert rec["class"] == "FLAW-0010" and rec["location"]["step"] == 3 and rec["location"]["span"][0] == 3, rec
        text = rd(f"{A}/{theorem}/{name}/receipt/outputs/proof.md").decode("utf-8")
        assert rec["dials"]["reference"] in text and "**Step 3.** By [" in text, text
        row["reference"] = rec["dials"]["reference"]
    report["cases"].append(row)
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert all(report["formal_equals_reference"].values()), report["formal_equals_reference"]
assert len(report["formal_equals_reference"]) == 7
assert len(report["cases"]) == 12
assert all(r["verdict"] == "PASS" for r in report["cases"]), [r["verdict"] for r in report["cases"]]

for theorem, name in cases(A):
    src = f"{A}/{theorem}/{name}"
    out = f"{DEST}/{theorem}/{name}"
    assert not os.path.isdir(out), f"{out} exists: goldens are never rewritten"
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
print("formal files equal their references:", report["formal_equals_reference"])
for r in report["cases"]:
    obs = r["observation"] or {}
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "first_error_line",
          obs.get("first_error_line"), "axioms", obs.get("axioms"), "n_flaws", r["n_flaws_total"], "confirmed",
          r["flaws_confirmed"], r["proof_status"], "correct", r["correct"], "lean_build", r["commanded_lean_build"])
