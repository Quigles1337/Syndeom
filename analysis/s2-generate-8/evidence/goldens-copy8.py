"""Confirm two production runs of the s2_grammar 0.1.0 goldens are byte-identical, check the
shape of every case (the controls build with standard axioms; the FLAW-0001 cases fail at their
site's calc line, one instance, the claim of record untouched), then place run A's outputs under
engine/generate/tests/goldens/ with a case.json each. Usage:
python -I -X utf8 goldens-copy8.py <run A base> <run B base> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
MAIN = "syndeom_s2_grammar_ring_chain"


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


assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "cases": [], "all_byte_identical": True}
controls = {}
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
                       "location": r["location"], "perturbation": r["dials"].get("perturbation"),
                       "monomial": r["dials"].get("monomial")} for r in gt["flaws"]]
    row["n_flaws_total"] = {"commanded": manifest["commanded"]["n_flaws_total"]["value"],
                            "achieved": manifest["achieved"]["n_flaws_total"]["value"]}
    row["flaws_confirmed"] = manifest["achieved"]["flaws_confirmed"]["value"]
    row["commanded_lean_build"] = manifest["commanded"].get("lean_build", {}).get("value")
    row["n_steps"] = manifest["commanded"].get("n_steps", {}).get("value")
    row["n_lemmas"] = manifest["commanded"].get("n_lemmas", {}).get("value")
    row["n_tokens"] = manifest["achieved"]["n_tokens"]["value"]
    assert manifest["generator"] == {"id": "s2_grammar", "version": "0.1.0", "rng": "syndeom-rng/1"}, manifest["generator"]
    assert gt["statement_status"]["value"] is True and gt["statement_status"]["method"] == "lean-kernel", gt
    assert gt["statement_status"]["reference"].endswith(f":{MAIN}"), gt
    if name.startswith("control"):
        assert gt["flaws"] == [] and gt["proof_status"] == "valid" and gt["correct"] is True, name
        if row["observation"] is not None:
            assert row["observation"]["exit_code"] == 0, name
            # within the standard three (SD2(ii)); the generated controls print propext and
            # Quot.sound where the hand-written probes printed propext alone
            assert row["observation"]["axioms"] and set(row["observation"]["axioms"]) <= {"propext", "Classical.choice", "Quot.sound"}, row["observation"]
        if name.endswith("-formal"):
            controls[name] = row["artifact_id"]
            assert gt["statement_status"]["reference"] == f"{row['artifact_id']}:{MAIN}", "the formal control names itself"
    if name.startswith("flaw-0001"):
        # one instance of FLAW-0001, a record per file; the chain fails at the site's calc line
        assert gt["proof_status"] == "invalid" and gt["correct"] is False, name
        assert {r["instance"] for r in gt["flaws"]} == {1}, gt["flaws"]
        assert all(r["class"] == "FLAW-0001" and r["lean_catchable"] == "YES-KERNEL" for r in gt["flaws"])
        sub = gt["flaws"][0]["sub_variant"]
        assert sub in ("propagated", "local")
        assert gt["flaws"][0]["severity"] == ("fatal" if sub == "propagated" else "repairable")
        assert row["n_flaws_total"] == {"commanded": 1, "achieved": 1} and row["flaws_confirmed"] == 1, row
        if row["observation"] is not None:
            assert row["observation"]["exit_code"] == 1, name
            formal = [r for r in gt["flaws"] if r["location"]["artifact"] == "formal"]
            assert len(formal) == 1 and formal[0]["location"]["declaration"] == MAIN
            assert row["observation"]["first_error_line"] == formal[0]["location"]["line"], (row["observation"], formal)
            assert row["commanded_lean_build"] == "fail"
    report["cases"].append(row)
# the short-length cases name the short formal control as their truth's reference
for row in report["cases"]:
    name = row["case"].split("/")[1]
    if "short" in name or name.startswith("flaw-0001"):
        assert row["statement_status"]["reference"] == f"{controls['control-short-formal']}:{MAIN}", row["case"]
    if "long" in name:
        assert row["statement_status"]["reference"] == f"{controls['control-long-formal']}:{MAIN}", row["case"]
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert len(report["cases"]) == 10
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
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "first_error_line",
          obs.get("first_error_line"), "axioms", obs.get("axioms"), "n_steps", r["n_steps"], "n_lemmas", r["n_lemmas"],
          "n_tokens", r["n_tokens"], "n_flaws", r["n_flaws_total"], "confirmed", r["flaws_confirmed"],
          r["proof_status"], "correct", r["correct"], "lean_build", r["commanded_lean_build"],
          [(x["sub_variant"], x["perturbation"], x["monomial"], x["location"]) for x in r["records"]])
