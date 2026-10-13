# STATE

- **Updated:** 2026-10-12 (Session 3 — Q14 ferry + S0.4 ADR-0001, close)
- **Phase:** S0 — FOUNDATION AND CITATION VERIFICATION
- **Gate:** S0 open. Done: S0.1 (sources register), S0.2 (founding sweep placed), S0.3
  (constitution ratified), S0.4 (manifest ADR-0001), S0.5 (taxonomy 0.1.0 ratified). Open: S0.6
  (scaffold), the deferred CLAIM-0006 examination, cold-start test.
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
  design (sweep placed; its ten ranked sources not yet retrieved). S-0001..S-0007 VERIFIED; S-0004
  and S-0005 INGESTED (two PDFs under `sources/`, CC0 and CC BY-NC 4.0); S-0007 is RFC 8785
  (digest-only).
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
- **ADR numbers:** ADR-0001 manifest schema — **written** (S0.4, Session 3; five choices flagged
  for Al in its Consequences, amendable by ADR before S1.3/S2.1); ADR-0002 toolchain pins and
  scaffold — reserved (S0.6; constraints Q6–Q9, Q17; also decides the JCS crate, the synthetic
  storage default, and the gate, per ADR-0001 Consequences).

## Deferred S0 actions (scheduled, not NEXT)

