"""Confirm two production runs of the s2_grammar 0.4.0 goldens are byte-identical, check the
shape of every case (the controls build with standard axioms; each FLAW-0010 `a` case carries one
informal-only N/A instance of severity gap spanning the fictional reference, the step being
`By <reference>` and the bare claim, the formal file of a pair the formal control's byte for byte,
`lean_axioms_standard` commanded and achieved true), then place run A's outputs under
engine/generate/tests/goldens/ with a case.json each. Usage:
python -I -X utf8 goldens-copy11.py <run A base> <run B base> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
MAIN = "syndeom_s2_grammar_ring_chain"
STANDARD = {"propext", "Classical.choice", "Quot.sound"}
AUTHORS = ["Placeholder", "Nonesuch", "Fictitious", "Hypothetical"]
YEARS = ["19XX", "20XX"]
VENUES_JOURNAL = ["Journal of Nonexistent Mathematics", "Annals of Imaginary Results",
                  "Fictitious Mathematical Quarterly"]
RESULTS = ["Theorem 4.2", "Lemma 3.1", "Proposition 2.5", "Corollary 1.7"]


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
                       "location": r["location"], "shape": r["dials"].get("shape"),
                       "claim": r["dials"].get("claim"), "reference": r["dials"].get("reference"),
                       "indices": {k: r["dials"].get(k) for k in ("author", "year", "venue", "result")},
                       "asserted": r["dials"].get("asserted"), "justification": r["dials"].get("justification")}
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
    assert manifest["generator"] == {"id": "s2_grammar", "version": "0.4.0", "rng": "syndeom-rng/1"}, manifest["generator"]
    assert gt["statement_status"]["value"] is True and gt["statement_status"]["method"] == "lean-kernel", gt
    assert gt["statement_status"]["reference"].endswith(f":{MAIN}"), gt
    obs = row["observation"]
    if obs is not None:
        # every 0.4.0 golden with a formal file builds with the standard axioms: the class
        # has no formal manifestation
        assert obs["exit_code"] == 0, name
        assert obs["axioms"] and set(obs["axioms"]) <= STANDARD, obs
        assert row["commanded_lean_build"] == "pass", row
        assert row["lean_axioms_standard"] == {"commanded": True, "achieved": True}, row
    if "control" in name:
        assert gt["flaws"] == [] and gt["proof_status"] == "valid" and gt["correct"] is True, name
        if row["n_steps"] is not None:
            assert row["n_lemmas"] == 0 and row["n_steps"] == 4, row
        if name.endswith("-formal"):
            controls[name] = row["artifact_id"]
            assert gt["statement_status"]["reference"] == f"{row['artifact_id']}:{MAIN}", "the formal control names itself"
            formal_control = rd(f"{A}/{grammar}/{name}/receipt/outputs/Proof.lean")
        if name.endswith("-informal"):
            informal_control = rd(f"{A}/{grammar}/{name}/receipt/outputs/proof.md")
    if "flaw-0010" in name:
        assert gt["proof_status"] == "invalid" and gt["correct"] is False, name
        assert len(gt["flaws"]) == 1, "FLAW-0010 manifests in the informal file only"
        r = gt["flaws"][0]
        assert r["instance"] == 1 and r["class"] == "FLAW-0010" and r["sub_variant"] == "a", r
        assert r["severity"] == "gap" and r["lean_catchable"] == "N/A", r
        assert r["location"]["artifact"] == "informal", r
        d = r["dials"]
        site = d["site"]
        assert r["location"]["step"] == site and 1 <= site <= 4, r
        assert d["shape"] == "journal" and d["claim"] is True, d
        reference = f"[{AUTHORS[d['author']]} {YEARS[d['year']]}, {VENUES_JOURNAL[d['venue']]}, {RESULTS[d['result']]}]"
        assert d["reference"] == reference and d["inserted"] == reference, d
        assert d["replaced"] == d["justification"] and d["justification"], d
        assert set(d["original_steps"]) == {str(site)}, d
        original = d["original_steps"][str(site)]
        assert original == f"{d['justification']}, {d['asserted']}", (original, d)
        assert r["location"]["span"] == [3, 3 + len(reference)], r
        assert row["n_flaws_total"] == {"commanded": 1, "achieved": 1} and row["flaws_confirmed"] == 1, row
        assert row["n_lemmas"] == 0 and row["n_steps"] == 4, row
        # the informal file: the site step is `By <reference>` and the bare claim; every other
        # step the informal control's
        md = rd(f"{A}/{grammar}/{name}/receipt/outputs/proof.md").decode("utf-8")
        steps = steps_of(md)
        control_steps = steps_of(informal_control.decode("utf-8"))
        assert set(steps) == set(control_steps) == {1, 2, 3, 4}, steps
        assert steps[site] == f"By {reference} {d['asserted']}", steps[site]
        assert control_steps[site] == original, (control_steps[site], original)
        for k in steps:
            if k != site:
                assert steps[k] == control_steps[k], k
        if name.endswith("-pair"):
            assert obs is not None
            assert rd(f"{A}/{grammar}/{name}/receipt/outputs/Proof.lean") == formal_control, "the formal file is the control's"
        else:
            assert obs is None
    report["cases"].append(row)
# every case names the 0.4.0 formal control of its length as its truth's reference
for row in report["cases"]:
    assert row["statement_status"]["reference"] == f"{controls['v0.4.0-control-short-formal']}:{MAIN}", row["case"]
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
    print(r["case"], r["verdict"], r["artifact_id"], "exit", obs.get("exit_code"), "axioms", obs.get("axioms"),
          "n_steps", r["n_steps"], "n_lemmas", r["n_lemmas"], "n_tokens", r["n_tokens"], "n_flaws", r["n_flaws_total"],
          "confirmed", r["flaws_confirmed"], r["proof_status"], "axioms_standard", r["lean_axioms_standard"],
          [(x["instance"], x["shape"], x["claim"], x["reference"], x["indices"], x["location"]) for x in r["records"]])
