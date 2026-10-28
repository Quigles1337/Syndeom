# S2 implementation packet 4 — `s2_template` 0.4.0 (ADR-0011, Session 43, 2026-10-28)

This packet records the fourth implementation session of ADR-0011 (D11): `s2_template` 0.4.0 —
the padding module (D2 `pad`, D5 FLAW-0013, Options 8), the first seed that carries padding
material, `sqrt2_irrational`, with its witness module and goldens, check T for the padding
declarations, and `padding_ratio` measured. Nothing here is a result: no CLAIM, no fidelity
sweep, no artifact under `corpora/`.

Branch `claude/s2-template-0.4.0`, from `main` `4cd1f7724f5d8ee1097f0afbc6d1ceafb0220f62` (PR
28's merge):

| Commit | What |
|---|---|
| `b54e2f16aa3c153258bf6ece67798bd270993cbb` | the 0.4.0 producer: padding steps in the plan, the padding module, the padded records and `padding_ratio`, the seed and its witness module, the tests, the crate README |
| `cc868a3cef8648059df71ed888493765214fe04a` | the 8 goldens of 0.4.0, produced at `b54e2f16…`, and the goldens README |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 43 |

## What was built

- **Padding steps (D2).** A `Step` may carry a `padding` header (`kind`, `name`, `claim`): a
  whole paragraph of true irrelevant material. In the informal file it is a step like any other,
  never numbered `Lemma N.` (so `n_lemmas` is unchanged); in the formal file the writer emits it
  as a `theorem` declaration before the main declaration, in step order, exactly as it emits a
  `theorem` lemma. `check_lemmas` refuses a step that is both a lemma and padding, a padding
  step whose text begins `Lemma N.`, and a padding declaration without proof lines.
  `FormalRendering.declarations` names the padding declaration for its step. No byte of any
  earlier version depends on this: no plan before 0.4.0 has a padding step.
- **The padding module (D2 `pad`, D5 FLAW-0013).** Padding is commanded by one FLAW-0013 spec
  together with a positive `padding_ratio`, and by nothing else: a positive ratio without the
  spec, or the spec with ratio 0, is refused (exit 2), as is the spec under any earlier version.
  The spec's dials are exactly `kinds` (a non-empty, duplicate-free subset of the four D5 kinds;
  the seed must carry material of the kinds named), `count` (a positive integer: how many
  paragraphs) and `placement` (`before`: every paragraph before the first step of the argument,
  in drawn order; `drawn`: each paragraph at a position drawn from the `padding` stream, before a
  step of the plan as it then stands). The paragraphs are drawn without replacement from the
  `padding` stream (`Stream::new(seed, "padding")`): for each instance, a paragraph among those
  remaining and, under `drawn`, a position. `site` must be `"auto"`, `sub_variant` null, the
  severity `cosmetic`. One record per paragraph in each file the artifact has, each paragraph
  its own instance (a pair's two records share it), so `n_flaws_0013` is the count. The records'
  dials carry the spec's three dials, the config's ratio (`ratio`), the paragraph drawn
  (`paragraph`, its 1-based index in the seed's material, and its `kind`), the resolved `site`,
  `inserted` (the text, or the declaration lines as emitted) and `replaced: null`; `ratio`,
  `kind` and `paragraph` join the reserved dial names. No originals are recorded: nothing is
  replaced, and the unpadded argument is recoverable by deleting the recorded steps (Options 8).
  Padding applies to the faithful argument of a true statement only: a false-statement seed has
  nothing to pad, and a config with a FLAW-0013 spec and another spec is refused (one spec per
  artifact, as in 0.3.0).
- **Records and checks (D5, D8).** The informal record spans the whole step, `[0, len)` in code
  points, `lean_catchable: NO`, confirmed when the step holds exactly the paragraph's text. The
  formal record is the matching declaration at its first line, `YES-TOOLING`, confirmed by check
  T — the FLAW-0007 `unused` condition, now shared: the build passes, the audit tail's listing
  names the declaration, and the declaration lies outside the transitive used-constants of the
  main declaration closed over the file's own constants.
- **`padding_ratio` measured (ADR-0001 D4, ADR-0011 D8).** Achieved as the tokens inside the
  FLAW-0013 spans over `n_tokens`, in binary64: the padded steps' tokens over the informal
  `n_tokens` when there is an informal file, else the padding declarations' lines' tokens over
  the formal `n_tokens`; 0.0 when nothing was padded (so every earlier manifest's `0.0` is
  unchanged). Commanded as the config's value with tolerance `abs 0.05` (`abs 0.0` at ratio 0,
  as before). A commanded ratio the count cannot reach is a FAIL verdict, not a refusal. The
  ratio never enters a generation decision — the integer `count` does — so FINDING-008 is
  closed (below).
- **The seed (D6)** `sqrt2_irrational`, from FLAWS.md 0.1.0's FLAW-0013 example (a correct
  five-step proof that √2 is irrational, preceded by restated definitions of rational number,
  gcd and parity and three separate derivations of 2 | p² ⇒ 2 | p) and FLAW-0011 example (the
  same statement), outside the S1 corpus. Its statement is the number-theoretic core: for all
  natural numbers p, q with gcd(p, q) = 1, p² ≠ 2q² (Lean: `(p q : ℕ) (h : Nat.Coprime p q) :
  p ^ 2 ≠ 2 * q ^ 2`), imports `Mathlib.Data.Nat.Notation` and `Mathlib.Tactic.Ring`; one style
  `descent` of five aligned steps (suppose p² = 2q²; p² even; p even, p = 2k; q² = 2k² so q even,
  q = 2m; 2 | gcd(p, q) = 1). Seven padding paragraphs, each with its unused declaration: four
  `restated-definition` (divisibility, rational numbers and gcd, coprimality, parity) and three
  `re-derivation` of `2 ∣ p ^ 2 → 2 ∣ p` (residues of squares; the odd form `2m + 1` expanded by
  `ring`; the product form with `Nat.mul_mod`). The witness module
  `lean/Syndeom/Seeds/Sqrt2Irrational.lean` proves the statement; `Audit.lean` audits it (eleven
  lines now).
