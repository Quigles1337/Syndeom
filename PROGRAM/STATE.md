# STATE

- **Updated:** 2026-10-14 UTC (Session 13 — the second Route A harvest: candidates 3–6 examined
  in order, 2 admitted (one with the first undetermined statement truth) and 2 rejected (one
  D2.1, one D8 window); prefix through `0912.0308`; `groundtruth/` holds four `raw/1` items.
  Session 12 recorded Al's D8 precision. Session 11 — the first Route A harvest: finder built and
  run (2,472 candidates), candidates 1–2 examined in order and both admitted and ingested
  (`groundtruth/` now holds two `raw/1` items with records); S-0021..S-0023 registered for the
  position paper's §0 disclosure; memory hazard added. Session 10 recorded Al's rulings of
  2026-10-13 (ADR-0001 `/2`, ADR-0003 resolved, ADR-0005) and built the producer; Session 9
  decided ADR-0003; Session 8 closed R3; Session 7 recorded the earlier rulings)
- **Phase:** S1 — GROUND-TRUTH CORPUS (real proofs as unit tests)
- **Gate:** S1 open; no S1 criterion has evidence yet (table below). **Gate S0 CLOSED 2026-10-13
  (Session 6):** the cold-start exam, run in a fresh conversation launched with the §6 prompt
  verbatim and alone, re-derived the program state from the repo in one pass without guessing and
  found the gate GREEN at `2b7c573`; result **PASS**; record in HANDOFF Session 6, whose first
  entry is the SD15 memory disclosure. **Al confirmed the PASS on 2026-10-13 (Session 10, ADR-0000
  Addendum 3): the closure is not provisional; no re-run.** The closed S0 table is retained below
  as the record.
