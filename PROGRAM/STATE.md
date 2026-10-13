# STATE

- **Updated:** 2026-10-13 UTC (Session 5 — CLAIM-0006 examination, close)
- **Phase:** S0 — FOUNDATION AND CITATION VERIFICATION
- **Gate:** S0 open on one criterion only. Done: S0.1 (sources register, incl. the CLAIM-0006
  examination), S0.2 (founding sweep placed), S0.3 (constitution ratified), S0.4 (manifest
  ADR-0001, ratified Session 4), S0.5 (taxonomy 0.1.0 ratified), S0.6 (ADR-0002: toolchains, Lean
  project with Mathlib, engine crate, gate, CI); repo green under the gate. **Open: the cold-start
  test (NEXT)**, which must run in a fresh conversation.
- **Founding sweep:** `PROGRAM/SWEEP-2026-10.md` (41,060 bytes, SHA-256
  `f4bfb2adcdf7c6714101ad6ddc2fdcf528e98e7c993accb8a5dbe04a8115b03e`, ferried by Al in Session 3
  and verified against Al's stated digest before placement). It is a **map, not evidence** (SD11
  banner on line 3): nothing enters any register from it; where it disagrees with the S0.1
  register, the register stands (its own ferry note says so). It was redacted under SD8 before the
  ferry. CLAIM-0006 stays UNKNOWN until the ten sources it ranks are retrieved and read — a later S0
  action (below), not Session 3's.
- **Default branch:** `main` — discovered from the clone of the empty remote and the REST API
  (ADR-0000 D3); confirmed by Al (Q2). Remote: `origin` = `https://github.com/Quigles1337/Syndeom`
  (**public**; created by Al 2026-10-12; Actions enabled; no `LICENSE` file — Q3 deferred).
  **ORIENT note (SD7):** `git symbolic-ref refs/remotes/origin/HEAD` resolves to `main`; if it ever
  fails, fallback `git remote show origin`, repair `git remote set-head origin -a`.
- **Build status:** the gate is `bash scripts/check.sh` (ADR-0002 D6): `lake build` + axiom audit
  of the golden fixture + `cargo fmt --check` + `cargo clippy -D warnings` + `cargo test` + LF check
  + register cross-reference check; it prints `GATE: GREEN` on success. **Last verified GREEN:
  2026-10-13 UTC before the Session-4 S0.6 commit** (Lean: Golden B built, axioms `[propext]`;
  Rust: 23/23 tests, clippy and fmt clean; 36 text files `i/lf`, registers coherent). The advisory
  CI (`.github/workflows/check.yml`) runs the same script and is never a required check.
