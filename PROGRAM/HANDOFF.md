# HANDOFF — append-only session log

Schema per §4.5 of the constitution. Entries are appended, newest last, and never edited after the
session that wrote them ends.

---

SESSION: 2026-10-12 @ d53f5b6 (Session 1 — genesis; the commit adding this entry lands on top of
d53f5b6)
PHASE / GATE: S0 — FOUNDATION AND CITATION VERIFICATION / Gate S0 open; constitution DRAFT pending
Al's ratification (S0.3)
LAUNCH CONTEXT (recorded for SD15; this was not an exam session): launched from Al's genesis prompt,
which by its nature carries the program design. Recalled memory at session start: the memory-index
lines for the two siblings (repo location + "run the loop from the repo" only — D1-compliant), the
user-identity and machine-hazard lines, and roughly thirty index lines for unrelated private
programs (withheld from this public repo). No SYNDEOM memory existed before this session; one was
written at close holding location and machine hazards only.
ADVANCED:
- Step 0, environment: remote `Quigles1337/Syndeom` confirmed public, empty, created
  2026-10-12T21:04:39Z, Actions enabled; no local clone existed; cloned to `C:\Users\LEET\Syndeom`;
  branch `main` discovered from the clone's unborn HEAD and the REST API (ADR-0000 D1–D3). Siblings
  read at KOBER `f94bec2` (`master`) and FR13DMAN `7c3e5dd` (`main`), both clean and in sync with
  their remotes, read-only; their working trees were not touched.
- Step 0, structural map: `PROGRAM/SIBLING-MAP-2026-10.md` — constitution skeleton section by
  section; rule-set inheritance (E1–E10 and F1–F12 → SD1–SD17); phase register format; register
  schemas; the §4 loop and HANDOFF fields; the §6 resume-prompt convention and the ADR-0003 memory
  discipline; FRIEDMAN's manifest schema D1–D7 plus crate facts; KOBER's Lean configuration;
  toolchain pins and the machine inventory; layout, gate, CI; twelve sibling-vs-sibling divergences;
  recorded hazards.
- Constitution DRAFT `PROGRAM/SYNDEOM-PROGRAM.md` v1.0: §0 eight charter items (problem, scope
  boundary, verdict ban, reserved "proven", two equal-rank success states, scoped novelty, detector
  priority, repo as memory); §1 SD1–SD17; §2 S0–S5 with gates and cold-start test; §3 registers
  including 3.7 FLAWS.md; §4 loop with LEAKAGE, DETERMINISM, MANIFEST, and DRIFT checks; §5 table;
  §6 prompts (genesis, ratification, resume).
- Registers: STATE (Gate S0 table; Q1–Q17; NEXT = Session-2 ratification); CLAIMS
  (CLAIM-0001..0006 UNKNOWN); SOURCES (S-0001..S-0005 UNVERIFIED, wording from the prompt only);
  FLAWS 0.1.0 DRAFT (FLAW-0001..0013, each with definition, manifests-in, Lean-catchable status,
  severity range, generator recipe, example, candidate detectors; summary table; the pre-registered
  expectation that four classes are invisible to compilation); HANDOFF; DECISIONS/ADR-0000.
- Hygiene: `.gitattributes` byte-identical to the siblings'; `.gitignore`; `git ls-files --eol`
  11/11 `i/lf w/lf`; byte check CR = 0, BOM = none, trailing LF on every file; cross-reference
  check: every Q, SD, FLAW, CLAIM, and S- reference resolves to a definition. Bootstrap commit
  `d53f5b6`.
- Not done, by instruction: no generator or detector code; no toolchain pinned; no web retrieval
  (S0.1 belongs to Session 2 or later).
BELIEF DELTAS:
- "Same pins as the siblings" is not one value. The siblings diverge on Rust components (clippy vs
  clippy + rustfmt), workspace root (repo root vs `engine/`), CI action majors (v4 vs v5), default
  branch, rule prefix, and commit tag — six of the twelve divergences in SIBLING-MAP §10. Evidence:
  the files themselves. Consequence: Q2, Q4–Q9.
- "Compatible with FRIEDMAN's manifest schema" cannot mean "parsable by FRIEDMAN's crate": every
  struct is `deny_unknown_fields`, and `Manifest::check` rejects any schema string other than
  `friedman-manifest/1` and any `origin` other than `synthetic`. Evidence:
  `engine/manifest/src/lib.rs` at `7c3e5dd`. Consequence: Q10 is a semantics decision for Al, not an
  engineering detail.
- KOBER's Lean pin (v4.33.1, no Mathlib) was chosen for finite decidable arithmetic; SYNDEOM's S1.2
  needs mathlib formalizations, and Mathlib pins its own toolchain. "KOBER's pin unless Al says
  otherwise" may be internally inconsistent with S1. Evidence: KOBER ADR-0001; the genesis prompt's
  S1 text. Consequence: Q8.
- The siblings' records of the memory-discipline corollary disagree: KOBER STATE says ratified by
  the architects; FRIEDMAN ADR-0003 and STATE say unratified, awaiting Al. Evidence: both STATE files
  at the tips read. Consequence: Q13.
- Cloning an empty GitHub repo with git 2.49 yields the server's unborn HEAD (`main`); `git init`
  on this machine yields `master` (system `init.defaultBranch`). That explains the siblings'
  difference, and it means branch discovery for an empty remote is `git clone` then
  `git symbolic-ref HEAD`, never `git init`. Evidence: ADR-0000 context.
- The executor's knowledge does not cover the five S0 sources Al named (the 6 October 2026 release
  post-dates it; "AGMAI" and "AHM" are not expandable from knowledge). Everything stays UNVERIFIED
  and the acronyms are deliberately not expanded. Evidence: SOURCES.md.
CONFUSIONS:
- Genesis step 0 says "if no repo exists yet, ask Al"; a remote existed but no clone did. Resolved
  as: the remote is the repo Al designated; the local path follows the KOBER ADR-0000 precedent and
  is flagged as Q1. Cost: none. Doc fix: ADR-0000 D2.
- Whether to use `S` as the rule prefix (program initial, FRIEDMAN style): it collides with phases
  S0–S5 and sources S-NNNN. Chose `SD`; flagged Q4. Cost: none.
- The Python check script printed `§` and `→` to a cp1252 console and died *after* every
  substantive check had passed, so the first read looked like a failure. Cost: one re-run. Doc fix:
  hazard recorded in the memory file; a successor should read the check's RESULT line, not its exit
  code, when the traceback is an encode error — or run it with `PYTHONIOENCODING=utf-8`.
- A `cd` inside one Bash call moved the harness's working directory for later calls (into
  `KOBER/lean`). Absolute paths thereafter. Cost: none.
HONESTY CHECK: no CLAIM or FLAW transitions (none earned). CLAIM-0001..0005 record only the genesis
prompt's wording; nothing was added from knowledge, including acronym expansions. CLAIM-0006 is
registered as the premise, not asserted. The constitution is labeled DRAFT and executor-authored,
not "verbatim from Al" (the siblings' provenance), because that is what happened (ADR-0000 D6). The
FLAWS.md examples are labeled synthetic and hand-checked; the FLAW-0010 example uses a placeholder,
not a real name. Downgrades: none needed.
LEAKAGE CHECK: checked, not assumed — no corpus of any kind was touched; `contact/` does not exist;
no real AI-generated proof was read. The only design inputs were the two sibling repositories and
Al's prompt. No leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results exist. Everything written is text under the LF
pin; the cross-reference check is a script over the committed bytes. Nothing to quarantine.
MANIFEST CHECK: no artifacts exist. "Proven" appears only as a defined term (§0.4, SD2) and inside
FLAW entries describing what the word must not be used for; no proof was labeled anything. Nothing
synthetic labeled real: the S0 sources are UNVERIFIED and the FLAWS examples are labeled synthetic.
DRIFT CHECK: no statement about any real manuscript (SD3); no statement about understanding (SD17 —
the padding class is framed as verification cost); no detectors exist, so no detector-shopping; no
referee contact (SD13). Scope note: the executor authored a constitution rather than placing one —
that is the genesis prompt's instruction, recorded in §6 and ADR-0000, not drift.
NEXT-SINGLE-ACTION: see STATE.md — Session-2 ratification: record Al's rulings on Q1–Q17, apply any
renames, remove the DRAFT markers, commit; then S0.1 (retrieve S-0001 from a primary source).
ESCALATIONS:
- Q1–Q17 in STATE.md — the values needed and not readable from source: local path, branch
  confirmation, LICENSE, rule prefix, commit tag, Rust components, workspace root, Lean/Mathlib, CI
  action versions, manifest compatibility semantics, schema fields, taxonomy conventions,
  memory-corollary status, founding sweep, retrieval channel, commit trailer, layout names.
- Ratification of the constitution draft (S0.3) and of FLAWS.md 0.1.0 (S0.5).
STOP-REASON: natural boundary — the Session-1 deliverables are complete (structural map,
constitution draft, every register skeleton plus FLAWS.md, question list) and the genesis prompt
says stop here and present for review. Repo LF-clean at `d53f5b6` and after this entry. This entry's
commit is the first push to `origin/main`, which establishes the default branch on GitHub; the next
ORIENT confirms it with `git remote show origin`.

---

SESSION: 2026-10-12 @ c803c96 (Session 2 — ratification + S0.1; the commit adding this entry and
the S0.1 register updates lands on top of c803c96)
LAUNCH CONTEXT (SD15; not an exam session): launched with the §6 Session-2 prompt carrying Al's
rulings on Q1–Q17, three pre-ratification edits, a standing S2 ruling, and retrieval hints — a
state-carrying prompt by design for a non-exam session. The session runs in the same conversation
as Session 1 (the executor holds its transcript); under the corollary ratified this session it is
therefore not cold, which is why no cold-start test was attempted. Recalled memory at start: the
memory-index lines for SYNDEOM and both siblings (location + hazards + "run the loop from the repo";
D1-compliant), user-identity and machine-hazard lines, and roughly thirty index lines for unrelated
private programs (withheld).
PHASE / GATE: S0 — FOUNDATION AND CITATION VERIFICATION / Gate S0 open. Done this session: S0.3
(ratified), S0.5 (taxonomy ratified), S0.1 (sources). Open: S0.4, S0.6, cold-start; S0.2 waiting on
a ferry.
PLAN NOTE: the inherited NEXT-SINGLE-ACTION was the ratification; Al's prompt directed "then continue
into S0.1 if budget allows". Ratification was executed first as the one thing (`c803c96`); S0.1
followed as Al's explicit direction, sequentially, not in parallel (FRIEDMAN Session 3 precedent for
an escalation response that directs two things).
ADVANCED:
- ORIENT from the repo: clean at `1fe9c06` = `origin/main`, `origin/HEAD` → `main`, 11/11 `i/lf`,
  cross-reference check ALL PASS, STATE/HANDOFF/CLAIMS/FLAWS read from disk and consistent.
