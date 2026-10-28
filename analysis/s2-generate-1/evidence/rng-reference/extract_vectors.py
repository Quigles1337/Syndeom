# Extract the two reference vectors from rand_xoshiro 0.3.0's retrieved sources (S-0081), check
# them against an implementation of ADR-0011 D4 written from the ADR's text, and print the Rust
# constants for engine/generate/tests/rng.rs. Reads the extracted files as data only.
import json
import re
import sys

src = sys.argv[1]
M = (1 << 64) - 1


def array_after(text, anchor):
    at = text.index(anchor)
    body = text[text.index("[", text.index("=", at)) + 1:]
    body = body[:body.index("]")]
    return [int(x) for x in re.findall(r"\d+", body)]


sm_text = open(src + "/src/splitmix64.rs", encoding="utf-8").read()
xo_text = open(src + "/src/xoshiro256starstar.rs", encoding="utf-8").read()
sm_test = sm_text[sm_text.index("fn reference()"):]
xo_test = xo_text[xo_text.index("fn reference()"):]
sm_seed = int(re.search(r"seed_from_u64\((\d+)\)", sm_test).group(1))
sm = array_after(sm_test, "let expected")
xo_seed_bytes = [int(x) for x in re.findall(r"\d+", xo_test[xo_test.index("from_seed(") + 10:xo_test.index("]);")])]
xo_state = [int.from_bytes(bytes(xo_seed_bytes[i:i + 8]), "little") for i in range(0, 32, 8)]
xo = array_after(xo_test, "let expected")


def splitmix64(x):
    x = (x + 0x9E3779B97F4A7C15) & M
    z = x
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M
    return x, z ^ (z >> 31)


def rotl(v, k):
    return ((v << k) | (v >> (64 - k))) & M


x, got_sm = sm_seed, []
for _ in sm:
    x, out = splitmix64(x)
    got_sm.append(out)
s, got_xo = list(xo_state), []
for _ in xo:
    got_xo.append((rotl((s[1] * 5) & M, 7) * 9) & M)
    t = (s[1] << 17) & M
    s[2] ^= s[0]; s[3] ^= s[1]; s[1] ^= s[2]; s[0] ^= s[3]; s[2] ^= t; s[3] = rotl(s[3], 45)

report = {
    "splitmix64": {"seed": sm_seed, "n": len(sm), "d4_python_matches": got_sm == sm},
    "xoshiro256starstar": {"state": xo_state, "n": len(xo), "d4_python_matches": got_xo == xo},
}
print(json.dumps(report))


def rust(name, values):
    lines = [f"const {name}: [u64; {len(values)}] = ["]
    lines += [f"    {v}," for v in values]
    lines.append("];")
    return "\n".join(lines)


with open(sys.argv[2], "w", encoding="utf-8", newline="\n") as f:
    f.write(rust("SPLITMIX64_REFERENCE", sm) + "\n\n" + rust("XOSHIRO256SS_REFERENCE", xo) + "\n")
with open(sys.argv[3], "w", encoding="utf-8", newline="\n") as f:
    json.dump({"splitmix64": {"seed": sm_seed, "outputs": sm},
               "xoshiro256starstar": {"state": xo_state, "outputs": xo}, "report": report}, f, indent=1)
    f.write("\n")
