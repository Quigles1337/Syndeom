# STATE

- **Updated:** 2026-10-13 UTC (Session 6 — Gate S0 cold-start exam: PASS; Gate S0 closed)
- **Phase:** S1 — GROUND-TRUTH CORPUS (real proofs as unit tests)
- **Gate:** S1 open; no S1 criterion has evidence yet (table below). **Gate S0 CLOSED 2026-10-13
  (Session 6):** the cold-start exam, run in a fresh conversation launched with the §6 prompt
  verbatim and alone, re-derived the program state from the repo in one pass without guessing and
  found the gate GREEN at `2b7c573`; result **PASS**; record in HANDOFF Session 6, whose first
  entry is the SD15 memory disclosure. One audit request for Al arising from that disclosure is in
  the standing escalations; if Al overrules the executor's judgment there, the closure becomes
  provisional (see the escalation). The closed S0 table is retained below as the record.
- **Founding sweep:** `PROGRAM/SWEEP-2026-10.md` (41,060 bytes, SHA-256
  `f4bfb2adcdf7c6714101ad6ddc2fdcf528e98e7c993accb8a5dbe04a8115b03e`, ferried by Al in Session 3
  and verified against Al's stated digest before placement; recomputed in Session 6). It is a
  **map, not evidence** (SD11 banner on line 3): nothing enters any register from it; where it
  disagrees with the S0.1 register, the register stands (its own ferry note says so). It was
  redacted under SD8 before the ferry. Its ten ranked sources were retrieved and read in Session 5
  (CLAIM-0006 examination, done); its runners-up remain unretrieved (carried note below).
- **Default branch:** `main` — discovered from the clone of the empty remote and the REST API
  (ADR-0000 D3); confirmed by Al (Q2). Remote: `origin` = `https://github.com/Quigles1337/Syndeom`
  (**public**; created by Al 2026-10-12; Actions enabled; no `LICENSE` file — Q3 deferred).
  **ORIENT note (SD7):** `git symbolic-ref refs/remotes/origin/HEAD` resolves to `main`; if it ever
  fails, fallback `git remote show origin`, repair `git remote set-head origin -a`.
- **Build status:** the gate is `bash scripts/check.sh` (ADR-0002 D6): `lake build` + axiom audit
  of the golden fixture + `cargo fmt --check` + `cargo clippy -D warnings` + `cargo test` + LF check
  + register cross-reference check; it prints `GATE: GREEN` on success. **Last verified GREEN:
  2026-10-13 03:45:45Z–03:46:17Z (Session 6) at `2b7c573`, and again on the tree the Session-6
  commit lands, before that commit** — Lean `Build completed successfully (8926 jobs)`, axioms
  `[propext]`; Rust fmt and clippy clean, 23/23 tests; 36 text files `i/lf` + 9 binaries; registers
  coherent (Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 18/18). Earlier GREEN runs: Session 4
  (before the S0.6 commit) and Session 5 (at `132788c`). **Standing instruction:** update this
  line at every GREEN run a HANDOFF entry records, so it never lags HANDOFF. **Timing:** about 30 s
  with a warm `lean/.lake` (Mathlib cache present); about 15 min on a fresh machine (toolchain
  install, nine clones, 8,908-file cache download) — a `lake build` that prints nothing for minutes
  under a pipe is the download, not a hang. The advisory CI (`.github/workflows/check.yml`) runs
  the same script and is never a required check; it completed `success` for `132788c` and
  `2b7c573`.
- **Toolchain (ADR-0002, Session 4):** Lean `leanprover/lean4:v4.34.1` (`lean/lean-toolchain`),
  which is Mathlib's own `lean-toolchain` at the pinned Mathlib commit
  `d13f23b723b8a846827a245b89c10fc7d3f11612` (tag `v4.34.1`; `lean/lakefile.toml` +
  `lean/lake-manifest.json`, eight transitive Lake deps locked) — the Q8 rule, chosen as the newest
  Mathlib release tag with a stable Lean (a load-bearing divergence from KOBER's v4.33.1/no-Mathlib).
  Rust `1.91.0` with clippy + rustfmt (`rust-toolchain.toml` at the root), workspace
  `engine/Cargo.toml`, crate `syndeom-manifest`. `engine/Cargo.lock` lists 25 packages: the 24
  dependencies S-0008 enumerates (5 direct, 19 transitive) plus the workspace crate itself
  (clarified Session 6; the ADR and S-0008 say "24 crates" and mean the dependencies). JCS
  serializer `serde_json_canonicalizer 0.3.2`. CI advisory on `actions/*@v5`. Licences of
  everything pinned: S-0008. Machine inventory as of 2026-10-12 in `PROGRAM/SIBLING-MAP-2026-10.md`
  §8. For the record (S5, not a reason for any pin — SD8): the contact corpus `openai/math` happens
  to pin the same Lean and Mathlib commit (S-0001); the ADR-0002 criterion was fixed before that
  was compared.
- **Constitution:** `PROGRAM/SYNDEOM-PROGRAM.md` v1.0, ratified by Al 2026-10-12 (Session 2);
  rulings of record in the ADR-0000 addendum. ADR-only.
- **Taxonomy:** `PROGRAM/FLAWS.md` 0.1.0 ratified (FLAW-0001..0014).
- **Registers (unchanged in Session 6):** CLAIM-0001..0005 and 0007 VERIFIED (0007 existence
  only); CLAIM-0006 **CONDITIONAL** in a narrowed form (unfalsified as of 2026-10-13; bounding
  sources S-0009..S-0018; a claim of absence cannot become VERIFIED by retrieval; the scoped
  novelty statement is in CLAIMS and lifts §0.6 only in that scope). S-0001..S-0018 VERIFIED;
  INGESTED: S-0004, S-0005 (declarations, CC0 and CC BY-NC 4.0) and S-0010..S-0016 (seven CC BY 4.0
  papers under `sources/`, ~16 MB; all nine stored PDFs re-hashed to their register digests in
  Session 6); S-0007 RFC 8785, S-0009 and S-0018 (arXiv non-exclusive licence) digest-only; S-0008
  the toolchain licence record; S-0017 the `leanprover/comparator` repository.
- **Manifest schema:** `syndeom-manifest/1`, ADR-0001 (Session 3; ratified Session 4). Formats
  `syndeom-informal-md/1`, `syndeom-lean/1`, `raw/1` (archival); ids
  `<generator.id>-v<version>-<config_hash[0:12]>[-s<seed>]`; JCS + SHA-256; FRIEDMAN ADR-0004
  arithmetic by reference; contact artifacts carry `ground_truth: null`; real items get a derived
  canonical artifact per D4 (iii).

## Memory policy

SD15 adopts FRIEDMAN ADR-0003 D1–D3 and the conversation-context corollary; the corollary is
**ratified for SYNDEOM** (Q13, ADR-0000 addendum), cited from FRIEDMAN, not duplicated. The
executor's memory store holds, for SYNDEOM, the repo location and machine hazards only. Exam
sessions write no memory; the Session-6 exam reproduced its recalled memory verbatim as the first
entry of its HANDOFF block and recorded the memory file's SHA-256 there.

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
  default, gate and CI — written (Session 4, S0.6), D8 records what was measured. **ADR-0003 is the
  next free number and is reserved for S1.1 (NEXT).**
- **Real items and detectors (ADR-0001 D4, ratified Session 4):** `raw/1` is archival; detectors
  consume only `syndeom-informal-md/1` and `syndeom-lean/1`; every S1/S5 item to be scored gets a
  derived canonical artifact (deterministic producer, `config.derived_from` + `method`, parent's
  ground truth with locations re-expressed and argued in the S1.3 record); the `raw/1` parent
  remains the item of record.

## Carried from S0 (notes, not actions)

- **Sweep runners-up unretrieved:** IMProofBench, Open Proof Corpus, FormalAlign, Beyond
  Compilation, ShadowBench, Sadasivan et al., First Proof batches, Greiffenhagen 2024. They stay
  outside the register until a later action retrieves them; the narrowed CLAIM-0006 scope says so.
  Retrieve one only when an S1 item or a later claim needs it; Gate S0's closure did not close
  this.

## Gate S1 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| S1.1 Errata and retractions: per item the original, the notice, the flaw's location, its FLAWS.md class argued in writing; derivation `documented-by-source`; licence per SD16 | **open — NEXT** | the harvest procedure (ADR-0003) is the next action; no item yet |
| S1.2 Informal–formal pairs with the mathlib commit pinned (positive statement-fidelity pairs; drifted pairs are synthetic and live in `corpora/`, SD12) | open | — |
| S1.3 Manifests for real items (`origin: ground-truth`; `raw/1` parent plus derived canonical artifact per ADR-0001 D4; nothing synthetic labeled real) | open | — |
| S1.4 Coverage and power: per flaw class, how many real items at what proof lengths; zero-coverage classes named as a finding | open | — |
| Licences recorded for anything touched (SD16) | open | — |
| Repo green under the gate | **standing** | Build status above |
| Cold-start test (§2, SD15) | open | a fresh conversation launched with the §6 Session-N prompt verbatim and alone, as for S0 (HANDOFF Session 5 reproduces the prompt) |

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

- None.

## Standing escalations (awaiting Al)

- **Q3 — Repository LICENSE.** Public repo, no licence file. Al's decision, due before S5.4 and
  before any paper cites the repo. Not a blocker. Recorded since Session 5 as **deferred pending
  Al's explicit decision text** (the slot in Sessions 3–5 arrived as unfilled template text; the
  executor cannot supply the decision or the reason — SD7). Al: please paste an SPDX identifier or
  the words "deferred pending <reason>". Nothing in S1 depends on it.
- **SD15 audit request (Session 6).** The cold-start exam's memory disclosure (HANDOFF Session 6,
  first entry) flags two memory-file lines as borderline — a paraphrase of three constitution rules
  (§0.3, §0.4, SD17) and the hazard "the gate runs `fmt --check`" — and withholds 85 unrelated
  private-program index lines by count because this repo is public. The executor judged neither
  line program state and recorded PASS. Al: confirm or overrule. If overruled, the exam is
  CONDITIONAL PASS, the memory is trimmed in a non-exam session, the exam is re-run cold before
  Gate S1, and Gate S0's closure is provisional until then. If the count-only convention should be
  the rule, it is an SD15 amendment by ADR.
