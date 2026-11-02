"""Mutation checks for the FLAW-0011 test (evidence that it can fail): apply one mutation to a
copy of grammar.rs in place, run the grammar tests, restore the original bytes, and report.
Usage: python -I -X utf8 mutate.py <out dir>"""
import hashlib
import subprocess
import sys

SRC = "C:/Users/LEET/Syndeom/engine/generate/src/grammar.rs"
ENGINE = "C:/Users/LEET/Syndeom/engine"
OUT = sys.argv[1]
MUTATIONS = [
    ("instance-shift", "        instance: inj.instance + 1,\n", "        instance: inj.instance + 2,\n"),
    (
        "no-severity-check",
        "    if !matches!(inner.severity, Severity::Gap | Severity::Fatal) {\n",
        "    if false && !matches!(inner.severity, Severity::Gap | Severity::Fatal) {\n",
    ),
]

original = open(SRC, "rb").read()
digest = hashlib.sha256(original).hexdigest()
results = []
try:
    for name, old, new in MUTATIONS:
        text = original.decode("utf-8")
        assert text.count(old) == 1, name
        open(SRC, "wb").write(text.replace(old, new).encode("utf-8"))
        r = subprocess.run(
            ["cargo", "test", "-p", "syndeom-generate", "--test", "grammar", "--",
             "d5_flaw_0011_composes_one_gap_or_fatal_instance_recorded_at_the_statement"],
            cwd=ENGINE, capture_output=True, text=True, encoding="utf-8",
        )
        log = r.stdout + r.stderr
        open(f"{OUT}/mutation-{name}.log", "w", encoding="utf-8", newline="\n").write(log)
        panics = [l for l in log.splitlines() if "panicked at" in l or l.startswith("  left") or l.startswith(" right") or "assertion" in l]
        results.append((name, r.returncode, panics[:4]))
finally:
    open(SRC, "wb").write(original)
assert hashlib.sha256(open(SRC, "rb").read()).hexdigest() == digest, "restored"
for name, rc, panics in results:
    print(f"{name}: exit {rc} ({'FAILED as expected' if rc != 0 else 'PASSED - the test did not bite'})")
    for p in panics:
        print("   ", p[:220])
print("grammar.rs restored, sha256", digest)
