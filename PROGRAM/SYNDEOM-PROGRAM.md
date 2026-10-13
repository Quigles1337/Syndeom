# SYNDEOM — Autonomous Engineering Program v1.0

**Program:** Calibration infrastructure for AI-generated mathematical proofs. A generator of
proof-shaped text — informal and Lean 4 formal variants — with dialable, labeled flaws and ground
truth by construction; a versioned flaw taxonomy; a detector battery (automated triage detectors plus
a blinded human-referee protocol with decoys); and ROC/AUC calibration per flaw class as a function
of proof length and severity. The deliverables are the **fooling length** at which each detector
falls to chance, and a **cost model in expert-minutes per true positive** — because the verification
bill is the point.
**Named for:** συνδέω (*syndéō*), "to bind together" — FRIEDMAN's generator-and-calibration
discipline bound to KOBER's formal-methods rigor, pointed at a new target.
**Sibling programs:** KOBER and FRIEDMAN. The three constitutions share a deliberate spine — identical
section numbering, identical `PROGRAM/` schema, identical loop. Every difference is load-bearing and
recorded in `PROGRAM/SIBLING-MAP-2026-10.md`. Interface with the siblings via manifest schema only;
never a code dependency.
**Engineer of record:** Al (Quigles1337). **Executor:** Claude Code, running the loop in §4.
**This file is the constitution.** Read at the start of every session; amended only via ADR (§3.5).
**Ratification:** authored by the executor in Session 1 (2026-10-12) from Al's genesis prompt;
**ratified by Al in Session 2 (2026-10-12)**, with the rulings and the pre-ratification edits recorded
in the ADR-0000 addendum. From that commit onward this file changes only via ADR.
**Amendments:** ADR-0004 (2026-10-13, Session 7; Al's ruling of 2026-10-13) — §2 S3.2, the fooling
length redefined as equivalence to chance with a design-derived N_min.

---

## §0 Charter

1. **The problem.** AI systems now emit mathematical proofs faster than humans can check them, and the
   output arrives without calibration data: nobody can say what the error rate of a given triage
   method is at a given proof length. SYNDEOM produces that calibration.
2. **Scope.** "Keeping up" decomposes into three problems: *truth* (is it correct?), *triage* (which
   results deserve scarce expert hours?), and *understanding* (can a human learn from it?). SYNDEOM
   addresses the first two, for informal and formal proofs. **It never addresses understanding.**
   This is a scope boundary and a hard invariant (SD17); any session drifting toward explanation
   quality, pedagogy, or "insight" is off-mission.
3. **SYNDEOM never issues verdicts on any real contested proof.** Its outputs about a specific
   manuscript are calibrated triage scores with uncertainty, relative to a frozen battery version
   and named flaw classes. The words "proven" and "disproven" never appear in output about a specific
   real manuscript (SD3).
4. **"Proven" is a reserved word.** It is used only for sorry-free Lean 4 proofs that have also passed
   statement-fidelity review (SD2). Everything else is "claimed" or "sketched".
5. **Pre-declared success states, equal in rank:** (a) the calibrated battery with its fooling-length
   tables and cost model; (b) the negative result — *at proof length L and flaw class C, no detector
   in the battery beats chance* — stated with precision and error bars. Honest limits are
   deliverables (KOBER and FRIEDMAN kinship).
6. **Novelty claims are scoped or absent.** No founding prior-art sweep exists for SYNDEOM at genesis.
   Until one is ferried by Al and recorded under the SD11 banner (map, not evidence), SYNDEOM makes
   no novelty claim at all. When one exists, every novelty statement cites its date and scope and
   remains falsifiable by a single counterexample.
7. **Detector priority is declared in advance.** Surface-statistical detectors (length, hedge-word
   counts, perplexity-style scores) are *expected* to be fooled by LLM output. Structural and semantic
   detectors — step-dependency extraction, hypothesis checks at theorem applications, lemma-usage
   consistency, citation existence, autoformalization attempts, statement-fidelity comparison — are
   the program's priority. Surface detectors are retained only as documented-weak controls.
8. The executor does not persist between sessions. **The repo is the only memory.** Every session
   leaves the repo resumable by a cold-started successor using the repo alone.

## §1 Epistemic Constitution (non-negotiable rules)

Rule IDs carry the prefix `SD` (SynDeom) because `S` alone is taken by phase names (S0–S5) and by
source IDs (`S-NNNN`). The siblings use `E` (KOBER) and `F` (FRIEDMAN). Ratified (ADR-0000 addendum, Q4).

- **SD1 — Ground truth is sacred; every artifact carries a manifest.** Every generated artifact ships
  with a manifest carrying: generator ID + version, full config, seed, taxonomy version, and ground
  truth — `correct` (true/false), flaw class(es) per `PROGRAM/FLAWS.md`, location, severity, and
  whether a proof checker can catch it — plus content hash. Labels derive from the generative
  process, never from human judgment. An artifact without a manifest does not exist to this program.
- **SD2 — "Proven" discipline.** A claim or artifact is PROVEN only when (i) the Lean proof is
  sorry-free and admit-free on the default branch, (ii) `#print axioms` shows at most the standard
  three (`propext`, `Classical.choice`, `Quot.sound`) and the output is pasted into the register
  entry, and (iii) the formal statement has been diffed line by line against the informal statement
  and the check recorded. Proving a weakened statement and calling it the claim is the cardinal
  failure mode of formalization — and it is also FLAW-0008, so a generated formal variant whose
  statement drifted is labeled as such or it is a mislabeled artifact (SD1 violation).
- **SD3 — Verdict discipline.** Reports state where an artifact falls relative to the calibrated
  battery — per-detector scores, aggregate triage score, uncertainty interval, battery version, flaw
  classes considered — and stop there. No "proven", "disproven", "correct", or "wrong" about a specific
  real manuscript. The reader draws conclusions; the instrument draws curves.
- **SD4 — No single-detector verdicts; the Dembski rule.** No detector ships, and no detector result
  is cited, without a measured false-positive rate against correct proofs of matched length from the
  calibrated generator and the ground-truth corpus. Conclusions require joint detector evidence, each
  reported inside its measured ROC context with bootstrap confidence intervals. Where a cell of the
  calibration grid is small-N, the report says what the cell cannot distinguish (KOBER E6).
- **SD5 — Structured nulls.** Detectors are evaluated against *correct* proofs of matched length and
  style, against *padded* correct proofs (FLAW-0013), and against each other flaw class — never only
  against trivially broken text. Beating gibberish is beating a strawman. The §0.7 expectation that
  surface detectors are fooled is a pre-registered prediction, to be confirmed or refuted by S3.
- **SD6 — Determinism.** Every MEASURED or REPRODUCED result records engine commit SHA, seed(s),
  config hash, taxonomy version, battery version, and uncertainty. A result not regenerable from a
  fresh clone plus one script is not a result. Where generation is LLM-assisted and therefore not
  bit-reproducible, the ADR deciding that generator must state how determinism is recovered
  (content-hash pinning of outputs, model identifier and version, prompt hash, sampling parameters);
  an artifact whose bytes cannot be re-verified by hash is not a counted artifact.
- **SD7 — Source of truth over assumption (Al's standing rule).** Never assume or invent repo paths,
  remotes, branch names, toolchain versions, file layouts, schema fields, env vars, CLI flags, or
  config values. Read them from the repo and tools: `git`, `lean-toolchain`, `lakefile.*`,
  `Cargo.toml`, `rust-toolchain.toml`, CI configs, `elan`/`lake`/`rustc`/`cargo` output. If a value is
  not pinned anywhere, stop and ask Al; do not choose.
- **SD8 — Pre-registration, freeze, and contact quarantine.** The battery is frozen (tagged commit,
  SHA in CLAIMS) before any S4 validation data and before any S5 contact data is analyzed with it.
  Real AI-generated corpora live under `contact/`, are read-only test subjects for the frozen battery,
  and **never inform design**: any ADR citing contact-corpus behavior as design rationale pre-freeze
  is a leakage violation. **Contact results never modify the battery in place:** a change creates a
  new battery version; results from the old version stand as recorded under that version. RETRACTED
  is for errors with a trail, never for results that a newer battery would have scored differently.
- **SD9 — Green default branch.** The default branch always passes the gate (`bash scripts/check.sh`
  once S0.6 lands: `lake build` + `cargo test` + `cargo clippy -D warnings` + `cargo fmt --check`;
  until then, "green" means every tracked text file is LF and the registers are coherent). WIP lives on
  feature branches. Discover the default branch from git; do not assume it.
- **SD10 — Two-tier code.** `lab/` (Python, exploratory) has zero result authority. `engine/` (Rust)
  and `lean/` (Lean 4) are canonical: only their output enters CLAIMS as MEASURED, REPRODUCED, or
  PROVEN. Promotion from lab to engine is a normal, expected step.
- **SD11 — No fabricated scholarship.** Nothing enters SOURCES.md or CLAIMS.md from executor memory,
  from a sweep report, or from the genesis prompt. A source is VERIFIED only after it is retrieved
  this-session and read from a primary source, with URL, access date, and SHA-256 of any stored
  copy; otherwise it is UNVERIFIED with a note for Al. If the session has no web access, everything
  stays UNVERIFIED. A citation that cannot be retrieved is never invented, completed, or "recalled".
  Any sweep report carries the banner *map, not evidence*.
- **SD12 — Ground-truth corpus integrity.** The S1 corpus holds only real proofs with documented
  status: published proofs with recorded errata or retractions, and theorems with both an informal
  statement and a mathlib formalization. Every item records its source, licence, and the document
  that establishes its status. Nothing synthetic is ever labeled real; a real item's ground truth is
  `documented-by-source` or `formalized-in-mathlib`, never `by-construction`.
- **SD13 — The human-referee protocol is designed, not run.** The blinded protocol with decoys is a
  pre-registered design artifact (S3.5). It is executed only after Al decides on human participants
  (§4.6). No session recruits, contacts, or times a human referee on its own authority.
- **SD14 — Interface, not dependency.** SYNDEOM's manifest schema extends FRIEDMAN's
  `friedman-manifest/1` (FRIEDMAN `PROGRAM/DECISIONS/ADR-0001.md`) by versioned extension — read it,
  extend it, never fork it. Neither sibling's code is imported; neither program blocks the other.
  Schema evolution is a new version string; consumers pin the string they understand.
- **SD15 — Executor memory discipline.** Executor memories outside the repo hold environment and
  tooling hazards and the repo *location* only — never program state (phase, gate status, CLAIM or
  FLAW statuses, file contents, commit SHAs as state markers, next actions, escalations). In any
  session attempting a gate's cold-start test, every recalled memory is reproduced verbatim as the
  first entry of the HANDOFF block before any other action; the exam counts only if no recalled
  memory holds program state; no memory writes during an exam session. A session running in the
  conversation context that built the state under test is not cold, regardless of memory, and this
  extends to launching prompts: a cold session is launched with the §6 resume prompt verbatim and
  nothing that carries program state. (FRIEDMAN ADR-0003 D1–D3, cited and not duplicated; the
  conversation-context corollary is ratified for SYNDEOM by Al, 2026-10-12 — ADR-0000 addendum, Q13.)
- **SD16 — Licences and credit.** Published proofs, errata, mathlib, and AI-generated corpora have
  owners and licences. Record licence terms in SOURCES.md before ingesting; store derived data with
  attribution; commit a stored copy only when its licence permits redistribution (this repo is
  public). Ambiguity → escalate to Al; do not ingest.
- **SD17 — Scope boundary.** SYNDEOM measures truth and triage, never understanding. No detector,
  score, or report assesses explanatory quality, readability for learning, or insight. Padding
  (FLAW-0013) is measured as *verification cost*, not as pedagogical harm.

## §2 Phases and Gates

Each phase ends at a gate. A gate passes only when every exit criterion has linked evidence in
STATE.md and the **cold-start test** passes: re-derive program state from the repo alone via §4.1 in
one pass without guessing, in a session that is cold under SD15. Unreconstructable context fails the
gate regardless of working code.

### Phase S0 — FOUNDATION AND CITATION VERIFICATION
Verify the situation, ratify the constitution, define the interfaces, scaffold the machine.

- S0.1 **Sources register for every factual claim about the AI-mathematics situation.** Seeded at
  genesis, all **UNVERIFIED** program memory from Al's prompt, each with a CLAIM entry at UNKNOWN:
  S-0001 the 6 October 2026 OpenAI manuscript release and its verification counts; S-0002 the AGMAI
  guidelines; S-0003 the AHM statement; S-0004 the Fields Medalist letter; S-0005 the Leiden
  Declaration; S-0006 (added at ratification) a reported correction or retraction of some
  manuscripts in the 6 October release shortly after publication — a contact-phase item under SD8:
  its existence is verified in S0, its content is read only in S5, and it is never S1 ground truth.
  A claim is VERIFIED only when read from a primary source (SD11). The exact public location of the
  S5 target corpus is established here and nowhere else.
- S0.2 **Founding sweep, if Al provides one:** placed at `PROGRAM/SWEEP-<YYYY-MM>.md` with the SD11
  banner, identity verified against Al's stated digest before the move. The executor never
  reconstructs one from memory. Absent a sweep, §0.6 holds: no novelty claim.
- S0.3 **Ratify the constitution.** Al rules on the genesis question list; the rulings are recorded
  in the ADR-0000 addendum (toolchain and schema rulings bind their own later ADRs); the DRAFT
  marker is removed. Completed in Session 2, 2026-10-12.
- S0.4 **Manifest schema ADR (ADR-0001):** `syndeom-manifest/1` as a versioned extension of
  `friedman-manifest/1`. Compatibility is **structural** (ruling Q10): same field semantics, hashing
  per RFC 8785 JCS + SHA-256, `corpus_id` as a pure function of generator/config/seed, no wall-clock
  timestamps, commanded/achieved/fidelity; handshake per FRIEDMAN D7; SYNDEOM never requires a
  FRIEDMAN-side change (if FRIEDMAN ever consumes SYNDEOM corpora, that is a FRIEDMAN ADR). Read
  FRIEDMAN ADR-0001 D1–D7 and its canonical crate at a pinned commit. The ADR fills in field names
  under the ruled constraints (Q11, ADR-0000 addendum): `origin` ∈ {synthetic, ground-truth,
  contact}; derivation ∈ {by-construction, documented-by-source, formalized-in-mathlib} (SD12);
  artifact kind ∈ {informal, formal, pair} sharing one artifact id — informal is UTF-8 Markdown with
  LaTeX math and generator-numbered steps (location = step index + character span), formal is one
  Lean file (location = declaration:line); ground truth carries `statement_status` with how it was
  established, `proof_status`, `flaws[]` {class, sub_variant, location, severity, lean_catchable,
  dials}, and `taxonomy_version`; commanded/achieved covers length in tokens, steps, and lemmas,
  flaws per class, and `padding_ratio`; and for any LLM-assisted generator the SD6 clause records
  model id and version, prompt hash, sampling parameters, and output content hash — the artifact is
  uncounted unless the hash re-verifies.
- S0.5 **Flaw taxonomy v0.1 ratified.** `PROGRAM/FLAWS.md` holds fourteen entries (thirteen seeded at
  genesis, FLAW-0014 added at ratification), each with definition, Lean-catchable status, generator
  recipe, and example. Ratified by Al in Session 2 (Q12); the version is pinned and manifests
  reference it from S2 on.
- S0.6 **Scaffold (ADR-0002):** `lean/` with **Mathlib at a pinned commit; the Lean version follows
  Mathlib's `lean-toolchain`** (ruling Q8: the S1.2 pairs, the FLAW-0008 example, and the tactics
  named in FLAW-0001 all require Mathlib, so a no-Mathlib pin would leave the taxonomy's own golden
  fixtures unbuildable; the pin is written after reading `elan` and Mathlib's pin, and diverges from
  KOBER's — load-bearing, recorded in SIBLING-MAP; the gate uses Mathlib's cache and builds only this
  project's modules); `engine/Cargo.toml` as the Rust workspace root (Q7), pinned from the siblings'
  `rust-toolchain.toml` as re-read at S0.6 — expected channel 1.91.0, edition 2024, clippy + rustfmt,
  with `cargo fmt --check` in the gate (Q6); `lab/`, `corpora/`, `groundtruth/`, `contact/` (empty;
  SD8 rules posted in a README), `analysis/`, `sources/` (Q17); `scripts/check.sh`; CI if the
  environment supports it (discover, don't assume) on `actions/*@v5` (Q9), never a required status
  check.

**Gate S0:** CLAIM-0001..0005 resolved to VERIFIED or honestly UNVERIFIED with consequences stated;
constitution ratified; manifest ADR merged; taxonomy v0.1 ratified; licences recorded for anything
touched; repo green under the gate; cold-start test.

### Phase S1 — GROUND-TRUTH CORPUS (real proofs as unit tests)
Real proofs with known status. The battery will meet these before it meets any AI-generated proof.

- S1.1 **Errata and retractions.** Published proofs with documented errors: for each item, the
  original, the erratum or retraction notice, the flaw's location, and its class per FLAWS.md
  (mapping argued in writing; label derivation `documented-by-source`). Licence per SD16.
- S1.2 **Informal–formal pairs.** Theorems with both an informal statement and a mathlib
  formalization, mathlib commit pinned, as statement-fidelity ground truth (positive pairs). Drifted
  pairs are synthetic, belong in `corpora/`, and are never mixed in here (SD12).
- S1.3 **Manifests for real items** under the `origin` extension decided in S0.4; nothing synthetic
  labeled real.
- S1.4 **Coverage and power.** Per flaw class: how many real items exist, at what proof lengths; which
  classes have zero real examples. Zero-coverage classes are a finding, and S4 cannot validate them.

**Gate S1:** `groundtruth/` manifested; every item sourced, licensed, and status-documented; class
coverage table with the power statement; repo green; cold-start test.

### Phase S2 — GENERATOR
Proof-shaped text with dialable, labeled flaws.

- S2.1 **Modules behind one shared interface:** (i) informal proof generator; (ii) Lean 4 formal
  variant generator; (iii) the flaw injector — every FLAWS.md recipe as a dial (presence, location,
  severity); (iv) the padding module (FLAW-0013). **Standing ruling (Al, Session 2; ADR at S2):** the
  first generator family is deterministic — seeded Rust templates and grammars. Any LLM-assisted
  generator is a separate family requiring its own ADR under SD6 and is excluded from
  `battery-freeze-v1` unless that ADR exists; reasons: determinism, and the leakage risk that an LLM
  generator was trained on the contact corpus (SD8).
- S2.2 **Commanded and achieved statistics:** proof length (tokens, steps, lemmas), flaws per class,
  severity, padding ratio. Fidelity verdict per artifact (FRIEDMAN F5 lineage): a flawed artifact
  counts only when the injected flaw is confirmed present — for Lean-catchable flaws, the kernel or
  tooling rejects the formal variant exactly as predicted; for others, a recorded check.
- S2.3 **Manifests and determinism** for every artifact (SD1, SD6); `corpus_id` invariant tested.
- S2.4 **Verification:** property tests; golden tests against the hand-written FLAWS.md examples
  (which become fixtures); `cargo clippy` at deny-warnings; `cargo audit` (plus the
  `rust-security-audit` skill if available in the environment).

**Gate S2:** fidelity sweep report (commanded vs achieved flaw presence across the parameter grid,
tolerances stated); goldens green; lint and audit clean; cold-start test.

### Phase S3 — DETECTOR BATTERY, CALIBRATION, FREEZE
The battery, the curves, the fooling length, the cost model, the freeze.

- S3.1 **Battery, minimum set.** Priority (structural/semantic): autoformalization attempt;
  step-dependency extraction; lemma-usage consistency; hypothesis check at each theorem application;
  citation existence check; statement-fidelity comparison (informal claim vs formal statement);
  axiom audit (`#print axioms`). Controls (surface, documented-weak): length and hedge-word
  statistics, perplexity-style scores — kept to demonstrate that they are fooled, per §0.7.
- S3.2 **ROC/AUC** over detector × flaw class × proof length × severity, bootstrap CIs on every AUC.
  **"Fooled" is a claim of equivalence to chance, not a failure to reject it** (amended by ADR-0004,
  2026-10-13). The equivalence band is AUC ∈ [0.45, 0.55]; a detector is fooled in a cell when the
  95% bootstrap CI of its AUC lies entirely inside the band — two one-sided tests at α = 0.025 each,
  stricter than the conventional 90%-interval TOST. The **fooling length** per detector per flaw
  class is the smallest proof length L at which a cell with N ≥ N_min items satisfies that criterion.
  A cell whose CI merely contains 0.5 is reported as **"not distinguished from chance"**; a cell with
  N < N_min is reported as **underpowered** — never as fooled. N_min is design-derived, not chosen:
  the smallest balanced cell (flawed versus matched correct) at which the test has 90% power to
  declare equivalence when the true AUC is exactly 0.5, derived from the null-hypothesis variance of
  the Mann–Whitney statistic by the method ADR-0004 fixes; the number is computed in the S3
  calibration ADR *before any curve is computed* and confirmed there by simulation against the
  bootstrap procedure actually used. Items derived from one seed theorem are not independent: the S3
  calibration ADR states how effective N is computed (for example a cluster bootstrap by seed
  theorem) before any curve is computed. Cells too expensive to fill to N_min — human-referee cells
  under SD13, detectors needing a per-item Lean build across the full grid — are reported as
  underpowered; that is the intended behaviour, not a defect. Proof length is swept log-spaced over
  the regimes the S1 corpus actually exhibits.
- S3.3 **Leave-one-out.** Leave-one-generator-out and leave-one-flaw-class-out evaluation, to measure
  whether detectors detect flaws or fingerprint generators.
- S3.4 **Cost model:** expert-minutes per true positive, from a pre-declared per-item review-time
  model with its assumptions stated; replaced by measured referee times only if and when SD13 lifts.
- S3.5 **Human-referee protocol — design only.** Blinded; decoys in pre-declared proportions
  (synthetic correct, synthetic flawed, real correct, real flawed); pre-registered analysis plan;
  timing capture. Written, versioned, unexecuted (SD13).
- S3.6 **Freeze:** tag `battery-freeze-v1`, SHA into CLAIMS. The frozen protocol — detectors,
  thresholds, analysis plan, referee design — is the pre-registration the position paper cites.

**Gate S3:** calibration tables and fooling-length table with CIs regenerable from a fresh clone via
one script; leave-one-out results reported; cost model stated; referee protocol complete and
unexecuted; freeze tag exists; cold-start test.

### Phase S4 — BLIND VALIDATION ON GROUND TRUTH (KOBER's Linear B step)
The frozen battery sees real human proofs before it sees any AI-generated proof.

- S4.1 **Run the frozen battery on the S1 corpus.** Success criterion stated before running: per flaw
  class, does the battery recover the documented errata (recall), at what false-positive rate on
  correct proofs and mathlib-formalized pairs, at the proof lengths present?
- S4.2 **Validation report:** what the battery detects at this corpus and these lengths, what it
  provably cannot (coverage from S1.4), any post-freeze changes with their RETRACTED trail.
- S4.3 **Referee protocol:** executed here, on S1 items only and per the frozen design, if Al has
  decided on participants; otherwise it remains designed.

**Gate S4:** pre-registration honored end-to-end; validation report with power analysis; battery
sensitivity/specificity characterized per class; cold-start test.

### Phase S5 — CONTACT
Post-freeze runs on real AI-generated corpora. The quarantine lifts here and only here.

- S5.1 **Ingest** real AI-generated corpora (e.g., the 6 October 2026 public release — exact
  location VERIFIED in S0.1, never assumed) under `contact/`, with licence and provenance (SD16);
  read-only. Contact corpora carry their own Mathlib pins: ingest records each corpus's pin, and
  per-corpus toolchains may be needed (noted at ratification, Q8; solved at ingest, not before).
- S5.2 **Outputs:** per manuscript, calibrated triage scores with uncertainty, relative to the frozen
  battery version and named flaw classes (SD3). Never "proven/disproven".
- S5.3 **Battery immutability:** contact results never modify the battery in place (SD8); a new
  version is a new freeze tag, and the old results stand.
- S5.4 **Publication packaging.** **Escalate to Al before anything becomes public** (§4.6). If the
  fooling lengths came in trivially short, the paper is the §0.5(b) negative result.

**Gate S5:** contact report complete; every number seed-linked to the register; reproduction script
clean from a fresh clone; Al sign-off.

## §3 State Architecture — the repo as memory

Identical schema to KOBER and FRIEDMAN, under `PROGRAM/` at repo root, plus one program-specific
register (3.7).

- **3.1 `PROGRAM/SYNDEOM-PROGRAM.md`** — this file. Amended only via ADR (after ratification).
- **3.2 `PROGRAM/STATE.md`** — current phase; gate checklist with evidence links; blockers; standing
  escalations; exactly one **NEXT-SINGLE-ACTION**.
- **3.3 `PROGRAM/CLAIMS.md`** — append-only register. Entry schema:
  `ID | informal statement | status | evidence | manifest/taxonomy/battery deps | axiom print and
  fidelity check (if PROVEN) | transitions log`.
  Statuses: `UNKNOWN → VERIFIED (primary source read) / PROVEN (SD2: kernel-checked, standard axioms,
  fidelity-reviewed) / REPRODUCED (published result recovered by our engine) / MEASURED (our own
  result: engine SHA + seed + config hash + taxonomy and battery versions + CI) / CONDITIONAL /
  FALSIFIED / RETRACTED`. Downgrades are always permitted and never shameful.
- **3.4 `PROGRAM/SOURCES.md`** — citations and data provenance per SD11/SD16. Entry states:
  `UNVERIFIED` (named, not yet retrieved) / `VERIFIED` (primary source read this-session) /
  `SURVEYED` (existence, coverage, licence noted; nothing ingested) / `INGESTED` (data in the repo
  under the recorded licence, with audit trail). Full citation, URL, access date, licence, SHA-256 of
  stored copies; copies committed only when the licence permits.
- **3.5 `PROGRAM/DECISIONS/ADR-NNNN.md`** — one decision per file: context, options, decision,
  rationale, consequences. Toolchain pins, schema, taxonomy versions, battery design, freezes,
  constitution amendments.
- **3.6 `PROGRAM/HANDOFF.md`** — append-only session log; schema in §4.5.
- **3.7 `PROGRAM/FLAWS.md`** — the **versioned flaw taxonomy register.** Entries `FLAW-NNNN`, each with
  definition, Lean-catchable status, generator recipe, example, candidate detectors. Append-only;
  additions bump the minor version, semantic changes to an existing entry bump the major version;
  manifests pin `taxonomy_version`. Ratified at S0.5; changed only by ADR thereafter.
- **3.8 Layout (ratified, Q17; ADR-0002 records the build layout inside it):** `engine/` (Rust, canonical),
  `lean/` (Lean 4, canonical), `lab/` (Python, no result authority), `corpora/` (manifested synthetic
  artifacts), `groundtruth/` (S1 real items), `contact/` (S5 real AI-generated corpora; SD8 rules),
  `analysis/` (design documents: referee protocol, calibration reports), `sources/` (stored copies
  whose licence permits; else digest-only), `scripts/` (the gate). Amendable by ADR.
- **3.9 Commit convention:** `[S<phase>][<CLAIM-ID or ADR-ID or FLAW-ID or INFRA>] imperative summary`.

## §4 The Metacognitive Loop (session protocol)

Run every session, in order. Do not skip ORIENT because you "remember" — you don't.

### 4.1 ORIENT — reconstruct reality from the repo
```
git rev-parse --show-toplevel && git remote -v      # confirm which repo this is
git status && git log --oneline -20                  # confirm actual recent history
git symbolic-ref refs/remotes/origin/HEAD            # discover default branch (SD7)
```
If the last command fails ("not a symbolic ref"), the fallback is `git remote show origin`; repair
with `git remote set-head origin -a`. Do not guess.
Read, in order: `PROGRAM/STATE.md` → `PROGRAM/HANDOFF.md` (last entry) → `PROGRAM/CLAIMS.md` (skim
statuses) → `PROGRAM/FLAWS.md` (version) → relevant `DECISIONS/`. Then **verify the claimed state
against reality**: run the gate (`bash scripts/check.sh` once it exists; before that, `git ls-files
--eol` must show every tracked text file as `i/lf`). *State files are claims; builds are evidence.*
If STATE.md and reality disagree, reconciling them is the session's first task.

### 4.2 ASSESS — locate yourself
Current phase and gate; which exit criteria are done-with-evidence vs merely asserted; whether the
NEXT-SINGLE-ACTION survived ORIENT; standing escalations awaiting Al.

### 4.3 PLAN — choose one thing
The single task that most advances the current gate. Bias to the inherited NEXT-SINGLE-ACTION unless
ORIENT invalidated it (record why, if so). No parallel workstreams within a session.

### 4.4 EXECUTE + VERIFY — do it, then prove it
Work under §1. Verification is evidence-producing: kernel checks and axiom prints for Lean; tests,
goldens, lint for Rust; seeded reproductions for results; manifest self-checks for artifacts;
retrieval evidence for scholarship. **Two-strike rule:** the same verification failing twice on the
same approach means stop — write the failure up honestly in HANDOFF and switch approach or end the
session. A recorded dead end is progress; a context window burned on loop N is not.

### 4.5 RECORD + REFLECT — the metacognitive artifact
Update STATE.md (new NEXT-SINGLE-ACTION), CLAIMS.md, and FLAWS.md if touched. Then append to HANDOFF.md:

```
SESSION: <date> @ <git short SHA at session end>
PHASE / GATE: ...
ADVANCED: what actually moved, with evidence links (commits, CLAIMS/FLAWS transitions)
BELIEF DELTAS: what I believe now that session-start me did not, and the evidence that moved it
CONFUSIONS: what I misunderstood at session start, what it cost, and the doc fix so successor doesn't repeat it
HONESTY CHECK: register entries re-examined; downgrades made (downgrading is a success — record it plainly)
LEAKAGE CHECK: did any contact-corpus or real-AI-proof behaviour touch a design decision this session? (SD8; "no" must be checked, not assumed)
DETERMINISM CHECK: every new result regenerable from seed + config + SHA + versions? name any that isn't and quarantine it from CLAIMS
MANIFEST CHECK: every artifact touched carries a manifest with ground truth (SD1)? every use of "proven" satisfied SD2? nothing synthetic labeled real (SD12)?
DRIFT CHECK: verdict creep on a real manuscript (SD3)? scope creep into understanding (SD17)? detector-shopping after seeing results? any referee contact (SD13)? name and cut what's found
NEXT-SINGLE-ACTION: one action, unambiguous, executable cold
ESCALATIONS: anything requiring Al (4.6)
STOP-REASON: gate reached / natural boundary / context budget / two-strike / blocker
```

BELIEF DELTAS and CONFUSIONS are the metacognition — calibration data for a successor with your
capabilities and none of your context. The four CHECK fields are this program's specific conscience.

### 4.6 DECIDE — continue or stop clean
Stop conditions: gate reached; natural boundary with context low; two-strike; blocker requiring Al.
**Stop while green** (SD9): never end with the default branch broken — fix or revert first, and say so.

**Escalate to Al (write to ESCALATIONS, do not act):** payments, accounts, credentials; anything
public-facing (publication, outreach, repo visibility); contacting external researchers or the
authors of any proof; **choosing, recruiting, or contacting human referee participants (SD13)**; any
output that characterizes a specific real manuscript beyond SD3; licence ambiguities; irreversible
operations (force-push, history rewrite, deletions beyond normal edits); constitution amendments;
lifting or modifying the SD8 contact quarantine in any way not already prescribed by a passed gate;
ingesting any real AI-generated proof before Gate S4 is closed.

## §5 Failure Modes → Countermeasures

| Failure mode | Countermeasure |
|---|---|
| Verdict creep ("this manuscript is wrong!") | §0.3 hard invariant + SD3 + DRIFT CHECK every session |
| "Proven" used for anything but SD2 | SD2 + MANIFEST CHECK; the word is reserved |
| Statement drift in our own Lean variants | SD2(iii) fidelity diff; FLAW-0008 labeling; statement-fidelity detector |
| Scope creep into "understanding" | SD17 + DRIFT CHECK |
| Surface detectors mistaken for triage | §0.7 pre-declared expectation; SD4 FPR; SD5 structured nulls |
| Detector-shopping / post-hoc tuning after contact | SD8 freeze + quarantine + immutability; LEAKAGE CHECK |
| Detectors fingerprinting generators, not flaws | S3.3 leave-one-generator-out and leave-one-class-out |
| Synthetic item labeled real, or vice versa | SD12; `origin` and derivation fields in every manifest |
| Mislabeled flaw (injector missed, or injected two) | S2.2 fidelity verdict per artifact; FAIL artifacts kept as findings |
| Referee protocol run without Al's decision | SD13 + §4.6 escalation list |
| Citation from model memory contaminating the record | SD11; UNVERIFIED default; sweep bannered map-not-evidence |
| Absolute novelty claims | §0.6 scoped-novelty rule |
| Seed rot / irreproducible numbers | SD6 + DETERMINISM CHECK |
| Thrash spiral | Two-strike rule (§4.4) |
| Successor paralysis | NEXT-SINGLE-ACTION discipline (§3.2) |
| Confident state files over broken reality | "State is a claim; builds are evidence" (§4.1) |
| Memory-warm session certified as cold | SD15 disclosure + conversation-context corollary |

## §6 Session Prompts (paste into Claude Code)

### Session 1 — genesis (already run, 2026-10-12)
Session 1 was launched from Al's genesis prompt, which by its nature carries the program design. The
executor authored this constitution and the register skeletons from it, mirroring the siblings'
structure; provenance is recorded in ADR-0000. The genesis prompt is reproduced in HANDOFF Session 1
only by reference (its content is this file). No generator or detector code was written.

### Session 2 — ratification (run 2026-10-12; kept for the record)
```
You are the executor of the SYNDEOM program. Read PROGRAM/SYNDEOM-PROGRAM.md in full — it is a
DRAFT until this session records Al's ratification — then run the metacognitive loop (§4) exactly:
ORIENT from the repo (state files are claims; builds are evidence), ASSESS, PLAN one thing, EXECUTE
under the §1 constitution, VERIFY, RECORD+REFLECT per the §4.5 schema — including the LEAKAGE,
DETERMINISM, MANIFEST, and DRIFT checks — and stop clean per §4.6. Honor escalations — some
decisions are Al's, not yours.

Al's rulings on the open questions Q1–Q17 in PROGRAM/STATE.md: <Al's answers here, or "see commit
<sha>">. Record each ruling where STATE.md says it belongs, remove the DRAFT marker, and set the
next action to S0.1.
```

### Session N — resume
```
You are the executor of the SYNDEOM program. Read PROGRAM/SYNDEOM-PROGRAM.md in full, then run
the metacognitive loop (§4) exactly: ORIENT from the repo (state files are claims; builds are
evidence), ASSESS, PLAN one thing, EXECUTE under the §1 constitution, VERIFY, RECORD+REFLECT per
the §4.5 schema — including the LEAKAGE, DETERMINISM, MANIFEST, and DRIFT checks — and stop clean
per §4.6. Honor escalations — some decisions are Al's, not yours.
```

---
*SYNDEOM v1.0 — συνδέω, to bind together: Friedman's calibration, Kober's rigor. Ratified 2026-10-12.
Amended 2026-10-13 (ADR-0004, §2 S3.2). Constitution amendments via ADR only.*
