# syndeom-generate — the first S2 generator family (ADR-0011)

Deterministic seeded templates and grammars (the standing ruling of Session 2). This crate holds:

- `src/plan.rs` — the plan, the one shared interface behind the four S2.1 modules, and its two
  pure renderers: `syndeom-informal-md/1` and `syndeom-lean/1` (D2); from 0.3.0 a step may be a
  lemma (`Lemma N.` in the informal file, its own declaration before the main one in the formal
  file);
- `src/rng.rs` — `syndeom-rng/1`, SplitMix64-seeded xoshiro256** streams per label (D4);
- `src/template.rs` — the producer `s2_template`, in its versions 0.1.0, 0.2.0 and 0.3.0: seed
  records, the config and its refusals (D3), the injector (D5), ground truth by construction
  (D7), and the manifest with checks K and T (D8, D9);
- `src/harness.rs` — the Lean harness: `lake env lean` from `lean/`, one process at a time, and
  an audit tail printing `#print axioms` of the main declaration and the direct used-constants
  of every constant the file defined (D8; the T check's listing, from 0.3.0);
- `src/provenance.rs`, `build.rs` and `input-set.txt` — the running binary's provenance: the
  hashes of every file of the producer's input set, compiled in at build time (D9 as amended by
  Al's ruling 5 on PR 26; FINDING-005 closed);
- `src/main.rs` — the CLI `generate`, `regenerate` and `verify` (D9);
- `seeds/` — the seed records, compiled into the binary;
- `tests/` — the RNG vectors, the D8 writer test (Golden A and Golden B), property and refusal
  tests, the provenance test, and the goldens (`tests/goldens/`, with their own README).

## Producer versions

- **`s2_template` 0.1.0** (Session 40): seeds `nat_le_sq` (ADR-0001 D8's theorem),
  `sq_plus_one_ge_two_x` (FLAWS.md FLAW-0001's example: `propagated` and `local`) and
  `n_sq_mod_four` (FLAW-0014's gap example); flaw counts per record.
- **`s2_template` 0.2.0** (Session 41): the 0.1.0 seeds and four more, each from its FLAWS.md
  0.1.0 example — `continuity_is_not_uniform` (FLAW-0002, the quantifier swap, gap),
  `x_over_abs_x` (FLAW-0014, the fatal case), `rolle_abs` (FLAW-0005 `violated`, fatal) and
  `horses` (FLAW-0006 `step-above-base`, fatal; the mode Al's rulings on PR 27 added to D5); flaw
  counts per instance (Al's ruling 3 on PR 26); build-time input hashes.
- **`s2_template` 0.3.0** (Session 42; the version `generate` emits): the 0.2.0 seeds and
  `n_lt_two_pow` (FLAW-0009's helper `n < 2 ^ n`, a lemma step; FLAW-0007's unused lemma
  `n² ≥ n`); lemma steps (D2); check T (D5, D8) for FLAW-0009 `axiom` (`truth` true, gap; false,
  fatal) and FLAW-0007 `unused` (cosmetic); records only in the files a class manifests in
  (FLAWS.md's *Manifests in*: FLAW-0009 is formal-only).

The crate regenerates all three versions: the goldens select the version their manifests name,
and `regenerate` and `verify` accept any of them. Every seed states its `source`, never under
`groundtruth/` (Al's ruling 1 of 2026-10-27).

## Seeds whose statement is false (0.2.0)

Three of the four FLAWS.md examples state false theorems: Rolle's theorem applied to `|x|`, the
horses, and `x / |x| = ±1` (in Lean `0 / |0| = 0`). Such a seed records `truth.value: false`
with the Lean declaration that proves the negation, and names its `counterexample`. It has no
control (ADR-0001's consistency rule admits no flawless artifact of a false statement): a config
that commands no flaw is refused (exit 2). Its style's steps are the convincing argument as the
example writes it, and its flaw material is `intrinsic`: nothing is replaced, and the record's
`dials` carry the site, the text as written, `replaced: null` and the counterexample. This reading
is flagged for Al in ADR-0011 *Amendments*.

## Flaw material and confirmation (S2.2)

- Template material is a whole-argument variant of one style. The flawed step list names the
  faithful step each step stands in for (`from`), the flaw site and the anchor text. The injector
  selects the material the spec names, drawing from the stream `injector/<class>/1`, and writes
  every replaced or removed original into the record's `dials`. One instance per artifact: two
  whole-argument variants cannot be made disjoint, so a second spec is refused (D5).
- Insertion material (0.3.0; FLAW-0007 `unused`) inserts one step that stands in for no
  faithful step (`from: null`); `site` is that step's index in the flawed argument, the record's
  span is the whole step, and `replaced` is null. At a lemma step the formal record's `inserted`
  and `replaced` are the declaration lines as the writer emits them, and `declaration` names the
  lemma; its location is that declaration at its first line.
- **Counts** (0.2.0): `n_flaws_total`, `n_flaws_<NNNN>` and `flaws_confirmed` count instances. A
  pair's two records of one instance share its `instance`; the instance is confirmed only when
  both records pass their checks (Al's owner note of 2026-10-28).
- **Checks:**
  - informal records: the recorded span holds exactly the recorded text and, where something was
    replaced, differs from it;
  - formal `YES-KERNEL` records: check K, meaning the build fails and its first error lies in the
    site's fragment lines;
  - formal `YES-TOOLING` records (0.3.0): check T, meaning the build passes and the audit tail's
    listing satisfies the class's condition — FLAW-0007 `unused`: the inserted lemma is among the
    file's own constants and outside the transitive used-constants of the main declaration,
    closed over the file's own constants (Lean's auxiliary `_proof_N` constants included);
    FLAW-0009: `#print axioms` of the main declaration lists the axiom.

  No other check exists yet, so no other class is confirmed by this version.
- **Not yet:** `s2_grammar`, padding (FLAW-0013; a non-zero `padding_ratio` is refused), the T
  check for FLAW-0013's formal declarations, FLAW-0009 `native_decide`, FLAW-0012 as a commanded
  class, and the fidelity sweep. They come in later sessions (D11).

## CLI

```
syndeom-generate generate --config <config.json> --seed <u64> --root <repo> \
    --engine-commit <40-hex> --receipt-dir <new directory outside the repo> [--dry-run]
syndeom-generate regenerate --manifest <corpora/<id>/manifest.json> --root <repo>
syndeom-generate verify --artifact <corpora/<id>> --root <repo> --receipt-dir <new directory>
```

`generate` refuses (exit 2) unless the engine commit is an exact commit and an ancestor of
`HEAD` whose blobs for the input set (`input-set.txt`: `engine/generate/**`,
`engine/manifest/**`, `lean/Syndeom/Seeds/**`, `engine/Cargo.toml`, `engine/Cargo.lock`,
`rust-toolchain.toml`, `lean/lean-toolchain`, `lean/lakefile.toml`, `lean/lake-manifest.json`)
equal the working tree **and** hash to the values compiled into the running binary; a file the
commit holds that the binary did not compile, or that the binary compiled and the commit lacks,
is refused too, as is a runtime `rustc` other than the compiled one or a project-local Cargo
override. The receipt records what was checked under `provenance`.

It compiles a formal file from a copy in the receipt directory, so no audit file ever lands
under `corpora/`. It then writes `corpora/<id>/` with exclusive creation. Identical bytes exit
0; different bytes under the same id exit 3. With `--dry-run`, the outputs go to the receipt
directory instead.

A failed confirmation is written with its FAIL verdict and exit 0.
