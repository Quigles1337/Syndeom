"""Orchestration and byte provenance only; Rust owns candidate validation and identity."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

BASE = "1a76d9ac21cd5d2e4311f9164178327c0ca128bb"
OLD = "s1_ingest-v0.1.0-63e23a952084"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--repo", type=Path, required=True)
    p.add_argument("--work", type=Path, required=True)
    args = p.parse_args()
    repo, work = args.repo.resolve(), args.work.resolve()
    work.mkdir(parents=True, exist_ok=False)
    packet = repo / "analysis/s1-first-parent-status"
    receipt = {"producer_revision": BASE, "started": datetime.datetime.now(datetime.timezone.utc).isoformat(),
               "helper_sha256": sha(Path(__file__).read_bytes()), "commands": []}

    def run(label, command, cwd=repo):
        command = [str(x) for x in command]
        start = datetime.datetime.now(datetime.timezone.utc).isoformat()
        r = subprocess.run(command, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        log = work / (label + ".log")
        log.write_bytes(r.stdout)
        receipt["commands"].append({"label": label, "command": command, "cwd": str(cwd),
                                    "started": start, "exit_code": r.returncode,
                                    "log": str(log), "log_sha256": sha(r.stdout)})
        if r.returncode:
            raise RuntimeError(f"{label}: exit {r.returncode}; see {log}")
        return r.stdout

    try:
        # Verify the actual unchanged producer sources against the recorded commit, not a
        # caller-supplied SHA string alone. Extra source/build configuration is refused.
        names = run("producer-files", ["git", "ls-tree", "-r", "--name-only", BASE,
                    "engine", "rust-toolchain.toml"]).decode().splitlines()
        receipt["producer_inputs"] = {}
        for name in names:
            before = subprocess.check_output(["git", "show", f"{BASE}:{name}"], cwd=repo)
            actual = (repo / name).read_bytes()
            assert before == actual, f"producer input changed: {name}"
            receipt["producer_inputs"][name] = sha(actual)
        extra = run("extra-engine-files", ["git", "ls-files", "--others", "--exclude-standard", "engine"])
        assert not extra.strip(), "untracked engine inputs"
        for c in [repo / ".cargo/config", repo / ".cargo/config.toml",
                  repo / "engine/.cargo/config", repo / "engine/.cargo/config.toml"]:
            assert not c.exists(), f"unrecorded Cargo configuration: {c}"
        run("rust-version", ["rustc", "-vV"])
        run("cargo-version", ["cargo", "-vV"])
        build = run("build", ["cargo", "build", "--locked", "-p", "syndeom-ingest",
                              "--message-format=json"], repo / "engine")
        artifacts = {}
        binary = None
        for line in build.splitlines():
            if not line.startswith(b"{"):
                continue
            row = json.loads(line)
            if row.get("reason") == "compiler-artifact":
                for file in row["filenames"]:
                    if file.endswith(".rlib"):
                        artifacts[row["target"]["name"]] = file
                if row["target"]["name"] == "syndeom-ingest":
                    binary = row["executable"]
        assert binary
        receipt["producer_binary"] = {"path": binary, "sha256": sha(Path(binary).read_bytes())}
        probe = work / "validate_candidate.exe"
        run("compile-validator", ["rustc", "--edition=2024", "-D", "warnings",
            packet / "validate_candidate.rs", "--extern", "syndeom_manifest=" + artifacts["syndeom_manifest"],
            "--extern", "serde_json=" + artifacts["serde_json"], "-L",
            "dependency=" + str(Path(artifacts["syndeom_manifest"]).parent), "-o", probe])
        receipt["validator_binary_sha256"] = sha(probe.read_bytes())
        source = repo / "groundtruth" / OLD / "original-0706.0357v70.tar.gz"
        outputs = []
        for i in [1, 2]:
            destination = work / f"generation-{i:02}"
            run(f"ingest-{i:02}", [binary, "--config", packet / "candidate-config.json",
                "--bytes", source, "--root", destination, "--engine-commit", BASE])
            manifests = list(destination.glob("groundtruth/*/manifest.json"))
            assert len(manifests) == 1
            outputs.append(manifests[0])
        assert outputs[0].read_bytes() == outputs[1].read_bytes()
        assert (outputs[0].parent / source.name).read_bytes() == (outputs[1].parent / source.name).read_bytes()
        candidate_id = json.loads(outputs[0].read_text())["artifact_id"]
        mapping = {"event_id": "first-parent-status-2026-10-20", "state": "proposed-not-effective",
                   "relation": "raw-metadata-amendment", "logical_item_key": OLD,
                   "old_artifact_id": OLD, "proposed_artifact_id": candidate_id,
                   "effective_now": OLD, "effective_if_accepted_and_integrated": candidate_id,
                   "effective_items_contributed": 1, "corpus_logical_items_now": 32,
                   "corpus_logical_items_if_accepted": 32,
                   "canonical_eligibility": "held-pending-policy-and-integration"}
        map_path = work / "amendment-map.json"
        map_path.write_text(json.dumps(mapping, indent=2) + "\n", encoding="utf-8", newline="\n")
        run("validate-candidate", [probe, repo / "groundtruth" / OLD / "manifest.json", outputs[0],
                                  packet / "WITNESS.md", packet / "candidate-config.json", map_path])
        retained = packet / "candidate" / candidate_id / "manifest.json"
        if retained.exists():
            assert retained.read_bytes() == outputs[0].read_bytes(), "retained candidate differs"
            assert (retained.parent / source.name).read_bytes() == source.read_bytes()
            assert (packet / "amendment-map.json").read_bytes() == map_path.read_bytes()
        receipt.update(result="PASS", candidate_id=candidate_id, candidate_manifest=str(outputs[0]),
                       manifest_sha256=sha(outputs[0].read_bytes()), source_sha256=sha(source.read_bytes()),
                       replay="both generated files byte-identical", mapping_sha256=sha(map_path.read_bytes()),
                       retained_candidate_compared=retained.exists())
    except Exception as e:
        receipt.update(result="FAIL", error=str(e))
        raise
    finally:
        receipt["finished"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
        (work / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({k: receipt[k] for k in ["result", "candidate_id", "manifest_sha256", "replay"]}, indent=2))


if __name__ == "__main__":
    main()
