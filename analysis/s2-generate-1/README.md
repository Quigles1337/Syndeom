# S2 implementation packet 1 — the generator crate (ADR-0011, Session 40, 2026-10-28)

This packet records the first implementation session of ADR-0011 (D11): the crate
`engine/generate/` → `syndeom-generate` 0.1.0 with the producer `s2_template` 0.1.0, the Lean
seed witnesses with the gate's axiom loop, the v0.1 goldens, `cargo audit`, and the security
review that D10 asks for. Nothing here is a result: no CLAIM, no fidelity sweep, no artifact under
`corpora/`.

Branch `claude/s2-generate-crate`, from `main` `f9f973012cbe1a7d7f1d8cd4520db998dca0e550`:

| Commit | What |
|---|---|
| `9b4b16f688bfc280a0e7ce0cda2da80558435f5b` | the crate: plan, renderers, RNG, harness, `s2_template`, CLI, seeds, RNG/writer/property tests |
| `21f83594a22d514bb9cdb1ec0261f659e7d1f4b9` | seed witnesses in the Lean library; `Audit.lean` and the gate's axiom loop (the D10 gate change, for Al's own review under ruling 3) |
| `f8cc5df249c3026d2685c17483ccba6301c5cf3a` | the 12 goldens and their test |
| `cd5633b3f1a7388565526e5d93f0afffabcc17c4` | the security review's four fixes; no emitted byte changed |
| the commit carrying this packet | this packet, STATE and HANDOFF |

## What was built

- **Shape.** One crate, edition 2024, `#![forbid(unsafe_code)]`, PolyForm-Noncommercial-1.0.0.
  Its only dependencies are `serde`, `serde_json` and `syndeom-manifest`. `engine/Cargo.lock`
  gains the workspace member and nothing external (D1).
- **Plan and renderers (D2).** Both renderers are pure. Golden A (279 bytes) and Golden B
  (168 bytes) are reproduced byte for byte from the `nat_le_sq` seed (`tests/writers.rs`).
- **`syndeom-rng/1` (D4).** It is implemented as the ADR states it. The first four outputs for
  seeds 0, 1 and 7 under the label `test`, plus one bounded draw, are pinned in `tests/rng.rs`.
  - The pinned values equal an independent Python implementation written from D4's text
    (`evidence/rng_d4.py`, `evidence/rng_d4.out.json`).
  - **Agreement with the published SplitMix64 and xoshiro256** reference code was not
    attempted** this session, so the attribution is unverified. D4 lets the definition stand on
    its own.
- **`s2_template` 0.1.0 (D3, D5–D9):**
  - Seeds `nat_le_sq`, `sq_plus_one_ge_two_x` and `n_sq_mod_four`. Each states its `source`:
    FLAWS.md 0.1.0's examples and ADR-0001 D8, never `groundtruth/` (ruling 1).
  - Flaw material for FLAW-0001 (`propagated`, fatal; `local`, repairable) and FLAW-0014 (gap,
    `holds: true`).
  - The full-config refusals of D3, and the injector's `auto` site drawn from
    `injector/<class>/1`.
  - Records whose `dials` hold the resolved site and every replaced or removed original.
  - Ground truth by construction, and commanded and achieved statistics with the fidelity
    verdict.
  - The CLI `generate`, `regenerate` and `verify`, with engine-commit verification, exclusive
    writes, same-id conflict (exit 3) and receipts.
- **Lean (D7, D10).** The witnesses are in `lean/Syndeom/Seeds/{NatLeSq,SqPlusOneGeTwoX,NSqModFour}.lean`.
  - `lake build` built 8,929 jobs, up from 8,926 with the three witnesses
    (`evidence/lake-build-witnesses.log`).
  - The audit prints four lines: `[propext]` for the golden fixture and `nat_le_sq`, and
    `[propext, Classical.choice, Quot.sound]` for the other two.
  - `scripts/check.sh` checks every axiom line against the standard three and the line count
    against `Audit.lean`'s `#print axioms` lines. The loop was exercised on six crafted outputs
    (`evidence/axiom-loop-test.{sh,out}`): standard sets and reordered subsets pass; `sorryAx`,
    `Lean.ofReduceBool`, a missing line and empty output fail.

## Evidence that the Lean material does what the records say