- **Q14 / Q15 — resolved in S0.** The sweep was ferried and placed (Session 3); no retrieval
  failed outright in S0.1 (the IAS page, HTTP 403, is corroborative only). Nothing awaited.
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## NEXT-SINGLE-ACTION

**S1.1 — write `PROGRAM/DECISIONS/ADR-0003.md`, the errata-and-retraction harvest procedure for
`groundtruth/`** (procedure only; no item is ingested in that session). The ADR decides:
(a) **where candidate items come from** — published proofs whose errata or retraction notices are
themselves public documents, the notice being the status document SD12 requires — and how a
candidate is admitted or rejected (a proof with a documented error, located in the text, whose
class can be argued from FLAWS.md 0.1.0; nothing AI-generated — that is S5);
(b) **the per-item record** — original, notice, the flaw's location, its FLAWS.md class with the
mapping argued in writing, derivation `documented-by-source`, source and licence per SD16;
(c) **licence handling** — a stored copy under `groundtruth/` only when the licence permits
redistribution (this repo is public), otherwise digest-only with `storage: external` (ADR-0001 D2);
ambiguity is escalated to Al, never ingested;
(d) **the artifact form** — the `raw/1` parent (item of record) plus the derived
`syndeom-informal-md/1` canonical artifact per ADR-0001 D4 (iii), with the location re-expression
argued per item in the S1.3 record;
(e) **the shape of the S1.4 coverage table** — flaw class × proof length, zero-coverage classes
named as a finding, and the power statement Gate S1 requires.
Read first, from the stored PDF under `sources/`: S-0015's ArxivMathGradingBench pipeline, the
precedent for harvesting human-authored papers with referee-identified errors (what it harvested,
how it located and labelled errors, what it licensed). Commit `[S1][ADR-0003] decide the
errata-harvest procedure`; update this file's Gate S1 table and set NEXT to the first harvested
items. Run `bash scripts/check.sh` before the commit (expect `GATE: GREEN`).
