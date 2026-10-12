# STATE

- **Updated:** 2026-10-12 (Session 1 — genesis, close)
- **Phase:** S0 — FOUNDATION AND CITATION VERIFICATION
- **Gate:** S0 open. Nothing done-with-evidence yet except the bootstrap; the constitution is a
  **DRAFT awaiting Al's ratification** (S0.3), which is the blocker for everything else.
- **Default branch:** `main` — discovered from the clone of the empty remote (`HEAD ->
  refs/heads/main`) and the REST API (`default_branch: main`), not chosen (ADR-0000 D3). Remote:
  `origin` = `https://github.com/Quigles1337/Syndeom` (**public**; created by Al 2026-10-12;
  Actions enabled; no `LICENSE` file). **ORIENT note (SD7):** on this working copy, which was cloned
  from an empty remote, `git symbolic-ref refs/remotes/origin/HEAD` may fail until the first push
  and fetch; fallback `git remote show origin`; repair `git remote set-head origin -a`.
- **Build status:** no gate exists yet (`scripts/check.sh` lands at S0.6). Until then "green" means
  `git ls-files --eol` shows every tracked file as `i/lf` and the registers are coherent. Last
  verified: 2026-10-12 at the Session-1 commits (HANDOFF Session 1).
- **Toolchain:** nothing pinned yet. The siblings' pins and this machine's inventory are recorded in
  `PROGRAM/SIBLING-MAP-2026-10.md` §8 for the record; the pins are decided by ADR-0002 at S0.6
  after Al rules on Q6–Q8.
- **Constitution:** `PROGRAM/SYNDEOM-PROGRAM.md` v1.0 **DRAFT**, executor-authored from the genesis
  prompt (ADR-0000 D6). Free to edit until ratified; ADR-only afterwards.

## Memory policy

SD15 adopts FRIEDMAN ADR-0003 D1–D3 and the conversation-context corollary from day 0. The
executor's memory store holds, for SYNDEOM, the repo location and machine hazards only (written at
the close of Session 1, which is not an exam session). Q13 asks Al to confirm the corollary's
ratification status, because the siblings' records differ.

## Gate S0 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| S0.1 CLAIM-0001..0005 resolved to VERIFIED or honestly UNVERIFIED with consequences | **open** | seeded UNKNOWN; S-0001..S-0005 UNVERIFIED (CLAIMS.md, SOURCES.md) |
| S0.2 Founding sweep placed with the SD11 banner (if Al provides one) | **open — Q14** | — |
| S0.3 Constitution ratified by Al; DRAFT marker removed | **open — Session 2** | draft at this commit; question list below |
| S0.4 Manifest schema ADR (`syndeom-manifest/1` extending `friedman-manifest/1`) | **open** | FRIEDMAN ADR-0001 D1–D7 and crate facts recorded in SIBLING-MAP §7.1; semantics question Q10, fields Q11 |
| S0.5 Flaw taxonomy v0.1 ratified | **open — Session 2** | `PROGRAM/FLAWS.md` 0.1.0 DRAFT, 13 entries |
| S0.6 Scaffold (`lean/`, `engine/`, `lab/`, `corpora/`, `groundtruth/`, `contact/`, `analysis/`, gate, CI) | **open** | blocked on Q6–Q9, Q17 |
| Licences recorded for anything touched | **done (nothing touched)** | no source retrieved, no dependency added |
| Repo green under the gate | **n/a until S0.6** | `git ls-files --eol` all `i/lf` at the Session-1 commits |
| Cold-start test (§2, SD15) | **open** | requires a cold session after S0.1–S0.6 |

## Blockers

- **Al's ratification of the draft constitution and rulings on Q1–Q17 (Session 2).** S0.1 could
  proceed without them (it needs only web access and SD11), but S0.4 and S0.6 cannot, and the
  constitution's own rule set should be ratified before it governs more than one session.