- **Toolchain (ADR-0002, Session 4):** Lean `leanprover/lean4:v4.34.1` (`lean/lean-toolchain`),
  which is Mathlib's own `lean-toolchain` at the pinned Mathlib commit
  `d13f23b723b8a846827a245b89c10fc7d3f11612` (tag `v4.34.1`; `lean/lakefile.toml` +
  `lean/lake-manifest.json`, eight transitive Lake deps locked) — the Q8 rule, chosen as the newest
  Mathlib release tag with a stable Lean (a load-bearing divergence from KOBER's v4.33.1/no-Mathlib).
  Rust `1.91.0` with clippy + rustfmt (`rust-toolchain.toml` at the root), workspace
  `engine/Cargo.toml`, crate `syndeom-manifest` (`engine/Cargo.lock`, 24 crates). JCS serializer
  `serde_json_canonicalizer 0.3.2`. CI advisory on `actions/*@v5`. Licences of everything pinned:
  S-0008. Machine inventory as of 2026-10-12 in `PROGRAM/SIBLING-MAP-2026-10.md` §8. For the
  record (S5, not a reason for any pin — SD8): the contact corpus `openai/math` happens to pin the
  same Lean and Mathlib commit (S-0001); the ADR-0002 criterion was fixed before that was compared.
- **Constitution:** `PROGRAM/SYNDEOM-PROGRAM.md` v1.0, ratified by Al 2026-10-12 (Session 2);
  rulings of record in the ADR-0000 addendum. ADR-only.
- **Taxonomy:** `PROGRAM/FLAWS.md` 0.1.0 ratified (FLAW-0001..0014).
- **Registers:** CLAIM-0001..0005 and 0007 VERIFIED (0007 existence only); CLAIM-0006 UNKNOWN by
  design until Session 5, now **CONDITIONAL** in a narrowed form (unfalsified as of 2026-10-13;
  bounding sources S-0009..S-0018; the scoped novelty statement is in CLAIMS and lifts §0.6 only
  in that scope). S-0001..S-0018 VERIFIED; INGESTED: S-0004, S-0005 (declarations, CC0 and CC
  BY-NC 4.0) and S-0010..S-0016 (seven CC BY 4.0 papers under `sources/`, ~16 MB); S-0007 RFC 8785,
  S-0009 and S-0018 (arXiv non-exclusive licence) digest-only; S-0008 the toolchain licence record;
  S-0017 the `leanprover/comparator` repository.
- **Manifest schema:** `syndeom-manifest/1`, ADR-0001 (Session 3). Formats `syndeom-informal-md/1`,
  `syndeom-lean/1`, `raw/1`; ids `<generator.id>-v<version>-<config_hash[0:12]>[-s<seed>]`; JCS +
  SHA-256; FRIEDMAN ADR-0004 arithmetic by reference; contact artifacts carry `ground_truth: null`.

## Memory policy

SD15 adopts FRIEDMAN ADR-0003 D1–D3 and the conversation-context corollary; the corollary is
**ratified for SYNDEOM** (Q13, ADR-0000 addendum), cited from FRIEDMAN, not duplicated. The
executor's memory store holds, for SYNDEOM, the repo location and machine hazards only.

## Standing rulings (recorded; their ADRs come later)

- **S2 generator families:** the first family is deterministic — seeded Rust templates and
  grammars. Any LLM-assisted generator is a separate family requiring its own ADR under SD6 and is
  excluded from `battery-freeze-v1` unless that ADR exists (determinism; leakage risk that an LLM
  generator was trained on the contact corpus). ADR at S2.
- **S5 toolchains:** contact corpora carry their own Mathlib pins; S5 ingest records them and may
  need per-corpus toolchains. Note, not solve, until ingest (Q8). First data point recorded in
  S-0001.
- **ADR numbers:** ADR-0001 manifest schema — **written (Session 3) and ratified by Al (Session
  4)** on all five flagged choices, with the derived-canonical-artifact convention added to D4 and
  the UTF-8 decoding sentence to D3 (see its *Amendments*); ADR-0002 toolchain pins, Lean project,
  Rust workspace, JCS crate, storage default, gate and CI — **written (Session 4, S0.6)**; its D8
  records what was measured. ADR-0003 is the next free number.
- **Real items and detectors (ADR-0001 D4, ratified Session 4):** `raw/1` is archival; detectors
  consume only `syndeom-informal-md/1` and `syndeom-lean/1`; every S1/S5 item to be scored gets a
  derived canonical artifact (deterministic producer, `config.derived_from` + `method`, parent's
  ground truth with locations re-expressed and argued in the S1.3 record); the `raw/1` parent
  remains the item of record.

## Deferred S0 actions (scheduled, not NEXT)

- **S0.1 continuation — examine CLAIM-0006 (ruled by Al, Session 3): done in Session 5.** The ten
  ranked sources were retrieved from primaries and read (S-0009..S-0018); CLAIM-0006 was narrowed
  and moved to CONDITIONAL (a claim of absence cannot become VERIFIED by retrieval — Al, Session 5).
  Not retrieved: the sweep's runners-up (IMProofBench, Open Proof Corpus, FormalAlign, Beyond
  Compilation, ShadowBench, Sadasivan et al., First Proof batches, Greiffenhagen 2024); they stay
  outside the register until a later action retrieves them, and the narrowed claim's scope says so.

