"""Confirm two production runs of the s2_grammar 0.7.0 goldens are byte-identical, check the shape
of every case (the controls build with standard axioms and are the 0.1.0 short control's bytes;
each FLAW-0011 case is the composite of one FLAW-0001 `propagated` `sign` instance — its proof
files the 0.1.0 FLAW-0001 `propagated` golden's byte for byte and its embedded records that
golden's records, then the composite's records at the statement, one per file, fatal and
YES-KERNEL by inheritance; an invalid proof of a true statement; two instances, both confirmed;
in a formal file the build failing with its first error at the embedded site's line), then place
run A's outputs under engine/generate/tests/goldens/ with a case.json each. Usage:
python -I -X utf8 goldens-copy14.py <run A base> <run B base> <report path>"""
import hashlib
import json
import os
import shutil
import sys

A, B, REPORT = sys.argv[1], sys.argv[2], sys.argv[3]
DEST = "C:/Users/LEET/Syndeom/engine/generate/tests/goldens"
MAIN = "syndeom_s2_grammar_ring_chain"
STANDARD = {"propext", "Classical.choice", "Quot.sound"}
EMBEDDED = [{"class": "FLAW-0001", "sub_variant": "propagated", "severity": "fatal", "site": "auto",
             "dials": {"perturbation": "sign"}}]


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


