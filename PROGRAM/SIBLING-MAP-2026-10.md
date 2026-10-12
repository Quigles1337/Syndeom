# SIBLING-MAP — structural map of KOBER and FRIEDMAN, read at genesis (2026-10-12)

Purpose: the Session-1 deliverable "structural map" (genesis prompt step 0), kept in-repo so a
successor can see exactly what SYNDEOM's constitution mirrors and where the siblings differ. It is a
*description of two repositories at two commits*, not evidence about anything else. Nothing here
enters CLAIMS or SOURCES.

## 0. What was read, and where

| Program | Local path | Remote | Branch | Tip read | Working tree |
|---|---|---|---|---|---|
| KOBER | `C:\Users\LEET\KOBER` | `https://github.com/Quigles1337/KOBER.git` | `master` | `f94bec2` (2026-08-25) | clean, = `origin/master` |
| FRIEDMAN | `C:\Users\LEET\FR13DMAN` | `https://github.com/Quigles1337/FR13DMAN.git` | `main` | `7c3e5dd` (2026-08-25) | clean, = `origin/main` |

Read in full (read-only; the sibling working trees were not touched): both constitutions; both
`STATE.md`, `CLAIMS.md`, `SOURCES.md`, `HANDOFF.md`; KOBER `DECISIONS/ADR-0000..0002`; FRIEDMAN
`DECISIONS/ADR-0000..0004` and `SWEEP-2026-08.md`; both `README.md`, `.gitattributes`, `.gitignore`,
`.github/workflows/check.yml`, `scripts/check.sh`; KOBER `lean/lean-toolchain`, `lean/lakefile.toml`,
`lean/lake-manifest.json`, `lean/Kober.lean`, `lean/Kober/Basic.lean`, `Cargo.toml`, `Cargo.lock`,
`engine/Cargo.toml`, `engine/src/lib.rs`, `rust-toolchain.toml`; FRIEDMAN `rust-toolchain.toml`,
`engine/Cargo.toml`, `engine/Cargo.lock`, `engine/manifest/Cargo.toml`, `engine/manifest/src/lib.rs`,
`engine/manifest/tests/goldens.rs`, `lab/verify_adr0001.py`, `corpora/README.md`,
`gallery/README.md`, `quarantine/README.md`. Neither sibling has a file named `*session*`; session
logs are `PROGRAM/HANDOFF.md`.

## 1. Constitution skeleton (identical spine)

| Section | KOBER | FRIEDMAN | SYNDEOM draft |
|---|---|---|---|
| Title line | `# KOBER — Autonomous Engineering Program v1.0` | same pattern | same pattern + DRAFT marker |
| Header block | **Program**, **Named for**, **Engineer of record** (Al), **Executor**, "This file is the constitution" | adds **Sibling program** | adds **Sibling programs**, **Draft status** |
| §0 Charter | 4 items | 6 items (adds scoped novelty 0.4, bounded play 0.6) | 8 items (scope boundary, verdict ban, reserved "proven", two success states, scoped novelty, detector priority) |
| §1 Epistemic Constitution | **E1–E10** | **F1–F12** | **SD1–SD17** |
| §2 Phases and Gates | Phase 0–4, items `P0.1…`, **Gate G0…G4** | Phase F0–F4, items `F0.1…`, **Gate F0…F4** | Phase S0–S5, items `S0.1…`, **Gate S0…S5** |
| §3 State Architecture | 3.1–3.6 registers, 3.7 commit convention | 3.1–3.6 registers, 3.7 layout, 3.8 commit convention | 3.1–3.6 registers, **3.7 FLAWS.md**, 3.8 layout, 3.9 commit convention |
| §4 Metacognitive Loop | 4.1 ORIENT … 4.6 DECIDE | identical, with 3 extra CHECK fields in 4.5 | identical, with 4 CHECK fields in 4.5 |
| §5 Failure Modes → Countermeasures | table | table | table |
| §6 Session Prompts | Session 1 bootstrap; Session N resume | same | Session 1 genesis (run); Session 2 ratification; Session N resume |
| Footer | `*KOBER v1.0 — for Alice… Constitution amendments via ADR only.*` | same pattern | same pattern |

