# syndeom-generate — the first S2 generator family (ADR-0011)

Deterministic seeded templates and grammars (the standing ruling of Session 2). This crate holds:

- `src/plan.rs` — the plan, the one shared interface behind the four S2.1 modules, and its two
  pure renderers: `syndeom-informal-md/1` and `syndeom-lean/1` (D2); from 0.3.0 a step may be a
  lemma (`Lemma N.` in the informal file, its own declaration before the main one in the formal
  file); from 0.4.0 a step may be FLAW-0013 padding (a whole paragraph in the informal file, its
  matching unused declaration before the main one in the formal file); from 0.6.0 a step may
  carry `premises` (what it cites, with the citing phrase), which render to nothing and form
  the plan's premise graph;
- `src/rng.rs` — `syndeom-rng/1`, SplitMix64-seeded xoshiro256** streams per label (D4);
- `src/template.rs` — the producer `s2_template`, in its versions 0.1.0, 0.2.0, 0.3.0, 0.4.0,
  0.5.0 and 0.6.0: seed records, the config and its refusals (D3), the injector (D5; from 0.5.0
  with FLAW-0004's hedge lexicon and FLAW-0011 as a commanded composite; from 0.6.0 with
  FLAW-0010's fictional-reference lexicon and FLAW-0003's `direct` and `cycle`), the padding
  module (D2 `pad`, D5 FLAW-0013; 0.4.0), ground truth by construction (D7), and the manifest
  with checks K and T (D8, D9);
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
- **`s2_template` 0.3.0** (Session 42): the 0.2.0 seeds and `n_lt_two_pow` (FLAW-0009's helper
  `n < 2 ^ n`, a lemma step; FLAW-0007's unused lemma `n² ≥ n`); lemma steps (D2); check T (D5,
  D8) for FLAW-0009 `axiom` (`truth` true, gap; false, fatal) and FLAW-0007 `unused` (cosmetic);
  records only in the files a class manifests in (FLAWS.md's *Manifests in*: FLAW-0009 is
  formal-only).
- **`s2_template` 0.4.0** (Session 43): the 0.3.0 seeds and
  `sqrt2_irrational` (the FLAWS.md FLAW-0013 and FLAW-0011 example statement, as the
  number-theoretic core `p ^ 2 ≠ 2 * q ^ 2` for coprime `p`, `q`, in five aligned steps), the
  first seed with padding material (seven paragraphs: four restated definitions and three
  re-derivations of `2 | p² ⇒ 2 | p`); the padding module (D2 `pad`, D5 FLAW-0013, Options 8);
  check T for the padding declarations; `padding_ratio` measured.
- **`s2_template` 0.5.0** (Session 44): the 0.4.0 seeds, with
  FLAW-0004 material on `sqrt2_irrational` (`hidden_step: routine-true` at step 3: the FLAWS.md
  FLAW-0011 example's `2 | p² ⇒ 2 | p` hedged, the parity argument dropped); FLAW-0004 (D5: the
  hedge drawn from the fixed lexicon and prefixed to the bare claim, the justification deleted,
  the aligned tactic lines omitted so the kernel reports an unsolved goal at the site, never
  `sorry`; `N/A` for an informal-only target); FLAW-0011 as a commanded composite (D5:
  `embedded`, one instance in this version — of FLAW-0001 through FLAW-0010 and of severity gap
  or fatal, Al's rulings 2 and 3 on PR 30 — on a true statement; the composite's own record at
  the statement, its severity and `lean_catchable` inherited); and the seed rule that no style
  step carries a padding header (FINDING-014).