## Standing escalations (awaiting Al) — the values the executor needed and could not read from source

Each item states what was chosen for the draft (if anything), where the choice came from, and what
Al is asked to do. Nothing below was invented: every default is a sibling precedent or a discovered
value, and every undecidable item is left open.

- **Q1 — Local path.** `C:\Users\LEET\Syndeom` (remote name + the one-repo-per-directory
  convention; KOBER ADR-0000 precedent). Confirm, or name the path and the executor moves it.
- **Q2 — Default branch.** `main`, discovered from the remote (unborn HEAD) and the REST API. The
  siblings differ (KOBER `master`, FRIEDMAN `main`). Confirm; the first push fixed it on GitHub.
- **Q3 — Visibility and LICENSE.** The repo is public (Al's creation setting) and has no `LICENSE`
  file; neither sibling has one (FRIEDMAN carries it as an open escalation). Choosing one is
  public-facing/legal — Al's. Not blocking S0–S4.
- **Q4 — Rule prefix.** Draft uses `SD1…SD17` because `S` is taken by phases (S0–S5) and sources
  (`S-NNNN`). Siblings: `E` (KOBER, "Epistemic"), `F` (FRIEDMAN, program initial). Confirm `SD`, or
  choose `E` or another prefix; the executor renames throughout.
- **Q5 — Phase/commit tag.** `[S<phase>]` (FRIEDMAN's program-initial style; KOBER uses `[P…]`).
  Confirm.
- **Q6 — Rust toolchain pin.** Both siblings pin `channel = "1.91.0"`, edition 2024 (installed
  here). Components differ: KOBER `clippy`; FRIEDMAN `clippy, rustfmt` with `cargo fmt --check` in
  the gate. Draft assumes FRIEDMAN's superset. Confirm, or name the pin.
- **Q7 — Rust workspace root.** KOBER: repo root `/Cargo.toml`; FRIEDMAN: `engine/Cargo.toml` with
  the stated reason that the program root carries no build files. Draft assumes FRIEDMAN's. Confirm.
- **Q8 — Lean toolchain and Mathlib.** KOBER pins `leanprover/lean4:v4.33.1`, **no Mathlib**
  (ADR-0001: its targets are finite arithmetic). SYNDEOM's S1.2 needs mathlib formalizations and
  S2.1(ii) generates Lean variants that look like real mathematics, so Mathlib is a real decision —
  and Mathlib pins its own `lean-toolchain`, so "same pin as KOBER" and "Mathlib" may conflict.
  Options for Al: (a) no Mathlib, v4.33.1, mathlib pairs treated as external read-only references
  (not built here); (b) Mathlib at a pinned commit, Lean version following Mathlib's pin, decided by
  ADR after `elan` and the Mathlib pin are read. Machine today: elan 4.2.3, v4.33.1 installed,
  elan's `stable` resolves to v4.35.0 (not installed).
- **Q9 — CI action versions.** KOBER `actions/checkout@v4` + `cache@v4`; FRIEDMAN `@v5` (bumped on
  Al's instruction). Draft assumes v5. Confirm. (Both: never a required status check.)
- **Q10 — Manifest compatibility semantics.** FRIEDMAN's crate uses `deny_unknown_fields` and
  rejects any `manifest_schema` other than `friedman-manifest/1`, so a FRIEDMAN consumer cannot
  parse a SYNDEOM manifest that adds proof-specific fields. "Compatible" can mean (a) **structural**:
  same field semantics, same hashing (RFC 8785 JCS + SHA-256), same `corpus_id` derivation, no
  timestamps, new schema string `syndeom-manifest/1`, extension fields under namespaced objects,
  handshake per FRIEDMAN D7 — the draft's reading; or (b) **wire**: FRIEDMAN must be able to read
  SYNDEOM manifests, which needs a schema bump on FRIEDMAN's side (an ADR there — Al's). Rule.
- **Q11 — Schema fields the S0.4 ADR must decide** (Al may pre-empt any): `origin` values for
  synthetic / ground-truth (S1) / contact (S5) artifacts; the artifact format replacing
  `friedman-tokens/1` (informal text file, Lean source file, or a pair; segmentation into steps for
  `location`); `ground_truth` extension (`statement_status`, `proof_status`, `flaws[]` with
  `class`, `location`, `severity`, `lean_catchable`, `taxonomy_version`); commanded/achieved
  vocabulary for proofs (length in tokens/steps/lemmas, flaws per class, padding ratio); and the
  SD6 determinism clause for any LLM-assisted generator (model identifier + version, prompt hash,
  sampling parameters, content-hash pinning).
- **Q12 — Taxonomy conventions.** IDs `FLAW-NNNN`; version `0.1.0` with the minor/major rule in
  FLAWS.md; Lean-catchable vocabulary `YES-KERNEL / YES-TOOLING / NO / N/A`; severity
  `cosmetic / repairable / gap / fatal`. Confirm or amend; then ratify the thirteen entries (S0.5).
- **Q13 — Memory-discipline corollary.** KOBER STATE records the conversation-context corollary
  (plus its extension to architect-written prompts) as *ratified by the architects* (2026-08-25);
  FRIEDMAN ADR-0003 and STATE record it as *unratified, awaiting Al*. SD15 treats it as binding.
  Confirm for SYNDEOM, and say whether SYNDEOM writes its own ADR or cites FRIEDMAN ADR-0003.
- **Q14 — Founding sweep.** Does a prior-art sweep for SYNDEOM exist? If yes: ferry it (path +
  size or SHA-256) and the executor places it at `PROGRAM/SWEEP-<YYYY-MM>.md` under the SD11
  banner. If no: §0.6 holds (no novelty claim); CLAIM-0006 stays UNKNOWN.
- **Q15 — S0.1 retrieval channel.** This harness exposes web search/fetch tools; whether they reach
  the five named sources is untested. If Al prefers to ferry primary copies (path + SHA-256), say so;
  otherwise the executor attempts retrieval and records UNVERIFIED on any failure, never a guess.
- **Q16 — Commit trailer.** Session-1 commits carry `Co-Authored-By: Claude Fable 5.1
  <noreply@anthropic.com>` (harness standard; FRIEDMAN's bootstrap commit had the same shape, later
  commits in both repos none). Keep or drop for future commits.
- **Q17 — Layout names.** Proposed `groundtruth/`, `contact/`, `analysis/`, `sources/` alongside the
  sibling-standard `engine/ lean/ lab/ corpora/ scripts/`. Confirm or rename before S0.6.

Standing (not questions): human-referee participants are Al's decision whenever S4.3 arrives
(SD13); any statement about a specific real manuscript is escalated (SD3); the repo name is
`Syndeom` (mixed case) while the siblings' repo names are upper-case — noted, no action requested.

## NEXT-SINGLE-ACTION

**Session 2 — ratification.** Read Al's rulings on Q1–Q17 (in the Session-2 prompt or in a commit
Al names). Record each where it belongs: Q1–Q5, Q13, Q16, Q17 → ADR-0000 D7 addendum; Q6–Q9 → noted
for ADR-0002 (written at S0.6); Q10–Q11 → noted for ADR-0001 (written at S0.4); Q12 → FLAWS.md
header, taxonomy ratified; Q14 → sweep placed if provided; Q15 → noted. Apply any renames the
rulings require across the constitution and registers. Remove the DRAFT markers (header, footer,
README, FLAWS.md header). Commit `[S0][ADR-0000] ratify the constitution and record Al's rulings`.
Then set NEXT-SINGLE-ACTION = **S0.1: retrieve S-0001 (the 6 October 2026 OpenAI release) from a
primary source**, recording URL, access date, licence, SHA-256, and the verification counts as
stated there — or UNVERIFIED with the reason.
