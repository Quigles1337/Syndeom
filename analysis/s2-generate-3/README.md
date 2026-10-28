# S2 implementation packet 3 — `s2_template` 0.3.0 (ADR-0011, Session 42, 2026-10-28)

This packet records the third implementation session of ADR-0011 (D11): `s2_template` 0.3.0 —
lemma steps in the plan (D2), the T check and the used-constants audit tail (D5, D8) for the
tooling-caught classes FLAW-0009 (`axiom`) and FLAW-0007 (`unused`), and the seed
`n_lt_two_pow` with its witness module and goldens. Nothing here is a result: no CLAIM, no
fidelity sweep, no artifact under `corpora/`.

Branch `claude/s2-template-0.3.0`, from `main` `7a05e75ed5ed872f92eb4453d353616c2f1d1dbf` (PR
27's merge):

| Commit | What |
|---|---|
| `59b505a320447f58c811ec394103f64ac9381a93` | the 0.3.0 producer: lemma steps, the audit tail and check T, records only where a class manifests, the seed and its witness module, the tests, the crate README |
| `5a313ddf8de6a870c2f41c9d65d8cb87a1595b2e` | FINDING-011: `verify` refuses a manifest whose `main_declaration` is not the producer's |
| `f0173a285a2d3797368827c1c82661019b3ec9f6` | the 9 goldens of 0.3.0, produced at `5a313ddf…`, and the goldens README |
| the record commit after it | this packet, STATE and HANDOFF Session 42 |

## What was built

- **Lemma steps (D2).** A `Step` may carry a `lemma` header (`kind` ∈ {`theorem`, `axiom`},
  `name`, `claim`). In the informal file it is a step whose text begins `Lemma N.`; in the formal
  file the writer emits every lemma step as its own declaration before the main declaration, in
  step order, each followed by a blank line (`theorem <name> : <claim> := by` with the step's
  fragment indented, or `axiom <name> : <claim>` with no proof). `check_lemmas` enforces the
  numbering (`Lemma 1.`, `Lemma 2.`, …, in document order), that only a headed step begins that
  way, and that an axiom has no proof lines while a theorem has some. `FormalRendering` now
  records, per step, the declaration its lines belong to; for a lemma step the line range is the
  whole declaration. No 0.1.0 or 0.2.0 byte depends on any of this (no seed of theirs has a lemma
  step), and the Golden A/B writer test is unchanged.
- **The audit tail (D8).** After `#print axioms <main>`, a fixed `#eval` over `MetaM` prints, for
  every constant the file itself defined (the environment's second-stage constant map,
  `env.constants.map₂`), its direct used-constants (`Lean.ConstantInfo.getUsedConstantsAsSet`),
  one `used-constants <name>: [...]` line each. `Observation` gains `used_constants`
  (per constant, sorted and deduplicated; absent from earlier versions' observations, so every
  existing `case.json` still parses), and `parse_used_constants` reads the listing.
- **Check T (D5).** For a formal `YES-TOOLING` record, confirmed iff the build passes and:
  FLAW-0007 `unused` — the inserted lemma's name is among the listed constants and outside the
  transitive used-constants of the main declaration closed over the listed constants
  (`used_closure`; Lean's auxiliary `<name>._proof_N` constants are listed and followed, imported
  constants are not); FLAW-0009 — `#print axioms` of the main declaration lists the axiom's name.
  Any other `YES-TOOLING` class stays unconfirmed, as before.
- **Records only where a class manifests.** `manifests_in` realizes FLAWS.md's *Manifests in*
  column: FLAW-0009 and FLAW-0008 formal-only, FLAW-0010 and FLAW-0003 `cycle` informal-only,
  the rest both. A FLAW-0009 pair carries one record (its informal file is the faithful one).
- **Material shapes.** Replacement material (0.1.0) and intrinsic material (0.2.0) are joined by
  **insertion material** (`inserts: true`): one step stands in for no faithful step (`from:
  null`), `site` is its index in the flawed argument, the informal record spans the whole step
  with `replaced: null`, and the originals are recorded as for replacement material. At a lemma
  step the formal record's `inserted` and `replaced` are the declaration lines as the writer
  emits them, its location is the lemma's declaration at its first line, and the dials carry
  `declaration` (now a reserved key).
- **The seed (D6)** `n_lt_two_pow`, from FLAWS.md 0.1.0's FLAW-0009 example (the helper
  `n < 2 ^ n` as an axiom) and FLAW-0007 example (the unused lemma `n² ≥ n`), outside the S1
  corpus; one style `helper_lemma` — Lemma 1 (`∀ n : ℕ, n < 2 ^ n`, by induction) and the
  conclusion `n + 1 ≤ 2 ^ n`; imports `Mathlib.Data.Nat.Notation` only. Materials: FLAW-0009
  `axiom` with `truth: true` (gap; the axiom states the helper) and `truth: false` (fatal; the
  axiom states the false lemma `∀ n : ℕ, n + 1 < 2 ^ n` and the conclusion adapts); FLAW-0007
  `unused` (cosmetic; Lemma 2 inserted, proved, cited nowhere). The witness module
  `lean/Syndeom/Seeds/NLtTwoPow.lean` proves the statement and the negation of the false lemma;
  `Audit.lean` audits both (ten lines now).