- Ratification (`c803c96`): Al's rulings recorded as a table in the ADR-0000 addendum (Q6–Q9, Q17
  bind ADR-0002; Q10–Q11 bind ADR-0001; Q13 ratifies the corollary by citation of FRIEDMAN
  ADR-0003; Q16 keeps the trailer); the S2 standing ruling (deterministic first generator family;
  LLM-assisted families need their own ADR and are outside `battery-freeze-v1` without it)
  recorded in §2 S2.1 and STATE; pre-ratification edits applied — S3.2 fooling length with the
  N_min power floor; FLAW-0014 incomplete case analysis (non-inductive) with gap and fatal
  examples; FLAW-0007 example states n ∈ ℕ; S-0006/CLAIM-0007 added as an SD8 contact-phase
  item; DRAFT markers removed from the constitution, README, FLAWS, STATE, ADR-0000; SIBLING-MAP
  §10 records the Mathlib pin as a load-bearing divergence from KOBER. No CLAIMS transitions at
  that commit (hints are not retrievals).
- S0.1 (this commit): all six sources retrieved and read from primary sources on 2026-10-12
  between 21:57Z and 22:07Z, every one with URL, access time, licence, and SHA-256 of the exact
  bytes read (SOURCES.md). CLAIM-0001..0005 UNKNOWN → VERIFIED; CLAIM-0007 UNKNOWN → VERIFIED
  (existence only). Two deposited PDFs committed under `sources/` with their Zenodo licences (CC0;
  CC BY-NC 4.0) and MD5s reproducing the records'. Counts for S-0001 derived from `openai/math`
  itself at two commits with units stated (722 → 719 manuscripts, 372 families; 300/719 "top-line
  results formalized"; 173 catalogued papers; 242 families with scope notes; 10 reasoning
  summaries; ≈4,000 problems) — not from press. The S5 contact-corpus location is now established.
- Check script repaired: it now reads Q definitions from the ADR-0000 addendum table as well as
  STATE (the rulings moved there this session).
BELIEF DELTAS:
- The post carries no counts at all; every number lives in the repository, and the repository
  carries three *different* formalization units (300 top-line results over 719 manuscripts; 173
  catalogued papers with a formalized main result; 242 families with a Lean scope note) that it
  does not reconcile. Evidence: README, history.md, formalization.yaml, lean/docs listing at the
  tip. Consequence: S5 outputs must name the unit on every number, and "~42% formalized" is a
  claim about one unit only.
- The repository's own update PR states "Lean builds were not run for this synchronization", and
  its formalization catalogue declares `review: status: unchecked` with `automation: agent`. The
  corpus therefore ships with explicitly unverified verification artifacts. Evidence: PR #1 body;
  formalization.yaml tail. Consequence for S3/S5: "has a Lean file" and "has been checked" are
  different predicates; the battery's axiom/build audit cannot assume the latter.
- The contact corpus pins `leanprover/lean4:v4.34.1` and Mathlib `d13f23b7…` with roughly forty
  further git dependencies and a tree the API truncates at 58,728 entries. Evidence: lean-toolchain,
  lake-manifest.json, trees API. Consequence: the Q8 "per-corpus toolchains" consequence is now a
  measured fact, not a hypothetical; S5 ingest must plan for it.
- Primary and secondary disagree on dates and counts in ways the register must carry, not
  smooth: the AHM statement page is undated (the 7 October date is the repost's); the Fields
  Medalist page lists 28 signatories today against 25 in the Zenodo deposit and Tao's "25 initial
  signatories"; the manuscript count is 722 at release and 719 after 7 October; search summaries
  said "722" and "377 results" — the 377 is the catalogue's highest result *number*, not a count
  (372 distinct). Evidence: the primaries at the digests recorded.
- Tool reachability is per-site, not per-tool: openai.com served curl and refused the fetch tool;
  ias.edu refused both; Zenodo refused its API to curl but served record pages to both. Evidence:
  HTTP codes in SOURCES.md. Consequence: try the other tool before recording UNVERIFIED.
CONFUSIONS:
- A `| grep` after the check script masked its non-zero exit, so the ratification commit went
  ahead before I read the script's verdict; the verdict was a stale regex (Q definitions had
  moved), not a repo defect, and the repo was coherent — but the sequence was wrong. Cost: one
  post-hoc verification. Doc fix: this entry; the script's regex is fixed; the sibling hazard "a
  grep in a pipeline is not a guard" is now also a SYNDEOM hazard in the memory file.
- `history.md` had to be read to establish S-0006's *existence*, and it also states the nature of
  the gaps. Resolved by transcribing only existence facts and counts, withholding names and gap
  descriptions from every register (SD8), and recording the exposure in the LEAKAGE CHECK. Cost:
  none. Doc fix: S-0006 carries an explicit "Withheld (SD8)" field so a successor knows what not
  to look for before S5.
- The CONTENTS.md link regex truncated one path at a parenthesis ("CAT(0"), making it look like one
  non-PDF link. It is a parse artifact; all 719 links are PDFs. Cost: one extra check.
- A cp1252 console killed a Python print after the work was done (again). Cost: one re-run.
  `PYTHONIOENCODING=utf-8` and `-X utf8` fixed it; recorded as a hazard.
HONESTY CHECK: six transitions, all UNKNOWN → VERIFIED on primary-source reads, each recording what
was *not* obtained: the IAS page (403), the AHM page's missing date, the three unreconciled
formalization units, the 25-vs-28 signatory difference, the arXiv licence that forbids storing.
CLAIM-0007 is VERIFIED *for existence only* and says so in its status. CLAIM-0006 stays UNKNOWN —
no sweep exists and nothing retrieved bears on the absence of prior calibration work. Nothing from
search-result summaries entered any register (the "722"/"377" figures were replaced by counts from
the repository). The Lean "~42%" is recorded as the repository's own phrasing, not as a verified
proportion of anything SYNDEOM measured. Downgrades: none needed.
LEAKAGE CHECK: checked, not assumed — no design decision was made this session on the basis of any
contact material: the ratification rulings came from Al; ADR-0001/0002 were not written. Exposure to
disclose: the executor read `history.md` and the `openai/math` README/catalogue headers to
establish existence and counts; `history.md` names withdrawn manuscripts and describes the gaps.
Those descriptions were not transcribed, not committed, and informed no decision; no manuscript, PDF,
or Lean file from the corpus was opened; `contact/` does not exist. The two PDFs ingested under
`sources/` are declarations about AI and mathematics, not proofs. Verdict: no leakage; one
disclosed exposure.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. Every retrieval is pinned by URL + access time
+ SHA-256 of the bytes read; repository files are pinned by git blob id and commit; the two stored
PDFs reproduce their records' MD5s. Nothing to quarantine.
MANIFEST CHECK: no artifacts exist. "Proven" was not applied to anything; the corpus's own
"formalized" counts are recorded as its phrasing, with its `review: status: unchecked`. Nothing
synthetic labeled real: the two stored PDFs are real documents recorded as INGESTED sources, not
corpus items.
DRIFT CHECK: no statement about any real manuscript's correctness (SD3) — S-0006 records that
withdrawals exist, not what is wrong in any paper; no statement about understanding (SD17); no
detectors exist; no referee contact (SD13). One scope note: ingesting two declaration PDFs under
`sources/` is bibliography, not corpus work; `groundtruth/` and `contact/` remain nonexistent.
NEXT-SINGLE-ACTION: see STATE.md — S0.4, write ADR-0001 (the proof-artifact manifest schema) under
the Q10/Q11 constraints, with D6-style goldens; then S0.6.
ESCALATIONS:
- Q3 repository LICENSE (standing, due before S5.4).
- Q14 founding sweep ferry (standing; no novelty claim until then).
- None new. No ferry needed for S0.1 (nothing load-bearing was unreachable).
STOP-REASON: natural boundary — S0.1 complete and recorded; S0.4 is a design task that should start
with a fresh read of the sibling's ADR. Repo LF-clean; this commit pushes to `origin/main`.

---

SESSION: 2026-10-12 @ a5f8043 (Session 3 — Q14 ferry + S0.4 ADR-0001; the commit adding this entry
and ADR-0001 lands on top of a5f8043; UTC had passed midnight to 2026-10-13 during the session, so
access times in SOURCES.md read 2026-10-13T02:xxZ)
LAUNCH CONTEXT (SD15; not an exam session): launched with the §6 Session-N resume prompt plus Al's
inputs (the Q14 ferry digest and a Q3 slot). The session runs in the same conversation as Sessions
1 and 2 (the executor holds a compacted transcript of both), so under the ratified corollary it is
not cold; no cold-start test was attempted. Recalled memory at start: the memory-index lines for
SYNDEOM and both siblings (location + hazards + "run the loop from the repo"; D1-compliant),
user-identity and machine-hazard lines, and roughly thirty index lines for unrelated private
programs (withheld).
PHASE / GATE: S0 — FOUNDATION AND CITATION VERIFICATION / Gate S0 open. Done this session: S0.2
(sweep placed), S0.4 (ADR-0001). Open: S0.6, the deferred CLAIM-0006 examination, cold-start.
PLAN NOTE: Al's prompt directed two things in order — the ferry, then "proceed with
NEXT-SINGLE-ACTION (S0.4) if the context budget allows after the ferry". Executed sequentially as
two commits (Session 2 / FRIEDMAN Session 3 precedent), not in parallel. The prompt's path slot read
`<path where Al saved SWEEP-2026-10.md>` literally; a bounded name search of the home directory
found no such file; Al then pasted the sweep's text mid-session, and its bytes were verified
against the stated digest instead — the digest, not the path, is the identity.
ADVANCED:
- ORIENT from the repo: clean at `770580e` = `origin/main`, `origin/HEAD` → `main`, 11 text files
  `i/lf` + 2 PDFs binary, cross-reference check ALL PASS, STATE/HANDOFF/CLAIMS/FLAWS/SOURCES read
  from disk and consistent with each other.