- **Versions (D1).** `Version::V0_4_0` is current; `generate` emits 0.4.0; the 0.1.0, 0.2.0 and
  0.3.0 goldens regenerate under the versions they name, byte for byte, with no golden rewritten.

## Evidence that the Lean material does what the records say

1. **Probes before records** (`evidence/lean-probe/`, `lake env lean` from `lean/`, one process
   at a time, `run.sh`; 2026-10-28T21:36:53Z–21:37:16Z, about 8 s each with the two imports):

   | Probe | Exit | What |
   |---|---|---|
   | `s2i_faithful` | 0 | the five-step argument as the control golden should emit it; `[propext, Quot.sound]` |
   | `s2i_witness` | 0 | the witness module's declaration; `[propext, Quot.sound]` |
   | `s2i_padded` | 0 | all seven padding declarations before the main declaration, with the audit tail: `[propext, Quot.sound]`; the listing names sixteen own constants; the main declaration uses only its own four `_proof_1_N` auxiliaries; each `_pad_` name is used only by its own auxiliaries |

2. **The Lean library.** `lake build` built 8,935 jobs, up from 8,934
   (`evidence/lake-build-witnesses.log`, 21:43:44Z–21:44:18Z); the audit prints eleven lines,
   every one within the standard three; the new witness prints `[propext, Quot.sound]`.
3. **The ratio probe** (`evidence/ratio-probe-summary.txt`; 21:49:58Z–21:50:39Z): the two padded
   configs produced once with ratio 1 for the informal and formal targets, to read what their
   counts achieve at seed 7 — 0.2784 (176 tokens) and 0.7127 (442 tokens) over the informal
   file, 0.3130 (262 tokens) and 0.6146 (467 tokens) over the formal file. The goldens command
   these to two decimals (`evidence/ratios.json`: 0.28 and 0.71 for informal and pair, 0.31 and
   0.61 for formal-only); these four FAIL-verdict probe artifacts are kept under the external
   root as the record and are not goldens.
