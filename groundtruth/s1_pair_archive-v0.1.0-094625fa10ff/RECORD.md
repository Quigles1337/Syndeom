# Proposed admission — Judson/Mathlib archival pair

Artifact: `s1_pair_archive-v0.1.0-094625fa10ff`; generator `s1_pair_archive` 0.1.0.
Config JCS SHA-256: `094625fa10ffde723ba3a3ae47ff75db2b1994b8f96f26f6eb562dcbf653ede0`.
Manifest exact-byte SHA-256: `ce102662b2377bc96c4a127a20fa1a343b07ae50fd855728150cca03fee0d3fa`.
Canonical child: [`s1_pair-v0.1.0-62fe0c816a6b`](../s1_pair-v0.1.0-62fe0c816a6b/RECORD.md).

Both full raw pair files are external: the 2,207,438-byte official PDF
(`4f74587b61861b4790d613f2adb64699f90fd1d07760204f1d8fc9fcaf63d14f`)
and the 5,328-byte pinned `PrimeFin.lean` Git blob
(`92a5630a19bff74ade4088f1253cf75b0ac8143c002e6b604ac0b77a39c9ecf6`).
The external archive also retains `Infinite.lean`, `PrimeDefs.lean`, and `mathlib-LICENSE`.
Its exact layout and digests are in the child source map and production receipt.
No selected extract or SYNDEOM alias is substituted for these complete raw objects.

Original production external directory:
`C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\external\s1_pair_archive-v0.1.0-094625fa10ff`.
Fresh reproduction retrieves these objects from the fixed URLs and refuses mismatched bytes.
Raw counts and the raw main declaration remain null except for byte length/hash.

**Admission status:** proposed on `codex/s1-2-positive-pair-implementation`; review
pending in the new draft implementation/admission PR. These are real generated objects,
not procedure rehearsal artifacts, but their presence on this branch does not constitute
accepted-main admission or PROVEN promotion. Gate S1 remains open.

Main baseline `1a8388b03017e4044cad01ac10e7e89f8fb5e38f` has 31 raw parents, one canonical representation and zero admitted
S1.2 positive pairs. The branch contains 32 raw parents and two canonical representations.
The new archive and child represent **one additional independent positive item**, not two.

## Source correspondence and argument review

The claim of record is Thomas W. Judson, *Abstract Algebra: Theory and Applications*,
Annual Edition 2025 (August 1), Theorem 2.2.6, section 2.2.2, printed p.28 / PDF page 41.
The complete official PDF and its copyright notice/Appendix A remain external. The original
[candidate review](../../analysis/s1-2/README.md) records the statement and its same-page
prime definition, the integer-to-natural correspondence, quantifier/conclusion scope,
zero-divisor convention, and lack of added parameters or typeclass hypotheses.

The selected informal representation is **statement only, zero proof steps**. Judson's
printed proof was not selected or transcribed. The offered argument is Mathlib's
`Nat.infinite_setOfPred_prime` at
`d13f23b723b8a846827a245b89c10fc7d3f11612`, whose pinned source declaration/body are
one-based lines **27–28 inclusive**, mapped to canonical declaration line 3.
The argument passes from unbounded primes to an infinite set; its dependency bodies,
including the factorial/minimum-prime-factor construction, appear in the fresh audit.
It is not a claim that Judson's argument was translated line by line.

S-0067 identifies the publication and GFDL-1.2-or-later grant, with no invariant sections
or cover texts. S-0068 identifies the Apache-2.0 Mathlib source and authors. SYNDEOM's
pair selection, literal transcription and original alias wrapper are separately attributed.
No source-author endorsement or textbook relicensing is implied. External storage is the
chosen operational design under ADR-0007, not an assertion that source licences forbid reuse.

The proposed manifest has `statement_status.value = true`, `method = mathlib`,
`proof_status = valid`, `correct = true`, and no flaws. These labels concern the reviewed
claim and Mathlib offered argument. They are proposed admission inputs and do not override
the pending review boundary. `commanded` and `achieved` are empty; fidelity PASS is
**vacuous for semantics**, separate from this written source correspondence review.

## Executed generation and checking

The corrected procedure/configuration commit is `d88fad2bb7b339c56bd9964167d3e0acddc8ee4e`.
The tested producer was committed **before generation** at
`eb1b96d5b8084ceb1de734fffc90c9b480ec32bf`. Its pre-generation gate ended
2026-10-20 06:28:55Z: Lean build and golden audit passed, fmt/clippy clean,
51 Rust tests passed, byte/register checks passed, `GATE: GREEN`.

