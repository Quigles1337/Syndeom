"""Confirm two production runs of the s2_grammar 0.6.0 goldens are byte-identical, check the
shape of every case (the controls build with standard axioms; each FLAW-0004 `routine-true` case
carries one cosmetic instance at a ring-closed chain step — the hedged step over its whole span
in the informal file, and, in a pair, the formal record at the calc line whose tactic was
omitted, YES-KERNEL in both files, the build failing with its first error at that line and the
formal file the formal control's but for that line; an informal-only artifact's record N/A —
a valid proof of a true statement), then place run A's outputs under
engine/generate/tests/goldens/ with a case.json each. Usage:
python -I -X utf8 goldens-copy13.py <run A base> <run B base> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
MAIN = "syndeom_s2_grammar_ring_chain"
STANDARD = {"propext", "Classical.choice", "Quot.sound"}
HEDGES = ["Clearly", "It is easy to see that", "Obviously", "By a standard argument", "One checks that"]
OMITTED = "ring"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def rd(p):
    return open(p, "rb").read()


def cases(base):
    out = []
    for grammar in sorted(d for d in os.listdir(base) if "." not in d):
        for name in sorted(os.listdir(f"{base}/{grammar}")):
            out.append((grammar, name))
    return out


def steps_of(md):
    """The informal file's step texts, 1-based (syndeom-informal-md/1: `**Step k.** <text>`)."""
    out = {}
    for para in md.split("\n\n"):
        if para.startswith("**Step "):
            k, text = para[len("**Step "):].split(".** ", 1)
            out[int(k)] = text
    return out


assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "cases": [], "all_byte_identical": True}
controls = {}
informal_control = None
formal_control = None
for grammar, name in cases(A):
    outs_a = sorted(os.listdir(f"{A}/{grammar}/{name}/receipt/outputs"))
    outs_b = sorted(os.listdir(f"{B}/{grammar}/{name}/receipt/outputs"))
    assert outs_a == outs_b, (grammar, name)
    row = {"case": f"{grammar}/{name}", "files": {}}
    for fn in outs_a:
        a = rd(f"{A}/{grammar}/{name}/receipt/outputs/{fn}")
        b = rd(f"{B}/{grammar}/{name}/receipt/outputs/{fn}")
        row["files"][fn] = {"sha256": sha(a), "bytes": len(a), "identical": a == b}
        if a != b:
            report["all_byte_identical"] = False
    ra = json.load(open(f"{A}/{grammar}/{name}/receipt/receipt.json", encoding="utf-8"))
    rb = json.load(open(f"{B}/{grammar}/{name}/receipt/receipt.json", encoding="utf-8"))
    row["engine_commit"] = ra["engine_commit"]
    row["same_engine_commit"] = ra["engine_commit"] == rb["engine_commit"]
    row["same_observation"] = ra["observation"] == rb["observation"]
    row["observation"] = ra["observation"]
    row["verdict"] = ra["verdict"]
    row["artifact_id"] = ra["artifact_ids"][0]
    row["producer"] = ra["producer"]
    manifest = json.load(open(f"{A}/{grammar}/{name}/receipt/outputs/manifest.json", encoding="utf-8"))
    gt = manifest["ground_truth"]
    row["proof_status"] = gt["proof_status"]
    row["correct"] = gt["correct"]
    row["statement_status"] = gt["statement_status"]
    row["records"] = [{"instance": r["instance"], "class": r["class"], "sub_variant": r["sub_variant"],
                       "severity": r["severity"], "lean_catchable": r["lean_catchable"],
                       "location": r["location"], "hidden_step": r["dials"].get("hidden_step"),
                       "hedge": r["dials"].get("hedge"), "hedge_text": r["dials"].get("hedge_text"),
                       "justification": r["dials"].get("justification"), "claim": r["dials"].get("claim"),
                       "omitted": r["dials"].get("omitted"),
                       "inserted": r["dials"].get("inserted"), "replaced": r["dials"].get("replaced")}
                      for r in gt["flaws"]]
    row["n_flaws_total"] = {"commanded": manifest["commanded"]["n_flaws_total"]["value"],
                            "achieved": manifest["achieved"]["n_flaws_total"]["value"]}
    row["flaws_confirmed"] = manifest["achieved"]["flaws_confirmed"]["value"]
    row["commanded_lean_build"] = manifest["commanded"].get("lean_build", {}).get("value")
    row["achieved_lean_build"] = manifest["achieved"].get("lean_build", {}).get("value")
    row["lean_axioms_standard"] = {"commanded": manifest["commanded"].get("lean_axioms_standard", {}).get("value"),
                                   "achieved": manifest["achieved"].get("lean_axioms_standard", {}).get("value")}
    row["n_steps"] = manifest["commanded"].get("n_steps", {}).get("value")
    row["n_lemmas"] = manifest["commanded"].get("n_lemmas", {}).get("value")
    row["n_tokens"] = manifest["achieved"]["n_tokens"]["value"]
    assert manifest["generator"] == {"id": "s2_grammar", "version": "0.6.0", "rng": "syndeom-rng/1"}, manifest["generator"]
    assert gt["statement_status"]["value"] is True and gt["statement_status"]["method"] == "lean-kernel", gt
    assert gt["statement_status"]["reference"].endswith(f":{MAIN}"), gt
    obs = row["observation"]
    if row["n_steps"] is not None:
        assert row["n_lemmas"] == 0 and row["n_steps"] == 4, row
    if "control" in name:
        assert gt["flaws"] == [] and gt["proof_status"] == "valid" and gt["correct"] is True, name
        if obs is not None:
            assert obs["exit_code"] == 0, name
            assert obs["axioms"] and set(obs["axioms"]) <= STANDARD, obs
            assert row["commanded_lean_build"] == "pass", row
            assert row["lean_axioms_standard"] == {"commanded": True, "achieved": True}, row
        if name.endswith("-formal"):
            controls[name] = row["artifact_id"]
            assert gt["statement_status"]["reference"] == f"{row['artifact_id']}:{MAIN}", "the formal control names itself"
            formal_control = rd(f"{A}/{grammar}/{name}/receipt/outputs/Proof.lean")
        if name.endswith("-informal"):
            assert obs is None
            informal_control = rd(f"{A}/{grammar}/{name}/receipt/outputs/proof.md")
    if "flaw-0004" in name:
        # a valid proof of a true statement: the instance is cosmetic (D7)
        assert gt["proof_status"] == "valid" and gt["correct"] is True, name
        is_pair = name.endswith("-pair")
        assert len(gt["flaws"]) == (2 if is_pair else 1), (name, "a pair carries a record in each file")
        ri = gt["flaws"][0]
        assert ri["instance"] == 1 and ri["class"] == "FLAW-0004" and ri["sub_variant"] is None, ri
        assert ri["severity"] == "cosmetic", ri
        assert ri["lean_catchable"] == ("YES-KERNEL" if is_pair else "N/A"), ri
        assert ri["location"]["artifact"] == "informal", ri
        d = ri["dials"]
        site = d["site"]
        assert site == ri["location"]["step"], ri
        assert d["hidden_step"] == "routine-true", d
        assert d["hedge_text"] == HEDGES[d["hedge"]], d
        assert d["inserted"] == f"{d['hedge_text']} {d['claim']}", d
        assert d["replaced"] == f"{d['justification']}, {d['claim']}", d
        assert d["replaced"].count(d["justification"]) == 1 and d["justification"] not in d["inserted"], d
        assert set(d["original_steps"]) == {str(site)} and d["original_steps"][str(site)] == d["replaced"], d
        assert "omitted" not in d, d
        # the informal file: the hedged step over its whole span; every other step the control's
        md = rd(f"{A}/{grammar}/{name}/receipt/outputs/proof.md").decode("utf-8")
        steps = steps_of(md)
        control_steps = steps_of(informal_control.decode("utf-8"))
        assert set(steps) == set(control_steps) == {1, 2, 3, 4}, steps
        assert steps[site] == d["inserted"] and control_steps[site] == d["replaced"], (steps[site], d)
        assert ri["location"]["span"] == [0, len(steps[site])], ri
        for k in steps:
            if k != site:
                assert steps[k] == control_steps[k], k
        assert row["n_flaws_total"] == {"commanded": 1, "achieved": 1} and row["flaws_confirmed"] == 1, row
        if is_pair:
            rf = gt["flaws"][1]
            assert rf["instance"] == 1 and rf["class"] == "FLAW-0004" and rf["severity"] == "cosmetic", rf
            assert rf["lean_catchable"] == "YES-KERNEL", rf
            assert rf["location"]["artifact"] == "formal" and rf["location"]["declaration"] == MAIN, rf
            df = rf["dials"]
            assert df["site"] == site and df["hedge"] == d["hedge"] and df["hedge_text"] == d["hedge_text"], df
            assert df["omitted"] == [OMITTED], df
            assert len(df["inserted"]) == 1 and len(df["replaced"]) == 1, df
            assert df["replaced"][0] == f"{df['inserted'][0]} {OMITTED}" and df["inserted"][0].endswith(" := by"), df
            assert df["original_fragments"] == {str(site): df["replaced"]}, df
            assert "declaration" not in df and "claim" not in df and "justification" not in df, df
            # the build fails with its first error at the record's line (check K), no axiom print
            assert obs is not None and obs["exit_code"] != 0, obs
            assert obs["first_error_line"] == rf["location"]["line"], (obs, rf["location"])
            assert obs["axioms"] is None, obs
            assert row["commanded_lean_build"] == "fail" and row["achieved_lean_build"] == "fail", row
            assert row["lean_axioms_standard"] == {"commanded": None, "achieved": None}, row
            # the formal file: the control's but for the one line, which is the flawed calc line
            lean = rd(f"{A}/{grammar}/{name}/receipt/outputs/Proof.lean").decode("utf-8").split("\n")
            ctrl = formal_control.decode("utf-8").split("\n")
            assert len(lean) == len(ctrl), (len(lean), len(ctrl))
            diff = [i for i in range(len(lean)) if lean[i] != ctrl[i]]
            assert diff == [rf["location"]["line"] - 1], diff
            assert lean[diff[0]].strip() == df["inserted"][0].strip() and ctrl[diff[0]].strip() == df["replaced"][0].strip()
        else:
            assert obs is None
            assert row["commanded_lean_build"] is None
    report["cases"].append(row)
# every case names the 0.6.0 formal control of its length as its truth's reference
for row in report["cases"]:
    assert row["statement_status"]["reference"] == f"{controls['v0.6.0-control-short-formal']}:{MAIN}", row["case"]
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert len(report["cases"]) == 4
assert all(r["verdict"] == "PASS" for r in report["cases"]), [r["verdict"] for r in report["cases"]]

for grammar, name in cases(A):
    src = f"{A}/{grammar}/{name}"
    out = f"{DEST}/{grammar}/{name}"
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
for r in report["cases"]:
    obs = r["observation"] or {}
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "first_error", obs.get("first_error_line"),
          "axioms", obs.get("axioms"), "n_steps", r["n_steps"], "n_lemmas", r["n_lemmas"], "n_tokens", r["n_tokens"],
          "n_flaws", r["n_flaws_total"], "confirmed", r["flaws_confirmed"], r["proof_status"], "lean_build",
          r["commanded_lean_build"], r["achieved_lean_build"],
          [(x["instance"], x["lean_catchable"], x["hedge_text"], x["location"]) for x in r["records"]])
