"""Run the committed ADR-0007 producer and admission checks into NEW evidence space.

Default artifact destination is isolated replay space. --artifact-root may explicitly
name the checkout for the authorized proposed admission. No accepted-main claim.
"""
import argparse
import copy
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import urllib.request

ARCHIVE_ID = "s1_pair_archive-v0.1.0-094625fa10ff"
CHILD_ID = "s1_pair-v0.1.0-62fe0c816a6b"
BASE = "1a8388b03017e4044cad01ac10e7e89f8fb5e38f"
CORRECTION = "d88fad2bb7b339c56bd9964167d3e0acddc8ee4e"
DECLS = ("Nat.infinite_setOfPred_prime", "syndeom_s1_2_judson_primes", "Nat.prime_def",
         "Nat.not_bddAbove_setOfPred_prime", "Nat.exists_infinite_primes", "Set.infinite_of_not_bddAbove")
EXPECTED_TYPE = "@Set.Infinite.{0} Nat (@Set.ofPred.{0} Nat fun (p : Nat) => Nat.Prime p)"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def utc():
    return datetime.now(timezone.utc).isoformat()


def write_json(path, value):
    with path.open("x", encoding="utf-8", newline="\n") as f:
        json.dump(value, f, indent=2, ensure_ascii=False)
        f.write("\n")


def snapshot(directory, timestamps=False):
    return {p.relative_to(directory).as_posix(): {
        "sha256": sha(p.read_bytes()), "n_bytes": p.stat().st_size,
        **({"mtime_ns": p.stat().st_mtime_ns} if timestamps else {})
    } for p in sorted(directory.rglob("*")) if p.is_file()}