- S0.2 (`a5f8043`): the pasted sweep, written LF with one trailing newline (41,060 bytes), hashed
  to Al's digest `f4bfb2ad…b03e` exactly (seven line-ending/BOM variants tried; only that one
  matched); placed unchanged at `PROGRAM/SWEEP-2026-10.md`; SD11 banner on line 3; mechanical SD8
  check (corpus-path and manuscript-identifier patterns) found only the redaction note itself and
  C9's "omitted here" sentence. STATE S0.2 done; Q14 resolved; CLAIM-0006 transition line (stays
  UNKNOWN); the ten-source retrieval recorded as a deferred S0 action per Al's ruling.
- S0.4 (this commit): `PROGRAM/DECISIONS/ADR-0001.md` — `syndeom-manifest/1`, D1–D9, written
  after re-reading FRIEDMAN's ADR-0001 D1–D7, ADR-0004, `engine/manifest/src/lib.rs`,
  `tests/goldens.rs`, `Cargo.toml`, and ADR-0002 D3/D4 at the pinned tip `7c3e5dd` (unchanged since
  Session 1; local clone at the same commit, read with `git show`, never built). RFC 8785 retrieved
  from the RFC Editor and recorded as S-0007 (VERIFIED, digest-only). Goldens: two fixtures written
  byte-for-byte by a script, every stated number computed from the bytes and cross-checked with
  `sha256sum`; the same pipeline reproduced FRIEDMAN's D6 vector before being trusted. STATE S0.4
  done; NEXT = S0.6 with a discovery-first procedure.
- Check script unchanged; `python … check_repo.py` run alone and gated on `$?` for both commits.
BELIEF DELTAS:
- Identity of a ferried file is the digest, not the path: a paste that hashes to Al's stated
  SHA-256 is the file, and a path alone would have proved nothing. Evidence: seven byte variants
  of the paste, exactly one match. Consequence: S0.2's "verified against Al's stated digest before
  the move" is satisfiable without filesystem access to Al's copy; recorded in STATE.
- A unit of location is a design decision with visible consequences: in Golden A the same span is
  `[35, 60)` in code points and `[37, 62)` in bytes because of one em dash. Evidence: D8.
  Consequence: the ADR fixes code points and the engine test must exercise a non-ASCII prefix.
- `exact` tolerance compares JSON values structurally in FRIEDMAN's crate (`Value == Value`), so
  an integer `0` and a number `0.0` are unequal under `exact`. Evidence: `statistic_verdict` in
  `lib.rs` at `7c3e5dd`. Consequence: ADR-0001 D4 makes the statistic registry declare each key's
  JSON type and uses `abs` for ratios; a successor writing the registry must not learn this by
  surprise.
- Real S1 items will not be Markdown, and the Q11 informal format describes what the generator
  emits, not what errata look like. Evidence: S1.1 (errata, retraction notices) and the S-0001
  corpus (PDF manuscripts). Consequence: `raw/1` with free-text locators; flagged for Al.
- The sibling tip did not move in seven weeks (`7c3e5dd`, 2026-08-25), so the SIBLING-MAP's §7.1
  facts are still those of the live code; re-reading cost one API call and removed an assumption.
CONFUSIONS:
- The Session-3 prompt's Q3 slot carried its template text (`<Al's licence ruling, or "still
  deferred">`) unfilled. Not a value; not chosen. Recorded in STATE as "no ruling delivered, Q3
  unchanged" so a successor does not read the placeholder as a ruling. Cost: none.
- A bounded `find` for the sweep file name under the home directory (maxdepth 4, OneDrive and
  AppData excluded) returned nothing, and a hash-every-recent-Markdown search was killed by its own
  60-second timeout before Al's paste made both moot. Cost: about two minutes. Doc fix: the ferry
  procedure in STATE now says a paste verified by digest is an acceptable ferry.
- Golden B's Lean proof (`simpa [sq] using Nat.le_mul_self n`) cannot be built this session — no
  toolchain is pinned. Rather than claim it builds, ADR-0001 D8 states that its build status is
  unasserted and makes S0.6 measure it. If it fails to build, the fixture's bytes change and the
  digests in D8 are recomputed by a new ADR-0001 transition, never by silent edit.
HONESTY CHECK: register entries re-examined. CLAIM-0006 stays UNKNOWN although the sweep's own
verdict says the claim survives in a narrowed form — the sweep is a map and Al ruled the ten
sources must be read first; the entry's transition says exactly that. S-0007 is VERIFIED on a
first-hand read of the sections used, not of the whole RFC, and says so. ADR-0001 marks every
placeholder in its example manifest and asserts nothing about Golden B's buildability. No
downgrades needed; no transition made on the strength of the sweep.
LEAKAGE CHECK: checked, not assumed. The sweep was read in full to place it; its contact-related
content is existence-level only by Al's redaction, and the mechanical check found no corpus paths or
manuscript identifiers. Design decisions this session (ADR-0001) cite FRIEDMAN, the constitution,
FLAWS.md, Q10/Q11, and RFC 8785 — none cites the sweep, `openai/math`, or any real AI-generated
proof. The one contact-adjacent fact used in the ADR is that contact corpora carry their own Lean
and Mathlib pins (S-0001, an S0.1 existence/toolchain fact), which shaped the `toolchain` field;
that is provenance, not behaviour of any manuscript. `contact/` still does not exist. Verdict: no
leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. Every stated number in ADR-0001 D8 is
recomputable from the fixture bytes printed in the ADR (digests by `sha256sum`, counts by the D3
rules, JCS forms by the §3.2 rules), and the computing script reproduced FRIEDMAN's D6 vector first.
RFC 8785 is pinned by URL, access time, size, and SHA-256. Nothing to quarantine.
MANIFEST CHECK: no artifacts exist yet; ADR-0001 defines what one is. "Proven" was not applied to
anything (Golden B is "unasserted"; `statement_status.method: lean-kernel` in the example carries a
PLACEHOLDER reference until S0.6). Nothing synthetic labeled real: the schema makes `by-construction`
exclusive to `origin = synthetic` and forbids ground truth on contact artifacts entirely.
DRIFT CHECK: no statement about any real manuscript (SD3) — the schema now makes one unwritable in a
contact manifest; no statement about understanding (SD17); no detectors exist, so no detector-
shopping; no referee contact (SD13). Scope note: ADR-0001 decided real-origin fields (ingest
producer, `raw/1`) that Q11 did not spell out; they are flagged for Al rather than treated as
settled.
NEXT-SINGLE-ACTION: see STATE.md — S0.6, write ADR-0002 and land the scaffold, discovery first
(`elan`, Mathlib's `lean-toolchain` at the chosen pin, the siblings' `rust-toolchain.toml`,
crates.io for the JCS crate, Actions availability), then the gate, then Golden B's build measured.
ESCALATIONS:
- Q3 repository LICENSE (standing; the Session-3 slot was unfilled; due before S5.4).
- ADR-0001 flags five design choices for Al's attention (its Consequences): ingest "producer" in
  `generator`, code-point spans, `raw/1` for non-Markdown real items, `ground_truth: null` for
  contact, Golden B unbuilt until S0.6. Non-blocking; amend by ADR if objected to before S1.3/S2.1.
- None new otherwise. Q14 is closed.
STOP-REASON: natural boundary — S0.2 and S0.4 complete and recorded; S0.6 begins with toolchain
discovery that deserves a fresh session (elan, Mathlib pin, crate re-read). Repo LF-clean; this
commit and `a5f8043` push to `origin/main`.

---

SESSION: 2026-10-12 @ 9137c64 (Session 4 — ADR-0001 ratified and amended, then S0.6: ADR-0002 and
the scaffold; the commit adding this entry lands on top of 9137c64; UTC was 2026-10-13 throughout)
LAUNCH CONTEXT (SD15; not an exam session): launched with the §6 Session-N resume prompt plus Al's
rulings on ADR-0001's five flagged choices and a Q3 slot. Same conversation as Sessions 1–3
(compacted transcript held), therefore not cold; no cold-start test attempted. Recalled memory at
start: the SYNDEOM/KOBER/FRIEDMAN memory-index lines (location + hazards; D1-compliant),
user-identity and machine-hazard lines, and roughly thirty index lines for unrelated private
programs (withheld).
PHASE / GATE: S0 — FOUNDATION AND CITATION VERIFICATION / Gate S0 open. Done this session: ADR-0001
ratification + amendment (`9137c64`), S0.6 (ADR-0002, scaffold, gate — this commit). Open: the
deferred CLAIM-0006 examination, then the cold-start test.
PLAN NOTE: Al's prompt directed the amendment commit first, then S0.6 "if the context budget allows".
Executed in that order as two commits. The Q3 slot carried its placeholder text again; recorded as
"no ruling delivered", not chosen.
ADVANCED:
- ORIENT: clean at `d3e5842` = `origin/main`, `origin/HEAD` → `main`, 13 text files `i/lf` + 2 PDFs,
  register check ALL PASS, constitution unchanged since `c803c96`.
- ADR-0001 (`9137c64`): Status now "ratified by Al in Session 4"; D3 gains the UTF-8 decoding
  sentence (ii) and the archival note on `raw/1` (iii); D4 gains the derived-canonical-artifact
  convention (iii); Consequences carry the Resolution; an Amendments log closes the file.
- Discovery (SD7), every value read: elan 4.2.3 with `stable` → v4.35.0 not installed; Mathlib has
  tags up to `v4.34.1` plus `v4.35.0-rc1..rc4` and **no `v4.35.0`**; tag `v4.34.1` = commit
  `d13f23b7…` whose `lean-toolchain` reads `leanprover/lean4:v4.34.1`; KOBER's Rust pin re-read
  (`1.91.0`, clippy only) and FRIEDMAN's (`1.91.0`, clippy + rustfmt); crates.io versions; Actions
  enabled on the repo with `v5` tags available for every GitHub-owned action used.