def theorem_text(md):
    """The informal file's statement text (syndeom-informal-md/1: `**Theorem.** <text>`)."""
    for para in md.split("\n\n"):
        if para.startswith("**Theorem.** "):
            return para[len("**Theorem.** "):]
    raise AssertionError("no theorem paragraph")


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
                       "location": r["location"], "site": r["dials"].get("site"),
                       "embedded_instances": r["dials"].get("embedded_instances"),
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
    assert manifest["generator"] == {"id": "s2_grammar", "version": "0.7.0", "rng": "syndeom-rng/1"}, manifest["generator"]
    assert gt["statement_status"]["value"] is True and gt["statement_status"]["method"] == "lean-kernel", gt
    assert gt["statement_status"]["reference"].endswith(f":{MAIN}"), gt
    obs = row["observation"]
    if row["n_steps"] is not None:
        assert row["n_lemmas"] == 0 and row["n_steps"] == 4, row
    target = name.rsplit("-", 1)[1]
    if "control" in name:
        assert gt["flaws"] == [] and gt["proof_status"] == "valid" and gt["correct"] is True, name
        # the bytes are the 0.1.0 short control's
        old = f"{DEST}/{grammar}/control-short-{target}"
        for fn in ("proof.md", "Proof.lean"):
            if os.path.exists(f"{A}/{grammar}/{name}/receipt/outputs/{fn}"):
                assert rd(f"{A}/{grammar}/{name}/receipt/outputs/{fn}") == rd(f"{old}/{fn}"), (name, fn)
        if obs is not None:
            assert obs["exit_code"] == 0, name
            assert obs["axioms"] and set(obs["axioms"]) <= STANDARD, obs
            assert row["commanded_lean_build"] == "pass", row
            assert row["lean_axioms_standard"] == {"commanded": True, "achieved": True}, row
        if name.endswith("-formal"):
            controls[name] = row["artifact_id"]
            assert gt["statement_status"]["reference"] == f"{row['artifact_id']}:{MAIN}", "the formal control names itself"
        if name.endswith("-informal"):
            assert obs is None
    if "flaw-0011" in name:
        # an invalid proof of a true statement: the embedded instance is fatal (D7)
        assert gt["proof_status"] == "invalid" and gt["correct"] is False, name
        # the proof files and the embedded records are the 0.1.0 FLAW-0001 propagated golden's
        old = f"{DEST}/{grammar}/flaw-0001-propagated-{target}"
        for fn in ("proof.md", "Proof.lean"):
            assert os.path.exists(f"{A}/{grammar}/{name}/receipt/outputs/{fn}") == os.path.exists(f"{old}/{fn}"), (name, fn)
            if os.path.exists(f"{old}/{fn}"):
                assert rd(f"{A}/{grammar}/{name}/receipt/outputs/{fn}") == rd(f"{old}/{fn}"), (name, fn)
        old_flaws = json.load(open(f"{old}/manifest.json", encoding="utf-8"))["ground_truth"]["flaws"]
        files = int(target in ("informal", "pair")) + int(target in ("formal", "pair"))
        assert len(old_flaws) == files and len(gt["flaws"]) == 2 * files, (name, len(gt["flaws"]))
        assert gt["flaws"][:files] == old_flaws, (name, "the embedded records are the standalone golden's")
        for r in gt["flaws"][:files]:
            assert r["instance"] == 1 and r["class"] == "FLAW-0001" and r["sub_variant"] == "propagated", r
            assert r["severity"] == "fatal" and r["lean_catchable"] == "YES-KERNEL", r
        md = None
        if os.path.exists(f"{A}/{grammar}/{name}/receipt/outputs/proof.md"):
            md = rd(f"{A}/{grammar}/{name}/receipt/outputs/proof.md").decode("utf-8")
        lean = None
        if os.path.exists(f"{A}/{grammar}/{name}/receipt/outputs/Proof.lean"):
            lean = rd(f"{A}/{grammar}/{name}/receipt/outputs/Proof.lean").decode("utf-8").split("\n")
        seen = []
        for r in gt["flaws"][files:]:
            assert r["instance"] == 2 and r["class"] == "FLAW-0011" and r["sub_variant"] is None, r
            assert r["severity"] == "fatal" and r["lean_catchable"] == "YES-KERNEL", r
            d = r["dials"]
            assert d["embedded"] == EMBEDDED and d["site"] == 0 and d["embedded_instances"] == [1], d
            assert d["replaced"] is None, d
            assert set(d) == {"embedded", "site", "embedded_instances", "inserted", "replaced"}, d
            loc = r["location"]
            if loc["artifact"] == "informal":
                text = theorem_text(md)
                assert loc["step"] == 0 and loc["span"] == [0, len(text)], loc
                assert d["inserted"] == text, d
            else:
                assert loc["artifact"] == "formal" and loc["declaration"] == MAIN, loc
                header = lean[loc["line"] - 1]
                assert header.startswith(f"theorem {MAIN} ") and d["inserted"] == [header], (loc, d)
            seen.append(loc["artifact"])
        assert seen == (["informal"] if target == "informal" else ["formal"] if target == "formal" else ["informal", "formal"]), seen
        assert row["n_flaws"] == {"n_flaws_0001": {"commanded": 1, "achieved": 1},
                                  "n_flaws_0011": {"commanded": 1, "achieved": 1},
                                  "n_flaws_total": {"commanded": 2, "achieved": 2}}, row["n_flaws"]
        assert row["flaws_confirmed"] == 2, row
        if target == "informal":
            assert obs is None and row["commanded_lean_build"] is None, row
        else:
            # the build fails with its first error at the embedded site's calc line (check K),
            # no axiom print; the composite's own formal record is at the header line
            formal_embedded = [r for r in gt["flaws"][:files] if r["location"]["artifact"] == "formal"]
            assert len(formal_embedded) == 1, formal_embedded
            assert obs is not None and obs["exit_code"] != 0, obs
            assert obs["first_error_line"] == formal_embedded[0]["location"]["line"], (obs, formal_embedded)
            assert obs["axioms"] is None, obs
            assert row["commanded_lean_build"] == "fail" and row["achieved_lean_build"] == "fail", row
            assert row["lean_axioms_standard"] == {"commanded": None, "achieved": None}, row
    report["cases"].append(row)
# every case names the 0.7.0 formal control of its length as its truth's reference
for row in report["cases"]:
    assert row["statement_status"]["reference"] == f"{controls['v0.7.0-control-short-formal']}:{MAIN}", row["case"]
commits = {r["engine_commit"] for r in report["cases"]}
report["engine_commits"] = sorted(commits)
assert report["all_byte_identical"], "the two runs differ"
assert len(commits) == 1, commits
assert all(r["same_observation"] for r in report["cases"])
assert len(report["cases"]) == 5
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
          "n_flaws", r["n_flaws"], "confirmed", r["flaws_confirmed"], r["proof_status"], "lean_build",
          r["commanded_lean_build"], r["achieved_lean_build"],
          [(x["instance"], x["class"], x["lean_catchable"], x["location"]) for x in r["records"]])