1. **Probes before code.** The formal proofs were written by hand and compiled with
   `lake env lean` before any Rust existed, one process at a time (`evidence/lean-probe/`):
   - the faithful proofs of `sq_plus_one_ge_two_x` and `n_sq_mod_four` build;
   - FLAW-0001 `propagated` fails at 6:47 (the perturbed `ring` step), then at 9:2;
   - FLAW-0001 `local` fails at 6:47 only;
   - the FLAW-0014 gap fails at 6:2 with "Alternative `inr` has not been provided" (the `cases`
     line).

   The first run warned that `Mathlib.Data.Real.Basic` is deprecated at the pinned Mathlib
   (since 2026-08-27, replace with `Mathlib.Basic.Real.Basic`). The seed uses the replacement,
   and the second run had no warning.
2. **The goldens.** They were produced by `generate --dry-run` at `21f83594…`
   (`evidence/goldens-run.mjs`, `evidence/goldens-run-summary.txt`):
   - 12 cases, each exit 0 with fidelity PASS;
   - the flawed formal cases have first error line 6, inside the site's fragment (check K);
   - each formal golden is byte-identical to its hand-written probe
     (`evidence/goldens-copy-report.json`).

   The receipts, with build and audit stdout and stderr, are under the external root
   `C:\Users\LEET\Codex\Temp\syndeom-generate-20261028T1706Z\goldens\`.
3. **The golden test** regenerates all 12 cases without Lean and compares the bytes. A one-byte
   canary edit made it fail, and it passed again once the file was restored.
4. **CLI checks:**
   - `regenerate` reproduced a golden's `proof.md` from its manifest alone;
   - `verify` re-ran Lean on the propagated formal golden and observed `fail`, agreeing with the
     manifest;
   - a reused receipt directory was refused (exit 1).

## `cargo audit` (D10)

The run was `cargo-audit 0.22.2` at 2026-10-28T17:40:28Z, outside the gate. It loaded 1,381
advisories (advisory-db commit `40f5be6fb12ed93fc591bf4ab23b7d793b3f2239`), scanned 35 crate
dependencies, and reported no vulnerability (exit 0; `evidence/cargo-audit.txt`).

## Security review (`rust-security-audit`, D10)

**Scope:** `engine/generate/src/{lib,rng,plan,template,harness,main}.rs` at `f8cc5df2…`,
re-checked at `cd5633b3…`.

**Threat model:** an offline research CLI with no network, keys, async code or `unsafe`.
- Its inputs are the operator's config and arguments, the compiled-in seed records, manifests
  and proof files read from disk, and Lean's output.
- The assets are the integrity of generated ground truth (SD1, SD6) and the operator's machine.
- The trust boundary is an artifact directory handed to `verify`: Lean elaboration can execute
  code.

**Pre-scan:** `cargo audit` is clean. `#![forbid(unsafe_code)]` is in both crate roots, and the
crate adds no external dependency. Clippy at `-D warnings` is clean. Clippy pedantic reported
documentation and `must_use` style only, which is not adopted. `cargo geiger` and `cargo deny`
are installed, but the repository has no `deny.toml`. Miri does not apply.

**Panic and index sweep:** the remaining `expect`s guard compiled-in invariants: hex from
`sha256_hex`, and the seed records parse, which a test asserts. Each index site is guarded by a
preceding check:
- the config keys are checked before `obj[k]`;
- `below(n) < n` holds over a non-empty list;
- the site index comes from `position()`;
- step indices are bounded by the rendered plan.

**Subprocesses:** `git` and `lake` are spawned without a shell. Their arguments are a validated
hex commit and paths.

```
FINDING-001: verify compiled any Proof.lean consistent with its own directory's manifest
Severity:    Medium
Category:    Logic
Location:    engine/generate/src/main.rs (verify)
Status:      Resolved (cd5633b3)
Description: `verify` checked only that Proof.lean matched the content_hash in the same directory's
             manifest, then compiled it. Lean elaboration can run code (`#eval` with IO), so a planted
             artifact directory meant code execution as the operator.
Recommendation/Resolution: compile only the bytes this producer emits for the manifest's config and
             seed, from compiled-in seeds. A planted file is refused (exit 2) before any Lean process
             or receipt directory exists (checked on a tampered copy of a golden).

FINDING-002: a receipt directory inside the repository was created before the refusal
Severity:    Low
Category:    Logic
Location:    engine/generate/src/main.rs (new_receipt_dir)
Status:      Resolved (cd5633b3)
Description: the directory was created, then canonicalized and refused, leaving an empty directory in
             the checkout.
Resolution:  the parent is canonicalized and the location checked first; `create_dir` (not `_all`)
             makes the new directory. Checked: an in-repo path is refused (exit 1) and not created.

FINDING-003: `Prepared::artifact_id` held an `expect` on config canonicalization
Severity:    Low
Category:    Panic
Location:    engine/generate/src/template.rs (Prepared::artifact_id)
Status:      Resolved (cd5633b3)
Description: unreachable after a successful `prepare` (every accepted config holds small numbers), but
             a panic on the CLI path all the same.
