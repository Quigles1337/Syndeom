"""Orchestrate Rust checks, capture actual commands, and compare bytes. No label/count authority."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[2])
parser.add_argument("--out", type=Path, required=True, help="New directory outside the checkout; existing parent required")
args = parser.parse_args()
repo = args.repo.resolve(strict=True)
out = args.out.resolve()
if out == repo or repo in out.parents:
    parser.error("output must be outside the checkout")
out.mkdir()  # Fail before touching existing evidence.

def git(*argv):
    return subprocess.check_output(["git", *argv], cwd=repo).decode().strip()

def hashes(folder):
    return {p.relative_to(folder).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(folder.rglob("*")) if p.is_file()}

receipt = {"started_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
           "checkout_revision": git("rev-parse", "HEAD"), "checkout_tree": git("rev-parse", "HEAD^{tree}"),
           "commands": [], "authority": "Rust s1_effective_inventory; Python only orchestration and byte comparison"}

def run(name, command, expected=0):
    log = out / (name + ".log")
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with log.open("xb") as f:
        completed = subprocess.run([str(x) for x in command], cwd=repo, stdout=f, stderr=subprocess.STDOUT)
    receipt["commands"].append({"name": name, "argv": [str(x) for x in command], "cwd": str(repo),
                                "started_utc": started, "finished_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
                                "exit_code": completed.returncode, "expected_exit": expected,
                                "log_sha256": hashlib.sha256(log.read_bytes()).hexdigest()})
    if (expected == 0 and completed.returncode != 0) or (expected == "nonzero" and completed.returncode == 0):
        raise RuntimeError(f"{name}: unexpected exit {completed.returncode}; see {log}")

try:
    command = ["cargo", "run", "--locked", "--manifest-path", repo / "engine/Cargo.toml", "-p", "syndeom-ingest",
               "--example", "s1_effective_inventory", "--", repo]
    run("report-a", command + [out / "report-a"])
    run("report-b", command + [out / "report-b"])
    first = hashes(out / "report-a")
    if first != hashes(out / "report-b"):
        raise RuntimeError("independent Rust outputs are not byte-identical")
    receipt["replay_hashes"] = first
    run("existing-output-refused", command + [out / "report-a"], expected="nonzero")
    if first != hashes(out / "report-a"):
        raise RuntimeError("refused rerun changed retained outputs")
    receipt["overwrite_refusal_preserved_outputs"] = True
    receipt["passed"] = True
except Exception as error:
    receipt["passed"] = False
    receipt["error"] = str(error)
finally:
    receipt["finished_utc"] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    receipt["head_after"] = git("rev-parse", "HEAD")
    receipt["status_after"] = git("status", "--porcelain=v1", "--untracked-files=all")
    with (out / "receipt.json").open("x", encoding="utf-8", newline="\n") as f:
        json.dump(receipt, f, indent=2)
        f.write("\n")
print(json.dumps({"passed": receipt["passed"], "receipt": str(out / "receipt.json")}))
sys.exit(0 if receipt["passed"] else 1)
