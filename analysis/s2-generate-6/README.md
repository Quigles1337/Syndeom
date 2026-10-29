# S2 implementation packet 6 — `s2_template` 0.6.0 (ADR-0011, Session 45, 2026-10-29)

This packet records the sixth implementation session of ADR-0011 (D11): `s2_template` 0.6.0 —
FLAW-0010 `a` (D5: the fictional-reference lexicon, `shape`, `claim`; the formal file unchanged)
realized on `sqrt2_irrational`'s step 3, and FLAW-0003 (`direct`: the goal or its contrapositive
as a lemma proved by a forward reference to the theorem; `cycle`: the lemma and the theorem citing
each other, checked on the plan's premise graph) on the new seed `bounded_monotone_converges`,
FLAWS.md's FLAW-0003 example, with its witness module, both with goldens. Nothing here is a
result: no CLAIM, no fidelity sweep, no artifact under `corpora/`.

Branch `claude/s2-template-0.6.0`, from `main` `854a4b4520583c1343d64fd55fe392f0a1fe8a2f` (PR
30's merge):

| Commit | What |
|---|---|
| `ef41a10ce16db34c1ebe3ea66a456df97df83be1` | the 0.6.0 producer: the plan's `premises`, the citation lexicon and draw, the FLAW-0010 and FLAW-0003 materials, their shape rules and checks, the seed and its witness module, the tests, the crate README |
| `45bdbe3c03d046ce89ac92f723b86d199462d7ba` | the 12 goldens of 0.6.0, produced at `ef41a10c…`, the goldens README and the count (59 cases) |
| the record commit after it | this packet, ADR-0011 *Amendments*, STATE and HANDOFF Session 45 |

## What was built

- **FLAW-0010 `a` (D5).** A seed's FLAW-0010 material is replacement material at a main-proof
  step whose `cited` names the step's bare claim and the justification the reference stands in
  for; its one material dial is `claim` (`true`, gap: the cited step is true; `false`, fatal:
  needs the seed's false-step material, which no seed carries). The config adds `shape`
  (`journal`, `book` or `preprint`), which selects the venue list and is not a property of the
  material. The injector selects the material as it selects every other, then draws the
  author, the year, the venue and the result from the fixed lexicon of obviously fictional
  tokens (`Placeholder`, `Nonesuch`, `Fictitious`, `Hypothetical`; `19XX`, `20XX`; three venues
  per shape; `Theorem 4.2`, `Lemma 3.1`, `Proposition 2.5`, `Corollary 1.7`) with the next four
  draws of the stream `injector/FLAW-0010/1`, and the site step becomes `By [<author> <year>,
  <venue>, <result>]` followed by the bare claim. The record spans the reference: `inserted` is
  the reference and `replaced` the justification; the dials carry `shape`, `claim`, the drawn
  indices (`author`, `year`, `venue`, `result`), `reference`, `asserted` (the bare claim),
  `justification`, `site` and `original_steps`. The formal fragment is the faithful one, so the
  instance is `N/A` for every target and informal-only (`manifests_in`); the target `formal` is
  refused, and sub-variants `b` and `c` are refused (ADR-0011 Options 5).
- **The FLAW-0010 check (D5, S2.2).** A FLAW-0010 record is confirmed when the span holds
  exactly the reference; the reference equals the one the recorded lexicon indices name (rebuilt
  from the lexicon); every token of it, brackets and commas stripped, is a token of a lexicon
  entry; the step is `By <reference>` followed by the bare claim; and the faithful step
  (`original_steps` at the site) held the justification exactly once while the step does not.
  `check_seed` holds the material to that shape: sub-variant `a`, `N/A`, exactly the dial
  `claim` with the severity it fixes, the site step's text the bare claim, every fragment and
  every other step's text faithful.
- **FLAW-0003 (D5).** Insertion material (the 0.3.0 mechanism) whose site is a `theorem` lemma,
  fatal. **`direct`** (dial `form`: `restated-goal` or `contrapositive`) inserts Lemma 1 — the
  goal, or its contrapositive, as a closed statement — whose formal proof is `exact
  syndeom_s2_template_bounded_monotone_converges` (or `fun … => hn (… a ha hb)`), a forward
  reference to the main declaration, which is declared after the lemma in the file; the kernel
  reports an unknown identifier at the lemma's proof line, inside the lemma's declaration (check
  K, `YES-KERNEL`), and the key step cites Lemma 1. **`cycle`** (dial `cycle_length`, 2 in
  0.6.0) inserts Lemma 1 whose proof cites the theorem, while the key step cites Lemma 1;
  informal-only (`NO`; the target `formal` is refused), the formal file keeping a faithful proof
  — the lemma proved from Mathlib, the main declaration using it — which builds.
- **The plan's `premises` (D2).** A step may carry `premises`, each `{cites, phrase}`: `cites`
  is a lemma's declaration name or `theorem` (the main declaration), `phrase` the citing text,
  which occurs exactly once in its step. They render to nothing. `premise_edges` is the plan's
  premise graph (from the step's declaration to what it cites) and `premise_cycle` finds a cycle
  of a given length through a declaration. In 0.6.0 only FLAW-0003 `cycle` material carries
  premises; every other material is refused if it does.
- **The `cycle` check (D5, S2.2).** A FLAW-0003 `cycle` record is confirmed when the span holds
  the inserted lemma step; the plan's premise graph contains a cycle of `cycle_length` edges
  through the lemma, equal to the recorded `cycle` (the declarations visited from the lemma:
  `[…_lemma_1, …bounded_monotone_converges]`); and each recorded citation (`step`, `span`,
  `phrase`) is at its span in the rendered step, exactly once. `check_seed` holds the material to
  its shape: inserts a `theorem` lemma, fatal, `NO`, exactly the dial `cycle_length: 2`, no
  formal reference to the main declaration, each premise's phrase once in its step and citing
  the theorem or a lemma of the material, the cycle present. `direct` material is held to
  `YES-KERNEL`, exactly the dial `form`, the forward reference in its lemma's fragment, and no
  premises (its check is K).
- **The seed (D6).** `bounded_monotone_converges`: for a monotone (non-decreasing) sequence of
  reals bounded above, there is `L : ℝ` with `Tendsto a atTop (nhds L)` — FLAWS.md 0.1.0's
  FLAW-0003 example, outside the S1 corpus (ruling 1 of 2026-10-27) — in two aligned steps on the
  supremum (`refine ⟨⨆ n, a n, ?_⟩`; `exact tendsto_atTop_ciSup hmono hbdd`), with the witness
  module `lean/Syndeom/Seeds/BoundedMonotoneConverges.lean` (imported by `lean/Syndeom.lean`,
  audited by `lean/Audit.lean`: twelve axiom lines now) and the three FLAW-0003 materials.
  `sqrt2_irrational` gains the FLAW-0010 material at step 3: the FLAW-0004 material's bare claim
  and justification reused, every fragment faithful.
- **Versions (D1).** `Version::V0_6_0` is current; `generate` emits 0.6.0; FLAW-0010 and
  FLAW-0003 are refused by every earlier version, and the new seed is a seed of none of them;
  the 47 earlier goldens regenerate under the versions they name, with no golden rewritten.

## Evidence that the Lean material does what the records say

1. **Probes before the material** (`evidence/lean-probe/`, `lake env lean` from `lean/`, one
   process at a time, `run.sh`; 2026-10-29T00:42:02Z–00:42:46Z, about 9 s each):

   | Probe | Exit | What |
   |---|---|---|
   | `s2i_faithful` | 0 | the faithful argument: `[propext, Classical.choice, Quot.sound]` |
   | `s2i_direct_restated` | 1 | Lemma 1 restating the goal, proved by `exact syndeom_s2_template_bounded_monotone_converges`: one error, `5:8: error(lean.unknownIdentifier)`, at the lemma's proof line; the main declaration elaborates |
   | `s2i_direct_contrapositive` | 1 | Lemma 1 as the contrapositive, proved through the same forward reference: one error, `5:30`, the same line |
   | `s2i_cycle` | 0 | the cycle's formal file (Lemma 1 proved from Mathlib, used by the main declaration): builds, `[propext, Classical.choice, Quot.sound]` |
   | `s2i_witness` | 0 | the witness module's content: `[propext, Classical.choice, Quot.sound]` |

   The seed record and the witness module were written after these probes compiled.
2. **The goldens.** 12 cases — FLAW-0010 `a` `journal` on `sqrt2_irrational` (informal, pair)
   and, on `bounded_monotone_converges`, the control (informal, formal), FLAW-0003 `direct`
   `restated-goal` (informal, formal, pair), `direct` `contrapositive` (informal, formal, pair)
   and `cycle` (informal, pair) — each produced by `generate --dry-run` with seed 7 at
   `ef41a10ce16db34c1ebe3ea66a456df97df83be1` by two separate runs (`evidence/goldens-run6.py`;
   run A 2026-10-29T01:02:06Z–01:04:18Z, run B 01:04:18Z–01:06:34Z; every case exit 0, fidelity
   PASS, the provenance check passed against the commit before each run;
   `evidence/goldens-run6a-summary.txt`, `evidence/goldens-run6b-summary.txt`).
   `evidence/goldens-copy6.py` compared the runs file by file — every `proof.md`, `Proof.lean`
   and `manifest.json` byte-identical, every observation equal, one engine commit — compared
   each formal file with its probe (the probe's `#print axioms` tail dropped) or, for the
   FLAW-0010 pair, with the committed 0.4.0 control golden's `Proof.lean`, and placed run A's
   outputs under `engine/generate/tests/goldens/` with a `case.json` each
   (`evidence/goldens-production-report.json`, SHA-256
   `5dd0943f08ffce7805d7728bd141d6ff4e234bea57d9699d7f1c168924eb771f`, 30,879 bytes):
   - every one of the seven formal files equals its reference;
   - the four `direct` formal and pair cases fail to build (exit 1) with the first error at
     line 5, inside the lemma's declaration (lines 4–5; check K); `lean_axioms_standard` is
     unmeasured (`null`, with its reason); the control, the `cycle` pair and the FLAW-0010 pair
     build (exit 0) with the standard axioms, and the `cycle` pair's used-constants listing
     shows the main declaration using Lemma 1 and Lemma 1 using `tendsto_atTop_ciSup` and
     `le_ciSup`;
   - at seed 7 the injector draws author 2, year 0, venue 1 and result 3: step 3 of the
     FLAW-0010 cases reads `By [Fictitious 19XX, Annals of Imaginary Results, Corollary 1.7]
     $p$ is even. Write $p = 2k$.`, the record spanning code points 3–64; the pair's
     `Proof.lean` is the faithful file;
   - the `cycle` record carries `cycle` `[…_lemma_1, …bounded_monotone_converges]` and the
     citations `by the theorem` (step 1, span 139–153) and `by Lemma 1` (step 2, span 27–37);
   - every instance is confirmed (`flaws_confirmed` 1); every flawed artifact is
     `proof_status: invalid`, `correct: false`, of a true statement.
   - The receipts, with build stdout and stderr, are under the external root
     `C:\Users\LEET\Codex\Temp\syndeom-s2-template-060-20261029T0029Z\goldens-6a\` and
     `goldens-6b\`; the sorted lists of the twelve `receipt.json` SHA-256s hash to
     `82f2cbda1cc3ab278aea2399f893bacbe90d765a5c9042ac58c25619e1d9c8c8` and
     `6183f15a984cbb12815b0da0fe391d6278b5bb5bdf9a2ea846a88b3ace9b43bd`.
3. **The artifact ids** (`s2_template-v0.6.0-<config_hash[0:12]>-s7`): FLAW-0010 `journal`
   `85901aa05744` / `056268672274` (informal / pair); `bounded_monotone_converges` control
   `8c528ce85bee` / `4f1af4b37228` (informal / formal); `direct` `restated-goal` `519269042b85`
   / `a3ed502beb34` / `dd04799bf167`; `direct` `contrapositive` `21d619bec38a` / `b24c3986bb97`
   / `89eefff7cd6d` (informal / formal / pair); `cycle` `53fd4a38ecf5` / `97f01aae2ad7`
   (informal / pair).
4. **Tests and lint.** The crate's 33 tests pass (goldens 1 over 59 cases, properties 22,
   provenance 2, rng 5, writers 3); `cargo clippy --all-targets --all-features -- -D warnings`
   and `cargo fmt --check` are clean. New: `d5_flaw_0010_replaces_the_justification_by_a_fictional_reference`
   (the draw order, the dials, the step text, the faithful formal file, the single informal
   record and its span, the commanded `lean_build` `pass`, forty seeds per shape all in the
   lexicon with the shape's venue, the lexicon check's refusals, twelve config refusals, the
   earlier versions) and `d5_flaw_0003_direct_forward_references_the_theorem_and_cycle_cites_in_a_circle`
   (both `direct` forms' files, lines and records, the K confirmation at the proof line and its
   failures, the `cycle`'s premises, cycle, citations and faithful formal file, the composite of
   a cycle, nine refusals, the earlier versions, and the seed shape rules of both sub-variants);
   `d6` checks the FLAW-0010 material's shape rules and that premises belong to cycle material
   only; `every_case` adds every shape of FLAW-0010 and refuses the cycle's formal target;
   `d10_locations` admits FLAW-0003's inserted lemma beside FLAW-0007's. The gate on the
   committed tree is recorded in the pull request and under the external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-10-29T01:06:58Z, outside the gate. It loaded 1,381
advisories (advisory-db commit `40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`), scanned 35 crate
dependencies, and reported no vulnerability (exit 0; `evidence/cargo-audit.txt`).
`engine/Cargo.lock` is unchanged by this revision.

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** the diff of `ef41a10c…` — `engine/generate/src/plan.rs` (`Premise`,
`Step.premises`, `premise_edges`, `premise_cycle`), `src/template.rs` (the citation lexicon,
`citation_reference`, `citation_tokens_in_lexicon`, `draw_citation`, `CitedStep`,
`check_cited_material`, `mentions_identifier`, `check_circularity_material`, the FLAW-0010 path
of `parse_spec`, `check_spec` and `inject`, the cycle dials, the two checks in `assemble`),
`seeds/sqrt2_irrational.json` and `seeds/bounded_monotone_converges.json`. The threat model is
packet 1's: an offline research CLI whose assets are the integrity of generated ground truth and
the operator's machine, and whose trust boundary is an artifact directory handed to `verify` and
a config handed to `generate`. New this revision: a config dial (`shape`) selects among three
compiled-in venue lists, and the plan carries a premise graph the producer walks.

**Pre-scan:** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate roots and
in the manifest crate; no external dependency was added; clippy at `-D warnings` is clean.

**Panic and index sweep of the diff:** no `unwrap`, `expect` or `panic!` was added. The indexes:
the four drawn lexicon indices are `below(len)` of their lists, and `citation_reference` uses
`get` and returns `None` out of range; `m.steps.get(m.site as usize - 1)` behind `m.site >= 1`;
`spec.dials["shape"]`, `["claim"]` and `["cycle_length"]` are map indexes behind `exact_keys`
or behind the material match (a spec reaches the cycle dials only when its dials equal the
material's `{cycle_length: 2}`), the pattern packets 4 and 5 recorded; `d[k]` on a
`serde_json::Value` yields `Null` for an absent key; `mentions_identifier` slices at byte
offsets `match_indices` returned, which are character boundaries; `premise_cycle` recurses over
simple paths (a node never repeats), so its depth is bounded by the plan's distinct declarations
and its `length` is the material's pinned dial.

**Serde:** `CitedStep` and `Premise` carry `deny_unknown_fields`; `Material.cited` and
`Step.premises` / `FlawedStep.premises` default to absent and empty, so every earlier seed and
every golden `case.json` still parses. The config's `claim` is required to be a boolean and
`shape` one of three strings before any draw.

**Subprocesses and `verify`:** unchanged. No config value reaches the proof text: the reference
is assembled from compiled-in lexicon entries selected by drawn indices, the bare claim and the
lemma texts are seed material, and `shape` only picks a list. `verify` still compiles only bytes
the producer itself emits and names only the producer's main declaration (FINDING-011).

```
FINDING-016: the premise-cycle walk is a depth-first search over simple paths
Severity:    Informational
Category:    Logic
Location:    engine/generate/src/plan.rs (premise_cycle)
Status:      Open (backlog)
Description: `premise_cycle` enumerates simple paths from the start declaration up to the
             requested length; on a large, densely cited plan that is exponential. In 0.6.0 the
             graph is seed material of two or three declarations and `cycle_length` is pinned
             to 2 by the only material that carries premises, so no config can grow the walk.
Recommendation: when `s2_grammar` generates premise graphs of commanded size, replace the walk
             by a breadth-first search for the shortest cycle through the start, and bound
             `cycle_length` by the plan's declaration count.
```

FINDING-015 (informational, backlog: a composite's truth witness is taken on the seed's word)
stands and now also covers the new seed's reference, `BoundedMonotoneConverges.lean`, which
the gate builds and audits. FINDING-006 (accepted risk), FINDING-007 (backlog), FINDING-009,
FINDING-010, FINDING-012 and FINDING-013 (accepted) stand as packets 1–5 record them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it. They continue
packet 5's numbering.

52. **The reference's form.** D5 says "a fictional reference built from a lexicon of obviously
    fictional tokens" and FLAWS.md's example is `[Placeholder 19XX, Theorem 4.2]`. The reading:
    `[<author> <year>, <venue>, <result>]`, the example's shape with a venue, drawn from the
    list the spec's `shape` names (journals, books or preprint identifiers, all obviously
    fictional), so that `shape` changes the reference's bytes. The lexicon's entries are the
    executor's; a change to them, or to the form, bumps the producer's version.
53. **The draw order** — author, year, venue, result — on the stream `injector/FLAW-0010/1`
    after the material draw, is part of the producer's rng definition (D4). The indices are
    recorded as the "lexicon indices drawn".
54. **`shape` is a config dial, not a material property.** A seed's FLAW-0010 material has the
    one dial `claim`; the injector matches the spec's dials without `shape` and records both.
55. **The record spans the reference.** D5's check says "the span equals the generated
    reference", so the location is the reference's span inside step 3, `inserted` is the
    reference and `replaced` the justification it stands in for; the whole faithful step is in
    `original_steps`. The check also requires the step to be `By <reference>` followed by the
    bare claim, and the faithful step to have held the justification exactly once — the
    FLAW-0004 reading (41) applied to a replacement rather than a deletion.
56. **`claim` is both a reserved recorded-dial name (FLAW-0004's bare claim) and FLAW-0010's
    own config dial (D5).** The reservation is lifted for FLAW-0010 only; the recorded FLAW-0010
    bare claim is named `asserted`.
57. **FLAW-0010 is `N/A` for every target, a pair included,** because the formal file is
    unchanged (FLAWS.md: `N/A` for the informal form; the `YES-KERNEL` formal analogue — a
    nonexistent lemma name — is not produced); a pair's `lean_build` is therefore commanded
    `pass`, and the record is informal-only, as FLAW-0009's is formal-only.
58. **`claim: false` is refused for lack of material** (it needs the seed's false-step material,
    which `sqrt2_irrational` does not carry — the same gap as FLAW-0004 `false`); `b` and `c`
    are refused as deferred (Options 5).
59. **`direct`'s site is the inserted lemma.** The informal change is two-fold (Lemma 1
    inserted, the key step rewritten to cite it), and the formal failure lives at the lemma's
    declaration, so the record's site is the lemma (one record per file), and the key step's
    rewrite is in `original_steps` — the 0.3.0 insertion mechanism.
60. **`direct` drops the faithful step 1.** The FLAWS.md example's "let $L = \lim a_n$" takes
    the place of the supremum step, so the material has two steps (Lemma 1 and the key step)
    and the dropped step is recorded in `original_steps` and `original_fragments`.
61. **The `cycle`'s formal file "keeps the faithful proof"** is read as: a faithful, valid
    proof — Lemma 1 proved from Mathlib and used by the main declaration — not the control's
    bytes, because the informal argument has a numbered lemma and D2 renders a `Lemma N.` step
    as a declaration. The file builds with the standard axioms. The alternative, a cycle whose
    lemma has no formal declaration, is not expressible under D2's lemma rule.
62. **`premises` (D2) enters the plan** as an optional per-step field, in 0.6.0 carried only by
    `cycle` material; `cites` is a lemma's declaration name or `theorem`; the record's `cycle`
    is the list of declarations visited from the lemma back to itself (`[lemma_1, main]` for
    length 2).
63. **The `cycle` check is structural**: the recorded cycle equals the plan's premise cycle
    through the lemma, and each citation phrase is at its recorded span in the rendered step
    (D5's "the plan's premise graph contains the cycle and the two citation phrases are at the
    recorded spans"); no Lean observation touches it, so a pair's formal build (which passes)
    neither confirms nor fails the record.
64. **The seed's statement form**: monotone read as non-decreasing (`Monotone a`), bounded
    above (`BddAbove (Set.range a)`), convergence to some `L : ℝ`; the witness is
    `tendsto_atTop_ciSup` applied at the supremum (`method: lean-kernel`, the module audited by
    the gate), not a Mathlib declaration of the existential form.
65. **`direct`'s location is the lemma's first line (4)** while Lean reports the error at the
    proof line (5): the record addresses the declaration at its first line, as the 0.3.0
    lemma-site records do, and check K accepts any first error inside the declaration's lines.
