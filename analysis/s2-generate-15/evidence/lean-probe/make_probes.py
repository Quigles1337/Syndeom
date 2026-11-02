"""Write the ineq_chain Lean probes (NEXT step 3: probes first, before any design in the crate).

An authoring aid only (SD10: no result authority). It computes the merged normal forms of a few
hand-chosen inequality chains and writes each probe as a Lean file. The Lean files are what is
compiled (`run.sh`, one process at a time from `lean/`), and their outputs are the evidence.

A chain expression is a sum of monomials c * x^i * y^j and absolute-value terms c * |x|,
c * |y|, c * |x + y|, merged and ordered: monomials by total degree, then by the exponent of x,
both descending; then |x|, |y|, |x + y|. Each step adds one true atom (an instance of: squares
are non-negative; two-term AM-GM; the triangle inequality) and is closed by linarith with the
atom's Mathlib lemma as the hint.
"""

import os
import sys
from fractions import Fraction

HERE = os.path.dirname(os.path.abspath(__file__))

ABS = ["x", "y", "x+y"]
ABS_LEAN = {"x": "|x|", "y": "|y|", "x+y": "|x + y|"}

# atom id -> (increment, hint, informal fact, source consumed in e_0)
ATOMS = {
    "sq_x_sub_y": ({("m", 2, 0): 1, ("m", 1, 1): -2, ("m", 0, 2): 1}, "sq_nonneg (x - y)", {}),
    "sq_x_add_y": ({("m", 2, 0): 1, ("m", 1, 1): 2, ("m", 0, 2): 1}, "sq_nonneg (x + y)", {}),
    "sq_x_sub_1": ({("m", 2, 0): 1, ("m", 1, 0): -2, ("m", 0, 0): 1}, "sq_nonneg (x - 1)", {}),
    "sq_y_sub_1": ({("m", 0, 2): 1, ("m", 0, 1): -2, ("m", 0, 0): 1}, "sq_nonneg (y - 1)", {}),
    "sq_x_add_1": ({("m", 2, 0): 1, ("m", 1, 0): 2, ("m", 0, 0): 1}, "sq_nonneg (x + 1)", {}),
    "sq_y_add_1": ({("m", 0, 2): 1, ("m", 0, 1): 2, ("m", 0, 0): 1}, "sq_nonneg (y + 1)", {}),
    "amgm_x_y": ({("m", 2, 0): 1, ("m", 1, 1): -2, ("m", 0, 2): 1}, "two_mul_le_add_sq x y",
                 {("m", 1, 1): 2}),
    "amgm_x_1": ({("m", 2, 0): 1, ("m", 1, 0): -2, ("m", 0, 0): 1}, "two_mul_le_add_sq x 1",
                 {("m", 1, 0): 2}),
    "amgm_y_1": ({("m", 0, 2): 1, ("m", 0, 1): -2, ("m", 0, 0): 1}, "two_mul_le_add_sq y 1",
                 {("m", 0, 1): 2}),
    "tri_x_y": ({("a", "x"): 1, ("a", "y"): 1, ("a", "x+y"): -1}, "abs_add_le x y",
                {("a", "x+y"): 1}),
}


def add(e, d, k=1):
    out = dict(e)
    for key, c in d.items():
        out[key] = out.get(key, 0) + k * c
        if out[key] == 0:
            del out[key]
    return out


def order(e):
    monos = sorted((k for k in e if k[0] == "m"), key=lambda k: (-(k[1] + k[2]), -k[1]))
    abss = [("a", a) for a in ABS if ("a", a) in e]
    return monos + abss


def mono_lean(c, i, j):
    parts = []
    if abs(c) != 1 or i + j == 0:
        parts.append(str(abs(c)))
    if i:
        parts.append("x" if i == 1 else f"x ^ {i}")
    if j:
        parts.append("y" if j == 1 else f"y ^ {j}")
    return " * ".join(parts)


def term_lean(key, c):
    if key[0] == "m":
        return mono_lean(c, key[1], key[2])
    a = ABS_LEAN[key[1]]
    return a if abs(c) == 1 else f"{abs(c)} * {a}"


def lean(e):
    if not e:
        return "0"
    out = ""
    for n, key in enumerate(order(e)):
        c = e[key]
        t = term_lean(key, c)
        if n == 0:
            out += ("-" if c < 0 else "") + t
        else:
            out += (" - " if c < 0 else " + ") + t
    return out


def chain(atoms):
    e0 = {}
    for a in atoms:
        e0 = add(e0, ATOMS[a][2])
    es = [e0]
    for a in atoms:
        es.append(add(es[-1], ATOMS[a][0]))
    return es


def evaluate(e, x, y):
    v = Fraction(0)
    for key, c in e.items():
        if key[0] == "m":
            v += c * (x ** key[1]) * (y ** key[2])
        else:
            v += c * abs({"x": x, "y": y, "x+y": x + y}[key[1]])
    return v


