"""Rehearse candidate evidence in a NEW external directory; never admit an artifact.

Python orchestrates pinned Rust pure APIs and Lean inspection, not manifest generation.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import urllib.request

PACKET = Path(__file__).resolve().parent
ROOT = PACKET.parents[1]
PIN = "d13f23b723b8a846827a245b89c10fc7d3f11612"
DECLARATIONS = (
    "Nat.infinite_setOfPred_prime", "syndeom_s1_2_judson_primes",
    "Nat.prime_def", "Nat.not_bddAbove_setOfPred_prime",
    "Nat.exists_infinite_primes", "Set.infinite_of_not_bddAbove",
)
ALLOWED = {"propext", "Classical.choice", "Quot.sound"}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def utc():
    return datetime.now(timezone.utc).isoformat()


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT).decode().strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--source-dir", type=Path,
                        help="Use retained primary bytes; otherwise retrieve exact published URLs.")
    args = parser.parse_args()
    out = args.output.resolve()
    if out == ROOT or ROOT in out.parents:
        parser.error("output must be outside the repository")
    out.mkdir(parents=True, exist_ok=False)
    receipt = {
        "kind": "candidate-feasibility-only", "started_at": utc(),
        "base_revision": "8adcb4865abd3f6c3ac882b70563d82caca6207c",
        "reviewed_procedure_revision": "c157e4e50b011fd11fc08b31d6963900923d2564",
        "source_map_correction_main_baseline": "1a8388b03017e4044cad01ac10e7e89f8fb5e38f",
        "checkout_revision": git("rev-parse", "HEAD"),
        "index_tree": git("write-tree"), "checkout_status": git("status", "--porcelain=v1"),
        "helper_sha256": digest(Path(__file__).read_bytes()),
        "packet_input_sha256": {
            name: digest((PACKET / name).read_bytes()) for name in
            ("Proof.lean", "inspection-tail.lean", "api_probe.rs",
             "archive-config.proposed.json", "canonical-config.proposed.json", "retrieval.json",
             "source_locator.py")
        },
        "sources": [], "commands": [], "full_manifest_validation": "not run: no producer or manifest",
        "corpus_admission": False,
    }

    def run(label, command, cwd):
        record = {"label": label, "argv": command, "cwd": str(cwd), "started_at": utc()}
        receipt["commands"].append(record)
        print(f"{label}: {' '.join(command)}", flush=True)
        with (out / f"{label}.log").open("xb") as log:
            result = subprocess.run(command, cwd=cwd, stdout=log, stderr=subprocess.STDOUT)
        data = (out / f"{label}.log").read_bytes()
        record.update({"exit_code": result.returncode, "completed_at": utc(),
                       "log_sha256": digest(data)})
        if result.returncode:
            raise RuntimeError(f"{label} failed: exit {result.returncode}; see its new log")
        return data.decode("utf-8")

    try:
        assert (ROOT / "lean/lean-toolchain").read_text().strip() == "leanprover/lean4:v4.34.1"
        mathlib = ROOT / "lean/.lake/packages/mathlib"
        assert subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=mathlib).decode().strip() == PIN
        for entry in json.loads((PACKET / "retrieval.json").read_text()):
            if entry["file"] == "judson-landing.html":
                continue  # Dynamic discovery page is not a replay input.
            name = entry["file"]
            started = utc()
            if args.source_dir:
                data = (args.source_dir / name).read_bytes()
                retrieval = {"method": "retained bytes", "path": str((args.source_dir / name).resolve())}
            else:
                with urllib.request.urlopen(entry["requested_url"], timeout=60) as response:
                    data = response.read()
                    retrieval = {"method": "HTTP", "status": response.status, "url": response.url}
            assert digest(data) == entry["sha256"], f"source hash drift: {name}"
            assert len(data) == entry["n_bytes"], f"source size drift: {name}"
            (out / name).write_bytes(data)
            receipt["sources"].append({"file": name, "sha256": digest(data),
                                       "started_at": started, "completed_at": utc(), **retrieval})
        receipt["mathlib_source_comparison"] = []
        for filename, local in {
            "PrimeFin.lean": "Mathlib/Data/Nat/PrimeFin.lean",
            "Infinite.lean": "Mathlib/Data/Nat/Prime/Infinite.lean",
            "PrimeDefs.lean": "Mathlib/Data/Nat/Prime/Defs.lean",
            "mathlib-LICENSE": "LICENSE",
        }.items():
            blob = subprocess.check_output(["git", "show", f"{PIN}:{local}"], cwd=mathlib)
            worktree = (mathlib / local).read_bytes()
            assert (out / filename).read_bytes() == blob, f"download differs from pinned Git blob: {local}"
            assert worktree == blob or worktree.replace(b"\r\n", b"\n") == blob, f"modified Mathlib input: {local}"
            receipt["mathlib_source_comparison"].append({
                "path": local, "pinned_blob_sha256": digest(blob), "worktree_sha256": digest(worktree),
                "checkout_eol": "exact blob" if worktree == blob else "CRLF checkout of LF blob",
            })
        config = json.loads((PACKET / "canonical-config.proposed.json").read_text())
        recipe = config["recipe"]
        informal = (f"# {recipe['title']}\n\n**Theorem.** {recipe['statement']}\n\n"
                    "**Proof.**\n\n∎\n").encode("utf-8")
        (out / "proof.md").write_bytes(informal)
        formal = (PACKET / "Proof.lean").read_bytes()
        assert digest(informal) == recipe["informal_sha256"]
        assert digest(formal) == recipe["formal_sha256"]
        spec = importlib.util.spec_from_file_location("source_locator", PACKET / "source_locator.py")
        locator = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(locator)
        receipt["source_locator_verification"] = locator.check_with_controls(
            (out / "PrimeFin.lean").read_bytes(), formal,
            json.loads((PACKET / "archive-config.proposed.json").read_text()), config)
        (out / "Proof.lean").write_bytes(formal)
        audit = formal + (PACKET / "inspection-tail.lean").read_bytes()
        (out / "CandidateAudit.lean").write_bytes(audit)
        receipt["candidate_bytes"] = {
            "proof.md": {"sha256": digest(informal), "n_bytes": len(informal)},
            "Proof.lean": {"sha256": digest(formal), "n_bytes": len(formal)},
            "CandidateAudit.lean": {"sha256": digest(audit), "n_bytes": len(audit)},
        }
        lake = shutil.which("lake")
        assert lake, "lake not on PATH"
        audit_output = run("candidate-audit", [lake, "env", "lean", str(out / "CandidateAudit.lean")],
                           ROOT / "lean")
        audited = {}
        for declaration in DECLARATIONS:
            matches = re.findall(r"'" + re.escape(declaration) + r"' depends on axioms:\s*\[([^\]]*)\]",
                                 audit_output)
            assert len(matches) == 1, f"missing/duplicate axiom output: {declaration}"
            axioms = [a.strip() for a in matches[0].split(",") if a.strip()]
            constants = [re.sub(r"\.\{[^{}]+\}$", "", a) for a in axioms]
            assert set(constants) <= ALLOWED, f"nonstandard axiom: {declaration}: {axioms}"
            audited[declaration] = {"printed": axioms, "constant_names": constants}
        receipt["axioms"] = audited
        cargo = shutil.which("cargo")
        rustc = shutil.which("rustc")
        assert cargo and rustc, "pinned Rust tools not on PATH"
        messages = run("manifest-library-build",
                       [cargo, "build", "--locked", "-p", "syndeom-manifest", "--message-format=json"],
                       ROOT / "engine")
        artifacts = [json.loads(line) for line in messages.splitlines() if line.startswith("{")]
        rlibs = [path for m in artifacts if m.get("reason") == "compiler-artifact"
                 and m["target"]["name"] == "syndeom_manifest"
                 for path in m["filenames"] if path.endswith(".rlib")]
        assert len(rlibs) == 1, "ambiguous library artifact"
        exe = out / ("api-probe.exe" if sys.platform == "win32" else "api-probe")
        dependency_dir = Path(rlibs[0]).parent
        if dependency_dir.name != "deps":
            dependency_dir = dependency_dir / "deps"
        run("compile-api-probe", [rustc, "--edition=2024", str(PACKET / "api_probe.rs"),
                                  "--extern", f"syndeom_manifest={rlibs[0]}",
                                  "-L", f"dependency={dependency_dir}", "-o", str(exe)], ROOT)
        api_output = run("api-probe", [str(exe), str(out / "proof.md"), str(out / "Proof.lean"),
                                      str(PACKET / "archive-config.proposed.json"),
                                      str(PACKET / "canonical-config.proposed.json")], ROOT)
        receipt["api_output"] = api_output
        receipt["result"] = "candidate checks passed; not admission"
        receipt["exit_code"] = 0
    except Exception as exc:
        receipt.update({"result": "failed", "error": str(exc), "exit_code": 1})
    finally:
        receipt["completed_at"] = utc()
        with (out / "receipt.json").open("x", encoding="utf-8", newline="\n") as handle:
            json.dump(receipt, handle, indent=2)
            handle.write("\n")
    print(json.dumps(receipt, indent=2))
    return receipt["exit_code"]


if __name__ == "__main__":
    sys.exit(main())