4. **The goldens.** 8 cases — the control (informal, formal), FLAW-0013 `drawn` with one
   re-derivation (informal, formal, pair) and FLAW-0013 `before` with all seven paragraphs
   (informal, formal, pair) — each produced by `generate --dry-run` with seed 7 at
   `b54e2f16aa3c153258bf6ece67798bd270993cbb` by two separate runs (`evidence/goldens-run4.py`;
   run A 2026-10-28T21:51:09Z–21:52:47Z, run B 21:52:47Z–21:54:28Z; every case exit 0, fidelity
   PASS, the provenance check passed against the commit before each run;
   `evidence/goldens-run4a-summary.txt`, `evidence/goldens-run4b-summary.txt`).
   `evidence/goldens-copy4.py` compared the runs file by file — every `proof.md`, `Proof.lean`
   and `manifest.json` byte-identical, every observation equal, one engine commit — compared
   each formal output with the probes, and placed run A's outputs under
   `engine/generate/tests/goldens/sqrt2_irrational/` with a `case.json` each
   (`evidence/goldens-production-report.json`, SHA-256
   `207f5a8ecb4d752bcf38bba1e48acccc34b1faf8f3fa54aac8d9a9d9a6bfa971`, 108,750 bytes):
   - the control golden is `s2i_faithful.lean` without the probe's `\n#print axioms …` tail; every
     padded formal and pair golden's declarations are blocks of `s2i_padded.lean`, in drawn
     order, followed by the probe's main declaration;
   - every formal and pair case builds (exit 0) and prints `[propext, Quot.sound]`; the padded
     cases' listings name their padding declarations, none reachable from the main declaration
     (check T confirms every instance: `flaws_confirmed` 1 and 7);
   - the padded informal and pair files have 6 and 12 steps, `n_lemmas` 0; the `before` file is
     the FLAWS.md example's shape — the seven paragraphs, then the five-step argument; the
     `drawn` file carries the second re-derivation (paragraph 6) before step 2 of the argument.
   - The receipts, with build and audit stdout and stderr, are under the external root
     `C:\Users\LEET\Codex\Temp\syndeom-s2-template-040-20261028T2136Z\goldens-4a\` and
     `goldens-4b\`; the sorted lists of the eight `receipt.json` SHA-256s hash to
     `4ab0cf60ef179998b100a05e8861102d703c16991774ce38c11298afde5e8f25` and
     `a8ecdfd7826f2bec0abc06310723985b5a870eca6b71bf040d8fc10282996aaa`.
5. **The artifact ids** (`s2_template-v0.4.0-<config_hash[0:12]>-s7`): control `e8996ca4c91c`
   / `2bbbb0bed9a7`; FLAW-0013 `drawn` 1 `a507dd6b84e2` / `c961b4f8b1b9` / `58c2d082ed3a`;
   FLAW-0013 `before` 7 `27f7b0a8feb9` / `4149f1b73e2c` / `69694d73791d` (informal / formal /
   pair, where present).
6. **Tests and lint.** The crate's 29 tests pass (goldens 1 over 43 cases, properties 18,
   provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features -- -D warnings`
   and `cargo fmt --check` are clean. New:
   `d5_padding_inserts_true_irrelevant_paragraphs_with_their_unused_declarations` (the FLAWS.md
   example's shape, the records and their spans, check T's refusal when the main declaration
   uses a paragraph, a ratio the count cannot reach as a FAIL verdict, the `drawn` placement
   keeping the argument's order, determinism, informal-only and formal-only, the seed's control)
   and `d3_padding_is_commanded_by_a_flaw_0013_spec_with_a_positive_ratio_and_refused_otherwise`
   (twenty-one refusals); `every_case` gains the two padded configs per target, with the
   commanded ratio the achieved one; `predicted` excludes the padding declarations from the
   main declaration's uses. The gate on the committed tree is recorded in the pull request and
   under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-10-28T21:54:47Z, outside the gate. It loaded 1,381
advisories (advisory-db commit `40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`), scanned 35 crate
dependencies, and reported no vulnerability (exit 0; `evidence/cargo-audit.txt`).
`engine/Cargo.lock` is unchanged by this revision.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** `engine/generate/src/plan.rs` (padding steps), `src/template.rs` (`pad`, the padded
records and `padding_ratio` in `assemble`, the seed shape rules) and
`seeds/sqrt2_irrational.json` at `b54e2f16…`. The threat model is packet 1's: an offline research
CLI whose assets are the integrity of generated ground truth and the operator's machine, and
whose trust boundary is an artifact directory handed to `verify`. New this revision: a config may
name a count and kinds of paragraphs; the padding declarations are Lean source compiled by the
harness.

