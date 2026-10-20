"""Orchestrate the single ADR-0006 example; Rust alone produces canonical artifacts.

Uses the standard library. All downloaded/reconstructed source text stays in a new external
directory. Run from a checkout of the S1.3 evidence commit or a later compatible checkout.
"""
import argparse
import hashlib
import json
import subprocess
import urllib.request
from datetime import datetime, timezone
from pathlib import Path


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work-dir", type=Path, required=True,
                        help="new directory outside the repository; refuses an existing path")
    parser.add_argument("--engine-commit",
                        help="producer commit; defaults to analysis/s1-3/real-result.json")
    parser.add_argument("--source", type=Path,
                        help="optional already retrieved original gzip; still hash-checked")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    work = args.work_dir.resolve()
    if work == root or root in work.parents:
        parser.error("work-dir must be outside the repository")
    if work.exists():
        parser.error("work-dir already exists; choose a fresh destination")
    commit = args.engine_commit
    if not commit:
        result = root / "analysis/s1-3/real-result.json"
        if not result.exists():
            parser.error("pass --engine-commit after committing the tested producer")
        commit = json.loads(result.read_text(encoding="utf-8"))["producer_commit"]
    parent_path = root / "groundtruth/s1_ingest-v0.1.0-00f28e252823/manifest.json"
    parent = json.loads(parent_path.read_text(encoding="utf-8"))
    expected = parent["artifact"]["files"]["informal"]
    work.mkdir(parents=True)
    url = "https://arxiv.org/e-print/1209.2022v2"
    access = datetime.now(timezone.utc).isoformat()
    if args.source:
        raw = args.source.read_bytes()
        retrieval = {"source_file": str(args.source.resolve()), "verified_utc": access}
    else:
        request = urllib.request.Request(url, headers={"User-Agent": "SYNDEOM ADR-0006 reproduction"})
        with urllib.request.urlopen(request, timeout=60) as response:
            raw = response.read()
            retrieval = {"url": url, "final_url": response.url, "http_status": response.status,
                         "accessed_utc": access}
    if digest(raw) != expected["content_hash"] or len(raw) != expected["n_bytes"]:
        raise SystemExit("original source hash/size mismatch; no derivation attempted")
    source = work / expected["file"]
    with source.open("xb") as stream:
        stream.write(raw)
    commands = []
    products = []
    for run in ("first", "regenerated"):
        output = work / run
        output.mkdir()
        command = ["cargo", "run", "--locked", "-q", "--manifest-path",
                   str(root / "engine/Cargo.toml"), "-p", "syndeom-derive", "--",
                   "--config", str(root / "groundtruth/derivations/1209.2022v2-proposition3.1.json"),
                   "--parent", str(parent_path), "--bytes", str(source), "--root", str(root),
                   "--external-dir", str(output), "--engine-commit", commit]
        completed = subprocess.run(command, cwd=root, capture_output=True, text=True,
                                   encoding="utf-8", errors="strict")
        with (work / f"{run}.log").open("x", encoding="utf-8", newline="\n") as stream:
            stream.write(completed.stdout + completed.stderr)
        commands.append({"argv": command, "exit_code": completed.returncode})
        if completed.returncode:
            raise SystemExit(f"Rust derivation refused; see {work / (run + '.log')}")
        child_id = completed.stdout.splitlines()[0]
        metadata = root / "groundtruth" / child_id
        products.append({name: (output / child_id / name).read_bytes()
                         for name in ["proof.md", "context.tex"]} | {
            name: (metadata / name).read_bytes() for name in ["manifest.json", "source-map.json"]})
    if products[0] != products[1]:
        raise SystemExit("regeneration mismatch")
    receipt = {
        "producer_commit": commit,
        "checkout_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root,
                                                    text=True).strip(),
        "parent_id": parent["artifact_id"], "child_id": child_id,
        "retrieval": retrieval, "source_sha256": digest(raw),
        "reproducer_sha256": digest(Path(__file__).read_bytes()),
        "commands": commands, "identical_regeneration": True,
        "outputs": {name: {"sha256": digest(value), "n_bytes": len(value)}
                    for name, value in products[0].items()},
        "validation": "Both Rust runs executed Manifest::check with actual bytes and check_derived; "
                      "source-fidelity review is a separate recorded reading, not this script's verdict."
    }
    with (work / "receipt.json").open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(receipt, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(receipt, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