## Gate S0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| S0.1 CLAIM-0001..0005 (and 0007) resolved to VERIFIED or honestly UNVERIFIED with consequences | **done** | all six VERIFIED 2026-10-12 from primary sources (CLAIMS.md transitions; SOURCES.md S-0001..S-0006 with URLs, access times, licences, digests). Consequences stated: S5 corpus location = `github.com/openai/math` (Apache-2.0); counts carry units and three unreconciled formalization units; the AHM page is undated; IAS page unretrievable (403); signatory counts recorded as of access date |
| S0.2 Founding sweep placed with the SD11 banner | **done** | `PROGRAM/SWEEP-2026-10.md`, SHA-256 `f4bfb2ad…b03e` = Al's stated digest (verified before the move, Session 3); banner present; redacted under SD8 before ferry; reconciliation note defers to the S0.1 register. §0.6 still holds: no novelty claim until CLAIM-0006 is examined |
| S0.3 Constitution ratified by Al; DRAFT marker removed | **done** | `c803c96` `[S0][ADR-0000] ratify the constitution and record Al's rulings`; ADR-0000 addendum |
| S0.4 Manifest schema ADR (`syndeom-manifest/1` extending `friedman-manifest/1`) | **done** | `PROGRAM/DECISIONS/ADR-0001.md` (Session 3): D1–D9 under Q10 (structural) and Q11 (fields); FRIEDMAN ADR-0001/0004 and crate re-read at pinned tip `7c3e5dd`; two goldens over fixed bytes (Golden A `0ac16447…a6a1`, Golden B `c568a981…458e`) with digests, counts, span example, ids; RFC 8785 retrieved as S-0007. Five open design choices flagged for Al in its Consequences (non-blocking) |
| S0.5 Flaw taxonomy v0.1 ratified | **done** | `PROGRAM/FLAWS.md` 0.1.0, 14 entries, ratified (Q12; changelog) at `c803c96` |
| S0.6 Scaffold (`lean/` with Mathlib, `engine/`, `lab/`, `corpora/`, `groundtruth/`, `contact/`, `analysis/`, `sources/`, gate, CI) | **done** | `PROGRAM/DECISIONS/ADR-0002.md` (Session 4): Lean `v4.34.1` = Mathlib's `lean-toolchain` at pinned `d13f23b7…` (tag `v4.34.1`), `lake-manifest.json` locked; `engine/` workspace with `syndeom-manifest` (ADR-0001 D1–D7, 23 tests incl. D8 goldens, ADR-0004 vectors, RFC 8785 vectors); `scripts/check.sh` + `scripts/check_registers.py`; advisory CI on `actions/*@v5`; READMEs incl. `contact/` SD8 rules; licences S-0008; D8 measured: Golden B builds, axioms `[propext]` |
| Licences recorded for anything touched | **done** | S-0001 Apache-2.0 (nothing copied); S-0002, S-0003 licence unstated (nothing copied); S-0004 CC0 and S-0005 CC BY-NC 4.0 (PDFs copied under those terms); no dependency added |
| Repo green under the gate | **done (and standing)** | `bash scripts/check.sh` → `GATE: GREEN` before the S0.6 commit (Build status above); every earlier commit was LF-clean with coherent registers |
| CLAIM-0006 examined (deferred S0 action, Al's ruling Session 3) | **done** | Session 5: the ten ranked sources retrieved from primaries and read (S-0009..S-0018; seven CC BY PDFs under `sources/`, two digest-only, one repository); **no source falsifies the claim as worded**; CLAIM-0006 narrowed and moved UNKNOWN → CONDITIONAL (unfalsified as of 2026-10-13, bounding sources listed); the scoped novelty statement recorded in CLAIMS |
| Cold-start test (§2, SD15) | **open — NEXT** | requires a cold session: a fresh conversation, launched with the §6 Session-N prompt verbatim and alone (reproduced in HANDOFF Session 5); the executor cannot run it from the conversation that built this state |

## Blockers

- None. S0.4 and S0.6 are unblocked by the rulings.

## Standing escalations (awaiting Al)

- **Q3 — Repository LICENSE.** Public repo, no licence file. Al's decision, due before S5.4 and
  before any paper cites the repo. Not a blocker. **Record of choice (Session 5):** Al's Session-5
  prompt asked that the record "show a choice rather than silence" and offered the forms
  "<decision>" or "deferred pending <reason>" — but the slot itself arrived as that template text,
  unfilled, as in Sessions 3 and 4. The executor cannot supply either the decision or the reason
  (SD7: if a value is not pinned, stop and ask; do not choose). **Q3 is therefore recorded as:
  deferred pending Al's explicit decision text**, which is the only choice the executor can make
  on Al's behalf. Nothing in the repo depends on it before S5.4.
- **Q14 — Founding sweep.** **Resolved in Session 3:** Al ferried the sweep by pasting its text
  (no file path was given); the pasted bytes, LF with one trailing newline, hashed to Al's stated
  digest exactly, and the file was placed. Nothing further is awaited.
- **Q15 — Ferry on failure.** Nothing failed outright in S0.1; the one unreachable page (the IAS
  project page, HTTP 403) is corroborative only. No ferry needed now.
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## NEXT-SINGLE-ACTION

**Gate S0 cold-start test (§2, SD15).** This action is not executable from the conversation that
built the state under test; it is executed by a *cold* successor and judged by what that successor
writes. Procedure:
(1) **Launch (Al):** a fresh conversation; the launching message is the §6 "Session N — resume"
prompt **verbatim and alone** (it is reproduced at the end of HANDOFF Session 5), carrying no
program state, no rulings, no hints.
(2) **The cold session** runs §4.1 ORIENT from the repo alone: reads the constitution, STATE, the
last HANDOFF entry, CLAIMS, FLAWS, the ADRs; runs `bash scripts/check.sh` as evidence (expect
`GATE: GREEN`; the first run on a fresh machine takes ~15 minutes for the Mathlib cache). As the
**first entry of its HANDOFF block**, before any other action, it reproduces verbatim every memory
it recalled (SD15); the exam counts only if no recalled memory holds program state.
(3) It then writes, in one pass without guessing, the re-derived program state: phase, gate
criteria with their evidence, the registers' statuses (CLAIM-0006 CONDITIONAL and why), the pins
(Lean, Mathlib commit, Rust channel), the standing escalations (Q3), and the next action — and
compares that against STATE.md. Every disagreement is reconciled as the session's first task
(§4.1). It records the exam result in HANDOFF (PASS / CONDITIONAL PASS / FAIL with the gaps) and
in the Gate S0 table, commits `[S0][INFRA] gate S0 cold-start exam: <result> at <sha>`, pushes.
(4) On PASS, Gate S0 is closed and the cold session sets NEXT-SINGLE-ACTION = S1.1 (begin the
ground-truth corpus: an ADR for the errata-harvest procedure, precedent in S-0015's
ArxivMathGradingBench pipeline, under SD12/SD16 and the ADR-0001 D4 derived-artifact convention).
On FAIL, it records the unreconstructable items as the doc fix and leaves NEXT = this test again.
