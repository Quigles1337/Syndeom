# STATE

- **Updated:** 2026-10-12 (Session 2 — ratification + S0.1, close)
- **Phase:** S0 — FOUNDATION AND CITATION VERIFICATION
- **Gate:** S0 open. Done: S0.1 (sources register), S0.3 (constitution ratified), S0.5 (taxonomy
  0.1.0 ratified). Open: S0.4 (manifest ADR), S0.6 (scaffold), cold-start test. S0.2 waits on a
  sweep Al will ferry later (Q14) and does not block.
- **Default branch:** `main` — discovered from the clone of the empty remote and the REST API
  (ADR-0000 D3); confirmed by Al (Q2). Remote: `origin` = `https://github.com/Quigles1337/Syndeom`
  (**public**; created by Al 2026-10-12; Actions enabled; no `LICENSE` file — Q3 deferred).
  **ORIENT note (SD7):** `git symbolic-ref refs/remotes/origin/HEAD` resolves to `main`; if it ever
  fails, fallback `git remote show origin`, repair `git remote set-head origin -a`.
- **Build status:** no gate exists yet (`scripts/check.sh` lands at S0.6). Until then "green" means
  `git ls-files --eol` shows every tracked **text** file as `i/lf` (the two PDFs under `sources/`
  are binary by `.gitattributes`) and the register cross-reference check passes. Last verified:
  2026-10-12 before the Session-2 close commit.
- **Toolchain:** nothing pinned yet. Ruled for ADR-0002 (S0.6): Rust from the siblings'
  `rust-toolchain.toml` as re-read then (expected 1.91.0, edition 2024, clippy + rustfmt, `fmt
  --check` in the gate), workspace root `engine/Cargo.toml`; **Lean version follows Mathlib's
  `lean-toolchain` at a pinned Mathlib commit** (Q8 — a load-bearing divergence from KOBER's
  v4.33.1/no-Mathlib); CI actions `@v5`, never a required status check. Machine inventory as of
  2026-10-12 in `PROGRAM/SIBLING-MAP-2026-10.md` §8. For the record (S5, not a SYNDEOM pin): the
  contact corpus `openai/math` uses `leanprover/lean4:v4.34.1` with Mathlib `d13f23b7…` (S-0001).
- **Constitution:** `PROGRAM/SYNDEOM-PROGRAM.md` v1.0, ratified by Al 2026-10-12 (Session 2);
  rulings of record in the ADR-0000 addendum. ADR-only.
- **Taxonomy:** `PROGRAM/FLAWS.md` 0.1.0 ratified (FLAW-0001..0014).
- **Registers:** CLAIM-0001..0005 and 0007 VERIFIED (0007 existence only); CLAIM-0006 UNKNOWN by
  design (no sweep). S-0001..S-0006 VERIFIED; S-0004 and S-0005 INGESTED (two PDFs under `sources/`,
  CC0 and CC BY-NC 4.0).

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
- **ADR numbers reserved:** ADR-0001 manifest schema (S0.4; constraints Q10–Q11 in the ADR-0000
  addendum); ADR-0002 toolchain pins and scaffold (S0.6; constraints Q6–Q9, Q17).