- **Versions (D1).** `Version::V0_3_0` is current; `generate` emits 0.3.0; the 0.1.0 and 0.2.0
  goldens regenerate under the versions they name, byte for byte, with no golden rewritten.

## Evidence that the Lean material does what the records say

1. **Probes before records** (`evidence/lean-probe/`, `lake env lean` from `lean/`, one process
   at a time, `run.sh`; 2026-10-28T20:08:06Z–20:08:23Z, then the two tail variants
   20:17:02Z–20:17:09Z):

   | Probe | Exit | What |
   |---|---|---|
   | `nltp_faithful` | 0 | Lemma 1 by induction and the conclusion; `[propext, Quot.sound]` |
   | `nltp_axiom_true` | 0 | the axiom of the helper; `#print axioms` lists `syndeom_s2_template_n_lt_two_pow_lemma_1`; the used-constants lines print in the form the parser reads |
   | `nltp_axiom_false` | 0 | the axiom of the false lemma with `Nat.le_of_lt`; `#print axioms` lists the axiom |
   | `nltp_unused` | 0 | Lemma 2 (`Nat.pow_two`, `Nat.le_mul_self`) inserted; the main declaration's used-constants name Lemma 1 only; Lemma 1's name an auxiliary `…lemma_1._proof_1_1` |
   | `nltp_witness` | 0 | the statement witness `[propext, Quot.sound]`; the negation witness depends on no axioms |
   | `nltp_tail_map2`, `nltp_tail_fold` | 0 | the tail over `env.constants.map₂` (chosen) and over `foldStage2` list the same four constants, auxiliary included |

2. **The Lean library.** `lake build` built 8,934 jobs, up from 8,933
   (`evidence/lake-build-witnesses.log`, 20:22:45Z); the audit prints ten lines, every one within
   the standard three.
