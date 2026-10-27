"""Both directions of the freeze check (ADR-0012, Amendments 1), against the live register and the
real Git objects, with the working tree mutated in memory: a changed frozen byte, a removed or
added frozen file, and an earlier byte of an append-only file fail; an append passes; editing,
removing or adding anything outside the register passes; a malformed register and a missing
default-branch ref fail loudly. Standard library only; nothing is written.

    python -I -X utf8 scripts/check_freeze_test.py <repo-root>
"""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import check_freeze  # noqa: E402

ROOT = None

OUTSIDE = [
    "PROGRAM/SYNDEOM-PROGRAM.md",
    "PROGRAM/DECISIONS/ADR-0005.md",
    "PROGRAM/DECISIONS/ADR-0012.md",
    "PROGRAM/SIBLING-MAP-2026-10.md",
    "PROGRAM/STATE.md",
    "PROGRAM/CLAIMS.md",
    "PROGRAM/FLAWS.md",
    "README.md",
    "LICENSE",
    "scripts/check.sh",
    "scripts/check_registers.py",
    "lean/Audit.lean",
    "engine/Cargo.toml",
    "engine/Cargo.lock",
    "engine/manifest/src/lib.rs",
    "engine/ingest/examples/support/effective.rs",
    "groundtruth/HARVEST-LOG.md",
    "groundtruth/README.md",
    "analysis/README.md",
    ".github/workflows/check.yml",
]


class Mutated:
    """A GitEnv whose working tree is overlaid with in-memory changes: bytes per path, or None
    for a removed file. Git objects (the freeze points and the default branch) are the real ones."""

    def __init__(self, base, changes=None, default_ref=None, drop_default=False):
        self.base = base
        self.changes = dict(changes or {})
        self._default = default_ref
        self._drop_default = drop_default

    def resolve(self, point):
        return self.base.resolve(point)

    def default_ref(self):
        if self._drop_default:
            return None
        return self._default or self.base.default_ref()

    def files_at(self, sha, scope):
        return self.base.files_at(sha, scope)

    def read_at(self, sha, path):
        return self.base.read_at(sha, path)

    def working_files(self, scope):
        present = set(self.base.working_files(scope))
        for path, data in self.changes.items():
            inside = path.startswith(scope) if scope.endswith("/") else path == scope
            if not inside:
                continue
            if data is None:
                present.discard(path)
            else:
                present.add(path)
        return sorted(present)

    def read_working(self, path):
        if path in self.changes:
            return self.changes[path]
        return self.base.read_working(path)


