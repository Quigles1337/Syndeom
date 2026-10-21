"""Offline verification of the ADR-0008 preflight against isolated temporary Git repositories.

Builds a bare "remote" and clones under a new work directory, drives the stale-tracking-branch
failure mode and the other meaningful cases through scripts/orient_preflight.py, and records every
command and outcome. It proves what the preflight does; it says nothing about any live remote's
current state. Standard library only; no network.

Usage:
    python -I -X utf8 scripts/orient_preflight_cases.py --work-dir DIR   (DIR must not exist)
Exit 0 when every case meets its expectation; 1 otherwise. Writes DIR/cases.json.
"""

import argparse
import datetime
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PREFLIGHT = os.path.join(HERE, "orient_preflight.py")
ENV = {**os.environ, "GIT_AUTHOR_NAME": "case", "GIT_AUTHOR_EMAIL": "case@example.invalid",
       "GIT_COMMITTER_NAME": "case", "GIT_COMMITTER_EMAIL": "case@example.invalid",
       "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_SYSTEM": os.devnull}


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


class Case:
    def __init__(self, name, log):
        self.name, self.log, self.steps, self.checks = name, log, [], []

    def sh(self, argv, cwd, check=True):
        proc = subprocess.run(argv, cwd=cwd, capture_output=True, text=True, encoding="utf-8",
                              errors="replace", env=ENV)
        self.steps.append({"argv": argv, "cwd": cwd, "exit_code": proc.returncode,
                           "stdout": proc.stdout[-3000:], "stderr": proc.stderr[-1500:]})
        if check and proc.returncode != 0:
            raise RuntimeError(f"{self.name}: {' '.join(argv)} -> {proc.returncode}: {proc.stderr}")
        return proc

    def git(self, cwd, *args, check=True):
        return self.sh(["git", *args], cwd, check=check)

    def preflight(self, cwd, *args, receipt):
        argv = [sys.executable, "-I", "-X", "utf8", PREFLIGHT, *args, "--repo", cwd, "--receipt", receipt]
        proc = self.sh(argv, cwd, check=False)
        with open(receipt, encoding="utf-8") as f:
            return proc.returncode, json.load(f)

    def expect(self, what, ok, detail=""):
        self.checks.append({"expect": what, "ok": bool(ok), "detail": detail})
        if not ok:
            print(f"  FAIL {self.name}: {what} {detail}")

    def record(self):
        passed = all(c["ok"] for c in self.checks)
        self.log.append({"case": self.name, "passed": passed, "checks": self.checks, "steps": self.steps})
        print(f"{'PASS' if passed else 'FAIL'} {self.name}")
        return passed