3. **The goldens.** 9 cases, each produced by `generate --dry-run` with seed 7 at
   `5a313ddf8de6a870c2f41c9d65d8cb87a1595b2e` by two separate runs (`evidence/goldens-run3.py`;
   run A 2026-10-28T20:32:24Z–20:33:31Z, run B 20:33:31Z–20:34:40Z; every case exit 0, fidelity
   PASS, the provenance check passed against the commit before each run;
   `evidence/goldens-run3a-summary.txt`, `evidence/goldens-run3b-summary.txt`).
   `evidence/goldens-copy3.py` compared the runs file by file — every `proof.md`, `Proof.lean`
   and `manifest.json` byte-identical, every observation equal, one engine commit — compared
   each formal output with its probe, and placed run A's outputs under
   `engine/generate/tests/goldens/n_lt_two_pow/` with a `case.json` each
   (`evidence/goldens-production-report.json`, SHA-256
   `e44f02d470278977bd6f29b5ab1616622eeb8a0e79c5bbddcdf8432a9d285c82`, 21,845 bytes):
   - all seven formal and pair goldens are byte-identical to their probes without the probe's own
     tail (`\n#print axioms …` and, for `nltp_axiom_true` and `nltp_unused`, the used-constants
     `#eval`);
   - every formal and pair case builds (exit 0); the control and the FLAW-0007 cases print
     `[propext, Quot.sound]`, the four FLAW-0009 cases print
     `[syndeom_s2_template_n_lt_two_pow_lemma_1]`; the FLAW-0007 cases' listing names the two
     lemmas, `…lemma_1._proof_1_1` and the main declaration, which uses Lemma 1 only.
   - The receipts, with build and audit stdout and stderr, are under the external root
     `C:\Users\LEET\Codex\Temp\syndeom-s2-template-030-20261028T2005Z\goldens-3a\` and
     `goldens-3b\`; the sorted lists of the nine `receipt.json` SHA-256s hash to
     `df4aee9da156f80fc7b303a2e71e4072fd83a8dcdc335a6ca0d968f08033c473` and
     `27cd18a58ae653952906c61268de629de872a52df6aba7a5e781d0d8b6c9b8c4`.
   - Two earlier productions are kept as the record: at `59b505a3…` (`goldens-a\`, `goldens-b\`,
     byte-identical, the same proof bytes and artifact ids, manifests differing only in the
     engine commit), superseded so that the goldens name the commit holding FINDING-011's fix;
     and one refused run (`goldens-2a\`; `evidence/goldens-run-refused-summary.txt`): with the
     goldens test's count pinned at 9 in the working tree while the commit carried 0, every case
     was refused with exit 2 — `engine_commit does not identify the checked-out producer input
     engine/generate/tests/goldens.rs` — which is the provenance check (D9 as amended) doing its
     job.
4. **The artifact ids** (`s2_template-v0.3.0-<config_hash[0:12]>-s7`): control `301add18edb4`
   / `a66bf629fb89`; FLAW-0009 gap `38df27ba3c69` / `51f145606f69`; FLAW-0009 fatal
   `0e14a45c6007` / `20216e43f213`; FLAW-0007 unused `057c6b09e6ee` / `a8e03589f863` /
   `e3cf5c1a91ae` (informal / formal / pair, where present).
5. **Tests and lint.** The crate's 27 tests pass (goldens 1 over 35 cases, properties 16,
   provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features -- -D warnings`
   and `cargo fmt --check` are clean. New: `d2_lemma_steps_render_as_declarations_before_the_main_declaration`
   (the layout, line ranges, declarations and shape refusals),
   `d5_check_t_confirms_the_tooling_caught_classes_only_as_predicted` (T confirms FLAW-0007 and
   FLAW-0009 as predicted and refuses confirmation when the lemma is used directly or through an
   auxiliary, when no listing exists, or when the axiom print lacks the axiom), the
   used-constants parser and closure cases in `d8_harness_parsers`, and
   `d9_verify_refuses_a_manifest_whose_main_declaration_is_not_the_producers`. The gate on the
   committed tree is recorded in the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-10-28T20:23:03Z, outside the gate. It loaded 1,381
advisories (advisory-db commit `40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`), scanned 35 crate
dependencies, and reported no vulnerability (exit 0; `evidence/cargo-audit.txt`).
`engine/Cargo.lock` is unchanged by this revision.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** `engine/generate/src/plan.rs`, `src/harness.rs`, `src/template.rs` at `59b505a3…`
and `src/main.rs` at `5a313ddf…`. The threat model is packet 1's: an offline research CLI whose
assets are the integrity of generated ground truth and the operator's machine, and whose trust
boundary is an artifact directory handed to `verify`. New this revision: the audit tail is Lean
source that `lake env lean` elaborates, and `#eval` runs code.

**Pre-scan:** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate roots; no
external dependency was added; clippy at `-D warnings` is clean.

**Panic and index sweep:** `style.steps[m.site - 1]`, `m.steps[m.site - 1]` (insertion
material) and `faithful[material.site - 1]` are guarded by `check_seed`, which `prepare` runs
before `inject`; `f.fragment_lines[inj.site - 1]` and `f.declarations[inj.site - 1]` index
vectors sized to the plan's steps; `used_closure` terminates by its `seen` set. Seeds are
compiled in, so no seed-record value is attacker-controlled; a config's `dials` may not name the
injector's keys, now including `declaration`.

**Serde:** `Observation.used_constants` defaults to `None` and `None` confirms nothing
(fail-closed); `deny_unknown_fields` stays on `Observation`, `Step`, `LemmaHeader`, `Material`
and `FlawedStep`.

**Subprocesses:** unchanged — `lake`, `git` and `rustc` are spawned without a shell, with
validated arguments.

