"""Confirm two production runs of the s2_grammar 0.8.0 goldens are byte-identical, check the shape
of every case, then place run A's outputs under engine/generate/tests/goldens/ with a case.json
each. The checks: every case is s2_grammar 0.8.0 on ineq_chain, true by construction, its truth's
reference the formal control of its own length; the controls are unflawed and valid, the formal
ones building with the standard axioms; each FLAW-0001 case has one instance (a record per file),
`propagated` fatal or `local` repairable, YES-KERNEL, with the `sign` perturbation and a witness
at which the recorded left side exceeds the recorded right side; in a formal file the build fails
with its first error at the site's line (check K); the pair's files are the informal and formal
cases' files. Usage:
python -I -X utf8 goldens-copy15.py <run A base> <run B base> <report path>"""
import hashlib
import json
import os
import shutil
import sys
from fractions import Fraction

A, B, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
MAIN = "syndeom_s2_grammar_ineq_chain"
STANDARD = {"propext", "Classical.choice", "Quot.sound"}
LENGTHS = {"short": (4, 0), "long": (24, 2)}


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


def out_file(base, grammar, name, fn):
    return f"{base}/{grammar}/{name}/receipt/outputs/{fn}"


assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "cases": [], "all_byte_identical": True}
controls = {}
for grammar, name in cases(A):
    assert grammar == "ineq_chain", grammar
    outs_a = sorted(os.listdir(f"{A}/{grammar}/{name}/receipt/outputs"))
    outs_b = sorted(os.listdir(f"{B}/{grammar}/{name}/receipt/outputs"))
    assert outs_a == outs_b, (grammar, name)
    row = {"case": f"{grammar}/{name}", "files": {}}
    for fn in outs_a:
        a = rd(out_file(A, grammar, name, fn))
        b = rd(out_file(B, grammar, name, fn))
        row["files"][fn] = {"sha256": sha(a), "bytes": len(a), "identical": a == b}
        assert b"\r" not in a, (name, fn)
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
    manifest = json.load(open(out_file(A, grammar, name, "manifest.json"), encoding="utf-8"))
    gt = manifest["ground_truth"]
    row["proof_status"] = gt["proof_status"]
    row["correct"] = gt["correct"]
    row["statement_status"] = gt["statement_status"]
    row["records"] = [{"instance": r["instance"], "class": r["class"], "sub_variant": r["sub_variant"],
                       "severity": r["severity"], "lean_catchable": r["lean_catchable"],
                       "location": r["location"], "site": r["dials"].get("site"),
                       "monomial": r["dials"].get("monomial"), "witness": r["dials"].get("witness"),
                       "inserted": r["dials"].get("inserted"), "replaced": r["dials"].get("replaced")}
                      for r in gt["flaws"]]
    row["n_flaws"] = {k: {"commanded": manifest["commanded"][k]["value"],
                          "achieved": manifest["achieved"][k]["value"]}
                      for k in sorted(manifest["commanded"]) if k.startswith("n_flaws")}
    row["flaws_confirmed"] = manifest["achieved"]["flaws_confirmed"]["value"]
    row["commanded_lean_build"] = manifest["commanded"].get("lean_build", {}).get("value")
    row["achieved_lean_build"] = manifest["achieved"].get("lean_build", {}).get("value")
    row["lean_axioms_standard"] = {"commanded": manifest["commanded"].get("lean_axioms_standard", {}).get("value"),
                                   "achieved": manifest["achieved"].get("lean_axioms_standard", {}).get("value")}
    row["n_steps"] = manifest["commanded"].get("n_steps", {}).get("value")
    row["n_lemmas"] = manifest["commanded"].get("n_lemmas", {}).get("value")
    row["n_tokens"] = manifest["achieved"]["n_tokens"]["value"]
    assert manifest["generator"] == {"id": "s2_grammar", "version": "0.8.0", "rng": "syndeom-rng/1"}, manifest["generator"]
    assert manifest["config"]["grammar"] == "ineq_chain", manifest["config"]
    assert gt["statement_status"]["value"] is True and gt["statement_status"]["method"] == "lean-kernel", gt
    assert gt["statement_status"]["reference"].endswith(f":{MAIN}"), gt
    assert gt["taxonomy_version"] == "0.1.0" and gt["derivation"] == "by-construction", gt
    obs = row["observation"]
    target = name.rsplit("-", 1)[1]
    length = "long" if "control-long" in name else "short"
    row["length"] = length
    n, m = LENGTHS[length]
    assert (manifest["config"]["length"]["n_steps"], manifest["config"]["length"]["n_lemmas"]) == (n, m), name
    if row["n_steps"] is not None:
        assert (row["n_steps"], row["n_lemmas"]) == (n + m, m), row
    if target == "informal":
        assert obs is None and row["commanded_lean_build"] is None, row
    if "control" in name:
        assert gt["flaws"] == [] and gt["proof_status"] == "valid" and gt["correct"] is True, name
        if obs is not None:
            assert obs["exit_code"] == 0 and obs["first_error_line"] is None, obs
            assert obs["axioms"] and set(obs["axioms"]) <= STANDARD, obs
            assert row["commanded_lean_build"] == "pass" and row["achieved_lean_build"] == "pass", row
            assert row["lean_axioms_standard"] == {"commanded": True, "achieved": True}, row
        if target == "formal":
            controls[length] = row["artifact_id"]
            assert gt["statement_status"]["reference"] == f"{row['artifact_id']}:{MAIN}", "the formal control names itself"
    if "flaw-0001" in name:
        sub = "propagated" if "propagated" in name else "local"
        severity = "fatal" if sub == "propagated" else "repairable"
        assert gt["proof_status"] == "invalid" and gt["correct"] is False, name
        files = int(target in ("informal", "pair")) + int(target in ("formal", "pair"))
        assert len(gt["flaws"]) == files, (name, len(gt["flaws"]))
        seen = []
        for r in gt["flaws"]:
            assert r["instance"] == 1 and r["class"] == "FLAW-0001" and r["sub_variant"] == sub, r
            assert r["severity"] == severity and r["lean_catchable"] == "YES-KERNEL", r
            d = r["dials"]
            assert d["perturbation"] == "sign", d
            w = d["witness"]
            assert Fraction(w["left"]) > Fraction(w["right"]), w
            assert abs(Fraction(w["x"])) <= 2 and abs(Fraction(w["y"])) <= 2, w
            assert (Fraction(w["x"]) * 2).denominator == 1 and (Fraction(w["y"]) * 2).denominator == 1, w
            assert d["monomial"]["replaced"] != d["monomial"]["inserted"], d
            loc = r["location"]
            if loc["artifact"] == "informal":
                assert set(d) == {"perturbation", "site", "monomial", "witness", "inserted", "replaced", "original_steps"}, d
                assert loc["step"] == d["site"], (loc, d)
            else:
                assert set(d) == {"perturbation", "site", "monomial", "witness", "inserted", "replaced", "original_fragments"}, d
                assert loc["declaration"] == MAIN, loc
                lean = rd(out_file(A, grammar, name, "Proof.lean")).decode("utf-8").split("\n")
                assert lean[loc["line"] - 1].strip() == d["inserted"][0].strip(), (loc, d)
            seen.append(loc["artifact"])
        assert seen == (["informal"] if target == "informal" else ["formal"] if target == "formal" else ["informal", "formal"]), seen
        assert row["n_flaws"] == {"n_flaws_0001": {"commanded": 1, "achieved": 1},
                                  "n_flaws_total": {"commanded": 1, "achieved": 1}}, row["n_flaws"]
        assert row["flaws_confirmed"] == 1, row
        if target != "informal":
            formal_record = [r for r in gt["flaws"] if r["location"]["artifact"] == "formal"][0]
            assert obs is not None and obs["exit_code"] != 0, obs
            assert obs["first_error_line"] == formal_record["location"]["line"], (obs, formal_record)
            assert obs["axioms"] is None, obs
            assert row["commanded_lean_build"] == "fail" and row["achieved_lean_build"] == "fail", row
            assert row["lean_axioms_standard"] == {"commanded": None, "achieved": None}, row
    report["cases"].append(row)
# every case names the 0.8.0 formal control of its length as its truth's reference
for row in report["cases"]:
    assert row["statement_status"]["reference"] == f"{controls[row['length']]}:{MAIN}", row["case"]
# a pair's files are its informal and formal cases' files
for sub in ("propagated", "local"):
    base = f"ineq_chain/v0.8.0-flaw-0001-{sub}"
    for fn, single in (("proof.md", "informal"), ("Proof.lean", "formal")):
        assert rd(out_file(A, "ineq_chain", f"v0.8.0-flaw-0001-{sub}-pair", fn)) == \
            rd(out_file(A, "ineq_chain", f"v0.8.0-flaw-0001-{sub}-{single}", fn)), (base, fn)
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert len(report["cases"]) == 10, len(report["cases"])
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
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "first_error",
          obs.get("first_error_line"), "axioms", obs.get("axioms"), "n_steps", r["n_steps"], "n_lemmas",
          r["n_lemmas"], "n_tokens", r["n_tokens"], "confirmed", r["flaws_confirmed"], r["proof_status"],
          "lean_build", r["commanded_lean_build"], r["achieved_lean_build"],
          [(x["location"], x["site"], x["monomial"], x["witness"]) for x in r["records"]])