def witness(left, right):
    """The first grid point (half-integers in [-2, 2], by |x| + |y|, then x, then y) at which
    left > right, or None."""
    pts = [Fraction(n, 2) for n in range(-4, 5)]
    grid = sorted(((x, y) for x in pts for y in pts), key=lambda p: (abs(p[0]) + abs(p[1]), p))
    for x, y in grid:
        if evaluate(left, x, y) > evaluate(right, x, y):
            return (x, y, evaluate(left, x, y), evaluate(right, x, y))
    return None


def lines(es, atoms, lemma_steps=(), override=None, claim_rhs=None):
    """The file's lines: lemma declarations for the steps in `lemma_steps` (1-based), then the
    main declaration. `override` maps a step index k to the expression its right side shows."""
    shown = list(es)
    for k, e in (override or {}).items():
        shown[k] = e
    n = len(atoms)
    out = ["import Mathlib.Basic.Real.Basic", "import Mathlib.Tactic.Linarith", ""]
    names = {}
    for j, k in enumerate(sorted(lemma_steps), start=1):
        name = f"syndeom_s2_grammar_ineq_chain_lemma_{j}"
        names[k] = name
        out.append(f"theorem {name} : ∀ x y : ℝ, {lean(shown[k - 1])} ≤ {lean(shown[k])} := by")
        out.append("  intro x y")
        out.append(f"  linarith [{ATOMS[atoms[k - 1]][1]}]")
        out.append("")
    rhs = lean(es[n]) if claim_rhs is None else claim_rhs
    out.append(f"theorem syndeom_s2_grammar_ineq_chain (x y : ℝ) : {lean(es[0])} ≤ {rhs} := by")
    for k in range(1, n + 1):
        proof = f"{names[k]} x y" if k in names else f"by linarith [{ATOMS[atoms[k - 1]][1]}]"
        if k == 1:
            out.append(f"  calc {lean(shown[0])} ≤ {lean(shown[1])} := {proof}")
        else:
            out.append(f"    _ ≤ {lean(shown[k])} := {proof}")
    return out


def write(name, text_lines, tail=True):
    body = "\n".join(text_lines) + "\n"
    if tail:
        body += "\n#print axioms syndeom_s2_grammar_ineq_chain\n"
    with open(os.path.join(HERE, name + ".lean"), "w", encoding="utf-8", newline="\n") as f:
        f.write(body)


def perturb(e, key, kind):
    out = dict(e)
    c = out[key]
    if kind == "sign":
        out[key] = -c
    elif kind in ("coefficient", "constant"):
        out[key] = c + (1 if c > 0 else -1)
    elif kind == "exponent-x":
        del out[key]
        raised = ("m", key[1] + 1, key[2])
        assert raised not in out
        out[raised] = c
    return out


