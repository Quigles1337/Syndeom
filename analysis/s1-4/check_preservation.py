"""Byte preservation check; no corpus-label or statistical authority."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--base", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root = args.repo.resolve()
    if args.output.exists():
        raise ValueError("new receipt path required")
    allowed_docs = {
        "README.md", "PROGRAM/STATE.md", "PROGRAM/DECISIONS/ADR-0007.md",
        "analysis/s1-2/README.md", "analysis/s1-2/implementation/README.md",
        "engine/pair/README.md",
    }
    prefixes = {
        "PROGRAM/HANDOFF.md", "PROGRAM/SOURCES.md",
        "groundtruth/README.md",
        "groundtruth/s1_pair_archive-v0.1.0-094625fa10ff/RECORD.md",
        "groundtruth/s1_pair-v0.1.0-62fe0c816a6b/RECORD.md",
    }
    data = subprocess.check_output(["git", "archive", "--format=tar", args.base], cwd=root)
    original = {}
    with tarfile.open(fileobj=io.BytesIO(data)) as archive:
        for entry in archive:
            if entry.isfile():
                original[entry.name] = archive.extractfile(entry).read()
    unchanged, appended, changed_docs = {}, {}, []
    for name, before in original.items():
        after = (root / name).read_bytes()
        if name in prefixes:
            if not after.startswith(before):
                raise AssertionError(f"historical prefix changed: {name}")
            appended[name] = {"prefix_bytes": len(before), "prefix_sha256": hashlib.sha256(before).hexdigest(), "appended_bytes": len(after)-len(before)}
        elif name in allowed_docs:
            if after != before:
                changed_docs.append(name)
        else:
            if after != before:
                raise AssertionError(f"protected baseline file changed: {name}")
            unchanged[name] = hashlib.sha256(before).hexdigest()
    pair = {}
    for name in original["engine/pair/input-files.txt"].decode().splitlines():
        if (root / name).read_bytes() != original[name]:
            raise AssertionError(f"frozen pair input changed: {name}")
        pair[name] = hashlib.sha256(original[name]).hexdigest()
    result = {"result": "PASS", "base_revision": args.base,
              "checkout_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root).decode().strip(),
              "unchanged_baseline_files": unchanged, "preserved_prefixes": appended,
              "current_summary_edits": changed_docs, "frozen_pair_inputs": pair}
    with args.output.open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(result, stream, indent=2)
        stream.write("\n")
    print(f"PASS: {len(unchanged)} unchanged baseline files; {len(prefixes)} preserved prefixes; {len(pair)} frozen pair inputs")


if __name__ == "__main__":
    main()