- `lean/`: `lean-toolchain`, `lakefile.toml` (Mathlib by URL + full rev), `Syndeom.lean`,
  `Syndeom/Fixtures/GoldenB.lean` byte-identical to ADR-0001 D8 (`c568a981…458e`), `Audit.lean`;
  elan installed v4.34.1; `lake update` locked nine packages (`lake-manifest.json` committed);
  `lake exe cache get` fetched 8,908 files.
- `engine/`: workspace + crate `syndeom-manifest` implementing ADR-0001 D1–D7 (types with
  `deny_unknown_fields` and a `required_option` helper so nullable fields must be present; both
  `artifact_id` forms; the informal segmenter with code-point spans; the Lean counter; the D6
  verdict rule; `check` with every D4 rule; `check_derived`). `tests/goldens.rs`: 23 tests — D8
  goldens incl. the span example, ADR-0001's example manifest parsed from the ADR text and
  self-checked against Golden A, the Lean module pinned to the fixture bytes, D2/D3/D4 rule
  rejections, the derived-artifact convention, ADR-0004's seven vectors, RFC 8785 §3.2 sample +
  Appendix B 24 rows + NaN/Inf + UTF-16 sorting. `cargo fmt --check`, `clippy -D warnings`, and
  `cargo test` all clean; 24 locked crates.
