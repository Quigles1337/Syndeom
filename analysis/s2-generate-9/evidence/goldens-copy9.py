"""Confirm two production runs of the s2_grammar 0.2.0 goldens are byte-identical, check the
shape of every case (the control builds with standard axioms; the FLAW-0007 `unused` cases carry
one cosmetic YES-TOOLING instance per inserted lemma, confirmed by check T; the FLAW-0013 cases
one cosmetic instance per paragraph, informal NO and formal YES-TOOLING, the achieved ratio within
the commanded tolerance), then place run A's outputs under engine/generate/tests/goldens/ with a
case.json each. Usage:
python -I -X utf8 goldens-copy9.py <run A base> <run B base> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
MAIN = "syndeom_s2_grammar_ring_chain"
STANDARD = {"propext", "Classical.choice", "Quot.sound"}


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


def closure(used, main):
    seen, stack = set(), [main]
    while stack:
        d = stack.pop()
        if d in seen:
            continue
        seen.add(d)
        for n in used.get(d, []):
            if n in used and n not in seen:
                stack.append(n)
    return seen


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
                       "location": r["location"], "kind": r["dials"].get("kind"),
                       "seed_form": r["dials"].get("seed_form"), "declaration": r["dials"].get("declaration")}
                      for r in gt["flaws"]]
    row["n_flaws_total"] = {"commanded": manifest["commanded"]["n_flaws_total"]["value"],
                            "achieved": manifest["achieved"]["n_flaws_total"]["value"]}
    row["flaws_confirmed"] = manifest["achieved"]["flaws_confirmed"]["value"]
    row["commanded_lean_build"] = manifest["commanded"].get("lean_build", {}).get("value")
    row["n_steps"] = manifest["commanded"].get("n_steps", {}).get("value")
    row["n_lemmas"] = manifest["commanded"].get("n_lemmas", {}).get("value")
    row["n_tokens"] = manifest["achieved"]["n_tokens"]["value"]
    row["padding_ratio"] = {"commanded": manifest["commanded"]["padding_ratio"]["value"],
                            "achieved": manifest["achieved"]["padding_ratio"]["value"]}
    assert manifest["generator"] == {"id": "s2_grammar", "version": "0.2.0", "rng": "syndeom-rng/1"}, manifest["generator"]
    assert gt["statement_status"]["value"] is True and gt["statement_status"]["method"] == "lean-kernel", gt
    assert gt["statement_status"]["reference"].endswith(f":{MAIN}"), gt
    obs = row["observation"]
    if obs is not None:
        # every 0.2.0 golden builds: the controls, the unused lemmas and the padding are all
        # cosmetic or absent, and the axioms are within the standard three (SD2(ii))
        assert obs["exit_code"] == 0, name
        assert obs["axioms"] and set(obs["axioms"]) <= STANDARD, obs
        assert row["commanded_lean_build"] == "pass", row
    if name.endswith("control-short"):
        pass
    if "control" in name:
        assert gt["flaws"] == [] and gt["proof_status"] == "valid" and gt["correct"] is True, name
        if name.endswith("-formal"):
            controls[name] = row["artifact_id"]
            assert gt["statement_status"]["reference"] == f"{row['artifact_id']}:{MAIN}", "the formal control names itself"
    if "flaw-0007" in name:
        # two unused lemmas, each its own instance, cosmetic, YES-TOOLING in both files; the
        # artifact is a valid proof; check T confirms every formal record
        assert gt["proof_status"] == "valid" and gt["correct"] is True, name
        assert {r["instance"] for r in gt["flaws"]} == {1, 2}, gt["flaws"]
        assert all(r["class"] == "FLAW-0007" and r["sub_variant"] == "unused" and r["severity"] == "cosmetic"
                   and r["lean_catchable"] == "YES-TOOLING" for r in gt["flaws"]), gt["flaws"]
        assert row["n_flaws_total"] == {"commanded": 2, "achieved": 2} and row["flaws_confirmed"] == 2, row
        if row["n_steps"] is not None:  # commanded only where an informal file exists
            assert row["n_lemmas"] == 2 and row["n_steps"] == 6, row
        formal = [r for r in gt["flaws"] if r["location"]["artifact"] == "formal"]
        if obs is not None:
            assert len(formal) == 2
            used = obs["used_constants"]
            reach = closure(used, MAIN)
            for r in formal:
                decl = r["location"]["declaration"]
                assert decl == r["dials"]["declaration"] and decl.startswith(f"{MAIN}_unused_"), r
                assert decl in used and decl not in reach, (decl, sorted(reach))
    if "flaw-0013" in name:
        assert gt["proof_status"] == "valid" and gt["correct"] is True, name
        assert {r["instance"] for r in gt["flaws"]} == {1, 2}, gt["flaws"]
        assert all(r["class"] == "FLAW-0013" and r["sub_variant"] is None and r["severity"] == "cosmetic"
                   for r in gt["flaws"]), gt["flaws"]
        for r in gt["flaws"]:
            if r["location"]["artifact"] == "informal":
                assert r["lean_catchable"] == "NO", r
            else:
                assert r["lean_catchable"] == "YES-TOOLING", r
                assert r["location"]["declaration"] == r["dials"]["declaration"] and \
                    r["dials"]["declaration"].startswith(f"{MAIN}_pad_"), r
            assert r["dials"]["kind"] in ("re-derivation", "unused-lemma"), r
        assert row["n_flaws_total"] == {"commanded": 2, "achieved": 2} and row["flaws_confirmed"] == 2, row
        if row["n_steps"] is not None:
            assert row["n_lemmas"] == 0 and row["n_steps"] == 6, row
        c, a = row["padding_ratio"]["commanded"], row["padding_ratio"]["achieved"]
        assert c > 0 and abs(c - a) <= 0.05, row["padding_ratio"]
        if obs is not None:
            used = obs["used_constants"]
            reach = closure(used, MAIN)
            for r in gt["flaws"]:
                if r["location"]["artifact"] == "formal":
                    decl = r["location"]["declaration"]
                    assert decl in used and decl not in reach, (decl, sorted(reach))
    report["cases"].append(row)
# every short case names the 0.2.0 short formal control as its truth's reference
for row in report["cases"]:
    assert row["statement_status"]["reference"] == f"{controls['v0.2.0-control-short-formal']}:{MAIN}", row["case"]
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert len(report["cases"]) == 8
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
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "axioms", obs.get("axioms"),
          "n_steps", r["n_steps"], "n_lemmas", r["n_lemmas"], "n_tokens", r["n_tokens"], "n_flaws", r["n_flaws_total"],
          "confirmed", r["flaws_confirmed"], r["proof_status"], "ratio", r["padding_ratio"],
          [(x["instance"], x["kind"], x["seed_form"], x["location"]) for x in r["records"]])