- **`s2_template` 0.6.0** (Session 45; the version `generate` emits): the 0.5.0 seeds, with
  FLAW-0010 material on `sqrt2_irrational` (sub-variant `a`, `claim: true`, at step 3: the
  FLAW-0004 material's bare claim, its justification replaced by a fictional reference), and
  `bounded_monotone_converges` (FLAWS.md's FLAW-0003 example: every monotone real sequence that
  is bounded above converges; two aligned steps on the supremum, with a witness module audited
  by the gate), carrying FLAW-0003 `direct` (`form: restated-goal` and `contrapositive`) and
  `cycle` (`cycle_length: 2`) material; FLAW-0010 `a` (D5: the reference drawn from the fixed
  lexicon of obviously fictional tokens and prefixed as `By <reference>` to the bare claim, the
  formal file unchanged, `N/A`); FLAW-0003 `direct` (D5: the goal, or its contrapositive, as
  Lemma 1 whose formal proof is a forward reference to the main declaration, check K) and
  `cycle` (D5: Lemma 1 cites the theorem and the key step cites Lemma 1; informal-only, `NO`;
  checked on the plan's premise graph and the citation phrases' spans; the formal file keeps a
  faithful proof).

The crate regenerates all six versions: the goldens select the version their manifests name,
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
- **Padding** (0.4.0; FLAW-0013): commanded by one FLAW-0013 spec together with a positive
  `padding_ratio`, and by nothing else (a ratio without the spec, or the spec with ratio 0, is
  refused). The spec's dials are `kinds` (a non-empty subset of `restated-definition`,
  `re-derivation`, `unused-lemma`, `verbose-rewrite` — the seed must carry material of the kinds
  named), `count` (how many paragraphs, a positive integer: the generation decision is an
  integer, and the ratio is compared only in `measure`, D4) and `placement` (`before`: every
  paragraph before the first step, in drawn order, as the FLAWS.md example's "preceded by";
  `drawn`: each paragraph at a position drawn from the `padding` stream, before a step of the
  plan as it then stands). The paragraphs are drawn without replacement from the `padding`
  stream. Each paragraph is its own instance with a record in each file the artifact has: the
  informal record spans the whole step, `[0, len)`, `NO`; the formal record is the matching
  declaration at its first line, `YES-TOOLING`. The records' dials carry the spec's dials, the
  config's ratio (`ratio`), the paragraph drawn (`paragraph`, its index in the seed's material,
  and `kind`), the resolved `site`, `inserted` (the text, or the declaration lines) and
  `replaced: null`; the unpadded argument is recoverable by deleting the recorded steps. A seed
  with padding material has it at the seed level, usable with every style; a seed whose
  statement is false has none. Padding and an injected flaw in one artifact are a later version.
- **FLAW-0004** (0.5.0): replacement material whose `hidden` names the bare claim, the
  justification and the omitted tactic lines, with the one dial `hidden_step` (`routine-true`
  cosmetic, `nontrivial-true` gap, `false` fatal; only `routine-true` has material in 0.5.0).
  The injector draws the hedge from the fixed lexicon (`Clearly`, `It is easy to see that`,
  `Obviously`, `By a standard argument`, `One checks that`) from the stream
  `injector/FLAW-0004/1` after the material, and the site step becomes the hedge, a space and
  the claim; the formal fragment is the faithful one without the omitted block (the `have`
  header and the `obtain` stay). The records carry `hedge` (the index), `hedge_text`, the
  `justification` and `claim` (informal) or the `omitted` lines (formal), `inserted`,
  `replaced` and the originals. The target `formal` is refused (FLAWS.md: manifests in
  informal); for a pair the instance is `YES-KERNEL`, for an informal-only artifact `N/A`.
- **FLAW-0011** (0.5.0): commanded with the one dial `embedded`, a list of FlawSpecs — exactly
  one in this version, since the materials are whole-argument variants — of FLAW-0001 through
  FLAW-0010 only (FLAW-0011's definition; Al's ruling 3 on PR 30) and of severity gap or fatal
  (Al's ruling 2 on PR 30: a composite of cosmetic or repairable instances is not an invalid
  proof and is refused), on a seed whose statement is true; the spec's severity must be the
  embedded instance's (it inherits). The embedded instance is injected as instance 1 and the
  composite is instance 2, with a record in each file the artifact has: step 0 with span
  `[0, len)` in the informal file, the main declaration at its header line in the formal file,
  `lean_catchable` inherited, dials `embedded`, `embedded_instances`, `site: 0`, `inserted` and
  `replaced: null`. Every composite is therefore an invalid proof of a true statement under D7.
  FLAWS.md's FLAW-0011 example embeds a cosmetic instance (the routine-true hedge) and so
  realizes FLAW-0004 `routine-true`, not a composite — an erratum note against FLAWS.md 0.1.0,
  recorded in ADR-0011 *Amendments* by Al's ruling 2 on PR 30; FLAWS.md's text is unchanged.
