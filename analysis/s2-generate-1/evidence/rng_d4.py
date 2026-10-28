# Independent implementation of ADR-0011 D4 (`syndeom-rng/1`), written from the ADR's text, to
# cross-check the Rust crate's conformance vectors. Standard library only.
import hashlib
import json

M = (1 << 64) - 1


def splitmix64(x):
    x = (x + 0x9E3779B97F4A7C15) & M
    z = x
    z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M
    return x, z ^ (z >> 31)


def rotl(v, k):
    return ((v << k) | (v >> (64 - k))) & M


def stream(seed, label):
    h = int.from_bytes(hashlib.sha256(label.encode("ascii")).digest()[:8], "big")
    x = seed ^ h
    s = []
    for _ in range(4):
        x, out = splitmix64(x)
        s.append(out)
    while True:
        result = (rotl((s[1] * 5) & M, 7) * 9) & M
        t = (s[1] << 17) & M
        s[2] ^= s[0]
        s[3] ^= s[1]
        s[1] ^= s[2]
        s[0] ^= s[3]
        s[2] ^= t
        s[3] = rotl(s[3], 45)
        yield result


out = {}
for seed in (0, 1, 7):
    g = stream(seed, "test")
    out[str(seed)] = ["0x%016x" % next(g) for _ in range(4)]
g = stream(7, "injector/FLAW-0001/1")
first = next(g)
out["below"] = {"seed": 7, "label": "injector/FLAW-0001/1", "next": "0x%016x" % first,
                "below_3": (first * 3) >> 64}
print(json.dumps(out, indent=1))