def main():
    log = []
    short = ["amgm_x_y", "tri_x_y", "sq_x_sub_1", "amgm_x_y"]
    es = chain(short)
    write("s2i_ic_short", lines(es, short))

    longer = ["sq_x_add_y", "amgm_x_1", "tri_x_y", "amgm_x_y", "sq_y_sub_1", "amgm_y_1",
              "tri_x_y", "sq_x_sub_y", "amgm_x_y", "sq_x_add_1", "sq_y_add_1", "amgm_x_1"]
    el = chain(longer)
    write("s2i_ic_long", lines(el, longer, lemma_steps=(4, 9)))

    # FLAW-0001 shapes at the short chain's step 2 (tri), 3 (sq) and 4 (amgm, the last).
    def flawed(name, site, key, kind, propagated):
        e_site = perturb(es[site], key, kind)
        w = witness(es[site - 1], e_site)
        override = {site: e_site}
        if propagated:
            cur = e_site
            for k in range(site + 1, len(short) + 1):
                cur = add(cur, ATOMS[short[k - 1]][0])
                override[k] = cur
        nxt = None
        if not propagated and site < len(short):
            nxt = witness(e_site, es[site + 1])
        write(name, lines(es, short, override=override), tail=False)
        log.append(f"{name}: site {site}, {kind} on {key}; step {es[site - 1] and lean(es[site - 1])} <= "
                   f"{lean(e_site)}; site witness {w}; next-step witness {nxt}")

    # run 2: two long controls, the atoms in a fixed stride through the table, to bound the
    # cost of a long chain (the length dial's range is 1 to 256 steps, ADR-0011 D6)
    keys = list(ATOMS)
    for n in (64, 256):
        seq = [keys[(7 * k) % len(keys)] for k in range(n)]
        write(f"s2i_ic_control_{n}", lines(chain(seq), seq))

    flawed("s2i_ic_propagated_sign", 2, ("m", 1, 1), "sign", True)
    flawed("s2i_ic_local_sign", 2, ("m", 1, 1), "sign", False)
    flawed("s2i_ic_local_sign_abs", 2, ("a", "x"), "sign", False)
    flawed("s2i_ic_local_coefficient", 3, ("m", 1, 0), "coefficient", False)
    flawed("s2i_ic_propagated_exponent_last", 4, ("m", 2, 0), "exponent-x", True)
    flawed("s2i_ic_local_constant_true", 3, ("m", 0, 0), "constant", False)

    # the atoms and the tactics, one example each, with the expected failures marked
    atoms = [
        "import Mathlib.Basic.Real.Basic",
        "import Mathlib.Tactic.Linarith",
        "",
        "-- the three atom families, stated alone",
        "example (x y : ℝ) : 0 ≤ (x - y) ^ 2 := sq_nonneg (x - y)",
        "example (x y : ℝ) : 2 * x * y ≤ x ^ 2 + y ^ 2 := two_mul_le_add_sq x y",
        "example (x : ℝ) : 2 * x * 1 ≤ x ^ 2 + 1 ^ 2 := two_mul_le_add_sq x 1",
        "example (x y : ℝ) : |x + y| ≤ |x| + |y| := abs_add_le x y",
        "-- each family closing a merged chain step by linarith with its lemma as the hint",
        "example (x y : ℝ) : 3 * x ^ 2 - 2 * x * y + y ^ 2 ≤ 4 * x ^ 2 - 4 * x * y + 2 * y ^ 2 := by linarith [sq_nonneg (x - y)]",
        "example (x y : ℝ) : 4 * x * y + |x + y| ≤ x ^ 2 + 2 * x * y + y ^ 2 + |x + y| := by linarith [two_mul_le_add_sq x y]",
        "example (x y : ℝ) : 2 * x + y ^ 2 ≤ x ^ 2 + y ^ 2 + 1 := by linarith [two_mul_le_add_sq x 1]",
        "example (x y : ℝ) : x ^ 2 + 2 * |x + y| ≤ x ^ 2 + |x| + |y| + |x + y| := by linarith [abs_add_le x y]",
        "example (x y : ℝ) : 5 * x ^ 2 + |x| ≤ 6 * x ^ 2 - 2 * x + 1 + |x| := by linarith [sq_nonneg (x - 1)]",
        "-- nlinarith with the same hints",
        "example (x y : ℝ) : 4 * x * y + |x + y| ≤ x ^ 2 + 2 * x * y + y ^ 2 + |x + y| := by nlinarith [two_mul_le_add_sq x y]",
        "example (x y : ℝ) : x ^ 2 + 2 * |x + y| ≤ x ^ 2 + |x| + |y| + |x + y| := by nlinarith [abs_add_le x y]",
        "-- nlinarith with no hint (it squares the atoms it is given; here, none)",
        "example (x y : ℝ) : 2 * x * y ≤ x ^ 2 + y ^ 2 := by nlinarith [sq_nonneg (x - y)]",
        "example (x y : ℝ) : 2 * x * y ≤ x ^ 2 + y ^ 2 := by nlinarith",
        "-- EXPECTED TO FAIL: linarith without the hint",
        "example (x y : ℝ) : 2 * x * y ≤ x ^ 2 + y ^ 2 := by linarith",
        "-- EXPECTED TO FAIL: linarith without the triangle hint",
        "example (x y : ℝ) : x ^ 2 + 2 * |x + y| ≤ x ^ 2 + |x| + |y| + |x + y| := by linarith",
    ]
    with open(os.path.join(HERE, "s2i_ic_atoms.lean"), "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(atoms) + "\n")

    positivity = [
        "import Mathlib.Basic.Real.Basic",
        "import Mathlib.Tactic.Positivity",
        "",
        "example (x y : ℝ) : 0 ≤ (x - y) ^ 2 := by positivity",
        "example (x y : ℝ) : 0 ≤ 2 * (x - 1) ^ 2 + |x + y| := by positivity",
        "-- EXPECTED TO FAIL: positivity proves 0 ≤ e and 0 < e only, not a chain step a ≤ b",
        "example (x y : ℝ) : x ^ 2 ≤ x ^ 2 + (x - 1) ^ 2 := by positivity",
    ]
    with open(os.path.join(HERE, "s2i_ic_positivity.lean"), "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(positivity) + "\n")

    for name, imports in (("s2i_ic_imports_real_only", ["import Mathlib.Basic.Real.Basic"]),
                          ("s2i_ic_imports_linarith_only", ["import Mathlib.Tactic.Linarith"])):
        body = imports + ["", "example (x y : ℝ) : 4 * x * y + |x + y| ≤ x ^ 2 + 2 * x * y + y ^ 2 + |x + y| := by linarith [two_mul_le_add_sq x y]"]
        with open(os.path.join(HERE, name + ".lean"), "w", encoding="utf-8", newline="\n") as f:
            f.write("\n".join(body) + "\n")

    for line in log:
        print(line)
    return 0


if __name__ == "__main__":
    sys.exit(main())
