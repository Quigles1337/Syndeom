"""New implementation scope; never weaken/run the historical 31+1 procedure checker."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib

BASE = "1a8388b03017e4044cad01ac10e7e89f8fb5e38f"
CORRECTION = "d88fad2bb7b339c56bd9964167d3e0acddc8ee4e"
ARCHIVE = "s1_pair_archive-v0.1.0-094625fa10ff"
CHILD = "s1_pair-v0.1.0-62fe0c816a6b"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", required=True, type=Path)
    parser.add_argument("--producer", required=True)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    root = args.root.resolve()

    def git(*parts):
        return subprocess.check_output(["git", *parts], cwd=root)

    protected, prefixes = {}, {}
    editable = {
        "README.md", "PROGRAM/STATE.md", "PROGRAM/DECISIONS/ADR-0007.md",
        "analysis/s1-2/README.md", "analysis/s1-2/reproduce.py",
        "analysis/s1-2/archive-config.proposed.json", "analysis/s1-2/canonical-config.proposed.json",
        "engine/Cargo.toml", "engine/Cargo.lock",
    }
    prefix_only = {"PROGRAM/HANDOFF.md", "PROGRAM/SOURCES.md", "analysis/s1-2/VERIFICATION.md"}
    names = git("ls-tree", "-r", "--name-only", BASE).decode().splitlines()
    for name in names:
        original, current = git("show", f"{BASE}:{name}"), (root / name).read_bytes()
        if name in prefix_only:
            assert current.startswith(original), f"main historical prefix changed: {name}"
            corrected_prefix = git("show", f"{CORRECTION}:{name}")
            assert current.startswith(corrected_prefix), f"correction historical prefix changed: {name}"
            prefixes[name] = {"main_sha256": hashlib.sha256(original).hexdigest(),
                             "correction_sha256": hashlib.sha256(corrected_prefix).hexdigest()}
        elif name not in editable:
            assert current == original, f"protected main file changed: {name}"
            protected[name] = hashlib.sha256(original).hexdigest()
    correction_files = {}
    for name in git("ls-tree", "-r", "--name-only", CORRECTION, "analysis/s1-2/correction",
                    "analysis/s1-2/source_locator.py", "analysis/s1-2/archive-config.proposed.json",
                    "analysis/s1-2/canonical-config.proposed.json", "analysis/s1-2/reproduce.py").decode().splitlines():
        original = git("show", f"{CORRECTION}:{name}")
        assert (root / name).read_bytes() == original, f"corrected procedure input/evidence changed: {name}"
        correction_files[name] = hashlib.sha256(original).hexdigest()
    old_lock = tomllib.loads(git("show", f"{BASE}:engine/Cargo.lock").decode())
    new_lock = tomllib.loads((root / "engine/Cargo.lock").read_text())
    old = {(p["name"], p["version"]): p for p in old_lock["package"]}
    new = {(p["name"], p["version"]): p for p in new_lock["package"]}
    assert all(new[k] == v for k, v in old.items()), "existing dependency pin or metadata changed"
    assert set(new) - set(old) == {("syndeom-pair", "0.1.0")}, "unexpected added dependency"
    assert tomllib.loads((root / "engine/Cargo.toml").read_text())["workspace"]["members"] == [
        "manifest", "ingest", "derive", "pair"]
    old_manifests = [name for name in names if name.startswith("groundtruth/") and name.endswith("/manifest.json")]
    current_paths = list((root / "groundtruth").glob("*/manifest.json"))
    added = {p.parent.name for p in current_paths} - {Path(n).parent.name for n in old_manifests}
    assert added == {ARCHIVE, CHILD}, added
    manifests = [json.loads(p.read_text()) for p in current_paths]
    raw = sum(any(f["format"] == "raw/1" for f in m["artifact"]["files"].values()) for m in manifests)
    assert (raw, len(manifests) - raw) == (32, 2)
    pairs = [m for m in manifests if m["artifact"]["kind"] == "pair"]
    assert {m["artifact_id"] for m in pairs} == {ARCHIVE, CHILD}
    assert all(m["engine_commit"] == args.producer for m in pairs)
    for name in set(git("ls-files").decode().splitlines()) - set(names):
        assert name in correction_files or name.startswith((
            "engine/pair/", "analysis/s1-2/implementation/",
            f"groundtruth/{ARCHIVE}/", f"groundtruth/{CHILD}/",
        )), f"unexpected added tracked path: {name}"
    for item, expected in [
        (ARCHIVE, {"manifest.json", "source-map.json", "RECORD.md"}),
        (CHILD, {"manifest.json", "source-map.json", "RECORD.md", "Proof.lean"}),
    ]:
        assert {p.name for p in (root / "groundtruth" / item).iterdir()} == expected, (
            "unexpected raw/restricted or missing artifact file", item)
    for rev in ["49e47237924eddf0c39c24aefad0eb9aa02ee8b3",
                "d2a33062435b5fbc21cb1562b2a33b5a8c0ca20e",
                "c157e4e50b011fd11fc08b31d6963900923d2564", CORRECTION, args.producer]:
        subprocess.run(["git", "merge-base", "--is-ancestor", rev, "HEAD"], cwd=root, check=True)
    record = {"baseline": BASE, "corrected_procedure": CORRECTION, "producer": args.producer,
              "checkout": git("rev-parse", "HEAD").decode().strip(), "checked_at": datetime.now(timezone.utc).isoformat(),
              "helper_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              "protected_main_files": protected, "protected_main_file_count": len(protected),
              "protected_correction_files": correction_files, "historical_prefixes": prefixes,
              "existing_dependency_pins": "unchanged; only local pair package added",
              "accepted_main_inventory_at_baseline": {"raw": 31, "canonical": 1, "positive_pairs": 0},
              "branch_inventory": {"raw": 32, "canonical": 2, "additional_independent_positive_items_proposed": 1},
              "admission_on_main": False, "result": "PASS"}
    with args.output.open("x", encoding="utf-8", newline="\n") as f:
        json.dump(record, f, indent=2)
        f.write("\n")
    print(f"PASS: {len(protected)} baseline files + {len(correction_files)} correction files preserved; "
          "historical prefixes and pins intact; branch 32+2 proposes one additional positive item")


if __name__ == "__main__":
    main()
