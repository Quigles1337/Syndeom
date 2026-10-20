"""Exact-byte preservation check; no mathematical or corpus-label authority."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile

BASE = "1a76d9ac21cd5d2e4311f9164178327c0ca128bb"
PAIR = "eb1b96d5b8084ceb1de734fffc90c9b480ec32bf"
PACKET = "analysis/s1-first-parent-status/"


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--repo", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    args = p.parse_args()
    root = args.repo.resolve()
    assert not args.output.exists(), "fresh receipt required"
    sha = lambda b: hashlib.sha256(b).hexdigest()
    git = lambda *a: subprocess.check_output(["git", *a], cwd=root)
    before = {}
    with tarfile.open(fileobj=io.BytesIO(git("archive", "--format=tar", BASE))) as archive:
        for e in archive:
            if e.isfile():
                before[e.name] = archive.extractfile(e).read()
    summaries = {"README.md", "PROGRAM/STATE.md"}
    prefixes = {"PROGRAM/HANDOFF.md", "PROGRAM/SOURCES.md",
                "groundtruth/s1_ingest-v0.1.0-63e23a952084/RECORD.md"}
    unchanged, appended, changed = {}, {}, []
    for name, old in before.items():
        new = (root / name).read_bytes()
        if name in prefixes:
            assert new.startswith(old), f"historical prefix changed: {name}"
            appended[name] = {"prefix_bytes": len(old), "prefix_sha256": sha(old),
                              "appended_bytes": len(new) - len(old)}
        elif name in summaries:
            if new != old:
                changed.append(name)
        else:
            assert old == new, f"protected baseline file changed: {name}"
            unchanged[name] = sha(old)
    current = set(git("ls-files", "--cached", "--others", "--exclude-standard").decode().splitlines())
    extra = current - before.keys()
    assert all(n.startswith(PACKET) for n in extra), f"unexpected new files: {extra}"
    pair = {}
    for name in before["engine/pair/input-files.txt"].decode().splitlines():
        pinned = git("show", f"{PAIR}:{name}")
        assert (root / name).read_bytes() == pinned == before[name], name
        pair[name] = sha(pinned)
    assert len(pair) == 20
    frozen = {k: v for k, v in unchanged.items() if k.startswith("analysis/s1-4/")}
    result = {"result": "PASS", "base_revision": BASE,
              "checkout_revision": git("rev-parse", "HEAD").decode().strip(),
              "unchanged_baseline_files": unchanged, "preserved_prefixes": appended,
              "current_summary_edits": changed, "frozen_s1_4_files": frozen,
              "frozen_pair_revision": PAIR, "frozen_pair_inputs": pair,
              "new_paths_only_in_packet": sorted(extra)}
    args.output.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(f"PASS: {len(unchanged)} unchanged old files; {len(frozen)} frozen S1.4 files; "
          f"{len(appended)} exact prefixes; {len(pair)} pinned pair inputs")


if __name__ == "__main__":
    main()