- **FLAW-0010 `a`** (0.6.0): replacement material whose `cited` names the bare claim and the
  justification the reference stands in for; the config's dials are `shape` (`journal`, `book`
  or `preprint`: which venue list the draw uses) and `claim` (`true`, gap: the cited step is
  true; `false`, fatal: it needs the seed's false-step material, which no seed carries yet).
  The injector draws the author, the year, the venue and the result from the fixed lexicon of
  obviously fictional tokens (`Placeholder`, `Nonesuch`, `Fictitious`, `Hypothetical`; `19XX`,
  `20XX`; three venues per shape; `Theorem 4.2`, `Lemma 3.1`, `Proposition 2.5`,
  `Corollary 1.7`) on the stream `injector/FLAW-0010/1` after the material, in that order, and
  the site step becomes `By [<author> <year>, <venue>, <result>]` followed by the bare claim.
  The record spans the reference: `inserted` is the reference, `replaced` the justification;
  the dials carry the drawn indices (`author`, `year`, `venue`, `result`), `reference`,
  `asserted` (the bare claim), `justification` and the originals. The formal file is unchanged,
  so the instance is `N/A` and informal-only (a pair's formal file is the faithful one; the
  target `formal` is refused). Sub-variants `b` and `c` need real references and are refused
  (ADR-0011 Options 5).
- **FLAW-0003** (0.6.0): insertion material whose site is a `theorem` lemma, fatal. `direct`
  (dial `form`: `restated-goal` or `contrapositive`) inserts Lemma 1 — the goal, or its
  contrapositive — whose formal proof references the main declaration, declared later in the
  file, so the kernel reports an unknown identifier at the lemma's declaration (check K,
  `YES-KERNEL`); the key step cites Lemma 1. `cycle` (dial `cycle_length`, 2 in this version)
  inserts Lemma 1, whose proof cites the theorem, and the key step cites Lemma 1; it is
  informal-only (`NO`; the target `formal` is refused), and the formal file keeps a faithful
  proof — the lemma proved from Mathlib, the main declaration using it — which builds. The
  material's steps carry `premises` (`cites`: a lemma's declaration or `theorem`; `phrase`: the
  citing text, exactly once in its step); the record's dials carry `cycle` (the declarations
  visited from the lemma back to itself) and `citations` (each phrase's step and span), and the
  originals.
- **Counts** (0.2.0): `n_flaws_total`, `n_flaws_<NNNN>` and `flaws_confirmed` count instances. A
  pair's two records of one instance share its `instance`; the instance is confirmed only when
  both records pass their checks (Al's owner note of 2026-10-28).
- **Checks:**
  - informal records: the recorded span holds exactly the recorded text and, where something was
    replaced, differs from it; a FLAW-0004 record (0.5.0) also requires the step to be the hedge
    entry, a space and the bare claim, the faithful text to have held the justification exactly
    once, and the step not to hold it;
  - formal `YES-KERNEL` records: check K, meaning the build fails and its first error lies in the
    site's fragment lines;
  - formal `YES-TOOLING` records (0.3.0): check T, meaning the build passes and the audit tail's
    listing satisfies the class's condition — FLAW-0007 `unused` and, from 0.4.0, FLAW-0013: the
    inserted declaration is among the file's own constants and outside the transitive
    used-constants of the main declaration, closed over the file's own constants (Lean's
    auxiliary `_proof_N` constants included); FLAW-0009: `#print axioms` of the main declaration
    lists the axiom.

  - a FLAW-0011 record (0.5.0): the statement's truth is by a kernel-checked witness or Mathlib
    (D7; the witness is a library module the gate builds) and every record of every embedded
    instance passes its check;
  - a FLAW-0010 record (0.6.0): the span equals the reference the recorded lexicon indices
    name, every token of it is in the lexicon, the step is `By <reference>` followed by the
    bare claim, and the faithful step held the justification exactly once;
  - a FLAW-0003 `cycle` record (0.6.0): the plan's premise graph contains the recorded cycle
    through the inserted lemma, and each citation phrase is at its recorded span in the
    rendered step. (`direct` is confirmed by check K.)

  No other check exists yet, so no other class is confirmed by this version.
- **`padding_ratio`** (0.4.0; ADR-0001 D4, ADR-0011 D8): achieved as the tokens inside the
  FLAW-0013 spans over `n_tokens` — the padded steps of the informal file when there is one,
  else the padding declarations' lines of the formal file — in binary64; commanded as the
  config's value with tolerance `abs 0.05` (`abs 0.0` at ratio 0). A commanded ratio the count
  of paragraphs cannot reach is a FAIL verdict, not a refusal.
- **Not yet:** `s2_grammar`, padding combined with an injected flaw (also inside a composite),
  composites of more than one instance, the kinds `unused-lemma` and `verbose-rewrite` (no seed
  carries their material), FLAW-0004 `nontrivial-true` and `false` and FLAW-0010 `claim: false`
  (no seed carries the false-step material), FLAW-0010 `b` and `c` (deferred), FLAW-0003
  `cycle` of length other than 2, FLAW-0009 `native_decide`, FLAW-0012 as a commanded class,
  the seeds for the remaining FLAWS.md examples (FLAW-0008, FLAW-0012), and the fidelity sweep.
  They come in later sessions (D11).

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
