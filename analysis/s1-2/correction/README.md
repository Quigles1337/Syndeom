# S1.2 merged source-locator correction

Phase A baseline: PR 3 merged at `1a8388b03017e4044cad01ac10e7e89f8fb5e38f`
(2026-10-20 05:48:01 UTC); reviewed head `c157e4e50b011fd11fc08b31d6963900923d2564`.
This correction is committed separately before producer implementation; the following
implementation record names that exact correction SHA.

The pinned Git blob `Mathlib/Data/Nat/PrimeFin.lean` has 5,328 bytes, SHA-256
`92a5630a19bff74ade4088f1253cf75b0ac8143c002e6b604ac0b77a39c9ecf6`.
Line 27 declares `infinite_setOfPred_prime`; line 28 contains its entire proof term;
line 29 is blank. The original **28–29** interval omitted the declaration.
Current maps use **27–28, one-based with both endpoints inclusive**, and links use `#L27`.
The canonical declaration starts at one-based line 3 with its proof term on line 4.
Unicode statement spans retain their existing zero-based half-open convention.

`../source_locator.py` checks the actual selected declaration/body lines, the archive
locator, and canonical alias location. It is a narrow reviewed-source check, not a
general Lean semantic parser. Three controls were rejected: the original 28–29 range
despite a valid raw hash; a declaration-only interval; and a wrong canonical line.

## Executed identity dependency chain

The unchanged Rust `syndeom-manifest` JCS and identity functions, called by
`config_identity.rs`, computed the corrected archive first. Its full hash and ID
were then assigned to the child before its identity was recomputed.

| Representation | Original config hash / ID | Corrected config hash / ID |
|---|---|---|
| Archive | `ad12b7ada4490b5011a231941dee44e00968a5b8ec6f1863c1183db905d7e46b` / `s1_pair_archive-v0.1.0-ad12b7ada449` | `094625fa10ffde723ba3a3ae47ff75db2b1994b8f96f26f6eb562dcbf653ede0` / `s1_pair_archive-v0.1.0-094625fa10ff` |
| Canonical | `7d397688349807e8ae35d9a7987512fda49ddd8caad8a83b2f2225b99eba3eea` / `s1_pair-v0.1.0-7d3976883498` | `62fe0c816a6b0ef928f3e4c2fb7e2bd2e215171ebf6b6130202e851e1e9664a3` / `s1_pair-v0.1.0-62fe0c816a6b` |

These remain prospective identities in this correction commit. No manifest or pair is
generated here. The primary claim, raw source bytes, 118-byte informal output and
142-byte alias are unchanged. Their SHA-256 hashes remain respectively
`56983d03aa658d0315659b88147f6277e7cb6b883e3ce5a6db099094b5949b95` and
`038c70bf73cacf9eca975bd58ac6d64c23f3227474654d0c18ee2a93f5be09a2`.

## Actual correction verification

Command executed from the repository:

```powershell
python -I -X utf8 analysis/s1-2/reproduce.py --output C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\correction-rehearsal-01 --source-dir C:\Users\LEET\Codex\Projects\Syndeom-S1.2-work-2026-10-20\sources
```

All four subprocesses exited **0**, completed **2026-10-20T06:00:46.510404+00:00**.
The corrected positive source map passed and all three negative locator controls were
rejected. The selected target/alias audit again uses only `propext`, `Classical.choice`,
`Quot.sound`; `Nat.prime_def` uses `propext`. Pure APIs confirm the corrected identities.
Helper SHA-256: `7ced484ef9492e1098d4f31663b89668f11bad604b00f5d066d2a344e3e28ea8`.
The receipt records checkout/index/status and every helper/input digest.

Fresh logs/receipts are under `evidence/`; raw copies stay at the external task directory
above. `transcripts.json` records CRLF-to-LF display copies. **All original
`../evidence/` receipts, transcripts, retrieval metadata and failed-run evidence remain
byte-for-byte historical evidence for the original configurations.** They were not
regenerated to claim verification of the corrected configs. No unexpected correction
rehearsal failure occurred; the three intentional negative controls are identified.

The preservation receipt checks against merged main, protects production code/pins,
original mathematical/output files and all old evidence, and verifies historical
HANDOFF/SOURCES prefixes and unchanged 31 raw + one canonical / zero-positive-pair counts.
The complete producer/artifact revision receives the required full gate and hosted checks
in Phase B; this correction is not represented as a producer/admission run.