Resolution:  `prepare` computes `config_hash` once and refuses on failure; `artifact_id` uses it.

FINDING-004: the harness took the first error reported against any `.lean` file
Severity:    Low
Category:    Logic
Location:    engine/generate/src/harness.rs (first_error_line)
Status:      Resolved (cd5633b3)
Description: check K reads the first error's line. An error attributed to another file would have been
             read as this file's line.
Resolution:  only errors whose path equals the compiled file's path as passed to Lean count (Lean
             prints it back unchanged, `//?/C:/…` on this machine); tested with a foreign-file error.

FINDING-005: engine_commit is verified against the working tree, not the running binary
Severity:    Medium
Category:    Logic (provenance, SD6)
Location:    engine/generate/src/main.rs (verify_engine_commit)
Status:      Open — for Al
Description: as in syndeom-derive (D9: "verified as syndeom-derive does"), the check compares the
             commit's blobs with the checkout. A binary built from older sources and run directly would
             pass and stamp an engine_commit whose bytes it did not use. `cargo run` and `cargo test`
             rebuild first; this session built immediately before every run.
Recommendation: embed the input set's hashes at build time and compare them with the commit, as
             syndeom-pair's build.rs does (COMPILED_INPUTS), in the next producer revision.

FINDING-006: config number forms with one canonical hash
Severity:    Informational
Category:    Serde
Location:    engine/generate/src/template.rs (parse_config)
Status:      Accepted risk
Description: `"padding_ratio": 0` and `0.0` (or `-0.0`) have one JCS form, so one config_hash and one
             artifact_id, but different `config` bytes in the manifest. A second generate under the
             other form exits 3, which protects D9 (iii).

FINDING-007: a failed write can leave a partial corpora/<id>/
Severity:    Informational
Category:    Logic
Location:    engine/generate/src/main.rs (generate)
Status:      Open (backlog)
Description: files are written one by one with exclusive creation. An I/O failure midway leaves a
             directory that later runs report as a conflict until it is removed by hand.

FINDING-008: padding_ratio is read as binary64 before rendering
Severity:    Informational
Category:    Logic (D4)
Location:    engine/generate/src/template.rs (prepare)
Status:      Open (for the FLAW-0013 session)
Description: v0.1 only refuses a non-zero ratio. When padding is realized, the dial must be an exact
             rational and the ratio compared only after rendering, as D4 requires.
```

## Implementation readings, flagged for Al

None of these changes ADR-0011's text. Each is how this implementation reads it:

1. **Lean-free goldens.** Golden manifests carry `engine_commit` `21f83594…`, the commit the
   producer ran from. `case.json` records the harness observation, so the golden test needs no
   Lean.
2. **Commanded keys.** `n_steps` and `n_lemmas` are commanded only when an informal file exists.
   A formal-only artifact commands the flaw counts, `padding_ratio`, `lean_build` and, when the
   build is predicted to pass, `lean_axioms_standard`.
3. **`n_tokens`** is measured on the informal file when there is one, else on the formal file.
4. **Flaw counts.** Achieved `n_flaws_total` and `n_flaws_<NNNN>` use the estimator
   `injected_instances_confirmed`, as ADR-0001's example manifest does. The commanded counts are
   records, so a pair counts each instance twice.
5. **Informal confirmation.** An informal record is confirmed structurally: the span holds exactly
   the inserted text, which differs from what it replaced. Check K needs a formal file.
6. **Flaw material.** Template material is a whole-argument variant, one instance per artifact
   in v0.1. A second spec is refused as not disjoint (D5's note).
7. **Receipt flag.** `--receipt-dir` is a required flag of `generate` and `verify`, because D9
   requires receipts but names no flag. With `--dry-run`, the outputs go there instead of
   `corpora/`.
8. **`nat_le_sq`'s witness** is its own Seeds module, as D7 says, beside Golden B's identical
   declaration.
9. **Deferred.** The used-constants part of the audit tail (the T check) waits for the first
   FLAW-0007, FLAW-0009 or FLAW-0013 producer.
10. **Reference vectors.** The RNG attribution is unverified (above).
11. **FINDING-005.** Whether to embed build-time input hashes in the next producer revision.
12. **Versioning a new seed.** A new seed makes `s2_template` emit bytes for a config that 0.1.0
    refused. STATE's NEXT reads ADR-0011 D1 as requiring a version bump (0.2.0), with the 0.1.0
    goldens kept regenerable under the version they name.