**Pre-scan:** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate roots and in
the manifest crate; no external dependency was added; clippy at `-D warnings` is clean.

**Panic and index sweep of the diff:** no `unwrap`, `expect` or `panic!` was added. The indexes:
`remaining.remove(pick)` and the drawn position are bounded by `Stream::below` over the live
lengths, and `count` is refused unless it is at most the number of eligible paragraphs, so a
config cannot make the loop long or the vector short; `f.fragment_lines[site - 1]`,
`f.declarations[site - 1]` and `st.steps[site]` index vectors sized by the padded plan and the
rendered file; `hi - lo + 1` has `hi ≥ lo` by construction. `padding_ratio` divides only when
the padded tokens are non-zero, and `n_tokens` is never zero for a rendered file. The ratio is
finite and non-negative by `parse_config`. Seeds are compiled in, so no seed-record value is
attacker-controlled.

**Serde:** `Paragraph` and `PaddingHeader` carry `deny_unknown_fields`; `Step.padding` defaults
to `None`, so every earlier seed and every golden `case.json` still parses; a `FlawedStep` has no
`padding` field and refuses one.

**Subprocesses and `verify`:** unchanged. `verify` still compiles only bytes the producer itself
emits for the manifest's (config, seed), names only the producer's main declaration in the audit
tail (FINDING-011), and takes the padding declarations' names from the compiled-in seed, never
from the manifest.

```
FINDING-008: padding_ratio is read as binary64 before rendering
Severity:    Informational
Category:    Logic (D4)
Location:    engine/generate/src/template.rs (parse_config, pad, assemble)
Status:      Resolved (0.4.0, b54e2f16) — the ratio never enters a generation decision: the
             paragraphs and positions are drawn from the `padding` stream and their number is
             the integer `count` dial; the config's ratio is copied into the records' `ratio`
             dial and compared only after rendering, in `measure`, in binary64 (ADR-0001 D6).
             It is not held as an integer pair: it is a measured statistic's command, not a
             dial that drives generation. Flagged as reading 30.

FINDING-014: a seed's own step may carry a padding header
Severity:    Informational
Category:    Serde / Logic
Location:    engine/generate/src/template.rs (check_seed), src/plan.rs (Step)
Status:      Open (backlog)
Description: `Step.padding` is deserialized with a default, so a seed record could mark one of
             its style's steps as padding, and `check_seed` does not refuse it; the renderers
             would then emit that step as an unused declaration and the control would carry
             no record of it. Seeds are compiled in and executor-authored (D6), so nothing
             attacker-controlled reaches this. Recommendation: a `check_seed` rule that no
             style step and no material step carries `padding`, in the next producer revision.
```

FINDING-006 (accepted risk), FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012 and
FINDING-013 (accepted) stand as packets 1–3 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 3's numbering. **All ten were accepted as read by Al's rulings of 2026-10-28 on PR 29**
(below); reading 30 carries the reading Al recorded with it.

29. **The seed's statement is the number-theoretic core.** FLAWS.md's example says "a proof that
    √2 is irrational"; the seed states, informally and formally, that no coprime natural numbers
    p, q have p² = 2q² (the informal statement adds "the square root of two is irrational" after
    a colon, and the title names √2). This keeps the imports small and the five steps aligned,
    and the witness is a lean-kernel declaration of this repository rather than Mathlib's
    `irrational_sqrt_two`, which states the real-number form (D7's `mathlib` method applies
    "when one states it"; the ℕ form is not a Mathlib declaration).
