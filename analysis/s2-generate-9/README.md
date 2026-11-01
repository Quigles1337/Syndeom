# S2 implementation packet 9 — `s2_grammar` 0.2.0 (ADR-0011, Session 48, 2026-11-01)

This packet records the ninth implementation session of ADR-0011 (D11): `s2_grammar` 0.2.0 —
FLAW-0007 `unused` and FLAW-0013 on the grammar `ring_chain`, with material generated from the
atom library (D5, D6) — with its goldens. Nothing here is a result: no CLAIM, no fidelity sweep,
no artifact under `corpora/`.

Branch `claude/s2-grammar-0.2.0`, from `main` `153140185cbb6a371d950b2f3d22cca95c093f82` (PR
33's merge):

| Commit | What |
|---|---|
| `de6965c55e9f296f7456bd3934ade481cd16bf2d` | the 0.2.0 producer: `src/grammar.rs` (FLAW-0007 `unused`, FLAW-0013, the version), `src/template.rs` (`Prepared.further`, measured by the one `assemble`; `seed_form` reserved), `src/lib.rs`, `tests/grammar.rs`, the crate README |
| `18f43844c968d00961c42e11cbf6f3a2a3b6e4c3` | the 8 goldens of `s2_grammar` 0.2.0, produced at `de6965c5…`, the goldens README and the count (87 cases) |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 48 |

## What was built

- **FLAW-0007 `unused` on `ring_chain` (D5).** `count` (1 to 13) true irrelevant identities —
  each a seed form the instance does not use, drawn without replacement on
  `injector/FLAW-0007/1` (so at most the other thirteen forms), equated to a presentation of its
  expansion drawn on the same stream — inserted as lemma steps (`theorem …_unused_<i> : ∀ x y :
  ℤ, e = p := by intro x y; ring`; `Lemma N.` in the informal file, with the cited lemmas' text
  shape) after the cited lemmas in document order and numbered after them, so that no faithful
  step changes and no cited lemma is renumbered; cited by nothing. Each lemma is its own
  instance, cosmetic and `YES-TOOLING` in both files: the informal record spans the whole step,
  the formal record is the declaration at its first line, confirmed by check T as `s2_template`
  0.3.0 reads it (the declaration listed by the audit tail and outside the main declaration's
  used-constants closure over the file's own constants). `length_steps` is 1 (one identity per
  lemma); the site is `auto` (the lemmas' positions are fixed); `vacuous` is refused
  (hand-authored unsatisfiable hypotheses; `s2_template`-only); a severity other than cosmetic
  is refused. The dials carry `count`, `length_steps`, `site`, `seed_form` (the form's 1-based
  index in the table), `inserted`, `replaced: null`, empty `original_steps` /
  `original_fragments` and, in the formal file, `declaration`. `Prepared` gains `further`, the
  instances after the first of a spec that yields several, and `assemble` measures
  `injection` and `further` alike; counts are per instance, so `n_flaws_0007` is `count`,
  commanded `n_lemmas` is `length.n_lemmas + count` and `n_steps` `length.n_steps +
  length.n_lemmas + count`.
- **FLAW-0013 on `ring_chain` (D5), as `s2_template` 0.4.0 reads it.** Commanded by one
  FLAW-0013 spec (`kinds`, `count` 1 to 32, `placement` `before` | `drawn`) together with a
  positive `padding_ratio`, and by nothing else. The paragraphs are generated on the stream
  `padding`: per paragraph a kind among the spec's kinds (`below(kinds.len())`), then the
  identity's draws — `re-derivation`: the instance's own polynomial in two drawn presentations;
  `unused-lemma`: a seed form the instance does not use (drawn with replacement) equated to a
  drawn presentation of its expansion — then, under `drawn`, a position before a step of the
  plan as it then stands (`before`: every paragraph before the first step, in drawn order).
  `restated-definition` and `verbose-rewrite` have no generated material and are refused. Each
  paragraph is a plain step (never `Lemma N.`; a fixed text template per kind) with its unused
  declaration `…_pad_<i>` (`∀ x y : ℤ, a = b`, proved by `intro x y; ring`), its own instance:
  the informal record `NO` over the whole step, confirmed when the step holds exactly the
  paragraph's text; the formal record `YES-TOOLING` at the declaration's first line, confirmed
  by check T. `padding_ratio` is measured as 0.4.0 measures it and compared only in `measure`.
  The dials carry the spec's dials, `ratio`, `site`, `kind`, `seed_form` (where a seed's
  material would carry `paragraph`: a grammar has no material list), `inserted`,
  `replaced: null` and, in the formal file, `declaration`.
- **Refusals (D3, D6).** Under 0.2.0: a positive `padding_ratio` without the FLAW-0013 spec, or
  the spec with ratio 0; two specs (a padded flawed argument is a later version); every class but
  FLAW-0001, FLAW-0007 `unused` and FLAW-0013; FLAW-0007's `count` outside 1–13, `length_steps`
  other than 1, a sub-variant other than `unused`, a severity other than cosmetic, an explicit
  site, a missing or extra dial; FLAW-0013's kinds without generated material, an empty or
  repeated kinds list, `count` outside 1–32, a placement other than `before` | `drawn`, a
  sub-variant, an explicit site, a severity other than cosmetic; `seed_form` as a config dial
  (reserved). Under 0.1.0, every FLAW-0007 and FLAW-0013 spec and every positive ratio stay
  refused, so the 0.1.0 goldens regenerate under the version they name.
- **Versions (D1).** `GrammarVersion::V0_2_0` is current; `generate` emits 0.2.0;
  `Producer::parse` admits nine producer versions. The bytes 0.2.0 emits for a 0.1.0 config are
  0.1.0's: only the version, the id and the manifest differ (tested).

## Evidence that the Lean material does what the records say

1. **Probes before the module** (`evidence/lean-probe/`, hand-written; `lake env lean` from
   `lean/`, one process at a time, `run.sh`; 2026-11-01T17:49:14Z–17:49:30Z, after the ORIENT
   gate had ended; the log `probes.log`):

   | Probe | Exit | What |
   |---|---|---|
   | `s2l_rc_unused` | 0 | a four-step chain on `(x - 2 * y) ^ 2` with Lemma 1 cited at step 2 and two unused lemmas (`(x + y) ^ 2 = y * (2 * x + y) + x ^ 2`, `(x + y) * (x - y) = x * x - y * y`) after it; `[propext, Quot.sound]`; the audit tail lists the four declarations and no auxiliary constant; the closure from the main declaration is `{main, lemma_1}` |
   | `s2l_rc_padded` | 0 | the same chain with a padding declaration before Lemma 1 (a re-derivation of the chain's polynomial, `x ^ 2 - 4 * x * y + 4 * y ^ 2 = (4 * y ^ 2 + x ^ 2) + (-4 * y * x)`) and one after it (`(x + y + 1) ^ 2 = x * (x + 2 * y + 2) + y ^ 2 + 2 * y + 1`); `[propext, Quot.sound]`; the closure from the main declaration is `{main, lemma_1}` |

   The probes fix the two shapes the generator may emit: unused lemmas after the cited ones, and
   padding declarations on either side of a cited lemma; both are declarations that nothing
   cites, listed by the audit tail and outside the main declaration's closure, which is what
   check T reads. The module was written after they compiled.
2. **The ratio probe** (`evidence/ratio-probe9.py`, `evidence/ratio-probe-summary.txt`,
   `evidence/ratios.json`; 2026-11-01T18:00:52Z–18:01:18Z, at `de6965c5…`): the FLAW-0013
   golden config (`kinds` `re-derivation` and `unused-lemma`, `count` 2, `drawn`) produced
   once with ratio 1 for the informal and formal targets, to read what its paragraphs achieve
   at seed 7 — 0.4 (62 of 155 tokens) over the informal file and 0.3704 (70 of 189 tokens) over
   the formal file. The goldens command these to two decimals (0.4 for informal and pair, 0.37
   for formal-only), as the 0.4.0 goldens do (reading 37; Al's reading on reading 30); the two
   FAIL-verdict probe artifacts are kept under the external root as the record and are not
   goldens.
3. **The goldens.** 8 cases — the control at the short length (`n_steps` 4, `n_lemmas` 0;
   informal, formal), FLAW-0007 `unused` with `count` 2 (informal, formal, pair) and FLAW-0013
   with `count` 2, `drawn` (informal, formal, pair) — each produced by `generate --dry-run` with
   seed 7 at `de6965c55e9f296f7456bd3934ade481cd16bf2d` by two separate runs
   (`evidence/goldens-run9.py`; run A 2026-11-01T18:01:43Z–18:03:50Z, run B 18:04:35Z–18:06:38Z;
   every case exit 0, fidelity PASS; the provenance check passed against the commit before
   each run; `evidence/goldens-run9a-summary.txt`, `evidence/goldens-run9b-summary.txt`).
   `evidence/goldens-copy9.py` compared the runs file by file — every `proof.md`, `Proof.lean`
   and `manifest.json` byte-identical, every observation equal, one engine commit — checked
   every case's shape and placed run A's outputs under `engine/generate/tests/goldens/ring_chain/`
   (`v0.2.0-…`) with a `case.json` each (`evidence/goldens-production-report.json`, SHA-256
   `54eb43d07a0bac654e409f62e07336d4948be07b143cdeb32c0325d5e94e38e7`, 62,669 bytes):
   - the control's proof bytes are the 0.1.0 short control's (`(x - 2 * y) ^ 2` at seed 7);
     its config hash is 0.1.0's and its id differs by the version
     (`s2_grammar-v0.2.0-cc0c6a4daf03-s7` formal, `…-82300f31bd83-s7` informal); the formal
     control builds and prints `[propext, Quot.sound]`;
   - the FLAW-0007 cases insert Lemma 1 `(x - y) ^ 2 = y * y + x * (-2 * y + x)` (seed form 2)
     and Lemma 2 `(x + y) * (x - y) = -y ^ 2 + x ^ 2` (seed form 3) at steps 1 and 2, the
     declarations `…_unused_1` at line 3 and `…_unused_2` at line 7; the formal files build,
     print `[propext, Quot.sound]`, and the main declaration's closure holds itself only, so
     both instances are confirmed (`flaws_confirmed` 2); `proof_status: valid`, `correct: true`,
     `n_lemmas` 2 and `n_steps` 6 commanded where an informal file exists, `lean_build` `pass`,
     `lean_axioms_standard` `true`; 143 informal tokens, 179 formal tokens;
   - the FLAW-0013 cases draw a `re-derivation` of the instance's own polynomial
     (`x * (-4 * y + x) + 4 * y ^ 2 = 4 * y * y + x * (x - 4 * y)`; `seed_form` 5) placed before
     step 4 of the plan as it then stood, and an `unused-lemma` (`(x + y) * (x - y) = -y ^ 2 +
     x ^ 2`; `seed_form` 3) at step 2 — the declarations `…_pad_2` at line 3 and `…_pad_1` at
     line 7, in document order; the formal files build, print `[propext, Quot.sound]`, the
     closure holds the main declaration only; the informal records (`NO`) span steps 4 and 2
     whole; achieved ratios 0.4 and 0.3704 within `abs 0.05` of the commanded 0.4 and 0.37;
     `n_steps` 6, `n_lemmas` 0; 155 informal tokens, 189 formal tokens;
   - every case is a true statement whose reference is the 0.2.0 short formal control
     (`s2_grammar-v0.2.0-cc0c6a4daf03-s7:syndeom_s2_grammar_ring_chain`), which names itself;
   - the receipts, with build and audit stdout and stderr, are under the external root
     `C:\Users\LEET\Codex\Temp\syndeom-s2-grammar-020-20261101T1800Z\goldens-9a\` and
     `goldens-9b\`; the SHA-256 of the sorted list of the eight `receipt.json` SHA-256s (one per
     line, LF-terminated) is `33060ebf6bee9a74f7b5e7f0886fa2d2ec69e0bb94994dfb945558af2e6ff167`
     (A) and `64811f83b8a1bd58ef9c56d945a945ab63772454cecae8a52e9df76fe88bddf6` (B); the
     receipts differ only in their timings.
4. **The artifact ids** (`s2_grammar-v0.2.0-<config_hash[0:12]>-s7`): control short
   `82300f31bd83` / `cc0c6a4daf03` (informal / formal); FLAW-0007 `unused` `b547d2bf7d1a` /
   `0924ba72990b` / `79eaa0f0ccb5` (informal / formal / pair); FLAW-0013 `0aa6313e3136` /
   `cdd5d214eb95` / `c85d97da4289`.
5. **Tests and lint.** The crate's 48 tests pass (goldens 1 over 87 cases, grammar 13,
   properties 24, provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features --
   -D warnings` and `cargo fmt --check` are clean. New, in `tests/grammar.rs`: FLAW-0007
   `unused` over four lengths and counts up to 13 (the plan — the cited lemmas, then the unused
   ones, then the chain, no faithful step changed; the identities' forms distinct and never the
   instance's; the texts and declarations; nothing citing them; the dials; two records per
   instance at the recorded locations; the counts; check T's refusal of confirmation when the
   main declaration uses a lemma directly or through an auxiliary, and without a listing; the
   informal-only and formal-only targets; determinism; the refusals; 0.1.0 refusing the class)
   and FLAW-0013 over five configurations up to 32 paragraphs (the paragraphs' kinds, texts,
   forms and declarations; the argument's order preserved; the records, informal `NO` and
   formal `YES-TOOLING`; the ratio's tolerance and the FAIL verdict of an unreachable ratio;
   both kinds drawn over seeds; determinism; twenty refusals; 0.1.0 refusing padding); the
   longest instance with 13 unused lemmas and with 32 paragraphs; the version facts; the 0.1.0
   bytes of a control and a FLAW-0001 artifact unchanged under 0.2.0. The gate on the committed
   tree is recorded in the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-11-01T17:55:04Z, outside the gate (`cargo audit --file
engine/Cargo.lock`). It loaded 1,395 advisories (advisory-db commit
`2295466c4e9df26647fd67d8fa1c705ac2fc7f81`), scanned 35 crate dependencies, and reported no
vulnerability (exit 0; `evidence/cargo-audit.txt`). `engine/Cargo.lock` is unchanged by this
revision: no dependency was added.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `de6965c5…` — `engine/generate/src/grammar.rs` (`insert_unused`, `pad`,
`other_forms`, `form_identity`, `identity_claim`, the version gate in `prepare_with`),
`src/template.rs` (`Prepared.further`; `assemble` iterating `injection` and `further`;
`seed_form` reserved), `src/lib.rs`, `tests/grammar.rs`. The threat model is packet 1's: an
offline research CLI whose assets are the integrity of generated ground truth and the operator's
machine, and whose trust boundary is an artifact directory handed to `verify` and a config
handed to `generate`. New this revision: two integer dials (`count` for either class) size a
generated insertion; a list of kind names and a placement name select among constants.

**Pre-scan (Phase 1):** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate
roots and in the manifest crate; no external dependency was added; clippy at `-D warnings` is
clean. `cargo geiger`, `cargo deny` and Miri are not installed on this machine and were not run;
there is no `unsafe` to map.

**Panic and index sweep of the diff (2C):** no `unwrap`, `expect` or `panic!` in the edited
sources outside tests. The indexes: `spec.dials["count"]`, `["length_steps"]`, `["kinds"]`,
`["placement"]` sit behind `exact_keys` on the dials (a `Map` index of a missing key would
panic; the key set is checked first, as 0.1.0 checks `perturbation`); `remaining.remove(stream.
below(remaining.len()))` removes from a list of thirteen at most thirteen times (`count` ≤ 13 is
checked first); `kinds[stream.below(kinds.len())]` and `others[stream.below(others.len())]` index
non-empty lists (kinds checked non-empty; `others` has thirteen entries); `plan.steps.insert(site
- 1, …)` inserts at `m + i - 1 ≤ plan.steps.len()`; `steps.insert(position, …)` at `instance - 1`
(`before`, at most the current length) or `below(steps.len())`; `lemmas.next().ok_or(…)` is a
refusal, not a panic. Integer arithmetic (2I): `count` is bounded before any use; the identities'
coefficients come from the fixed table and `poly_pow`/`poly_mul` on its own forms (magnitude at
most 25, as packet 8 records). No input-controlled quantity enters an arithmetic operation.

**Serde and the config (2F):** the kind names are parsed by `serde_json::from_value::<PaddingKind>`
(an unknown name is refused), the counts via `as_u64` with the range filter, the placement by
string comparison with two constants; `seed_form` joins the reserved dial names, so a config
cannot forge a recorded dial. `form` could not be reserved: it is FLAW-0003 `direct`'s config
dial, which the goldens test caught at the first run (the record is in HANDOFF Session 48).

**Subprocesses and `verify`:** unchanged. `verify` compiles only bytes the producer the manifest
names itself emits for the manifest's `(config, seed)`, and the audit tail names only the
producer's main declaration (FINDING-011's guard holds). No config text reaches the proof text:
the kinds and the placement are compared to constants, the counts are bounded, and every byte of
both files comes from the fixed table, the renderers and the fixed paragraph templates.

**Determinism:** `BTreeMap`, `BTreeSet` and `Vec` only; the config's `padding_ratio` enters no
generation decision (the integer `count` does; the ratio is copied into the records and compared
in `measure`); every draw is labelled (`injector/FLAW-0007/1`, `padding`).

No new finding. **FINDING-018** (informational, backlog: `derive`, `Derivation::n` and `render`
trust their callers' preconditions) stands and its scope gains `form_identity`, which indexes
`SEED_FORMS` by a form its callers draw from `other_forms` or take from the derivation; the
recommendation is unchanged. FINDING-015 does not extend to the grammar (packet 8);
FINDING-016 and FINDING-017 (informational, backlog) stand; FINDING-006 (accepted risk),
FINDING-007 (backlog), FINDING-009, FINDING-010, FINDING-012 and FINDING-013 (accepted) stand as
packets 1–8 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 8's numbering.

100. **The unused lemmas follow the cited lemmas in document order and are numbered after
     them.** D5 names no position for FLAW-0007 `unused`'s lemmas; the template placed its one
     lemma at the material's site. Placing them after the cited lemmas leaves every faithful
     step, every cited lemma's number and name and every citation unchanged, so the records
     carry no originals. The alternative — positions drawn among the lemma block — would
     renumber the cited lemmas after each insertion, changing their `Lemma N.` texts, their
     names (`…_lemma_<j>`, which carry the number) and the chain's citations, and the records
     would then list every renumbered step as changed; listed, not taken. A detector that keys
     on "the last lemmas are the unused ones" keys on a generator fingerprint, which S3.3's
     leave-one-generator-out measures.
101. **Each unused lemma is its own instance,** as each padding paragraph is (reading 32), so
     `n_flaws_0007` is `count` and a pair has two records per lemma; the first instance is
     `Prepared.injection` and the rest `Prepared.further`, which the one `assemble` measures
     exactly as it measures `injection` (reading 112).
102. **The material of an unused lemma** is a seed form the instance does not use, drawn
     without replacement — so `count` is at most thirteen — equated to a presentation of its
     expansion drawn on the same stream: the chain's own first-step shape, so an unused lemma
     is indistinguishable in shape from a cited one. `length_steps` is fixed at 1 (one identity
     per lemma, as the template's material has it); a multi-step unused lemma — a chain inside
     a lemma — is a later version.
103. **`seed_form` is recorded** (the form's 1-based index in the table) where a seed's material
     records `paragraph`, so the draw is reversible from the manifest; it joins the reserved
     dial names. `form` was the first choice and could not be reserved: it is FLAW-0003
     `direct`'s config dial, and the goldens test refused the 0.6.0 goldens at the first run.
104. **FLAW-0007 `unused`'s site is `auto` only** (the lemmas' positions are fixed by 100), and
     `vacuous` is refused: a lemma with jointly unsatisfiable hypotheses and its application are
     hand-authored mathematics, `s2_template`-only (D6's list).
105. **Which of D5's four kinds a generated identity can be.** A `re-derivation` is the
     instance's own polynomial presented again — two drawn presentations equated, "a check" of
     the theorem's own identity; an `unused-lemma` is an identity of a seed form the instance
     does not use. `restated-definition` has no material: the chain states no definition to
     restate. `verbose-rewrite` has no material: a verbose rewrite of a chain step would need a
     position anchored to that step (reading 33) and a text that re-derives it, which is the
     chain's own step again. Both are refused for lack of material, as the template refuses the
     kinds its seed lacks.
106. **The padding draw order** per paragraph on `padding` is the kind (`below(kinds.len())`,
     with replacement), the identity's draws (two presentations for a `re-derivation`; the form
     with replacement and one presentation for an `unused-lemma`) and then, under `drawn`, the
     position — the template's order (the paragraph, then the position) with the generated
     material's draws in the paragraph's place. The template draws its paragraphs without
     replacement because its material is finite; a repeated form here yields a different
     presentation in general, and a repeated paragraph would still be true irrelevant material.
107. **The paragraph texts are fixed templates** (`As a check, the same polynomial admits
     another presentation: …`, `Note the identity: …`), as the chain's justifications are. A
     surface detector may key on them; that is a documented-weak control's expected behaviour
     (§0.7), not a defect, and a structural detector sees the unused declaration either way.
108. **The 0.2.0 goldens include the short control (informal, formal),** which STATE's NEXT did
     not name, so that the flawed goldens' truth reference (the formal control of the same
     version, length and seed; reading 90) names a golden; the control's proof bytes are
     0.1.0's. Eight cases, not six.
109. **Commanded `n_lemmas` counts the unused lemmas** (they are `Lemma N.` steps) and
     **`n_steps` counts them and the padding paragraphs** — D8's "the plan's step count after
     injection and padding", the template's rule applied unchanged; both only where an informal
     file exists, as before.
110. **`before` places the paragraphs before the plan's first step,** which may be a lemma step,
     so a padding declaration can precede a lemma declaration in the formal file (the padded
     probe compiles that shape); under `drawn`, the declarations' document order is the steps'
     order, not the instances' (the goldens' `…_pad_2` precedes `…_pad_1`).
111. **The goldens' commanded ratios** are the achieved ratios to two decimals (0.4 for informal
     and pair, 0.37 for formal-only), not the exact values, so the goldens exercise the
     tolerance (reading 37).
112. **`Prepared.further`** is an additive field beside `injection`, rather than `injection`
     becoming a list: every `s2_template` version leaves it empty, and the tests and the
     template's own code keep reading `injection`; D5's "several instances in one artifact are
     applied in `instance` order" is realized for one spec's instances, which cannot overlap.