def artifacts(root, external, timestamps=False):
    result = {}
    for item in (ARCHIVE_ID, CHILD_ID):
        for prefix, directory in [("metadata", root / "groundtruth" / item),
                                  ("external", external / item)]:
            for name, info in snapshot(directory, timestamps).items():
                # RECORD is human admission evidence, not generated deterministic artifact bytes.
                if name != "RECORD.md":
                    result[f"{prefix}/{item}/{name}"] = info
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--producer", required=True)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--source-dir", type=Path)
    args = parser.parse_args()
    root, out = args.root.resolve(), args.output.resolve()
    assert out != root and root not in out.parents, "evidence output must be external"
    out.mkdir(parents=True, exist_ok=False)
    packet = root / "analysis/s1-2"
    artifact_root = args.artifact_root.resolve() if args.artifact_root else out / "artifacts"
    if not args.artifact_root:
        artifact_root.mkdir()
    assert artifact_root.is_dir()
    external = out / "external"
    external.mkdir()
    (out / "receipts").mkdir()
    (out / "logs").mkdir()
    source_dir = out / "sources"
    source_dir.mkdir()
    receipt = {
        "kind": "actual producer generation and proposed-admission verification",
        "main_baseline": BASE, "corrected_procedure_revision": CORRECTION,
        "producer_revision": args.producer, "started_at": utc(),
        "helper_sha256": sha(Path(__file__).read_bytes()),
        "checkout_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root).decode().strip(),
        "artifact_root": str(artifact_root), "external_root": str(external),
        "commands": [], "sources": [], "refusal_controls": [], "accepted_main_admission": False,
    }

    def run(label, command, cwd=root, expected=0, classification="positive"):
        record = {"label": label, "argv": [str(x) for x in command], "cwd": str(cwd),
                  "started_at": utc(), "expected_exit": expected, "classification": classification}
        receipt["commands"].append(record)
        print(f"{label}: {' '.join(record['argv'])}", flush=True)
        log = out / "logs" / f"{label}.log"
        with log.open("xb") as f:
            p = subprocess.run(command, cwd=cwd, stdout=f, stderr=subprocess.STDOUT)
        data = log.read_bytes()
        record.update({"exit_code": p.returncode, "completed_at": utc(),
                       "raw_log_sha256": sha(data), "log": str(log)})
        if p.returncode != expected:
            raise RuntimeError(f"{label}: expected exit {expected}, got {p.returncode}; {log}")
        return data.decode("utf-8", errors="replace")

    exe = root / "engine/target/debug" / ("syndeom-pair.exe" if sys.platform == "win32" else "syndeom-pair")

    def cli(label, mode, ar=artifact_root, ex=external, config=None, sources=None,
            dry=False, producer=None, expected=0, receipt_path=None):
        cfg = config or packet / f"{'archive' if mode == 'archive' else 'canonical'}-config.proposed.json"
        dest = receipt_path or out / "receipts" / label
        command = [str(exe), "--mode", mode, "--config", str(cfg), "--root", str(root),
                   "--artifact-root", str(ar), "--external-dir", str(ex),
                   "--engine-commit", producer or args.producer, "--receipt-dir", str(dest)]
        if mode == "archive":
            command += ["--sources-dir", str(sources or source_dir)]
        if dry:
            command += ["--dry-run"]
        run(label, command, expected=expected,
            classification="controlled refusal" if expected else "positive")
        return dest

    try:
        run("producer-build", ["cargo", "build", "--locked", "-p", "syndeom-pair"], root / "engine")
        receipt["executable_sha256"] = sha(exe.read_bytes())
        receipt["rust_version"] = run("rust-version", ["rustc", "--version"]).strip()
        receipt["lean_version"] = run("lean-version", ["lake", "env", "lean", "--version"], root / "lean").strip()
        for entry in json.loads((packet / "retrieval.json").read_text()):
            if entry["file"] == "judson-landing.html":
                continue
            start = utc()
            if args.source_dir:
                path = args.source_dir / entry["file"]
                data = path.read_bytes()
                details = {"method": "retained exact bytes", "path": str(path.resolve())}
            else:
                with urllib.request.urlopen(entry["requested_url"], timeout=60) as response:
                    data = response.read()
                    details = {"method": "HTTP", "status": response.status, "final_url": response.url}
            assert sha(data) == entry["sha256"] and len(data) == entry["n_bytes"], entry["file"]
            (source_dir / entry["file"]).write_bytes(data)
            receipt["sources"].append({**entry, **details, "verification_started": start,
                                       "verification_completed": utc()})
        for name in ("archive-config.proposed.json", "canonical-config.proposed.json",
                     "Proof.lean", "inspection-tail.lean"):
            receipt.setdefault("packet_input_sha256", {})[name] = sha((packet / name).read_bytes())
        cli("archive", "archive")
        cli("canonical", "canonical")
        emitted = artifact_root / "groundtruth" / CHILD_ID / "Proof.lean"
        run("emitted-file-compile", ["lake", "env", "lean", str(emitted)], root / "lean")
        actual = emitted.read_bytes()
        audit = out / "EmittedAudit.lean"
        audit.write_bytes(actual + (packet / "inspection-tail.lean").read_bytes())
        output = run("emitted-type-axiom-audit", ["lake", "env", "lean", str(audit)], root / "lean")
        types = {}
        for decl in DECLS[:2]:
            matches = [s.rstrip("\r") for s in re.findall(
                r"(?m)^" + re.escape(decl) + r" : ([^\n]+)", output)]
            assert matches == [EXPECTED_TYPE], (decl, matches)
            types[decl] = matches[0]
        axioms = {}
        for decl in DECLS:
            matches = re.findall(r"'" + re.escape(decl) + r"' depends on axioms:\s*\[([^\]]*)\]", output)
            assert len(matches) == 1, decl
            printed = [x.strip() for x in matches[0].split(",") if x.strip()]
            names = [re.sub(r"\.\{[^{}]+\}$", "", x) for x in printed]
            assert set(names) <= {"propext", "Classical.choice", "Quot.sound"}, (decl, names)
            axioms[decl] = {"printed": printed, "constant_names": names}
        receipt["emitted_audit"] = {"actual_file_sha256": sha(actual), "audit_sha256": sha(audit.read_bytes()),
                                    "types": types, "axioms": axioms, "result": "PASS"}
        # Honest controlled Lean subprocess failure, distinct from the actual emitted proof.
        failure = out / "ControlledLeanFailure.lean"
        failure.write_bytes(actual + b"\nexample : False := by\n  exact True.intro\n")
        run("controlled-lean-failure", ["lake", "env", "lean", str(failure)], root / "lean",
            expected=1, classification="synthetic subprocess failure fixture; not admitted")
        assert emitted.read_bytes() == actual
        before = artifacts(artifact_root, external, timestamps=True)
        cli("archive-identical-retry", "archive")
        cli("canonical-identical-retry", "canonical")
        cli("canonical-existing-dry-run", "canonical", dry=True)
        assert artifacts(artifact_root, external, timestamps=True) == before
        receipt["identical_retry_and_dry_run_preserve_bytes_and_mtimes"] = True
        dry_root, dry_external = out / "dry-artifacts", out / "dry-external"
        dry_root.mkdir()
        dry_external.mkdir()
        cli("archive-new-dry-run", "archive", ar=dry_root, ex=dry_external, dry=True)
        assert snapshot(dry_root) == snapshot(dry_external) == {}
        replay_root, replay_external = out / "replay-artifacts", out / "replay-external"
        replay_root.mkdir()
        replay_external.mkdir()
        cli("replay-archive", "archive", ar=replay_root, ex=replay_external)
        cli("replay-canonical", "canonical", ar=replay_root, ex=replay_external)
        expected_artifacts = artifacts(artifact_root, external)
        assert artifacts(replay_root, replay_external) == expected_artifacts
        receipt["fresh_replay"] = {"result": "byte-for-byte identical", "files": expected_artifacts,
                                   "deterministic_file_count": len(expected_artifacts),
                                   "receipt_differences": "timestamps, directories and run commands are intentionally outside artifact identity"}
        controls = out / "controls"
        controls.mkdir()

        def fixture(name):
            directory = controls / name
            ar, ex = directory / "artifacts", directory / "external"
            ar.mkdir(parents=True)
            ex.mkdir()
            for item in (ARCHIVE_ID, CHILD_ID):
                shutil.copytree(artifact_root / "groundtruth" / item, ar / "groundtruth" / item)
                shutil.copytree(external / item, ex / item)
            return directory, ar, ex

        def rejection(name, mutate, mode="canonical", dry=False, producer=None):
            directory, ar, ex = fixture(name)
            config = json.loads((packet / f"{'archive' if mode == 'archive' else 'canonical'}-config.proposed.json").read_text())
            src = directory / "sources"
            if mode == "archive":
                shutil.copytree(source_dir, src)
            mutate(directory, ar, ex, src, config)
            cfg = directory / "config.json"
            write_json(cfg, config)
            before_files = snapshot(directory, timestamps=True)
            dest = cli(name, mode, ar=ar, ex=ex, config=cfg, sources=src,
                       dry=dry, producer=producer, expected=2)
            assert snapshot(directory, timestamps=True) == before_files, f"failed run changed fixture: {name}"
            failure_receipt = json.loads((dest / "receipt.json").read_text())
            assert failure_receipt["exit_code"] == 2 and failure_receipt["result"] == "failed"
            receipt["refusal_controls"].append({"name": name, "exit_code": 2, "preservation": "exact bytes and mtimes",
                "reason": failure_receipt["error"], "receipt": str(dest / "receipt.json"),
                "classification": "development mutation fixture, not corpus data"})

        def set_recipe(key, value):
            return lambda d, ar, ex, src, cfg: cfg["recipe"].__setitem__(key, value)

        rejection("wrong-declaration", set_recipe("formal_declaration", "Nat.other"))
        rejection("wrong-type", set_recipe("formal_type", "True"))
        rejection("invented-informal-proof", set_recipe("informal_proof_steps", ["invented"]))
        rejection("unknown-field", lambda d, ar, ex, src, cfg: cfg.__setitem__("unknown", True))
        rejection("unsupported-method", lambda d, ar, ex, src, cfg: cfg.__setitem__("method", "unsupported/1"))
        rejection("wrong-parent-link", lambda d, ar, ex, src, cfg: cfg.__setitem__("derived_from", "wrong"))
        rejection("wrong-parent-config-hash", lambda d, ar, ex, src, cfg: cfg.__setitem__("parent_config_sha256", "0" * 64))
        rejection("source-line-regression", lambda d, ar, ex, src, cfg:
                  cfg["recipe"]["source_map"]["formal"].__setitem__("source_lines_one_based", [28, 29]))
        rejection("missing-source", lambda d, ar, ex, src, cfg: (src / "PrimeFin.lean").unlink(), mode="archive")
        rejection("altered-formal-source", lambda d, ar, ex, src, cfg:
                  (src / "PrimeFin.lean").write_bytes((src / "PrimeFin.lean").read_bytes() + b"\n"), mode="archive")

        def rehash_substitution(d, ar, ex, src, cfg):
            wrong = b"controlled replacement PDF fixture\n"
            (src / "aata-20250801.pdf").write_bytes(wrong)
            cfg["files"]["informal"]["sha256"] = sha(wrong)
            cfg["files"]["informal"]["n_bytes"] = len(wrong)
        rejection("substituted-source-with-matching-caller-hash", rehash_substitution, mode="archive")

        def parent_metadata(d, ar, ex, src, cfg):
            path = ar / "groundtruth" / ARCHIVE_ID / "manifest.json"
            parent = json.loads(path.read_text())
            parent["ground_truth"]["statement_status"]["reference"] = "unreviewed metadata"
            path.write_text(json.dumps(parent, indent=2) + "\n", encoding="utf-8", newline="\n")
        rejection("changed-parent-metadata", parent_metadata)
        rejection("changed-parent-source-map", lambda d, ar, ex, src, cfg:
                  (ar / "groundtruth" / ARCHIVE_ID / "source-map.json").write_bytes(b"{}\n"))
        rejection("missing-actual-parent-file", lambda d, ar, ex, src, cfg:
                  (ex / ARCHIVE_ID / "PrimeFin.lean").unlink())
        rejection("changed-actual-parent-bytes", lambda d, ar, ex, src, cfg:
                  (ex / ARCHIVE_ID / "PrimeFin.lean").write_bytes(b"changed\n"))

        def collision(d, ar, ex, src, cfg):
            (ar / "groundtruth" / CHILD_ID / "source-map.json").unlink()
            (ar / "groundtruth" / CHILD_ID / "Proof.lean").write_bytes(b"historical collision fixture\n")
        rejection("late-output-collision", collision)
        rejection("late-output-collision-dry-run", collision, dry=True)
        rejection("producer-before-implementation", lambda *a: None, mode="archive", producer=CORRECTION)
        # Reusing the retained receipt must refuse before any artifact or receipt modification.
        retained = out / "receipts/archive"
        prior_receipt = (retained / "receipt.json").read_bytes()
        prior = artifacts(artifact_root, external, timestamps=True)
        cli("existing-receipt-refused", "archive", receipt_path=retained, expected=2)
        assert (retained / "receipt.json").read_bytes() == prior_receipt
        assert artifacts(artifact_root, external, timestamps=True) == prior
        receipt["retained_receipt_overwrite_refused"] = True
        receipt["result"] = "PASS: generation, actual-byte validation, emitted Lean audit, replay and controls; draft admission only"
        receipt["exit_code"] = 0
    except Exception as exc:
        receipt.update({"result": "FAILED", "error": str(exc), "exit_code": 1})
    finally:
        receipt["completed_at"] = utc()
        write_json(out / "verification-receipt.json", receipt)
    print(json.dumps({k: receipt[k] for k in ("result", "exit_code", "completed_at")}), flush=True)
    return receipt["exit_code"]


if __name__ == "__main__":
    sys.exit(main())