30. **How padding is commanded.** D3's config has `padding_ratio`; D5 lists FLAW-0013's dials as
    `ratio` and `kinds`; D4 forbids a floating-point value in a generation decision. The reading:
    padding is commanded by a FLAW-0013 spec whose dials are `kinds`, `count` and `placement`,
    together with a positive `padding_ratio`; `count` (an integer) is the generation decision;
    `ratio` is the config's value copied into the record's dials; the commanded ratio is a
    measured statistic checked at `abs 0.05` (D8), and a ratio the count cannot reach is a FAIL
    verdict. A config therefore commands what its count yields, as the goldens do.
    **Accepted, with Al's reading recorded (rulings on PR 29):** the integer `count` is the
    generation decision because D4 forbids a floating-point value there; `padding_ratio` is a
    commanded statistic measured from the emitted file and checked at `abs 0.05`, and a ratio
    the count cannot reach is a FAIL verdict. A ratio-targeted mode, if a later version wants
    one, must choose its count by exact integer comparison of token quotients.
31. **`placement`.** D5 says positions are drawn from the `padding` stream. `drawn` draws each
    paragraph's position before a step of the plan as it then stands, never after the last step,
    so the conclusion stays last; `before` puts every paragraph before the first step, in drawn
    order, to realize the FLAWS.md example's "preceded by"; the paragraph order is drawn in both.
    The stream label is `padding`, D4's example label.
32. **One instance per paragraph.** D5's "one record per inserted paragraph" is read with ruling 3
    on PR 26: each paragraph is an instance, with two records in a pair, so `n_flaws_0013` and
    `n_flaws_total` count the paragraphs.
33. **Padding steps are never lemmas.** A paragraph is a plain step in the informal file (so
    `n_lemmas` is unchanged) and a `theorem` declaration in the formal file. The kinds
    `unused-lemma` and `verbose-rewrite` are parsed but have no material in 0.4.0 and are refused
    for lack of it; an `unused-lemma` paragraph as a `Lemma N.` step would need the renumbering
    of later lemmas decided, and a `verbose-rewrite` paragraph a position anchored to the step it
    rewrites.
34. **`padding_ratio` of a formal-only artifact.** ADR-0001 D4 defines the ratio as the tokens
    inside the FLAW-0013 spans over `n_tokens`, and spans exist only in informal locations. For an
    informal or pair artifact the ratio is the padded steps' tokens over the informal `n_tokens`;
    for a formal-only artifact it is the padding declarations' lines' tokens over the formal
    `n_tokens`, since that file is the one `n_tokens` is measured on.
35. **One spec per artifact.** Padding with an injected flaw in one artifact is refused in 0.4.0,
    as a second whole-argument variant was in 0.3.0; FLAW-0011's composite and padded flawed
    arguments are later versions. `pad` runs on the faithful argument only, so a false-statement
    seed has nothing to pad.
36. **The record dials.** A padding record carries no `original_steps` or `original_fragments`
    (nothing is replaced; the argument is recoverable by deleting the recorded steps, Options 8)
    and carries `paragraph` and `kind`, so the draw is reversible from the manifest.
37. **The goldens' commanded ratios** are the achieved ratios to two decimals (0.28 and 0.71 for
    informal and pair, 0.31 and 0.61 for formal-only), not the exact values, so the goldens
    exercise the tolerance rather than an equality.
38. **Check T for FLAW-0013** is the FLAW-0007 `unused` condition applied to each padding
    declaration: listed by the audit tail and outside the main declaration's transitive
    used-constants, closed over the file's own constants.

## Al's rulings on PR 29, and their application

Al ruled on 2026-10-28 (verbatim in ADR-0011 *Amendments*, STATE *Standing rulings* and HANDOFF
Session 43's addendum; copy `owner-rulings-pr29.txt` under the external root, 696 bytes, SHA-256
`b9e05c65bbcdb85d7ec9c0ef7ef139c05101bbf7e7a6ea97b6abd34562f8fec7`): readings 29 through 38 are
accepted as read, and reading 30 is recorded with Al's reading of D4 and D8 — the integer `count`
is the generation decision because D4 forbids a floating-point value there; `padding_ratio` is a
commanded statistic measured from the emitted file and checked at `abs 0.05`; a ratio the count
cannot reach is a FAIL verdict; a ratio-targeted mode, if a later version wants one, must choose
its count by exact integer comparison of token quotients. 0.4.0 already does what the ruling
records, so nothing was regenerated and the producer version stays 0.4.0. The records are commit
4 of the branch; the gate on its committed tree, the step-7 recheck and the push are in the pull
request and under the external root.
