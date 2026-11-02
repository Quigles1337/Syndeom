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
  0.5.0, 0.6.0 and 0.7.0: seed records, the config and its refusals (D3), the injector (D5; from
  0.5.0 with FLAW-0004's hedge lexicon and FLAW-0011 as a commanded composite; from 0.6.0 with
  FLAW-0010's fictional-reference lexicon and FLAW-0003's `direct` and `cycle`; from 0.7.0 with
  FLAW-0008's statement drift and FLAW-0012's dropped hypothesis), the padding module (D2 `pad`,
  D5 FLAW-0013; 0.4.0), ground truth by construction (D7), and the manifest with checks K and T
  (D8, D9);
- `src/grammar.rs` — the producer `s2_grammar`, in its versions 0.1.0, 0.2.0, 0.3.0, 0.4.0,
  0.5.0, 0.6.0, 0.7.0 and 0.8.0: the shared config and its checks, the grammars each version
  carries (`grammars`) and the classes each realizes (`realizes_on`), and the grammar
  `ring_chain` (D6) — its atom library (the seed forms, the monomials and the
  presentations a chain is built from), the derivation (`grammar/plan`, `grammar/step/<k>`,
  `grammar/lemma/<j>`), the plan with its lemma steps, ground truth by construction with the
  control as the truth's reference (D7), FLAW-0001 `propagated` and `local` with generated
  material (D5), from 0.2.0 FLAW-0007 `unused` (irrelevant identities as lemma steps cited by
  nothing, drawn on `injector/FLAW-0007/1`) and FLAW-0013 (padding paragraphs of generated
  identities with their unused declarations, drawn on `padding`), from 0.3.0 FLAW-0009
  `axiom` (a cited lemma's declaration replaced by an axiom — of the same identity, or of a
  false one used ex falso at the citing step — drawn on `injector/FLAW-0009/1`), and from
  0.4.0 FLAW-0010 `a` (a ring-closed step's justification replaced by a fictional reference
  from the template's lexicon, drawn on `injector/FLAW-0010/1`; the formal file unchanged), and
  from 0.5.0 FLAW-0003 `cycle` (a cited lemma's closing note replaced by a citation of the
  theorem, the lemma drawn on `injector/FLAW-0003/1`, the chain step citing the lemma unchanged,
  so the plan's premise graph holds the cycle; the formal file unchanged), and from 0.6.0
  FLAW-0004 (a ring-closed step's justification replaced by a hedge drawn from the template's
  lexicon on `injector/FLAW-0004/1` and prefixed to the bare claim, the `calc` line's tactic
  omitted so that the kernel reports an unsolved goal at that line), and from 0.7.0 FLAW-0011,
  the composite (one embedded gap-or-fatal instance of a class the grammar realizes, applied by
  that class's own injector, the composite's records at the statement); the plan is rendered
  and measured by the same renderers and the same `assemble` as the template's;
- `src/ineq.rs` — from `s2_grammar` 0.8.0, the second grammar, `ineq_chain` (D6): its atom
  library (three families of true atoms over ℝ), the expressions in merged normal form, the
  derivation (`grammar/ineq_chain/step/<k>`, `grammar/ineq_chain/lemma/<j>`), the plan with its
  lemma steps, ground truth by construction with the formal control as the truth's reference
  (D7), and FLAW-0001 `propagated` and `local` read for inequalities (a perturbation admitted
  only when a recorded grid point shows the perturbed step false; drawn on
  `injector/FLAW-0001/1`); rendered and measured by the same renderers and `assemble`;
- `src/lib.rs` — the producer dispatch: `generate` prepares the producer a config names
  (`theorem`: `s2_template`; `grammar`: `s2_grammar`), `regenerate` and `verify` the producer and
  version a manifest names (`Producer`);
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
- **`s2_template` 0.6.0** (Session 45): the 0.5.0 seeds, with
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
- **`s2_template` 0.7.0** (Session 46; the version `generate` emits): the 0.6.0 seeds and two
  more — `bolzano_weierstrass` (FLAWS.md's FLAW-0008 example: every bounded real sequence has a
  convergent subsequence; three aligned steps on the compact interval `[-M, M]`, with a witness
  module audited by the gate), carrying FLAW-0008 material for each of D5's five
  transformations, and `bounded_continuous_sup` (FLAWS.md's FLAW-0012 example: every bounded
  continuous `f : ℝ → ℝ` attains its supremum, false by `arctan`; the extreme-value argument of
  a compact interval transplanted, in four aligned steps; the witness module proves the
  negation), carrying FLAW-0012 material; FLAW-0008 (D5: the formal statement drifts by exactly
  one recorded edit while the informal file stays faithful; the drifted file builds with its own
  proof, `NO`, the record formal-only at the main declaration's header line) and FLAW-0012 (D5:
  the FLAW-0005 `violated` instance at the hidden use site, check K, and the class's own
  instance at the statement, confirmed with it and the seed's negation witness).

- **`s2_grammar` 0.1.0** (Session 47): the grammar `ring_chain` with its control and FLAW-0001
  (`propagated`, `local`; `perturbation` ∈ {`sign`, `coefficient`, `constant`, `exponent`}); see
  *The grammar `ring_chain`* below. Flaw counts per instance.
- **`s2_grammar` 0.2.0** (Session 48): FLAW-0007 `unused` (`count` 1 to 13 irrelevant
  identities, each a seed form the instance does not use, as lemma steps after the cited
  lemmas, cited nowhere, one instance each; `length_steps` 1) and FLAW-0013 (`count` 1 to 32
  generated padding paragraphs of the kinds `re-derivation` and `unused-lemma`, each with its
  unused declaration; `padding_ratio` measured) on `ring_chain`; the 0.1.0 bytes unchanged.
- **`s2_grammar` 0.3.0** (Session 49): FLAW-0009 `axiom` on
  `ring_chain` — a cited lemma's declaration replaced by an `axiom` with no proof: of the same
  identity for `truth: true` (gap), or, for `truth: false` (fatal), of a false identity (the
  lemma's right-hand side perturbed once) used ex falso at the citing step, the axiom's instance
  at a witness point refuted by `norm_num`; the informal file unchanged, the record formal-only
  at the axiom's declaration, check T on the axiom print; `native_decide` refused; the 0.1.0 and
  0.2.0 bytes unchanged.
- **`s2_grammar` 0.4.0** (Session 50): FLAW-0010 `a` on
  `ring_chain` — a ring-closed chain step's justification replaced by a fictional reference
  drawn from `s2_template` 0.6.0's lexicon on `injector/FLAW-0010/1` (the site, then the
  author, the year, the venue of the `shape` and the result); the step `By <reference>` and the
  bare claim; `claim: true` (gap) only, `claim: false` refused and, by Al's rulings on PR 36,
  without material on the grammar as on the template; the formal file unchanged, the record
  informal-only, `N/A`, confirmed by the FLAW-0010 check; the 0.1.0, 0.2.0 and 0.3.0 bytes
  unchanged.
- **`s2_grammar` 0.5.0** (Session 51): FLAW-0003 `cycle` on
  `ring_chain` — a cited lemma, drawn on `injector/FLAW-0003/1`, has its informal closing note
  replaced by a citation of the theorem (`this follows from the theorem`), recorded as the
  lemma step's premise, while the chain step that cites the lemma is unchanged, so the plan's
  premise graph holds the cycle `[…_lemma_<j>, syndeom_s2_grammar_ring_chain]`
  (`cycle_length` 2, fatal); the formal file unchanged, the record informal-only, `NO`, at the
  lemma step spanning the citation phrase, confirmed by the structural check; `direct` refused
  (`s2_template`-only); the 0.1.0 to 0.4.0 bytes unchanged.
- **`s2_grammar` 0.6.0** (Session 52): FLAW-0004 on
  `ring_chain` — a ring-closed chain step, drawn on `injector/FLAW-0004/1`, has its
  justification deleted and a hedge, drawn next on the same stream from the template's fixed
  lexicon, prefixed to the bare claim; the aligned `calc` line loses its tactic step (`_ = e_k
  := by ring` becomes `_ = e_k := by`, the `by` kept, never `sorry`), so that Lean reports one
  `unsolved goals` error at that line (check K at the site); `hidden_step` `routine-true` only
  (cosmetic; `nontrivial-true` and `false` refused for lack of material); a pair's instance
  `YES-KERNEL` in both files with `lean_build` commanded `fail`, an informal-only artifact's
  `N/A`; the target `formal` refused; the 0.1.0 to 0.5.0 bytes unchanged.
- **`s2_grammar` 0.7.0** (Session 53): FLAW-0011, the composite, on
  `ring_chain` — the one dial `embedded` lists exactly one FlawSpec of a class FLAW-0001 through
  FLAW-0010 that the grammar realizes, of severity gap or fatal (FLAW-0001 `propagated`,
  FLAW-0003 `cycle`, FLAW-0009 `axiom` with either truth, FLAW-0010 `a` with `claim: true`), the
  composite's severity equal to it; the embedded spec applied by its class's own injector as
  instance 1, unchanged (its site, draws, dials and target rule; the proof files the embedded
  artifact's byte for byte), and the composite as instance 2, its records at the statement in
  each file the artifact has, `lean_catchable` inherited; every composite an invalid proof of a
  true statement; no new RNG label; the 0.1.0 to 0.6.0 bytes unchanged.
- **`s2_grammar` 0.8.0** (Session 54; the version `generate` emits): the second grammar,
  `ineq_chain`, with its control and FLAW-0001 `propagated` and `local` — see *The grammar
  `ineq_chain`* below; every other class refused on it; `ring_chain` unchanged (its 0.7.0 bytes
  and draws; only the version, the id and the manifest differ).

The crate regenerates all fifteen producer versions (seven of `s2_template`, eight of `s2_grammar`):
the goldens select the producer and version their manifests name, and `regenerate` and `verify`
accept any of them. Every seed and each grammar's atom library state their `source`, never under
`groundtruth/` (Al's ruling 1 of 2026-10-27).

## The grammar `ring_chain` (`s2_grammar` 0.1.0, 0.2.0, 0.3.0, 0.4.0, 0.5.0, 0.6.0, 0.7.0 and 0.8.0)

- **The instance.** A config `{"grammar": "ring_chain", "length": {"n_steps", "n_lemmas"},
  "target", "flaws", "padding_ratio"}` (D3; `n_steps` 1 to 256, `n_lemmas` 0 to 32 and at most
  `n_steps`; `padding_ratio` 0 unless FLAW-0013 is commanded, from 0.2.0) names a polynomial
  identity over ℤ in `x` and `y`: a seed form `e_0` — one of fourteen products and powers of
  small binomials and trinomials (`(x + y) ^ 2`, `(x + 2 * y) * (x - y)`, `(x + y + 1) ^ 2`, …),
  drawn from the stream `grammar/plan` — and `n_steps` presentations `e_1 … e_n` of its
  expansion, each drawn from its own stream `grammar/step/<k>` and differing from the one
  before. The theorem is `e_0 = e_n` for all integers `x`, `y`; its argument is a `calc` chain
  whose step `k` is the equality `e_{k-1} = e_k`, closed by `ring`. `n_lemmas` of the chain's
  steps (drawn from `grammar/lemma/<j>`, ascending) are instead stated as lemma steps
  (`theorem …_lemma_<j> : ∀ x y : ℤ, e_{k-1} = e_k := by intro x y; ring`, before the main
  declaration in document order, D2) and cited by the chain step (`_ = e_k := …_lemma_<j> x y`;
  the step's `premises` name the lemma). Every equality of the chain is a true identity by
  construction: `e_1 … e_n` present one polynomial.
- **Presentations (the atom library's rewrites).** A presentation of a polynomial — a canonical
  list of monomials `c · x^i · y^j` — is a drawn permutation of its terms, a drawn style per
  term (`x ^ 2` or `x * x`; `x * y` or `y * x`) and a drawn shape: expanded; one variable
  factored out of the terms it divides (`x * (x + 2 * y) + y ^ 2`), first or last; or the terms
  regrouped as two parenthesized sums. A shape the terms cannot take (a variable dividing fewer
  than two terms; a grouping of two terms) resolves to expanded. The LaTeX rendering juxtaposes
  factors (`2xy`, `x(x + 2y) + y^2`) and writes a split square with `\cdot`.
- **Ground truth (D7).** `statement_status` is `value: true`, `method: lean-kernel`,
  `reference: <artifact_id of the control with target formal, the same grammar, length and
  seed>:syndeom_s2_grammar_ring_chain` — the control whose `lean_build: pass` is measured and
  recorded in its own manifest (D8); the formal control's reference names itself.
- **FLAW-0001 (D5).** The injector draws, on `injector/FLAW-0001/1`, a ring-closed chain step
  (never a lemma-cited one; for `local`, never the last) when the spec's `site` is `auto`, and
  the perturbed monomial among the kind's eligible targets at that step: `sign` flips one term's
  sign; `coefficient` raises one non-constant term's coefficient by one in magnitude;
  `constant` does the same to the constant term, or adds `+ 1` when there is none; `exponent`
  raises one exponent by one where the raised monomial is not another term's. The site is
  re-rendered under its own presentation with the perturbed terms, so its identity is false and
  `ring` fails at that `calc` line (check K; the probes under `analysis/s2-generate-8/`).
  `propagated` (fatal) re-derives every later step, and restates every lemma a later step cites,
  from the perturbed polynomial, so that every later identity is true again and the chain ends
  away from the claim of record (step 0 is untouched); `local` (repairable) changes the site
  alone, so the next step asserts the correct consequence. The records' dials carry
  `perturbation`, `site` (a plan step index), `monomial` (`replaced`, `inserted`), the site's
  text or fragment `inserted` and `replaced`, and `original_steps` / `original_fragments` for
  every step the variant changed (a restated lemma's lines included).
- **FLAW-0007 `unused` (0.2.0; D5).** `count` (1 to 13) true irrelevant identities, each a seed
  form the instance does not use — drawn without replacement on `injector/FLAW-0007/1` —
  equated to a presentation of its expansion drawn on the same stream, inserted as lemma steps
  (`theorem …_unused_<i> : ∀ x y : ℤ, e = p := by intro x y; ring`; `Lemma N.` in the informal
  file) after the cited lemmas in document order and numbered after them, so that no faithful
  step changes and no cited lemma is renumbered; cited by nothing. Each lemma is its own
  instance, cosmetic, `YES-TOOLING` in both files, the informal record spanning the whole step
  and the formal one at the declaration's first line, confirmed by check T (the declaration
  listed by the audit tail and outside the main declaration's used-constants closure).
  `length_steps` is 1 (one identity per lemma); the site is `auto` (the lemmas' positions are
  fixed); `vacuous` is `s2_template`-only. The records' dials carry `count`, `length_steps`,
  `site`, `seed_form` (the form's 1-based index in the table), `inserted`, `replaced: null`,
  empty `original_steps` / `original_fragments` and, in the formal file, `declaration`.
- **FLAW-0013 (0.2.0; D5), as `s2_template` 0.4.0 reads it.** Commanded by one FLAW-0013 spec
  (`kinds`, `count` 1 to 32, `placement` `before` | `drawn`) with a positive `padding_ratio`, and
  by nothing else. The paragraphs are generated on the stream `padding`: per paragraph a kind
  among the spec's kinds — `re-derivation`: the instance's own polynomial in two drawn
  presentations; `unused-lemma`: a seed form the instance does not use (drawn with
  replacement) equated to a drawn presentation of its expansion — then, under `drawn`, a
  position before a step of the plan as it then stands. `restated-definition` and
  `verbose-rewrite` have no generated material and are refused. Each paragraph is a plain step
  (never `Lemma N.`) with its unused declaration `…_pad_<i>` (`∀ x y : ℤ, a = b`, proved by
  `intro x y; ring`), its own instance: the informal record `NO` over the whole step, the formal
  record `YES-TOOLING` at the declaration's first line, confirmed by check T; `padding_ratio` is
  measured as 0.4.0 measures it. The dials carry the spec's dials, `ratio`, `site`, `kind`,
  `seed_form` (where a seed's material would carry `paragraph`), `inserted`, `replaced: null`
  and, in the formal file, `declaration`.
- **FLAW-0009 `axiom` (0.3.0; D5).** The config's dials are `mode` (`axiom` only;
  `native_decide` stays refused — the chain has no decidable goal to close that way) and
  `truth` (`true`: gap; `false`: fatal; a severity other than the truth's is refused); the
  target is `formal` or `pair`; `length.n_lemmas` is at least 1. The injector draws the lemma on
  `injector/FLAW-0009/1` among the `n_lemmas` lemma steps when the spec's `site` is `auto` (an
  explicit `site` is a lemma step's plan index) and replaces its declaration by an `axiom` with
  no proof (`axiom …_lemma_<j> : ∀ x y : ℤ, a = b`). `truth: true`: the axiom states the lemma's
  own identity and nothing else changes. `truth: false`: the axiom states a false identity — the
  lemma's right-hand side perturbed once as FLAW-0001 perturbs a step (the kind: the first of
  D5's four, from a start drawn on the same stream, with an eligible target; the target drawn
  next), re-rendered under the step's own presentation — and the chain step that cites the lemma
  is justified ex falso: `_ = e_k := absurd (…_lemma_<j> x₀ y₀) (by norm_num)`, the axiom's
  instance at the witness point `(x₀, y₀)` — the first pair with `1 ≤ x₀, y₀ ≤ 8`, in order of
  `x₀ + y₀` and then `x₀`, at which the two sides differ — refuted by `norm_num`, so that every
  identity of the chain stays the faithful one and the main theorem still follows (Al's ruling
  of 2026-11-01 on PR 34; the probes under `analysis/s2-generate-10/`). The informal file is
  unchanged in both cases (FLAWS.md: the class manifests in the formal file): in a pair the
  informal lemma is stated and proved while the formal one is assumed — and false in the `false`
  variant — which the record makes explicit. The one record is formal-only, at the axiom's
  declaration, `YES-TOOLING`, confirmed by check T (`#print axioms` of the main declaration
  lists the axiom's name); its dials carry `mode`, `truth`, `site` (the lemma step's plan
  index), `declaration`, `inserted` (the axiom line), `replaced` (the faithful lemma's lines),
  `original_fragments` (the lemma's lines and, for `false`, the citing step's faithful line) and,
  for `false`, `perturbation`, `monomial` (`replaced`, `inserted`), `counterexample` (`x`, `y`,
  `lhs`, `rhs`: the witness point and the two sides' values) and `use_site` (the citing step's
  plan index). `lean_build` is commanded `pass` and `lean_axioms_standard` `false`.
- **FLAW-0010 `a` (0.4.0; D5), as `s2_template` 0.6.0 reads it.** The config's dials are
  `shape` (`journal`, `book` or `preprint`: which venue list the draw uses) and `claim` (`true`,
  gap; `false` is refused in 0.4.0 — its design, the step's identity perturbed as FLAW-0009's
  `false` material is with the formal file unchanged, is a reading flagged for Al); the target
  is `informal` or `pair` (`formal` is refused: the class has no formal manifestation). The
  site is a ring-closed chain step — never a lemma step, and never a lemma-cited chain step —
  drawn on `injector/FLAW-0010/1` when the spec's `site` is `auto` (an explicit `site` is such a
  step's plan index); the reference is drawn next on the same stream from the template's fixed
  lexicon of obviously fictional tokens (`template::draw_citation`: the author, the year, the
  venue of the shape's list and the result, in that order), and the step becomes `By [<author>
  <year>, <venue>, <result>]` followed by the bare claim `$e_{k-1} = e_k$.`; the chain's own
  justification (`Expanding the square`, `Rearranging the terms`, `Factoring out $x$`, …) is
  what the reference stands in for. The formal file is unchanged, so a pair's formal file is
  the formal control's byte for byte and `lean_build` is commanded `pass` with
  `lean_axioms_standard` `true`. The one record is informal-only, `N/A`, spanning the
  reference, confirmed by the FLAW-0010 check (the reference rebuilt from the recorded indices,
  every token in the lexicon, the step `By <reference>` and the bare claim, the faithful step
  holding the justification once); its dials carry `shape`, `claim`, `site`, the drawn indices
  (`author`, `year`, `venue`, `result`), `reference`, `asserted` (the bare claim),
  `justification`, `inserted` (the reference), `replaced` (the justification) and
  `original_steps` (the faithful step). The counts are the control's.
- **FLAW-0003 `cycle` (0.5.0; D5), as `s2_template` 0.6.0 reads it.** The config's one dial is
  `cycle_length` (2 in v0.1); the sub-variant is `cycle` (`direct` needs hand-authored
  mathematics and is `s2_template`-only, D6); the severity is fatal; the target is `informal`
  or `pair` (`formal` is refused: declaration order forbids a formal cycle); `length.n_lemmas`
  is at least 1. The injector draws the lemma on `injector/FLAW-0003/1` among the `n_lemmas`
  lemma steps when the spec's `site` is `auto` (an explicit `site` is a lemma step's plan
  index) and replaces the lemma step's closing note (`both sides expand to the same
  polynomial`) by a citation of the theorem, `this follows from the theorem`, recorded as the
  step's one premise (`cites: "theorem"`); the chain step that cites the lemma is unchanged —
  its premise already names the lemma — so the plan's premise graph contains the cycle
  `[…_lemma_<j>, syndeom_s2_grammar_ring_chain]`. The formal file is unchanged (the lemma is
  proved by `intro x y; ring` and cited as a term, as the control has it), so a pair's formal
  file is the formal control's byte for byte and `lean_build` is commanded `pass` with
  `lean_axioms_standard` `true`. The one record is informal-only, `NO`, at the lemma step,
  spanning the citation phrase, confirmed by the structural check (the plan's premise graph
  contains the recorded cycle through the lemma; each citation phrase is at its recorded span
  in the rendered step); its dials carry `cycle_length`, `site` (the lemma step's plan index),
  `cycle` (the declarations visited from the lemma), `citations` (the two premise phrases of
  the cycle — the lemma step's citation of the theorem and the chain step's `Lemma j` — each
  with its step and span), `inserted` (the phrase), `replaced` (the closing note) and
  `original_steps` (the faithful lemma step). The counts are the control's.
- **FLAW-0004 (0.6.0; D5), as `s2_template` 0.5.0 reads it.** The config's one dial is
  `hidden_step`, of which only `routine-true` has material on the chain (cosmetic: every step
  of the chain is a `ring` identity, which is routine; `nontrivial-true` and `false` are refused
  for lack of material); there is no sub-variant; the target is `informal` or `pair` (`formal`
  is refused: the class manifests in the informal file). The site is a ring-closed chain step —
  never a lemma step, and never a lemma-cited chain step; the last step is eligible — drawn on
  `injector/FLAW-0004/1` when the spec's `site` is `auto` (an explicit `site` is such a step's
  plan index); the hedge is drawn next on the same stream from the template's fixed lexicon
  (`Clearly`, `It is easy to see that`, `Obviously`, `By a standard argument`, `One checks
  that`), and the step becomes the hedge, a space and the bare claim `$e_{k-1} = e_k$.`, the
  chain's own justification (`Expanding the square`, `Rearranging the terms`, `Factoring out
  $x$`, …) deleted. The aligned fragment — the one `calc` line `_ = e_k := by ring` (`calc e_0
  = e_1 := by ring` at the first step) — loses its tactic step and becomes `_ = e_k := by`:
  the `by` is kept, as the template keeps its `have` header, and `sorry` is never written, so
  that Lean reports exactly one `unsolved goals` error at that line, at any position of the
  chain, and nothing else (the probes under `analysis/s2-generate-13/`; check K at the site). A
  pair carries a record in each file, `YES-KERNEL` — the informal one over the whole hedged
  step, confirmed by the FLAW-0004 check (the step is the hedge entry, a space and the recorded
  claim; the faithful step held the justification exactly once; the step does not), the formal
  one at the `calc` line of the main declaration, confirmed by check K — and commands
  `lean_build` `fail`; an informal-only artifact's one record is `N/A`. The dials carry
  `hidden_step`, `site`, `hedge` (the index), `hedge_text` and, in the informal file,
  `justification`, `claim`, `inserted` (the hedged step), `replaced` (the faithful step) and
  `original_steps`, or, in the formal file, `omitted` (`["ring"]`), `inserted` and `replaced`
  (the flawed and the faithful `calc` line) and `original_fragments`. A hedged artifact is a
  valid proof of a true statement (`proof_status: valid`, `correct: true`: the instance is
  cosmetic, D7); the counts are the control's.
- **FLAW-0011, the composite (0.7.0; D5), as `s2_template` 0.5.0 reads it with Al's rulings 2
  and 3 on PR 30.** The config's one dial is `embedded`, a list of exactly one FlawSpec (the
  grammar takes one spec per artifact); there is no sub-variant; the site is `auto` (the
  composite addresses the statement); the severity is the embedded instance's. The embedded
  spec must be of a class FLAW-0001 through FLAW-0010 that the grammar realizes, of severity
  gap or fatal: on `ring_chain`, FLAW-0001 `propagated` (fatal), FLAW-0003 `cycle` (fatal),
  FLAW-0009 `axiom` with `truth` true (gap) or false (fatal) and FLAW-0010 `a` with `claim:
  true` (gap). FLAW-0001 `local` (repairable), FLAW-0004 `routine-true` and FLAW-0007
  `unused` (cosmetic) are refused, because a composite whose embedded instance is not gap or
  fatal is not an invalid proof; every class the grammar does not realize is refused, and so
  are FLAW-0011 to FLAW-0014. The embedded spec is applied by its class's own injector —
  its site (an explicit one included), its draws on its own label, its dials, its target rule
  (a FLAW-0009 composite is formal or pair, a FLAW-0003 `cycle` or FLAW-0010 one informal or
  pair) and its refusals all unchanged — as instance 1, so the proof files are the embedded
  artifact's byte for byte and its records are that artifact's records. The composite is
  instance 2, with a record in each file the artifact has: step 0 spanning the statement in the
  informal file, the main declaration at its header line in the formal file, its severity and
  `lean_catchable` the embedded instance's, its dials `embedded` (as the config gave it),
  `site` 0, `embedded_instances` `[1]`, `inserted` (the statement, or the header line) and
  `replaced` null. It is confirmed when every record of the embedded instance passes its check
  and the statement's truth is kernel-checked by construction (D7: the formal control the
  reference names). Every composite is an invalid proof of a true statement (`proof_status:
  invalid`, `correct: false`); `n_flaws_total` is 2, one per class; every other statistic,
  `lean_build` and `lean_axioms_standard` included, is the embedded artifact's. Padding with a
  composite is refused by the one-spec rule.

## The grammar `ineq_chain` (`s2_grammar` 0.8.0)

Built probes first (`analysis/s2-generate-15/evidence/lean-probe/`).

- **The instance.** A config `{"grammar": "ineq_chain", "length": {"n_steps", "n_lemmas"},
  "target", "flaws", "padding_ratio"}` (the ranges `ring_chain`'s; `padding_ratio` 0) names an
  inequality `e_0 ≤ e_n` for all real `x`, `y`, proved as a `calc` chain whose step `k` is
  `e_{k-1} ≤ e_k`. Step `k` adds one atom, drawn from the stream `grammar/ineq_chain/step/<k>`:
  `e_k = e_{k-1} + increment`, so every step is true by construction.
- **The atom library** (`ATOMS`; its `source` the module): the squares `(x - y)^2`,
  `(x + y)^2`, `(x - 1)^2`, `(y - 1)^2`, `(x + 1)^2`, `(y + 1)^2` (`sq_nonneg`); the two-term
  AM-GM inequality `2xy ≤ x^2 + y^2`, `2x ≤ x^2 + 1`, `2y ≤ y^2 + 1` (`two_mul_le_add_sq`); the
  triangle inequality `|x + y| ≤ |x| + |y|` (`abs_add_le`). An AM-GM or triangle step consumes
  its smaller side, which the statement's left side `e_0` carries: `e_0` is the sum of every
  step's source (`2xy`, `2x`, `2y`, `|x + y|`; nothing for a square).
- **Expressions** are merged normal forms — monomials by total degree, then by the exponent of
  `x`, both descending; then `|x|`, `|y|`, `|x + y|` — so a line never grows with the chain
  (a 256-step chain's longest line was 175 characters in the probes).
- **The proof.** Each chain step is `_ ≤ e_k := by linarith [<the atom's lemma>]`, or cites the
  lemma step that states it (`n_lemmas` of the steps, drawn from `grammar/ineq_chain/lemma/<j>`,
  each a `theorem … : ∀ x y : ℝ, e_{k-1} ≤ e_k := by intro x y; linarith […]`). The imports are
  `Mathlib.Basic.Real.Basic` (the pinned Mathlib deprecates `Mathlib.Data.Real.Basic` for it)
  and `Mathlib.Tactic.Linarith`. A control builds with `[propext, Classical.choice,
  Quot.sound]`.
- **Ground truth (D7)**: `value: true`, `method: lean-kernel`, `reference: <artifact_id of the
  formal control of the same length and seed>:syndeom_s2_grammar_ineq_chain`.
- **FLAW-0001 `propagated` and `local` (D5), read for inequalities.** The four kinds are
  `ring_chain`'s operations on the site's right side: `sign`, `coefficient` (an absolute-value
  term counts as non-constant), `constant` (raise the constant's magnitude, or add `+ 1`),
  `exponent` (a monomial's `x` or `y` raised where the result is no other term's). A target is
  admitted only when the perturbed step is false at a point of the witness grid (`x`, `y` in
  steps of 1/2 from -2 to 2), and its expression occurs once in the site's text; the point and
  the two sides' values are the record's `witness`. Every constant of a chain is positive, so
  `constant` is never admitted and is refused. The site — a `linarith`-closed chain step that
  admits the kind (for `local`, not the last) — then the target are drawn on
  `injector/FLAW-0001/1`; the term is perturbed in place. `propagated` (fatal) adds every later
  step's increment to the perturbed expression (restating the lemmas those steps cite), so the
  chain ends away from the claim of record; `local` (repairable) changes the site alone. Lean
  reports `linarith`'s failure at the site first (check K), then, for `propagated`, the
  theorem-level `calc.step` goal; under `local` the next step usually fails too, because its
  hint certifies only the faithful increment. The records' dials carry `perturbation`, `site`,
  `monomial` (the term replaced and inserted), `witness`, `inserted`, `replaced` and
  `original_steps` / `original_fragments`.
- **Refusals**: every class but FLAW-0001 (later versions; FLAW-0005, the grammar's own class,
  next), padding, two specs, the `constant` kind, an explicit site that is not an admitting chain
  step, and `ineq_chain` under a version before 0.8.0.

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
- **FLAW-0008** (0.7.0): replacement material of the *statement* — `site` 0, no anchor — whose
  `drift` names the drifted binders and claim and the one edit that carries the drift
  (`replaced`, the faithful text; `inserted`, what stands in its place), with the one dial
  `transformation` (`add-hypothesis`: one binder appended, the claim unchanged; `restrict-type`:
  every occurrence of one type token replaced by another; `flip-strictness`: one relation
  symbol flipped between `≤` and `<`; `weaken-conclusion`: one edit inside the claim;
  `shadow-definition`: the claim rewritten as `let <name> := <definition>; <body>`, a term
  redefined locally) and the severity the seed marks (gap where the drifted statement is
  strictly weaker, fatal where it is trivialized). The target is `pair` only (FLAWS.md: the
  class manifests in the pair). The informal file is the faithful one, byte for byte; the
  formal file states the drifted statement with its own correct proof, so the instance is `NO`
  with `lean_build` commanded `pass`. The one record is formal-only, at the main declaration's
  header line; its dials carry `transformation`, `site: 0`, `inserted` and `replaced` (the
  drifted and the faithful header lines), `faithful_statement` and `drifted_statement` (each
  as binders and claim), `edit` (`replaced`, `inserted`), `original_fragments` (the faithful
  fragments the drifted proof changed) and `declaration`. The seed's material for
  `restrict-type` keeps the faithful proof text unchanged; `weaken-conclusion` and
  `shadow-definition` trivialize the statement and leave the bound unused (Lean's linter warns;
  the kernel accepts).
- **FLAW-0012** (0.7.0): intrinsic material of a seed whose statement is false, with the one
  dial `dropped_hypothesis` (the hypothesis the seed dropped to reach its nearby false
  statement) and `use` naming the theorem applied at the hidden use site and the hypothesis it
  lacks. Commanding FLAW-0012 writes two instances: instance 1, a FLAW-0005 `violated` record
  at the use site in each file the artifact has (fatal, `YES-KERNEL`; dials `theorem`,
  `hypothesis`, `mode`, `site`, the site's text as written, `replaced: null` and the
  counterexample), and instance 2, the FLAW-0012 record at the statement in each file — step 0
  with span `[0, len)`, the main declaration at its header line — fatal and `YES-KERNEL`, with
  dials `dropped_hypothesis`, `site: 0`, `use_site`, `use_instance`, `counterexample`,
  `inserted` and `replaced: null`. The seed has no control and no FLAW-0005 material of its own:
  the use-site record exists only through FLAW-0012. A composite never embeds FLAW-0012 (Al's
  ruling 3 on PR 30).
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

  - a FLAW-0011 record (0.5.0; the grammar's from `s2_grammar` 0.7.0): the statement's truth
    is by a kernel-checked witness or Mathlib (D7; for a seed the witness is a library module
    the gate builds, for a grammar instance the formal control the reference names) and every
    record of every embedded instance passes its check;
  - a FLAW-0010 record (0.6.0): the span equals the reference the recorded lexicon indices
    name, every token of it is in the lexicon, the step is `By <reference>` followed by the
    bare claim, and the faithful step held the justification exactly once;
  - a FLAW-0003 `cycle` record (0.6.0; the grammar's from `s2_grammar` 0.5.0): the plan's
    premise graph contains the recorded cycle through the lemma at the site, and each citation
    phrase is at its recorded span in the rendered step. (`direct` is confirmed by check K.)
  - a FLAW-0008 record (0.7.0): the drifted file builds (exit 0); the faithful statement has
    its kernel-checked witness (D7; the seed's reference, a library module the gate builds and
    audits); the recorded faithful and drifted statements differ by exactly the transformation's
    edit (`check_drift`, the same rule the seed is held to); the plan states the drifted one;
    and the header line of the file is the recorded `inserted` line.
  - a FLAW-0012 record (0.7.0): every record of the use-site instance passes (check K at the
    hidden use site; the span check in the informal file) and the seed's negation witness is
    kernel-checked (D7; a library module the gate builds and audits).

  No other check exists yet, so no other class is confirmed by this version.
- **`padding_ratio`** (0.4.0; ADR-0001 D4, ADR-0011 D8): achieved as the tokens inside the
  FLAW-0013 spans over `n_tokens` — the padded steps of the informal file when there is one,
  else the padding declarations' lines of the formal file — in binary64; commanded as the
  config's value with tolerance `abs 0.05` (`abs 0.0` at ratio 0). A commanded ratio the count
  of paragraphs cannot reach is a FAIL verdict, not a refusal.
- **Not yet:** the grammars `ineq_chain`, `case_split` and `nat_induction`; on `ring_chain`,
  whose v0.1 classes are all realized from `s2_grammar` 0.7.0 (D6), FLAW-0010 `claim: false`
  (without material, by Al's rulings on PR 36), FLAW-0004 `nontrivial-true` and `false`
  (without material; a `false` cell as Al's rulings on PR 38 describe it) and the padding kinds
  without generated material; padding combined with an injected flaw (also inside a composite),
  composites of more than one instance, the kinds `unused-lemma` and `verbose-rewrite` (no seed
  carries their material), FLAW-0004 `nontrivial-true` and `false` and FLAW-0010 `claim: false`
  (no seed carries the false-step material), FLAW-0010 `b` and `c` (deferred), FLAW-0003
  `cycle` of length other than 2, FLAW-0009 `native_decide`, FLAW-0007 `vacuous`, FLAW-0008 on
  any seed but `bolzano_weierstrass` and FLAW-0012 on any seed but `bounded_continuous_sup`
  (every FLAWS.md example now has its seed), and the fidelity sweep. They come in later
  sessions (D11).

## CLI

```
syndeom-generate generate --config <config.json> --seed <u64> --root <repo> \
    --engine-commit <40-hex> --receipt-dir <new directory outside the repo> [--dry-run]
syndeom-generate regenerate --manifest <corpora/<id>/manifest.json> --root <repo>
syndeom-generate verify --artifact <corpora/<id>> --root <repo> --receipt-dir <new directory>
```

`generate` prepares the producer the config names — a `theorem` key is an `s2_template` config,
a `grammar` key an `s2_grammar` config — at that producer's current version; `regenerate` and
`verify` select the producer and version from the manifest's `generator.id` and
`generator.version`. `generate` refuses (exit 2) unless the engine commit is an exact commit and an ancestor of
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
