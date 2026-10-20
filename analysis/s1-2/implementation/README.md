# S1.2 producer implementation and proposed admission

The narrow Rust `syndeom-pair` producer is implemented and committed. Its real archival
pair and canonical child passed actual-byte validation, emitted Lean/type/axiom checks,
fresh deterministic replay and refusal controls. **Admission is proposed on this branch;
the new implementation/admission PR stays draft and unmerged.**

| Revision | Exact identity |
|---|---|
| Observed merged PR 3 / main baseline | `1a8388b03017e4044cad01ac10e7e89f8fb5e38f` |
| Source-map correction, committed first | `d88fad2bb7b339c56bd9964167d3e0acddc8ee4e` |
| Tested producer, committed before generation | `eb1b96d5b8084ceb1de734fffc90c9b480ec32bf` |
| Producer tree checked by the pre-generation gate | `ad9cbee9a1939ddf3e0deb200a8afbe8db642d53` |

The subsequent artifact/evidence commit is the draft PR head and contains these records.
The final gate receipt and PR identify that exact commit/tree after it exists; no made-up
self-referential commit hash is embedded in its own files.

## Review the correction and artifacts

The [separate correction](../correction/README.md) changes the formal interval from 28–29
to **27–28, one-based inclusive**, and anchors to `#L27`. It computes the archive JCS
identity first, then propagates its link/hash into the child. The actual raw-line check
rejects the original wrong interval despite a valid raw digest. Three locator controls
and the corrected rehearsal passed. All old receipts still describe their original configs.

| Representation | Generated ID | Full JCS configuration SHA-256 |
|---|---|---|
| Archive | `s1_pair_archive-v0.1.0-094625fa10ff` | `094625fa10ffde723ba3a3ae47ff75db2b1994b8f96f26f6eb562dcbf653ede0` |
| Canonical | `s1_pair-v0.1.0-62fe0c816a6b` | `62fe0c816a6b0ef928f3e4c2fb7e2bd2e215171ebf6b6130202e851e1e9664a3` |

The raw sources, mathematical claim, reviewed statement transcription and 142-byte alias
are unchanged. The actual archive-manifest hash is `ce102662b2377bc96c4a127a20fa1a343b07ae50fd855728150cca03fee0d3fa`; the child-manifest
hash is `6903276efcd9fed08ddb25a0d2dddc10648ead8d409d8ce53a73553cf1a7d556`. Parent linkage, counts, context and exact file digests are in the
[archive RECORD](../../../groundtruth/s1_pair_archive-v0.1.0-094625fa10ff/RECORD.md) and
[canonical RECORD](../../../groundtruth/s1_pair-v0.1.0-62fe0c816a6b/RECORD.md).

The source correspondence review is distinct from kernel validation and from vacuous
empty-statistics fidelity PASS. The selected informal statement has **zero proof steps**.
The offered argument is Mathlib's existing proof, not a transcription of Judson's printed
argument. Complete raw objects/licensing context and canonical informal bytes remain external.

## Implementation and actual verification

[`engine/pair/`](../../../engine/pair/README.md) freezes the accepted source/config/recipe
contract, validates both raw and canonical files plus `check_derived` and stronger metadata
invariants, checks actual source lines, verifies the compiled executable-input closure,
and preflights the complete output set even during dry runs. New receipt directories are
exclusive; exact-byte retries preserve existing files and timestamps. Existing schema,
ingest/derive implementations and dependency pins are unchanged; the lockfile adds only
the new local package. Path checks include collisions, links/junctions, hardlinks and aliases.

The pre-generation full gate was green at 06:28:55Z with 51 tests. Production completed at
06:34:11Z: 33 commands behaved as expected, including emitted-file compilation/audit,
11-file byte-identical replay, 18 real-CLI mutation refusals and receipt-overwrite refusal.
The target/alias have the same full type and only accepted axioms. A separately labeled
synthetic Lean failure returned 1 and preserved the actual proof. Two earlier development
test failures and their fixes are disclosed, with retained logs.

See [VERIFICATION.md](VERIFICATION.md), the
[production receipt](evidence/production/verification-receipt.json),
[CLI receipts](evidence/production/receipts/), and
[preservation receipt](evidence/preservation.json).
The final exact artifact/evidence commit gate and hosted check are recorded in the draft PR.

## Reproduce into a new destination

Run from this branch or a descendant retaining the same producer input closure. The helper
uses the recorded committed producer and fresh HTTP retrieval by default. It builds Rust,
verifies inputs, generates into an isolated artifact root, compiles/audits the emitted Lean
file, replays into another fresh root and exercises the labeled controls:

```powershell
python -I -X utf8 analysis/s1-2/implementation/reproduce.py --root C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical --producer eb1b96d5b8084ceb1de734fffc90c9b480ec32bf --output C:\Users\LEET\Codex\Projects\Syndeom-S1.2-reproduction-01
```

The output path must not exist and must be outside the producer repository. Choose a new
path for each run. Omit `--artifact-root` for safe isolated reproduction; supplying the
repository root explicitly was used only for this branch's real proposed artifacts.
If the fixed upstream objects become unavailable, `--source-dir <retained-five-object-directory>`
reads retained exact bytes and records that method honestly; it does not accept new hashes.
The original retained source directory is
`C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\sources`.
The executed original top-level command used the same helper from the external task folder,
`--output ...\production-run-01 --artifact-root ...\Syndeom-S1.3-Canonical`;
the production receipt records all resulting concrete child commands.

Scoped preservation (new output file), then the ordinary gate:

```powershell
python -I -X utf8 analysis/s1-2/implementation/check_preservation.py --root C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical --producer eb1b96d5b8084ceb1de734fffc90c9b480ec32bf --output C:\Users\LEET\Codex\Projects\Syndeom-S1.2-preservation-new.json
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

No retained evidence path is a default output. The historical procedure-only checker retains
its original 31+1/no-pair assertions and is not the implementation preservation command.

## Counts and single next action

Accepted main at `1a8388b03017e4044cad01ac10e7e89f8fb5e38f`: **31 raw + 1 canonical, zero admitted positive pairs**.
Proposed branch: **32 raw + 2 canonical**, representing **one additional independent
positive item**. No PROVEN promotion; Gate S1 and S1.4 remain open. The separate first-parent
promotion hold is unchanged; paused harvest resumes at candidate 28, `1403.6317`.

**Review the correction, producer and both admission RECORDs with their actual generation,
audit, replay, refusal, preservation and final-gate evidence; accept or reject the one
proposed pair before counting it on main. Leave this draft open until that review.**