- **Founding sweep:** `PROGRAM/SWEEP-2026-10.md` (41,060 bytes, SHA-256
  `f4bfb2adcdf7c6714101ad6ddc2fdcf528e98e7c993accb8a5dbe04a8115b03e`, ferried by Al in Session 3
  and verified against Al's stated digest before placement; recomputed in Session 6). It is a
  **map, not evidence** (SD11 banner on line 3): nothing enters any register from it; where it
  disagrees with the S0.1 register, the register stands (its own ferry note says so). It was
  redacted under SD8 before the ferry. Its ten ranked sources were retrieved and read in Session 5
  (CLAIM-0006 examination, done); its runners-up remain unretrieved (carried note below).
- **Advisory note:** `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md` (27,238 bytes, LF, SHA-256
  `f3e40b684a8b437fa4f0ba9d059d23d6ae61ce88ce8bca84ceb49fd7c181fa2c`, ferried by Al under ruling
  R3 of 2026-10-13 and verified against the stated digest before the commit, Session 8). A
  ChatGPT-produced design discussion of 2026-10-13 relayed verbatim under a provenance header,
  with Claude's verification record and its rights status under the R1 boundary (the DARQ
  MNEMOSYNE material in it is proprietary and outside the Apache-2.0 release; the program is named
  with its owner's authorization, Al, 2026-10-13). **SD11 banner: advisory, not evidence** —
  nothing enters any register from it; its two "recorded counterexamples" are not in the Mnemosyne
  repository (its header says so); its §6/§7 finding is Mnemosyne change control, not SYNDEOM's.
  Its only standing effect here is R4 (candidate S2 seed family, standing rulings below).
- **Default branch:** `main` — discovered from the clone of the empty remote and the REST API
  (ADR-0000 D3); confirmed by Al (Q2). Remote: `origin` = `https://github.com/Quigles1337/Syndeom`
  (**public**; created by Al 2026-10-12; Actions enabled; **`LICENSE` = Apache-2.0 since Session 7**
  — Q3 resolved by Al 2026-10-13, ADR-0000 Addendum 2 R1; the release boundary is stated in
  README.md § Licence; text provenance S-0019).
  **ORIENT note (SD7):** `git symbolic-ref refs/remotes/origin/HEAD` resolves to `main`; if it ever
  fails, fallback `git remote show origin`, repair `git remote set-head origin -a`.
- **Build status:** the gate is `bash scripts/check.sh` (ADR-0002 D6): `lake build` + axiom audit
  of the golden fixture + `cargo fmt --check` + `cargo clippy -D warnings` + `cargo test` + LF check
  + register cross-reference check; it prints `GATE: GREEN` on success. **Last verified GREEN:
  2026-10-14 03:01:46Z–03:02:24Z (Session 13, detached run with a log) on the Session-13 harvest
  tree with every edit staged, and again on the final tree before the harvest commit** — Lean `Build completed successfully (8926 jobs)`,
  axioms `[propext]`; Rust fmt and clippy clean, 29/29 tests (24 manifest + 5 ingest); 58 text files `i/lf` + 10 binaries;
  registers coherent (Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 31/31). Earlier GREEN runs:
  Session 4 (before the S0.6 commit), Session 5 (at `132788c`), Session 6 (at `2b7c573` and on its
  own tree), Session 7 (22:54:28Z–23:04:29Z, cold OS cache), Session 8 (23:21:33Z–23:31:44Z and
  23:32:45Z–23:33:19Z), Session 9 (23:35:52Z–23:36:23Z and 23:36:55Z–23:37:25Z) and Session 10's
  rulings commit (2026-10-14 00:07:19Z–00:17:54Z) and producer commit (00:43:55Z–00:52:27Z and
  00:54:48Z–00:55:45Z, detached) Session 11 (02:07:11Z–02:18:18Z and 02:19:15Z–02:21:08Z,
  detached) and Session 12 (02:34:42Z–02:45:22Z and 02:46:06Z–02:46:35Z, detached), each on its
  own tree before its commit. A run the harness
  killed for system memory (00:21:40Z, in the axiom audit) is not a GREEN run; since then the
  gate is run detached with a log (HANDOFF Session 10). **Standing instruction:** update this line at every GREEN run a HANDOFF entry
  records, so it never lags HANDOFF. **Timing:** about 30 s with a warm `lean/.lake` *and* a warm
  OS file cache; **about 10 min after a reboot even with `lean/.lake` warm** (Session 7: the axiom
  audit, `lake env lean Audit.lean`, loaded Mathlib's oleans from a cold disk cache, lean.exe
  above 1.4 GB — not a hang; run the gate in the background with a log); about 15 min on a fresh
  machine (toolchain install, nine clones, 8,908-file cache download) — a `lake build` that prints
  nothing for minutes under a pipe is the download, not a hang. The advisory CI
  (`.github/workflows/check.yml`) runs the same script and is never a required check; it
  completed `success` for `132788c` and `2b7c573`.
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
  **amended twice: ADR-0004 (2026-10-13, Session 7) — §2 S3.2, fooling length as equivalence to
  chance with a design-derived N_min; ADR-0005 (2026-10-13, Session 10) — SD15, count-only
  disclosure of unrelated memory index lines in a cold-start exam.** Rulings of record: the
  ADR-0000 addendum (Session 2, Q1–Q17), Addendum 2 (Al's rulings of 2026-10-13, R1–R4) and
  Addendum 3 (Al's rulings of 2026-10-13 on ADR-0003 (i)–(v) and the SD15 audit). ADR-only; the
  constitution's version string is unchanged because no rule assigns version numbers to
  amendments (noted in ADR-0004).
- **Taxonomy:** `PROGRAM/FLAWS.md` 0.1.0 ratified (FLAW-0001..0014).
- **Registers (unchanged in Session 6):** CLAIM-0001..0005 and 0007 VERIFIED (0007 existence
  only); CLAIM-0006 **CONDITIONAL** in a narrowed form (unfalsified as of 2026-10-13; bounding
  sources S-0009..S-0018; a claim of absence cannot become VERIFIED by retrieval; the scoped
  novelty statement is in CLAIMS and lifts §0.6 only in that scope). S-0001..S-0018 VERIFIED;
  INGESTED: S-0004, S-0005 (declarations, CC0 and CC BY-NC 4.0) and S-0010..S-0016 (seven CC BY 4.0
  papers under `sources/`, ~16 MB; all nine stored PDFs re-hashed to their register digests in
  Session 6); S-0007 RFC 8785, S-0009 and S-0018 (arXiv non-exclusive licence) digest-only; S-0008
  the toolchain licence record; S-0017 the `leanprover/comparator` repository. Later entries:
  S-0019 (the LICENSE text, INGESTED), S-0020 (Mann–Whitney 1947, digest-only), S-0021..S-0023
  (the Mathathon open letter, joint statement and event page — the position paper's §0
  disclosure, no CLAIM, digest-only, Session 11), S-0024..S-0027 (the Route A items'
  originals and notices, Session 11; S-0024 INGESTED under `groundtruth/`, the rest digest-only),
  S-0028..S-0031 (Route A items 3 and 4, Session 13, all digest-only).
- **Manifest schema:** `syndeom-manifest/2` since 2026-10-13 (ADR-0001 amended, Session 10:
  `statement_status.value` admits `null` with `method: "undetermined"`, `null ⇒ proof_status
  invalid`; `/1` remains accepted and names the pre-amendment schema — the D8 goldens), ADR-0001
  (Session 3; ratified Session 4). Formats
  `syndeom-informal-md/1`, `syndeom-lean/1`, `raw/1` (archival); ids
  `<generator.id>-v<version>-<config_hash[0:12]>[-s<seed>]`; JCS + SHA-256; FRIEDMAN ADR-0004
  arithmetic by reference; contact artifacts carry `ground_truth: null`; real items get a derived
  canonical artifact per D4 (iii).

## Memory policy

SD15 adopts FRIEDMAN ADR-0003 D1–D3 and the conversation-context corollary; the corollary is
**ratified for SYNDEOM** (Q13, ADR-0000 addendum), cited from FRIEDMAN, not duplicated. The
executor's memory store holds, for SYNDEOM, the repo location and machine hazards only. Exam
sessions write no memory; the Session-6 exam reproduced its recalled memory verbatim as the first
entry of its HANDOFF block and recorded the memory file's SHA-256 there. **Disclosure convention
(ADR-0005, 2026-10-13):** an exam reproduces verbatim every index line naming SYNDEOM or a
sibling and the SYNDEOM memory file in full with its digest, and withholds unrelated programs'
index lines by count with a recorded name-search check, because this repository is public. The
memory file was trimmed in Session 10 on Al's order (the paragraph paraphrasing §0.3, §0.4 and
SD17 removed; hazards kept); its digest is in HANDOFF Session 10 beside the Session-6 one.

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
  default, gate and CI — written (Session 4, S0.6), D8 records what was measured; **ADR-0003
  errata-and-retraction harvest procedure for `groundtruth/` (S1.1) — written Session 9**, D1–D9,
  five choices (i)–(v) ruled by Al 2026-10-13 (Session 10; Amendments); ADR-0004 constitution
  amendment to §2 S3.2 (Al's ruling R2 of 2026-10-13) — written Session 7; ADR-0005 constitution
  amendment to SD15 (count-only exam disclosure; Al's SD15 audit ruling of 2026-10-13) — written
  Session 10. **ADR-0006 is the next free number.**
- **S3 calibration ADR obligation for undetermined items (Al's ruling (ii), 2026-10-13):**
  items whose `statement_status.value` is `null` (`method: "undetermined"`, ADR-0001 as amended)
  **count in proof-flaw cells and are excluded from every cell that uses the truth dimension,
  the FLAW-0011 and FLAW-0012 composites included.** Stated in the S3 calibration ADR before any
  curve, beside the ADR-0004 obligations above.
- **S3 calibration ADR obligations (ADR-0004, 2026-10-13):** before any curve is computed it must
  (1) compute N_min by the method ADR-0004 fixes — the smallest balanced cell at which the
  equivalence test (95% bootstrap CI of the AUC inside [0.45, 0.55]) has 90% power at true AUC
  0.5, from the null-hypothesis variance of the Mann–Whitney statistic — and confirm the number by
  simulation against the bootstrap procedure actually used; (2) state how effective N is computed
  for items derived from one seed theorem (for example a cluster bootstrap by seed theorem).
  Cells that cannot reach N_min — human-referee cells under SD13, detectors needing a per-item
  Lean build across the full grid — are reported **underpowered** by design.
- **S2 seed family (Al's ruling R4 of 2026-10-13; ADR at S2):** a Mnemosyne-derived seed family —
  the static-reduction theorem DM-FM-THM-001 and the labelled variants the advisory note proposes —
  is a **candidate** hand-authored golden-fixture family for S2.4, beside the FLAWS.md examples. A
  pipeline test, not calibration data (one theorem, six variants). Its licence line is decided at
  S2 under the R1 release boundary (ADR-0000 Addendum 2). Nothing built yet. The program is named
  with its program owner's authorization (Al, 2026-10-13).
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
| S1.1 Errata and retractions: per item the original, the notice, the flaw's location, its FLAWS.md class argued in writing; derivation `documented-by-source`; licence per SD16 | **in progress — prefix through arXiv id `0912.0308` (candidates 1–6 of 2,472; Sessions 11 and 13): 6 examined, 4 admitted, 2 rejected (one D2.1 — a mis-stated quoted theorem with no proof offered; one D8 — correcting version after the window); `groundtruth/HARVEST-LOG.md` is the record of every candidate; NEXT resumes at candidate 7** | `PROGRAM/DECISIONS/ADR-0003.md` (Session 9; Al's rulings on (i)–(v) recorded Session 10): D1 routes (arXiv comments, Crossref update relations, zbMATH index; AI-generated excluded), D2 admission, D3 class mapping (undetermined truth admitted under `syndeom-manifest/2`), D4 record (`manifest.json`, bytes, `RECORD.md`, S-entries, `groundtruth/HARVEST-LOG.md`), D5 licences (NC/ND `external`), D6 `raw/1` parent + derived artifact + the producer, D7 log, D8 window, no skipping, prefix gate, D9 the S1.4 table; routes live-checked 2026-10-13. **Producer:** `engine/ingest/` → `syndeom-ingest` (`s1_ingest` 0.1.0, Session 10): writes a `/2` manifest from a config and bytes, refuses on any ADR-0001 D4 or ADR-0003 shape violation (exit 2) and on a same-id byte conflict (exit 3); goldens over fixed bytes (fixture `915d3ed6…1390`, 72 bytes) and two fixed configs — boolean truth (`s1_ingest-v0.1.0-d2134e7fd2c3`, manifest `1f2cf7b9…1f63`) and the null case (`s1_ingest-v0.1.0-4f5d4e93adaa`, manifest `451bde2d…8340`) — config hashes cross-checked in Python. Evidence slots still empty: the harvest log, the first admitted items |
| S1.2 Informal–formal pairs with the mathlib commit pinned (positive statement-fidelity pairs; drifted pairs are synthetic and live in `corpora/`, SD12) | open | — |
| S1.3 Manifests for real items (`origin: ground-truth`; `raw/1` parent plus derived canonical artifact per ADR-0001 D4; nothing synthetic labeled real) | **`raw/1` parents: 4 manifested (Sessions 11, 13); derived canonical artifacts: none — the derivation producer and its ADR do not exist yet** | `s1_ingest-v0.1.0-63e23a952084/` (arXiv 0706.0357 v70, Lemma 1.1; FLAW-0012 + FLAW-0005; statement false; original committed, CC BY 4.0); `s1_ingest-v0.1.0-744652f4b95f/` (arXiv 0707.3799 v2, Theorem 1 b); FLAW-0011 + FLAW-0005; statement true; `external`); `s1_ingest-v0.1.0-5ba6c0f07557/` (arXiv 0902.0680 v1, the dense-subspace lemma; FLAW-0005 gap; **statement undetermined — `value: null`, the first use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-a8ac73896464/` (arXiv 0908.3213 v4, Theorem 4.4; FLAW-0001 propagated + local; statement false; `external`) — all under `groundtruth/`, each with `manifest.json` (`syndeom-manifest/2`, `engine_commit` of the producer's commit, self-checked by the producer) and `RECORD.md` (the D3 class argument); S-0024..S-0031 |
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

- **Q3 — RESOLVED 2026-10-13 (Session 7).** Al: SPDX `Apache-2.0` for SYNDEOM's original software
  and technical documentation, with the release boundary recorded in ADR-0000 Addendum 2 R1 and
  README.md § Licence. `LICENSE` committed (S-0019), `license` field set on `syndeom-manifest`.
  Nothing awaited. One note for Al, not a blocker: the Apache Software Foundation's own host
  (`www.apache.org`) refused connections from both tools throughout Session 7, so the committed
  text is the SPDX License List's rendering, cross-checked whitespace-for-whitespace against an
  independent copy (S-0019); a later session can diff it against the ASF file when the host answers.
- **Advisory note ferry (Al's ruling R3 of 2026-10-13) — RESOLVED 2026-10-13 (Session 8).** In
  Session 7 the file was absent and a mid-session paste hashed to 25,652 bytes /
  `0ee1eda0…baa80` (lossy chat rendering), so the executor stopped and escalated. Al then placed
  the file at the path; Session 8 verified it byte-exact against the stated digest and committed
  it as ferried, body unedited (Advisory note bullet above; ADR-0000 Addendum 2 R3). Nothing
  awaited. Standing form for any future ferry: a path to bytes on disk, never a paste.
- **ADR-0003 flagged choices (i)–(v) — RESOLVED 2026-10-13 (Session 10; ADR-0000 Addendum 3).**
  (i), (ii), (v) ratified as written; (iii) ratified with the gate clarified (Gate S1 may close on
  a prefix of the Route A stream); (iv) overruled (NC and ND originals are `external`,
  digest-only). ADR-0001 amended for (ii) under `syndeom-manifest/2`; the S3 obligation is in the
  standing rulings. Nothing awaited.
- **SD15 audit request (Session 6) — CONFIRMED 2026-10-13 (Session 10).** The exam's PASS stands;
  Gate S0's closure is not provisional; no re-run. The memory paraphrase line was trimmed (digest
  in HANDOFF Session 10); the count-only convention is now SD15 by ADR-0005. Nothing awaited.
- **Q14 / Q15 — resolved in S0.** The sweep was ferried and placed (Session 3); no retrieval
  failed outright in S0.1 (the IAS page, HTTP 403, is corroborative only). Nothing awaited.
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## NEXT-SINGLE-ACTION

**S1.1 — the second Route A harvest session, resuming from the log's last candidate** (ADR-0003
D7, D8 as amended: a prefix is enough). Procedure, executable cold:
(a) read `groundtruth/HARVEST-LOG.md`: run 1's query, window 2023-01-01..2026-09-30, ascending
arXiv id, **last candidate processed `0912.0308` (candidate 6 of 2,472)**; the next candidate is
`0912.0966` (v6, "… A small correction made to the statement of Theorem 8");
(b) regenerate the candidate list — `python -I -X utf8 lab/route_a_candidates.py 2023-01-01
2026-09-30 <scratchpad>/route-a` (pages are cached under that directory only for the session
that fetched them; a fresh run re-fetches 57 ranges in about 30 minutes) — and confirm its
digest against the log's run record or record the new digest as run 2 with the difference stated
(arXiv adds and revises papers; the examined prefix is unaffected because ids are assigned at
submission and the finder sorts by id);
(c) examine candidates from `0912.0966` onward **in ascending id, skipping none**, each with
`python -I -X utf8 lab/route_a_examine.py <id> <correcting version> <scratchpad>/exam/<id>`
(both versions' abstract pages for licence and submission history, both e-prints, the TeX diff,
the AI-assistance scan) against ADR-0003 D2's seven criteria; log every verdict with its reason;
(d) for each admitted item, in this order: the S-entries for original and notice in SOURCES.md
(licence per D5 as amended: CC0 / CC BY / CC BY-SA / public domain → `committed`, NC, ND,
arXiv non-exclusive or assumed → `external`); the per-item config (the four Session-11/13 configs
are the pattern, including the null-truth one: ADR-0003 D4 fields, `statement_status` `null` + `"undetermined"` when the notice
leaves the truth open); then from `engine/`: `cargo run -q -p syndeom-ingest -- --config
<item.json> --bytes <original bytes> --root .. --engine-commit $(git rev-parse HEAD)`; then
`RECORD.md` into the item directory (the four Session-11/13 records are the pattern); the `.gz`
originals are binary in `.gitattributes` and the register checker;
(e) stop at the session's budget before examining the next candidate; update the log's counts
and last-candidate line, this file's Gate S1 table (S1.1, S1.3 rows), and set NEXT to the next
prefix. Grey zones are `escalated` and listed under standing escalations; nothing ambiguous is
ingested. Run the gate detached with a log before the commit (expect `GATE: GREEN`); commit
`[S1][ADR-0003] harvest Route A prefix through arXiv id <last id processed>: <n> admitted`.