The gate rule is identical in both: a gate passes only with linked evidence in STATE.md **and** a
cold-start test (re-derive program state from the repo alone via §4.1, one pass, no guessing).

## 2. Rule sets and SYNDEOM's inheritance

| KOBER | FRIEDMAN | Theme | SYNDEOM |
|---|---|---|---|
| E1 zero sorry / zero new axioms, `#print axioms` pasted | — | kernel trust base | SD2 |
| E2 conditionality on an enumerated reading set | — | hypotheses explicit | (absorbed into SD2/SD12: ground truth explicit) |
| E3 statement fidelity | — | formal vs informal diff | SD2(iii); also FLAW-0008 |
| E4 provenance per data point | F1 manifest per corpus | ground truth | SD1 |
| E5 no fabricated scholarship | F11 (adds sweep-as-map) | citations | SD11 |
| E6 small-N honesty | (F3.2 bootstrap CIs; §0.3 negative result) | power | SD4 second sentence; §0.5(b) |
| E7 source of truth over assumption | F7 | never assume | SD7 (+ Al's standing rule verbatim-ish) |
| E8 pre-registration / freeze / RETRACTED trail | F8 (adds quarantine + leakage) | freeze | SD8 (adds battery immutability on contact) |
| E9 green default branch | F9 | gate | SD9 |
| E10 licences and credit | (F11 licences before ingestion) | licences | SD16 |
| — | F2 no single-statistic verdicts | joint evidence | SD4 |
| — | F3 Dembski rule (measured FPR) | FPR | SD4 |
| — | F4 structured nulls | strawman nulls | SD5 |
| — | F5 target fidelity (commanded vs achieved) | generator fidelity | S2.2 fidelity verdict |
| — | F6 determinism | reproducibility | SD6 (+ LLM-assisted clause) |
| — | F10 two-tier code | lab vs engine | SD10 (+ lean/ canonical) |
| — | F12 verdict discipline | no verdicts | SD3 |
| — | ADR-0003 memory discipline | memory | SD15 |
| — | — | referee protocol gating | SD13 |
| — | — | interface not dependency (F0.3 text) | SD14 |
| — | — | ground-truth corpus integrity | SD12 |
| — | — | scope boundary | SD17 |

## 3. Phase register format

Both siblings: `### Phase <X> — NAME`, one tagline line, bulleted items `- <X>.<n> …` (bold item
names in FRIEDMAN), then `**Gate <X>:**` listing exit criteria and ending with "cold-start test".
Sub-steps inside an item are prose; reproductions/claims are named inline (`CLAIM-0001`, `R1`).
SYNDEOM follows FRIEDMAN's bold-item style. Phase lists: KOBER 0 FOUNDATION → 1 THEOREM → 2 ENGINE →
3 BATTERY → 4 VERDICT; FRIEDMAN F0 FOUNDATION → F1 REPRODUCTIONS → F2 GENERATOR → F3 CALIBRATION →
F4 CONTACT; SYNDEOM S0 FOUNDATION AND CITATION VERIFICATION → S1 GROUND-TRUTH CORPUS → S2 GENERATOR
→ S3 DETECTOR BATTERY, CALIBRATION, FREEZE → S4 BLIND VALIDATION → S5 CONTACT (as specified by Al).

## 4. Register types and schemas

| Register | KOBER | FRIEDMAN | Notes for SYNDEOM |
|---|---|---|---|
| `STATE.md` | header (Updated / Phase / Gate / Default branch + ORIENT note / Build status), gate table `Criterion \| Status \| Evidence`, pre-staged next gate table, Blockers, Standing escalations, **NEXT-SINGLE-ACTION** | same, plus a **Toolchain** line and a Memory-policy section | same shape; adds the open-question list Q1–Q17 under Standing escalations |
| `CLAIMS.md` | `ID \| informal statement \| status \| evidence \| reading-set deps \| axiom print (if PROVEN) \| transitions log`; statuses UNKNOWN → VERIFIED / **PROVEN** / CONDITIONAL / FALSIFIED / RETRACTED | `… \| manifest/config deps \| …`; statuses add **REPRODUCED** and **MEASURED** | union: PROVEN + REPRODUCED + MEASURED; deps = manifest/taxonomy/battery; axiom print + fidelity check |
| `SOURCES.md` | states SURVEYED / INGESTED; stored copies under `sources/` with SHA-256 (the S-0001 PDF is committed under CC BY-NC-ND) | storage rule: commit only if licence permits; S-0005 is a dependency-licence table | adds UNVERIFIED / VERIFIED states (genesis requirement); FRIEDMAN storage rule adopted |
| `DECISIONS/ADR-NNNN.md` | `- **Status:** accepted (Session N, date)`; Context / Options / Choice / Rationale / Consequences | `- **Date:**`, `- **Status:**`; Context / Options considered / Decision (D1, D2 …) / Consequences | FRIEDMAN's D-numbered form |
| `HANDOFF.md` | entries inside ``` fences, `---` separators | plain text, `---` separators | plain text |
| `SWEEP-2026-08.md` | — | founding sweep with the F11 banner at line 1 and a ferry note at the end | `SWEEP-<YYYY-MM>.md` if Al ferries one |
| Commit convention | `[P<phase>][CLAIM-ID\|ADR-ID\|INFRA]` | `[F<phase>][CLAIM-ID\|ADR-ID\|INFRA]` | `[S<phase>][CLAIM-ID\|ADR-ID\|FLAW-ID\|INFRA]` |

ADR numbering as used: KOBER 0000 bootstrap, 0001 Lean pin + Mathlib, 0002 layout + Rust + CI;
FRIEDMAN 0000 bootstrap, 0001 manifest schema + KOBER handshake, 0002 toolchain + workspace + JCS +
commit policy + gate, 0003 memory discipline (ratified by Al), 0004 verdict arithmetic pinned to
binary64 (ratified by Al). KOBER has ADR-0003 *reserved* for the memory policy, not yet drafted.

## 5. The session loop (§4) and the HANDOFF block

Identical in both: 4.1 ORIENT (three git commands; read STATE → HANDOFF last entry → CLAIMS skim →
DECISIONS; then run the gate — "state files are claims; builds are evidence") → 4.2 ASSESS → 4.3
PLAN one thing → 4.4 EXECUTE + VERIFY (two-strike rule) → 4.5 RECORD + REFLECT → 4.6 DECIDE (stop
while green; escalation list).

HANDOFF fields: `SESSION`, `PHASE / GATE`, `ADVANCED`, `BELIEF DELTAS`, `CONFUSIONS`,
`HONESTY CHECK`, [FRIEDMAN only: `LEAKAGE CHECK`, `DETERMINISM CHECK`], `DRIFT CHECK`,
`NEXT-SINGLE-ACTION`, `ESCALATIONS`, `STOP-REASON`. Exam sessions (both programs, 2026-08-25) open
the block with a **verbatim disclosure of recalled memory** before anything else. SYNDEOM keeps
FRIEDMAN's three extra checks and adds `MANIFEST CHECK`.

Both programs' ORIENT notes record that `git symbolic-ref refs/remotes/origin/HEAD` can fail on a
working copy that was `git init`-ed and remote-added later; fallback `git remote show origin`; repair
`git remote set-head origin -a`. SYNDEOM's §4.1 carries the fallback inline.

## 6. The §6 resume-prompt convention and the memory discipline

- §6 holds two prompts: **Session 1 — bootstrap** (constitution placement, register creation, run
  the loop from Phase 0) and **Session N — resume** (read the constitution in full; run §4 exactly;
  honor escalations). The resume prompt is **state-free by design**: all state lives in STATE.md.
- FRIEDMAN ADR-0003 (ratified by Al 2026-08-25): **D1** memories hold environment/tooling hazards
  and repo location only, never program state; **D2** an exam session reproduces every recalled
  memory verbatim first, and the exam counts only if none holds program state (unrelated private
  programs disclosed by count); **D3** no memory writes in an exam session. **Corollary** (a session
  in the builder's conversation context is not cold, extended to prompts written by the
  architects): recorded as *unratified* in FRIEDMAN ADR-0003 and STATE; recorded as *ratified by the
  architects' Session-5 prompt* in KOBER STATE (which also reads the extension as: a cold session is
  launched with the §6 resume prompt verbatim and nothing else). **The two records differ** — Q13.
- Consequence both programs hit: a gate's cold-start test must be run in a fresh conversation with a
  purged memory store, launched with the §6 resume prompt verbatim. Both programs' G0/F0 gates were
  still open on that criterion at the tips read.

## 7. Schema files

### 7.1 FRIEDMAN corpus manifest — `friedman-manifest/1` (ADR-0001, enforced by crate `friedman-manifest` at `engine/manifest/src/lib.rs`, tip `7c3e5dd`)

- **D1 format:** JSON restricted to I-JSON; on disk UTF-8, no BOM, 2-space indent, keys in JCS
  order, single trailing LF; file `manifest.json`.
- **D2 layout:** `corpora/<corpus_id>/manifest.json` + `corpus.txt`;
  `corpus_id = <generator.id>-v<generator.version>-<config_hash[0:12]>-s<seed>` — a pure function of
  (generator, config, seed), tested as an engine invariant.
- **D3 corpus byte format `friedman-tokens/1`:** one document per line, tokens separated by one
  U+0020, no edge spaces, every line LF-terminated, no empty lines, no CR, no BOM.
- **D4 fields (all required unless noted):** `manifest_schema` (`"friedman-manifest/1"`),
  `corpus_id`, `origin` (`"synthetic"`; `"natural"`, `"quarantine"` reserved), `generator`
  {`id`, `version` semver, `rng`}, `engine_commit` (40-hex), `seed` (u64), `config` (full, no
  defaults elided), `config_hash`, `ground_truth` {`label` = `"mechanism"` always, `family` =
  `generator.id`, `derivation` = `"by-construction"`}, `corpus` {`file`, `format`, `content_hash`,
  `n_documents`, `n_tokens`, `n_types`, `doc_length` {`min`,`max`,`mean`}}, `commanded`
  (stat → {`value`, `tolerance`}), `achieved` (stat → {`value` or null, `estimator`, `ci95` or null,
  `reason` iff null}), `fidelity` {`per_statistic`, `verdict`}, `notes` (optional). **No wall-clock
  timestamps** (byte-identical regeneration). Tolerance forms `{"abs": a}`, `{"rel": r}`,
  `{"exact": true}`; verdict on the point estimate, `ci95` never enters it; missing/null achieved →
  FAIL.
- **D5 hashing:** SHA-256 lowercase hex; `config_hash` over the RFC 8785 (JCS) canonical bytes of
  `config` (crate `serde_json_canonicalizer 0.3.2`, verified against the RFC's own vectors);
  `content_hash` over the exact corpus bytes.
- **D6 golden:** corpus `ba ka ba\nka\n` (12 bytes) → `content_hash 8d106b97…75fd`; config
  `{"alphabet":["ba","ka"],"order":1,"transition_bias":0.75}` (57 JCS bytes) →
  `config_hash 22cdc154…dc33`; `corpus_id markov-v1.0.0-22cdc154338b-s7`.
- **D7 handshake surface:** a consumer may use exactly `corpora/<corpus_id>/{manifest.json,
  corpus.txt}`; the fields it needs are `ground_truth.label`, `ground_truth.family`,
  `corpus.format`, `corpus.content_hash`, `fidelity.verdict` (refuse FAIL). Copy by content hash;
  never import code. Schema evolution = new version string; consumers pin the string they
  understand.
- **ADR-0004:** the D4 arithmetic is IEEE 754 binary64, round-to-nearest-even, no FMA, `<=`
  inclusive; seven conformance vectors identical in Rust and Python; "any KOBER consumer must pass
  them before consuming a manifest".
- **Crate facts that constrain "compatibility":** every struct is `#[serde(deny_unknown_fields)]`;
  `Manifest::check` rejects `manifest_schema ≠ "friedman-manifest/1"`, `origin ≠ "synthetic"`,
  `ground_truth.label ≠ "mechanism"`. A FRIEDMAN consumer therefore cannot parse a SYNDEOM manifest
  that adds fields or uses another schema string — see Q10.
- **Commit policy (ADR-0002 D3):** manifests committed; `corpus.txt` gitignored and regenerated;
  golden fixtures live in `engine/**/tests/`.

### 7.2 KOBER Lean configuration (`lean/`, tip `f94bec2`)

- `lean/lean-toolchain`: `leanprover/lean4:v4.33.1`
- `lean/lakefile.toml`: `name = "kober"`, `defaultTargets = ["Kober"]`, one `[[lean_lib]] name = "Kober"`
- `lean/lake-manifest.json`: `version "1.2.0"`, `packagesDir ".lake/packages"`, `packages []`,
  `fixedToolchain false` — **no Mathlib** (ADR-0001: finite decidable arithmetic only; Mathlib would
  be an ADR amendment).
- Gate runs `cd lean && lean --version && lake build` — the `cd` matters: the elan shim resolves the
  toolchain from the working directory, and `lake --dir` does not anchor it (KOBER CI run 1 failed
  on exactly this).

## 8. Toolchain pins (siblings) and machine inventory (2026-10-12)

| Item | KOBER | FRIEDMAN | Installed on this machine today |
|---|---|---|---|
| Lean | `leanprover/lean4:v4.33.1`, no Mathlib | — | elan 4.2.3; v4.15.0, v4.32.2, v4.33.0, v4.33.1, v4.34.0-rc1 installed; elan's default `stable` now resolves to **v4.35.0 (not installed)**; `lean --version` in `KOBER/lean` prints 4.33.1, Lake 5.0.0 |
| Rust channel | `1.91.0` | `1.91.0` | rustup 1.29.0; 1.70.0, 1.85.0, **1.91.0**, 1.97.1, stable (= 1.91.0), nightly; rustc 1.91.0 (f8297e351 2025-10-28), cargo 1.91.0 |
| Rust components | `["clippy"]` | `["clippy", "rustfmt"]` | clippy 0.1.91 and rustfmt 1.8.0-stable present for 1.91.0 |
| Edition | 2024 | 2024 | — |
| Workspace root | `/Cargo.toml` (member `engine/`, crate `kober-engine`, no deps) | `engine/Cargo.toml` (member `manifest`, crate `friedman-manifest`, `publish = false`) | — |
| Crate deps | none | serde 1.0.229 (derive), serde_json 1.0.151 (`float_roundtrip`), serde_json_canonicalizer 0.3.2, sha2 0.11.0, hex 0.4.3 (24 locked crates, licences in S-0005) | — |
| Python | — | lab verifier (stdlib only) | `python` 3.11.9, `python3` 3.13.14 |
| Other | poppler 26.02.0 at `C:\Users\LEET\tools\poppler-26.02.0` (recorded in KOBER S-0001) | cargo-audit 0.22.2 (recorded in STATE) | gh 2.74.0 (authenticated Quigles1337), git 2.49.0.windows.1, node 22.23.1 |

Nothing in this table is a SYNDEOM pin. Pins are decided by ADR-0002 at S0.6 after re-discovery.

## 9. Layout, gate, and CI

| | KOBER | FRIEDMAN |
|---|---|---|
| Directories | `PROGRAM/ analysis/ engine/ lean/ scripts/ sources/` | `PROGRAM/ corpora/ engine/ gallery/ lab/ quarantine/ scripts/` |
| `.gitignore` | `.lake/`, `/target/`, OS noise | `engine/target/`, Python caches, `corpora/**/corpus.txt` |
| `.gitattributes` | `* text=auto eol=lf` + pdf/png/jpg/zip binary | byte-identical content |
| `scripts/check.sh` | `cd lean && lean --version && lake build`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; prints `ALL GREEN` | `cd engine`; `rustc --version`; `cargo --version`; `cargo fmt --all -- --check`; `cargo test --workspace`; `cargo clippy … -D warnings`; `python lab/verify_adr0001.py`; prints `ALL GREEN` |
| CI `check.yml` | push to `master` + PR; `actions/checkout@v4`, `actions/cache@v4`; installs elan with `--default-toolchain none`; caches `~/.elan`, cargo registry, `target` | push to `main` + PR; `actions/checkout@v5`, `actions/cache@v5` (bumped by Al's instruction, `91faff1`); toolchain read from `rust-toolchain.toml` by `sed`, never hard-coded |
| Standing CI rule | never a *required* status check (billing history: 0-step failed jobs) | same |
| Directory READMEs | — | `corpora/`, `gallery/`, `quarantine/` (quotes F8 verbatim; checked programmatically) |

## 10. Divergences between the siblings (each becomes a question in STATE.md)

1. Default branch `master` vs `main` → Q2 (SYNDEOM discovered `main`).
2. Rule prefix `E` vs `F` → Q4 (SYNDEOM draft uses `SD`).
3. Phase/commit tag `[P…]` vs `[F…]` → Q5 (SYNDEOM draft uses `[S…]`).
4. Rust components clippy vs clippy+rustfmt; gate with/without `fmt --check` → Q6.
5. Workspace root repo root vs `engine/` → Q7.
6. Lean present (4.33.1, no Mathlib) vs absent → Q8 (SYNDEOM needs mathlib pairs in S1.2).
7. CI action versions v4 vs v5 → Q9.
8. CLAIMS statuses (PROVEN vs REPRODUCED/MEASURED) → union adopted; no question.
9. HANDOFF fences vs plain; ADR form → FRIEDMAN's adopted; no question.
10. Memory-discipline corollary ratified (KOBER record) vs unratified (FRIEDMAN record) → Q13.
11. Stored source copies committed (KOBER, licence permitting) vs digest-only (FRIEDMAN) → both
    rules are the same rule ("commit only if licence permits"); adopted; no question.
12. Commit trailers: FRIEDMAN bootstrap carried `Co-Authored-By` + `Claude-Session`; all later
    commits in both repos carry none → Q16.

## 11. Hazards recorded by the siblings (machine and harness; D1-legal memory content)

- `core.autocrlf = true` at system level; `.gitattributes` LF pin is mandatory; verify with
  `git ls-files --eol`, never by eye; MSYS `grep` reports false carriage returns.
- Long heredocs fail to parse in this harness; a bare `cat > file` blocks on stdin; `sed -i` and
  `python -c` replacement strings turn `\r`/`\xef` escapes into raw bytes. Write multi-line files
  with the editor tools; pass commit messages via `-F file`.
- A `grep` inside an `&&` chain is not a guard (a match exits 0). Never filter a command whose
  failure you need to see (`2>&1 | grep` hid a missing-file error).
- `awk '{print \$1}'` inside a double-quoted shell string breaks; use `cut` or single quotes.
- Deep `find` over the home directory hangs on OneDrive reparse points; bound searches with
  `-maxdepth` or a specific directory.
- The Bash tool's working directory persists between calls; use absolute paths.
- Publisher pages (ScienceDirect, MDPI, Taylor & Francis, academia.edu) return 403 to tool fetches;
  Crossref, arXiv, the GitHub API, and institutional repositories work. `winget` was broken once.
- elan resolves the toolchain from the working directory; `lake --dir` does not anchor it — `cd`
  into the Lean project in the gate.
- GitHub Actions billing suspensions produce 0-step failed jobs that look like red builds; check
  step counts before believing a red run; never require a status check.
- `git symbolic-ref refs/remotes/origin/HEAD` fails on an init-then-remote-add working copy;
  fallback `git remote show origin`; repair `git remote set-head origin -a`.
- STATE's "last CI success" line lags the last push by one run by construction; `gh run list` at
  ORIENT is the evidence.
- The sibling working trees can change mid-session (a sibling session may be live); never touch
  them, and treat cross-repo observations as instantly stale.
