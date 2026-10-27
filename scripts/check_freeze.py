"""SYNDEOM freeze check (ADR-0012, Amendments 1): the frozen scopes and append-only files that
PROGRAM/STATE.md records in its "Freeze register" section are verified against Git objects.

A frozen scope is byte-identical to its version at the row's freeze point, with no file added or
removed under it. An append-only file has both its version at the freeze point and its version on
the default branch (refs/remotes/origin/HEAD, as last fetched) as exact prefixes. The register is
STATE's claim; this script is the evidence. Standard library only; reads Git objects through one
cat-file process; never writes to the repository; needs no network.

    python -I -X utf8 scripts/check_freeze.py <repo-root>
"""
import re
import subprocess
import sys
from pathlib import Path

HEADING = "## Freeze register"
ROW = re.compile(r"^\| `([^`]+)` \| (frozen|append-only) \| `([^`]+)` \| (.*) \|$")
HEX40 = re.compile(r"^[0-9a-f]{40}$")


class RegisterError(Exception):
    """The register section is missing or malformed; the gate fails rather than guessing."""


class Row:
    def __init__(self, path, kind, point, record):
        self.path, self.kind, self.point, self.record = path, kind, point, record

    def __repr__(self):
        return f"Row({self.path!r}, {self.kind!r}, {self.point!r})"


def load_register(state_text):
    """Parse the table under the 'Freeze register' heading. Any table row that is not a register
    row in the exact column form is a malformed register."""
    rows, in_section, found = [], False, False
    for line in state_text.splitlines():
        if line.startswith("## "):
            in_section = line.strip() == HEADING
            found = found or in_section
            continue
        if not in_section or not line.startswith("|"):
            continue
        if line.startswith("| Path |") or line.startswith("|---"):
            continue
        m = ROW.match(line)
        if not m:
            raise RegisterError(f"malformed freeze-register row: {line}")
        path, kind, point, record = m.groups()
        if path.startswith("/") or ".." in path.split("/") or "\\" in path:
            raise RegisterError(f"unsafe path in freeze register: {path}")
        if kind == "append-only" and path.endswith("/"):
            raise RegisterError(f"an append-only entry must be a file: {path}")
        if any(r.path == path for r in rows):
            raise RegisterError(f"duplicate freeze-register path: {path}")
        rows.append(Row(path, kind, point, record))
    if not found:
        raise RegisterError(f"PROGRAM/STATE.md has no '{HEADING}' section")
    if not rows:
        raise RegisterError("the freeze register has no rows")
    return rows