## Gate S0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| S0.1 CLAIM-0001..0005 (and 0007) resolved to VERIFIED or honestly UNVERIFIED with consequences | **done** | all six VERIFIED 2026-10-12 from primary sources (CLAIMS.md transitions; SOURCES.md S-0001..S-0006 with URLs, access times, licences, digests). Consequences stated: S5 corpus location = `github.com/openai/math` (Apache-2.0); counts carry units and three unreconciled formalization units; the AHM page is undated; IAS page unretrievable (403); signatory counts recorded as of access date |
| S0.2 Founding sweep placed with the SD11 banner | **waiting on Al's ferry (Q14); not blocking** | none exists as of 2026-10-12; §0.6 holds; CLAIM-0006 UNKNOWN |
| S0.3 Constitution ratified by Al; DRAFT marker removed | **done** | `c803c96` `[S0][ADR-0000] ratify the constitution and record Al's rulings`; ADR-0000 addendum |
| S0.4 Manifest schema ADR (`syndeom-manifest/1` extending `friedman-manifest/1`) | **open — NEXT** | ruled constraints Q10 (structural) and Q11 (fields) in the ADR-0000 addendum and §2 S0.4; FRIEDMAN D1–D7 and crate facts in SIBLING-MAP §7.1 |
| S0.5 Flaw taxonomy v0.1 ratified | **done** | `PROGRAM/FLAWS.md` 0.1.0, 14 entries, ratified (Q12; changelog) at `c803c96` |
| S0.6 Scaffold (`lean/` with Mathlib, `engine/`, `lab/`, `corpora/`, `groundtruth/`, `contact/`, `analysis/`, `sources/`, gate, CI) | **open** | rulings Q6–Q9, Q17 recorded (ADR-0000 addendum; §2 S0.6); `sources/` already exists (S0.1 copies) |
| Licences recorded for anything touched | **done** | S-0001 Apache-2.0 (nothing copied); S-0002, S-0003 licence unstated (nothing copied); S-0004 CC0 and S-0005 CC BY-NC 4.0 (PDFs copied under those terms); no dependency added |
| Repo green under the gate | **n/a until S0.6** | `git ls-files --eol` all text files `i/lf` at every commit so far; cross-reference check passes |
| Cold-start test (§2, SD15) | **open** | requires a cold session (fresh conversation, purged memory, §6 resume prompt verbatim) after S0.4 and S0.6 |

## Blockers

- None. S0.4 and S0.6 are unblocked by the rulings.

## Standing escalations (awaiting Al)

- **Q3 — Repository LICENSE.** Public repo, no licence file. Al's decision, due before S5.4 and
  before any paper cites the repo. Not a blocker.
- **Q14 — Founding sweep.** Al will ferry one later (path + SHA-256); the executor then places it at
  `PROGRAM/SWEEP-<YYYY-MM>.md` under the SD11 banner after verifying identity. Until then, no
  novelty claim.
- **Q15 — Ferry on failure.** Nothing failed outright in S0.1; the one unreachable page (the IAS
  project page, HTTP 403) is corroborative only. No ferry needed now.
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## NEXT-SINGLE-ACTION

**S0.4 — write ADR-0001, the proof-artifact manifest schema `syndeom-manifest/1`.** Procedure:
(1) read FRIEDMAN `PROGRAM/DECISIONS/ADR-0001.md` D1–D7, ADR-0004, and `engine/manifest/src/lib.rs`
at a commit you pin in the ADR (the sibling tip read in Session 1 was `7c3e5dd`; re-read, do not
assume it is unchanged); (2) decide under the Q10 ruling (structural compatibility; new schema
string; handshake per FRIEDMAN D7; no FRIEDMAN-side change) and the Q11 constraints (ADR-0000
addendum: `origin`, derivation, artifact kind {informal, formal, pair} with one artifact id, Markdown
with LaTeX and generator-numbered steps, one Lean file, `location` forms, two-dimensional ground
truth with `flaws[]` fields, `taxonomy_version`, commanded/achieved vocabulary, the SD6 LLM clause);
(3) keep FRIEDMAN's hashing (RFC 8785 JCS + SHA-256), `corpus_id`-style pure-function ids, no
timestamps, and the D4 binary64 tolerance arithmetic by reference; (4) include hand-computed goldens
in the FRIEDMAN D6 style for one informal artifact and one Lean artifact (fixed bytes, digests
computed with `sha256sum`, no placeholders in digests); (5) state the step-segmentation rule for
informal `location`; (6) commit `[S0][ADR-0001] decide the proof-artifact manifest schema`. Then set
NEXT-SINGLE-ACTION = S0.6 (ADR-0002 scaffold, beginning with `elan` and Mathlib pin discovery).
