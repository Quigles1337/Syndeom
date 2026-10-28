# S2 implementation packet 2 — `s2_template` 0.2.0 (ADR-0011, Session 41, 2026-10-28)

This packet records the second implementation session of ADR-0011 (D11): `s2_template` 0.2.0
under Al's rulings 3 and 5 on PR 26 and the owner note of 2026-10-28 — build-time input hashes
(FINDING-005 closed), flaw counts per instance, and the four next seeds with their witness
modules and goldens. Nothing here is a result: no CLAIM, no fidelity sweep, no artifact under
`corpora/`.

Branch `claude/s2-template-0.2.0`, from `main` `de3e4a62fde75eea2c80c17ac6210955bb868a0f` (PR
26's merge):

| Commit | What |
|---|---|
| `da2bd83b84b3ff5202ca900c9bcff9bf6f9eddb8` | Al's owner note of 2026-10-28 recorded before anything else (ADR-0011 *Amendments*, STATE, HANDOFF) |
| `af28cc96aa75cbabb0a6171d4c636a8cbc87f773` | the 0.2.0 producer: provenance, version selection, counts per instance, four seeds, four witness modules, the tests |
| `d7b5d8185ef683ef3618f01a2d9cc6ba7983cee4` | the 14 goldens of 0.2.0 and the goldens README |
| the record commit after it | this packet, STATE and HANDOFF Session 41 |

## What was built

- **Provenance (D9 as amended by ruling 5).** `engine/generate/input-set.txt` names the
  producer's input set (three directories, six files); `build.rs` compiles the SHA-256 of every
  file in it, and the pinned `rustc --version`, into the binary; `src/provenance.rs` requires,
  for every input the commit names, that the commit's blob, the checkout's bytes and the compiled
  hash agree, refuses an input the commit holds that the binary did not compile or the reverse,
  refuses a checkout file the commit lacks, and checks the runtime `rustc` and the absence of
  project-local Cargo overrides. The receipt records the check under `provenance`.
  `tests/provenance.rs` checks that the compiled set covers exactly the input set on disk; the
  CLI refusal test adds an ancestor commit that lacks the crate (`f9f9730`; exit 2, no receipt).
- **Versions (D1).** `Version {0.1.0, 0.2.0}`; `prepare_with(version)` and `seeds(version)`;
  `generate` emits 0.2.0; `regenerate` and `verify` accept the version a manifest names; the
  goldens test selects the version from each case's `manifest.json`. 0.1.0 keeps its three seeds
  and its per-record counts, so every 0.1.0 golden regenerates byte for byte under 0.1.0.
- **Counts per instance (D8; ruling 3; the owner note).** `n_flaws_total`, `n_flaws_<NNNN>` and
  `flaws_confirmed` count instances in the commanded and the achieved block; an instance is
  confirmed only when every one of its records passes its check. The property test equates the
  counts with the instance count, and a pair test asserts one instance, two records, PASS, and
  FAIL with zero confirmed when the formal record fails check K; under 0.1.0 the same pair
  commands two.
- **Seeds (D6).** Each from its FLAWS.md 0.1.0 example, outside the S1 corpus:

  | Seed | Class and dials | Severity | Statement | Witness |
  |---|---|---|---|---|
  | `continuity_is_not_uniform` | FLAW-0002, `swap: forall-exists` | gap | true | `syndeom_seed_continuity_is_not_uniform` (the statement) |
  | `x_over_abs_x` | FLAW-0014, `omitted_case: "x = 0"`, `holds: false` | fatal | false | `syndeom_seed_x_over_abs_x_false` (the negation) |
  | `rolle_abs` | FLAW-0005, Rolle's theorem, differentiability, `mode: violated` | fatal | false | `syndeom_seed_rolle_abs_false` (the negation) |
  | `horses` | FLAW-0006, `mode: step-above-base` (relabeled from `shift-index` by Al's rulings on PR 27), `failing_case: "n + 1 = 2"` | fatal | false | `syndeom_seed_horses_false` (the negation) |

- **Seeds whose statement is false.** They carry `truth.value: false` with the negation witness
  and a `counterexample` in words; they have no control (a flawless config is refused, exit 2);
  their style's steps are the convincing argument and their material is `intrinsic`: the record's
  `dials` hold the site, the text as written, `replaced: null` and the counterexample.
  `check_seed` enforces the shape (a true statement has neither).
- **Renderer.** A declaration without binders (`rolle_abs`) is written with one space before the
  colon; every 0.1.0 seed has binders, so no 0.1.0 byte changed.
- **Lean.** `lean/Syndeom/Seeds/{ContinuityIsNotUniform,XOverAbsX,RolleAbs,Horses}.lean`;
  `lake build` built 8,933 jobs, up from 8,929 (`evidence/lake-build-witnesses.log`); the audit
  prints eight lines, every one within the standard three.

## Evidence that the Lean material does what the records say

1. **Probes before records** (`evidence/lean-probe/`, compiled with `lake env lean` from `lean/`,
   one process at a time, `run.sh`). The first run (2026-10-28T18:41:45Z–18:42:53Z):

   | Probe | Exit | What |
   |---|---|---|
   | `xoa_convincing` | 1 | `4:2: Alternative `eq` has not been provided` — the `cases` line, step 1 |
   | `xoa_false` | 0 | `[propext, Classical.choice, Quot.sound]` |
   | `cinu_faithful` | 0 | `[propext, Classical.choice, Quot.sound]` |
   | `cinu_flawed` | 1 | `6:2: Type mismatch` — the swapped `obtain`, step 1; nothing else |
   | `rolle_convincing` | 1 | `7:146: unsolved goals ⊢ ¬x = 0` — the Rolle application, step 3; then `8:37` in the same step |
   | `rolle_false` | 0 | `[propext, Classical.choice, Quot.sound]` |
   | `horses_convincing` | 1 | first run: a membership slip at 18:126 (`a ∈ s.erase b` needs `ha`, not `hb`); then the overlap line |
   | `horses_false` | 1 | first run: `norm_num` without its import |

   The two horses probes were corrected and rerun (18:51:36Z–18:51:49Z): `horses_convincing`
   now fails only at `20:84`, the `obtain` that claims the overlap non-empty (`omega` refuses the
   goal without `2 ≤ m`), and `horses_false` builds with the standard three axioms. Two further
   files are kept as exploration: `horses_convincing2.lean` (a two-line form of the same step,
   which fails the same way) and `omega_test.lean`, which shows that `omega`'s "No usable
   constraints found" is its message for a satisfiable system here (`0 < m` admits `m = 1`),
   while `1 < m` closes the goal. The one-line form is the seed's.
2. **The goldens.** 14 cases produced by `generate --dry-run` at `af28cc96…` with seed 7
   (`evidence/goldens-run.py`, `evidence/goldens-run-summary.txt`; 2026-10-28T18:58:21Z–19:00:08Z;
   every run exit 0 with fidelity PASS; the provenance check passed against the commit before
   each run). `evidence/goldens-copy.py` placed them under `engine/generate/tests/goldens/` with
   a `case.json` each and compared the formal outputs with the probes
   (`evidence/goldens-copy-report.json`):
   - the eight flawed formal and pair goldens are byte-identical to their probes;
   - the control golden of `continuity_is_not_uniform` is its faithful probe minus the probe's
     trailing `#print axioms` line (the comparison is stated, not byte-exact);
   - the first error lines are 6 (`continuity_is_not_uniform`), 4 (`x_over_abs_x`), 7
     (`rolle_abs`) and 20 (`horses`), each inside the site's fragment (check K); the control's
     audit printed `[propext, Classical.choice, Quot.sound]`.

   The receipts, with build and audit stdout and stderr, are under the external root
   `C:\Users\LEET\Codex\Temp\syndeom-s2-template-020-20261028T1830Z\goldens\`; the sorted list of
   the 14 `receipt.json` SHA-256s hashes to `184058adb90da165df23dbf7ce3acc1b3484f0e7506ea270db0efbedb1543919`.
3. **The golden test** regenerates all 26 cases without Lean, each under the version its manifest
   names, and compares the bytes. Before the 0.2.0 goldens were added, the same test at
   `af28cc96…` passed with the 12 of 0.1.0 and the 0.2.0 count pinned at zero.
4. **The artifact ids** (`s2_template-v0.2.0-<config_hash[0:12]>-s7`): `continuity_is_not_uniform`
   control `e12dbfbc34ae` / `b3c5dd026066`, FLAW-0002 `c6a316e15cbf` / `5b355d64d88f` /
   `b831cf02b9ea`; `x_over_abs_x` `06d157803870` / `a5c34a25cc46` / `ec4a0514442b`; `rolle_abs`
   `6018ec4011e7` / `2aa8e0ec7f12` / `92ed2e47e131`; `horses` `679bf6831274` / `4d89c8372b31` /
   `8666b30995c2` (informal / formal / pair).
5. **Tests and lint.** The crate's 24 tests pass (and still pass after the regeneration) (goldens 1, properties 13, provenance 2, rng 5,
   writers 3); `cargo clippy --all-targets --all-features -- -D warnings` and `cargo fmt --check`
   are clean. The gate on the committed tree is recorded in the pull request and under the
   external root.

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-10-28T19:02:32Z, outside the gate. It loaded 1,381
advisories (advisory-db commit `40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`), scanned 35 crate
dependencies, and reported no vulnerability (exit 0; `evidence/cargo-audit.txt`).
`engine/Cargo.lock` is unchanged by this revision (the build dependency is the workspace's own
`syndeom-manifest`).

## Security review of the new code (`rust-security-audit`, D10)

**Scope:** `engine/generate/build.rs`, `src/provenance.rs`, `src/template.rs` (the version,
counting and intrinsic paths), `src/main.rs` (version selection) at `af28cc96…`, re-checked at
`d7b5d818…`. The threat model is packet 1's: an offline research CLI whose assets are the
integrity of generated ground truth and the operator's machine, and whose trust boundary is an
artifact directory handed to `verify`.

**Pre-scan:** `cargo audit` is clean; `#![forbid(unsafe_code)]` stands in both crate roots; no
external dependency was added; clippy at `-D warnings` is clean.

**Panic and index sweep:** `style.steps[site - 1]` (intrinsic path) and
`faithful[material.site - 1]` are guarded by `check_seed`, which `prepare` runs before `inject`;
`material.steps[site - 1]` comes from `position()`; `r.class[5..]` slices a class that
`check_spec` has matched against FLAWS.md's names; `COMPILED_INPUTS` lookups are by iterator.
`build.rs` panics on an unreadable input file, which fails the build rather than omitting a file.

**Subprocesses:** `git` and `rustc` are spawned without a shell, with a validated hex commit and
paths as arguments, as before.

```
FINDING-005: engine_commit is verified against the working tree, not the running binary
Status:      Resolved (af28cc96) — the compiled input hashes, the commit's blobs and the checkout
             must agree for every input; a stale binary is refused (exit 2).

FINDING-009: the runtime toolchain check resolves `rustc` from PATH
Severity:    Informational
Category:    Trust
Location:    engine/generate/src/provenance.rs (verify_engine_commit)
Status:      Accepted (as syndeom-pair)
Description: `rustc --version` is run from PATH and compared with the compiled line. A planted
             `rustc` on PATH could answer anything; the same PATH already built the binary, so
             this is the operator's own toolchain trust, not a new boundary.

FINDING-010: build.rs panics on an unreadable or unlisted input
Severity:    Informational
Category:    Robustness
Location:    engine/generate/build.rs
Status:      Accepted
Description: a build that cannot read an input file fails loudly instead of embedding a partial
             set. This is the intended failure.
```

FINDING-006 (accepted risk), FINDING-007 (backlog) and FINDING-008 (the FLAW-0013 session) stand as
packet 1 records them.

## Implementation readings, flagged for Al

None of these changes ADR-0011's text beyond its *Amendments*. Each is how this implementation
reads it. They continue packet 1's numbering.

13. **Seeds whose statement is false** (also in ADR-0011 *Amendments*). Three of the four FLAWS.md
    examples state false theorems. Such a seed has no control, its material is intrinsic
    (nothing replaced; the counterexample in the dials), and it is recorded under the class
    FLAWS.md files its example under; a FLAW-0012 record is written only when FLAW-0012 is
    commanded, in a later version.
14. **FLAW-0006's mode for the horses** (also in the amendment): `shift-index`, with
    `failing_case: "n + 1 = 2"`. **Not accepted (Al's rulings on PR 27):** D5's mode enum gains
    `step-above-base` (the inductive step's argument is valid only for n above the base, so the
    base → base+1 step is unjustified; `failing_case` names that first unjustified step), and the
    horses record is relabeled to it, with `failing_case` kept; see *Al's rulings on PR 27* below.
15. **The no-binders header.** `theorem <name> : <claim> := by` for a closed statement; a D2
    rendering detail with no 0.1.0 byte affected.
16. **Both versions in one crate.** `generate` emits 0.2.0 only; `regenerate` and `verify`
    accept 0.1.0 and 0.2.0 by the manifest's `generator.version`; the goldens select likewise.
17. **The FLAW-0002 theorem and its severity.** The seed's statement is "a continuous `f` with
    `f 0 = 1` exceeds `1/2` near `0`", whose faithful proof uses continuity at `0`; the flawed
    step asserts the uniform statement and applies it at `y = 0`. The record is `gap`: the
    uniform claim is unjustified but the conclusion holds. The fatal variant FLAWS.md mentions
    (one `δ` for all `x` essential, e.g. `x²`) is not realized.
18. **`rolle_abs` in `HasDerivAt` form.** Mathlib's `exists_deriv_eq_zero` needs no
    differentiability and `deriv |·| 0 = 0` by convention, so a `deriv`-form claim would be true;
    the seed states `∃ c ∈ Ioo (-1) 1, HasDerivAt (fun x => |x|) 0 c`, which is false.
19. **The horses formalization.** A type `H` with `colour : H → ℕ`, finite sets as `Finset H`,
    the base case `n = 1` inside the successor step; the negation witness instantiates `Fin 2`
    with `Fin.val` as the colour.
20. **`omega`'s message.** "No usable constraints found" at the horses' overlap line is the
    message for a satisfiable system (`m = 1`), as `omega_test.lean` shows; the probe records
    the message as Lean printed it.

## Al's rulings on PR 27, and their application

Al ruled on 2026-10-28 (verbatim in ADR-0011 *Amendments*, STATE *Standing rulings* and HANDOFF
Session 41's addendum; copy `owner-rulings-pr27.txt` under the external root, 1,443 bytes,
SHA-256 `c465f9e5cecb42677e958cb950f3249492c6d75251901ffa6fd343d7aceb6b8a`): readings 13, 15,
16, 17, 18, 19 and 20 are accepted as read; reading 14 is not accepted as `shift-index`.

- **D5 amended.** FLAW-0006's `mode` gains a fourth value, `step-above-base`: the inductive
  step's argument is valid only for n above the base, so the base → base+1 step is unjustified;
  `failing_case` names that first unjustified step. The mode is descriptive for intrinsic
  material and changes no proof byte.
- **The horses record relabeled.** `seeds/horses.json` carries `mode: "step-above-base"`,
  `failing_case: "n + 1 = 2"`; the refusal case in `tests/properties.rs` and the crate README
  follow. The seed's steps, fragments and witness are untouched.
- **The 0.2.0 goldens regenerated** at the commit that holds the relabeled seed, by two
  separate runs compared byte for byte (`evidence/goldens-run2.py`, `evidence/goldens-copy2.py`,
  `evidence/goldens-regeneration-report.json`):
  - the commit is `c899b0732db8b7a1a104822772a428f534a651cd` (the records of the rulings, the
    relabeled seed, the refusal test and the READMEs);
  - run A 2026-10-28T19:28:06Z–19:30:19Z and run B 19:30:19Z–19:32:14Z, each 14 cases, each case
    exit 0 with fidelity PASS, the provenance check passed against the commit before each run
    (`evidence/goldens-run2a-summary.txt`, `evidence/goldens-run2b-summary.txt`);
  - every output file (`proof.md`, `Proof.lean`, `manifest.json`) byte-identical between the
    runs, every observation equal, one engine commit; the report's SHA-256 is `cb8aba701e26466461d8e05b3c3a05ad99b59f78b30eaecbae65a0c99b1ad2ad`
    (9,130 bytes); the receipts are under the external root's `goldens-2a\` and
    `goldens-2b\`, whose sorted `receipt.json` SHA-256 lists hash to `a79a05b9…f0ee` and
    `872b85fe…0e92`;
  - the horses proof bytes are unchanged from the first production (`Proof.lean`
    `9e198c7c…`, `proof.md` `d516f093…`); its artifact ids are now `22c3147d9d48` (informal),
    `05964cd1eb74` (formal) and `7b76255e38a5` (pair); the other eleven cases differ from the
    first production only in the engine commit they name.
- **The goldens test** expects 14 cases of 0.2.0 again, each regenerated under 0.2.0, and passes.
