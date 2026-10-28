# syndeom-generate — the first S2 generator family (ADR-0011)

Deterministic seeded templates and grammars (the standing ruling of Session 2). This crate holds:

- `src/plan.rs` — the plan, the one shared interface behind the four S2.1 modules, and its two
  pure renderers: `syndeom-informal-md/1` and `syndeom-lean/1` (D2);
- `src/rng.rs` — `syndeom-rng/1`, SplitMix64-seeded xoshiro256** streams per label (D4);
- `src/template.rs` — the producer `s2_template` 0.1.0: seed records, the config and its
  refusals (D3), the injector (D5), ground truth by construction (D7), and the manifest (D8, D9);
- `src/harness.rs` — the Lean harness: `lake env lean` from `lean/`, one process at a time, and
  an audit tail printing `#print axioms` (D8);
- `src/main.rs` — the CLI `generate`, `regenerate` and `verify` (D9);
- `seeds/` — the seed records, compiled into the binary;
- `tests/` — the RNG vectors, the D8 writer test (Golden A and Golden B), property and refusal
  tests, and the goldens (`tests/goldens/`, with their own README).

## Scope of 0.1.0 (the first implementation session, ADR-0011 D11)

- **Seeds:** `nat_le_sq` (ADR-0001 D8's theorem), `sq_plus_one_ge_two_x` (FLAWS.md FLAW-0001's
  example: `propagated` and `local`) and `n_sq_mod_four` (FLAW-0014's gap example). Each states
  its `source`, never under `groundtruth/` (Al's ruling 1 of 2026-10-27).
- **Flaw material:** a whole-argument variant of one style. The flawed step list names the faithful
  step each step stands in for (`from`), the flaw site and the anchor text. The injector selects
  the material the spec names, drawing from the stream `injector/<class>/1`, and writes every
  replaced or removed original into the record's `dials`. One instance per artifact: two
  whole-argument variants cannot be made disjoint, so a second spec is refused (D5).
- **Confirmation (S2.2):**
  - informal records: the recorded span holds exactly the inserted text, and it differs from
    what it replaced;
  - formal `YES-KERNEL` records: check K, meaning the build fails and its first error lies in
    the site's fragment lines.

  No other check exists yet, so no other class is confirmed by this version.
- **Not yet:** `s2_grammar`, padding (FLAW-0013; a non-zero `padding_ratio` is refused), lemma
  steps, the T check and the used-constants audit (FLAW-0007, FLAW-0009, FLAW-0013), and the
  fidelity sweep. They come in later sessions (D11).

## CLI

```
syndeom-generate generate --config <config.json> --seed <u64> --root <repo> \
    --engine-commit <40-hex> --receipt-dir <new directory outside the repo> [--dry-run]
syndeom-generate regenerate --manifest <corpora/<id>/manifest.json> --root <repo>
syndeom-generate verify --artifact <corpora/<id>> --root <repo> --receipt-dir <new directory>
```

`generate` refuses (exit 2) unless the engine commit is an ancestor of `HEAD` whose blobs for
the input set (`engine/generate/**`, `engine/manifest/**`, `engine/Cargo.toml`,
`engine/Cargo.lock`, `rust-toolchain.toml`, `lean/lean-toolchain`, `lean/lakefile.toml`,
`lean/lake-manifest.json`, `lean/Syndeom/Seeds/**`) equal the working tree.

It compiles a formal file from a copy in the receipt directory, so no audit file ever lands
under `corpora/`. It then writes `corpora/<id>/` with exclusive creation. Identical bytes exit
0; different bytes under the same id exit 3. With `--dry-run`, the outputs go to the receipt
directory instead.

A failed confirmation is written with its FAIL verdict and exit 0.
