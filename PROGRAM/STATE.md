# STATE

- **Updated:** 2026-10-29 UTC (2026-10-28 America/New_York). This was Session 47:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone, in a
    fresh conversation;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1–3). The matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing
    HANDOFF's lines in HANDOFF Session 47, Entry 2;
  - **not an examination**: Gate S2 has nothing to examine. The session certifies nothing. Two
    ORIENT order deviations are recorded (HANDOFF Session 47, Entries 2 and 3): after disclosing
    and before ADR-0008 step 5, §4.1's local Git commands, `git branch -vv`, `git tag -l`, the
    hashing of the cited HANDOFF lines and a listing of HANDOFF's headings ran in the same batch
    that read ADR-0008; and the ORIENT gate was launched alongside the reading of STATE.

  **`s2_grammar` 0.1.0 is built** (STATE's NEXT as it stood: S2 ordering step 4, continued — the
  first grammar) on branch `claude/s2-grammar-0.1.0`, in three commits, for Al's review. `main`
  is unchanged until Al merges. The family's second producer exists: the grammar `ring_chain`
  with its control and FLAW-0001, and the producer dispatch.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `a2370a01a31e727b40253593d3e694f10031ff51`, tree
    `d668ecf25c1ecde98606a9596a6003d0a3590d6b`, with the disclosure append the only change.
    - That is **PR 32's merge** (by Al, 2026-10-29T03:48:52Z; parents `89cdcf4` and `0db21f4`),
      so `6dba3615…` and `624c22f6…` are real commits on `main`. No later ruling on PR 32 exists
      (no review, no comment but the executor's own; no open pull request or issue).
    - The receipt is `orient-preflight.json` under
      `C:\Users\LEET\Codex\Temp\syndeom-orient-20261029T0355Z\`.
    - The ORIENT gate was GREEN at `a2370a0` (2026-10-29T03:56:59Z–03:57:52Z: 8,938 Lean jobs,
      fourteen axiom lines within the standard three, fmt, clippy and every test, LF, the
      registers and the 65-row freeze register).
  - **Commits:**
    - `25ad34fe45c1eea8527b3aab47829b31c8685931` the 0.1.0 producer: `src/grammar.rs` — the
      atom library (fourteen seed forms: products and powers of small binomials and trinomials
      in `x` and `y`; the presentations: a drawn permutation of the monomials, a style per term,
      a shape — expanded, one variable factored out, or regrouped), the derivation
      (`grammar/plan`, `grammar/step/<k>`, `grammar/lemma/<j>`), the plan (lemma steps first,
      `∀ x y : ℤ, …` proved by `intro x y; ring` and cited as terms; the `calc` chain closed by
      `ring`), ground truth (D7: `lean-kernel`, the reference the formal control's artifact id
      and the main declaration; the formal control names itself), the config (D3: `grammar`,
      `length` {`n_steps` 1–256, `n_lemmas` 0–32, at most `n_steps`}, `target`, `flaws`,
      `padding_ratio` 0) and FLAW-0001 (D5: a ring-closed chain step and one perturbed monomial
      — `sign`, `coefficient`, `constant`, `exponent` — drawn on `injector/FLAW-0001/1`, the site
      re-rendered under its own presentation; `propagated` re-derives every later step and
      restates the lemmas cited after the site, the claim of record untouched; `local` changes
      the site alone); `Producer` in `Prepared` and `assemble`; the dispatch (`generate` by the
      config's `theorem`/`grammar` key, `regenerate`/`verify` by the manifest's `generator`);
      `tests/grammar.rs` (eleven tests); `tests/goldens.rs` by producer; the crate README;
    - `29f5f75879521e78f2486e5f8e1f4557b0f6f779` the 10 goldens of `s2_grammar` 0.1.0 — the
      control at (4, 0) and (24, 2) (informal, formal each), FLAW-0001 `propagated` and `local`
      with `sign` at (4, 0) (informal, formal, pair each) — produced at `25ad34fe…` by two
      byte-identical runs, and the goldens README (79 cases);
    - the record commit: `analysis/s2-generate-8/`, ADR-0011 *Amendments*, this file and HANDOFF.
  - **Measured at implementation** (`analysis/s2-generate-8/README.md`):
    - six hand-written Lean probes compiled one process at a time before the module was
      written: a six-step control and a 32-step control with two cited lemmas build with the
      standard axioms; in each of four perturbed chains (sign and exponent propagated,
      coefficient and constant local) the first error is at the perturbed step's own line, the
      propagated ones followed only by the theorem-level `calc.step` goal;
    - the 69 earlier goldens regenerate byte for byte under the producers and versions they
      name; the 10 of `s2_grammar` 0.1.0 pass fidelity — the controls build with `[propext,
      Quot.sound]`, the FLAW-0001 formal and pair cases fail at line 4, the site's `calc` line
      (check K), every instance confirmed; at seed 7 the seed form is `(x - 2 * y) ^ 2` and the
      flawed cases flip the sign of `4 * y ^ 2` at step 1;
    - `cargo audit` clean (advisory-db `40f5be6f…`; `Cargo.lock` unchanged); the security review
      records FINDING-018 (informational, backlog).
  - **Readings flagged for Al** (packet readings 85–99): the length dials count the chain and
    the lemmas separately, the commanded `n_steps` their sum (85); a lemma states one chain step
    and is cited as a term (86); lemmas first, numbered by the steps they state (87); the
    presentations are the rewrites, repeats allowed, the LaTeX conventions (88); the seed form
    table (89); **the truth reference names the formal control for every target (90)**; the one
    import (91); **FLAW-0001's site a ring-closed chain step, never a cited one, `local` never
    the last (92)**; the perturbation on the site's ordered terms, `constant` added when absent,
    `exponent` collisions excluded (93); **`propagated` re-derives and restates the lemmas cited
    after the site; `local` leaves a second false step after the first error (94)**; the
    `monomial` dial and a restated lemma's `original_fragments` entry (95); severity fixed by the
    sub-variant (96); the grammar's remaining v0.1 classes as later versions (97); the dispatch
    by the config's key (98); the goldens' lengths (99).
  - **Not yet built:** the grammars `ineq_chain`, `case_split` and `nat_induction`; on
    `ring_chain` every class but FLAW-0001 and padding material; the `s2_template` sub-variants
    without material (FLAW-0004 `nontrivial-true` and `false`, FLAW-0010 `claim: false`,
    FLAW-0007 `vacuous`, FLAW-0009 `native_decide`, the padding kinds `unused-lemma` and
    `verbose-rewrite`, padding combined with an injected flaw); the sweep (ADR-0011 D11).
  - **Unchanged:** registers' statuses, the manifests under `groundtruth/`, frozen packets, the
    Gate S1 table, the constitution, the licence, the tag, `corpora/` (no artifact written), every
    `s2_template` seed, dial, check and golden. **NEXT** waits on this pull request's merge.
- **Previous update (Session 46):** 2026-10-29 UTC (2026-10-28 America/New_York). This was Session 46:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone, in a
    fresh conversation;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1–3). The matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing
    HANDOFF's lines in HANDOFF Session 46, Entry 2;
  - **not an examination**: Gate S2 has nothing to examine. The session certifies nothing.

  **`s2_template` 0.7.0 is built** (STATE's NEXT as it stood: S2 ordering step 4, continued) on
  branch `claude/s2-template-0.7.0`, in three commits and, after Al's rulings on PR 32, a fourth
  recording them, for Al's review. `main` is unchanged until Al merges. Every FLAWS.md 0.1.0
  example now has its seed, and every class is realizable by `s2_template`.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `89cdcf45baaf13834a8f178e0614b8b9b8420937`, tree
    `246ae476fc83205efd852b1e1ac54a3be1cabcf1`, with the disclosure append the only change.
    - That is **PR 31's merge** (by Al, 2026-10-29T01:33:32Z; parents `854a4b4` and `bd63c4d`),
      so `ef41a10c…` and `45bdbe3c…` are real commits on `main`. No later ruling on PR 31 exists
      (no review, no comment but the executor's own; no open pull request or issue).
    - The receipt is `orient-1.json` under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261029T0140Z\`.
    - The ORIENT gate was GREEN at `89cdcf4` (2026-10-29T01:41:26Z–01:42:17Z: 8,936 Lean jobs,
      twelve axiom lines within the standard three, fmt, clippy and every test, LF, the registers
      and the 65-row freeze register).
  - **Commits:**
    - `6dba3615a001151116632169bd93b25440878ac0` the 0.7.0 producer: FLAW-0008 (D5: replacement
      material of the statement, `site` 0, whose `drift` names the drifted binders and claim and
      the one edit; the dial `transformation`; `pair` only; the informal file faithful, the
      formal file the drifted statement with its own proof, `NO`, `lean_build` `pass`; the
      record formal-only at the main declaration's header line with both statements and the edit;
      `check_drift`, one textual rule per transformation, on the seed and on the record alike),
      FLAW-0012 (D5: intrinsic material of a false statement with `use` naming the hidden use
      site's theorem and hypothesis; the dial `dropped_hypothesis`; the FLAW-0005 `violated`
      instance at the site, check K, and the FLAW-0012 instance at the statement, confirmed with
      it and the negation witness; two instances), the seed `bolzano_weierstrass` (FLAWS.md's
      FLAW-0008 example, three aligned steps on the compact interval, five drift materials:
      `add-hypothesis` gap, `restrict-type` gap with the proof text unchanged, `flip-strictness`
      gap, `weaken-conclusion` fatal, `shadow-definition` fatal) with its witness module, the
      seed `bounded_continuous_sup` (FLAWS.md's FLAW-0012 example, false by `arctan`, four
      aligned steps, the hidden use site step 3) with its negation witness (`Audit.lean` now
      fourteen lines), `plan::header_line`, the tests and the crate README;
    - `624c22f68a39178dabad9a9c01b1e647059e0a24` the 10 goldens of 0.7.0 — on `bolzano_weierstrass` the control (informal,
      formal) and FLAW-0008 for each transformation (pair); on `bounded_continuous_sup`
      FLAW-0012 (informal, formal, pair) — produced at `6dba3615…` by two byte-identical runs
      (a run between them refused by the provenance check after a mid-run edit of an input-set
      file, recorded and not used), and the goldens README (69 cases);
    - the record commit: `analysis/s2-generate-7/`, ADR-0011 *Amendments*, this file and HANDOFF.
  - **Measured at implementation** (`analysis/s2-generate-7/README.md`):
    - twelve Lean probes compiled one process at a time before the seed records were written:
      the faithful statement, the five drifted statements and both witnesses build with the
      standard three axioms (ℝ → ℤ with the faithful proof text unchanged); the convincing
      argument fails with one `Application type mismatch` at line 11, the `have hxb` line, the
      first fragment line of step 3, and every other line elaborates (the `sorry` control); the
      trivialized drifts draw Lean's unused-variable warning on `hb`;
    - the library builds with the two modules (8,938 jobs) and the audit prints fourteen lines
      within the standard three;
    - the 59 goldens of 0.1.0–0.6.0 regenerate byte for byte under their versions; the 10 of
      0.7.0 pass fidelity, every formal file byte-identical to its probe, the five drifted pairs
      building with the standard axioms and their `proof.md` the control's, the FLAW-0012 formal
      and pair cases failing at line 11 (check K), every instance confirmed;
    - `cargo audit` clean (advisory-db `40f5be6f…`; `Cargo.lock` unchanged); the security review
      records FINDING-017 (informational, backlog).
  - **Readings flagged for Al** (packet readings 66–84): FLAW-0008 at the statement, formal-only
    (66); the drift as one edit under one textual rule per transformation (67); **FLAWS.md's
    example not realizable as one drift — its hypothesis as `add-hypothesis`, `weaken-conclusion`
    the executor's (68)**; ℝ → ℤ (69); the hypothesis's bound flipped (70); **`shadow-definition`
    as a `let` inside the claim (71)**; the severities (72); **the linter warning on the
    trivialized drifts, the class staying `NO` (73)**; "both statements build" by the artifact's
    build and the witness reference (74); `original_fragments` only (75); a composite may embed
    FLAW-0008 (76); **FLAW-0012 as two instances (77)**; the use-site record's dials and the
    seed without a control (78); FLAW-0012's confirmation (79); the failure at the `have hxb`
    line (80); the statement form and the `arctan` witness (81); `dropped_hypothesis` free text
    (82); the seed-level imports (83); `YES-KERNEL` kept informal-only (84).
  - **Not yet built:** `s2_grammar`, padding combined with an injected flaw (also inside a
    composite), composites of more than one instance, the kinds `unused-lemma` and
    `verbose-rewrite`, FLAW-0004 `nontrivial-true` and `false`, FLAW-0010 `claim: false`, `b`
    and `c`, FLAW-0009 `native_decide`, FLAW-0007 `vacuous`, FLAW-0008 and FLAW-0012 on any
    other seed, and the sweep (ADR-0011 D11).
  - **Unchanged:** registers' statuses, the manifests under `groundtruth/`, frozen packets, the
    Gate S1 table, the constitution, the licence, the tag, `corpora/` (no artifact written) and
    every 0.1.0, 0.2.0, 0.3.0, 0.4.0, 0.5.0 and 0.6.0 golden. **NEXT** waits on this pull
    request's merge.
  - **Al's rulings of 2026-10-28 on PR 32** (verbatim under *Standing rulings*; ADR-0011
    *Amendments*; HANDOFF Session 46's addendum), recorded on the same branch: readings 66–84
    accepted as read; reading 68 recorded with Al's erratum note against FLAWS.md 0.1.0 (the
    FLAW-0008 example applies two transformations where the recipe says exactly one, and its
    changed conclusion is false without the added hypothesis; under D5 the example realizes as
    the `add-hypothesis` drift with the example's proof route; FLAWS.md's text unchanged);
    reading 73 recorded with Al's note (a linter warning is neither a kernel rejection nor an
    observation this program records; whether an unused-hypothesis lint enters the battery, and
    with it the observation schema, is for the S3 detector ADR). No code, seed or golden
    changed; the version stays 0.7.0.
- **Previous update (Session 45):** 2026-10-29 UTC (2026-10-28 America/New_York). This was Session 45:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone, in a
    fresh conversation;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1–3). The matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing
    HANDOFF's lines in HANDOFF Session 45, Entry 2;
  - **not an examination**: Gate S2 has nothing to examine. The session certifies nothing.

  **`s2_template` 0.6.0 is built** (STATE's NEXT as it stood: S2 ordering step 4, continued) on
  branch `claude/s2-template-0.6.0`, in three commits, for Al's review. `main` is unchanged until
  Al merges.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `854a4b4520583c1343d64fd55fe392f0a1fe8a2f`, tree
    `f2357f53ce78d1acc1d3aef8c872f264914e54bf`, with the disclosure append the only change.
    - That is **PR 30's merge** (by Al, 2026-10-29T00:25:46Z; parents `fbcc6c4` and `dbce6d9`),
      so `1c0c5fcd…` and `5afc00d7…` are real commits on `main`. No later ruling on PR 30 exists
      (no review, no comment but the executor's own; no open pull request or issue).
    - The receipt is `orient-receipt-1.json` under
      `C:\Users\LEET\Codex\Temp\syndeom-s2-template-060-20261029T0029Z\`.
    - The ORIENT gate was GREEN at `854a4b4` (2026-10-29T00:32:24Z–00:33:14Z: 8,935 Lean jobs,
      eleven axiom lines within the standard three, fmt, clippy and every test, LF, the registers
      and the 65-row freeze register).
  - **Commits:**
    - `ef41a10ce16db34c1ebe3ea66a456df97df83be1` the 0.6.0 producer: FLAW-0010 `a` (D5: the
      material's `cited` — the bare claim and the justification the reference stands in for; the
      config's `shape` and `claim`; the reference `[<author> <year>, <venue>, <result>]` drawn
      from the fixed lexicon of obviously fictional tokens on the stream `injector/FLAW-0010/1`
      after the material and prefixed as `By <reference>` to the claim; the record spanning the
      reference; the formal file unchanged, `N/A`, informal-only; the check rebuilding the
      reference from the recorded indices and requiring every token in the lexicon), FLAW-0003
      (D5: `direct`, `form` `restated-goal` | `contrapositive`, Lemma 1 proved by a forward
      reference to the main declaration, check K; `cycle`, `cycle_length` 2, Lemma 1 citing the
      theorem and the key step citing Lemma 1, informal-only, checked on the plan's premise graph
      and the citation phrases' spans; the formal file a faithful proof), the plan's `premises`
      (D2), the seed `bounded_monotone_converges` (FLAWS.md's FLAW-0003 example) with its witness
      module (`Audit.lean` now twelve lines), the `sqrt2_irrational` FLAW-0010 material at step 3
      (the FLAW-0004 material's bare claim reused), the tests and the crate README;
    - `45bdbe3c03d046ce89ac92f723b86d199462d7ba` the 12 goldens of 0.6.0 — FLAW-0010 `journal`
      (informal, pair) and, on the new seed, the control (informal, formal), `direct`
      `restated-goal` and `contrapositive` (informal, formal, pair each) and `cycle` (informal,
      pair) — produced at `ef41a10c…` by two byte-identical runs, and the goldens README (59
      cases);
    - the record commit: `analysis/s2-generate-6/`, ADR-0011 *Amendments*, this file and HANDOFF.
  - **Measured at implementation** (`analysis/s2-generate-6/README.md`):
    - five Lean probes compiled one process at a time before the seed record was written: the
      faithful argument, the cycle's formal file and the witness build with the standard three
      axioms; both `direct` forms fail with one `unknownIdentifier` error at the lemma's proof
      line (5), inside the lemma's declaration (lines 4–5), and no cascade;
    - the 47 goldens of 0.1.0–0.5.0 regenerate byte for byte under their versions; the 12 of
      0.6.0 pass fidelity, every formal file byte-identical to its probe (or, for the FLAW-0010
      pair, to the faithful control), the four `direct` formal and pair cases failing at line 5
      (check K), the control, the cycle pair and the FLAW-0010 pair building with the standard
      axioms, the reference drawn at seed 7 `[Fictitious 19XX, Annals of Imaginary Results,
      Corollary 1.7]`, every instance confirmed;
    - `cargo audit` clean (advisory-db `40f5be6f…`; `Cargo.lock` unchanged); the security review
      records FINDING-016 (informational, backlog).
  - **Readings flagged for Al** (packet readings 52–65): the reference's form and the lexicon
    (52); the draw order (53); `shape` a config dial, not a material property (54); the record
    spanning the reference, `replaced` the justification (55); `claim` reserved for FLAW-0004
    and FLAW-0010's own dial (56); `N/A` for every target, a pair's `lean_build` `pass` (57);
    `claim: false`, `b` and `c` refused (58); `direct`'s site the inserted lemma (59); the
    supremum step dropped (60); **the `cycle`'s formal file a faithful proof with the lemma
    declared, not the control's bytes (61)**; `premises` and the recorded cycle (62); the cycle
    check structural (63); the seed's statement form and witness (64); the `direct` location at
    the declaration's first line (65).
  - **Not yet built:** `s2_grammar`, padding combined with an injected flaw (also inside a
    composite), composites of more than one instance, the kinds `unused-lemma` and
    `verbose-rewrite`, FLAW-0004 `nontrivial-true` and `false`, FLAW-0010 `claim: false`, `b`
    and `c`, FLAW-0009 `native_decide`, FLAW-0007 `vacuous`, FLAW-0012 as a commanded class, the
    seeds for the remaining FLAWS.md examples (FLAW-0008 and FLAW-0012), and the sweep (ADR-0011
    D11).
  - **Unchanged:** registers' statuses, the manifests under `groundtruth/`, frozen packets, the
    Gate S1 table, the constitution, the licence, the tag, `corpora/` (no artifact written) and
    every 0.1.0, 0.2.0, 0.3.0, 0.4.0 and 0.5.0 golden. **NEXT** waits on this pull request's
    merge.
  - **Al's rulings of 2026-10-28 on PR 31** (verbatim under *Standing rulings*; ADR-0011
    *Amendments*; HANDOFF Session 45's addendum), recorded on the same branch: readings 52–65
    accepted as read; reading 52 recorded with Al's note (the references placeholder-shaped by
    D5's design, S3 reading the class's detection rates as rates against obviously fictional
    references; a plausible-shaped lexicon a later producer version that never attaches a real
    name to a fictional work); reading 61 recorded with Al's note (the cycle pair's formal file
    a valid proof by construction, the instance informal-only, the pair invalid under D7's
    either-file rule). No code, seed or golden changed; the version stays 0.6.0.
- **Previous update (Session 44):** 2026-10-28 UTC (2026-10-28 America/New_York). This was Session 44:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone, in a
    fresh conversation;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1–3). The matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing
    HANDOFF's lines in HANDOFF Session 44, Entry 2;
  - **not an examination**: Gate S2 has nothing to examine. The session certifies nothing. One
    ORIENT deviation is recorded (HANDOFF Session 44, Entry 2): after disclosing and before
    ADR-0008 step 5, the append's verification printed the previous block's last six lines.

  **`s2_template` 0.5.0 is built** (STATE's NEXT as it stood: S2 ordering step 4, continued) on
  branch `claude/s2-template-0.5.0`, in three commits and, after Al's rulings on PR 30, three
  more, for Al's review. `main` is unchanged until Al merges.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `fbcc6c4e682e6cbaaf14d8e42a28cfcd5d2f6cb0`, tree
    `13d4e586a7974dcebe72353a9bafe8f774044139`, with the disclosure append the only change.
    - That is **PR 29's merge** (by Al, 2026-10-28T23:06:26Z; parents `4cd1f77` and `319635e`),
      so `b54e2f16…` and `cc868a3c…` are real commits on `main`. No later ruling on PR 29 exists
      (no review, no comment but the executor's own; no open pull request or issue).
    - The receipt is `live-orient-1.json` under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T2316Z\`.
    - The ORIENT gate was GREEN at `fbcc6c4` (2026-10-28T23:16:57Z–23:17:47Z: 8,935 Lean jobs,
      eleven axiom lines within the standard three, fmt, clippy and every test, LF, the registers
      and the 65-row freeze register).
  - **Commits:**
    - `7be41123c698a07b9e56cab2aa2a1e9d257944b8` the 0.5.0 producer: FLAW-0004 (D5: the
      material's `hidden` — the bare claim, the justification, the omitted tactic lines; the hedge
      drawn from the fixed five-entry lexicon on the stream `injector/FLAW-0004/1` after the
      material and prefixed to the claim; the proof block of the aligned `have` omitted, never
      `sorry`, so the kernel reports an unsolved goal at the site, check K; `N/A` for an
      informal-only target; the D5 informal check), FLAW-0011 as a commanded composite (D5:
      `embedded`, one FlawSpec, on a true statement; the composite's record at step 0 and at the
      main declaration's header line, severity and `lean_catchable` inherited, confirmed by the
      truth witness and every embedded check; counts per instance), the `sqrt2_irrational`
      seed's FLAW-0004 `routine-true` material at step 3 (FLAWS.md's FLAW-0011 example), the
      FINDING-014 rule in `check_seed`, the tests and the crate README;
    - `4edc3f73f43e32d615cd7ba01093215f133c3e08` the 4 goldens of 0.5.0 — FLAW-0004 (informal,
      pair) and the composite (informal, pair) — produced at `7be41123…` by two byte-identical
      runs, and the goldens README (47 cases);
    - the record commit: `analysis/s2-generate-5/`, ADR-0011 *Amendments*, this file and HANDOFF.
  - **Measured at implementation** (`analysis/s2-generate-5/README.md`):
    - two Lean probes compiled one process at a time before the seed material was written: the
      hedged file with the whole proof of `hp` omitted fails with one `unsolved goals` error at
      line 7 (the `by` of the `have`), no cascade; the alternative with only the odd case omitted
      fails at the same line and is recorded, not built;
    - the 43 goldens of 0.1.0–0.4.0 regenerate byte for byte under their versions; the 4 of 0.5.0
      pass fidelity, both pair files byte-identical to the probe and failing at line 7 (check K),
      hedge 1 (`It is easy to see that`) drawn at seed 7, every instance confirmed; the artifacts
      are `valid` and `correct: true` (reading 43);
    - `cargo audit` clean (advisory-db `40f5be6f…`; `Cargo.lock` unchanged); the security review
      closes FINDING-014 and records FINDING-015 (informational, backlog).
  - **Readings flagged for Al** (packet readings 39–51): the hedge drawn, not commanded; the
    whole proof block omitted; the bare claim and the justification recorded separately; `N/A`
    for an informal-only target; a composite of a cosmetic instance a valid, correct proof under
    D7 against FLAW-0011's recipe (reading 43); the embedded instance numbered first; the truth
    witness taken by construction; one embedded instance per composite; the composite's
    locations; `lean_build` commanded `fail`; FLAW-0004 in both files of a pair; `embedded`
    general beyond the example; `nontrivial-true` and `false` without material.
  - **Not yet built:** `s2_grammar`, padding combined with an injected flaw (also inside a
    composite), composites of more than one instance, the kinds `unused-lemma` and
    `verbose-rewrite`, FLAW-0004 `nontrivial-true` and `false`, FLAW-0009 `native_decide`,
    FLAW-0007 `vacuous`, FLAW-0012 as a commanded class, the seeds for the remaining FLAWS.md
    examples (FLAW-0003, 0008, 0010 and 0012), and the sweep (ADR-0011 D11).
  - **Unchanged:** registers' statuses, the manifests under `groundtruth/`, frozen packets, the
    Gate S1 table, the constitution, the licence, the tag, `corpora/` (no artifact written) and
    every 0.1.0, 0.2.0, 0.3.0 and 0.4.0 golden. **NEXT** waits on this pull request's merge.
  - **Al's rulings of 2026-10-28 on PR 30** (verbatim under *Standing rulings*; ADR-0011
    *Amendments*; HANDOFF Session 44's addendum), applied on the same branch in three further
    commits: readings 39, 40, 41, 42, 44, 45, 47, 48, 49, 50 and 51 accepted as read; reading 43
    replaced — D7 stands, FLAW-0011 requires an embedded instance of severity gap or fatal, a
    composite whose embedded instances are all cosmetic is refused with a test, and FLAWS.md's
    FLAW-0011 example realizes FLAW-0004 `routine-true` (an erratum note against FLAWS.md 0.1.0
    in ADR-0011 *Amendments*; FLAWS.md's text unchanged); reading 46 replaced — a composite
    embeds FLAW-0001 through FLAW-0010 only. The fix commit
    `1c0c5fcdf1ffd375b109f272461a63c49330e007` refuses both (the cosmetic composite goldens
    removed); the goldens commit `5afc00d7809fa34a0c163e08c6d8e353d374bcae` holds the 0.5.0 set regenerated at the fix commit
    from two byte-identical runs — FLAW-0004 (informal, pair; proof bytes, ids and ground truth
    unchanged) and the composite of `continuity_is_not_uniform`'s FLAW-0002 `forall-exists` gap
    (informal, pair; the executor's choice; its proof files the 0.2.0 goldens' bytes, the pair
    failing at line 6) — so that all four name one engine commit; the record commit holds the
    records. The version stays 0.5.0.
- **Previous update (Session 43):** 2026-10-28 UTC (2026-10-28 America/New_York). This was Session 43:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone, in a
    fresh conversation;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1–3). The matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing
    HANDOFF's lines in HANDOFF Session 43, Entry 2;
  - **not an examination**: Gate S2 has nothing to examine. The session certifies nothing.

  **`s2_template` 0.4.0 is built** (STATE's NEXT as it stood: S2 ordering step 4, continued) on
  branch `claude/s2-template-0.4.0`, in four commits, for Al's review. `main` is unchanged until
  Al merges.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `4cd1f7724f5d8ee1097f0afbc6d1ceafb0220f62`, tree
    `f9c8e1d03df23ef34cf1db417c4e43318062ac60`, with the disclosure append the only change.
    - That is **PR 28's merge** (by Al, 2026-10-28T21:12:20Z; parents `7a05e75` and `caa4db2`),
      so `5a313ddf…` and `caa4db2c…` are real commits on `main`. No later ruling on PR 28 exists
      (no review, no comment but the executor's own; no open pull request or issue).
    - The receipt is `orient-1.json` under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T2119Z\`.
    - The ORIENT gate was GREEN at `4cd1f77` (2026-10-28T21:21:03Z–21:22:00Z: 8,934 Lean jobs,
      ten axiom lines within the standard three, fmt, clippy and every test, LF, the registers
      and the freeze register).
  - **Commits:**
    - `b54e2f16aa3c153258bf6ece67798bd270993cbb` the 0.4.0 producer: padding steps in the plan
      (D2), the padding module `pad` (D5 FLAW-0013: dials `kinds`, `count`, `placement`; one
      instance per paragraph; check T for the padding declarations; `padding_ratio` measured as
      the FLAW-0013 span tokens over `n_tokens`), the seed `sqrt2_irrational` with its five
      aligned steps and seven padding paragraphs, its witness module (`Audit.lean` now eleven
      lines), the tests and the crate README;
    - `cc868a3cef8648059df71ed888493765214fe04a` the 8 goldens of 0.4.0 — the control (informal,
      formal), FLAW-0013 `drawn` with one re-derivation and FLAW-0013 `before` with all seven
      paragraphs (informal, formal, pair each) — produced at `b54e2f16…` by two byte-identical
      runs, and the goldens README;
    - `3aa87f7b9b18e596b22f669d43646d19f9f71678` the record commit: `analysis/s2-generate-4/`,
      ADR-0011 *Amendments*, this file and HANDOFF;
    - the rulings commit: ADR-0011 *Amendments*, this file, the packet README and HANDOFF's
      addendum (Al's rulings on PR 29, below).
  - **Measured at implementation** (`analysis/s2-generate-4/README.md`):
    - three Lean probes compiled one process at a time before the seed record was written: the
      faithful argument, the witness and the padded file build with `[propext, Quot.sound]`, and
      the padded file's used-constants listing shows every padding declaration unreachable from
      the main declaration, which uses only its own auxiliaries;
    - the 35 goldens of 0.1.0–0.3.0 regenerate byte for byte under their versions; the 8 of 0.4.0
      build (exit 0), pass fidelity and are confirmed by check T; the control golden is its probe
      without the tail, and the padded goldens' declarations are the padded probe's blocks in
      drawn order; the padded informal files have 6 and 12 steps and `n_lemmas` 0; the achieved
      ratios are 0.2784 / 0.7127 (informal and pair) and 0.3130 / 0.6146 (formal-only), commanded
      to two decimals;
    - `cargo audit` clean (advisory-db `40f5be6f…`; `Cargo.lock` unchanged); the security review
      closes FINDING-008 and records FINDING-014 (informational, backlog).
  - **Readings flagged for Al** (packet readings 29–38): the seed's statement form; the padding
    command (a FLAW-0013 spec with `kinds`, `count`, `placement` plus a positive ratio — `count`
    the generation decision, the ratio measured and checked at `abs 0.05`); `placement`; one
    instance per paragraph; padding steps never lemmas; `padding_ratio` of a formal-only
    artifact; one spec per artifact; the record dials; the goldens' two-decimal ratios; check T
    for FLAW-0013.
  - **Al's rulings of 2026-10-28 on PR 29** (verbatim under *Standing rulings*; ADR-0011
    *Amendments*; HANDOFF Session 43's addendum), recorded on the same branch: readings 29–38
    accepted as read; reading 30 recorded with Al's reading of D4/D8 (the integer `count` is the
    generation decision because D4 forbids a floating-point value there; `padding_ratio` is a
    commanded statistic measured from the emitted file and checked at `abs 0.05`; a ratio the
    count cannot reach is a FAIL verdict; a ratio-targeted mode, if a later version wants one,
    chooses its count by exact integer comparison of token quotients). No code, seed or golden
    changed; the version stays 0.4.0.
  - **Not yet built:** `s2_grammar`, padding combined with an injected flaw, the kinds
    `unused-lemma` and `verbose-rewrite`, FLAW-0009 `native_decide`, FLAW-0007 `vacuous`, the
    seeds for the remaining FLAWS.md examples (FLAW-0003, 0004, 0008, 0010, 0011 and 0012), and
    the sweep (ADR-0011 D11).
  - **Unchanged:** registers' statuses, the manifests under `groundtruth/`, frozen packets, the
    Gate S1 table, the constitution, the licence, the tag, `corpora/` (no artifact written) and
    every 0.1.0, 0.2.0 and 0.3.0 golden. **NEXT** waits on this pull request's merge.
- **Previous update (Session 42):** 2026-10-28 UTC (2026-10-28 America/New_York). This was Session 42:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone, in a
    fresh conversation;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1–3). The matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing
    HANDOFF's lines in HANDOFF Session 42, Entry 2;
  - **not an examination**: Gate S2 has nothing to examine. The session certifies nothing.

  **`s2_template` 0.3.0 is built** (STATE's NEXT as it stood: S2 ordering step 4, continued) on
  branch `claude/s2-template-0.3.0`, in four commits, for Al's review. `main` is unchanged until
  Al merges.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `7a05e75ed5ed872f92eb4453d353616c2f1d1dbf`, tree
    `c95ea6fb1c3fb8733d90539874719a610674c832`, with the disclosure append the only change.
    - That is **PR 27's merge** (by Al, 2026-10-28T19:49:50Z; parents `de3e4a6` and `016f369`),
      so `af28cc9` and `c899b07` are real commits on `main`. No later ruling on PR 27 exists (no
      review, no comment but the executor's own; no open pull request or issue).
    - The receipt is `orient-receipt-1.json` under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1956Z\`.
    - The ORIENT gate was GREEN (*Current build verification*).
  - **Commits:**
    - `59b505a320447f58c811ec394103f64ac9381a93` the 0.3.0 producer: lemma steps (D2), the
      used-constants audit tail and check T (D5, D8) for FLAW-0007 `unused` and FLAW-0009,
      records only where a class manifests, insertion material, the seed `n_lt_two_pow`
      (FLAW-0009's helper and FLAW-0007's unused lemma) with its witness module (`Audit.lean`
      now ten lines), the tests and the crate README;
    - `5a313ddf8de6a870c2f41c9d65d8cb87a1595b2e` FINDING-011: `verify` refuses a manifest whose
      `main_declaration` is not the producer's (the name was interpolated into the compiled
      audit file; present since 0.1.0);
    - `f0173a285a2d3797368827c1c82661019b3ec9f6` the 9 goldens of 0.3.0, produced at
      `5a313ddf…` by two byte-identical runs, and the goldens README;
    - the record commit: `analysis/s2-generate-3/`, this file and HANDOFF.
  - **Measured at implementation** (`analysis/s2-generate-3/README.md`):
    - seven Lean probes compiled one process at a time before the seed record was written: the
      faithful file and the witnesses build with the standard three axioms; both axiom variants
      list the smuggled axiom under `#print axioms`; the unused lemma is absent from the main
      declaration's used-constants, which name Lemma 1 only (the tail also lists Lean's
      auxiliary `…lemma_1._proof_1_1`);
    - the 12 goldens of 0.1.0 and the 14 of 0.2.0 regenerate byte for byte under their versions;
      the 9 of 0.3.0 build (exit 0) and are confirmed by check T, each formal and pair golden
      byte-identical to its probe without the probe's tail;
    - `cargo audit` clean (advisory-db `40f5be6f…`; `Cargo.lock` unchanged); the security review
      found FINDING-011 (High, resolved in the same pull request) and two informational items.
  - **Readings flagged for Al** (packet readings 21–28): lemma steps as headed steps; the audit
    tail over every constant the file defined; records only where a class manifests (a
    FLAW-0009 pair has one record); FLAW-0009 `truth: false` changes the formal lemma's
    statement with the informal file unchanged; insertion material; lemma-site formal records;
    one seed for two classes; FINDING-011's fix keeps the version at 0.3.0.
  - **Al's rulings of 2026-10-28 on PR 28** (verbatim under *Standing rulings*; ADR-0011
    *Amendments*; HANDOFF Session 42's addendum), recorded on the same branch: readings 21–28
    accepted as read; reading 24 recorded with Al's reading of FLAWS.md's FLAW-0009 recipe (the
    same statement for `truth: true`; for `truth: false` a false statement from which the main
    theorem still follows, its negation proved by the witness module; the record carrying the
    replaced lemma and the inserted axiom). No code, seed or golden changed; FLAWS.md unchanged.
  - **Not yet built:** `s2_grammar`, padding (FLAW-0013), FLAW-0009 `native_decide`, FLAW-0007
    `vacuous`, the seeds for the remaining FLAWS.md examples (FLAW-0003, 0004, 0008, 0010,
    0011, 0012 and 0013), and the sweep (ADR-0011 D11).
  - **Unchanged:** registers' statuses, the manifests under `groundtruth/`, frozen packets, the
    Gate S1 table, the constitution, the licence, the tag, `corpora/` (no artifact written) and
    every 0.1.0 and 0.2.0 golden. **NEXT** waits on this pull request's merge.