class FreezeCheck(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.env = check_freeze.GitEnv(ROOT)
        cls.rows = check_freeze.load_register(
            (Path(ROOT) / "PROGRAM" / "STATE.md").read_text(encoding="utf-8")
        )
        cls.frozen = [r for r in cls.rows if r.kind == "frozen"]
        cls.append_only = [r for r in cls.rows if r.kind == "append-only"]
        cls.scopes = [r.path for r in cls.rows]

    @classmethod
    def tearDownClass(cls):
        cls.env.close()

    def problems(self, changes=None, **kw):
        _, problems = check_freeze.run_check(self.rows, Mutated(self.env, changes, **kw))
        return problems

    def first_frozen_file(self, row):
        sha = self.env.resolve(row.point)
        files = self.env.files_at(sha, row.path)
        self.assertTrue(files, row.path)
        return files[0]

    def test_register_has_both_kinds_and_resolving_freeze_points(self):
        self.assertTrue(self.frozen)
        self.assertTrue(self.append_only)
        for row in self.rows:
            self.assertIsNotNone(self.env.resolve(row.point), row.point)
        self.assertIsNotNone(self.env.default_ref())

    def test_clean_working_tree_passes(self):
        self.assertEqual(self.problems(), [])

    def test_changed_frozen_byte_fails_in_every_frozen_scope(self):
        for row in self.frozen:
            path = self.first_frozen_file(row)
            original = self.env.read_working(path)
            self.assertIsNotNone(original, path)
            for label, mutated in [
                ("appended", original + b" "),
                ("flipped", bytes([original[0] ^ 1]) + original[1:]),
                ("truncated", original[:-1]),
                ("rewritten", b"any text at all\n"),
            ]:
                problems = self.problems({path: mutated})
                self.assertTrue(
                    any(f"bytes changed in {path}" in p for p in problems), f"{path} {label}"
                )

    def test_removed_or_added_frozen_file_fails(self):
        for row in self.frozen:
            path = self.first_frozen_file(row)
            self.assertTrue(any(f"missing {path}" in p for p in self.problems({path: None})), path)
            if row.path.endswith("/"):
                added = row.path + "new-receipt.json"
                self.assertTrue(
                    any(f"added under frozen scope: {added}" in p for p in self.problems({added: b"{}"})),
                    added,
                )

    def test_earlier_byte_of_append_only_file_fails_and_append_passes(self):
        default = self.env.default_ref()
        for row in self.append_only:
            original = self.env.read_working(row.path)
            point = self.env.read_at(self.env.resolve(row.point), row.path)
            main = self.env.read_at(default, row.path)
            self.assertIsNotNone(point, row.path)
            self.assertTrue(original.startswith(point), row.path)
            self.assertTrue(main is None or original.startswith(main), row.path)
            # Earlier bytes: the first byte, a byte inside the freeze-point version, a byte inside
            # the default-branch version, and a truncation below either length.
            positions = {0, len(point) // 2, len(point) - 1}
            if main is not None:
                positions |= {len(main) // 2, len(main) - 1}
            for at in sorted(positions):
                flipped = original[:at] + bytes([original[at] ^ 1]) + original[at + 1 :]
                problems = self.problems({row.path: flipped})
                self.assertTrue(
                    any(row.path in p and "not an exact prefix" in p for p in problems),
                    f"{row.path} byte {at}",
                )
            shortest = len(point) if main is None else min(len(point), len(main))
            problems = self.problems({row.path: original[: shortest - 1]})
            self.assertTrue(any(row.path in p and "not an exact prefix" in p for p in problems))
            self.assertEqual(self.problems({row.path: original + b"\nappended entry\n"}), [])
            self.assertTrue(any("missing" in p for p in self.problems({row.path: None})))

    def test_editing_removing_or_adding_outside_the_register_passes(self):
        changes = {}
        for path in OUTSIDE:
            self.assertFalse(
                any(path == s or (s.endswith("/") and path.startswith(s)) for s in self.scopes),
                f"{path} is inside the register",
            )
            original = self.env.read_working(path)
            self.assertIsNotNone(original, path)
            changes[path] = bytes([original[0] ^ 1]) + original[1:] + b" "
        changes["engine/generate/src/lib.rs"] = b"// an S2 crate\n"
        changes["PROGRAM/DECISIONS/ADR-0013.md"] = b"# ADR-0013\n"
        changes["groundtruth/HARVEST-LOG.md"] = self.env.read_working("groundtruth/HARVEST-LOG.md") + b"| row |\n"
        self.assertEqual(self.problems(changes), [])
        removed = {path: None for path in OUTSIDE if path != "PROGRAM/STATE.md"}
        self.assertEqual(self.problems(removed), [])

    def test_malformed_register_fails(self):
        good = "## Freeze register\n\n| Path | Kind | Freeze point | Derived from |\n|---|---|---|---|\n"
        self.assertEqual(
            len(check_freeze.load_register(good + "| `analysis/s1-4/` | frozen | `gate-s1-registration` | x |\n")),
            1,
        )
        for bad in [
            "| `analysis/s1-4/` | pinned | `gate-s1-registration` | x |\n",
            "| analysis/s1-4/ | frozen | `gate-s1-registration` | x |\n",
            "| `analysis/s1-4/` | frozen | gate-s1-registration | x |\n",
            "| `PROGRAM/HANDOFF.md/` | append-only | `gate-s1-registration` | x |\n",
            "| `../x` | frozen | `gate-s1-registration` | x |\n",
            "| `analysis/s1-4/` | frozen | `gate-s1-registration` | x |\n"
            "| `analysis/s1-4/` | frozen | `gate-s1-registration` | twice |\n",
        ]:
            with self.assertRaises(check_freeze.RegisterError, msg=bad):
                check_freeze.load_register(good + bad)
        with self.assertRaises(check_freeze.RegisterError):
            check_freeze.load_register("## Something else\n\n| `a/` | frozen | `t` | x |\n")
        with self.assertRaises(check_freeze.RegisterError):
            check_freeze.load_register(good)

    def test_unresolvable_freeze_point_and_missing_default_ref_fail(self):
        rows = [check_freeze.Row("analysis/s1-4/", "frozen", "no-such-tag-anywhere", "x")]
        _, problems = check_freeze.run_check(rows, Mutated(self.env))
        self.assertTrue(any("does not resolve" in p for p in problems))
        problems = self.problems(drop_default=True)
        self.assertTrue(any("refs/remotes/origin/HEAD is not set" in p for p in problems))


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    ROOT = sys.argv[1]
    unittest.main(argv=[sys.argv[0]], verbosity=1)
