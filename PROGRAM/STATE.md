# STATE

- **Updated:** 2026-10-12 (Session 4 — ADR-0001 ratified and amended; S0.6 ADR-0002 + scaffold +
  gate, close; UTC 2026-10-13 during the session)
- **Phase:** S0 — FOUNDATION AND CITATION VERIFICATION
- **Gate:** S0 open. Done: S0.1 (sources register), S0.2 (founding sweep placed), S0.3
  (constitution ratified), S0.4 (manifest ADR-0001, ratified Session 4), S0.5 (taxonomy 0.1.0
  ratified), S0.6 (ADR-0002: toolchains, Lean project with Mathlib, engine crate, gate, CI). Open:
  the deferred CLAIM-0006 examination (NEXT), then the cold-start test.
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
  design (sweep placed; its ten ranked sources not yet retrieved). S-0001..S-0008 VERIFIED; S-0004
  and S-0005 INGESTED (two PDFs under `sources/`, CC0 and CC BY-NC 4.0); S-0007 is RFC 8785
  (digest-only); S-0008 is the toolchain and dependency licence record at the ADR-0002 pins.
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

- **S0.1 continuation — examine CLAIM-0006 (ruled by Al, Session 3):** now the NEXT-SINGLE-ACTION
  (below). Retrieve and read, from primary sources, the ten sources ranked in
  `PROGRAM/SWEEP-2026-10.md` under "Verdict (3)". Each enters SOURCES.md only on retrieval, as
  UNVERIFIED-then-VERIFIED in the ordinary way (SD11); the sweep's descriptions of them are hints,
  not citations. Only after all ten are read does CLAIM-0006 move (to VERIFIED in the sweep's
  narrowed form, or to FALSIFIED by a counterexample). Before the Gate S0 cold-start test.

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
| CLAIM-0006 examined (deferred S0 action, Al's ruling Session 3) | **open — NEXT** | the ten sources ranked in `PROGRAM/SWEEP-2026-10.md` "Verdict (3)" are not yet retrieved; CLAIM-0006 UNKNOWN |
| Cold-start test (§2, SD15) | **open** | requires a cold session (fresh conversation, purged memory, §6 resume prompt verbatim) after the CLAIM-0006 examination |

## Blockers

- None. S0.4 and S0.6 are unblocked by the rulings.

## Standing escalations (awaiting Al)

- **Q3 — Repository LICENSE.** Public repo, no licence file. Al's decision, due before S5.4 and
  before any paper cites the repo. Not a blocker. The Session-3 and Session-4 prompts' Q3 slots
  both carried the placeholder text unfilled, so no ruling was delivered; Q3 remains deferred,
  unchanged.
- **Q14 — Founding sweep.** **Resolved in Session 3:** Al ferried the sweep by pasting its text
  (no file path was given); the pasted bytes, LF with one trailing newline, hashed to Al's stated
  digest exactly, and the file was placed. Nothing further is awaited.
- **Q15 — Ferry on failure.** Nothing failed outright in S0.1; the one unreachable page (the IAS
  project page, HTTP 403) is corroborative only. No ferry needed now.
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## NEXT-SINGLE-ACTION

**S0.1 continuation — examine CLAIM-0006 by retrieving the sweep's ten ranked sources.** Procedure
(SD11 throughout; the sweep is a map, its descriptions are hints):
(1) Open `PROGRAM/SWEEP-2026-10.md`, section "Verdict", item "(3) Ten sources SYNDEOM must cite,
ranked". For each of the ten, in rank order, retrieve the **primary** source (arXiv abstract page
and PDF by the arXiv id the sweep names; the GitHub repository for `leanprover/comparator`; the
agmai.org page already VERIFIED as S-0002 — re-read, do not re-cite the sweep) with `curl -A
"Mozilla/5.0 …"` or the fetch tool (reachability is per site — try the other tool before recording
a failure), record URL, access time (UTC), licence, and SHA-256 of the bytes read; store a copy
under `sources/` only if the licence permits (arXiv licences vary per paper — read each one).
(2) Add one SOURCES.md entry per source (numbered consecutively after S-0008), state VERIFIED only on a first-hand read;
anything unreachable stays UNVERIFIED with a note for Al (Q15: never guess, never complete a
citation from memory). Record for each what it *does* report (by error type? by length? level?)
in the source's own words with page or section references — that is the evidence CLAIM-0006 turns
on.
(3) Then, and only then, write CLAIM-0006's transition: VERIFIED in a narrowed form stated
precisely (as of the access date; what breakdowns prior work reports; what none reports), or
FALSIFIED if any source reports detection performance jointly by flaw class and proof length on
research-level AI-generated proofs. Add the scoped novelty statement to CLAIMS; §0.6's "no novelty
claim" lifts only at that transition and only in that scope.
(4) Run `bash scripts/check.sh` (must stay GREEN), commit `[S0][CLAIM-0006] retrieve the sweep's
ten ranked sources and examine the novelty claim`, push. Then set NEXT-SINGLE-ACTION = Gate S0
cold-start test, which must be launched in a fresh conversation with the §6 Session-N prompt
verbatim (SD15) — the executor cannot run it from this conversation.
