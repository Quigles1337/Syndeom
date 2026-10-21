"""ADR-0008 orientation preflight: establish a fetched default-branch snapshot and identify the
exact checkout before any program state is reconstructed or a NEXT-SINGLE-ACTION is executed.

Standard library only; no result authority (SD10). It runs `git`, records what it ran, and never
force-pushes, resets, checks out over changes, discards work, or merges anything but a
fast-forward. The historical build gate (`scripts/check.sh`) does not call it: this check needs
the network, the gate must not.

Usage:
    python -I -X utf8 scripts/orient_preflight.py orient  [--repo DIR] [--remote NAME]
        [--expect-url URL] [--sync] [--allow-dirty PATH ...] [--worktree DIR] [--receipt FILE]
    python -I -X utf8 scripts/orient_preflight.py recheck --baseline SHA [--repo DIR]
        [--remote NAME] [--receipt FILE]

Exit codes:
    0  snapshot established: HEAD (or the new worktree) is the fetched default-branch tip and the
       working tree carries no changes beyond the explicitly allowed paths
    2  fetch failed: there is no current snapshot; do not reconstruct state or execute a cached
       NEXT from this checkout
    3  snapshot identified but the checkout is not it (behind without --sync, ahead, diverged, or
       unrelated dirty work): nothing was changed; use --sync (clean, behind-only, on the default
       branch) or --worktree DIR (an isolated checkout at the fetched snapshot)
    4  recheck: the target moved since the baseline; do not publish onto the stale baseline
    1  usage error, identity mismatch, or an unexpected git failure
"""

import argparse
import datetime
import json
import os
import subprocess
import sys

SCHEMA = "syndeom-orient-preflight/1"


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


class Git:
    def __init__(self, repo, receipt):
        self.repo = repo
        self.receipt = receipt

    def run(self, *args, check=True):
        argv = ["git", *args]
        started = utc()
        proc = subprocess.run(argv, cwd=self.repo, capture_output=True, text=True,
                              encoding="utf-8", errors="replace")
        self.receipt["commands"].append({
            "argv": argv, "cwd": self.repo, "started_utc": started, "finished_utc": utc(),
            "exit_code": proc.returncode, "stdout_head": proc.stdout[:2000],
            "stderr_head": proc.stderr[:2000]})
        if check and proc.returncode != 0:
            raise RuntimeError(f"git {' '.join(args)} failed ({proc.returncode}): {proc.stderr.strip()}")
        return proc

    def out(self, *args):
        return self.run(*args).stdout.strip()


def finish(receipt, path, code, result):
    receipt["result"] = result
    receipt["exit_code"] = code
    receipt["finished_utc"] = utc()
    text = json.dumps(receipt, indent=1, ensure_ascii=False) + "\n"
    if path:
        with open(path, "x", encoding="utf-8", newline="\n") as f:  # never overwrite a receipt
            f.write(text)
    summary = {k: receipt.get(k) for k in ("mode", "classification", "action", "result", "exit_code")}
    for k in ("fetched", "head", "examined_checkout", "baseline", "moved_to"):
        if k in receipt:
            summary[k] = receipt[k]
    print(json.dumps(summary, indent=1, ensure_ascii=False))
    return code


def default_branch(git, remote, receipt):
    """Discover the remote's default branch; never assume it."""
    proc = git.run("ls-remote", "--symref", remote, "HEAD", check=False)
    for line in proc.stdout.splitlines():
        if line.startswith("ref: refs/heads/") and line.rstrip().endswith("HEAD"):
            name = line.split()[1][len("refs/heads/"):]
            receipt["default_branch"] = {"name": name, "source": "git ls-remote --symref"}
            return name
    sym = git.run("symbolic-ref", f"refs/remotes/{remote}/HEAD", check=False)
    if sym.returncode == 0:
        name = sym.stdout.strip().split("/")[-1]
        receipt["default_branch"] = {"name": name, "source": f"refs/remotes/{remote}/HEAD"}
        return name
    git.run("remote", "set-head", remote, "-a")  # the SD7 repair; non-destructive
    name = git.out("symbolic-ref", f"refs/remotes/{remote}/HEAD").split("/")[-1]
    receipt["default_branch"] = {"name": name, "source": f"git remote set-head {remote} -a"}
    return name


def fetch(git, remote, receipt):
    started = utc()
    proc = git.run("fetch", "--prune", remote, check=False)
    receipt["fetch"] = {"remote": remote, "started_utc": started, "finished_utc": utc(),
                        "exit_code": proc.returncode, "stderr_tail": proc.stderr[-1500:]}
    return proc.returncode == 0


