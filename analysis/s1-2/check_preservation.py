"""Check the scoped follow-up against its immutable main baseline; write a NEW receipt."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
BASE = "8adcb4865abd3f6c3ac882b70563d82caca6207c"
EDITABLE = {"PROGRAM/STATE.md", "PROGRAM/HANDOFF.md", "PROGRAM/SOURCES.md", "engine/derive/README.md"}


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    assert not args.output.exists(), "refuse to overwrite evidence"
    names = git("ls-tree", "-r", "--name-only", BASE).decode().splitlines()
    protected = {}
    prefixes = {}
    for name in names:
        before = git("show", f"{BASE}:{name}")
        after = (ROOT / name).read_bytes()
        if name not in EDITABLE:
            assert before == after, f"protected bytes changed: {name}"
            protected[name] = hashlib.sha256(before).hexdigest()
        elif name in {"PROGRAM/HANDOFF.md", "PROGRAM/SOURCES.md"}:
            assert after.startswith(before), f"historical prefix changed: {name}"
            prefixes[name] = {"n_bytes": len(before), "sha256": hashlib.sha256(before).hexdigest()}
    tracked = set(git("ls-files").decode().splitlines())
    for name in tracked - set(names):
        assert name == "PROGRAM/DECISIONS/ADR-0007.md" or name.startswith("analysis/s1-2/"), name
    manifests = [json.loads(p.read_text()) for p in (ROOT / "groundtruth").glob("*/manifest.json")]
    raw = sum(any(f["format"] == "raw/1" for f in m["artifact"]["files"].values()) for m in manifests)
    canonical = len(manifests) - raw
    assert (raw, canonical) == (31, 1), (raw, canonical)
    assert all(m["artifact"]["kind"] != "pair" for m in manifests), "new pair admitted"
    for revision in ("49e47237924eddf0c39c24aefad0eb9aa02ee8b3",
                     "d2a33062435b5fbc21cb1562b2a33b5a8c0ca20e"):
        subprocess.run(["git", "merge-base", "--is-ancestor", revision, "HEAD"], cwd=ROOT, check=True)
    record = {
        "base_revision": BASE, "checkout_revision": git("rev-parse", "HEAD").decode().strip(),
        "checked_at": datetime.now(timezone.utc).isoformat(),
        "helper_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "protected_count": len(protected), "protected_sha256": protected,
        "exact_historical_prefixes": prefixes,
        "raw_parents": raw, "canonical_representations": canonical, "admitted_pairs": 0,
        "s1_3_producer_and_evidence_ancestry": "preserved", "result": "PASS",
    }
    with args.output.open("x", encoding="utf-8", newline="\n") as handle:
        json.dump(record, handle, indent=2)
        handle.write("\n")
    print(f"PASS: {len(protected)} protected files byte-identical; exact HANDOFF/SOURCES prefixes; "
          f"{raw} raw + {canonical} canonical; no pair admitted; producer/evidence ancestry retained")


if __name__ == "__main__":
    main()