- **S0.1 continuation — examine CLAIM-0006 (ruled by Al, Session 3):** retrieve and read, from
  primary sources, the ten sources ranked in `PROGRAM/SWEEP-2026-10.md` under "Verdict (3)". Each
  enters SOURCES.md only on retrieval, as UNVERIFIED-then-VERIFIED in the ordinary way (SD11); the
  sweep's descriptions of them are hints, not citations. Only after all ten are read does CLAIM-0006
  move (to VERIFIED in the sweep's narrowed form, or to FALSIFIED by a counterexample). Runs after
  S0.4 and S0.6, before the Gate S0 cold-start test is attempted, as one session's single action.

## Gate S0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| S0.1 CLAIM-0001..0005 (and 0007) resolved to VERIFIED or honestly UNVERIFIED with consequences | **done** | all six VERIFIED 2026-10-12 from primary sources (CLAIMS.md transitions; SOURCES.md S-0001..S-0006 with URLs, access times, licences, digests). Consequences stated: S5 corpus location = `github.com/openai/math` (Apache-2.0); counts carry units and three unreconciled formalization units; the AHM page is undated; IAS page unretrievable (403); signatory counts recorded as of access date |
| S0.2 Founding sweep placed with the SD11 banner | **done** | `PROGRAM/SWEEP-2026-10.md`, SHA-256 `f4bfb2ad…b03e` = Al's stated digest (verified before the move, Session 3); banner present; redacted under SD8 before ferry; reconciliation note defers to the S0.1 register. §0.6 still holds: no novelty claim until CLAIM-0006 is examined |
| S0.3 Constitution ratified by Al; DRAFT marker removed | **done** | `c803c96` `[S0][ADR-0000] ratify the constitution and record Al's rulings`; ADR-0000 addendum |
| S0.4 Manifest schema ADR (`syndeom-manifest/1` extending `friedman-manifest/1`) | **done** | `PROGRAM/DECISIONS/ADR-0001.md` (Session 3): D1–D9 under Q10 (structural) and Q11 (fields); FRIEDMAN ADR-0001/0004 and crate re-read at pinned tip `7c3e5dd`; two goldens over fixed bytes (Golden A `0ac16447…a6a1`, Golden B `c568a981…458e`) with digests, counts, span example, ids; RFC 8785 retrieved as S-0007. Five open design choices flagged for Al in its Consequences (non-blocking) |
| S0.5 Flaw taxonomy v0.1 ratified | **done** | `PROGRAM/FLAWS.md` 0.1.0, 14 entries, ratified (Q12; changelog) at `c803c96` |
| S0.6 Scaffold (`lean/` with Mathlib, `engine/`, `lab/`, `corpora/`, `groundtruth/`, `contact/`, `analysis/`, `sources/`, gate, CI) | **open** | rulings Q6–Q9, Q17 recorded (ADR-0000 addendum; §2 S0.6); `sources/` already exists (S0.1 copies) |
| Licences recorded for anything touched | **done** | S-0001 Apache-2.0 (nothing copied); S-0002, S-0003 licence unstated (nothing copied); S-0004 CC0 and S-0005 CC BY-NC 4.0 (PDFs copied under those terms); no dependency added |
| Repo green under the gate | **n/a until S0.6** | `git ls-files --eol` all text files `i/lf` at every commit so far; cross-reference check passes |
| Cold-start test (§2, SD15) | **open** | requires a cold session (fresh conversation, purged memory, §6 resume prompt verbatim) after S0.4 and S0.6 |

## Blockers

- None. S0.4 and S0.6 are unblocked by the rulings.

## Standing escalations (awaiting Al)

- **Q3 — Repository LICENSE.** Public repo, no licence file. Al's decision, due before S5.4 and
  before any paper cites the repo. Not a blocker. The Session-3 prompt's Q3 slot carried its
  placeholder text unfilled, so no ruling was delivered; Q3 remains deferred, unchanged.
- **Q14 — Founding sweep.** **Resolved in Session 3:** Al ferried the sweep by pasting its text
  (no file path was given); the pasted bytes, LF with one trailing newline, hashed to Al's stated
  digest exactly, and the file was placed. Nothing further is awaited.
- **Q15 — Ferry on failure.** Nothing failed outright in S0.1; the one unreachable page (the IAS
  project page, HTTP 403) is corroborative only. No ferry needed now.
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## NEXT-SINGLE-ACTION

**S0.6 — write ADR-0002 and land the scaffold.** Procedure, every value read, none assumed (SD7):
(1) **Discovery first, recorded in the ADR:** `elan --version`, `elan show`, `lake --version`;
Mathlib's current `lean-toolchain` and the commit it is read at (`gh api repos/leanprover-community/mathlib4/contents/lean-toolchain?ref=<sha>` after choosing the pin from `git ls-remote` or the API — the pin is the commit, the Lean version is whatever that commit's `lean-toolchain` says, Q8); the siblings' `rust-toolchain.toml` as re-read then (FRIEDMAN's at `7c3e5dd` says channel `1.91.0`, components clippy + rustfmt — confirm KOBER's too, and `rustc --version`/`cargo --version` on this machine); crates.io for the JCS serializer (FRIEDMAN chose `serde_json_canonicalizer 0.3.2`; re-read, then test against the RFC 8785 vectors from S-0007); whether GitHub Actions is available on this repo (it is enabled per Q2 notes — verify, and keep CI advisory, never a required check, Q9; `actions/*@v5`).
(2) **Decide in ADR-0002:** Lean toolchain + Mathlib pin + `lake` layout under `lean/` (gate uses Mathlib's cache and builds only this project's modules); `engine/Cargo.toml` workspace root (Q7) with a `manifest` crate implementing ADR-0001 D1–D7 and the D8 goldens + ADR-0004 vectors as tests; JCS crate; synthetic storage default (ADR-0001 D2); directories `lab/`, `corpora/`, `groundtruth/`, `contact/` (empty, README posting the SD8 rules), `analysis/`, `sources/` (exists) (Q17); `scripts/check.sh` = `lake build` + `cargo test` + `cargo clippy -D warnings` + `cargo fmt --check` (Q6); `.gitignore` additions (`.lake/`, `engine/target/`, and the synthetic storage rule).
(3) **Land it and prove it:** `bash scripts/check.sh` green locally, including Golden B's build at the pinned toolchain (record its `lean_build` and `#print axioms` outcome in the ADR as measured, replacing the "unasserted" note by a new transition, never by editing ADR-0001's text); LF check; register check.
(4) Commit `[S0][ADR-0002] pin the toolchains and land the scaffold and gate`, push, then set NEXT-SINGLE-ACTION = the deferred CLAIM-0006 examination (STATE "Deferred S0 actions"), after which the Gate S0 cold-start test is attempted in a cold session (SD15).