def identity(git, args, receipt):
    receipt["repo_toplevel"] = git.out("rev-parse", "--show-toplevel")
    receipt["git_dir"] = git.out("rev-parse", "--git-dir")
    url = git.out("remote", "get-url", args.remote)
    receipt["remote"] = {"name": args.remote, "url": url}
    if args.expect_url and url.rstrip("/").removesuffix(".git") != args.expect_url.rstrip("/").removesuffix(".git"):
        raise RuntimeError(f"remote {args.remote} is {url}, expected {args.expect_url}")


def snapshot_refs(git, remote, branch, receipt):
    ref = f"refs/remotes/{remote}/{branch}"
    sha = git.out("rev-parse", ref)
    receipt["fetched"] = {"ref": ref, "sha": sha, "tree": git.out("rev-parse", f"{ref}^{{tree}}")}
    head = git.out("rev-parse", "HEAD")
    br = git.run("symbolic-ref", "-q", "--short", "HEAD", check=False)
    receipt["head"] = {"sha": head, "tree": git.out("rev-parse", "HEAD^{tree}"),
                       "branch": br.stdout.strip() if br.returncode == 0 else None}
    return ref, sha, head


def working_tree(git, allowed, receipt):
    lines = git.run("status", "--porcelain=v1", "--untracked-files=normal").stdout.splitlines()
    changed = []
    for line in lines:
        path = line[3:]
        if " -> " in path:
            path = path.split(" -> ", 1)[1]
        changed.append({"status": line[:2], "path": path})
    allowed = set(allowed or [])
    unrelated = [c["path"] for c in changed if c["path"] not in allowed]
    receipt["working_tree"] = {"changed": changed, "allowed": sorted(allowed),
                               "allowed_present": sorted(p for p in allowed if any(c["path"] == p for c in changed)),
                               "unrelated": unrelated}
    return unrelated


def divergence(git, ref, receipt):
    counts = git.out("rev-list", "--left-right", "--count", f"HEAD...{ref}").split()
    ahead, behind = int(counts[0]), int(counts[1])
    receipt["ahead"] = ahead
    receipt["behind"] = behind
    receipt["local_only"] = git.run("log", "--format=%H %s", "-n", "50", f"{ref}..HEAD").stdout.splitlines()
    receipt["remote_only"] = git.run("log", "--format=%H %s", "-n", "50", f"HEAD..{ref}").stdout.splitlines()
    if ahead == 0 and behind == 0:
        c = "synchronized"
    elif ahead == 0:
        c = "behind"
    elif behind == 0:
        c = "ahead"
    else:
        c = "diverged"
    receipt["classification"] = c
    return c


def orient(args):
    receipt = {"schema": SCHEMA, "mode": "orient", "started_utc": utc(), "commands": []}
    git = Git(args.repo, receipt)
    try:
        identity(git, args, receipt)
        if not fetch(git, args.remote, receipt):
            receipt["action"] = "stop: no current snapshot (fetch failed); the cached checkout is not evidence"
            return finish(receipt, args.receipt, 2, "FETCH-FAILED")
        branch = default_branch(git, args.remote, receipt)
        ref, fetched, head = snapshot_refs(git, args.remote, branch, receipt)
        unrelated = working_tree(git, args.allow_dirty, receipt)
        cls = divergence(git, ref, receipt)
        receipt["examined_checkout"] = receipt["repo_toplevel"]

        if cls == "synchronized" and not unrelated:
            receipt["action"] = "none: HEAD is the fetched default-branch tip"
            return finish(receipt, args.receipt, 0, "SNAPSHOT-ESTABLISHED")

        if cls == "behind" and not unrelated and args.sync:
            if receipt["head"]["branch"] != branch:
                receipt["action"] = (f"not fast-forwarded: HEAD is on {receipt['head']['branch']!r}, "
                                     f"not the default branch {branch!r}")
            else:
                ff = git.run("merge", "--ff-only", ref, check=False)
                receipt["fast_forward"] = {"exit_code": ff.returncode, "stdout_head": ff.stdout[:500],
                                           "stderr_head": ff.stderr[:500]}
                new_head = git.out("rev-parse", "HEAD")
                receipt["head_after_sync"] = {"sha": new_head, "tree": git.out("rev-parse", "HEAD^{tree}")}
                if ff.returncode == 0 and new_head == fetched and not working_tree(git, args.allow_dirty, receipt):
                    receipt["action"] = "fast-forwarded the default branch to the fetched tip (ff-only)"
                    return finish(receipt, args.receipt, 0, "SNAPSHOT-ESTABLISHED")
                receipt["action"] = "fast-forward refused or did not reach the fetched tip; nothing else was changed"

        if args.worktree:
            wt = git.run("worktree", "add", "--detach", args.worktree, ref, check=False)
            receipt["worktree"] = {"path": args.worktree, "exit_code": wt.returncode,
                                   "stderr_head": wt.stderr[:500]}
            if wt.returncode == 0:
                wgit = Git(args.worktree, receipt)
                wsha = wgit.out("rev-parse", "HEAD")
                wclean = wgit.run("status", "--porcelain=v1").stdout.strip() == ""
                receipt["worktree"].update({"head": wsha, "tree": wgit.out("rev-parse", "HEAD^{tree}"), "clean": wclean})
                if wsha == fetched and wclean:
                    receipt["examined_checkout"] = args.worktree
                    receipt["action"] = ("isolated worktree created at the fetched tip; the original checkout "
                                         f"({cls}; unrelated changes: {len(unrelated)}) is untouched")
                    return finish(receipt, args.receipt, 0, "SNAPSHOT-ESTABLISHED-IN-WORKTREE")

        reasons = []
        if cls != "synchronized":
            reasons.append(cls)
        if unrelated:
            reasons.append(f"{len(unrelated)} unrelated working-tree change(s)")
        receipt["action"] = ("stop: the checkout is not the fetched snapshot (" + "; ".join(reasons) +
                             "); nothing was changed. Use --sync on a clean checkout of the default branch "
                             "that is only behind, or --worktree DIR for an isolated checkout at the snapshot.")
        return finish(receipt, args.receipt, 3, "CHECKOUT-NOT-CURRENT")
    except RuntimeError as e:
        receipt["error"] = str(e)
        receipt["action"] = "stop: identity or git failure"
        return finish(receipt, args.receipt, 1, "ERROR")