Actual-source production ran 2026-10-20T06:33:04.160227+00:00 through
2026-10-20T06:34:11.430260+00:00. All five freshly downloaded objects matched their frozen
hashes and byte counts. Actual commands, exits, environment, hashes and logs are in the
[production receipt](../../analysis/s1-2/implementation/evidence/production/verification-receipt.json)
and [CLI receipts](../../analysis/s1-2/implementation/evidence/production/receipts/).

- Both actual raw files passed `Manifest::check`; both actual canonical files passed
  `Manifest::check`; `check_derived` and all ADR-0007 stronger invariants passed.
- The actual parent identity/config/producer, full raw output set, shared
  source/item/licence/status/toolchain/ground-truth fields, exact recipe and mapped
  source lines were checked. Generated files were read back and checked again.
- The executable's 20 compiled input hashes matched both the producer Git blobs and
  current checkout. Pinned Mathlib Git blobs were checked; exact LF-to-CRLF checkout
  conversion was separately recorded, without changing raw archival bytes.
- Fresh generation replay produced all 11 deterministic files identically, including
  both manifests, both source maps, the alias and all external source/output objects.
  Run timestamps, destinations and command lines legitimately differ between receipts.
- Identical retries preserved artifact bytes and mtimes. Dry runs checked the whole
  output set; a fresh archive dry run wrote no artifact. Eighteen real-CLI mutation
  controls returned 2 with failure receipts and unchanged fixture bytes/mtimes;
  reusing a retained receipt destination also returned 2 without overwriting it.

The actual emitted alias compiled, then its full elaborated type matched the target:

```text
@Set.Infinite.{0} Nat (@Set.ofPred.{0} Nat fun (p : Nat) => Nat.Prime p)
```

The target, alias, `Nat.not_bddAbove_setOfPred_prime`, `Nat.exists_infinite_primes`,
and `Set.infinite_of_not_bddAbove` use only `propext`, `Classical.choice`, `Quot.sound`.
`Nat.prime_def` uses only `propext`. No `sorryAx`, added axiom or `Lean.ofReduceBool`
was reported. See the [fresh emitted-byte audit](../../analysis/s1-2/implementation/evidence/production/logs/emitted-type-axiom-audit.txt).
The intentionally false `True.intro : False` example was a separate synthetic Lean
subprocess failure control (exit 1), never an admitted artifact or an empirical
statement-fidelity result. The emitted proof remained unchanged.

The final committed implementation/artifact revision is gated separately; its exact
SHA/tree, command and result are recorded in the draft PR and the external final-gate
receipt without modifying that checked commit. See the [implementation evidence
review and reproduction instructions](../../analysis/s1-2/implementation/README.md).

## Exposure and remaining review

This candidate was exposed during development. It is not unseen evaluation data,
a population sample, held-out isolation evidence, detector evidence or private
calibration input. No prior corpus item, source status, receipt, dependency pin or
historical record is relabeled. The first-parent promotion hold and harvest resume at
candidate 28, `1403.6317`, remain unchanged.

**Next action:** review the correction, producer and both admission RECORDs with the
actual-byte, Lean, replay, refusal, preservation and final-gate evidence; accept or
reject this one proposed pair before counting it on main. This task leaves the draft open.


## Post-merge disposition — 2026-10-20 UTC

The owner, Quigles1337, merged PR 4 at **18:10:13Z**. Main/merge commit
`e6a7cae4392dd1565ee57f9fb727cc01e453687d` and PR head
`0b30d782163ed961262dbf70328dc1d16cf58746` share tree
`2e4d454048ac916a0047bd908ae3051ba1b981af`. This is the owner's completed admission
action; the earlier proposed/pending wording is the preserved original record.

The ADR-0007 D6 evidence was re-examined: actual raw/canonical file checks, stronger
parent/child metadata checks, emitted full-type/axiom audit, replay/refusal tests and
the final exact-commit gate are recorded in the implementation packet and PR 4. The
source correspondence and selected Mathlib argument are documented separately from
kernel acceptance and vacuous mechanical fidelity PASS. No substantive unmet D6
obligation was identified in that retained packet. GitHub has no separate reviewer
approval; this addendum invents none and claims no new Lean execution or PROVEN promotion.

Count the archive and canonical child as **one admitted logical positive pair**.
The informal selection remains statement-only (zero proof steps), development-exposed,
and unsuitable as a positive-length informal proof control. Storage and source labels
are unchanged. The S1.4 compatibility replay separately rebuilt the unchanged producer
and reproduced all 11 output files from retained bytes in fresh destinations; its scope
and execution evidence are in `analysis/s1-4/`.

Gate S1 remains open. The separate first-parent hold is unresolved; current qualifications
and next action are in `analysis/s1-4/OBLIGATIONS.md`. All preceding bytes are preserved.