```
FINDING-011: verify compiles an audit file naming the manifest's main_declaration unchecked
Severity:    High
Category:    Logic (code execution through Lean elaboration)
Location:    engine/generate/src/main.rs (verify), engine/generate/src/harness.rs (audit_bytes)
Status:      Resolved (5a313ddf) — verify refuses, before any Lean run and with no receipt,
             a manifest whose main_declaration is not the declaration the producer emits for
             the manifest's config and seed; the CLI test covers a foreign and an injected name.
Description: `verify` checked that Proof.lean is the bytes this producer emits, then wrote an
             audit file of those bytes plus `#print axioms <main_declaration>` with the name
             taken from the manifest, and compiled it. A name holding a newline injected Lean
             commands, and `#eval` can run IO. Present since 0.1.0; the audit tail's own `#eval`
             made it worth closing now.

FINDING-012: the used-constants parser splits a listing on ", "
Severity:    Informational
Category:    Logic
Location:    engine/generate/src/harness.rs (parse_used_constants)
Status:      Accepted
Description: a constant whose escaped name contains ", " would be split into two entries of
             a uses list. The keys (the file's own constants, named by the producer) are read
             before the first ": [" and are never affected, so the closure can gain a spurious
             non-key name but never a key; T stays fail-closed.

FINDING-013: the audit tail is Lean code with `#eval`
Severity:    Informational
Category:    Logic
Location:    engine/generate/src/harness.rs (AUDIT_TAIL)
Status:      Accepted
Description: the tail is a fixed string with no interpolation; it runs only on bytes the
             producer itself emitted from compiled-in seeds (generate) or verified to equal them
             (verify, now with FINDING-011 closed). The compiled file lives in the receipt
             directory, never under corpora/.
```

FINDING-006 (accepted risk), FINDING-007 (backlog), FINDING-008 (the FLAW-0013 session),
FINDING-009 and FINDING-010 (accepted) stand as packets 1 and 2 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 2's numbering.

21. **Lemma steps are steps.** D2's `lemmas : [Lemma]` is realized as a step carrying a lemma
    header, in document order, so the site index stays uniform and alignment is explicit; the
    formal writer renders lemma steps first. A lemma's claim is a closed statement (no binders),
    and its name is `<main declaration>_lemma_<N>`.
22. **"Every declaration in the file" is every constant the file defined.** The audit tail
    enumerates the environment's second-stage constant map rather than a name list passed by
    the Rust side, so Lean's auxiliary constants (`…lemma_1._proof_1_1`, from `omega`) are
    listed and the closure follows them. Imported constants are never followed: they cannot
    reference the file.
23. **Records only where a class manifests**, per FLAWS.md's *Manifests in* column
    (`manifests_in`). A FLAW-0009 pair therefore has one record and `n_flaws_total: 1`; the
    owner note's rule that both of a pair's records must pass is vacuous for it. FLAW-0004 is
    left as "both" (D5 gives it a formal action for pairs) until a seed realizes it.
24. **FLAW-0009 `truth: false` changes the formal lemma's statement.** The axiom states the
    false lemma `∀ n : ℕ, n + 1 < 2 ^ n`, the conclusion's fragment becomes `Nat.le_of_lt (…)`,
    and the informal file is unchanged — its Lemma 1 still states the true helper, because the
    class manifests in the formal file only. The witness module proves the negation of the false
    lemma, so the label is by construction. Severity `fatal` (FLAWS.md: "axiom false").
25. **Insertion material** (`inserts: true`; FLAW-0007 `unused`): the inserted step's `from` is
    null, `site` indexes the flawed argument, the informal record spans the whole step with
    `replaced: null` and no counterexample, and the originals are recorded. The material's
    dials (`count: 1`, `length_steps: 1`) are what a spec must name; any other value is refused.
26. **Lemma-site formal records** carry the declaration lines as emitted (header unindented,
    proof lines indented) in `inserted`/`replaced`, the lemma's declaration at its first line as
    location, and `declaration` in the dials. Main-proof sites keep 0.1.0's fragment form.
27. **One seed, two classes.** `n_lt_two_pow` realizes both FLAW-0009's and FLAW-0007's examples;
    `n + 1 ≤ 2 ^ n` is the executor's choice of a theorem whose proof uses the helper. Its import
    is the single module `Mathlib.Data.Nat.Notation`; the proofs use core tactics
    (`Nat.pow_succ`, `omega`, `decide`, `Nat.pow_two`, `Nat.le_mul_self`).
28. **FINDING-011's fix changes no emitted byte**, so the producer version stays 0.3.0; the
    goldens were re-produced at the commit holding the fix so that they name the complete
    producer. In the property tests the predicted control observation now lists
    `[propext, Quot.sound]` (what the harness observes) — tests only.
