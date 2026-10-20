"""Check the reviewed two-line Mathlib source interval, not general Lean semantics.

Source intervals are one-based and inclusive at both ends. Canonical declaration
locations are one-based. Exact LF Git-blob bytes are required, not checkout bytes.
"""
import copy
import hashlib

RAW_HASH = "92a5630a19bff74ade4088f1253cf75b0ac8143c002e6b604ac0b77a39c9ecf6"
DECLARATION = "theorem infinite_setOfPred_prime : { p | Prime p }.Infinite :="
BODY = "  Set.infinite_of_not_bddAbove not_bddAbove_setOfPred_prime"
ALIAS = "theorem syndeom_s1_2_judson_primes : Set.Infinite {p : ℕ | Nat.Prime p} :="


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check(raw, canonical, archive, child):
    require(len(raw) == 5328 and hashlib.sha256(raw).hexdigest() == RAW_HASH,
            "not the reviewed pinned Git blob")
    require(b"\r" not in raw, "raw source must retain LF Git-blob bytes")
    mapping = child["recipe"]["source_map"]["formal"]
    interval = mapping["source_lines_one_based"]
    require(isinstance(interval, list) and len(interval) == 2
            and all(type(n) is int for n in interval), "invalid source interval")
    first, last = interval
    lines = raw.decode("utf-8").splitlines()
    require(1 <= first <= last <= len(lines), "source interval out of bounds")
    require(lines[first - 1:last] == [DECLARATION, BODY],
            "source interval must contain the selected declaration and its complete reviewed proof body")
    require(interval == [27, 28], "unexpected reviewed source location")
    require(archive["files"]["formal"]["locator"]
            == f"Nat.infinite_setOfPred_prime, lines {first}-{last}",
            "archive locator disagrees with the checked interval")
    require(mapping["source_file"] == "PrimeFin.lean"
            and mapping["source_declaration"] == "Nat.infinite_setOfPred_prime",
            "wrong source identity")
    require(mapping["canonical_declaration"] == "syndeom_s1_2_judson_primes",
            "wrong canonical declaration")
    line = mapping["canonical_line"]
    target = canonical.decode("utf-8").splitlines()
    require(type(line) is int and line == 3 and target[line - 1:line + 1]
            == [ALIAS, "  Nat.infinite_setOfPred_prime"],
            "canonical location must contain the reviewed alias declaration and proof term")
    return {"source_interval_one_based_inclusive": interval, "canonical_line_one_based": line,
            "declaration": DECLARATION, "proof_body": BODY, "raw_sha256": RAW_HASH}


def check_with_controls(raw, canonical, archive, child):
    result = {"positive": check(raw, canonical, archive, child), "negative_controls": []}
    for name, interval, canonical_line in [
        ("original off-by-one range with unchanged valid raw hash", [28, 29], 3),
        ("declaration without proof body", [27, 27], 3),
        ("wrong canonical declaration line", [27, 28], 4),
    ]:
        a, c = copy.deepcopy(archive), copy.deepcopy(child)
        a["files"]["formal"]["locator"] = (
            f"Nat.infinite_setOfPred_prime, lines {interval[0]}-{interval[1]}")
        c["recipe"]["source_map"]["formal"]["source_lines_one_based"] = interval
        c["recipe"]["source_map"]["formal"]["canonical_line"] = canonical_line
        try:
            check(raw, canonical, a, c)
        except ValueError as exc:
            result["negative_controls"].append({"control": name, "rejected": True, "reason": str(exc)})
        else:
            raise AssertionError(f"negative locator control was accepted: {name}")
    return result
