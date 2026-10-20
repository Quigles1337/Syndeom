# S1.2 verification record

Base revision: `8adcb4865abd3f6c3ac882b70563d82caca6207c`.
Base tree: `a1b2d003950972741a9768408554a9764a973f26`.
The final committed-tree gate is recorded in the draft PR with its exact revision/tree,
UTC completion, exit code and raw transcript digest. No historical receipt was regenerated.

## Checks executed

| Check | Actual result | Evidence |
|---|---|---|
| Primary-source retrieval | Fixed Judson 2025 PDF and pinned Mathlib objects HTTP 200, exact sizes/digests retained | `retrieval.json`; external `sources/` described in README |
| PDF passage/rights review | Page 41 / printed p.28 and copyright page rendered with `pdftoppm`, visually inspected; `pdftotext -layout`; licence appendix read | S-0067, README source/mapping table |
| Initial focused Lean import/type/dependency/axiom audit | Exit 0, 04:51:53.156402Z; selected theorem and alias use standard three, prime definition uses `propext` | `evidence/initial-audit.json`; output identical to `candidate-audit.txt` |
| Rehearsal 01 | Exit 1 before any Lean subprocess: LF blob vs CRLF checkout comparison | `evidence/rehearsal-01.failed.json`; harness corrected |
| Rehearsal 02 | Lean and locked library build exit 0; rustc exit 1 (missing dependency search directory) | `evidence/rehearsal-02.failed.json`, `rehearsal-02-compiler.txt`; harness corrected |
| Rehearsal 03 | All four subprocesses exit 0, ended 04:57:22.757990Z; pure Rust parsers/counts/JCS hashes/IDs agree; two negative format controls rejected | `evidence/rehearsal-03.json`, `candidate-audit.txt`, `api-probe.txt` |
| Orientation repository gate | Exit 0 / GATE: GREEN, 04:58:19.439313Z; 8,926 Lean jobs, golden `[propext]`, fmt/clippy clean, 37 tests, 150 tracked files, registers Q17/SD17/FLAW14/CLAIM7/S68 | `evidence/gate-orientation.json`, `gate-orientation.txt` |
| Preservation against base | 146 protected files identical; exact historical HANDOFF and SOURCES prefixes; 31 raw parents + one canonical representation; zero pairs; S1.3 producer/evidence ancestry retained | `evidence/preservation.json`; `check_preservation.py` |

The orientation gate began with a clean base checkout but finished while documentation/source
records were being drafted. Its receipt records both statuses and the unchanged index tree.
It confirms the unchanged production inputs and the tracked documents it actually read; it
is not represented as a gate of the later staged packet. The new candidate declaration's
audit is separate from the existing golden theorem audited by `scripts/check.sh`.

Current reproduction helper SHA-256 at the successful rehearsal:
`373c51eb54f8136a756725aca619b5db1906ed3abd63d9ffbe21a217c6058e59`.
Exact argument arrays, working directories, checkout/status, packet input hashes, timestamps,
subprocess exits and raw output hashes are in `rehearsal-03.json`. Candidate audit source
SHA-256: `76cad90b70f93d2b6a311be0f7602fba6415757e5c30c14185148ba61c3572fa`.
Raw audit transcript SHA-256:
`7ed258afaefc2cbb83d166592c9a9d64a4d834e6df0fa1c523e9cf9a7bed09c3`.

Committed transcript copies replace Windows CRLF with LF only, as required by the repository.
`evidence/transcripts.json` links original external paths, raw digests and committed digests.
The original raw logs/receipts remain in
`C:\Users\LEET\Codex\Projects\Syndeom-S1.2-work-2026-10-20`.
No old S1.3 or PR 2 evidence is an output destination.

## Final staged packet gate

`bash scripts/check.sh` exited **0 / GATE: GREEN** at **2026-10-20T05:17:18.734356Z**,
on index tree **`1034c92f7724e9f86f590c3ae344c1d3291c9c71`** over the base revision above.
The index tree and status were identical before/after the run. Lean completed 8,926
jobs, the golden used `[propext]`, fmt/clippy passed, all 37 existing tests passed,
172 tracked files passed byte checks, and Q17/SD17/FLAW14/CLAIM7/S68 had no undefined
references. Receipt: `evidence/gate-staged-packet.json`; transcript:
`evidence/gate-staged-packet.txt`. Raw transcript SHA-256:
`f25501a69ace5d40bb5b1518022ba21e9c1f37b170cc7d0d76b0ecb79e67611e`.

Recording this result adds the two new gate files and updates current session prose.
It does not change any tested helper, configuration, proof, production input or pin.
Byte/register, source-prefix and diff checks run again before the recording commit;
the full gate then runs on that exact clean commit, with its result in the draft PR.
A pre-publication fetch still found main at the recorded base; no reconciliation was
needed. Thirteen relative links in the new ADR/packet were checked and resolve.

## Commands

Candidate rehearsal actually executed:

```powershell
python -I -X utf8 analysis/s1-2/reproduce.py --output C:\Users\LEET\Codex\Projects\Syndeom-S1.2-work-2026-10-20\rehearsal-03 --source-dir C:\Users\LEET\Codex\Projects\Syndeom-S1.2-work-2026-10-20\sources
```

To reproduce, substitute a new output directory. Omit `--source-dir` to retrieve and
hash-check the fixed public URLs. See README for the four executed subprocess commands.
The helper refuses an existing output destination; it does not update retained receipts.

The full repository gate is invoked using Git Bash on this Windows machine:

```powershell
& 'C:\Program Files\Git\bin\bash.exe' scripts/check.sh
```

Preservation check (a new output file is required):

```powershell
python -I -X utf8 analysis/s1-2/check_preservation.py --output C:\Users\LEET\Codex\Projects\Syndeom-S1.2-preservation-replay.json
```

Additional hygiene checks: `git diff --check`, `rustfmt --edition 2024 --check
analysis/s1-2/api_probe.rs`, and `python -I -X utf8 scripts/check_registers.py .`.
These validate the analytical probe and documentation alongside the full gate.

## Explicit limits

No candidate manifest or production pair producer exists in this slice. Full candidate
`Manifest::check`, `Manifest::check_derived`, producer refusal/output-replay tests and corpus
admission were **not run**. The existing 37 tests remain existing infrastructure tests.
The analytical parser controls are not new producer tests. No proof of equivalence is used
as a substitute for source correspondence, and no source-proof formalization is claimed.
No S1.4 coverage analysis, S2 family, detector/contact evaluation, private evidence import,
schema change, dependency change or promotion of the held first parent occurred.


## Post-merge source-locator correction

The preceding record is historical PR 3 evidence for its original configurations.
The original 28–29 formal interval omitted the declaration; the corrected interval is
27–28 (one-based inclusive). See [correction/README.md](correction/README.md) and its fresh
receipt/log set for corrected hashes/IDs, actual line checks, rejected negative controls
and the successful replay. The old logs and helper/input hashes above were not rewritten.


## Implementation follow-up — separate real generation evidence

The original receipts above and the Phase A correction evidence remain unchanged.
Producer `eb1b96d5b8084ceb1de734fffc90c9b480ec32bf` was tested/gated and committed before
actual generation. `implementation/VERIFICATION.md` and its separate `evidence/` directory
record real manifests, both-file/parent-child checks, emitted Lean/type/axiom audit,
11-file fresh replay, refusal controls and scoped preservation. The new draft proposes
one pair for admission review. It does not relabel the old pure-API rehearsal as admission.
