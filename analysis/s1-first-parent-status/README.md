# First-parent status-support review

**2026-10-20 UTC · written witness checked; amendment proposed; promotion held.**

At a=1,b=2,k=2, the unrestricted character expansion in arXiv:0706.0357v70,
Lemma 1.1's proof gives **1 versus 0**. At a=1,b=2,s=2, the lemma's residual is
**log(2)/3 > 0**. These are separately checked executor witnesses. The author's v71
notice identifies an error; it does not supply this counterexample or explicitly
declare falsity. Only this lemma and displayed step are reviewed.

- [WITNESS.md](WITNESS.md): exact source hashes/locators, both witnesses, convergence,
  nonvanishing, a second derivation, attribution and the formalization boundary.
- [Witness.lean](Witness.lean): actual modulo-2 character, phases, coefficient and finite
  inequality; convergent geometric series and positivity. The analytic bridge is written,
  not fully formalized. Actual eight-theorem axiom output is retained.
- [PROPOSED-RULING.md](PROPOSED-RULING.md): the exact owner decision, policy conflict,
  identity consequences, acceptance requirements and separated disposition statuses.
- [candidate-config.json](candidate-config.json), [amendment-map.json](amendment-map.json)
  and [generated candidate](candidate/s1_ingest-v0.1.0-15ab865466d6/manifest.json): complete
  proposed raw amendment, generated outside the live corpus with the existing Rust CLI.
- [VERIFICATION.md](VERIFICATION.md): actual checks, failed prototypes, reproduction,
  preservation, skipped checks and final committed-tree gate handling.

The live corpus remains 34 representations / 32 logical items. The two raw identities
in the proposal refer to one existing source claim; no 33rd item is admitted.
The old embedded metadata remains unchanged, so a sidecar alone does not make it an
eligible canonical-production parent. The owner ruling and a consistent integration
are still needed. No new canonical artifact is produced.

PR 5 was merged by the owner at **2026-10-20 20:41:46Z**. This packet starts at main
`1a76d9ac21cd5d2e4311f9164178327c0ca128bb`, tree
`3b9028c0502add53ac646108132d89ba5f1e5666`, identical to PR 5's head tree. The historical
S1.4 corpus `e6a7cae4392dd1565ee57f9fb727cc01e453687d` and reporting implementation
`4e46564ac62bc2bd7a5bef54265e9b2dd26772bb` remain frozen. Every byte under `analysis/s1-4/`,
including old pending-review wording, its disposition overlay and first-write bins, is
preserved as historical evidence. This review is a separate proposed event.

**Single next action:** owner reviews the exact proposed ruling and generated candidate.
Gate S1 remains open in this informed session. Its later cold-start examination must
be a separate fresh conversation with the constitution's §6 resume prompt verbatim
and alone. Harvest remains paused after `1403.4485`; candidate 28 / `1403.6317` is next.
