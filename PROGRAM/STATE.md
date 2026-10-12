# STATE

- **Updated:** 2026-10-12 (Session 2 — ratification)
- **Phase:** S0 — FOUNDATION AND CITATION VERIFICATION
- **Gate:** S0 open. Done: S0.3 (constitution ratified), S0.5 (taxonomy 0.1.0 ratified). Open: S0.1
  (sources), S0.4 (manifest ADR), S0.6 (scaffold), cold-start test. S0.2 waits on a sweep Al will
  ferry later (Q14) and does not block.
- **Default branch:** `main` — discovered from the clone of the empty remote and the REST API
  (ADR-0000 D3); confirmed by Al (Q2). Remote: `origin` = `https://github.com/Quigles1337/Syndeom`
  (**public**; created by Al 2026-10-12; Actions enabled; no `LICENSE` file — Q3 deferred).
  **ORIENT note (SD7):** `git symbolic-ref refs/remotes/origin/HEAD` resolved to `main` in Session 2
  after the first push; if it ever fails, fallback `git remote show origin`, repair
  `git remote set-head origin -a`.
- **Build status:** no gate exists yet (`scripts/check.sh` lands at S0.6). Until then "green" means
  `git ls-files --eol` shows every tracked file as `i/lf` and the registers are coherent. Last
  verified: 2026-10-12, Session 2 ORIENT at `1fe9c06` (11/11 `i/lf`, cross-reference check ALL PASS)
  and again before each Session-2 commit.
- **Toolchain:** nothing pinned yet. Ruled for ADR-0002 (S0.6): Rust from the siblings'
  `rust-toolchain.toml` as re-read then (expected 1.91.0, edition 2024, clippy + rustfmt, `fmt
  --check` in the gate), workspace root `engine/Cargo.toml`; **Lean version follows Mathlib's
  `lean-toolchain` at a pinned Mathlib commit** (Q8 — a load-bearing divergence from KOBER's
  v4.33.1/no-Mathlib); CI actions `@v5`, never a required status check. Machine inventory as of
  2026-10-12 in `PROGRAM/SIBLING-MAP-2026-10.md` §8.
- **Constitution:** `PROGRAM/SYNDEOM-PROGRAM.md` v1.0, **ratified by Al 2026-10-12 (Session 2)**;
  rulings of record in the ADR-0000 addendum. ADR-only from here.
- **Taxonomy:** `PROGRAM/FLAWS.md` 0.1.0 ratified (FLAW-0001..0014).

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
  need per-corpus toolchains. Note, not solve, until ingest (Q8).
- **ADR numbers reserved:** ADR-0001 manifest schema (S0.4; constraints Q10–Q11 in the ADR-0000
  addendum); ADR-0002 toolchain pins and scaffold (S0.6; constraints Q6–Q9, Q17).

## Gate S0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| S0.1 CLAIM-0001..0005 (and 0007) resolved to VERIFIED or honestly UNVERIFIED with consequences | **open — NEXT** | seeded UNKNOWN; S-0001..S-0006 UNVERIFIED with Al's retrieval hints recorded as "named by Al" (SOURCES.md) |
| S0.2 Founding sweep placed with the SD11 banner | **waiting on Al's ferry (Q14); not blocking** | none exists as of 2026-10-12; §0.6 holds; CLAIM-0006 UNKNOWN |
| S0.3 Constitution ratified by Al; DRAFT marker removed | **done** | Session 2 ratification commit (`[S0][ADR-0000] ratify the constitution and record Al's rulings`; SHA in HANDOFF Session 2); ADR-0000 addendum |
| S0.4 Manifest schema ADR (`syndeom-manifest/1` extending `friedman-manifest/1`) | **open** | ruled constraints Q10 (structural) and Q11 (fields) in the ADR-0000 addendum and §2 S0.4; FRIEDMAN D1–D7 and crate facts in SIBLING-MAP §7.1 |
| S0.5 Flaw taxonomy v0.1 ratified | **done** | `PROGRAM/FLAWS.md` 0.1.0, 14 entries, ratified (Q12; changelog) |
| S0.6 Scaffold (`lean/` with Mathlib, `engine/`, `lab/`, `corpora/`, `groundtruth/`, `contact/`, `analysis/`, `sources/`, gate, CI) | **open** | rulings Q6–Q9, Q17 recorded (ADR-0000 addendum; §2 S0.6) |
| Licences recorded for anything touched | **done (nothing touched yet)** | no source retrieved, no dependency added, as of this update |
| Repo green under the gate | **n/a until S0.6** | `git ls-files --eol` all `i/lf` at every commit so far |
| Cold-start test (§2, SD15) | **open** | requires a cold session (fresh conversation, purged memory, §6 resume prompt verbatim) after S0.1–S0.6 |

## Blockers

- None. S0.1 proceeds on web retrieval (Q15); S0.4 and S0.6 are unblocked by the rulings.

## Standing escalations (awaiting Al)

- **Q3 — Repository LICENSE.** Public repo, no licence file. Al's decision, due before S5.4 and
  before any paper cites the repo. Not a blocker.
- **Q14 — Founding sweep.** Al will ferry one later (path + SHA-256); the executor then places it at
  `PROGRAM/SWEEP-<YYYY-MM>.md` under the SD11 banner after verifying identity. Until then, no
  novelty claim.
- **Q15 — Ferry on failure.** Any S0.1 source the executor cannot reach is recorded UNVERIFIED with
  the reason; Al ferries a copy (path + SHA-256).
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## NEXT-SINGLE-ACTION

**S0.1 — retrieve S-0001 from a primary source.** Named by Al (UNVERIFIED until read): the OpenAI
post "Sharing AI progress in mathematics" (6 October 2026) and the public GitHub repository
`openai/math`. Procedure: (1) fetch the post and the repository (GitHub API for metadata, licence
file, contents listing, and any Lean formalization catalogue; raw bytes of each retrieved document
saved outside the tree and SHA-256'd); (2) read the licence **from the repository**, and commit a
copy under `sources/` only if it permits redistribution — otherwise record URL + access date +
SHA-256 only; (3) derive the verification counts **from the repository itself** (contents listing
and Lean catalogue), not from press, recording the **unit** (manuscripts vs families) with every
number; (4) record S-0001 as VERIFIED with full citation, URL, access date, licence, digests, and the
repository commit SHA read — or UNVERIFIED with the exact failure; (5) transition CLAIM-0001
accordingly, with a note that the S5 contact-corpus location is now established (or not). Then
continue S0.1 through S-0002..S-0006 in the same way, S-0006 as an existence check only (SD8).