- **Previous update (Session 41):** 2026-10-28 UTC (2026-10-28 America/New_York). This was Session 41:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim followed by an
    owner note, in a fresh conversation;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1–3). The matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing
    HANDOFF's lines in HANDOFF Session 41, Entry 2;
  - **not an examination**: the owner note carries program state, and Gate S2 has nothing to
    examine. The session certifies nothing.

  **`s2_template` 0.2.0 is built** (STATE's NEXT as it stood: S2 ordering step 4, continued) on
  branch `claude/s2-template-0.2.0`, in four commits, for Al's review. `main` is unchanged until
  Al merges.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `de3e4a62fde75eea2c80c17ac6210955bb868a0f`, tree
    `4117ac9d979c9c837de533e14baa8a562f870c4d`, with the disclosure append the only change.
    - That is **PR 26's merge** (by Al, 2026-10-28T18:10:24Z; parents `f9f9730` and `c4250d8`),
      so `21f8359` is a real commit on `main`.
    - The receipt is `orient-1.json` under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1815Z\`.
    - The ORIENT gate was GREEN (*Current build verification*).
  - **Al's owner note of 2026-10-28** (verbatim under *Standing rulings*; ADR-0011 *Amendments*;
    HANDOFF Session 41, Entry 3) was recorded first (`da2bd83`): ADR-0001 stays unedited on
    flaw-count units, and a pair's injected instance is confirmed only when both of its records
    pass their checks.
  - **Commits:**
    - `da2bd83` the records;
    - `af28cc96aa75cbabb0a6171d4c636a8cbc87f773` the 0.2.0 producer: build-time input hashes
      (`input-set.txt`, `build.rs`, `src/provenance.rs`; FINDING-005 closed), the version
      selection (0.1.0 and 0.2.0 in one crate; `generate` emits 0.2.0), counts per instance, the
      seeds `continuity_is_not_uniform`, `x_over_abs_x`, `rolle_abs` and `horses`, their witness
      modules, `Audit.lean` (eight lines) and the tests;
    - `d7b5d81` the 14 goldens of 0.2.0;
    - the record commit: `analysis/s2-generate-2/`, this file and HANDOFF.
  - **Measured at implementation** (`analysis/s2-generate-2/README.md`):
    - eight Lean probes compiled one process at a time before any seed record was written: the
      three convincing arguments fail exactly at their flaw sites, and the faithful argument and
      the three negation witnesses build with the standard three axioms;
    - the 12 goldens of 0.1.0 regenerate byte for byte under 0.1.0, and the 14 of 0.2.0 under
      0.2.0 (fidelity PASS; check K at lines 6, 4, 7 and 20; every flawed formal and pair golden
      byte-identical to its probe; every pair one instance with two records);
    - the provenance test: the compiled hashes cover exactly the input set on disk, and the CLI
      refuses an ancestor commit that lacks the crate (exit 2);
    - `cargo audit` clean (advisory-db `40f5be6f…`; `Cargo.lock` unchanged); the security review
      of the new code found nothing above informational.
  - **Readings flagged for Al** (ADR-0011 *Amendments*; packet readings 13–20): seeds whose
    statement is false have no control and intrinsic material; FLAW-0006's mode for the horses;
    the no-binders header; both versions in one crate; the FLAW-0002 theorem and its gap
    severity; `rolle_abs` in `HasDerivAt` form; the horses formalization; `omega`'s message.
  - **Al's rulings of 2026-10-28 on PR 27** (verbatim under *Standing rulings*; ADR-0011
    *Amendments*; HANDOFF Session 41's addendum), applied on the same branch: readings 13 and
    15–20 accepted; reading 14 replaced by the new D5 mode `step-above-base`; the horses record
    relabeled (proof bytes unchanged); the 14 goldens of 0.2.0 regenerated at one engine commit,
    `c899b073…`, from two byte-identical runs.
  - **Not yet built:** `s2_grammar`, padding (FLAW-0013), lemma steps, the T check, the seeds
    for the remaining FLAWS.md examples (FLAW-0003, 0004, 0007, 0008, 0009, 0010, 0011, 0012 and
    0013), and the sweep (ADR-0011 D11).
  - **Unchanged:** registers' statuses, the manifests under `groundtruth/`, frozen packets, the
    Gate S1 table, the constitution, the licence, the tag, `corpora/` (no artifact written) and
    every 0.1.0 golden. **NEXT** waits on this pull request's merge.
- **Previous update (Session 40):** 2026-10-28 UTC (2026-10-28 America/New_York). This was Session 40:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone, in a
    fresh conversation;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1–3). The matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing
    HANDOFF's lines in HANDOFF Session 40, Entry 2;
  - **not an examination**: Gate S2 has nothing to examine. The session certifies nothing;
  - **one interruption**: a safety classifier stopped one response during ORIENT step 6, after
    the ORIENT gate had ended GREEN. Nothing was written by it. Al then directed the session to
    continue (verbatim under *Standing rulings*; HANDOFF Session 40, Entry 3).

  **ADR-0011's crate is built** (STATE's NEXT as it stood: S2 ordering step 4). It is on branch
  `claude/s2-generate-crate` (PR 26), in seven commits: five before Al's rulings on PR 26, and
  two applying them. `main` is unchanged until Al merges.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `f9f973012cbe1a7d7f1d8cd4520db998dca0e550`, tree
    `84a010018335bfdc94bcc2214babd593b5b734bb`, with the disclosure append the only change.
    - That is **PR 25's merge** (by Al, 2026-10-28T17:02:54Z; parents `ef567ec` and `41008ed`).
      No later ruling on PR 25 exists.
    - The receipt is `orient-receipt-1.json` under
      `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1706Z\`.
    - The ORIENT gate was GREEN (*Current build verification*).
  - **Commits:**
    - `9b4b16f` the crate: plan, renderers, `syndeom-rng/1`, harness, `s2_template` 0.1.0, the
      CLI, three seeds, and tests;
    - `21f8359` the seed witnesses in the Lean library, with `Audit.lean` and the gate's axiom
      loop. This is the D10 gate change that ruling 3 gives its own review;
    - `f8cc5df` the 12 goldens;
    - `cd5633b` the security review's four fixes;
    - `8697ab3` the packet `analysis/s2-generate-1/`, this file and HANDOFF;
    - `91dde9a` and the record commit, both after Al's rulings on PR 26 (below).
  - **Measured at implementation** (`analysis/s2-generate-1/README.md`):
    - Golden A and Golden B are reproduced byte for byte by the writers;
    - the RNG vectors agree with an independent Python implementation of D4;
    - 12 goldens (each seed's control and FLAWS.md example, informal and formal) have fidelity
      PASS. The flawed formal cases fail at the site's fragment (check K). The three flawed formal
      goldens are byte-identical to their hand-written probes, and the two faithful probes are
      their goldens plus a trailing `#print axioms` line. This was corrected by Al's ruling 6 on
      PR 26: it first read "every formal golden equals its probe";
    - `cargo audit` is clean (advisory-db `40f5be6f…`);
    - the security review found five issues. FINDING-001..004 are resolved. FINDING-005, the
      binary-provenance gap, is to be closed by `s2_template` 0.2.0 (ruling 5).
  - **Al's rulings of 2026-10-28 on PR 26** (verbatim under *Standing rulings*; ADR-0011
    *Amendments*; HANDOFF Session 40's addendum), applied on the same branch in two more commits:
    - `91dde9a` (ruling 4) verifies `syndeom-rng/1` against `rand_xoshiro` 0.3.0's reference
      vectors (S-0081): SplitMix64 50 outputs, xoshiro256** 10 outputs, through
      `Stream::from_state`. No output changed;
    - the record commit holds rulings 1–6: the gate change accepted; readings 1, 2, 3, 5–9, 11
      and 12 accepted; reading 4 replaced (flaw counts are per instance, binding the next
      revision); 0.2.0's version and build-time input hashes; the probe wording corrected.
  - **Not yet built:** `s2_grammar`, padding (FLAW-0013), lemma steps, the T check, the seeds for
    the other FLAWS.md examples, and the sweep. They are later sessions (ADR-0011 D11).
  - **Unchanged:** registers' statuses, the manifests under `groundtruth/`, frozen packets, the
    Gate S1 table, the constitution, the licence, the tag and `corpora/` (no artifact written).
    **NEXT** waits on this pull request's merge.
- **Previous update (Session 39):** 2026-10-28 UTC (2026-10-28 America/New_York). This was Session 39:
  - launched with the §6 Session-N resume prompt verbatim, followed by an owner note, pasted into
    an existing Claude Code conversation: the one that had just done Session 38's post-merge
    housekeeping at Al's direction (below);
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1 and 2): the matched index lines
    (85, 86, 88, 89, 106, 107; `0418e621…534c`; 104 withheld) and the SYNDEOM memory file
    (`ce2ce6de…2cc0`) are the bytes Session 35's Entry 1 reproduced, verified by hashing HANDOFF's
    lines in HANDOFF Session 39, Entry 2;
  - **not cold and not an examination**: the conversation read program state before the prompt,
    and the owner note carries program state. The session certifies nothing.

  **The first later prefix's inventory event is built** (STATE's NEXT as it stood; Al's ruling of
  2026-10-28 on its scope), on branch `claude/s1-later-prefix-inventory`, in two commits, for Al's
  review. `main` is unchanged until Al merges.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `ef567ec9824319924300a3f2d620e8121a16af18`, tree
    `4bef2ccc6b24a0ed9c60294608c7dae839e28f88`, with the disclosure append the only change. That
    is **PR 24's merge** (by Al, 2026-10-28T05:27:56Z; parents `a793954` and `c43f9f9`). Receipt
    `orient-receipt.json` under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1553Z\`. ORIENT
    gate GREEN (*Current build verification*).
  - **Session 38's post-merge housekeeping — DONE on 2026-10-28, not in Session 38's
    conversation** (Al's owner note, item 4; *Standing rulings*). A forced Windows restart closed
    that conversation overnight. At Al's direction a fresh conversation, not launched with the §6
    prompt and not a numbered session, found the checkout clean on
    `claude/s1-held-harvest-integration` at `c43f9f9`, with one worktree and no restart damage. It
    fetched, read the default branch name from git, switched to `main` and fast-forwarded it from
    `a793954` to `ef567ec`. It changed no repository file and wrote no memory. That conversation
    then received this session's launch (HANDOFF Session 39, Entries 1 and 3).
  - **Commit 1** (`46cdcb8`): ADR-0013 (proposed) and a new tool revision in new files,
    `engine/ingest/examples/s1_later_prefix.rs` with `support/later_prefix.rs`, plus the packet's
    pins (`analysis/s1-later-prefix-1/config/`) and eight gate tests. The pinned
    `s1_effective_inventory` and `s1_4_coverage`, their packets and retained reports, and every
    manifest are unchanged.
  - **Measured** from commit 1's clean committed tree, two runs byte-identical: **PASS, 40
    archived representations (38 raw, 2 canonical) / 37 effective logical items**, 5 of them in
    the prefix. 30 harvest candidates (24 admitted, 4 rejected, 2 found-but-unclassifiable), prefix
    through `1404.4599`. Statement status 5 true / 18 false / 14 undetermined; unflagged documented
    subset 23; the same nine zero-coverage classes. Session 23's projection of 40 / 37 is now
    measured. The pinned tool refuses at that tree on the first prefix addition, as ADR-0012 D7
    says it must.
  - **Commit 2:** the retained outputs (`analysis/s1-later-prefix-1/report/`), the ninth gate test
    (the working tree yields them byte-identically), ADR-0013's measurements, ADR-0005
    *Amendments* 3 (Al's owner note, items 2 and 3), STATE and this block's close.
  - **Flags for Al (ADR-0013):** A, one tool and one packet; B, the five prefix roots unflagged;
    C, effective labels; D, the freeze-register row for the packet, proposed and not added.
  - **Al's rulings of 2026-10-28 on PR 25** (verbatim under *Standing rulings*; ADR-0013
    *Amendments* 1). A, B and C were accepted. D7 was amended: the recorded inputs pin the
    population, and counts are measured outputs, not conditions in the code. D was ruled: a final
    commit on the branch adds the packet's frozen row. They were applied on the same branch in
    three further commits:
    - commit 3 (`97e0d54`) removes the literal counts from the tool and the tests, takes the scope
      from the harvest log's pinned prefix, names the amendment map in `coverage.json`, rewrites D7
      and D8, and records the rulings;
    - commit 4 (`bcdcdfa36cc6be8a74409d34d08ea6edabd3ac3d`) holds the packet regenerated at
      `97e0d54`. Two runs were byte-identical, and every count is unchanged;
    - commit 5 adds the frozen row `analysis/s1-later-prefix-1/` with commit 4 as its freeze point.
      The register then holds 65 rows (60 frozen, 5 append-only), ALL PASS, with the new row at
      7 files, 7 byte-identical and 0 added.
  - **Unchanged:** registers' statuses, manifests, labels, frozen packets, the retained reports,
    the Gate S1 table, the constitution, the licence and the tag. The freeze register changes only
    by the one row of Al's ruling D. **NEXT** is S2 ordering step 4, ADR-0011's crate, after this
    pull request merges.
- **Previous update (Session 38):** 2026-10-28 UTC (2026-10-28 America/New_York). This was Session 38:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim, followed by an
    owner note;
  - SD15 disclosure first, **by digest** (ADR-0005 *Amendments* 1 and 2): the matched index lines
    (`0418e621…534c`) and the SYNDEOM memory file (`ce2ce6de…2cc0`) are the bytes Session 35's
    Entry 1 reproduced, verified by hashing HANDOFF's lines in HANDOFF Session 38, Entry 2;
  - **not an examination**: the owner note carries program state, and Gate S2 has nothing to
    examine. The session certifies nothing.

  **The held harvest is integrated as the first later prefix** (STATE's NEXT as it stood; Al's
  rulings Q-H1..Q-H7 of 2026-10-28), on branch `claude/s1-held-harvest-integration`, in two
  commits, for Al's review. `main` is unchanged until Al merges.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `a7939547b9d103baa3fd0474149e476e3e0b0dcb`, tree
    `bfa2ce236bdbf775a8485ec9e9b24b9db08ad5db`, with the disclosure append the only change. That
    is **PR 23's merge** (by Al, 2026-10-28T04:45:54Z; a merge commit with parents `c48705c` and
    `97fc197`), and its tree is PR 23's head tree. No worktree was needed. Receipt
    `orient-receipt.json` under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028-claude-opus\`.
  - **Housekeeping of Session 37's append and worktree — DONE in Session 37's conversation after
    PR 23's merge; verified here, not redone** (Al's note; receipt `housekeeping-receipt.json`
    under `C:\Users\LEET\Codex\Temp\syndeom-held-harvest-review-20261028\`, SHA-256
    `520163ba…d9af`, log `85dabb3f…b817`, re-hashed):
    - the worktree `C:\Users\LEET\Syndeom-wt-2026-10-27` is gone, and `git worktree list` shows
      the one checkout;
    - the discarded append (3,497 bytes, `42206164…4c11`, retained as `disclosure.lf.md`) is a
      110-byte provisional header plus the 3,387-byte body `1047d059…d400`, which occurs exactly
      once in HEAD's HANDOFF, at line 7198 (Session 37, Entry 1);
    - the checkout is on `main` at the fetched tip, HANDOFF's blob equals `origin/main`'s, the
      build products are in place, no stash, and the local branches are kept.
  - **Commit 1, the integration**, per REVIEW.md §4 with Al's rulings applied:
    - five item directories under `groundtruth/`: `s1_ingest-v0.1.0-34988f7c8306` (item 32),
      `s1_ingest-v0.1.0-915251f36fc5` (33), `s1_ingest-v0.1.0-bca249b3fa72` (34),
      `s1_ingest-v0.1.0-c6179869edf7` (35) and `s1_ingest-v0.1.0-11ffe4f87954` (36);
    - the manifests regenerated by `s1_ingest` 0.1.0 at `a793954` (their `engine_commit`) from
      Session 22's preserved configs and originals (pins unchanged). Each equals its held manifest
      up to the renumbering, the session label, the id, the config hash and the engine commit, and
      a rerun is byte-identical;
    - the records carried with the new ids, S-numbers and label, each with an *Integrated* line;
    - SOURCES S-0074..S-0080 appended: the branch's S-0066..S-0072, renumbered, each with an
      *Integrated* line (Session 22's retrievals, not re-fetched);
    - `groundtruth/HARVEST-LOG.md` continued as a new dated section after its end, the tag's
      bytes an exact prefix (Q-H7);
    - the labels as recorded (Q-H1, Q-H2/Q-H3, Q-H5); the further unnamed results not items (Q-H4).
  - **Commit 2:** five frozen rows in the freeze register, one per new item directory, with commit
    1's full id `d2569d61588368a60bbf7558fb9f0703f96e3a5b` as their freeze point (Q-H7). The register has 64 rows: 59 frozen, 5
    append-only.
  - **The inventory event is not in this pull request — Al's ruling of 2026-10-28 (Session 38;
    *Standing rulings*):** it is the next pull request. The committed corpus is 40 manifests (37
    `s1_ingest`, one each of `s1_derive`, `s1_pair` and `s1_pair_archive`), the count Session 23
    projected. That is a count of files, not a measurement. The pinned effective-inventory tool
    refuses the additions at this tree, as ADR-0012 D7 says it must.
  - **Unchanged:** registers' statuses, the 35 accepted item directories' bytes, frozen packets,
    the retained report, the Gate S1 table, the constitution, the licence, the tag and the held
    branch. **NEXT** is the inventory event, after this pull request merges.
- **Previous update (Session 37):** 2026-10-28 UTC (2026-10-27 America/New_York). This was Session 37:
  - launched from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim, followed by an
    owner note;
  - SD15 disclosure first, **by digest**, under Al's ruling of 2026-10-27, which is recorded as
    ADR-0005 *Amendments* 1 (HANDOFF Session 37, Entries 1–2);
  - **not an examination**: the owner messages carry facts about this repository, and Gate S2 has
    nothing to examine. The session certifies nothing.

  **The held harvest's admission review is written** (S2 ordering step 3, STATE's NEXT as it
  stood). Five items, five agree, no disagreement, seven questions for Al. Nothing was integrated
  or approved.
  - **Orientation.** The ADR-0008 preflight found `C:\Users\LEET\Syndeom` on `main` at
    `f8d71b8c…`, **behind** fetched `origin/main` `c48705c2d23183340eef5c01ff13059905513985` by
    three commits, with the disclosure append as the only change.
    - The fetched tip is **PR 22's merge**, by Al, 2026-10-27T23:06:37Z. Its tree
      `a6b256840d764818c6d314259883967e0e10c7a4` is the tree Session 36 gated GREEN on PR 22's
      head `fd8fa9e`.
    - The fast-forward was refused, because the incoming commits touch HANDOFF. The preflight
      therefore created the worktree `C:\Users\LEET\Syndeom-wt-2026-10-27` at the fetched tip, and
      the disclosure was carried into it (ADR-0008 D3).
    - Receipts `orient-1.json`, `orient-2-sync.json` and `orient-3-worktree.json` are under
      `C:\Users\LEET\Codex\Temp\syndeom-held-harvest-review-20261028\`.
    - The original checkout's append stands uncommitted (Standing escalations).
  - **The review** is in `analysis/s1-held-harvest-review/` (`REVIEW.md`, `receipt.json`).
    - Session 22's preserved evidence re-verified: 147 of 147 files, `ALL MATCH`.
    - Every pinned original equals its preserved e-print.
    - The stream is run 1's (`e164c0b1…2d48`): candidates 28–30 in order, nothing skipped.
    - ADR-0003 D2 and D3 were re-applied per item, and items 32–36 all AGREE.
    - Questions Q-H1..Q-H7 go to Al (Standing escalations).
    - Two integration findings go beyond Session 22's conditions. The branch inserts its log rows
      mid-table, which would fail the append-only freeze row. Its "Session 21" label is
      Session 22 on `main`.
  - **ADR-0005 *Amendments* 1** records Al's disclosure ruling verbatim. SD15's constitution text
    is not edited; Al has since ruled that the amendment stands as an addendum (*Amendments* 2).
  - **Unchanged:** registers' statuses, manifests, frozen packets, the retained report, the
    accepted corpus (35 / 32), the constitution, the licence, the tag and the held branch.
  - **Al's rulings on PR 23 (2026-10-28)** are recorded verbatim in a third commit on the same
    branch: under *Standing rulings* below, in HANDOFF Session 37's addendum, and, for SD15's
    wording and the digest refinement, in ADR-0005 *Amendments* 2.
    - The five items' labels stand as recorded (Q-H1, Q-H2/Q-H3, Q-H5). The further unnamed
      results stay out as items and join the existing unnamed-result question (Q-H4).
    - **Integrate now**, as the first later prefix, per REVIEW.md §4 (Q-H6). The harvest log
      continues as a new dated section after its end. The five new item directories get frozen
      rows in a second commit, with the first commit as their freeze point (Q-H7).
    - Pull requests keep merging with merge commits, never squash (Q-H7).
    - SD15's wording: ADR-0005's amendments stand as an addendum, and the constitution is not
      edited. The by-digest comparison is now of the matched index lines and the SYNDEOM memory
      file, not of the whole index.
  - **NEXT** is the integration, after this pull request merges. Nothing is integrated yet.
- **Previous update (Session 36):** 2026-10-27 UTC (Session 36 — an informed working session, not an examination,
  continuing Session 35's conversation at Al's direction after PR 21's merge; not launched with
  the §6 prompt, so no SD15 disclosure): **ADR-0012 *Amendments* 1 — the narrowing's safety
  property tested, and the freeze register checked by the gate (Al's direction of 2026-10-27).**
  - **Orientation:** the ADR-0008 preflight (receipt `orient-1-sync.json` under
    `C:\Users\LEET\Codex\Temp\syndeom-adr-0012-freeze-20261027\`) fast-forwarded the clean `main`
    from `6d89eb2f…` to fetched `origin/main` `f8d71b8c8034638f14e9a5885ae348c8fdde24c4`, tree
    `35bb1439f4d7ac4ca54734ca61192ec4a68b3ff1`. That is **PR 21's merge** (2026-10-27T21:36:26Z,
    by Al); its tree equals PR 21's head tree (`9dc0d1e`), which Session 35 gated GREEN.
    **ADR-0012 is accepted** at that merge.
  - **The invariance test:** `inventory_is_byte_identical_under_every_outside_mutation` in
    `engine/ingest/tests/input_set.rs` — every outside file mutated four ways at once, three
    files added outside `groundtruth/`, the whole report byte-identical. Gate Rust tests 88 → 89.
  - **The freeze register** (section below): 59 rows, all at `gate-s1-registration` — the seven
    S1 packets, one row per accepted item directory (35), `groundtruth/derivations/`, the sweep,
    the advisory note and the nine stored copies frozen (54 rows, 384 files); HANDOFF, SOURCES,
    `groundtruth/HARVEST-LOG.md`, `groundtruth/README.md` and ADR-0003 append-only — the first
    nine derived from this file's records, the other fifty added by Al's rulings 1–4 of
    2026-10-27 on PR 22 (verbatim in ADR-0012 *Amendments*; Standing escalations).
    `scripts/check_freeze.py` and `scripts/check_freeze_test.py` (8 tests) join the gate after the
    register cross-reference check; the advisory CI fetches history and tags and sets
    `origin/HEAD`. The scopes not proposed stay unchecked by ruling.
  - **Unchanged:** registers' statuses, manifests, frozen packets, the retained report, the
    accepted corpus, the constitution, the licence, the tag, the held branch and the S1.3
    reproduction's input set. **NEXT** stays at ordering step 3 and follows this pull request's
    merge.
- **Previous update (Session 35):** 2026-10-27 UTC (Session 35 — a session launched from `C:\Users\LEET` with the §6
  Session-N resume prompt verbatim followed by an owner note; SD15 disclosure first (HANDOFF
  Session 35, appended without reading the file, under a provisional header completed after
  ADR-0008 step 5); **not an examination** — the owner note carries program state, and Gate S2
  has nothing to examine; it certifies nothing): **ADR-0012, the S1.3 effective-inventory
  reproduction's byte checks narrowed to its input set — decided, implemented and measured; no
  S2 code (S2 ordering step 2; STATE's NEXT as it stood).**
  - **Orientation:** the ADR-0008 preflight found the checkout `C:\Users\LEET\Syndeom` on `main`
    **synchronized** with fetched `origin/main` `6d89eb2f8a138a53d824d035febffc93423e1e0d`
    (**PR 20's merge**, by Al, 2026-10-27T20:28:10Z; tree
    `93ded916fb8cbfaea21f7d2a2367f86415fe3fc9`, the tree Session 34 gated GREEN on PR 20's head
    `751fd97`), with the disclosure append the only change (receipt `orient-1.json` under
    `C:\Users\LEET\Codex\Temp\syndeom-adr-0012-20261027\`). No worktree: the checkout was the
    snapshot.
  - **Housekeeping (Al's ruling 4 of 2026-10-27) — DONE in Session 34's conversation after the
    merge; verified here from its receipt, not redone** (`housekeeping-receipt.json` under
    `C:\Users\LEET\Codex\Temp\syndeom-adr-0011-20261027\`, SHA-256 `170e0d58…4670`): the worktree
    `C:\Users\LEET\Syndeom-wt-20261027` is gone (`git worktree list` shows the one checkout); the
    discarded append's body (14,931 bytes, `36dda70c…8d2f`, recomputed from the retained copy)
    equals the committed body at HEAD lines 6286–6444 (recomputed from the HEAD blob); the
    checkout was switched to `main` and fast-forwarded by the preflight
    (`housekeeping-orient-sync.json`); the local branch `claude/s2-adr-0011-generator` was kept.
  - **ADR-0012** (`PROGRAM/DECISIONS/ADR-0012.md`, proposed; implemented in the same commit):
    `preserve()` checks exactly the reproduction's input set — the taxonomy, every accepted item
    directory under `groundtruth/`, the S1.4 dispositions overlay and the four accepted-candidate
    packet files it reads: 79 of the 375 baseline files, derived from the baseline list and listed
    in the output — and refuses additions under `groundtruth/`; nothing else is byte-checked, so a
    constitution amendment, an ADR, SIBLING-MAP, `scripts/check.sh`, `lean/Audit.lean`,
    `engine/Cargo.toml` or `engine/Cargo.lock` neither stops it nor fails the gate. The four
    case-by-case exceptions (ADR-0009 D5; ADR-0010 D6) are replaced by the rule; the pair-input
    comparison with the checkout is removed (the hashes stay recorded from the producer revision;
    the replay claim is the revision range below). Tests: `relicense_exceptions` (6) replaced by
    `input_set` (6); `first_parent_amendment` keeps 15, two rewritten to the new boundary.
    **Measured:** at `3cd5558` with that revision's tool the retained `inventory.json` is
    byte-identical; at this revision's tree the reproduction passes and differs from the retained
    only in the `preservation` object (ADR-0012 D6 lists the eleven paths).
  - **Unchanged:** registers' statuses, manifests, frozen packets, the retained report, the
    accepted corpus (35 / 32), the constitution, the licence, the tag and the held branch. **NEXT**
    moves to ordering step 3, the held harvest's admission review.
- **Previous update (Session 34):** 2026-10-27 UTC (Session 34 — a session launched from `C:\Users\LEET` with the §6
  Session-N resume prompt verbatim; SD15 disclosure first (HANDOFF Session 34, appended without
  reading the file, under a provisional header completed after ADR-0008 step 5); **not an
  examination** — Gate S2 has nothing to examine, which the session could establish only after
  orienting; it certifies nothing): **ADR-0011, the first generator family's design, written —
  decision only, no code (S2.1; STATE's NEXT as it stood).**
  - **Orientation:** the ADR-0008 preflight found the checkout `C:\Users\LEET\Syndeom` on branch
    `claude/s2-citation-cff` at `8ae94316f327194da86e3ce64b58d2573987e325`, **behind** fetched
    `origin/main` `a741ebf956be0bd87ebf1b606f0b9c6df1853043` (**PR 19's merge**, by Al; the same
    tree `a5e7df5488530c560ab7ec9eb608037f92a03820`, which Session 33 gated GREEN on `8ae9431`),
    with the disclosure append the only change. Not on the default branch, so no fast-forward:
    the preflight created the isolated worktree `C:\Users\LEET\Syndeom-wt-20261027` at the fetched
    tip (exit 0, `SNAPSHOT-ESTABLISHED-IN-WORKTREE`; receipts `orient-1-worktree-refused.json` and
    `orient-2-worktree.json` under `C:\Users\LEET\Codex\Temp\syndeom-adr-0011-20261027\`), and the
    disclosure was carried into it as the first action there (ADR-0008 D3). The original
    checkout's append stands uncommitted (Standing escalations).
  - **ADR-0011** (`PROGRAM/DECISIONS/ADR-0011.md`, proposed): one plan data structure as the shared
    interface behind the four S2.1 modules; two producers, `s2_template` (hand-authored seed
    theorems — the FLAWS.md examples — with aligned informal and formal proofs) and `s2_grammar`
    (derivation grammars with a commanded length); `syndeom-rng/1`, specified in the ADR, no new
    dependency; one dial per FLAWS.md `0.1.0` recipe with its kernel or tooling confirmation
    check; commanded and achieved keys with tolerances; `syndeom-manifest/2` manifests with
    `regenerable` storage; goldens from the FLAWS.md examples; `cargo audit` outside the offline
    gate; the crate `engine/generate/` under PolyForm, implemented only after the byte-check
    narrowing. **D12 flags for Al, undecided:** whether the R4 Mnemosyne-derived seed family
    enters, and its licence line under R1(b).
  - **Unchanged:** registers' statuses, manifests, frozen packets, the accepted corpus, the
    constitution, the licence and the tag. **NEXT** moves to ordering step 2, the narrowing.
  - **Al's rulings on PR 20 (2026-10-27; a second commit on the same branch, verbatim in ADR-0011
    *Amendments* and HANDOFF Session 34):** (1) independence from the S1 corpus — no seed, grammar
    or template derived from any `groundtruth/` item; every seed record states its source;
    binding on later producer versions (ADR-0011 D6); (2) the R4 seed family stays out of this
    repository, option (iii) (D12); (3) the non-blocking clarifications accepted as recorded, the
    `scripts/check.sh` and `lean/Audit.lean` edits after the narrowing with their own review;
    (4) housekeeping for the first session after the merge (Standing escalations; NEXT).
    **ADR-0011 is accepted, in effect at the merge.**
- **Previous update (Session 33):** 2026-10-27 UTC (Session 33 — an informed working session, not an examination,
  continuing Session 32's conversation at Al's direction; not launched with the §6 prompt, so no
  SD15 disclosure): **`CITATION.cff` added, and its refresh joins the deposit procedure, before
  the tag (Al's ruling of 2026-10-27, Al's rulings on PR 19's two calls and Al's ruling on the
  open point, verbatim in ADR-0010 *Amendments* and HANDOFF Session 33).**
  - **Orientation:** the ADR-0008 preflight (receipt
    `C:\Users\LEET\Codex\Temp\syndeom-citation-cff-20261027\orient-1-sync.json`)
    fast-forwarded the clean `main` to fetched `origin/main`
    `99bdf0362e12d7d7c405566396d7a1a7f01c04a1`, tree `c063cc6589fc4f5d7605690ae7103cbd87993f5c`.
    That is **PR 18's merge** (2026-10-27T06:05:00Z, by Al). Its tree equals the PR's head tree,
    which Session 32 gated GREEN.
  - **`CITATION.cff`** (Citation File Format 1.2.0, schema-valid): `doi` is the concept DOI
    `10.5281/zenodo.23682780`; the version DOI `10.5281/zenodo.23682781` is listed under
    `identifiers` as the Gate S1 design registration; `version`, `date-released` and `title`
    match the latest deposit. It has no `commit` field (Al's ruling on PR 19).
  - **The deposit procedure** now refreshes `CITATION.cff` in the commit that is tagged, after the
    version DOI is reserved in the Zenodo draft. The tagged file is the reference: Zenodo is
    corrected to match it, and the records commit after the deposit confirms the match (Standing
    escalations, first bullet; ADR-0010 *Amendments*).
  - **Unchanged:**
    - the licence, the preservation tool, the README and the `gate-s1-registration` tag;
    - registers' statuses, manifests, frozen packets and the accepted corpus;
    - NEXT.
- **Previous update (Session 32):** 2026-10-27 UTC (Session 32 — an informed working session, not an examination,
  continuing Session 31's conversation at Al's direction; not launched with the §6 prompt, so no
  SD15 disclosure): **the Gate S1 design registration is published on Zenodo, and the records
  now say so (Al's instruction and rulings of 2026-10-27, quoted in HANDOFF Session 32).**
  - **Orientation:** the ADR-0008 preflight (receipt
    `C:\Users\LEET\Codex\Temp\syndeom-post-deposit-records-20261027\orient-1-sync.json`)
    fast-forwarded the clean `main` to fetched `origin/main`
    `2018d5e79ea23fa200e560d628bec5e91edf0956`, tree `3d7889def475b2b2f30d8acfd3dee31589a06a6e`.
    That is **PR 17's merge** (2026-10-27T02:48:36Z, by Al). Its tree equals the PR's head tree,
    which Session 31 gated GREEN.
  - **The deposit is done** (Standing escalations, first bullet): the annotated tag
    `gate-s1-registration` on `2018d5e`, the archive built from it, and Zenodo record 23682781
    (version DOI `10.5281/zenodo.23682781`; concept DOI `10.5281/zenodo.23682780`). Every
    identifier was confirmed from Zenodo's API before it was written.
  - **README:** the passages that presented Gate S1 as open now cite its closure (this file,
    HANDOFF Session 29, PR 15) and NEXT below; a DOI badge and a Citation section were added.
  - **Corrected here:** the Standing-escalations bullet on Session 29's pull request now records
    PR 15's merge.
  - **Unchanged:**
    - the licence, the preservation tool, and the `gate-s1-registration` tag;
    - registers' statuses, manifests, frozen packets and the accepted corpus;
    - NEXT.
- **Previous update (Session 31):** 2026-10-27 UTC (Session 31 — an informed working session, not an examination,
  continuing Session 30's conversation at Al's direction; not launched with the §6 prompt, so no
  SD15 disclosure): **third-party notices and the Apache-2.0 text, ahead of the Gate S1
  registration deposit (Al's rulings of 2026-10-27; ADR-0010 *Amendments*).**
  - **Orientation:** the ADR-0008 preflight (receipt
    `C:\Users\LEET\Codex\Temp\syndeom-registration-licence-20261027\orient-1-sync.json`)
    fast-forwarded the clean `main` to fetched `origin/main`
    `368bcfb5c05c23536a32bfc15b9537173fb0fef7`, tree `d33402ca2bfa0efcee577b2855b0741c07cdbe6e`.
    That is **PR 16's merge** (2026-10-27T02:09:06Z, by Al). Its tree equals the PR's head tree,
    which Session 30 gated GREEN.
  - **The audit**, file by file against the SD16 records at `368bcfb`, found:
    - one file whose recorded rights permit no redistribution (the advisory note, R1(b));
    - one CC BY-NC 4.0 copy (S-0005);
    - Apache-2.0 files, with no Apache-2.0 text in the tree;
    - 30 ground-truth items whose derived records had no licence recorded for themselves.
  - **Added:**
    - `LICENSES/Apache-2.0.txt`: S-0019's text, fetched again from SPDX. Its SHA-256 equals
      S-0019's record, and it is byte-identical to `LICENSE` through `c9372b1`.
    - `THIRD-PARTY-NOTICES.md`: every file not under PolyForm, with its licence and attribution.
    - A pointer in README § Licence, and an S-0019 addendum (no new S-number).
  - **The registration deposit** (rulings 1, 2 and 4) follows this pull request's merge (Standing
    escalations):
    - the tag `gate-s1-registration` on the merge commit;
    - a zip from the tag that excludes the advisory note and the Leiden PDF.
  - **Unchanged:**
    - the licence (ADR-0010 D1–D7) and the preservation tool;
    - registers' statuses, manifests, frozen packets and the accepted corpus;
    - NEXT.
- **Previous update (Session 30):** 2026-10-27 UTC (Session 30 — an informed working session, not an examination,
  continuing the conversation of Session 29 at Al's direction): **ADR-0010, the relicense to the
  PolyForm Noncommercial License 1.0.0, accepted by Al on 2026-10-26 with rulings C1–C5.**
  - **Orientation:** the ADR-0008 preflight (receipt
    `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\preflight-orient.json`) fast-forwarded
    the clean `main` to fetched `origin/main` `c9372b1355a6d687dec6287f908639ac670db659`, tree
    `4caa81f0c56416a02830d19d4a11eb45c9c4a9f4`. That is **PR 15's merge** (2026-10-27T00:25:39Z,
    by Al), whose tree is the one Session 29 gated on its committed rulings commit.
  - **The licence:** `LICENSE` now holds the Required Notice line and the verbatim PolyForm text
    (S-0073, SHA-256 `c0ea4a89…fabbe5`, equal to Al's stated digest). Copyright is held personally
    by Alexander David Zalewski; commercial licensing is at adz@alphx.io. The four crates'
    `license` fields are `PolyForm-Noncommercial-1.0.0`, and README § Licence carries a
    plain-language summary.
  - **The preservation check** gained three narrow exceptions (ADR-0010 D6), with six narrowness
    tests in the gate. The S1.3 reproduction at `3cd5558` still yields the retained inventory
    byte-identical; at HEAD it passes, differing only in three `preservation` fields (bullet
    below).
  - **Replay of the admitted S1.2 pair from HEAD ends here.** It runs on `e6a7cae…`..`c9372b1`
    (procedure below; run once at `c9372b1` in a clean clone: PASS, byte-identical to the
    original production). The 18 committed receipts are unchanged.
  - **Unchanged:** historical "Apache" statements in frozen files, and the pair's recorded wrapper
    licence (C4, C5).
  - **Ordering:** the S2 generator decision is now **ADR-0011**, and the byte-check narrowing
    lands before any S2 code (NEXT).
  - **Unchanged:** registers' statuses, manifests, frozen packets and the accepted corpus.
- **Previous update (Session 29):** 2026-10-26 UTC (Session 29 — **the Gate S1 cold-start examination: PASS; Gate S1
  CLOSED; phase S2 opens**, effective on `main` when Al merges this session's pull request). A
  fresh conversation with no prior turns, launched from `C:\Users\LEET` (not a git work tree; no
  Git snapshot attached) with the §6 Session-N resume prompt in its ADR-0009 wording, verbatim and
  alone. It opened as ADR-0009 prescribes: SD15 disclosure first, appended to HANDOFF without
  reading it (2026-10-26T23:14:39Z), under a provisional header completed after the preflight
  (HANDOFF Session 29). Its ADR-0008 preflight found the checkout **synchronized** with fetched
  `origin/main` `e029c80e1be149bef8fe6ceb76eed59c235ed8b4`, tree
  `67023777965d5747c5e3e5fe3f655825f658c046` (receipt
  `C:\Users\LEET\Codex\Temp\syndeom-s1-cold-start-exam-20261026\preflight-orient.json`). **PR 14
  (Session 28's record) merged** at 2026-10-26T22:35:49Z (18:35:49 America/New_York), merged by
  Al, creating that main. Its parents are the PR's base `0afbbd1…` and its head
  `fea670843d85212148be5995d244a97c83829d4e`, and its tree equals the head's, which Session 28's
  staged-tree gate certified GREEN (receipt
  `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\gate-staged.json`). CI
  pull_request run 40413519657 and push run 40413650650 both succeeded. Two later events are
  recorded only in that external root and are reconciled here:
  - the launch-directory hazard line Session 28 proposed was appended to the executor's SYNDEOM
    memory file on Al's approval, after the merge (`memory-change.txt`; Memory policy below);
  - four merged branches were deleted from `origin` on Al's instruction
    (`deleted-branches-manifest.txt`); the held branch was kept.

  The examination re-derived the program state from the repository in one pass. The ORIENT gate
  was GREEN, and the S1.3 reproduction on a fresh clone printed `PASS: 35 archived
  representations / 32 effective logical items`, differing from the retained inventory only in
  `exact_files_checked` 370 → 369, as this file predicted. Every Gate S1 criterion has evidence
  (table below). Three SD15 judgments were stated for Al's audit; **Al upheld all three on
  2026-10-26** (Standing escalations, verbatim), so the PASS is not conditional. Registers, frozen
  packets and the accepted corpus are unchanged.
- **Previous update (Session 28):** 2026-10-26 UTC (Session 28 — a fresh conversation launched with the §6 Session-N
  resume prompt in its ADR-0009 wording, verbatim and alone, but **inside the checkout**
  `C:\Users\LEET\Syndeom`. Before its first action the harness attached a Git status and history
  snapshot (branch, `git status`, the last five commit subjects) to the conversation, which SD15
  counts as program state read before disclosing, so it is **not an examination**. It certifies
  nothing and is recorded as a non-exam session at Al's direction of 2026-10-26.) It opened as
  ADR-0009 prescribes: SD15 disclosure first, appended to HANDOFF without reading it, under a
  provisional header completed after the preflight (HANDOFF Session 28). Its ADR-0008 preflight
  found the checkout **synchronized** with fetched `origin/main`
  `0afbbd17cdc5e9e93691612a95bd5950e3243e10`, tree `27106131f522357184171e19d0e0e51ba2f30d54`
  (receipt `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\preflight-orient.json`).
  **PR 13 (Session 27's record) merged** at 2026-10-26T21:42:45Z (17:42:45 America/New_York),
  merged by Al, creating that main. Its parents are the PR's base `6e198e0…` and its head
  `fbadee37b5148e25cd803e6df01d5346dd9e05b9`, and its tree equals the head's, which Session 27
  gated GREEN (receipt `C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\gate-staged.json`,
  raw log re-hashed and matching). CI pull_request run 40403469499 and push run 40406649416 both
  succeeded. **Launch-directory finding** (Memory policy below): a session launched inside the
  checkout receives that snapshot before its first action, so it can never be an examination.
  The examination is launched from outside the checkout, from `C:\Users\LEET`, as earlier
  sessions were. Registers, frozen packets and the accepted corpus (35 archived representations,
  32 effective logical items) are unchanged; Gate S1 stays OPEN.
- **Previous update (Session 27):** 2026-10-26 UTC (Session 27 — launched with the §6 Session-N resume prompt in its
  ADR-0009 wording, but inside the conversation that built Session 26: **not cold** under SD15's
  conversation-context corollary, so **not an examination**, and it certifies nothing; recorded
  as a non-cold §6 launch, not as a Gate S1 attempt). It opened exactly as ADR-0009 prescribes —
  SD15 disclosure first, appended to HANDOFF without reading it, under a provisional header
  completed after the preflight (HANDOFF Session 27) — and its ADR-0008 preflight found the
  checkout **synchronized** with fetched `origin/main`
  `6e198e066c128976d55d0194f278cc645baf9249`, tree `4ad8422654cb56d9ffe68a717100df5af240a22e`
  (receipt `C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\preflight-orient.json`). **PR 12
  (ADR-0009) merged** at 2026-10-26T21:03:28Z (17:03:28 America/New_York), merged by Al, creating
  that main; its parents are the PR's base `3cd5558…` and its head
  `61cc974ca67bb47d0ac7aaa5f2a9b6c219969b19`, and its tree equals the head's, which Session 26
  gated GREEN (receipt `C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\gate-final.json`, raw
  log re-hashed and matching). CI pull_request run 40397221741 and push run 40401087386 both
  succeeded. ADR-0009, the amended SD15 and §6 Session-N prompt, and the reproduction tool's
  one-line fix are on main. Registers, frozen packets and the accepted corpus (35 archived
  representations, 32 effective logical items) are unchanged; Gate S1 stays OPEN.
- **Previous update (Session 26):** 2026-10-26 UTC (Session 26 — an informed working session launched with Al's task
  text, not the §6 prompt; not an examination, and it certifies nothing about Gate S1). **PR 11
  (Session 25's record) merged** at 2026-10-26T19:55:55Z (15:55:55 America/New_York; merge
  committer stamp 19:55:54Z), merged by Al, creating main
  `3cd5558223294571dd85164eb0d2730cb1911637`. Its parents are the PR's base `5356412…` and its
  head `e1c81505017bd772dc4cb9caf36aaaf41cd90717`, and its tree
  `125b1ed56d8dcab8074119eb51fc3467dbb09dda` equals the head's, which Session 25 gated GREEN
  (receipt `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\gate-staged.json`, raw log
  re-hashed and matching). CI pull_request run 40389787616 and push run 40391129353 both
  succeeded. **ADR-0009 accepted by Al (2026-10-26) and applied:** every session launched with
  the §6 Session-N resume prompt now makes the SD15 disclosure first, whether or not it attempts
  a gate test, before any action but reading the constitution and ADR-0005 in full and reading and
  hashing the memory; it appends the disclosure to HANDOFF without reading that file, under a
  provisional block header completed after orientation; a session that reads program state before
  disclosing cannot be an examination. The §6 prompt's first sentence now says so, and the prompt
  points to ADR-0008 for orientation. ADR-0008 D1 step 0 follows (bullet below). The
  constitution's SHA-256 is now `110f5360af60075a8e799dce0dc235d1ef8785bd40b0b34c38fa53ddb1d5479c`
  (38,958 bytes; previously `89c198b0…8102`). The Session-25 escalation is decided. The first
  staged-tree gate failed (4 S1.3 reproduction tests rejected the amended constitution); by Al's
  option-1 ruling the reproduction tool now skips the constitution, which changes only by ADR, as
  it skips README and STATE (ADR-0009 D5; one line). **S1.3 reproduces at both revisions:** at
  `3cd5558` the retained `inventory.json` is byte-identical; at HEAD the reproduction passes and
  differs only in `preservation.exact_files_checked`, 370 → 369 (bullet below). Registers, frozen
  packets and the accepted corpus (35 archived representations, 32 effective logical items) are
  unchanged; Gate S1 stays OPEN.
- **Previous update (Session 25):** 2026-10-26 UTC (Session 25 — a fresh conversation launched with the §6 Session-N
  resume prompt; it attempted the Gate S1 cold-start examination and is **ineligible as an
  examination**: before appending its SD15 disclosure it ran orientation commands (including a
  fetch) and read STATE, HANDOFF entries, Git history and ADR-0008, contrary to ADR-0008 D1 step 0
  and SD15's "before any other action". The disclosure was then appended late, in full, and
  stands (HANDOFF Session 25). Its ADR-0008 preflight then found the checkout **synchronized**
  with fetched `origin/main` `535641227f408095f36a92000e808ed5b65e135c`, tree
  `1fdeda3eca982fbaec04ab2b1f8fdaaeb82c70ea` (receipt
  `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\preflight-orient.json`). **PR 10 (the
  Session-24 cold-start preparation) merged** at 2026-10-26T19:28:28Z (15:28:28
  America/New_York; merge committer stamp 19:28:27Z), merged by Al, creating that main. Its tree
  exactly equals tested head `3f0171a7dc36f90154739b5bbafce11da8f37dc2` (`git diff --quiet`
  exit 0; full gate GREEN on that tree 2026-10-26T19:22:04Z–19:22:58Z, receipt
  `C:\Users\LEET\Codex\Temp\syndeom-cold-start-prep-20261026\gate-commit-3f0171a.json`); its first
  parent is the PR's base `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10`, so main has no intervening
  commits. Tested-head CI (run 40386340630) and main-push CI (run 40387061357) both succeeded.
  `git ls-remote --heads origin` now lists only `main` and the held
  `claude/s1-harvest-candidates-28-30` at `c3a3d36`. The external root's
  `deleted-branches-manifest.txt` (written 19:29Z) lists ten merged branches removed after the
  merge, the cleanup Session 24's escalation records as authorized by Al. Registers,
  frozen packets, the accepted corpus (35 archived representations, 32 effective logical items)
  and the constitution's bytes are unchanged; Gate S1 stays OPEN; the reason step 0 was easy to
  miss is escalated (Standing escalations, first bullet).
- **Previous update (Session 24):** 2026-10-26 UTC (an informed preparation session, not the Gate S1 examination,
  opened per ADR-0008 D1 steps 1–5 from fetched `origin/main`
  `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10`, tree `b5fe517fb4fa58846df83187aaa11ee7c2954762`):
  PR 9 (the Session-23 orientation repair) merged at **2026-10-22T00:07:26Z** (2026-10-21
  20:07:26 America/New_York), creating main `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10`. Its tree
  `b5fe517fb4fa58846df83187aaa11ee7c2954762` exactly equals tested head
  `5e24d37d8a1882d182a27076ba03905588513542` (full gate GREEN on that tree,
  2026-10-22T00:00:49Z–00:04:39Z; receipt in PR 9 and under
  `C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\final-gate-5e24d37d8a18\`); its
  first parent is the PR's base `d8c0d9cbcef0af5249cbf439808b74d8828bd69d`, so fetched main has
  no intervening commits. Tested-head and main-push CI both succeeded. Stale statements are
  corrected: Blockers; the checkout note in NEXT-SINGLE-ACTION; the conditional "after/once this
  reconciliation is on main" wording in the Gate line, the Gate S1 table's cold-start row and
  NEXT; and Current build verification (the PR 8 readiness bullet is kept as historical). The
  session is recorded in HANDOFF Session 24. Registers, frozen packets, the constitution's bytes
  and the accepted corpus (35 archived representations, 32 effective logical items) are
  unchanged; Gate S1 stays OPEN and the next action is still the separate genuine examination,
  opened per ADR-0008 D1.
- **Previous update (Session 23):** 2026-10-21 UTC (the orientation repair under Al's decisions of
  2026-10-21, an informed working session on branch `claude/s1-orientation-repair` from `main`
  `d8c0d9cbcef0af5249cbf439808b74d8828bd69d`): **ADR-0008**, the accepted addendum governing
  §4.1 (fetch and identify the examined snapshot before reconstructing state; preflight
  `scripts/orient_preflight.py` with receipts; offline cases all passed), is the governing
  orientation procedure (bullet below). Al's decision 1 preserves the held harvest branch at
  `c3a3d36` and defers its admission review until after Gate S1 closes; decision 3 keeps the
  existing dispositions. Session 22's retrieval evidence for candidates 28–31 is preserved
  externally with full hashes; the accepted corpus (35 archived representations, 32 effective
  logical items), registers, frozen packets and the constitution's bytes are unchanged. Gate S1
  stays OPEN; the next action is the separate genuine examination, opened per ADR-0008 D1.
  Event: `analysis/s1-orientation-repair/`.
- **Previous update (Session 22):** 2026-10-21 UTC — a session launched with the §6 resume prompt ran
  the paused harvest from an unfetched working copy (its `origin/main` ref was 24 commits stale,
  so §4.1 reconstructed the Session-20 state); its candidates 28–30, all admitted (five items),
  are held on branch `claude/s1-harvest-candidates-28-30` for Al's disposition and are **not on
  main**. Main's corpus, registers, frozen S1.4 packet and readiness packet are unchanged; the
  ORIENT note now says to fetch first; Gate S1 stays OPEN for the separate cold-start
  examination, which remains the next action (HANDOFF Session 22).
- **Previous update (PR 8 readiness):** 2026-10-21 UTC — PR 7's actual main activation and S1 readiness reconciled.
  Owner acceptance remains dated 2026-10-20 America/New_York, at the exact ruling
  `37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f`; the hash-bound decision and ADR-0003
  are unchanged. PR 7 merged at **2026-10-21T03:33:35Z** (2026-10-20 23:33:35
  America/New_York), creating main `1c56028532ea135603dbad5423d5555313bfd9ec`.
  Its tree `14b451866491dc4a2b866ec0b1d726d792385916` exactly equals tested head
  `a337375dbae49150a1a40fa46f6d1417835aadd2`; fetched main has no intervening commits.
  The existing Rust consumer passed on clean merged main and reproduced the retained
  inventory bytes. Logical key `s1_ingest-v0.1.0-63e23a952084` selects exactly one
  effective status from `s1_ingest-v0.1.0-15ab865466d6`: 35 archived representations
  / 32 effective logical items, unflagged subset 18. **Status-support hold discharged
  on verified main.** Both raw objects and original producing revisions remain intact.
  S-0025 is author error/locator evidence; the accepted written counterexample is
  separately attributed. The full analytic bridge remains unformalized.
  [Activation and fresh evidence](../analysis/s1-post-merge-readiness/README.md);
  [current effective inventory](../analysis/s1-first-parent-integration/report/inventory.json);
  [S1 readiness assessment](../analysis/s1-post-merge-readiness/READINESS.md).
  No additional substantive S1 exit criterion is identified as unmet for this admitted
  prefix; Gate S1 stays OPEN pending the separate genuine cold-start examination.
- **Previous update (first-parent proposal, before PR 6 merge):** 2026-10-20 UTC — PR 5 owner merge reconciled; first-parent mathematical
  status-support witness checked, metadata amendment proposed, canonical promotion held.
  PR 5 merged at 20:41:46Z to main `1a76d9ac21cd5d2e4311f9164178327c0ca128bb`, tree
  `3b9028c0502add53ac646108132d89ba5f1e5666` (same as PR head
  `618271daf7b8b091382f800826dbe03156f4d803`); initial worktree clean, no open PRs.
  At a=1,b=2,k=2 the displayed expansion gives 1 versus 0; at s=2 the original lemma's
  residual is log(2)/3>0, derived with convergence/nonvanishing and checked again via
  the missing Euler factor. Author testimony, executor reasoning and partial Lean
  support are distinct. New packet: `analysis/s1-first-parent-status/`.
  Actual Rust ingest generated and validated candidate `s1_ingest-v0.1.0-15ab865466d6`
  twice outside the live corpus. Its false/counterexample reference replaces the old
  source-document attribution prospectively; both embedded statuses agree. Owner ruling
  and integration remain pending. Old metadata, all S1.4 bytes and 34 representations /
  32 logical items remain unchanged. No canonical production or Gate S1 closure.
- **Previous update (S1.4 before PR 5 merge):** 2026-10-20 UTC — post-merge reconciliation and S1.4 completed for the
  existing prefix, with Gate S1 open. Owner merged PR 4 at 18:10:13Z; actual base/corpus
  `e6a7cae4392dd1565ee57f9fb727cc01e453687d`, tree
  `2e4d454048ac916a0047bd908ae3051ba1b981af`; initial worktree clean, no open PRs.
  Reporting implementation `4e46564ac62bc2bd7a5bef54265e9b2dd26772bb` was committed before
  final generation. Rust recomputed 34 representations / 32 roots / two canonical children,
  five observed classes / nine zero classes, 30 roots missing canonical length, one valid
  positive pair and 22 documentary source-array groups. Four report artifacts replayed
  byte-identically; fresh pair build and 11-file historical replay passed with all 20
  producer inputs unchanged. Evidence: `analysis/s1-4/`. Source labels are unchanged;
  the first-parent hold, seven truth and seven severity questions remain explicit.
- **Previous update (S1.2 before PR 4 merge):** 2026-10-20 UTC (S1.2 producer and proposed admission under ADR-0007.
  Source-map correction `d88fad2bb7b339c56bd9964167d3e0acddc8ee4e` precedes tested producer
  `eb1b96d5b8084ceb1de734fffc90c9b480ec32bf`. Actual generation, both-file and parent/child
  validation, emitted Lean/type/axiom audit, 11-file fresh replay and CLI refusal checks passed.
  Branch inventory: 32 raw + two canonical, proposing one additional independent positive item;
  accepted-main baseline remains 31 raw + one canonical, zero admitted S1.2 pairs.
  Draft review and admission remain pending; Gate S1 stays open. Evidence:
  `analysis/s1-2/implementation/`. First-parent hold and harvest resume are unchanged.)
- **Previous update (S1.2 procedure):** 2026-10-20 UTC (S1.2 procedure and first candidate review: ADR-0007;
  Judson 2025 Theorem 2.2.6 paired prospectively with `Nat.infinite_setOfPred_prime`
  at the existing Mathlib pin. Primary source/rights and statement correspondence reviewed;
  focused Lean/axiom and Rust pure-API checks passed. No pair admitted or producer built;
  31 raw parents plus one canonical representation, the separate promotion hold and harvest
  resume point are unchanged. Gate S1 remains open. Draft review packet: `analysis/s1-2/`.)
- **Previous update (S1.3):** 2026-10-20 UTC (S1.3 first canonical derivation: `syndeom-derive` 0.1.0;
  parent `s1_ingest-v0.1.0-00f28e252823`, child `s1_derive-v0.1.0-c08f9e3ec092`;
  producer committed first at `49e47237924eddf0c39c24aefad0eb9aa02ee8b3`, real generation
  and independent regeneration passed. Source text remains external; one separate parent
  promotion hold recorded. No harvest advancement. S1.2/S1.4 and Gate S1 remain open.)
- **Previous update (historical):** 2026-10-14 UTC (Session 20 — the ninth Route A harvest: candidates 25–27
  examined in order, all three admitted yielding 6 items — four from one published erratum (a
  main theorem withdrawn to a conjecture, the proposition and the assertion its proof needed,
  and a moduli theorem shown false by the erratum's own example), one true theorem whose proof
  invoked a comparison principle outside its hypotheses, one wrong degree shift; prefix through
  `1403.4485`; `groundtruth/` holds thirty-one `raw/1` items. Session 19 — candidates 22–24: all
  three admitted yielding 5 items (four undetermined). Session 18 — candidates 19–21: 2 admitted
  (3 items), 1 rejected (D8, a cumulative comment). Session 17 — candidates 16–18: all three
  admitted yielding 4 items (the third undetermined statement truth; the first
  `retraction`-kind item). Session 16 — candidates 13–15: 2 admitted (3
  items), 1 found-but-unclassifiable. Session 15 — candidates 11–12: 1 admitted (2 items, a
  journal correction with a counterexample figure), 1 found-but-unclassifiable. Session 14 — candidates 7–10: 3
  admitted (4 items), 1 rejected; the examination helper steps over withdrawn versions. Session 13 — candidates 3–6: 2 admitted
  (one with the first undetermined statement truth), 2 rejected. Session 12 recorded Al's D8
  precision. Session 11 — the first Route A harvest: finder built and
  run (2,472 candidates), candidates 1–2 examined in order and both admitted and ingested
  (`groundtruth/` now holds two `raw/1` items with records); S-0021..S-0023 registered for the
  position paper's §0 disclosure; memory hazard added. Session 10 recorded Al's rulings of
  2026-10-13 (ADR-0001 `/2`, ADR-0003 resolved, ADR-0005) and built the producer; Session 9
  decided ADR-0003; Session 8 closed R3; Session 7 recorded the earlier rulings)
- **Phase:** S2 — GENERATOR (entered 2026-10-26, Session 29, at Gate S1's closure; effective on
  `main` at the merge of Session 29's pull request). S1 — GROUND-TRUTH CORPUS is closed on the
  admitted prefix.
- **Gate:** **S2 OPEN** (Gate S2 exit criteria below; nothing evidenced yet).
  **Gate S1 CLOSED 2026-10-26 (Session 29):** the cold-start examination met every launch
  condition. It was a fresh conversation launched from `C:\Users\LEET` with the §6 Session-N
  prompt (ADR-0009 wording) verbatim and alone, and the SD15 disclosure came first. It re-derived
  the program state from the repository in one pass without guessing, and found the gate GREEN at
  `e029c80`; result **PASS**; record in HANDOFF Session 29, whose first entry is the disclosure.
  The PASS rests on three SD15 judgments stated for Al's audit (Standing escalations). Earlier
  attempts stay as recorded: Sessions 22 and 25 ineligible; Session 27 not cold; Session 28 not an
  examination. S1.1 closed on the admitted unskipped prefix through candidate 27 / `1403.4485`,
  not an exhausted harvest (ADR-0003 D8, Al's ruling (iii)). PR 4 admitted one positive
  statement-fidelity pair; PR 5 merged the frozen S1.4 report; PR 6 archived the
  witness/proposal; the owner accepted its exact ruling and PR 7 activated verified integration on
  main. The criterion-by-criterion readiness assessment above carries all qualifications. Harvest
  resume at candidate 28 / `1403.6317` remains saved; any later harvest is a later prefix under
  its own gate record and never rewrites the Gate S1 table (ADR-0003 D8). The closed S1 table is
  retained below as the record.
  **Gate S0 CLOSED 2026-10-13
  (Session 6):** the cold-start exam, run in a fresh conversation launched with the §6 prompt
  verbatim and alone, re-derived the program state from the repo in one pass without guessing and
  found the gate GREEN at `2b7c573`; result **PASS**; record in HANDOFF Session 6, whose first
  entry is the SD15 memory disclosure. **Al confirmed the PASS on 2026-10-13 (Session 10, ADR-0000
  Addendum 3): the closure is not provisional; no re-run.** The closed S0 table is retained below
  as the record.
- **Founding sweep:** `PROGRAM/SWEEP-2026-10.md` (41,060 bytes, SHA-256
  `f4bfb2adcdf7c6714101ad6ddc2fdcf528e98e7c993accb8a5dbe04a8115b03e`, ferried by Al in Session 3
  and verified against Al's stated digest before placement; recomputed in Session 6). It is a
  **map, not evidence** (SD11 banner on line 3): nothing enters any register from it; where it
  disagrees with the S0.1 register, the register stands (its own ferry note says so). It was
  redacted under SD8 before the ferry. Its ten ranked sources were retrieved and read in Session 5
  (CLAIM-0006 examination, done); its runners-up remain unretrieved (carried note below).
- **Advisory note:** `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md` (27,238 bytes, LF, SHA-256
  `f3e40b684a8b437fa4f0ba9d059d23d6ae61ce88ce8bca84ceb49fd7c181fa2c`, ferried by Al under ruling
  R3 of 2026-10-13 and verified against the stated digest before the commit, Session 8). A
  ChatGPT-produced design discussion of 2026-10-13 relayed verbatim under a provenance header,
  with Claude's verification record and its rights status under the R1 boundary (the DARQ
  MNEMOSYNE material in it is proprietary and outside SYNDEOM's release — R1(b), carried forward by
  ADR-0010; the program is named
  with its owner's authorization, Al, 2026-10-13). **SD11 banner: advisory, not evidence** —
  nothing enters any register from it; its two "recorded counterexamples" are not in the Mnemosyne
  repository (its header says so); its §6/§7 finding is Mnemosyne change control, not SYNDEOM's.
  Its only standing effect here is R4 (candidate S2 seed family, standing rulings below).
- **Default branch:** `main` — discovered from the clone of the empty remote and the REST API
  (ADR-0000 D3); confirmed by Al (Q2). Remote: `origin` = `https://github.com/Quigles1337/Syndeom`
  (**public**; created by Al 2026-10-12; Actions enabled).
  **`LICENSE` = PolyForm Noncommercial 1.0.0 since ADR-0010 (2026-10-26)**, preceded by
  `Required Notice: Copyright 2026 Alexander David Zalewski
  (https://github.com/Quigles1337/Syndeom)`:
  - copyright held personally by Alexander David Zalewski;
  - commercial licensing at adz@alphx.io;
  - text provenance S-0073;
  - the four crates' `license` fields set to `PolyForm-Noncommercial-1.0.0`.

  It was Apache-2.0 from Session 7 through `c9372b1` (Q3, ADR-0000 Addendum 2 R1; text S-0019),
  and copies obtained under Apache-2.0 remain under it. The release boundary (third-party
  material; DARQ Labs proprietary material) is carried forward and stated in README.md §
  Licence. **Since Session 31 (Al's rulings of 2026-10-27; ADR-0010 *Amendments*):**
  - `THIRD-PARTY-NOTICES.md` lists every file not under PolyForm, with its licence and
    attribution;
  - `LICENSES/Apache-2.0.txt` holds the Apache-2.0 text (S-0019 addendum) for the files under it.
  **ORIENT note (SD7):** `git symbolic-ref refs/remotes/origin/HEAD` resolves to `main`; if it ever
  fails, fallback `git remote show origin`, repair `git remote set-head origin -a`.
  **Fetch first (Session 22, 2026-10-21):** run `git fetch origin` before §4.1's `git status` /
  `git log`, then compare `main` with `origin/main` (`git status -sb`; `git log main..origin/main`;
  `gh pr list --state merged`): a working copy's remote ref is a claim until fetched, and on
  2026-10-21 a §6-launched session reconstructed a 24-commit-stale state through §4.1 without
  noticing and ran the superseded harvest NEXT. A diverged local commit goes to a branch, never a
  force-push. (Historical note; the binding procedure is ADR-0008, next bullet.)
- **GOVERNING ORIENTATION AMENDMENT — `PROGRAM/DECISIONS/ADR-0008.md` (accepted by Al,
  2026-10-21; decision 2 in `analysis/s1-orientation-repair/OWNER-DECISIONS.md`).** Before any
  program state is reconstructed or any NEXT is executed: verify the repository and remote
  identity; `git fetch --prune origin` (a failed fetch stops the session — no cached NEXT);
  discover the default branch from the remote; record the fetched tip's full SHA and tree, the
  checked-out SHA, tree and branch, the working-tree status and the ahead/behind commit lists;
  select the snapshot — a synchronized clean checkout is it, a clean behind-only checkout of the
  default branch is fast-forwarded (ff-only) and re-verified, anything ahead, diverged or dirty is
  left untouched and examined from an isolated worktree at the fetched tip; reconstruct only from
  that checkout; re-fetch and compare before every push or PR, and stop if the target moved.
  Never force-push, reset, discard or blind-merge. `python -I -X utf8
  scripts/orient_preflight.py orient [--sync | --worktree DIR] [--allow-dirty PROGRAM/HANDOFF.md]
  --receipt FILE` performs and records it (exit 0 snapshot established; 2 fetch failed; 3
  checkout not current; `recheck --baseline SHA` exits 4 when the target moved); the gate does
  not call it. For every session launched with the §6 Session-N prompt (ADR-0009, amending
  step 0), SD15's disclosure (ADR-0005) precedes the preflight, appended to HANDOFF without
  reading that file, under a provisional block header completed after step 5; its HANDOFF append
  is the one declared change, and it is never erased or rewritten; a synchronized clean workspace
  may be prepared by an informed session beforehand, but the fresh executor still performs its
  own disclosure and its own preflight and records the fetched SHA. ADR-0009 (2026-10-26)
  amended SD15, the §6 Session-N prompt and ADR-0008 step 0: the constitution's SHA-256 is now
  `110f5360af60075a8e799dce0dc235d1ef8785bd40b0b34c38fa53ddb1d5479c` (38,958 bytes), and the
  preservation baseline `89c198b0…8102` is historical. ADR-0005 is unchanged.
- **S1.3 REPRODUCTION AFTER ADR-0009 — two facts (ADR-0009 D5; Al's option-1 ruling of
  2026-10-26).** The effective-inventory reproduction (`cargo run --locked --manifest-path
  engine/Cargo.toml -p syndeom-ingest --example s1_effective_inventory -- <clean clone> <new
  output directory outside it>`; packet `analysis/s1-first-parent-integration/`) checks its
  accepted baseline byte for byte at the clone's HEAD. ADR-0009 amended the constitution, and the
  tool's `preserve()` now skips `PROGRAM/SYNDEOM-PROGRAM.md` beside `README.md` and
  `PROGRAM/STATE.md`, because the constitution changes only by ADR and never belonged in a
  data-reproduction byte check (source `engine/ingest/examples/support/effective.rs` SHA-256
  `516bd44e…9775` → `92a816bc…b30c`; nothing else in the tool changed). **(1) At main
  `3cd5558223294571dd85164eb0d2730cb1911637`** (that revision's tool) the reproduction prints
  `PASS: 35 archived representations / 32 effective logical items` and the retained
  `inventory.json` is reproduced **byte-identical** (SHA-256 `b626d1b5…1b803`). **(2) At HEAD**
  (ADR-0009's tree and later, unless a later change says otherwise) it prints the same `PASS`,
  and its `inventory.json` differs from the retained one **only** in
  `preservation.exact_files_checked`, **370 → 369** — the constitution is no longer counted among
  the exact files; every count and item is unchanged (SHA-256 `df3a2c7f…a7f5`). An examiner
  should expect exactly that one difference at HEAD; it is not a broken S1.3. Evidence, in
  scratch clones: R1 at `3cd5558` (`repro.log`) and R5 on ADR-0009's staged tree
  (`repro-R5.log`), both under `C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\`. The gate's
  `first_parent_amendment` tests, whose fixture reads the same baseline from the working tree,
  pass 15 of 15 with the fix; without it four of them failed (the first staged-tree gate, same
  root, `gate-staged.json`).
- **S1.3 REPRODUCTION AFTER ADR-0010 (2026-10-26, Session 30).** The tool gained three narrow
  exceptions (ADR-0010 D6): `LICENSE` is skipped, `PROGRAM/SOURCES.md` is append-only, and the
  four crate manifests may differ only in their `license` line. Tool source SHA-256: `92a816bc…b30c`
  → `49e6cd6d…1a67`. The gate test `engine/ingest/tests/relicense_exceptions.rs` (6 tests) shows
  each exception is narrow.
  - **At `3cd5558`, with that revision's tool,** the retained `inventory.json` is still reproduced
    byte-identical (`b626d1b5…1b803`).
  - **At HEAD** the reproduction prints the same `PASS: 35 archived representations / 32
    effective logical items`. Its `inventory.json` differs from the retained one **only** in
    `preservation`:
    - `exact_files_checked` **363** (370 retained, 369 after ADR-0009);
    - `append_only_prefixes_checked` gains `PROGRAM/SOURCES.md`;
    - the new field `license_line_only_files_checked` lists the four manifests.

    Every count and item is unchanged.
  - In `provenance.json`, `tool_source_sha256` changes for `effective.rs`,
    `engine/ingest/Cargo.toml` and `engine/manifest/Cargo.toml`.

  An examiner should expect exactly these differences. Measurements and logs are under
  `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\`.
- **S1.2 PAIR REPLAY — where it runs (ADR-0010 *Consequences*).**
  - **Replay from HEAD ended at ADR-0010.** The four crate manifests' `license` line changed, and
    `engine/pair/Cargo.toml` and `engine/manifest/Cargo.toml` are two of the producer's 20 pinned
    inputs. A `syndeom-pair` built at HEAD therefore embeds their new digests, and
    `verify_producer` refuses producer revision `eb1b96d`.
  - **Where replay runs:** any `main` revision from `e6a7cae4392dd1565ee57f9fb727cc01e453687d`
    (PR 4's merge, the first with the helper) through `c9372b1355a6d687dec6287f908639ac670db659`
    (the last before ADR-0010). Each descends from `eb1b96d` and holds its 20 inputs
    byte-identical (`git diff --quiet eb1b96d c9372b1 -- $(cat engine/pair/input-files.txt)` exits
    0). A checkout of `eb1b96d` itself holds the producer but not the helper, which was added in
    `0b30d78`.
  - **Procedure:**
    1. Clone `origin` into a new directory and `git checkout --detach
       c9372b1355a6d687dec6287f908639ac670db659` (or any revision in that range).
    2. In its `lean/`, establish the pinned Mathlib build (`lake exe cache get`, then `lake
       build`). The producer reads `lean/.lake/packages/mathlib` at the pinned revision and
       compiles the emitted file.
    3. From the checkout root, run `python -I -X utf8 analysis/s1-2/implementation/reproduce.py
       --root <that checkout> --producer eb1b96d5b8084ceb1de734fffc90c9b480ec32bf --output <new
       path outside the checkout>`. Omit `--artifact-root`. Add `--source-dir
       <retained-five-object-directory>` only if the fixed upstream objects are unavailable; the
       original retained set is at
       `C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\sources`.

    The helper builds the producer, verifies its inputs, generates into an isolated root,
    compiles and audits the emitted Lean file, replays into a second fresh root and runs the
    labelled controls (`analysis/s1-2/implementation/README.md`, "Reproduce into a new
    destination").
  - The 18 committed receipts that record the producer's input closure — 16 under
    `analysis/s1-2/implementation/evidence/production/receipts/` and 2 under
    `analysis/s1-4/evidence/` — are unaffected.
  - **Run once, exactly as written, at Al's direction (Session 30, 2026-10-27): PASS. No step
    needed correction.** Evidence is under the external root
    `C:\Users\LEET\Codex\Temp\syndeom-pair-replay-20261026\`: the procedure as written, the
    wrapper, the raw log `replay.raw.log` (SHA-256 `42383a2a…c04e`) and `replay-out/`.
    1. Fresh clone of `origin`, detached at `c9372b1`, tree `4caa81f0…`, clean (01:16:53Z–01:16:56Z).
    2. `lake exe cache get` (01:16:56Z–01:24:19Z) and `lake build` (01:24:19Z–01:30:05Z), both
       exit 0, about 13 min with the machine's Mathlib cache already populated.
    3. `reproduce.py` (01:30:05Z–01:31:14Z), exit 0.

    Its `verification-receipt.json` (SHA-256 `37d7037e…cc42`) reads `PASS: generation,
    actual-byte validation, emitted Lean audit, replay and controls; draft admission only`:
    - checkout `c9372b1…`, producer `eb1b96d…`; rustc 1.91.0; Lean 4.34.1;
    - all five source objects fetched over HTTP 200 and hash-verified (`--source-dir` not
      needed);
    - 33 commands, each with its expected exit; all 18 refusal controls refused (exit 2);
    - emitted-file audit PASS, standard axioms only;
    - fresh replay byte-identical (11 files).

    The 11 file digests equal those of the original production receipt
    (`analysis/s1-2/implementation/evidence/production/verification-receipt.json`), and the
    emitted `Proof.lean` equals the original (`038c70bf…`). The five generated metadata files
    (both manifests, both source maps, the emitted `Proof.lean`) are byte-identical to the
    admitted pair committed under `groundtruth/`. Replay from HEAD was not attempted;
    ADR-0010's statement that it is refused rests on the producer's code.
- **S1.3 orientation verification (2026-10-20):** existing Lean/Rust code passed the gate,
  with documentation reconciliation in progress; 29 existing tests passed, axioms `[propext]`.
  The new crate was enabled only after this gate. Its focused producer checks now pass
  37 tests (24 manifest + 5 ingest + 8 derivation), fmt and clippy. Logs: `analysis/s1-3/`.
  Real generation/reproduction passed from the producer commit. The final staged artifact/evidence
  tree passed the full gate; `analysis/s1-3/gate-final.txt` records exit 0 and GATE: GREEN.
- **Current build verification (Session 46).** The ORIENT gate ran on `main` `89cdcf4` (tree
  `246ae476…`, PR 31's merge) in the original checkout, with the disclosure append the only
  change. **GATE: GREEN**, 2026-10-29T01:41:26Z–01:42:17Z, exit 0.
  - Lean: the build (8,936 jobs) and the axiom audit, twelve lines within the standard three.
  - Rust: fmt, clippy and the tests pass.
  - Every tracked text file is LF. The registers and the freeze register (65 rows) pass.
  - The log is `gate-orient.log` (stdout and stderr merged; SHA-256 `d4c4406a…1e3c`, 17,649
    bytes) under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261029T0140Z\`.

  The pull request's head is gated on its committed tree. That result is recorded in the pull
  request and under the external root, not in the tree it certifies.
- **Previous build verification (Session 45).** The ORIENT gate ran on `main` `854a4b4` (tree
  `f2357f53…`, PR 30's merge) in the original checkout, with the disclosure append the only
  change. **GATE: GREEN**, 2026-10-29T00:32:24Z–00:33:14Z, exit 0.
  - Lean: the build (8,935 jobs) and the axiom audit, eleven lines within the standard three.
  - Rust: fmt, clippy and the tests pass.
  - Every tracked text file is LF. The registers and the freeze register (65 rows) pass.
  - The log is `gate-orient.log` (stdout and stderr merged; SHA-256 `c849e408…6487`, 17,548
    bytes) under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-060-20261029T0029Z\`.

  The pull request's head is gated on its committed tree. That result is recorded in the pull
  request and under the external root, not in the tree it certifies.
- **Previous build verification (Session 42).** The ORIENT gate ran on `main` `7a05e75` (tree
  `c95ea6fb…`, PR 27's merge) in the original checkout, with the disclosure append the only
  change. **GATE: GREEN**, 2026-10-28T19:58:16Z–19:59:07Z, exit 0.
  - Lean: the build (8,933 jobs) and the axiom audit, eight lines within the standard three.
  - Rust: fmt, clippy and the tests pass.
  - Every tracked text file is LF. The registers and the freeze register (65 rows) pass.
  - The log is `gate-orient.log` (stdout and stderr merged; SHA-256 `28fc78e5…3d47`, 17,329
    bytes) under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1956Z\`.

  The pull request's head is gated on its committed tree. That result is recorded in the pull
  request and under the external root, not in the tree it certifies.
- **Previous build verification (Session 39).** ORIENT gate on `main` `ef567ec` (tree
  `4bef2ccc…`, PR 24's merge) in the original checkout, with the disclosure append the only
  change: **GATE: GREEN**, 2026-10-28T15:53:14Z–16:02:55Z, exit 0, stderr empty. Lean 8,926 jobs,
  axioms `[propext]`; fmt, clippy and 89 Rust tests pass; 485 tracked files LF; registers and the
  freeze register (64 rows: 59 frozen, 5 append-only) ALL PASS. Raw log `gate-orient.raw.log`
  (SHA-256 `428ae8dd…18b7`) under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1553Z\`. The
  pull request's head is gated on its committed tree; that result is recorded in the pull request
  and under the external root, not in the tree it certifies.
- **Previous build verification (Session 38).** ORIENT gate on `main` `a793954` (tree
  `bfa2ce23…`, PR 23's head tree) in the original checkout, with the disclosure append the only
  change: **GATE: GREEN**, 2026-10-28T04:58:53Z–05:07:50Z, exit 0, stderr empty. Lean 8,926 jobs,
  axioms `[propext]`; fmt, clippy and the Rust tests pass; registers and freeze register (59 rows)
  ALL PASS. Raw log `gate-orient.raw.log` (SHA-256 `ae41b1c6…0791`) under
  `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028-claude-opus\`. The pull request's head is
  gated on its committed tree; that result is recorded in the pull request and under the external
  root, not in the tree it certifies.
- **Previous build verification (Session 37).** There was no separate ORIENT gate on `main`
  `c48705c`.
  - Its tree `a6b256840d764818c6d314259883967e0e10c7a4` is the tree Session 36 gated GREEN on PR
    22's head `fd8fa9e`: run 2026-10-27T22:10:04Z–22:19:22Z, recorded in `gate-committed-2.json`
    under `C:\Users\LEET\Codex\Temp\syndeom-adr-0012-freeze-20261027\`.
  - That run's raw log SHA-256 is `0ac3de25…739e`, re-hashed and equal this session.
  - Before the gate, `lean/.lake` (144,221 files, 7.2 GB) and `engine/target` (2,886 files) were
    copied from the original checkout into the worktree by robocopy, with no failures. These are
    untracked build products; the copy log is under this session's external root.
  - The worktree's Mathlib package `HEAD` is `d13f23b7…`, the `lake-manifest.json` pin.
  - The review's commit `a5144ce` was gated on its committed tree (tree `9c1b29ec…`), with a
    clean working tree before and after: **GATE: GREEN**, 2026-10-28T01:36:37Z–01:47:41Z.
    - Lean: 8,926 jobs, axioms `[propext]`. Rust: 89 tests.
    - 475 tracked files. Freeze register: 59 rows, ALL PASS.
    - Receipt `gate-committed.json`, raw log SHA-256 `002f7f67…f45c`, under the external root.
  - The rulings commit on top of it is gated on its own committed tree, with a clean working
    tree. That result is recorded in the pull request and under the external root
    (`gate-committed-2.json`), not in the tree it certifies.
- **Previous build verification (Session 36):** no separate ORIENT gate on main `f8d71b8`. Its
  tree `35bb1439f4d7ac4ca54734ca61192ec4a68b3ff1` is the tree Session 35 gated GREEN on PR 21's
  head `9dc0d1e` (run 2026-10-27T21:17:49Z–21:21:08Z, `gate-committed.json`, raw log SHA-256
  `6cba5569…bc28`, under `C:\Users\LEET\Codex\Temp\syndeom-adr-0012-20261027\`). During the work,
  `cargo fmt --check`, `cargo clippy -D warnings` and the ingest crate's tests (15 + 5 + 7)
  passed; `scripts/check_freeze.py` reported ALL PASS on the working tree (9 rows: 296 frozen
  files byte-identical, no additions; both append-only prefixes intact), in about one second;
  `scripts/check_freeze_test.py` passed 8 of 8; after Al's rulings the check reports ALL PASS
  on 59 rows (384 frozen files, five append-only prefixes) and the tests 8 of 8 in about three
  seconds. Each of this pull request's commits (the amendment, then Al's rulings on PR 22)
  receives its own full gate on its committed tree, with a clean working tree, recorded in the
  pull request and under `C:\Users\LEET\Codex\Temp\syndeom-adr-0012-freeze-20261027\`
  (`gate-committed.json`, `gate-committed-2.json`), not in the tree it certifies.
- **Session 35 build verification:** ORIENT gate on the checkout `C:\Users\LEET\Syndeom`
  at fetched `origin/main` `6d89eb2f8a138a53d824d035febffc93423e1e0d` (tree
  `93ded916fb8cbfaea21f7d2a2367f86415fe3fc9`) with one declared change, the SD15 disclosure
  appended to `PROGRAM/HANDOFF.md`: full gate, detached with a log, 2026-10-27T20:54:16Z–21:03:15Z,
  exit 0 / `GATE: GREEN` — Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 88 Rust tests,
  468 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 73 (raw log SHA-256
  `627ef560f579fa33f8e3e712bf5094b941b835149c8ea8691c277719a938a416`, under
  `C:\Users\LEET\Codex\Temp\syndeom-adr-0012-20261027\`). The gate's cargo phase compiled the
  pre-ADR-0012 sources: its 88 tests include the six-test suite ADR-0012 removes, which does not
  compile against the revised tool. The S1.3 reproduction was run twice in scratch clones, outside
  the gate: at `3cd5558` with that revision's tool (retained inventory byte-identical) and on this
  revision's tree with the revised tool (PASS; `preservation` object only) — ADR-0012
  *Measurements*. This pull request's commit receives its own full gate and reproduction on its
  committed tree, with a clean working tree, recorded in the pull request and under the same
  root, not in the tree it certifies.
- **Session 34 build verification:** ORIENT gate on the examined worktree at fetched
  `origin/main` `a741ebf956be0bd87ebf1b606f0b9c6df1853043` (tree
  `a5e7df5488530c560ab7ec9eb608037f92a03820`) with one declared change, the carried SD15
  disclosure appended to `PROGRAM/HANDOFF.md`: full gate, detached with a log, 2026-10-27T19:30:55Z–19:41:28Z, exit 0 / `GATE: GREEN` — Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 88 Rust tests, 467 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 73 (raw log SHA-256 `245980cdba01e84de7541471091b0b74a3a9b3b4b55156ec0a0364e06778067e`). Receipt and raw log are under the
  external root `C:\Users\LEET\Codex\Temp\syndeom-adr-0011-20261027\` (`gate-orient.raw.log`,
  `gate-orient.json`). The Rust reproduction was not rerun: this session changes only `PROGRAM/`
  text. Each of this pull request's commits (adding ADR-0011, then recording Al's rulings on PR
  20) receives its own full gate on its committed tree, with a clean working tree, recorded in
  the pull request and under the same root (`gate-committed.json`, `gate-committed-2.json`), not
  in the tree it certifies.
- **Session 33 build verification:** no separate ORIENT gate on main `99bdf03`. Its
  tree `c063cc6589fc4f5d7605690ae7103cbd87993f5c` is the tree Session 32 gated GREEN on PR 18's
  head `d1e4d75`:
  - run 2026-10-27T05:46:42Z–05:57:09Z, recorded in `gate-committed.json`;
  - raw log SHA-256 `b37f667f…c603`, under
    `C:\Users\LEET\Codex\Temp\syndeom-post-deposit-records-20261027\`.

  Each of this pull request's commits (adding `CITATION.cff`, then recording Al's rulings on it)
  receives a full gate on its committed tree, with a clean working tree. Those gates are recorded
  in the pull request and under `C:\Users\LEET\Codex\Temp\syndeom-citation-cff-20261027\`, not in
  the tree they certify.
- **Session 32 build verification:** no separate ORIENT gate on main `2018d5e`. Its
  tree `3d7889def475b2b2f30d8acfd3dee31589a06a6e` is the tree Session 31 gated GREEN on PR 17's
  head `a6e2038`:
  - run 2026-10-27T02:35:08Z–02:41:56Z, recorded in `gate-staged.json`;
  - raw log SHA-256 `833fde02…d951`, under
    `C:\Users\LEET\Codex\Temp\syndeom-registration-licence-20261027\`.

  The commit recording the deposit receives a full gate on its committed tree, with a clean
  working tree. That gate is recorded in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-post-deposit-records-20261027\`, not in the tree it
  certifies.
- **Session 31 build verification:** no separate ORIENT gate on main `368bcfb`. Its
  tree `d33402ca2bfa0efcee577b2855b0741c07cdbe6e` is the tree Session 30 gated GREEN on PR 16's
  head `0470684`:
  - run 2026-10-27T01:32:57Z–01:42:06Z, recorded in `gate-committed-2.json`;
  - raw log SHA-256 `dadddea6…7594`, under `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\`.

  The commit adding the notices receives a full gate on the exact tree it commits. That gate is
  recorded in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-registration-licence-20261027\`, not in the tree it
  certifies.
- **Session 30 build verification:** no separate ORIENT gate on main `c9372b1`. Its
  tree `4caa81f0c56416a02830d19d4a11eb45c9c4a9f4` is the tree Session 29 gated GREEN on its
  committed rulings commit `6b3dbda` (2026-10-27T00:00:33Z–00:08:28Z; `gate-committed.json`,
  raw log SHA-256 `437716ec…58f7`, under
  `C:\Users\LEET\Codex\Temp\syndeom-s1-cold-start-exam-20261026\`). During the work, `cargo
  test` for the preservation suites (15 + 6 tests), `cargo clippy -D warnings` and `cargo fmt
  --check` passed. The commit that adds ADR-0010 receives a full gate on its committed tree,
  recorded in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\`, not in the tree it certifies.
- **Session 29 build verification:** ORIENT gate on main
  `e029c80e1be149bef8fe6ceb76eed59c235ed8b4` (tree `67023777965d5747c5e3e5fe3f655825f658c046`)
  with one declared change, the unstaged SD15 disclosure appended to `PROGRAM/HANDOFF.md`: full
  gate, detached with a log, 2026-10-26T23:16:58Z–23:25:02Z, exit 0 / `GATE: GREEN` — Lean 8,926
  jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests, 462 tracked files, Q 17 / SD 17 /
  FLAW 14 / CLAIM 7 / S- 72. Receipt and raw log (SHA-256
  `04c556afd76e90e3aacf940de3376087e9394c2832ab15118f581f37fb620943`) are under the external
  root `C:\Users\LEET\Codex\Temp\syndeom-s1-cold-start-exam-20261026\` (`gate-orient.json`,
  `gate-orient.raw.log`). The S1.3 effective-inventory reproduction ran on a fresh clone of
  `origin` at `e029c80` (2026-10-26T23:25:45Z–23:25:50Z) and printed the expected `PASS`. Its
  `inventory.json` (SHA-256 `df3a2c7f…a7f5`) differs from the retained one only in
  `preservation.exact_files_checked` 370 → 369 (`repro-head/`, `repro-head.log` under the same
  root). The commit that adds this bullet and HANDOFF Session 29 receives its own full gate on
  its exact staged tree, recorded in its pull request and under the same root, not in the tree it
  certifies.
- **Session 28 build verification:** no separate ORIENT gate on main `0afbbd1`. Its
  tree `27106131f522357184171e19d0e0e51ba2f30d54` is the tree Session 27's staged-tree gate
  certified (2026-10-26T21:09:57Z–21:19:09Z, GREEN; `gate-staged.json`, raw log SHA-256
  `d6fd8942…79d1` re-hashed and matching, under
  `C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\`). The commit that adds this bullet and
  HANDOFF Session 28 receives its own full gate on its exact staged tree. That gate is recorded
  in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\`, not in the tree it
  certifies. No Rust reproduction was rerun: this session changes only `PROGRAM/STATE.md` and
  `PROGRAM/HANDOFF.md`.
- **Session 27 build verification:** no separate ORIENT gate on main `6e198e0`: its
  tree `4ad8422654cb56d9ffe68a717100df5af240a22e` is the tree Session 26's final staged-tree gate
  certified (2026-10-26T20:34:44Z–20:35:33Z, GREEN; `gate-final.json`, raw log SHA-256
  `b25710de…c96e` re-hashed and matching, under
  `C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\`). The commit that adds this bullet and
  HANDOFF Session 27 receives its own full gate on its exact staged tree, recorded in its pull
  request and under `C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\`, not in the tree it
  certifies. No Rust reproduction was rerun: this session changes only `PROGRAM/STATE.md` and
  `PROGRAM/HANDOFF.md`.
- **Session 26 build verification:** no separate ORIENT gate on main `3cd5558`: its
  tree `125b1ed56d8dcab8074119eb51fc3467dbb09dda` is the tree Session 25's final staged-tree gate
  certified (2026-10-26T19:44:55Z–19:45:36Z, GREEN; `gate-staged.json`, raw log SHA-256
  `b003ccee…054b` re-hashed and matching, under
  `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\`). The commit that adds ADR-0009,
  the amendments, this bullet and HANDOFF Session 26 receives its own full gate on its exact
  staged tree, recorded in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\`, not in the tree it certifies. The first
  staged-tree gate (2026-10-26T20:16:14Z–20:16:51Z, nothing committed) was RED: four
  `first_parent_amendment` tests rejected the amended constitution (`gate-staged.json`); the
  tool's one-line fix (ADR-0009 D5) followed Al's ruling. The S1.3 reproduction was rerun at
  `3cd5558` and on the amended tree (bullet near the top of this file); no other Rust
  reproduction was rerun, because only `PROGRAM/` text and that one tool line change.
- **Session 25 build verification:** main `535641227f408095f36a92000e808ed5b65e135c`
  (tree `1fdeda3eca982fbaec04ab2b1f8fdaaeb82c70ea`) with one declared change, the unstaged SD15
  disclosure appended to `PROGRAM/HANDOFF.md`: full gate, detached with a log,
  2026-10-26T19:36:57Z–19:41:40Z, exit 0 / `GATE: GREEN` — Lean 8,926 jobs, golden axioms
  `[propext]`, fmt/clippy, 82 Rust tests, 461 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 /
  S- 72. Receipt and raw log (SHA-256
  `a97599f8c585071fb9c7a1fa3773f27133a1ab0840d4603a4c47816b831fe2ff`) under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\` (`gate-orient.json`,
  `gate-orient.raw.log`). PR 10's second commit `3f0171a`, whose tree main carries, was gated
  GREEN by Session 24 (previous update above). The commit that adds this bullet and HANDOFF
  Session 25 receives its own full gate, recorded in its pull request and under the same
  external root, not in the tree it certifies. The Rust reproduction was not rerun: this session
  changes only `PROGRAM/STATE.md` and `PROGRAM/HANDOFF.md`.
- **PR 10 build verification:** PR 10's first commit
  `0801ecb52d4ec839bed90291527b11ca8c3e1281`, tree `78a30035d7137eb0a52ab200b3ba25406331c855`
  (main `ffb63a6` plus this session's first STATE edits): full gate on that exact staged tree,
  committed unchanged, detached with a log, 2026-10-26T19:02:21Z–19:12:29Z, exit 0 /
  `GATE: GREEN` — Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests, 461
  tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 72. Receipt and raw log (SHA-256
  `112787b8c0c5fca28b40926171e090c6f9f13788f0cee04dc39fb8e0b643bf13`) under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-cold-start-prep-20261026\` (`gate-staged.json`,
  `gate-staged.raw.log`). The Rust reproduction was not rerun: PR 10 changes only
  `PROGRAM/STATE.md` and `PROGRAM/HANDOFF.md`. The commit that adds this bullet and HANDOFF
  Session 24 receives its own full gate, recorded in PR 10 and under the same external root,
  not in the tree it certifies.
- **Historical build verification (PR 8 readiness):** clean merged main
  `1c56028532ea135603dbad5423d5555313bfd9ec`, tree
  `14b451866491dc4a2b866ec0b1d726d792385916`, full local gate **GREEN** at
  `2026-10-21T04:00:41.304180+00:00`: Lean 8,926 jobs, golden axioms `[propext]`,
  fmt/clippy, all 82 Rust tests, 428-file LF/BOM/register checks, exit 0.
  Actual [receipt](../analysis/s1-post-merge-readiness/evidence/main-gate.json) and
  [log](../analysis/s1-post-merge-readiness/evidence/main-gate.log).
  PR 7's tested head has the identical tree; both tested-head and main-push CI succeeded.
  Fresh existing-consumer reproduction passed at 2026-10-21T03:50:28.787156+00:00:
  inventory byte-identical to retained output, provenance bound to the merge, overwrite
  refused unchanged. Rust rechecked all accepted-baseline preservation conditions,
  including 35 S1.4 files, 47 witness/proposal files and 20 original pair inputs.
  The final reconciliation commit receives its own full gate and reproduction;
  exact head/tree, actual results and external receipts are recorded in its associated
  PR, outside the tree certified. No earlier gate substitutes for that final result.
  See [event and verification convention](../analysis/s1-post-merge-readiness/README.md).
- **Session 23 build verification (2026-10-21):** implementation commit
  `d4dd5da956a02bbb4895dc6f6e15d0e16bcf9f23` (tree `4a8155dfecfbe422781a832e6550b257c55c84f6`):
  full gate on that exact staged tree, detached with a log, 23:44:48Z–23:55:37Z, exit 0 /
  `GATE: GREEN` — Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests, 455
  tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 72; the unchanged Rust reproduction on the
  clean committed tree passed (23:58:50Z–23:58:54Z; inventory SHA-256 `b626d1b5…1b803` identical
  to the retained integration inventory; 35 archived representations / 32 effective logical
  items unchanged). Receipts and logs: `analysis/s1-orientation-repair/evidence/`. The recording
  commit that adds them with the event README and HANDOFF Session 23 receives its own gate and
  reproduction, recorded in its pull request and under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\`, not in the tree it certifies.
- **Session 22 build verification (2026-10-21):** clean `main` `26c2555` (PR 8's merge) full gate,
  detached with a log, 22:47:14Z–22:48:06Z: Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy,
  82 Rust tests, 447 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 72, exit 0 —
  `GATE: GREEN`; and again on the final tree of the Session-22 reconciliation commit before it was
  committed (detached run with a log immediately before the commit; its times are left out of this file to avoid a self-referential edit, `GATE: GREEN`).
- **Historical build verification (first-parent proposal):** clean-base `bash scripts/check.sh` at
  `1a76d9ac21cd5d2e4311f9164178327c0ca128bb`, tree
  `3b9028c0502add53ac646108132d89ba5f1e5666`, passed 2026-10-20T21:44:58Z:
  Lean build, golden axioms `[propext]`, fmt/clippy, all 67 Rust tests, 328 tracked files
  and registers. Both actual orientation runs are retained in the new packet; the second
  was an unnecessary retry while a sandbox process listing hid the first running process.
  The exact packet Lean file separately compiles with all eight audited theorems using
  `[propext, Classical.choice, Quot.sound]`; no complete analytic bridge is claimed.
  Candidate validation and two fresh generations passed. The final clean committed tree
  receives its own full gate, focused Lean audit and preservation check; actual revision,
  tree, results and external receipts belong in this task's draft PR, without a self-referential
  recording commit. Details and retained development failures: new packet `VERIFICATION.md`.
- **Historical build verification (S1.4):** full `bash scripts/check.sh` on the initial reporting implementation
  `9375af283e748e915ffc6e31df12f1bb81122e1d`, tree
  `96cf29f99656b039561170cc9356d0839f97a6ef`, ended **2026-10-20T20:23:14.733002+00:00**:
  exit 0 / GATE: GREEN; Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy clean,
  67 Rust tests including 16 new reporting integration tests; 301 tracked files and
  register checks passed. Receipt/log: `analysis/s1-4/evidence/gate-implementation.json`
  and `.txt`. Review then separated known statement-role exclusions from genuinely
  missing proof lengths; corrected tool `4e46564ac62bc2bd7a5bef54265e9b2dd26772bb`
  passed all 16 focused tests and clippy before two fresh final report runs.
  The final report/documentation commit is gated separately; its exact clean
  commit/tree and actual outcome are retained externally and in this task's draft PR,
  without editing the checked tree to embed its own hash. The earlier orientation run
  reached the Lean build/audit but failed fmt while untracked reporting drafts were being
  edited; it is retained as a failed development run, never a clean-base certificate.
- **Historical build-status record (before S1.4):** the gate is `bash scripts/check.sh` (ADR-0002 D6): `lake build` + axiom audit
  of the golden fixture + `cargo fmt --check` + `cargo clippy -D warnings` + `cargo test` + LF check
  + register cross-reference check; it prints `GATE: GREEN` on success. **Last verified GREEN:
  2026-10-20 06:28:55Z (S1.2 pre-generation producer tree)** — Lean 8,926 jobs,
  golden axioms `[propext]`, fmt/clippy clean, 51 tests, 203 tracked files,
  Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 68, exit 0. Exact unchanged tested tree
  `ad9cbee9a1939ddf3e0deb200a8afbe8db642d53` became producer
  `eb1b96d5b8084ceb1de734fffc90c9b480ec32bf` before generation.
  Logs/receipt: `analysis/s1-2/implementation/evidence/producer-checks/producer-gate-01.*`.
  Fresh actual emitted-file audit and full production verification passed at 06:34:11Z.
  The subsequent exact artifact/evidence commit gate is retained in an external receipt
  and the new draft PR without modifying that checked tree. **Previous GREEN:
  2026-10-20 05:17:18Z (S1.2 staged procedure/candidate packet)** — Lean 8,926 jobs,
  golden axioms `[propext]`, fmt/clippy clean, 37 tests, 172 tracked files;
  Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 68, exit 0. Tested index tree
  `1034c92f7724e9f86f590c3ae344c1d3291c9c71`; new log/receipt:
  `analysis/s1-2/evidence/gate-staged-packet.txt` / `gate-staged-packet.json`.
  Final recording-only edits are checked before commit; the final committed-tree gate
  is recorded in the unmerged draft PR without modifying the checked tree. **Previous GREEN:
  2026-10-20 04:58:19Z (S1.2 orientation, documentation drafting in progress)** — Lean 8,926
  jobs, golden axioms `[propext]`, fmt/clippy clean, 37 tests, 150 tracked files checked;
  registers Q 17, SD 17, FLAW 14, CLAIM 7, S- 68; exit 0. This run began on main `8adcb48`
  and finished with source/doc drafts; it is not a certification of the final packet tree.
  Receipt/log: `analysis/s1-2/evidence/gate-orientation.json` and `gate-orientation.txt`.
  The separate S1.2 candidate audit uses the standard three axioms; it is not the golden
  audit and does not admit a pair. **Previous GREEN:
  2026-10-20 04:26:20Z (PR 2 corrected documentation tree)** — Lean 8,926 jobs, golden axioms
  `[propext]`, fmt/clippy clean, 37 tests passed (24 manifest + 5 ingest + 8 derivation),
  149 tracked files checked; Q 17, SD 17, FLAW 14, CLAIM 7, S- 66; exit 0. Log:
  `analysis/s1-3/gate-pr2-review.txt`. HANDOFF retains main's exact historical prefix, followed
  by the original S1.3 block; its last NEXT and this file's single NEXT both name S1.2.
  The final recording commit is gated again before integration; PR 2 and the integration
  merge message record that exact checked revision/tree and result without rewriting the
  checked files. **Previous GREEN:
  2026-10-20 03:47:27Z (S1.3 staged artifact/evidence tree)** — Lean 8,926 jobs, golden axioms
  `[propext]`, fmt/clippy clean, 37 tests passed (24 manifest + 5 ingest + 8 derivation),
  147 tracked files checked, registers coherent (Q 17, SD 17, FLAW 14, CLAIM 7, S- 66),
  exit 0. Log: `analysis/s1-3/gate-final.txt`. Final recording-only edits also pass byte/register
  and diff checks before commit. **Previous GREEN:
  2026-10-20 03:10:57Z (Session 21, documentation branch)** — local gate after populating the
  pinned Mathlib cache: Lean 8,926 jobs, axioms `[propext]`, Rust fmt and clippy clean, 29/29
  tests, 122 tracked files checked, registers coherent; exit 0. Final recording edits also passed
  the byte/register and diff checks before the documentation commit.
  **Previous GREEN: 2026-10-14 21:48:44Z–21:50:35Z (Session 20, detached run with a log) on the Session-20 harvest
  tree with every edit staged, and again on the final tree before the harvest commit** — Lean `Build completed successfully (8926 jobs)`,
  axioms `[propext]`; Rust fmt and clippy clean, 29/29 tests (24 manifest + 5 ingest); 112 text files `i/lf` + 10 binaries;
  registers coherent (Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 65/65). Earlier GREEN runs:
  Session 4 (before the S0.6 commit), Session 5 (at `132788c`), Session 6 (at `2b7c573` and on its
  own tree), Session 7 (22:54:28Z–23:04:29Z, cold OS cache), Session 8 (23:21:33Z–23:31:44Z and
  23:32:45Z–23:33:19Z), Session 9 (23:35:52Z–23:36:23Z and 23:36:55Z–23:37:25Z) and Session 10's
  rulings commit (2026-10-14 00:07:19Z–00:17:54Z) and producer commit (00:43:55Z–00:52:27Z and
  00:54:48Z–00:55:45Z, detached) Session 11 (02:07:11Z–02:18:18Z and 02:19:15Z–02:21:08Z,
  detached) Session 12 (02:34:42Z–02:45:22Z and 02:46:06Z–02:46:35Z, detached) Session 13
  (03:01:46Z–03:02:24Z and 03:03:06Z–03:03:41Z, detached) Session 14 (03:47:34Z–03:48:31Z and
  03:49:23Z–03:49:58Z, detached) Session 15 (03:52:24Z–04:03:12Z cold, 03:58:38Z–04:03:12Z and
  the final-tree run, detached), Session 16 (04:52:05Z–05:02:27Z cold, 05:07:44Z–05:09:07Z and
  05:09:56Z–05:10:25Z, detached), Session 17 (08:08:51Z–08:18:18Z cold, 08:30:22Z–08:30:47Z
  and 08:30:57Z–08:31:23Z, detached), Session 18 (19:44:55Z–19:55:18Z cold, 20:05:35Z–20:09:55Z
  and 20:10:27Z–20:11:44Z, detached) and Session 19 (20:14:35Z–20:15:39Z, 20:35:33Z–20:43:22Z
  and 20:43:48Z–20:49:47Z, detached), each on its own tree before its commit. A run the harness
  killed for system memory (00:21:40Z, in the axiom audit) is not a GREEN run; since then the
  gate is run detached with a log (HANDOFF Session 10). **Standing instruction:** update this line at every GREEN run a HANDOFF entry
  records, so it never lags HANDOFF. **Timing:** about 30 s with a warm `lean/.lake` *and* a warm
  OS file cache; **about 10 min after a reboot even with `lean/.lake` warm** (Session 7: the axiom
  audit, `lake env lean Audit.lean`, loaded Mathlib's oleans from a cold disk cache, lean.exe
  above 1.4 GB — not a hang; run the gate in the background with a log); about 15 min on a fresh
  machine (toolchain install, nine clones, 8,908-file cache download) — a `lake build` that prints
  nothing for minutes under a pipe is the download, not a hang. The advisory CI
  (`.github/workflows/check.yml`) runs the same script and is never a required check; it
  completed `success` for `132788c` and `2b7c573`.
- **Toolchain (ADR-0002, Session 4):** Lean `leanprover/lean4:v4.34.1` (`lean/lean-toolchain`),
  which is Mathlib's own `lean-toolchain` at the pinned Mathlib commit
  `d13f23b723b8a846827a245b89c10fc7d3f11612` (tag `v4.34.1`; `lean/lakefile.toml` +
  `lean/lake-manifest.json`, eight transitive Lake deps locked) — the Q8 rule, chosen as the newest
  Mathlib release tag with a stable Lean (a load-bearing divergence from KOBER's v4.33.1/no-Mathlib).
  Rust `1.91.0` with clippy + rustfmt (`rust-toolchain.toml` at the root), workspace
  `engine/Cargo.toml`, crates `syndeom-manifest`, `syndeom-ingest`, `syndeom-derive`, `syndeom-pair`.
  `engine/Cargo.lock` lists 34 packages: the new local pair crate and the 26 unchanged baseline entries (24 dependencies
  plus two workspace crates), six new decompression dependencies (S-0066), and `syndeom-derive`.
  The former 25-entry sentence predated the ingest crate. JCS
  serializer `serde_json_canonicalizer 0.3.2`. CI advisory on `actions/*@v5`. Licences of
  everything pinned: S-0008. Machine inventory as of 2026-10-12 in `PROGRAM/SIBLING-MAP-2026-10.md`
  §8. For the record (S5, not a reason for any pin — SD8): the contact corpus `openai/math` happens
  to pin the same Lean and Mathlib commit (S-0001); the ADR-0002 criterion was fixed before that
  was compared.
- **Constitution:** `PROGRAM/SYNDEOM-PROGRAM.md` v1.0, ratified by Al 2026-10-12 (Session 2);
  **amended three times: ADR-0004 (2026-10-13, Session 7) — §2 S3.2, fooling length as equivalence to
  chance with a design-derived N_min; ADR-0005 (2026-10-13, Session 10) — SD15, count-only
  disclosure of unrelated memory index lines in a cold-start exam; ADR-0009 (2026-10-26, Session
  26) — SD15 and the §6 Session-N prompt, memory disclosure first in every session launched with
  that prompt (SHA-256 now `110f5360…479c`).** Rulings of record: the
  ADR-0000 addendum (Session 2, Q1–Q17), Addendum 2 (Al's rulings of 2026-10-13, R1–R4) and
  Addendum 3 (Al's rulings of 2026-10-13 on ADR-0003 (i)–(v) and the SD15 audit). ADR-only; the
  constitution's version string is unchanged because no rule assigns version numbers to
  amendments (noted in ADR-0004).
- **Taxonomy:** `PROGRAM/FLAWS.md` 0.1.0 ratified (FLAW-0001..0014).
- **Registers (unchanged in Session 6):** CLAIM-0001..0005 and 0007 VERIFIED (0007 existence
  only); CLAIM-0006 **CONDITIONAL** in a narrowed form (unfalsified as of 2026-10-13; bounding
  sources S-0009..S-0018; a claim of absence cannot become VERIFIED by retrieval; the scoped
  novelty statement is in CLAIMS and lifts §0.6 only in that scope). S-0001..S-0018 VERIFIED;
  INGESTED: S-0004, S-0005 (declarations, CC0 and CC BY-NC 4.0) and S-0010..S-0016 (seven CC BY 4.0
  papers under `sources/`, ~16 MB; all nine stored PDFs re-hashed to their register digests in
  Session 6); S-0007 RFC 8785, S-0009 and S-0018 (arXiv non-exclusive licence) digest-only; S-0008
  the toolchain licence record; S-0017 the `leanprover/comparator` repository. Later entries:
  S-0019 (the LICENSE text, INGESTED), S-0020 (Mann–Whitney 1947, digest-only), S-0021..S-0023
  (the Mathathon open letter, joint statement and event page — the position paper's §0
  disclosure, no CLAIM, digest-only, Session 11), S-0024..S-0027 (the Route A items'
  originals and notices, Session 11; S-0024 INGESTED under `groundtruth/`, the rest digest-only),
  S-0028..S-0031 (Route A items 3 and 4, Session 13, all digest-only), S-0032..S-0037 (Route A
  items 5–8, Session 14, all digest-only), S-0038..S-0039 (Route A items 9 and 10, Session 15,
  digest-only), S-0040..S-0043 (Route A items 11–13, Session 16, digest-only),
  S-0044..S-0049 (Route A items 14–17, Session 17, digest-only);
  S-0050..S-0053 (Route A items 18–20, Session 18, digest-only);
  S-0054..S-0059 (Route A items 21–25, Session 19, digest-only);
  S-0060..S-0065 (Route A items 26–31, Session 20, digest-only).
- **Manifest schema:** `syndeom-manifest/2` since 2026-10-13 (ADR-0001 amended, Session 10:
  `statement_status.value` admits `null` with `method: "undetermined"`, `null ⇒ proof_status
  invalid`; `/1` remains accepted and names the pre-amendment schema — the D8 goldens), ADR-0001
  (Session 3; ratified Session 4). Formats
  `syndeom-informal-md/1`, `syndeom-lean/1`, `raw/1` (archival); ids
  `<generator.id>-v<version>-<config_hash[0:12]>[-s<seed>]`; JCS + SHA-256; FRIEDMAN ADR-0004
  arithmetic by reference; contact artifacts carry `ground_truth: null`; real items get a derived
  canonical artifact per D4 (iii).

## Freeze register

Checked by the gate — `scripts/check_freeze.py`, with both directions tested by
`scripts/check_freeze_test.py` (ADR-0012 *Amendments* 1; Al's direction of 2026-10-27, and Al's
rulings 1–4 of 2026-10-27 on PR 22, transcribed verbatim in ADR-0012 *Amendments*). A **frozen**
scope (a directory ending in `/`, or one file) is byte-identical to its version at the freeze
point, with no file added or removed under it (ruling 2). An **append-only** file has both its
version at the freeze point and its version on the default branch (`refs/remotes/origin/HEAD` as
last fetched; ADR-0008) as exact prefixes. A freeze point is a tag or a full commit id; every
row's is the `gate-s1-registration` tag (`2018d5e79ea23fa200e560d628bec5e91edf0956`) — the S1
packets' by Al's direction, the append-only rows' by ruling 2, the rest by ruling 3 — except
the five rows of the first later prefix (Session 38), whose freeze point is the commit that adds
their directories, `d2569d61588368a60bbf7558fb9f0703f96e3a5b` (Al's ruling Q-H7 of 2026-10-28 on PR 23), and the
row of that prefix's inventory packet (Session 39), whose freeze point is the commit that holds the
regenerated packet, `bcdcdfa36cc6be8a74409d34d08ea6edabd3ac3d` (Al's ruling D of 2026-10-28 on PR
25). Every row
cites the record it derives from. A later prefix adds new item directories under `groundtruth/`
and new stored copies under `sources/` beside the rows below, which name one item directory or
one stored copy each, never the parent directory (ruling 3); a change to or removal of a frozen
file is a deliberate register edit under a ruling (ruling 3). Pull requests merge with merge
commits, never squash, so that a commit used as a freeze point stays in `main`'s history (Al's
ruling Q-H7 of 2026-10-28 on PR 23). The register is a claim; the gate's
run is the evidence. The table's rows must have exactly this column form, or the gate fails.

| Path | Kind | Freeze point | Derived from |
|---|---|---|---|
| `analysis/s1-2/` | frozen | `gate-s1-registration` | the S1.2 packet (121 files): Gate S1 table, S1.2 row; "the frozen packets under `analysis/`" left byte-identical (Standing escalations, examination step 0 bullet, ADR-0009 D4); Bounds under NEXT |
| `analysis/s1-3/` | frozen | `gate-s1-registration` | the S1.3 packet (17 files): "S1.3 orientation verification" bullet; the same frozen-packets records |
| `analysis/s1-4/` | frozen | `gate-s1-registration` | "the frozen S1.4 packet" / "the frozen S1.4 report": Gate S1 table, S1.4 row ("reproducible and frozen"); Gate bullet; Previous update (Session 22); Standing escalations ("never rewrites … the frozen S1.4 packet") |
| `analysis/s1-first-parent-status/` | frozen | `gate-s1-registration` | the first-parent proposal packet: "all 47 accepted packet files" byte-identical (Historical build verification, PR 8 readiness); Previous update (first-parent proposal) |
| `analysis/s1-first-parent-integration/` | frozen | `gate-s1-registration` | the integration packet and "the retained report" (Updated bullet, Session 35; `preservation-inputs.json` among the records left byte-identical, ADR-0009 D4) |
| `analysis/s1-post-merge-readiness/` | frozen | `gate-s1-registration` | "readiness packet … unchanged" (Previous update, Session 22); Historical build verification (PR 8 readiness) |
| `analysis/s1-orientation-repair/` | frozen | `gate-s1-registration` | the orientation-repair event packet: its README among the records left byte-identical (ADR-0009 D4); Previous update (Session 23) |
| `analysis/s1-later-prefix-1/` | frozen | `bcdcdfa36cc6be8a74409d34d08ea6edabd3ac3d` | the first later prefix's inventory packet (ADR-0013; 7 files: README, the pins, the overlay and the four outputs regenerated after Al's rulings); the freeze point is the commit that holds the regenerated packet; Al's ruling D of 2026-10-28 on PR 25 |
| `PROGRAM/HANDOFF.md` | append-only | `gate-s1-registration` | §3.6 append-only session log; "its HANDOFF append … is never erased or rewritten" (ADR-0008 bullet); "HANDOFF entries (append-only)" (examination step 0 bullet); freeze point by ruling 2 |
| `PROGRAM/SOURCES.md` | append-only | `gate-s1-registration` | "`PROGRAM/SOURCES.md` append-only (ADR-0010)" (S1.3 reproduction tool bullet); "SOURCES historical prefix unchanged" (Gate S1 table, licences row); freeze point by ruling 2 |
| `groundtruth/s1_derive-v0.1.0-c08f9e3ec092/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-003a35498654/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-00f28e252823/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-0833f61c748c/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-15ab865466d6/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); the effective representation of the amended item (first-parent status support bullet); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-1c4c9a7747d0/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-22ae000cb879/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-2d93a266558a/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-315546e87259/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-40e6c0da8821/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-448d7aefa486/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-45c3ee8319d5/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-486ef22f632b/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-5ba6c0f07557/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-63e23a952084/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); "stays immutable and remains the logical key" (first-parent status support bullet); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-685b874cf579/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-68ed1abfecb4/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-6ae03b18a1bd/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-72415457dce4/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-744652f4b95f/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-82b4e66d0016/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-a4822a4be06e/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-a8ac73896464/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-a93badadec27/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-be3b17e73425/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-cafb56cb27fb/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-da3a4784c2be/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-dc0e0b0d5b87/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-dc1e44f39bb9/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-e0c199d3c60a/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-e3cbdfa7ad1c/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-f634c259799f/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-fd2bfef40d31/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_pair-v0.1.0-62fe0c816a6b/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_pair_archive-v0.1.0-094625fa10ff/` | frozen | `gate-s1-registration` | accepted corpus item (35 archived representations / 32 effective logical items, unchanged since Session 23); Al's ruling 3 on PR 22 |
| `groundtruth/s1_ingest-v0.1.0-34988f7c8306/` | frozen | `d2569d61588368a60bbf7558fb9f0703f96e3a5b` | first later prefix, item 32 (candidate 28, arXiv 1403.6317 Theorem 1), integrated Session 38; the freeze point is the commit that adds the directory; Al's ruling Q-H7 of 2026-10-28 on PR 23 |
| `groundtruth/s1_ingest-v0.1.0-915251f36fc5/` | frozen | `d2569d61588368a60bbf7558fb9f0703f96e3a5b` | first later prefix, item 33 (candidate 29, arXiv 1403.8075 Proposition 4.1), integrated Session 38; the freeze point is the commit that adds the directory; Al's ruling Q-H7 of 2026-10-28 on PR 23 |
| `groundtruth/s1_ingest-v0.1.0-bca249b3fa72/` | frozen | `d2569d61588368a60bbf7558fb9f0703f96e3a5b` | first later prefix, item 34 (candidate 29, arXiv 1403.8075 Theorem 4.3), integrated Session 38; the freeze point is the commit that adds the directory; Al's ruling Q-H7 of 2026-10-28 on PR 23 |
| `groundtruth/s1_ingest-v0.1.0-c6179869edf7/` | frozen | `d2569d61588368a60bbf7558fb9f0703f96e3a5b` | first later prefix, item 35 (candidate 29, arXiv 1403.8075 Theorem 4.4), integrated Session 38; the freeze point is the commit that adds the directory; Al's ruling Q-H7 of 2026-10-28 on PR 23 |
| `groundtruth/s1_ingest-v0.1.0-11ffe4f87954/` | frozen | `d2569d61588368a60bbf7558fb9f0703f96e3a5b` | first later prefix, item 36 (candidate 30, arXiv 1404.4599 Proposition 2.22), integrated Session 38; the freeze point is the commit that adds the directory; Al's ruling Q-H7 of 2026-10-28 on PR 23 |
| `groundtruth/derivations/` | frozen | `gate-s1-registration` | the reviewed TeX derivation recipe (ADR-0006; S1.3 bullet); Al's ruling 3 on PR 22 |
| `groundtruth/HARVEST-LOG.md` | append-only | `gate-s1-registration` | the record of every candidate (ADR-0003 D7; Gate S1 table, S1.1 row); a later prefix appends rows (D8); Al's ruling 3 on PR 22 |
| `groundtruth/README.md` | append-only | `gate-s1-registration` | the corpus README, append-only in the S1.3 tool until ADR-0012; Al's ruling 3 on PR 22 |
| `PROGRAM/SWEEP-2026-10.md` | frozen | `gate-s1-registration` | Founding sweep bullet: 41,060 bytes, SHA-256 `f4bfb2ad…b03e`, verified against Al's stated digest; the tag's blob carries that digest; Al's ruling 3 on PR 22 |
| `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md` | frozen | `gate-s1-registration` | Advisory note bullet: 27,238 bytes, SHA-256 `f3e40b68…fa2c`, verified against the stated digest; the tag's blob carries that digest; Al's ruling 3 on PR 22 |
| `sources/arxiv-2501.03124v5-prmbench.pdf` | frozen | `gate-s1-registration` | stored copy of S-0010 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `sources/arxiv-2510.13744v1-hard2verify.pdf` | frozen | `gate-s1-registration` | stored copy of S-0011 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `sources/arxiv-2605.10141v1-formalrewardbench.pdf` | frozen | `gate-s1-registration` | stored copy of S-0014 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `sources/arxiv-2605.12524v1-proofgrid.pdf` | frozen | `gate-s1-registration` | stored copy of S-0013 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `sources/arxiv-2605.20531v3-pseudo-formalization.pdf` | frozen | `gate-s1-registration` | stored copy of S-0015 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `sources/arxiv-2606.29493v1-faults-in-our-formal-benchmarking.pdf` | frozen | `gate-s1-registration` | stored copy of S-0016 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `sources/arxiv-2607.11849v2-advancedmathbench.pdf` | frozen | `gate-s1-registration` | stored copy of S-0012 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `sources/leiden-declaration-2026-06-02-zenodo-20302944.pdf` | frozen | `gate-s1-registration` | stored copy of S-0005 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `sources/mathandai-severe-misalignment-2026-09-11-zenodo-22737751.pdf` | frozen | `gate-s1-registration` | stored copy of S-0004 with its register digest (SD11; Registers bullet, re-hashed Session 6); Al's ruling 3 on PR 22 |
| `PROGRAM/DECISIONS/ADR-0003.md` | append-only | `gate-s1-registration` | "appended ADR-0003" (first-parent status support bullet); its prefix pinned by the S1.3 tool until ADR-0012; Al's ruling 3 on PR 22 |

Ruled unchecked (Al's ruling 3 on PR 22, 2026-10-27 — "the scopes you did not propose stay
unchecked"): the 20 pair-producer inputs (pinned to `eb1b96d5…` for the replay range only; two
changed at ADR-0010 and two change at S2's first code commit — "S1.2 pair replay" bullet);
`PROGRAM/CLAIMS.md` and `PROGRAM/FLAWS.md` (append-only registers in §3's sense, but a CLAIM's
Status line and a FLAW example have been updated in place, so they are not byte-append-only); the
constitution, `LICENSE`, `LICENSES/`, `THIRD-PARTY-NOTICES.md` and `CITATION.cff` (change only by
ADR or by the deposit procedure); the other ADRs (Status lines and *Amendments* change);
`groundtruth/amendment-map.json` (not proposed; bound by the S1.3 reproduction's compiled copy,
ADR-0012 D1).

## Memory policy

SD15 adopts FRIEDMAN ADR-0003 D1–D3 and the conversation-context corollary; the corollary is
**ratified for SYNDEOM** (Q13, ADR-0000 addendum), cited from FRIEDMAN, not duplicated. The
executor's memory store holds, for SYNDEOM, the repo location and machine hazards only. Exam
sessions write no memory; the Session-6 exam reproduced its recalled memory verbatim as the first
entry of its HANDOFF block and recorded the memory file's SHA-256 there. **Disclosure convention
(ADR-0005, 2026-10-13):** an exam reproduces verbatim every index line naming SYNDEOM or a
sibling and the SYNDEOM memory file in full with its digest, and withholds unrelated programs'
index lines by count with a recorded name-search check, because this repository is public. The
memory file was trimmed in Session 10 on Al's order (the paragraph paraphrasing §0.3, §0.4 and
SD17 removed; hazards kept); its digest is in HANDOFF Session 10 beside the Session-6 one.
**Disclosure timing (ADR-0009, 2026-10-26):** every session launched with the §6 Session-N prompt
makes this disclosure first, whether or not it attempts a gate test, appended to HANDOFF without
reading that file; a session that reads program state before disclosing cannot be an examination;
such a session writes memory only once it has established that it is not an examination.
**Disclosure by digest (ADR-0005 *Amendments* 1; Al's ruling of 2026-10-27, Session 37).** When
every memory file's SHA-256 matches an earlier HANDOFF disclosure that reproduced the same bytes,
the disclosure records instead:
- the digests, line counts and byte facts;
- the matched index line numbers and the withheld count;
- a citation of that earlier entry.

Verbatim reproduction is owed only for a new digest. If it cannot be completed, the session stops
and escalates to Al.
**Refined, and SD15's wording ruled (ADR-0005 *Amendments* 2; Al's rulings of 2026-10-28 on PR
23).** The comparison is of two digests, not of the whole index's: the matched index lines' (each
with its LF, in index order) and the SYNDEOM memory file's. The whole index's digest is recorded
as a fact. Verbatim reproduction is owed only when the matched lines or the SYNDEOM file change.
Baseline: the matched lines (85, 86, 88, 89, 106, 107; 1,198 bytes) `0418e621…534c` and the
memory file `ce2ce6de…2cc0`, both reproduced in HANDOFF Session 35's Entry 1. ADR-0005's
amendments stand as an addendum; the constitution is not edited.
**Readings confirmed, and the header's date stated (ADR-0005 *Amendments* 3; Al's owner note of
2026-10-28).** Both readings of the comparison are Al's now: the matched lines' digest is over
each line with its LF, in index order, and a renumbering without a byte change is no change, with
the new numbers and the withheld count recorded. The provisional block header carries the UTC
date and is marked provisional; ADR-0005 now says so, because a session reads it before
disclosing and reads ADR-0008 only after.
**Launch directory (Session 28, 2026-10-26; recorded at Al's direction):** whenever Claude Code
is launched inside a git work tree, it attaches a Git status and history snapshot to the
conversation before the first action: the current branch, `git status` and the last five commit
subjects. A session launched inside the checkout `C:\Users\LEET\Syndeom` therefore holds program
state before it can disclose (SD15's "Git history or status"), so it **can never be an
examination**, whatever its prompt, memory or disclosure order.
The launch directory also selects the memory store the harness loads:
- inside the checkout, the key `C--Users-LEET-Syndeom`, whose memory directory is empty, so no
  index is loaded;
- from `C:\Users\LEET`, the machine-wide index that ADR-0005's disclosure handles.

`C:\Users\LEET` is not a git work tree (`git rev-parse` exits 128 there). Sessions 6, 7, 25 and
27 record the harness loading that index at start, so they were launched from it, and none
records a Git snapshot. The examination is launched from `C:\Users\LEET`, outside the checkout.
Its disclosure records what the harness attached before the first action. No constitution or ADR
text changes: this applies SD15 as written.
**Hazard line added after PR 14 (2026-10-26; Al's approval):** the launch-directory hazard
Session 28 proposed was appended to the executor's SYNDEOM memory file after PR 14 merged. The file
went from SHA-256 `163c7b06…2787` to `ce2ce6de…2cc0`; the old bytes are an exact prefix, and the
index is unchanged at `cd943842…fc69`. The record of the write is Session 28's external root
(`memory-change.txt`). Session 29 disclosed the new file in full, diffed it against Session 28's
reproduced copy (the only change is the appended five lines), and judged it a harness hazard, not
program state, subject to Al's audit (Standing escalations). Session 29, launched from
`C:\Users\LEET`, confirms the finding from the other side: the environment block read "Is a git
repository: false", no Git snapshot was attached, and the `C--Users-LEET` index loaded.

## Standing rulings (recorded; their ADRs come later)

- **S2 generator families:** the first family is deterministic — seeded Rust templates and
  grammars. Any LLM-assisted generator is a separate family requiring its own ADR under SD6 and is
  excluded from `battery-freeze-v1` unless that ADR exists (determinism; leakage risk that an LLM
  generator was trained on the contact corpus). ADR at S2.
- **S5 toolchains:** contact corpora carry their own Mathlib pins; S5 ingest records them and may
  need per-corpus toolchains. Note, not solve, until ingest (Q8). First data point recorded in
  S-0001.
- **ADR numbers:** ADR-0001 manifest schema — written (Session 3), ratified by Al (Session 4) with
  its *Amendments*; ADR-0002 toolchain pins, Lean project, Rust workspace, JCS crate, storage
  default, gate and CI — written (Session 4, S0.6), D8 records what was measured; **ADR-0003
  errata-and-retraction harvest procedure for `groundtruth/` (S1.1) — written Session 9**, D1–D9,
  five choices (i)–(v) ruled by Al 2026-10-13 (Session 10; Amendments); ADR-0004 constitution
  amendment to §2 S3.2 (Al's ruling R2 of 2026-10-13) — written Session 7; ADR-0005 constitution
  amendment to SD15 (count-only exam disclosure; Al's SD15 audit ruling of 2026-10-13) — written
  Session 10; ADR-0006 reviewed TeX derivation — written 2026-10-20 (S1.3);
  ADR-0007 first positive-pair procedure and candidate — written 2026-10-20 (S1.2);
  ADR-0008 orientation synchronization and snapshot selection, the accepted addendum governing
  §4.1 — written and accepted 2026-10-21 (Session 23); ADR-0009 constitution amendment to SD15
  and the §6 Session-N prompt (disclosure first) — written and accepted 2026-10-26 (Session 26);
  ADR-0010 relicense to PolyForm Noncommercial 1.0.0, with three preservation-check exceptions —
  written and accepted 2026-10-26 (Session 30); ADR-0011 the first generator family (S2.1) —
  written Session 34 (2026-10-27), decision only, accepted by Al's rulings 1–4 of 2026-10-27 on
  PR 20, in effect since its merge (2026-10-27T20:28:10Z); ADR-0012 the S1.3 effective-inventory
  reproduction's byte checks narrowed to its input set — written, implemented and measured
  Session 35 (2026-10-27), accepted at PR 21's merge (2026-10-27T21:36:26Z); **amended Session 36
  (*Amendments* 1: the invariance test and the freeze register; Al's rulings 1–4 of 2026-10-27 on
  PR 22 recorded; merged at PR 22, `c48705c`, 2026-10-27T23:06:37Z); ADR-0005 amended Session 37
  (*Amendments* 1: disclosure by digest, Al's ruling of 2026-10-27; *Amendments* 2: SD15's wording
  and the comparison refined, Al's rulings of 2026-10-28 on PR 23); ADR-0005 amended Session 39
  (*Amendments* 3: the two readings confirmed and the provisional header's UTC date, Al's owner
  note of 2026-10-28); ADR-0013 the first later prefix's inventory event — written, implemented
  and measured Session 39 (2026-10-28), accepted as amended by Al's rulings of 2026-10-28 on PR 25
  (*Amendments* 1), in effect at the merge; ADR-0011 amended Sessions 41–45 (*Amendments*: Al's
  owner note of 2026-10-28 and the `s2_template` 0.2.0, 0.3.0, 0.4.0, 0.5.0 and 0.6.0
  revisions, with Al's rulings on PRs 26, 27, 28, 29, 30 and 31); ADR-0014 is the next free
  number.**
- **S3 calibration ADR obligation for undetermined items (Al's ruling (ii), 2026-10-13):**
  items whose `statement_status.value` is `null` (`method: "undetermined"`, ADR-0001 as amended)
  **count in proof-flaw cells and are excluded from every cell that uses the truth dimension,
  the FLAW-0011 and FLAW-0012 composites included.** Stated in the S3 calibration ADR before any
  curve, beside the ADR-0004 obligations above.
- **S3 calibration ADR obligations (ADR-0004, 2026-10-13):** before any curve is computed it must
  (1) compute N_min by the method ADR-0004 fixes — the smallest balanced cell at which the
  equivalence test (95% bootstrap CI of the AUC inside [0.45, 0.55]) has 90% power at true AUC
  0.5, from the null-hypothesis variance of the Mann–Whitney statistic — and confirm the number by
  simulation against the bootstrap procedure actually used; (2) state how effective N is computed
  for items derived from one seed theorem (for example a cluster bootstrap by seed theorem).
  Cells that cannot reach N_min — human-referee cells under SD13, detectors needing a per-item
  Lean build across the full grid — are reported **underpowered** by design.
- **S2 seed family (Al's ruling R4 of 2026-10-13; ADR at S2):** a Mnemosyne-derived seed family —
  the static-reduction theorem DM-FM-THM-001 and the labelled variants the advisory note proposes —
  is a **candidate** hand-authored golden-fixture family for S2.4, beside the FLAWS.md examples. A
  pipeline test, not calibration data (one theorem, six variants). Its licence line is decided at
  S2 under the R1 release boundary (ADR-0000 Addendum 2). Nothing built yet. The program is named
  with its program owner's authorization (Al, 2026-10-13).
- **Real items and detectors (ADR-0001 D4, ratified Session 4):** `raw/1` is archival; detectors
  consume only `syndeom-informal-md/1` and `syndeom-lean/1`; every S1/S5 item to be scored gets a
  derived canonical artifact (deterministic producer, `config.derived_from` + `method`, parent's
  ground truth with locations re-expressed and argued in the S1.3 record); the `raw/1` parent
  remains the item of record.
- **Al's rulings of 2026-10-28 on PR 23 — verbatim** (the held harvest review, SD15's wording,
  disclosure by digest refined). Received in Session 37's conversation after PR #23 opened and
  transcribed verbatim from Al's message. The exact copy is `owner-rulings-pr23.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-held-harvest-review-20261028\` (1,854 bytes, SHA-256
  `0047396b604535ab17a4a2b0ddaa7dfa74b027d63c8ca2d69f03463aded8449b`). The text is also in
  HANDOFF Session 37's addendum, and the last two rulings are in ADR-0005 *Amendments* 2. Their
  effect is under *Standing escalations* and NEXT.

~~~text
Rulings on PR #23 (held harvest review):

Q-H1, item 32: false by source document, as recorded, so its recorded class stands. "The wrong theorem" belongs to the wording group main's items were read under; changing that standard would be a corpus-wide decision, not a one-item exception.
Q-H2 and Q-H3, items 34 and 35: gap, as recorded, by the entry-range practice main's items follow.
Q-H4: the unnamed results stay out as items and join the existing unnamed-result question.
Q-H5, item 36: undetermined, as recorded. "Overcome" speaks to the later paper's reworked concepts, not to Proposition 2.22 as stated.
Q-H6: integrate now, as the first later prefix, following section 4's plan.
Q-H7: the harvest log continues as a new dated section after the file's current end, with its own table header, rows and counts, keeping the tag's bytes as an exact prefix. The five new item directories get frozen rows, added in a second commit on the integration branch with the commit that adds the directories as their freeze point. Pull requests keep merging with merge commits, never squash, so that freeze point stays in main's history.
SD15 wording: ADR-0005's amendment stands as an addendum; the constitution is not edited, since every session reads ADR-0005 in full before disclosing.
Disclosure by digest, refined: compare the digest of the matched index lines and of the SYNDEOM memory file, not of the whole index. Record the whole index's digest as a fact; verbatim reproduction is owed only when the matched lines or the SYNDEOM file change.

Record these rulings verbatim where the program records rulings (STATE, HANDOFF, and ADR-0005 for the digest refinement), as a new commit on PR #23's branch (no force-push). Run the gate on the committed tree, fetch fresh before pushing, and stop for my review. The housekeeping waits until after the merge.
~~~

- **Al's ruling of 2026-10-28 on the inventory event's scope (Session 38).** Asked through the
  harness's question tool before any integration commit; the question, the three options and the
  answer are copied exactly in `owner-ruling-inventory-scope.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028-claude-opus\` (1,460 bytes, SHA-256
  `d4f1b6f1d3a5a0cc7c433954e57fb5114594999ed0f511858bc186f80c690180`). Al chose the first option,
  whose text was:

~~~text
Separate PR next (Recommended) — This PR is the integration as ruled: commit 1 holds items, SOURCES 0074–0080, records, the log section and STATE; commit 2 adds the five freeze rows. STATE records 40/37 as committed but not yet measured, and notes the pinned tool refuses at HEAD as D7 predicts. The inventory event (new tool revision + coverage run) becomes NEXT, in its own PR, before S2 step 4.
~~~

- **Al's owner note of 2026-10-28 (Session 39) — verbatim.** It followed the §6 prompt in
  Session 39's launching message. The copy is `owner-note-as-received.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1553Z\` (1,570 bytes, SHA-256
  `435a26e3d83837e03bf96576373fcee25edc470146eda09c6bd18e1e76d265a9`), transcribed from the
  message as rendered to the executor. It is also in HANDOFF Session 39, Entry 3. Items 2 and 3
  are recorded as ADR-0005 *Amendments* 3. Their effect is under *Standing escalations* and in
  the Updated bullet.

~~~text
Owner note, to act on only after you have disclosed and oriented, and to record verbatim in this session's records:
1. The new manifests' engine_commit is the commit the producer ran from, as main's existing items record it. REVIEW.md's "at the integration commit" could not be met literally, and this is accepted.
2. Both readings of the digest rule recorded in ADR-0005 are confirmed: the matched lines' digest is taken over the byte form Session 37 verified against Session 35's reproduction, and a renumbering of matched lines without a change to their bytes is no change, with the new numbers and the withheld count recorded.
3. ADR-0005 also states the provisional header's UTC date requirement, so a session meets it before it can read ADR-0008. Record this as an ADR-0005 amendment.
4. Session 38's conversation was closed by a forced Windows restart overnight on 2026-10-27/28, before its post-merge housekeeping could be done there. At the owner's direction on 2026-10-28, a fresh Claude Code conversation, not launched with the §6 prompt and not a numbered session, did the housekeeping: it found the checkout C:\Users\LEET\Syndeom clean on claude/s1-held-harvest-integration at c43f9f9 with one worktree and no restart damage, fetched, read the default branch name from git, switched to it, and fast-forwarded main from a793954 to ef567ec. It made no disclosure, changed no repository file, committed, pushed and deleted nothing, and opened no PR. Record this in STATE as Session 38's housekeeping, noting that it was not done in Session 38's conversation.
~~~

- **Al's rulings of 2026-10-28 on PR 25 (ADR-0013) — verbatim.** Received in Session 39's
  conversation after PR 25 opened. The copy is `owner-rulings-adr-0013.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1553Z\` (2,072 bytes, SHA-256
  `e8fbcac1155cab68240e69c86d0a65139a3e3c70a0ce9c362a577404861c3b1a`), transcribed from the
  message as rendered to the executor. The rulings are also in ADR-0013 *Amendments* 1 and in
  HANDOFF Session 39's addendum. Their effect is under *Standing escalations* and in the Updated
  bullet.

~~~text
Rulings on ADR-0013 (record them verbatim in ADR-0013, STATE and this session's HANDOFF block, as new commits on PR #25's branch; no force-push):

Flag A: accepted. One tool and one packet.

Flag B: accepted. The five prefix roots count as unflagged. An item is flagged by a recorded open issue; the admission review and my rulings Q-H1–Q-H5 closed every reading raised on items 32–36, and dispositions.json records that basis per item. A later issue enters through the overlay.

Flag C: accepted. The coverage table uses the effective labels with the first-parent amendment applied. The S1.4 table stays frozen history, and the packet names which labels it used and the amendment map.

D7, the population pin: amended. The population is pinned by D2's recorded inputs — the accepted integration's input set and the five prefix item directories, by name and byte hash, with the harvest log read to its pinned prefix — and by nothing else. Counts are measured outputs, recorded in the packet and frozen there; they are not conditions in the code. Remove the literal 40/37/38 comparisons from the tool's guard and from its messages, and the literal-count assertions from the tests, keeping the identity checks, the byte-identity test against the retained report, and any assertion derived from the recorded inputs. Take the "through candidate 30 / 1404.4599" scope from prefix-inputs.json or from the pinned harvest-log prefix, not from a constant. Rewrite D7 and D8 to match. The pinned ADR-0012 tool keeps its literals: it is frozen, and this ruling applies from this event on. Regenerate the packet, since provenance records the tool's sources, and confirm two runs are still byte-identical.

Flag D: on the branch, after the regeneration and before the merge, as PR #24 did: a final commit adds one frozen row for analysis/s1-later-prefix-1/ whose freeze point is the commit that holds the regenerated packet. I will merge with a merge commit so that commit's id survives.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

- **Al's direction of 2026-10-28 (Session 40) — verbatim.** It was received in Session 40's
  conversation after a safety classifier stopped one of the session's responses. It is also in
  HANDOFF Session 40, Entry 3, transcribed from the message as rendered to the executor.

~~~text
Continue in this session. Your disclosure, orientation and gate stand; redo nothing. Record the interruption in this session's HANDOFF block in one sentence — the step it happened at, and that nothing was written — and say in one sentence which step the stopped response was carrying out, without reproducing it. Then proceed: PLAN the one thing STATE names, ADR-0011's crate, and EXECUTE it on a fresh branch from main. If a classifier stops you again at any point, do not reword or retry: stop at the nearest point where the committed tree passes the gate, complete the header, record both interruptions, push the branch, open the PR, and report. Otherwise run the loop to a clean stop per §4.6 and stop for my review.
~~~

- **Al's rulings of 2026-10-28 on PR 26 (ADR-0011's crate) — verbatim.** They were received in
  Session 40's conversation after PR 26 opened.
  - The copy is `owner-rulings-pr26.txt` under
    `C:\Users\LEET\Codex\Temp\syndeom-pr26-rulings-20261028\` (2,281 bytes, SHA-256
    `f4f6e2b05f0bf39045079a621cadcd567eed3b007109cc858ee4020b4ccac267`), transcribed from the
    message as rendered to the executor.
  - The rulings are also in ADR-0011 *Amendments* and in HANDOFF Session 40's addendum. Their
    effect is under *Standing escalations*, in the Updated bullet and in NEXT.

~~~text
Rulings on PR #26 (record them verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

1. The gate change in 21f8359 is accepted: the seed witnesses are library modules, and the audit parses every #print axioms line against the standard three axioms with a count check.

2. Readings 1, 2, 3, 5, 6, 7, 8 and 9 are accepted as read.

3. Reading 4 is not accepted as read. Flaw counts are per instance, not per record: one injected flaw rendered in both files of a pair is one instance with two locations, and n_flaws_total and n_flaws_<NNNN> count instances. No current golden is a pair, so nothing regenerates now; the rule binds the next producer revision before the first pair golden, and ADR-0001's wording is clarified if it needs it.

4. Reading 10: the attribution is to be verified against retrieved vectors, not left unverified. An independent implementation from the published algorithms reproduces this crate's D4 vectors and matches the reference vectors in rand_xoshiro 0.3.0's tests (src/splitmix64.rs: seed 1477776061723855037, 50 outputs; src/xoshiro256starstar.rs: state 1, 2, 3, 4, 10 outputs; both recorded there as produced with Vigna's reference C at xoshiro.di.unimi.it). Retrieve those two vectors, register the source under SD11, add them as tests in engine/generate/tests/rng.rs (a state constructor for the stream may be needed for the xoshiro vector; it must change no output), and lift the "unverified" note in the tests header, the packet README and D4's conformance record.

5. Readings 11 and 12: accepted. A new seed bumps s2_template to 0.2.0 with the 0.1.0 goldens kept regenerable under the version they name, and 0.2.0 embeds its input hashes at build time as syndeom-pair does, closing FINDING-005.

6. Record correction: the packet README and HANDOFF say each formal golden is byte-identical to its hand-written probe. That holds for three of five; the two faithful probes carry a trailing #print axioms line the goldens do not. Correct the wording; the probes stay as they are.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review. I will merge with a merge commit: the goldens pin engine_commit 21f8359, which must stay a real commit on main.
~~~

- **Al's owner note of 2026-10-28 (Session 41) — verbatim.** It followed the §6 prompt in
  Session 41's launching message. The copy is `owner-note-as-received.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1815Z\` (504 bytes, SHA-256
  `a80498bc38ee1e643a5cc0fa29e58c33ecb016dddb308fc8cda9b740db4ea4f1`), transcribed from the
  message as rendered to the executor. It is also in HANDOFF Session 41, Entry 3, and in ADR-0011
  *Amendments* with 0.2.0's amendment. Its effect is under *Standing escalations* and in NEXT.

~~~text
Owner note, to act on only after you have disclosed and oriented, and to record verbatim in this session's records:
1. ADR-0001 stays unedited on flaw-count units: its registry names n_flaws_total and n_flaws_<NNNN> without a unit and already counts flaws_confirmed in instances, and the per-instance rule recorded in ADR-0011's amendments binds this producer.
2. For a pair artifact, an injected instance is confirmed only when both of its records pass their checks. Record this with 0.2.0's amendment.
~~~

- **Al's rulings of 2026-10-28 on PR 27 (`s2_template` 0.2.0) — verbatim.** Received in
  Session 41's conversation after PR 27 opened. The copy is `owner-rulings-pr27.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-s2-template-020-20261028T1830Z\` (1,443 bytes, SHA-256
  `c465f9e5cecb42677e958cb950f3249492c6d75251901ffa6fd343d7aceb6b8a`), transcribed from the
  message as rendered to the executor. The rulings are also in ADR-0011 *Amendments* and in
  HANDOFF Session 41's addendum. Their effect is under *Standing escalations* and in the Updated
  bullet.

~~~text
Rulings on PR #27 / s2_template 0.2.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 13, 15, 16, 17, 18, 19 and 20 are accepted as read.

Reading 14 (FLAW-0006's mode for the horses) is not accepted as shift-index. D5's modes {drop-base, shift-index, strong-weak} name no case that fits, as the amendment says, and shift-index asserts an index shift the horses argument does not contain: its induction is set up correctly, and the defect is that the overlap step is valid only for n+1 >= 3, leaving the first step (1 -> 2) unjustified -- the kind FLAWS.md's own prose names ("proves the step only for n above the base"). Extend D5's FLAW-0006 mode enum with a fourth value, step-above-base, defined as "the inductive step's argument is valid only for n above the base, so the base -> base+1 step is unjustified; failing_case names that first unjustified step." Relabel the horses record's dials.mode (in the seed and in all three horses goldens) to step-above-base, keep failing_case "n + 1 = 2", and regenerate so the 0.2.0 goldens share one engine_commit. The horses proof bytes do not change (the mode is descriptive for intrinsic material); only the manifests do. Confirm two runs are byte-identical and report the engine_commit the regenerated goldens name.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

- **Al's rulings of 2026-10-28 on PR 28 (`s2_template` 0.3.0) — verbatim.** Received in
  Session 42's conversation after PR 28 opened. The copy is `owner-rulings-pr28.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-s2-template-030-20261028T2005Z\` (764 bytes, SHA-256
  `798ec70baacbd1d7716cadcbeac39213815878afc7c766123709e8194368e8ed`), transcribed from the
  message as rendered to the executor. The rulings are also in ADR-0011 *Amendments* and in
  HANDOFF Session 42's addendum. Their effect is under *Standing escalations* and in the Updated
  bullet.

~~~text
Rulings on PR #28 / s2_template 0.3.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 21, 22, 23, 24, 25, 26, 27 and 28 are accepted as read.

On reading 24, record also: FLAWS.md's FLAW-0009 recipe ("an axiom of the same statement; dial true | false") is read as same statement for truth: true, and for truth: false a false statement from which the main theorem still follows, with a witness module proving its negation; the record carries the replaced lemma and the inserted axiom so the modality mismatch in a pair is explicit in the ground truth. FLAWS.md's text is left as it is.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

- **Al's rulings of 2026-10-28 on PR 29 (`s2_template` 0.4.0) — verbatim.** Received in
  Session 43's conversation after PR 29 opened. The copy is `owner-rulings-pr29.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-s2-template-040-20261028T2136Z\` (696 bytes, SHA-256
  `b9e05c65bbcdb85d7ec9c0ef7ef139c05101bbf7e7a6ea97b6abd34562f8fec7`), transcribed from the
  message as rendered to the executor. The rulings are also in ADR-0011 *Amendments* and in
  HANDOFF Session 43's addendum. Their effect is under *Standing escalations* and in the Updated
  bullet.

~~~text
Rulings on PR #29 / s2_template 0.4.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 29 through 38 are accepted as read.

On reading 30, record also: the integer count is the generation decision because D4 forbids a floating-point value there; padding_ratio is a commanded statistic measured from the emitted file and checked at abs 0.05, and a ratio the count cannot reach is a FAIL verdict. A ratio-targeted mode, if a later version wants one, must choose its count by exact integer comparison of token quotients.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~
- **Al's rulings of 2026-10-28 on PR 30 (`s2_template` 0.5.0) — verbatim.** Received in
  Session 44's conversation after PR 30 opened (2026-10-29 UTC). The copy is
  `owner-rulings-pr30.txt` under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-050-20261028T2330Z\`
  (1,248 bytes, SHA-256 `8907df8358260ff8da2ce8d2b7db9b30b63516816a122b0793063983ed3be618`),
  transcribed from the message as rendered to the executor. The rulings are also in ADR-0011
  *Amendments* and in HANDOFF Session 44's addendum. Their effect is under *Standing
  escalations* and in the Updated bullet.

~~~text
Rulings on PR #30 / s2_template 0.5.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as new commits on the branch; no force-push):

1. Readings 39, 40, 41, 42, 44, 45, 47, 48, 49, 50 and 51 are accepted as read.

2. Reading 43: D7 stands. FLAW-0011 requires at least one embedded instance of severity gap or fatal; a composite whose embedded instances are all cosmetic is not an invalid proof and is refused, with a test. Under D7, FLAWS.md's FLAW-0011 example embeds a cosmetic instance and therefore realizes FLAW-0004 routine-true, which the 0.5.0 goldens already hold; record that as an erratum note against FLAWS.md 0.1.0 and leave FLAWS.md's text as it is. Replace the two FLAW-0011 goldens with a composite embedding one gap-or-fatal instance from a seed's existing material (your choice), informal and pair, from two byte-identical runs, and regenerate all 0.5.0 goldens at the fix commit so they name one engine_commit; the FLAW-0004 proof bytes must not change.

3. Reading 46: a composite embeds instances of FLAW-0001 through FLAW-0010 only, by FLAW-0011's definition; FLAW-0012 and FLAW-0013 are never embedded.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~
- **Al's rulings of 2026-10-28 on PR 31 (`s2_template` 0.6.0) — verbatim.** Received in
  Session 45's conversation after PR 31 opened (2026-10-29 UTC). The copy is
  `owner-rulings-pr31.txt` under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-060-20261029T0029Z\\`
  (793 bytes, SHA-256 `fe937f3eae7cb0d4f250df5e001e76bd922ae4c3fe40b15e8f35523f52f8748a`),
  transcribed from the message as rendered to the executor. The rulings are also in ADR-0011
  *Amendments* and in HANDOFF Session 45's addendum. Their effect is under *Standing
  escalations* and in the Updated bullet.

~~~text
Rulings on PR #31 / s2_template 0.6.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 52 through 65 are accepted as read.

On reading 52, record also: FLAW-0010 a's references are placeholder-shaped by D5's design, so S3 reads detection rates for that class as rates against obviously fictional references; a plausible-shaped lexicon, if one is ever wanted, is a later producer version and never attaches a real name to a fictional work.

On reading 61, record also: the cycle pair's formal file is a valid proof by construction; the instance is informal-only, and the pair is invalid under D7's either-file rule.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

- **Al's rulings of 2026-10-28 on PR 32 (`s2_template` 0.7.0) — verbatim.** Received in
  Session 46's conversation after PR 32 opened. The copy is `owner-rulings-pr32.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-s2-template-070-20261029T0220Z\` (883 bytes, SHA-256
  `f8fc35725e468c98df398dfd5fac8a7fbaea03a897229c2d6b799db73e52d6cb`), transcribed from the
  message as rendered to the executor. The rulings are also in ADR-0011 *Amendments* and in
  HANDOFF Session 46's addendum. Their effect is under *Standing escalations* and in the Updated
  bullet.

~~~text
Rulings on PR #32 / s2_template 0.7.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 66 through 84 are accepted as read.

On reading 68, record also, as an erratum note against FLAWS.md 0.1.0: the FLAW-0008 example applies two transformations where the recipe says exactly one, and its changed conclusion is false without the added hypothesis; under D5 the example realizes as the add-hypothesis drift with the example's proof route. FLAWS.md's text is left as it is.

On reading 73, record also: a linter warning is neither a kernel rejection nor an observation this program records; whether an unused-hypothesis lint enters the battery, and with it the observation schema, is for the S3 detector ADR.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

## Carried from S0 (notes, not actions)

- **Sweep runners-up unretrieved:** IMProofBench, Open Proof Corpus, FormalAlign, Beyond
  Compilation, ShadowBench, Sadasivan et al., First Proof batches, Greiffenhagen 2024. They stay
  outside the register until a later action retrieves them; the narrowed CLAIM-0006 scope says so.
  Retrieve one only when an S1 item or a later claim needs it; Gate S0's closure did not close
  this.

## Gate S2 exit criteria — OPEN (opened 2026-10-26, Session 29)

| Criterion | Status | Evidence |
|---|---|---|
| S2.1 Modules behind one shared interface: informal generator, Lean 4 formal-variant generator, flaw injector (every FLAWS.md recipe as a dial: presence, location, severity), padding module (FLAW-0013); first family deterministic (standing ruling) | **ADR-0011 written (Session 34, 2026-10-27; decision only, no code) and accepted by Al's rulings 1–4 of 2026-10-27 on PR 20, in effect at the merge; D6's S1-corpus independence rule binds every seed; the byte-check narrowing precedes any S2 code. Session 40 (merged as PR 26): the crate `engine/generate/` — the plan, both renderers, the injector, `syndeom-rng/1`, the Lean harness and `s2_template` 0.1.0 with three seeds. Session 41 (merged as PR 27): `s2_template` 0.2.0 — four more seeds from the FLAWS.md examples (FLAW-0002 gap, FLAW-0005 violated, FLAW-0006 and FLAW-0014 fatal), three of them seeds whose statement is false (no control, intrinsic material, negation witnesses). Session 42 (merged as PR 28): `s2_template` 0.3.0 — lemma steps in the plan and the seed `n_lt_two_pow` (FLAW-0009 `axiom` gap and fatal, FLAW-0007 `unused` cosmetic). Session 43 (merged as PR 29): `s2_template` 0.4.0 — the padding module (FLAW-0013: whole paragraphs of true irrelevant material with matching unused declarations; dials `kinds`, `count`, `placement`; one instance per paragraph) and the seed `sqrt2_irrational` with seven padding paragraphs. Session 44 (merged as PR 30): `s2_template` 0.5.0 — FLAW-0004 (the hedge drawn from the fixed lexicon, the justification deleted, the aligned proof block omitted; `hidden_step`) on `sqrt2_irrational` and FLAW-0011 as a commanded composite (`embedded`, one instance of severity gap or fatal of FLAW-0001 through FLAW-0010, Al's rulings 2 and 3 on PR 30; its record at the statement). Session 45 (merged as PR 31): `s2_template` 0.6.0 — FLAW-0010 `a` (the fictional reference drawn from the fixed lexicon and prefixed to the bare claim; `shape`, `claim`; the formal file unchanged, `N/A`) on `sqrt2_irrational` and FLAW-0003 on the new seed `bounded_monotone_converges` (`direct`: the goal or its contrapositive as Lemma 1 proved by a forward reference to the theorem, check K; `cycle`: Lemma 1 and the theorem citing each other, informal-only, checked on the plan's premise graph — the plan gains `premises`, D2). Session 46 (merged as PR 32): `s2_template` 0.7.0 — FLAW-0008 on the new seed `bolzano_weierstrass` (the formal statement drifts by exactly one recorded edit for each of D5's five transformations while the informal file stays faithful; the drifted file builds with its own proof, `NO`; the record formal-only at the main declaration's header line) and FLAW-0012 on the new seed `bounded_continuous_sup` (a false statement's convincing argument, the true-case proof transplanted; the FLAW-0005 `violated` instance at the hidden use site, check K, and the class's own instance at the statement); every FLAWS.md 0.1.0 class is now realizable by `s2_template`. Session 47 (PR pending): `s2_grammar` 0.1.0 — the family's second producer: the grammar `ring_chain` (a polynomial identity over ℤ in `x` and `y`, a seed form drawn from a fixed table and `n_steps` presentations of its expansion, proved as a `calc` chain closed by `ring`, with `n_lemmas` steps stated as lemma steps and cited; the truth's reference the formal control's artifact id) with its control and FLAW-0001 `propagated` and `local` (generated material: one perturbed monomial at a drawn ring-closed step, the site re-rendered under its own presentation), and the producer dispatch (`generate` by the config's `theorem`/`grammar` key; one `assemble` for both producers); the grammars `ineq_chain`, `case_split` and `nat_induction`, and the grammar's other v0.1 classes, are later versions** | `PROGRAM/DECISIONS/ADR-0011.md` (D1–D12, *Amendments*); `engine/generate/`; `analysis/s2-generate-1/README.md`; `analysis/s2-generate-2/README.md`; `analysis/s2-generate-3/README.md`; `analysis/s2-generate-4/README.md`; `analysis/s2-generate-5/README.md`; `analysis/s2-generate-6/README.md`; `analysis/s2-generate-7/README.md`; `analysis/s2-generate-8/README.md` |
| S2.2 Commanded and achieved statistics (tokens, steps, lemmas, flaws per class, severity, padding ratio); per-artifact fidelity verdict | **in progress: implemented for `s2_template` 0.1.0 (Session 40, PR 26), 0.2.0 (Session 41, PR 27), 0.3.0 (Session 42, PR 28), 0.4.0 (Session 43, PR 29), 0.5.0 (Session 44, PR 30), 0.6.0 (Session 45, PR 31) and 0.7.0 (Session 46, PR 32), and for `s2_grammar` 0.1.0 (Session 47, PR pending) — commanded/achieved per ADR-0011 D8, check K for formal `YES-KERNEL` records, the structural check for informal records; from 0.2.0 the counts are per instance and a pair's instance is confirmed only when both of its records pass (ruling 3 on PR 26; Al's owner note of 2026-10-28); from 0.3.0 check T for formal `YES-TOOLING` records (FLAW-0007 `unused` by the used-constants closure over the file's own constants, FLAW-0009 by `#print axioms`) and records only in the files a class manifests in; from 0.4.0 check T for FLAW-0013's padding declarations and `padding_ratio` measured as the FLAW-0013 span tokens over `n_tokens` (commanded at `abs 0.05`; a ratio the paragraph count cannot reach is a FAIL verdict); from 0.5.0 the FLAW-0004 informal check (the step is the hedge entry, a space and the bare claim; the faithful text held the justification exactly once) and the composite's check (the truth witness by construction, every embedded instance's records passing); from 0.6.0 the FLAW-0010 check (the span is the reference rebuilt from the recorded lexicon indices, every token in the lexicon, the step `By <reference>` and the bare claim, the faithful step holding the justification once) and the FLAW-0003 `cycle` check (the plan's premise graph contains the recorded cycle through the inserted lemma; each citation phrase at its recorded span), `direct` by check K; from 0.7.0 the FLAW-0008 check (the drifted file builds, the faithful statement has its kernel-checked witness, the recorded faithful and drifted statements differ by exactly the transformation's edit under one textual rule per transformation, the plan states the drifted one) and the FLAW-0012 check (every record of the use-site instance passes — K at the hidden use site — and the seed's negation witness is kernel-checked); for `s2_grammar` 0.1.0, `n_steps` commanded `exact` as the plan's step count (`length.n_steps + length.n_lemmas`) and `n_lemmas` as `length.n_lemmas`, `n_tokens` achieved only, and FLAW-0001 confirmed by check K at the site's `calc` line** | `engine/generate/src/template.rs`; `engine/generate/src/grammar.rs`; `analysis/s2-generate-3/README.md` (readings 21–28 accepted by Al's rulings on PR 28); `analysis/s2-generate-4/README.md` (readings 29–38 accepted by Al's rulings on PR 29); `analysis/s2-generate-5/README.md` (readings 39–51 ruled by Al's rulings on PR 30: 43 and 46 replaced, the rest accepted); `analysis/s2-generate-6/README.md` (readings 52–65 accepted by Al's rulings on PR 31); `analysis/s2-generate-7/README.md` (readings 66–84 accepted by Al's rulings on PR 32); `analysis/s2-generate-8/README.md` (readings 85–99 flagged for Al) |
| S2.3 Manifests and determinism for every artifact (SD1, SD6); `corpus_id` invariant tested | **in progress: every manifest self-checked before writing; determinism and id tests; `regenerate` reproduces `content_hash`; from 0.2.0 the running binary's compiled input hashes, the engine commit's blobs and the checkout must agree on every input (FINDING-005 closed, Session 41); 79 goldens regenerate under the producer and version their manifests name; `verify` refuses a manifest whose `main_declaration` is not the producer's (FINDING-011, Session 42)** | `engine/generate/tests/properties.rs`, `tests/goldens.rs`, `tests/provenance.rs`; `engine/generate/src/provenance.rs`, `build.rs`, `input-set.txt` |
| S2.4 Verification: property tests; goldens against the FLAWS.md examples; `cargo clippy` deny-warnings; `cargo audit` | **in progress: property and refusal tests; Golden A/B writer test; 79 goldens — 69 for twelve seeds (12 of 0.1.0, 14 of 0.2.0, 9 of 0.3.0, 8 of 0.4.0, 4 of 0.5.0, 12 of 0.6.0, 10 of 0.7.0, each pair counted once) and 10 for the grammar `ring_chain` (`s2_grammar` 0.1.0: the controls at two lengths and FLAW-0001); clippy clean; `cargo audit` clean (advisory-db `40f5be6f…`, Sessions 40–47); security reviews recorded (packets 1–8); the RNG verified against published reference vectors (S-0081; ruling 4)** | `engine/generate/tests/`; `analysis/s2-generate-1/`; `analysis/s2-generate-2/`; `analysis/s2-generate-3/`; `analysis/s2-generate-4/`; `analysis/s2-generate-5/`; `analysis/s2-generate-6/`; `analysis/s2-generate-7/`; `analysis/s2-generate-8/` |
| Fidelity sweep report (commanded vs achieved flaw presence across the parameter grid, tolerances stated) | **open** | — |
| Goldens green; lint and audit clean | **open** | — |
| Cold-start test (§2, SD15) | **open** | — |

## Gate S1 exit criteria — CLOSED 2026-10-26 (Session 29)

| Criterion | Status | Evidence |
|---|---|---|
| S1.1 Errata and retractions: per item the original, the notice, the flaw's location, its FLAWS.md class argued in writing; derivation `documented-by-source`; licence per SD16 | **evidenced for the admitted unskipped prefix; harvest not exhausted — prefix through arXiv id `1403.4485` (candidates 1–27 of 2,472; Sessions 11, 13, 14, 15, 16, 17, 18, 19, 20): 27 examined, 21 admitted yielding 31 items, 4 rejected (two D2.1 — mis-stated quoted theorems with no proof offered; two D8 — one correcting version after the window, one cumulative comment whose only correction predates it), 2 found-but-unclassifiable (withdrawal stubs naming no describable error, one of them superseded by a paper that is silent on the mistake); `groundtruth/HARVEST-LOG.md` is the record of every candidate; harvest resume remains candidate 28** | `PROGRAM/DECISIONS/ADR-0003.md` (Session 9; Al's rulings on (i)–(v) recorded Session 10): D1 routes (arXiv comments, Crossref update relations, zbMATH index; AI-generated excluded), D2 admission, D3 class mapping (undetermined truth admitted under `syndeom-manifest/2`), D4 record (`manifest.json`, bytes, `RECORD.md`, S-entries, `groundtruth/HARVEST-LOG.md`), D5 licences (NC/ND `external`), D6 `raw/1` parent + derived artifact + the producer, D7 log, D8 window, no skipping, prefix gate, D9 the S1.4 table; routes live-checked 2026-10-13. **Producer:** `engine/ingest/` → `syndeom-ingest` (`s1_ingest` 0.1.0, Session 10): writes a `/2` manifest from a config and bytes, refuses on any ADR-0001 D4 or ADR-0003 shape violation (exit 2) and on a same-id byte conflict (exit 3); goldens over fixed bytes (fixture `915d3ed6…1390`, 72 bytes) and two fixed configs — boolean truth (`s1_ingest-v0.1.0-d2134e7fd2c3`, manifest `1f2cf7b9…1f63`) and the null case (`s1_ingest-v0.1.0-4f5d4e93adaa`, manifest `451bde2d…8340`) — config hashes cross-checked in Python. Orientation reconciliation (2026-10-20): the log and 31 admitted items exist; the former empty-slots sentence was stale. |
| S1.2 Informal–formal pairs with the mathlib commit pinned (positive statement-fidelity pairs; drifted pairs are synthetic and live in `corpora/`, SD12) | **one positive pair admitted by owner merge PR 4** | ADR-0007; both pair RECORD post-merge dispositions; actual producer/both-file/lineage/type/axiom/replay evidence in `analysis/s1-2/implementation/`; merge metadata in `analysis/s1-4/config/merge-evidence.json`. No separate GitHub reviewer approval, no PROVEN promotion; statement-only informal selection, zero steps. |
| S1.3 Manifests for real items (`origin: ground-truth`; `raw/1` parent plus derived canonical artifact per ADR-0001 D4; nothing synthetic labeled real) | **35 archived representations (33 raw + two canonical), 32 effective logical items; 30 without canonical lengths. First-parent status support discharged on verified main by PR 7; fresh Rust reproduction and preservation evidence in `analysis/s1-post-merge-readiness/`. After ADR-0009 and ADR-0012 the reproduction gives the retained inventory byte-identical at main `3cd5558` with that revision's tool, and at HEAD a PASS differing from the retained report only in the `preservation` object, whose byte checks are now the input set alone (ADR-0012 D6; bullet near the top of this file).** | `s1_ingest-v0.1.0-63e23a952084/` (arXiv 0706.0357 v70, Lemma 1.1; FLAW-0012 + FLAW-0005; old raw retained; stable logical key resolves to accepted false/counterexample representation `s1_ingest-v0.1.0-15ab865466d6`; canonical selection/context/locator review still required; original committed, CC BY 4.0); `s1_ingest-v0.1.0-744652f4b95f/` (arXiv 0707.3799 v2, Theorem 1 b); FLAW-0011 + FLAW-0005; statement true; `external`); `s1_ingest-v0.1.0-5ba6c0f07557/` (arXiv 0902.0680 v1, the dense-subspace lemma; FLAW-0005 gap; **statement undetermined — `value: null`, the first use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-a8ac73896464/` (arXiv 0908.3213 v4, Theorem 4.4; FLAW-0001 propagated + local; statement false; `external`); `s1_ingest-v0.1.0-be3b17e73425/` and `s1_ingest-v0.1.0-72415457dce4/` (arXiv 1005.2398 v2, Proposition 3.3.5 and the lemma following it; FLAW-0012 + FLAW-0005 each; statements false — the proposition by the author's counterexample; `external`); `s1_ingest-v0.1.0-e0c199d3c60a/` (arXiv 1009.3998 v4, Lemma 13.2; FLAW-0011 + FLAW-0001 local; statement true; `external`); `s1_ingest-v0.1.0-448d7aefa486/` (arXiv 1010.2391 v9, Lemma A.3; FLAW-0012 + FLAW-0005; statement false; `external`); `s1_ingest-v0.1.0-1c4c9a7747d0/` and `s1_ingest-v0.1.0-2d93a266558a/` (arXiv 1107.0863 v3, Lemma 12 and Theorem 1; FLAW-0012 each; the lemma false by the authors' counterexample figure, the theorem by source document; `external`); `s1_ingest-v0.1.0-e3cbdfa7ad1c/` (arXiv 1112.6107 v2, Theorem 1; FLAW-0012; statement false by source document — the misstated last clause, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-22ae000cb879/` and `s1_ingest-v0.1.0-fd2bfef40d31/` (arXiv 1201.3327 v2, the former Lemma 5.8 and the former Theorem 5.9; FLAW-0012 fatal and FLAW-0005 gap; the lemma false by source document, **the theorem undetermined — `value: null`, the second use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-dc1e44f39bb9/` (arXiv 1202.5531 v3, Theorem 6; FLAW-0012; statement false by source document, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-68ed1abfecb4/` and `s1_ingest-v0.1.0-a4822a4be06e/` (arXiv 1206.1893 v6, Propositions 34 and 35; FLAW-0012 + FLAW-0005, and FLAW-0005 gap; the first false by source document, **the second undetermined — `value: null`, the third use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-da3a4784c2be/` (arXiv 1206.2228 v1, Theorem 1; FLAW-0012; statement false by source document; **`status.kind` `retraction`, the first**; `external`); `s1_ingest-v0.1.0-685b874cf579/` (arXiv 1206.5522 v5, Proposition 5.3; FLAW-0012 + FLAW-0005; statement false by source document — the hypotheses corrected, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-00f28e252823/` and `s1_ingest-v0.1.0-a93badadec27/` (arXiv 1209.2022 v2, Proposition 3.1 and Theorem 3.2; FLAW-0012, and FLAW-0011 + FLAW-0005 gap; the proposition false by the printed counterexample, the theorem true by source document; `external`); `s1_ingest-v0.1.0-cafb56cb27fb/` (arXiv 1209.4040 v3, Theorem 4.5; FLAW-0005 gap; **undetermined — `value: null`, the fourth use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-45c3ee8319d5/`, `s1_ingest-v0.1.0-486ef22f632b/` and `s1_ingest-v0.1.0-dc0e0b0d5b87/` (arXiv 1210.6709 v1, Theorems 1.6, 2.2 and 2.1; FLAW-0005 fatal, fatal and gap; **all three undetermined — `value: null`, the fifth to seventh uses of ruling (ii)**; `external`); `s1_ingest-v0.1.0-40e6c0da8821/` (arXiv 1303.3650 v2, Theorem 4.6; FLAW-0012 + FLAW-0005; statement false by source document, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-003a35498654/`, `s1_ingest-v0.1.0-0833f61c748c/`, `s1_ingest-v0.1.0-82b4e66d0016/` and `s1_ingest-v0.1.0-315546e87259/` (arXiv 1304.5623 v4, Proposition 3.5, Theorem 3.6 (4), Theorem 4.3 and Theorem 5.3; FLAW-0004 gap, FLAW-0005 gap, FLAW-0012 + FLAW-0004 + FLAW-0004, FLAW-0005 gap; **three undetermined — `value: null`, the eighth to tenth uses of ruling (ii)**, Theorem 4.3 false by the erratum's example; `external`); `s1_ingest-v0.1.0-f634c259799f/` (arXiv 1312.1045 v1, Theorem A.1; FLAW-0011 + FLAW-0005 gap; statement true by source document; `external`); `s1_ingest-v0.1.0-6ae03b18a1bd/` (arXiv 1403.4485 v3, Proposition 5.1; FLAW-0001 propagated + local; statement false by source document; `external`) — all under `groundtruth/`, each with `manifest.json` (`syndeom-manifest/2`, `engine_commit` of the producer's commit, self-checked by the producer) and `RECORD.md` (the D3 class argument); S-0024..S-0065 |
| S1.4 Coverage and power: per flaw class, how many real items at what proof lengths; zero-coverage classes named as a finding | **merged in PR 5; complete for pinned prefix, qualified, reproducible and frozen** | `analysis/s1-4/report/report.md`, inventory/coverage/provenance JSON, frozen bins, two fresh byte-identical runs. Five observed classes, nine zero, 32 historical roots / 17 unflagged roots. Every real-item cell expected underpowered; no numerical power or detector run. `analysis/s1-4/OBLIGATIONS.md` is the historical ledger. The later effective inventory records 35 representations / 32 logical items / 18 unflagged; current requirements and limits are mapped in `analysis/s1-post-merge-readiness/READINESS.md`. |
| Licences recorded for anything touched (SD16) | **recorded; existing storage choices preserved** | Per-source completeness matrix and source records in `analysis/s1-4/`; SOURCES historical prefix unchanged. No new external source ingested or rights inferred. First-parent status support is discharged on verified main under the accepted scoped ruling. External bytes retain historical evidence and must be retrieved/reverified before scoring; the older dispositions and exclusions remain unchanged. |
| Repo green under the gate | **done (and standing)** | ORIENT gate GREEN at `e029c80` plus the declared disclosure append, 2026-10-26T23:16:58Z–23:25:02Z (Current build verification above); the closing commit's staged-tree gate is recorded in its pull request |
| Cold-start test (§2, SD15) | **done — PASS (Session 29, 2026-10-26)**; earlier: Sessions 22 and 25 ineligible attempts; Session 27 not cold; Session 28 not an examination | HANDOFF Session 29, whose first entry is the SD15 disclosure (index and memory file reproduced per ADR-0005; memory judged to hold no program state, for Al's audit). The session was a fresh conversation launched from `C:\Users\LEET` (no Git snapshot) with the §6 Session-N prompt (ADR-0009 wording) verbatim and alone. The ADR-0008 preflight found it synchronized at fetched `origin/main` `e029c80e1be149bef8fe6ceb76eed59c235ed8b4`, tree `67023777…`. It re-derived in one pass from the repository: phase and gate; S1.1 (harvest-log tally: 27 rows, 21 admitted yielding 31 items, 4 rejected, 2 found-but-unclassifiable; last `1403.4485`; ADR-0003 D8 prefix rule); S1.2; S1.3 (fresh-clone Rust reproduction `PASS: 35 / 32`, only `exact_files_checked` 370 → 369); S1.4 (prefix sentence, zero coverage, power statement); registers, pins, escalations and the next action. It compared these with this file: agreement, with three reconciliations of events after this file was written (PR 14's merge; the post-merge memory line and branch deletions recorded in Session 28's external root; the frozen packets' historical `gate_s1` fields). Gate GREEN (row above). The PASS rests on three SD15 judgments stated for Al's audit (Standing escalations). |

## Gate S0 exit criteria — CLOSED 2026-10-13 (Session 6)

| Criterion | Status | Evidence |
|---|---|---|
| S0.1 CLAIM-0001..0005 (and 0007) resolved to VERIFIED or honestly UNVERIFIED with consequences | **done** | all six VERIFIED 2026-10-12 from primary sources (CLAIMS.md transitions; SOURCES.md S-0001..S-0006 with URLs, access times, licences, digests). Consequences stated: S5 corpus location = `github.com/openai/math` (Apache-2.0); counts carry units and three unreconciled formalization units; the AHM page is undated; IAS page unretrievable (403); signatory counts recorded as of access date |
| S0.2 Founding sweep placed with the SD11 banner | **done** | `PROGRAM/SWEEP-2026-10.md`, SHA-256 `f4bfb2ad…b03e` = Al's stated digest (verified before the move, Session 3; recomputed Session 6); banner present; redacted under SD8 before ferry; reconciliation note defers to the S0.1 register |
| S0.3 Constitution ratified by Al; DRAFT marker removed | **done** | `c803c96` `[S0][ADR-0000] ratify the constitution and record Al's rulings`; ADR-0000 addendum |
| S0.4 Manifest schema ADR (`syndeom-manifest/1` extending `friedman-manifest/1`) | **done** | `PROGRAM/DECISIONS/ADR-0001.md` (Session 3; ratified Session 4): D1–D9 under Q10 (structural) and Q11 (fields); FRIEDMAN ADR-0001/0004 and crate re-read at pinned tip `7c3e5dd`; two goldens over fixed bytes (Golden A `0ac16447…a6a1`, Golden B `c568a981…458e`); RFC 8785 retrieved as S-0007; the five flagged choices ruled and the derived-artifact convention added (*Amendments*) |
| S0.5 Flaw taxonomy v0.1 ratified | **done** | `PROGRAM/FLAWS.md` 0.1.0, 14 entries, ratified (Q12; changelog) at `c803c96` |
| S0.6 Scaffold (`lean/` with Mathlib, `engine/`, `lab/`, `corpora/`, `groundtruth/`, `contact/`, `analysis/`, `sources/`, gate, CI) | **done** | `PROGRAM/DECISIONS/ADR-0002.md` (Session 4): Lean `v4.34.1` = Mathlib's `lean-toolchain` at pinned `d13f23b7…` (tag `v4.34.1`), `lake-manifest.json` locked; `engine/` workspace with `syndeom-manifest` (ADR-0001 D1–D7, 23 tests incl. D8 goldens, ADR-0004 vectors, RFC 8785 vectors); `scripts/check.sh` + `scripts/check_registers.py`; advisory CI on `actions/*@v5`; READMEs incl. `contact/` SD8 rules; licences S-0008; D8 measured: Golden B builds, axioms `[propext]` |
| Licences recorded for anything touched | **done** | S-0001 Apache-2.0 (nothing copied); S-0002, S-0003 licence unstated (nothing copied); S-0004 CC0 and S-0005 CC BY-NC 4.0 (PDFs copied under those terms); S-0008 for every toolchain pin; S-0010..S-0016 CC BY 4.0 (PDFs copied); S-0009, S-0018 digest-only |
| Repo green under the gate | **done (and standing)** | `bash scripts/check.sh` → `GATE: GREEN` at `2b7c573` in Session 6 and on the Session-6 tree (Build status above); GREEN at every earlier recorded run |
| CLAIM-0006 examined (deferred S0 action, Al's ruling Session 3) | **done** | Session 5: the ten ranked sources retrieved from primaries and read (S-0009..S-0018; seven CC BY PDFs under `sources/`, two digest-only, one repository); **no source falsifies the claim as worded**; CLAIM-0006 narrowed and moved UNKNOWN → CONDITIONAL (unfalsified as of 2026-10-13, bounding sources listed); the scoped novelty statement recorded in CLAIMS |
| Cold-start test (§2, SD15) | **done — PASS** | Session 6, a fresh conversation launched with the §6 prompt verbatim and alone: recalled memory reproduced verbatim as the first HANDOFF entry (no program state found; two borderline lines flagged for Al's audit); program state re-derived in one pass from the repo — phase, every S0 criterion with its evidence, all CLAIM and SOURCE statuses with CLAIM-0006's reason, the Lean/Mathlib/Rust pins from the files on disk, the manifest schema from ADR-0001, Q3 and SD13 as the open escalations, the next action — and compared against STATE.md: agreement on every item; two precision reconciliations (the Build-status line lagged HANDOFF by one session; "24 crates" = dependencies, 25 lock packages with the workspace crate) made as the session's first task. Gate GREEN at `2b7c573` (03:45:45Z–03:46:17Z). HANDOFF Session 6; commit `[S0][INFRA] gate S0 cold-start exam: PASS at 2b7c573` |

## Blockers

- **None for S2's first action.** Gate S1's last open criterion, the cold-start
  examination, passed in Session 29 (2026-10-26). The closure takes effect on `main` at
  the merge of that session's pull request and rests on the SD15 judgments under Standing
  escalations. The paragraph below is the S1 blocker as it stood before the examination,
  kept as the record.
- *(Historical, before Session 29)* **Gate S1 remains OPEN for the genuine cold-start examination** (constitution §2,
  SD15, §4.1 and §6). No additional substantive S1 exit criterion is identified as
  unmet for the admitted prefix in `analysis/s1-post-merge-readiness/READINESS.md`.
  The first-parent status-support hold is discharged on verified main. The frozen
  S1.4 findings and older nonblocking questions/dispositions/exclusions stand.
  Canonical selection/context/locator review, missing canonical lengths and external-byte
  verification remain future scoring prerequisites, not an obligation to canonicalize
  the entire prefix or fill every flaw class before S1. This informed session cannot
  certify the examination. The readiness reconciliation (PR 8), the orientation repair
  (PR 9), the cold-start preparation (PR 10), Session 25's record (PR 11), ADR-0009 (PR 12) and
  Session 27's record (PR 13) are merged; their review records are those merged pull requests.
  Session 25, launched as the examination,
  disclosed late and is ineligible; Al decided the step-0 escalation by ADR-0009 (2026-10-26),
  which puts the disclosure first in the §6 prompt itself (Standing escalations, first bullet).
  Session 28, a fresh §6 launch inside the checkout, received the harness's Git snapshot before
  its first action and is not an examination; the examination is launched from `C:\Users\LEET`
  (Memory policy).

## Standing escalations and dispositions

- **`s2_grammar` 0.1.0 — awaiting Al's review and merge** (Session 47; branch
  `claude/s2-grammar-0.1.0`; `analysis/s2-generate-8/README.md`). Merge with a merge commit, so
  that `25ad34fe45c1eea8527b3aab47829b31c8685931`, the goldens' `engine_commit`, stays a real
  commit on `main`.
  - **The readings as flagged** (packet readings 85–99): the length dials count the chain's
    equalities and the lemma steps separately, so the commanded `n_steps` is their sum (85); a
    lemma states one chain step's identity and is cited as a term, so every equality is closed
    by `ring` directly or in its lemma (86); lemmas first in document order, numbered by the
    steps they state (87); the presentations are the grammar's rewrites — a fresh draw per
    step, consecutive steps distinct, earlier forms allowed to recur; LaTeX by juxtaposition
    with `\cdot` for a split square (88); the fourteen seed forms, every one using both
    variables (89); **the truth reference names the control with target `formal` for every
    target, and the formal control names itself (90)**; the one import `Mathlib.Tactic.Ring`
    (91); **FLAW-0001's site a ring-closed chain step — never a lemma-cited step or a lemma
    step; for `local` never the last — an explicit site a plan step index (92)**; the
    perturbation on the site's ordered terms, re-rendered under the site's own presentation;
    coefficients grow in magnitude; `constant` adds `+ 1` when there is none; `exponent`
    excludes collisions (93); **`propagated` re-derives every later step on its own stream and
    restates every lemma cited after the site, the claim of record untouched; `local` leaves the
    next step's identity false too, so Lean reports a second error after the first at the site,
    which is what K reads (94)**; the `monomial` dial, and a restated lemma's
    `original_fragments` entry its declaration lines (95); severity fixed by the sub-variant,
    another refused (96); the grammar's remaining v0.1 classes as later versions (97); the
    dispatch by the config's key, `Prepared` carrying a `Producer` (98); the goldens at (4, 0)
    and (24, 2), FLAW-0001 at (4, 0) with `sign` (99).
  - **FINDING-018** (informational, backlog: `derive`, `Derivation::n` and `render` trust their
    callers' preconditions, which `prepare_with` enforces); FINDING-015 does not extend to the
    grammar (its truth rests on the construction and the control's measured build); FINDING-016
    and FINDING-017 standing.
- **`s2_template` 0.7.0 — RULED by Al on 2026-10-28 (rulings on PR 32, verbatim under
  *Standing rulings* and in ADR-0011 *Amendments*); merged as PR 32 by Al on 2026-10-29** (Session 46; branch
  `claude/s2-template-0.7.0`; `analysis/s2-generate-7/README.md`). Merge with a merge commit,
  so that `6dba3615a001151116632169bd93b25440878ac0`, the 0.7.0 goldens' `engine_commit`, stays
  a real commit on `main`.
  - Readings 66–84 are accepted as read. Reading 68 carries Al's recorded erratum note against
    FLAWS.md 0.1.0: the FLAW-0008 example applies two transformations where the recipe says
    exactly one, and its changed conclusion is false without the added hypothesis; under D5 the
    example realizes as the `add-hypothesis` drift with the example's proof route; FLAWS.md's
    text is left as it is. Reading 73 carries Al's recorded note: a linter warning is neither a
    kernel rejection nor an observation this program records; whether an unused-hypothesis lint
    enters the battery, and with it the observation schema, is for the S3 detector ADR. Nothing
    regenerated; the version stays 0.7.0. Nothing else is awaited on the readings.
  - **The readings as flagged** (packet readings 66–84): FLAW-0008's material a replacement of
    the statement at site 0 with its record formal-only at the main declaration's header line
    (66); the drift as one edit under one textual rule per transformation, applied to the seed
    and to the record alike, both statement texts and the edit in the dials (67); **FLAWS.md's
    example applies two transformations at once and its changed conclusion is not provable
    without the added hypothesis, so the seed realizes the example's hypothesis as
    `add-hypothesis` and realizes `weaken-conclusion` as dropping `StrictMono φ` — the
    executor's choice (68)**; `restrict-type` as ℝ → ℤ (FLAWS.md says ℝ to ℕ; ℕ has no `|·|`),
    a special case, gap, the proof text unchanged (69); `flip-strictness` in the hypothesis, a
    stronger hypothesis, gap (70); **`shadow-definition` as a `let` inside the claim, a term
    redefined locally to the statement, the `def`-before-the-theorem alternative not expressible
    under D2 while the informal file must stay faithful (71)**; the severities by the seed's
    marking, gap for the strictly weaker drifts and fatal for the trivialized ones (72); **the
    trivialized drifts leave the bound `hb` unreferenced and Lean's linter warns — a warning
    the kernel ignores, the class staying `NO` (73)**; "both statements build" discharged by the
    artifact's own build and the seed's witness reference (74); `original_fragments` recorded,
    no `original_steps`, the informal record never emitted (75); a composite may embed
    FLAW-0008 (76); **FLAW-0012 as two instances — FLAW-0005 `violated` at the hidden use site
    and FLAW-0012 at the statement — the composite's shape, because records of one instance
    share a class (77)**; the use-site record's dials from the seed's `use`, `mode` fixed
    `violated`, the seed without a control or FLAW-0005 material of its own (78); FLAW-0012's
    confirmation from the use-site instance and the negation witness's reference (79); the
    transplanted proof's failure an application type mismatch at the `have hxb` line, with no
    cascade (80); the seed's statement form, "attains its supremum" as a maximum, the negation
    by `arctan` (81); `dropped_hypothesis` a free-text dial matched against the material, the
    use site nameable explicitly (82); the seed-level imports covering every material, so the
    control imports `Mathlib.Topology.Instances.Int` unused (83); FLAW-0012's informal-only
    artifact keeping `YES-KERNEL` on both records (84).
  - **FINDING-017** (informational, backlog: the drift rules are textual, not token-aware; no
    config text reaches a statement in 0.7.0); FINDING-015 now also covers the two new
    witness references; FINDING-016 standing.
- **`s2_template` 0.6.0 — RULED by Al on 2026-10-28 (rulings on PR 31, verbatim under
  *Standing rulings* and in ADR-0011 *Amendments*); merged as PR 31 by Al on 2026-10-29**
  (Session 45; branch `claude/s2-template-0.6.0`; `analysis/s2-generate-6/README.md`).
  - Readings 52–65 are accepted as read. Reading 52 carries Al's recorded note: FLAW-0010 `a`'s
    references are placeholder-shaped by D5's design, so S3 reads detection rates for that class
    as rates against obviously fictional references; a plausible-shaped lexicon, if one is ever
    wanted, is a later producer version and never attaches a real name to a fictional work.
    Reading 61 carries Al's recorded note: the cycle pair's formal file is a valid proof by
    construction; the instance is informal-only, and the pair is invalid under D7's either-file
    rule. Nothing regenerated; the version stays 0.6.0. Nothing else is awaited on the readings.
  - **The readings as flagged** (the packet; ADR-0011 *Amendments*): the reference's form
    `[<author> <year>, <venue>, <result>]` and the executor's lexicon of obviously fictional
    tokens (52); the draw order author, year, venue, result as part of the rng definition (53);
    `shape` a config dial matched outside the material (54); the record spanning the reference
    with `replaced` the justification and the check reading D5's "the step text minus its
    justification" as `By <reference>` plus the bare claim (55); `claim` both a reserved
    recorded-dial name and FLAW-0010's config dial (56); **FLAW-0010 `N/A` for every target,
    a pair's formal file faithful and its `lean_build` commanded `pass` (57)**; `claim: false`
    refused for lack of false-step material, `b` and `c` deferred (58); `direct`'s site the
    inserted lemma with the key step's rewrite in `original_steps` (59); the faithful supremum
    step dropped by `direct` (60); **the `cycle`'s formal file a faithful proof with Lemma 1
    declared and proved from Mathlib, not the control's bytes, because D2 renders a `Lemma N.`
    step as a declaration (61)**; `premises` (D2) as an optional step field, the recorded
    `cycle` as the declarations visited (62); the `cycle` check structural, untouched by any
    Lean observation (63); the seed's statement form (monotone as non-decreasing, bounded
    above) and its witness through `tendsto_atTop_ciSup` (64); the `direct` record at the
    declaration's first line while Lean reports the proof line (65).
  - **FINDING-016** (informational, backlog): the premise-cycle walk is a depth-first search
    over simple paths; bounded today by the seed material and the pinned `cycle_length`.
    FINDING-015 (informational, backlog) stands and covers the new seed's witness reference.
  - Merge with a merge commit, so that `ef41a10c…`, the 0.6.0 goldens' `engine_commit`, stays a
    real commit on `main`.
- **`s2_template` 0.5.0 — RULED by Al on 2026-10-28 (rulings on PR 30, verbatim under
  *Standing rulings* and in ADR-0011 *Amendments*); MERGED as PR 30 by Al, 2026-10-29T00:25:46Z,
  merge commit `854a4b4`** (Session 44; branch
  `claude/s2-template-0.5.0`; `analysis/s2-generate-5/README.md`).
  - Readings 39, 40, 41, 42, 44, 45, 47, 48, 49, 50 and 51 are accepted as read. Reading 43 is
    replaced: D7 stands, and FLAW-0011 requires at least one embedded instance of severity gap
    or fatal — a composite whose embedded instances are all cosmetic is refused, with a test;
    FLAWS.md's FLAW-0011 example realizes FLAW-0004 `routine-true` (an erratum note against
    FLAWS.md 0.1.0 in ADR-0011 *Amendments*; FLAWS.md unchanged); the two cosmetic composite
    goldens were replaced by a composite of `continuity_is_not_uniform`'s FLAW-0002 gap, and
    all 0.5.0 goldens regenerated at the fix commit `1c0c5fcd…` (the FLAW-0004 proof bytes
    unchanged). Reading 46 is replaced: a composite embeds FLAW-0001 through FLAW-0010 only.
    Applied in commits `1c0c5fcd…`, `5afc00d7…` and the record commit; nothing else is awaited on
    the readings.
  - **The readings as flagged** (the packet; ADR-0011 *Amendments*): the hedge drawn, not commanded (39);
    the whole proof block of the aligned `have` omitted, the `have` header and the `obtain` kept
    (40); the bare claim and the justification recorded separately, and the D5 informal check
    read as the hedge entry plus the claim (41); `N/A` for an informal-only target as the
    instance's value (42); **a composite of a cosmetic instance — FLAWS.md's own FLAW-0011
    example — is `valid` and `correct: true` under D7, against the recipe's `proof_status:
    invalid`; the alternatives are to force `invalid` or to refuse cosmetic-only composites
    (43)**; the embedded instance numbered before the composite (44); the truth witness taken by
    construction, no per-artifact Lean run (45); one embedded instance per composite, nesting,
    FLAW-0012 and FLAW-0013 refused, padding with a composite refused — whether a composite may
    embed FLAW-0013 is Al's (46); the composite's locations at step 0 and the main declaration's
    header line (47); `lean_build` commanded `fail` for a composite pair (48); FLAW-0004 recorded
    in both files of a pair (49); `embedded` general beyond the example, tested without Lean
    (50); `nontrivial-true` and `false` parsed but without material (51).
  - **FINDING-014** closed by 0.5.0. **FINDING-015** (informational): a composite's truth witness
    reference is not resolved against `lean/` by the producer; backlog for a later revision.
  - Merge with a merge commit, so that `7be41123…`, the 0.5.0 goldens' `engine_commit`, stays a
    real commit on `main`.
  - **Merged by Al as PR 30** (2026-10-29T00:25:46Z, merge commit `854a4b4`; parents `fbcc6c4`
    and `dbce6d9`), so `1c0c5fcd…` (the regenerated goldens' `engine_commit`) and `5afc00d7…`
    are real commits on `main`. Nothing awaited.
- **`s2_template` 0.4.0 — RULED by Al on 2026-10-28 (rulings on PR 29, verbatim under
  *Standing rulings* and in ADR-0011 *Amendments*); MERGED as PR 29 by Al, 2026-10-28T23:06:26Z,
  merge commit `fbcc6c4`** (Session 43; branch
  `claude/s2-template-0.4.0`; `analysis/s2-generate-4/README.md`).
  - Readings 29–38 are accepted as read. Reading 30 carries Al's recorded reading of D4 and D8:
    the integer `count` is the generation decision because D4 forbids a floating-point value
    there; `padding_ratio` is a commanded statistic measured from the emitted file and checked
    at `abs 0.05`; a ratio the count cannot reach is a FAIL verdict; a ratio-targeted mode, if a
    later version wants one, chooses its count by exact integer comparison of token quotients.
    Nothing regenerated; the version stays 0.4.0. Nothing else is awaited on the readings.
  - **The readings as flagged** (the packet; ADR-0011 *Amendments*): the seed's statement form (the
    number-theoretic core of "√2 is irrational"); how padding is commanded (a FLAW-0013 spec with
    `kinds`, `count` and `placement` plus a positive `padding_ratio`; `count` the generation
    decision, the ratio measured and checked at `abs 0.05`); `placement` (`before` | `drawn`);
    one instance per paragraph; padding steps never lemmas (`unused-lemma` and
    `verbose-rewrite` without material); `padding_ratio` of a formal-only artifact over the
    declarations' tokens; one spec per artifact (no padding with an injected flaw yet); the
    record dials without originals; the goldens' two-decimal commanded ratios; check T for
    FLAW-0013 as the FLAW-0007 `unused` condition.
  - **FINDING-008** (informational, packet 1): closed by 0.4.0. **FINDING-014** (informational):
    a seed's own step may carry a padding header unrefused; backlog for the next revision.
  - Merge with a merge commit, so that `b54e2f16…`, the 0.4.0 goldens' `engine_commit`, stays a
    real commit on `main`.
- **`s2_template` 0.3.0 — RULED by Al on 2026-10-28 (rulings on PR 28, verbatim under
  *Standing rulings* and in ADR-0011 *Amendments*); merged as PR 28** (Session 42; branch
  `claude/s2-template-0.3.0`; `analysis/s2-generate-3/README.md`).
  - Readings 21–28 are accepted as read. Reading 24 carries Al's recorded reading of FLAWS.md's
    FLAW-0009 recipe: the same statement for `truth: true`; for `truth: false` a false statement
    from which the main theorem still follows, its negation proved by the witness module; the
    record carrying the replaced lemma and the inserted axiom (the 0.3.0 formal record's
    `replaced` and `inserted`), so the modality mismatch in a pair is explicit in the ground
    truth; FLAWS.md unchanged. Nothing regenerated; the version stays 0.3.0. Nothing else is
    awaited on the readings.
  - **FINDING-011** (High): `verify` interpolated the manifest's `main_declaration` into the
    compiled audit file, present since 0.1.0; fixed in the same pull request (`5a313ddf…`).
  - Merge with a merge commit, so that `5a313ddf…`, the 0.3.0 goldens' `engine_commit`, stays a
    real commit on `main`.
  - **Merged by Al as PR 28** (2026-10-28T21:12:20Z, merge commit `4cd1f77`; parents `7a05e75`
    and `caa4db2`), so `5a313ddf…` and `caa4db2c…` are real commits on `main`. Nothing awaited.
- **`s2_template` 0.2.0 — RULED by Al on 2026-10-28 (rulings on PR 27, verbatim under
  *Standing rulings* and in ADR-0011 *Amendments*); merged as PR 27** (Session 41; branch
  `claude/s2-template-0.2.0`; `analysis/s2-generate-2/README.md`).
  - Readings 13, 15, 16, 17, 18, 19 and 20 are accepted as read.
  - Reading 14 is replaced: D5's FLAW-0006 mode enum gains `step-above-base` (the inductive
    step's argument is valid only for n above the base, so the base → base+1 step is
    unjustified; `failing_case` names that first unjustified step). The horses record is
    relabeled in the seed and, by regeneration, in its three goldens; the proof bytes are
    unchanged. All 14 goldens of 0.2.0 are regenerated at one engine commit — the commit holding
    the relabeled seed, `c899b0732db8b7a1a104822772a428f534a651cd` — from two runs confirmed
    byte-identical; the horses proof bytes are unchanged.
  - Merge with a merge commit, so that the 0.2.0 goldens' `engine_commit` stays a real commit on
    `main`.
  - The owner note of 2026-10-28 is recorded (`da2bd83`); nothing else is awaited on it.
  - **Merged by Al as PR 27** (2026-10-28T19:49:50Z, merge commit `7a05e75`; parents `de3e4a6`
    and `016f369`), so `c899b07` is a real commit on `main`. Nothing awaited.
- **ADR-0011's crate, the first implementation — RULED by Al on 2026-10-28 (rulings 1–6 on
  PR 26, verbatim under *Standing rulings* and in ADR-0011 *Amendments*); merged as PR 26**
  (Session 40; branch `claude/s2-generate-crate`; `analysis/s2-generate-1/README.md`).
  - The gate change `21f8359` is accepted.
  - Readings 1, 2, 3, 5–9, 11 and 12 are accepted. Reading 4 is replaced: flaw counts are per
    instance. Reading 10 is superseded: the RNG attribution is verified (S-0081).
  - `s2_template` 0.2.0 embeds build-time input hashes, closing FINDING-005.
  - Al merges with a merge commit, so `21f8359` (the goldens' `engine_commit`) stays a real
    commit on `main`.
  - **Merged by Al as PR 26** (2026-10-28T18:10:24Z, merge commit `de3e4a6`; parents `f9f9730`
    and `c4250d8`), so `21f8359` is a real commit on `main`.
  - **The pair-confirmation reading — DECIDED** (Al's owner note of 2026-10-28, item 2;
    *Standing rulings*): a pair's injected instance is confirmed only when both of its records
    pass their checks. ADR-0001 stays unedited on flaw-count units (item 1). Recorded in
    ADR-0011 *Amendments* with 0.2.0's amendment (Session 41). Nothing awaited.
- **Held harvest admission review — RULED by Al on 2026-10-28** (rulings on PR 23, verbatim
  under *Standing rulings*; HANDOFF Session 37's addendum). The record is
  `analysis/s1-held-harvest-review/REVIEW.md` §2–§4, with its `receipt.json`. All five items on
  `claude/s1-harvest-candidates-28-30` at `c3a3d36` agreed with the preserved evidence under
  ADR-0003 D2 and D3.
  - **Q-H1:** item 32 is false by source document, as recorded, and its class (FLAW-0012) stands.
    "The wrong theorem" belongs to the wording group main's items were read under. Changing that
    standard would be a corpus-wide decision.
  - **Q-H2 / Q-H3:** items 34 and 35 are `gap`, as recorded, by the entry-range practice main's
    items follow.
  - **Q-H4:** the further unnamed results stay out as items and join the existing unnamed-result
    question (*Carried questions* and *Older questions*, below).
  - **Q-H5:** item 36 is `undetermined`, as recorded.
  - **Q-H6:** integrate now, as the first later prefix, per REVIEW.md §4 (option (a)).
  - **Q-H7:** the harvest log continues as a new dated section after the file's current end,
    with its own table header, rows and counts, the tag's bytes an exact prefix. The five new item
    directories get frozen rows, added in a second commit on the integration branch. Their freeze
    point is the commit that adds the directories. Pull requests keep merging with merge commits,
    never squash, so that freeze point stays in `main`'s history (*Freeze register*).
  - **Integrated in Session 38** on branch `claude/s1-held-harvest-integration` (Updated
    bullet): commit 1 adds the five items, S-0074..S-0080 and the log section; commit 2 adds their
    frozen rows. **Merged by Al as PR 24** (2026-10-28T05:27:56Z, merge commit `ef567ec`).
  - **`engine_commit` — ACCEPTED** (Al's owner note of 2026-10-28, item 1; *Standing rulings*):
    the five manifests record `a793954`, the commit the producer ran from, as main's existing items
    do. REVIEW.md §4 step 3's "at the integration commit" could not be met literally.
- **SD15 disclosure by digest — RULED** (Al's ruling of 2026-10-27, ADR-0005 *Amendments* 1; Al's
  rulings of 2026-10-28 on PR 23, *Amendments* 2).
  - ADR-0005's amendments stand as an addendum, and the constitution is not edited.
  - The observation that the index is machine-wide is answered by the refinement. The by-digest
    comparison is of the matched index lines and the SYNDEOM memory file, so an edit to an
    unrelated index line no longer makes the next disclosure owe verbatim reproduction. The whole
    index's digest is recorded as a fact.
  - **Two readings are the executor's, flagged for Al:** the matched lines' digest is taken over
    each matched line with its LF, in index order; and a renumbering alone, with the lines' bytes
    unchanged, is not a change. **Both CONFIRMED** by Al's owner note of 2026-10-28, item 2, and
    recorded as ADR-0005 *Amendments* 3.
  - **The provisional header's UTC date** (Session 38's note) is now stated in ADR-0005 too, which
    a session reads before it discloses (Al's owner note, item 3; ADR-0005 *Amendments* 3).
- **Session 37's uncommitted append and worktree — housekeeping for after this pull request
  merges, on Al's approval** (the pattern of Session 34 and ruling 4 on PR 20; Al, 2026-10-28:
  "The housekeeping waits until after the merge.").
  - `C:\Users\LEET\Syndeom`, on `main` at `f8d71b8`, holds Session 37's disclosure append
    uncommitted: 3,497 bytes, body `1047d059…d400`.
  - The worktree `C:\Users\LEET\Syndeom-wt-2026-10-27` holds the copied build products.
  - The housekeeping: verify that the committed body equals the append, discard the append,
    fast-forward `main`, and remove the worktree.
  - **DONE in Session 37's conversation after PR 23's merge** (2026-10-28T04:46:50Z–04:49:10Z;
    `housekeeping-receipt.json` and `housekeeping.log` under the same external root), **and
    verified in Session 38** from the receipt, the retained copies, the HEAD blob and the live
    checkout (Updated bullet). Nothing awaited.
- **The inventory event for the first later prefix — RULED by Al on 2026-10-28 (Session 38;
  *Standing rulings*): a separate pull request, next.** The integration commits 40 manifests (37
  effective logical items by the projection) and does not measure them. The pinned
  effective-inventory tool (`engine/ingest/examples/s1_effective_inventory.rs`, 35 / 32) refuses
  the additions, and the S1.4 coverage tool pins its own corpus revision (ADR-0012 D7). Neither
  is changed or re-run by the integration. **Built in Session 39** on branch
  `claude/s1-later-prefix-inventory` (Updated bullet): ADR-0013 (proposed), the tool
  `s1_later_prefix`, and the packet `analysis/s1-later-prefix-1/`, which measures **40 / 37**. It
  awaits Al's review and merge, with a merge commit. **ADR-0013's flags — RULED by Al on
  2026-10-28 on PR 25** (verbatim under *Standing rulings*):
  - **A:** one tool and one packet for the inventory and the coverage run. Accepted.
  - **B:** the five prefix roots count as unflagged. Accepted. An item is flagged by a recorded
    open issue, and a later issue enters through the overlay.
  - **C:** the coverage counts effective labels, with the accepted first-parent amendment
    applied. Accepted. The S1.4 table stays frozen history, and the packet names its labels and
    the amendment map.
  - **D7:** amended. The recorded inputs pin the population, and the counts are measured outputs,
    frozen in the packet. Applied in commit 3, with the packet regenerated in commit 4.
  - **D:** one frozen row for `analysis/s1-later-prefix-1/`, whose freeze point is the commit
    holding the regenerated packet, added by the branch's final commit.
- **Session 38's post-merge housekeeping — DONE on 2026-10-28, outside Session 38's
  conversation** (Al's owner note of 2026-10-28, item 4, verbatim under *Standing rulings*;
  Updated bullet). Nothing awaited.
- **Freeze register — DECIDED by Al on 2026-10-27 (rulings 1–4 on PR 22, verbatim in ADR-0012
  *Amendments* and HANDOFF Session 36; PR 22 merged 2026-10-27T23:06:37Z, creating `main`
  `c48705c2d23183340eef5c01ff13059905513985`).** (1) The invariance test, the
  checker and its tests, the CI changes and the nine enforced rows: accepted. (2) The two
  choices: accepted — a frozen scope refuses additions and removals, and the append-only rows'
  freeze point is the `gate-s1-registration` tag. (3) The awaiting scopes, all at that tag: one
  frozen row per accepted item directory under `groundtruth/` (so later prefixes add new
  directories), `HARVEST-LOG.md` and `README.md` append-only, `derivations/` frozen; the sweep
  and the advisory note frozen (a later change or removal is a register edit under a ruling);
  each of the nine stored copies under `sources/` as its own frozen row (so new copies can be
  stored beside them); ADR-0003 append-only; the scopes not proposed stay unchecked. (4)
  Recorded as a second commit on the branch. Nothing else is awaited from Al on the register.
- **ADR-0011 — DECIDED by Al on 2026-10-27 (rulings 1–4 on PR 20, verbatim in ADR-0011
  *Amendments* and HANDOFF Session 34); PR 20 merged 2026-10-27T20:28:10Z, creating `main`
  `6d89eb2f8a138a53d824d035febffc93423e1e0d` (Session 35's orientation).** (1) Independence from the S1 corpus: no seed, grammar or template derived from
  any item under `groundtruth/`, which is what the frozen battery is validated against in S4;
  seeds come from FLAWS.md or other material outside the S1 corpus, and every seed record states
  its source; binding on later producer versions (ADR-0011 D6). (2) D12 (h1)/(h2): option (iii) —
  the R4 Mnemosyne-derived family stays out of this repository as a private fixture under an
  external root, for private pipeline tests only, never calibration data, never in the public
  goldens, the seeds directory or any artifact here; making it public would be its own
  amendment. (3) The non-blocking clarifications are accepted as recorded; the
  `scripts/check.sh` and `lean/Audit.lean` edits come after the narrowing and get their own
  review. (4) Housekeeping, next bullet. Nothing else is awaited from Al on ADR-0011.
- **Two checkouts — Al's instruction (ruling 4 of 2026-10-27) for the first session after PR 20's
  merge:** remove the worktree `C:\Users\LEET\Syndeom-wt-20261027`; in the original checkout
  `C:\Users\LEET\Syndeom` (branch `claude/s2-citation-cff` at `8ae9431`), confirm the uncommitted
  `PROGRAM/HANDOFF.md` append is the same disclosure now committed by comparing the body's
  SHA-256 — the committed body, from the `MEMORY DISCLOSURE (SD15 as amended …` line through the
  `Conversation-context corollary` line, is
  `36dda70ceeb161b49069dfca5b43bf85be77f584fbdd4ddfc89792e83eb88d2f` (14,931 bytes; the original
  append's whole entry, provisional header included, was 15,108 bytes, `ef8239fd…9429`) — then
  discard it. Al's ruling authorizes that discard: ADR-0008's prohibition covers undeclared work,
  and this append is declared, verified and committed. **DONE in Session 34's conversation after
  the merge (2026-10-27T20:29:39Z–20:32:17Z; receipt `housekeeping-receipt.json`, log and
  `housekeeping-orient-sync.json` under `C:\Users\LEET\Codex\Temp\syndeom-adr-0011-20261027\`),
  and verified in Session 35 from the receipt, the retained copies and the HEAD blob (Updated
  bullet). Nothing awaited.**
- **Gate S1 registration deposit — DONE: published on Zenodo on 2026-10-27** (Al's rulings of
  2026-10-27, ADR-0010 *Amendments*; HANDOFF Session 32). Nothing awaited.
  - **The tag.** `gate-s1-registration`, annotated, with the message "SYNDEOM design registration
    at Gate S1 closure" (tag object `a42e7c09a275157127d89cc931d35742422c0c7f`, 2026-10-27T02:51:14Z).
    It is on PR 17's merge `2018d5e79ea23fa200e560d628bec5e91edf0956`, tree
    `3d7889def475b2b2f30d8acfd3dee31589a06a6e`, and was pushed. No GitHub release.
  - **The archive.** `Syndeom-gate-s1-registration.zip`, built with `git archive --format=zip
    --prefix=Syndeom-gate-s1-registration/` from the tag, outside the repository:
    - 13,370,992 bytes; SHA-256 `95f7d1b7d37aeb604b633b902d8f5e13ab46f5cfd5b582d8d64a299f52039cfc`;
      MD5 `f78dab5eafc6e95f595b1f38b537c9f9`;
    - 464 files: the tag's 466 less exactly the two excluded ones, each byte-identical to its blob;
    - excluded: `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md` (proprietary, R1(b)) and
      `sources/leiden-declaration-2026-06-02-zenodo-20302944.pdf` (CC BY-NC 4.0; S-0005 keeps its
      locator and SHA-256). Both stay in the repository.

    Receipt and verification: `C:\Users\LEET\Codex\Temp\syndeom-gate-s1-registration-20261027\`.
  - **The deposit.** Al published it as Zenodo record 23682781
    (`https://zenodo.org/records/23682781`) on 2026-10-27, licence `polyform-noncommercial-1.0.0`:
    - version DOI (this registration) `10.5281/zenodo.23682781`;
    - concept DOI (all versions) `10.5281/zenodo.23682780`;
    - the file as Zenodo reports it: `Syndeom-gate-s1-registration.zip`, 13,370,992 bytes,
      `md5:f78dab5eafc6e95f595b1f38b537c9f9`.

    Session 32 confirmed each value from Zenodo's API. It also downloaded the published file,
    which is byte-identical to the built archive. README § Citation carries Zenodo's citation and
    BibTeX.
  - **Every later deposit refreshes `CITATION.cff` before its tag** (Al's ruling of 2026-10-27,
    Al's rulings on PR 19's two calls and Al's ruling on the open point, Session 33; ADR-0010
    *Amendments*):
    1. Al reserves the version DOI in the Zenodo draft before the tag. The draft's publication
       date is set to the planned publication date.
    2. The commit that is tagged refreshes the file:
       - the reserved version DOI is added under `identifiers`, with a description naming the
         deposit; none is replaced;
       - `version`, `date-released` (the planned publication date) and `title` become the new
         deposit's;
       - `doi` stays the concept DOI `10.5281/zenodo.23682780`.

       Each archive therefore cites itself.
    3. The draft whose reserved DOI has been written into a tagged commit must be the draft that
       is published.
    4. The tagged file is the reference, because the archive cannot change and Zenodo's metadata
       can. After the deposit:
       - a published value that differs from the tagged file is corrected on Zenodo to match it;
       - if publication slips past the planned date, Zenodo's publication date stays at the
         file's `date-released`, and the actual publication time stays in Zenodo's own record;
       - the records commit confirms the match, instead of refreshing the file, and notes any
         correction made on Zenodo;
       - if a value cannot be brought into line on Zenodo, never retag or rebuild: that records
         commit records the discrepancy and corrects `main`'s `CITATION.cff`.

    The file has no `commit` field: it can never hold the hash of the commit that contains it.
    `version` names the tag, and the README and the Zenodo record give the full hash.
- **SD15 audit request — Gate S1 examination (Session 29, 2026-10-26) — DECIDED by Al on
  2026-10-26: all three judgments upheld; the PASS is not conditional.** Al's rulings,
  transcribed verbatim from Al's message in the Session 29 conversation (also in HANDOFF
  Session 29):

  ~~~text
  Rulings on your three judgments: I uphold all three.
  1. The memory holds no program state, including the new hazard line, which records harness behavior only.
  2. The test -f existence check is not a read of program state: the prompt names the file, and checking that it exists reveals nothing more.
  3. Recording the other attached context after the disclosure is fine: none of it is memory, and none of it names this program or a sibling.

  Order of work: keep NEXT at ADR-0010 per the phase order, with the held harvest review scheduled after it. I will rule on the Mnemosyne seed family when ADR-0010's draft is in front of me.
  ~~~

  The request as Session 29 wrote it: Al, please confirm or overrule three judgments on which
  the PASS rests (HANDOFF Session 29):
  1. no reproduced memory holds program state, including the launch-directory hazard appended
     after PR 14 and the state-adjacent hazards the disclosure names;
  2. the `test -f` existence check in the disclosure's append command is not a read of program
     state;
  3. recording the non-memory harness context (account e-mail, withheld; the tool catalogue; the
     environment block) after the disclosure rather than in it does not affect eligibility.

  If any is overruled, the examination becomes CONDITIONAL PASS. Gate S1's closure is then
  provisional until a cold re-run, and memory is changed only in a non-exam session.
- **Session 29's pull request merged as PR 15** at 2026-10-27T00:25:39Z (2026-10-26 20:25:39
  America/New_York), by Al, creating `main` `c9372b1355a6d687dec6287f908639ac670db659`. Its
  parents are `e029c80…` and the PR's head `6b3dbda5983ed3c3e41ab5d96a2863a043855a88`
  (`claude/s1-session-29-cold-start-exam`), and its tree `4caa81f0…` equals the head's. Gate
  S1's closure is in effect on `main`.
- **Merged branches deleted (Al's instruction, 2026-10-26, after Session 28):**
  `claude/s1-adr-0009-disclosure-first`, `claude/s1-session-25-exam-attempt`,
  `claude/s1-session-27-sec6-launch` and `claude/s1-session-28-launch-directory`. Their tips are
  recorded with restore commands in `deleted-branches-manifest.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\`. Session 29's
  `git ls-remote --heads origin` lists only `main` and the held branch. Session 28's escalation on
  those branches is closed.
- **Examination step 0 — DECIDED by Al on 2026-10-26: ADR-0009, accepted with rulings 1–4 and
  additions 5–6 (transcribed verbatim in the ADR).** Options (a) and (b) together: SD15 and the §6
  Session-N prompt are amended so that every session launched with that prompt discloses first,
  whether or not it attempts a gate test; the prompt's first sentence instructs the disclosure,
  says to append it to HANDOFF without reading that file, allows a provisional block header, and
  points to ADR-0008 for orientation; ADR-0008 D1 step 0 applies to every such session. A session
  that reads program state before disclosing (any repository file but the constitution and
  ADR-0005, a directory listing, Git history or status, or a build) cannot be an examination.
  Not retroactive: Sessions 22 and 25 stay ineligible. The constitution's preservation hash
  `89c198b0…8102` is historical; the amended SHA-256 is `110f5360…479c`. Records of past
  baselines (HANDOFF entries, ADR-0008's header, the frozen packets under `analysis/`) are left
  byte-identical (ADR-0009 D4). The escalation as Session 25 wrote it is in HANDOFF Session 25 and
  in PR 11's STATE.
- **S1.3 reproduction tool: the full narrowing of the byte checks is the next tool revision (Al,
  2026-10-26; ADR-0009 D5, ADR-0010 D6–D7).** The tool should limit its byte checks to the
  inputs it depends on, so that an amendment of a file it does not use neither stops it running
  at HEAD nor fails the gate's `first_parent_amendment` tests. It now carries four case-by-case
  exceptions:
  - the constitution skipped (ADR-0009 D5);
  - `LICENSE` skipped (ADR-0010);
  - `PROGRAM/SOURCES.md` append-only (ADR-0010);
  - the four crate manifests' `license` line (ADR-0010).

  Per Al (2026-10-26), with three exceptions added at once, the full narrowing is the right
  next tool revision rather than a fourth exception. It **lands before any S2 code, including
  ADR-0011's crate**: S2's first code commit edits `engine/Cargo.toml` and `engine/Cargo.lock`,
  which are frozen exact files and two of the 20 pinned pair-producer inputs, so the current
  check would turn the gate RED. Still exact-checked today: every other non-input file in the
  baseline, ADR-0005 and SIBLING-MAP among them (an edit to ADR-0005 failed the reproduction in
  Session 26's probe). **DONE — ADR-0012 (Session 35, 2026-10-27): the byte checks are limited to
  the reproduction's input set (79 of the 375 baseline files, derived from the baseline list), the
  four exceptions are replaced by the rule, and the pair-input comparison with the checkout is
  removed; on `main` since PR 21's merge (2026-10-27T21:36:26Z). The freeze register (ADR-0012
  *Amendments* 1, Session 36) byte-checks the frozen packets and the append-only registers in
  the gate, independently of the tool. The population pin (35 archived representations / 32
  logical items) stands, so a later prefix still needs its own inventory event (ADR-0012 D7).**

- **First-parent status support — accepted, verified and activated on main:**
  Al accepted the exact ruling on 2026-10-20 America/New_York; appended ADR-0003 and
  `analysis/s1-first-parent-integration/OWNER-DECISION.md` record it. The original raw
  `s1_ingest-v0.1.0-63e23a952084` stays immutable and remains the logical key; its
  effective status comes only from `s1_ingest-v0.1.0-15ab865466d6`. Rust validation,
  historical preservation and the full gate passed. This discharges the status-support
  hold on main after PR 7's actual merge at 2026-10-21T03:33:35Z. Fresh merged-main
  verification and the event are in `analysis/s1-post-merge-readiness/`. Do not request
  the same ruling again. Canonical selection/context/locator review remains separate.
- **Carried questions restored from HANDOFF Session 20:** seven false-versus-undetermined
  readings (items 8, 10, 11, 14, 15, 18, 25); seven gap-versus-fatal severity readings
  (items 13, 16, 21, 24, 26, 27, 29); whether unnamed downstream results under candidate 25
  meet ADR-0003 D2.3. They were recorded as nonblocking and remain undecided. By Al's ruling
  Q-H4 of 2026-10-28, the unnamed-result question also covers the held branch's further unnamed
  results: Corollary 12 and the demoted Theorem 3 of 1403.6317, Lemma 4.2 of 1403.8075, and
  Lemmas 2.9 and 2.21 of 1404.4599. Exact IDs,
  evidence and analysis consequences: `analysis/s1-4/config/dispositions.json` and
  `analysis/s1-4/OBLIGATIONS.md`. Nothing is relabeled or newly harvested.
- **Held branch now eligible for review (Session 29):** decision 1 below deferred the held
  branch's admission review "until after Gate S1 closes". With Gate S1 closed (at the merge of
  Session 29's pull request), the review is eligible. **Order decided by Al on 2026-10-26
  (verbatim in the SD15 audit bullet above): NEXT stays at ADR-0010 per the phase order, with
  the held harvest review scheduled after it.** ADR-0010 in that ruling is the generator
  decision, renumbered ADR-0011 by ADR-0010. Nothing on the branch is approved by this
  eligibility. Its integration would add `groundtruth/` members, which `preserve()` still refuses
  after ADR-0012 ("unexpected addition in preserved scope"), and would change the population the
  effective-inventory tool pins (35 / 32), so the integration is its own inventory event with its
  own snapshot and tool revision (ADR-0012 D7); SOURCES entries are no longer byte-checked.
- **Later harvest held off main — DECIDED by Al on 2026-10-21 (decision 1, transcribed in
  `analysis/s1-orientation-repair/OWNER-DECISIONS.md`): preserve branch
  `claude/s1-harvest-candidates-28-30` at its original recorded commit
  `c3a3d369689d56bab3678f519904087b313daf4e`; defer its admission review and integration until
  after Gate S1 closes. This preserves the work and approves nothing: the five proposed labels
  are not approved, no future source identifiers are reserved, accepted main's harvest position
  stays at candidate 27 / `1403.4485` (candidate 28 / `1403.6317` saved), and merging or
  cherry-picking the stale branch is not authorized. The branch's manifests, producer provenance
  (`engine_commit` `8e222a4`), records and rows stay as recorded; Session 22's retrieval
  evidence (originals, saved pages, receipts, diffs, configs; 147 files, 17,824,213 bytes) is
  preserved with full SHA-256 pins under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\held-evidence\`, manifest
  copied to `analysis/s1-orientation-repair/evidence/held-evidence-manifest.json`; ten abstract
  pages exist only as hash-and-size receipts (listed there). Candidate 31 remains undecided.
  **Reviewed in Session 37 (S2 ordering step 3):** `analysis/s1-held-harvest-review/REVIEW.md` —
  five of five items agree. **Ruled by Al on 2026-10-28:** integrate now as the first later
  prefix (option (a)), with the labels as recorded (first bullet of this section).
  Do not request this decision again. The integration conditions recorded in Session 22 remain
  the description of the later review. — Session 22's original record of the hold: branch
  `claude/s1-harvest-candidates-28-30` (commit `c3a3d36` on base `8e222a4`, pushed) holds Route A
  candidates 28–30, examined in order and all admitted — five items: arXiv 1403.6317 Theorem 1
  (FLAW-0012, false by source document); 1403.8075 Proposition 4.1 (FLAW-0004), Theorems 4.3 and
  4.4 (FLAW-0005), all undetermined; 1404.4599 Proposition 2.22 (FLAW-0005, undetermined) — produced
  by a session that had not fetched and so ran the paused harvest instead of the examination. It
  is **not mergeable as it stands**: its S-0066..S-0072 collide with main's (they become
  the seven numbers after main's last entry — 0073–0079 when this was written; **0074–0080 since
ADR-0010 registered S-0073**), so every config hash, artifact id, record, log row and STATE row must be
  regenerated at the integration commit (the originals' bytes are pinned by SHA-256 in its
  S-entries and manifests); main's Rust inventory consumers fail closed on any manifest beyond the
  35 archived representations, so a later prefix needs its own snapshot and coverage run under
  ADR-0003 D8/D9 and never rewrites the Gate S1 table or the frozen S1.4 packet. Options for Al:
  (a) integrate after Gate S1 closes, as the first later prefix under its own record (the
  executor recommends this); (b) integrate before the examination with a new snapshot and an S1.4
  re-run on 36 items; (c) discard and re-harvest later from the log. Until Al rules, main's
  harvest resume point stays candidate 28 / `1403.6317`; the branch's HARVEST-LOG rows, records
  and HANDOFF Session 22 hold the full account, including candidate 31 opened but undecided.
- **Older questions — RETAINED by Al on 2026-10-21 (decision 3):** the seven truth questions,
  seven severity questions and the unnamed-result question keep their existing dispositions
  (`analysis/s1-4/config/dispositions.json`, `analysis/s1-4/OBLIGATIONS.md`). The questions the
  held branch introduced (an eighth truth reading, two further severity readings, further unnamed
  results, its item 36's "overcome in a later paper" wording) remain pending that branch's later
  admission review. Do not request this decision again. Session 37's review states them as
  Q-H1..Q-H5 (`analysis/s1-held-harvest-review/REVIEW.md` §3). **Ruled by Al on 2026-10-28:**
  Q-H1, Q-H2/Q-H3 and Q-H5 as recorded, by the standards main's items were read under. Q-H4's
  further unnamed results join the unnamed-result question.
- **Conditional projections, not inventory (Session 23):** the accepted population remains 35
  archived representations and 32 effective logical items. If all five held items were admitted
  unchanged, the errata/retraction-only total would be 36 items; the whole-corpus projection
  would be 37 effective logical items and 40 archived representations, absent other changes.
  These are projections conditional on a review that has not happened, not measured results.
  **Session 38:** the review happened (Session 37) and the five items are committed on the
  integration branch: 40 manifests under `groundtruth/`, counted from the files. The counts stay
  unmeasured until the inventory event (NEXT).

- **Q3 — RESOLVED 2026-10-13 (Session 7); the licence choice SUPERSEDED 2026-10-26 by ADR-0010**
  (PolyForm Noncommercial 1.0.0; the release boundary carried forward; earlier Apache-2.0 grants
  stand). The record as written: Al: SPDX `Apache-2.0` for SYNDEOM's original software
  and technical documentation, with the release boundary recorded in ADR-0000 Addendum 2 R1 and
  README.md § Licence. `LICENSE` committed (S-0019), `license` field set on `syndeom-manifest`.
  Nothing awaited. One note for Al, not a blocker: the Apache Software Foundation's own host
  (`www.apache.org`) refused connections from both tools throughout Session 7, so the committed
  text is the SPDX License List's rendering, cross-checked whitespace-for-whitespace against an
  independent copy (S-0019); a later session can diff it against the ASF file when the host answers.
  Session 31 (2026-10-27): the host still refused connections. The same SPDX text, fetched again
  and equal to S-0019's digest, is now `LICENSES/Apache-2.0.txt` (S-0019 addendum).
- **Advisory note ferry (Al's ruling R3 of 2026-10-13) — RESOLVED 2026-10-13 (Session 8).** In
  Session 7 the file was absent and a mid-session paste hashed to 25,652 bytes /
  `0ee1eda0…baa80` (lossy chat rendering), so the executor stopped and escalated. Al then placed
  the file at the path; Session 8 verified it byte-exact against the stated digest and committed
  it as ferried, body unedited (Advisory note bullet above; ADR-0000 Addendum 2 R3). Nothing
  awaited. Standing form for any future ferry: a path to bytes on disk, never a paste.
- **ADR-0003 flagged choices (i)–(v) — RESOLVED 2026-10-13 (Session 10; ADR-0000 Addendum 3).**
  (i), (ii), (v) ratified as written; (iii) ratified with the gate clarified (Gate S1 may close on
  a prefix of the Route A stream); (iv) overruled (NC and ND originals are `external`,
  digest-only). ADR-0001 amended for (ii) under `syndeom-manifest/2`; the S3 obligation is in the
  standing rulings. Nothing awaited.
- **SD15 audit request (Session 6) — CONFIRMED 2026-10-13 (Session 10).** The exam's PASS stands;
  Gate S0's closure is not provisional; no re-run. The memory paraphrase line was trimmed (digest
  in HANDOFF Session 10); the count-only convention is now SD15 by ADR-0005. Nothing awaited.
- **Q14 / Q15 — resolved in S0.** The sweep was ferried and placed (Session 3); no retrieval
  failed outright in S0.1 (the IAS page, HTTP 403, is corroborative only). Nothing awaited.
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## Preserved harvest resume procedure (not this session's priority)

*Session 38 note:* once the integration merges, the log's last candidate is `1404.4599`
(candidate 30), and a resumed harvest starts at candidate 31, `1406.7174`, which Session 22 opened
and left undecided. The harvest stays paused. The procedure below is kept as written for the
pre-integration position; its candidate-28..30 text is history.

**S1.1 — the tenth Route A harvest session, resuming from the log's last candidate** (ADR-0003
D7, D8 as amended: a prefix is enough). Procedure, executable cold:
(a) read `groundtruth/HARVEST-LOG.md`: run 1's query, window 2023-01-01..2026-09-30, ascending
arXiv id, **last candidate processed `1403.4485` (candidate 27 of 2,472)**; the next candidate is
`1403.6317` (v3 of 2024-10-15, math.SG, "Comparison of spectral invariants in Lagrangian and
Hamiltonian Floer theory", comment "We removed the wrong theorem from the first version, which was
brought to our attention by R. Leclercq" — identify the removed theorem from the diff, and read
v2's comment and date first: if v2 removed it, the correction may lie outside the window), then
`1403.8075` (v11 of 2026-01-09, math.GM, "Algorithm, probability, and prime numbers", a withdrawal
whose comment reads "Withdrawn due to a logical error in Section 4. In Proposition 4.1, the
argument incorrectly treats primality verification as equivalent to checking leading digits in
multiple base representations, and the subsequent independence assumption for fp(1),...,fp(n) is
unjustified. This invalidates the derivation of Theorems 4.3-4.4" — the helper fails on a withdrawn
version's missing e-print, decide from the abstract pages and v10 as for candidate 18; math.GM is
general mathematics, so read D2.1 and D2.6 with care), then `1404.4599` (v5 of 2024-01-12,
math.CO, comment beginning "The construction of finite n-acyclic groupoids in Section 2.4 is
flawed and I know of no direct repair: completion turns out to be incompatible with restriction to
proper subsets of the generator set, so that the induction towards Proposition 2.22 does not
stabilise as claimed. This problem has been overcome in arxiv:1806.08664 …"). A cumulative revision
comment can carry correction language from a version outside the window (candidate 20): read every
version's comment on its own abstract page before deciding D8. A comment's theorem number may
follow the corrected version's numbering (candidate 23): resolve the number in both versions
before locating. A remark or erratum in the corrected version that names further results is part
of the status document and yields further items (candidate 23, three; candidate 25, four from one
published erratum naming results at several depths of one dependency chain); links the erratum
does not name are not items (D2.3), and the records say which.
If a correcting version is itself a withdrawal (no e-print), the helper prints the abstract data
and fails on the source fetch (candidates 11, 13 and 18): decide from the abstract pages and the
original (candidate 18 was admitted that way, its comment describing and locating the error), or
add a `--no-sources` fallback first; a page-and-line reference is resolved against the arXiv PDF
(`pdftotext -layout`, as for candidate 18); the helper takes the correcting version as a bare
number (`4`, not `v4`);
(b) regenerate the candidate list — `python -I -X utf8 lab/route_a_candidates.py 2023-01-01
2026-09-30 <scratchpad>/route-a` (pages are cached under that directory only for the session
that fetched them; a fresh run re-fetches 57 ranges in about 30 minutes) — and confirm its
digest against the log's run record or record the new digest as run 2 with the difference stated
(arXiv adds and revises papers; the examined prefix is unaffected because ids are assigned at
submission and the finder sorts by id);
(c) examine candidates from `1403.6317` onward **in ascending id, skipping none**, each with
`python -I -X utf8 lab/route_a_examine.py <id> <correcting version> <scratchpad>/exam/<id>`
(both versions' abstract pages for licence and submission history, both e-prints, the TeX diff,
the AI-assistance scan) against ADR-0003 D2's seven criteria; log every verdict with its reason;
(d) for each admitted item, in this order: the S-entries for original and notice in SOURCES.md
(licence per D5 as amended: CC0 / CC BY / CC BY-SA / public domain → `committed`, NC, ND,
arXiv non-exclusive or assumed → `external`); the per-item config (the thirty-one Session-11/13/14/15/16/17/18/19/20 configs
are the pattern, including the null-truth one and the two-items-from-one-notice cases: ADR-0003 D4 fields, `statement_status` `null` + `"undetermined"` when the notice
leaves the truth open); then from `engine/`: `cargo run -q -p syndeom-ingest -- --config
<item.json> --bytes <original bytes> --root .. --engine-commit $(git rev-parse HEAD)`; then
`RECORD.md` into the item directory (the thirty-one Session-11/13/14/15/16/17/18/19/20 records are the pattern); the `.gz`
originals are binary in `.gitattributes` and the register checker;
(e) stop at the session's budget before examining the next candidate; update the log's counts
and last-candidate line, this file's Gate S1 table (S1.1, S1.3 rows), and set NEXT to the next
prefix. Grey zones are `escalated` and listed under standing escalations; nothing ambiguous is
ingested. Run the gate detached with a log before the commit (expect `GATE: GREEN`); commit
`[S1][ADR-0003] harvest Route A prefix through arXiv id <last id processed>: <n> admitted`.


## NEXT-SINGLE-ACTION

**S2 ordering step 4, continued: `s2_grammar` 0.2.0 — FLAW-0007 `unused` and FLAW-0013 on
`ring_chain`, with material generated from the atom library — after the 0.1.0 pull request
(branch `claude/s2-grammar-0.1.0`) merges.** Any ruling on its readings (85–99) is recorded
first. The grammar now carries the length axis for FLAW-0001; D6's remaining v0.1 classes on a
grammar (FLAW-0003 `cycle`, FLAW-0004, FLAW-0007 `unused`, FLAW-0009, FLAW-0010 `a`, FLAW-0011,
FLAW-0013) come version by version, the declaration-level ones first because their material is
mechanical on a chain (true irrelevant identities as unused lemmas and as padding paragraphs);
the grammars `ineq_chain`, `case_split` and `nat_induction` follow; the sweep (D11) runs when the
grammars realize their v0.1 classes. The `s2_template` sub-variants still without material
(FLAW-0004 `nontrivial-true` and `false`, FLAW-0010 `claim: false`, FLAW-0007 `vacuous`,
FLAW-0009 `native_decide`, the padding kinds `unused-lemma` and `verbose-rewrite`, padding
combined with an injected flaw) are later template versions, after the grammars.

Executable cold:
1. Open per ADR-0008 D1. If the session was launched with the §6 prompt, the SD15 disclosure comes
   first, by digest where ADR-0005 *Amendments* 1–3 allow it.
2. Confirm that the 0.1.0 pull request is merged, with a merge commit on fetched `origin/main`,
   and that `25ad34fe45c1eea8527b3aab47829b31c8685931` (the `s2_grammar` 0.1.0 goldens'
   `engine_commit`) is an ancestor of `main`. **If either fails, record that and stop.** Any
   later ruling — on readings 85–99, on FINDING-018, or on FINDING-015, FINDING-016 or
   FINDING-017 — is recorded verbatim, in STATE, HANDOFF and, where it amends the design,
   ADR-0011 *Amendments*, before anything else; a ruling that changes the grammar, a dial or a
   check bumps the producer again, and no golden is rewritten.
3. **`s2_grammar` 0.2.0, FLAW-0007 `unused` on `ring_chain`** (ADR-0011 D5: `count`,
   `length_steps`, cosmetic, `YES-TOOLING`, check T as 0.3.0 reads it — the lemma's name
   outside the main declaration's used-constants closure): true irrelevant identities
   generated from the atom library — a seed form the instance does not use, or two
   presentations of a drawn polynomial — inserted as lemma steps cited by nothing
   (`theorem …_unused_<i> : ∀ x y : ℤ, A = B := by intro x y; ring`), each its own instance as
   the template's insertion material is, drawn on `injector/FLAW-0007/1`; the records at the
   inserted declarations.
4. On the same branch, **FLAW-0013 on `ring_chain`** (D5's kinds; 0.4.0's `kinds`, `count` and
   `placement` dials with a positive `padding_ratio`): whole paragraphs of true irrelevant
   material from the same generated identities, each with its matching unused declaration,
   one record per paragraph in each file, `padding_ratio` measured as 0.4.0 measures it; which
   of D5's four kinds a generated identity can be is a reading to flag (a restated form of the
   theorem's own polynomial is a `re-derivation`; an unrelated identity an `unused-lemma`), the
   kinds without material refused for lack of it. Lean probes compiled first, one process at a
   time: a chain with one unused lemma and one with padding declarations, the audit tail's
   used-constants listing read for both.
5. The goldens: FLAW-0007 `unused` (informal, formal, pair) and FLAW-0013 (informal, formal,
   pair) at the short length (4, 0), seed 7, two production runs compared byte for byte; the
   earlier goldens stay regenerable under the producers and versions they name.
6. Never change the pinned tools (`s1_effective_inventory`, `s1_4_coverage`,
   `s1_later_prefix`), their packets, the Gate S1 table or the frozen S1.4 packet; never change
   an `s2_template` seed, dial, check or golden, nor an `s2_grammar` 0.1.0 golden, in this
   revision.
7. Gate the committed tree, fetch fresh before pushing, open a pull request and stop. Al merges,
   with a merge commit, never squash.

Bounds: the frozen packets, the registers, the constitution, the licence, the tag, the accepted
corpus, the first later prefix and the held branch's commits are untouched until a ruling says
otherwise. No cherry-pick,
no merge of the held branch, no force-push. The harvest stays paused after candidate 30;
candidate 31 (`1406.7174`) stays undecided. The seven truth questions, the seven severity questions
and the unnamed-result question keep their existing dispositions; the last also covers the held
branch's further unnamed results (Q-H4). Missing canonical lengths and external-byte verification
remain prerequisites for scoring real items (S4), not for S2.
