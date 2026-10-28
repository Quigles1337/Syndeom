//! `syndeom-rng/1` — ADR-0011 D4. The definition in the ADR is normative; SplitMix64 and
//! xoshiro256** are attributions. Everything is unsigned 64-bit, wrapping, and no floating-point
//! value ever enters a draw.

use syndeom_manifest::sha256_hex;

/// The `generator.rng` string every producer of this family records (ADR-0011 D1, D4).
pub const RNG_ID: &str = "syndeom-rng/1";

/// One SplitMix64 step on the state `x` (ADR-0011 D4).
pub fn splitmix64(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// `h(ℓ)`: the first 8 bytes of SHA-256(ℓ), read big-endian. Labels are ASCII (ADR-0011 D4).
pub fn label_hash(label: &str) -> u64 {
    assert!(label.is_ascii(), "rng labels are ASCII: {label:?}");
    let digest = sha256_hex(label.as_bytes());
    u64::from_str_radix(&digest[..16], 16).expect("sha256_hex is lowercase hex")
}

/// A labelled xoshiro256** stream seeded by SplitMix64 from `seed ⊕ h(label)`.
#[derive(Debug, Clone)]
pub struct Stream {
    s: [u64; 4],
}

impl Stream {
    pub fn new(seed: u64, label: &str) -> Self {
        let mut x = seed ^ label_hash(label);
        let s = [
            splitmix64(&mut x),
            splitmix64(&mut x),
            splitmix64(&mut x),
            splitmix64(&mut x),
        ];
        Self { s }
    }

    /// The next xoshiro256** output.
    pub fn next_u64(&mut self) -> u64 {
        let [s0, s1, s2, s3] = &mut self.s;
        let result = s1.wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let t = *s1 << 17;
        *s2 ^= *s0;
        *s3 ^= *s1;
        *s1 ^= *s2;
        *s0 ^= *s3;
        *s2 ^= t;
        *s3 = s3.rotate_left(45);
        result
    }

    /// `below(n)` = `(next() · n) ≫ 64` in 128-bit arithmetic: loop-free, platform-independent.
    pub fn below(&mut self, n: u64) -> u64 {
        ((u128::from(self.next_u64()) * u128::from(n)) >> 64) as u64
    }
}
