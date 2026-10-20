"""Orchestrate Rust reports and an optional historical pair replay; no label/count authority.

Run from repository root. Output must be a new directory outside the checkout.
The report command is authoritative Rust; Python records commands, hashes and byte equality.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys


def sha(data):
    return hashlib.sha256(data).hexdigest()


def utc():
    return datetime.now(timezone.utc).isoformat()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--corpus", required=True)
    parser.add_argument("--tool", required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--pair-sources", type=Path)
    parser.add_argument("--pair-retained-external", type=Path)
    args = parser.parse_args()
    root, out = args.repo.resolve(), args.out.resolve()
    if root == out or root in out.parents:
        raise ValueError("use a new output directory outside the repository")
    if bool(args.pair_sources) != bool(args.pair_retained_external):
        raise ValueError("supply both pair source and retained external directories")
    out.mkdir(parents=True, exist_ok=False)
    (out / "logs").mkdir()
    receipt = {"started_at": utc(), "helper_sha256": sha(Path(__file__).read_bytes()),
               "corpus_revision": args.corpus, "tool_revision": args.tool,
               "output": str(out), "commands": [], "success": False}

    def git(*argv):
        return subprocess.check_output(["git", *argv], cwd=root)

    def run(label, argv, cwd=root, env=None, expected=0):
        log = out / "logs" / f"{label}.log"
        record = {"label": label, "argv": list(map(str, argv)), "cwd": str(cwd),
                  "started_at": utc(), "expected_exit": expected}
        receipt["commands"].append(record)
        print(label, flush=True)
        with log.open("xb") as stream:
            result = subprocess.run(argv, cwd=cwd, env=env, stdout=stream, stderr=subprocess.STDOUT)
        record.update(exit_code=result.returncode, ended_at=utc(), log_sha256=sha(log.read_bytes()), log=str(log))
        if result.returncode != expected:
            raise RuntimeError(f"{label}: exit {result.returncode}; see {log}")
        return log.read_bytes()

    def snapshot(folder):
        return {p.relative_to(folder).as_posix(): sha(p.read_bytes())
                for p in sorted(folder.rglob("*")) if p.is_file()}

    try:
        receipt["checkout_revision"] = git("rev-parse", "HEAD").decode().strip()
        receipt["checkout_tree"] = git("rev-parse", "HEAD^{tree}").decode().strip()
        receipt["rustc"] = run("rustc-version", ["rustc", "--version"]).decode().strip()
        if not receipt["rustc"].startswith("rustc 1.91.0 "):
            raise ValueError("unexpected Rust toolchain")
        command = ["cargo", "run", "--locked", "-p", "syndeom-manifest", "--example", "s1_4_coverage", "--", "report", str(root), args.corpus, args.tool]
        for label in ("report-a", "report-b"):
            run(label, [*command, str(out / label)], root / "engine")
        first, second = snapshot(out / "report-a"), snapshot(out / "report-b")
        if first != second:
            raise AssertionError("deterministic report replay differs")
        receipt["deterministic_artifacts"] = first
        receipt["fresh_destinations_identical"] = True
        run("existing-output-refusal", [*command, str(out / "report-a")], root / "engine", expected=2)
        if snapshot(out / "report-a") != first:
            raise AssertionError("existing-output refusal changed retained bytes")

        if args.pair_sources:
            producer = "eb1b96d5b8084ceb1de734fffc90c9b480ec32bf"
            before = snapshot(args.pair_retained_external)
            source_before = snapshot(args.pair_sources)
            env = os.environ.copy()
            target = out / "fresh-pair-target"
            if target.exists():
                raise ValueError("fresh build target already exists")
            env["CARGO_TARGET_DIR"] = str(target)
            run("fresh-pair-build", ["cargo", "build", "--locked", "-p", "syndeom-pair"], root / "engine", env)
            exe = target / "debug" / ("syndeom-pair.exe" if sys.platform == "win32" else "syndeom-pair")
            receipt["pair_executable_sha256"] = sha(exe.read_bytes())
            artifact_root, external = out / "pair-artifacts", out / "pair-external"
            artifact_root.mkdir()
            external.mkdir()
            (out / "pair-receipts").mkdir()
            for mode in ("archive", "canonical"):
                argv = [str(exe), "--mode", mode, "--config", str(root / f"analysis/s1-2/{mode}-config.proposed.json"),
                        "--root", str(root), "--artifact-root", str(artifact_root), "--external-dir", str(external),
                        "--engine-commit", producer, "--receipt-dir", str(out / "pair-receipts" / mode)]
                if mode == "archive":
                    argv += ["--sources-dir", str(args.pair_sources.resolve())]
                run("pair-replay-" + mode, argv)
            generated = {}
            for p in sorted(artifact_root.rglob("*")):
                if p.is_file():
                    relative = p.relative_to(artifact_root).as_posix()
                    expected = git("show", f"{args.corpus}:{relative}")
                    if p.read_bytes() != expected:
                        raise AssertionError(f"historical metadata differs: {relative}")
                    generated["metadata/" + relative] = sha(expected)
            for p in sorted(external.rglob("*")):
                if p.is_file():
                    relative = p.relative_to(external)
                    expected = (args.pair_retained_external / relative).read_bytes()
                    if p.read_bytes() != expected:
                        raise AssertionError(f"historical external output differs: {relative}")
                    generated["external/" + relative.as_posix()] = sha(expected)
            if snapshot(args.pair_retained_external) != before or snapshot(args.pair_sources) != source_before:
                raise AssertionError("retained sources/outputs changed")
            receipt["pair_replay"] = {"producer_revision": producer, "outputs": generated,
                "retained_sources_unchanged": True, "retained_outputs_unchanged": True,
                "source_method": "retained exact source bytes; no new HTTP retrieval",
                "lean_scope": "not executed in compatibility replay; historical emitted-file audit remains evidence",
                "sources_directory": str(args.pair_sources.resolve()),
                "retained_external_directory": str(args.pair_retained_external.resolve())}
        receipt["success"] = True
    finally:
        receipt["ended_at"] = utc()
        with (out / "execution-receipt.json").open("x", encoding="utf-8", newline="\n") as f:
            json.dump(receipt, f, indent=2)
            f.write("\n")


if __name__ == "__main__":
    main()
