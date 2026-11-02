"""Mutation checks for s2_grammar 0.8.0 (packet 15): apply one mutation at a time to
engine/generate/src/ineq.rs, run the ineq_chain tests, record whether they fail, then restore the
source byte for byte and verify its digest before the next mutation. A test that passed on its
first run is not yet evidence that it can fail (HANDOFF Session 53, BELIEF DELTAS).
Usage: python -I -X utf8 mutate15.py <log directory>"""
import hashlib
import os
import subprocess
import sys

SRC = "C:/Users/LEET/Syndeom/engine/generate/src/ineq.rs"
ENGINE = "C:/Users/LEET/Syndeom/engine"
LOGDIR = sys.argv[1]

# (name, the text replaced (its first occurrence), the replacement, what the mutation breaks)
MUTATIONS = [
    ("no-witness-filter",
     "let witness = witness(left, &terms)?;",
     "let witness = witness(left, &terms).unwrap_or(Witness { x: String::new(), y: String::new(), "
     "left: String::new(), right: String::new() });",
     "a perturbation that leaves the step true is admitted"),
    ("wrong-square",
     "increment: &[m(1, 2, 0), m(-2, 1, 1), m(1, 0, 2)],",
     "increment: &[m(1, 2, 0), m(-3, 1, 1), m(1, 0, 2)],",
     "the first square atom's increment is no longer (x - y)^2, so a faithful step can be false"),
    ("propagation-dropped",
     "flawed.e[k] = add(&flawed.e[k - 1], ATOMS[d.atoms[k - 1]].increment);",
     "flawed.e[k] = d.e[k].clone();",
     "propagated stops carrying the perturbation forward"),
]


def sha(b):
    return hashlib.sha256(b).hexdigest()


os.makedirs(LOGDIR, exist_ok=True)
original = open(SRC, "rb").read()
digest = sha(original)
print("source", digest, len(original), "bytes")
for name, old, new, why in MUTATIONS:
    text = original.decode("utf-8")
    assert text.count(old) >= 1, name
    mutated = text.replace(old, new, 1)
    assert mutated != text, name
    with open(SRC, "wb") as f:
        f.write(mutated.encode("utf-8"))
    try:
        r = subprocess.run(
            ["cargo", "test", "-p", "syndeom-generate", "--test", "ineq", "--quiet"],
            cwd=ENGINE, capture_output=True, text=True, encoding="utf-8", errors="replace",
        )
    finally:
        with open(SRC, "wb") as f:
            f.write(original)
    restored = sha(open(SRC, "rb").read())
    assert restored == digest, f"{name}: the source was not restored"
    out = (r.stdout + r.stderr).replace("\r\n", "\n").replace("\r", "\n")
    failed = sorted({l.split(" ")[0] for l in out.split("\n") if l.endswith("--- FAILED")}
                    | {l.strip() for l in out.split("\n") if l.startswith("    d") and "FAILED" not in l})
    with open(f"{LOGDIR}/mutation-{name}.log", "w", encoding="utf-8", newline="\n") as f:
        f.write(f"mutation: {name}\nbreaks: {why}\nreplaced: {old}\nwith: {new}\n"
                f"cargo test exit: {r.returncode}\nsource restored: {restored}\n\n{out}")
    print(name, "exit", r.returncode, "failing tests:", failed, "restored", restored == digest)