def commit(case, repo, name, text):
    path = os.path.join(repo, name)
    with open(path, "a", encoding="utf-8", newline="\n") as f:
        f.write(text)
    case.git(repo, "add", "-A")
    case.git(repo, "commit", "-q", "-m", f"add {text.strip()}")
    return case.git(repo, "rev-parse", "HEAD").stdout.strip()


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--work-dir", required=True)
    args = ap.parse_args()
    work = os.path.abspath(args.work_dir)
    os.makedirs(work)  # refuses an existing directory
    log = []
    remote = os.path.join(work, "remote.git")
    A = os.path.join(work, "A")      # the other executor
    B = os.path.join(work, "B")      # the stale working copy
    rec = lambda name: os.path.join(work, name + ".receipt.json")
    results = []

    # setup: A creates the history, pushes main; B clones and is in sync at c1
    c = Case("setup", log)
    c.git(work, "init", "--bare", "-b", "main", remote)
    c.git(work, "init", "-q", "-b", "main", A)
    c.git(A, "config", "core.autocrlf", "false")
    c1 = commit(c, A, "base.txt", "c1\n")
    commit(c, A, "PROGRAM/HANDOFF.md".replace("/", os.sep) if False else "HANDOFF.md", "SESSION: fixture\n")
    c1 = c.git(A, "rev-parse", "HEAD").stdout.strip()
    c.git(A, "remote", "add", "origin", remote)
    c.git(A, "push", "-q", "-u", "origin", "main")
    c.git(work, "clone", "-q", remote, B)
    c.git(B, "config", "core.autocrlf", "false")
    c.expect("B at c1 after clone", c.git(B, "rev-parse", "HEAD").stdout.strip() == c1)
    results.append(c.record())

    # case 1: remote moves; B's cached tracking branch still says "in sync"
    c = Case("1-stale-tracking-branch-appears-synchronized", log)
    c2 = commit(c, A, "base.txt", "c2\n")
    c.git(A, "push", "-q", "origin", "main")
    sb = c.git(B, "status", "-sb").stdout.strip().splitlines()[0]
    behind_cached = c.git(B, "rev-list", "--count", "HEAD..origin/main").stdout.strip()
    c.expect("git status -sb shows no behind count before fetch", "behind" not in sb and "ahead" not in sb, sb)
    c.expect("cached origin/main equals HEAD (0 behind) before fetch", behind_cached == "0", behind_cached)
    c.expect("yet the remote tip differs from B's HEAD", c.git(B, "rev-parse", "HEAD").stdout.strip() != c2)
    results.append(c.record())

    # case 2 + 3: the preflight fetches, exposes the newer snapshot, and refuses to call the stale checkout current
    c = Case("2-3-fetch-exposes-newer-snapshot-and-stale-checkout-is-not-current", log)
    code, r = c.preflight(B, "orient", receipt=rec("case2"))
    c.expect("exit 3 (checkout not current)", code == 3, str(code))
    c.expect("classification behind", r.get("classification") == "behind", str(r.get("classification")))
    c.expect("fetched sha is the remote's c2", r.get("fetched", {}).get("sha") == c2)
    c.expect("one remote-only commit listed", len(r.get("remote_only", [])) == 1)
    c.expect("HEAD unchanged (still c1)", c.git(B, "rev-parse", "HEAD").stdout.strip() == c1)
    c.expect("fetch recorded with exit 0 and times", r.get("fetch", {}).get("exit_code") == 0 and "started_utc" in r.get("fetch", {}))
    c.expect("receipt records head, fetched, trees, remote, default branch",
             all(k in r for k in ("head", "fetched", "remote", "default_branch", "working_tree")))
    results.append(c.record())

    # case 4: clean behind-only synchronization succeeds safely (ff-only)
    c = Case("4-clean-behind-only-sync-fast-forwards", log)
    code, r = c.preflight(B, "orient", "--sync", receipt=rec("case4"))
    c.expect("exit 0", code == 0, str(code))
    c.expect("HEAD now equals fetched c2", c.git(B, "rev-parse", "HEAD").stdout.strip() == c2)
    c.expect("fast-forward recorded", r.get("fast_forward", {}).get("exit_code") == 0)
    c.expect("working tree clean", c.git(B, "status", "--porcelain").stdout.strip() == "")
    results.append(c.record())

    # case 5: ahead/diverged commits and unrelated dirty work are preserved; a worktree gives the snapshot
    c = Case("5-ahead-diverged-and-dirty-work-preserved", log)
    c3_local = commit(c, B, "local.txt", "local work\n")
    c3 = commit(c, A, "base.txt", "c3\n")
    c.git(A, "push", "-q", "origin", "main")
    with open(os.path.join(B, "base.txt"), "a", encoding="utf-8", newline="\n") as f:
        f.write("uncommitted edit\n")
    with open(os.path.join(B, "untracked.txt"), "w", encoding="utf-8", newline="\n") as f:
        f.write("untracked\n")
    code, r = c.preflight(B, "orient", "--sync", receipt=rec("case5a"))
    c.expect("exit 3 even with --sync", code == 3, str(code))
    c.expect("classification diverged", r.get("classification") == "diverged", str(r.get("classification")))
    c.expect("unrelated dirty paths reported", set(r.get("working_tree", {}).get("unrelated", [])) == {"base.txt", "untracked.txt"},
             str(r.get("working_tree", {}).get("unrelated")))
    c.expect("local commit preserved", c.git(B, "rev-parse", "HEAD").stdout.strip() == c3_local)
    c.expect("uncommitted edit preserved", open(os.path.join(B, "base.txt"), encoding="utf-8").read().endswith("uncommitted edit\n"))
    c.expect("untracked file preserved", os.path.exists(os.path.join(B, "untracked.txt")))
    c.expect("remote commit not merged", c.git(B, "merge-base", "--is-ancestor", c3, "HEAD", check=False).returncode != 0)
    wt = os.path.join(work, "B-worktree")
    code, r = c.preflight(B, "orient", "--worktree", wt, receipt=rec("case5b"))
    c.expect("exit 0 with an isolated worktree", code == 0, str(code))
    c.expect("worktree HEAD is the fetched c3", r.get("worktree", {}).get("head") == c3)
    c.expect("examined checkout is the worktree", r.get("examined_checkout") == wt)
    c.expect("original checkout still at its local commit", c.git(B, "rev-parse", "HEAD").stdout.strip() == c3_local)
    c.expect("original dirty work still present", os.path.exists(os.path.join(B, "untracked.txt")))
    results.append(c.record())

    # case 6: fetch failure does not permit stale-state execution
    c = Case("6-fetch-failure-stops", log)
    c.git(B, "remote", "set-url", "origin", os.path.join(work, "missing.git"))
    head_before = c.git(B, "rev-parse", "HEAD").stdout.strip()
    code, r = c.preflight(B, "orient", "--sync", receipt=rec("case6"))
    c.expect("exit 2", code == 2, str(code))
    c.expect("result FETCH-FAILED", r.get("result") == "FETCH-FAILED")
    c.expect("no snapshot recorded", "fetched" not in r)
    c.expect("HEAD unchanged", c.git(B, "rev-parse", "HEAD").stdout.strip() == head_before)
    c.git(B, "remote", "set-url", "origin", remote)
    results.append(c.record())

    # case 7: target movement is detected before publication
    c = Case("7-target-moved-before-publication", log)
    C = os.path.join(work, "C")
    c.git(work, "clone", "-q", remote, C)
    code, r = c.preflight(C, "orient", receipt=rec("case7a"))
    baseline = r["fetched"]["sha"]
    c.expect("C synchronized at the baseline", code == 0 and baseline == c3)
    c4 = commit(c, A, "base.txt", "c4\n")
    c.git(A, "push", "-q", "origin", "main")
    code, r = c.preflight(C, "recheck", "--baseline", baseline, receipt=rec("case7b"))
    c.expect("exit 4 (target moved)", code == 4, str(code))
    c.expect("moved_to is c4", r.get("moved_to") == c4)
    c.expect("baseline is an ancestor of the new tip", r.get("baseline_is_ancestor") is True)
    c.expect("nothing was published (remote tip is A's c4)",
             c.git(C, "ls-remote", remote, "refs/heads/main").stdout.split()[0] == c4)
    code, r = c.preflight(C, "recheck", "--baseline", c4, receipt=rec("case7c"))
    c.expect("recheck against the current tip exits 0", code == 0, str(code))
    results.append(c.record())

    # case 8: an examination's own HANDOFF append is accounted for, never erased
    c = Case("8-examination-handoff-append-accounted-for", log)
    c.git(C, "merge", "-q", "--ff-only", "origin/main")
    with open(os.path.join(C, "HANDOFF.md"), "a", encoding="utf-8", newline="\n") as f:
        f.write("SESSION: examination disclosure\n")
    code, r = c.preflight(C, "orient", receipt=rec("case8a"))
    c.expect("without an allowance the append counts as an unrelated change (exit 3)",
             code == 3 and r.get("working_tree", {}).get("unrelated") == ["HANDOFF.md"], str(code))
    code, r = c.preflight(C, "orient", "--allow-dirty", "HANDOFF.md", receipt=rec("case8b"))
    c.expect("with the allowance the snapshot is established (exit 0)", code == 0, str(code))
    c.expect("the allowed path is reported as present", r.get("working_tree", {}).get("allowed_present") == ["HANDOFF.md"])
    c.expect("the disclosure text is still there",
             open(os.path.join(C, "HANDOFF.md"), encoding="utf-8").read().endswith("SESSION: examination disclosure\n"))
    results.append(c.record())

    summary = {"schema": "syndeom-orient-preflight-cases/1", "recorded_utc": utc(), "work_dir": work,
               "python": sys.version.split()[0],
               "git": subprocess.run(["git", "--version"], capture_output=True, text=True).stdout.strip(),
               "all_passed": all(results), "cases": log,
               "note": "offline fixture; establishes nothing about any live remote's current state"}
    with open(os.path.join(work, "cases.json"), "x", encoding="utf-8", newline="\n") as f:
        json.dump(summary, f, indent=1, ensure_ascii=False)
        f.write("\n")
    print("all passed" if all(results) else "FAILURES")
    return 0 if all(results) else 1


if __name__ == "__main__":
    sys.exit(main())