- `scripts/check.sh` (the gate) and `scripts/check_registers.py` (moved into the repo from the
  executor's scratchpad); `.github/workflows/check.yml` (advisory, `actions/*@v5`); READMEs for
  `lab/`, `corpora/`, `groundtruth/`, `contact/` (SD8 rules posted), `analysis/`; `.gitignore`
  with the `regenerable` storage rule; `rust-toolchain.toml` at the root.
- SOURCES: S-0008 — licences of the Lean toolchain, Mathlib, its eight Lake dependencies, and the
  24 crates, each read from the pinned checkout or `cargo metadata`.
- ADR-0002 written with the discovery table and D8 measured results: `lake build` succeeded
  (`Built Syndeom.Fixtures.GoldenB (172s)`, 8,926 jobs, 8,924 from Mathlib's cache); the axiom
  audit prints `[propext]`; the whole Lean sequence took 02:56:39Z–03:10:45Z. ADR-0001's
  Amendments log records the Golden B measurement as a transition. Gate: `bash scripts/check.sh`
  run before this commit (result in STATE "Build status").
BELIEF DELTAS:
- "Latest stable Lean" and "latest Mathlib release" are different points: elan's `stable` is
  v4.35.0 while Mathlib's newest release tag is `v4.34.1` (master is on rc4). The Q8 rule — Lean
  follows Mathlib — resolves it without a judgment call. Evidence: `elan show`, the tag list.
- The newest Mathlib release tag is the very commit the first contact corpus pins (S-0001). The
  criterion (newest release tag with a stable Lean) was fixed in STATE's S0.6 procedure before the
  tag list was read, so the coincidence is one of dates; it is disclosed in ADR-0002 and below.
- serde's derive silently maps a *missing* `Option` field to `None`, so "present as `null`"
  (ADR-0001 D4) needs an explicit `deserialize_with` helper; without it the test
  `d4_nullable_fields_must_be_present` fails. Evidence: that test, red before the helper.
- Rust byte-string literals cannot carry `\u{…}` escapes; `cargo fmt` reformats freely-written
  code, so the committed form is whatever rustfmt emits and the gate checks exactly that.
- A `lake build` that imports all of Mathlib is silent for minutes when its output is piped
  through `tail`; a successor should run it unpiped or tail the log with `tr '\r' '\n'`.
CONFUSIONS:
- The Session-4 prompt's Q3 slot carried its placeholder text, as in Session 3. Recorded; not a
  value. Cost: none.
- `du -sh lean/.lake` during the cache unpack exceeded the tool's 120 s limit and was backgrounded;
  never measure `.lake` while lake is writing it. Cost: one stray background job.
- The first `cargo build` failed on a missing `Ord` derive and the first test build on `\u{…}` in a
  byte string; both one-line fixes. Cost: two iterations, no two-strike.
HONESTY CHECK: ADR-0002 D8 states only what was run this session; the Lean result is recorded as
measured, not asserted. Golden B's axiom audit prints the axiom set — that satisfies SD2(i)–(ii)
for the fixture but **not SD2(iii)** (no recorded statement-fidelity diff against an informal
statement), so Golden B is "builds, standard axioms", never PROVEN, and no register says otherwise.
S-0008 licences were read from the checked-out files and crate metadata, not assumed. No
downgrades needed.
LEAKAGE CHECK: checked, not assumed. No contact material was read this session; `contact/` now
exists, empty, with the SD8 rules posted. One disclosure: the Mathlib pin chosen by ADR-0002's
criterion (newest release tag with a stable Lean) is the same commit the `openai/math` corpus pins
(S-0001). The criterion was written into STATE's S0.6 procedure in Session 3 and the tag list was
read afterwards; the corpus's pin was neither a reason nor a tiebreaker, and the ADR says so.
Verdict: no leakage; one disclosed coincidence.
DETERMINISM CHECK: every pin is a commit or a locked version (`lean-toolchain`, `lakefile.toml`
rev, `lake-manifest.json`, `rust-toolchain.toml`, `Cargo.lock`); the goldens reproduce ADR-0001
D8 from fixed bytes; Mathlib's compiled modules are fetched by `lake exe cache get` for that
commit and regenerable from a fresh clone by the gate alone. No MEASURED or REPRODUCED result
entered CLAIMS. Nothing to quarantine.
MANIFEST CHECK: no artifacts exist; ADR-0001's example manifest now self-checks in the engine
against Golden A's bytes. "Proven" applied to nothing (see HONESTY CHECK). Nothing synthetic
labeled real; the READMEs for `corpora/`, `groundtruth/`, `contact/` restate the origin rules.
DRIFT CHECK: no statement about any real manuscript (SD3); no understanding claims (SD17); no
detectors, so no detector-shopping; no referee contact (SD13). Scope note: the gate's axiom audit
is infrastructure for SD2, not a detector.
NEXT-SINGLE-ACTION: see STATE.md — the deferred CLAIM-0006 examination: retrieve and read the ten
sources ranked in `PROGRAM/SWEEP-2026-10.md` "Verdict (3)" from primary sources, one S-entry each,
then move CLAIM-0006 on evidence; after that, the Gate S0 cold-start test in a fresh conversation.
ESCALATIONS:
- Q3 repository LICENSE (standing; the Session-4 slot was unfilled; due before S5.4).
- For Al's awareness, not a decision: the Mathlib pin coincides with the first contact corpus's
  (LEAKAGE CHECK above); the advisory CI's first run result is unknown until the push lands.
- None new otherwise.
STOP-REASON: natural boundary — S0.6 complete and the gate green; the next action (ten
primary-source retrievals for CLAIM-0006) is a retrieval session of its own, and the cold-start
test after it must be a fresh conversation. Repo green; this commit pushes to `origin/main`.

---

SESSION: 2026-10-13 UTC @ 132788c (Session 5 — the deferred CLAIM-0006 examination; the commit
adding this entry lands on top of 132788c)
LAUNCH CONTEXT (SD15; not an exam session): launched with the §6 Session-N resume prompt plus Al's
inputs (a Q3 slot and the instructions for the CLAIM-0006 examination). Same conversation as
Sessions 1–4 (compacted transcript held), therefore not cold; no cold-start test attempted.
Recalled memory at start: the SYNDEOM/KOBER/FRIEDMAN memory-index lines (location + hazards;
D1-compliant), user-identity and machine-hazard lines, and roughly thirty index lines for
unrelated private programs (withheld).
PHASE / GATE: S0 — FOUNDATION AND CITATION VERIFICATION / Gate S0 open on the cold-start test
only. Done this session: the CLAIM-0006 examination (S0.1 continuation). Open: the cold-start test.
PLAN NOTE: one thing — the ten retrievals and the claim's examination — as Al's prompt and STATE's
NEXT-SINGLE-ACTION both directed. Al's prompt allowed several sessions; one sufficed. The Q3 slot
arrived as template text for the third time; recorded as a choice ("deferred pending Al's explicit
decision text"), not as silence, per Al's instruction and SD7.
ADVANCED:
- ORIENT: clean at `132788c` = `origin/main`; `bash scripts/check.sh` → `GATE: GREEN` as evidence
  (Golden B builds, axioms `[propext]`, 23/23 tests); constitution unchanged since `c803c96`; the
  advisory CI run for `132788c` completed `success`.
- Retrieval: all nine arXiv papers the sweep ranks retrieved as abstract page + latest PDF + HTML
  (HTTP 200 each, 03:23Z) and the `leanprover/comparator` repository read through the GitHub API;
  Hard2Verify's ACL 2026 listing confirmed on the Anthology page; AdvancedMathBench's quoted
  "nearly 90%" sentence found in the v2 HTML after the PDF extraction split it. Licences read per
  paper: seven CC BY 4.0 (PDFs stored under `sources/`, ~16 MB), two arXiv non-exclusive
  (digest-only), comparator Apache-2.0.
- Reading: each paper's text was extracted and read against the claim's two axes — by error/flaw
  class, by proof length — plus level and whether the proofs were AI-generated; the findings are in
  each S-entry's "Bears on CLAIM-0006" field in the source's own words with figure/table/section
  references (SOURCES.md S-0009..S-0018; S-0002/S-0006 cross-referenced for rank 9).
- CLAIM-0006: **no retrieved source falsifies the claim as worded** (said first, as Al asked). The
  claim is narrowed to the narrowest form the sources leave standing — tighter than the sweep's
  §(2) draft — and moved UNKNOWN → CONDITIONAL (unfalsified as of 2026-10-13; bounding sources
  listed; FALSIFIED by any single counterexample). The scoped novelty statement is recorded; §0.6
  lifts only in that scope.
- STATE: Q3 recorded as a choice; CLAIM-0006 row done; registers line; NEXT-SINGLE-ACTION = the
  Gate S0 cold-start test with a procedure a cold successor can execute; the resume prompt a cold
  session must receive is reproduced verbatim and alone at the end of this entry.
BELIEF DELTAS:
- The length axis has a precedent I did not expect to be this concrete: ProofGrid (S-0013) plots
  proof-checking accuracy as a function of proof length and reports a challenge set of 720–987-unit
  proofs on which "only two of [the four strongest models] manage to perform better than flipping
  a coin" — a measured near-chance regime at long length, in formal natural deduction, with no
  flaw-class breakdown. Evidence: its Figures 7–8 and Table 6. Consequence: SYNDEOM's fooling
  length is not the first observation of the phenomenon; it is the first that would be per flaw
  class, power-floored, and on mathematical proofs. The novelty statement says exactly that.
- The one class × length result (S-0009) is weaker than the sweep implied: 50 GPT-4-rewritten
  word-problem solutions per step count, one error class, "not significantly affected". Evidence:
  its Appendix E.1. Consequence: the narrowed claim excludes word problems explicitly rather than
  arguing about whether they are proofs.
- Research-level AI-generated proofs with human-labelled errors already exist in a benchmark
  (S-0015's FirstProofGradingBench: 16 proofs, 24 referee-identified errors), and the measured
  verifiers leave "roughly 40%" of research-paper errors uncaught at k = 8. Evidence: S-0015 §4.
  Consequence: S4/S5 have an external validation set to position against, and the claim's "no
  breakdown at research level" is a statement about breakdowns, not about the absence of labels.
- A claim of absence has a proper register state: CONDITIONAL with the bounding set named. Al's
  ruling that retrieval cannot make it VERIFIED matched the register vocabulary without an
  amendment. Evidence: §3.3 statuses; Al's prompt.
- arXiv's HTML rendering is the better text source for sentence-level verification; poppler's
  extraction split one load-bearing sentence into unmatched fragments. Evidence: the "nearly 90%"
  search. Consequence: recorded as a hazard.
CONFUSIONS:
- The Session-5 prompt's Q3 slot was template text again. Al's instruction to "show a choice
  rather than silence" was followed by recording the only choice the executor can make — deferred
  pending Al's explicit text — and flagging it below. Cost: none; it is an escalation, not a value.
- The ACL Anthology page carries its `<meta>` attributes content-first, so my first metadata regex
  found nothing; the venue, Anthology ID and pages were read from the page text instead. Cost: one
  extra call.
- The sweep's "cite the version" hint for AdvancedMathBench was right: the category analysis is in
  v2 (29 Sep 2026), and the register names v2 explicitly.
HONESTY CHECK: the claim was examined as worded before being narrowed, and the register says no
source falsifies it as worded — a weaker claim about word problems (S-0009) is reported rather than
argued away. CLAIM-0006 is CONDITIONAL, not VERIFIED, with the bounding set and the unretrieved
runners-up named; the novelty statement is scoped to the retrieval date and set. Every quoted
sentence in S-0009..S-0018 was read in the retrieved text; three secondary-coverage items the sweep
mentions (Retraction Watch, aggregator counts, blogs) stayed out of the register. ProofGrid's
proof-length unit is recorded as the paper's own unit because the paper does not define it where
the PL1-PC-c lengths are stated. Downgrades: none; one upgrade (UNKNOWN → CONDITIONAL) with
evidence.
LEAKAGE CHECK: checked, not assumed. No contact material was read: nothing from `openai/math` was
opened; S-0006 was cross-referenced by its existing register entry only. The papers read describe
other corpora (Hard2Verify responses, First Proof submissions, miniF2F proofs); they informed the
novelty claim's scope and nothing in the battery design — no detector, generator, or threshold was
decided this session. `contact/` remains empty. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. Every retrieval is pinned by URL, access
time, size, and SHA-256; stored PDFs reproduce the retrieved digests byte for byte; comparator is
pinned by tip commit. Nothing to quarantine.
MANIFEST CHECK: no artifacts exist; the seven stored PDFs are INGESTED sources under `sources/`,
not corpus items, and are binary by `.gitattributes`. "Proven" applied to nothing. Nothing
synthetic labeled real.
DRIFT CHECK: no statement about any real manuscript's correctness (SD3) — the register reports what
papers report about verifiers, never a verdict on a proof; no understanding claims (SD17); no
detectors, so no detector-shopping; no referee contact (SD13). One scope note: the register
records cost datapoints (Hard2Verify's 90 + 63 minutes per response) for S3.4; they are quotes,
not a model.
NEXT-SINGLE-ACTION: Gate S0 cold-start test, in a fresh conversation launched with the prompt below
verbatim and alone (procedure in STATE.md). Not executable from this conversation.
ESCALATIONS:
- **Q3 repository LICENSE** — three prompts in a row have carried the template text; the executor
  recorded "deferred pending Al's explicit decision text". Al: please paste the decision (an SPDX
  identifier, or the words "deferred pending <reason>") so the record carries Al's choice, not the
  executor's description of its absence. Due before S5.4.
- The cold-start test must be launched by Al: a new conversation, the prompt below, nothing else.
- None new otherwise.
STOP-REASON: natural boundary — the CLAIM-0006 examination is complete and recorded; the next
action cannot run in this conversation (SD15). Repo green; this commit pushes to `origin/main`.

COLD-START RESUME PROMPT (the exact §6 "Session N — resume" text; a cold session receives this and
nothing else):
```
You are the executor of the SYNDEOM program. Read PROGRAM/SYNDEOM-PROGRAM.md in full, then run
the metacognitive loop (§4) exactly: ORIENT from the repo (state files are claims; builds are
evidence), ASSESS, PLAN one thing, EXECUTE under the §1 constitution, VERIFY, RECORD+REFLECT per
the §4.5 schema — including the LEAKAGE, DETERMINISM, MANIFEST, and DRIFT checks — and stop clean
per §4.6. Honor escalations — some decisions are Al's, not yours.
```

SESSION: 2026-10-13 UTC @ 2b7c573 (Session 6 — Gate S0 cold-start exam; the commit adding this
entry lands on top of 2b7c573)
RECALLED MEMORY (SD15 — the first entry of this block, verbatim, before any other action; this is
an exam session, so no memory was written): the launching message was the §6 "Session N — resume"
prompt verbatim and alone, in a fresh conversation holding no transcript of Sessions 1–5. The
harness loaded the executor's memory index (99 lines) at start. The lines that name SYNDEOM or a
sibling program are reproduced below exactly as loaded, and the memory file the SYNDEOM line
links was read in full and is reproduced after them exactly as read. The index's remaining 85
lines concern unrelated private programs; none mentions SYNDEOM (checked by a case-insensitive
grep for "syndeom" over the index, which matched lines 98–99 only), and they are withheld from
this public file by count rather than by text. Nothing in any recalled memory was used as a value:
every value in the re-derivation below was read from the repo this session.

Index lines 1–4, 83–87 and 98–99 of `MEMORY.md`, verbatim:
```text
# Project Memory

## User Identity
- **GitHub**: Quigles1337 · **Email**: adz@alphx.io · commit with `--author="Quigles1337 <adz@alphx.io>"`
## KOBER (Linear A falsification infrastructure)
- [Repo pointer + machine hazards](project_kober.md) — `C:\Users\LEET\KOBER`, remote `Quigles1337/KOBER` (**public**); the constitution `PROGRAM/KOBER-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (memory-policy ADR).

## FR13DMAN (FRIEDMAN — designedness-detection instrument)
- [Repo pointer + machine hazards](project_friedman.md) — `C:\Users\LEET\FR13DMAN` / `Quigles1337/FR13DMAN` (**public**, leetspeak name); sibling of KOBER; the constitution `PROGRAM/FRIEDMAN-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (ADR-0003).
## SYNDEOM (AI-proof calibration — third sibling of KOBER/FRIEDMAN)
- [Repo pointer + machine hazards](project_syndeom.md) — `C:\Users\LEET\Syndeom` / `Quigles1337/Syndeom` (**public**); the constitution `PROGRAM/SYNDEOM-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (SD15 / FR13DMAN ADR-0003).
```

File `project_syndeom.md` (SHA-256 `feff158ccb872ca5627d393fc91aa544cca4a60d25d6af393282112f7ea8f798`), verbatim:
```text
---
name: project-syndeom
description: "SYNDEOM program - repo C:\\Users\\LEET\\Syndeom (remote Quigles1337/Syndeom, public); constitution and ALL program state live IN the repo, never here"
metadata:
  node_type: memory
  type: project
  originSessionId: 07cebf44-c690-47a3-8c97-d4b909c4e75b
  modified: 2026-10-13T03:32:54.297Z
---

**SYNDEOM** — repo `C:\Users\LEET\Syndeom`, remote `https://github.com/Quigles1337/Syndeom`
(**public**). Third sibling of [[project-kober]] and [[project-friedman]].

This file locates the repo and records machine hazards. Per the constitution's memory-discipline
rule (adopting FR13DMAN ADR-0003) it records NO program state: no phase, gate, claims, commits,
toolchain versions, escalations, or next actions.

**Standing rule:** the repo is the sole memory of the program (constitution at
`PROGRAM/SYNDEOM-PROGRAM.md`). Every session: read the constitution, then run its §4 loop from
`PROGRAM/STATE.md` and `PROGRAM/HANDOFF.md`. Never resume from this file. Discover the default
branch, toolchain, and every other value from the repo — never assume one, and never carry a value
across from a sibling program.

**Hard invariants (standing prohibitions, not state):** SYNDEOM never issues verdicts on any real
contested proof; "proven" is reserved for sorry-free Lean proofs that also passed
statement-fidelity review; it measures truth and triage, never understanding.

**Never work in the sibling KOBER or FR13DMAN trees** — identical `PROGRAM/` schema, and their
sessions may be live concurrently.

Machine and harness hazards:
- System-level git config has `core.autocrlf=true`; the repo pins LF via `.gitattributes`. Verify
  with `git ls-files --eol` and a byte-level CR/BOM check, never by eye.
- The Python console is cp1252: set `PYTHONIOENCODING=utf-8` before any script that prints
  non-ASCII (`§`, `→`, `—`), or it dies with a UnicodeEncodeError after doing its work.
- Cloning an empty GitHub repo yields the server's default branch (`main` here); `git init` on this
  machine yields `master` (system `init.defaultBranch`). Read the branch, don't infer it.
- Long heredocs fail to parse in this harness; write multi-line files with the Write/Edit tools and
  pass commit messages via `-F file`. A `grep` inside an `&&` chain is not a guard.
- The Bash tool's working directory persists between calls (a `cd` in one call moves later calls);
  use absolute paths.
- Deep `find` over the home dir hangs on OneDrive reparse points; bound searches.
- Publisher pages (ScienceDirect, MDPI, Taylor & Francis, academia.edu) return 403 to tool
  fetches; Crossref, arXiv, the GitHub API, and institutional repositories work.
- `git symbolic-ref refs/remotes/origin/HEAD` may fail on a working copy cloned from an empty
  remote until the first push and fetch; fallback `git remote show origin`.
- elan resolves the Lean toolchain from the working directory; `cd` into the Lean project before
  `lake build`. elan's `stable` may resolve to a version that is not installed.
- Reachability is per site, not per tool: openai.com serves `curl` (browser UA) but 403s the
  WebFetch tool; ias.edu 403s both (Cloudflare interstitial); Zenodo 403s its REST API to curl but
  serves record pages and `?download=1` file links. Try the other tool before recording a failure.
- `python script.py | grep …` masks the script's exit code; never put a verifier behind a pipe
  when its verdict gates a commit. Run it alone, then act on `$?`.
- Python needs `-X utf8` (or `PYTHONIOENCODING=utf-8`) to print `§`, `→`, `π` on this console.
- The GitHub trees API truncates large repos (`"truncated": true`); use per-directory contents
  calls for counts. The contents API returns base64 content only for files under 1 MB.
- Hashing every recent file under the home dir (`find … | xargs sha256sum`) blows a 60–90 s
  timeout even at maxdepth 4 with OneDrive/AppData excluded. Search by name only; verify a ferried
  file by its digest once a candidate (or a paste) is in hand — the digest is the identity.
- The console clock is local but `date -u` and web access times are UTC; a session that starts late
  on day D records retrievals dated D+1 UTC. Write the session date and access times separately.
- A `lake build` that imports all of Mathlib prints nothing for minutes when piped through `tail`
  (first build of a one-module project took ~3 min after the cache); run it in the background
  with a log and read the log with `tr '\r' '\n'` — the cache tool writes progress with CRs.
  The full cold Lean setup (toolchain + nine clones + ~8,900 cache files + build) took 14 min.
- Never `du` the `lean/.lake` tree while lake is writing it; it blew the 120 s tool timeout.
- `cargo fmt` rewrites freshly written Rust; the gate runs `fmt --check`, so commit the formatted
  form. Rust byte-string literals cannot hold `\u{…}` escapes — use `"…".as_bytes()`.
- For arXiv papers, `arxiv.org/html/<id>` is the better text source for sentence-level checks;
  poppler's `pdftotext` (at `C:\Users\LEET\tools\poppler-26.02.0\poppler-26.02.0\Library\bin`)
  splits sentences across columns and figures. Read licence, version count and dates from the
  `arxiv.org/abs/<id>` `<meta name="citation_*">` tags; ACL Anthology pages put `content=` before
  `name=` in their meta tags, so match either attribute order.
```
Judgment on the recalled memory: no recalled memory holds program state in SD15's enumeration
(phase, gate status, CLAIM or FLAW statuses, file contents as state, commit SHAs as markers, next
actions, escalations). Two lines of the memory file are flagged for Al's audit as borderline: the
"Hard invariants" paragraph paraphrases three constitution rules (§0.3, §0.4, SD17), and one hazard
line says "the gate runs `fmt --check`" (the Q6 ruling). Both are constitution and design facts, not
state; the executor judges them D1-legal (the siblings' convention for hazard content). If Al rules
either line program state, this exam downgrades to CONDITIONAL PASS (ESCALATIONS).
PHASE / GATE: S0 — FOUNDATION AND CITATION VERIFICATION / Gate S0 open on the cold-start exam only,
which this session is. Result: **PASS** (below). Gate S0 is CLOSED at this commit; the phase
advances to S1 — GROUND-TRUTH CORPUS.
PLAN NOTE: one thing — the exam that STATE's NEXT-SINGLE-ACTION scheduled. It survived ORIENT
unchanged; the inherited procedure (1)–(4) was followed as written. The two reconciliations the exam
surfaced were made in this commit as the session's first task (§4.1).
ADVANCED:
- ORIENT (§4.1): `git rev-parse --show-toplevel` = `C:/Users/LEET/Syndeom`; `origin` =
  `https://github.com/Quigles1337/Syndeom.git`; `git symbolic-ref refs/remotes/origin/HEAD` →
  `refs/remotes/origin/main` (no fallback needed); HEAD = `origin/main` = `2b7c573`; worktree
  clean. Read in order: the constitution (463 lines), STATE, HANDOFF Session 5, CLAIMS in full,
  FLAWS header and entry list, ADR-0000 addendum, ADR-0001 header, section list and Amendments,
  ADR-0002 in full, every SOURCES state line and S-0008's crate enumeration, `scripts/check.sh`,
  `scripts/check_registers.py`, `.github/workflows/check.yml`, the pin files on disk, the six
  directory READMEs, `.gitignore`, `.gitattributes`, the SIBLING-MAP section list.
- Evidence run: `bash scripts/check.sh` at `2b7c573`, 03:45:45Z–03:46:17Z (warm `lean/.lake`
  from Session 4): `Build completed successfully (8926 jobs)`; audit `'syndeom_golden_nat_le_sq'
  depends on axioms: [propext]`; `cargo fmt --all --check` clean; `cargo clippy -D warnings`
  clean; `cargo test` 23 passed, 0 failed; `git ls-files --eol` 36 `i/lf` + 9 `i/-text` (the
  PDFs), no `i/crlf` or `i/mixed`; `check_registers.py`: 45 tracked files CR/BOM/trailing-LF
  clean; Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 18/18 defined; `RESULT: ALL PASS`;
  `GATE: GREEN`. The advisory CI run for `2b7c573` (run 38342304442) completed `success` at
  03:46:38Z.
- Byte checks beyond the gate: `PROGRAM/SWEEP-2026-10.md` 41,060 bytes, SHA-256 `f4bfb2ad…b03e`
  (= the digest STATE and CLAIM-0006 record; banner on line 3); `lean/Syndeom/Fixtures/GoldenB.lean`
  SHA-256 `c568a981…458e` (= ADR-0001 D8, ADR-0002 D3); every one of the nine PDFs under
  `sources/` hashes to a digest present in SOURCES.md, byte for byte, as Session 5's DETERMINISM
  CHECK claimed.
- THE EXAM — program state re-derived in one pass from the repo (what was read → what it
  establishes), then compared against STATE.md:
  - Phase S0; Gate S0 criteria S0.1–S0.6 done. S0.1: CLAIM-0001..0005 and 0007 carry
    UNKNOWN → VERIFIED transitions dated Session 2 with S-0001..S-0006 (S-0004, S-0005 INGESTED);
    the S5 contact corpus location `github.com/openai/math` is in CLAIM-0001's evidence. S0.2: the
    sweep file, digest recomputed, banner present. S0.3: ADR-0000 addendum; commit `c803c96`;
    the constitution header says ratified Session 2. S0.4: ADR-0001 accepted, ratified Session 4,
    Amendments recorded (`d3e5842`, `9137c64`). S0.5: FLAWS 0.1.0, fourteen `## FLAW-` headings,
    ratified. S0.6: ADR-0002 D1–D8; the pin files on disk agree with it (below); six directory
    READMEs; gate and CI present; `132788c`. CLAIM-0006 examined: `2b7c573`. Cold-start: this
    session.
  - Registers. CLAIM statuses 0001..0007: VERIFIED ×5, CONDITIONAL (0006), VERIFIED (0007,
    existence only). CLAIM-0006 is CONDITIONAL because a claim of absence cannot become VERIFIED
    by retrieval; it is narrowed to the joint flaw-class × proof-length breakdown above word-problem
    level (and any joint breakdown at research level), bounded by S-0009..S-0018, FALSIFIED by one
    counterexample; the scoped novelty statement lifts §0.6 only in that scope. SOURCES: 18
    entries; INGESTED S-0004, S-0005, S-0010..S-0016 (the nine PDFs on disk); the rest VERIFIED
    (S-0006 existence only; S-0009 and S-0018 digest-only; S-0017 a repository).
  - Pins. `lean/lean-toolchain` = `leanprover/lean4:v4.34.1`; `lean/lakefile.toml` requires
    mathlib at `d13f23b723b8a846827a245b89c10fc7d3f11612`; `lake-manifest.json` pins nine packages;
    `rust-toolchain.toml` channel `1.91.0`, components clippy + rustfmt; `engine/Cargo.toml` is a
    workspace (resolver 3, member `manifest`). Installed and matching: elan 4.2.3, Lake 5.0.0 /
    Lean 4.34.1, rustc and cargo 1.91.0, Python 3.11.9.
  - Manifest schema `syndeom-manifest/1` (ADR-0001): formats `syndeom-informal-md/1`,
    `syndeom-lean/1`, `raw/1` (archival); ids `<generator.id>-v<version>-<config_hash[0:12]>` with
    `-s<seed>` for synthetic; RFC 8785 (S-0007) + SHA-256; FRIEDMAN ADR-0004 arithmetic by
    reference; `ground_truth: null` for contact; the derived-canonical-artifact convention in D4.
    Each read in the ADR's own D2–D6 text.
  - Standing escalations: Q3 LICENSE deferred (ADR-0000 addendum row Q3; HANDOFF Session 5
    records "deferred pending Al's explicit decision text"); referee participants are Al's (SD13).
    Nothing else open. Next action before this session: the cold-start exam.
- Comparison: STATE.md agrees on every item above. Two precision findings, neither
  unreconstructable: (1) STATE's "Build status" still named the Session-4 run as the last GREEN,
  while HANDOFF Session 5 records a GREEN run at `132788c`; both are superseded by this session's
  run at `2b7c573`, now recorded, with a standing instruction to update the line at every recorded
  GREEN run. (2) "24 crates" (STATE, ADR-0002 D4, S-0008) counts the dependencies S-0008
  enumerates (5 direct, 19 transitive); `engine/Cargo.lock` holds 25 `[[package]]` entries because
  the workspace crate `syndeom-manifest` is the 25th. Clarified in STATE's toolchain line; the
  ADR and the register are correct as worded and are not edited.
- EXAM RESULT: **PASS.** Every item was re-derived from the repo alone, in one pass, without
  guessing; the gate is GREEN at the tip and again on the tree this commit lands; the two findings
  are staleness and precision, reconciled here. Gate S0 CLOSED at this commit (`[S0][INFRA] gate
  S0 cold-start exam: PASS at 2b7c573`). NEXT-SINGLE-ACTION = S1.1 (below).
- STATE: Phase → S1; the S0 table closed with this exam's evidence and retained; a Gate S1 table
  opened (S1.1–S1.4, licences, green, cold-start — all open); Build status and toolchain lines
  updated; ADR-0003 reserved for S1.1; the sweep's unretrieved runners-up carried as a note, not
  an action.
BELIEF DELTAS:
- The repo is sufficient, now tested rather than asserted. Session-start me had the §6 prompt and
  nothing else; constitution + STATE + HANDOFF + registers + ADRs reconstructed every pin, status
  and escalation, and the gate confirmed every build claim. Evidence: the exam list above.
  Consequence: §0.8 has one passed test, at S0; each later gate repeats it.
- The gate costs ~30 s with a warm cache against the ~15 min STATE warned of. Evidence: the log
  timestamps 03:45:45Z–03:46:17Z. Consequence: STATE now gives both numbers, so a successor can
  tell a cache download from a hang.
- SD15's "reproduce every recalled memory verbatim" meets "this repo is public" when the memory
  index also lists unrelated private programs; the constitution does not name the case. Resolution
  used: verbatim for every line naming SYNDEOM or a sibling (grep-checked), count-only for the
  rest, stated openly. Evidence: the first entry of this block. Consequence: flagged to Al; if the
  rule should say so, that is an SD15 amendment by ADR.
- A cold session's first useful act is to distrust the freshest-looking line: STATE's Build status
  lagged HANDOFF by a session. Evidence: finding (1). Consequence: the standing instruction in
  STATE.
CONFUSIONS:
- The crate count (24 vs 25) cost one extra read of S-0008's enumeration; the doc fix is the one
  clause in STATE's toolchain line.
- STATE's Build-status line lagged HANDOFF by one session; the doc fix is the standing instruction
  in that line.
- Nothing else misunderstood; the §4.1 reading order was sufficient as written.
HONESTY CHECK: no register entry changed status this session; CLAIMS, SOURCES and FLAWS are
untouched. CLAIM-0006's CONDITIONAL was re-examined against its evidence: status, bounding set and
the "what this examination cannot establish" sentence are all present; nothing to downgrade. The
exam result is PASS rather than CONDITIONAL PASS on the executor's judgment of the two borderline
memory lines, stated in the first entry so Al can overrule it. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
was opened; S-0001 and S-0006 were read only as register entries; no design decision was taken
this session (no ADR, detector, generator or threshold). Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The only numbers produced are the gate's,
regenerable by `bash scripts/check.sh` at `2b7c573`, and the digests above, regenerable by
`sha256sum`. Nothing to quarantine.
MANIFEST CHECK: no artifact was touched or created; `corpora/` and `groundtruth/` hold READMEs
only. "Proven" was applied to nothing (Golden B remains SD2(i)–(ii) only, per ADR-0002 D8).
Nothing synthetic labeled real.
DRIFT CHECK: no statement about any real manuscript (SD3); no understanding claims (SD17); no
detectors exist, so no detector-shopping; no referee contact (SD13). The session did the scheduled
exam and the two reconciliations it surfaced, nothing else.
NEXT-SINGLE-ACTION: S1.1 — write `PROGRAM/DECISIONS/ADR-0003.md`, the errata-and-retraction
harvest procedure for `groundtruth/` (procedure only; no item is ingested in that session). It
decides: (a) where candidate items come from — published proofs whose errata or retraction notices
are themselves public documents, the notice being the status document SD12 requires — and how a
candidate is admitted or rejected; (b) the per-item record: original, notice, the flaw's location,
its FLAWS.md class with the mapping argued in writing, derivation `documented-by-source`; (c)
licence handling per SD16 — a stored copy only when the licence permits, otherwise digest-only
with `storage: external` (ADR-0001 D2) — ambiguity escalated, never ingested; (d) the `raw/1`
parent plus the derived `syndeom-informal-md/1` canonical artifact per ADR-0001 D4 (iii), with the
location re-expression argued per item; (e) the shape of the S1.4 coverage table (flaw class ×
proof length, zero-coverage classes named). Read first, from the stored PDF: S-0015's
ArxivMathGradingBench pipeline, as the precedent for harvesting human-authored papers with
referee-identified errors. Commit `[S1][ADR-0003] decide the errata-harvest procedure`; the first
harvested items are the action after.
ESCALATIONS:
- **Q3 repository LICENSE** — unchanged: deferred pending Al's explicit decision text (an SPDX
  identifier, or "deferred pending <reason>"). Due before S5.4; nothing in S1 depends on it.
- **SD15 audit request (new):** Al, please confirm or overrule the executor's judgment that the two
  flagged memory lines (the "Hard invariants" paraphrase; "the gate runs `fmt --check`") are not
  program state, and whether withholding the 85 unrelated index lines by count is acceptable for a
  public repo. If overruled: this exam becomes CONDITIONAL PASS, the memory is trimmed in a
  non-exam session, and the exam is re-run cold before Gate S1 — Gate S0's closure would then be
  provisional until that re-run. If the count-only convention should be the rule, it is an SD15
  amendment by ADR.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: gate reached — Gate S0 closed by the cold-start exam. Repo green on the committed tree
(`bash scripts/check.sh` re-run after these edits, before the commit); this commit pushes to
`origin/main`.

SESSION: 2026-10-13 UTC @ b1b17d2 (Session 7 — Al's rulings of 2026-10-13 recorded; the commit
adding this entry lands on top of b1b17d2)
LAUNCH CONTEXT (SD15; not an exam session): launched with the §6 Session-N resume prompt plus Al's
instructions for the session — four rulings (R1 LICENSE, R2 the S3.2 amendment, R3 an advisory-note
ferry, R4 a standing ruling for S2), one jurisdiction note, a leakage instruction, and the order
that NEXT stays S1.1. The harness loaded the executor's memory index at start; the SYNDEOM memory
file holds the repo location and machine hazards only (SD15). Memory written this session: hazards
only (three retrieval/ferry hazards; no program state), recorded below.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open; no S1 criterion advanced — this was
bookkeeping of decisions already made, and it built nothing. Gate S0 closed (Session 6) — unchanged.
PLAN NOTE: one thing — record Al's rulings of 2026-10-13 where STATE and the ADR-0000 addendum say
rulings belong, as Session 2 recorded Q1–Q17. The inherited NEXT-SINGLE-ACTION (S1.1, ADR-0003) was
not this session's task; ORIENT did not invalidate it; it stands unchanged at the end.
ADVANCED:
- ORIENT (§4.1): toplevel `C:/Users/LEET/Syndeom`; `origin` =
  `https://github.com/Quigles1337/Syndeom.git`; `git symbolic-ref refs/remotes/origin/HEAD` →
  `refs/remotes/origin/main`; HEAD = `origin/main` = `b1b17d2` (after `git fetch`); worktree clean,
  stash empty. Read: the constitution (465 lines), STATE, HANDOFF Session 6, CLAIMS headings and
  CLAIM-0006 in full, FLAWS header, ADR-0000 in full, ADR-0001 header and Amendments, ADR-0002
  header, SOURCES header with S-0007 and S-0018 as format exemplars, `scripts/check.sh`,
  `scripts/check_registers.py`, `.github/workflows/check.yml`, `engine/Cargo.toml`,
  `engine/manifest/Cargo.toml`, `rust-toolchain.toml`, `.gitattributes`, `.gitignore`,
  `analysis/README.md`. The sibling trees were read for licence practice only (root listings,
  READMEs, Cargo manifests, STATE licence lines) and not touched.
- ORIENT discrepancy: the file R3 names, `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md`, was not
  in the tree, not in `origin` (fetched), not in the stash, and not found by a bounded name search
  of `C:\Users\LEET` (depth 3, OneDrive/AppData/build dirs pruned). Al then pasted the note's text
  into the session mid-turn. Transcribed exactly as received into the scratchpad and hashed:
  25,652 bytes, 0 CR, SHA-256 `0ee1eda0ab0ef11ffe70c52a1118ca1235b1306fe5895fcf75914c46081baa80`;
  the ruling states 27,238 bytes and `f3e40b684a8b437fa4f0ba9d059d23d6ae61ce88ce8bca84ceb49fd7c181fa2c`.
  Mismatch, with visible cause: the chat rendering flattened the three-column table to spaced text,
  dropped the `=` from four displayed equations (the §6 field equation, the boxed static reduction,
  the difference-field equation, the cost ratio), and stripped heading and bullet markers. Per R3
  ("on mismatch, stop and escalate") the file was not committed and nothing from the paste entered
  any register; the paste was read once, for the LEAKAGE CHECK below, and otherwise left alone.
- R1 — LICENSE (executed): the Apache Software Foundation host refused connections from both tools
  all session (curl ×3 on two hostnames, `Could not connect to server` after ~21 s each; the fetch
  tool `connect ECONNREFUSED 151.101.2.132:443`; DNS fine). Retrieved instead from the SPDX License
  List for the identifier Al named — `https://spdx.org/licenses/Apache-2.0.txt`, 22:38:32Z, 10,279
  bytes, SHA-256 `c274f803…366c` — and cross-checked against an independent copy from the GitHub
  Licenses API (22:44:34Z, 11,357 bytes, `c71d239d…0ab4`): identical after whitespace
  normalization. Committed `LICENSE` = SPDX bytes + one trailing LF (gate rule): 10,280 bytes,
  `074e6e32…15ff`. README.md gained a Licence section stating Al's boundary and pointing to
  SOURCES.md; `license = "Apache-2.0"` added to `engine/manifest/Cargo.toml` `[package]` (the only
  package table — `engine/Cargo.toml` is a bare `[workspace]`). No NOTICE file: KOBER (`f94bec2`)
  and FRIEDMAN (`7c3e5dd`) have no LICENSE, NOTICE, README licence section or Cargo `license`
  field. S-0019 records every URL, time and digest. ADR-0000: Q3 row annotated resolved; Addendum 2
  R1. STATE: Q3 escalation closed; default-branch line updated.
- R2 — S3.2 amendment (executed): `PROGRAM/DECISIONS/ADR-0004.md` written — D1–D7 are Al's (i)–(vii)
  verbatim in substance; *Method* fixes the N_min derivation from the null variance of U (S-0020
  eqs. (2) and (5)), reproduces that variance by an independent covariance count, gives the
  normal-approximation closed form as the S3 calibration ADR's starting value, and lists the four
  steps that ADR performs before any curve (analytic n; simulation against the real bootstrap with
  the real tie structure; type-I at the band edges and coverage at 0.5; effective-N at cluster
  level). **No number computed** (deferred as ruled). §2 S3.2 replaced to match, citing the ADR;
  Amendments line in the constitution's header and footer; version string unchanged (no rule).
  STATE: ADR-0003 stays reserved for S1.1, ADR-0005 next free, S3 calibration ADR obligations
  listed. S-0020: Project Euclid served an Incapsula challenge to curl (three URLs) and empty
  content to the fetch tool; the DOI resolves (302 observed); Crossref confirms the record; the
  publisher PDF was retrieved from the Internet Archive's 2026-09-27 capture (`id_` raw route),
  779,952 bytes, `7647e2a2…0788`, rendered to images (no text layer) and read: p. 50 title/authors,
  p. 51 eq. (2) E(U) = nm/2, p. 55 eq. (5) E[(U − nm/2)²] = nm(n+m+1)/12. VERIFIED, digest-only
  (licence unstated on the copy).
- R3 — advisory ferry (not executed; escalated): recorded in ADR-0000 Addendum 2 R3 with the stated
  path, size, digest and provenance header summary, the non-execution and its evidence; STATE
  standing escalation says exactly what is needed (the file at the path, or a path on this machine
  holding the exact bytes). The program it names is named with its owner's authorization (Al,
  2026-10-13) — recorded in Addendum 2 and STATE.
- R4 — S2 seed family (recorded): STATE standing rulings; Addendum 2 R4. Candidate hand-authored
  golden-fixture family for S2.4 beside the FLAWS.md examples; a pipeline test, not calibration
  data; licence line decided at S2 under the R1 boundary; ADR at S2. Nothing built.
- Jurisdiction: one line of record in Addendum 2 — the note's §6/§7 finding is a DARQ MNEMOSYNE
  change-control item under that program's governance; SYNDEOM records nothing about it beyond
  what the note's header says. Nothing about the finding itself is written anywhere here.
- Evidence run (§4.1): `bash scripts/check.sh` on the fully edited tree, 22:54:28Z–23:04:29Z:
  `Build completed successfully (8926 jobs)`; audit `'syndeom_golden_nat_le_sq' depends on axioms:
  [propext]`; `cargo fmt --all --check` clean; `cargo clippy -D warnings` clean; `cargo test` 23
  passed, 0 failed; `git ls-files --eol` no `i/crlf` or `i/mixed`; `check_registers.py` 45 tracked
  files CR/BOM/trailing-LF clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, **S- 20/20** (S-0019
  and S-0020 new); `RESULT: ALL PASS`; `GATE: GREEN`. Ten minutes rather than Session 6's thirty
  seconds: the axiom audit loaded Mathlib's oleans from a cold OS file cache after a reboot
  (lean.exe above 1.4 GB) with `lean/.lake` itself warm — not a hang; STATE's timing note now says
  so. After staging, 47 tracked files: 38 `i/lf` + 9 `i/-text` (LICENSE and ADR-0004 added; both
  byte-checked LF, no BOM, trailing LF). The gate is re-run on the final tree, after this entry and
  the STATE lines are in, before the commit (STOP-REASON).
- Memory (SD15, hazards only): four hazard bullets appended to the SYNDEOM memory file — paste
  ferries never match a digest; apache.org unreachable / SPDX and GitHub licence copies; Project
  Euclid's bot challenge and the Wayback `id_` route plus image-reading of textless scans; the
  gate's cold-OS-cache timing. Re-read after writing: the file still names no phase, gate status,
  claim status, commit, next action or escalation (grep for those tokens matched only the policy
  sentence that forbids them).
BELIEF DELTAS:
- A ferried file's identity is its digest, and a chat paste cannot carry it: the harness rendering
  of a Markdown file flattens tables, drops `=` lines from display math and strips heading markers,
  so a paste of a 27,238-byte file arrived as 25,652 bytes. Evidence: the two digests above.
  Consequence: a ferry is a path to bytes on disk, never a paste; STATE's escalation asks for that;
  the hazard is in memory.
- "Canonical source" can be unreachable while the document is available from the authority for the
  identifier Al named: the SPDX License List is the SPDX text for `Apache-2.0`, and an independent
  copy cross-checks it. Evidence: S-0019. Consequence: the record says plainly that the committed
  bytes are not from the ASF host, and a later session diffs them when the host answers.
- The Internet Archive's `id_` raw-capture route serves publisher PDFs that the publisher's own host
  hides behind a bot challenge, and the copy is identified by its digest like any other retrieval.
  Evidence: S-0020. Consequence: hazard in memory; the licence still governs (digest-only).
- Mann and Whitney 1947 is a scan without a text layer: `pdftotext` returned only the JSTOR footer;
  the content was read from rendered page images. Evidence: `pdfinfo` 11 pages, 3 lines of text.
  Consequence: for old scans, render and read; do not conclude "no content" from empty text.
- The equivalence framing makes the §0.5(b) negative result more expensive and more honest at once:
  a cell must be large enough to *show* chance, and dependent variants of one seed theorem cannot
  manufacture that power (ADR-0004 D6). Consequence: D7's underpowered cells are a pre-declared
  finding, not a defect.
CONFUSIONS:
- Session-start me expected the advisory file on disk because the ruling said "Al has placed"; it
  was not there. Cost: a bounded search, a paste transcription and a hash. Doc fix: STATE's
  escalation names the two acceptable forms of the ferry; the memory hazard says a paste never
  matches a digest.
- Whether to split the session into two commits (the ADR-0004 set and the rulings set): the STATE
  and SOURCES edits interleave, and Session 2's `[S0][ADR-0000] ratify the constitution and record
  Al's rulings` is the precedent for one rulings commit. One commit, tagged ADR-0000, naming
  ADR-0004 in the summary. Doc fix: none; the precedent is in `git log`.
- `pdftotext` on the archived PDF produced three lines; a minute was spent before rendering pages.
  Doc fix: the BELIEF DELTA above and a memory hazard.
HONESTY CHECK: no CLAIM changed status; CLAIMS.md and FLAWS.md untouched. New SOURCES entries:
S-0019 INGESTED (the licence text, with the ASF-unreachable caveat stated in the entry) and S-0020
VERIFIED (digest-only; licence unstated on the copy, so no copy committed). CLAIM-0006's scoped
novelty statement was re-read against the amended S3.2 ("a power-floored fooling length") and
remains true. R3 is recorded as stopped, not as done. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. (1) `contact/` holds only its README; nothing from
`openai/math` was opened; S-0001 and S-0006 were not reread. (2) The advisory paste — unverified
text, read once — was searched for contact-corpus terms (`contact`, `openai/math`, `6 October`,
`withdraw`, `retraction`, S-0001, S-0006, CLAIM-0001, CLAIM-0007): no match; read in full, it cites
no contact-corpus behaviour; its examples are one private theorem's algebraic variants. This
reading is repeated on the verified file when it arrives. (3) ADR-0004's rationale cites S-0020
and its own derivation; the advisory note is named only as the provenance of the prompt for Al's
ruling, never as rationale. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. Every digest above is regenerable by
`sha256sum` over the recorded URL at the recorded route (the Wayback capture is timestamped; the
SPDX file carries an ETag); the gate's numbers by `bash scripts/check.sh`. ADR-0004 computes no
number by design. Nothing to quarantine.
MANIFEST CHECK: no artifact touched or created; `corpora/` and `groundtruth/` hold READMEs only.
"Proven" applied to nothing. Nothing synthetic labeled real. R4 names a *candidate* fixture family
and nothing was authored for it.
DRIFT CHECK: no statement about any real manuscript (SD3) — the Mnemosyne theorem appears only in
Al's ruling text as a candidate fixture source, with no assessment of it; no understanding claims
(SD17); no detectors exist, so no detector-shopping, and ADR-0004 was ruled before any curve; no
referee contact (SD13). The jurisdiction line records nothing about the Mnemosyne finding. Scope:
the four rulings and nothing else; R3 stopped at the digest, as ruled.
NEXT-SINGLE-ACTION: S1.1 — write `PROGRAM/DECISIONS/ADR-0003.md`, the errata-and-retraction
harvest procedure for `groundtruth/` (procedure only; no item is ingested in that session),
exactly as STATE.md states it: decide (a) where candidate items come from and how one is admitted
or rejected; (b) the per-item record (original, notice, the flaw's location, its FLAWS.md class
argued in writing, derivation `documented-by-source`, source and licence per SD16); (c) licence
handling — stored copy only when the licence permits, else digest-only with `storage: external`
(ADR-0001 D2), ambiguity escalated; (d) the `raw/1` parent plus the derived `syndeom-informal-md/1`
canonical artifact per ADR-0001 D4 (iii); (e) the shape of the S1.4 coverage table. Read first,
from the stored PDF under `sources/`: S-0015's ArxivMathGradingBench pipeline. Commit
`[S1][ADR-0003] decide the errata-harvest procedure`. Unchanged from Session 6.
ESCALATIONS:
- **Advisory ferry (R3) — awaiting the file.** Al: place `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md`
  (27,238 bytes, LF, SHA-256 `f3e40b68…fa2c`) in the tree, or name a path on this machine holding
  the exact bytes. The next session verifies the digest, commits it as ferried with the body
  unedited, repeats the leakage reading on the verified file, and records placement in HANDOFF.
  The paste in this session is not usable for that and was not committed.
- **SD15 audit request (Session 6)** — unchanged; still awaiting Al's confirm-or-overrule.
- **Note, not a blocker:** when `www.apache.org` answers, diff `LICENSE` against
  `https://www.apache.org/licenses/LICENSE-2.0.txt` after whitespace normalization (S-0019 gives the
  command) and record the ASF digest in S-0019.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: natural boundary — the session's one task is recorded as far as it can be executed;
R3 is a blocker requiring Al (the file). Repo green on the committed tree (gate re-run after every
edit, before the commit); this commit pushes to `origin/main`.
