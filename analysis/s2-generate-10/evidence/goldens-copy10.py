"""Confirm two production runs of the s2_grammar 0.3.0 goldens are byte-identical, check the
shape of every case (the control builds with standard axioms; each FLAW-0009 `axiom` case carries
one formal-only YES-TOOLING instance at the axiom's declaration — gap for `truth: true`, fatal
for `truth: false` — whose axiom the main declaration's print lists, `lean_axioms_standard`
commanded and achieved false, the informal file of a pair the control's), then place run A's
outputs under engine/generate/tests/goldens/ with a case.json each. Usage:
python -I -X utf8 goldens-copy10.py <run A base> <run B base> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
MAIN = "syndeom_s2_grammar_ring_chain"
LEMMA = f"{MAIN}_lemma_1"
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


assert cases(A) == cases(B), "the two runs produced different case sets"
report = {"run_a": A, "run_b": B, "cases": [], "all_byte_identical": True}
controls = {}
informal_control = None
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
                       "location": r["location"], "truth": r["dials"].get("truth"),
                       "declaration": r["dials"].get("declaration"),
                       "perturbation": r["dials"].get("perturbation"),
                       "counterexample": r["dials"].get("counterexample"),
                       "use_site": r["dials"].get("use_site")}
                      for r in gt["flaws"]]
    row["n_flaws_total"] = {"commanded": manifest["commanded"]["n_flaws_total"]["value"],
                            "achieved": manifest["achieved"]["n_flaws_total"]["value"]}
    row["flaws_confirmed"] = manifest["achieved"]["flaws_confirmed"]["value"]
    row["commanded_lean_build"] = manifest["commanded"].get("lean_build", {}).get("value")
    row["lean_axioms_standard"] = {"commanded": manifest["commanded"].get("lean_axioms_standard", {}).get("value"),
                                   "achieved": manifest["achieved"].get("lean_axioms_standard", {}).get("value")}
    row["n_steps"] = manifest["commanded"].get("n_steps", {}).get("value")
    row["n_lemmas"] = manifest["commanded"].get("n_lemmas", {}).get("value")
    row["n_tokens"] = manifest["achieved"]["n_tokens"]["value"]
    assert manifest["generator"] == {"id": "s2_grammar", "version": "0.3.0", "rng": "syndeom-rng/1"}, manifest["generator"]
    assert gt["statement_status"]["value"] is True and gt["statement_status"]["method"] == "lean-kernel", gt
    assert gt["statement_status"]["reference"].endswith(f":{MAIN}"), gt
    obs = row["observation"]
    if obs is not None:
        # every 0.3.0 golden builds: the axiom is accepted by the kernel and caught by the print
        assert obs["exit_code"] == 0, name
        assert obs["axioms"], obs
        assert row["commanded_lean_build"] == "pass", row
    if "control" in name:
        assert gt["flaws"] == [] and gt["proof_status"] == "valid" and gt["correct"] is True, name
        if obs is not None:
            assert set(obs["axioms"]) <= STANDARD, obs
            assert row["lean_axioms_standard"] == {"commanded": True, "achieved": True}, row
        if name.endswith("-formal"):
            controls[name] = row["artifact_id"]
            assert gt["statement_status"]["reference"] == f"{row['artifact_id']}:{MAIN}", "the formal control names itself"
        if name.endswith("-informal"):
            informal_control = rd(f"{A}/{grammar}/{name}/receipt/outputs/proof.md")
    if "flaw-0009" in name:
        truth = "gap" in name
        severity = "gap" if truth else "fatal"
        assert gt["proof_status"] == "invalid" and gt["correct"] is False, name
        assert len(gt["flaws"]) == 1, "FLAW-0009 manifests in the formal file only"
        r = gt["flaws"][0]
        assert r["instance"] == 1 and r["class"] == "FLAW-0009" and r["sub_variant"] is None, r
        assert r["severity"] == severity and r["lean_catchable"] == "YES-TOOLING", r
        assert r["location"]["artifact"] == "formal" and r["location"]["declaration"] == LEMMA, r
        d = r["dials"]
        assert d["mode"] == "axiom" and d["truth"] is truth and d["site"] == 1 and d["declaration"] == LEMMA, d
        assert len(d["inserted"]) == 1 and d["inserted"][0].startswith(f"axiom {LEMMA} : ∀ x y : ℤ, "), d
        assert d["replaced"][0].startswith(f"theorem {LEMMA} : ∀ x y : ℤ, ") and d["replaced"][1:] == ["  intro x y", "  ring"], d
        if truth:
            assert set(d["original_fragments"]) == {"1"}, d
            assert not any(k in d for k in ("perturbation", "monomial", "counterexample", "use_site")), d
        else:
            assert set(d["original_fragments"]) == {"1", str(d["use_site"])}, d
            assert d["perturbation"] in ("sign", "coefficient", "constant", "exponent"), d
            assert set(d["counterexample"]) == {"x", "y", "lhs", "rhs"}, d
            assert d["counterexample"]["lhs"] != d["counterexample"]["rhs"], d
        assert row["n_flaws_total"] == {"commanded": 1, "achieved": 1} and row["flaws_confirmed"] == 1, row
        assert row["lean_axioms_standard"] == {"commanded": False, "achieved": False}, row
        if row["n_steps"] is not None:  # commanded only where an informal file exists
            assert row["n_lemmas"] == 1 and row["n_steps"] == 5, row
        assert obs is not None
        assert LEMMA in obs["axioms"] and not set(obs["axioms"]) <= STANDARD, obs
        # the formal file: the axiom at the first declaration line, listed by the audit tail
        lean = rd(f"{A}/{grammar}/{name}/receipt/outputs/Proof.lean").decode("utf-8")
        assert f"\n{d['inserted'][0]}\n\n" in lean and f"theorem {LEMMA} " not in lean, name
        assert lean.split("\n")[r["location"]["line"] - 1] == d["inserted"][0], name
        assert LEMMA in obs["used_constants"], obs
        if not truth:
            x, y = d["counterexample"]["x"], d["counterexample"]["y"]
            assert f" := absurd ({LEMMA} {x} {y}) (by norm_num)\n" in lean, name
        if name.endswith("-pair"):
            assert rd(f"{A}/{grammar}/{name}/receipt/outputs/proof.md") == informal_control, "the informal file is the control's"
    report["cases"].append(row)
# every case names the 0.3.0 formal control of its length as its truth's reference
for row in report["cases"]:
    assert row["statement_status"]["reference"] == f"{controls['v0.3.0-control-lemma-formal']}:{MAIN}", row["case"]
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert len(report["cases"]) == 6
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
          "confirmed", r["flaws_confirmed"], r["proof_status"], "axioms_standard", r["lean_axioms_standard"],
          [(x["instance"], x["truth"], x["perturbation"], x["counterexample"], x["use_site"], x["location"]) for x in r["records"]])