def recheck(args):
    receipt = {"schema": SCHEMA, "mode": "recheck", "started_utc": utc(), "commands": [],
               "baseline": args.baseline}
    git = Git(args.repo, receipt)
    try:
        identity(git, args, receipt)
        if not fetch(git, args.remote, receipt):
            receipt["action"] = "stop: fetch failed; the target cannot be confirmed; do not publish"
            return finish(receipt, args.receipt, 2, "FETCH-FAILED")
        branch = default_branch(git, args.remote, receipt)
        ref, fetched, head = snapshot_refs(git, args.remote, branch, receipt)
        if fetched == args.baseline:
            receipt["action"] = "target unchanged since the baseline; publication may proceed as planned"
            return finish(receipt, args.receipt, 0, "TARGET-UNCHANGED")
        receipt["moved_to"] = fetched
        receipt["new_commits"] = git.run("log", "--format=%H %s", "-n", "50",
                                         f"{args.baseline}..{ref}", check=False).stdout.splitlines()
        receipt["baseline_is_ancestor"] = git.run("merge-base", "--is-ancestor", args.baseline, ref,
                                                  check=False).returncode == 0
        receipt["action"] = ("stop: the target moved since the baseline; re-orient on the new tip and "
                             "integrate deliberately (fast-forward, rebase or merge you have read); never force-push")
        return finish(receipt, args.receipt, 4, "TARGET-MOVED")
    except RuntimeError as e:
        receipt["error"] = str(e)
        receipt["action"] = "stop: identity or git failure"
        return finish(receipt, args.receipt, 1, "ERROR")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="mode", required=True)
    for name in ("orient", "recheck"):
        p = sub.add_parser(name)
        p.add_argument("--repo", default=os.getcwd(), help="the checkout to examine (default: cwd)")
        p.add_argument("--remote", default="origin")
        p.add_argument("--expect-url", default=None, help="fail unless the remote URL matches")
        p.add_argument("--receipt", default=None, help="write the JSON receipt here (refuses to overwrite)")
        if name == "orient":
            p.add_argument("--sync", action="store_true",
                           help="fast-forward (ff-only) a clean, behind-only checkout of the default branch")
            p.add_argument("--allow-dirty", nargs="*", default=[], metavar="PATH",
                           help="working-tree paths whose changes are expected (an examination's HANDOFF append)")
            p.add_argument("--worktree", default=None, metavar="DIR",
                           help="create an isolated detached worktree at the fetched tip when the checkout is not it")
        else:
            p.add_argument("--baseline", required=True, help="the fetched default-branch SHA the work was based on")
    args = parser.parse_args()
    args.repo = os.path.abspath(args.repo)
    return orient(args) if args.mode == "orient" else recheck(args)


if __name__ == "__main__":
    sys.exit(main())