class GitEnv:
    """Git objects and the working tree of one repository, read through one process each."""

    def __init__(self, root):
        self.root = Path(root)
        self._batch = None
        # Git objects and refs do not change during a run; the test harness runs the check many
        # times over one environment, so every git answer is memoized.
        self._memo = {}

    def _cached(self, key, compute):
        if key not in self._memo:
            self._memo[key] = compute()
        return self._memo[key]

    def git(self, *args, check=True):
        result = subprocess.run(
            ["git", "-C", str(self.root), *args],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        if check and result.returncode != 0:
            raise RuntimeError(
                f"git {' '.join(args)}: exit {result.returncode}: {result.stderr.decode(errors='replace').strip()}"
            )
        return result

    def resolve(self, point):
        """A tag or commit name to a full commit id; None when it does not resolve."""

        def compute():
            r = self.git("rev-parse", "--verify", "--quiet", f"{point}^{{commit}}", check=False)
            sha = r.stdout.decode().strip()
            return sha if r.returncode == 0 and HEX40.match(sha) else None

        return self._cached(("resolve", point), compute)

    def default_ref(self):
        """refs/remotes/origin/<default>, or None when origin/HEAD is not set."""

        def compute():
            r = self.git("symbolic-ref", "-q", "refs/remotes/origin/HEAD", check=False)
            ref = r.stdout.decode().strip()
            return ref if r.returncode == 0 and ref.startswith("refs/remotes/") else None

        return self._cached(("default_ref",), compute)

    def files_at(self, sha, scope):
        """Paths under scope (a directory ending in '/' or a single file) at the commit."""

        def compute():
            r = self.git("ls-tree", "-r", "-z", "--name-only", sha, "--", scope.rstrip("/"))
            paths = [p.decode() for p in r.stdout.split(b"\0") if p]
            if scope.endswith("/"):
                return [p for p in paths if p.startswith(scope)]
            return [p for p in paths if p == scope]

        return list(self._cached(("files_at", sha, scope), compute))

    def working_files(self, scope):
        """Tracked and untracked (not ignored) working-tree paths under scope."""

        def compute():
            r = self.git(
                "ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", scope.rstrip("/")
            )
            paths = sorted({p.decode() for p in r.stdout.split(b"\0") if p})
            if scope.endswith("/"):
                return [p for p in paths if p.startswith(scope)]
            return [p for p in paths if p == scope]

        return list(self._cached(("working_files", scope), compute))

    def read_at(self, sha, path):
        """Blob bytes of <sha>:<path>, or None when absent."""
        return self._cached(("blob", sha, path), lambda: self._read_blob(sha, path))

    def _read_blob(self, sha, path):
        if self._batch is None:
            self._batch = subprocess.Popen(
                ["git", "-C", str(self.root), "cat-file", "--batch"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
            )
        self._batch.stdin.write(f"{sha}:{path}\n".encode())
        self._batch.stdin.flush()
        header = self._batch.stdout.readline().decode().strip().split()
        if len(header) == 2 and header[1] == "missing":
            return None
        if len(header) != 3 or header[1] != "blob":
            raise RuntimeError(f"unexpected cat-file answer for {sha}:{path}: {header}")
        size = int(header[2])
        data = self._batch.stdout.read(size)
        if self._batch.stdout.read(1) != b"\n":
            raise RuntimeError("cat-file framing error")
        return data

    def read_working(self, path):
        """Working-tree bytes, read once per process (the tree is one snapshot for one run; the
        test harness overlays its mutations on top of these reads)."""

        def compute():
            p = self.root / path
            return p.read_bytes() if p.is_file() else None

        return self._cached(("working", path), compute)

    def close(self):
        if self._batch is not None:
            self._batch.stdin.close()
            self._batch.wait()
            self._batch.stdout.close()
            self._batch = None


def run_check(rows, env):
    """Verify every register row against env. Returns (report lines, problems)."""
    lines, problems = [], []
    default = env.default_ref()
    if default is None:
        problems.append(
            "default-branch ref unavailable: refs/remotes/origin/HEAD is not set "
            "(repair: git remote set-head origin -a)"
        )
    for row in rows:
        sha = env.resolve(row.point)
        if sha is None:
            problems.append(f"{row.kind} {row.path}: freeze point {row.point} does not resolve to a commit")
            continue
        if row.kind == "frozen":
            frozen = env.files_at(sha, row.path)
            if not frozen:
                problems.append(f"frozen {row.path}: nothing under it at {row.point} ({sha[:7]})")
                continue
            bad = 0
            for path in frozen:
                expected = env.read_at(sha, path)
                actual = env.read_working(path)
                if actual is None:
                    problems.append(f"frozen {row.path}: missing {path}")
                    bad += 1
                elif actual != expected:
                    problems.append(f"frozen {row.path}: bytes changed in {path}")
                    bad += 1
            added = [p for p in env.working_files(row.path) if p not in set(frozen)]
            for path in added:
                problems.append(f"frozen {row.path}: added under frozen scope: {path}")
            lines.append(
                f"frozen {row.path} @ {row.point} ({sha[:7]}): {len(frozen)} files, "
                f"{len(frozen) - bad} byte-identical, {len(added)} added"
            )
        else:
            point_bytes = env.read_at(sha, row.path)
            if point_bytes is None:
                problems.append(f"append-only {row.path}: absent at {row.point} ({sha[:7]})")
                continue
            actual = env.read_working(row.path)
            if actual is None:
                problems.append(f"append-only {row.path}: missing from the working tree")
                continue
            ok_point = actual.startswith(point_bytes)
            if not ok_point:
                problems.append(
                    f"append-only {row.path}: its {row.point} version is not an exact prefix "
                    f"(earlier bytes changed or truncated)"
                )
            note = "default-branch prefix not checked"
            if default is not None:
                main_bytes = env.read_at(default, row.path)
                if main_bytes is None:
                    note = f"absent on {default}"
                elif actual.startswith(main_bytes):
                    note = f"{default} version ({len(main_bytes)} bytes) is an exact prefix"
                else:
                    problems.append(
                        f"append-only {row.path}: its {default} version is not an exact prefix "
                        f"(earlier bytes changed or truncated)"
                    )
                    note = f"{default} version is NOT a prefix"
            lines.append(
                f"append-only {row.path} @ {row.point} ({sha[:7]}): {len(point_bytes)} -> "
                f"{len(actual)} bytes, freeze-point prefix {'ok' if ok_point else 'BROKEN'}; {note}"
            )
    return lines, problems


def main(argv):
    if len(argv) != 2:
        print(__doc__)
        return 2
    root = Path(argv[1])
    env = GitEnv(root)
    rows = []
    try:
        rows = load_register((root / "PROGRAM" / "STATE.md").read_text(encoding="utf-8"))
        lines, problems = run_check(rows, env)
    except RegisterError as error:
        lines, problems = [], [f"register: {error}"]
    finally:
        env.close()
    print("== freeze register ==")
    for line in lines:
        print(line)
    for problem in problems:
        print(f"PROBLEM {problem}")
    frozen = sum(1 for r in rows if r.kind == "frozen")
    print(f"{len(rows)} register rows: {frozen} frozen, {len(rows) - frozen} append-only")
    print("RESULT:", "ALL PASS" if not problems else f"{len(problems)} PROBLEM(S)")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
