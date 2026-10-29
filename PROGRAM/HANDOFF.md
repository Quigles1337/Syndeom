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

SESSION: 2026-10-13 UTC @ 06df7db (Session 8 — R3 closed: the advisory note ferried, verified and
committed; the commit adding this entry lands on top of 06df7db)
LAUNCH CONTEXT (SD15; not an exam session): the same conversation as Session 7, continued on Al's
message that the file R3 names is in place, "written byte-exact by Claude on 2026-10-13 and
hash-verified; untracked on top of 06df7db", with the order to close R3 first and then proceed to
NEXT (S1.1). Not cold. No memory written this session.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open; no criterion advanced. Gate S0 closed
(Session 6) — unchanged.
PLAN NOTE: one thing — close R3: verify the digest, commit the file as ferried with its body
unedited, repeat the leakage reading on the verified bytes, record the closure. S1.1 follows as its
own session (Session 9) with its own commit and HANDOFF entry, under the §4.3 one-thing rule; both
were ordered by Al in one message.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `06df7db`; exactly one untracked file, the one R3 names.
  `wc -c` 27,238; `sha256sum` `f3e40b684a8b437fa4f0ba9d059d23d6ae61ce88ce8bca84ceb49fd7c181fa2c`
  = Al's stated digest; 0 CR, no BOM, trailing LF, 520 lines; `.gitattributes` applies
  `text=auto eol=lf`; after staging `git ls-files --eol` shows `i/lf w/lf`, blob `db92abb`. The
  committed blob is therefore exactly the verified bytes; nothing in the body was edited.
- Read in full (520 lines). The header says what R3 says it says: the SD11 banner ("ADVISORY, NOT
  EVIDENCE … Nothing in this file enters any SYNDEOM register"); produced by ChatGPT at Al's
  request on 2026-10-13 and relayed verbatim; verified by Claude on 2026-10-13 against both local
  clones; rights — the DARQ MNEMOSYNE material is proprietary and not part of the Apache-2.0
  release (the R1 boundary), the program named with its owner's authorization (Al, 2026-10-13);
  disposition — the §5 equivalence point taken up by R2, the cases a candidate S2 seed family
  (R4), nothing else adopted; "Not found" — the two counterexamples are ChatGPT's own, not the
  repository's, with the arithmetic checked by hand; the §6/§7 scope item is DARQ MNEMOSYNE change
  control, not a SYNDEOM item. Recorded here that the file was placed and verified, and that the
  program it names is named with its program owner's authorization (Al, 2026-10-13).
- Records: ADR-0000 Addendum 2 R3 row annotated closed; STATE gains an "Advisory note" bullet
  beside the founding-sweep bullet (path, bytes, digest, banner, what it does and does not do
  here) and the R3 escalation is marked resolved, with the standing form for any future ferry (a
  path to bytes on disk, never a paste).
- Register check with the file staged (`check_registers.py`, run directly before the gate): 48
  tracked files CR/BOM/trailing-LF clean; the file's ID tokens (FLAW-0001/0008/0011/0013, Q3,
  SD11/SD12/SD13) all resolve; S- 20/20; `RESULT: ALL PASS`.
- Evidence run (§4.1): `bash scripts/check.sh` with the file and the closure edits staged,
  23:21:33Z–23:31:44Z: `Build completed successfully (8926 jobs)`; audit
  `'syndeom_golden_nat_le_sq' depends on axioms: [propext]`; fmt and clippy clean; `cargo test`
  23 passed, 0 failed; `git ls-files --eol` no `i/crlf` or `i/mixed`; `check_registers.py` 48
  tracked files clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 20/20; `RESULT: ALL PASS`;
  `GATE: GREEN`. Ten minutes again — the axiom audit's Mathlib load was cold a second time
  (STATE's timing note already covers it). Re-run on the final tree after this entry, before the
  commit (STOP-REASON).
BELIEF DELTAS:
- A ferry closes in one short session once the bytes are on disk: ORIENT found the stated digest
  on the first hash. Evidence: above. Consequence: STATE's escalation form ("the file at the path,
  or a path to the exact bytes") is the standard ask for every future ferry; it is now written
  into the resolved escalation.
- The Session-7 paste and the Session-8 file differ by 1,586 bytes and all of the Markdown
  structure, and carry the same words. Consequence: the Session-7 leakage reading was of the right
  text, but the record of that reading is made only now, on verified bytes — the digest, not the
  reading, is what makes it a record.
CONFUSIONS: none new. One decision explained rather than confused: Session 3 took Al's "ferry,
then NEXT" order as one session with two commits and one HANDOFF entry; this time the R3 closure
and S1.1 are two sessions, so that the closure is a complete record with its own commit even if
S1.1 were cut short by the context budget. Either form is within §4; this one is the safer record.
HONESTY CHECK: no register entry changed status; CLAIMS, SOURCES and FLAWS untouched. The note is
not a SOURCES entry — like the founding sweep it is a bannered PROGRAM document, a ruling-of-record
artifact, and nothing is cited from it. Downgrades: none.
LEAKAGE CHECK: checked, not assumed, on the verified file. Searched for `contact/`, "contact
corpus", `openai/math`, "6 October", "October 6", "withdraw", "retract", any S-NNNN and any
CLAIM-NNNN: no match. Read in full: every worked example is the private Mnemosyne static-reduction
theorem and its algebraic variants; the three external references are the Lean reference manual's
page on validating proofs, an equivalence-testing primer and a neural-network calibration paper —
none is a contact corpus and none is cited by SYNDEOM. `contact/` holds only its README. No design
decision was taken this session. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The file's identity is its digest,
regenerable by `sha256sum`; the gate's numbers by `bash scripts/check.sh`. Nothing to quarantine.
MANIFEST CHECK: no artifact touched or created; the note is a PROGRAM document, not an artifact,
and carries no manifest (as the sweep does not). Nothing synthetic labeled real. "Proven" applied
to nothing.
DRIFT CHECK: the note discusses a real private theorem and a real specification; SYNDEOM records
no assessment of either — the note's assessments are the note's, under its banner, and the only
SYNDEOM-side statement about them is the jurisdiction line already in Addendum 2 (SD3). No
understanding claims (SD17). No detectors exist. No referee contact (SD13). Nothing was done
beyond the closure.
NEXT-SINGLE-ACTION: S1.1 — write `PROGRAM/DECISIONS/ADR-0003.md`, the errata-and-retraction
harvest procedure for `groundtruth/`, exactly as STATE.md states it (procedure only; no item
ingested). Begun in Session 9 on Al's order in the same message.
ESCALATIONS:
- SD15 audit request (Session 6) — unchanged; still awaiting Al's confirm-or-overrule.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers (S-0019).
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: natural boundary — R3 closed. Repo green on the committed tree (gate run with every
edit staged, before the commit); this commit pushes to `origin/main`. Session 9 follows at once.

SESSION: 2026-10-13 UTC @ d9f53de (Session 9 — S1.1: ADR-0003, the errata-harvest procedure; the
commit adding this entry lands on top of d9f53de)
LAUNCH CONTEXT (SD15; not an exam session): the same conversation as Sessions 7 and 8, continued
on Al's order in the Session-8 message: "then proceed to NEXT-SINGLE-ACTION as STATE.md states it
(S1.1, ADR-0003)". Not cold. Memory written this session: hazards only (three API-route lines),
recorded under ADVANCED.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open; **S1.1 procedure decided (ADR-0003); no
item ingested** (as the action specified). Gate S0 closed (Session 6) — unchanged.
PLAN NOTE: the inherited NEXT-SINGLE-ACTION, executed as STATE stated it: S-0015's pipeline read
first from the stored PDF; the ADR decides (a) sources and admission, (b) the per-item record, (c)
licence handling, (d) the artifact form, (e) the S1.4 table; commit
`[S1][ADR-0003] decide the errata-harvest procedure`. One deviation from the Session-6 text, with
its reason: NEXT is set to the ingest producer, not to "the first harvested items", because
ADR-0001 D4 requires every ground-truth manifest's `generator` to name the `engine/` producer
that wrote it, so no item can exist before the producer does (ADR-0003 D6, Consequences).
ADVANCED:
- ORIENT: HEAD = `origin/main` = `d9f53de` after the Session-8 push; worktree clean. Read for
  this action: the constitution's S1 text and SD12/SD16; STATE's NEXT block; ADR-0001 D2, D3, D4
  in full (storage enum, `raw/1`, the ingest-producer fields, the derived-canonical-artifact
  convention); FLAWS.md in full (all fourteen entries, vocabulary, summary table); the
  `groundtruth/`, `corpora/`, `contact/`, `lab/` READMEs; `engine/manifest/src/lib.rs`'s public
  surface (the types `Manifest`, `Generator`, `Storage`, `RawLocation`, `GroundTruth`, `Flaw`,
  and the functions `content_hash`, `config_hash`, `artifact_id` — what the producer will use).
- **S-0015 read first** (stored PDF `sources/arxiv-2605.20531v3-pseudo-formalization.pdf`,
  digest re-verified `a6c93783…7439`; text extracted; §4.2, Table 1, §5 opening, §7.1, Appendix
  D): the ArxivMathGradingBench pipeline is (1) arXiv API over `math.*` in a window, (2) two
  case-insensitive regexes over the revision comment — `correct|errat|error|fix|mistake|bug|
  flaw|wrong|revised|amendment` and `lemma|theorem|proposition`, (3) an LLM (GPT 5.4 nano)
  classifying major / minor / none, (4) removal of non-permissive licences and false positives
  (100 → 35), (5) the PDF of the version before the correction; labels are location-only; their
  false-positive counts are upper bounds because documented errors are a subset of the errors
  present. Read, not cited as evidence for anything but its own procedure. One inconsistency
  noted: §4.2 says the window is 2023–2025 inclusive, Appendix D says 2025-01-01 to 2025-07-30.
- **Routes live-checked** (discovery only; nothing ingested; no S-entry): arXiv API `co:` search
  in `math.*` (23:28:55Z; `http://` answers 301 with an empty body — use `https://`): 319 for
  `co:erratum`, 4,108 for the S-0015-shaped query; Crossref `filter=update-type:erratum,
  type:journal-article` with `query.bibliographic=theorem proof` (23:26:34Z): 134, each with an
  `update-to` DOI; `update-type:retraction`: 66,928 across all fields; zbMATH Open
  `ti:erratum` (23:26:34Z): 4,791 with DOI links. Recorded in ADR-0003's *Measured* section with
  URLs, times and counts.
- **ADR-0003 written** (`PROGRAM/DECISIONS/ADR-0003.md`): D1 three routes (arXiv comments;
  Crossref update relations; zbMATH as the mathematics index) and the exclusions (anything
  AI-generated; unread secondary indexes; discussions that are not status documents); D2 seven
  admission criteria and the six-value verdict vocabulary; D3 the class mapping argued per item
  from the notice and the version diff, severity and `lean_catchable` from the FLAWS.md entry,
  `statement_status` from the status document, `proof_status` invalid by construction of S1.1;
  D4 the per-item record (manifest per ADR-0001 D4, committable bytes, `RECORD.md`, S-entries
  before ingestion, `groundtruth/HARVEST-LOG.md`); D5 licence handling (what is `committed`,
  what is `external`, ambiguity escalated); D6 the `raw/1` parent as item of record, the derived
  artifact = the affected result's statement and proof (method fixed at S1.3), and the
  deterministic producer `s1_ingest` 0.1.0 built before the first item; D7 the append-only
  harvest log with resumability; D8 the first-harvest window 2023-01-01..2026-09-30 (before the
  6 October release), ascending id, **no skipping**, no target count; D9 the S1.4 table shape
  and the power statement. Five choices flagged for Al: (i) the two files beside the ADR-0001
  layout, (ii) pending-truth items (recommendation: an ADR-0001 amendment adding
  `statement_status.value: null` / `method: "undetermined"`), (iii) the window and no-skipping
  rule, (iv) NC/ND originals committable as raw copies, (v) no LLM in admission.
- STATE: Gate S1 row S1.1 → procedure decided; ADR-0003 no longer reserved, ADR-0005 next free;
  the flagged choices added to standing escalations (not blockers, except (ii) for any
  pending-truth item); NEXT replaced (below). Memory: three hazard lines (the arXiv API
  `https`/`co:` route, the Crossref `update-type` filter with `mailto`, the zbMATH Open search
  endpoint) — tool hazards, no program state.
- Evidence run (§4.1): `bash scripts/check.sh` with ADR-0003 and the STATE edits staged,
  23:35:52Z–23:36:23Z: `Build completed successfully (8926 jobs)`; audit
  `'syndeom_golden_nat_le_sq' depends on axioms: [propext]`; fmt and clippy clean; `cargo test`
  23 passed, 0 failed; `git ls-files --eol` no `i/crlf` or `i/mixed`; `check_registers.py` 49
  tracked files clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 20/20; `RESULT: ALL PASS`;
  `GATE: GREEN` (31 s — warm OS cache this time). Re-run on the final tree after this entry,
  before the commit (STOP-REASON).
BELIEF DELTAS:
- The ingest producer is a prerequisite the Session-6 NEXT did not foresee: ADR-0001 D4's
  `generator` field must name the `engine/` script that wrote the directory, so hand-written
  manifests are not an option. Evidence: ADR-0001 D4, read this session. Consequence: NEXT is the
  producer; the first items follow it.
- Location-only labels (S-0015's) are not enough for SYNDEOM: the class, severity and
  Lean-catchability need the notice *and* the diff of the two versions, so each candidate costs
  one reading of each. Evidence: S-0015 §7.1 ("we only have coarse-grained labels"); FLAWS.md
  definitions. Consequence: D2.4 and D3; the harvest is reading-bound, not supply-bound.
- Supply is in the thousands on every route, so a small admitted set is a budget outcome, not a
  scarcity finding — which is exactly why D8's no-skipping rule exists: a curated hundred would
  look richer and prove less. Evidence: the route counts.
- Real errata sometimes leave the statement's truth open, and the manifest schema cannot say so.
  Evidence: S-0015 Table 1's "we do not know whether it is true"; ADR-0001 D4's `value: true /
  false`. Consequence: pending-truth, flagged (ii).
- A paper can be inconsistent with itself about its own parameters (S-0015's two windows); read
  the appendix before quoting the body.
CONFUSIONS:
- The arXiv API over `http://` returned 301 with an empty body and looked dead for one call;
  `https://` answers. Cost: one retry. Doc fix: memory hazard; the ADR's Measured section names
  the host.
- Whether the live route checks needed SOURCES entries: no — they are discovery, not sources;
  nothing was retrieved as scholarship and nothing is cited from them. Doc fix: the ADR says so
  in its Measured section.
HONESTY CHECK: no register entry changed status; CLAIMS, SOURCES and FLAWS untouched (S-0015
re-read only, digest re-verified). The ADR's Measured numbers are route sizes at a timestamp and
say so; nothing from the route responses entered any register. Five choices are flagged as
choices, not slipped into the decision text. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. ADR-0003 names `openai/math` (S-0001), the 7 October
withdrawals (S-0006) and the First Proof batches only to exclude them from S1; nothing under
`contact/` was opened. The route checks returned metadata of human-authored papers; one arXiv
comment thanked an "AI Math Referee" for finding the error — an AI-found error in a human proof,
admissible under D2.6 and recorded as such, with no influence on any decision. No detector,
threshold or battery design exists to be influenced. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The route counts are time-stamped API
responses that will differ tomorrow and are explicitly not results; the S-0015 digest and the
ADR's own bytes are regenerable by `sha256sum`. Nothing to quarantine.
MANIFEST CHECK: no artifact created; the ADR specifies the manifests future items will carry
under ADR-0001 and names the producer that will write them. Nothing synthetic labeled real;
"proven" applied to nothing.
DRIFT CHECK: no assessment of any paper (the three corrigendum titles and five arXiv comments
appear as route evidence, quoted, with no judgment); no understanding claims (SD17); no detectors,
so no detector-shopping; no referee contact (SD13). Scope: the ADR and its records; the producer
was not started.
NEXT-SINGLE-ACTION: S1.1 — **build the deterministic ingest producer** (ADR-0003 D6): add a
workspace member `engine/ingest/` (binary `syndeom-ingest`, `generator.id` `s1_ingest`, version
`0.1.0`, `rng` `"none"`) that takes a per-item config file carrying the ADR-0003 D4 fields
(`sources`, `item`, `licence`, `status`, `statement_status`, `proof_status`, `flaws[]` with
`raw/1` locators, `notes`) and the path of the original's bytes; computes `content_hash`,
`n_bytes`, `config_hash` and `artifact_id` with the existing `syndeom-manifest` functions; sets
`origin: "ground-truth"`, `seed: null`, `derivation: "documented-by-source"`, `commanded: {}`,
`fidelity` vacuous PASS, `engine_commit` from `git rev-parse HEAD`, `toolchain: null`; writes
`groundtruth/<artifact_id>/manifest.json` (JCS bytes) and self-checks it against ADR-0001 D4's
consistency rules, refusing to write on any violation; one golden test over fixed bytes and a
fixed config with hand-computed digests (ADR-0001 D8 style); `cargo fmt`, `clippy -D warnings`,
in the gate. Commit `[S1][INFRA] add the s1_ingest producer for ground-truth manifests`. Then the
first Route A harvest session under ADR-0003 D8 (window 2023-01-01..2026-09-30, ascending id, no
skipping), each admitted item with its S-entries, `RECORD.md`, and a HARVEST-LOG line.
ESCALATIONS:
- **ADR-0003 flagged choices (i)–(v)** — for Al's ratification; not blockers, the executor
  proceeds under them as written; **(ii) must be ruled before any pending-truth item is
  ingested** (recommendation: amend ADR-0001 D4 to admit `statement_status.value: null` with
  `method: "undetermined"`).
- SD15 audit request (Session 6) — unchanged.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers (S-0019).
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: natural boundary — the NEXT-SINGLE-ACTION done as specified. Repo green on the
committed tree (gate run with every edit staged, before the commit); this commit pushes to
`origin/main`.

SESSION: 2026-10-13 UTC, running into 2026-10-14 UTC @ 226a51c (Session 10 — Al's rulings of
2026-10-13 on ADR-0003 (i)–(v) and the SD15 audit recorded and committed as `226a51c` on top of
`4fe43a4`; then the `s1_ingest` producer built, verified, blocked for a while when the harness
stopped its gate run for system memory, then gated GREEN detached on Al's order and committed as
the commit adding this entry)
LAUNCH CONTEXT (SD15; not an exam session): the §6 Session-N resume prompt plus Al's rulings,
in the same conversation as Sessions 7–9. Not cold. Memory written this session: the trim Al
ordered (below) and nothing else; no hazards added.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1: procedure decided (ADR-0003, now
ruled); **the ingest producer built (ADR-0003 D6)**; no item ingested. Gate S0 closed and, since
this session, **confirmed not provisional**.
PLAN NOTE: two things in the order Al set — first the rulings of record (as Sessions 2 and 7 did:
ADR-0000 addendum, each amended ADR, STATE), then the inherited NEXT-SINGLE-ACTION (the producer,
as STATE stated it, against the schema as amended by ruling (ii)). Two commits, one HANDOFF entry
(the Session-3 precedent). The inherited NEXT survived ORIENT unchanged.
ADVANCED:
- ORIENT: toplevel `C:/Users/LEET/Syndeom`, `origin` the GitHub remote, `symbolic-ref` →
  `main`, HEAD = `origin/main` = `4fe43a4`, clean. Read: the constitution in full as amended by
  ADR-0004; STATE; HANDOFF Session 9; `PROGRAM/DECISIONS/` (ADR-0005 confirmed free); ADR-0001's
  schema-evolution text (Context line on structural compatibility; D9's "schema evolution is a new
  version string; consumers pin the string they understand"; the Session-4 Amendments precedent);
  SD14; `engine/manifest/src/lib.rs` and `tests/goldens.rs` in full; the memory file.
- **Rulings recorded** — ADR-0000 Addendum 3 (table: (i)–(v) and the SD15 audit, Al's words
  quoted where load-bearing); ADR-0003 amended (Status; D3 admission of undetermined-truth items;
  D5 licence sets per (iv); D8 prefix gate and D9 "prefix through arXiv id <last id processed>"
  per (iii); Rationale resolutions; Amendments entry); **ADR-0001 amended** per (ii): nullable
  `statement_status.value` iff `method: "undetermined"`, `correct` derived false, new rule
  `null ⇒ proof_status invalid`, and the schema-string decision read from the rules — both SD14
  and D9 say schema evolution is a new version string, no carve-out, so **`syndeom-manifest/2`**
  from this amendment on with `/1` accepted for pre-amendment manifests (the D8 goldens unchanged
  and valid); the alternative reading recorded and rejected. **ADR-0005** written (SD15 count-only
  disclosure: D1 what is verbatim, D2 count-only, D3 the name-search check, D4 pass criterion
  unchanged, D5 Session 6 retroactively conformant); SD15's sentence amended citing it;
  constitution header and footer Amendments lines updated. STATE: Updated line; Gate S0 line
  (confirmed, not provisional); constitution line (amended twice; three addenda); manifest-schema
  line (`/2`); memory-policy paragraph (ADR-0005; the trim); ADR numbers (ADR-0006 next free);
  the S3 obligation for undetermined items beside ADR-0004's; the two escalations resolved.
- **Crate change for (ii)** (`syndeom-manifest`): `MANIFEST_SCHEMA = "syndeom-manifest/2"`,
  `MANIFEST_SCHEMA_V1`, `MANIFEST_SCHEMAS`; `StatementStatus.value: Option<bool>` (key required,
  null allowed); `StatementMethod::Undetermined`; `Manifest::check` accepts both strings and adds
  three rules (null iff undetermined; null not under `/1`; null ⇒ invalid), derives `correct` as
  `value == Some(true) && valid`; `tests/goldens.rs` pins the D8 example as `/1` with `Some(true)`
  and adds `d4_amendment_2026_10_13_null_statement_truth` (passes under `/2`, key must be present,
  rejected under `/1`, null⇔undetermined both ways, null forces invalid, `correct` stays derived).
  `cargo fmt`, clippy `-D warnings` clean, **24/24 tests**.
- **Memory trim (SD15 audit (a))**: the "Hard invariants" paragraph removed from the SYNDEOM
  memory file; the `fmt --check` hazard line kept; nothing else changed. Digests: Session 6
  `feff158ccb872ca5627d393fc91aa544cca4a60d25d6af393282112f7ea8f798` (as recorded in HANDOFF
  Session 6); after the Session 7 and 9 hazard additions, before the trim,
  `9a31f8169af05f4ac9fb4fd49fcf237167996fb1b95aad240fc5b8868ceeddbb`; **after the trim
  `bc6de3600c87f7c5fb5f088a3c305dfc434e0c694dea4006791c5a5d75a6f8f1`** (7,453 bytes). A grep of
  the file for phase, gate status, NEXT, escalation, CLAIM, ADR, commit and session tokens matches
  only the policy sentence that forbids them.
- Evidence run for the first commit (§4.1): `bash scripts/check.sh` with every recording edit
  staged, 2026-10-14 00:07:19Z–00:17:54Z (cold OS cache again): `Build completed successfully
  (8926 jobs)`; audit `[propext]`; fmt and clippy clean; `cargo test` 24 passed, 0 failed;
  `git ls-files --eol` clean; `check_registers.py` 50 tracked files clean, Q 17/17, SD 17/17,
  FLAW 14/14, CLAIM 7/7, S- 20/20; `RESULT: ALL PASS`; `GATE: GREEN`. Commit **`226a51c`**
  `[S1][ADR-0000] record Al's rulings of 2026-10-13 on ADR-0003 (i)-(v) and the SD15 audit;
  ADR-0005; manifest/2`, pushed; `origin/main` = `226a51c`.
- **Producer built — the NEXT-SINGLE-ACTION (ADR-0003 D6).** `engine/ingest/` added to the
  workspace (`members = ["manifest", "ingest"]`; `Cargo.lock` gains only the `syndeom-ingest`
  package — no external dependency, S-0008 unchanged). `src/main.rs`: arguments parsed by hand
  (no new crate); `ItemConfig` with `deny_unknown_fields` — `sources`, `item`, `licence`,
  `status`, `statement_status`, `proof_status`, `flaws[]` with a `raw/1` `locator`, `file {name,
  storage}`, `notes`; `manifest.config` is the file minus `notes` (notes never enter the id);
  ADR-0003 shape checks (S1.1 status kinds `erratum` | `retraction`; `proof_status` invalid by
  construction; no `regenerable` storage; a bare file name; at least one flaw; classes
  FLAW-0001..0014 for taxonomy 0.1.0; non-empty locators; `S-NNNN` sources; `item` and `licence`
  non-empty); the manifest built with `MANIFEST_SCHEMA` (`/2`), `origin: "ground-truth"`,
  `generator {s1_ingest, 0.1.0, none}`, `seed null`, `toolchain null`, a `raw/1` file record with
  `content_hash` and `n_bytes`, ground truth with `Location::Raw` locators, `commanded {}`,
  `achieved {}`, vacuous fidelity; `Manifest::check` with the bytes **before any write** (exit 2
  listing every problem — the crate's rules including null ⇒ invalid); `write_or_refuse`
  (identical bytes are fine, different bytes under the same id exit 3 and nothing is overwritten,
  ADR-0001 D2); `--dry-run` prints the disk JSON and writes nothing. **Goldens** (`tests/golden.rs`):
  fixture `original.bin` 72 bytes `915d3ed6d9f7e388466bd03ea1f11f603fa57477fdf3724e6327537698ad1390`;
  config A (boolean truth, `external`) and B (null truth, `committed`), each citing the placeholder
  a four-zero placeholder id and saying so; config hashes `d2134e7fd2c3f7042d08f00f5a6536322e145f110722247532932a9337f8f0a7`
  and `4f5d4e93adaa08b8191c53d7ccfa2a57c4f49126822c84b9b9ce2a6e22ac59e0` computed independently
  in Python (sorted-key JCS + SHA-256) and reproduced by the binary; ids
  `s1_ingest-v0.1.0-d2134e7fd2c3` and `s1_ingest-v0.1.0-4f5d4e93adaa`; manifest digests pinned
  from the first run after every field had been checked against those values: A
  `1f2cf7b9230a74d26f4aff5128140ad997a53d76ca562c1e06a39224bf951f63` (2,305 bytes), B
  `451bde2d114eacdde2b8b7aeb3404efdb6b594121e4835614a19075e609b8340` (2,929 bytes). Tests: the
  fixture bytes; golden A (every field, external copies nothing, a second run is idempotent);
  golden B (bytes copied verbatim, a literal `null` written with the key present); dry-run writes
  nothing; refusals — exit 2 for a valid proof on an S1.1 item, a class outside 0.1.0,
  `regenerable` storage, a null truth with a non-undetermined method (the crate's rule reached
  through the binary), an unknown key; exit 1 for a malformed engine commit; exit 3 for a
  same-id byte conflict, with the existing file untouched. `cargo fmt` clean; clippy
  `-D warnings` clean after one dead-code warning was resolved by making the producer validate
  `sources`, `item` and `licence`; **5/5 ingest tests, 24/24 manifest tests**, sandboxes under the
  OS temp dir removed after each test. `groundtruth/README.md` now names the producer and the
  `RECORD.md` / `HARVEST-LOG.md` convention.
- STATE: S1.1 row (producer built, goldens), S1.3 row (the `raw/1` producer exists; the
  derivation producer and its ADR do not); NEXT replaced by the first Route A harvest session,
  steps (a)–(e), executable cold.
- **Second commit NOT made — the gate run on the staged producer tree was stopped by the harness.**
  Foreground verification before the gate: `cargo fmt --all --check` clean, `cargo clippy
  --all-targets --all-features -- -D warnings` clean, `cargo test` 5/5 + 24/24 (00:20:14Z–00:20:16Z);
  `git ls-files --eol` shows every new file `i/lf`; `check_registers.py` with the producer staged:
  56 tracked files clean, all cross-references resolve, `RESULT: ALL PASS`. The full gate started
  00:21:40Z, completed `lake build` (`Build completed successfully (8926 jobs)`), and was in the
  axiom audit (`lake env lean Audit.lean`, loading Mathlib's oleans; lean.exe above 3.2 GB) when the
  harness killed the shell at about 00:30Z because the machine was critically low on memory. At
  00:32:39Z the child `lean.exe` (3.2 GB) and two `lake.exe` were still listed by `tasklist`; by
  the next check, seconds later, all three had exited on their own and free memory was 6.7 GB —
  the executor stopped nothing. The harness forbids restarting the stopped command unasked, so
  **the producer is left staged and
  uncommitted in the working tree; `origin/main` is green at `226a51c`.** The gate must run GREEN
  on this tree before the commit (Al's instruction and SD9); the commit message is ready in the
  executor's scratchpad and reproduced in STATE's NEXT. Nothing on `main` is broken.
- **Resolved in the same conversation on Al's order** ("run gate with a log in the background in a
  separate powershell"): the gate ran detached from the session's shells — `Start-Process` of Git
  Bash (`C:\Program Files\Gitinash.exe`; the `bash` on PowerShell's PATH is the WSL stub
  `C:\WINDOWS\system32ash.exe`, which exits at once) on a four-line wrapper script writing to
  `gate-10c.log` — 00:43:55Z–00:52:27Z on `226a51c` with the 11 producer files staged: `Build
  completed successfully (8926 jobs)`; audit `[propext]`; fmt and clippy clean; `cargo test`
  5/5 ingest + 24/24 manifest; `git ls-files --eol` clean; `check_registers.py` 56 tracked files
  clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 20/20; `RESULT: ALL PASS`; `GATE: GREEN`;
  `rc=0`. The gate was then re-run the same way on the final tree (this entry and STATE's lines
  updated) before the commit; the producer is committed as the commit adding this entry.
BELIEF DELTAS:
- "Schema evolution is a new version string" has no proportionality clause, and reading it as
  written costs almost nothing: two accepted strings and three conditional rules. Evidence: SD14,
  ADR-0001 D9, the crate diff. Consequence: `/2` is the current schema; a consumer that only knows
  `/1` is told so by the string, which is the whole point of the rule.
- A nullable truth value is not a hole in the schema but the triage case the charter describes
  (§0.2: truth versus triage); S-0015's "we do not know whether it is true" is exactly an item the
  corpus wants. Evidence: ruling (ii) and the S3 obligation it carries. Consequence: such items
  count in proof-flaw cells only.
- Overruled is cheap when the record was a flag: (iv) changed one bullet, and the reason of record
  is Al's, quoted. Evidence: ADR-0003 D5. Consequence: keep flagging choices rather than deciding
  them silently.
- A producer that "decides nothing" still needs a shape layer the schema does not have: the crate
  accepts any `S-NNNN`, any of four status kinds and any well-formed class; ADR-0003's narrower
  admissions (two status kinds, fourteen classes, invalid by construction) have to live in the
  producer. Evidence: `check_shape` versus `Manifest::check`. Consequence: ADR-0001 is enforced in
  the crate, ADR-0003 in the producer; both refuse, neither repairs.
- `notes` must stay outside `config`, or editing a note changes the artifact id. Evidence:
  ADR-0001 D2 (id = f(producer, config, seed)) and D4 ("never load-bearing"). Consequence: the
  config file's `notes` key is lifted to `manifest.notes`.
- A manifest digest pinned from a first run is a weaker golden than a hand-computed one; it is
  acceptable only because every load-bearing field was first checked against an independently
  computed value. Evidence: the test preamble says exactly that. Consequence: a change to the disk
  form fails the digest and must re-justify it in the open.
CONFUSIONS:
- One ADR-0003 edit missed its anchor (the D3 pending-truth sentence had wrapped differently from
  the draft in the scratchpad); re-read and re-targeted. Cost: one round. Doc fix: none needed —
  always grep the committed text, not the draft.
- clippy's dead-code warning on three parsed-but-unread config fields: resolved by validating
  them, which the producer should have done anyway. Cost: one build. Doc fix: none.
- The register checker flagged the goldens' placeholder source id when this entry first named it
  as a literal: a `S-` + four digits token in any `.md` must be a defined entry. Fixed by
  describing the placeholder instead of writing it; the JSON fixtures are not scanned. Cost: one
  check run. Doc fix: this line.
- Whether the binary should read the engine commit from `git` itself: no — it is an argument, so
  the goldens are deterministic and the runbook passes `$(git rev-parse HEAD)`; STATE's NEXT says
  so.
HONESTY CHECK: no CLAIM or SOURCE changed status; FLAWS untouched. S-0005's INGESTED state is
unchanged by ruling (iv), as the ruling says. The D8 goldens are byte-identical and still pass as
`/1`. ADR-0001's amendment records the alternative reading it rejected. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the only mentions are ADR-0003's exclusions and
STATE's S5 notes, unchanged. No detector or threshold exists. The producer writes manifests from
inputs and hashes and decides nothing. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. Every digest above is regenerable by
`sha256sum`; the producer's goldens are fixed bytes and fixed configs with independently computed
hashes (Python JCS cross-check for the config hashes). Nothing to quarantine.
MANIFEST CHECK: no real artifact was created — the producer's golden directories live in the test
sandbox under the OS temp dir and are deleted after each test; their configs cite a placeholder
source id (four zeros, no such entry) and say so in `notes`. Nothing synthetic labeled real; "proven" applied to nothing.
DRIFT CHECK: no statement about any real manuscript (SD3); no understanding claims (SD17); no
detectors, so no detector-shopping; no referee contact (SD13). Scope: the rulings, the schema
amendment they required, and the producer NEXT named.
NEXT-SINGLE-ACTION: S1.1 — the first Route A harvest session under ADR-0003 D8 as amended (a
prefix is enough), exactly as STATE.md now states it: (a) create `groundtruth/HARVEST-LOG.md`
with the D7 header (query, window 2023-01-01..2026-09-30, ascending arXiv id, "last candidate
processed: none"); (b) the `lab/` candidate finder — standard library only, the two S-0015
stage-1 regexes over the arXiv API's Atom feed for `cat:math.*` in the window — run and its
output digest logged; (c) candidates examined in ascending arXiv id, none skipped, each against
ADR-0003 D2's seven criteria, every verdict logged with its reason; (d) per admitted item: the
S-entries for original and notice in SOURCES.md before ingestion (licences per D5 as amended),
the per-item config, `RECORD.md` with the D3 class argument, then from `engine/`
`cargo run -p syndeom-ingest -- --config <item.json> --bytes <original> --root ..
--engine-commit $(git rev-parse HEAD)`; (e) stop at the session's budget, the log's last
candidate being the resume point. Commit `[S1][ADR-0003] harvest Route A prefix through arXiv
id <last id processed>: <n> admitted`; update the Gate S1 table; set NEXT to the next prefix.
ESCALATIONS:
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers (S-0019).
- Human-referee participants remain Al's decision (SD13) — unchanged.
- Nothing else awaits Al: ADR-0003's five choices and the SD15 audit are resolved.
STOP-REASON: natural boundary — the rulings recorded and the NEXT-SINGLE-ACTION done. One
blocker on the way, recorded above: the harness stopped the producer's gate run for low system
memory; on Al's order the gate ran detached and GREEN. Repo green on each committed tree (gate
run with every edit staged, before each commit); both commits push to `origin/main`.

SESSION: 2026-10-14 UTC @ 30e2a4a (Session 11 — housekeeping, then the first Route A harvest: two
candidates examined, two admitted and ingested; the commit adding this entry lands on top of
30e2a4a)
LAUNCH CONTEXT (SD15; not an exam session): the §6 Session-N resume prompt plus Al's session
instructions (two housekeeping items, then the inherited NEXT), in the same conversation as
Sessions 7–10. Not cold. Memory written this session: the two tooling hazards Al ordered
(HOUSEKEEPING 1) and nothing else.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: the first prefix
harvested — 2 of 2,472 Route A candidates examined, both admitted; S1.3: two `raw/1` parents
manifested, no derived artifact yet. Gate S0 closed and confirmed.
PLAN NOTE: the inherited NEXT-SINGLE-ACTION, executed as STATE stated it, after the two
housekeeping items Al placed before it. One thing: the harvest; the housekeeping needed no
decision. The inherited NEXT survived ORIENT unchanged.
ADVANCED:
- ORIENT: toplevel `C:/Users/LEET/Syndeom`, `origin` the GitHub remote, `symbolic-ref` → `main`,
  HEAD = `origin/main` = `30e2a4a`, clean; DECISIONS ADR-0000..0005; SOURCES last entry S-0020 →
  S-0021 next. Evidence gate at ORIENT, detached with a log (`gate-11a.log`): 01:06:18Z–01:06:53Z
  on `30e2a4a`, `GATE: GREEN`. Read: the constitution in full as amended (ADR-0004, ADR-0005),
  STATE, HANDOFF Session 10, S-0003 (the date-carrier pattern), the lab and groundtruth READMEs.
- **HOUSEKEEPING 1 — memory hazard (SD15):** two bullets appended to the SYNDEOM memory file —
  PowerShell's `bash` is the WSL stub, name `C:\Program Files\Git\bin\bash.exe`; run the gate
  detached with a log under the scratchpad so a harness kill does not lose the result. Digests:
  Session 10 `bc6de3600c87f7c5fb5f088a3c305dfc434e0c694dea4006791c5a5d75a6f8f1`; **after this
  session's append `80c4333bd22eec23d9d05d6d0dc2422c60b3840e4b0a065616af5b9ba39d24c7`** (8,044
  bytes). No program state written (grep for phase, gate status, NEXT, escalation, CLAIM, ADR,
  commit and session tokens matches only the policy sentence).
- **HOUSEKEEPING 2 — three sources for the position paper's §0 disclosure**, retrieved and read
  under SD11, each digest-only (no licence stated on any page): **S-0021** the "Open Letter about
  the Mathathon" (Proofs and Prompts, published 2026-09-10, "Received 8 September 2026", "At the
  time of publication, there are 771 signatories", "Anthropic and OpenAI, will supply participants
  with 2 million dollars in AI credits" — all on the page, quoted); **S-0022** the "Joint
  Statement about Mathathon" (2026-09-28; "We no longer receive sponsorships from developers of
  proprietary AI models"; the Tao cross-post of the same day retrieved as corroboration, the S-0003
  pattern — here the primary is itself dated); **S-0023** the Mathathon page "Old Problems, New
  Proofs" (undated; "Round 1 on November 13th to November 15th at Caltech"). Each entry's
  "bears on" field: the position paper's §0 disclosure; no CLAIM. Reader comments not
  transcribed. Al's summaries were checked against the pages, not copied.
- **S-0019 note:** one attempt at `www.apache.org` as ordered, 01:06:29Z, connection timed out;
  carried unchanged, recorded in the entry.
- **Harvest — the finder.** `lab/route_a_candidates.py` (standard library, zero result
  authority): the API-side pre-filter over the comment field (18 correction words × 3 result
  words) and `cat:math.*`, the two S-0015 stage-1 regexes as the post-filter, latest version ≥ 2,
  latest update ≥ window start, sorted by ascending id. **Two strikes on the first form** (one
  range, 500 per page): HTTP 429 at the first request, then after three good pages persistent
  429/503 at offset 1,500 until the retries ran out — the arXiv API refuses deep offsets for a
  large result set. **Switched approach** (§4.4): the query split into 57 original-submission-date
  ranges (yearly 1991–2019, quarterly 2020–2026), 200 per page, never past offset 1,000 in a
  range, every page cached on disk, 6 s between requests, backoff on 429/503. Run 1:
  01:28:02Z–01:56:25Z, 73 requests, 11 transient errors retried, largest range 462 hits, **8,081
  API hits, 2,472 candidates** (71 marked `latest update after window`), `candidates.tsv` SHA-256
  `e164c0b1693c7a6cdf242dcca853db28517f8c210d17af7ba82d2446dd752d48` (not committed; the log
  records the query, the run and every examined candidate). `lab/route_a_examine.py` fetches both
  versions' abstract pages and e-prints, records digests, diffs the TeX and scans for
  AI-assistance mentions.
- **Harvest — the examination, in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md`,
  run 1, rows 1–2):
  - **Candidate 1, arXiv 0706.0357** (math.GM, one author, 71 versions since 2007), v71
    (2026-06-07) "To fix an error in Lemma 1.1": the diff shows the lemma's identity acquired a
    family of terms over the prime divisors of the modulus and its proof the missing coprimality
    condition on the character-orthogonality formula. **Admitted**: FLAW-0012 (statement, fatal)
    + FLAW-0005 (proof, fatal); `statement_status` false by source document; v70 CC BY 4.0 →
    original **committed** (`original-0706.0357v70.tar.gz`, 8,236 B, `5987d810…cca8`);
    `s1_ingest-v0.1.0-63e23a952084`; S-0024, S-0025. The paper's category and revision count
    played no part — the no-skipping rule admits what the criteria admit, and the record says so.
  - **Candidate 2, arXiv 0707.3799** (math.RT, two authors, "to appear in Moscow Math J"), v3
    (2026-02-25) "corrects a mistake in the proof of Theorem 1b) … All the statements are
    unchanged": the diff replaces the proof of part b) with an explicitly W-equivariant
    identification of the affine Grassmannian's components, which the v2 proof had used without
    establishing. **Admitted**: FLAW-0011 (composite, gap) + FLAW-0005 (gap); `statement_status`
    true by source document; v2 licence "assumed-1991-2003" (arXiv's page: equivalent to the
    non-exclusive-distrib licence, retrieved and digested) → **`external`**, digest-only
    (`3f9593e1…3053`); `s1_ingest-v0.1.0-744652f4b95f`; S-0026, S-0027.
  - Each item: S-entries written **before** the producer ran (SD16); config with the D4 fields;
    `syndeom-ingest` wrote `manifest.json` (`syndeom-manifest/2`, `engine_commit` `30e2a4aa…`,
    self-check passed; ids equal to the Python-computed JCS hashes); `RECORD.md` with the D3
    class argument, the alternatives considered, the quoted locating sentences and the licence
    basis. Both records state that nothing in them assesses the paper beyond its own status
    document (SD3).
  - **Counts by verdict:** examined 2 — admitted 2, rejected 0, found-but-unclassifiable 0,
    pending 0, pending-truth 0, escalated 0. **Last candidate processed `0707.3799`**; the session
    stopped at its budget before examining candidate 3 (`0812.3181`), not because the stream
    changed.
- **Tooling:** `.gitattributes` gains `*.gz binary` and `*.tgz binary`, and the register checker's
  binary list gains `.gz`/`.tgz`, so committed gzip originals are not line-ending-checked (git
  already stored the file `i/-text`). `groundtruth/README.md` unchanged (it already names the
  producer and the record files).
- STATE: Gate S1 rows S1.1 (prefix through `0707.3799`, 2 admitted) and S1.3 (two `raw/1`
  parents manifested, no derived artifact); registers line; Updated line; NEXT = the second
  harvest session resuming at candidate 3, with the exact commands.
- Evidence run (§4.1), detached with a log (`gate-11b.log`), every edit staged:
  02:07:11Z–02:18:18Z (cold OS cache): `Build completed successfully (8926 jobs)`; audit
  `[propext]`; fmt and clippy clean; `cargo test` 5/5 ingest + 24/24 manifest; `git ls-files
  --eol` clean (the `.tar.gz` original stored `i/-text`); `check_registers.py` 64 tracked files
  clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, **S- 27/27**; `RESULT: ALL PASS`; `GATE:
  GREEN`; `rc=0`. Re-run the same way on the final tree (this entry and STATE's build line in)
  before the commit (STOP-REASON).>
BELIEF DELTAS:
- The arXiv API cannot be paged deeply: a single large query dies at offset ≈1,500 with 429/503
  however patient the retries. Evidence: two failed runs, then 57 shallow ranges that all
  succeeded. Consequence: the finder is range-split and cached; the hazard is in the log header,
  and the examined prefix never depends on a re-fetch.
- "Ascending arXiv id" puts old papers with recent corrections first: the stream's head is
  2007–2012 submissions corrected in 2023–2026, including long-revised math.GM preprints.
  Evidence: candidates 1–15. Consequence: the corpus's early prefix is what the rule yields, not
  what a curator would pick; D8 exists for exactly that reason, and each record says the
  category played no part.
- A revision comment plus a TeX diff is enough to argue a FLAWS.md class in writing for both
  items examined, one at the statement level (FLAW-0012 with its FLAW-0005 use site) and one at
  the proof level under a true statement (FLAW-0011 with FLAW-0005). Evidence: the two RECORD
  files. Consequence: D3's "argued from the notice and the diff" is workable at roughly one hour
  per candidate including retrieval; the harvest is reading-bound, as ADR-0003 predicted.
- The 2008 version of a 2007 paper carries arXiv's "assumed" licence on its abstract page, not
  the non-exclusive one; the licence is per version and must be read, not inferred from the
  date. Evidence: S-0026 versus S-0027. Consequence: `external` for the original, `committed`
  would have been wrong.
CONFUSIONS:
- The first finder's failure looked like rate limiting to be waited out; it was the deep-offset
  limit. Cost: two runs, about 25 minutes. Doc fix: the log header and the script docstring say
  so; the memory hazard list does not (program-independent API behaviour; a successor reads the
  script).
- A file-name slip (`.tar.gz` for a single gzipped TeX) cost one producer run; the producer
  refused cleanly (exit 1, file not found) and wrote nothing. Doc fix: none needed.
- The committed `.tar.gz` original was not in the checker's binary list; caught by running the
  checker before the gate. Doc fix: the list and `.gitattributes` now cover `.gz`/`.tgz`.
HONESTY CHECK: no CLAIM changed status. New SOURCES: S-0021..S-0023 VERIFIED digest-only
(licence unstated on each page), S-0024 INGESTED (CC BY 4.0, committed), S-0025..S-0027 VERIFIED
digest-only (S-0026's assumed licence quoted from arXiv's own page). The two items' ground truth
is what their authors' notices and diffs document; the records name the alternative classes
considered and why they were not chosen. The finder's hit counts are a snapshot and say so.
Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the Route A window ends 2026-09-30, before the 6
October release, by rule; both admitted papers predate it by years and carry no AI-assistance
mention (helper scan of both corrected sources: none). The three disclosure sources concern a
hackathon and its sponsorship, not any proof corpus, and inform no design. No detector or
threshold exists. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The two manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `30e2a4a` (ids and hashes reproduced
independently in Python); every retrieval is pinned by URL, access time and SHA-256; the finder's
snapshot digest is recorded with the caveat that arXiv changes. Nothing to quarantine.
MANIFEST CHECK: two artifacts created, each with a `syndeom-manifest/2` manifest carrying ground
truth with derivation `documented-by-source`, flaw records with `raw/1` locators, `correct:
false`, taxonomy 0.1.0, self-checked by the producer before writing (SD1). Nothing synthetic
labeled real; nothing real labeled `by-construction` (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any real manuscript beyond its own status document (SD3) — each record
and log row says so and names only the result the notice names; the executor's one algebraic
check (that the omitted term in item 1 is non-zero) is a check of the corrected statement the
authors published, recorded as such. No understanding claims (SD17). No detectors, so no
detector-shopping. No referee contact (SD13). The session did not examine candidate 3 to avoid
a half-examined candidate at the budget boundary.
NEXT-SINGLE-ACTION: S1.1 — the second Route A harvest session, resuming at candidate 3
(`0812.3181`) in ascending id with no skipping, exactly as STATE.md states it: regenerate or
re-cache the candidate list and record its digest, examine with `lab/route_a_examine.py`,
register sources before ingestion, ingest with `syndeom-ingest`, write `RECORD.md`, log every
verdict, stop at budget, update the log's last-candidate line and the Gate S1 table.
ESCALATIONS:
- None new: no grey zone arose in candidates 1–2 (no licence ambiguity, no authorship question).
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers (S-0019;
  one attempt this session timed out).
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — the session's one task advanced to a clean boundary (two
candidates fully recorded, the log's resume point set). Repo green on the committed tree (gate
run detached with a log, every edit staged, before the commit); this commit pushes to
`origin/main`.

SESSION: 2026-10-14 UTC @ 81f4f98 (Session 12 — Al's D8 precision recorded; the commit adding this
entry lands on top of 81f4f98)
LAUNCH CONTEXT (SD15; not an exam session): Al's follow-up to Session 11 in the same
conversation — an ADR-0003 D8 precision and a harvest-log note; no ruling, no build. Not cold.
No memory written.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open; unchanged from Session 11.
PLAN NOTE: one thing — the precision Al ordered, recorded where ADR amendments go (D8 bullet,
Amendments entry) and in the log header. The inherited NEXT-SINGLE-ACTION survived unchanged.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `81f4f98`, clean; the D8 window bullet, the ADR's Amendments
  tail and the log's window bullet located and read.
- ADR-0003 D8 gains a **Precision** bullet (Al, 2026-10-14, after Session 11): the window bounds
  the date of the correcting version (Route A) or notice (Routes B, C), never the paper's original
  submission; ascending id therefore puts the oldest papers first, by design. Amendments entry
  added. `groundtruth/HARVEST-LOG.md`'s window bullet carries the same precision and consequence.
  STATE's Updated line names the session. No decision changed; Session 11 applied the window this
  way already (both admitted items are 2007 submissions corrected in 2026).
- Evidence run, detached with a log (`gate-12a.log`), every edit staged: 02:34:42Z–02:45:22Z;
  64 tracked files clean, S- 27/27, `RESULT: ALL PASS`, `GATE: GREEN`, `rc=0`; re-run the same
  way on the final tree (this entry and STATE's build line in) before the commit.
BELIEF DELTAS: none beyond Session 11's — the precision states what the harvest already did.
CONFUSIONS: none.
HONESTY CHECK: no register entry changed; no item touched. Downgrades: none.
LEAKAGE CHECK: nothing under `contact/` opened; nothing AI-generated cited; no design decision
taken (a wording precision only). Verdict: no leakage.
DETERMINISM CHECK: no results. Nothing to quarantine.
MANIFEST CHECK: no artifact touched; the two `groundtruth/` manifests are unchanged (digests as
committed in `81f4f98`). "Proven" applied to nothing.
DRIFT CHECK: no statement about any manuscript; no understanding claims; no detectors; no
referee contact. Scope: the precision and its two records.
NEXT-SINGLE-ACTION: unchanged — S1.1, the second Route A harvest session resuming at candidate 3
(`0812.3181`), exactly as STATE.md states it.
ESCALATIONS: none new; the S-0019 apache.org note and SD13 carry unchanged.
STOP-REASON: natural boundary — the follow-up recorded. Repo green on the committed tree (gate run
detached with a log, every edit staged, before the commit); this commit pushes to `origin/main`.

SESSION: 2026-10-14 UTC @ bb0fc32 (Session 13 — the second Route A harvest: candidates 3–6
examined, 2 admitted, 2 rejected; the commit adding this entry lands on top of bb0fc32)
LAUNCH CONTEXT (SD15; not an exam session): Al's one-word order "resume" in the same
conversation as Sessions 7–12; the inherited NEXT-SINGLE-ACTION executed as STATE stated it. Not
cold. No memory written.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: prefix through arXiv id
`0912.0308` (6 of 2,472 candidates; 4 admitted, 2 rejected); S1.3: four `raw/1` parents
manifested, no derived artifact yet. Gate S0 closed and confirmed.
PLAN NOTE: the inherited NEXT (the second harvest session, resuming at candidate 3), unchanged by
ORIENT. One thing; no housekeeping this session.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `bb0fc32`, clean; the constitution last changed at `226a51c`
  (read in full in Session 11 of this conversation, unchanged since); STATE's NEXT and the
  harvest log's resume point read. Evidence gate at ORIENT, detached (`gate-13a.log`):
  02:51:07Z–02:51:42Z on `bb0fc32`, `GATE: GREEN`.
- **Candidate list:** regenerated with `lab/route_a_candidates.py` from the 73 cached pages in
  this conversation's scratchpad — 0 API requests, 2,472 candidates, digest identical to run 1
  (`e164c0b1…2d48`); noted in the log, no new run record.
- **Candidates examined in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md` rows 3–6):
  - **3, arXiv 0812.3181** (lecture notes, math.AP), v3 (2023-07-20) "Corrects an error in
    statement of Theorem 9.2": the diff changes the Duistermaat–Guillemin statement's description
    of the Maslov term ("number of conjugate points" → the Morse index), a theorem the notes quote
    from the literature without proof. **Rejected, D2.1**: no offered proof to label — a
    FLAW-0010(c)-shaped mis-citation in expository notes. The diff also fixes unannounced small
    errors (a symbol order, a Huygens statement, coordinates) none of which the notice documents,
    so none is usable (D2.2); recorded in the row.
  - **4, arXiv 0902.0680** (math.DS), withdrawn as v2 in 2011, corrected by v3 on 2026-09-30 (the
    window's last day) with an explicit "Erratum on a previous version": "a gap in the
    dense-subspace argument for pointwise convergence"; v3 re-proves only the Fourier-dimension
    case by a new dense class and says the general statements "are not addressed". **Admitted**:
    the v1 lemma `density`'s almost-everywhere step invokes a weak-convergence criterion whose
    tightness hypothesis is asserted, not established — FLAW-0005 (gap); **`statement_status`
    null / `undetermined`** (the general lemma neither re-proved nor retracted) — the first item
    under Al's ruling (ii), written by the producer under `syndeom-manifest/2` and self-checked;
    v1 licence arXiv non-exclusive → `external`; `s1_ingest-v0.1.0-5ba6c0f07557`; S-0028, S-0029.
    The record states plainly that the notice names the argument and the step-level locator is
    the executor's reading, with FLAW-0004 as the alternative if the author's gap lay elsewhere in
    the same argument.
  - **5, arXiv 0908.3213** (math.RA/DG), v5 (2024-07-29) "The statement of Theorem 4.4
    (assertions (1) and (2)) was fixed and the proof was modified accordingly": the diff reverses
    the signs in the listed complex structures and fixes a normalization constant in the proof;
    an executor's six-line check with the theorem's own brackets shows the v4 structures fail the
    abelian condition on four pairs and the v5 structures pass. **Admitted**: FLAW-0001
    `propagated` (statement, fatal) + FLAW-0001 `local` (proof, repairable); `statement_status`
    false by source document, the computation recorded as the executor's confirmation; v4 licence
    arXiv non-exclusive → `external`; `s1_ingest-v0.1.0-a8ac73896464`; S-0030, S-0031. v4's own
    2012 correction (Theorem 3.2, Theorem 3.4) is outside the window and not harvested — the D8
    precision of Session 12 applied.
  - **6, arXiv 0912.0308** (math.CA): the correcting version v3 is dated 2026-10-02, after the
    window end. **Rejected, D8 window**; abstract pages only, no sources fetched; its comment
    ("Corrected Lemma 6.4 which was false as stated") is logged as a candidate for a later window.
  - **Counts by verdict, this session:** examined 4 — admitted 2, rejected 2 (one D2.1, one D8),
    found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0. **Cumulative:** examined
    6 — admitted 4, rejected 2. **Last candidate processed `0912.0308`**; the session stopped at
    its budget before examining candidate 7 (`0912.0966`).
- Each admitted item: S-entries written **before** the producer ran (SD16); config with the D4
  fields; `syndeom-ingest` at `bb0fc32` wrote `manifest.json` (`syndeom-manifest/2`,
  `engine_commit` `bb0fc32…`, self-check passed — including the null-truth rules); `RECORD.md`
  with the D3 class argument, alternatives considered, quoted locating sentences and licence basis.
- **Helper limits found:** `lab/route_a_examine.py` aborts when the version before the
  correcting one is a withdrawal (no e-print, HTTP 404) — candidate 4 was finished by hand with
  the same calls (abs v2/v3, e-print v3, a v1→v3 diff); a successor should fetch the last
  non-withdrawn version as the original. Not fixed this session; recorded here and in CONFUSIONS.
- STATE: Updated line; Gate S1 rows S1.1 (prefix through `0912.0308`, 6/4/2) and S1.3 (four
  parents, the null-truth one named); registers line (S-0028..S-0031); NEXT resumes at candidate
  7 with the four configs and records as the pattern.
- Evidence run, detached with a log (`gate-13b.log`), every edit staged: 03:01:46Z–03:02:24Z:
  `Build completed successfully (8926 jobs)`; audit `[propext]`; fmt and clippy clean; `cargo
  test` 5/5 + 24/24; `git ls-files --eol` clean; `check_registers.py` 68 tracked files clean,
  Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, **S- 31/31**; `RESULT: ALL PASS`; `GATE: GREEN`;
  `rc=0`. Re-run the same way on the final tree (this entry and STATE's build line in) before
  the commit (STOP-REASON).
BELIEF DELTAS:
- Lecture notes with a corrected *quoted* theorem are not S1.1 items: the corpus needs an offered
  proof, and a mis-stated citation has none. Evidence: candidate 3. Consequence: the D2.1
  rejection reason is now written once in the log and reusable; such items are the natural ground
  truth for a citation-content detector, a note for S3, not for S1.
- The undetermined-truth case is real and arrives early: one of the first four admitted items has
  a lemma neither re-proved nor retracted. Evidence: candidate 4's erratum. Consequence: ruling
  (ii) and the `/2` schema earned their keep on the fourth item; the S3 obligation for such items
  (proof-flaw cells only) already has a member.
- A notice that locates an error to an *argument* rather than a step forces a choice: admit with
  the step-level reading marked as the executor's, or refuse to classify. Evidence: candidate 4.
  Consequence: admitted, with the caveat and the alternative class written into the record and
  the locator — the honest form of "argued in writing" when the author did not say which step.
- A small algebraic check can confirm a statement-level falsity from the published brackets
  alone, with no new mathematics. Evidence: candidate 5's abelian-condition check. Consequence:
  such checks are recorded as the executor's confirmation, never as the ground truth's source.
- The D8 precision mattered on the fourth examined paper: a 2012 correction and a 2024
  correction in one revision history, only the latter in the window. Evidence: candidate 5.
CONFUSIONS:
- The examination helper's first run on candidate 4 died on the withdrawn v2's missing e-print;
  cost: one extra fetch round. Doc fix: this entry; the helper's withdrawal handling is a small
  change for the next harvest session (fetch the last non-withdrawn version as the original).
- Candidate 5 has two correcting versions; the first (2012) is outside the window. Cost: one
  diff read that was not needed. Doc fix: the log row and the record say which correction is the
  item's.
HONESTY CHECK: no CLAIM changed status. New SOURCES S-0028..S-0031, all VERIFIED digest-only
(every licence arXiv non-exclusive or CC BY on the corrected version). Item 3's step-level class
rests on the executor's reading of the proof and says so in the record, the locator and the log;
item 4's falsity rests on the notice and an executor's computation recorded as confirmation. The
two rejections state their criterion. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the window excludes the release by rule and
candidate 6 was rejected by that rule; all four examined papers are 2008–2009 submissions by
named human authors, with no AI-assistance mention in the corrected sources (helper scan on 4 and
5; candidate 3's sources scanned by the helper, none). No detector or threshold exists. Verdict:
no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The two manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `bb0fc32`; the ids reproduced independently in
Python; the candidate list reproduced from cache with the identical digest; every retrieval is
pinned by URL, access time and SHA-256; the abelian-condition check is a short script quoted in
the record. Nothing to quarantine.
MANIFEST CHECK: two artifacts created, each with a `syndeom-manifest/2` manifest carrying ground
truth with derivation `documented-by-source`, flaw records with `raw/1` locators, `correct:
false`, taxonomy 0.1.0, self-checked by the producer before writing (SD1); one carries
`statement_status.value: null` with `method: "undetermined"` exactly as ADR-0001 as amended
requires. Nothing synthetic labeled real (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3): each record and
row names only the result the notice names; the executor's computation on candidate 5 confirms
what the authors' correction states and is labelled as a confirmation; the executor's step-level
reading on candidate 4 is labelled as a reading. No understanding claims (SD17). No detectors,
so no detector-shopping. No referee contact (SD13). Candidate 7 was not opened.
NEXT-SINGLE-ACTION: S1.1 — the third Route A harvest session, resuming at candidate 7
(`0912.0966`, v6, "… A small correction made to the statement of Theorem 8") in ascending id
with no skipping, exactly as STATE.md states it; before the first examination, give
`lab/route_a_examine.py` the withdrawal handling noted above.
ESCALATIONS:
- None new: no grey zone arose in candidates 3–6.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers (S-0019).
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — a clean boundary (four candidates fully recorded, the resume point
set). Repo green on the committed tree (gate run detached with a log, every edit staged, before
the commit); this commit pushes to `origin/main`.

SESSION: 2026-10-14 UTC @ 7c952e6 (Session 14 — the third Route A harvest: candidates 7–10
examined, 3 admitted yielding 4 items, 1 rejected; the commit adding this entry lands on top of
7c952e6)
LAUNCH CONTEXT (SD15; not an exam session): Al's one-word order "resume" in the same conversation
as Sessions 7–13; the inherited NEXT-SINGLE-ACTION executed as STATE stated it. Not cold. No
memory written.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: prefix through arXiv id
`1010.2391` (10 of 2,472 candidates; 7 admitted yielding 8 items, 3 rejected); S1.3: eight
`raw/1` parents manifested, no derived artifact yet. Gate S0 closed and confirmed.
PLAN NOTE: the inherited NEXT (the third harvest session, resuming at candidate 7, with the
helper's withdrawal handling first), unchanged by ORIENT. One thing.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `7c952e6`, clean; the constitution unchanged since `226a51c`
  (read in full in Session 11 of this conversation); the log's resume point read. Evidence gate
  at ORIENT, detached (`gate-14a.log`): 03:36:09Z on `7c952e6` — see the gate line below for its
  result (it was still loading Mathlib when the examinations ran).
- **Helper fix (NEXT's first step):** `lab/route_a_examine.py` now reads the submission history
  of the correcting version first and takes as the original the last version before it that is
  not marked withdrawn (a withdrawn version has no e-print); the source loop and the diff use
  that version. Exercised on no withdrawn case this session; the Session-13 case (`0902.0680`) is
  the test it was written for.
- **Candidate list:** regenerated from the 73 cached pages — 0 API requests, 2,472 candidates,
  digest identical to run 1 (`e164c0b1…2d48`).
- **Candidates examined in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md` rows 7–10):
  - **7, arXiv 0912.0966** (Tao–Vu, Annals of Probability; math.SP), v6 (2026-06-30) "A small
    correction made to the statement of Theorem 8": the diff adds "distinct" to the index set of
    the correlation sum in Theorem 8's statement. Theorem 8 is the sine-kernel result the paper
    says Nagao and Wadati "established" and cites; the paper proves its extension, not it.
    **Rejected, D2.1** (no offered proof to label), the candidate-3 shape again.
  - **8, arXiv 1005.2398** (Alper; math.AG), v3 (2026-03-22) "the addition of a finitely
    presented hypothesis to Proposition 3.3.5": v3's footnotes say the published statements of
    the proposition and of the lemma following it "were incorrect", the proposition's footnote
    giving the counterexample ℤ → ℤ[{x_p}]/(p x_p, x_p^p). **Admitted as two items** (two claims
    of record from one notice): the proposition — FLAW-0012 (statement, fatal) + FLAW-0005 (the
    (2) ⇒ (1) step, fatal), `statement_status` false by **counterexample** (the first use of that
    method); the lemma — the same two classes through its one-line reduction to the proposition,
    false by source document. Shared original (v2, arXiv non-exclusive → `external`) and shared
    S-entries S-0032, S-0033; ids `s1_ingest-v0.1.0-be3b17e73425`, `s1_ingest-v0.1.0-72415457dce4`.
    The lemma's record notes that dependence between the two is the S3 calibration ADR's
    effective-N business (ADR-0004 D6).
  - **9, arXiv 1009.3998** (Green–Tao–Ziegler, Annals of Math.; math.CO), v5 (2026-04-23)
    "corrects a mistake in the proof of Lemma 13.2 (the wrong filtration was specified)": the diff
    replaces the generating sets of a filtration of G³ inside the proof, with a footnote naming
    who found and who fixed it; the lemma is unchanged. **Admitted**: FLAW-0001 `local`
    (repairable; the wrong object written, the intended one's properties asserted) inside
    FLAW-0011; FLAW-0004 named as the alternative reading; `statement_status` true; v4 arXiv
    non-exclusive → `external`; `s1_ingest-v0.1.0-e0c199d3c60a`; S-0034, S-0035. The AI-mention
    scanner's one hit ("llm") is the substring of `\smallmatrix`, checked.
  - **10, arXiv 1010.2391** (Sieber, DCDS-A; math.DS), v10 (2023-10-05) "correcting mistakes in
    formulation of Lemma A.3 …: center x of neighborhood must be in C¹": the statement quantified
    over D^{0,1} while its proof opened "Let x be an element of D¹". **Admitted**: FLAW-0012
    (statement, fatal) + FLAW-0005 (proof, fatal); `statement_status` false by source document,
    with the stricter `undetermined` reading named in the record for Al; v9 arXiv non-exclusive
    → `external`; `s1_ingest-v0.1.0-448d7aefa486`; S-0036, S-0037. v9's own 2020 corrections are
    outside the window (D8 precision).
  - **Counts by verdict, this session:** examined 4 — admitted 3 candidates yielding 4 items,
    rejected 1 (D2.1), found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0.
    **Cumulative:** examined 10 — admitted 7 candidates (8 items), rejected 3. **Last candidate
    processed `1010.2391`**; the session stopped at its budget before candidate 11 (`1102.1087`).
- Each admitted item: S-entries written **before** the producer ran (SD16); config with the D4
  fields; `syndeom-ingest` at `7c952e6` wrote `manifest.json` (`syndeom-manifest/2`,
  `engine_commit` `7c952e6…`, self-check passed); `RECORD.md` with the D3 class argument,
  alternatives considered, quoted locating sentences and licence basis. Two items from one
  candidate share bytes and S-entries and differ in `item` and `flaws`, hence in id.
- STATE: Updated line; Gate S1 rows S1.1 (prefix through `1010.2391`, 10/7(8)/3) and S1.3 (eight
  parents); registers line (S-0032..S-0037); NEXT resumes at candidate 11.
- Gates: the ORIENT run (`gate-14a.log`) finished `GATE: GREEN` on `7c952e6` at 03:47:37Z after
  an eleven-minute cold load. Evidence run on the staged tree, detached (`gate-14b.log`),
  03:47:34Z–03:48:31Z (it began three seconds before 14a's final register step ended; no Lean
  contention): `Build completed successfully (8926 jobs)`; audit `[propext]`; fmt and clippy
  clean; `cargo test` 5/5 + 24/24; `git ls-files --eol` clean; `check_registers.py` 76 tracked
  files clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, **S- 37/37**; `RESULT: ALL PASS`; `GATE:
  GREEN`; `rc=0`. Re-run the same way on the final tree (this entry and STATE's build line in)
  before the commit (STOP-REASON).
BELIEF DELTAS:
- One notice can document several results, and each is its own claim of record: the
  two-items-from-one-candidate case arrived on the eighth candidate. Evidence: candidate 8.
  Consequence: the configs differ in `item` and `flaws`, the bytes and S-entries are shared, and
  the dependence is for the S3 calibration ADR's effective-N rule, not for the harvest.
- The candidate-3 shape (a mis-stated theorem quoted from the literature) recurs — twice in ten
  candidates, both in respected papers; the D2.1 rejection is reusable, and these are the natural
  seeds for a citation-content detector's ground truth in S3. Evidence: candidates 3 and 7.
- Half the admitted items so far (4 of 8) have a false statement of record and half of those
  come with the author's own counterexample or an executor-checkable one; FLAW-0012 with a
  FLAW-0005 use site is the modal shape of a documented erratum in this stream. Evidence: rows
  1–10. Consequence: S1.4's class coverage will be uneven, as §2 S1.4 anticipates; nothing to
  engineer around.
- The AI-mention scan needs a word boundary: `\smallmatrix` contains "llm". Evidence: candidate
  9. Consequence: the hit was checked, not trusted; the scanner's pattern is a cheap improvement
  for the next session (not changed this session, to keep the tool identical across the prefix).
CONFUSIONS:
- Candidate 7 looked like a statement-level FLAW-0012 until the TeX showed Theorem 8 is a cited
  result; the rejection criterion is D2.1, not a classification question. Cost: one extra local
  read. Doc fix: the log row says to check whether the affected theorem is proved in the paper
  before classifying — the same lesson as candidate 3, now applied first.
- Whether candidate 8's second result is a second item or a second flaw record: a second item,
  because it is a distinct claim of record (ADR-0003 D6: one derived artifact per affected
  result). Doc fix: the lemma's record and the log row explain the choice.
HONESTY CHECK: no CLAIM changed status. New SOURCES S-0032..S-0037, all VERIFIED digest-only
(arXiv non-exclusive on every original; CC BY only on the corrected v10 of candidate 10).
Item 7's class (FLAW-0001 local for a misdefined object) and item 8's statement value (false
rather than undetermined) are the executor's readings and are marked as such with the
alternative named. The scanner's false positive is recorded rather than silently dismissed.
Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the window excludes the release by rule; all four
examined papers are 2009–2010 submissions by named human authors; the only AI-mention hit was a
TeX macro substring. No detector or threshold exists. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The four manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `7c952e6`; the ids reproduced independently in
Python; the candidate list reproduced from cache with the identical digest; every retrieval is
pinned by URL, access time and SHA-256. Nothing to quarantine.
MANIFEST CHECK: four artifacts created, each with a `syndeom-manifest/2` manifest carrying ground
truth with derivation `documented-by-source`, flaw records with `raw/1` locators, `correct:
false`, taxonomy 0.1.0, self-checked by the producer before writing (SD1); one uses the
`counterexample` method with the author's counterexample as its reference. Nothing synthetic
labeled real (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3): each record and
row names only the result the notice names; readings and alternatives are labelled as the
executor's. No understanding claims (SD17). No detectors, so no detector-shopping. No referee
contact (SD13). Candidate 11 was not opened.
NEXT-SINGLE-ACTION: S1.1 — the fourth Route A harvest session, resuming at candidate 11
(`1102.1087`, v11, cs.DM primary) in ascending id with no skipping, exactly as STATE.md states
it; candidate 11's comment is unusual (a long self-critical note on an algorithm and several
theorems) — read it against D2.1–D2.7 like any other, and log the verdict with its criterion.
ESCALATIONS:
- None new: no grey zone arose in candidates 7–10. One reading is flagged for Al's attention in
  item 8's record, not as a blocker: the v9 statement of candidate 10's Lemma A.3 is recorded
  `false` on the author's "must be in C¹"; the stricter `undetermined` is the alternative.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers (S-0019).
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — a clean boundary (four candidates fully recorded, the resume point
set). Repo green on the committed tree (gate run detached with a log, every edit staged, before
the commit); this commit pushes to `origin/main`.

SESSION: 2026-10-14 UTC @ 335344f (Session 15 — the fourth Route A harvest: candidates 11–12
examined, 1 admitted yielding 2 items, 1 found-but-unclassifiable; the commit adding this entry
lands on top of 335344f)
LAUNCH CONTEXT (SD15; not an exam session): Al's one-word order "resume" in the same conversation
as Sessions 7–14; the inherited NEXT-SINGLE-ACTION executed as STATE stated it. Not cold. No
memory written.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: prefix through arXiv id
`1107.0863` (12 of 2,472 candidates; 8 admitted yielding 10 items, 3 rejected, 1
found-but-unclassifiable); S1.3: ten `raw/1` parents manifested, no derived artifact yet. Gate S0
closed and confirmed.
PLAN NOTE: the inherited NEXT (the fourth harvest session, resuming at candidate 11), unchanged
by ORIENT. One thing.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `335344f`, clean; the constitution unchanged since `226a51c`
  (read in full in Session 11 of this conversation); the log's resume point read. Evidence gate at
  ORIENT, detached (`gate-15a.log`): see the gate line below.
- **Candidate list:** regenerated from the 73 cached pages — 0 API requests, 2,472 candidates,
  digest identical to run 1 (`e164c0b1…2d48`).
- **Candidates examined in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md` rows 11–12):
  - **11, arXiv 1102.1087** ("Graph Theory", one author, cs.DM primary with a math cross-list):
    the correcting version v11 (2024-01-01) is itself a withdrawal stub — the submission history
    marks versions 2, 5, 9 and 11 withdrawn and v11 has no e-print — whose comment lists problems
    ("The algorithm in Section 5.3 does not appear to follow the Turing model … Theorem 29 was not
    stated properly … It is not clear if this list of errors is comprehensive") without
    describing any error, and there is no corrected version to confirm against. The helper took
    v10 as the original (the withdrawal handling added in Session 14 worked) and then stopped on
    v11's missing e-print; v10 was read. **Found-but-unclassifiable, D2.4** — the first of that
    verdict; nothing stored.
  - **12, arXiv 1107.0863** (Chepoi–Hagen; math.MG), v4 (2026-04-22) "Previous version had an
    error in Lemma 12, affecting Theorem 1. Current version has appendix correcting Theorem 1
    under additional hypothesis … Appendix appears as journal correction" (a JCTB DOI quoted in
    the comment): the appendix states Lemma 12 "is false as stated" with a counterexample figure
    (a 5-cycle in Υ₀(U) without a normal hyperplane) and Theorem 1 "incorrect in the stated
    generality", corrected by adding the hypothesis that no vertex link contains a 5-cycle.
    **Admitted as two items**: Lemma 12 — FLAW-0012 (fatal), `statement_status` false by the
    authors' **counterexample**, the failing proof step not named by the notice and deliberately
    not guessed; Theorem 1 — FLAW-0012 (fatal) through its reliance on Lemma 12,
    `statement_status` false by source document with the `undetermined` reading named in the
    record (the disproving examples are attributed to a forthcoming paper). Shared original (v3,
    4.2 MB with figures, arXiv non-exclusive → `external`) and shared S-entries S-0038, S-0039;
    ids `s1_ingest-v0.1.0-1c4c9a7747d0`, `s1_ingest-v0.1.0-2d93a266558a`. The scanner's "llm"
    hit is the citation key `BallmanSwiatkowski`, checked.
  - **Counts by verdict, this session:** examined 2 — admitted 1 candidate yielding 2 items,
    rejected 0, found-but-unclassifiable 1, pending 0, pending-truth 0, escalated 0.
    **Cumulative:** examined 12 — admitted 8 candidates (10 items), rejected 3,
    found-but-unclassifiable 1. **Last candidate processed `1107.0863`**; the session stopped at
    its budget before candidate 13 (`1109.1964`).
- Each admitted item: S-entries written **before** the producer ran (SD16); config with the D4
  fields; `syndeom-ingest` at `335344f` wrote `manifest.json` (`syndeom-manifest/2`,
  `engine_commit` `335344f…`, self-check passed); `RECORD.md` with the D3 class argument, the
  alternatives considered, quoted locating sentences and licence basis.
- **Helper limit found:** a correcting version that is itself a withdrawal has no e-print; the
  helper prints the abstract-page data and then fails on the source fetch (candidate 11). Not
  fixed this session (the abstract data sufficed for the verdict); the next session may add a
  `--no-sources` fallback when the correcting version is withdrawn.
- STATE: Updated line; Gate S1 rows S1.1 (prefix through `1107.0863`, 12/8(10)/3/1) and S1.3 (ten
  parents); registers line (S-0038..S-0039); NEXT resumes at candidate 13, with a note on how to
  treat its "superseded by" comment.
- Gates: the ORIENT run (`gate-15a.log`) on the clean tree, 03:52:24Z–04:03:12Z (cold load),
  `GATE: GREEN`. Evidence run on the staged tree, detached (`gate-15b.log`), 03:58:38Z–04:03:12Z,
  overlapping the ORIENT run's tail without harm: `Build completed successfully (8926 jobs)`;
  audit `[propext]`; fmt and clippy clean; `cargo test` 5/5 + 24/24; `git ls-files --eol` clean;
  `check_registers.py` 80 tracked files clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, **S-
  39/39**; `RESULT: ALL PASS`; `GATE: GREEN`; `rc=0`. Re-run the same way on the final tree (this
  entry and STATE's build line in) before the commit (STOP-REASON). Lesson for the next session:
  do not launch the ORIENT gate and the pre-commit gate within ten minutes of each other on a cold
  cache; wait for the first to end.
BELIEF DELTAS:
- A withdrawal with a list of complaints is not a status document that locates and describes an
  error: the found-but-unclassifiable verdict exists for exactly this, and the first such
  candidate was a long-revised paper whose author withdrew it with doubts about his own
  computational model. Evidence: candidate 11. Consequence: D2.4 is the operative criterion;
  nothing is guessed.
- The strongest status documents are journal corrections republished on arXiv as appendices:
  they name the false lemma, draw the counterexample, and state which other results stand.
  Evidence: candidate 12. Consequence: two items with the cleanest provenance so far; the
  counterexample method applies to the lemma, the document method to the theorem.
- Not guessing the failing step is sometimes the right record: when the notice locates an error
  to a proof but a counterexample settles the statement, FLAW-0012 alone is the honest class and
  a use-site record would be invention. Evidence: item 9's record versus item 3's (where the
  executor's step-level reading was given with a caveat because no counterexample existed).
  Consequence: the two forms are both in the records, each labelled.
CONFUSIONS:
- The helper's withdrawal handling covers a withdrawn *original* but not a withdrawn *correcting*
  version. Cost: one traceback; the verdict needed only the abstract pages. Doc fix: this entry
  and the log row; a `--no-sources` fallback is the next session's option.
HONESTY CHECK: no CLAIM changed status. New SOURCES S-0038, S-0039, VERIFIED digest-only (arXiv
non-exclusive on both versions). Item 10's statement value (false rather than undetermined) is
the executor's reading of "incorrect in the stated generality" and says so, with the alternative
named. Item 9 records no use-site flaw rather than a guessed one. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the window excludes the release by rule; both
examined papers are 2011 submissions by named human authors; the only AI-mention hit was a
citation-key substring. No detector or threshold exists. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The two manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `335344f`; the ids reproduced independently in
Python; the candidate list reproduced from cache with the identical digest; every retrieval is
pinned by URL, access time and SHA-256. Nothing to quarantine.
MANIFEST CHECK: two artifacts created, each with a `syndeom-manifest/2` manifest carrying ground
truth with derivation `documented-by-source`, a flaw record with a `raw/1` locator, `correct:
false`, taxonomy 0.1.0, self-checked by the producer before writing (SD1); one uses the
`counterexample` method. Nothing synthetic labeled real (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3): the records
quote the authors' own words for both falsities and name the alternative reading where the
document leaves room. No understanding claims (SD17). No detectors. No referee contact (SD13).
Candidate 13 was not opened.
NEXT-SINGLE-ACTION: S1.1 — the fifth Route A harvest session, resuming at candidate 13
(`1109.1964`, v2, "Crucial mistake in Proposition 2.1. Article superseded by …") in ascending id
with no skipping, exactly as STATE.md states it, including the note on locating the superseding
paper before deciding.
ESCALATIONS:
- None new. Item 10's record flags one reading for Al's attention, not as a blocker: Theorem 1
  of candidate 12 is recorded `false` on the authors' "incorrect in the stated generality";
  `undetermined` is the alternative.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers (S-0019).
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — a clean boundary (two candidates fully recorded, the resume point
set). Repo green on the committed tree (gate run detached with a log, every edit staged, before
the commit); this commit pushes to `origin/main`.

SESSION: 2026-10-14 UTC @ be22f72 (Session 16 — the fifth Route A harvest: candidates 13–15
examined, 2 admitted yielding 3 items, 1 found-but-unclassifiable; the commit adding this entry
lands on top of be22f72)
LAUNCH CONTEXT (SD15; not an exam session): the same conversation as Sessions 7–15, continued
across a context compaction; the inherited NEXT-SINGLE-ACTION executed as STATE stated it,
including its note on locating the superseding paper for candidate 13. Not cold. No memory
written.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: prefix through arXiv id
`1201.3327` (15 of 2,472 candidates; 10 admitted yielding 13 items, 3 rejected, 2
found-but-unclassifiable); S1.3: thirteen `raw/1` parents manifested, no derived artifact yet.
Gate S0 closed and confirmed.
PLAN NOTE: the inherited NEXT (the fifth harvest session, resuming at candidate 13), unchanged
by ORIENT. One thing.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `be22f72`, clean; the constitution unchanged since `226a51c`;
  the log's resume point read. Evidence gate at ORIENT, detached (`gate-16a.log`),
  04:52:05Z–05:02:27Z (cold load), `GATE: GREEN`.
- **Candidate list:** regenerated from the 73 cached pages — 0 API requests, 2,472 candidates,
  digest identical to run 1 (`e164c0b1…2d48`).
- **Candidates examined in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md` rows 13–15):
  - **13, arXiv 1109.1964** (Loubeau and Vergara-Diaz; math.DG): v2 (2023-10-17) is a withdrawal
    stub — "Crucial mistake in Proposition 2.1. Article superseded by 'Harmonicity of normal
    almost contact metric structures', with four authors …". As NEXT directed, the superseding
    paper was located by title (arXiv:2310.11089, four authors, CC BY 4.0, v1 2023-10-17, v2
    2026-03-02) and read: it does not describe the mistake (a referee acknowledgement only).
    Proposition 2.1 of v1 is located (the covariant-derivative characterisation of normality),
    but no document says what is wrong with it and no corrected version of the paper exists.
    **Found-but-unclassifiable, D2.4** — the second such verdict; nothing stored. The helper
    failed on the withdrawn correcting version's missing e-print as on candidate 11 (the limit
    noted in Session 15, still unfixed; the abstract data sufficed).
  - **14, arXiv 1112.6107** (Hamenstädt; math.DS), v3 (2025-04-13) "Correction of the last
    statement in Theorem 1, proofs are not affected. Details added, writing improved": the diff
    shows the last clause of Theorem 1 changing from σ-invariant measures on the base 𝒰 to
    Θ^t-invariant measures on the suspension X (the space the semi-conjugacy is defined on), and
    the introduction now calls the semi-conjugacy partial, "not surjective". **Admitted**:
    FLAW-0012 (fatal), `statement_status` false by source document; the record names the
    stricter reading (a misstatement rather than a falsity → `undetermined`) as the alternative
    for Al. Original v2 (itself the 2023 revision splitting the 2011 paper into three parts;
    arXiv non-exclusive → `external`); S-0040, S-0041; id `s1_ingest-v0.1.0-e3cbdfa7ad1c`. The
    scanner's "llm" hit is the TeX label `fillmore`, checked.
  - **15, arXiv 1201.3327** (Pottmeyer; math.NT, published), v3 (2023-05-05) "There was an error
    in the proof of the former Lemma 5.8. This false lemma and the former Theorem 5.9 have been
    deleted in this version"; v3's text says a counterexample by Amoroso and Terracini proves
    "that the whole statement was false" and "I am not able to classify all these field
    extensions". **Admitted as two items**: the former Lemma 5.8 — FLAW-0012 (fatal),
    `statement_status` false by source document (the counterexample is not in the document, so
    not the counterexample method), the failing proof step not named by the notice and
    deliberately not guessed; the former Theorem 5.9 — FLAW-0005 (gap: its proof combines the
    false lemma with the preceding results), **`statement_status` null / `undetermined`** —
    deleted and replaced by a weaker proposition, neither re-proved nor disproved: the second use
    of Al's ruling (ii). Shared original v2 (arXiv non-exclusive → `external`) and shared
    S-entries S-0042, S-0043; ids `s1_ingest-v0.1.0-22ae000cb879`,
    `s1_ingest-v0.1.0-fd2bfef40d31`. The scanner's "llm" hit is the surname Ullmo, checked.
    v2's own 2013 corrections are outside the window (D8 precision).
  - **Counts by verdict, this session:** examined 3 — admitted 2 candidates yielding 3 items,
    rejected 0, found-but-unclassifiable 1, pending 0, pending-truth 0, escalated 0.
    **Cumulative:** examined 15 — admitted 10 candidates (13 items), rejected 3,
    found-but-unclassifiable 2. **Last candidate processed `1201.3327`**; the session stopped at
    its budget before candidate 16 (`1202.5531`).
- Each admitted item: S-entries written **before** the producer ran (SD16); config with the D4
  fields; `syndeom-ingest` at `be22f72` wrote `manifest.json` (`syndeom-manifest/2`,
  `engine_commit` `be22f72…`, self-check passed; the ids reproduced independently in Python
  before the producer ran); `RECORD.md` with the D3 class argument, the alternatives considered,
  quoted locating sentences and licence basis.
- STATE: Updated line; Gate S1 rows S1.1 (prefix through `1201.3327`, 15/10(13)/3/2) and S1.3
  (thirteen parents); registers line (S-0040..S-0043); build line; NEXT resumes at candidate 16
  with the next three candidates' comments quoted and the withdrawn-correcting-version note.
- Gates: the ORIENT run (`gate-16a.log`) on the clean tree, 04:52:05Z–05:02:27Z (cold load),
  `GATE: GREEN`. Evidence run on the staged tree, detached (`gate-16b.log`), 05:07:44Z–05:09:07Z, launched
  only after the ORIENT run had ended (the Session-15 lesson): `Build completed successfully
  (8926 jobs)`; audit `[propext]`; fmt and clippy clean; `cargo test` 5/5 + 24/24; `git ls-files
  --eol` clean; `check_registers.py` 86 tracked files clean, Q 17/17, SD 17/17, FLAW
  14/14, CLAIM 7/7, **S- 43/43**; `RESULT: ALL PASS`; `GATE: GREEN`; `rc=0`. Re-run the same way
  on the final tree (this entry and STATE's build line in) before the commit (STOP-REASON).
BELIEF DELTAS:
- A "superseded by" withdrawal is not made classifiable by the superseding paper unless that
  paper describes the mistake: here the four-author successor acknowledges a referee and says
  nothing about what was wrong. Evidence: candidate 13. Consequence: the second
  found-but-unclassifiable, by the same D2.4 rule as the first; the NEXT note's procedure
  (locate, read, decide) worked and cost one extra retrieval.
- "Correction of the statement, proofs not affected" is the FLAW-0012 shape with a built-in
  ambiguity: the author asserts the proofs prove the corrected statement, which makes the
  original statement false as a reading of what was proved, but the author never says "false".
  Evidence: candidate 14. Consequence: recorded `false` with the `undetermined` alternative
  named, as for items 8 and 10 — a pattern now frequent enough to put to Al (escalation below).
- The deletion of a theorem whose only support was a false lemma is the clean undetermined case:
  the author withdraws the claim without disproving it. Evidence: candidate 15, Theorem 5.9.
  Consequence: the second `value: null` item, classed FLAW-0005 (the false lemma as the failed
  premise) rather than FLAW-0012, and the dependence between the two items noted in the record
  for the S3 effective-N rule.
CONFUSIONS:
- The helper's withdrawn-correcting-version limit (Session 15) hit again on candidate 13. Cost:
  one traceback, as before; the abstract pages sufficed. Doc fix: STATE's NEXT now says what to
  do; the `--no-sources` fallback is still the next session's option, deliberately not built
  mid-harvest so the examination procedure stays identical across the prefix.
- The AI-mention scanner's substring hits ("llm" in `fillmore` and `Ullmo` this session) now
  number five across the prefix, all false positives, each checked by hand. Cost: a grep each;
  a word-boundary fix is deferred for the same reason.
HONESTY CHECK: no CLAIM changed status. New SOURCES S-0040..S-0043, VERIFIED digest-only (arXiv
non-exclusive on all four versions). Item 11's statement value (false rather than undetermined)
is the executor's reading of "correction of the last statement" and says so, with the
alternative named. Item 12 records no failing step rather than a guessed one; item 13's truth is
null because the document leaves it open. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the window excludes the release by rule; the
three examined papers are 2011–2012 submissions by named human authors, and the superseding paper
read for candidate 13 is a 2023 four-author paper; the only AI-mention hits were substrings. No
detector or threshold exists. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The three manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `be22f72`; the ids reproduced independently in
Python before the producer ran; the candidate list reproduced from cache with the identical
digest; every retrieval is pinned by URL, access time and SHA-256. Nothing to quarantine.
MANIFEST CHECK: three artifacts created, each with a `syndeom-manifest/2` manifest carrying ground
truth with derivation `documented-by-source`, a flaw record with a `raw/1` locator, `correct:
false`, taxonomy 0.1.0, self-checked by the producer before writing (SD1); one carries
`statement_status.value: null` with `method: "undetermined"` (ADR-0001 as amended). Nothing
synthetic labeled real (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3): the records quote
the authors' words and name the alternative reading where the document leaves room; the
superseding paper for candidate 13 was read for what it says about the mistake and nothing else.
No understanding claims (SD17). No detectors. No referee contact (SD13). Candidate 16 was not
opened.
NEXT-SINGLE-ACTION: S1.1 — the sixth Route A harvest session, resuming at candidate 16
(`1202.5531`, v4 of 2026-08-23, "The main irreducibility theorem in the previous version is false
in the stated generality. The error occurs in Step 4 of the proof of Theorem 6 …") in ascending id
with no skipping, exactly as STATE.md states it.
ESCALATIONS:
- For Al's attention, not a blocker: three records now carry `false` with `undetermined` named
  as the alternative where the author writes "corrected" or "incorrect in the stated generality"
  rather than "false" (items 8, 10 and 11). A standing ruling either way would make the three
  records uniform; the executor's reading is `false` because a statement that had to be changed
  is not what the argument established.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers
  (S-0019); not attempted this session.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — a clean boundary (three candidates fully recorded, the resume point
set). Repo green on the committed tree (gate run detached with a log, every edit staged, before
the commit); this commit pushes to `origin/main`.

SESSION: 2026-10-14 UTC @ bc12219 (Session 17 — the sixth Route A harvest: candidates 16–18
examined, all three admitted yielding 4 items; the commit adding this entry lands on top of
bc12219)
LAUNCH CONTEXT (SD15; not an exam session): Al's one-word order "resume" in the same conversation
as Sessions 7–16, about three hours after the Session-16 commit; the inherited
NEXT-SINGLE-ACTION executed as STATE stated it. Not cold. No memory written.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: prefix through arXiv id
`1206.2228` (18 of 2,472 candidates; 13 admitted yielding 17 items, 3 rejected, 2
found-but-unclassifiable); S1.3: seventeen `raw/1` parents manifested, no derived artifact yet.
Gate S0 closed and confirmed.
PLAN NOTE: the inherited NEXT (the sixth harvest session, resuming at candidate 16), unchanged
by ORIENT. One thing.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `bc12219`, clean; the constitution unchanged since `226a51c`;
  the log's resume point read. Evidence gate at ORIENT, detached (`gate-17a.log`),
  08:08:51Z–08:18:18Z (cold load after three idle hours), `GATE: GREEN`.
- **Candidate list:** regenerated from the 73 cached pages — 0 API requests, 2,472 candidates,
  digest identical to run 1 (`e164c0b1…2d48`).
- **Candidates examined in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md` rows 16–18):
  - **16, arXiv 1202.5531** (Massri; math.AG, a preprint), v4 (2026-08-23) "Major revision. The
    main irreducibility theorem in the previous version is false in the stated generality. The
    error occurs in Step 4 of the proof of Theorem 6. The present version replaces the incorrect
    general statement by a valid result … under suitable hypotheses": v4 is a different paper
    under a new title, so the diff localises nothing beyond the comment; v4's introduction says
    quadratic hulls "need not be irreducible" and exhibits reducible ones without naming the old
    theorem. **Admitted**: Theorem 6 ("The variety M_y is irreducible", for every representation
    and every y) — FLAW-0012 (fatal), `statement_status` false by source document with the
    `undetermined` reading named as the alternative; the author locates the error to Step 4 (the
    hypersurface claim) without saying what fails there, and the record gives the executor's
    reading (Terracini's lemma applied at a special rather than a general point) labelled as
    the executor's. Original v3 (arXiv non-exclusive → `external`); S-0044, S-0045; id
    `s1_ingest-v0.1.0-dc1e44f39bb9`.
  - **17, arXiv 1206.1893** (Tao and Vu; math.PR; Annals of Probability, v6 the published
    version), v7 (2024-05-25) "notes an error in Proposition 34, and indicates a workaround so as
    not to impact the remainder of the paper": the one addition is a "(Note added after
    publication.)" after Proposition 35 — "Proposition 34 is incorrect; the random variable Y′
    constructed above does not, in fact, obey the hypotheses of Azuma's inequality. A version of
    the proposition can be salvaged by replacing C_i … with the larger quantity sup_η C_i …;
    however, for the purposes of establishing Proposition 35 … it is better to proceed as
    follows. Firstly, one should impose an additional mild moment hypothesis …". **Admitted as
    two items**: Proposition 34 — FLAW-0012 (fatal) + FLAW-0005 (Azuma's inequality applied to
    Y′, fatal), `statement_status` false by source document, `undetermined` named as the
    alternative; Proposition 35 ("Martingale concentration", the result the rest of the paper
    uses) — FLAW-0005 (gap: its proof applies Proposition 34), **`statement_status` null /
    `undetermined`** — re-established only under an added hypothesis, the third use of Al's
    ruling (ii). Shared original v6 (the publisher's TeX; arXiv non-exclusive → `external`) and
    shared S-entries S-0046, S-0047; ids `s1_ingest-v0.1.0-68ed1abfecb4`,
    `s1_ingest-v0.1.0-a4822a4be06e`. The scanner reports no hit; the helper was run twice (the
    first run's summary was lost to a display filter; the digests are from the second).
  - **18, arXiv 1206.2228** (Beeson; math.MG, a 46-page preprint), v2 (2024-05-27) a withdrawal
    with the comment "Theorem 1 is wrong; tilings described in later papers contradict it. Line
    4, page 8, is the error in the alleged proof. This paper is not used in any of my other
    papers on triangle tiling": the comment describes the error and locates it, which the
    withdrawals of candidates 11 and 13 did not; the v1 PDF was retrieved (one extra request) to
    resolve the page-and-line reference to the kernel argument's sentence "Now consider the third
    vector (c, 0, −a), which is also in 𝒟. It must be a linear combination of the first two".
    **Admitted**: Theorem 1 ("every row of the 𝐝 matrix has a zero") — FLAW-0012 (fatal),
    `statement_status` false by source document (the contradicting tilings are in unnamed later
    papers, not retrieved); the executor's reading of the mechanism ((0, b, −c) is not
    orthogonal to (a, b, c)) is in the record, labelled. **First item with `status.kind`
    `retraction`** (the producer accepts `erratum` or `retraction`; a withdrawal is the latter).
    Original v1 (arXiv non-exclusive → `external`); S-0048, S-0049; id
    `s1_ingest-v0.1.0-da3a4784c2be`. The helper failed on the withdrawn v2's missing e-print as
    before; the abstract pages and v1 sufficed.
  - **Counts by verdict, this session:** examined 3 — admitted 3 candidates yielding 4 items,
    rejected 0, found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0.
    **Cumulative:** examined 18 — admitted 13 candidates (17 items), rejected 3,
    found-but-unclassifiable 2. **Last candidate processed `1206.2228`**; the session stopped at
    its budget before candidate 19 (`1206.5522`).
- Each admitted item: S-entries written **before** the producer ran (SD16); config with the D4
  fields; `syndeom-ingest` at `bc12219` wrote `manifest.json` (`syndeom-manifest/2`,
  `engine_commit` `bc12219…`, self-check passed; the ids reproduced independently in Python
  before the producer ran; the four manifests re-read and their schema, commit, truth values and
  methods asserted); `RECORD.md` with the D3 class argument, the alternatives considered, quoted
  locating sentences and licence basis.
- STATE: Updated line; Gate S1 rows S1.1 (prefix through `1206.2228`, 18/13(17)/3/2) and S1.3
  (seventeen parents); registers line (S-0044..S-0049); build line; NEXT resumes at candidate 19
  with the next two candidates' comments quoted, the withdrawn-correcting-version note extended
  (candidate 18 admitted from the abstract pages and the original) and the PDF page-and-line
  method noted.
- Gates: the ORIENT run (`gate-17a.log`) on the clean tree, 08:08:51Z–08:18:18Z (cold load),
  `GATE: GREEN`. Evidence run on the staged tree, detached (`gate-17b.log`), 08:30:22Z–08:30:47Z, launched
  after the ORIENT run had ended: `Build completed successfully (8926 jobs)`; audit `[propext]`;
  fmt and clippy clean; `cargo test` 5/5 + 24/24; `git ls-files --eol` clean;
  `check_registers.py` 94 tracked files clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7,
  **S- 49/49**; `RESULT: ALL PASS`; `GATE: GREEN`; `rc=0`. Re-run the same way on the final tree
  (this entry and STATE's build line in) before the commit (STOP-REASON).
BELIEF DELTAS:
- A withdrawal can be a complete status document: when its comment names the result, says it is
  false and why, and locates the error, the found-but-unclassifiable verdict does not apply —
  the rule was never "withdrawals are unclassifiable" but D2.4. Evidence: candidate 18 against
  candidates 11 and 13. Consequence: the first `retraction`-kind item; the helper's limit on
  withdrawn correcting versions is a nuisance, not a blocker, and the next session may still add
  the `--no-sources` fallback.
- The strongest post-publication notes name the failed hypothesis: "does not, in fact, obey the
  hypotheses of Azuma's inequality" is FLAW-0005 in the authors' own words, and the two-item
  shape (false auxiliary result, dependent result re-established under an added hypothesis)
  recurs. Evidence: candidate 17 against candidate 15. Consequence: the third undetermined item,
  with the same FLAW-0005-gap reading as item 13.
- A complete rewrite is the weakest kind of corrected version for localisation: it confirms only
  that the general claim is not re-proved, and the diff is useless; the comment carries the whole
  location. Evidence: candidate 16. Consequence: the record quotes the comment, gives the
  executor's step-level reading labelled as such, and names the `undetermined` alternative — the
  fifth record of that shape (escalation below).
CONFUSIONS:
- The helper takes the correcting version as a bare number; "v4" raised a `ValueError`. Cost:
  one re-run. Doc fix: this entry; STATE's NEXT already writes the bare form.
- A display filter (`grep -v '^[-+@ ]'`, meant to drop diff lines) also dropped the helper's
  summary lines for candidate 17, which begin with "--"; the helper was re-run (four requests).
  Cost: one re-run and a 20-second wait. Doc fix: this entry; filter on the diff header instead
  (`awk '/^--- v6/{skip=1} /more diff lines/{skip=0; next} !skip'` worked for candidate 18).
- The helper's withdrawn-correcting-version limit hit a third time (candidate 18). Not fixed
  mid-harvest, deliberately, so the examination procedure stays identical across the prefix.
HONESTY CHECK: no CLAIM changed status. New SOURCES S-0044..S-0049, VERIFIED digest-only (arXiv
non-exclusive on every version with an e-print; the withdrawn stub shows no licence). Items 14,
15 and 17 record `false` on the authors' "false in the stated generality", "incorrect" and
"wrong" with the alternative named; item 16's truth is null because the authors re-establish it
only under an added hypothesis. The executor's step-level readings for items 14 and 17 are
labelled as the executor's and are not the basis of any class. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the window excludes the release by rule; the
three examined papers are 2012 submissions by named human authors; the scanner reports no
AI-assistance mention on any version read. No detector or threshold exists. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The four manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `bc12219`; the ids reproduced independently in
Python before the producer ran; the candidate list reproduced from cache with the identical
digest; every retrieval is pinned by URL, access time and SHA-256 (the v1 PDF of candidate 18 to
the minute). Nothing to quarantine.
MANIFEST CHECK: four artifacts created, each with a `syndeom-manifest/2` manifest carrying ground
truth with derivation `documented-by-source`, flaw records with `raw/1` locators, `correct:
false`, taxonomy 0.1.0, self-checked by the producer before writing (SD1); one carries
`statement_status.value: null` with `method: "undetermined"` (ADR-0001 as amended); one carries
`status.kind: "retraction"`, the first. Nothing synthetic labeled real (SD12). "Proven" applied
to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3): the records quote
the authors' words, name the alternative reading where the document leaves room, and label the
executor's mechanism readings as the executor's. No understanding claims (SD17). No detectors. No
referee contact (SD13). Candidate 19 was not opened.
NEXT-SINGLE-ACTION: S1.1 — the seventh Route A harvest session, resuming at candidate 19
(`1206.5522`, v6 of 2024-06-23, math.AT, "37 pages. Corrected Proposition 5.3") in ascending id
with no skipping, exactly as STATE.md states it.
ESCALATIONS:
- For Al's attention, not a blocker (carried from Session 16, now five records): items 8, 10, 11,
  14 and 15 carry `false` with `undetermined` named as the alternative where the author writes
  "corrected", "incorrect" or "false in the stated generality" rather than giving a
  counterexample. A standing ruling either way would make the five records uniform; the
  executor's reading is `false` because in each case the author replaces the statement by a
  different one.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers
  (S-0019); not attempted this session.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — a clean boundary (three candidates fully recorded, the resume point
set). Repo green on the committed tree (gate run detached with a log, every edit staged, before
the commit); this commit pushes to `origin/main`.

SESSION: 2026-10-14 UTC @ fd309a5 (Session 18 — the seventh Route A harvest: candidates 19–21
examined, 2 admitted yielding 3 items, 1 rejected; the commit adding this entry lands on top of
fd309a5)
LAUNCH CONTEXT (SD15; not an exam session): Al's one-word order "resume" in the same conversation
as Sessions 7–17 (continued after a context compaction), about eleven hours after the Session-17
commit; the inherited NEXT-SINGLE-ACTION executed as STATE stated it. Not cold. Memory: two
machine hazards added to the executor's memory file (no program state; SD15).
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: prefix through arXiv id
`1209.2022` (21 of 2,472 candidates; 15 admitted yielding 20 items, 4 rejected, 2
found-but-unclassifiable); S1.3: twenty `raw/1` parents manifested, no derived artifact yet.
Gate S0 closed and confirmed.
PLAN NOTE: the inherited NEXT (the seventh harvest session, resuming at candidate 19), unchanged
by ORIENT. One thing.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `fd309a5`, clean; the constitution unchanged since `226a51c`;
  the log's resume point read. Evidence gate at ORIENT, detached (`gate-18a.log`),
  19:44:55Z–19:55:18Z (cold load after eleven idle hours), `GATE: GREEN`.
- **Candidate list:** regenerated from the 73 cached pages — 0 API requests, 2,472 candidates,
  digest identical to run 1 (`e164c0b1…2d48`).
- **Candidates examined in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md` rows 19–21):
  - **19, arXiv 1206.5522** (Ayala and Francis; math.AT; J. Topol. 8 (2015), the original v5 the
    accepted version), v6 (2024-06-23) "37 pages. Corrected Proposition 5.3", with the
    acknowledgement's new sentence "We thank Amabel Wilson and Theo Johnson–Freyd for correcting
    the hypotheses in the statement of Proposition 5.3": the 61-line diff shows the hypotheses
    "X a nilpotent space whose first n homotopy groups are finite … R any commutative ring"
    replaced by "X a nilpotent n-connective space of finite type over R such that π_n X is
    finite", and the proof's convergence step rewritten so that Dwyer's criterion (a connected
    base, a nilpotent π₁-action) is derived from n-connectivity. **Admitted**: Proposition 5.3 —
    FLAW-0012 (fatal) + FLAW-0005 (Dwyer's theorem applied without its hypotheses, fatal),
    `statement_status` false by source document with the `undetermined` reading named as the
    alternative; the executor's reading of which hypothesis fails (the base X^{S^k} need not be
    connected under the old hypotheses) is in the record, labelled. Original v5 (arXiv
    non-exclusive → `external`); S-0050, S-0051; id `s1_ingest-v0.1.0-685b874cf579`. The v6
    abstract page was re-fetched once (byte-identical) to read the journal fields.
  - **20, arXiv 1208.2806** (Maakestad; math.AG; CC BY-NC-SA 4.0), v14 (2023-11-29) with a
    cumulative comment "Example 2.21 added. A minor correction to Example 3.10. September 2022: …
    March 2023: … Nov 2023: Example 4.5 and Theorem 4.6 on Calabi-Yau manifolds added": the
    abstract pages of v9–v12 were fetched (four requests) and show the correction sentence
    already on v9 (2020-11-12); the in-window v13→v14 diff (43 lines) adds an example, a theorem
    and a note and corrects nothing. **Rejected (D8)**: the correcting version is outside the
    window; D2.1 and D2.3 would fail as well (an Example, "minor correction"). Nothing stored.
  - **21, arXiv 1209.2022** (Galindo; math.QA; a seven-page note), v3 (2024-12-06) "In this
    version, Proposition 3.1, which discussed the equality of the unitary center and the ordinary
    center, has been removed. While these centers are equivalent as braided fusion categories,
    the unitary center is generally a non-replete subcategory. The proof of Theorem 3.2 has been
    revised to avoid reliance on this proposition", with an acknowledgement to Tian Lan and a
    remark in the proposition's place that prints the counterexample (Hilb_G: URep(G) ⊊ Rep(G)
    inside the center, URep(G) inside the unitary center). **Admitted as two items**:
    Proposition 3.1 ("𝒵*(𝒞) = 𝒵(𝒞)") — FLAW-0012 (fatal), `statement_status` false by the
    `counterexample` method (the second use, after item 9), the failing step not named by the
    notice and the executor's reading (the polar decomposition's unitary factor taken to be a
    morphism of the center) labelled; Theorem 3.2 ("Every braiding of a unitary fusion category
    is unitary", the note's first main result) — FLAW-0011 (gap) + FLAW-0005 (gap: the proof's
    one step applies Proposition 3.1), `statement_status` **true** by source document (the
    statement unchanged, re-proved directly in the center) — the second true-statement item,
    after item 2. Shared original v2 (arXiv non-exclusive → `external`) and shared S-entries
    S-0052, S-0053; ids `s1_ingest-v0.1.0-00f28e252823`, `s1_ingest-v0.1.0-a93badadec27`. The v3
    abstract page was re-fetched once (byte-identical; no journal fields).
  - **Counts by verdict, this session:** examined 3 — admitted 2 candidates yielding 3 items,
    rejected 1, found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0.
    **Cumulative:** examined 21 — admitted 15 candidates (20 items), rejected 4,
    found-but-unclassifiable 2. **Last candidate processed `1209.2022`**; the session stopped at
    its budget before candidate 22 (`1209.4040`).
- Each admitted item: S-entries written **before** the producer ran (SD16); config with the D4
  fields; `syndeom-ingest` at `fd309a5` wrote `manifest.json` (`syndeom-manifest/2`,
  `engine_commit` `fd309a5…`, self-check passed; the ids reproduced independently in Python
  before the producer ran; the three manifests re-read and their schema, commit, derivation,
  flaw classes and severities, truth values and methods asserted); `RECORD.md` with the D3 class
  argument, the alternatives considered, quoted locating sentences and licence basis.
- STATE: Updated line; Gate S1 rows S1.1 (prefix through `1209.2022`, 21/15(20)/4/2) and S1.3
  (twenty parents); registers line (S-0050..S-0053); build line; NEXT resumes at candidate 22
  with the next three candidates' comments quoted and the cumulative-comment D8 lesson added.
- Gates: the ORIENT run (`gate-18a.log`) on the clean tree, 19:44:55Z–19:55:18Z (cold load),
  `GATE: GREEN`. Evidence run on the staged tree, detached (`gate-18b.log`), 20:05:35Z–20:09:55Z, launched
  after the ORIENT run had ended: `Build completed successfully (8926 jobs)`; audit `[propext]`;
  fmt and clippy clean; `cargo test` 5/5 + 24/24; `git ls-files --eol` clean;
  `check_registers.py` 100 tracked files clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7,
  **S- 53/53**; `RESULT: ALL PASS`; `GATE: GREEN`; `rc=0`. Re-run the same way on the final tree
  (this entry and STATE's build line in) before the commit (STOP-REASON).
BELIEF DELTAS:
- A Route A comment is the *latest* version's comment, and some authors keep a cumulative
  changelog there: correction language in it may belong to a version outside the window.
  Evidence: candidate 20, whose "minor correction" sentence dates from v9 (2020) while the
  finder saw it on v14 (2023). Consequence: D8 is decided from each version's own abstract page
  (four extra requests here); the lesson is in STATE's NEXT. The finder is unchanged — it is a
  recall net, and the examination step is where D8 is decided.
- "Corrected the hypotheses" is the item-8 shape again: a statement quantified over too large a
  class, a proof that silently needs the smaller one, and the use site is a named theorem's
  application. Evidence: candidate 19 (Dwyer's convergence theorem; the v6 proof derives its
  hypotheses from the new assumption). Consequence: FLAW-0012 + FLAW-0005 as for item 8, and a
  sixth record of the `false`-with-`undetermined`-alternative shape (escalation below).
- A corrected version can print the counterexample inside a remark rather than in the comment,
  and then the `counterexample` method applies even though the comment says only "removed".
  Evidence: candidate 21. Consequence: the second counterexample-method item (after item 9), and
  the second true-statement item (after item 2): the dependent main theorem is re-proved as
  stated, so its truth is `true` by source document rather than `null`.
CONFUSIONS:
- A display filter on the diff header (`awk '/^--- v5/{skip=1} /more diff lines/{skip=0; next}
  !skip'`) also swallowed the helper's trailing scanner line when the diff was short (no "more
  diff lines" marker to end the skip). Cost: the scanner was re-run by hand on the saved sources
  (no hit). Doc fix: this entry; save the helper's whole output to a file and print its head and
  tail instead (done for candidates 20 and 21).
- Python under Git Bash does not resolve `/c/Users/…` paths; one abstract-page fetch failed on
  the output path and was re-run with `C:/Users/…`. Cost: one retry. Doc fix: memory hazard.
- A `sed` substitution carrying an apostrophe inside the pattern silently did not match when
  quoted from Bash; the record sentence was fixed with Python instead. Cost: one retry.
HONESTY CHECK: no CLAIM changed status. New SOURCES S-0050..S-0053, VERIFIED digest-only (arXiv
non-exclusive on every version read). Item 18 records `false` on the authors' "corrected the
hypotheses" with the alternative named; item 19 records `false` on a counterexample the author
prints; item 20 records `true` because the author re-proves the unchanged statement. The
executor's step-level readings for items 18 and 19 are labelled as the executor's and are not
the basis of any class. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the window excludes the release by rule; the
three examined papers are 2012 submissions by named human authors; the scanner reports no
AI-assistance mention on any version read. No detector or threshold exists. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The three manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `fd309a5`; the ids reproduced independently in
Python before the producer ran; the candidate list reproduced from cache with the identical
digest; every retrieval is pinned by URL, access time and SHA-256 (the two re-fetched abstract
pages byte-identical to the helper's). Nothing to quarantine.
MANIFEST CHECK: three artifacts created, each with a `syndeom-manifest/2` manifest carrying
ground truth with derivation `documented-by-source`, flaw records with `raw/1` locators,
`correct: false`, taxonomy 0.1.0, self-checked by the producer before writing (SD1); one carries
`statement_status.value: true` (the proof invalid, the statement re-proved); none is null.
Nothing synthetic labeled real (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3): the records quote
the authors' words, name the alternative reading where the document leaves room, and label the
executor's mechanism readings as the executor's. No understanding claims (SD17). No detectors. No
referee contact (SD13). Candidate 22 was not opened.
NEXT-SINGLE-ACTION: S1.1 — the eighth Route A harvest session, resuming at candidate 22
(`1209.4040`, v4 of 2025-09-10, math.SG, "v4:The notion of “weakly regularizing” (in Def.4.3) was
strengthened to fix a gap in the proof of the main theorem (Thm.4.5)") in ascending id with no
skipping, exactly as STATE.md states it.
ESCALATIONS:
- For Al's attention, not a blocker (carried from Sessions 16 and 17, now six records): items 8,
  10, 11, 14, 15 and 18 carry `false` with `undetermined` named as the alternative where the
  author writes "corrected", "incorrect" or "false in the stated generality" rather than giving
  a counterexample. A standing ruling either way would make the six records uniform; the
  executor's reading is `false` because in each case the author replaces the statement by a
  different one.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers
  (S-0019); not attempted this session.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — a clean boundary (three candidates fully recorded, the resume point
set). Repo green on the committed tree (gate run detached with a log, every edit staged, before
the commit); this commit pushes to `origin/main`.

SESSION: 2026-10-14 UTC @ b05537f (Session 19 — the eighth Route A harvest: candidates 22–24
examined, all three admitted yielding 5 items; the commit adding this entry lands on top of
b05537f)
LAUNCH CONTEXT (SD15; not an exam session): Al's one-word order "resume" in the same conversation
as Sessions 7–18, three minutes after the Session-18 push; the inherited NEXT-SINGLE-ACTION
executed as STATE stated it. Not cold. No memory written (no new machine hazard).
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: prefix through arXiv id
`1303.3650` (24 of 2,472 candidates; 18 admitted yielding 25 items, 4 rejected, 2
found-but-unclassifiable); S1.3: twenty-five `raw/1` parents manifested, no derived artifact
yet. Gate S0 closed and confirmed.
PLAN NOTE: the inherited NEXT (the eighth harvest session, resuming at candidate 22), unchanged
by ORIENT. One thing.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `b05537f`, clean; the constitution unchanged since `226a51c`;
  the log's resume point read. Evidence gate at ORIENT, detached (`gate-19a.log`),
  20:14:35Z–20:15:39Z (warm), `GATE: GREEN`.
- **Candidate list:** regenerated from the 73 cached pages — 0 API requests, 2,472 candidates,
  digest identical to run 1 (`e164c0b1…2d48`).
- **Candidates examined in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md` rows 22–24):
  - **22, arXiv 1209.4040** (Wehrheim; math.SG, a preprint), v4 (2025-09-10) "The notion of
    "weakly regularizing" (in Def.4.3) was strengthened to fix a gap in the proof of the main
    theorem (Thm.4.5)": the v4 source keeps commented-out edit notes at each change, which with
    the diff locate the step — Definition 4.3 (iii) goes from kernel regularity
    (ker D_{E′}Φ(r,0) ⊂ E′_1) to the implication D_{E′}Φ(r,0) e ∈ F_1 ⟹ e ∈ E′_1, and the
    proof's "injectivity on W_0 … follows directly from the estimate for m=1 and the weak
    regularization property (iii)" becomes a spelled-out implication plus a new uniform-bound
    argument; the Floer application's lemma is re-proved for the stronger property.
    **Admitted**: Theorem 4.5 — FLAW-0005 (gap: the m = 1 estimate applied to a w ∈ W_0 whose
    membership in W_1 the old clause does not give), FLAW-0004 named as the alternative;
    **`statement_status` null / `undetermined`** (the fourth use of ruling (ii)): the theorem's
    text is unchanged but its defining hypothesis is stronger. v2's and v3's own corrections
    (2013, 2016) are outside the window. Original v3 (arXiv non-exclusive → `external`);
    S-0054, S-0055; id `s1_ingest-v0.1.0-cafb56cb27fb`.
  - **23, arXiv 1210.6709** (Kwiatkowska; math.LO), v2 (2023-04-20) "Statement of Theorem 1.7
    is corrected", retitled from "Large conjugacy classes …" to "Large orbits …": **v1 has no
    Theorem 1.7** — a new Proposition 1.3 in v2 shifts the main theorem (v1's Theorem 1.6,
    "Aut(ℙ) has a comeager conjugacy class") to 1.7, where it now reads "The action
    Aut(ℙ) ↷ I(ℙ) has a comeager orbit" (I(ℙ) the involutions); v2's Remark 2.3 adds "We do
    not know if Aut(ℙ), has a dense conjugacy class or if H(P) has a dense conjugacy class. All
    existing "proofs" seem to have a flow" [sic], which names v1's Theorems 2.2 and 2.1 by
    content. The diff restricts the class 𝓕 to involutions, adds "symmetric" to the
    antidiagonal lemma (Lemma 2.12) and the characterisation lemma, and lifts through a new
    proposition to an involution. **Admitted as three items**, all **`statement_status` null /
    `undetermined`** (the fifth to seventh uses of ruling (ii)): Theorem 1.6 — FLAW-0005
    (fatal); Theorem 2.2 (Oppenheim's dense-class result, proved in the paper) — FLAW-0005
    (fatal); Theorem 2.1 (H(P), a reduction to 2.2) — FLAW-0005 (gap, the item-16 shape). The
    executor's reading of the mechanism — an antidiagonal's epimorphic image is symmetric, so
    Lemma 2.12 cannot serve the non-symmetric relations v1's class contains — is in the records,
    labelled; the document names no step. Shared original v1 (`external`); S-0056, S-0057; ids
    `s1_ingest-v0.1.0-45c3ee8319d5`, `s1_ingest-v0.1.0-486ef22f632b`,
    `s1_ingest-v0.1.0-dc0e0b0d5b87`.
  - **24, arXiv 1303.3650** (Fu and Wan; math.AG, "to appear in Quarterly Journal of
    Mathematics" per the v2 comment), v3 (2025-01-06) "José Alves Oliveira and Lucas Reis point
    out an error in the old version of Theorem 4.6. The assumption that Tr(g) is not of the form
    r^q-r is corrected to that Tr(g) is not of the form r^p-r": the 40-line diff changes exactly
    that in the statement and in the proof's one sentence that uses it. **Admitted**: Theorem
    4.6 — FLAW-0012 (fatal) + FLAW-0005 (the nonconstancy criterion for the Artin–Schreier sheaf
    applied with the weaker form, fatal), `statement_status` false by source document with
    `undetermined` named as the alternative; the executor's reading (every r^q − r is an
    s^p − s, not conversely) labelled. Original v2 (`external`); S-0058, S-0059; id
    `s1_ingest-v0.1.0-40e6c0da8821`.
  - **Counts by verdict, this session:** examined 3 — admitted 3 candidates yielding 5 items,
    rejected 0, found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0.
    **Cumulative:** examined 24 — admitted 18 candidates (25 items), rejected 4,
    found-but-unclassifiable 2. **Last candidate processed `1303.3650`**; the session stopped at
    its budget before candidate 25 (`1304.5623`).
- Each admitted item: S-entries written **before** the producer ran (SD16); config with the D4
  fields; `syndeom-ingest` at `b05537f` wrote `manifest.json` (`syndeom-manifest/2`,
  `engine_commit` `b05537f…`, self-check passed; the ids reproduced independently in Python
  before the producer ran; the five manifests re-read and their schema, commit, derivation,
  flaw classes and severities, truth values and methods asserted); `RECORD.md` with the D3 class
  argument, the alternatives considered, quoted locating sentences and licence basis. The three
  correcting versions' abstract pages were re-fetched once each (byte-identical) to read the
  journal fields, which are absent on all three.
- STATE: Updated line; Gate S1 rows S1.1 (prefix through `1303.3650`, 24/18(25)/4/2) and S1.3
  (twenty-five parents); registers line (S-0054..S-0059); build line; NEXT resumes at candidate
  25 with the next three candidates' comments quoted and two lessons added (a comment's theorem
  number may follow the corrected version's numbering; a remark naming further results by
  content yields further items).
- Gates: the ORIENT run (`gate-19a.log`) on the clean tree, 20:14:35Z–20:15:39Z, `GATE: GREEN`.
  Evidence run on the staged tree, detached (`gate-19b.log`), 20:35:33Z–20:43:22Z, launched after the
  ORIENT run had ended: `Build completed successfully (8926 jobs)`; audit `[propext]`; fmt and
  clippy clean; `cargo test` 5/5 + 24/24; `git ls-files --eol` clean; `check_registers.py`
  110 tracked files clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, **S- 59/59**;
  `RESULT: ALL PASS`; `GATE: GREEN`; `rc=0`. Re-run the same way on the final tree (this entry
  and STATE's build line in) before the commit (STOP-REASON).
BELIEF DELTAS:
- A revision comment's theorem number is the *corrected* version's number when the correction
  inserts numbered material: v1 of candidate 23 has no Theorem 1.7. Evidence: the two versions'
  environment lists. Consequence: resolve the number in both versions before locating; the
  lesson is in STATE's NEXT.
- The status document can grow by a remark that names further results by content ("We do not
  know if Aut(ℙ) has a dense conjugacy class …") without the comment naming them, and D2.2
  makes the corrected version part of the document. Evidence: candidate 23, three items from a
  five-word comment. Consequence: the multi-item shape now includes items the comment does not
  mention; the records say so.
- Severity for a FLAW-0005 whose "hypothesis" is an invoked result: fatal when the needed
  hypothesis (or the invoked lemma's applicability) is shown false for objects in the proof's
  scope or the result is withdrawn, gap when it is merely unestablished or the invoked result is
  open. Evidence: this session's four FLAW-0005 records (fatal, fatal, gap, gap) against items
  13, 16 and 20. Consequence: a consistent reading of D3's consequence clause and the entry's
  range; recorded in each item's RECORD.md, and flagged below for Al because D3's "weakened →
  fatal" clause read literally would make items 16 and 24 fatal rather than gap.
- Undetermined truth is now common, not rare: seven of twenty-five items (3, 13, 16, 21, 22,
  23, 24). Evidence: this session alone added four. Consequence: the S3 obligation for
  undetermined items (standing rulings) is a sizeable slice of the corpus; the effective-N rule
  (ADR-0004 D6) matters for candidate 23's three dependent items.
CONFUSIONS:
- A `cut` of helper output hid the helper's own `-- tex diff` line count for candidate 22 on the
  first read; the saved `helper.out` and `wc -l` resolved it. No cost beyond one command.
- An unquoted `I(P)` in a Bash `echo` was parsed as a subshell and aborted a multi-command
  probe; re-run with quotes. Cost: one retry. Doc fix: this entry.
- Two placeholders left in a draft record ("Definition 3.x", a doubled hash fragment) were
  caught by the placement script's assertions and fixed before staging. Cost: nil; the
  assertions are the control.
HONESTY CHECK: no CLAIM changed status. New SOURCES S-0054..S-0059, VERIFIED digest-only (arXiv
non-exclusive on every version read). Items 21–24 record `null` because the documents leave
the truth open ("gap", "we do not know"); item 25 records `false` on "an error … is corrected"
with the alternative named. The executor's step-level readings for items 22, 23 and 25 are
labelled as the executor's; for candidate 23 the located step (Lemma 2.12's uses) comes from the
diff, not from the author's words, and the records say so. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the window excludes the release by rule; the
three examined papers are 2012–2013 submissions by named human authors; the scanner reports no
AI-assistance mention on any version read. No detector or threshold exists. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The five manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `b05537f`; the ids reproduced independently in
Python before the producer ran; the candidate list reproduced from cache with the identical
digest; every retrieval is pinned by URL, access time and SHA-256. Nothing to quarantine.
MANIFEST CHECK: five artifacts created, each with a `syndeom-manifest/2` manifest carrying
ground truth with derivation `documented-by-source`, flaw records with `raw/1` locators,
`correct: false`, taxonomy 0.1.0, self-checked by the producer before writing (SD1); four carry
`statement_status.value: null` with `method: "undetermined"` (ADR-0001 as amended). Nothing
synthetic labeled real (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3): the records quote
the authors' words, name the alternative reading where the document leaves room, and label the
executor's mechanism readings as the executor's. No understanding claims (SD17). No detectors. No
referee contact (SD13). Candidate 25 was not opened.
NEXT-SINGLE-ACTION: S1.1 — the ninth Route A harvest session, resuming at candidate 25
(`1304.5623`, v5 of 2023-02-08, math.AG, "due to a mistake in Proposition 3.5, the
unirationality of supersingular K3 surfaces remains a conjecture: see the appendix for erratum")
in ascending id with no skipping, exactly as STATE.md states it.
ESCALATIONS:
- For Al's attention, not a blocker (carried from Sessions 16–18, now seven records): items 8,
  10, 11, 14, 15, 18 and 25 carry `false` with `undetermined` named as the alternative where
  the author writes "corrected", "incorrect", "error" or "false in the stated generality"
  rather than giving a counterexample. A standing ruling either way would make the seven
  records uniform; the executor's reading is `false` because in each case the author replaces
  the statement by a different one.
- For Al's attention, not a blocker (new): ADR-0003 D3's severity clause "the result withdrawn,
  weakened or shown false → fatal", read literally, conflicts with the FLAW-0005 entry's own
  range ("gap: hypothesis true but unshown") for items whose statement is weakened by an added
  hypothesis while the truth is left open (items 13, 16, 21, 24: recorded gap). The executor
  reads the entry's range as governing and D3's clause as the tie-breaker; a one-line
  clarification in D3 would settle it.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers
  (S-0019); not attempted this session.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — a clean boundary (three candidates fully recorded, the resume point
set). Repo green on the committed tree (gate run detached with a log, every edit staged, before
the commit); this commit pushes to `origin/main`.

SESSION: 2026-10-14 UTC @ 1cfcc61 (Session 20 — the ninth Route A harvest: candidates 25–27
examined, all three admitted yielding 6 items; the commit adding this entry lands on top of
1cfcc61)
LAUNCH CONTEXT (SD15; not an exam session): Al's one-word order "resume" in the same conversation
as Sessions 7–19, about forty minutes after the Session-19 push; the inherited NEXT-SINGLE-ACTION
executed as STATE stated it. Not cold. No memory written (no new machine hazard).
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 open. S1.1 in progress: prefix through arXiv id
`1403.4485` (27 of 2,472 candidates; 21 admitted yielding 31 items, 4 rejected, 2
found-but-unclassifiable); S1.3: thirty-one `raw/1` parents manifested, no derived artifact
yet. Gate S0 closed and confirmed.
PLAN NOTE: the inherited NEXT (the ninth harvest session, resuming at candidate 25), unchanged
by ORIENT. One thing.
ADVANCED:
- ORIENT: HEAD = `origin/main` = `1cfcc61`, clean; the constitution unchanged since `226a51c`;
  the log's resume point read. Evidence gate at ORIENT, detached (`gate-20a.log`),
  21:27:41Z–21:38:17Z (cold load again after forty idle minutes), `GATE: GREEN`.
- **Candidate list:** regenerated from the 73 cached pages — 0 API requests, 2,472 candidates,
  digest identical to run 1 (`e164c0b1…2d48`).
- **Candidates examined in ascending id, none skipped** (`groundtruth/HARVEST-LOG.md` rows 25–27):
  - **25, arXiv 1304.5623** (Liedtke; math.AG; Invent. Math. 200 (2015), with a published
    erratum Invent. Math. 223 (2021) listed on the abstract page), v5 (2023-02-08) "due to a
    mistake in Proposition 3.5, the unirationality of supersingular K3 surfaces remains a
    conjecture: see the appendix for erratum": the appendix names, at four depths of one
    dependency chain, Proposition 3.5 (its proof "only shows" a weaker proposition — a degree-p
    multisection from A_ξ[p]-torsors, not necessarily radicial; found by Bragg and Lieblich),
    Assertion (4) of Theorem 3.6 ("whose proof requires that one can find degree p multisections
    in Proposition 3.5 that are radicial"), Theorem 4.3 (a separate mistake: the map "is only
    defined on geometric points", and "the computations in Step 6 of the proof only show that the
    normalisation of the geometric generic fibre is isomorphic to ℙ¹", with an explicit
    σ_0(N) = 3 example whose geometric generic fibre is singular) and Theorem 5.3 (the main
    theorem, "remains a conjecture"). The 732-line diff leaves Sections 3–5 unchanged.
    **Admitted as four items**: Proposition 3.5 — FLAW-0004 (gap; "By construction, D_ξ → ξ×S is
    a family of A_ξ[F]-torsors … radicial"), null / `undetermined`; Theorem 3.6 (4) — FLAW-0005
    (gap), null / `undetermined`; Theorem 4.3 — FLAW-0012 (fatal) + FLAW-0004 (Step 6, fatal) +
    FLAW-0004 (the morphism's definition, gap), **false by the `counterexample` method** (the
    third use; the record states the caveat that the example concerns the constructed map, which
    is what the statement's "give rise to" asserts); Theorem 5.3 — FLAW-0005 (gap; "follows from
    Theorem 5.1"), null / `undetermined`. Theorems 4.5, 5.1 and 5.5 are affected by the chain but
    not named by the erratum and are not items; the records say so. Shared original v4
    (`external`); S-0060, S-0061; ids `s1_ingest-v0.1.0-003a35498654`,
    `s1_ingest-v0.1.0-0833f61c748c`, `s1_ingest-v0.1.0-82b4e66d0016`,
    `s1_ingest-v0.1.0-315546e87259`.
  - **26, arXiv 1312.1045** (Davini and Zavidovique; math.AP; J. Differential Equations 258
    (2015)), v2 (2023-04-05) "We have revised the Appendix by slightly changing the statement
    (and the proof) of Proposition A.2 (Comparison Principle). We needed Proposition A.2 in this
    modified form to fill a gap in the proof of Theorem A.1 …": Theorem A.1's statement is
    unchanged word for word; the comparison principle now needs its growth condition for one pair
    of constants plus either the condition for every α or continuity of the subsolution, and the
    theorem's proof verifies the hypotheses at both applications (explicit bounds on the Perron
    envelopes; continuity for the time-shift pair). **Admitted**: Theorem A.1 — FLAW-0011 (gap)
    + FLAW-0005 (gap), **`statement_status` true** by source document (the third true-statement
    item, after items 2 and 20); Proposition A.2 is not an item (no error reported in it); the
    executor's reading of which application was the gap (the Lipschitz step) is labelled.
    Original v1 (`external`); S-0062, S-0063; id `s1_ingest-v0.1.0-f634c259799f`.
  - **27, arXiv 1403.4485** (Franz; math.AT; IMRN 2015), v4 (2023-06-12) "incorrect degree shift
    in Proposition 5.1 corrected": one summand 𝒦_{m+2}[(m+1)d_V − d_U + 1] becomes
    𝒦_{m+2}[(m+2)d_V − 2d_U − 1], the proof's shifts are recomputed explicitly, the conclusion
    syzord = m is unchanged; the acknowledgement names the finder. **Admitted**: Proposition 5.1
    — FLAW-0001 propagated (statement, fatal) + FLAW-0001 local (proof, repairable),
    `statement_status` false by source document — the item-4 shape, deliberately *not* added to
    the "corrected"-wording escalation list because a wrong shift constant is a false graded
    statement, not a possibly-unproved one. The scanner's "llm" hit is the citation "Stillman".
    v2's and v3's own corrections (2014, 2015) are outside the window. Original v3
    (`external`); S-0064, S-0065; id `s1_ingest-v0.1.0-6ae03b18a1bd`.
  - **Counts by verdict, this session:** examined 3 — admitted 3 candidates yielding 6 items,
    rejected 0, found-but-unclassifiable 0, pending 0, pending-truth 0, escalated 0.
    **Cumulative:** examined 27 — admitted 21 candidates (31 items), rejected 4,
    found-but-unclassifiable 2. **Last candidate processed `1403.4485`**; the session stopped at
    its budget before candidate 28 (`1403.6317`).
- Each admitted item: S-entries written **before** the producer ran (SD16); config with the D4
  fields; `syndeom-ingest` at `1cfcc61` wrote `manifest.json` (`syndeom-manifest/2`,
  `engine_commit` `1cfcc61…`, self-check passed; the ids reproduced independently in Python
  before the producer ran; the six manifests re-read and their schema, commit, derivation, flaw
  classes, sub-variants and severities, truth values and methods asserted); `RECORD.md` with the
  D3 class argument, the alternatives considered, quoted locating sentences and licence basis.
  The three correcting versions' abstract pages were re-fetched once each (byte-identical) to
  read the journal fields — all three papers are published, and candidate 25's erratum is too.
- STATE: Updated line; Gate S1 rows S1.1 (prefix through `1403.4485`, 27/21(31)/4/2) and S1.3
  (thirty-one parents); registers line (S-0060..S-0065); build line; NEXT resumes at candidate
  28 with the next three candidates' comments quoted (one a withdrawal, one in math.GM) and the
  erratum lesson added (record each named result at every depth of the chain; unnamed links are
  not items).
- Gates: the ORIENT run (`gate-20a.log`) on the clean tree, 21:27:41Z–21:38:17Z (cold load),
  `GATE: GREEN`. Evidence run on the staged tree, detached (`gate-20b.log`), 21:48:44Z–21:50:35Z, launched
  after the ORIENT run had ended: `Build completed successfully (8926 jobs)`; audit `[propext]`;
  fmt and clippy clean; `cargo test` 5/5 + 24/24; `git ls-files --eol` clean;
  `check_registers.py` 122 tracked files clean, Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7,
  **S- 65/65**; `RESULT: ALL PASS`; `GATE: GREEN`; `rc=0`. Re-run the same way on the final tree
  (this entry and STATE's build line in) before the commit (STOP-REASON).
BELIEF DELTAS:
- A published erratum appended to the arXiv version is the richest status document the route has
  met: it names results at several depths of one dependency chain, says for each what the proof
  actually shows, and can carry an explicit example. Evidence: candidate 25 (four items; the
  example makes Theorem 4.3 a counterexample-method item while its siblings are undetermined).
  Consequence: the record stops at the named results; the unnamed links (Theorems 4.5, 5.1, 5.5)
  are described, not recorded — a D2.3 reading flagged below.
- "Fill a gap in the proof of Theorem X by modifying Proposition Y" is a true-statement shape:
  the theorem stands, the auxiliary result is reshaped to cover the use, and the auxiliary result
  itself carries no documented error. Evidence: candidate 26 against items 2 and 20. Consequence:
  FLAW-0011 + FLAW-0005 gap with `true`, and the auxiliary proposition is not an item.
- FLAW-0001 does not join the "corrected"-wording escalation: when the correction replaces one
  constant by another in the same statement, the old statement is false if the new is true, and
  the doubt the escalation records (false or merely unproved) does not arise. Evidence:
  candidate 27 against item 4. Consequence: the escalated list stays at seven.
- The undetermined share keeps growing: ten of thirty-one items (3, 13, 16, 21–24, 26, 27, 29).
  Evidence: three more this session. Consequence: the S3 obligation for undetermined items is now
  a third of the corpus; the effective-N rule (ADR-0004 D6) matters for the two four-item and
  three-item candidates (25, 23).
CONFUSIONS:
- A `grep -nE '\\section\*?\{'` found no section headers in two files where a plain `grep -n
  'section{'` did; the backslash escaping differed between the two pattern styles. Cost: two
  re-greps. Doc fix: this entry — use the plain pattern.
- A `sed -n "$((L-4)),…"` with an empty `L` (the preceding `grep` had not matched) became
  `sed -n -4,…` and printed sed's usage. Cost: one retry with a different anchor. Doc fix: this
  entry — test the grep result before arithmetic on it.
- The six RECORD drafts were written before the ids were known and reference the directory name
  only, as all records do; the placement script asserts no placeholder survives. No cost.
HONESTY CHECK: no CLAIM changed status. New SOURCES S-0060..S-0065, VERIFIED digest-only (arXiv
non-exclusive on every version read; the published erratum of candidate 25 is cited from the
abstract page and not retrieved). Items 26, 27 and 29 record `null` on "remains a conjecture" and
"the proof only shows"; item 28 records `false` on the erratum's own example, with the caveat that
the example concerns the constructed map; item 30 records `true` on an unchanged statement
re-proved; item 31 records `false` on "incorrect degree shift". The executor's step-level
readings for items 26 and 30 are labelled as the executor's. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the window excludes the release by rule; the
three examined papers are 2013–2015 submissions by named human authors; the scanner's one hit
("llm" in "Stillman") was checked and is a citation. No detector or threshold exists. Verdict:
no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The six manifests are regenerable from
their configs and bytes by `syndeom-ingest` at `1cfcc61`; the ids reproduced independently in
Python before the producer ran; the candidate list reproduced from cache with the identical
digest; every retrieval is pinned by URL, access time and SHA-256. Nothing to quarantine.
MANIFEST CHECK: six artifacts created, each with a `syndeom-manifest/2` manifest carrying ground
truth with derivation `documented-by-source`, flaw records with `raw/1` locators (two carrying
FLAW-0001 sub-variants `propagated` and `local`), `correct: false`, taxonomy 0.1.0, self-checked
by the producer before writing (SD1); three carry `statement_status.value: null` with `method:
"undetermined"` (ADR-0001 as amended), one `true`, one `false` by `counterexample`, one `false` by
`source-document`. Nothing synthetic labeled real (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3): the records quote
the authors' words, name the alternative reading where the document leaves room, and label the
executor's mechanism readings as the executor's; the later literature on the K3 conjecture is
mentioned only as the introduction mentions it. No understanding claims (SD17). No detectors. No
referee contact (SD13). Candidate 28 was not opened.
NEXT-SINGLE-ACTION: S1.1 — the tenth Route A harvest session, resuming at candidate 28
(`1403.6317`, v3 of 2024-10-15, math.SG, "We removed the wrong theorem from the first version,
which was brought to our attention by R. Leclercq") in ascending id with no skipping, exactly as
STATE.md states it.
ESCALATIONS:
- For Al's attention, not a blocker (carried from Sessions 16–19, unchanged at seven records):
  items 8, 10, 11, 14, 15, 18 and 25 carry `false` with `undetermined` named as the alternative
  where the author writes "corrected", "incorrect", "error" or "false in the stated generality"
  rather than giving a counterexample. Item 31 (a wrong constant) is deliberately not on this
  list. A standing ruling either way would make the seven records uniform.
- For Al's attention, not a blocker (carried from Session 19): ADR-0003 D3's severity clause
  "withdrawn, weakened or shown false → fatal" against the FLAW-0004/0005 entries' own ranges for
  statements left open — this session's items 26, 27 and 29 (gap) follow the entries' ranges, as
  items 13, 16, 21 and 24 did. A one-line clarification in D3 would settle it.
- For Al's attention, not a blocker (new): D2.3 reads "the status document names the result".
  Candidate 25's erratum names four results and withdraws the main theorem, which also voids
  Theorems 4.5, 5.1 and 5.5 that it does not name; the executor recorded only the named four. If
  Al prefers every result the status document voids by its own words to be an item, the three
  unnamed links can be added from the same S-entries without re-examination.
- Note, not a blocker: diff `LICENSE` against the ASF file when `www.apache.org` answers
  (S-0019); not attempted this session.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: context budget — a clean boundary (three candidates fully recorded, the resume point
set). Repo green on the committed tree (gate run detached with a log, every edit staged, before
the commit); this commit pushes to `origin/main`.

---

SESSION: 2026-10-20 UTC @ 8e222a4 (Session 21 — README architecture and operating guide;
base commit shown, documentation branch `codex/institutional-readme`)
PHASE / GATE: S1 — ground-truth corpus; S1 remains open. Al requested the README upgrade instead
of the next harvest. No research gate or standing ruling changes.
ADVANCED:
- Rewrote README.md as an architectural guide: current implementation and dated research
  snapshot, artifact identity and storage contracts, the planned calibration/validation/contact
  boundary, exact toolchain pins, a fixture-only dry run, repository map, and governance.
- Added three native Mermaid diagrams with circular nodes, fine curved relationships,
  restrained red emphasis, and accessible titles/descriptions. Solid relationships depict
  implemented ingestion/identity; dashed paths depict the planned research workflow.
- Checked the account against source code, manifests, the constitution, and ADRs. Independently
  counted 31 archival manifests, 30 external originals and one committed original, ten
  undetermined statement statuses, five represented flaw classes, and 14 taxonomy entries.
  These are inventory checks, not calibration or corpus-coverage results.
- Validation: all 49 relative links and seven navigation anchors resolve; README is UTF-8/LF,
  without BOM, with a trailing newline. Mermaid 11.17.2 parsed and rendered all three diagrams;
  visual inspection and node-bounds checks found no overlapping nodes. The README's exact
  dry-run command succeeded, emitted `s1_ingest-v0.1.0-d2134e7fd2c3` under `/2`, and created no
  corpus item. Rendering dependencies and QA files are outside the repository.
- Full repository gate: `GATE: GREEN`, exit 0 at **2026-10-20 03:10:57Z**. The local cache-plus-gate
  run began at 02:54:20Z; Lean completed 8,926 jobs; the golden theorem's axioms were `[propext]`;
  Rust fmt and clippy passed; all 29 tests passed (24 manifest, five ingest); 122 tracked files
  passed byte checks; Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 65/65, with no undefined
  references. Logs: `gate-verified.stdout.log` and `gate-verified.stderr.log` in the local
  documentation QA scratch directory (not committed). Final recording edits also passed the
  byte/register and diff checks before committing.
BELIEF DELTAS: the README needs an explicit implementation boundary. The manifest library and
ingestion producer exist, but canonical derivation, generation, detectors, calibration, and contact
scoring remain future work. Evidence: workspace members, source code, and STATE's S1 checklist.
CONFUSIONS: a fresh checkout's direct `lake build` began rebuilding uncached dependencies.
Stopped that session-owned process tree, ran the pinned `lake exe cache get`, and resumed the
gate. The README documents the cache-first sequence. No repository build code was changed.
HONESTY CHECK: no CLAIM, SOURCE, FLAW, or ADR entry changed. No scientific result was added; the
research snapshot is explicitly dated to Session 20. “Proven” remains subject to SD2.
LEAKAGE CHECK: only the rules in `contact/README.md` were read; the directory contains no contact
subjects. No contact behavior informed a design decision.
DETERMINISM CHECK: no engine, Lean source, toolchain, manifest, or proof bytes changed. The
documented dry run uses checked-in fixture bytes and the actual engine commit.
MANIFEST CHECK: no proof artifact was created or altered; all 31 ground-truth manifests remain
unchanged. The dry-run fixture was not admitted as real evidence.
DRIFT CHECK: no verdict on a real manuscript, no understanding claims, no detector design or
tuning, and no referee contact.
NEXT-SINGLE-ACTION: S1.1 — the tenth Route A harvest, resuming at candidate 28 (`1403.6317`),
in ascending id with no skipping, following STATE.md's existing procedure.
ESCALATIONS: no new research escalation. The standing items recorded in Session 20 remain.
STOP-REASON: natural boundary — README complete and verified; documentation branch prepared for
review. Historical HANDOFF bytes are preserved exactly; the new session is appended.


---

SESSION: 2026-10-20 UTC @ producer 49e4723 (S1.3 first canonical derivation; the later
artifact/evidence commit adds this entry on top of the committed producer).
LAUNCH CONTEXT (SD15; not an exam): the owner's supplied S1.3 implementation brief, with
prior conversation context; informed working session. No cold-start certification. The owner
explicitly prioritized S1.3 over the inherited harvest NEXT and authorized this public feature
branch, code/docs, commits, push and draft PR, within existing rights. No external contact,
merge or visibility change is authorized or performed by this slice.
PHASE / GATE: S1 open. 31 raw parents; one derived external representation of an existing
parent, not a 32nd independent item. S1.2 positive pairs and S1.4 coverage/power remain open.
The harvest prefix through candidate 27, `1403.4485`, and resume at 28, `1403.6317`, are intact.
ADVANCED:
- ORIENT from public main `8e222a49a64273c9cefc35bca5ae15a0a05fb9ee`; read the constitution
  in full, state/latest handoff, claim statuses, taxonomy and applicable decisions/prefix ruling.
  Existing writable checkout has unrelated local edits; the deliberately read-only review
  checkout is intact. Work used a separate local clone outside OneDrive, on
  `codex/s1-3-canonical-derivation`. No applicable repository/ancestor AGENTS file was found;
  the owner's local-storage instruction was followed.
- Reconciled STATE/groundtruth README assertions of empty evidence against the actual 31
  manifests/log. Orientation gate passed on existing Lean/Rust code, with doc reconciliation
  in progress (29 tests, golden audit `[propext]`). New Rust workspace membership was enabled
  afterward. A no-cache setup attempt was interrupted; `lake exe cache get` succeeded; that
  interrupted attempt is not a passing gate. Logs in `analysis/s1-3/`.
- ADR-0006 specifies the narrow gzip/TeX reviewed-slice procedure. Rust reuses the ratified
  parser, counts, hashes, ID/serialization and both validators. Full decompressed context is
  retained externally; only selected UTF-8 slices are normalized, by ASCII whitespace alone.
  The procedure rejects missing/ambiguous boundaries/context, omissions, unsupported changes,
  bad mappings, changed sources/inheritance and conflicting output bytes.
- 37 tests pass: 24 manifest + 5 ingest unchanged, 8 derivation. The artificial mechanical
  source is test data only. Fixed hashes/ID/manifest serialization were independently
  cross-checked in Python for that simple JSON fixture. No existing dependency pin changed;
  the six added registry packages and licences are in S-0066/dependencies.json.
- Fresh primary retrieval/read of S-0024/S-0025 and S-0052/S-0053, with hashes matching
  history. Selected the Galindo Proposition 3.1 parent because the v3 remark supplies direct
  counterexample testimony, and the original short proof has explicit boundaries. The
  recorded licence requires proof/context bytes to remain external; the existing bibliographic
  and status metadata is retained.
- Committed the tested producer at `49e47237924eddf0c39c24aefad0eb9aa02ee8b3` BEFORE the real
  manifest. Produced child `s1_derive-v0.1.0-c08f9e3ec092`, independently regenerated identical
  outputs, both runs passing actual-byte and parent/child checks. 770 bytes, 75 format tokens,
  one proof step, zero lemma steps. Written fidelity review and source ranges are in its RECORD;
  FLAW-0012 instance 1 anchors the entire statement, step 0 `[0,79)`, without a fabricated
  proof-step attribution. Parent labels and all other flaw metadata remain exact.
- Compared 54 protected baseline files byte-for-byte, and historical contents of both changed
  parent RECORDs remain full prefixes. No harvest advancement, old evidence replacement,
  existing producer/library/golden change, Lean change or dependency upgrade.
BELIEF DELTAS: the short CC BY first item is not automatically a supportable first child. Its
record attributes an executor inference to source-established falsity; primary rereading led
to a promotion hold, preserving the historical manifest and assigning no alternative truth.
The Galindo status document supplies the counterexample testimony needed for the selected
slice. The original gzip contains non-UTF-8 bibliography bytes, so decoding the whole source
as UTF-8 would lose fidelity; complete opaque context plus strict selected-slice decoding
resolves this without repairing or discarding source content.
CONFUSIONS: stale empty-evidence sentences and the manifest-only 25-package count were caught
and reconciled (baseline actually 26, now 33 with the new crate/dependencies). The development
golden-capture run intentionally failed its placeholder before independently checked constants
were installed. A first register check found CRLF in copied metadata; normalized copies passed.
A proof-printing inspection hit Windows cp1252; the UTF-8 rerun completed. No repeated failure
was hidden or reported as green. The explicit reproduction command uses UTF-8 and a new
external destination. The fresh clone's initial Lean build needed the pinned cache preparation.
HONESTY CHECK: CLAIMS/FLAWS and all 31 historical manifests unchanged. The known first parent
is held only for promotion, with the precise discrepancy appended; this is not a silent
reclassification or an unrequested corpus-wide audit. Source testimony and executor inference
are distinguished in both records. No generalized compatibility/performance conclusion.
LEAKAGE CHECK: checked current scope and protected-file comparison: `contact/` remains its
README alone, no contact artifact or real-AI-proof behavior was used in source selection,
procedure design or validation. No detector exists in this workstream; SD8 unchanged.
DETERMINISM CHECK: engine commit, version, recipe, source/parent/context hashes, fixed mechanical
goldens, two independent actual Rust runs and exact stable manifest bytes are recorded in
`analysis/s1-3/`. Python orchestrates only. No new MEASURED/REPRODUCED claim was added to CLAIMS.
MANIFEST CHECK: canonical child has origin ground-truth, parent link, inherited documentary
labels and canonical span, actual-byte check plus check_derived passed; its raw parent remains
the item of record. Artificial mechanical fixture stays outside the corpus. No PROVEN claim.
DRIFT CHECK: preserved original erroneous statement/proof, no repaired argument, no source
verdict beyond the documented status. No referee contact, generator, detector, quarantine,
schema/taxonomy amendment, informal–formal pair or coverage analysis implemented. Previous
closed work stayed outside this assignment; no contents or judgments were imported.
NEXT-SINGLE-ACTION: S1.2 — specify the first positive informal–formal pair procedure: read
constitution §2 S1.2, ADR-0001 D3/D4 and pinned Lake files, discover the next free ADR (0007
here), then write the one-pair procedure/selection requirements for an actually retrieved,
licensed informal statement paired with an existing pinned Mathlib declaration. STATE gives
the exact starting commands. No pair is already admitted; synthetic goldens cannot be relabeled.
ESCALATIONS: the first-parent promotion hold needs a reviewed source-status determination
(primary testimony or separately reviewed counterexample, correctly attributed) and an
explicit trail for any label amendment. It does not block the selected child. Prior standing
S1 rulings/questions remain recorded in STATE; none was silently resolved here. Public draft
publication is explicitly authorized by the owner for this slice and permitted material.
STOP-REASON: completed S1.3 engineering boundary; final staged-tree gate is recorded below.
The feature branch is prepared for authorized draft publication after this evidence commit.
Gate S1 remains open.

CONCURRENT MAIN: a pre-publication fetch found `origin/main` at
`be7d1445edf942821716b2d5c0d7b02c0d9f2257`, merging the separately prepared documentation PR.
Its only changed paths are README, STATE and HANDOFF, not producer/proof inputs. This branch
retains the original baseline and producer commit; no merge or history rewrite was performed.
The draft PR calls out the overlapping state/history records so both sessions can be preserved
at owner-controlled integration. The other local checkout is now clean after that independent
publication; this executor did not alter it or the deliberately read-only review checkout.

FINAL VERIFICATION: 2026-10-20 03:47:27Z, `bash scripts/check.sh`, exit 0 / GATE: GREEN. Lean build
completed 8,926 jobs; existing golden axiom audit `[propext]`; fmt and clippy clean; 37 tests
passed (24 manifest, 5 ingest, 8 derivation); 147 staged/tracked files passed byte checks;
Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 66/66, no undefined references. Log:
`analysis/s1-3/gate-final.txt`. This audit is for the existing Lean golden, not a formalization
of the canonical informal child. The subsequent log/recording-only edits pass byte/register
and diff checks before commit. No producer, recipe, proof, manifest or source-map bytes changed
after the successful real run. The documented default-commit reproduction command also passed
and matched the retained first receipt; its separate receipt is `documented-command.json`.

PR 2 REVIEW FOLLOW-UP — 2026-10-20 UTC, from merge
`64ed84ef16df982468cd386a5b2803e1a022e8ef`.
Al explicitly requested the supplied chronology fix, the complete gate, and merging PR 2.
This authorization supersedes the earlier slice's no-merge instruction; no visibility or
research-scope change is involved.
ADVANCED:
- Applied the supplied `pr2-fix.patch` with `git apply --check` then `git apply`. Its SHA-256
  is `6dbe65da0718eb25ce8fb7845b7f9a043d6209e236440249719e4cd56e0aea17`.
  The result exactly matched both supplied corrected files before further recording edits.
- Restored Session 21 before S1.3. The 235,705 HANDOFF bytes from main `be7d144` are an exact
  prefix, followed by all 9,269 bytes of the original S1.3 appended block from `d2a3306`.
  This follow-up is appended after both; neither historical session was rewritten.
- Restored STATE's two-space continuation indentation and marked Session 20 as a previous
  historical update. Reconciled README's implementation status, canonical-child boundary,
  reproduction link and repository map with S1.3 in this PR, before integration.
- Checked all 144 protected existing files against `d2a3306`, byte-for-byte: production inputs,
  pins, source records, recipes, manifests, proof metadata and retained verification evidence
  are unchanged. The evidence README also retains its original contents as an exact prefix.
  `analysis/s1-3/pr2-preservation.json` records these checks; README's 58 relative links and
  nine anchors resolve, and its three Mermaid diagrams are unchanged.
VERIFICATION: `bash scripts/check.sh`, **GATE: GREEN**, exit 0 at **2026-10-20 04:26:20Z**,
on the staged correction tree `0530f15cfb0e06e1109d4f8878bf04b311a17186` over `64ed84e`.
Lean completed 8,926 jobs; golden axioms `[propext]`; fmt/clippy clean; 37 tests passed
(24 manifest, five ingest, eight derivation); 149 tracked files checked; Q 17, SD 17,
FLAW 14, CLAIM 7, S- 66, with no undefined references. New log:
`analysis/s1-3/gate-pr2-review.txt`; the original `gate-final.txt` and receipts are unchanged.
The final recording commit is gated again before push/merge. Its exact revision, tree, UTC
completion, exit code and transcript hash are recorded in PR 2 and the integration merge
message so that recording that final run does not change the checked file tree.
BELIEF DELTAS / CONFUSIONS: a content-complete merge can still reverse handoff chronology and
leave the wrong NEXT at the tail. Exact-prefix and last-NEXT checks catch this failure.
HONESTY CHECK: this is documentation reconciliation and local verification, not a new research
result or a cold-start exam. The Lean audit remains for the existing golden fixture.
LEAKAGE CHECK: `contact/` is still its README alone and unchanged; no contact material used.
DETERMINISM / MANIFEST CHECK: original producer identity, source/proof/context hashes and
canonical metadata are unchanged; no artifact or historical receipt was regenerated.
DRIFT CHECK: no source reclassification, new pair, harvest, generator, detector or schema change.
NEXT-SINGLE-ACTION: S1.2 — specify the first positive informal–formal pair procedure, using
the detailed starting instructions in STATE.md and the original S1.3 entry above.
ESCALATIONS: the separate first-parent promotion hold and prior standing S1 questions remain.
STOP-REASON: review correction complete; integrate only after the final committed-tree gate.

---

SESSION: 2026-10-20 UTC @ base 8adcb4865abd3f6c3ac882b70563d82caca6207c
(S1.2 positive-pair procedure and first candidate review; final recording revision in the
draft PR). This is an informed working session, not a cold-start exam.
PHASE / GATE: S1 open. The owner requested procedure/selection, a concrete primary-source
candidate, actual feasibility evidence, and an unmerged draft PR. No admission in this slice.
ADVANCED:
- Read the constitution/instructions, STATE/latest HANDOFF, relevant source/claim/taxonomy
  registers, ADR-0001/0002/0003/0006, manifest library, both specialized producers and pins.
  Confirmed default branch main; a fresh `codex/s1-2-positive-pair-procedure` branch started
  at `8adcb48`, with no earlier S1.2 packet. Existing local caches were reused outside OneDrive.
- Wrote ADR-0007 as ordinary procedure under existing authority. It defines primary-source
  eligibility, claim/offered-proof roles, source correspondence, genuine archival/canonical
  pairing, storage, deterministic executable provenance, both-file/parent-child validation,
  exact-type/axiom review, refusals, counting and the next implementation/admission sequence.
  No schema/constitution or S1.1 policy amendment. STATE now correctly names ADR-0008 next.
- Retrieved and visually read Judson's official Annual Edition 2025 PDF: Theorem 2.2.6,
  section 2.2.2, printed p.28 / PDF page 41. It states infinitude of primes; the same-page
  definition resolves the domain/divisor conventions. GFDL-1.2-or-later notice and full
  licence appendix were read. One mathematical candidate examined; no alternatives needed.
  S-0067/S-0068 append fresh primary evidence without changing existing source labels.
- Matched exact pinned `Nat.infinite_setOfPred_prime` and audited its type, dependency bodies
  and transitive axioms with Lean 4.34.1 / Mathlib `d13f23b...`. The target and the SYNDEOM alias
  have the same closed elaborated type, with no extra parameters/typeclass assumptions.
  Both depend on only `propext`, `Classical.choice`, `Quot.sound`; `Nat.prime_def` on `propext`.
  The initial focused audit and later replays exited 0, with identical audit transcript hash.
- `analysis/s1-2/` gives a clause-by-clause mapping. Statement fidelity is supported for the
  selected source claim. The Mathlib proof is the offered argument; Judson's printed proof
  remains source context, with no line-by-line proof-argument fidelity claim. No informal
  proof steps were invented. The existing parser accepts zero proof steps.
- Designed an actual raw pair from the full retrieved PDF and full pinned Mathlib source
  module, with a canonical statement/alias pair linked by `derived_from`. No fake parent,
  erratum, authorship or producer SHA. Raw sources and canonical informal bytes stay external;
  the small original alias can be committed. Rights are explicit; no owner decision blocks
  this external-source design. Public redistribution of a GFDL derivative is a separate choice.
- Rehearsed only existing Rust pure APIs in external development output: informal 118 bytes,
  7 tokens, 0 steps/lemmas; formal 142 bytes, 14 tokens, 4 lines, 1 declaration.
  Proposed config hashes/IDs, raw/target hashes, full configurations and exact commands are
  in the packet. No Manifest was created; full candidate byte/parent-child validation
  remains an obligation of the future producer/admission slice.
- Corrected `engine/derive/README.md`: dry run validates inputs/derivation/manifests but
  skips `write_outputs` content-collision preflight. No producer behavior changed.
VERIFICATION:
- Final staged packet `bash scripts/check.sh`: exit 0 / GATE: GREEN at 2026-10-20 05:17:18Z,
  tree `1034c92f7724e9f86f590c3ae344c1d3291c9c71`; 8,926 Lean jobs, golden `[propext]`,
  fmt/clippy clean, 37 tests, 172 tracked files, Q17/SD17/FLAW14/CLAIM7/S68.
  Index/status identical before and after. New `evidence/gate-staged-packet.json`
  and `gate-staged-packet.txt`; all 146 protected files and both historical prefixes
  pass `check_preservation.py`. The recording commit is gated again before publication.
- Orientation `bash scripts/check.sh` exited 0 / GATE: GREEN at 04:58:19Z: Lean 8,926 jobs,
  existing golden `[propext]`, fmt/clippy clean, 37 tests (24 manifest + 5 ingest + 8 derive),
  150 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 68. It began on main and finished
  while documentation was being drafted, so it is not represented as the final packet gate.
  New receipt/log: `analysis/s1-2/evidence/gate-orientation.json` / `gate-orientation.txt`.
- Successful candidate rehearsal ended 04:57:22.757990Z; all four subprocesses exited 0.
  `evidence/rehearsal-03.json`, `candidate-audit.txt`, `api-probe.txt` retain its result.
  Two corrected harness failures are retained: (01) comparing downloaded LF blobs directly
  to a clean CRLF Windows dependency checkout; (02) passing the wrong transitive-library
  search directory to standalone rustc. Neither was a failed theorem audit. Current helper
  checks Git blobs, records checkout EOL separately and uses Cargo's `deps` directory.
- Final scoped gate and preservation evidence are recorded in `analysis/s1-2/VERIFICATION.md`.
  The final committed revision is gated again before push and the unmerged draft PR records
  that exact revision/tree and transcript digest without changing the checked files.
BELIEF DELTAS: the existing pair contract already supports mathlib status, positive truth/valid
proof, genuine raw/formal parents and zero-step informal statements. A specialized CLI's
restrictions do not imply a schema blocker. A successful empty-statistics fidelity verdict
would be vacuous for semantic alignment; the source table supplies the actual argument.
CONFUSIONS: old editions/search snippets have different theorem numbers; the fixed PDF settles
2.2.6. A pinned clean Windows checkout need not have Git-blob byte endings. The two harness
mistakes above are recorded with failed receipts and concrete fixes, not hidden as successes.
HONESTY CHECK: no existing source statuses, CLAIMS or FLAWS entries changed. Candidate feasibility
is not source-wide certification, corpus admission, PROVEN promotion, or a source-proof audit.
LEAKAGE CHECK: `contact/` remains its original README and is byte-identical; no contact corpus,
real-AI-proof behavior, detector result, private Eigenverse evidence or calibration material
was used for selection/design. Development exposure of this candidate is explicit.
DETERMINISM CHECK: frozen source bytes, full proposed configs, source map, Rust hashes/counts,
toolchain pins, audit command/output and exact helper/input hashes are retained. The dynamic
discovery page is not a generation input. No claimed producer SHA exists before implementation.
MANIFEST CHECK: no corpus artifact touched or admitted; no synthetic fixture relabeled real.
The proposed pair's truth and offered-proof validity are distinct fields. Full manifest and
parent/child checks have not been executed for a nonexistent producer; this is stated openly.
DRIFT CHECK: no source-proof verdict creep, S1.4 work, S2 family, detector, referee contact,
schema change or broadened ingestion. The default branch and all historical evidence stay intact.
NEXT-SINGLE-ACTION: after this draft procedure/candidate review is accepted, implement only
the narrow Rust archive/canonical pair producer of ADR-0007, using the two proposed configs.
Test and commit it first; freshly verify frozen source bytes, generate the genuine raw pair
and canonical child from its actual SHA, run both-file/parent-child/kernel/axiom/refusal/replay
checks and the gate, and review the admission RECORD before counting the first S1.2 pair.
Start with ADR-0007 and `analysis/s1-2/README.md`; STATE gives the same executable next action.
ESCALATIONS: no new owner rights/policy choice for this design. The separate first-parent
promotion hold remains unresolved. Harvest resume is candidate 28, `1403.6317`; S1.4 and
other standing S1 criteria remain open. Counts remain 31 raw parents + one canonical child.
STOP-REASON: procedure/selection boundary reached; publish scoped evidence as a draft PR only,
leave unmerged, and defer implementation/admission to the next authorized slice.


---

SESSION: 2026-10-20 UTC, S1.2 implementation Phase A, baseline
`1a8388b03017e4044cad01ac10e7e89f8fb5e38f` (merged PR 3 at 05:48:01Z).
PHASE / GATE: S1 open; bounded procedure/config correction before producer implementation.
ADVANCED: confirmed pinned raw Git blob declaration/body at lines 27–28, not 28–29.
Corrected ADR-0007, current packet links/maps and both configs. Rust calculated archive
hash `094625fa10ffde723ba3a3ae47ff75db2b1994b8f96f26f6eb562dcbf653ede0`, propagated its parent ID/hash, then
child hash `62fe0c816a6b0ef928f3e4c2fb7e2bd2e215171ebf6b6130202e851e1e9664a3`. Original output/source bytes unchanged.
VERIFICATION: corrected rehearsal ended 2026-10-20T06:00:46.510404+00:00; all four commands exit 0;
actual interval/canonical location passed; old interval, missing proof body and wrong
canonical line controls rejected. Target/alias remain within standard-three axioms.
Fresh evidence is `analysis/s1-2/correction/`; merged-main preservation checks recorded there.
BELIEF DELTAS: a valid content hash and self-consistent config hash do not validate a locator.
CONFUSIONS: the former interval started on the proof line; exact blob enumeration resolves it.
HONESTY CHECK: old receipts still describe old configs; no regeneration or claimed admission.
LEAKAGE CHECK: same development-exposed Judson candidate; no contact/private material used.
DETERMINISM CHECK: Rust identity authority; input/helper hashes and actual source lines retained.
MANIFEST CHECK: no producer/manifest generated in Phase A; 31 raw + one canonical, zero positive pairs.
DRIFT CHECK: no production inputs, pins, source statuses or historical bytes changed.
NEXT-SINGLE-ACTION: commit this correction, then implement the narrow ADR-0007 pair producer
on this branch, test/gate and commit it before real generation; prepare an unmerged draft
implementation/admission PR. STATE's existing implementation next action remains consistent.
ESCALATIONS: none new; first-parent hold unchanged; harvest resumes candidate 28, `1403.6317`.
STOP-REASON: Phase A correction boundary; continue authorized Phase B after its separate commit.


---

SESSION: 2026-10-20 UTC, S1.2 implementation Phase B and proposed admission.
PHASE / GATE: S1 open; new draft implementation/admission PR remains unmerged.
BASELINE: merged PR 3 main `1a8388b03017e4044cad01ac10e7e89f8fb5e38f`, re-fetched unchanged.
CORRECTED PROCEDURE: `d88fad2bb7b339c56bd9964167d3e0acddc8ee4e`, committed before implementation.
PRODUCER: `eb1b96d5b8084ceb1de734fffc90c9b480ec32bf`, exact gated tree
`ad9cbee9a1939ddf3e0deb200a8afbe8db642d53`, committed before real generation.
ADVANCED: narrow `syndeom-pair` archive/canonical implementation; frozen reviewed contract,
actual raw/canonical checks and stronger inherited metadata, compiled input closure,
complete preflight including dry runs, exclusive outputs and fresh failure receipts.
Generated archive `s1_pair_archive-v0.1.0-094625fa10ff` and child `s1_pair-v0.1.0-62fe0c816a6b` are proposed for review.
VERIFICATION: pre-generation gate 06:17:12Z–06:28:55Z, exit 0 / GATE: GREEN,
8,926 Lean jobs, golden axioms `[propext]`, clean fmt/clippy, 51 tests, 203 tracked files,
Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 68. STATE records this same run.
Real production 06:33:04Z–06:34:11Z: fresh five-object retrieval, actual both-file
Manifest checks and check_derived/stronger checks, emitted Lean compile and exact full-type
match, accepted transitive axioms, 11-file byte-identical fresh replay, idempotence/dry-run
preservation, 18 mutation refusals and receipt-overwrite refusal all passed. A separate
synthetic Lean subprocess failure exited 1 as expected and left the actual proof unchanged.
FINAL TREE GATE: after the artifact/evidence commit, retain the exact clean-commit gate
receipt/log externally and record that SHA/tree/result and hosted check in the draft PR;
do not edit the checked tree merely to embed its own final commit hash.
EVIDENCE: `analysis/s1-2/implementation/`; actual source correspondence and admission
review in both new RECORDs; original PR 3 and correction evidence remain byte-identical.
PRESERVATION: new scoped check passed: 162 baseline files, 20 correction files, historical
prefixes and all existing dependency entries unchanged; exactly two expected representations.
FAILURES: retained first unit-test run exposed a Windows extended-drive-prefix bug;
second exposed a fixture's premature path normalization; both fixed before the successful
14-test run, producer gate and commit. No unexpected real-generation/audit failure.
BELIEF DELTAS: a verified executable closure and complete output preflight are required
in addition to manifest consistency; a source locator also needs actual line inspection.
CONFUSIONS: resolved source interval and drive-prefix fixture failures have explicit evidence.
HONESTY CHECK: real generated objects are distinguished from rehearsal and accepted-main
admission; source correspondence, kernel checks and empty-statistics PASS are separate.
LEAKAGE CHECK: same exposed Judson development candidate; no contact/private Eigenverse
material, detector output or claim of held-out isolation.
DETERMINISM CHECK: recorded actual producer closure; all eleven deterministic output files
replayed identically; run timestamps/destinations are excluded from artifact identity.
MANIFEST CHECK: 32 raw + two canonical on this branch propose one extra independent positive
item. Accepted-main baseline remains 31 raw + one canonical with zero admitted S1.2 pairs.
DRIFT CHECK: existing corpus, source statuses, pins, historical prefixes and original
evidence preserved; no schema, ingest/derive contract, S2, detector or harvest work.
NEXT-SINGLE-ACTION: review the correction, producer and both actual admission RECORDs with
generation/audit/replay/refusal/preservation/final-gate evidence; accept or reject this one
proposed pair before counting it on main. STATE names the same review action.
ESCALATIONS: no new implementation blocker; admission review/merge deferred by instruction.
The first-parent promotion hold remains; paused harvest resumes candidate 28, `1403.6317`.
STOP-REASON: implementation and verification complete; publish the new draft for review,
leave unmerged, and do not promote any CLAIM or close Gate S1.


---

SESSION: 2026-10-20 UTC, post-merge reconciliation and S1.4; reporting implementation
`4e46564ac62bc2bd7a5bef54265e9b2dd26772bb`; final report/documentation revision identified
by this task's draft PR and external exact-commit gate receipt.
PHASE / GATE: S1.4 complete for the existing prefix; Gate S1 open. Informed session, not cold.
BASELINE: fetched main `e6a7cae4392dd1565ee57f9fb727cc01e453687d`, tree
`2e4d454048ac916a0047bd908ae3051ba1b981af`; initial worktree clean, no open PRs. Owner
merged PR 4 at 18:10:13Z; main push CI run 39472403912 succeeded. PR head/tree and merge
metadata confirmed; no separate GitHub reviewer approval was recorded.
ADVANCED: separately versioned Rust report using the typed manifest library, lineage
validation, tracked Git inputs and exact hashes; canonical/role-specific frozen octave
bins; review overlays for holds and older escalations; per-item inventory, all-class D9
tables, source-group completeness, interim power and remaining-obligations ledger.
Reconciled current STATE/README/ADR and pair summaries; appended pair/source dispositions.
RESULTS: 34 representations, 32 roots, two canonical children; 46 flaw instances in five
observed classes, nine zero classes; 30 roots missing canonical length; one valid positive
pair, zero positive-length informal proof controls; 22 documentary source-array groups.
Truth 5 true / 17 false / 10 undetermined; offered proofs 1 valid / 31 invalid. Historical
counts retain the first-parent hold. Seven truth and seven severity questions plus the
hold give 15 flagged roots; unflagged subset 17, truth-qualified 5 true / 9 false / 18
excluded. Categories overlap missing canonical lengths and undetermined truth.
VERIFICATION: full gate on initial tool `9375af283e748e915ffc6e31df12f1bb81122e1d` ended 2026-10-20T20:23:14.733002+00:00, exit 0 / GATE: GREEN,
Lean 8,926 jobs; golden axioms `[propext]`; fmt/clippy clean; 67 Rust tests, including
16 report integration tests; 301 tracked files and register references passed.
Initial report and compatibility production 20:11:26Z–20:12:26Z: both fresh report destinations byte-identical across
four artifacts; retained-output refusal exited 2 and preserved bytes. Fresh pair build
in an empty Cargo target and archive/canonical replay exited 0, all 11 deterministic
output files identical. All 20 producer inputs preserved. No historical mutation suite
or emitted-file Lean audit was rerun; prior evidence remains explicitly prior.
FINAL REPORT: review separated known statement-only role exclusions from genuinely
missing proof lengths and made matrix admission/status evidence explicit. Corrected tool
`4e46564ac62bc2bd7a5bef54265e9b2dd26772bb` was committed before final generation; 16 focused tests and clippy passed.
Two new final report runs ended 2026-10-20T20:25:44.186035+00:00; all four artifacts were
byte-identical and existing-output refusal preserved them. Frozen bins and historical
pair inputs did not change. The earlier pair build/replay remains applicable to its
unchanged 20-file producer closure; no redundant historical mutation suite was run.
FINAL TREE GATE: the final report/documentation commit is gated again as a clean commit;
exact SHA/tree/result and external receipt/log are placed in the draft PR without editing
the checked tree to embed its own identity. The implementation gate is not a substitute.
FAILURES: the orientation run passed Lean build/audit but failed fmt because untracked
report drafts changed while it ran; not a clean-base certificate. A development test/
clippy invocation exposed a moved String used in report formatting; borrowing the key
fixed it before the implementation commit. Actual report/replay commands had no execution failure; the first report remains retained
externally as the pre-correction version, not substituted for the final artifacts.
BELIEF DELTAS: the current summary omitted seven truth and seven severity questions that
HANDOFF Sessions 19–20 still carried. An admitted archive count is not an unqualified
analysis denominator, and one source-backed pair is not two samples or a full proof control.
CONFUSIONS: current PR 4 pending-admission prose was stale after the owner merge. Current
summaries now separate the owner action, source evidence, kernel evidence and CLAIM status.
HONESTY CHECK: no historical label changed; first-parent false/source-document attribution
remains recorded and disputed. No independent reviewer, fresh primary retrieval, PROVEN
promotion, numerical power, AUC, measured false-positive rate or N_min is invented.
LEAKAGE CHECK: only existing S1 evidence and public merge/CI metadata used. No contact
data, private Eigenverse material, detector or model evaluation; exposure remains known
development/harvest review or unknown additional exposure, never asserted unseen/blinded.
DETERMINISM CHECK: code/config committed before generation; Rust owns all parsing, counts,
eligibility and deterministic outputs. Python only orchestrates, records exits/hashes and
compares bytes. Corpus/tool revisions and exact closures are retained in provenance.json.
MANIFEST CHECK: all tracked real manifests validated with the existing API; parent/child
and committed bytes checked. Missing external bytes are explicitly not newly verified
by the inventory. Analysis files are reporting artifacts, not synthetic corpus examples.
DRIFT CHECK: no frozen producer file, source label, historical receipt, manifest, proof,
recipe, dependency pin or taxonomy changed. HANDOFF/SOURCES and both pair RECORDs retain
their exact historical prefixes. No schema/admission/statistical-policy amendment.
NEXT-SINGLE-ACTION: resolve the status-support hold on `s1_ingest-v0.1.0-63e23a952084`
from its appended RECORD, S-0024/S-0025 and retained S1.3 retrieval evidence; record actual
primary support or a separately checked counterexample and an explicit ADR-0003 D3
disposition. Preserve labels unless an attributed amendment is authorized and supported.
ESCALATIONS: the explicit hold and earlier truth/severity/unnamed-result questions remain
visible in STATE and `analysis/s1-4/OBLIGATIONS.md`; no new implementation permission needed.
Cold-start closure requires a later fresh session with §6 verbatim and alone. Harvest
resume remains candidate 28, `1403.6317`; the fixed prefix ends at `1403.4485`.
STOP-REASON: authorized report and verification complete; publish one draft PR, keep unmerged,
leave Gate S1 open and carry the concrete source-status review forward.


---

SESSION: 2026-10-20 UTC, first-parent status-support resolution; Codex (GPT-6), one
executor. The owner supplied the candidate argument and requested a checked disposition,
complete candidate amendment, verification and one unmerged draft PR.
PHASE / GATE: S1; Gate S1 remains open. This is an informed session, not a cold-start test.
BASELINE: fetched default main `1a76d9ac21cd5d2e4311f9164178327c0ca128bb`, tree
`3b9028c0502add53ac646108132d89ba5f1e5666`; initial worktree clean, no open PRs.
PR 5 was merged by Quigles1337 at 20:41:46Z; PR head
`618271daf7b8b091382f800826dbe03156f4d803` has the same tree; no GitHub reviews recorded.
The new branch is `codex/s1-first-parent-status-resolution`. Current summaries reconcile
the merge; historical S1.4 prose/receipts/overlay and all 35 packet files stay frozen.

ADVANCED: exact v70/v71 archives freshly retrieved, matching retained hashes and lengths;
definitions, Lemma 1.1 and the displayed proof step read at exact TeX locators. New HTML
hashes are separate retrieval events. NIST DLMF primary identities retrieved/read and
registered as S-0069 through S-0072, digest-only. Existing licence/storage choices preserved.
New packet: `analysis/s1-first-parent-status/`.

MATHEMATICAL CHECK: at a=1,b=2 the unique character is 1 on odds, 0 on evens and A=-1.
At k=2 the displayed unrestricted expansion gives 1 versus 0. Separately, at s=2 the
lemma residual is log(2)/3>0. The written check justifies absolute/local uniform
convergence, nonvanishing, differentiation/regrouping, Mangoldt support and starting
indices. A second derivation uses the missing Euler factor and zeta logarithmic derivative.
v71's correction term is only a correspondence check. Both checks are by this executor,
not independent humans or blind reviewers. No other result or paper-wide verdict.

KERNEL SCOPE: the exact `Witness.lean` compiled at 21:49:47Z; eight theorems each audited
with `[propext, Classical.choice, Quot.sound]`. Actual character uniqueness, phases,
coefficient and finite inequality, infinite geometric HasSum and positivity are covered.
The source all-character-sum reduction is a written correspondence; the full M/L analytic
bridge is written, not formalized. No complete kernel-refutation or PROVEN claim.

POLICY / DISPOSITION: mathematical noncancellation gap addressed at the disclosed written
level; author attribution clarified. The notice identifies an error but does not give
this witness or explicitly declare falsity. Old false/source-document/S-0025 metadata
remains unchanged. ADR-0001 permits a counterexample reference; ADR-0003 D3's source-notice
rule, SD12's derivation attribution and ratified admission boundary need a case-specific
owner ruling. The exact proposed text and acceptance requirements are prepared, not assumed.
Metadata amendment is proposed; canonical eligibility remains held. An evidence sidecar
alone does not repair the embedded old-parent metadata.

CANDIDATE: existing Rust ingest at actual source revision
`1a76d9ac21cd5d2e4311f9164178327c0ca128bb` generated
`s1_ingest-v0.1.0-15ab865466d6` twice into fresh external output trees. Both outputs are
byte-identical. The complete candidate is retained outside live `groundtruth/`, under
the packet's `candidate/`. Both embedded statuses are false/counterexample with the exact
witness hash; all other config fields, source bytes and labels agree with the old item.
Actual source closure, locked build, binary hash, command/exit logs and Rust validation
are retained. Four negative controls reject wrong ID/bytes, double counting and premature
effective replacement. `amendment-map.json` is a minimal one-case relation, no `derived_from`.
The live corpus stays 34 representations / 32 logical items. A later admitted replacement
would remain one effective logical item; no 33rd independent example is manufactured.

CHECKS: clean-base full gate ended 21:44:58Z, exit 0 / GATE: GREEN: Lean build, golden
axioms `[propext]`, fmt/clippy, 67 Rust tests, 328 files and register checks. Two actual
orientation logs retained; an unnecessary retry started after sandbox process visibility
hid the original still-running process. Candidate checks and explicit scoped Lean audit
passed. Preservation checks cover 323 unchanged baseline files, all 35 S1.4 files, three
exact historical prefixes and the pair producer's 20 inputs against its pinned revision.
The final clean committed tree is gated, explicitly Lean-audited, candidate-replayed and
preservation-checked separately; exact commit/tree/results and external first-write
receipts are placed in the draft PR without a self-referential recording commit.

FAILURES: first Lean prototype failed implicit character/geometric simplification; the
second left a weighted-series normalization goal. Both code/logs and failed `sorryAx`
output are retained as failed attempts. Explicit identities/ring equality replaced broad
simplification; third prototype and exact packet passed. No accepted witness has sorry/admit.
Initial path-discovery misses and denied CIM process visibility are disclosed in VERIFICATION.
BELIEF DELTAS: a correct explicit residual repairs a mathematical gap but does not make
the author's short notice its source. Status method/reference are identity-bearing config,
so a sidecar cannot silently repair a canonical parent's inherited metadata.
CONFUSIONS: resolved process-visibility and Lean normalization issues; remaining ambiguity
is policy, with a concrete proposed decision and validated object ready for owner review.
HONESTY CHECK: source testimony, executor reasoning, kernel scope, structural validity and
owner acceptance are separate. Two checks do not imply independent human review. No new
claim status, new canonical artifact, blanket relabel or unrecorded policy change.
LEAKAGE CHECK: source case/witnesses are development-exposed; no held-out fidelity claim,
contact data, private Eigenverse material, human recruitment or detector work.
DETERMINISM CHECK: actual existing producer source/lockfile bytes verified; Rust owns
config hash/ID and typed validation. Python orchestrates and compares bytes only. Both
fresh two-file outputs replay identically; raw logs and earlier receipts are not overwritten.
MANIFEST CHECK: old and proposed objects validate against actual archive bytes. New config
hash is `15ab865466d67dfd18ed487b4e51a3dc1719b07a00b89b3627e92dc111c95364`; source remains
`5987d8106ae69669b10c876a0915d5ad52ce44a814ea7fe63600c1737642cca8`. Proposed relation contributes
one effective item; future consumers must honor that relation before integration/use.
DRIFT CHECK: frozen S1.4 report, inventory, provenance, config, bins, overlay and receipts
unchanged; old manifests/configurations/proofs/source archives unchanged. HANDOFF/SOURCES
and held RECORD preserve complete byte prefixes. No engine, schema, taxonomy or pin changes.
The seven truth questions, seven severity questions and unnamed-dependent-results question
retain their historical nonblocking classification and exclusions. No fourteen new adjudications.
NEXT-SINGLE-ACTION: owner reviews and accepts or rejects the exact case-specific ruling in
`analysis/s1-first-parent-status/PROPOSED-RULING.md` and the generated candidate. If accepted,
a separate verified integration records the ruling and consistent effective metadata/counting
before releasing the hold. STATE names the same action.
ESCALATIONS: the exact D3/SD12/admission-boundary and amendment decision is now concrete;
no further implementation permission is needed for this draft. Remaining Gate S1 work is
valid disposition/integration of the hold and the later genuine cold-start examination,
with a green repository. Canonical lengths are scoring prerequisites, not a blanket gate
requirement. Cold-start examination uses §6 verbatim and alone in a separate fresh conversation.
Harvest prefix stays through `1403.4485`; candidate 28 / `1403.6317` remains saved next.
STOP-REASON: evidence and candidate preparation complete; publish one draft PR for review,
keep unmerged and Gate S1 open. No harvest advancement, canonical production or S2/S3 start.


---

OWNER DECISION EVENT: 2026-10-20 America/New_York — accepted first-parent amendment

Al / Quigles1337 explicitly accepted in full the “Exact proposed owner ruling” in
`analysis/s1-first-parent-status/PROPOSED-RULING.md` at
`37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f`. Actual instruction and scope:
`analysis/s1-first-parent-integration/OWNER-DECISION.md`; normative record: appended
ADR-0003 amendment of this date. The decision is final for this case and requires no
second acceptance. It covers only Lemma 1.1 of arXiv:0706.0357v70 and candidate
`s1_ingest-v0.1.0-15ab865466d6` for old logical key `s1_ingest-v0.1.0-63e23a952084`.

The separately attributed written counterexample supplies this case's statement-truth
support under the narrow D3/SD12 exception. S-0025 remains author error/locator evidence.
The full analytic source-to-Lean bridge is not formalized and no PROVEN claim is made.
Integration must still verify bytes, witness binding, configuration, identity, effective
mapping, one-item counting and historical preservation before releasing status support.
The old object, retained candidate, original producing revisions, proposed map and frozen
S1.4 packet remain evidence. The acceptance does not itself assert that integration has
passed or reached main. A subsequent verification entry records the actual result.
Canonical selection, context and locator review remain separate; Gate S1 stays open.


---

SESSION: 2026-10-20 America/New_York / 2026-10-21 UTC — accepted first-parent integration
@ implementation `c87c827a27ce40af7e08101d452ba5532704c9c5`; final recording revision
and exact-tree gate are identified in the unmerged draft PR and external receipts.
PHASE / GATE: S1 / OPEN. This is an informed continuation, not a cold-start examination.

ADVANCED: recorded Al's actual acceptance of the exact ruling at main/accepted packet
`37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f`. PRs 4/5/6 were already merged.
The owner decision event above and appended ADR-0003 record the narrow D3/SD12
exception for Lemma 1.1 of arXiv:0706.0357v70. No second approval was requested.
The accepted candidate was copied exactly into live groundtruth, preserving its original
producing revision `1a76d9ac21cd5d2e4311f9164178327c0ca128bb` and historical notes.
The old object and its producing revision remain immutable. The one-case raw amendment
map keeps `s1_ingest-v0.1.0-63e23a952084` as the stable logical key and selects
`s1_ingest-v0.1.0-15ab865466d6` for exactly one effective status, never `derived_from`.

EVIDENCE: `analysis/s1-first-parent-integration/` retains decision, baseline preservation
pins, Rust inventory/provenance, actual logs/receipts and reproduction instructions.
The consumer checks exact source/candidate bytes, witness/config/hash/ID and both
status copies, mapping, historical prefixes, the complete old corpus and packets,
and all 20 original pair inputs. Two actual reports match byte-for-byte and existing
output is refused unchanged. All four fresh historical S1.4 replay outputs match
the frozen packet; no historical receipt or first-write bin was replaced.

ACTUAL COUNTS: 35 archived representations (33 raw, two canonical), 32 effective logical
items, 46 flaws across five classes, nine zero classes, 30 without canonical lengths.
Statement truth 5/17/10 true/false/undetermined; proof status 1 valid/31 invalid.
Unflagged subset 18; truth-qualified 5 true/10 false. The seven older truth and seven
severity questions plus unnamed-result question retain their dispositions/exclusions.

CHECKS: clean accepted baseline full gate passed at 2026-10-21T02:57:48.435698Z
(67 tests, 375 files). Fifteen focused integration tests pass. Clean implementation
full gate ended `2026-10-21T03:18:43.794933+00:00`, exit 0 / GATE: GREEN:
Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, all 82 Rust tests, 403 files
and coherent registers. Rust verified 370 exact historical files, three exact-prefix
appends, all 35 frozen S1.4 files, all 47 accepted packet files and the original pair
closure. The final recording commit is separately gated and reproduced externally;
its exact head/tree/log/hash/receipt is published in the draft PR without editing it.

DISPOSITION: accepted ruling + verified integration conditions discharge this case's
status-support hold on the branch. Main remains at the accepted baseline until the
integration is reviewed and merged; only that later merge makes the change effective
there. Canonical production still requires selection, context and locator review.
The full manuscript M/L analytic bridge is not formalized. Written executor reasoning
and scoped Lean support remain separate from S-0025 author testimony. No PROVEN claim.

BELIEF DELTAS: accepting the scoped evidence category permits consistent metadata
integration; keeping the old logical key and one effective representative prevents an
amendment from manufacturing another example or retaining the superseded attribution.
CONFUSIONS: the first checker compared ingest input notes as hashed config; the existing
producer keeps notes outside config. Corrected the check, not the candidate. The first
focused run (12 pass/3 fail) and first clippy run (collapsible if) remain retained; their
corrected runs pass. Initial path/sandbox and staging/edit-guard misses changed no
historical files. LF-readable transcripts have explicit hash maps to lossless raw gzip.
HONESTY CHECK: owner acceptance, branch integration, final-tree gate and main activation
are distinct. The witness investigation was not repeated; there is no new independent
human or blind review and no new source verification or mathematical status transition.
LEAKAGE CHECK: examined only existing governance, archived development inputs, tooling
and integration evidence; no contact corpus or held-out AI-proof behavior informed this
change. No external researcher or referee contact occurred.
DETERMINISM CHECK: Rust owns metadata/counting under SD10. The complete baseline Git
tree and compiled source are bound; outputs replay exactly. Python only orchestrates
commands and compares/retains bytes. New inventory is separately dated and revision-bound.
MANIFEST CHECK: both raw objects retain their original IDs/configs/producing revisions;
the accepted candidate and its committed source match the original packet. Available
committed bytes are checked; absent external originals are disclosed, not re-verified.
DRIFT CHECK: no schema change, generator, adapter, detector, pair regeneration, new
canonical artifact or harvest. Old labels, historical mathematics/contexts and all
dependency pins are intact; no general amendment platform or executor label authority.
NEXT-SINGLE-ACTION: review the draft integration PR and merge only after review; this
execution leaves it unmerged. After reviewed merge, record actual main activation and
assess readiness for the separate genuine cold-start examination using §6 verbatim
and alone. No readiness pass or cold-start result is asserted now.
ESCALATIONS: no renewed ruling request. Older truth/severity/unnamed-result dispositions
remain nonblocking; canonical review and later cold-start work remain distinct.
STOP-REASON: verified integration and evidence ready for draft review; keep Gate S1 open.


---

SESSION: 2026-10-21 UTC (began 2026-10-20 America/New_York) — post-merge activation/readiness
@ merged-main orientation `1c56028532ea135603dbad5423d5555313bfd9ec`;
the final reconciliation revision/tree and exact-final-commit results are retained
externally and in the PR associated with this commit, without a self-referential commit.
PHASE / GATE: S1 / OPEN. Informed working session; no cold-start examination.

ADVANCED: re-fetched origin/main and verified PR 7 actually merged at
2026-10-21T03:33:35Z / 2026-10-20 23:33:35 America/New_York. Main is
`1c56028532ea135603dbad5423d5555313bfd9ec`, parents `37f1d59` and `a337375`;
its tree `14b451866491dc4a2b866ec0b1d726d792385916` exactly equals the tested
PR head `a337375dbae49150a1a40fa46f6d1417835aadd2`. Both head and main-push
workflows concluded success. No intervening main changes or unrelated local work
were present. Created `codex/s1-post-merge-readiness` from that clean merged main.

The owner's acceptance, verified branch integration and actual default-branch
activation are now distinguished in the separate event
`analysis/s1-post-merge-readiness/README.md`. The accepted decision, original
candidate and proposal, old integration evidence and frozen S1.4 ledger retain
their historical wording. The unchanged Rust consumer validates merged main:
logical key `s1_ingest-v0.1.0-63e23a952084` selects only
`s1_ingest-v0.1.0-15ab865466d6`, so this case's status-support hold is discharged
on main. The S-0025 author error/locator and separately attributed written witness
remain distinct. No repeated owner acceptance or new mathematical investigation.

Reconciled only current README/STATE sections and appended this entry plus a dated
groundtruth README clarification. All prior HANDOFF and groundtruth README bytes
remain exact prefixes. The new readiness assessment maps the constitution's S1
criteria to actual evidence: the admitted prefix is sufficient under ADR-0003 D8;
one positive statement-fidelity pair is admitted; manifests and qualified coverage
are recorded; no additional substantive S1 exit criterion is identified as unmet.
Gate S1 remains OPEN for the separate genuine cold-start examination. A zero-step
statement pair is not an informal proof control. Thirty missing canonical lengths,
external-byte verification and canonical selection/context/locator review remain
future scoring prerequisites, not invented requirements to canonicalize all items.

ACTUAL VERIFICATION: the existing reproduction completed at
2026-10-21T03:50:28.787156+00:00 on clean merged main, two Rust exits 0 and expected
overwrite refusal exit 1 with outputs preserved. Inventory SHA-256
`b626d1b53f50a23ee5eae9e99fc6ea387bb83f51c807d843d68a29dd1651b803` equals
the retained inventory; fresh provenance names the actual merge/tree and differs
appropriately from the original producing checkout. Counts remain 35 representations
(33 raw, two canonical), 32 effective logical items, 18 unflagged, statement truth
5/17/10, qualified truth 5/10, offered-proof status 1 valid/31 invalid, 46 flaws,
five observed/nine zero classes and 30 logical items without canonical representation.
Clean merged-main full gate ended `2026-10-21T04:00:41.304180+00:00`, exit 0 /
GATE: GREEN: Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests,
428 tracked files and register checks. Receipt/log are in the new event; main's
HEAD/tree and clean state were unchanged before/after.
The exact final committed reconciliation tree is checked separately with the full
gate and existing Rust reproduction; its actual receipt, logs, byte comparison and
head/tree stay outside the tree they certify and in the associated PR. No earlier
implementation or merged-main gate substitutes for that exact-final-tree result.

PRESERVATION: Rust rechecks 370 exact accepted-baseline files, three historical
prefixes, all 35 S1.4 files, all 47 witness/proposal packet files and all 20 frozen
pair producer inputs. The later reconciliation additionally compares every file
against merged main: only README/STATE reconciliation and two exact-prefix appends
are permitted; the new event is separate. No earlier evidence, failed-run record,
first-write bin, object, producer revision, accepted manifest/source, decision/map,
source record/licence, taxonomy, ADR or dependency pin is changed.

BELIEF DELTAS: GitHub's actual merge and identical tree activate the already accepted
and verified one-case change on main; a fresh Rust run confirms its effective count
and status. The first-parent hold is no longer a gate obstacle. The remaining S1
action is a genuine cold examination, with the existing qualifications visible.
CONFUSIONS: current summaries had retained branch-only pending-merge language after
PR 7 landed. Reconciled those statements and placed chronology in a later event,
leaving historical records intact. Different checkout provenance is expected even
when inventory bytes are identical. The gate's quiet Lean axiom stage is not treated
as a failure or a reason to start duplicate runs.
HONESTY CHECK: re-read CLAIM statuses and FLAWS version; no transitions or relabeling.
No complete analytic source-to-Lean bridge, PROVEN claim, numerical power, detector
reliability, positive-length informal proof control or blind evaluation is established.
LEAKAGE CHECK: inspected existing governance, corpus metadata, prior evidence and
merge/check metadata only; no contact-corpus or real-AI-proof behavior informed a
design decision. No contact data, researcher outreach or referee participation.
DETERMINISM CHECK: existing Rust retains all metadata/count authority under SD10;
Python only orchestrates and compares bytes. Receipts bind actual commands, exits,
checkout/tree and raw-log hashes; readable copies have separate hashes and lossless
raw gzip companions. No output destination or historical receipt was reused.
MANIFEST CHECK: no new proof artifact; both raw objects and their original producing
revisions, candidate manifest/source and effective map remain exact. Available
committed bytes are verified; absent external originals retain historical evidence
only and must be retrieved/reverified before scoring.
DRIFT CHECK: no producer/detector changes, schema, generator family, canonical artifact,
candidate regeneration, harvest or witness reinvestigation. The seven truth questions,
seven severity questions and unnamed-result question keep their existing nonblocking
dispositions and exclusions. S1.4 remains the frozen original-snapshot report.
NEXT-SINGLE-ACTION: once this reconciliation is on main, a separate genuinely fresh
session attempts Gate S1's cold-start examination under §2/SD15 using §6's Session-N
resume prompt verbatim and alone. Do not carry program state in its launching prompt
or use a full-history continuation as the exam. This session does not close S1.
ESCALATIONS: no renewed owner ruling is needed. Existing older questions and future
scoring prerequisites remain as documented; no additional substantive S1 blocker
was identified. Temporary draft/review status belongs to PR metadata, not STATE's next action.
STOP-REASON: bounded activation/readiness reconciliation complete for review; leave
the reconciliation PR unmerged and Gate S1 OPEN pending the separate fresh examination.

SESSION: 2026-10-21 UTC @ 26c2555 (Session 22 — a §6-launched session that ran the paused harvest
NEXT from an unfetched working copy; candidates 28–30 harvested and held on branch
`claude/s1-harvest-candidates-28-30`; main reconciled; the commit adding this entry lands on top
of 26c2555)
LAUNCH CONTEXT (SD15; not the examination — see below): a fresh conversation launched with the §6
Session-N resume prompt verbatim and alone, the launch the merged NEXT-SINGLE-ACTION asked for.
Recalled memory: the index line locating this program (repo path, remote, the rule that the
constitution is the sole authority and that no program state lives outside the repo), the KOBER and
FR13DMAN sibling pointer lines, and this program's memory file (repo location and machine hazards
only; no program state). **This session is not the Gate S1 cold-start examination and cannot be
certified as one:** (i) it did not reproduce its recalled memory as its first HANDOFF entry before
any other action; (ii) it wrote memory (three machine hazards — the finder's page-cache reuse, a
grep/heredoc backslash trap, and fetch-before-ORIENT — no program state); (iii) above all, its §4.1
ORIENT ran on a working copy whose `origin/main` ref had not been fetched since Session 20, so
`git status` reported `main` in sync with `origin/main` while the remote was 24 commits ahead
(PRs #1–#8, 2026-10-19..21, a Codex executor, merged by Al). It reconstructed the Session-20
state, executed the Session-20 NEXT (the tenth harvest), and discovered the divergence only when
`git push` was refused after a full harvest. The examination remains the NEXT-SINGLE-ACTION, for
another fresh session launched the same way that fetches first.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 OPEN, awaiting the cold-start examination
(unchanged). The admitted gate prefix stays through candidate 27 / `1403.4485` (ADR-0003 D8;
frozen S1.4; 35 archived representations / 32 effective logical items); the later harvest of
candidates 28–30 is held off main (below).
PLAN NOTE: the inherited NEXT as read from the unfetched copy was the tenth harvest; ORIENT against
the fetched remote would have found the examination instead. The session's second half reconciled;
this entry records both halves.
ADVANCED:
- ORIENT as run, on the stale copy: HEAD `8e222a4` read as `origin/main`, clean; STATE, HANDOFF
  Session 20, CLAIMS, FLAWS 0.1.0 and ADR-0003 read; gate detached (`gate-orient.log`)
  21:44:16Z–21:54:48Z, cold OS cache, `GATE: GREEN` at `8e222a4`.
- **Harvest on the stale base — held on branch `claude/s1-harvest-candidates-28-30`, commit
  `c3a3d36`, pushed; nothing of it is on main.** The candidate list was regenerated by the finder
  from the 73 pages cached by Sessions 11–20, copied into this session's scratchpad (0 requests,
  2,472 candidates, digest identical to run 1). Candidates 28–30 were examined in ascending id,
  none skipped, and all three admitted, five items:
  - 28, arXiv 1403.6317 (Đuretić, Katić, Milinković; math.SG): v2 of 2014-03-28 withdrawn "due
    to an error in the assumptions of the Theorem 1", v3 of 2024-10-15 "We removed the wrong
    theorem from the first version, which was brought to our attention by R. Leclercq"; the
    645-line diff removes Theorem 1 (`thm:HD`, the Hofer-distance equality d(φ¹_H, Id) =
    Δ(L, φ¹_H(L)) for admissible, quasi-autonomous H), its proof section, the Hofer-distance
    subsection and a corollary, and renormalises the invariants through PSS. Theorem 1 —
    FLAW-0012 (statement, fatal), false by source document, `undetermined` named as the
    alternative (the eighth record of the "corrected"-wording shape); the failing step not named,
    not guessed. Branch ids/S-numbers: `s1_ingest-v0.1.0-eeea00c52724`; S-0066, S-0067.
  - 29, arXiv 1403.8075 (Nishii; math.GM): v9 of 2023-03-23 the last version with an e-print; v10
    (2025-12-27) and v11 (2026-01-09) withdrawals — "In Proposition 4.1 the argument incorrectly
    treats primality verification as equivalent to checking leading digits in multiple base
    representations, and the subsequent claim that fp(1),...,fp(n) are independent is unjustified.
    These invalidate the probabilistic step leading to Theorems 4.3 -- 4.4". The helper stops at
    the withdrawn versions' missing e-prints; v9 was fetched and scanned by hand (no AI hit).
    Proposition 4.1 — FLAW-0004 (the equivalence asserted without justification, dial `false`,
    fatal); Theorem 4.3 — FLAW-0005 (gap: Theorem 3.4 applied with its independence hypothesis
    unjustified); Theorem 4.4 — FLAW-0005 (gap: a reduction to 4.3); all three null /
    `undetermined`; `status.kind` `retraction`. Branch ids: `s1_ingest-v0.1.0-b77cf8e312d0`,
    `-c38c8c65dd2c`, `-e8075d1f1a0d`; S-0068, S-0069.
  - 30, arXiv 1404.4599 (Otto; math.CO; extends a LICS 2013 paper): v4 of 2015-10-15 the
    original, v5 of 2024-01-12 a withdrawal — "The construction of finite n-acyclic groupoids in
    Section 2.4 is flawed and I know of no direct repair: completion turns out to be incompatible
    with restriction to proper subsets of the generator set, so that the induction towards
    Proposition 2.22 does not stabilise as claimed. This problem has been overcome in
    arXiv:1806.08664." Proposition 2.22 confirmed as Section 2's twenty-second environment by
    count; its proof consumes Lemma 2.21's preservation clause, which the lemma's proof derives
    from Lemma 2.9 (completion commuting with reducts) — the compatibility the notice says fails.
    The later paper (same author) was read in part for its account of the flaw ("the proposal of
    a naive completion operation accounts for the major flaw"; found by Julian Bitterlich in
    2019) and registered as a supplementary S-entry. Proposition 2.22 — FLAW-0005 (fatal),
    FLAW-0004 named as the alternative; null / `undetermined` with the `true` reading by the later
    paper named as the alternative and escalated; `status.kind` `retraction`. Branch id
    `s1_ingest-v0.1.0-22b1b48f3565`; S-0070, S-0071, S-0072.
  Each item: S-entries before the producer (SD16); `syndeom-ingest` at `8e222a4` wrote `/2`
  manifests (ids reproduced independently in Python first; self-checks passed; content hashes equal
  the e-print digests); `RECORD.md` per item; HARVEST-LOG rows 28–30 and counts; the branch's STATE
  rows, registers, build line, escalations and NEXT (candidate 31). Gates on the branch tree,
  detached with logs: 22:17:08Z–22:23:24Z (staged) and 22:25:58Z–22:26:33Z (final), `GATE: GREEN`
  (29 Rust tests, 132 tracked files, S- 72/72 in the branch's own numbering).
- **Why the branch cannot be merged as it stands** (the integration conditions, whenever Al orders
  it): (1) its S-0066..S-0072 collide with main's S-0066..S-0072 (flate2, Judson, Mathlib, DLMF ×4)
  and become the seven numbers after S-0072 (0073–0079); (2) `config.sources` then changes, so every config hash and
  `artifact_id` changes — the five manifests are regenerated by the producer at the integration
  commit from the pinned bytes (SHA-256 in the branch's S-entries and manifests; the e-prints are
  in this session's scratchpad only); (3) the RECORDs, HARVEST-LOG rows and STATE text are
  re-pointed at the new ids and S-numbers; (4) main's Rust consumers
  (`engine/ingest/examples/support/effective.rs`, `engine/manifest/examples/support/coverage.rs`)
  read every `groundtruth/*/manifest.json` and fail closed on an unexpected population ("one
  amendment must yield 35 representations"), so a later prefix needs its own inventory snapshot and
  coverage run under ADR-0003 D8/D9 — "reported as a later prefix under its own gate record; it
  never rewrites the Gate S1 table" — never a regeneration of the frozen S1.4 packet; (5) the
  branch's STATE rewrote the Gate S1 table rows S1.1 and S1.3 for the longer prefix; on main those
  rows stay at the gate prefix and the later prefix is recorded separately.
- **Candidate 31 (`1406.7174`, Ritter, math.SG; v2 2015-07-16, v3 2023-12-27) was opened but not
  decided**, after the branch's commit and before the divergence was found: both abstract pages
  (v2 44,368 B `4b306576…fdcc0`, v3 44,365 B `abc7321d…e9e0`), both e-prints (v2 114,892 B
  `df580e82…d921`; v3 115,786 B `f8f5aefc…9c62`, single gzipped `main.tex`) and the 72-line v2→v3
  diff were read. The diff replaces `E` by `(X, ω_X)` in Theorem 1.5(2) (the comment's "typo"),
  renames `r_{g^∧}(1)` to `r(g^∧)`, adds a footnote to Lemma 1.6 ("The Lemma as written assumes
  that the lift g^∧ exists (preserving constant discs): this holds for example if D is connected
  …"), adds a footnote in Section 3.2 on the fixed locus, and in Corollary 4.16 changes the
  computed `SH^*(E) ≅ Λ[x_1]/(x_1² − 4t²)` to `≅ Λ` with a footnote ("This assumes char K = 0
  …"); the AI scan's one hit, "llm", is `\smallmatrix`. The verdict (Corollary 4.16 the likely
  item; the typo not one) belongs to the next harvest session, which examines from the log's last
  candidate as D7 requires.
- **Reconciliation on main:** `git fetch` showed 24 remote-only commits — README (PR 1); ADR-0006
  `syndeom-derive` with one canonical child (PR 2); ADR-0007 `syndeom-pair` with one positive pair
  (PRs 3, 4); the frozen S1.4 coverage (PR 5); the first-parent status amendment
  `s1_ingest-v0.1.0-15ab865466d6` with `groundtruth/amendment-map.json` (PRs 6, 7); PR 8's
  readiness reconciliation naming the examination as NEXT. Their STATE pauses the harvest at
  candidate 27 and their S-0066..S-0072 are taken. The local harvest commit was moved to its branch,
  `main` fast-forwarded to `26c2555` (working tree clean, tree identical to the remote), and the
  ORIENT gate run on it, detached with a log (`gate-22a.log`): 22:47:14Z–22:48:06Z, `GATE: GREEN`
  (82 Rust tests, 447 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 72).
- **STATE (this commit, minimal):** Updated line (the PR-8 line demoted to "Previous update");
  the ORIENT note gains "Fetch first"; a "Session 22 build verification" bullet; a dated bullet
  under *Standing escalations and dispositions* records the held branch with its integration
  conditions and the three options; the NEXT keeps the examination and adds the fetch-first
  sentence and the held-branch note. HARVEST-LOG, SOURCES, CLAIMS, FLAWS, the ADRs, `groundtruth/`,
  `analysis/` and the engine are untouched on main. The memory file gained the fetch hazard
  (tooling only; no program state).
BELIEF DELTAS:
- A working copy's `origin/main` is a claim, not evidence, until fetched: §4.1's three commands
  cannot detect a moved remote, and `## main...origin/main` with no ahead/behind count means
  "equal to the last fetch", not "equal to the remote". Evidence: a whole session on a
  24-commit-stale base that passed every §4.1 check and a GREEN gate. Consequence: STATE's ORIENT
  note now says to fetch first; §4.1 itself needs the line (escalated).
- Two executors now write to this repo — Claude in the harvest sessions, Codex through PRs #1–#8 —
  and their states diverge silently unless every session fetches and reads the merged-PR record.
  Evidence: this session. Consequence: the ORIENT note names `git log main..origin/main` and
  `gh pr list --state merged` as the divergence checks.
- The frozen-prefix design (ADR-0003 D8; S1.4 frozen on 27 candidates; Rust consumers asserting 35
  representations) makes any later harvest a separate record, not an extension of the gate set.
  Evidence: the inventory consumer's fail-closed population check. Consequence: the branch needs
  its own snapshot at integration; the gate prefix stays as merged.
- From the harvest, for the record: a withdrawal comment can be a complete status document for
  several results at once, and the helper never reaches its AI scan then (candidate 29); "this
  problem has been overcome in <later paper>" is a third status shape beside "corrected" and
  "remains a conjecture" (candidate 30); shared counters — paper-wide (candidate 28) or per
  section behind macros (candidate 30) — make a result's number resolvable only by counting the
  environments; the finder's cached pages regenerate the candidate list with zero API requests.
CONFUSIONS:
- The stale-ref confusion above. Cost: one harvest session landing off main, plus this
  reconciliation. Doc fix: STATE's ORIENT note and NEXT, the memory hazard, and the §4.1 amendment
  proposal below.
- A PowerShell `(Get-Content …) -replace … | Set-Content -NoNewline` joined the gate wrapper into
  one line, so the first ORIENT gate on the new main was a no-op with an empty log and two idle
  bash processes. Cost: ten minutes. Doc fix: this entry — write wrappers with `printf '%s\n'`.
- `grep -E '^\\b…'` matched word boundaries rather than `\b`-prefixed TeX macros, and a quoted
  heredoc fed to the Bash tool stripped one backslash level from Python source, writing backspace
  characters into the memory file. Cost: three re-greps and one rewrite. Doc fix: the memory file.
- The register checker reads every `S-NNNN` token as a reference: this entry's integration note
  wrote the branch's future source numbers as ids, and the final-tree gate failed on two
  undefined sources until the note was rephrased. Cost: one ten-minute gate run. Doc fix: name
  a not-yet-existing source by number only, never as `S-NNNN`.
HONESTY CHECK: no CLAIM changed status; no register on main changed. The branch's seven S-entries
and five items are VERIFIED/digest-only there and are not part of main's registers; its item 32
records `false` with `undetermined` named as the alternative, its items 33–36 `null`; the
executor's step-level readings are labelled as the executor's. This session's own standing as an
examination is withdrawn above. Downgrades: none.
LEAKAGE CHECK: checked, not assumed. `contact/` holds only its README; nothing from `openai/math`
or any AI-generated corpus was opened or cited; the papers read are 2014 submissions by named human
authors and one 2018–2024 paper by one of them; the scanner ran on candidate 28's v3 and by hand on
candidates 29's v9 and 30's v4 (no hit). No detector or threshold exists. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The branch's five manifests regenerate from
their configs and bytes at `8e222a4`; the candidate list regenerated from the cache with the
identical digest; every retrieval is pinned by URL, access time and SHA-256, and three abstract
pages re-fetched byte-identical. Main's inventory and coverage bytes were not touched. Nothing to
quarantine.
MANIFEST CHECK: five artifacts exist on the branch only, each a self-checked `syndeom-manifest/2`
manifest with `documented-by-source` ground truth, `raw/1` locators, `correct: false` and taxonomy
0.1.0; none on main. Nothing synthetic labeled real (SD12). "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript beyond its own status document (SD3) — the records quote
the authors' words and label the executor's readings; the executor's possible counter-reasoning
about candidate 28's withdrawn theorem was deliberately not written down. No understanding claims
(SD17). No detectors. No referee contact (SD13). No contact ingest. The Gate S1 table on main was
not rewritten and the frozen S1.4 and readiness packets were not touched.
NEXT-SINGLE-ACTION: Gate S1's cold-start examination, in a separate genuinely fresh session
launched with the §6 Session-N resume prompt verbatim and alone, which **runs `git fetch origin`
before §4.1's commands**, reconstructs from the fetched `origin/main` and records its SHA, and
reproduces its recalled memory as its first HANDOFF entry — exactly as STATE.md states it. (The
held harvest branch is Al's decision below; a later harvest session, when ordered, resumes at the
log's last candidate.)
ESCALATIONS:
- **Al's decision — the held harvest branch `claude/s1-harvest-candidates-28-30` (candidates
  28–30, five items):** (a) integrate after Gate S1 closes, as the first later prefix under its own
  record (ADR-0003 D8), with the five integration conditions above; (b) integrate before the
  examination with a new inventory snapshot and an S1.4 re-run on 36 items (the frozen packet
  stays; the readiness packet would need a successor); or (c) discard and re-harvest later from the
  log. The executor recommends (a).
- **Constitution amendment proposal (ADR-0008 if Al agrees):** add `git fetch origin` as the first
  line of §4.1's ORIENT block, and after the symbolic-ref line: "compare `main` with `origin/main`;
  a diverged local commit goes to a branch, never a force-push". Not applied — constitution
  amendments are Al's.
- **For the examination's design:** §4.1 can pass on a stale working copy; the exam record should
  state the fetched `origin/main` SHA it reconstructed from.
- Carried (unchanged, now also in STATE's dispositions): the seven false-versus-undetermined
  readings (the branch would add an eighth, item 32); the seven gap-versus-fatal readings (the
  branch would add items 34 and 35); the unnamed-result D2.3 question (the branch would add
  candidate 28's Corollary 12 and demoted Theorem 3, candidate 30's Lemmas 2.9 and 2.21); and, new
  on the branch, item 36's "overcome in a later paper" truth wording.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: natural boundary — the reconciliation is complete and the NEXT-SINGLE-ACTION is an
examination this session cannot perform; the held branch's disposition awaits Al and blocks
nothing. Repo green on the committed tree (gate run detached with a log on the final tree before
the commit); this commit pushes to `origin/main` as a fast-forward.

SESSION: 2026-10-21 UTC @ d4dd5da956a02bbb4895dc6f6e15d0e16bcf9f23 (Session 23 — orientation repair under Al's decisions of
2026-10-21: ADR-0008 accepted, held-harvest evidence preserved, Gate S1 OPEN; branch
`claude/s1-orientation-repair` from main `d8c0d9cbcef0af5249cbf439808b74d8828bd69d`; this entry
lands in the recording commit on top of the implementation commit `d4dd5da956a02bbb4895dc6f6e15d0e16bcf9f23`)
LAUNCH CONTEXT (SD15; an informed working session, not an examination): the same conversation as
Session 22, continued with Al's task text carrying the three owner decisions and the task; not
cold. Memory written: two machine hazards (a second checkout of this repository exists on this
machine; use `git -C`), no program state.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 OPEN, awaiting the separate genuine cold-start
examination (unchanged). Accepted population unchanged: 35 archived representations, 32 effective
logical items. The held harvest branch is preserved at `c3a3d369689d56bab3678f519904087b313daf4e`
(decision 1); the existing dispositions stand (decision 3).
ORIENT (ADR-0008 D1 as this session ran it): identity `C:/Users/LEET/Syndeom`, remote
`https://github.com/Quigles1337/Syndeom.git`; `git fetch origin` at 2026-10-21T23:30:27Z, exit 0;
default branch `main` (`refs/remotes/origin/HEAD`); fetched `origin/main` =
`d8c0d9cbcef0af5249cbf439808b74d8828bd69d` (tree `f98e7832b3f8c4371e7fae7c6deeffff737f3c84`) =
local `main` = HEAD; working tree clean; no local-only or remote-only commits; the held branch at
`c3a3d369689d56bab3678f519904087b313daf4e` on `origin` — all three observed references verified.
Repair branch created in place (the checkout was clean and at the fetched tip, so no worktree was
needed). Live preflight receipts: `analysis/s1-orientation-repair/evidence/live-orient-*.json`.
PLAN NOTE: Al's task — preserve the held evidence, write the accepted addendum, verify the
failure mode, preserve the accepted corpus and checks, deliver on a reviewable PR. One thing.
ADVANCED:
- **Owner decisions recorded** verbatim in `analysis/s1-orientation-repair/OWNER-DECISIONS.md`
  and bound in STATE's dispositions, ADR-0008 and this entry: (1) the held branch preserved at
  its recorded commit, admission review and integration deferred until after Gate S1 closes,
  nothing approved, reserved, advanced or merged; (2) ADR-0008 as the accepted addendum
  governing §4.1 with the constitution's bytes preserved; (3) existing dispositions retained,
  the held branch's questions pending its later review. Not to be requested again.
- **A. Held evidence preserved:** Session 22's scratchpad (147 files, 17,824,213 bytes) copied to
  `C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\held-evidence\`, every file
  hashed before and verified after the copy; manifest SHA-256
  `d3ea78799dd3e4bbe660368660c17473065768e5ba0b27c7b8c34c89660b87ce`, copied into the event.
  Every saved e-print matches its printed receipt. **Unavailable bytes, reported exactly:** the
  abstract pages of 1403.6317 v1 and v3, 1403.8075 v9 and v11, 1404.4599 v4 and v5, 1406.7174 v2
  and v3, and 1806.08664 — receipts (size, SHA-256, UTC time) exist; the helper never saved the
  bytes; nothing was re-downloaded to stand in for them. Candidate 31's partial inspection (both
  e-prints with full hashes, the 72-line diff, the helper output) is pinned; it remains undecided.
  The branch's manifests and provenance are untouched.
- **B. ADR-0008 written** (`PROGRAM/DECISIONS/ADR-0008.md`): context (the §4.1 block as written
  and the Session-22 failure), options, D1 the complete amended procedure (step 0 SD15 disclosure
  for an examination; identity; fetch or stop; default-branch discovery; snapshot and checkout
  identification with full SHAs, trees, status, ahead/behind lists; selection — synchronized,
  ff-only for clean behind-only on the default branch, isolated worktree otherwise; reconstruct
  only from the examined checkout; recheck before publication; prohibitions), D2 the preflight
  and its receipt and exit codes, D3 the examination accounting (the HANDOFF append is the one
  declared change, never erased; carry-forward through a worktree; preparing a synchronized
  workspace outside the examination; eligibility stays with SD15/ADR-0005), D4 verification, D5
  what is unchanged. The constitution's SHA-256 `89c198b0f1ca7b44f96c9d13816409f65acfb0b8eb3e119cf37003ff15d48102`
  equals the preservation pin; SD15, ADR-0005 and the §6 prompt unchanged.
- **C. Implementation and verification:** `scripts/orient_preflight.py` (orient/recheck, JSON
  receipt, exit 0/2/3/4/1; never force-push, reset, discard or non-ff merge; not in the gate)
  and `scripts/orient_preflight_cases.py` (temporary bare remote and clones). The run of
  2026-10-21 (git 2.49.0.windows.1, Python 3.11.9) passed all eight cases: stale tracking branch
  appears synchronized; fetch exposes the newer snapshot; the stale checkout is not current;
  clean behind-only sync fast-forwards; ahead/diverged commit and dirty work preserved, worktree
  provided; fetch failure stops; target movement detected before publication; the examination's
  HANDOFF append accounted for. Every command and outcome: `evidence/orient-preflight-cases.json`.
  Live: `orient` plain exit 3 (synchronized; two new scripts untracked), with the two scripts
  declared exit 0 at `d8c0d9c`.
- **D. Corpus and checks preserved:** no groundtruth, register, analysis packet, ADR other than
  the new ADR-0008, engine source or pin changed. STATE and README summaries updated (ADR-0008
  bullet, decisions, projections 36/37/40 as conditional, NEXT per ADR-0008 D1); the
  Session-22 attempt record stands as unsuccessful.
- Implementation commit `d4dd5da956a02bbb4895dc6f6e15d0e16bcf9f23` (tree `4a8155dfecfbe422781a832e6550b257c55c84f6`): gate on
  the exact staged tree, detached with a log, 2026-10-21T23:44:48Z–23:55:37Z, exit 0 / `GATE: GREEN` (Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests); 455 tracked files; registers Q 17 / SD 17
  / FLAW 14 / CLAIM 7 / S- 72. Rust reproduction (`reproduce.py`, unchanged consumer) on the
  clean committed tree: **passed** at 2026-10-21T23:58:50Z–23:58:54Z — two Rust runs exit 0 with byte-identical outputs, the overwrite attempt refused (exit 1) with outputs unchanged; inventory SHA-256 `b626d1b53f50a23ee5eae9e99fc6ea387bb83f51c807d843d68a29dd1651b803`, identical to the retained integration inventory (35 archived representations / 32 effective logical items unchanged); provenance bound to `d4dd5da956a02bbb4895dc6f6e15d0e16bcf9f23` / `4a8155dfecfbe422781a832e6550b257c55c84f6`. Receipts in `analysis/s1-orientation-repair/evidence/`.
- **E. Unrelated local work observed, preserved:** `C:\Users\LEET\Documents\ChatGPT\Syndeom`
  (another executor's checkout) on local branch `codex/s1-cold-start-attempt` at
  `af43198324f414687040ef214a3ecb3e71c97e58`, three commits beyond `26c2555` not on `origin` (two
  non-counting cold-start attempt records and a root `AGENTS.md` placing the disclosure before
  orientation). Read only; not fetched, merged, modified or reset. ADR-0008 agrees with its
  ordering and does not depend on it.
- The recording commit (event README, this entry, STATE's build bullet) receives its own gate
  and reproduction, recorded in the pull request and under the external root. The branch is
  pushed and a draft PR opened, left unmerged for review; the pre-publication recheck
  (`orient_preflight.py recheck --baseline d8c0d9c…`) runs immediately before the push.
BELIEF DELTAS:
- The accepted baseline's Rust consumer refuses a dirty checkout, so a preservation reproduction
  certifies only a committed tree; the two-commit pattern (implementation, then recording) is the
  honest way to put its receipt in the repository. Evidence: the exit-1 run on the staged tree.
- Fixtures prove a check; only a live receipt speaks about the live remote. Evidence: the eight
  passing cases say nothing about `origin`; the two live receipts do.
- This machine holds two checkouts and two executors; the remote is the only shared truth, and a
  checkout's remote-tracking ref is a claim until fetched (ADR-0008's premise). Evidence: the
  Codex checkout's three unpushed commits beside Session 22's stale-ref failure.
CONFUSIONS:
- The Bash tool's working directory drifted after a `cd`, so a bare `git checkout -b` failed
  with "not a git repository"; every git command now uses `git -C`. Cost: one retry. Doc fix:
  the memory file.
- The reproduction consumer's clean-checkout requirement was not visible from its usage line;
  the first run failed on the staged tree. Cost: one five-second run. Doc fix: the event README
  records the failed run as a development run and the rule.
HONESTY CHECK: no CLAIM changed status; no register changed; nothing labeled, promoted or
re-measured. The held branch's five labels remain proposals (decision 1). The projections
36/37/40 are stated as conditional. Session 22's ineligibility is restated, not softened.
LEAKAGE CHECK: checked, not assumed — no contact corpus, no AI-generated proof, no detector; the
session touched governance, evidence pins, scripts and summaries only. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The held evidence is pinned by SHA-256
before and after the copy; the preflight receipts record exact commands; the case run is
re-runnable from a fresh work directory. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed on main; the held branch's five manifests are
untouched. "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript; no understanding claims; no detectors; no referee
contact; no S2 work; no examination conducted or implied; no fresh-context subagent described as
cold; the frozen constitution, packets and pins untouched.
NEXT-SINGLE-ACTION: once this repair is on `main`, the separate genuine Gate S1 cold-start
examination in a fresh session launched with the §6 Session-N resume prompt verbatim and alone,
opened per ADR-0008 D1 (step 0 the SD15 disclosure appended first; steps 1–5 the preflight with
`--allow-dirty PROGRAM/HANDOFF.md`, its receipt recorded with the fetched `origin/main` SHA and
tree and the examined checkout) — exactly as STATE.md states it.
ESCALATIONS:
- The draft pull request for this repair awaits Al's review and merge; it is left unmerged.
- The Codex checkout's unpushed branch (above) is Al's to dispose of; nothing here depends on it.
- The held branch's admission review (after Gate S1 closes) will decide the questions it
  introduced; the integration conditions recorded in Session 22 describe that review.
- Human-referee participants remain Al's decision (SD13) — unchanged.
STOP-REASON: task complete at its boundary — the bounded repair is implemented, verified,
recorded and delivered on a reviewable branch; the next action is an examination this session
cannot perform. Repo green on each committed tree (gates recorded above and in the PR).

SESSION: 2026-10-26 UTC @ 0801ecb52d4ec839bed90291527b11ca8c3e1281 (Session 24 — informed
preparation for the Gate S1 cold-start examination, not the examination; branch
`claude/s1-cold-start-prep` from main `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10`; this entry lands
in the second commit on top of `0801ecb52d4ec839bed90291527b11ca8c3e1281`, PR 10)
LAUNCH CONTEXT (SD15; an informed working session, not an examination): launched with Al's task
text (record PR 9's merge, correct stale STATE statements, review executor memory under SD15,
prepare the checkout), not with the §6 prompt; not cold, and it attempts and certifies nothing
about the §2/SD15 test. Memory: reviewed and changed only with Al's approval (ADVANCED, C); no
program state written.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 OPEN, awaiting the separate genuine cold-start
examination (unchanged). Accepted population unchanged: 35 archived representations, 32 effective
logical items. The held branch stays at `c3a3d369689d56bab3678f519904087b313daf4e` (decision 1).
ORIENT (ADR-0008 D1 steps 1–5, `orient_preflight.py orient --sync`): identity
`C:/Users/LEET/Syndeom`, remote `https://github.com/Quigles1337/Syndeom.git`; `git fetch --prune
origin` exit 0, 2026-10-26T18:59:20.94Z–18:59:21.70Z; default branch `main` (from `git ls-remote
--symref`); fetched `origin/main` = `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10` (tree
`b5fe517fb4fa58846df83187aaa11ee7c2954762`); HEAD before = `main` at
`d8c0d9cbcef0af5249cbf439808b74d8828bd69d` (tree `f98e7832b3f8c4371e7fae7c6deeffff737f3c84`),
clean, behind 3 / ahead 0, remote-only `d4dd5da`, `5e24d37`, `ffb63a6`; classification behind;
fast-forwarded (ff-only) and re-verified: examined checkout `C:/Users/LEET/Syndeom` at `ffb63a6…`,
tree `b5fe517f…`, clean. Receipt:
`C:\Users\LEET\Codex\Temp\syndeom-cold-start-prep-20261026\orient-1-sync.json`. No separate ORIENT gate on `ffb63a6`: its tree is the tree PR 9's final gate
certified (that receipt's raw log re-hashed, matching); this session's first gate ran on its own
edited tree (below).
PLAN NOTE: Al's task — make the repository, the local checkout and the executor memory ready for
a genuine cold-start examination, without conducting it. One thing.
ADVANCED:
- **A. PR 9's merge recorded in STATE** (the form of PR 7's record), every value read from git
  before it was written: merge commit `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10`, parents
  `d8c0d9c…` (the PR's base) and `5e24d37…` (its head); merged 2026-10-22T00:07:26Z
  (2026-10-21 20:07:26 America/New_York; `gh pr view 9` mergedAt and the merge commit's committer
  stamp agree); `ffb63a6^{tree}` = `5e24d37^{tree}` = `b5fe517f…` and `git diff --quiet` exits
  0, so the merge tree exactly equals the tested head; that head's gate (external receipt
  `final-gate-5e24d37d8a18\final-gate.json`, 00:00:49Z–00:04:39Z, GATE: GREEN); CI `check`
  pull_request run 39697685870 and push run 39697756714 both success.
- **B. Stale STATE statements corrected:** Blockers no longer says review status lives in PR
  metadata (PRs 8 and 9 are merged); NEXT no longer claims where a local checkout sits (any such
  claim goes stale when the checkout moves; the examination establishes its own snapshot); the
  Gate line, the Gate S1 table's cold-start row and NEXT say plainly that PRs 8 and 9 are on main
  instead of "after/once this reconciliation is on main"; Current build verification names this
  PR's first committed tree, the PR 8 readiness bullet kept as historical. No register, frozen
  packet, ADR, groundtruth item, engine source or pin changed; the constitution's SHA-256 remains
  `89c198b0f1ca7b44f96c9d13816409f65acfb0b8eb3e119cf37003ff15d48102`.
- **C. SD15 memory review** of every executor memory file naming SYNDEOM, KOBER or FRIEDMAN
  (eight files: the index, the three program files, three pre-trim `.bak` backups, one memory
  for unrelated public repositories). Every state-bearing line was reported to Al verbatim; with
  Al's approval: (a) the SYNDEOM memory file — a dated incident narrative and a description of
  the constitution's §4.1 text replaced with a hazard-only line, a run count and a default-branch
  value removed; SHA-256 now `163c7b066d8e36392a218599e6b41958e8a947684e3ed2a2e8bae205dfc22787`
  (10,573 bytes; before this session
  `5b0505d1c06855e515eddb84039e32017a3330c975c8803a36b11cf9a7aad1df`); (b) the three `.bak`
  backups, which held sibling programs' state, moved (not deleted) out of the memory folder to
  `C:\Users\LEET\memory-archive\claude-memory-bak-20261026\`, hashes verified after the move;
  (c) the unrelated memory reworded so it no longer names this program or a sibling. The current
  KOBER and FRIEDMAN files and the index lines hold no program state and are unchanged.
- **D. Gates.** First commit `0801ecb52d4ec839bed90291527b11ca8c3e1281` (tree
  `78a30035d7137eb0a52ab200b3ba25406331c855`): full gate on the exact staged tree, committed
  unchanged, detached with a log, 2026-10-26T19:02:21Z–19:12:29Z, exit 0 / `GATE: GREEN` (Lean
  8,926 jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests, 461 tracked files, Q 17 /
  SD 17 / FLAW 14 / CLAIM 7 / S- 72); receipt and raw log under the external root. Pre-publication
  recheck (`recheck --baseline ffb63a6…`) exit 0, target unchanged. The commit carrying this
  entry receives its own gate, recorded in PR 10 and under the external root.
BELIEF DELTAS:
- A statement in STATE about where a local checkout sits is true only until the checkout moves.
  Evidence: Session 23's note placed this checkout on the repair branch; at this session's start
  it was on `main`, three commits behind the fetched tip.
- A memory trim leaves the untrimmed text behind unless its backups are removed. Evidence: three
  unindexed `.bak` files beside the trimmed ones still held sibling phases, gate closures, SHAs
  and next actions.
CONFUSIONS:
- The behind checkout lacked `scripts/orient_preflight.py` (added by the commits it was behind).
  The preflight ran from a copy extracted from the fetched ref (`git show
  origin/main:scripts/orient_preflight.py`, blob `c09abc5d…` identical). Cost: none. Doc fix:
  this line.
HONESTY CHECK: no CLAIM, FLAW or SOURCE entry changed status; nothing labeled, promoted or
re-measured. The Rust reproduction was not rerun (only STATE and HANDOFF change), and STATE says
so. Session 22's ineligibility stands; nothing here counts toward Gate S1.
LEAKAGE CHECK: checked, not assumed — no contact corpus, AI-generated proof or detector touched;
the session changed summaries and executor memory only. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. Receipts record exact commands, trees and
digests. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed. "Proven" applied to nothing.
DRIFT CHECK: no verdict on any manuscript; no understanding claims; no detectors; no referee
contact; no S2 work; no examination conducted or implied.
NEXT-SINGLE-ACTION: the separate genuine Gate S1 cold-start examination in a fresh session
launched with the §6 Session-N resume prompt verbatim and alone, opened per ADR-0008 D1 (step 0
the SD15 disclosure appended first; steps 1–5 the preflight with `--allow-dirty
PROGRAM/HANDOFF.md`, its receipt recorded with the fetched `origin/main` SHA and tree and the
examined checkout) — exactly as STATE.md states it.
ESCALATIONS:
- PR 10 awaits Al's review and merge; it is left unmerged. After the merge Al has authorized the
  executor to fetch, leave this checkout on `main` at the fetched tip with a clean tree, and
  delete remote branches git confirms are fully merged into `main`, excluding
  `claude/s1-harvest-candidates-28-30` (decision 1).
- The held branch's admission review (after Gate S1 closes), the Codex checkout's unpushed branch
  (Session 23, E; not re-inspected) and human-referee participants (SD13) remain Al's — unchanged.
STOP-REASON: natural boundary — the preparation is complete and delivered on a reviewable PR; the
next action is an examination this session cannot perform. Repo green on each committed tree
(gates recorded above and in the PR).

SESSION: 2026-10-26 UTC (Session 25 — a fresh conversation launched with the constitution's §6
Session-N resume prompt; it attempted Gate S1's cold-start examination and is **not eligible as an
examination**, because its SD15 disclosure came late (first entry below). This header line was
written with the disclosure, so the session-end SHA is on this block's STOP-REASON line, not here.)
RECALLED MEMORY (SD15 as amended by ADR-0005; ADR-0008 D1 step 0 — the first entry of this block,
appended 2026-10-26T19:36:13Z by a script that copied the memory bytes verbatim; no executor memory has been
written this session and none will be):
- Launch: the launching message was the §6 "Session N — resume" prompt, text identical to
  constitution lines 474–478, delivered alone inside the harness's pasted-content wrapper, in a
  fresh conversation holding no transcript of any earlier session. The harness also attached its
  usual context: the executor's memory index (`MEMORY.md`), the user's e-mail address, and the
  environment block. No other memory file was loaded by the harness.
- **Disclosure order — LATE.** Before this entry was written the session had already, in this
  order: (1) read the SYNDEOM memory file below (`cat`); run `git status`, `git log --oneline
  -15` and `ls` in `C:\Users\LEET\Syndeom`; (2) run `git fetch --all --prune` (no output, exit 0),
  `git status -sb`, `git log --oneline -5 origin/main`, `git branch -a`, `ls PROGRAM` and
  `wc -l PROGRAM/*.md`; (3) read the constitution in full; (4) read `PROGRAM/STATE.md` in full;
  (5) listed `PROGRAM/DECISIONS/` and the `SESSION:` header lines of this file; (6) read ADR-0008 and
  ADR-0005 in full and this file's Session 6 block (the Gate S0 exam, as a disclosure pattern);
  (7) computed the digests and name matches reproduced here. Items (1)–(2) are orientation
  commands, and (2)'s fetch preceded the preflight; items (1), (2), (4), (5) and (6) read Git
  history, STATE, earlier HANDOFF entries and an ADR other than ADR-0005. ADR-0008 D1 step 0
  permits only the constitution and ADR-0005 before step 5, and SD15 puts this disclosure "before
  any other action". The order is an eligibility criterion (ADR-0008 D3: "launch context, memory
  content, disclosure order"), so **this session cannot perform or certify the §2/SD15
  examination**, whatever its later findings. The disclosure is made anyway, in full, so that the
  record of what this session recalled is complete; it is never to be erased or rewritten.
- The memory index on disk at disclosure time: 110 lines, 15,420 bytes, SHA-256
  `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`. The harness reported at
  launch that the index had changed on disk since its last read (lines 2–18 shown; the changed
  text is line 10, a machine hazard about `grep`, naming no program). The lines that name SYNDEOM
  or a sibling program are reproduced below exactly as they stand on disk, and they are identical
  to the text the harness loaded; lines 1–4 (title and user identity) are reproduced as Session 6
  did. **Withheld by count (ADR-0005 D2): 99 lines — lines 5–84, 90–105 and 108–110** — which
  concern unrelated programs and machine notes. **Check (ADR-0005 D3):** a case-insensitive
  Python regular-expression search of the whole index for `syndeom|kober|friedman|fr13dman`
  (Python, not Git Bash `grep`, because of the `grep -i` abort hazard; a canary string matched
  and a non-matching one did not) matched lines **85, 86, 88, 89, 106, 107** only, every one of
  them among the lines reproduced.

Index lines 1–4, 85–89 and 106–107 of `MEMORY.md`, verbatim:
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

File `project_syndeom.md` (10,573 bytes, LF, SHA-256
`163c7b066d8e36392a218599e6b41958e8a947684e3ed2a2e8bae205dfc22787`), verbatim:
```text
---
name: project-syndeom
description: "SYNDEOM program - repo C:\\Users\\LEET\\Syndeom (remote Quigles1337/Syndeom, public); constitution and ALL program state live IN the repo, never here"
metadata:
  node_type: memory
  type: project
  originSessionId: 07cebf44-c690-47a3-8c97-d4b909c4e75b
  modified: 2026-10-26T19:18:39.389Z
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

**Never work in the sibling KOBER or FR13DMAN trees** — identical `PROGRAM/` schema, and their
sessions may be live concurrently.

Machine and harness hazards:
- System-level git config has `core.autocrlf=true`; the repo pins LF via `.gitattributes`. Verify
  with `git ls-files --eol` and a byte-level CR/BOM check, never by eye.
- The Python console is cp1252: set `PYTHONIOENCODING=utf-8` before any script that prints
  non-ASCII (`§`, `→`, `—`), or it dies with a UnicodeEncodeError after doing its work.
- Cloning an empty GitHub repo yields the server's default branch; `git init` on this
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
- A ferried file pasted into the chat never matches its stated digest: the harness rendering of a
  Markdown file flattens tables to spaced text, drops `=` lines out of display math, strips heading
  and bullet markers, and prefixes/suffixes stray quotes. Transcribe and hash the paste once to
  show the mismatch, then ask for a path on disk holding the exact bytes. Never "repair" a paste
  into the ferried artifact.
- `www.apache.org` / `apache.org` (Fastly, 151.101.x) refused TCP connections from both curl and
  the fetch tool for a whole session while DNS resolved; the SPDX License List
  (`spdx.org/licenses/<id>.txt`, no trailing newline) and the GitHub Licenses API
  (`gh api licenses/<key>` — no leading slash, or Git Bash rewrites it as a path) both serve
  licence texts and cross-check each other after whitespace normalization.
- Project Euclid sits behind an Incapsula bot challenge (HTTP 200 with a ~1 KB challenge page to
  curl, empty content to the fetch tool). The Internet Archive's raw-capture route
  `web.archive.org/web/<timestamp>id_/<url>` serves the archived publisher PDF; find the timestamp
  with `archive.org/wayback/available?url=…`. Old JSTOR scans have no text layer — `pdftotext`
  returns only the JSTOR footer — so render pages with `pdftoppm -r 110 -png` and read the images.
- `scripts/check.sh` can take 10+ minutes after a reboot even with `lean/.lake` warm: the axiom
  audit (`lake env lean Audit.lean`) loads all of Mathlib's oleans from a cold disk cache
  (lean.exe grows past 1.4 GB). Run the gate in the background with a log; the ~30 s figure holds
  only with a warm OS file cache.
- arXiv API: `http://export.arxiv.org/api/query` answers 301 with an empty body (looks dead to a
  plain curl); use `https://export.arxiv.org/api/query`. The `co:` field prefix searches the
  revision comment (`search_query=co:erratum+AND+cat:math.*`); results are Atom XML.
- Crossref REST: `https://api.crossref.org/works?filter=update-type:erratum,type:journal-article`
  works (also `correction`, `corrigendum`, `retraction`); each hit carries `update-to`
  `[{type, DOI}]`; always pass `mailto=` for the polite pool. No subject filter — use zbMATH
  (`https://api.zbmath.org/v1/document/_search?search_string=ti:erratum`, JSON with
  `status.nr_total_results` and DOI `links`) to confine to mathematics.
- PowerShell's `bash` on this machine resolves to the Windows WSL stub (`C:\WINDOWS\system32\bash.exe`),
  which exits immediately; a launcher must name `C:\Program Files\Git\bin\bash.exe` explicitly.
- Run the gate detached with a log under the session scratchpad — `Start-Process` of Git Bash on a
  wrapper script that echoes start/end timestamps and the exit code, with `-RedirectStandardOutput`
  — so a harness kill (it reaps background shells when system memory is low) does not lose the
  result; a background `until grep -q 'GATE end' <log>; do sleep 10; done` is the notifier.
- Never run two gates at once: each axiom audit loads Mathlib's oleans (lean.exe past 1.4 GB), and
  two together starve the machine. Wait for `=== GATE end` with a foreground
  `until grep -q '=== GATE end' <log>; do sleep 10; done` loop (this harness tolerates it), and
  launch the pre-commit gate only after the ORIENT gate has ended.
- Python launched from Git Bash does not resolve `/c/Users/...` paths (FileNotFoundError on open);
  write `C:/Users/...` inside Python strings, and keep `/c/...` for the shell side only.
- The Route A finder caches its arXiv API pages only under the scratchpad of the session that
  fetched them (`<Temp>/claude/C--Users-LEET/<session-id>/scratchpad/route-a/pages/`). Copy
  `pages/*.xml` and `finder-state.json` from an earlier session's scratchpad (find it with
  `ls -d .../C--Users-LEET/*/scratchpad/route-a`) into the new one before running the finder:
  0 requests instead of ~30 min of fetching, and the digest check still proves identity.
- In `grep -E` / `awk` on this machine a pattern written `'^\\b[A-Z]'` matches a word boundary,
  not a literal backslash-b; write `'^[\\]b[A-Z]'` to match TeX macros such as `\bD`, `\bL`.
  A `\\section` pattern that found nothing in one call and everything in another was the same
  trap; prefer a bracket expression for a literal backslash. A quoted heredoc (`<<'EOF'`) fed
  to the Bash tool also strips one backslash level from Python source, leaving backspace
  characters where `\b` was meant: write backslash-bearing text with the Write/Edit tools.
- A working copy's `origin/<branch>` ref is only as fresh as its last fetch: `git status` can
  report a checkout as in sync while the remote is many commits ahead. Never treat a checkout's
  tracking ref as the remote's state. Never force-push; a diverged local commit goes to a branch.
- **A second checkout of this repository exists on this machine** at
  `C:\Users\LEET\Documents\ChatGPT\Syndeom` (used by another executor, Codex; it can hold its
  own unpushed local branches). Never work in it, reset it, or treat its refs as the remote's;
  `C:\Users\LEET\Syndeom` is this executor's checkout, and `origin` is the only shared truth.
  Durable external evidence roots for this program live under `C:\Users\LEET\Codex\Temp\`
  (one new directory per event, never reused); the session scratchpad is not durable.
- Use `git -C <path>` for every git command in the Bash tool: its working directory drifts
  between calls (a `cd` persists), and a bare `git` then fails with "not a git repository".
```
Judgment on the recalled memory: no reproduced memory holds program state in SD15's enumeration
(phase, gate status, CLAIM or FLAW statuses, file contents, commit SHAs as state markers, next
actions, escalations). Lines an auditor may want to look at, each judged an environment or tooling
hazard and not state: the external evidence root under `C:\Users\LEET\Codex\Temp\` (a location
convention); the second checkout used by another executor (environment); the Route A finder's
per-session page cache (a tool's caching behaviour; it names no harvest position); the gate's
timing, the axiom-audit memory use and the one-gate-at-a-time rule (tooling); "the gate runs
`fmt --check`" (kept by Al's Session-10 ruling). Values recalled from memory and used: the repo
location, and the hazards as hazards (the "fetch is only as fresh as the last fetch" hazard is why
the session fetched first, which is exactly the order error recorded above). No value in any
register, STATE row or later reconstruction in this block is taken from memory.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 OPEN. This session is the launch the inherited
NEXT asked for, and it is an **ineligible** examination attempt (first entry). Gate S1 stays OPEN,
and nothing here counts toward it.
ORIENT (ADR-0008 D1 steps 1–5, run after the disclosure: `python -I -X utf8
scripts/orient_preflight.py orient --expect-url https://github.com/Quigles1337/Syndeom.git
--allow-dirty PROGRAM/HANDOFF.md --receipt …\preflight-orient.json`, exit 0,
SNAPSHOT-ESTABLISHED): identity `C:/Users/LEET/Syndeom`, remote
`https://github.com/Quigles1337/Syndeom.git`; `git fetch --prune origin` exit 0,
2026-10-26T19:36:22.35Z–19:36:23.12Z; default branch `main` (from `git ls-remote --symref`);
fetched `origin/main` = `535641227f408095f36a92000e808ed5b65e135c` (tree
`1fdeda3eca982fbaec04ab2b1f8fdaaeb82c70ea`); HEAD = `main` at the same SHA and tree; ahead 0 /
behind 0, no local-only or remote-only commits; working tree `PROGRAM/HANDOFF.md` modified
(`allowed_present`), nothing unrelated; classification synchronized; examined checkout
`C:/Users/LEET/Syndeom`. Receipt SHA-256 `1ad9f62728ff7faf9c98cd70a45230cc7422ce10753107993b220f75aa14d149`
under the external root `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\`. The
constitution and ADR-0005 on disk equal the snapshot's blobs (SHA-256 `89c198b0…8102`, the
preservation baseline, and `873f0751…7595`). The pre-disclosure `git status` showed `main` at
`5356412` up to date with its tracking ref; the pre-disclosure `git fetch --all --prune` printed
nothing; the preflight then found HEAD equal to the fetched tip. So what the session read before
disclosing was this same snapshot. Read in §4.1 order: STATE (in full, before the disclosure),
HANDOFF Session 24 (after it), CLAIMS statuses, the FLAWS header, ADR-0005 and ADR-0008 in full,
then the gate.
PLAN NOTE: once the order error was found (before the disclosure was written), the inherited
NEXT, the examination, could no longer be carried out. Session 22 is the precedent for a
procedurally defective attempt: the attempt is recorded and certifies nothing. One thing: (A)
reconcile STATE with PR 10's merge (§4.1's first task, because STATE predated the merge), and (B)
record this attempt as ineligible, with its cause, and escalate the discoverability gap. No
re-derivation is offered for certification. The checks in C are the verification of STATE
against reality that every session owes.
ADVANCED:
- **A. PR 10's merge recorded in STATE** (Updated bullet). Every value was read from git and `gh`:
  merge commit `535641227f408095f36a92000e808ed5b65e135c`, parents
  `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10` (the PR's base) and
  `3f0171a7dc36f90154739b5bbafce11da8f37dc2` (its head); `gh pr view 10` mergedAt
  2026-10-26T19:28:28Z, mergedBy `Quigles1337`; committer stamp 2026-10-26T15:28:27-04:00
  (19:28:27Z, one second before mergedAt); `5356412^{tree}` = `3f0171a^{tree}` = `1fdeda3e…`, and
  `git diff --quiet` exits 0. The head's gate receipt, `gate-commit-3f0171a.json` (19:22:04Z–19:22:58Z,
  GREEN, raw log SHA-256 `5710d20c…8c8a`), was read from Session 24's external root. CI
  pull_request run 40386340630 and push run 40387061357 both succeeded. The remote's heads are now
  `main` and the held branch only. Local branches `claude/s1-cold-start-prep` and
  `claude/s1-orientation-repair` still exist in this checkout (their remote copies are gone); they
  are left as they are.
- **B. STATE updated:** the Gate line, the Gate S1 table's cold-start row, Blockers, a new first
  standing escalation, NEXT's leading paragraph and Current build verification. Session 24's
  Updated bullet is kept as "Previous update (Session 24)", and its build bullet as "PR 10 build
  verification".
- **C. ORIENT verification of STATE against the snapshot (non-certifying):** `lean/lean-toolchain`
  `leanprover/lean4:v4.34.1`; mathlib rev `d13f23b723b8a846827a245b89c10fc7d3f11612` in
  `lean/lakefile.toml`; `rust-toolchain.toml` channel `1.91.0` with clippy + rustfmt;
  `engine/Cargo.lock` 34 packages; workspace members manifest, ingest, derive and pair; 35
  `manifest.json` files under `groundtruth/` (32 `s1_ingest`, plus one each of `s1_pair_archive`,
  `s1_derive` and `s1_pair`); the effective inventory
  `analysis/s1-first-parent-integration/report/inventory.json`, SHA-256 `b626d1b5…1b803` (=
  STATE's), with counts 35 archived / 33 raw / 2 canonical / 32 effective / 18 unflagged / 5
  observed and 9 zero-coverage classes; `groundtruth/HARVEST-LOG.md` last candidate `1403.4485`
  (27 of 2,472), resume at 28; CLAIM statuses VERIFIED ×5, CONDITIONAL (0006) and VERIFIED (0007,
  existence only); FLAWS `0.1.0`, 14 entries; ADR-0008 the highest ADR, so ADR-0009 is free; the
  held branch at `c3a3d369…` on the remote. Every item agrees with STATE. The one disagreement was
  PR 10's merge (A).
- **D. Gates.** The ORIENT gate on main `5356412` plus the disclosure ran 19:36:57Z–19:41:40Z and
  was GREEN (STATE, Current build verification; receipt `gate-orient.json`). The commit carrying
  this block gets its own full gate on its exact staged tree, recorded in its PR and under the
  external root.
BELIEF DELTAS:
- Step 0 is a trap for an executor that follows §4.1's text literally. Session-start me read
  SD15's "in any session attempting a gate's cold-start test" as a condition to settle later, from
  STATE. ADR-0008 D1 makes STATE off limits until after the disclosure, so the condition has to be
  settled before reading anything that could settle it. Evidence: the constitution and ADR-0005
  never name ADR-0008 (`grep -c`: 0 in each), and the only statements that an examination is
  scheduled are STATE's NEXT and ADR-0008. Consequence: the escalation's option (a).
- The rule can be satisfied as written. SD15's last sentence ("a cold session is launched with
  the §6 resume prompt verbatim and nothing that carries program state") lets a careful cold
  executor conclude that it may be the examination, and disclosing first costs nothing. So this
  is partly my error and partly fragile design, not a contradiction. Evidence: those same
  sentences. Consequence: option (c) in the escalation is an honest option, but without an
  amendment the next attempt depends on that inference.
- My first command ran before I had read the constitution. The prompt's order ("Read … in full,
  then run the loop") already forbade that. Evidence: item (1) of the disclosure's order list.
  Consequence: none for the repository; it is executor discipline, stated so that Al can weigh it.
- A late disclosure can still be complete and untampered. Evidence: a script copied the bytes,
  and the memory file's digest `163c7b06…2787` equals the post-trim digest Session 24 recorded
  with Al's approval. The order rule still binds, so this does not restore eligibility.
CONFUSIONS:
- I took the memory hazard "a working copy's `origin/<branch>` ref is only as fresh as its last
  fetch" as a cue to fetch at once. In a session that may be an examination, the disclosure comes
  first. Cost: the examination. Doc fix: the escalation, because a fix in STATE or HANDOFF cannot
  reach a cold executor in time; only the constitution or ADR-0005 can.
- A `sed` edit of my own helper script stripped its `\n` escapes (the recorded backslash hazard).
  It was caught before the script ran, and the disclosure's bytes came from the script after the
  fix. Cost: one edit. Doc fix: none needed, since the hazard is already recorded.
HONESTY CHECK: no CLAIM, FLAW or SOURCE entry changed status; nothing was labeled, promoted,
measured or reproduced. The attempt is recorded as ineligible, not as a PASS, a CONDITIONAL PASS
or a "PASS pending ruling", and the ORIENT checks are recorded as verification, not as an
examination result. Session 22's ineligibility stands. Downgrades: none. The Gate S1 cold-start
row now records this unsuccessful attempt.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened; no AI-generated proof, detector,
generator or threshold was touched; no design decision was taken (the escalation proposes a
procedural amendment that concerns no data). Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The gate result and the digests can be
regenerated (`bash scripts/check.sh` at the recorded tree; `sha256sum`), and the receipts record
exact commands, trees and times. Nothing to quarantine.
MANIFEST CHECK: no artifact was created or changed; "proven" was applied to nothing; nothing
synthetic was labeled real.
DRIFT CHECK: no statement about any real manuscript (SD3); no understanding claims (SD17); no
detectors; no referee contact (SD13); no S2 work; no examination certified or implied; the held
branch untouched.
NEXT-SINGLE-ACTION: the Gate S1 cold-start examination in a separate fresh session, launched with
the §6 Session-N resume prompt verbatim and alone and opened per ADR-0008 D1. Its very first
action, before any command and before reading anything but the constitution and ADR-0005, is the
SD15 disclosure appended to HANDOFF; then comes the preflight with `--allow-dirty
PROGRAM/HANDOFF.md`, exactly as STATE.md states it. Al may rule on the step-0 escalation first,
but the attempt does not depend on it.
ESCALATIONS:
- **New — examination step 0 cannot be reached in time from the files a cold executor may read
  first** (STATE, Standing escalations, first bullet). Options: (a) amend SD15 by ADR
  (recommended), (b) amend the §6 prompt, (c) no amendment. This is Al's decision; nothing is
  drafted or applied.
- This session's branch `claude/s1-session-25-exam-attempt` and its pull request await Al's
  review and merge; they are left unmerged.
- Unchanged: the held branch's admission review after Gate S1 closes (decision 1); the Codex
  checkout's unpushed branch (Session 23, E; not re-inspected); human-referee participants (SD13).
STOP-REASON: natural boundary. The only open Gate S1 criterion is an examination this session can
no longer perform. Session-end SHA: the commit carrying this block, on top of main
`535641227f408095f36a92000e808ed5b65e135c` on branch `claude/s1-session-25-exam-attempt`; its hash
and its gate are recorded in the pull request, not here (that would be a self-reference). Repo
green: the ORIENT gate above, and the final staged-tree gate before the commit.

SESSION: 2026-10-26 UTC (Session 26 — informed working session: ADR-0009, the SD15 / §6 disclosure-first
amendment; branch `claude/s1-adr-0009-disclosure-first` from main
`3cd5558223294571dd85164eb0d2730cb1911637`; the session-end SHA is the commit carrying this block,
recorded in its pull request, not here)
LAUNCH CONTEXT (SD15; an informed working session, not an examination): launched with Al's task
text (draft ADR-0009 proposing Session 25's options (a) and (b) and ADR-0008 step 0 beyond
examinations; show the exact before/after text; change nothing until accepted; then apply, record,
update every record of the constitution's SHA-256, update STATE and HANDOFF, gate, open a PR and
stop), not with the §6 prompt. Not cold; it attempts and certifies nothing about Gate S1. The
amended step 0 does not bind it (it was not launched with the §6 prompt). Memory: the harness
loaded the executor's memory index; the SYNDEOM memory file was read at session start; no
executor memory was written.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 OPEN, awaiting the separate cold-start
examination (unchanged). Accepted population unchanged: 35 archived representations, 32 effective
logical items. The held branch stays at `c3a3d369689d56bab3678f519904087b313daf4e` (decision 1).
ORIENT: at session start, before the preflight, the session ran `git status -sb`, `git log
--oneline -10` and `ls` in this checkout, then `git fetch --all --prune` (exit 0; `origin/main`
moved `5356412..3cd5558`, PR 11's merge), and read the constitution, ADR-0005, ADR-0008, STATE and
HANDOFF Sessions 24–25 from branch `claude/s1-session-25-exam-attempt` at `e1c8150`, whose tree
`125b1ed56d8dcab8074119eb51fc3467dbb09dda` equals the fetched tip's (`git diff --quiet e1c8150
3cd5558` exit 0): what it read was the fetched snapshot's content. The ADR-0008 D1 steps 1–5
receipt came afterwards, before any edit: `git checkout main` (clean), then `python -I -X utf8
scripts/orient_preflight.py orient --expect-url https://github.com/Quigles1337/Syndeom.git --sync
--receipt …\preflight-orient.json`, exit 0, SNAPSHOT-ESTABLISHED: identity
`C:/Users/LEET/Syndeom`, remote `https://github.com/Quigles1337/Syndeom.git`; `git fetch --prune
origin` exit 0, 2026-10-26T20:10:11.20Z–20:10:12.04Z; default branch `main` (from `git ls-remote
--symref`); fetched `origin/main` = `3cd5558…` (tree `125b1ed5…`); HEAD before = `main` at
`535641227f408095f36a92000e808ed5b65e135c` (tree `1fdeda3e…`), clean, behind 2 / ahead 0,
remote-only `e1c8150`, `3cd5558`; classification behind; fast-forwarded (ff-only); examined
checkout `C:/Users/LEET/Syndeom` at `3cd5558…`. Receipt SHA-256
`a6de02956584746b701d59df9aece098ee918fd8637e1b0c5a5a42a672458ea7` under the external root
`C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\`. No separate ORIENT gate: tree `125b1ed5…`
is the tree Session 25's final staged-tree gate certified (19:44:55Z–19:45:36Z, GREEN; its raw log
re-hashed, `b003ccee…054b`, matching).
PLAN NOTE: Al's task, one thing — decide and apply the step-0 escalation by ADR. Draft first (no
repository change), stop for Al's decision, then apply exactly what Al accepted.
ADVANCED:
- **A. Draft, measured before it was offered.** Mapping every record of the constitution's
  SHA-256 (`git grep 89c198b0` at `3cd5558`: 17 lines in 15 files) found that the S1.3
  effective-inventory reproduction checks its accepted baseline byte for byte at HEAD,
  constitution and ADR-0005 included. A scratch-clone probe (2026-10-26T20:01:43Z–20:01:58Z):
  `3cd5558` PASS with `inventory.json` identical to the retained `b626d1b5…1b803`; one word of the
  constitution changed → `FAIL: historical bytes changed: PROGRAM/SYNDEOM-PROGRAM.md`; one word of
  ADR-0005 changed → the same on ADR-0005. The draft therefore left ADR-0005 and SIBLING-MAP
  unedited and offered the break to Al as D5. The before/after text of every change went to Al
  with four decision points; nothing in the repository changed until Al's acceptance.
- **B. Al's acceptance (2026-10-26)**, with rulings 1–4 (accept the break, STATE to say plainly
  that S1.3 is reproduced at `3cd5558`, and a non-blocking note on the tool; scope "launched with
  the §6 Session-N resume prompt"; keep the memory-reading allowance and the ADR-0008 pointer; add
  a directory listing to the definition of reading program state) and additions 5–6 (append
  without reading HANDOFF; provisional block header). Transcribed verbatim in ADR-0009.
- **C. Applied.** Constitution: SD15 (the rest of the paragraph reflowed, words unchanged), the §6
  Session-N prompt (first sentence: the disclosure; then appending without reading, the
  provisional header, and ADR-0008 for orientation), the header and footer *Amendments* lines;
  version string unchanged. SHA-256 `89c198b0f1ca7b44f96c9d13816409f65acfb0b8eb3e119cf37003ff15d48102`
  (37,609 bytes) → `110f5360af60075a8e799dce0dc235d1ef8785bd40b0b34c38fa53ddb1d5479c` (38,958
  bytes), LF, no BOM. ADR-0008: the line before step 0, step 0 (scope, appending without reading,
  provisional header, ADR-0008 readable after disclosing, directory listings, memory), step 4,
  D3's first two bullets, and a new *Amendments* section naming both digests. ADR-0009 recorded,
  accepted.
- **D. Records of the constitution's SHA-256.** Updated: STATE (the ADR-0008 orientation bullet;
  the Session-25 escalation, now decided, and its text removed from STATE — it stands in HANDOFF
  Session 25 and in PR 11). Left byte-identical as records of their own baselines (ADR-0009 D4):
  HANDOFF lines 3794, 3910, 4183; ADR-0008 line 23 (annotated by its *Amendments* entry); the
  eleven frozen evidence files under `analysis/` listed in ADR-0009.
- **E. STATE:** new Updated bullet (PR 11's merge: 2026-10-26T19:55:55Z, by Al, parents `5356412`
  and `e1c8150`, tree equal to the head's, CI runs 40389787616 and 40391129353 success, all read
  from `gh` and git); a bullet stating the two S1.3 reproduction facts after ADR-0009 (G); the
  Gate line, the
  Constitution line, Memory policy (disclosure timing), the Gate S1 table's S1.3 and cold-start
  rows, Blockers, the escalation decided, the non-blocking note on the reproduction tool, Current
  build verification and NEXT.
- **F. Reproduction evidence** (`run-repro.sh`, log `repro.log` under the external root,
  2026-10-26T20:12:33Z–20:12:44Z, scratch clones of this checkout): R1 at `3cd5558` (tree
  `125b1ed5…`) PASS, `inventory.json` `b626d1b5…1b803` = retained; R2 with the constitution,
  ADR-0008 and ADR-0009 as amended (clone tree `d23b2e1b…`) `FAIL: historical bytes changed:
  PROGRAM/SYNDEOM-PROGRAM.md`; R3 the same with the constitution restored (tree `c300eabc…`) PASS,
  inventory identical. So the constitution is the only file of this change the tool rejects;
  STATE is a named summary and HANDOFF a preserved prefix for it.
- **G. The first staged-tree gate was RED; Al's option 1 fixed it.** Full gate on staged tree
  `1e43f5d5…` (2026-10-26T20:16:14Z–20:16:51Z, exit 101, nothing committed; `gate-staged.json`):
  4 of the 15 `first_parent_amendment` tests failed with `historical bytes changed:
  PROGRAM/SYNDEOM-PROGRAM.md` — their fixture reads the frozen baseline from the working tree.
  Ruling 1's premise (the historical-byte tests use in-memory fixtures) was wrong, as Al recorded;
  this session's draft had missed the same test. Extent (`tests-extent.log`, `--no-fail-fast`,
  scratch clones): on that tree only those four failed; with the constitution restored all 82
  passed; with a one-line `preserve()` skip all 82 passed and the reproduction passed at HEAD,
  differing only in `preservation.exact_files_checked` 370 → 369. Four options went to Al; Al
  chose option 1. Applied: `preserve()` skips `PROGRAM/SYNDEOM-PROGRAM.md` beside `README.md` and
  `PROGRAM/STATE.md`, one comment line citing ADR-0009; `cargo fmt --check` clean; source
  `effective.rs` SHA-256 `516bd44e20a73259f7577ffd6a41230d99141d33ca16147d0b0b7719eea99775` →
  `92a816bcc7cf02acec98bcda1fc7775444f331aede1ca9786c23d750d4dcb30c`; nothing else in the tool
  changed. R5 (`run-repro-2.sh`, `repro-R5.log`, 2026-10-26T20:32:02Z–20:32:11Z) on the staged tree
  then holding the fix: PASS, `inventory.json` `df3a2c7f…a7f5`, one field difference from the
  retained file, `preservation.exact_files_checked` 370 → 369. ADR-0009 (Status, Scope, Context,
  D5, Consequences, the ruling verbatim) and STATE (both facts; the non-blocking note marked
  partly done) record it. The commit carrying this block gets a full gate on its exact staged
  tree, detached with a log, recorded in its pull request and under the external root.
BELIEF DELTAS:
- A rule that tells a cold session how to *disclose* has to tell it how to *write*, too. Evidence:
  file-editing tools read a file before changing it, and HANDOFF's earlier entries are program
  state (Al's addition 5); the block header's session number is likewise unknowable before
  orientation (addition 6). Session-start me had covered only the order of reading.
- Amending the constitution was not free: a frozen reproduction and the gate's tests built on it
  treat it as an input. Evidence: R1–R3 and the red gate (G). Consequence: the tool skips the
  constitution (D5); narrowing the rest of its byte checks is a note, partly done.
- "The gate is unaffected" was a claim, and only the gate could settle it. Evidence: the red
  gate. A test that mutates blobs in memory can still load them from the working tree.
CONFUSIONS:
- I fetched and read STATE, HANDOFF and the ADRs before running the ADR-0008 preflight, which D1
  orders first for every session. No consequence here — the checkout's tree equalled the fetched
  tip's, and the receipt was taken before any edit — but the order was wrong. Doc fix: none; D1
  already says it, and the amended §6 prompt now names ADR-0008 for a §6-launched session.
- My draft looked for tests that read HEAD only under `examples/` and missed
  `engine/ingest/tests/first_parent_amendment.rs`, whose fixture reads the working tree; Al's
  ruling 1 then rested on the same miss. Cost: one red staged-tree gate and a second ruling;
  nothing was committed red. Doc fix: ADR-0009 Context and D5, and STATE's non-blocking note,
  which says the gate's tests run the same check.
HONESTY CHECK: no CLAIM, FLAW or SOURCE entry changed status; nothing labeled, promoted, measured
or reproduced as a new result (R1–R5 re-ran an existing reproduction to locate a constraint and
check the fix). Sessions 22 and 25 stay ineligible; ADR-0009 is not retroactive. Downgrades: the
premise of ruling 1 is recorded as wrong in ADR-0009, and the first gate is recorded as RED, not
omitted.
LEAKAGE CHECK: checked, not assumed — `contact/` not opened; no AI-generated proof, detector,
generator or threshold touched; the amendment concerns session procedure only. Verdict: no
leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. R1–R5 record exact revisions, trees and
digests and rerun from `run-repro.sh` and `run-repro-2.sh`. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK: no statement about any real manuscript (SD3); no understanding claims (SD17); no
detectors; no referee contact (SD13); no S2 work; no examination conducted or implied; the held
branch untouched.
NEXT-SINGLE-ACTION: the Gate S1 cold-start examination in a separate fresh session launched with
the constitution's §6 Session-N resume prompt as amended by ADR-0009, verbatim and alone; its
first action is the SD15 disclosure appended to HANDOFF without reading that file, under a
provisional header; then ADR-0008 D1 steps 1–5 with `--allow-dirty PROGRAM/HANDOFF.md` — exactly
as STATE.md states it. An examiner rerunning the S1.3 reproduction gets the retained inventory
byte-identical at `3cd5558` and, at HEAD, a PASS differing only in
`preservation.exact_files_checked` 370 → 369.
ESCALATIONS:
- This session's branch `claude/s1-adr-0009-disclosure-first` and its pull request await Al's
  review and merge; they are left unmerged.
- Unchanged: the held branch's admission review after Gate S1 closes (decision 1); the Codex
  checkout's unpushed branch (Session 23, E; not re-inspected); human-referee participants (SD13).
STOP-REASON: natural boundary — the accepted amendment is applied, recorded and delivered on a
reviewable PR; Al reviews before merge. Repo green: Session 25's gate on this session's base tree,
and the final staged-tree gate before the commit (recorded in the PR); the first staged-tree gate
was RED and nothing was committed until the fix was in.

SESSION: 2026-10-26 UTC (Session 27 — launched with the §6 Session-N resume prompt in its ADR-0009
wording, inside the conversation that built Session 26: not cold, not an examination; branch
`claude/s1-session-27-sec6-launch` from main `6e198e066c128976d55d0194f278cc645baf9249`; header
completed after ADR-0008 step 5 — it was written provisionally with the disclosure, which is
unchanged; the session-end SHA is the commit carrying this block, recorded in its pull request)
RECALLED MEMORY (SD15 as amended by ADR-0009; ADR-0005 D1–D3; ADR-0008 D1 step 0 — the first entry of
this block, appended 2026-10-26T21:07:05Z by a script that copied the memory bytes verbatim and opened
`PROGRAM/HANDOFF.md` for appending only, never reading it; no executor memory has been written this
session):
- Launch: the launching message was the §6 "Session N — resume" prompt in its ADR-0009 wording,
  delivered alone inside the harness's pasted-content wrapper — but **in the same conversation that
  had just drafted, applied and opened ADR-0009 as Session 26**. That conversation holds the program's
  state in its context. Under SD15's conversation-context corollary ("a session running in the
  conversation context that built the state under test is not cold, regardless of memory") **this
  session is not cold and cannot be an examination**, whatever its disclosure order. It attempts and
  certifies nothing about any gate's cold-start test. The disclosure is made because ADR-0009 binds
  every session launched with this prompt, examination or not.
- Disclosure order in this session: (1) read `PROGRAM/SYNDEOM-PROGRAM.md` (the harness answered from
  its record that the file was unchanged since this conversation last read it) and
  `PROGRAM/DECISIONS/ADR-0005.md` in full; (2) wrote this script to the session scratchpad and ran
  it to read and hash the memory and print this block; (3) ran it again to append. No git command,
  directory listing, build or other repository read came first in this session.
- The memory index on disk at disclosure time: 110 lines, 15,420 bytes, SHA-256
  `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`. The harness loaded it at this
  conversation's start; no other memory file was loaded by the harness. The SYNDEOM memory file was
  read earlier in this conversation (Session 26) and again now. **Withheld by count (ADR-0005 D2):
  104 lines — lines 1–84, 87, 90–105, 108–110** — the title, the user-identity lines and lines
  concerning unrelated programs and machine notes. **Check (ADR-0005 D3):** a case-insensitive Python
  regular-expression search of the whole index for `syndeom|kober|friedman|fr13dman` (Python, because
  of the Git Bash `grep -i` abort hazard; a canary string matched and a non-matching one did not)
  matched lines **85, 86, 88, 89, 106, 107** only, every one of them reproduced below.

Index lines 85, 86, 88, 89, 106, 107 of `MEMORY.md`, verbatim, in order:
```text
## KOBER (Linear A falsification infrastructure)
- [Repo pointer + machine hazards](project_kober.md) — `C:\Users\LEET\KOBER`, remote `Quigles1337/KOBER` (**public**); the constitution `PROGRAM/KOBER-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (memory-policy ADR).
## FR13DMAN (FRIEDMAN — designedness-detection instrument)
- [Repo pointer + machine hazards](project_friedman.md) — `C:\Users\LEET\FR13DMAN` / `Quigles1337/FR13DMAN` (**public**, leetspeak name); sibling of KOBER; the constitution `PROGRAM/FRIEDMAN-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (ADR-0003).
## SYNDEOM (AI-proof calibration — third sibling of KOBER/FRIEDMAN)
- [Repo pointer + machine hazards](project_syndeom.md) — `C:\Users\LEET\Syndeom` / `Quigles1337/Syndeom` (**public**); the constitution `PROGRAM/SYNDEOM-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (SD15 / FR13DMAN ADR-0003).
```

File `project_syndeom.md` (10,573 bytes, LF, SHA-256
`163c7b066d8e36392a218599e6b41958e8a947684e3ed2a2e8bae205dfc22787`), verbatim:
```text
---
name: project-syndeom
description: "SYNDEOM program - repo C:\\Users\\LEET\\Syndeom (remote Quigles1337/Syndeom, public); constitution and ALL program state live IN the repo, never here"
metadata:
  node_type: memory
  type: project
  originSessionId: 07cebf44-c690-47a3-8c97-d4b909c4e75b
  modified: 2026-10-26T19:18:39.389Z
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

**Never work in the sibling KOBER or FR13DMAN trees** — identical `PROGRAM/` schema, and their
sessions may be live concurrently.

Machine and harness hazards:
- System-level git config has `core.autocrlf=true`; the repo pins LF via `.gitattributes`. Verify
  with `git ls-files --eol` and a byte-level CR/BOM check, never by eye.
- The Python console is cp1252: set `PYTHONIOENCODING=utf-8` before any script that prints
  non-ASCII (`§`, `→`, `—`), or it dies with a UnicodeEncodeError after doing its work.
- Cloning an empty GitHub repo yields the server's default branch; `git init` on this
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
- A ferried file pasted into the chat never matches its stated digest: the harness rendering of a
  Markdown file flattens tables to spaced text, drops `=` lines out of display math, strips heading
  and bullet markers, and prefixes/suffixes stray quotes. Transcribe and hash the paste once to
  show the mismatch, then ask for a path on disk holding the exact bytes. Never "repair" a paste
  into the ferried artifact.
- `www.apache.org` / `apache.org` (Fastly, 151.101.x) refused TCP connections from both curl and
  the fetch tool for a whole session while DNS resolved; the SPDX License List
  (`spdx.org/licenses/<id>.txt`, no trailing newline) and the GitHub Licenses API
  (`gh api licenses/<key>` — no leading slash, or Git Bash rewrites it as a path) both serve
  licence texts and cross-check each other after whitespace normalization.
- Project Euclid sits behind an Incapsula bot challenge (HTTP 200 with a ~1 KB challenge page to
  curl, empty content to the fetch tool). The Internet Archive's raw-capture route
  `web.archive.org/web/<timestamp>id_/<url>` serves the archived publisher PDF; find the timestamp
  with `archive.org/wayback/available?url=…`. Old JSTOR scans have no text layer — `pdftotext`
  returns only the JSTOR footer — so render pages with `pdftoppm -r 110 -png` and read the images.
- `scripts/check.sh` can take 10+ minutes after a reboot even with `lean/.lake` warm: the axiom
  audit (`lake env lean Audit.lean`) loads all of Mathlib's oleans from a cold disk cache
  (lean.exe grows past 1.4 GB). Run the gate in the background with a log; the ~30 s figure holds
  only with a warm OS file cache.
- arXiv API: `http://export.arxiv.org/api/query` answers 301 with an empty body (looks dead to a
  plain curl); use `https://export.arxiv.org/api/query`. The `co:` field prefix searches the
  revision comment (`search_query=co:erratum+AND+cat:math.*`); results are Atom XML.
- Crossref REST: `https://api.crossref.org/works?filter=update-type:erratum,type:journal-article`
  works (also `correction`, `corrigendum`, `retraction`); each hit carries `update-to`
  `[{type, DOI}]`; always pass `mailto=` for the polite pool. No subject filter — use zbMATH
  (`https://api.zbmath.org/v1/document/_search?search_string=ti:erratum`, JSON with
  `status.nr_total_results` and DOI `links`) to confine to mathematics.
- PowerShell's `bash` on this machine resolves to the Windows WSL stub (`C:\WINDOWS\system32\bash.exe`),
  which exits immediately; a launcher must name `C:\Program Files\Git\bin\bash.exe` explicitly.
- Run the gate detached with a log under the session scratchpad — `Start-Process` of Git Bash on a
  wrapper script that echoes start/end timestamps and the exit code, with `-RedirectStandardOutput`
  — so a harness kill (it reaps background shells when system memory is low) does not lose the
  result; a background `until grep -q 'GATE end' <log>; do sleep 10; done` is the notifier.
- Never run two gates at once: each axiom audit loads Mathlib's oleans (lean.exe past 1.4 GB), and
  two together starve the machine. Wait for `=== GATE end` with a foreground
  `until grep -q '=== GATE end' <log>; do sleep 10; done` loop (this harness tolerates it), and
  launch the pre-commit gate only after the ORIENT gate has ended.
- Python launched from Git Bash does not resolve `/c/Users/...` paths (FileNotFoundError on open);
  write `C:/Users/...` inside Python strings, and keep `/c/...` for the shell side only.
- The Route A finder caches its arXiv API pages only under the scratchpad of the session that
  fetched them (`<Temp>/claude/C--Users-LEET/<session-id>/scratchpad/route-a/pages/`). Copy
  `pages/*.xml` and `finder-state.json` from an earlier session's scratchpad (find it with
  `ls -d .../C--Users-LEET/*/scratchpad/route-a`) into the new one before running the finder:
  0 requests instead of ~30 min of fetching, and the digest check still proves identity.
- In `grep -E` / `awk` on this machine a pattern written `'^\\b[A-Z]'` matches a word boundary,
  not a literal backslash-b; write `'^[\\]b[A-Z]'` to match TeX macros such as `\bD`, `\bL`.
  A `\\section` pattern that found nothing in one call and everything in another was the same
  trap; prefer a bracket expression for a literal backslash. A quoted heredoc (`<<'EOF'`) fed
  to the Bash tool also strips one backslash level from Python source, leaving backspace
  characters where `\b` was meant: write backslash-bearing text with the Write/Edit tools.
- A working copy's `origin/<branch>` ref is only as fresh as its last fetch: `git status` can
  report a checkout as in sync while the remote is many commits ahead. Never treat a checkout's
  tracking ref as the remote's state. Never force-push; a diverged local commit goes to a branch.
- **A second checkout of this repository exists on this machine** at
  `C:\Users\LEET\Documents\ChatGPT\Syndeom` (used by another executor, Codex; it can hold its
  own unpushed local branches). Never work in it, reset it, or treat its refs as the remote's;
  `C:\Users\LEET\Syndeom` is this executor's checkout, and `origin` is the only shared truth.
  Durable external evidence roots for this program live under `C:\Users\LEET\Codex\Temp\`
  (one new directory per event, never reused); the session scratchpad is not durable.
- Use `git -C <path>` for every git command in the Bash tool: its working directory drifts
  between calls (a `cd` persists), and a bare `git` then fails with "not a git repository".
```
Judgment on the recalled memory: deferred to after orientation (this block's later fields); the
disclosure above is complete as it stands and is never to be erased or rewritten.
MEMORY JUDGMENT (after orientation): no reproduced memory holds program state in SD15's
enumeration. The index lines and the memory file are byte-identical to those Session 25 disclosed
and judged (index `cd943842…fc69`, memory file `163c7b06…2787`); the same lines an auditor may want
to look at (the external evidence root, the second checkout, the Route A page cache, gate timing
and the one-gate rule, "the gate runs `fmt --check`") are environment and tooling hazards. No value
in any register or STATE row below was taken from memory — but this session's *conversation*
holds the program's state (LAUNCH, first bullet of the disclosure), which is why it is not cold.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 OPEN. This session is a §6 launch inside the
builder's conversation: not cold under SD15's corollary, so not an examination, and it certifies
nothing. Whether the launch was meant as the examination is Al's to say; it is recorded as a
non-cold §6 launch, not as a Gate S1 attempt. Accepted population unchanged: 35 archived
representations, 32 effective logical items. The held branch stays at
`c3a3d369689d56bab3678f519904087b313daf4e` (decision 1).
ORIENT (ADR-0008 D1 steps 1–5, run after the disclosure: `python -I -X utf8
scripts/orient_preflight.py orient --expect-url https://github.com/Quigles1337/Syndeom.git
--allow-dirty PROGRAM/HANDOFF.md --receipt …\preflight-orient.json`, exit 0,
SNAPSHOT-ESTABLISHED): identity `C:/Users/LEET/Syndeom`, remote
`https://github.com/Quigles1337/Syndeom.git`; `git fetch --prune origin` exit 0,
2026-10-26T21:07:14.03Z–21:07:14.85Z; default branch `main` (from `git ls-remote --symref`);
fetched `origin/main` = `6e198e066c128976d55d0194f278cc645baf9249` (tree
`4ad8422654cb56d9ffe68a717100df5af240a22e`); HEAD = `main` at the same SHA and tree; ahead 0 /
behind 0, no local-only or remote-only commits; working tree `PROGRAM/HANDOFF.md` modified
(`allowed_present`), nothing unrelated; classification synchronized; examined checkout
`C:/Users/LEET/Syndeom`. Receipt SHA-256
`79dd210da55382b180469064c93ce0e0a1b17845cee3ebd059b3c3c5ab554bac` under the external root
`C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\`. Session 26 had left this checkout on its
PR branch; it was found on `main` at PR 12's merge, moved between the sessions, not by this one.
Step 5's confirmation: the constitution (`110f5360…479c`) and ADR-0005 (`873f0751…7595`) on disk
equal the snapshot's blobs. Then, in §4.1 order: STATE, HANDOFF Session 26, CLAIMS statuses
(VERIFIED ×5, CONDITIONAL 0006, VERIFIED 0007 existence only), FLAWS `0.1.0` (14 entries),
ADR-0009 and ADR-0008 — all as this conversation wrote or last read them, except that STATE did
not yet record PR 12's merge. No separate ORIENT gate: the snapshot's tree `4ad84226…` is the tree
Session 26's final staged-tree gate certified (2026-10-26T20:34:44Z–20:35:33Z, GREEN;
`gate-final.json`, raw log `b25710de…c96e` re-hashed and matching).
PLAN NOTE: the inherited NEXT (the Gate S1 cold-start examination) survived ORIENT but cannot be
carried out here: this session is not cold. One thing: reconcile STATE with PR 12's merge, the
one disagreement §4.1 found, and record this launch.
ADVANCED:
- **A. The ADR-0009 opening, executed as written for the first time.** The constitution and
  ADR-0005 read in full (the harness answered the constitution read from its record that the file
  was unchanged since this conversation last read it); the memory read and hashed by a scratchpad
  script; the disclosure appended through an append-only file handle, never reading HANDOFF, at
  2026-10-26T21:07:05Z; a provisional header, completed after step 5 (this block's first lines)
  with the disclosure unchanged; the preflight accepted the declared append (`allowed_present`,
  `unrelated` empty). Before the header was completed, HANDOFF's committed prefix was unchanged and
  its tail was exactly a newline plus the appended block (SHA-256 `0bb0d5b3…25e4`).
- **B. PR 12's merge recorded in STATE** (Updated bullet), every value read from git and `gh`:
  merge commit `6e198e066c128976d55d0194f278cc645baf9249`, parents
  `3cd5558223294571dd85164eb0d2730cb1911637` (the PR's base) and
  `61cc974ca67bb47d0ac7aaa5f2a9b6c219969b19` (its head); `gh pr view 12` mergedAt
  2026-10-26T21:03:28Z, mergedBy `Quigles1337`; committer stamp 2026-10-26T17:03:28-04:00;
  `6e198e0^{tree}` = `61cc974^{tree}` = `4ad84226…` and `git diff --quiet` exits 0. CI
  pull_request run 40397221741 and push run 40401087386 both succeeded. ADR-0009 and the amended
  §6 prompt are on main. STATE also gains: Session 26's Updated bullet kept as "Previous update",
  Current build verification, Blockers, the Gate S1 table's cold-start row and NEXT.
- **C. Gate.** The commit carrying this block gets a full gate on its exact staged tree, detached
  with a log, recorded in its pull request and under the external root.
BELIEF DELTAS:
- The ADR-0009 opening is executable as written: an append-only script, a provisional header and
  the preflight's `--allow-dirty PROGRAM/HANDOFF.md` fit together without reading HANDOFF early.
  Evidence: A and the receipt.
- The prompt's wording cannot make a session cold; only its context can. Evidence: SD15's
  corollary, and this session, whose disclosure is procedurally complete and certifies nothing.
CONFUSIONS:
- Whether this launch was meant as the Gate S1 examination is unknown to the executor. Cost: none
  to the repository; the examination is still owed. Doc fix: NEXT now says plainly that the prompt
  pasted into a conversation that already holds the program's state yields a non-cold session.
HONESTY CHECK: no CLAIM, FLAW or SOURCE entry changed status; nothing labeled, promoted, measured
or reproduced. This session is recorded as non-cold and not an examination, neither as a PASS nor
as an ineligible attempt. Downgrades: none.
LEAKAGE CHECK: checked, not assumed — `contact/` not opened; no AI-generated proof, detector,
generator or threshold touched. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results; receipts record exact commands, trees and
digests. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK: no statement about any real manuscript (SD3); no understanding claims (SD17); no
detectors; no referee contact (SD13); no S2 work; no examination conducted or implied; the held
branch untouched.
NEXT-SINGLE-ACTION: the Gate S1 cold-start examination in a separate fresh session — a new
conversation that holds no transcript of earlier sessions — launched with the constitution's §6
Session-N resume prompt (ADR-0009 wording) verbatim and alone, opened by that prompt's disclosure
and then ADR-0008 D1 — exactly as STATE.md states it.
ESCALATIONS:
- This session's branch `claude/s1-session-27-sec6-launch` and its pull request await Al's review
  and merge; they are left unmerged.
- Whether this §6 launch was meant as the examination is Al's to say (PHASE / GATE); nothing here
  depends on the answer.
- The merged branches `claude/s1-session-25-exam-attempt` and `claude/s1-adr-0009-disclosure-first`
  remain on the remote; deleting them is Al's call (Session 24's authorization covered PR 10's
  cleanup only).
- Unchanged: the held branch's admission review after Gate S1 closes (decision 1); the Codex
  checkout's unpushed branch (Session 23, E; not re-inspected); human-referee participants (SD13).
STOP-REASON: natural boundary — the only open Gate S1 criterion is an examination this session
cannot perform; the reconciliation is delivered on a reviewable PR. Repo green: Session 26's
final gate on this session's base tree, and the staged-tree gate before the commit (recorded in
the PR).

SESSION: 2026-10-26 UTC (Session 28 — a fresh conversation launched with the §6 Session-N resume
prompt in its ADR-0009 wording, verbatim and alone, but inside the checkout `C:\Users\LEET\Syndeom`:
the harness attached Git status and history before the first action, so it is not an
examination; finished as a recorded non-exam session at Al's direction; branch
`claude/s1-session-28-launch-directory` from main `0afbbd17cdc5e9e93691612a95bd5950e3243e10`;
header completed after ADR-0008 step 5 — it was written provisionally with the disclosure, which
is unchanged; the session-end SHA is the commit carrying this block, recorded in its pull request)

### SD15 memory disclosure (first entry of this block)

Written before any other action and appended by a shell `cat >>` without reading this file
(ADR-0009; the §6 Session-N prompt). Actions taken before this entry, in order, and nothing else:
(1) read `PROGRAM/SYNDEOM-PROGRAM.md` in full; (2) read `PROGRAM/DECISIONS/ADR-0005.md` in full;
(3) listed and hashed this session's project memory directory; (4) located executor memory
outside it by a bounded search of `~/.claude/projects/*/memory` (directory names, file counts,
and the names of files matching the program's or a sibling's name); (5) read and hashed the
memory reproduced below and ran the ADR-0005 D3 check. No other repository file was read, no
directory of the checkout was listed, no git command was run, and no build was started.

**Launch.** A fresh conversation with no prior turns, so not the builder's conversation. The
whole first message is a paste that reads, on visual comparison, word for word as the §6
Session-N prompt; it is not byte-verified (a paste is not bytes).

**Harness context before the first action: Git status and history.** Before the first tool
call, the harness attached a gitStatus snapshot to the conversation. Verbatim:

~~~text
Current branch: main

Main branch (you will usually use this for PRs): main

Git user: Quigles1337

Status:
(clean)

Recent commits:
0afbbd1 Merge pull request #13 from Quigles1337/claude/s1-session-27-sec6-launch
fbadee3 [S1][INFRA] Session 27: a §6 launch in the builder's conversation, not cold; PR 12's merge in STATE
6e198e0 Merge pull request #12 from Quigles1337/claude/s1-adr-0009-disclosure-first
61cc974 [S1][ADR-0009] disclosure first in every §6-launched session: amend SD15, the §6 Session-N prompt and ADR-0008 step 0
3cd5558 Merge pull request #11 from Quigles1337/claude/s1-session-25-exam-attempt
~~~

That is Git status and history (a phase marker, session numbers, ADR and PR numbers) received
before disclosing. SD15: "A session that reads program state before disclosing — any repository
file but this constitution and ADR-0005, a listing of the checkout's directories, Git history or
status, or a build — cannot be an examination." **This session therefore cannot be an
examination**, whatever its memory holds. The executor did not request the snapshot; the harness
attached it at launch. The harness also attached the account e-mail address (withheld; not
program-related) and the tool and skill catalogue (no program content).

**Recalled memory (what the harness loaded): none.** The working directory is
`C:\Users\LEET\Syndeom`. Its project memory directory
`~/.claude/projects/C--Users-LEET-Syndeom/memory/` holds 0 files (directory mtime
2026-10-26 17:45:30 -0400, this session's launch), so no memory index was loaded into context.
No user-level `~/.claude/CLAUDE.md` exists.

**Executor memory read for this disclosure.** The machine-wide memory used by sessions launched
from `C:\Users\LEET` is in `~/.claude/projects/C--Users-LEET/memory/`. Its index, `MEMORY.md`, has
110 lines and 15,420 bytes, is LF only (0 CR), has mtime 2026-10-26 15:30:13 -0400, and has
SHA-256 `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`.

(a) Index lines naming this program or a sibling, verbatim (ADR-0005 D1(a)), concatenated from
the file's bytes: lines 85, 86, 88, 89, 106 and 107, in order:

~~~text
## KOBER (Linear A falsification infrastructure)
- [Repo pointer + machine hazards](project_kober.md) — `C:\Users\LEET\KOBER`, remote `Quigles1337/KOBER` (**public**); the constitution `PROGRAM/KOBER-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (memory-policy ADR).
## FR13DMAN (FRIEDMAN — designedness-detection instrument)
- [Repo pointer + machine hazards](project_friedman.md) — `C:\Users\LEET\FR13DMAN` / `Quigles1337/FR13DMAN` (**public**, leetspeak name); sibling of KOBER; the constitution `PROGRAM/FRIEDMAN-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (ADR-0003).
## SYNDEOM (AI-proof calibration — third sibling of KOBER/FRIEDMAN)
- [Repo pointer + machine hazards](project_syndeom.md) — `C:\Users\LEET\Syndeom` / `Quigles1337/Syndeom` (**public**); the constitution `PROGRAM/SYNDEOM-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (SD15 / FR13DMAN ADR-0003).
~~~

(b) This program's memory file, `project_syndeom.md`, in full and exactly as read, concatenated
from the file's bytes rather than transcribed. It has 0 CR, no BOM, mtime 2026-10-26 15:18:39
-0400, and SHA-256 `163c7b066d8e36392a218599e6b41958e8a947684e3ed2a2e8bae205dfc22787`:

~~~markdown
---
name: project-syndeom
description: "SYNDEOM program - repo C:\\Users\\LEET\\Syndeom (remote Quigles1337/Syndeom, public); constitution and ALL program state live IN the repo, never here"
metadata:
  node_type: memory
  type: project
  originSessionId: 07cebf44-c690-47a3-8c97-d4b909c4e75b
  modified: 2026-10-26T19:18:39.389Z
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

**Never work in the sibling KOBER or FR13DMAN trees** — identical `PROGRAM/` schema, and their
sessions may be live concurrently.

Machine and harness hazards:
- System-level git config has `core.autocrlf=true`; the repo pins LF via `.gitattributes`. Verify
  with `git ls-files --eol` and a byte-level CR/BOM check, never by eye.
- The Python console is cp1252: set `PYTHONIOENCODING=utf-8` before any script that prints
  non-ASCII (`§`, `→`, `—`), or it dies with a UnicodeEncodeError after doing its work.
- Cloning an empty GitHub repo yields the server's default branch; `git init` on this
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
- A ferried file pasted into the chat never matches its stated digest: the harness rendering of a
  Markdown file flattens tables to spaced text, drops `=` lines out of display math, strips heading
  and bullet markers, and prefixes/suffixes stray quotes. Transcribe and hash the paste once to
  show the mismatch, then ask for a path on disk holding the exact bytes. Never "repair" a paste
  into the ferried artifact.
- `www.apache.org` / `apache.org` (Fastly, 151.101.x) refused TCP connections from both curl and
  the fetch tool for a whole session while DNS resolved; the SPDX License List
  (`spdx.org/licenses/<id>.txt`, no trailing newline) and the GitHub Licenses API
  (`gh api licenses/<key>` — no leading slash, or Git Bash rewrites it as a path) both serve
  licence texts and cross-check each other after whitespace normalization.
- Project Euclid sits behind an Incapsula bot challenge (HTTP 200 with a ~1 KB challenge page to
  curl, empty content to the fetch tool). The Internet Archive's raw-capture route
  `web.archive.org/web/<timestamp>id_/<url>` serves the archived publisher PDF; find the timestamp
  with `archive.org/wayback/available?url=…`. Old JSTOR scans have no text layer — `pdftotext`
  returns only the JSTOR footer — so render pages with `pdftoppm -r 110 -png` and read the images.
- `scripts/check.sh` can take 10+ minutes after a reboot even with `lean/.lake` warm: the axiom
  audit (`lake env lean Audit.lean`) loads all of Mathlib's oleans from a cold disk cache
  (lean.exe grows past 1.4 GB). Run the gate in the background with a log; the ~30 s figure holds
  only with a warm OS file cache.
- arXiv API: `http://export.arxiv.org/api/query` answers 301 with an empty body (looks dead to a
  plain curl); use `https://export.arxiv.org/api/query`. The `co:` field prefix searches the
  revision comment (`search_query=co:erratum+AND+cat:math.*`); results are Atom XML.
- Crossref REST: `https://api.crossref.org/works?filter=update-type:erratum,type:journal-article`
  works (also `correction`, `corrigendum`, `retraction`); each hit carries `update-to`
  `[{type, DOI}]`; always pass `mailto=` for the polite pool. No subject filter — use zbMATH
  (`https://api.zbmath.org/v1/document/_search?search_string=ti:erratum`, JSON with
  `status.nr_total_results` and DOI `links`) to confine to mathematics.
- PowerShell's `bash` on this machine resolves to the Windows WSL stub (`C:\WINDOWS\system32\bash.exe`),
  which exits immediately; a launcher must name `C:\Program Files\Git\bin\bash.exe` explicitly.
- Run the gate detached with a log under the session scratchpad — `Start-Process` of Git Bash on a
  wrapper script that echoes start/end timestamps and the exit code, with `-RedirectStandardOutput`
  — so a harness kill (it reaps background shells when system memory is low) does not lose the
  result; a background `until grep -q 'GATE end' <log>; do sleep 10; done` is the notifier.
- Never run two gates at once: each axiom audit loads Mathlib's oleans (lean.exe past 1.4 GB), and
  two together starve the machine. Wait for `=== GATE end` with a foreground
  `until grep -q '=== GATE end' <log>; do sleep 10; done` loop (this harness tolerates it), and
  launch the pre-commit gate only after the ORIENT gate has ended.
- Python launched from Git Bash does not resolve `/c/Users/...` paths (FileNotFoundError on open);
  write `C:/Users/...` inside Python strings, and keep `/c/...` for the shell side only.
- The Route A finder caches its arXiv API pages only under the scratchpad of the session that
  fetched them (`<Temp>/claude/C--Users-LEET/<session-id>/scratchpad/route-a/pages/`). Copy
  `pages/*.xml` and `finder-state.json` from an earlier session's scratchpad (find it with
  `ls -d .../C--Users-LEET/*/scratchpad/route-a`) into the new one before running the finder:
  0 requests instead of ~30 min of fetching, and the digest check still proves identity.
- In `grep -E` / `awk` on this machine a pattern written `'^\\b[A-Z]'` matches a word boundary,
  not a literal backslash-b; write `'^[\\]b[A-Z]'` to match TeX macros such as `\bD`, `\bL`.
  A `\\section` pattern that found nothing in one call and everything in another was the same
  trap; prefer a bracket expression for a literal backslash. A quoted heredoc (`<<'EOF'`) fed
  to the Bash tool also strips one backslash level from Python source, leaving backspace
  characters where `\b` was meant: write backslash-bearing text with the Write/Edit tools.
- A working copy's `origin/<branch>` ref is only as fresh as its last fetch: `git status` can
  report a checkout as in sync while the remote is many commits ahead. Never treat a checkout's
  tracking ref as the remote's state. Never force-push; a diverged local commit goes to a branch.
- **A second checkout of this repository exists on this machine** at
  `C:\Users\LEET\Documents\ChatGPT\Syndeom` (used by another executor, Codex; it can hold its
  own unpushed local branches). Never work in it, reset it, or treat its refs as the remote's;
  `C:\Users\LEET\Syndeom` is this executor's checkout, and `origin` is the only shared truth.
  Durable external evidence roots for this program live under `C:\Users\LEET\Codex\Temp\`
  (one new directory per event, never reused); the session scratchpad is not durable.
- Use `git -C <path>` for every git command in the Bash tool: its working directory drifts
  between calls (a `cd` persists), and a bare `git` then fails with "not a git repository".
~~~

(c) Other memory naming this program: none. The sibling files `project_kober.md` (SHA-256
`b093bc495be4bedc56f421c2cc6852dab6a66ab5b621a108cbf49112ed64f7d6`) and `project_friedman.md`
(SHA-256 `1d7d173d6a97ba086fd663878c24d87e2bf3ee29414831ae71648e4022ff5832`) each contain 0
case-insensitive matches for `syndeom`. The harness loaded neither, neither was read beyond that
count, and neither is reproduced (Session-6 practice per ADR-0005: the index lines plus this
program's file). The three other non-empty project memory directories on the machine (names
withheld; unrelated programs) hold no file matching `syndeom|kober|friedman|fr13dman`.

**Withheld by count (ADR-0005 D2):** 104 index lines, at lines 1–84, 87, 90–105 and 108–110
(33 blank, 71 non-blank). In step (5), a `stat` also printed into this session's context the
names and mtimes of the other `project_*.md` files in that directory. Their names are withheld
here, and their contents were not read.

**Check (ADR-0005 D3):** `grep -n -i -E 'syndeom|kober|friedman|fr13dman'` over the whole index
matched lines 85, 86, 88, 89, 106 and 107, exactly the lines reproduced under (a). `fr13dman` is
added to the ADR's list of names because FRIEDMAN's repository name is leetspeak; it matched no
line beyond those. No withheld line names the program or a sibling.

**Assessment against SD15's enumeration** (phase, gate status, CLAIM or FLAW statuses, file
contents, commit SHAs as state markers, next actions, escalations): no reproduced line states
any of these. Some items are hazards in form but sit next to program state; they are recorded
here for Al's audit:
- the gate hazards name `scripts/check.sh` and its axiom audit `lake env lean Audit.lean`, which
  gives away the gate's existence and composition;
- the "Route A finder" hazard names a program tool, its cache layout and its arXiv use;
- the Crossref/zbMATH erratum-query and Project Euclid/JSTOR hazards imply that errata retrieval
  (S1.1-type work) has been done.

None says what is done or what comes next. For this session the point is moot, since it is
already excluded from examination above.

**Memory writes:** none this session.

MEMORY JUDGMENT (after orientation): no reproduced memory holds program state in SD15's
enumeration. The index (`cd943842…fc69`) and the SYNDEOM memory file (`163c7b06…2787`) are
byte-identical to those Sessions 25 and 27 disclosed and judged. The lines the disclosure flagged
as state-adjacent (the gate's axiom audit, the Route A page cache, the erratum-query routes) are
hazards in form, of the kind Session 27 judged environment and tooling hazards, and this session
agrees. Eligibility does not turn on this judgment: the harness's Git snapshot already excludes
this session. No value in STATE or in this block was taken from memory or from the snapshot;
every value was read again from git, `gh` and the files after the preflight.
LAUNCH-DIRECTORY FINDING (recorded in STATE's Memory policy at Al's direction of 2026-10-26): a
session launched inside the checkout receives a Git status and history snapshot from the harness
before its first action. Here that was the current branch, `git status` and the last five commit
subjects, reproduced in the disclosure. The session therefore holds program state before it can
disclose (SD15: "Git history or status"), so it can never be an examination, whatever its
prompt, memory or disclosure order. The launch directory also selects the memory store the
harness loads: this session's key, `C--Users-LEET-Syndeom`, has an empty memory directory, so no
index was loaded. `C:\Users\LEET` is not a git work tree: `git -C /c/Users/LEET rev-parse
--is-inside-work-tree` exits 128 with "not a git repository". HANDOFF Sessions 6, 7, 25 and 27
each record the harness loading the executor's memory index at start (the `C--Users-LEET`
index), so they were launched from `C:\Users\LEET`, and none records a Git snapshot. The
examination is therefore launched from `C:\Users\LEET`, outside the checkout, as those sessions
were. No constitution or ADR text changes: SD15 already names Git history and status, and already
requires a cold launch to carry "nothing that carries program state".
PHASE / GATE: S1 — GROUND-TRUTH CORPUS / Gate S1 OPEN. This session is not an examination and
certifies nothing. At Al's direction it is recorded as a non-exam §6 launch, not as a Gate S1
attempt. Accepted population unchanged: 35 archived representations, 32 effective logical items.
The held branch stays at `c3a3d369689d56bab3678f519904087b313daf4e` (decision 1).
ORIENT (ADR-0008 D1 steps 1–5, run after the disclosure: `python -I -X utf8
C:/Users/LEET/Syndeom/scripts/orient_preflight.py orient --repo C:/Users/LEET/Syndeom --expect-url
https://github.com/Quigles1337/Syndeom --allow-dirty PROGRAM/HANDOFF.md --receipt …`, exit 0,
SNAPSHOT-ESTABLISHED):
- identity `C:/Users/LEET/Syndeom`, remote `https://github.com/Quigles1337/Syndeom.git`;
- `git fetch --prune origin` exit 0, 2026-10-26T21:50:02.64Z–21:50:03.41Z;
- default branch `main` (from `git ls-remote --symref`);
- fetched `origin/main` = `0afbbd17cdc5e9e93691612a95bd5950e3243e10` (tree
  `27106131f522357184171e19d0e0e51ba2f30d54`); HEAD = `main` at the same SHA and tree;
- ahead 0 / behind 0, no local-only or remote-only commits;
- working tree: `PROGRAM/HANDOFF.md` modified (`allowed_present`), nothing unrelated;
- classification synchronized; examined checkout `C:/Users/LEET/Syndeom`;
- receipt SHA-256 `5f62151301ef8e27ccf114be6b5d40aaef06dff5077610b94316b8145f8ea393`, filed as
  `preflight-orient.json` under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\`.
Step 5's confirmation: the constitution (`110f5360…479c`, 38,958 bytes), ADR-0005 and ADR-0008
on disk equal HEAD's (`git diff --quiet HEAD` exit 0). Then, in §4.1 order: STATE, HANDOFF
Session 27, CLAIMS statuses (VERIFIED ×5, CONDITIONAL 0006, VERIFIED 0007 existence only), FLAWS
`0.1.0` (14 entries), ADR-0008 in full and ADR-0009's header. Two disagreements:
- STATE did not yet record PR 13's merge;
- STATE's ADR-numbers line still called ADR-0009 the next free number, but
  `PROGRAM/DECISIONS/` holds ADR-0000..0009.
No separate ORIENT gate: the snapshot's tree `27106131…` is the tree Session 27's staged-tree
gate certified (2026-10-26T21:09:57Z–21:19:09Z, GREEN; `gate-staged.json`, raw log
`d6fd8942…79d1` re-hashed and matching). Orientation paused once (a harness interruption); the
executor reported to Al and resumed on Al's direction to finish as a recorded non-exam session.
PLAN NOTE: the inherited NEXT (the Gate S1 cold-start examination) survived ORIENT but cannot be
carried out here, because the Git snapshot excludes this session. One thing, at Al's direction:
record the launch-directory finding (STATE and this block) so that the next launch can be cold,
with NEXT naming the launch directory. The two §4.1 reconciliations go with it (PR 13's merge;
the next free ADR number).
ADVANCED:
- **A. The ADR-0009 opening, executed as written.**
  - The constitution and ADR-0005 were read in full, the memory read and hashed, and the
    harness-attached context recorded.
  - The disclosure was composed from prose parts and the memory files' own bytes: the reproduced
    index lines are `cmp`-identical to lines 85/86/88/89/106/107, and the memory file re-hashes to
    `163c7b06…2787` inside the composed text. It was checked for CR and BOM.
  - It was appended by `cat >>` without reading HANDOFF; the clock read 2026-10-26T21:49:42Z
    immediately after the append. The tail then compared byte-equal to the composed file
    (SHA-256 `e210faa6…0334`, 17,730 bytes, including a leading newline and the provisional
    header).
  - The header was completed after step 5 (this block's first lines). The disclosure body, from
    its `###` heading to "**Memory writes:** none this session.", is unchanged: 236 lines,
    17,619 bytes, SHA-256 `481ac774f6553c285a9c6a62b8a4792805a901913b3540da023b982a36b6e2ac`,
    re-checked before the commit.
  - HANDOFF's committed prefix (392,025 bytes) is unchanged. A copy of the appended bytes is
    `disclosure-appended.md` under the external root.
- **B. The launch-directory finding recorded** (above; in STATE: the Updated bullet, the Gate
  line, Memory policy, Blockers, the Gate S1 table's cold-start row and NEXT).
- **C. PR 13's merge recorded in STATE**, every value read from git and `gh`:
  - merge commit `0afbbd17cdc5e9e93691612a95bd5950e3243e10`, with parents
    `6e198e066c128976d55d0194f278cc645baf9249` (the PR's base) and
    `fbadee37b5148e25cd803e6df01d5346dd9e05b9` (its head);
  - `gh pr view 13`: mergedAt 2026-10-26T21:42:45Z, mergedBy `Quigles1337`; committer stamp
    2026-10-26T17:42:45-04:00;
  - `0afbbd1^{tree}` = `fbadee3^{tree}` = `27106131…`, and `git diff --quiet` exits 0;
  - CI pull_request run 40403469499 and push run 40406649416 both succeeded; no open pull
    requests.
  STATE also gains: Session 27's Updated bullet kept as "Previous update", Current build
  verification, and the ADR-numbers line (ADR-0009 written and accepted; ADR-0010 the next free
  number).
- **D. A memory hazard line proposed for Al's approval** (ESCALATIONS); nothing written to memory.
- **E. Gate.** The commit carrying this block gets a full gate on its exact staged tree, detached
  with a log, recorded in its pull request and under the external root.
BELIEF DELTAS:
- The launch directory, not the prompt, decides what the harness puts in context before the
  first action. Inside a git work tree that includes a Git status and history snapshot, and the
  memory store is keyed by the directory. Evidence: this session's disclosure; Sessions 6, 7, 25
  and 27.
- SD15's eligibility has three independent channels, and each has now failed once: disclosure
  order (Session 25), conversation context (Session 27) and harness launch context (this
  session). A procedurally complete disclosure settles neither of the other two.
CONFUSIONS:
- At launch the executor did not expect harness context beyond memory. It noticed the snapshot
  while composing the disclosure and recorded it there, before any other action. Cost: none to
  the repository; the examination is still owed. Doc fix: STATE's Memory policy and NEXT.
- `grep -c $'\r'` inside a double-quoted `"$(…)"` in the Bash tool counted every line (239 of
  239), not CR bytes; `tr -cd '\r' < file | wc -c` gave 0. Cost: one extra check before the
  append. Doc fix: this line (for CR checks, count bytes with `tr`, not `grep`).
HONESTY CHECK: no CLAIM, FLAW or SOURCE entry changed status; nothing labeled, promoted, measured
or reproduced. This session is recorded as a non-exam §6 launch, neither as a PASS nor as an
ineligible attempt. Downgrades: none.
LEAKAGE CHECK: checked, not assumed — `contact/` not opened; no AI-generated proof, detector,
generator or threshold touched. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results; receipts record exact commands, trees and
digests. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK: no statement about any real manuscript (SD3); no understanding claims (SD17); no
detectors; no referee contact (SD13); no S2 work; no examination conducted or implied; the held
branch untouched.
NEXT-SINGLE-ACTION: the Gate S1 cold-start examination in a separate fresh session, exactly as
STATE.md states it:
- a new conversation that holds no transcript of earlier sessions;
- **launched from `C:\Users\LEET`, outside the checkout, as Sessions 6, 7, 25 and 27 were**;
- with the constitution's §6 Session-N resume prompt (ADR-0009 wording), verbatim and alone;
- opened by that prompt's disclosure, which records what the harness attached before the first
  action, and then ADR-0008 D1.
ESCALATIONS:
- This session's branch `claude/s1-session-28-launch-directory` and its pull request await Al's
  review and merge; they are left unmerged.
- **Proposed executor-memory hazard line, awaiting Al's approval — nothing written to memory**
  (for the hazard list of the SYNDEOM memory file; harness behaviour only, no program state):
  "- Launched inside a git work tree (e.g. `C:\Users\LEET\Syndeom`), Claude Code attaches a
  gitStatus snapshot (branch, `git status`, the last five commit subjects) to the conversation
  before the first tool call, and loads memory from that directory's own project key
  (`C--Users-LEET-Syndeom`, empty) instead of the `C--Users-LEET` index; `C:\Users\LEET` is not a
  git work tree."
- The merged branches `claude/s1-adr-0009-disclosure-first`, `claude/s1-session-25-exam-attempt`
  and `claude/s1-session-27-sec6-launch` remain on the remote; deleting them is Al's call.
- Unchanged: the held branch's admission review after Gate S1 closes (decision 1); the Codex
  checkout's unpushed branch (Session 23, E; not re-inspected); human-referee participants (SD13).
STOP-REASON: natural boundary, at Al's direction. The only open Gate S1 criterion is an
examination this session cannot perform; the record is delivered on a reviewable PR, and the
session stops for Al's review. Repo green: Session 27's final gate on this session's base tree,
and the staged-tree gate before the commit (recorded in the PR).

SESSION: 2026-10-26 UTC (Session 29 — the Gate S1 cold-start examination: a fresh conversation
launched from `C:\Users\LEET`, outside the checkout, with the §6 Session-N resume prompt in its
ADR-0009 wording, verbatim and alone; disclosure first; examined fetched `origin/main`
`e029c80e1be149bef8fe6ceb76eed59c235ed8b4`, tree `67023777965d5747c5e3e5fe3f655825f658c046`;
result PASS; branch `claude/s1-session-29-cold-start-exam`; header completed after ADR-0008
step 5 — it was written provisionally with the disclosure, which is unchanged; the session-end
SHA is the commit carrying this block, recorded in its pull request)

**SD15 MEMORY DISCLOSURE — begin** (first entry of this block; SD15 as amended by ADR-0005 and
ADR-0009; ADR-0005 D1–D3)

- **Launch.** This session was launched with the §6 Session-N resume prompt, verbatim (compared
  word for word against `PROGRAM/SYNDEOM-PROGRAM.md` lines 481–493 as read this session), as the
  first message of a fresh conversation: no earlier turns, no other launching text. The harness
  working directory was `C:\Users\LEET`, which is not a git work tree, so no gitStatus snapshot
  was attached; the harness loaded the memory index of the `C--Users-LEET` project key.
- **Everything done before this entry**, in order: (1) read `PROGRAM/SYNDEOM-PROGRAM.md` in full
  (500 lines); (2) read `PROGRAM/DECISIONS/ADR-0005.md` in full (79 lines); (3) in the memory
  directory `C:\Users\LEET\.claude\projects\C--Users-LEET\memory\` only: `sha256sum` and `wc -l`
  of `MEMORY.md` and `project_syndeom.md`, the D3 search below with a canary, `cat` of
  `project_syndeom.md`, `file` on both, counts of fence characters, CR bytes and blank lines, and
  `date` / `date -u` (2026-10-26 19:13 EDT = 23:13 UTC). No other repository file was read; no
  directory listing of the checkout, no git command, no build.
- **Memory index** `MEMORY.md`: 110 lines, LF, SHA-256
  `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`.
- **D3 check.** Case-insensitive search of the whole index with
  `awk '{l=tolower($0); if (l ~ /syndeom|kober|friedman|fr13dman/) print NR}'` (awk, not grep: on
  this machine Git Bash grep can abort on `-i` and look like zero matches): exit 0, matched lines
  **85, 86, 88, 89, 106, 107** and no others. Canary: the same construction searching for
  `darq records (mvp)` matched line 22, so the search was live. Every matched line is reproduced
  under D1 (a).
- **D1 (a) — index lines naming this program or a sibling**, lines 85, 86, 88, 89, 106, 107 in
  that order, exactly as loaded (extracted from the file by line number, not transcribed):

````text
## KOBER (Linear A falsification infrastructure)
- [Repo pointer + machine hazards](project_kober.md) — `C:\Users\LEET\KOBER`, remote `Quigles1337/KOBER` (**public**); the constitution `PROGRAM/KOBER-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (memory-policy ADR).
## FR13DMAN (FRIEDMAN — designedness-detection instrument)
- [Repo pointer + machine hazards](project_friedman.md) — `C:\Users\LEET\FR13DMAN` / `Quigles1337/FR13DMAN` (**public**, leetspeak name); sibling of KOBER; the constitution `PROGRAM/FRIEDMAN-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (ADR-0003).
## SYNDEOM (AI-proof calibration — third sibling of KOBER/FRIEDMAN)
- [Repo pointer + machine hazards](project_syndeom.md) — `C:\Users\LEET\Syndeom` / `Quigles1337/Syndeom` (**public**); the constitution `PROGRAM/SYNDEOM-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (SD15 / FR13DMAN ADR-0003).
````

- **D2 — withheld by count.** 104 index lines are withheld from this public record: lines 1–84,
  87, 90–105 and 108–110 (33 of them blank, 71 non-blank). They concern unrelated programs and
  the machine; by the D3 check none of them names SYNDEOM, KOBER, FRIEDMAN or FR13DMAN.
- **D1 (b) — this program's memory file in full**, `project_syndeom.md`, 135 lines, LF, ends in a
  newline, SHA-256 `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, exactly
  as read (its bytes are concatenated here by the shell, not transcribed):

````text
---
name: project-syndeom
description: "SYNDEOM program - repo C:\\Users\\LEET\\Syndeom (remote Quigles1337/Syndeom, public); constitution and ALL program state live IN the repo, never here"
metadata:
  node_type: memory
  type: project
  originSessionId: 07cebf44-c690-47a3-8c97-d4b909c4e75b
  modified: 2026-10-26T19:18:39.389Z
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

**Never work in the sibling KOBER or FR13DMAN trees** — identical `PROGRAM/` schema, and their
sessions may be live concurrently.

Machine and harness hazards:
- System-level git config has `core.autocrlf=true`; the repo pins LF via `.gitattributes`. Verify
  with `git ls-files --eol` and a byte-level CR/BOM check, never by eye.
- The Python console is cp1252: set `PYTHONIOENCODING=utf-8` before any script that prints
  non-ASCII (`§`, `→`, `—`), or it dies with a UnicodeEncodeError after doing its work.
- Cloning an empty GitHub repo yields the server's default branch; `git init` on this
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
- A ferried file pasted into the chat never matches its stated digest: the harness rendering of a
  Markdown file flattens tables to spaced text, drops `=` lines out of display math, strips heading
  and bullet markers, and prefixes/suffixes stray quotes. Transcribe and hash the paste once to
  show the mismatch, then ask for a path on disk holding the exact bytes. Never "repair" a paste
  into the ferried artifact.
- `www.apache.org` / `apache.org` (Fastly, 151.101.x) refused TCP connections from both curl and
  the fetch tool for a whole session while DNS resolved; the SPDX License List
  (`spdx.org/licenses/<id>.txt`, no trailing newline) and the GitHub Licenses API
  (`gh api licenses/<key>` — no leading slash, or Git Bash rewrites it as a path) both serve
  licence texts and cross-check each other after whitespace normalization.
- Project Euclid sits behind an Incapsula bot challenge (HTTP 200 with a ~1 KB challenge page to
  curl, empty content to the fetch tool). The Internet Archive's raw-capture route
  `web.archive.org/web/<timestamp>id_/<url>` serves the archived publisher PDF; find the timestamp
  with `archive.org/wayback/available?url=…`. Old JSTOR scans have no text layer — `pdftotext`
  returns only the JSTOR footer — so render pages with `pdftoppm -r 110 -png` and read the images.
- `scripts/check.sh` can take 10+ minutes after a reboot even with `lean/.lake` warm: the axiom
  audit (`lake env lean Audit.lean`) loads all of Mathlib's oleans from a cold disk cache
  (lean.exe grows past 1.4 GB). Run the gate in the background with a log; the ~30 s figure holds
  only with a warm OS file cache.
- arXiv API: `http://export.arxiv.org/api/query` answers 301 with an empty body (looks dead to a
  plain curl); use `https://export.arxiv.org/api/query`. The `co:` field prefix searches the
  revision comment (`search_query=co:erratum+AND+cat:math.*`); results are Atom XML.
- Crossref REST: `https://api.crossref.org/works?filter=update-type:erratum,type:journal-article`
  works (also `correction`, `corrigendum`, `retraction`); each hit carries `update-to`
  `[{type, DOI}]`; always pass `mailto=` for the polite pool. No subject filter — use zbMATH
  (`https://api.zbmath.org/v1/document/_search?search_string=ti:erratum`, JSON with
  `status.nr_total_results` and DOI `links`) to confine to mathematics.
- PowerShell's `bash` on this machine resolves to the Windows WSL stub (`C:\WINDOWS\system32\bash.exe`),
  which exits immediately; a launcher must name `C:\Program Files\Git\bin\bash.exe` explicitly.
- Run the gate detached with a log under the session scratchpad — `Start-Process` of Git Bash on a
  wrapper script that echoes start/end timestamps and the exit code, with `-RedirectStandardOutput`
  — so a harness kill (it reaps background shells when system memory is low) does not lose the
  result; a background `until grep -q 'GATE end' <log>; do sleep 10; done` is the notifier.
- Never run two gates at once: each axiom audit loads Mathlib's oleans (lean.exe past 1.4 GB), and
  two together starve the machine. Wait for `=== GATE end` with a foreground
  `until grep -q '=== GATE end' <log>; do sleep 10; done` loop (this harness tolerates it), and
  launch the pre-commit gate only after the ORIENT gate has ended.
- Python launched from Git Bash does not resolve `/c/Users/...` paths (FileNotFoundError on open);
  write `C:/Users/...` inside Python strings, and keep `/c/...` for the shell side only.
- The Route A finder caches its arXiv API pages only under the scratchpad of the session that
  fetched them (`<Temp>/claude/C--Users-LEET/<session-id>/scratchpad/route-a/pages/`). Copy
  `pages/*.xml` and `finder-state.json` from an earlier session's scratchpad (find it with
  `ls -d .../C--Users-LEET/*/scratchpad/route-a`) into the new one before running the finder:
  0 requests instead of ~30 min of fetching, and the digest check still proves identity.
- In `grep -E` / `awk` on this machine a pattern written `'^\\b[A-Z]'` matches a word boundary,
  not a literal backslash-b; write `'^[\\]b[A-Z]'` to match TeX macros such as `\bD`, `\bL`.
  A `\\section` pattern that found nothing in one call and everything in another was the same
  trap; prefer a bracket expression for a literal backslash. A quoted heredoc (`<<'EOF'`) fed
  to the Bash tool also strips one backslash level from Python source, leaving backspace
  characters where `\b` was meant: write backslash-bearing text with the Write/Edit tools.
- A working copy's `origin/<branch>` ref is only as fresh as its last fetch: `git status` can
  report a checkout as in sync while the remote is many commits ahead. Never treat a checkout's
  tracking ref as the remote's state. Never force-push; a diverged local commit goes to a branch.
- **A second checkout of this repository exists on this machine** at
  `C:\Users\LEET\Documents\ChatGPT\Syndeom` (used by another executor, Codex; it can hold its
  own unpushed local branches). Never work in it, reset it, or treat its refs as the remote's;
  `C:\Users\LEET\Syndeom` is this executor's checkout, and `origin` is the only shared truth.
  Durable external evidence roots for this program live under `C:\Users\LEET\Codex\Temp\`
  (one new directory per event, never reused); the session scratchpad is not durable.
- Use `git -C <path>` for every git command in the Bash tool: its working directory drifts
  between calls (a `cd` persists), and a bare `git` then fails with "not a git repository".
- Launched inside a git work tree (e.g. `C:\Users\LEET\Syndeom`), Claude Code attaches a
  gitStatus snapshot (branch, `git status`, the last five commit subjects) to the conversation
  before the first tool call, and loads memory from that directory's own project key
  (`C--Users-LEET-Syndeom`, empty) instead of the `C--Users-LEET` index; `C:\Users\LEET` is not a
  git work tree.
````

- **D1 (c) — other recalled memory.** None. The harness loaded only the index; the only memory
  file read is `project_syndeom.md`. The sibling files `project_kober.md` and
  `project_friedman.md` were neither loaded nor read.
- **Reading against SD15's enumeration** (phase, gate status, CLAIM or FLAW statuses, file
  contents, commit SHAs as state markers, next actions, escalations): the reproduced index lines
  hold repo locations, remotes, visibility and the rule to resume from the repo. The memory file
  holds the repo location and environment/tooling hazards; no phase, gate, claim or flaw status,
  commit SHA, next action or escalation. Items a reviewer may weigh as near the line, named so
  they are not hidden: it names program tooling and paths as hazards — the gate script and its
  axiom-audit step (`scripts/check.sh`, `lake env lean Audit.lean`), the cache layout of the
  "Route A finder" (`route-a/pages/*.xml`, `finder-state.json`), the size of a cold Lean setup
  ("nine clones", "~8,900 cache files"), a second checkout used by another executor, and the
  location of durable evidence roots (`C:\Users\LEET\Codex\Temp\`). Its frontmatter records
  `modified: 2026-10-26T19:18:39.389Z` (UTC), i.e. a write before this session began. This
  session reports these facts and does not rule on them; a ruling on whether any of them is
  program state is Al's.
- **Exam status.** Whether this session attempts a gate's cold-start test is decided after
  ORIENT and recorded below. No memory is written by this session in either case.

**SD15 MEMORY DISCLOSURE — end**

MEMORY JUDGMENT (after orientation): no reproduced memory holds program state in SD15's
enumeration (phase, gate status, CLAIM or FLAW statuses, file contents, commit SHAs as state
markers, next actions, escalations).
- The index (`cd943842…fc69`) is byte-identical to the one Sessions 25, 27 and 28 disclosed.
- The SYNDEOM memory file (`ce2ce6de…2cc0`, 135 lines) differs from the `163c7b06…2787` those
  sessions disclosed only by its last five lines, appended. Evidence: Session 28's reproduced
  copy, extracted from HANDOFF by fence, re-hashes to `163c7b06…2787`, and `diff` against today's
  file shows only `130a131,135`. Those lines are the launch-directory hazard Session 28 proposed
  under ESCALATIONS as "awaiting Al's approval — nothing written to memory".
- The repository does not record that write; Session 28's external root does. Its
  `memory-change.txt` reads "hazard line appended on Al's approval of 2026-10-26, after PR 14
  merged", with before/after digests equal to the two above and the index unchanged. The file's
  mtime, 2026-10-26 18:45:43 -0400 (22:45:43Z), falls after PR 14's merge (22:35:49Z) and before
  this session's disclosure (23:14:39Z). Its frontmatter `modified:` still reads 19:18:39Z: stale
  metadata, not a state marker.
- The new lines describe harness behaviour (what the harness attaches inside a git work tree and
  which memory key it loads). They state none of SD15's items. They do tell a reader that the
  launch directory matters, which STATE's NEXT also says, so they are listed for Al's audit
  (ESCALATIONS) beside the items the disclosure names: the gate's axiom audit, the Route A page
  cache, the size of a cold Lean setup, the second checkout and the evidence-root location.
HARNESS CONTEXT (completing the disclosure's Launch bullet, which is unchanged): before the first
tool call the harness attached, besides the memory index:
- an environment block giving the working directory `C:\Users\LEET` and "Is a git repository:
  false";
- the account e-mail address (withheld; not program-related);
- the tool, skill and MCP-server catalogue (no program content).
There was no gitStatus snapshot and no program CLAUDE.md. The disclosure entry named the index and
the absence of a Git snapshot, but not the e-mail address or the catalogue; they are recorded here,
for Al's audit.
THE APPEND (for the audit): the command was `test -f <HANDOFF path> && cat <header> <disclosure>
>> <HANDOFF path>`, an existence test of the file the prompt names; it does not read the file's
contents or list a directory.
- Appended bytes: 16,486 (provisional header 140, disclosure 16,346), SHA-256
  `246a53501c7acebd018fcf5f4efc6198752a63e8a5291ac755d59f0cdf7349e1`, at 2026-10-26T23:14:39Z
  (the file's mtime; nothing else wrote to it before the header's completion).
- The disclosure itself runs from its "begin" line to its "end" line: 200 lines, 16,346 bytes,
  SHA-256 `9126eb15356c0b91e8765493c9b103bea83265d63958cdf082f742e883b5387f`, unchanged since.
- After the append, the working file equalled HEAD's 421,496-byte HANDOFF followed by exactly
  those bytes (`cmp` exit 0). Inside the composed text the memory file re-hashes to
  `ce2ce6de…2cc0`, and the six index lines hash equal to the source lines (`0418e621…534c`).
- The header line was later replaced by this block's first line; nothing else in the block before
  this entry changed. A copy of the composed bytes is `disclosure-composed.md` under the external
  root.
PHASE / GATE: S1 — GROUND-TRUTH CORPUS → **Gate S1 CLOSED by this examination (PASS)**; phase S2 —
GENERATOR opens. The closure is carried by this session's pull request and takes effect on `main`
when Al merges it.
ORIENT (ADR-0008 D1 steps 1–5, run after the disclosure: `python -I -X utf8
C:/Users/LEET/Syndeom/scripts/orient_preflight.py orient --repo C:/Users/LEET/Syndeom --expect-url
https://github.com/Quigles1337/Syndeom --allow-dirty PROGRAM/HANDOFF.md --receipt …`, exit 0,
SNAPSHOT-ESTABLISHED):
- identity `C:/Users/LEET/Syndeom`, remote `https://github.com/Quigles1337/Syndeom.git`;
- `git fetch --prune origin` exit 0, 2026-10-26T23:14:53.38Z–23:14:54.16Z;
- default branch `main` (from `git ls-remote --symref`);
- fetched `origin/main` = `e029c80e1be149bef8fe6ceb76eed59c235ed8b4` (tree
  `67023777965d5747c5e3e5fe3f655825f658c046`); HEAD = `main` at the same SHA and tree;
- ahead 0 / behind 0, no local-only or remote-only commits;
- working tree: `PROGRAM/HANDOFF.md` modified (`allowed_present`), nothing unrelated;
- classification synchronized; examined checkout `C:/Users/LEET/Syndeom`;
- receipt SHA-256 `6fa397ad2e29bebd3d2d25237626e2beb7ea9933779ef0d51a6eeb055ecd8417`, filed as
  `preflight-orient.json` under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-s1-cold-start-exam-20261026\`.
Step 5's confirmation: the constitution (`110f5360…479c`, 38,958 bytes, the amended digest
ADR-0008's amendment and STATE record), ADR-0005 (`873f0751…7595`) and ADR-0008
(`ccdfa9d7…e9ff`) on disk equal HEAD's (`git diff --quiet HEAD -- …` exit 0). Then, in §4.1 order:
STATE in full; HANDOFF Session 28, and Session 6's examination record as the precedent for an
examination's shape; CLAIMS statuses; FLAWS version; ADR-0008 in full; ADR-0003 D8–D9;
`analysis/s1-post-merge-readiness/READINESS.md`.
THE EXAM — program state re-derived in one pass from the repository, with git, `gh` and builds
as evidence (what was read → what it establishes), then compared with STATE:
- **Phase and gate.** Phase S1, Gate S1 OPEN. Every criterion but the cold-start test is
  evidenced, and that test is the next action (STATE's Gate S1 table and NEXT; READINESS). Gate S0
  CLOSED 2026-10-13 (Session 6's PASS, confirmed by Al).
- **S1.1** (ADR-0003; `groundtruth/HARVEST-LOG.md`).
  - The log holds 27 candidate rows, in ascending arXiv id. Its verdict column, tallied from the
    file: 21 admitted (14 single-item, 5 two-item, 1 three-item, 1 four-item, so 31 items),
    4 rejected and 2 found-but-unclassifiable. The last row is candidate 27, `1403.4485`.
  - ADR-0003 D8 (Al's ruling (iii)): Gate S1 may close on an unselected prefix, with the log
    recording the last candidate.
  - Saved resume point: candidate 28 / `1403.6317`.
- **S1.2:** one positive pair (ADR-0007; owner merge PR 4), with READINESS's limits (zero proof
  steps; no PROVEN promotion).
- **S1.3:** the Rust effective-inventory reproduction, run on a fresh clone of `origin` at
  `e029c80` (tree `67023777…`, clean).
  - Command: `cargo run --locked -q --manifest-path engine/Cargo.toml -p syndeom-ingest --example
    s1_effective_inventory -- <clone> <new directory>`; exit 0, 2026-10-26T23:25:45Z–23:25:50Z.
  - Output: `PASS: 35 archived representations / 32 effective logical items; historical bytes
    preserved; s1_ingest-v0.1.0-63e23a952084 -> s1_ingest-v0.1.0-15ab865466d6`.
  - Counts:
    - representations: 33 raw + 2 canonical;
    - 46 flaw instances, in 5 observed classes, with 9 zero-coverage classes;
    - statement status 5 true / 17 false / 10 undetermined;
    - proof status 1 valid / 31 invalid;
    - unflagged subset 18; 30 logical items without a canonical representation.
  - Its `inventory.json` has SHA-256
    `df3a2c7f9f6de13faec4e7634c29275b00758ffbfdb9aacdf73a2aae0364a7f5`. Compared with the
    retained `analysis/s1-first-parent-integration/report/inventory.json` (`b626d1b5…1b803`), a
    JSON walk and `diff` find exactly one difference, `preservation.exact_files_checked`
    370 → 369. That is the difference STATE says to expect at HEAD after ADR-0009 D5, and the
    digest STATE records.
- **S1.4:** `analysis/s1-4/report/report.md` states "Prefix through arXiv id 1403.4485."; it has a
  zero-coverage and interim-power section, and declares every real-item cell expected
  underpowered (ADR-0003 D9). Its 34 representations describe its frozen snapshot (READINESS says
  so).
- **Licences:** recorded per source; nothing touched this session.
- **Registers:**
  - CLAIM-0001..0005 VERIFIED; 0006 CONDITIONAL (narrowed, unfalsified as of 2026-10-13); 0007
    VERIFIED (existence only);
  - SOURCES S-0001..S-0072;
  - FLAWS `0.1.0`, with 14 `## FLAW-` headings;
  - the gate's register check agrees (Q 17, SD 17, FLAW 14, CLAIM 7, S- 72).
- **Pins,** read from the files:
  - `lean/lean-toolchain`: `leanprover/lean4:v4.34.1`;
  - `lean/lakefile.toml`: mathlib rev `d13f23b723b8a846827a245b89c10fc7d3f11612`;
  - `rust-toolchain.toml`: channel `1.91.0`, clippy + rustfmt;
  - `engine/Cargo.toml`: resolver 3, members `manifest`, `ingest`, `derive`, `pair`;
  - `engine/Cargo.lock`: 34 packages.
- **Green.** The ORIENT gate (`bash scripts/check.sh`) ran on the snapshot plus the declared
  append, detached with a log, 2026-10-26T23:16:58Z–23:25:02Z: exit 0, `GATE: GREEN`.
  - Lean 8,926 jobs; golden axioms `[propext]`.
  - fmt and clippy clean; 82 Rust tests (8 + 15 + 5 + 24 + 16 + 8 + 6).
  - 462 tracked files CR/BOM/trailing-LF clean; registers as above; `RESULT: ALL PASS`.
  - Receipt `gate-orient.json` and raw log `gate-orient.raw.log` (6,875 bytes, SHA-256
    `04c556afd76e90e3aacf940de3376087e9394c2832ab15118f581f37fb620943`) are under the external
    root.
- **Main and the remote** (git and `gh`).
  - PR 14 (Session 28's record) merged at 2026-10-26T22:35:49Z, merged by `Quigles1337`, as
    `e029c80…`. Its parents are `0afbbd1…` (the PR's base) and
    `fea670843d85212148be5995d244a97c83829d4e` (its head).
  - `e029c80^{tree}` = `fea6708^{tree}` = `67023777…` (`git diff --quiet` exit 0). That is the
    tree Session 28's staged gate certified (`gate-staged.json`, GREEN 22:31:35Z–22:33:27Z).
  - CI pull_request run 40413519657 and push run 40413650650 both succeeded. No open pull requests.
  - Remote heads are only `main` and the held `claude/s1-harvest-candidates-28-30` at
    `c3a3d369689d56bab3678f519904087b313daf4e`.
- **Escalations and dispositions,** as STATE lists them:
  - the held branch (decision 1; its review comes after Gate S1 closes);
  - the older questions, retained (decision 3);
  - the S1.3 tool's non-blocking note (partly done);
  - human-referee participants, Al's (SD13).
- **Next action before this session:** this examination, launched as STATE's NEXT describes.
Comparison with STATE: agreement on every item above. Three reconciliations, none of them
unreconstructable:
1. STATE does not record PR 14's merge (it was written before the merge); recorded now.
2. Two later events are recorded only in Session 28's external root, not in the repository:
   - the memory hazard line (MEMORY JUDGMENT above);
   - the deletion of four merged branches from `origin` on Al's instruction
     (`deleted-branches-manifest.txt`, 22:46:06Z): `claude/s1-adr-0009-disclosure-first`
     `61cc974…`, `claude/s1-session-25-exam-attempt` `e1c8150…`,
     `claude/s1-session-27-sec6-launch` `fbadee3…` and `claude/s1-session-28-launch-directory`
     `fea6708…`. Each was verified an ancestor of `e029c80` before deletion, and the held branch
     was kept.

   `git ls-remote --heads origin` agrees. Session 28's escalation that the merged branches
   "remain on the remote" is closed; recorded in STATE now.
3. The frozen packets' own `gate_s1` fields read OPEN. They are historical by design and are not
   edited.
EXAM RESULT: **PASS.**
- Every item was re-derived from the repository alone, in one pass, without guessing.
- The gate is GREEN on the snapshot, and again on the tree this commit lands (staged-tree gate,
  recorded in the pull request).
- The S1.3 reproduction reproduces the retained inventory with exactly the one expected
  difference.
- The three reconciliations concern events after STATE was written, not missing context.
- The session is eligible under SD15 as recorded in the disclosure and above: a fresh
  conversation with no prior turns; launched from `C:\Users\LEET` (not a git work tree; no Git
  snapshot); the §6 Session-N prompt in its ADR-0009 wording, verbatim and alone on visual
  comparison (a paste is not bytes); the disclosure first; no memory written. PASS rather than
  CONDITIONAL PASS rests on the executor's judgments named under ESCALATIONS, so Al can overrule
  them.

Gate S1 is CLOSED at this commit, effective on `main` at the merge. STATE moves to phase S2, with
a Gate S2 table and the S1 table retained as the record.
ADVANCED:
- **A. The ADR-0009 opening, executed as written** (the disclosure and the facts above).
- **B. The Gate S1 cold-start examination: PASS.** Gate S1 closed; phase S2 opened in STATE, with
  a Gate S2 table.
- **C. STATE reconciled:**
  - PR 14's merge, every value read from git and `gh`;
  - the post-merge memory line and branch deletions;
  - the Updated bullet, Phase, Gate and both gate tables;
  - Memory policy, Current build verification, Blockers, Standing escalations and NEXT.
- **D. Gate on the staged tree before the commit,** detached with a log, recorded in the pull
  request and under the external root, not in the tree it certifies.
BELIEF DELTAS:
- The repository was sufficient for S1 as it was for S0, now tested rather than asserted. Every
  S1 number was re-derived from files and builds: the harvest-log tally, the Rust inventory
  reproduction on a fresh clone, and the gate. Evidence: the exam list above.
- The launch-directory finding holds from the other side. Launched from `C:\Users\LEET`, the
  environment block said "Is a git repository: false", no Git snapshot appeared, and the
  `C--Users-LEET` index loaded.
- Between sessions, events can happen that only the external root records (a memory write, branch
  deletions). The repository remains the program's memory. Reconciling it with reality takes
  `git ls-remote` and a memory digest compared with the last disclosure. An examiner who skipped
  those comparisons would not have seen the reconciliations, though the state would still
  reconstruct.
CONFUSIONS:
- The provisional header used a `## SESSION —` heading rather than HANDOFF's `SESSION:` line,
  because HANDOFF's format cannot be read before disclosing. It was completed to the convention
  after step 5. Doc fix: none needed (ADR-0008 allows any provisional header); this note.
- The first extraction of Session 28's reproduced memory looked for a `~~~text` fence, but
  Session 28 used `~~~markdown`. The result was an empty file and one extra command. Doc fix:
  this line.
- One read-only command (a listing under `groundtruth/`) was interrupted before it returned
  output. It changed nothing; the corpus counts were taken from the harvest log and the Rust
  reproduction instead.
HONESTY CHECK:
- No CLAIM, FLAW or SOURCE entry changed status. Nothing was labeled, promoted or measured. The
  S1.3 run reproduces an existing record and makes no new claim.
- CLAIM-0006's CONDITIONAL was re-read: status, narrowed scope and bounding sources are present.
  Nothing to downgrade.
- PASS rather than CONDITIONAL PASS rests on three judgments stated for Al's audit
  (ESCALATIONS).
- Downgrades: none.
LEAKAGE CHECK: checked, not assumed — `contact/` was not opened; no AI-generated proof, detector,
generator or threshold was touched; no design decision was taken. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED results. The gate and the inventory reproduction are
regenerable from the commands, SHAs, trees and digests above. Nothing to quarantine.
MANIFEST CHECK: no artifact was created or changed; the reproduction wrote only outside the
repository. "Proven" was applied to nothing. Nothing synthetic was labeled real.
DRIFT CHECK:
- no statement about any real manuscript (SD3);
- no understanding claims (SD17);
- no detectors, so no detector-shopping;
- no referee contact (SD13);
- no S2 work started; the held branch untouched.
NEXT-SINGLE-ACTION: S2.1 — write `PROGRAM/DECISIONS/ADR-0010.md`, the first generator family's
design (decision only; no generator code in that session), exactly as STATE.md's NEXT states it.
Open with ADR-0008 D1, preceded by the SD15 disclosure if launched with the §6 prompt.
ESCALATIONS:
- **SD15 audit request (this examination).** Al, please confirm or overrule three judgments:
  1. No reproduced memory holds program state, including the launch-directory hazard appended
     after PR 14 and the items the disclosure names.
  2. The `test -f` existence check in the append command is not a read of program state.
  3. Recording the non-memory harness context (account e-mail, catalogue, environment block)
     after the disclosure rather than in it does not affect eligibility.

  If any is overruled, this examination becomes CONDITIONAL PASS. Gate S1's closure is then
  provisional until a cold re-run, and memory is changed only in a non-exam session.
- This session's branch `claude/s1-session-29-cold-start-exam` and its pull request await Al's
  review and merge; they are left unmerged. Gate S1's closure takes effect on `main` at the merge.
- **Held branch, now eligible.** Al's decision 1 deferred the held branch's admission review
  "until after Gate S1 closes". Its order relative to S2's ADR-0010 is Al's call; NEXT defaults
  to the phase order.
- At ADR-0010, the R4 seed family's inclusion and licence line (ADR-0000 Addendum 2 R1) are Al's.
- Unchanged from Session 28: human-referee participants (SD13); the Codex checkout's unpushed
  branch (not inspected).
STOP-REASON: gate reached — Gate S1 closed by the cold-start examination (effective at the merge).
Repo green: the ORIENT gate on the snapshot, and the staged-tree gate before the commit (recorded
in the pull request).
**Memory writes:** none this session (an examination writes none).
AL'S RULINGS (2026-10-26, received in this session's conversation after PR #15 opened; recorded at
Al's direction; transcribed verbatim from Al's message):

~~~text
Rulings on your three judgments: I uphold all three.
1. The memory holds no program state, including the new hazard line, which records harness behavior only.
2. The test -f existence check is not a read of program state: the prompt names the file, and checking that it exists reveals nothing more.
3. Recording the other attached context after the disclosure is fine: none of it is memory, and none of it names this program or a sibling.

Order of work: keep NEXT at ADR-0010 per the phase order, with the held harvest review scheduled after it. I will rule on the Mnemosyne seed family when ADR-0010's draft is in front of me.

Record these rulings verbatim in STATE and in your HANDOFF block within PR #15, rerun the gate on the committed tree, push, and stop. Change nothing else.
~~~

Effect, recorded in STATE:
- The three SD15 judgments are upheld, so this examination's PASS and Gate S1's closure are not
  conditional (Standing escalations, where the rulings are verbatim; the Updated bullet).
- NEXT stays at ADR-0010, with the held harvest review scheduled after it (the held-branch bullet
  and NEXT's ordering note).
- The Mnemosyne seed family, ADR-0010 item (h), is ruled on when the draft is before Al; NEXT
  already flags it for Al.

Nothing else changed. The commit carrying this entry is gated on its committed tree; the result
is recorded under the external root `C:\Users\LEET\Codex\Temp\syndeom-s1-cold-start-exam-20261026\`
(`gate-committed.json`), not in the tree it certifies. **Memory writes:** none.

SESSION: 2026-10-27 UTC (2026-10-26 America/New_York) (Session 30 — an informed working session,
not an examination, continuing Session 29's conversation at Al's direction; it was not launched
with the §6 prompt, so it makes no SD15 disclosure; ADR-0010, the relicense to the PolyForm
Noncommercial License 1.0.0; branch `claude/s2-adr-0010-relicense` from `main`
`c9372b1355a6d687dec6287f908639ac670db659`; the session-end SHA is the commit carrying this
block, recorded in its pull request)
PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (nothing evidenced). Gate S1 CLOSED (Session 29).
ORIENT (ADR-0008 D1 steps 1–5: `python -I -X utf8 scripts/orient_preflight.py orient --repo
C:/Users/LEET/Syndeom --expect-url https://github.com/Quigles1337/Syndeom --sync --receipt …`,
exit 0, SNAPSHOT-ESTABLISHED):
- identity `C:/Users/LEET/Syndeom`, remote `https://github.com/Quigles1337/Syndeom.git`;
- `git fetch --prune origin` exit 0, 2026-10-27T00:30:13.90Z–00:30:14.65Z;
- default branch `main`;
- fetched `origin/main` = `c9372b1355a6d687dec6287f908639ac670db659`, tree
  `4caa81f0c56416a02830d19d4a11eb45c9c4a9f4`;
- classification behind; the clean `main` was fast-forwarded (ff-only) to the fetched tip;
- receipt SHA-256 `14e87ba102f4dd399d4c52bcf916638b1a5138d2f84f5951c2c82a07c5cd266a`
  (`preflight-orient.json` under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\`).

PR 15 merged at 2026-10-27T00:25:39Z, by Al, as `c9372b1`. Its tree `4caa81f0…` is the tree
Session 29 gated GREEN on its committed rulings commit `6b3dbda`. STATE on `main` records Gate S1
CLOSED and phase S2, as Al's step 1 required.
WHAT AL ASKED (Session 30's opening message): orient and confirm PR 15; inventory every licence
statement, whether the constitution names Apache-2.0, and which affected files the preservation
check freezes; give options per conflict; draft ADR-0010; ask for the commercial-licensing
contact; then, after acceptance, apply, gate the committed tree, open a PR and stop for review.
ADVANCED:
- **A. Inventory (changed nothing).**
  - Licence statements: `LICENSE`; README § Licence; the four crate manifests; STATE (the
    Default-branch, Q3 and advisory-note bullets); S-0019; ADR-0000 R1.
  - Absent: SPDX headers, CITATION or package metadata, and any licence in
    `lean/lakefile.toml`.
  - The constitution names no licence for the repository (SD16 is generic).
  - Frozen: every affected file except README and STATE is in the 375-file baseline that
    `preserve()` checks; `LICENSE`, SOURCES.md and the crate manifests are checked exactly.
  - Five conflicts (C1–C5) with options went to Al. The draft is
    `ADR-0010.draft.md` under the external root (SHA-256 `75cf86e0…2c24`).
- **B. Al accepted, with rulings C1(a), C2(a), C3(a), C4(a), C5(a)**, the Required Notice URL
  without `.git`, and the contact adz@alphx.io. All are transcribed verbatim in ADR-0010.
- **C. The C2 precondition (Al: confirm nothing produced embeds the Cargo licence field).**
  - Partly true. No `CARGO_PKG_LICENSE`, and nothing reads the licence value.
  - The pair producer embeds whole-file digests of `engine/pair/Cargo.toml` and
    `engine/manifest/Cargo.toml` (`COMPILED_INPUTS`), and its receipts record them. The S1.3 tool
    embeds `engine/ingest/Cargo.toml` and `engine/manifest/Cargo.toml` in `tool_source_sha256`.
  - Reported to Al before any change. Al ruled C2 option (i), all four files, with three
    conditions (verbatim in ADR-0010).
  - Condition 2 checked before committing: the only gate test that calls `verify_producer`
    (`engine/pair/tests/io_provenance.rs`) runs it against a scratch repository under
    `engine/target/pair-tests/`, never against the checkout.
- **D. Applied:**
  - `LICENSE`: the Required Notice line, one blank line, then the verbatim text. It was fetched
    2026-10-27T00:48:02Z, HTTP 200, 4,563 bytes, SHA-256 `c0ea4a89…fabbe5`, equal to Al's digest,
    and re-hashes to that digest inside `LICENSE` (4,662 bytes; whole-file SHA-256
    `f91dbffc…d24e`).
  - The four `license` lines.
  - The tool's three exceptions (ADR-0010 D6), and `engine/ingest/tests/relicense_exceptions.rs`
    (6 narrowness tests).
  - S-0073 appended (SOURCES.md is now HEAD's bytes plus that entry).
  - ADR-0010; README § Licence (a plain-language summary checked against the licence text, the
    contact, the boundary, earlier Apache grants).
  - STATE: Updated; Default branch; the advisory bullet; the two S1.3/S1.2 facts bullets; Current
    build verification; ADR numbers; the narrowing note; the held-branch bullets; Q3; the Gate S2
    row; NEXT.
- **E. Measured** on a clean clone of a local measurement commit with every change but the
  ADR/README/STATE/HANDOFF text. That commit was folded back before the final commit and never
  pushed.
  - At HEAD: `PASS: 35 / 32`. `inventory.json` (`d7432889…14c5`) differs from the retained one
    only in `preservation`: `exact_files_checked` 363; `append_only_prefixes_checked` gains
    SOURCES.md; new `license_line_only_files_checked`. `tool_source_sha256` changes for
    `effective.rs` and the two embedded crate manifests.
  - At `3cd5558` with that revision's tool: the retained `inventory.json` is byte-identical
    (`b626d1b5…1b803`).
  - Tool source `92a816bc…b30c` → `49e6cd6d…1a67`.
- **F. Gate** on the committed tree, detached with a log, recorded in the pull request and under
  the external root, not in the tree it certifies.
BELIEF DELTAS:
- "Nothing embeds the licence field" and "nothing embeds the file" are different claims. The
  pair producer binds its manifests by whole-file hash, so a one-line metadata change ends HEAD
  replay of the admitted pair. Evidence: `engine/pair/build.rs`, `provenance.rs`, the 18 receipts.
  Consequence: ADR-0010 *Consequences* and STATE's replay bullet.
- The preservation check now has four case-by-case exceptions. Any S2 code commit edits frozen
  pair inputs (`engine/Cargo.toml`, `engine/Cargo.lock`), so the full narrowing is a
  precondition for S2 code, not a nicety. Evidence: `input-files.txt` and the frozen list.
  Consequence: the ordering in NEXT.
CONFUSIONS:
- **Two misstatements by the executor to Al, both corrected in the record:**
  - "26 committed receipts (24 + 2)". A `git grep -l … | head` truncated the listing; the exact
    count is 18 (16 + 2). Al's ruling text repeats 26. ADR-0010 records 18 and says why. Doc
    fix: count with the full listing, never through `head`.
  - "Replay from a checkout of `eb1b96d` still works." The producer checks only its inputs, but
    the documented helper `reproduce.py` was added in `0b30d78`, after `eb1b96d`. Replay with the
    helper runs on `main` `e6a7cae…` through `c9372b1`, which ADR-0010 and STATE now state
    exactly.
- Al's condition 1 expected `inventory.json` not to change. It changes in three `preservation`
  fields, though no count or item changes. ADR-0010 records that rather than restating the
  expectation.
- A STATE sentence that gave the held branch's renumbered sources with the `S-` prefix (0074–0080) tripped the register check (undefined
  references), and S-0073's state line parsed as `INGESTED.`. Both were fixed before the
  commit; the checker caught them. Doc fix: this line.
HONESTY CHECK:
- One SOURCE added (S-0073, INGESTED) with its retrieval evidence. No CLAIM or FLAW status
  changed; no artifact, manifest or frozen packet changed.
- The licence statements in frozen historical files are deliberately left (C4, C5).
- The replay procedure in STATE is assembled from the S1.2 README and the producer's code and
  was not re-run this session; it says so.
- Downgrades: none.
LEAKAGE CHECK: checked, not assumed — `contact/` was not opened; no detector, generator or
threshold was touched; the decision concerns licensing and a reproduction tool. Verdict: no
leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED claims. The reproduction outputs are regenerable from
the commands, SHAs and trees recorded in ADR-0010 and under the external root. Nothing to
quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK:
- no statement about any real manuscript (SD3);
- no understanding claims (SD17);
- no detectors; no referee contact (SD13);
- no S2 code (ADR-0011 not started); the held branch untouched.
NEXT-SINGLE-ACTION: S2.1 — write `PROGRAM/DECISIONS/ADR-0011.md`, the first generator family's
design (decision only), exactly as STATE.md's NEXT states it. Then, before any S2 code: the full
narrowing of the S1.3 tool's byte checks.
ESCALATIONS:
- This session's branch `claude/s2-adr-0010-relicense` and its pull request await Al's review;
  left unmerged. The licence changes publicly at the merge.
- Al rules on R4's Mnemosyne seed family when ADR-0011's draft is in front of him (Al,
  2026-10-26).
- Unchanged: the held branch's admission review, after ADR-0011 (Al), and in practice after the
  narrowing; human-referee participants (SD13).
STOP-REASON: natural boundary — the accepted ADR is applied and delivered on a reviewable pull
request; the session stops for Al's review, as Al directed. Repo green: the committed-tree gate,
recorded in the pull request.
**Memory writes:** none.
SESSION 30 ADDENDUM (2026-10-27 UTC, after PR 16 opened; at Al's direction, before merge):
- **The S1.2 pair replay was run once, exactly as STATE writes it, in a clean clone at
  `c9372b1`: PASS, with no correction needed.** This supersedes the HONESTY CHECK line above that
  says it was not re-run.
- Timings: clone 01:16:53Z–01:16:56Z; `lake exe cache get` 01:16:56Z–01:24:19Z; `lake build`
  01:24:19Z–01:30:05Z; `reproduce.py` 01:30:05Z–01:31:14Z; every step exit 0.
- Receipt: `PASS: generation, actual-byte validation, emitted Lean audit, replay and controls`
  (SHA-256 `37d7037e…cc42`). Five sources were fetched over HTTP and hash-verified; 33 commands
  ran as expected; 18 refusal controls refused.
- Results: the fresh replay is byte-identical across 11 files, and those digests equal the
  original production receipt's. The generated pair metadata and emitted `Proof.lean` are
  byte-identical to the committed admitted pair.
- Recorded in STATE (the S1.2 PAIR REPLAY bullet) and ADR-0010 *Consequences*. Evidence is under
  `C:\Users\LEET\Codex\Temp\syndeom-pair-replay-20261026\`.
- The commit carrying this addendum is gated on its committed tree, recorded in the pull request
  and under `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\`.
- **Memory writes:** none.

SESSION: 2026-10-27 UTC (2026-10-26 America/New_York) (Session 31 — an informed working session,
not an examination, continuing Session 30's conversation at Al's direction; it was not launched
with the §6 prompt, so it makes no SD15 disclosure; the licence audit for the Gate S1
registration deposit and Al's rulings on it; branch `claude/s2-third-party-notices` from `main`
`368bcfb5c05c23536a32bfc15b9537173fb0fef7`; the session-end SHA is the commit carrying this
block, recorded in its pull request)
PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (nothing evidenced). Gate S1 CLOSED (Session 29).
ORIENT:
- **Al's first message (read-only audit):**
  - `git fetch --prune --tags origin`, exit 0; `main` moved `c9372b1` → `368bcfb`.
  - `git ls-remote --symref origin HEAD` named `main`. Fetched `origin/main` =
    `368bcfb5c05c23536a32bfc15b9537173fb0fef7`, tree `d33402ca2bfa0efcee577b2855b0741c07cdbe6e`,
    equal to the remote's live `main`.
  - The checkout was on `claude/s2-adr-0010-relicense` at `0470684` (the same tree), clean. It
    was not changed. The preflight was not run for this read-only step.
- **Al's second message (writes; ADR-0008 D1 steps 1–5):**
  - The clean checkout was switched to `main`. Then `orient_preflight.py orient --repo
    C:/Users/LEET/Syndeom --expect-url https://github.com/Quigles1337/Syndeom.git --sync
    --receipt …` ran: the script at `368bcfb`, from a copy under the external root.
  - Result: exit 0, SNAPSHOT-ESTABLISHED. `git fetch --prune origin` exit 0,
    2026-10-27T02:25:19.50Z–02:25:20.26Z; default branch `main`.
  - Fetched tip `368bcfb…`, tree `d33402ca…`. Classification behind; `main` fast-forwarded (ff-only)
    from `c9372b1`.
  - Receipt SHA-256 `7e0c774d274f962e07e60016ecbd83347a579e8cdad2b6753714b3d6dd965793`
    (`orient-1-sync.json` under the external root
    `C:\Users\LEET\Codex\Temp\syndeom-registration-licence-20261027\`).

PR 16 merged at 2026-10-27T02:09:06Z, by Al, as `368bcfb`. Its first parent is `c9372b1` (PR 15's
merge), and its tree `d33402ca…` equals PR 16's head `0470684`'s tree, which Session 30 gated
GREEN. Nothing has landed after PR 16: there is no other commit and no open pull request. No tags
existed locally or on the remote.
WHAT AL ASKED:
- **The first message**, verbatim in ADR-0010 *Amendments*:
  - confirm PR 16 and record `main`;
  - audit every file not under SYNDEOM's own licence against the SD16 records, changing nothing;
  - only if the audit is clean, tag `gate-s1-registration` and build the zip.
- **Then rulings 1–4**, verbatim in the same place:
  - leave the advisory note and the Leiden PDF out of the deposit;
  - before tagging, add `LICENSES/Apache-2.0.txt` and `THIRD-PARTY-NOTICES.md`;
  - record the rulings, and check the frozen checks first;
  - gate, open a pull request and stop;
  - after the merge, tag and build.
ADVANCED:
- **A. Audit (read-only, 464 files at `368bcfb`). The executor stopped before tagging.**
  - Twelve third-party copies, each hash-matched to its S-entry: seven CC BY 4.0 PDFs
    (S-0010..S-0016); a CC0 PDF (S-0004); a CC BY-NC 4.0 PDF (S-0005); three identical CC BY 4.0
    v70 e-prints (S-0024).
  - **Not redistributable:** `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md`, recorded as
    `PROPRIETARY_ALL_RIGHTS_RESERVED` (R1(b); ADR-0010 D2(b)).
  - Apache-2.0 files (the alias wrapper; Lean's printed Mathlib declarations), with no Apache-2.0
    text in the tree since ADR-0010.
  - 30 ground-truth items whose manifests record their originals as not redistributable. The
    originals are absent from the tree. No licence was recorded for the derived record files as
    such.
- **B. Al's rulings 1–4** (above).
- **C. Frozen checks, read before any file was written.** None is tripped. The list is in ADR-0010
  *Amendments*: `preserve()`'s addition scopes, its README/LICENSE skips and its SOURCES prefix
  rule; the pair and S1.4 closures; the register check's byte and reference rules.
- **D. Applied:**
  - `LICENSES/Apache-2.0.txt`: SPDX text fetched at 02:26:03Z, equal to S-0019's
    `c274f803…366c`. Plus one LF it is `074e6e32…15ff`, `cmp`-identical to `LICENSE` at
    `c9372b1`. The ASF host refused both attempts.
  - `THIRD-PARTY-NOTICES.md`.
  - README § Licence (a pointer to both files).
  - The S-0019 addendum, appended to SOURCES.md. Its prefix was verified byte-identical to HEAD's
    file.
  - ADR-0010 *Amendments*. STATE: Updated; Default branch; Session 31 build verification; Q3's ASF
    note; a deposit escalation.
- **E. Gate** on the exact staged tree, committed unchanged. Detached with a log; recorded in the
  pull request and under the external root, not in the tree it certifies.
BELIEF DELTAS:
- **ADR-0010 D2(a)** placed "the ground-truth artifacts derived from sources" outside the licence.
  Their manifests' `licence` field describes the external original, so the derived record files
  had no recorded licence of their own. Ruling 3 now records them as SYNDEOM's, with the
  quotations remaining their authors'. Evidence: the 30 manifests' `config.licence` and
  `storage: external`. Consequence: ADR-0010 *Amendments*, "D2(a), read with ruling 3".
- **Any new S-entry would shift the held branch's planned renumbering again** (0074–0080 → one
  later). An appended addendum to S-0019 records the retrieval without a number. Evidence: STATE's
  held-branch bullet. Consequence: the addendum form.
CONFUSIONS:
- **The audit's first report missed a third byte-identical file of printed Mathlib declarations,**
  `analysis/s1-2/implementation/evidence/production/logs/emitted-type-axiom-audit.txt`. The
  first pass looked by file type and name. A content search found the file while the notices
  were prepared; it is listed and was reported to Al. Doc fix: look for third-party text by
  content, not by file name.
HONESTY CHECK:
- **Sources and registers:** no S-entry added; the S-0019 addendum leaves its state unchanged. No
  CLAIM or FLAW status changed, and no artifact or manifest changed.
- **Attribution:** taken from the S-entries as recorded. Where an entry names the first and last
  authors only, the notices say so and point to the arXiv page.
- **Mathlib credit:** checked against `Mathlib/Data/Nat/PrimeFin.lean` in the pinned checkout at
  `d13f23b…`.
- Downgrades: none.
LEAKAGE CHECK: checked, not assumed — `contact/` was not opened; no detector, generator or
threshold was touched; the work concerns licences and notices. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED claims. The deposit archive is built after the merge,
from the tag, outside the tree. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK:
- no statement about any real manuscript (SD3);
- no understanding claims (SD17);
- no detectors; no referee contact (SD13);
- no S2 code; the held branch untouched.
NEXT-SINGLE-ACTION: after Al merges this session's pull request, tag the merge commit
`gate-s1-registration` and build the deposit archive exactly as STATE's Standing escalations
state it, then report to Al. STATE's NEXT (S2.1, ADR-0011) is unchanged.
ESCALATIONS:
- This session's branch `claude/s2-third-party-notices` and its pull request await Al's review;
  left unmerged.
- The deposit, pending that merge (Standing escalations).
- Unchanged: R4's seed family at ADR-0011's draft; the held branch's review; human-referee
  participants (SD13).
STOP-REASON: natural boundary — Al's ruling 3 ends at a gated, reviewable pull request; the
session stops for Al's review. Repo green: the gate on the committed tree, recorded in the pull
request.
**Memory writes:** none.

SESSION: 2026-10-27 UTC (2026-10-27 America/New_York) (Session 32 — an informed working session,
not an examination, continuing Session 31's conversation at Al's direction; it was not launched
with the §6 prompt, so it makes no SD15 disclosure; it records the Gate S1 registration deposit
and the tag-and-archive step before it; branch `claude/s2-post-deposit-records` from `main`
`2018d5e79ea23fa200e560d628bec5e91edf0956`; the session-end SHA is the commit carrying this
block, recorded in its pull request)
PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (nothing evidenced). Gate S1 CLOSED (Session 29).
BEFORE THIS SESSION — THE TAG AND THE ARCHIVE (Session 31's NEXT, carried out in the same
conversation after PR 17's merge, with no repository change, as Al instructed; recorded here at
Al's direction):
- **Al's instruction (2026-10-27), verbatim:**

~~~text
PR #17 is merged. On your four checks: all accepted as you did them, including the partial author lists, since CC BY 4.0 §3(a)(2) allows attribution by a link to a page that carries the full information.

Proceed: fetch and confirm main is PR #17's merge with nothing after it. Then create the annotated tag gate-s1-registration on it with the message "SYNDEOM design registration at Gate S1 closure", push the tag, and build the archive as specified, excluding PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md and the Leiden Declaration PDF. Report the archive's path, size in bytes, SHA-256 and MD5, and its file list compared with the tag's tree. Change no repository file and open no PR.
~~~

- **Main:** fetched; `origin/main` was `2018d5e79ea23fa200e560d628bec5e91edf0956`, PR 17's merge
  (2026-10-27T02:48:36Z, by Al), tree `3d7889def475b2b2f30d8acfd3dee31589a06a6e`, the tree
  Session 31 gated GREEN. Its parents were `368bcfb…` and `a6e2038…`. Nothing had landed after
  it, no pull request was open, and no tag existed.
- **The tag:** `gate-s1-registration`, annotated, message "SYNDEOM design registration at Gate S1
  closure", tagger Quigles1337 at 2026-10-27T02:51:14Z. Tag object
  `a42e7c09a275157127d89cc931d35742422c0c7f`, peeling to `2018d5e`. Pushed; `git ls-remote
  --tags origin` shows both. No GitHub release (`gh release list` lists none).
- **The archive:** `git archive --format=zip --prefix=Syndeom-gate-s1-registration/ -o <path>
  gate-s1-registration -- . ':(exclude)PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md'
  ':(exclude)sources/leiden-declaration-2026-06-02-zenodo-20302944.pdf'`, written at 02:51:24Z
  to `C:\Users\LEET\Codex\Temp\syndeom-gate-s1-registration-20261027\Syndeom-gate-s1-registration.zip`:
  13,370,992 bytes, SHA-256 `95f7d1b7d37aeb604b633b902d8f5e13ab46f5cfd5b582d8d64a299f52039cfc`,
  MD5 `f78dab5eafc6e95f595b1f38b537c9f9` (`SHA256SUMS` and `MD5SUMS` beside it).
- **Verification** (`archive-verification.json` beside it, SHA-256 `e1d2d71e…0068`; verdict
  PASS):
  - 570 entries, 464 files and 106 directories, all under the prefix;
  - the tag's tree has 466 files. The zip holds exactly those less the two excluded files, both
    of which are in the tree and absent from the zip; none is missing and none extra;
  - every file is byte-identical to its blob (`git cat-file`) and every executable bit matches;
    the zip's CRCs are sound (`testzip`); the zip comment is the commit id;
  - a second build was byte-identical.
- Reported to Al. Al then made the deposit.
ORIENT (ADR-0008 D1 steps 1–5: `python -I -X utf8 <copy of scripts/orient_preflight.py> orient
--repo C:/Users/LEET/Syndeom --expect-url https://github.com/Quigles1337/Syndeom.git --sync
--receipt …`, exit 0, SNAPSHOT-ESTABLISHED):
- the clean checkout was switched from `claude/s2-third-party-notices` (at `a6e2038`, the same
  tree) to `main` at `368bcfb`; the script ran from a byte-identical copy (SHA-256
  `7f5f0ecd…e5c8`) under the external root;
- identity `C:/Users/LEET/Syndeom`, remote `https://github.com/Quigles1337/Syndeom.git`;
- `git fetch --prune origin` exit 0, 2026-10-27T05:32:49.84Z–05:32:51.15Z; default branch `main`
  (`git ls-remote --symref`);
- fetched `origin/main` `2018d5e…`, tree `3d7889de…`. Classification behind (two commits:
  `a6e2038` and PR 17's merge); `main` fast-forwarded (ff-only); examined checkout
  `C:/Users/LEET/Syndeom`;
- receipt SHA-256 `92d4ba93…e60a` (`orient-1-sync.json` under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-post-deposit-records-20261027\`).

Nothing had landed after PR 17, and no pull request was open. The tag `gate-s1-registration` was
on the remote, peeling to `2018d5e`. The constitution's SHA-256 is `110f5360…479c`, unchanged.
WHAT AL ASKED — the instruction, verbatim except for one visibly marked edit (note after it):

~~~text
SYNDEOM — post-deposit README and records update (owner instruction, 2026-10-27)

Context. The Gate S1 design registration is published on Zenodo. These values were
verified against Zenodo's API; confirm each from Zenodo yourself before writing it
anywhere, and stop and report if any differs:
- Record: https://zenodo.org/records/23682781 (published 2026-10-27)
- Version DOI (this registration): 10.5281/zenodo.23682781
- Concept DOI (all versions): 10.5281/zenodo.23682780
- File: Syndeom-gate-s1-registration.zip, 13,370,992 bytes,
  md5:f78dab5eafc6e95f595b1f38b537c9f9, SHA-256 95f7d1b7d37aeb604b633b902d8f5e13ab46f5cfd5b582d8d64a299f52039cfc
  (read the SHA-256 from your build receipt,
  C:\Users\LEET\Codex\Temp\syndeom-gate-s1-registration-20261027\SHA256SUMS)
- Source snapshot: annotated tag gate-s1-registration -> 2018d5e79ea23fa200e560d628bec5e91edf0956

Do not assume any repository value: default branch, branch naming, commit identity,
session number, gate command, file paths. Read each from the repository, git or the
program records. Begin with the ADR-0008 orientation preflight, as the program requires.

One bounded package, then stop for my review:

1. README.md
   a. Find every passage that presents Gate S1 as open or its cold-start examination
      as pending (the Status section has at least two). Replace them with what
      happened, citing the records rather than restating them: the Session 29
      examination passed, closure is recorded in PROGRAM/STATE.md and the Session 29
      entry in PROGRAM/HANDOFF.md, and it reached main via PR #15. Take the current
      NEXT from STATE rather than writing your own. Where the README says something
      happens "after Gate S1 closes" (for example, admission review of the held
      harvest candidates 28-30), state the current position as STATE records it, and
      do not start or imply any decision I haven't made.
   b. Add a DOI badge near the top using the concept DOI.
   c. Add a short Citation section. Cite the version DOI when referring to the
      registered design; the concept DOI resolves to the latest version. Take the
      citation text and BibTeX from Zenodo's own export of the record, not from
      memory. Include the archive's file name, its MD5 as Zenodo reports it, and its
      SHA-256. In one or two sentences, say that the archive omits the two files listed
      in THIRD-PARTY-NOTICES.md (link it rather than repeating the reasons) and that it
      is a registration archive: the full gate runs on a full clone at the tag.
   d. Change nothing else in the README: no rewrites of other sections, no reformatting.

2. Program records. Update every record that still describes the deposit as pending,
   starting with STATE's "Gate S1 registration deposit" standing item, so the repository
   doesn't contradict itself, and record the identifiers above. Add this session's
   HANDOFF entry, with this message verbatim as the program does for owner instructions.
   Do not add a new ADR or a new source number ([S-]0074 to [S-]0080 stay reserved for the
   held branch), and do not touch the gate-s1-registration tag.

3. Before writing anything, check whether these edits trip any frozen check, and report
   before working around one. Add no new files in this package; if you think one is
   warranted (for example CITATION.cff), propose it in the PR description instead.

4. Run the full gate on the committed tree with a clean working tree. Fetch fresh before
   pushing and confirm the default branch hasn't moved. Push a branch, open a PR, and stop.
   Do not merge, tag or force-push.

Report: commit SHA and tree, the gate's time window and result, the README diff in
brief, the records you changed, any frozen-check findings, and the PR link.
~~~

Note on the marked edit (Al's ruling, option (a), below): in item 2 Al wrote both numbers of the
reserved range with the `S-` prefix. Here the prefix is bracketed, `[S-]`, because the register
check (`scripts/check_registers.py`, which the preservation baseline pins byte for byte) fails on
any `S-`number that SOURCES.md does not define, and neither number is defined. Nothing else is
changed. The exact message, with one final LF, is 3,768 bytes, SHA-256
`ae971f2c990995007f63c38af5f709d0b0ebde094113779af4895815f286021f`. It is unedited in this
session's pull request description and at
`C:\Users\LEET\Codex\Temp\syndeom-post-deposit-records-20261027\owner-instruction-2026-10-27.txt`.

AL'S RULINGS (2026-10-27, on the executor's report of the frozen-check finding and four calls),
verbatim:

~~~text
Rulings on the frozen-check finding and your four calls:

Option (a). Quote my message verbatim in HANDOFF except for the one visibly marked edit to the reserved range, with your note, the exact text's SHA-256 and its byte count. Put the unedited message in the PR description and keep the exact copy under the external root. This message contains no register numbers, so quote it verbatim as it stands.

Call 1: accepted, with one constraint. The top banner should say Gate S1 is closed and S2 is next, without implying any S2 work has begun. The phase table's S1 row becomes Closed and nothing else in the table moves.

Call 2: accepted as proposed, including the HANDOFF record of the tag-and-archive step after PR #17's merge, with my instruction quoted verbatim and the archive's verification results.

Call 3: bring it into this package. The README will point readers to STATE as the controlling record for Gate S1's closure, so STATE should not still say that Session 29's pull request awaits review and merge. Update that bullet to record PR #15's merge, reading the merge commit and time from git and GitHub. Change only current-status statements that are now false, and leave dated history entries as they were written.

Call 4: accepted. Propose CITATION.cff in the PR description; do not add it.

Then build, run the gate, push, open the PR and stop for my review.
~~~

ADVANCED:
- **A. Zenodo, confirmed before anything was written.** The exports and headers are under the
  external root's `zenodo\`, fetched 05:33:05Z–05:33:09Z:
  - `api/records/23682781`: HTTP 200; `status` `published`; `publication_date` 2026-10-27;
    `doi` `10.5281/zenodo.23682781`; `conceptdoi` `10.5281/zenodo.23682780`; licence
    `polyform-noncommercial-1.0.0`; version `gate-s1-registration`;
  - one file, `Syndeom-gate-s1-registration.zip`, 13,370,992 bytes,
    `md5:f78dab5eafc6e95f595b1f38b537c9f9`;
  - the concept record resolves to this record, its only version;
  - the published file, downloaded, is byte-identical (`cmp`) to the built archive, with the
    SHA-256 `95f7d1b7…039cfc` of the build receipt's `SHA256SUMS`;
  - the citation (`text/x-bibliography`, APA) and the BibTeX (`/export/bibtex`) are Zenodo's
    exports, copied into the README unchanged by a script.

  Every value in Al's instruction matched.
- **B. Frozen checks, before any file was written.** One tripped: the register check, by the
  verbatim quotation (above). No other: `README.md` and `PROGRAM/STATE.md` are skipped by
  `preserve()`, `PROGRAM/HANDOFF.md` is append-only there, ADR-0010 is outside the preservation
  baseline, and no file is added. Reported to Al before writing; Al ruled option (a).
- **C. README:**
  - the DOI badge (concept DOI);
  - the banner: Gate S1 closed, S2 next;
  - the Status section's two passages: the closure, cited to STATE, HANDOFF Session 29 and PR 15,
    and NEXT as STATE has it;
  - the held harvest's "after Gate S1 closes" sentence: eligible, scheduled after ADR-0011 by the
    owner's ordering, nothing approved;
  - the phase table's S1 row: Closed;
  - a Citation section.

  Nothing else in the README changed.
- **D. Records:**
  - STATE: Updated; Current build verification; the deposit escalation, now done, with the
    identifiers; the bullet on Session 29's pull request, now PR 15's merge (read from git and
    GitHub);
  - ADR-0010 *Amendments*: a dated entry recording the deposit;
  - HANDOFF: this entry.
- **E. Gate** on the committed tree with a clean working tree; recorded in the pull request and
  under the external root, not in the tree it certifies.
BELIEF DELTAS:
- **The register check reads quoted text too.** It scans every tracked Markdown file, fenced
  blocks included. An owner message that names an undefined `S-`number therefore cannot be
  committed byte-verbatim while the number is undefined. Evidence: the `S-` pattern in
  `scripts/check_registers.py` runs over all `.md` text; the instruction's item 2. Consequence:
  the marked edit above, by Al's ruling.
CONFUSIONS:
- The tag-and-archive step has no session number of its own. It was Session 31's NEXT, carried
  out in the same conversation with no repository change, so it had no HANDOFF entry. This entry
  records it, at Al's direction.
HONESTY CHECK:
- Every Zenodo value was read from Zenodo's API or exports in this session, not copied from the
  instruction. The SHA-256 was read from the build receipt and recomputed on the downloaded file.
- PR 15's merge time, parents and tree were read from GitHub and git.
- No register status, manifest, frozen packet or constitution byte changed; no S-entry was added;
  the tag was not touched. Downgrades: none.
LEAKAGE CHECK: checked, not assumed — `contact/` was not opened; no detector, generator or
threshold was touched; the work concerns the records of a deposit. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED claims. The archive's build was deterministic (a
second build was byte-identical). Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK:
- no statement about any real manuscript (SD3);
- no understanding claims (SD17);
- no detectors; no referee contact (SD13);
- no S2 code; the held branch untouched.
NEXT-SINGLE-ACTION: STATE's NEXT, unchanged — S2.1, `PROGRAM/DECISIONS/ADR-0011.md`, decision
only.
ESCALATIONS:
- This session's branch `claude/s2-post-deposit-records` and its pull request await Al's review;
  left unmerged.
- `CITATION.cff` is proposed in the pull request description, not added (Al's ruling on call 4).
- Unchanged: R4's seed family at ADR-0011's draft; the held branch's review; human-referee
  participants (SD13).
STOP-REASON: natural boundary — Al's instruction ends at a gated, reviewable pull request.
Repo green: the gate on the committed tree, recorded in the pull request.
**Memory writes:** none.

SESSION: 2026-10-27 UTC (2026-10-27 America/New_York) (Session 33 — an informed working session,
not an examination, continuing Session 32's conversation at Al's direction; it was not launched
with the §6 prompt, so it makes no SD15 disclosure; `CITATION.cff` and its refresh at each
deposit; branch `claude/s2-citation-cff` from `main` `99bdf0362e12d7d7c405566396d7a1a7f01c04a1`;
the session-end SHA is the commit carrying this block, recorded in its pull request)
PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (nothing evidenced). Gate S1 CLOSED (Session 29).
ORIENT (ADR-0008 D1 steps 1–5: `python -I -X utf8 <copy of scripts/orient_preflight.py> orient
--repo C:/Users/LEET/Syndeom --expect-url https://github.com/Quigles1337/Syndeom.git --sync
--receipt …`, exit 0, SNAPSHOT-ESTABLISHED):
- the clean checkout was switched from `claude/s2-post-deposit-records` (at `d1e4d75`, the same
  tree) to `main` at `2018d5e`; the script ran from a byte-identical copy (SHA-256
  `7f5f0ecd…e5c8`) under the external root;
- identity `C:/Users/LEET/Syndeom`, remote `https://github.com/Quigles1337/Syndeom.git`;
- `git fetch --prune origin` exit 0, 2026-10-27T06:06:01.54Z–06:06:02.63Z; default branch `main`
  (`git ls-remote --symref`);
- fetched `origin/main` `99bdf0362e12d7d7c405566396d7a1a7f01c04a1`, tree
  `c063cc6589fc4f5d7605690ae7103cbd87993f5c`. Classification behind (two commits: `d1e4d75` and
  PR 18's merge); `main` fast-forwarded (ff-only); examined checkout `C:/Users/LEET/Syndeom`;
- receipt SHA-256 `bdc7bc82…33b6` (`orient-1-sync.json` under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-citation-cff-20261027\`).

PR 18 merged at 2026-10-27T06:05:00Z, by Al, as `99bdf03`. Its parents are `2018d5e` and the PR's
head `d1e4d75`, and its tree equals the head's, which Session 32 gated GREEN. No pull request was
open. The constitution's SHA-256 is `110f5360…479c`, unchanged.
WHAT AL ASKED — the ruling, verbatim:

~~~text
PR #18 merged. CITATION.cff ruling: use the concept DOI, 10.5281/zenodo.23682780, as the main doi, and list the version DOI, 10.5281/zenodo.23682781, under identifiers, described as the Gate S1 design registration. Keep version and date-released matching the latest deposit, and add updating CITATION.cff to the deposit procedure so each future deposit refreshes them. Check the frozen checks before adding the file and report before working around one. One file on its own branch and PR; run the gate, fetch fresh before pushing, and stop for my review.
~~~

THE SCOPE QUESTION, before any write. The executor reported that adding the file trips no frozen
check, and asked how "one file on its own branch and PR" fits with adding the step to the deposit
procedure, which lives in ADR-0010 *Amendments* and STATE's deposit item; a session also appends
HANDOFF. Three options were offered:
- the file plus the step recorded in those records;
- the file alone, with the step stated in a comment inside it;
- the file now and the records in a later pull request.

Al's answer, verbatim: "Add the file and record the step (Recommended)".
ADVANCED:
- **A. Frozen checks, before the file was added.** None is tripped:
  - `preserve()` rejects additions only under `analysis/s1-4/`,
    `analysis/s1-first-parent-status/`, `engine/manifest/examples/`, `engine/manifest/src/`,
    `engine/pair/src/` and `groundtruth/`;
  - the S1.4 tool's `ls-files` covers only `engine/manifest/examples` and `analysis/s1-4/config`;
  - the register check requires a non-`.md` file to have LF line endings, no BOM and a final LF;
  - `.gitattributes` (`* text=auto eol=lf`) makes the file LF text.
- **B. `CITATION.cff`** (1,031 bytes, LF, no BOM, final LF):
  - valid against the Citation File Format 1.2.0 schema (`schema.json` from the
    `citation-file-format` repository's tag `1.2.0`, SHA-256 `0b8d2214…169b`, under the external
    root's `schema\`), whose licence list includes `PolyForm-Noncommercial-1.0.0`;
  - every field cross-checked against Zenodo's record: `doi` equals `conceptdoi`; the one
    identifier equals the version DOI; `version`, `date-released`, `title`, author name,
    affiliation, ORCID and licence equal the record's; `commit` equals the tag's commit;
  - Zenodo's API (`versions/latest`, 06:09:31Z) confirmed 23682781 as the latest and only
    version, so the file matches the latest deposit.
- **C. Records:**
  - ADR-0010 *Amendments*: a dated entry with Al's ruling verbatim, the scope, the file, the added
    procedure step and the frozen-check review;
  - STATE: Updated; Current build verification; the deposit item's new procedure step;
  - HANDOFF: this entry.
- **D. Gate** on the committed tree with a clean working tree; recorded in the pull request and
  under the external root, not in the tree it certifies.
BELIEF DELTAS:
- **A refresh after the deposit cannot be inside that deposit.** The version DOI and publication
  date exist only once Zenodo publishes, so the refresh lands in the records commit after it, and
  each archive carries the preceding `CITATION.cff`. Evidence: Session 32's sequence (tag,
  archive, deposit, then records). Consequence: the step is recorded that way, and the
  alternative, reserving the DOI before the tag, is put to Al in the pull request.
CONFUSIONS:
- "One file on its own branch and PR" alongside a procedure change that lives in three records.
  Resolved by Al's answer above before any write.
HONESTY CHECK:
- Every identifier was read from Zenodo's API or from git, not copied from the ruling.
- Two choices in the added step are the executor's and are flagged for Al in the pull request:
  later version DOIs are added under `identifiers` rather than replacing the Gate S1 entry, and
  `commit` and `title` are refreshed along with `version` and `date-released`.
- No register status, manifest, frozen packet or constitution byte changed; no S-entry added; the
  tag was not touched. Downgrades: none.
LEAKAGE CHECK: checked, not assumed — `contact/` was not opened; no detector, generator or
threshold was touched; the work concerns citation metadata. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED claims. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK:
- no statement about any real manuscript (SD3);
- no understanding claims (SD17);
- no detectors; no referee contact (SD13);
- no S2 code; the held branch untouched.
NEXT-SINGLE-ACTION: STATE's NEXT, unchanged — S2.1, `PROGRAM/DECISIONS/ADR-0011.md`, decision
only.
ESCALATIONS:
- This session's branch `claude/s2-citation-cff` and its pull request await Al's review; left
  unmerged.
- For Al, in the pull request: whether later version DOIs add to `identifiers` or replace the
  entry, and whether to reserve the DOI so each archive carries its own `CITATION.cff`.
- Unchanged: R4's seed family at ADR-0011's draft; the held branch's review; human-referee
  participants (SD13).
STOP-REASON: natural boundary — Al's ruling ends at a gated, reviewable pull request. Repo green:
the gate on the committed tree, recorded in the pull request.
**Memory writes:** none.
AL'S RULINGS ON PR 19'S TWO CALLS (2026-10-27, received in this session's conversation after PR #19
opened; recorded at Al's direction in a second commit on the same branch; transcribed verbatim
from Al's message):

~~~text
Rulings on PR #19's two calls:

Call 1: accepted. Each deposit adds its version DOI under identifiers; none is replaced.

Call 2: reserve the DOI first. For every future deposit, I reserve the version DOI in the Zenodo draft before the tag, and the CITATION.cff refresh (the new version DOI under identifiers, version, date-released and title) lands in the commit that gets tagged, so each archive cites itself. The records commit after the deposit confirms the published values instead of refreshing them. date-released is the planned publication date, and the Zenodo draft's publication date is set to match. A draft whose reserved DOI has been written into a tagged commit must be the draft that is published.

Consequence: remove the commit field from CITATION.cff now. Under this timing the file can never contain the hash of the commit that contains it; version names the tag, and the README and the Zenodo record carry the full hash. Update the file's header comment to match.

Record these rulings where PR #19 recorded the refresh step, add them to this PR as a new commit on the same branch (no force-push), run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

Before writing: `recheck --baseline 99bdf03…` re-fetched (exit 0, `TARGET-UNCHANGED`; receipt
`recheck-2-before-rulings.json` under the external root). `main` was still `99bdf03`, and the
branch was at `5ae0f2c` locally and on the remote.

Effect:
- **Call 1:** each deposit adds its version DOI under `identifiers`; none is replaced.
- **Call 2:** the refresh moves before the tag:
  - Al reserves the version DOI in the Zenodo draft, with its publication date set to the planned
    date;
  - the commit that is tagged carries the new version DOI, `version`, `date-released` (the
    planned date) and `title`, so each archive cites itself;
  - the draft whose reserved DOI is in a tagged commit is the one published;
  - the records commit after the deposit confirms instead of refreshing.
- **`CITATION.cff`:** the `commit` field is removed, and the header comment states the ruled
  procedure. The file is still valid against the Citation File Format 1.2.0 schema and matches
  Zenodo's record.
- **Where it is recorded:**
  - ADR-0010 *Amendments*: a new dated entry that supersedes the earlier entry's timing, its
    refreshed `commit` and its consequence;
  - STATE: the Updated bullet, the deposit item's step and Current build verification.

This decides the alternative the belief delta above put to Al and the second escalation above (the
two calls); the branch and its pull request still await Al's review. Nothing else changed. The
commit carrying this addendum is gated on its committed tree. The result is recorded in the pull
request and under the external root `C:\Users\LEET\Codex\Temp\syndeom-citation-cff-20261027\`
(`gate-committed-2.json`), not in the tree it certifies. **Memory writes:** none.
AL'S RULING ON THE OPEN POINT (2026-10-27, received in this session's conversation after the second
commit was pushed; recorded at Al's direction in a third commit on the same branch; transcribed
verbatim from Al's message):

~~~text
Ruling on the open point: the tagged CITATION.cff is the reference, because the archive cannot change and Zenodo's metadata can. If a published value differs from the tagged file, correct it on Zenodo to match the file. For a slip past the planned date, keep Zenodo's publication date at the file's date-released; the actual publication time stays in Zenodo's own record. The records commit after the deposit confirms the match and notes any correction made on Zenodo. If a value cannot be brought into line on Zenodo, never retag or rebuild: record the discrepancy in that records commit and correct main's CITATION.cff there.

Record this where the procedure lives, as a new commit on PR #19's branch (no force-push), run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

The open point: the executor's report on the second commit, made in this session's conversation,
noted that Al's rulings did not say what the records commit after a deposit does if a published
value differs from the tagged `CITATION.cff`, for example when publication slips past the planned
`date-released`.

Before writing: `recheck --baseline 99bdf03…` re-fetched (exit 0, `TARGET-UNCHANGED`; receipt
`recheck-4-before-ruling.json` under the external root). `main` was still `99bdf03`, the branch
was at `0465a8a` locally and on the remote, and PR 19 was open.

Effect (step 4 of the deposit procedure; steps 1–3 stand):
- the tagged `CITATION.cff` is the reference, because the archive cannot change and Zenodo's
  metadata can;
- a published value that differs from the tagged file is corrected on Zenodo to match it. After a
  slip past the planned date, Zenodo's publication date stays at the file's `date-released`, and
  the actual publication time stays in Zenodo's own record;
- the records commit after the deposit confirms the match and notes any correction made on Zenodo;
- if a value cannot be brought into line on Zenodo, never retag or rebuild: that records commit
  records the discrepancy and corrects `main`'s `CITATION.cff`.

Where it is recorded:
- ADR-0010 *Amendments*: a new dated entry that replaces step 4 of the entry above it;
- STATE: the Updated bullet and the deposit item's step 4.

`CITATION.cff` is unchanged. The `gate-s1-registration` tag's tree holds no `CITATION.cff`, and the
file matches Zenodo's record, re-fetched before writing (saved under the external root as
`zenodo/record-23682781-third-commit.json`). This decides the open point; the branch and its pull
request still await Al's review. Nothing else changed. The commit carrying this addendum is gated
on its committed tree. The result is recorded in the pull request and under the external root
`C:\Users\LEET\Codex\Temp\syndeom-citation-cff-20261027\` (`gate-committed-3.json`), not in the
tree it certifies. **Memory writes:** none.

---

SESSION: 2026-10-27 UTC (2026-10-27 America/New_York) (Session 34 — a session launched from
`C:\Users\LEET`, outside the checkout, with the §6 Session-N resume prompt verbatim; SD15
disclosure first; not an examination, because Gate S2 has nothing to examine, which could be
established only after orienting; examined fetched `origin/main`
`a741ebf956be0bd87ebf1b606f0b9c6df1853043`, tree `a5e7df5488530c560ab7ec9eb608037f92a03820`, from
the isolated worktree `C:\Users\LEET\Syndeom-wt-20261027`; branch `claude/s2-adr-0011-generator`;
header completed after ADR-0008 step 5 — it was written provisionally with the disclosure, which is
unchanged, and the CARRIED line below is part of the header; the session-end SHA is the commit
carrying this block, recorded in its pull request)
CARRIED (ADR-0008 D3): the disclosure below was first appended at 2026-10-27T19:17:58Z to the original checkout C:/Users/LEET/Syndeom (branch claude/s2-citation-cff @ 8ae94316f327194da86e3ce64b58d2573987e325, tree a5e7df5488530c560ab7ec9eb608037f92a03820; that append is uncommitted and left as it stands). The ADR-0008 preflight classified that checkout behind origin/main a741ebf956be0bd87ebf1b606f0b9c6df1853043 (same tree) and created this worktree; the same disclosure bytes are appended here as the first action in this checkout.

MEMORY DISCLOSURE (SD15 as amended by ADR-0005 and ADR-0009; first entry, before any other action)
- Order of operations: launched from the home directory C:/Users/LEET (not the work tree) with the §6 Session-N prompt verbatim. Read first, in full: the constitution and ADR-0005. Then the memory index and the memory file below were read and hashed. Nothing else was read before this append: no other repository file, no directory listing, no git history or status, no build. Appended by shell (cat >>) without reading HANDOFF; the block header above is provisional.
- Recalled memory loaded by the harness: the C--Users-LEET index MEMORY.md, 110 lines, SHA-256 cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69. No other memory file was loaded.
- D1(a) index lines naming this program or a sibling (KOBER, FRIEDMAN), exactly as loaded, line-number prefixed:

```
85: ## KOBER (Linear A falsification infrastructure)
86: - [Repo pointer + machine hazards](project_kober.md) — `C:\Users\LEET\KOBER`, remote `Quigles1337/KOBER` (**public**); the constitution `PROGRAM/KOBER-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (memory-policy ADR).
88: ## FR13DMAN (FRIEDMAN — designedness-detection instrument)
89: - [Repo pointer + machine hazards](project_friedman.md) — `C:\Users\LEET\FR13DMAN` / `Quigles1337/FR13DMAN` (**public**, leetspeak name); sibling of KOBER; the constitution `PROGRAM/FRIEDMAN-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (ADR-0003).
106: ## SYNDEOM (AI-proof calibration — third sibling of KOBER/FRIEDMAN)
107: - [Repo pointer + machine hazards](project_syndeom.md) — `C:\Users\LEET\Syndeom` / `Quigles1337/Syndeom` (**public**); the constitution `PROGRAM/SYNDEOM-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (SD15 / FR13DMAN ADR-0003).
```

- D1(b) the program memory file project_syndeom.md in full, exactly as read from disk (10957 bytes), SHA-256 ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0:

```
---
name: project-syndeom
description: "SYNDEOM program - repo C:\\Users\\LEET\\Syndeom (remote Quigles1337/Syndeom, public); constitution and ALL program state live IN the repo, never here"
metadata:
  node_type: memory
  type: project
  originSessionId: 07cebf44-c690-47a3-8c97-d4b909c4e75b
  modified: 2026-10-26T19:18:39.389Z
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

**Never work in the sibling KOBER or FR13DMAN trees** — identical `PROGRAM/` schema, and their
sessions may be live concurrently.

Machine and harness hazards:
- System-level git config has `core.autocrlf=true`; the repo pins LF via `.gitattributes`. Verify
  with `git ls-files --eol` and a byte-level CR/BOM check, never by eye.
- The Python console is cp1252: set `PYTHONIOENCODING=utf-8` before any script that prints
  non-ASCII (`§`, `→`, `—`), or it dies with a UnicodeEncodeError after doing its work.
- Cloning an empty GitHub repo yields the server's default branch; `git init` on this
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
- A ferried file pasted into the chat never matches its stated digest: the harness rendering of a
  Markdown file flattens tables to spaced text, drops `=` lines out of display math, strips heading
  and bullet markers, and prefixes/suffixes stray quotes. Transcribe and hash the paste once to
  show the mismatch, then ask for a path on disk holding the exact bytes. Never "repair" a paste
  into the ferried artifact.
- `www.apache.org` / `apache.org` (Fastly, 151.101.x) refused TCP connections from both curl and
  the fetch tool for a whole session while DNS resolved; the SPDX License List
  (`spdx.org/licenses/<id>.txt`, no trailing newline) and the GitHub Licenses API
  (`gh api licenses/<key>` — no leading slash, or Git Bash rewrites it as a path) both serve
  licence texts and cross-check each other after whitespace normalization.
- Project Euclid sits behind an Incapsula bot challenge (HTTP 200 with a ~1 KB challenge page to
  curl, empty content to the fetch tool). The Internet Archive's raw-capture route
  `web.archive.org/web/<timestamp>id_/<url>` serves the archived publisher PDF; find the timestamp
  with `archive.org/wayback/available?url=…`. Old JSTOR scans have no text layer — `pdftotext`
  returns only the JSTOR footer — so render pages with `pdftoppm -r 110 -png` and read the images.
- `scripts/check.sh` can take 10+ minutes after a reboot even with `lean/.lake` warm: the axiom
  audit (`lake env lean Audit.lean`) loads all of Mathlib's oleans from a cold disk cache
  (lean.exe grows past 1.4 GB). Run the gate in the background with a log; the ~30 s figure holds
  only with a warm OS file cache.
- arXiv API: `http://export.arxiv.org/api/query` answers 301 with an empty body (looks dead to a
  plain curl); use `https://export.arxiv.org/api/query`. The `co:` field prefix searches the
  revision comment (`search_query=co:erratum+AND+cat:math.*`); results are Atom XML.
- Crossref REST: `https://api.crossref.org/works?filter=update-type:erratum,type:journal-article`
  works (also `correction`, `corrigendum`, `retraction`); each hit carries `update-to`
  `[{type, DOI}]`; always pass `mailto=` for the polite pool. No subject filter — use zbMATH
  (`https://api.zbmath.org/v1/document/_search?search_string=ti:erratum`, JSON with
  `status.nr_total_results` and DOI `links`) to confine to mathematics.
- PowerShell's `bash` on this machine resolves to the Windows WSL stub (`C:\WINDOWS\system32\bash.exe`),
  which exits immediately; a launcher must name `C:\Program Files\Git\bin\bash.exe` explicitly.
- Run the gate detached with a log under the session scratchpad — `Start-Process` of Git Bash on a
  wrapper script that echoes start/end timestamps and the exit code, with `-RedirectStandardOutput`
  — so a harness kill (it reaps background shells when system memory is low) does not lose the
  result; a background `until grep -q 'GATE end' <log>; do sleep 10; done` is the notifier.
- Never run two gates at once: each axiom audit loads Mathlib's oleans (lean.exe past 1.4 GB), and
  two together starve the machine. Wait for `=== GATE end` with a foreground
  `until grep -q '=== GATE end' <log>; do sleep 10; done` loop (this harness tolerates it), and
  launch the pre-commit gate only after the ORIENT gate has ended.
- Python launched from Git Bash does not resolve `/c/Users/...` paths (FileNotFoundError on open);
  write `C:/Users/...` inside Python strings, and keep `/c/...` for the shell side only.
- The Route A finder caches its arXiv API pages only under the scratchpad of the session that
  fetched them (`<Temp>/claude/C--Users-LEET/<session-id>/scratchpad/route-a/pages/`). Copy
  `pages/*.xml` and `finder-state.json` from an earlier session's scratchpad (find it with
  `ls -d .../C--Users-LEET/*/scratchpad/route-a`) into the new one before running the finder:
  0 requests instead of ~30 min of fetching, and the digest check still proves identity.
- In `grep -E` / `awk` on this machine a pattern written `'^\\b[A-Z]'` matches a word boundary,
  not a literal backslash-b; write `'^[\\]b[A-Z]'` to match TeX macros such as `\bD`, `\bL`.
  A `\\section` pattern that found nothing in one call and everything in another was the same
  trap; prefer a bracket expression for a literal backslash. A quoted heredoc (`<<'EOF'`) fed
  to the Bash tool also strips one backslash level from Python source, leaving backspace
  characters where `\b` was meant: write backslash-bearing text with the Write/Edit tools.
- A working copy's `origin/<branch>` ref is only as fresh as its last fetch: `git status` can
  report a checkout as in sync while the remote is many commits ahead. Never treat a checkout's
  tracking ref as the remote's state. Never force-push; a diverged local commit goes to a branch.
- **A second checkout of this repository exists on this machine** at
  `C:\Users\LEET\Documents\ChatGPT\Syndeom` (used by another executor, Codex; it can hold its
  own unpushed local branches). Never work in it, reset it, or treat its refs as the remote's;
  `C:\Users\LEET\Syndeom` is this executor's checkout, and `origin` is the only shared truth.
  Durable external evidence roots for this program live under `C:\Users\LEET\Codex\Temp\`
  (one new directory per event, never reused); the session scratchpad is not durable.
- Use `git -C <path>` for every git command in the Bash tool: its working directory drifts
  between calls (a `cd` persists), and a bare `git` then fails with "not a git repository".
- Launched inside a git work tree (e.g. `C:\Users\LEET\Syndeom`), Claude Code attaches a
  gitStatus snapshot (branch, `git status`, the last five commit subjects) to the conversation
  before the first tool call, and loads memory from that directory's own project key
  (`C--Users-LEET-Syndeom`, empty) instead of the `C--Users-LEET` index; `C:\Users\LEET` is not a
  git work tree.
```

- D1(c) other recalled memory naming this program: none was loaded. A case-insensitive search of every file in the memory directory for the program name matched only the two files reproduced above.
- D2 withheld by count: 104 of the 110 index lines, at line numbers 1-84, 87, 90-105, 108-110. They concern unrelated programs and non-program sections of the index; none names this program or a sibling (D3).
- D3 check: case-insensitive search of the whole index for syndeom, kober, friedman and fr13dman (the FRIEDMAN repository name, added as an extra), by awk tolower() because grep -i aborts on this Git Bash; exit status 0; awk line count equals wc -l. Matched lines: 85, 86, 88, 89, 106, 107. Every matched line is reproduced under D1(a); no withheld line names the program or a sibling.
- D4 assessment: no reproduced memory holds program state under the SD15 enumeration (phase, gate status, CLAIM or FLAW status, file contents, commit SHA as state marker, next action, escalation). The index lines are repo pointers; the memory file holds the repo location, the second-checkout and evidence-root locations, and environment and tooling hazards. Borderline, flagged for audit by Al: two hazard lines name program tooling without its state (Audit.lean in the gate-duration hazard; the Route A finder page cache). No memory has been written this session; none will be if a cold-start exam is attempted.
- Conversation-context corollary: before the launch prompt the conversation held only the index above and a model-selection command; nothing carrying program state. Cold as of this entry; whether a gate exam is attempted is decided after ORIENT and recorded in this block.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (S2.1: ADR-0011 proposed — the decision, no code;
nothing else evidenced). Gate S1 CLOSED (Session 29).
ORIENT (ADR-0008 D1: step 0, then steps 1–5 by `python -I -X utf8
C:/Users/LEET/Syndeom/scripts/orient_preflight.py orient --repo C:/Users/LEET/Syndeom --remote
origin --expect-url https://github.com/Quigles1337/Syndeom --allow-dirty PROGRAM/HANDOFF.md
--receipt …`, then step 6 from the worktree):
- **Step 0.** The disclosure above was assembled from the memory files' bytes by a script and
  appended with `cat >>` to the original checkout's HANDOFF at 2026-10-27T19:17:58Z (the file's
  mtime; entry 15,108 bytes, SHA-256 `ef8239fd338da5fc0b945c22b0d1427b329f2c72354fb74ca745ef48f6759429`;
  the script and the entry are under the external root). The only byte of HANDOFF inspected
  beforehand was its last one (`tail -c1`, to decide whether a newline was needed), in the spirit
  of Session 29's `test -f` check, which Al ruled is not a read of program state. The
  constitution's `cat` output was persisted by the harness to a file, which was then read in
  full; that copy is byte-identical to the snapshot's constitution (step 5).
- **Identity:** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`.
- **Run 1** (receipt `orient-1-worktree-refused.json`, SHA-256
  `5a8eb680cea4b57f2d157c0769c7e1a08c39f6bcebb199e97e59614ef095e6fe`): `git fetch --prune origin`
  exit 0, 2026-10-27T19:18:40.10Z–19:18:40.82Z; default branch `main` (`git ls-remote --symref`);
  fetched `origin/main` `a741ebf956be0bd87ebf1b606f0b9c6df1853043`, tree
  `a5e7df5488530c560ab7ec9eb608037f92a03820`; `HEAD` `8ae94316f327194da86e3ce64b58d2573987e325` on
  branch `claude/s2-citation-cff`, the same tree; working tree ` M PROGRAM/HANDOFF.md` only
  (`allowed_present`; `unrelated: []`); ahead 0, behind 1 — the one remote-only commit is PR 19's
  merge (by Al). Classification **behind**, but not on the default branch, so no fast-forward:
  exit 3, `CHECKOUT-NOT-CURRENT`, nothing changed.
- **Run 2** (receipt `orient-2-worktree.json`, SHA-256
  `f29521a9a3716263b714e00487aeb49c4730e66cdce07016444a87cfb1a381a2`): the same facts after a
  second fetch (19:19:46Z–19:19:48Z); `--worktree C:/Users/LEET/Syndeom-wt-20261027` added a
  detached worktree at `a741ebf9…`, clean; exit 0, `SNAPSHOT-ESTABLISHED-IN-WORKTREE`. The
  original checkout is untouched.
- **Step 5.** The disclosure's body (14,931 bytes, SHA-256 `36dda70ceeb161b49069dfca5b43bf85be77f584fbdd4ddfc89792e83eb88d2f` — the entry
  less its provisional header) was appended to the worktree's HANDOFF as the first action there,
  under a provisional header with the CARRIED line (ADR-0008 D3); the original checkout's append
  stands. The constitution read in step 0 is byte-identical to the snapshot's (SHA-256
  `110f5360af60075a8e799dce0dc235d1ef8785bd40b0b34c38fa53ddb1d5479c`, 38,958 bytes); ADR-0005
  (`873f0751745155c96b30f3a1d40f8cacc1ed9fca478e3e2488449f8152e87595`), ADR-0008 and ADR-0009 are
  byte-identical in both checkouts.
- **Step 6, from the worktree only:** STATE → HANDOFF Sessions 30–33 → CLAIMS (seven entries;
  statuses as the gate prints them) → FLAWS (`0.1.0`, fourteen entries) → ADR-0001, ADR-0002,
  ADR-0010, the `syndeom-manifest`, `-ingest`, `-derive` and `-pair` crates, the S1.2 packet's
  audit convention, `scripts/check.sh` and `scripts/check_registers.py`. `git ls-files --eol`:
  every tracked text file `i/lf`; the only `i/-text` entries are the `.gz` binaries. Tool
  versions read: Lean `v4.34.1` / Mathlib `d13f23b7…` (pins unchanged), `cargo 1.91.0`,
  `cargo-audit 0.22.2`, `Lake 5.0.0-src+5045d00`. No pull request was open; the only tag is
  `gate-s1-registration` → `2018d5e`.
- **ORIENT gate, GREEN** on the worktree at `a741ebf9…` with the carried append as the only tracked
  change: full gate, detached with a log, 2026-10-27T19:30:55Z–19:41:28Z, exit 0 / `GATE: GREEN` —
  Lean `Build completed successfully (8926 jobs)`, golden axioms `[propext]`, fmt and clippy
  clean, 88 Rust tests (82 plus Session 30's six `relicense_exceptions`), 467 tracked files,
  Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 73; raw log SHA-256
  `245980cdba01e84de7541471091b0b74a3a9b3b4b55156ec0a0364e06778067e` (`gate-orient.raw.log`,
  `gate-orient.json` under `C:\Users\LEET\Codex\Temp\syndeom-adr-0011-20261027\`). Before the run,
  `lean/.lake` and `engine/target` were copied from the original checkout into the worktree with
  robocopy (untracked build products; the Mathlib package's `HEAD` verified at the pin), so the
  gate did not start cold; lake then rebuilt what it needed. The ADR-0011 draft was being written
  into the worktree during the run, untracked and therefore outside the gate's `git ls-files`
  scope; it is gated by the committed-tree run recorded in the pull request.
WHAT THE PROMPT ASKED: the §6 Session-N resume prompt, verbatim (compared by reading against
`PROGRAM/SYNDEOM-PROGRAM.md` lines 481–493: the same text), and nothing else — no instruction
from Al. STATE's NEXT-SINGLE-ACTION survived ORIENT: ADR-0010 is on `main` (PR 16), the numbering
holds, and nothing after Session 33 changed the S2 position.
ADVANCED:
- **A. The SD15 disclosure**, first entry of this block, carried into the worktree (ORIENT).
- **B. `PROGRAM/DECISIONS/ADR-0011.md`, proposed** (50,079 bytes; decides STATE's items (a)–(g),
  flags (h)): D1 one family, two producers `s2_template` 0.1.0 and `s2_grammar` 0.1.0, no new
  dependency; D2 the plan — aligned informal steps and formal fragments — as the one shared
  interface, with the pipeline plan → inject → pad → render → measure → manifest → write; D3 the
  full config and `FlawSpec` shapes, refusals, controls; D4 `syndeom-rng/1` (SplitMix64 seeding,
  xoshiro256** labelled streams, loop-free bounded draws, no floating point in decisions); D5 the
  dial table for FLAW-0001..FLAW-0014 with each instance's `lean_catchable`, predicted
  `lean_build` and confirmation check (K: kernel rejects at the recorded line; T: the audit tail's
  axioms and used-constants); D6 the v0.1 seed set (the FLAWS.md examples and ADR-0001 D8's
  theorem) and four grammars (`ring_chain`, `ineq_chain`, `case_split`, `nat_induction`) with
  what each realizes; D7 ground truth by construction and the witness modules under
  `lean/Syndeom/Seeds/`; D8 commanded keys and tolerances, achieved estimators, the one-process
  Lean harness in the S1.2 audit pattern, FAIL artifacts as findings, the fidelity sweep; D9
  manifests, `engine_commit` verification, the id invariant's four tests, the CLI, receipts; D10
  property tests, goldens (Golden A's and B's bytes reproduced as a writer test), the gate's audit
  generalization after the narrowing, `cargo audit` outside the offline gate; D11 placement,
  licence, ordering, cost; D12 flagged for Al — (h1) the R4 Mnemosyne-derived seed family, (h2)
  its licence line under R1(b), with three costed options and no recommendation.
- **C. STATE:** the Updated bullet; Current build verification (Session 34); the ADR-numbers
  line; the Gate S2 table's S2.1 row; two new standing escalations (ADR-0011 awaiting Al; two
  checkouts to tidy); the S1.3-tool escalation now points at NEXT; NEXT rewritten to ordering
  step 2 — the narrowing, recorded as ADR-0012.
- **D. HANDOFF:** this block.
- **E. Gate** on the committed tree with a clean working tree, recorded in the pull request and
  under the external root, not in the tree it certifies.
- No code, no register status, no manifest, no frozen packet, no preservation-tool source, no
  constitution byte, no README line, no S-entry.
BELIEF DELTAS:
- **A §6 launch lands on whatever branch the checkout was left on, and ADR-0008 then routes it to
  a worktree even when no byte differs.** The checkout was on Session 33's branch at main's own
  tree; the fast-forward case requires the default branch, so run 1 exited 3. Evidence: receipt 1.
  Consequence: the two-checkouts escalation; an informed session that ends on `main` (ADR-0008
  D3's last bullet) spares the next §6 launch a worktree.
- **The S2 family needs an alignment structure, not a text interface.** A pair's two records share
  one `instance` (ADR-0001 D4) and a kernel check must name the fragment's line range; both need
  the plan. Consequence: ADR-0011 D2.
- **ADR-0001 D8's Golden A and B are byte fixtures, not generator goldens.** Golden B's config has
  no `style` or `padding_ratio`, so no single full-config producer emits both D8 ids. Consequence:
  phase-prefixed producer ids and a writer test that reproduces the bytes (ADR-0011 D1, D10).
- **`cargo audit` cannot live in the gate**, which must stay offline (ADR-0008). Consequence:
  ADR-0011 D10 puts it beside the gate with its advisory-database commit recorded.
- **The gate runs 88 Rust tests now, not 82.** Evidence: the ORIENT log's fifteen `test result`
  lines. Consequence: STATE's build line.
CONFUSIONS:
- **ADR-0009 was read after the disclosure and before step 5**, beside ADR-0008. ADR-0008 step 0
  allows only ADR-0008 there. SD15's ineligibility clause concerns reads *before* disclosing, and
  this session is not an examination in any case, but the deviation is recorded here so that a
  future examination reads ADR-0008 alone between its disclosure and step 5. No document change
  is proposed: the rule is already written.
- **The harness reports robocopy's exit code 1 as a failure**; for robocopy, 1 means files were
  copied. Cost: one extra check. Not written to memory (Memory writes below).
- **A `cat` of the constitution came back as a harness-persisted file**, not inline. Reading that
  file is reading the constitution; its bytes were compared to the snapshot's (step 5). Cost:
  none.
HONESTY CHECK:
- Register entries re-examined: none changed; the gate prints the seven CLAIM statuses and the
  73 source states as before. Downgrades: none.
- ADR-0011 is **proposed**, not accepted; nothing in it is MEASURED; its cost statements are
  expectations and say so. Its D4 constants are written from the executor's knowledge of the
  published algorithms, not from a retrieved source: the ADR makes its own definition normative
  and leaves the attribution to be checked against retrieved vectors at implementation. Its D6
  seed names are the executor's, and the Lean feasibility of each seed (the real-analysis ones in
  particular) is not demonstrated here.
- The 88-test count was summed by hand from the log's `test result` lines.
- The word "proven" appears in ADR-0011 D10 only inside a quotation of ADR-0001's own phrase about
  goldens ("wrong until proven otherwise"); no SD2 use.
LEAKAGE CHECK: checked, not assumed — `contact/` holds only its README (read for the house rules)
and nothing else was opened; no detector, threshold or generator code exists; every design input
is FLAWS.md's executor-authored examples, ADR-0001's fixtures and the existing producers; S-0001
and S-0006 are cited in the ADR only for their existence. Verdict: no leakage.
DETERMINISM CHECK: no MEASURED or REPRODUCED claims. The gate run is recorded with its log
digest. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing (the one occurrence is
a quotation, above); nothing synthetic labeled real.
DRIFT CHECK:
- no statement about any real manuscript (SD3);
- no understanding claims (SD17); FLAW-0013 stays a cost in the ADR;
- no detector chosen or tuned; no referee contact (SD13);
- no S2 code; the held branch untouched; the SD8 quarantine untouched.
NEXT-SINGLE-ACTION: STATE's NEXT — S2 ordering step 2, the full narrowing of the S1.3
reproduction tool's byte checks, decided and recorded as ADR-0012; before any S2 code.
ESCALATIONS:
- **ADR-0011 D12 for Al:** (h1) whether the R4 Mnemosyne-derived seed family enters as an
  `s2_template` seed; (h2) its licence line under R1(b) — three options costed, none recommended.
  Three non-blocking clarifications are listed there. This session's pull request awaits Al's
  review; left unmerged.
- **Two checkouts to tidy (Al's call):** the original checkout's uncommitted disclosure append on
  `claude/s2-citation-cff` (never erased by the session, ADR-0008 D3) and the worktree
  `C:\Users\LEET\Syndeom-wt-20261027` (STATE, Standing escalations).
- Unchanged: the held branch's admission review, scheduled by Al after ADR-0011; human-referee
  participants (SD13).
STOP-REASON: natural boundary — the inherited NEXT-SINGLE-ACTION ends at a gated, reviewable pull
request, and the next action is a different task. Repo green: the ORIENT gate above, and the
committed-tree gate recorded in the pull request.
**Memory writes:** none. (This session established after orienting that it is not an
examination, so a hazard write would have been permitted; none was made.)
AL'S RULINGS ON PR 20 (2026-10-27, received in this session's conversation after PR #20 opened;
recorded at Al's direction in a second commit on the same branch, no force-push; transcribed
verbatim from Al's message; exact copy `owner-rulings-pr20-2026-10-27.txt` under the external
root, SHA-256 `c5ac5ec64a0ca3a80c95cc48390c658e07d553fe1ce97bef28a4d7d121a4ba6c`):

~~~text
Rulings on PR #20 (ADR-0011):

1. Independence from the S1 corpus. Add to ADR-0011, before merge: no seed, grammar or template may be derived from any item under groundtruth/, because that corpus is what the frozen battery is validated against in S4. Seeds come from FLAWS.md or other material outside the S1 corpus, and every seed record states its source. This binds later producer versions as the seed set grows.

2. D12, the R4 seed family: option (iii). Keep it out of this repository as a private fixture under an external root, for private pipeline tests only; it never becomes calibration data. Do not author it into the public goldens, the seeds directory or any artifact here. If I want it public later, that will be its own amendment.

3. The non-blocking clarifications in D12: accepted as recorded. The edits to scripts/check.sh and lean/Audit.lean come after the narrowing and get their own review.

4. Housekeeping, for the first session after the merge: remove the worktree at C:\Users\LEET\Syndeom-wt-20261027. In the original checkout, confirm the uncommitted HANDOFF append is the same disclosure now committed by comparing the body's SHA-256, then discard it.

Record these rulings in ADR-0011's Amendments and wherever STATE tracks them, as a new commit on PR #20's branch (no force-push). Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

Before writing: `recheck --baseline a741ebf9…` re-fetched (exit 0, `TARGET-UNCHANGED`; receipt
`recheck-2-before-rulings.json` under the external root). `main` was still `a741ebf9…`, the branch
was at `eba1462` locally and on the remote, and PR 20 was open.

Effect:
- **Ruling 1** → ADR-0011 D6 gains the independence rule: no seed, grammar or template derived
  from any `groundtruth/` item (the S4 validation corpus); seeds from FLAWS.md or other material
  outside the S1 corpus; every seed record states its source (a `source` field); a grammar's atoms
  and the padding material bound the same way; binding on later producer versions. The v0.1 seed
  set already complies; the admitted S1.2 pair is therefore not a seed.
- **Ruling 2** → D12 (h1)/(h2) resolved: option (iii). The R4 Mnemosyne-derived family stays out
  of this repository as a private fixture under an external root, for private pipeline tests
  only; never calibration data; never in the public goldens, the seeds directory or any artifact
  here. Recorded in D12 and *Consequences*; D6's seed set unchanged.
- **Ruling 3** → the non-blocking clarifications stand as recorded; the `scripts/check.sh` and
  `lean/Audit.lean` edits (D10) come after the narrowing and get their own review.
- **Ruling 4** → housekeeping for the first session after the merge: remove the worktree; in the
  original checkout compare the uncommitted append's body SHA-256 with the committed disclosure's
  (`36dda70c…8d2f`) and discard it. Recorded in STATE's "Two checkouts" escalation and at the head
  of NEXT; **not executed now** — the merge has not happened.
- ADR-0011's Status: accepted, in effect at the merge; *Amendments*: the rulings verbatim and
  their effect. STATE: the Updated bullet, Current build verification, the ADR-numbers line, the
  Gate S2 S2.1 row, the two standing escalations, NEXT. HANDOFF: this addendum.

Nothing else changed: no register status, manifest, frozen packet, preservation-tool source,
constitution byte, README line or code. The commit carrying this addendum is gated on its
committed tree; the result is recorded in the pull request and under the external root
(`gate-committed-2.json`), not in the tree it certifies. **Memory writes:** none.

---

SESSION: 2026-10-27 UTC (2026-10-27 America/New_York) (Session 35 — a session launched from
`C:\Users\LEET`, outside the checkout, with the §6 Session-N resume prompt verbatim followed by
an owner note; SD15 disclosure first; not an examination — the owner note carries program
state, and Gate S2 has nothing to examine; examined fetched `origin/main`
`6d89eb2f8a138a53d824d035febffc93423e1e0d`, tree `93ded916fb8cbfaea21f7d2a2367f86415fe3fc9`,
in the original checkout `C:\Users\LEET\Syndeom` on `main`, synchronized; branch
`claude/s2-adr-0012-input-set`; header completed after ADR-0008 step 5 — it was written
provisionally with the disclosure, which is unchanged; the session-end SHA is the commit
carrying this block, recorded in its pull request)

### Entry 1 — SD15 memory disclosure (ADR-0005 D1–D4; ADR-0009)

- **Order of operations.** Made before any other action. The only things read first were `PROGRAM/SYNDEOM-PROGRAM.md` in full (the command's output exceeded the tool's display limit, so the full text was read back from the harness's persisted copy of that same command's output; the repository file was not opened a second time) and `PROGRAM/DECISIONS/ADR-0005.md` in full, then the memory reproduced here, read with line numbers and hashed. No listing of the checkout, no git command, no build and no other repository file preceded this entry. The entry was appended by a shell script that does not read HANDOFF's existing contents (it tests only that the file exists and records its byte length before and after the append).
- **Launch context.** Launched from `C:\Users\LEET`, which is not a git work tree, so the harness loaded the `C--Users-LEET` memory index. The launching prompt was the §6 Session-N resume prompt verbatim **followed by an owner note that carries program state** (a session number, a merged PR number, a housekeeping outcome and a receipt path). By SD15's conversation-context corollary this session is therefore **not cold and cannot serve as any gate's cold-start examination**; the disclosure is made anyway, as ADR-0009 requires of every session launched with that prompt.
- **Recalled memory loaded by the harness:** exactly one item, the executor memory index `C:\Users\LEET\.claude\projects\C--Users-LEET\memory\MEMORY.md` — 110 lines, LF, no CR bytes, SHA-256 `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`. The harness-loaded copy was compared by inspection against the on-disk file and no difference was found. No other memory was loaded by the harness (D1(c): none).
- **D1(a) — every index line naming SYNDEOM or a sibling (KOBER, FRIEDMAN/FR13DMAN), verbatim, in index order — lines 85, 86, 88, 89, 106, 107 — emitted by `awk` from the file:**

````text
## KOBER (Linear A falsification infrastructure)
- [Repo pointer + machine hazards](project_kober.md) — `C:\Users\LEET\KOBER`, remote `Quigles1337/KOBER` (**public**); the constitution `PROGRAM/KOBER-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (memory-policy ADR).
## FR13DMAN (FRIEDMAN — designedness-detection instrument)
- [Repo pointer + machine hazards](project_friedman.md) — `C:\Users\LEET\FR13DMAN` / `Quigles1337/FR13DMAN` (**public**, leetspeak name); sibling of KOBER; the constitution `PROGRAM/FRIEDMAN-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (ADR-0003).
## SYNDEOM (AI-proof calibration — third sibling of KOBER/FRIEDMAN)
- [Repo pointer + machine hazards](project_syndeom.md) — `C:\Users\LEET\Syndeom` / `Quigles1337/Syndeom` (**public**); the constitution `PROGRAM/SYNDEOM-PROGRAM.md` is the sole authority — always run its §4 loop from the repo, never from memory. No program state is recorded here or in the linked file (SD15 / FR13DMAN ADR-0003).
````

- **D1(b) — this program's own memory file `project_syndeom.md`, in full, exactly as read (LF, no CR bytes, ends with a newline), SHA-256 `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, emitted by `cat` from the file:**

````text
---
name: project-syndeom
description: "SYNDEOM program - repo C:\\Users\\LEET\\Syndeom (remote Quigles1337/Syndeom, public); constitution and ALL program state live IN the repo, never here"
metadata:
  node_type: memory
  type: project
  originSessionId: 07cebf44-c690-47a3-8c97-d4b909c4e75b
  modified: 2026-10-26T19:18:39.389Z
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

**Never work in the sibling KOBER or FR13DMAN trees** — identical `PROGRAM/` schema, and their
sessions may be live concurrently.

Machine and harness hazards:
- System-level git config has `core.autocrlf=true`; the repo pins LF via `.gitattributes`. Verify
  with `git ls-files --eol` and a byte-level CR/BOM check, never by eye.
- The Python console is cp1252: set `PYTHONIOENCODING=utf-8` before any script that prints
  non-ASCII (`§`, `→`, `—`), or it dies with a UnicodeEncodeError after doing its work.
- Cloning an empty GitHub repo yields the server's default branch; `git init` on this
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
- A ferried file pasted into the chat never matches its stated digest: the harness rendering of a
  Markdown file flattens tables to spaced text, drops `=` lines out of display math, strips heading
  and bullet markers, and prefixes/suffixes stray quotes. Transcribe and hash the paste once to
  show the mismatch, then ask for a path on disk holding the exact bytes. Never "repair" a paste
  into the ferried artifact.
- `www.apache.org` / `apache.org` (Fastly, 151.101.x) refused TCP connections from both curl and
  the fetch tool for a whole session while DNS resolved; the SPDX License List
  (`spdx.org/licenses/<id>.txt`, no trailing newline) and the GitHub Licenses API
  (`gh api licenses/<key>` — no leading slash, or Git Bash rewrites it as a path) both serve
  licence texts and cross-check each other after whitespace normalization.
- Project Euclid sits behind an Incapsula bot challenge (HTTP 200 with a ~1 KB challenge page to
  curl, empty content to the fetch tool). The Internet Archive's raw-capture route
  `web.archive.org/web/<timestamp>id_/<url>` serves the archived publisher PDF; find the timestamp
  with `archive.org/wayback/available?url=…`. Old JSTOR scans have no text layer — `pdftotext`
  returns only the JSTOR footer — so render pages with `pdftoppm -r 110 -png` and read the images.
- `scripts/check.sh` can take 10+ minutes after a reboot even with `lean/.lake` warm: the axiom
  audit (`lake env lean Audit.lean`) loads all of Mathlib's oleans from a cold disk cache
  (lean.exe grows past 1.4 GB). Run the gate in the background with a log; the ~30 s figure holds
  only with a warm OS file cache.
- arXiv API: `http://export.arxiv.org/api/query` answers 301 with an empty body (looks dead to a
  plain curl); use `https://export.arxiv.org/api/query`. The `co:` field prefix searches the
  revision comment (`search_query=co:erratum+AND+cat:math.*`); results are Atom XML.
- Crossref REST: `https://api.crossref.org/works?filter=update-type:erratum,type:journal-article`
  works (also `correction`, `corrigendum`, `retraction`); each hit carries `update-to`
  `[{type, DOI}]`; always pass `mailto=` for the polite pool. No subject filter — use zbMATH
  (`https://api.zbmath.org/v1/document/_search?search_string=ti:erratum`, JSON with
  `status.nr_total_results` and DOI `links`) to confine to mathematics.
- PowerShell's `bash` on this machine resolves to the Windows WSL stub (`C:\WINDOWS\system32\bash.exe`),
  which exits immediately; a launcher must name `C:\Program Files\Git\bin\bash.exe` explicitly.
- Run the gate detached with a log under the session scratchpad — `Start-Process` of Git Bash on a
  wrapper script that echoes start/end timestamps and the exit code, with `-RedirectStandardOutput`
  — so a harness kill (it reaps background shells when system memory is low) does not lose the
  result; a background `until grep -q 'GATE end' <log>; do sleep 10; done` is the notifier.
- Never run two gates at once: each axiom audit loads Mathlib's oleans (lean.exe past 1.4 GB), and
  two together starve the machine. Wait for `=== GATE end` with a foreground
  `until grep -q '=== GATE end' <log>; do sleep 10; done` loop (this harness tolerates it), and
  launch the pre-commit gate only after the ORIENT gate has ended.
- Python launched from Git Bash does not resolve `/c/Users/...` paths (FileNotFoundError on open);
  write `C:/Users/...` inside Python strings, and keep `/c/...` for the shell side only.
- The Route A finder caches its arXiv API pages only under the scratchpad of the session that
  fetched them (`<Temp>/claude/C--Users-LEET/<session-id>/scratchpad/route-a/pages/`). Copy
  `pages/*.xml` and `finder-state.json` from an earlier session's scratchpad (find it with
  `ls -d .../C--Users-LEET/*/scratchpad/route-a`) into the new one before running the finder:
  0 requests instead of ~30 min of fetching, and the digest check still proves identity.
- In `grep -E` / `awk` on this machine a pattern written `'^\\b[A-Z]'` matches a word boundary,
  not a literal backslash-b; write `'^[\\]b[A-Z]'` to match TeX macros such as `\bD`, `\bL`.
  A `\\section` pattern that found nothing in one call and everything in another was the same
  trap; prefer a bracket expression for a literal backslash. A quoted heredoc (`<<'EOF'`) fed
  to the Bash tool also strips one backslash level from Python source, leaving backspace
  characters where `\b` was meant: write backslash-bearing text with the Write/Edit tools.
- A working copy's `origin/<branch>` ref is only as fresh as its last fetch: `git status` can
  report a checkout as in sync while the remote is many commits ahead. Never treat a checkout's
  tracking ref as the remote's state. Never force-push; a diverged local commit goes to a branch.
- **A second checkout of this repository exists on this machine** at
  `C:\Users\LEET\Documents\ChatGPT\Syndeom` (used by another executor, Codex; it can hold its
  own unpushed local branches). Never work in it, reset it, or treat its refs as the remote's;
  `C:\Users\LEET\Syndeom` is this executor's checkout, and `origin` is the only shared truth.
  Durable external evidence roots for this program live under `C:\Users\LEET\Codex\Temp\`
  (one new directory per event, never reused); the session scratchpad is not durable.
- Use `git -C <path>` for every git command in the Bash tool: its working directory drifts
  between calls (a `cd` persists), and a bare `git` then fails with "not a git repository".
- Launched inside a git work tree (e.g. `C:\Users\LEET\Syndeom`), Claude Code attaches a
  gitStatus snapshot (branch, `git status`, the last five commit subjects) to the conversation
  before the first tool call, and loads memory from that directory's own project key
  (`C--Users-LEET-Syndeom`, empty) instead of the `C--Users-LEET` index; `C:\Users\LEET` is not a
  git work tree.
````

- **D2 — withheld by count.** 104 of the 110 index lines are withheld because this repository is public: lines 1–84, 87, 90–105 and 108–110. They are the index title, the user-identity line, blank lines, machine-layout and working-preference notes, and the section headers and entries of unrelated private programs.
- **D3 — the check.** A case-insensitive search of the whole index for `syndeom`, `kober`, `friedman` and `fr13dman` (awk `tolower` alternation, exit code 0; cross-checked by four single-pattern `grep -n -i` runs, each exit code 0) matched lines 85, 86, 88, 89, 106 and 107 and no others. Every matched line is reproduced under D1(a); no withheld line names this program or a sibling.
- **D4 — program-state assessment of what is reproduced.** The six index lines hold repo locations, remotes, visibility and the memory-policy statement. `project_syndeom.md` holds the repo location and remote, a second-checkout hazard, and environment and tooling hazards. Nothing reproduced states a phase, a gate status, a CLAIM or FLAW status, file contents, a commit SHA as a state marker, a next action or an escalation. Two hazard items name program tooling without stating its status — the gate's axiom audit under `scripts/check.sh`, and the "Route A finder" page cache — and are flagged for Al's audit as the nearest thing to state in the file. No memory was written before or during this disclosure.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (S2.1: ADR-0011 accepted, in effect since PR 20's
merge; no code). Gate S1 CLOSED (Session 29).
ORIENT (ADR-0008 D1: step 0, then steps 1–5 by `python -I -X utf8
C:/Users/LEET/Syndeom/scripts/orient_preflight.py orient --repo C:/Users/LEET/Syndeom --remote
origin --expect-url https://github.com/Quigles1337/Syndeom --allow-dirty PROGRAM/HANDOFF.md
--receipt …`, then step 6 from the same checkout):
- **Step 0.** The disclosure above was assembled from the memory files' bytes by a script
  (`append-disclosure.sh`: three authored fragments, the six index lines emitted by `awk`, the
  memory file emitted by `cat`, both digests re-verified before emission, the four-backtick fence
  checked for collision) and appended by shell redirection between 2026-10-27T20:49:33Z (the
  script's write) and 20:50:41Z (the preflight's start); the exact mtime was not captured. The
  script tested only that HANDOFF exists and recorded its byte length before and after (538,110 →
  554,137; 16,027 bytes appended; the reproduced memory file re-hashes inside HANDOFF to
  `ce2ce6de…2cc0`). The constitution's `cat` output was persisted by the harness to a file and
  read back in full; step 5 confirms the snapshot's constitution is byte-identical (SHA-256
  `110f5360af60075a8e799dce0dc235d1ef8785bd40b0b34c38fa53ddb1d5479c`, 38,958 bytes) and so is
  ADR-0005 (`873f0751…7595`). The script, the fragments and the appended entry are under the
  external root `C:\Users\LEET\Codex\Temp\syndeom-adr-0012-20261027\`.
- **Launch context, recorded after the disclosure:** the §6 prompt verbatim plus Al's owner
  note (the ruling-4 housekeeping was done in Session 34's conversation after PR 20 merged;
  verify it in the original checkout on the default branch, no extra worktree, and record it
  from the receipt instead of redoing it). The note carries program state, so this session was
  never cold (SD15's corollary). The harness also attached the tool catalogue, the environment
  block ("Is a git repository: false") and the account e-mail (withheld); none is memory, none
  names this program.
- **Identity:** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`.
- **Run 1** (receipt `orient-1.json`, SHA-256 `2a0d154b…6418`): `git fetch --prune origin` exit 0,
  2026-10-27T20:50:41.31Z–20:50:42.98Z; default branch `main` (`git ls-remote --symref`); fetched
  `origin/main` `6d89eb2f8a138a53d824d035febffc93423e1e0d`, tree
  `93ded916fb8cbfaea21f7d2a2367f86415fe3fc9`; `HEAD` the same commit on branch `main`; working
  tree ` M PROGRAM/HANDOFF.md` only (`allowed_present`; `unrelated: []`); ahead 0, behind 0.
  Classification **synchronized**; exit 0, `SNAPSHOT-ESTABLISHED`; the checkout is the snapshot.
  `6d89eb2f…` is PR 20's merge (by Al, 2026-10-27T20:28:10Z); its tree equals PR 20's head tree
  (`751fd97`), which Session 34 gated GREEN (`gate-committed-2.json`).
- **Step 5.** `git diff --stat`: HANDOFF only (+162 lines). Constitution and ADR-0005 as above;
  ADR-0008 `ccdfa9d7…e9ff`.
- **Deviation recorded:** ADR-0009 was read beside ADR-0008 between the disclosure and step 5
  (Session 34 made the same deviation; step 0 names ADR-0008 alone). Without consequence here —
  the session was not cold — and recorded so that an examination does not repeat it.
- **Step 6:** STATE → HANDOFF Session 34 (lines 6275–6651) → CLAIMS (seven entries; six
  VERIFIED, CLAIM-0006 CONDITIONAL) → FLAWS (`0.1.0`, fourteen entries) → ADR-0010 D6–D7,
  ADR-0011 *Amendments*, the reproduction tool and its tests, the integration packet's README,
  VERIFICATION and baseline list, the S1.4 coverage module, `scripts/check.sh`,
  `scripts/check_registers.py`. `git log --oneline -20`: PR 20's merge at the tip; tags:
  `gate-s1-registration` only; `git ls-remote --heads origin`: `main` and seven feature branches
  (the held branch among them). No pull request open.
- **Housekeeping verification (Al's note):** from the Session-34 receipt
  (`housekeeping-receipt.json`, `170e0d58…4670`; `housekeeping.log`, `67f159cc…f7da`;
  `housekeeping-orient-sync.json`, `41235974…aee2`): `git worktree list` shows one checkout; the
  worktree directory is absent; local branches include `claude/s2-adr-0011-generator@751fd97`
  (kept) and `claude/s2-citation-cff@8ae9431`; the HEAD blob at `8ae9431` is `5b453c0c…340a` and
  equals the retained `handoff-head-8ae9431.md` (504,909 bytes); the HEAD blob at `6d89eb2` is
  `cc8f47b4…dc23` and equals `handoff-origin-main.md` (538,110 bytes — the length this session's
  append started from); the retained discarded entry `handoff-appended-tail.md` (15,108 bytes,
  `ef8239fd…9429`) yields the body `36dda70c…8d2f` (14,931 bytes) by the recorded markers, equal
  to HEAD's lines 6286–6444. Consistent with the receipt in every field checked; nothing redone.
- **ORIENT gate, GREEN** on the checkout at `6d89eb2f…` with the disclosure append as the only
  change: full gate, detached with a log (`gate-orient.sh`, Start-Process of Git Bash),
  2026-10-27T20:54:16Z–21:03:15Z, exit 0 / `GATE: GREEN` — Lean `Build completed successfully
  (8926 jobs)`, golden axioms `[propext]`, fmt and clippy clean, 88 Rust tests, 468 tracked
  files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 73; raw log 136 lines, SHA-256
  `627ef560f579fa33f8e3e712bf5094b941b835149c8ea8691c277719a938a416` (`gate-orient.raw.log`
  under the external root). The gate's cargo phase compiled the pre-ADR-0012 sources (CONFUSIONS).
WHAT THE PROMPT ASKED: the §6 Session-N resume prompt verbatim (the same text as
`PROGRAM/SYNDEOM-PROGRAM.md` lines 481–493), followed by Al's owner note on the housekeeping,
given in substance under ORIENT. STATE's NEXT-SINGLE-ACTION survived ORIENT: its precondition
(the ruling-4 housekeeping) was already done and is verified above; ADR-0012 depended on nothing
else pending.
ADVANCED:
- **A. The SD15 disclosure**, first entry of this block, and the housekeeping verification
  (ORIENT), recorded in STATE (Updated bullet; the "Two checkouts" escalation closed).
- **B. ADR-0012** (`PROGRAM/DECISIONS/ADR-0012.md`, proposed): D1 the input set (79 of 375
  baseline files, derived from the baseline list: the taxonomy, 34 item directories holding 73
  files, the dispositions overlay, four candidate-packet files), D2 what is checked and nothing
  else, D3 the four exceptions replaced, D4 the pair-input comparison removed (hashes still
  recorded from the producer revision), D5 the workspace manifest and lock file, D6 the output's
  `preservation` object, D7 what is not decided (the population pin; the packets' immutability
  is the program's commitment), D8 the tests.
- **C. The tool:** `engine/ingest/examples/support/effective.rs` `49e6cd6d…1a67` →
  `2f505633…85f9` (+106 bytes): `input_set()`, `candidate_dir()`, the new `preserve()`, the pair
  hashes read from the producer revision, `pair_input_check` in provenance;
  `LICENSE_LINE_FILES`, `license_line_restored`, `pair_input_unchanged` and the `Cow` import
  removed. `cargo fmt`, clippy at deny-warnings and the ingest crate's tests (15 + 5 + 6) pass.
- **D. Tests:** `relicense_exceptions.rs` (6) removed; `input_set.rs` (6) added;
  `first_parent_amendment.rs` two of fifteen rewritten to the new boundary (ADR-0012 D8).
- **E. Measurements** (ADR-0012 *Measurements*; receipts under the external root): at
  `3cd5558` with that revision's tool, PASS, the retained `inventory.json` byte-identical
  (`b626d1b5…1b803`; 21:04:50Z–21:05:04Z); on main's tree with the revised tool (scratch clone,
  throwaway commit `74dc18da…`, never pushed), PASS, `inventory.json` `b9d6d736…0fcc` differing
  from the retained in exactly the eleven `preservation` paths (`inventory-diff.txt`), and
  provenance in the checkout fields, `pair_input_check` and three `tool_source_sha256` entries
  (`provenance-diff.txt`); 21:05:50Z–21:06:01Z.
- **F. Records:** ADR-0009 and ADR-0010 gain one *Amendments* entry each; README § Reproduce
  the effective inventory, one sentence; STATE: the Updated bullet, Current build verification,
  the ADR-numbers line, the Gate S1 S1.3 row, three standing escalations (ADR-0011 merged; two
  checkouts DONE; S1.3 tool DONE), the held-branch bullet, NEXT rewritten to ordering step 3
  (the held harvest's admission review). The Gate S2 table is unchanged.
- **G. HANDOFF:** this block.
- **H. Gate** on the committed tree with a clean working tree, and the reproduction from a fresh
  clone of the commit, recorded in the pull request and under the external root, not in the
  tree they certify.
- No register status, manifest, frozen packet, retained report, constitution byte, licence,
  tag or S2 code changed; the held branch untouched.
BELIEF DELTAS:
- **A data reproduction should pin what it reads, and no more.** The baseline's 375-file pin
  was a frozen-packet guarantee carried inside a census; each amendment elsewhere broke the
  census (ADR-0009, ADR-0010, and S2's first commit next). Evidence: the input set is 79 files,
  read from the tool; the other 296 pinned files were never inputs. Consequence: ADR-0012 D1–D3.
- **The pair-input comparison guarded a claim the program had already retired.** STATE records
  the replay as pinned to a revision range since ADR-0010; comparing HEAD's bytes with the
  producer revision certified nothing current and would have failed at S2's first commit.
  Consequence: D4, D5.
- **Al's housekeeping had already been done, and the receipt was enough to verify it without a
  state-changing command.** Evidence: the worktree absent; the blob and body digests recomputed
  from Git objects and the retained copies. Consequence: no redo; STATE records it.
- **The §6 prompt plus an owner note is not a cold launch.** The note named a session number, a
  merged PR and a receipt path, so this session could never be an examination; the disclosure is
  still made (ADR-0009). Consequence: recorded in the disclosure and the header.
CONFUSIONS:
- **Engine edits began while the ORIENT gate was still running.** The gate's cargo phase had
  already compiled: its 88 tests include the removed six-test suite, which cannot compile
  against the revised tool, and `cargo fmt --check` passed although `cargo fmt` later
  reformatted the edited files; so the GREEN is the pre-edit tree's. Cost: this paragraph. Doc
  fix: none needed — the hazard line already says to wait for `=== GATE end`; a successor should
  also not edit tracked engine sources until it appears.
- **ADR-0009 read beside ADR-0008 before step 5** (ORIENT, deviation recorded).
- **The reproduction needs a clean checkout**, so the HEAD measurement could not run in the
  working checkout with the disclosure uncommitted; a scratch clone with a throwaway commit was
  the measurement vehicle, recorded as such (never pushed; its commit id is in the ADR).
- **`grep -c` with zero matches ends an `&&` chain** (exit 1); one Bash call stopped before its
  last reads and was rerun. Cost: one call. The hazard is already in memory ("a `grep` inside an
  `&&` chain is not a guard").
HONESTY CHECK:
- Register entries re-examined: none changed; the gate prints the seven CLAIM statuses and the
  73 source states as before. Downgrades: none.
- ADR-0012 is **proposed**; its measurements are deterministic, seed-free runs with the engine
  commit, tool digests and output digests recorded (SD6), not CLAIMS entries.
- The input set was read from the tool's code, its tests and the baseline list (SD7), not from
  the packet README, which describes the tool as it was at integration and is left unedited.
- The "88 tests" and "79 inputs" counts were produced by the tools (`cargo test` output; the
  `input_set` test), not summed by hand.
- The HEAD measurement ran on a throwaway commit whose tree is main's plus this commit's three
  engine files; the committed tree's own reproduction runs after the commit and is recorded in
  the pull request.
LEAKAGE CHECK: checked, not assumed — `contact/` was not opened; no detector, threshold or
generator code exists or was written; every input to the decision is the tool's source, its
tests, the baseline list and the program's own records. Verdict: no leakage.
DETERMINISM CHECK: every number here is regenerable: the two reproductions from `3cd5558` and
from the committed tree with the receipts' commands; the gate from `scripts/check.sh` on the
committed tree; the digests from the files named. Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK:
- no statement about any real manuscript (SD3);
- no understanding claims (SD17);
- no detector chosen or tuned; no referee contact (SD13);
- no S2 code; the held branch untouched; the SD8 quarantine untouched.
NEXT-SINGLE-ACTION: STATE's NEXT — S2 ordering step 3, the held harvest's admission review
(branch `claude/s1-harvest-candidates-28-30` at `c3a3d36`, five items), reviewed and recorded,
not integrated; then step 4, ADR-0011's crate, after Al rules on the review.
ESCALATIONS:
- **ADR-0012 for Al's review** on this session's pull request: the input-set rule (D1–D2), the
  replacement of the four exceptions (D3), the removal of the pair-input comparison (D4), and D7
  — the frozen packets' immutability now rests on review and their own receipts, not on this
  tool.
- **Two checkouts:** closed (housekeeping done in Session 34, verified here).
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary — the inherited NEXT-SINGLE-ACTION ends at a gated, reviewable pull
request, and the next action is a different task. Repo green: the ORIENT gate above; the
committed-tree gate recorded in the pull request.
**Memory writes:** none. (This session established at its launch that it is not an examination,
so a hazard write would have been permitted; none was needed — the one new observation, not
editing engine sources before `=== GATE end`, is a corollary of a hazard already recorded.)

---

SESSION: 2026-10-27 UTC (2026-10-27 America/New_York) (Session 36 — an informed working session,
not an examination, continuing Session 35's conversation at Al's direction after PR 21's merge;
not launched with the §6 prompt, so no SD15 disclosure; examined fetched `origin/main`
`f8d71b8c8034638f14e9a5885ae348c8fdde24c4`, tree `35bb1439f4d7ac4ca54734ca61192ec4a68b3ff1`, in
the checkout `C:\Users\LEET\Syndeom` on `main` after a fast-forward; branch
`claude/s2-adr-0012-freeze-register`; the session-end SHA is the commit carrying this block,
recorded in its pull request)
PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (S2.1: ADR-0011 accepted; no code). Gate S1 CLOSED
(Session 29).
LAUNCH CONTEXT: Al's direction of 2026-10-27, received in Session 35's conversation after PR 21
merged (quoted in substance): PR 21 is merged (`f8d71b8`); before ordering step 3, one more
package on a new branch from main — (1) a test that, with every outside file mutated as in the
existing input-set tests, the inventory's content outside the preservation object stays
byte-identical; (2) close ADR-0012 D7's gap with a separate freeze check in the gate, independent
of the S1.3 reproduction: every file STATE records as frozen byte-identical to its version at the
commit where it was frozen (for the S1 packets, the `gate-s1-registration` tag), every file STATE
records as append-only with both its freeze-point version and its default-branch version as exact
prefixes; derive both lists from STATE's records, list and ask where STATE gives no freeze point,
put both lists in the PR description, test both directions; (3) record it as an amendment to
ADR-0012, NEXT returns to ordering step 3 after this PR; gate the committed tree, fetch fresh
before pushing, open a PR and stop. Not cold; not an examination.
ORIENT (ADR-0008 D1 steps 1–5 by the preflight; receipt `orient-1-sync.json` under the external
root `C:\Users\LEET\Codex\Temp\syndeom-adr-0012-freeze-20261027\`):
- **Identity:** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main` (`git ls-remote --symref`).
- **Fetch and snapshot:** 2026-10-27T21:39:45Z, exit 0; the clean checkout on `main` at
  `6d89eb2f…` was **behind** by one commit, PR 21's merge `f8d71b8c8034638f14e9a5885ae348c8fdde24c4`
  (by Al, 2026-10-27T21:36:26Z; parents `6d89eb2f…` and `9dc0d1e6…`), and was fast-forwarded
  (ff-only) to it: `SNAPSHOT-ESTABLISHED`. Its tree `35bb1439…` equals PR 21's head tree, which
  Session 35 gated GREEN on the committed tree (`gate-committed.json`, 21:17:49Z–21:21:08Z); no
  separate ORIENT gate (the Sessions 30–33 convention).
- **Step 6:** STATE's records of what is frozen, append-only, unchanged, immutable or
  digest-bound (every line matching those words, read in context); ADR-0012; the tag's tree
  (466 files; `analysis/` holds seven `s1-*` packets); `git diff --stat gate-s1-registration HEAD`
  for `analysis/`, `groundtruth/`, `sources/`, the sweep, the advisory, ADR-0003 and FLAWS:
  **empty**; HANDOFF and SOURCES since the tag: no removed or changed line, the tag's bytes an
  exact prefix (476,214 → 568,823; 180,782 → 180,782); the tag's blobs for the sweep and the
  advisory carry exactly the digests STATE records (`f4bfb2ad…b03e`, `f3e40b68…fa2c`);
  `refs/remotes/origin/HEAD` → `refs/remotes/origin/main`. Files changed since the tag anywhere:
  twelve, all `PROGRAM/`, README, `CITATION.cff` and the ingest tool and tests.
ADVANCED:
- **A. The invariance test** (`engine/ingest/tests/input_set.rs`,
  `inventory_is_byte_identical_under_every_outside_mutation`): four worlds — every one of the 31
  outside files appended, flipped, rewritten (the two marker files to their marker alone) or
  removed (the two marker files kept), plus three additions outside `groundtruth/` — each yields
  a report byte-identical to the unmutated one, `preservation` included; a byte appended to the
  taxonomy still stops the report. `cargo fmt`, clippy at deny-warnings (after one type alias for
  the closure array) and the crate's tests (15 + 5 + 7) pass.
- **B. The freeze check:** `scripts/check_freeze.py` (register parser; `GitEnv` over one
  `cat-file --batch` process with memoized answers; `run_check` with injectable environment;
  CLI printing one line per row and `RESULT: ALL PASS`) and `scripts/check_freeze_test.py` (8
  unittest cases, in memory over the live register and the real objects). `scripts/check.sh`
  runs both after the register cross-reference check. `.github/workflows/check.yml`: full
  history and tags, then `git remote set-head origin -a`.
- **C. The register** (STATE, new section *Freeze register*): seven frozen rows at
  `gate-s1-registration` (296 files) and two append-only rows (HANDOFF, SOURCES), each citing the
  STATE records it derives from; an *Awaiting a freeze point* list with proposals; a *Not
  proposed* list with reasons. On the working tree: 9 rows, ALL PASS, about one second.
- **D. ADR-0012:** Status accepted (PR 21's merge; the amendment directed the same day);
  *Amendments* 1 (the test, the check, the lists, the two flagged choices, D7's second bullet
  superseded).
- **E. STATE:** the Updated bullet; Current build verification (Session 36); the ADR-numbers
  line; a new standing escalation (the freeze-register rulings); the S1.3-tool escalation
  (PR 21 merged; the register checks what the tool no longer does); NEXT's sentence on the
  unmerged pull request. NEXT stays at ordering step 3.
- **F. HANDOFF:** this block. **G. Gate** on the committed tree with a clean working tree,
  recorded in the pull request and under the external root, not in the tree it certifies.
- No register status, manifest, frozen packet, retained report, constitution byte, licence, tag,
  corpus file or S2 code changed; the held branch untouched.
BELIEF DELTAS:
- **A freeze is a claim STATE can make in a parsable form, and the gate can check it against Git
  objects.** The same shape as §4.1's "state files are claims; builds are evidence": the register
  says what froze and where, `git cat-file` says whether it is still so. Evidence: 9 rows, 296
  files, two prefixes, ALL PASS in about a second. Consequence: D7's gap closed without widening
  the reproduction.
- **Nothing the program calls frozen has moved since the tag.** Evidence: empty diffs under
  every candidate scope; HANDOFF only grown; SOURCES unchanged. Consequence: the register passes
  at `HEAD` with no exception of any kind.
- **The derivation surfaces what STATE never pinned.** The corpus, the ferried files, `sources/`
  and ADR-0003 are recorded as unchanged, immutable or digest-bound, but no record names the
  commit at which they froze. Consequence: the *Awaiting a freeze point* list and the escalation,
  as Al's rule requires, rather than a choice.
- **Memoizing Git answers is the difference between a test suite in the gate and one outside
  it.** Evidence: 40 s with a process per answer, 1.8 s memoized. Consequence: the eight tests
  run in every gate.
CONFUSIONS:
- **clippy's `type_complexity`** on an array of boxed-closure tuples; one type alias. Cost: one
  rerun.
- **The first freeze-test run took 40 s**, every `run_check` spawning its own `git ls-tree`,
  `ls-files`, `rev-parse` and `symbolic-ref`; memoization fixed it. Cost: one rerun. Doc fix:
  none; the checker documents the memo.
- **The append-only rows' freeze point.** Al's rule says to list and ask where STATE gives no
  freeze point; STATE records HANDOFF and SOURCES as append-only but names no commit. Leaving
  the append-only kind empty would have left the mechanism unexercised by the live register, so
  the rows take the tag and the choice is flagged in STATE, the ADR and the pull request. Recorded
  as a choice, not a derivation.
- **A frozen scope's additions.** Al's spec speaks of files being byte-identical; refusing
  additions under a frozen directory is the executor's reading of "frozen packet" (the former
  `preserve()` did the same). Flagged.
HONESTY CHECK:
- Register entries re-examined: none changed; the gate prints the seven CLAIM statuses and the
  73 source states as before. Downgrades: none.
- ADR-0012's Status moved to **accepted** on the strength of Al's merge of PR 21 and the direction
  to amend it; no sentence of Al's says "accepted". Flagged here; Al may word it otherwise.
- Every row of the register cites the STATE records it comes from; the two choices that are not
  derivations are named as such in three places. The file counts (121, 17, 35, 47, 46, 19, 11,
  296) and byte lengths are the tools' output, not hand sums.
- The invariance test asserts the whole report byte-identical, which is stronger than the
  property Al named (content outside the `preservation` object); the stronger property holds
  because `preserve()` derives its output from the baseline list, not from the mutated files.
LEAKAGE CHECK: checked, not assumed — `contact/` not opened; no detector, threshold or generator
code exists or was written; the inputs to this package are STATE's records, the tag's tree, the
tool's tests and Al's direction. Verdict: no leakage.
DETERMINISM CHECK: every number here is regenerable: the freeze check from `scripts/check_freeze.py`
on any checkout holding the tag and `origin/HEAD`; the tests from the gate; the diffs from `git`.
Nothing to quarantine.
MANIFEST CHECK: no artifact created or changed; "proven" applied to nothing; nothing synthetic
labeled real.
DRIFT CHECK:
- no statement about any real manuscript (SD3);
- no understanding claims (SD17);
- no detector chosen or tuned; no referee contact (SD13);
- no S2 code; the held branch untouched; the SD8 quarantine untouched.
NEXT-SINGLE-ACTION: STATE's NEXT — S2 ordering step 3, the held harvest's admission review,
after this pull request's merge; then step 4, ADR-0011's crate, after Al rules on the review.
ESCALATIONS:
- **Freeze register, for Al's rulings** (STATE, Standing escalations, first bullet): the scopes
  awaiting a freeze point, each with a proposal; the two executor choices (additions refused
  under frozen scopes; the tag as the append-only rows' freeze point). Both lists are in the pull
  request's description.
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction — stop at the pull request for review. Repo green: the gate on the
committed tree, recorded in the pull request.
**Memory writes:** none.
AL'S RULINGS ON PR 22 (2026-10-27, received in this session's conversation after PR #22 opened;
recorded at Al's direction in a second commit on the same branch, no force-push; transcribed
verbatim from Al's message; exact copy `owner-rulings-pr22-2026-10-27.txt` under the external
root, 1,214 bytes, SHA-256 `da23f9efa2d5549456aafc66f3bc7b710aa5bc9b42b4b68a8aadba70655b2bda`):

~~~text
Rulings on PR #22:

1. Accepted: the invariance test as written (the whole report byte-identical is stronger than I asked), the freeze checker and its tests, the CI changes, and the nine enforced rows.

2. Your two choices: accepted. A frozen scope also refuses additions and removals, and the two append-only rows use the gate-s1-registration tag as their freeze point.

3. The awaiting scopes, all at gate-s1-registration:
- groundtruth/: accepted as proposed. One frozen row per accepted item directory, so later prefixes can add new item directories; HARVEST-LOG.md and README.md append-only; derivations/ frozen.
- The sweep and the advisory note: frozen as proposed. Any later change or removal is then a deliberate register edit under a ruling.
- sources/: freeze each of the nine stored copies as its own file row, not the directory, so new sources can still be stored when a later prefix registers them.
- ADR-0003: append-only as proposed.
- The scopes you did not propose stay unchecked.

4. Record these rulings in ADR-0012's Amendments and the Freeze register, as a new commit on PR #22's branch (no force-push). Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

Before writing: `recheck --baseline f8d71b8c…` re-fetched at 2026-10-27T22:05:53Z (exit 0,
`TARGET-UNCHANGED`; receipt `recheck-2-before-rulings.json`). `main` was still `f8d71b8c…`, the
branch at `73a16eb` locally and on the remote, PR 22 open; the clean checkout was switched from
`main` back to the branch.

Effect:
- **Ruling 1** → the invariance test, the checker and its tests, the CI changes and the nine
  rows stand as committed.
- **Ruling 2** → the two choices are ruled: a frozen scope refuses additions and removals; the
  append-only rows' freeze point is the tag. The register's preamble now says so.
- **Ruling 3** → fifty rows join the register, all at `gate-s1-registration`, generated from the
  tag's tree by a script rather than typed: one frozen row per accepted item directory under
  `groundtruth/` (35 rows, 76 files; a later prefix adds new directories beside them),
  `groundtruth/derivations/` frozen, `groundtruth/HARVEST-LOG.md` and `groundtruth/README.md`
  append-only, the sweep and the advisory note frozen, each of the nine stored copies under
  `sources/` as its own frozen row with its register entry (S-0004, S-0005, S-0010..S-0016, read
  from SOURCES.md), ADR-0003 append-only. 59 rows: 54 frozen (384 files), 5 append-only; the
  check reports ALL PASS in about a second; the tests 8 of 8 in about three seconds (each
  working-tree file is now read once per process). The scopes not proposed stay unchecked by
  ruling, `groundtruth/amendment-map.json` among them.
- **Ruling 4** → recorded in ADR-0012 *Amendments* (the rulings verbatim and their effect), the
  Freeze register (rows and preamble), STATE (the Updated bullet, Current build verification,
  the ADR-numbers line, the standing escalation closed) and this addendum.

Nothing else changed: no register status, manifest, frozen packet, retained report,
constitution byte, licence, tag, corpus file or S2 code. The commit carrying this addendum is
gated on its committed tree; the result is recorded in the pull request and under the external
root (`gate-committed-2.json`), not in the tree it certifies. **Memory writes:** none.

---

SESSION: 2026-10-28 UTC (2026-10-27 America/New_York) (Session 37 — a session launched from
`C:\Users\LEET`, outside the checkout, with the §6 Session-N resume prompt verbatim followed by
an owner note; SD15 disclosure first, by digest (Al's ruling, ADR-0005 *Amendments* 1); not an
examination — the owner messages carry facts about this repository, and Gate S2 has nothing to
examine; examined fetched `origin/main` `c48705c2d23183340eef5c01ff13059905513985`, tree
`a6b256840d764818c6d314259883967e0e10c7a4`, in the worktree `C:\Users\LEET\Syndeom-wt-2026-10-27`
(the original checkout `C:\Users\LEET\Syndeom` was behind and its fast-forward was refused; the
disclosure was carried here per ADR-0008 D3); branch `claude/s2-held-harvest-review`; header
completed after ADR-0008 step 5 — it was written provisionally with the disclosure, which is
unchanged; the session-end SHA is the commit carrying this block, recorded in its pull request)

### Entry 1 — SD15 memory disclosure, by digest (Al's ruling of 2026-10-27, amending ADR-0005's procedure)

Made before any other repository read except `PROGRAM/SYNDEOM-PROGRAM.md` and
`PROGRAM/DECISIONS/ADR-0005.md`, both read in full. No git command, no build, no listing of the
checkout's directories, and no read of this file preceded it. Appended by shell
(`tr -d '\r' … >>`), not by a file-editing tool.

**Memory in context.** The harness loaded one memory: the index
`C:\Users\LEET\.claude\projects\C--Users-LEET\memory\MEMORY.md`. The working directory was
`C:\Users\LEET`, which is not a git work tree, so no gitStatus snapshot was attached. This session
read two memory files from disk: the index and this program's own memory file
`project_syndeom.md`. It read no other memory file. The harness-loaded copy of the index was
compared with the file on disk by inspection only; it cannot be hashed.

| File | SHA-256 | Bytes | Lines | CR bytes | BOM | Final byte | Non-ASCII bytes |
|---|---|---|---|---|---|---|---|
| `MEMORY.md` | `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69` | 15420 | 110 | 0 | none (`23 20 50`) | `0a` | 334 |
| `project_syndeom.md` | `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0` | 10957 | 135 | 0 | none (`2d 2d 2d`) | `0a` | 71 |

Lines = `wc -l` = awk `NR`. Both digests were computed twice this session, at the interrupted
first attempt and again immediately before this append, and the two results were identical.

**Check (ADR-0005 D3).** The whole index was searched case-insensitively with awk over
`tolower($0)` for `syndeom|kober|friedman|fr13dman`. A control search for `project memory`
matched line 1, which shows the search runs. Matched lines: **85, 86, 88, 89, 106, 107** (6).
**Withheld by count: 104 lines**, namely 1–84, 87, 90–105 and 108–110; none of them names this
program or a sibling.

**Content not reproduced again.** Under Al's ruling, both digests match a disclosure already
recorded in this file that reproduced the same bytes. So that this disclosure comes before any read
of this file, the earlier entry is located and cited in Entry 2 of this block by a digest search
made after this append. If either digest is not found there, verbatim reproduction is owed, and the
session stops and escalates to Al under the ruling.

**Program-state check (SD15, D4).** Both files were read in full this session. Every entry in
`project_syndeom.md` is an environment or tooling hazard or the repo location. None records phase,
gate status, CLAIM or FLAW status, file contents, commit SHAs as state markers, next actions or
escalations. The six matched index lines point to the repo and to the siblings' repos, and state
that no program state is kept in memory.

**Launch context.** The launching prompt was the §6 Session-N resume prompt, followed by an owner
note about an earlier aborted launch. The first attempt at this disclosure was stopped by a safety
classifier partway through a response, before anything was appended. A second owner message then
gave the by-digest ruling. Both owner messages are recorded verbatim later in this block. Because
the launching context went beyond the §6 prompt verbatim, and the ruling states a fact about this
file's contents, **this session claims no cold-start exam status** unless Al rules otherwise. No
memory is written this session.

### Entry 2 — the disclosure's citation, verified after the append (Al's ruling; ADR-0005 *Amendments* 1)

- **Cited entry:** **HANDOFF Session 35, Entry 1** (lines 6664–6818 of this file at `c48705c`).
  It reproduced D1(a), the six index lines, at lines 6672–6677, and D1(b), `project_syndeom.md`
  in full, at lines 6683–6817. It records the index digest `cd943842…fc69` at line 6668 and the
  memory file's digest `ce2ce6de…2cc0` at line 6680.
- **Verified by bytes, not only by stated digests.** The fenced copy at lines 6683–6817 hashes to
  `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, the file's digest today.
  Lines 6672–6677 are byte-identical to index lines 85, 86, 88, 89, 106 and 107 as extracted
  today; both hash to `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`.
  Sessions 29 (lines 5134, 5157) and 34 (lines 6288, 6300) record the same two digests.
  **The ruling's premise holds for both files; no digest is new; nothing is owed verbatim.**
- **Where the disclosure was made.** It was first appended in `C:\Users\LEET\Syndeom` (on `main`
  at `f8d71b8`, behind fetched `origin/main` by 3) at 2026-10-28T01:17:35Z, from the file's
  mtime. That is the disclosure time. A fast-forward was refused because the incoming commits
  touch this file, so the preflight created the worktree `C:\Users\LEET\Syndeom-wt-2026-10-27` at
  the fetched tip. The disclosure was carried into it as the first action there (ADR-0008 D3), at
  about 01:18:49Z, under a header marked as carried. The original append stands uncommitted in
  the original checkout and is left as it stands (Escalations).
- **Byte identity of the two copies.** The disclosure body, from Entry 1's heading to its end,
  is 3,387 bytes with SHA-256 `1047d0598b90f9ab32e156b8cff11f338a4d634e1b2f3a01c30a31af6f56d400`,
  identical in both checkouts. The original append was 3,497 bytes (`42206164…4c11`, provisional
  header included); the carried append was 3,565 bytes. Copies are kept under
  `C:\Users\LEET\Codex\Temp\syndeom-held-harvest-review-20261028\`.
- **Dating.** Entry 1 calls the ruling "of 2026-10-27", its America/New_York date. Al's message
  arrived at about 2026-10-28T01:1xZ UTC.

### Entry 3 — the launches of 2026-10-27, recorded at Al's direction

**The aborted launch.** It is recorded as Al's note reports it; this session has no other record
of it. An earlier launch on 2026-10-27 stopped before making its disclosure, after a safety
classifier cut off its response. It wrote nothing to the repository. It began on Claude Fable 5.1
and ended on Claude Opus 5.5. It is not a session of this log and holds no number.

**This session's interrupted first attempt.** Session 37's first response did the following:
1. It read `PROGRAM/SYNDEOM-PROGRAM.md` and `PROGRAM/DECISIONS/ADR-0005.md` in full.
2. It read both memory files in full: `MEMORY.md` and `project_syndeom.md`.
3. It wrote `memcheck.sh` in the session scratchpad and ran it. The script computed the two
   digests, line counts, CR and BOM bytes and the D3 name search, and listed the memory
   directory's file names. It read no other memory file.
4. A safety classifier stopped the response before anything was appended. Nothing was written to
   the repository, and nothing of the checkout was read.

The executor reported this and stopped. The withheld content was not regenerated in any form. Al
then gave the ruling below. The disclosure in Entry 1 was made after it, by digest. The digests
were computed again immediately before the append and were identical to the first attempt's.

**Launching prompt.** Its first 13 lines are byte-identical to the constitution's §6 Session-N
block. That was checked by `cmp` after the disclosure, against the lines extracted from the
snapshot's `PROGRAM/SYNDEOM-PROGRAM.md`. An owner note followed them. The exact copy of the whole
prompt is `launch-prompt-as-received.txt` under the external root (1,463 bytes, SHA-256
`c6977e523cd6de36634e95bc2b3e4bf039948036d601de5f1be2028f7b3a5d1b`). The note, verbatim:

~~~text
Owner note, to act on only after you have disclosed and oriented: an earlier launch today stopped before making its disclosure, after a safety classifier cut off its response. It wrote nothing to the repository; it began on Fable 5.1 and ended on Opus 5.5. Record it in this session's HANDOFF block as an aborted launch.
~~~

### Entry 4 — Al's ruling on the disclosure, verbatim (also ADR-0005 *Amendments* 1)

The ruling was received after the interrupted first attempt and transcribed verbatim from Al's
message. The exact copy is `owner-ruling-disclosure-by-digest.txt` under the external root
`C:\Users\LEET\Codex\Temp\syndeom-held-harvest-review-20261028\` (1,064 bytes, SHA-256
`cf5c416d161ce431968f58233b0bbfca5e766361dfa756de606e92f0e69b2c81`).

~~~text
Ruling on the disclosure (amends ADR-0005's procedure; record it there and in this session's HANDOFF block):

Both memory digests you computed already appear in HANDOFF, in disclosures that reproduced these exact bytes. When every memory file's SHA-256 matches a disclosure already recorded in HANDOFF, the disclosure is by digest: record each file's SHA-256, line count and byte facts, the matched index line numbers and the withheld count, and cite the earlier HANDOFF entry that reproduced the same bytes. Do not reproduce the content again. Verbatim reproduction is required only when a digest is new; if that reproduction cannot be completed, stop and escalate to me rather than working around it.

Proceed in this session: make the disclosure by digest as the first entry of a new HANDOFF block, by shell append as before, then orient and continue to STATE's NEXT. Record this ruling verbatim in ADR-0005's Amendments and in your HANDOFF block, together with the earlier aborted launch from my previous note and this session's own interrupted first attempt.
~~~

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (S2.1: ADR-0011 accepted; no code). Gate S1 CLOSED
(Session 29).
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipts under the external root):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-28T01:17:49Z, exit 0: `f8d71b8..c48705c main -> origin/main`.
- **Snapshot.**
  - The checkout was on `main` at `f8d71b8c…`, **behind by 3**: `73a16eb`, `fd8fa9e`, and PR 22's
    merge `c48705c`, made by Al at 2026-10-27T23:06:37Z.
  - The working-tree change was the disclosure append only (`allowed_present`;
    `unrelated: []`).
  - `--sync` exited 3 and changed nothing. The incoming commits touch `PROGRAM/HANDOFF.md`.
  - `--worktree C:/Users/LEET/Syndeom-wt-2026-10-27` exited 0 (`SNAPSHOT-ESTABLISHED-IN-WORKTREE`).
    The worktree's HEAD is `c48705c…`, its tree `a6b2568…`, and it was clean.
  - Receipts: `orient-1.json`, `orient-2-sync.json`, `orient-3-worktree.json`.
- **Step-0 bytes.** The constitution, ADR-0005 and ADR-0008, as read before and just after the
  disclosure, are byte-identical to the snapshot's copies:
  - constitution `110f5360…479c`, 38,958 B, ADR-0009's amended digest;
  - ADR-0005 `873f0751…7595`;
  - ADR-0008 `ccdfa9d7…e9ff`.
- **Step 6, reconstruction.** STATE (Updated bullet, gate tables, Standing escalations, NEXT),
  then HANDOFF Session 36, then CLAIMS (0001–0005 VERIFIED, 0006 CONDITIONAL, 0007 VERIFIED
  existence only), then FLAWS `0.1.0`, then ADR-0003 D1–D9 and ADR-0008. They agree with one
  another and with the gate receipt's register counts.
  - No separate ORIENT gate was run. The fetched tree is PR 22's head tree, which Session 36 gated
    GREEN (`gate-committed-2.json`, 22:10:04Z–22:19:22Z). Its raw log was re-hashed and equals
    `0ac3de25…739e`.
- **NEXT survived ORIENT.** It was S2 ordering step 3, and its condition (PR 22 merged) now holds.
ADVANCED:
- **A. The held harvest review.** It is `analysis/s1-held-harvest-review/`, holding `REVIEW.md`,
  `receipt.json`, `evidence-reverify.json` and `verify_held_evidence.py`.
  - Session 22's preserved evidence: 147 of 147 files re-hash to the committed manifest
    (`ALL MATCH`).
  - The five manifests' pins equal the preserved originals' full digests.
  - The candidate list is run 1's (`e164c0b1…2d48`, HANDOFF line 1464): rows 28–30 in order,
    nothing skipped, every correcting version in the window.
  - Every record's claim of record was located in the preserved TeX by label and by counting the
    numbered environments on the paper's own counter.
  - Every notice sentence was found, five of them against page or TeX bytes and the rest against
    the receipts' transcriptions.
  - **Five items, five agree, no disagreement.**
  - **Questions Q-H1..Q-H7 for Al.** Q-H1 couples item 32's truth reading to its class.
  - **The integration plan** is REVIEW.md §4. It adds two findings to Session 22's conditions:
    the log's append-only conflict and the Session 22 label.
- **B. ADR-0005 *Amendments* 1.** Al's disclosure ruling, verbatim, with its effect on D1–D4 and
  what it does not touch.
- **C. STATE.** Changed: the Updated bullet, Current build verification, the Memory policy
  paragraph, the ADR-numbers line, the freeze-register escalation (PR 22 merged), three new
  standing escalations, the held-harvest and older-questions bullets, and NEXT.
- **D. HANDOFF.** This block. Entries 1 and 2 were appended by shell, and the header was completed
  after step 5.
- **E. Build products.** Copied into the worktree for the gate, untracked: `lean/.lake` (144,221
  files, 7.2 GB) and `engine/target` (2,886 files), both by robocopy with 0 failed. Mathlib's
  `HEAD` is `d13f23b7…`, at the pin.
- Nothing changed in any register status, manifest, frozen packet, retained report, constitution
  byte, licence, tag, corpus file or the held branch. No S2 code was written.
BELIEF DELTAS:
- **The held branch's content stands up; its obstacles are mechanical.** All five readings agree
  with the bytes. What blocks a merge is numbering, regeneration and the append-only log. The
  last of these is new since Session 22: the freeze register (PR 22) makes the branch's mid-table
  log rows a gate failure (they diverge from the tag at offset 41,721).
- **One label depends on another.** FLAW-0012 requires a false statement, so item 32's class
  stands or falls with its truth reading. Q-H1 is therefore a class question as well.
- **Disclosure by digest fits a stable memory, not a moving one.** The index is machine-wide, and
  any other program's session can change its digest. Recorded for Al, not acted on.
CONFUSIONS:
- **The provisional header's date.** It carried the America/New_York date (2026-10-27).
  ADR-0008 D1 asks for the UTC date, marked provisional. The completed header gives both dates.
  Cost: none, since the header is not part of the disclosure.
- **My manifest summary read the wrong key.** It used a top-level `files` where the pin is
  `artifact.files.informal`, and printed "CHECK" for all five. It was rerun with the right key
  over the full digests: five MATCH. Cost: one rerun.
- **The awk backslash hazard struck twice.** Patterns written as `\\section` matched nothing, and
  environment macros (`\bP`, `\bL`) hid the counter. Both counts were redone, the first with
  bracket expressions and the second in Python using `chr(92)`. Cost: three reruns. Doc fix: none
  needed; the hazard is already in the executor's memory file.
- **robocopy exit 1 means "files copied"**, so the background task reported it as a failure. The
  log shows 0 failed. Cost: one read.
- **The later paper's quote.** "absence of a simple completion" did not match the TeX until the
  `\emph{}` markup was allowed for. The quote stands.
HONESTY CHECK:
- Register entries re-examined: none changed, and nothing was downgraded. The review approves
  nothing, and the branch's labels stay unapproved.
- **Limits, stated in REVIEW.md:**
  - six notice comments were checked against receipt transcriptions, not page bytes;
  - the records' precedent lists (main's items 13, 16, 21, 24, 26, 27, 29) were not re-checked;
  - the class-fit note on item 36 is a reading, not a relabeling;
  - the arXiv pages were not re-fetched, so the review is a statement about the preserved bytes,
    not about the live pages.
- Entry 2's citation was verified by hashing Session 35's reproduced bytes, not only by matching
  its stated digests.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened. No detector, threshold or
generator design was touched. The inputs were the held branch, Session 22's preserved evidence,
ADR-0003, FLAWS.md `0.1.0` and Al's messages. Verdict: no leakage.
DETERMINISM CHECK:
- The re-verification is regenerable on this machine from `verify_held_evidence.py`, the committed
  manifest and the external evidence root. That root is outside the repository, as the held
  evidence always was.
- The review's line numbers are regenerable from the preserved TeX.
- No MEASURED result was produced, and nothing entered CLAIMS. Nothing to quarantine.
MANIFEST CHECK: no artifact was created or changed and no manifest was written. "Proven" was
applied to nothing, and nothing synthetic was labeled real.
DRIFT CHECK:
- The review reads notices and texts and states agreement with the branch's records. It makes no
  verdict on any paper (SD3), and REVIEW.md says so.
- No understanding claims (SD17).
- No detector chosen or tuned, and no referee contact (SD13).
- No S2 code, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. Act on Al's rulings on the held harvest review (Q-H1..Q-H7):
- the integration per REVIEW.md §4 if Al orders it;
- otherwise S2 ordering step 4, ADR-0011's crate;
- with no ruling, record that and stop.
ESCALATIONS:
- **Held harvest review**, Q-H1..Q-H7 (STATE, Standing escalations, first bullet;
  REVIEW.md §3–§4).
- **SD15's constitution sentence** against the by-digest ruling: a constitution amendment, or
  ADR-0005 *Amendments* 1 as an addendum? Also the index-digest observation.
- **Housekeeping** after this pull request merges, on approval: the original checkout's
  uncommitted append and the worktree `C:\Users\LEET\Syndeom-wt-2026-10-27`.
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. NEXT's step 4 is an escalation to Al, and the next action needs
Al's rulings. Repo green: the gate on the committed tree is recorded in the pull request.
**Memory writes:** none, deliberately. Any write would change a memory file's digest and make the
next disclosure owe verbatim reproduction.
AL'S RULINGS ON PR 23 (2026-10-28 UTC, received in this session's conversation after PR #23
opened; recorded at Al's direction in a third commit on the same branch, no force-push;
transcribed verbatim from Al's message; exact copy `owner-rulings-pr23.txt` under the external
root, 1,854 bytes, SHA-256 `0047396b604535ab17a4a2b0ddaa7dfa74b027d63c8ca2d69f03463aded8449b`):

~~~text
Rulings on PR #23 (held harvest review):

Q-H1, item 32: false by source document, as recorded, so its recorded class stands. "The wrong theorem" belongs to the wording group main's items were read under; changing that standard would be a corpus-wide decision, not a one-item exception.
Q-H2 and Q-H3, items 34 and 35: gap, as recorded, by the entry-range practice main's items follow.
Q-H4: the unnamed results stay out as items and join the existing unnamed-result question.
Q-H5, item 36: undetermined, as recorded. "Overcome" speaks to the later paper's reworked concepts, not to Proposition 2.22 as stated.
Q-H6: integrate now, as the first later prefix, following section 4's plan.
Q-H7: the harvest log continues as a new dated section after the file's current end, with its own table header, rows and counts, keeping the tag's bytes as an exact prefix. The five new item directories get frozen rows, added in a second commit on the integration branch with the commit that adds the directories as their freeze point. Pull requests keep merging with merge commits, never squash, so that freeze point stays in main's history.
SD15 wording: ADR-0005's amendment stands as an addendum; the constitution is not edited, since every session reads ADR-0005 in full before disclosing.
Disclosure by digest, refined: compare the digest of the matched index lines and of the SYNDEOM memory file, not of the whole index. Record the whole index's digest as a fact; verbatim reproduction is owed only when the matched lines or the SYNDEOM file change.

Record these rulings verbatim where the program records rulings (STATE, HANDOFF, and ADR-0005 for the digest refinement), as a new commit on PR #23's branch (no force-push). Run the gate on the committed tree, fetch fresh before pushing, and stop for my review. The housekeeping waits until after the merge.
~~~

Before writing: `recheck --baseline c48705c2…` re-fetched at 2026-10-28T04:13:50Z (exit 0,
`TARGET-UNCHANGED`; receipt `recheck-3-rulings-start.json`). `main` was still `c48705c2…`. The
branch was at `a5144ce` locally and on the remote, and the worktree was clean. PR 23 was open
and mergeable, with no reviews or comments. The held branch is still at `c3a3d36` on the remote.

Effect:
- **Q-H1, Q-H2/Q-H3, Q-H5** → the five items' labels stand as recorded: item 32 false by source
  document (FLAW-0012), items 34 and 35 `gap`, item 36 `undetermined`. Nothing was relabeled.
- **Q-H4** → the further unnamed results stay out as items: Corollary 12 and the demoted
  Theorem 3 of 1403.6317, Lemma 4.2 of 1403.8075, and Lemmas 2.9 and 2.21 of 1404.4599. STATE's
  *Carried questions* and *Older questions* bullets now list them under the unnamed-result
  question. The frozen S1.4 packet, which records that question's disposition, is not touched.
- **Q-H6, Q-H7** → STATE's NEXT is the integration, after this pull request merges. It follows
  REVIEW.md §4 on a fresh branch from `main`, continues the harvest log as a new dated section,
  and then adds five frozen rows in a second commit, with the first commit as their freeze point.
  The merge-commit rule joins the *Freeze register* preamble. Nothing was integrated, and no row
  was added.
- **SD15 wording, the refinement** → ADR-0005 *Amendments* 2: the two paragraphs verbatim, their
  effect, and the baseline below. STATE's *Memory policy* paragraph and the SD15 escalation are
  updated, and that escalation is closed.
- **The instruction paragraph** → recorded in this addendum, in STATE and in ADR-0005.
  - STATE: *Standing rulings* (the message verbatim), the Updated bullet, Current build
    verification, the ADR-numbers line, *Standing escalations*, the held-harvest,
    carried-questions and older-questions bullets, the *Freeze register* preamble, and NEXT.
  - The housekeeping is not done.

Baseline under the refined rule (ADR-0005 *Amendments* 2). Computed from the files on disk after
the rulings; hashes and counts only, nothing reproduced.
- Matched index lines 85, 86, 88, 89, 106 and 107: 6 lines, 1,198 bytes, SHA-256
  `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`, equal to HANDOFF lines
  6672–6677 (Session 35, Entry 1).
- `project_syndeom.md`: `ce2ce6de…2cc0`, equal to HANDOFF lines 6683–6817 (the same entry).
- The whole index, a fact: `cd943842…fc69`, 15,420 bytes, 110 lines, unchanged since Entry 1. The
  name search matched the same six lines, and the canary matched line 1.

Flagged for Al: two readings of the refinement are the executor's, recorded in ADR-0005
*Amendments* 2.
- The matched lines' digest is taken over each matched line with its LF, in index order. That is
  the form Entry 2 verified against Session 35's reproduction.
- A renumbering of the matched lines alone, with their bytes unchanged, is not a change.

NEXT-SINGLE-ACTION (superseding the block's): STATE's NEXT. After PR 23 merges, integrate the
held harvest per REVIEW.md §4 and Al's rulings, in two commits; then S2 ordering step 4.
ESCALATIONS (superseding the block's):
- **Housekeeping** after this pull request merges: the original checkout's uncommitted append and
  the worktree (Al: "The housekeeping waits until after the merge.").
- The two readings of the refinement above, for Al's confirmation.
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction — stop for review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root (`gate-committed-2.json`), not in the
tree it certifies. Checks for this addendum: no leakage (`contact/` not opened; no detector,
threshold or generator touched); nothing measured; no artifact or manifest written; no verdict on
any paper (SD3). **Memory writes:** none.

---

SESSION: 2026-10-28 UTC (2026-10-28 America/New_York) (Session 38 — a session launched from
`C:\Users\LEET`, outside the checkout, with the §6 Session-N resume prompt verbatim followed by
an owner note; SD15 disclosure first, by digest (ADR-0005 *Amendments* 1 and 2); not an
examination — the owner note carries program state, and Gate S2 has nothing to examine;
examined fetched `origin/main` `a7939547b9d103baa3fd0474149e476e3e0b0dcb`, tree
`bfa2ce236bdbf775a8485ec9e9b24b9db08ad5db`, in the original checkout `C:\Users\LEET\Syndeom` on
`main`, synchronized; branch `claude/s1-held-harvest-integration`; header completed after
ADR-0008 step 5 — it was written provisionally with the disclosure, which is unchanged; the
session-end SHA is the commit carrying this block, recorded in its pull request)

### Entry 1 — SD15 memory disclosure (by digest; ADR-0005 D1–D4 as amended by Amendments 1 and 2)

Made before any other action except reading `PROGRAM/SYNDEOM-PROGRAM.md` and
`PROGRAM/DECISIONS/ADR-0005.md` in full and reading and hashing the memory below. No other
repository file, directory listing, Git history or status, or build was read or run first.
Appended by shell append (`cat <scratch file> >> PROGRAM/HANDOFF.md`) without reading HANDOFF.

**Launch and coldness.** Claude Code, model Opus 5.5 (`claude-opus-5-5`), launched in
`C:\Users\LEET` (not a git work tree, so no gitStatus snapshot was attached), memory loaded from
the `C--Users-LEET` index. The launching prompt was the §6 Session-N resume prompt verbatim,
followed by an owner note that names program state (a PR merge, an earlier session's
conversation, the path of a housekeeping receipt). Under SD15's launching-prompt corollary this
session is therefore **not cold**. It attempts no gate's cold-start test and nothing in it counts
as an exam. The disclosure is made because SD15 (as amended by ADR-0009) requires it of every
session launched with that prompt.

**Recalled memory loaded by the harness.** One file, the memory index `MEMORY.md`. No other memory
file was loaded by the harness. `project_syndeom.md` was read by the executor for this disclosure
(D1 (b)).

**The whole index, as a fact (Amendment 2).**
`C:\Users\LEET\.claude\projects\C--Users-LEET\memory\MEMORY.md`: SHA-256
`cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`, 110 lines, 15,420 bytes,
0 CR, 334 non-ASCII bytes, final byte LF.

**Name search (D3).** Case-insensitive search of every index line for `syndeom`, `kober` and
`friedman`, by awk `tolower` + `index` (not `grep -i`, a machine hazard). Matched lines: 85, 86,
88, 89, 106, 107 (exit 0). Canary: 41 index lines match `darq`, so the search ran over the whole
file. Supplementary search for `fr13dman`: lines 88, 89, 107, all inside the matched set.

**Withheld by count (D2).** 104 lines: 1–84, 87, 90–105, 108–110. None matches the search above.

**Matched index lines, by digest (D1 (a), Amendment 2).** Lines 85, 86, 88, 89, 106, 107, each with
its LF terminator, concatenated in index order: 6 lines, 1,198 bytes, 0 CR, 30 non-ASCII bytes,
SHA-256 `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`.

**The SYNDEOM memory file, by digest (D1 (b), Amendment 2).**
`C:\Users\LEET\.claude\projects\C--Users-LEET\memory\project_syndeom.md`: SHA-256
`ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, 135 lines, 10,957 bytes,
0 CR, 71 non-ASCII bytes, final byte LF. Read in full this session.

**Other recalled memory naming this program (D1 (c)).** None loaded.

**Citation.** Both digests equal the baseline recorded in ADR-0005 Amendment 2 (computed in
Session 37 after the PR 23 rulings). That baseline states that they equal the digests of HANDOFF
lines 6672–6677 and 6683–6817, which Session 35's Entry 1 reproduced. The whole index's digest is
also unchanged from that baseline. The disclosure is therefore by digest and reproduces no
content. At the time of this append the citation rests on ADR-0005's statement, not on HANDOFF
itself. Entry 2, written after this append so that the disclosure stays first, verifies it by
hashing those HANDOFF lines.

**D4.** On its face the SYNDEOM memory file holds the repo location and machine and harness
hazards only: no phase, gate, claim or flaw status, commit used as a state marker, next action or
escalation. Because this session is not cold, D4 decides no exam here.

No memory was written before this entry.

### Entry 2 — the disclosure's citation, verified after the append (ADR-0005 *Amendments* 1 and 2)

- **Cited entry:** **HANDOFF Session 35, Entry 1** (its heading at line 6664 of this file at
  `a793954`). It reproduced D1(a), the six matched index lines, at lines 6672–6677, and D1(b),
  `project_syndeom.md` in full, at lines 6683–6817.
- **Verified by bytes, not only by stated digests.** Extracted from `a793954`'s HANDOFF blob:
  - lines 6672–6677 hash to `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c` and
    are byte-identical (`cmp`) to index lines 85, 86, 88, 89, 106 and 107 as extracted this
    session;
  - lines 6683–6817 hash to `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0` and
    are byte-identical (`cmp`) to `project_syndeom.md` on disk.
  **Neither digest is new, so nothing is owed verbatim** (Amendment 2). The renumbering reading
  flagged in Amendment 2 did not arise: the matched lines keep their numbers.
- **The append.** The scratch file holding Entry 1 and its provisional header was 3,746 bytes,
  0 CR, no BOM, SHA-256 `d1f563f2…72c3`, appended by `cat … >>` before 2026-10-28T04:56:31Z (the clock
  read by the next command). After
  the append, HEAD's HANDOFF blob followed by the scratch file hashed to the working file's digest
  (`863032a5…394b`): a pure append. The provisional header line was then replaced by the completed
  header above, by a script that asserted the disclosure body (3,642 bytes, `7432b96a…310f`) was
  unchanged.
- **Constitution and ADR bytes (ADR-0008 step 5).** As read in step 0 and as in the snapshot:
  the constitution `110f5360…479c` (38,958 B, ADR-0009's amended digest), ADR-0005 `b3da80df…f8cb`
  (12,382 B, with *Amendments* 2), and ADR-0008 `ccdfa9d7…e9ff`. Working-tree blobs equal HEAD's.

### Entry 3 — the launch, and the housekeeping verified from its receipt (owner note)

**Launching prompt.** Its first 13 lines are byte-identical (`cmp`) to the constitution's §6
Session-N block, lines 481–493 of the snapshot's `PROGRAM/SYNDEOM-PROGRAM.md`. An owner note
followed them. The copy as received is `launch-prompt-as-received.txt` under the external root
`C:\Users\LEET\Codex\Temp\syndeom-orient-20261028-claude-opus\` (1,645 bytes, SHA-256
`a4b7ef85…becb`); it was written from the prompt as rendered to the executor, so it is a
transcription. The note, verbatim:

~~~text
Owner note, to act on only after you have disclosed and oriented: the housekeeping STATE lists for the first session after PR #23's merge was already carried out in Session 37's conversation after the merge. Verify the result yourself (no extra worktree; the original checkout on the default branch with nothing modified except your own disclosure append), then record it from the receipt at C:\Users\LEET\Codex\Temp\syndeom-held-harvest-review-20261028\housekeeping-receipt.json instead of redoing it.
~~~

**What was verified, and how** (nothing was redone):
- The receipt `housekeeping-receipt.json` (4,696 bytes, `520163ba…d9af`, schema
  `syndeom-housekeeping-receipt/1`, 2026-10-28T04:46:50Z–04:49:10Z) and its log
  `housekeeping.log` (172 lines, `85dabb3f…b817`, equal to the receipt's `log_sha256`).
- **No extra worktree:** `git worktree list --porcelain` shows only `C:/Users/LEET/Syndeom`, and
  the directory `C:\Users\LEET\Syndeom-wt-2026-10-27` does not exist.
- **The original checkout:** on `main` at `a7939547…`, the fetched tip (preflight: synchronized,
  ahead 0, behind 0); `git status --porcelain --untracked-files=all` lists only
  `PROGRAM/HANDOFF.md`, whose change is this session's append; no stash; `lean/.lake` and
  `engine/target` present.
- **PR 23:** `gh pr view 23` — `MERGED`, 2026-10-28T04:45:54Z, by `Quigles1337`, merge commit
  `a7939547…`, head `97fc1979…`. `a793954` has two parents (`c48705c`, `97fc197`), and its tree
  `bfa2ce23…` equals `97fc197`'s.
- **The discarded append:** the retained `disclosure.lf.md` (3,497 bytes, `42206164…4c11`) is the
  110-byte provisional header plus `disclosure-body.lf.md` (3,387 bytes, `1047d059…d400`). That
  body occurs exactly once in `a793954`'s HANDOFF blob, at byte 583,989, line 7198 (Session 37,
  Entry 1).
- The receipt's other statements (the fast-forward of 7 commits, the worktree's clean status and
  its HEAD being an ancestor of `origin/main`) are recorded from the receipt and its log. The
  live state they produced is what was checked above.

### Entry 4 — Al's ruling on the inventory event's scope (Session 38)

After orienting, and before any integration commit, the executor asked Al through the harness's
question tool how to scope REVIEW.md §4 step 7, "a new inventory snapshot under its own event"
(ADR-0012 D7: "its own snapshot and tool revision"). The pinned effective-inventory tool
hard-codes 35 / 32 and refuses additions, so the snapshot needs new engine code, and it can run
only on the committed integration tree, which does not fit inside Q-H7's two commits. The
question, the three options and the answer are copied exactly in
`owner-ruling-inventory-scope.txt` under the external root (1,460 bytes, SHA-256
`d4f1b6f1d3a5a0cc7c433954e57fb5114594999ed0f511858bc186f80c690180`). Al chose the first option:

~~~text
Separate PR next (Recommended) — This PR is the integration as ruled: commit 1 holds items, SOURCES 0074–0080, records, the log section and STATE; commit 2 adds the five freeze rows. STATE records 40/37 as committed but not yet measured, and notes the pinned tool refuses at HEAD as D7 predicts. The inventory event (new tool revision + coverage run) becomes NEXT, in its own PR, before S2 step 4.
~~~

Recorded in STATE (*Standing rulings*, *Standing escalations*, NEXT).

### Entry 5 — commit 2, the five frozen rows (Q-H7)

Commit 1 is `d2569d61588368a60bbf7558fb9f0703f96e3a5b` (tree `1e2fafad…9bd7`), the commit that
adds the five item directories. The second commit adds five frozen rows to STATE's *Freeze
register*, one per new directory, each with that full id as its freeze point. It also amends the
preamble sentence so that it names the exception, and gives commit 1's id in the Updated bullet.
On the working tree before commit 2, `check_freeze.py` resolved the freeze point for all five
rows: 2 files each, byte-identical, 0 added. The register has **64 rows (59 frozen, 5
append-only): ALL PASS**, and `check_freeze_test.py` passes 8 of 8.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (S2.1: ADR-0011 accepted; no code). Gate S1 CLOSED
(Session 29). This session's work is S1 corpus maintenance (the first later prefix, ADR-0003 D8),
which Al ordered ahead of S2 ordering step 4.
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipt `orient-receipt.json`, SHA-256
`6980008e…cfa8`, under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028-claude-opus\`):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-28T04:56:37Z–04:56:38Z, exit 0.
- **Snapshot.** `main` at `a7939547…`, tree `bfa2ce23…`, **synchronized**: ahead 0, behind 0. The
  only change was the disclosure append (`allowed_present`; `unrelated: []`). No worktree, as the
  owner note expected.
- **Step 6.** STATE (the Updated bullet, the gate tables, Standing rulings and escalations, NEXT),
  then HANDOFF Session 37 in full, then CLAIMS (0001–0005 VERIFIED, 0006 CONDITIONAL, 0007
  VERIFIED existence only), then FLAWS `0.1.0` (14 entries), then ADR-0008, ADR-0012 D7 and
  REVIEW.md. They agree with one another.
- **ORIENT gate GREEN** on this checkout, 04:58:53Z–05:07:50Z (STATE, *Current build
  verification*).
- **NEXT survived ORIENT.** Its condition held: PR 23 was merged with a merge commit on fetched
  `origin/main`.
ADVANCED:
- **The held harvest is integrated** as the first later prefix, on `claude/s1-held-harvest-integration`:
  - commit 1, `d2569d6`: five item directories, S-0074..S-0080, the log section, STATE and this
    block's Entries 1–4;
  - commit 2: the five frozen rows and this block's close.
- **Provenance proved, not assumed.** Each preserved Session-22 config equals its held manifest's
  `config` and `notes`. Each regenerated manifest equals its held manifest up to the renumbering,
  the session label, the id, the config hash and the engine commit. The originals' pins
  (`97b28cf2…`, `c171e338…9270`, `2e326a33…`) are unchanged, and a rerun is byte-identical.
- **New ids:** item 32 `34988f7c8306`, 33 `915251f36fc5`, 34 `bca249b3fa72`, 35 `c6179869edf7`,
  36 `11ffe4f87954`. Manifest SHA-256 prefixes: `01913586`, `9584cacd`, `f7470f85`, `23d202e5`,
  `fb270361`.
- **Append-only files:** SOURCES 180,782 → 200,485 bytes, HARVEST-LOG 48,006 → 56,562 bytes
  (the tag's bytes and HEAD's bytes an exact prefix of each), and HANDOFF.
- **Housekeeping verified** from its receipt and the live checkout (Entry 3); STATE records it as
  done.
- **Al's ruling** on the inventory event's scope (Entry 4), and NEXT moved to that event.
- **Evidence:** the configs, the integration scripts and the id map are under the external root
  `integration/` (15 files listed in `MANIFEST-sha256.txt`, `d61f3cea…8570`).
BELIEF DELTAS:
- **The integration was as mechanical as REVIEW.md §4 said.** The preserved configs reproduce the
  held manifests exactly, so the regeneration changed only what the renumbering forces. No
  reading had to be redone.
- **The inventory event is larger than its bullet.** The pinned effective-inventory tool is code
  for a single event (`BASE`, a compiled preservation list, the 35 / 32 pin), so a later prefix
  needs a new tool revision, not a parameter. That design is not mechanical, and Al split it into
  its own pull request.
- **The gate stays GREEN with additions under `groundtruth/`.** `cargo test --all-targets`
  compiles the examples that refuse additions but does not run them. The refusal lives only in
  the manual reproduction, so the gate does not enforce the population pin. That is consistent
  with ADR-0012 D7, which does not ask the gate to.
CONFUSIONS:
- **The provisional header again lacked the UTC date** that ADR-0008 D1 step 0 asks for, the same
  slip as Session 37's. The cause is structural: that form is in ADR-0008, which a session may
  read only after it has disclosed. Cost: none, since the header is not part of the disclosure.
  Doc fix: a note for Al (ESCALATIONS); nothing was edited.
- **The Bash tool's `cd` persisted**, a hazard already in memory. It was noticed at once, and
  every later command uses absolute paths. Cost: none.
- **A `grep -c` over backslash paths printed 0** for a string `sed` showed was present (the Git
  Bash backslash hazard). Verified by `sed` instead. Cost: one look.
- **Entry 2's first draft dated the append "about 04:55Z".** The evidence only bounds it, before
  the 04:56:31Z clock reading, so it was corrected before commit 1.
HONESTY CHECK:
- No register status changed, and nothing was downgraded or upgraded.
- The 40 manifests are a count of files, not a MEASURED result. The inventory event measures
  them.
- The seven SOURCES entries remain VERIFIED by Session 22's retrievals. They were not re-fetched;
  each now says so in its *Integrated* line. The six abstract pages held only as receipts stay
  receipts.
- Two of the receipt's statements could not be re-checked live: the fast-forward's commit count,
  and the worktree's clean status before its removal. They are recorded as the receipt's.
- The launch-prompt copy is a transcription from the prompt as rendered to the executor.
- **`engine_commit` is `a793954`, the base the producer ran at**, as main's items do (Session 20's
  items carry the parent of the commit that added them). REVIEW.md §4 step 3's words "at the
  integration commit, which becomes their `engine_commit`" cannot be met literally, because a
  commit cannot contain its own id. Flagged for Al.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened. No detector, threshold or
generator design was touched. The inputs were the held branch, Session 22's preserved evidence,
REVIEW.md, ADR-0003, ADR-0012 and Al's rulings. The five items are S1 corpus, and ADR-0011 D6
bars them from seeding any generator. Verdict: no leakage.
DETERMINISM CHECK:
- The five manifests are regenerable from the preserved configs and originals with `s1_ingest`
  0.1.0 at `a793954`, shown by a byte-identical rerun.
- The records, SOURCES entries and log section are regenerable from the held branch's blobs by
  the scripts under the external root.
- No MEASURED result was produced, and nothing entered CLAIMS. Nothing to quarantine.
MANIFEST CHECK:
- Every new artifact carries a `syndeom-manifest/2` manifest, `origin: ground-truth`, written and
  self-checked by the producer (exit 0), with derivation `documented-by-source`.
- Nothing synthetic is labeled real.
- "Proven" was applied to nothing.
DRIFT CHECK:
- The records remain readings of documents and make no verdict on any paper (SD3).
- No understanding claims (SD17).
- No detector chosen or tuned, and no referee contact (SD13).
- No S2 code, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. After this pull request merges, the inventory event for the
first later prefix, in its own pull request; then S2 ordering step 4.
ESCALATIONS:
- **This pull request:** review and merge, with a merge commit (Q-H7).
- **`engine_commit` reading** (HONESTY CHECK): the base the producer ran at, not the integration
  commit itself.
- **Two readings of the digest refinement**, from Session 37, still await confirmation. This
  session's disclosure used both: the matched lines' digest is taken over each line with its LF,
  and their numbers did not change.
- **Note for Al, no action asked:** ADR-0008's provisional-header form (the UTC date, marked
  provisional) cannot be seen before the disclosure, and two sessions in a row missed it
  harmlessly.
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. The pull request is open for Al's review, and NEXT waits on its
merge. Repo green: the gate on the committed tree is recorded in the pull request and under the
external root, not in the tree it certifies. **Memory writes:** none, deliberately. A write
would change a memory digest and make the next disclosure owe verbatim reproduction.

---

SESSION: 2026-10-28 UTC (2026-10-28 America/New_York) (Session 39 — launched with the §6
Session-N resume prompt verbatim followed by an owner note, pasted into an existing Claude Code
conversation that had first done Session 38's post-merge housekeeping at Al's direction; SD15
disclosure first, by digest (ADR-0005 *Amendments* 1 and 2); not cold and not an examination —
the conversation read program state before the prompt, and the owner note carries program state;
examined fetched `origin/main` `ef567ec9824319924300a3f2d620e8121a16af18`, tree
`4bef2ccc6b24a0ed9c60294608c7dae839e28f88`, in the original checkout `C:\Users\LEET\Syndeom` on
`main`, synchronized; branch `claude/s1-later-prefix-inventory`; header completed after ADR-0008
step 5 — it was written provisionally with the disclosure, which is unchanged; the session-end SHA
is the commit carrying this block, recorded in its pull request)

### Entry 1 — SD15 memory disclosure (by digest, ADR-0005 Amendments 1–2)

Appended by shell (`cat >>`) without reading HANDOFF. The memory was read and hashed at
2026-10-28T15:50:31Z (11:50:31 America/New_York).

**How this session was launched: not cold, and not an examination.**
- The §6 Session-N prompt, verbatim, followed by a four-item owner note, was pasted into an
  existing Claude Code conversation (model Claude Opus 5.5, `claude-opus-5-5`). The launch carries
  program state (the owner note) and the conversation is not fresh, so under SD15 this session is
  not cold. It attempts no gate's cold-start test.
- The conversation is the one the owner note's item 4 describes. It was opened on 2026-10-28 at
  the owner's direction for post-merge housekeeping, without the §6 prompt. Before the §6 prompt
  arrived it read program state (Git status and history, and PR titles), so under SD15 it cannot
  be an examination on that ground as well.

**What the conversation did before the §6 prompt.**
- The harness loaded the memory index (`C--Users-LEET`) at conversation start.
- It searched every git checkout directly under the home directory, this one included, for commit
  `ef567ec`. None held it before the fetch.
- `gh search prs` over the owner's merged PRs. The results included the titles and merge times of
  this repository's PRs #1–#24, and PRs of other repositories, which are not named here.
- `gh pr view 24`, and `gh api` for the repository's default branch and for `main`'s head commit
  and its parents.
- It read `project_syndeom.md` in full.
- In `C:\Users\LEET\Syndeom`: `git status`; `git worktree list --porcelain`; `git branch -vv`
  (every local branch's name, tip and subject); `git log -1`; checks for lock files and for an
  unfinished merge, rebase, cherry-pick or revert; `git stash list`; `git remote -v`;
  `git fetch origin`; `git symbolic-ref refs/remotes/origin/HEAD`; `git remote show origin`;
  `git merge-base --is-ancestor main origin/main`; `git fsck`; `git switch main`;
  `git merge --ff-only origin/main` (its output listed created paths under `groundtruth/`);
  `git status`; `git worktree list`.
- It read no repository file, wrote no memory, and committed, pushed and deleted nothing.

**What this session did before this entry.** It read `PROGRAM/SYNDEOM-PROGRAM.md` and
`PROGRAM/DECISIONS/ADR-0005.md` in full. In one shell command it read and hashed the memory below
and took the time. Nothing else.

**The memory, by digest.** The matched lines and `project_syndeom.md` both have the digests of
ADR-0005 Amendment 2's baseline, so under Amendments 1–2 their content is not reproduced again.
- Memory index `MEMORY.md`, as a fact: SHA-256
  `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`, 15,420 bytes, 110 lines,
  0 CR, 334 non-ASCII bytes, no BOM (first bytes `23 20 50`), ends with LF. Last modified
  2026-10-26 15:30 America/New_York. This equals the whole-index fact in the baseline.
- Name search (D3): a case-insensitive search for `syndeom|kober|friedman` over all 110 lines
  (`awk` under `LC_ALL=C`, on `tolower` of each line) matched lines 85, 86, 88, 89, 106 and 107.
  A second search for `fr13dman|syndeo` matched 88, 89, 106 and 107, a subset. A search for
  `συνδ` matched nothing.
- Matched lines, each with its LF, concatenated in index order: 6 lines, 1,198 bytes, 0 CR,
  30 non-ASCII bytes, SHA-256 `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`.
  The baseline records this as the digest of HANDOFF lines 6672–6677, reproduced by Session 35's
  Entry 1.
- `project_syndeom.md`: SHA-256
  `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, 10,957 bytes, 135 lines,
  0 CR, 71 non-ASCII bytes, no BOM (first bytes `2d 2d 2d`), ends with LF. Last modified
  2026-10-26 18:45 America/New_York. The baseline records this as the digest of HANDOFF lines
  6683–6817, from the same entry.
- Withheld by count (D2): 104 lines, numbers 1–84, 87, 90–105 and 108–110. By the search above,
  none of them names this program or a sibling.
- D1 (c): the harness loaded no other memory that names this program.
- Citation: Session 35's Entry 1. A later entry verifies it by hashing those HANDOFF lines, so
  that this entry stays first.
- No memory was written in this conversation, before or after the §6 prompt.

### Entry 2 — the disclosure's citation, verified after the append (ADR-0005 *Amendments* 1 and 2)

- **Cited entry:** **HANDOFF Session 35, Entry 1** (its heading at line 6664 of this file at
  `ef567ec`). It reproduced D1 (a), the six matched index lines, at lines 6672–6677, and D1 (b),
  `project_syndeom.md` in full, at lines 6683–6817.
- **Verified by bytes, not only by stated digests.** Extracted from `ef567ec`'s HANDOFF blob:
  - lines 6672–6677 hash to `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`
    and are byte-identical (`cmp`) to index lines 85, 86, 88, 89, 106 and 107 as extracted for
    Entry 1;
  - lines 6683–6817 hash to `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`
    and are byte-identical (`cmp`) to `project_syndeom.md` on disk.

  Neither digest is new, so nothing is owed verbatim. The matched lines kept their numbers, so
  the renumbering reading did not arise.
- **The append.** The scratch file holding Entry 1 and its provisional header was 4,478 bytes,
  0 CR, no BOM, SHA-256 `a6b2705e…353f`, appended by `cat … >>` at 2026-10-28T15:51:55Z (the clock
  read by the same command). HANDOFF's last 4,478 bytes then hashed to the same digest, and HEAD's
  HANDOFF blob followed by the scratch file hashes to the working file's digest (`67e6ffda…2d69`):
  a pure append. The provisional header line (109 bytes) was then replaced by the completed header
  above, by a script that asserted the disclosure body (4,368 bytes, `6c163587…fe6e`) unchanged.
- **The provisional header carried the UTC date** and the word "provisional", as ADR-0008 D1 step
  0 asks. This session knew the requirement from the owner note's item 3 before it read ADR-0008.
- **Constitution and ADR bytes (ADR-0008 step 5).** As read in step 0 and as in the snapshot: the
  constitution `110f5360…479c` (38,958 B, ADR-0009's amended digest), ADR-0005 `b3da80df…f8cb`
  (12,382 B) and ADR-0008 `ccdfa9d7…e9ff`. Their working-tree blobs equal HEAD's.

### Entry 3 — the launch, the owner note, and Session 38's housekeeping

**Launching prompt.** Its first 13 lines are byte-identical (`cmp`) to the constitution's §6
Session-N block, lines 481–493 of the snapshot's `PROGRAM/SYNDEOM-PROGRAM.md` (1,141 bytes,
`00151726…3dd3`). An owner note followed them. Both copies are under the external root
`C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1553Z\`: `launch-prompt-first-13-lines.txt` and
`owner-note-as-received.txt` (1,570 bytes, SHA-256 `435a26e3…65a9`). They were written from the
message as rendered to the executor, so they are transcriptions. The note, verbatim:

~~~text
Owner note, to act on only after you have disclosed and oriented, and to record verbatim in this session's records:
1. The new manifests' engine_commit is the commit the producer ran from, as main's existing items record it. REVIEW.md's "at the integration commit" could not be met literally, and this is accepted.
2. Both readings of the digest rule recorded in ADR-0005 are confirmed: the matched lines' digest is taken over the byte form Session 37 verified against Session 35's reproduction, and a renumbering of matched lines without a change to their bytes is no change, with the new numbers and the withheld count recorded.
3. ADR-0005 also states the provisional header's UTC date requirement, so a session meets it before it can read ADR-0008. Record this as an ADR-0005 amendment.
4. Session 38's conversation was closed by a forced Windows restart overnight on 2026-10-27/28, before its post-merge housekeeping could be done there. At the owner's direction on 2026-10-28, a fresh Claude Code conversation, not launched with the §6 prompt and not a numbered session, did the housekeeping: it found the checkout C:\Users\LEET\Syndeom clean on claude/s1-held-harvest-integration at c43f9f9 with one worktree and no restart damage, fetched, read the default branch name from git, switched to it, and fast-forwarded main from a793954 to ef567ec. It made no disclosure, changed no repository file, committed, pushed and deleted nothing, and opened no PR. Record this in STATE as Session 38's housekeeping, noting that it was not done in Session 38's conversation.
~~~

**The housekeeping (item 4), from this conversation's own record.** The conversation that did it is
the one this session then ran in (Entry 1). Before the §6 prompt it observed and did the following:
- the checkout on `claude/s1-held-harvest-integration` at `c43f9f9` (PR 24's head), with a clean
  working tree;
- `git worktree list` showing one worktree;
- no lock file, no unfinished merge, rebase, cherry-pick or revert, no stash, and `git fsck`
  reporting nothing;
- five local branches whose upstreams were gone, from earlier merges, left alone;
- `git fetch origin` moved `origin/main` from `a793954` to `ef567ec`, PR 24's merge, with parents
  `a793954` and `c43f9f9`;
- the default branch, `main`, read from `git symbolic-ref refs/remotes/origin/HEAD` and
  confirmed by `git remote show origin`;
- `git switch main`, then `git merge --ff-only origin/main`, a fast-forward of three commits;
- the result was `main` at `ef567ec`, equal to `origin/main`, with a clean working tree.

It wrote no file and no memory, and committed, pushed and deleted nothing. Its commands are listed
in Entry 1. This session's preflight then found the checkout synchronized at `ef567ec` (ORIENT,
below). STATE records the housekeeping as Session 38's, done outside Session 38's conversation.

**Items 1–3** are recorded in STATE (*Standing rulings*, *Standing escalations*) and, for item 3,
as ADR-0005 *Amendments* 3.

### Entry 4 — the inventory event, built and measured (ADR-0013)

- **Commit 1** `46cdcb8` (tree `d5a24f99…d276`): ADR-0013 (proposed); the tool, in new files
  (`engine/ingest/examples/s1_later_prefix.rs`, `support/later_prefix.rs`); the packet's pins
  (`analysis/s1-later-prefix-1/config/prefix-inputs.json`, `config/dispositions.json`) and
  README; eight gate tests; and this block's Entries 1–3.
- **The run**, on the clean checkout at `46cdcb8`, 2026-10-28T16:07:27Z–16:07:31Z: two runs
  `PASS: 40 archived representations / 37 effective logical items; prefix through 1404.4599`,
  byte-identical; an overwrite refused. The pinned `s1_effective_inventory` refuses at the same
  tree on the first prefix addition, as ADR-0012 D7 says. The numbers are in ADR-0013
  *Measurements* and STATE's Updated bullet.
- **Commit 2:** the four outputs retained under `report/`, the ninth test (the working tree yields
  `inventory.json`, `coverage.json` and `report.md` byte-identically), ADR-0013's measurements,
  ADR-0005 *Amendments* 3, STATE, and this close. The retained-report test was held out of commit
  1 because no report existed yet, and restored byte-identically from a saved copy.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN (S2.1: ADR-0011 accepted; no code). Gate S1 CLOSED
(Session 29). This session's work is S1 corpus maintenance (the first later prefix's inventory
event, ADR-0003 D8), which Al ordered ahead of S2 ordering step 4.
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipt `orient-receipt.json`, SHA-256
`1eb0dd09…5587`, under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1553Z\`):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-28T15:52:11Z–15:52:12Z, exit 0.
- **Snapshot.** `main` at `ef567ec…`, tree `4bef2ccc…`, **synchronized**: ahead 0, behind 0. The
  only change was the disclosure append (`allowed_present`; `unrelated: []`). No worktree.
- **Step 6.** Read in this order: STATE (the Updated bullet, the freeze register, memory policy,
  standing rulings, the gate tables, blockers, standing escalations, NEXT); HANDOFF Session 38 in
  full; CLAIMS (0001–0005 VERIFIED, 0006 CONDITIONAL, 0007 VERIFIED existence only); FLAWS `0.1.0`
  (14 entries); then ADR-0008, ADR-0003 D8–D9, ADR-0012 and REVIEW.md §4. They agree with one
  another and with the tree.
- **ORIENT gate GREEN**, 15:53:14Z–16:02:55Z (STATE, *Current build verification*).
- **NEXT survived ORIENT.** Its condition held: PR 24 was merged with a merge commit on fetched
  `origin/main`.
ADVANCED:
- **The first later prefix's inventory event** (Entry 4): ADR-0013 (proposed), the tool
  `s1_later_prefix`, the packet `analysis/s1-later-prefix-1/` and nine gate tests. Session 23's
  projection of 40 / 37 is measured: 40 archived representations, 37 effective logical items.
- **Al's owner note recorded** (Entry 3): verbatim in STATE (*Standing rulings*); items 2 and 3 as
  ADR-0005 *Amendments* 3; item 4 as Session 38's housekeeping in STATE.
- **Escalations closed:** the `engine_commit` reading (accepted), the two digest readings
  (confirmed), and the provisional-header note (now in ADR-0005).
BELIEF DELTAS:
- **The pinned modules' public surface was enough.** The new revision reuses `input_set`,
  `validate_case`, `lineage`, `harvest`, `qualify` and `coverage()` without touching a pinned
  byte. Only the private assembly steps are re-done.
- **An append-only input can be pinned by its prefix.** Pinning the harvest log's first 56,562
  bytes, and parsing only those, lets a later harvest append rows without changing or stopping
  this event.
- **The gate now enforces this event's population.** Session 38 found that the gate compiles the
  pinned inventory without running it. The new tests run this event's inventory on a fixed list of
  files, so a later prefix's directories cannot enter the fixture. The gate stays GREEN when the
  next prefix lands, while the CLI refuses it.
CONFUSIONS:
- **Clippy's `duplicate_mod`** fired because `effective.rs` loads `coverage.rs` privately and the
  new tool loads it again. The pinned file cannot change, so the second load carries
  `#[allow(clippy::duplicate_mod)]` with a comment saying why. Cost: one lint round.
- **The retained-report test was drafted before the report existed.** It was cut for commit 1 and
  restored by script for commit 2. Cost: none.
- **A §6 launch into a working conversation.** The prompt arrived in the conversation that had just
  done the housekeeping. SD15's corollary already decides the case (not cold, cannot be an
  examination); the disclosure states it. No doc fix is needed.
HONESTY CHECK:
- No register status changed, and nothing was upgraded or downgraded. The 40 / 37 is a
  deterministic census of pinned metadata, as the S1.3 and S1.4 counts are. It enters no CLAIM.
- Flags B and C change counts: the unflagged subset is 23 rather than 18 under B, and C fixes
  whose labels are counted. They are stated as flags, not settled.
- The launch-prompt and owner-note copies are transcriptions of the message as rendered.
- The housekeeping is recorded from this conversation's own command output. No receipt file was
  written at the time.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened. No detector, threshold or generator
design was touched. The inputs were the committed corpus, the pinned modules, ADR-0003, ADR-0008,
ADR-0012, REVIEW.md and Al's rulings. Verdict: no leakage.
DETERMINISM CHECK:
- The outputs regenerate from a clean checkout of `46cdcb8`, or of any tree with the same inputs,
  with the one command in the packet's README. Two runs were byte-identical.
- The gate test regenerates `inventory.json`, `coverage.json` and `report.md` from the working
  tree. No seed is involved, since this is a census.
- Nothing entered CLAIMS, so there is nothing to quarantine.
MANIFEST CHECK:
- No artifact or manifest was written or changed. Every manifest read passes `Manifest::check` and
  the label validator.
- Nothing synthetic is labeled real.
- "Proven" was applied to nothing.
DRIFT CHECK:
- No verdict on any paper (SD3), and no understanding claims (SD17).
- No detector was chosen or tuned, and there was no referee contact (SD13).
- No S2 code, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. After this pull request merges, and with Al's rulings on ADR-0013's
flags recorded first, S2 ordering step 4: ADR-0011's crate.
ESCALATIONS:
- **This pull request:** review and merge, with a merge commit.
- **ADR-0013's flags A–D:** one tool and one packet; the five prefix roots unflagged; effective
  labels; the freeze-register row for the packet, proposed and not added.
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. The pull request is open for Al's review, and NEXT waits on its
merge. Repo green: the gate on the committed tree is recorded in the pull request and under the
external root, not in the tree it certifies. **Memory writes:** none, deliberately. A write would
change a memory digest and make the next disclosure owe verbatim reproduction.
AL'S RULINGS ON PR 25 (2026-10-28 UTC, received in this session's conversation after PR #25
opened; recorded at Al's direction in new commits on the same branch, no force-push; transcribed
verbatim from Al's message; copy `owner-rulings-adr-0013.txt` under the external root, 2,072
bytes, SHA-256 `e8fbcac1155cab68240e69c86d0a65139a3e3c70a0ce9c362a577404861c3b1a`):

~~~text
Rulings on ADR-0013 (record them verbatim in ADR-0013, STATE and this session's HANDOFF block, as new commits on PR #25's branch; no force-push):

Flag A: accepted. One tool and one packet.

Flag B: accepted. The five prefix roots count as unflagged. An item is flagged by a recorded open issue; the admission review and my rulings Q-H1–Q-H5 closed every reading raised on items 32–36, and dispositions.json records that basis per item. A later issue enters through the overlay.

Flag C: accepted. The coverage table uses the effective labels with the first-parent amendment applied. The S1.4 table stays frozen history, and the packet names which labels it used and the amendment map.

D7, the population pin: amended. The population is pinned by D2's recorded inputs — the accepted integration's input set and the five prefix item directories, by name and byte hash, with the harvest log read to its pinned prefix — and by nothing else. Counts are measured outputs, recorded in the packet and frozen there; they are not conditions in the code. Remove the literal 40/37/38 comparisons from the tool's guard and from its messages, and the literal-count assertions from the tests, keeping the identity checks, the byte-identity test against the retained report, and any assertion derived from the recorded inputs. Take the "through candidate 30 / 1404.4599" scope from prefix-inputs.json or from the pinned harvest-log prefix, not from a constant. Rewrite D7 and D8 to match. The pinned ADR-0012 tool keeps its literals: it is frozen, and this ruling applies from this event on. Regenerate the packet, since provenance records the tool's sources, and confirm two runs are still byte-identical.

Flag D: on the branch, after the regeneration and before the merge, as PR #24 did: a final commit adds one frozen row for analysis/s1-later-prefix-1/ whose freeze point is the commit that holds the regenerated packet. I will merge with a merge commit so that commit's id survives.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

How each ruling is applied (commit 3 unless stated):
- **A, B and C** → recorded as accepted in ADR-0013 (*Status*, D1, D4, D5, *Flags*). For C, the
  packet names its labels and the amendment map: `coverage.json` gains `amendment_map` (path,
  SHA-256, logical item key, effective artifact id), and `label_basis` and `report.md` say which
  labels were used.
- **D7** → the tool's guard compares no count with a literal. The 40, 37 and 38 comparisons and
  the `LAST_CANDIDATE` constant are gone, and so is the prefix pins' comparison with 5. The guard
  keeps the identity checks:
  - every review names a logical item;
  - the amended item aliases exactly its two representations;
  - the harvest admissions equal the corpus roots.

  The refusal message names those identities, and the PASS line prints the measured values. The
  scope is measured from the harvest log's pinned prefix: `prefix_through_candidate` and
  `prefix_through_arxiv_id`, with `scope_source`. Tests:
  - the literal-count test is replaced by `population_follows_the_recorded_inputs`, whose every
    assertion is derived from the fixture's recorded inputs;
  - the two tests that asserted a literal count now assert outputs equal to the unchanged
    fixture's;
  - the byte-identity test is kept.

  A grep of the tool and the tests for the whole words 40, 37, 38, 30, `1404.4599` and
  `LAST_CANDIDATE` finds nothing (exit 1). A canary with the same grep form finds the freeze-point
  tree id, so the absence is real. D7 and D8 are rewritten. The pinned ADR-0012 tool is
  untouched.
- **Regeneration** → commit 4. The byte-identity test is held out of commit 3, because the
  retained report is stale until then, and restored in commit 4 from a saved copy, as in
  commits 1–2.
- **D** → commit 5, the freeze-register row with commit 4's full id as its freeze point.

The rulings, applied:
- **Commit 3** `97e0d54d1a68d66861b26f6e02af29d346e417ab` (tree `3f05bbc0…466f`): the tool, the
  tests, ADR-0013, the packet README, STATE, and the part of this addendum above. Eight tests
  passed with the byte-identity test held out.
- **Regeneration**, on the clean checkout at `97e0d54`, 2026-10-28T16:48:47Z–16:48:50Z, two runs:
  `PASS: 40 archived representations / 37 effective logical items, measured; prefix through
  1404.4599`. The four outputs are byte-identical across the runs (`inventory.json`
  `fa407409…f66c`, `coverage.json` `a0c4d864…17f5`, `report.md` `129b0fdc…d35c`,
  `provenance.json` `df83d3e2…0a9c`). Against the first run, every count is equal. Only the
  intended fields differ: `accepted_inventory`'s wording, and `amendment_map`,
  `prefix_through_candidate`, `scope_source`, `label_basis` and `gate_s1_table` in coverage, with
  the report header and provenance following. The run directories are `regen-1/` and `regen-2/`
  under the external root.
- **Commit 4** `bcdcdfa36cc6be8a74409d34d08ea6edabd3ac3d` (tree `18368337…9150`): the regenerated
  packet, the byte-identity test restored from its saved copy (9 of 9 pass), and ADR-0013's
  measurements.
- **Commit 5** (this one): the frozen row `analysis/s1-later-prefix-1/` at `bcdcdfa…` (Al's ruling
  D), the register preamble naming that exception, STATE, and this close. `check_freeze.py`: 65
  rows (60 frozen, 5 append-only), ALL PASS; the new row has 7 files, 7 byte-identical and 0
  added. The packet is not touched after commit 4.
BELIEF DELTA: once the inputs are pinned by name and hash, the counts are redundant as conditions.
The literal guard could only disagree with the pinned inputs, so it was a second source of truth.
The amended tool has none, and the gate test derives every expectation from the same inputs.
CONFUSION: none new. The held-out-test step was needed again, because the outputs changed and
the retained report could only be regenerated from a committed tree.
CHECKS for this addendum: no leakage (`contact/` not opened; no detector, threshold or generator
touched); no manifest, label or register status changed; every count is unchanged from the first
run; no verdict on any paper (SD3). **Memory writes:** none.
ESCALATIONS (superseding the block's):
- **This pull request:** review and merge with a merge commit, so that the freeze point
  `bcdcdfa` survives in `main`'s history (Al's ruling D).
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction, stop for review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root, not in the tree it certifies.

SESSION: 2026-10-28 UTC (2026-10-28 America/New_York) (Session 40 — a fresh conversation launched
from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone; SD15 disclosure first,
by digest (ADR-0005 *Amendments* 1–3); not an examination — Gate S2 has nothing to examine;
examined fetched `origin/main` `f9f973012cbe1a7d7f1d8cd4520db998dca0e550`, tree
`84a010018335bfdc94bcc2214babd593b5b734bb`, in the original checkout `C:\Users\LEET\Syndeom` on
`main`, synchronized; branch `claude/s2-generate-crate`; header completed after ADR-0008 step 5 —
it was written provisionally with the disclosure, which is unchanged; the session-end SHA is the
commit carrying this block, recorded in its pull request)

### Entry 1 — SD15 memory disclosure, by digest (ADR-0005 D1–D4, Amendments 1–3)

This disclosure was made before any other action. The only steps before it were:
- reading `PROGRAM/SYNDEOM-PROGRAM.md` and `PROGRAM/DECISIONS/ADR-0005.md` in full;
- reading and hashing the memory below.

No other repository file, directory listing, Git history or status, or build was read or run
first. HANDOFF's existing contents were not read. This entry was appended by a Node script
(`fs.appendFileSync`), not by a file-editing tool. The facts below were computed from the files on
disk at 2026-10-28T17:05:04Z (UTC).

- **Memory the harness loaded:** only the index
  `C:\Users\LEET\.claude\projects\C--Users-LEET\memory\MEMORY.md`. The session was launched in
  `C:\Users\LEET`, which is not a git work tree, so no gitStatus snapshot was attached. No other
  memory file was loaded by the harness. This session read `project_syndeom.md` in full for this
  disclosure.
- **The whole index, as a fact:** 110 lines, 15,420 bytes, 0 CR, 334 non-ASCII bytes, ends with LF,
  SHA-256 `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`. This is the same
  digest as the whole-index fact in ADR-0005 Amendment 2's baseline.
- **Name search (D3):** a case-insensitive search for `/syndeom|kober|friedman/i` over every index
  line matched lines 85, 86, 88, 89, 106 and 107 (6 lines). It ran as a Node regex, not Git Bash
  grep. An extra case-insensitive search for `fr13dman` matched lines 88, 89 and 107, all among
  the six. The canary `/project memory/i` matched line 1, which shows the search runs.
- **Matched index lines (D1 a):** lines 85, 86, 88, 89, 106 and 107, each with its LF terminator,
  concatenated in index order. They total 1,198 bytes, with 0 CR and 30 non-ASCII bytes, SHA-256
  `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`. The line numbers and the
  digest are the same as in Amendment 2's baseline.
- **Withheld by count (D2):** 104 lines, at line numbers 1–84, 87, 90–105 and 108–110. None of
  them matched the name search.
- **The SYNDEOM memory file (D1 b):** `project_syndeom.md`, 135 lines, 10,957 bytes, 0 CR,
  71 non-ASCII bytes, ends with LF, SHA-256
  `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`. This is the same digest as in
  Amendment 2's baseline.
- **Other recalled memory (D1 c):** none.
- **The by-digest path (Amendment 1, refined by Amendment 2, readings confirmed by Amendment 3):**
  both compared digests (the matched lines and the SYNDEOM file) equal the digests of bytes that
  HANDOFF Session 35, Entry 1 reproduced, according to ADR-0005 Amendment 2:
  - HANDOFF lines 6672–6677 hold the matched lines;
  - HANDOFF lines 6683–6817 hold the file.

  Their content is not reproduced again. At this point the citation comes from ADR-0005. Entry 2
  verifies it after this append, by hashing those HANDOFF lines.
- **D4, as read by this session:** the SYNDEOM file holds environment and tooling hazards and
  locations only: the repo, the second checkout, and the external evidence roots. It holds no
  phase, gate status, CLAIM or FLAW status, commit SHA used as a state marker, next action or
  escalation. The six matched index lines name the three programs' repos and say that no state is
  recorded there.
- **Memory writes:** none in this session, and none will be made.
- **Launch:** the launching message is the §6 Session-N prompt with no owner note appended. By
  eye, it matches §6's text line for line.

### Entry 2 — the disclosure's citation, verified after the append; the header completed

- **Cited entry:** HANDOFF Session 35, Entry 1 (heading at line 6664, "Entry 1 — SD15 memory
  disclosure (ADR-0005 D1–D4; ADR-0009)"). Extracted from HEAD's HANDOFF blob (`f9f9730`, blob
  SHA-256 `76ace902…ccdd`):
  - lines 6672–6677 hash to `0418e621…534c` and are byte-identical to index lines 85, 86, 88, 89,
    106 and 107;
  - lines 6683–6817 hash to `ce2ce6de…2cc0` and are byte-identical to `project_syndeom.md` on disk.

  Neither digest is new, so nothing is owed verbatim. The matched lines kept their numbers.
- **The append.** Entry 1 with its provisional header was 3,607 bytes, SHA-256 `e1ffe4e9…a52d`,
  appended by `fs.appendFileSync` at 2026-10-28T17:05Z; HANDOFF's size grew by exactly that much.
- **The header.** A script checked that HANDOFF's last 3,607 bytes were the appended entry, then
  replaced the 80-byte provisional header line with the completed header (728 bytes). It asserted
  that the disclosure body (3,526 bytes, `351c8267…2933`) was unchanged and that HEAD's HANDOFF
  blob is an exact prefix of the working file.
- **Constitution and ADR bytes (ADR-0008 step 5).** As read in step 0 and as in the snapshot: the
  constitution `110f5360…479c` (38,958 B, ADR-0009's amended digest) and ADR-0005 `b08e543a…ce11`
  (15,291 B, with *Amendments* 1–3).

### Entry 3 — an interruption, and Al's direction

A safety classifier stopped one of this session's responses during ORIENT step 6, after the
ORIENT gate had ended GREEN, and that response wrote nothing to the repository, the external root
or the memory store. The stopped response was carrying out the review of the ORIENT gate's log,
the last check of ADR-0008 step 6, before ASSESS and PLAN.

The executor then reported the session's position and asked whether to continue. Al's direction,
transcribed verbatim from the message as rendered to the executor:

~~~text
Continue in this session. Your disclosure, orientation and gate stand; redo nothing. Record the interruption in this session's HANDOFF block in one sentence — the step it happened at, and that nothing was written — and say in one sentence which step the stopped response was carrying out, without reproducing it. Then proceed: PLAN the one thing STATE names, ADR-0011's crate, and EXECUTE it on a fresh branch from main. If a classifier stops you again at any point, do not reword or retry: stop at the nearest point where the committed tree passes the gate, complete the header, record both interruptions, push the branch, open the PR, and report. Otherwise run the loop to a clean stop per §4.6 and stop for my review.
~~~

### Entry 4 — ADR-0011's crate, built (S2 ordering step 4)

All on branch `claude/s2-generate-crate`, from `main` `f9f9730`:
- **Commit 1** `9b4b16f688bfc280a0e7ce0cda2da80558435f5b`: the crate `engine/generate/`, with
  Cargo.toml and Cargo.lock gaining the workspace member only, and this block's Entries 1–3.
- **Commit 2** `21f83594a22d514bb9cdb1ec0261f659e7d1f4b9`: the seed witnesses, `Audit.lean`, and
  the gate's axiom loop. This is the D10 gate change that ruling 3 gives its own review.
- **Commit 3** `f8cc5df249c3026d2685c17483ccba6301c5cf3a`: the 12 goldens, produced by
  `generate --dry-run` at commit 2, with the Lean harness run one process at a time
  (2026-10-28T17:37:02Z–17:38:56Z).
- **Commit 4** `cd5633b3f1a7388565526e5d93f0afffabcc17c4`: the security review's four fixes; no
  emitted byte changed.
- **Commit 5** (this one): the packet `analysis/s2-generate-1/`, STATE, and this close.

The evidence, the security review and readings 1–12 are in the packet's README. Every receipt is
under `C:\Users\LEET\Codex\Temp\syndeom-generate-20261028T1706Z\`.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN. S2.1–S2.4 are in progress: this is the first
implementation session (ADR-0011 D11). Gate S1 is CLOSED (Session 29).
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipt `orient-receipt-1.json`, SHA-256
`6dd8577d…0f37`, under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1706Z\`):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-28T17:06:23Z–17:06:24Z, exit 0.
- **Snapshot.** `main` at `f9f97301…`, tree `84a01001…`, **synchronized**: ahead 0, behind 0. The
  only change was the disclosure append (`allowed_present`; `unrelated: []`). No worktree.
- **Step 5.** The constitution (`110f5360…`) and ADR-0005 (`b08e543a…`), as read in step 0, equal
  the snapshot's blobs.
- **Step 6.** Read in this order:
  - STATE (the Updated bullet, the freeze register, memory policy, standing rulings, the gate
    tables, blockers, standing escalations, NEXT);
  - HANDOFF Session 39 in full;
  - CLAIMS: 0001–0005 VERIFIED, 0006 CONDITIONAL, 0007 VERIFIED (existence only);
  - FLAWS `0.1.0` (14 entries);
  - ADR-0008, ADR-0011, ADR-0001's D4 registry and example, and `syndeom-manifest`'s API.

  They agree with one another and with the tree.
- **ORIENT gate GREEN**, 17:07:43Z–17:08:31Z (STATE, *Current build verification*).
- **NEXT survived ORIENT.** PR 25 was merged by Al with the merge commit `f9f9730`. Its only
  comment is the executor's own, so there was no later ruling to record.
ADVANCED:
- **ADR-0011's crate** (S2.1–S2.4 in progress): the plan and both renderers (Golden A and Golden B
  reproduced), `syndeom-rng/1`, `s2_template` 0.1.0 with three seeds, the injector, ground truth,
  commanded, achieved and fidelity, the Lean harness, and the CLI.
- **12 goldens**, each fidelity PASS, with check K confirmed by Lean for the three flawed formal
  cases.
- **The gate's axiom audit** now covers the seed witnesses.
- **`cargo audit`** is clean, and the security review is recorded with four findings fixed.
BELIEF DELTAS:
- **Lean prints the compiled file's path exactly as it was passed:** the Windows verbatim form
  `//?/C:/…`, because the CLI canonicalizes paths. Check K can therefore match the file exactly,
  and FINDING-004 relies on that.
- **`Mathlib.Data.Real.Basic` is deprecated at the pinned Mathlib** (since 2026-08-27; replace with
  `Mathlib.Basic.Real.Basic`). It warns rather than fails. A seed that used it would carry the
  warning into every build output.
- **Compile cost is the import set's.** A seed with narrow imports compiles in about 8–17 s.
  `import Mathlib` (the `nat_le_sq` bytes that Golden B fixes) takes about 50 s. Full-Mathlib
  seeds will dominate the sweep's cost.
- **ADR-0012's narrowing did its job.** Editing `engine/Cargo.toml`, `engine/Cargo.lock`,
  `lean/Audit.lean` and `scripts/check.sh` left every existing test green, because those files
  are outside the S1.3 reproduction's input set.
CONFUSIONS:
- **The goldens could not be produced while their own files sat in the working tree.**
  Engine-commit verification walks `engine/generate/` and refuses any file the commit lacks. The
  order is therefore: commit the producer, generate into external receipt directories, then add
  the goldens. Cost: none, because the order was planned. The goldens README records it.
- **`--dry-run` first wrote its outputs nowhere.** It now writes them into the receipt
  directory, so the goldens needed no `corpora/` artifact before the sweep. Cost: one edit.
- **The classifier interruption** (Entry 3). Cost: one round trip, and nothing was written.
HONESTY CHECK:
- No register status changed. The packet's measurements are implementation evidence, not CLAIMS.
- The RNG vectors are checked against D4's text, not against the published reference code. The
  attribution is unverified, and the test and the packet say so.
- The engine produced the golden manifests; they were not written by hand.
  - The formal proof bytes were written by hand first and matched exactly.
  - The informal bytes are seed material, rendered. They were checked by reading. Only
    `nat_le_sq` has an independent byte check, against Golden A.
- FINDING-005 stays open: `engine_commit` identifies the checked-out sources, not the running
  binary.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened. The seeds come from FLAWS.md 0.1.0
and ADR-0001 D8 only, and no `groundtruth/` item was read for design (D6, ruling 1). No real
AI-generated proof was used. Verdict: no leakage.
DETERMINISM CHECK:
- Every golden regenerates byte for byte from its config, seed and recorded observation.
- Two calls with the same input are identical.
- The RNG vectors are pinned.
- No wall-clock value enters an artifact; receipts carry durations only.
- Nothing entered CLAIMS, so there is nothing to quarantine.
MANIFEST CHECK:
- Every golden manifest passed `Manifest::check` with its bytes before it was written: `origin`
  synthetic, `derivation` by-construction.
- Nothing synthetic is labeled real.
- "Proven" was applied to nothing. The witnesses are `lean-kernel` references audited by the
  gate, not CLAIMS.
DRIFT CHECK:
- No verdict on any paper (SD3), and no understanding claims (SD17).
- No detector was chosen or tuned, and there was no referee contact (SD13).
- No S3 work, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. After this pull request merges, with Al's rulings recorded first,
add the next `s2_template` seeds: the FLAWS.md examples that check K confirms.
ESCALATIONS:
- **This pull request:** review and merge, with a merge commit.
- **Commit `21f8359`:** the D10 gate change, under its own review (ruling 3 of 2026-10-27).
- **Readings 1–12** in `analysis/s2-generate-1/README.md`, FINDING-005 among them.
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. The first implementation session's scope (ADR-0011 D11) is built,
and the pull request is open for Al's review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root, not in the tree it certifies.
**Memory writes:** none. A write would change a memory digest and make the next disclosure owe
verbatim reproduction.

SESSION 40 ADDENDUM — AL'S RULINGS ON PR 26 (2026-10-28 UTC, received in this session's
conversation after PR #26 opened; recorded at Al's direction in new commits on the same branch,
no force-push; transcribed verbatim from Al's message as rendered to the executor; copy
`owner-rulings-pr26.txt` under `C:\Users\LEET\Codex\Temp\syndeom-pr26-rulings-20261028\`, 2,281
bytes, SHA-256 `f4f6e2b05f0bf39045079a621cadcd567eed3b007109cc858ee4020b4ccac267`):

~~~text
Rulings on PR #26 (record them verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

1. The gate change in 21f8359 is accepted: the seed witnesses are library modules, and the audit parses every #print axioms line against the standard three axioms with a count check.

2. Readings 1, 2, 3, 5, 6, 7, 8 and 9 are accepted as read.

3. Reading 4 is not accepted as read. Flaw counts are per instance, not per record: one injected flaw rendered in both files of a pair is one instance with two locations, and n_flaws_total and n_flaws_<NNNN> count instances. No current golden is a pair, so nothing regenerates now; the rule binds the next producer revision before the first pair golden, and ADR-0001's wording is clarified if it needs it.

4. Reading 10: the attribution is to be verified against retrieved vectors, not left unverified. An independent implementation from the published algorithms reproduces this crate's D4 vectors and matches the reference vectors in rand_xoshiro 0.3.0's tests (src/splitmix64.rs: seed 1477776061723855037, 50 outputs; src/xoshiro256starstar.rs: state 1, 2, 3, 4, 10 outputs; both recorded there as produced with Vigna's reference C at xoshiro.di.unimi.it). Retrieve those two vectors, register the source under SD11, add them as tests in engine/generate/tests/rng.rs (a state constructor for the stream may be needed for the xoshiro vector; it must change no output), and lift the "unverified" note in the tests header, the packet README and D4's conformance record.

5. Readings 11 and 12: accepted. A new seed bumps s2_template to 0.2.0 with the 0.1.0 goldens kept regenerable under the version they name, and 0.2.0 embeds its input hashes at build time as syndeom-pair does, closing FINDING-005.

6. Record correction: the packet README and HANDOFF say each formal golden is byte-identical to its hand-written probe. That holds for three of five; the two faithful probes carry a trailing #print axioms line the goldens do not. Correct the wording; the probes stay as they are.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review. I will merge with a merge commit: the goldens pin engine_commit 21f8359, which must stay a real commit on main.
~~~

Before applying the rulings, a fresh fetch at 2026-10-28T17:59Z showed `origin/main` still at
`f9f9730` and the branch at `8697ab3`, in sync with its remote.

**The gate on PR 26's head, before the rulings:** `8697ab3` (tree `a2345dd4…`) was GREEN,
2026-10-28T17:48:05Z–17:48:47Z, exit 0, with the tree clean before and after.
- Lean: 8,929 jobs; four axiom lines within the standard three.
- Rust: fmt and clippy pass, and 114 tests pass.
- Records: 570 tracked files are LF; the registers and the freeze register (65 rows) ALL PASS.
- Raw log SHA-256 `2214f509…473a`, under `C:\Users\LEET\Codex\Temp\syndeom-generate-20261028T1706Z\`.

The preflight's `recheck` exited 0, and the push created the branch.

How each ruling is applied:
- **Ruling 1 (the gate change accepted)** → recorded in ADR-0011 *Amendments* and STATE.
- **Ruling 2 (readings 1, 2, 3, 5–9 accepted)** → recorded, and the packet README marks them.
- **Ruling 3 (counts per instance)** → recorded as an amendment of D8.
  - `s2_template` 0.1.0 counts records, and no 0.1.0 golden is a pair, so no code or byte
    changed now.
  - STATE's NEXT makes per-instance counting part of 0.2.0, with the property test that equates
    the counts with records changed then.
  - ADR-0001 is left unedited: it names `n_flaws_total` and `n_flaws_<NNNN>` without a unit and
    already defines `flaws_confirmed` as a count of instances.
  - One reading is the executor's, open for Al: a pair instance counts as confirmed when each of
    its records passes its check.
- **Ruling 4 (the RNG attribution)** → commit `91dde9a6763526e6d9f2131ce196f6d6a54b2bd3`.
  - **Retrieval** at 2026-10-28T17:59:51Z–17:59:52Z: the crates.io index entry for
    `rand_xoshiro` (SHA-256 `6c7f3b35…f838`) and the 0.3.0 archive (15,597 bytes, SHA-256
    `0be70256…831f`, equal to the index's `cksum`, not yanked). The archive was extracted as data
    into a new directory.
  - **The vectors**, extracted by script:
    - `src/splitmix64.rs` test `reference`: seed 1477776061723855037, 50 outputs, comment naming
      `splitmix64.c`;
    - `src/xoshiro256starstar.rs` test `reference`: state 1, 2, 3, 4, 10 outputs. Its comment
      names `xoshiro128starstar.c`, which is recorded as written.
  - **Agreement:** both the crate and the independent Python implementation of D4 reproduce them.
  - **Registered** as S-0081, digest-only.
  - **Tests** in `engine/generate/tests/rng.rs`, through the new `Stream::from_state`. The 12
    goldens still regenerate, so no output changed.
  - **Notes lifted:** the "unverified" note in the tests header, the packet README and, by the
    amendment, D4's conformance record.
- **Ruling 5 (0.2.0)** → recorded. NEXT step 3 makes 0.2.0 the first move: the version bump,
  build-time input hashes closing FINDING-005, per-instance counts, and the 0.1.0 goldens kept
  regenerable. The packet marks FINDING-005 as scheduled.
- **Ruling 6 (the record correction)** → the packet README, the goldens README and STATE now say
  that the three flawed formal goldens are byte-identical to their probes. The two faithful probes
  are their goldens followed by the probe's own `\n#print axioms <main>\n` tail, which the copy
  script removed before comparing. The probes are unchanged.

  **This block's Entry 4 is corrected here, because HANDOFF is append-only.** Its HONESTY CHECK
  bullet "The formal proof bytes were written by hand first and matched exactly" holds for the
  three flawed cases only; for the two faithful cases it holds after removing the probe's axiom
  print. The same overstatement is in commit `f8cc5df`'s message and in PR 26's description. The
  commit message stands as history; the PR comment for this addendum corrects it.

BELIEF DELTA: a test vector's provenance can be checked one link further than its file. The
crates.io index publishes the archive's SHA-256, so the vectors used here are pinned to bytes the
registry vouches for. They do not rest only on a repository copy.
CONFUSION: the copy report's `equals_probe: true` came from a comparison that first stripped the
probes' axiom-print tail. The session then summarized it as "byte-identical", and the report's
field name hid the normalization. Cost: a record correction. Fix: state the normalization
whenever a comparison is not byte-exact.
CHECKS for this addendum:
- No leakage: `contact/` was not opened, and the only source added is an RNG crate's tests.
- No manifest, golden byte, label or register status changed. SOURCES gained S-0081,
  append-only.
- No verdict on any paper (SD3). **Memory writes:** none.
ESCALATIONS (superseding the block's):
- **This pull request:** review and merge with a merge commit, so that `21f8359`, the goldens'
  `engine_commit`, stays a real commit on `main`.
- **For the next revision:** the pair-confirmation reading under ruling 3.
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction, stop for review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root, not in the tree it certifies.

SESSION: 2026-10-28 UTC (2026-10-28 America/New_York) (Session 41 — a fresh conversation launched
from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim followed by an owner note; SD15
disclosure first, by digest (ADR-0005 *Amendments* 1–3); not an examination — the owner note
carries program state, and Gate S2 has nothing to examine; examined fetched `origin/main`
`de3e4a62fde75eea2c80c17ac6210955bb868a0f`, tree `4117ac9d979c9c837de533e14baa8a562f870c4d`, in
the original checkout `C:\Users\LEET\Syndeom` on `main`, synchronized; branch
`claude/s2-template-0.2.0`; header completed after ADR-0008 step 5 — it was written provisionally
with the disclosure, which is unchanged; the session-end SHA is the commit carrying this block,
recorded in its pull request)

### Entry 1 — SD15 memory disclosure, by digest (ADR-0005 D1–D4 as amended by Amendments 1–3)

Appended 2026-10-28 UTC by shell append (`cat >> PROGRAM/HANDOFF.md` of a scratchpad file, CRs
stripped), before any other command or repository read except `PROGRAM/SYNDEOM-PROGRAM.md` and
`PROGRAM/DECISIONS/ADR-0005.md`, both read in full, and the memory hashing below. HANDOFF's
existing contents were not read before this append. No directory listing, Git command, or build
preceded it. No memory was written. Executor model this session: Claude Fable 5.1. The launching
prompt was the §6 Session-N resume prompt verbatim, followed by an owner note (two items on
flaw-count units and pair-artifact confirmation), which is recorded verbatim later in this block.

**Memory loaded by the harness this session.** The index `MEMORY.md` only, under
`C:\Users\LEET\.claude\projects\C--Users-LEET\memory\` (the session was launched from
`C:\Users\LEET`, which is not a git work tree). No other recalled memory file was loaded into
context (ADR-0005 D1(c): none). The program's own memory file `project_syndeom.md` was read in
full and hashed for this disclosure; it is not reproduced (Amendment 1), see its digest below.

**Whole index, as a fact (Amendment 2).** SHA-256
`cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`, 15,420 bytes, 110 lines
(LF-terminated, last byte 0x0a), 0 CR, 334 non-ASCII bytes. Equal to the digest ADR-0005
Amendment 2 recorded as unchanged since Session 37's Entry 1.

**The name check (D3).** Case-insensitive search of every index line (awk `tolower` with
`index`/regex, not `grep -i`, per the Git Bash grep 3.0 hazard) for this program's name and its
siblings' names:
- `syndeom` → lines 106, 107
- `kober` → lines 85, 86, 89, 106
- `friedman` → lines 88, 89, 106
- union → lines 85, 86, 88, 89, 106, 107 (6 lines)
- extra check, the sibling's leetspeak repo name `fr13dman` → lines 88, 89, 107, all inside the
  union.
Every matched line is among the six whose digest is recorded under D1(a) below; no withheld line
names the program or a sibling.

**Matched index lines (D1(a)), by digest.** Lines 85, 86, 88, 89, 106 and 107, each with its LF
terminator, concatenated in index order: 6 lines, 1,198 bytes, 0 CR, 30 non-ASCII bytes, SHA-256
`0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`. This equals the baseline
ADR-0005 Amendment 2 recorded, which that amendment states is the digest of HANDOFF lines
6672–6677 as reproduced verbatim by Session 35's Entry 1. The line numbers are unchanged from
that baseline (no renumbering).

**Program memory file (D1(b)), by digest.** `project_syndeom.md`: SHA-256
`ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, 10,957 bytes, 135 lines
(LF-terminated, last byte 0x0a), 0 CR, 71 non-ASCII bytes. Equal to the baseline ADR-0005
Amendment 2 recorded as the digest of HANDOFF lines 6683–6817, from the same Session-35 entry.
Read in full this session: it holds the repo location, the sibling pointers, the standing
repo-is-the-only-memory rule, and machine/harness hazards; it names no phase, gate, CLAIM or FLAW
status, commit SHA, toolchain version, escalation, or next action.

**Withheld (D2).** 104 index lines, occupying lines 1–84, 87, 90–105 and 108–110. They concern
unrelated programs on this machine and are withheld by count because this repository is public.

**Citation (Amendment 1).** Session 35, Entry 1, the verbatim reproduction whose bytes these two
digests equal, cited through ADR-0005 Amendment 2's *Baseline for the next disclosure*. Because
this entry was appended without reading HANDOFF, the equality is asserted here against the
digests recorded in ADR-0005; the check against HANDOFF's own bytes (hashing lines 6672–6677 and
6683–6817) is made in an entry located after this one, so that the disclosure stays first
(Session 37's precedent).

**Pass criterion (D4).** The reproduced memory, taken by digest, is the same bytes Session 35
reproduced; read this session, the SYNDEOM memory file holds no program state, and the six
matched index lines are the index pointers to this program and its siblings. Withheld lines are
outside the program by D3. Whether this session attempts a gate's cold-start test is decided
after orientation; this disclosure was made either way, as SD15 (amended by ADR-0009) requires.

**Method.** Scratchpad script `sd15_hash.sh` (GNU coreutils in Git Bash: `sha256sum`, `wc`, `tr`,
`od`, `awk`), run at 2026-10-28T18:13:08Z against the files on disk. Digests only; nothing was
reproduced.

### Entry 2 — the disclosure's citation, verified after the append; the header completed

- **Cited entry:** HANDOFF Session 35, Entry 1 (heading at line 6664, "Entry 1 — SD15 memory
  disclosure (ADR-0005 D1–D4; ADR-0009)"). Checked in the working file, whose first 674,452 bytes
  are HEAD's HANDOFF blob (`de3e4a6`, blob SHA-256 `3bcc5938…e6ba`):
  - lines 6672–6677 (1,198 bytes) hash to `0418e621…534c` and are byte-identical to index lines
    85, 86, 88, 89, 106 and 107;
  - lines 6683–6817 (10,957 bytes) hash to `ce2ce6de…2cc0` and are byte-identical to
    `project_syndeom.md` on disk.

  Neither digest is new, so nothing is owed verbatim. The matched lines kept their numbers.
- **The append.** Entry 1 with its provisional header was 4,684 bytes, SHA-256 `cb4653eb…2308`,
  appended by `cat >>` at 2026-10-28T18:14:37Z. HANDOFF grew from 674,452 to 679,136 bytes,
  exactly that much, and HEAD's blob is an exact prefix of the working file.
- **The header.** A script checked that HANDOFF's last 4,684 bytes were the appended entry, then
  replaced the one provisional header line (`## Session <N> — 2026-10-28 UTC (provisional header,
  completed after orientation)`, 84 bytes) with the completed `SESSION:` header. It asserted that
  the disclosure body, from `### Entry 1` to the end of the append, was unchanged, byte for byte.
- **Constitution and ADR bytes (ADR-0008 step 5).** As read in step 0 and as in the snapshot at
  `origin/main` `de3e4a6`: the constitution `110f5360…479c` (38,958 B, ADR-0009's amended digest),
  ADR-0005 `b08e543a…ce11` (15,291 B, with *Amendments* 1–3) and ADR-0008 `ccdfa9d7…e9ff`
  (18,589 B).

### Entry 3 — the launch, and the owner note (a ruling, recorded before anything else)

**The launch.** A fresh conversation from `C:\Users\LEET`, launched with the §6 Session-N resume
prompt verbatim, followed by an owner note of two items. By eye the prompt matches §6's text line
for line. The harness attached no Git snapshot (`C:\Users\LEET` is not a work tree) and loaded the
`C--Users-LEET` index. Executor model: Claude Fable 5.1.

**Not an examination.** The owner note carries program state (it names ADR-0001's registry,
ADR-0011's amendments and the 0.2.0 revision), and Gate S2 has nothing to examine. The session
certifies nothing.

**The owner note, verbatim.** Transcribed from the message as rendered to the executor; the exact
copy is `owner-note-as-received.txt` under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1815Z\`
(504 bytes, SHA-256 `a80498bc38ee1e643a5cc0fa29e58c33ecb016dddb308fc8cda9b740db4ea4f1`).

~~~text
Owner note, to act on only after you have disclosed and oriented, and to record verbatim in this session's records:
1. ADR-0001 stays unedited on flaw-count units: its registry names n_flaws_total and n_flaws_<NNNN> without a unit and already counts flaws_confirmed in instances, and the per-instance rule recorded in ADR-0011's amendments binds this producer.
2. For a pair artifact, an injected instance is confirmed only when both of its records pass their checks. Record this with 0.2.0's amendment.
~~~

**Where it is recorded.** Here; STATE (*Standing rulings*); and ADR-0011 *Amendments*, with
0.2.0's amendment, as item 2 directs. NEXT step 2 says a later ruling is recorded before anything
else, so this record is the branch's first commit.

**Effect.**
- Item 1 closes the clause of ruling 3 on PR 26 that ADR-0001's wording "is clarified if it
  needs it": ADR-0001 is not edited. The per-instance rule lives in ADR-0011's amendments and
  binds `s2_template` from 0.2.0 on.
- Item 2 decides the reading Session 40 flagged (Standing escalations): a pair's injected
  instance is confirmed only when both of its records pass their checks, the informal structural
  check and the formal check K. 0.2.0 implements it, and the open point is closed.

### Entry 4 — `s2_template` 0.2.0, built (S2 ordering step 4, continued)

All on branch `claude/s2-template-0.2.0`, from `main` `de3e4a6`:
- **Commit 1** `da2bd83b84b3ff5202ca900c9bcff9bf6f9eddb8`: the owner note recorded before
  anything else — ADR-0011 *Amendments* (with 0.2.0's amendment as decided), STATE (*Standing
  rulings*, *Standing escalations*) and this block's Entries 1–3.
- **Commit 2** `af28cc96aa75cbabb0a6171d4c636a8cbc87f773`: the 0.2.0 producer — build-time input
  hashes (`input-set.txt`, `build.rs`, `src/provenance.rs`; FINDING-005 closed), the version
  selection (0.1.0 and 0.2.0 in one crate; `generate` emits 0.2.0; `regenerate` and `verify`
  accept the version a manifest names), counts per instance, the seeds
  `continuity_is_not_uniform`, `x_over_abs_x`, `rolle_abs` and `horses` with their witness
  modules, `Syndeom.lean`, `Audit.lean` (eight lines) and the tests. `Cargo.lock` unchanged.
- **Commit 3** `d7b5d8185ef683ef3618f01a2d9cc6ba7983cee4`: the 14 goldens of 0.2.0, produced by
  `generate --dry-run` at commit 2 with the Lean harness run one process at a time
  (2026-10-28T18:58:21Z–19:00:08Z), and the goldens README.
- **Commit 4** (this one): the packet `analysis/s2-generate-2/`, STATE, and this close.

The evidence, the security review and readings 13–20 are in the packet's README. Every receipt is
under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-020-20261028T1830Z\` (the probes, the 14
golden receipts whose sorted SHA-256 list hashes to `184058ad…3919`, the lake build log and
`cargo audit`), and the orientation receipts under
`C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1815Z\`.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN. S2.1–S2.4 are in progress: this is the second
implementation session (ADR-0011 D11). Gate S1 is CLOSED (Session 29).
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipt `orient-1.json`, SHA-256
`88b6727c…d782`, under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1815Z\`):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-28T18:15:15Z–18:15:16Z, exit 0.
- **Snapshot.** `main` at `de3e4a62…`, tree `4117ac9d…`, **synchronized**: ahead 0, behind 0. The
  only change was the disclosure append (`allowed_present`; `unrelated: []`). No worktree.
- **Step 5.** The constitution (`110f5360…`), ADR-0005 (`b08e543a…`) and ADR-0008 (`ccdfa9d7…`),
  as read in step 0, equal the snapshot's blobs.
- **Step 6.** Read in this order:
  - STATE (the Updated bullets of Sessions 37–40, the freeze register, memory policy, standing
    rulings, the gate tables, blockers, standing escalations, NEXT);
  - HANDOFF Session 40 in full (Entries 1–4, the close and the addendum);
  - CLAIMS: 0001–0005 VERIFIED, 0006 CONDITIONAL, 0007 VERIFIED (existence only);
  - FLAWS `0.1.0` (14 entries); SOURCES through S-0081;
  - ADR-0008, ADR-0011 with its amendments, ADR-0001 in full; the crate's sources, tests, seeds
    and goldens, the witness modules, the gate script, `syndeom-pair`'s `build.rs` and
    provenance module, and implementation packet 1.

  They agree with one another and with the tree.
- **ORIENT gate GREEN**, 18:18:36Z–18:19:30Z (log SHA-256 `24eb6474…b735`, 16,438 bytes): 8,929
  Lean jobs; four axiom lines within the standard three; fmt, clippy and 116 tests pass; every
  tracked text file LF; the registers and the 65-row freeze register ALL PASS.
- **NEXT survived ORIENT.** PR 26 was merged by Al with the merge commit `de3e4a6` at
  2026-10-28T18:10:24Z, and `21f8359` is an ancestor of `main`. The owner note was the only later
  ruling; it was recorded before anything else (Entry 3; commit 1).
ADVANCED:
- **`s2_template` 0.2.0** (S2.1–S2.4 in progress): the running binary's provenance (ruling 5;
  FINDING-005 closed), counts per instance with the pair rule (ruling 3; the owner note), four
  seeds from the FLAWS.md examples with their Lean witnesses, 14 goldens, and the 0.1.0 goldens
  regenerable under 0.1.0.
- **The gate's axiom audit** now covers eight declarations, four of them negations of false
  statements.
- **`cargo audit`** is clean, and the security review of the new code is recorded.
BELIEF DELTAS:
- **Three of the four examples named for 0.2.0 are false theorems**, in Lean's reading and in
  FLAWS.md's own words: Rolle on `|x|` ("no such c exists"), the horses, and `x/|x| = ±1` at
  `x = 0`. ADR-0001's consistency rule (`value == false ⇒ proof_status == "invalid"`) therefore
  forbids a control for them; D3's "every seed theorem has its control" cannot hold for a false
  seed. Evidence: the negation witnesses build; the self-check.
- **Mathlib's Rolle needs no differentiability** (`exists_deriv_eq_zero`), because `deriv` is `0`
  where a function is not differentiable; the `deriv`-form claim about `|x|` would be true. The
  seed is in `HasDerivAt` form, which is false. Evidence: `Rolle.lean` 54–64, `Deriv/Abs.lean`
  184, the witness.
- **A structured `cases … with` reports a missing alternative at the `cases` line**, so the
  omitted-case shape of FLAW-0014 has a formal site; `rcases` with too few patterns would rename
  instead. Evidence: `xoa_convincing` at `4:2`.
- **`omega`'s "No usable constraints found" is its message for a satisfiable system** here, not
  for an empty hypothesis set. Evidence: `omega_test.lean` (`0 < m` fails, `1 < m` closes).
- **Build-time input hashing is cheap**: a rebuild on any input change, and a 4.6 s test build.
CONFUSIONS:
- **The horses membership proof**: `a ∈ s.erase b` needs `⟨hab, ha⟩`, not `hb`. Cost: one probe
  rerun (8 s). The record keeps the first run's exit codes in the probe summary.
- **`norm_num` is not available from `Mathlib.Data.Finset.Card` alone**; the horses witness uses
  `decide`. Cost: the same rerun.
- **The block header's form**: the existing blocks begin with a `SESSION:` line, not a Markdown
  heading; the provisional header was a `## ` line. Cost: none (the header is replaceable by
  design), and the completed header follows the `SESSION:` form. ADR-0008 step 0 and ADR-0005
  *Amendments* 3 could name the form; noted, not changed.
HONESTY CHECK:
- No register status changed. The packet's measurements are implementation evidence, not CLAIMS.
- The goldens' observations are what the harness measured at `af28cc9`; nothing was hand-edited.
- The horses golden's first error is at line 20 while its formal record's location is line 17,
  the fragment's first line (ADR-0011 D5); K holds because 20 lies within the fragment's lines
  17–20. The other three first errors are at their fragments' first lines.
- For an intrinsic record the informal check confirms only that the recorded text is present at
  the recorded span; the substantive confirmation is K on the formal file. The informal-only
  artifacts of the three false seeds are confirmed by that structural check alone, as 0.1.0's
  informal artifacts were.
- The first probe run's two slips are recorded in the packet; the corrected probes, not the
  first ones, are the evidence.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened; no `groundtruth/` item was read for
design; the seeds come from FLAWS.md 0.1.0 only (D6, ruling 1); the Mathlib files read are library
code at the pinned commit; no real AI-generated proof was used. Verdict: no leakage.
DETERMINISM CHECK:
- Every golden regenerates byte for byte from its config, seed and recorded observation, under
  the version its manifest names; two `prepare` calls agree; the RNG vectors are pinned.
- No wall-clock value enters an artifact; receipts carry durations only.
- Nothing entered CLAIMS, so there is nothing to quarantine.
MANIFEST CHECK:
- Every golden manifest passed `Manifest::check` with its bytes before it was written: `origin`
  synthetic, `derivation` by-construction; the three false statements carry `value: false` with
  the declaration that proves the negation, `proof_status: "invalid"`, `correct: false`.
- Nothing synthetic is labeled real.
- "Proven" was applied to nothing. The witnesses are `lean-kernel` references audited by the
  gate, not CLAIMS.
DRIFT CHECK:
- No verdict on any paper (SD3), and no understanding claims (SD17).
- No detector was chosen or tuned, and there was no referee contact (SD13).
- No S3 work, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. After this pull request merges, with any ruling recorded first,
`s2_template` 0.3.0: the T check (the audit tail's used-constants listing and its Rust closure,
lemma steps in the plan) with the tooling-caught classes, `n_lt_two_pow` (FLAW-0009) and
FLAW-0007 `unused`.
ESCALATIONS:
- **This pull request:** review and merge with a merge commit, so that `af28cc9`, the 0.2.0
  goldens' `engine_commit`, stays a real commit on `main`.
- **Readings 13–20** in `analysis/s2-generate-2/README.md`, and the two readings in ADR-0011
  *Amendments* (seeds whose statement is false; FLAW-0006's mode for the horses).
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. The 0.2.0 scope STATE's NEXT named is built, and the pull request
is open for Al's review. Repo green: the gate on the committed tree is recorded in the pull
request and under the external root, not in the tree it certifies.
**Memory writes:** none. A write would change a memory digest and make the next disclosure owe
verbatim reproduction.

SESSION 41 ADDENDUM — AL'S RULINGS ON PR 27 (2026-10-28 UTC, received in this session's
conversation after PR #27 opened; recorded at Al's direction in new commits on the same branch,
no force-push; transcribed verbatim from Al's message as rendered to the executor; copy
`owner-rulings-pr27.txt` under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-020-20261028T1830Z\`,
1,443 bytes, SHA-256 `c465f9e5cecb42677e958cb950f3249492c6d75251901ffa6fd343d7aceb6b8a`):

~~~text
Rulings on PR #27 / s2_template 0.2.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 13, 15, 16, 17, 18, 19 and 20 are accepted as read.

Reading 14 (FLAW-0006's mode for the horses) is not accepted as shift-index. D5's modes {drop-base, shift-index, strong-weak} name no case that fits, as the amendment says, and shift-index asserts an index shift the horses argument does not contain: its induction is set up correctly, and the defect is that the overlap step is valid only for n+1 >= 3, leaving the first step (1 -> 2) unjustified -- the kind FLAWS.md's own prose names ("proves the step only for n above the base"). Extend D5's FLAW-0006 mode enum with a fourth value, step-above-base, defined as "the inductive step's argument is valid only for n above the base, so the base -> base+1 step is unjustified; failing_case names that first unjustified step." Relabel the horses record's dials.mode (in the seed and in all three horses goldens) to step-above-base, keep failing_case "n + 1 = 2", and regenerate so the 0.2.0 goldens share one engine_commit. The horses proof bytes do not change (the mode is descriptive for intrinsic material); only the manifests do. Confirm two runs are byte-identical and report the engine_commit the regenerated goldens name.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

Before applying the rulings, a fresh fetch at 2026-10-28T19:27:00Z showed `origin/main` still at `de3e4a6`
and the branch at `20f8033`, in sync with its remote.

How each ruling is applied:
- **Readings 13, 15–20 accepted** → recorded in ADR-0011 *Amendments*, STATE and the packet.
- **Reading 14 replaced** → D5's FLAW-0006 mode enum gains `step-above-base`, with its
  definition, in ADR-0011 *Amendments* (the D5 row's mode set is superseded on that point); the
  horses seed record's `dials.mode` is relabeled with `failing_case` kept; the refusal case in
  `tests/properties.rs` and the crate README follow. The horses proof bytes do not change: the
  seed's steps, fragments and witness are untouched, and only the material's dials differ.
- **Regeneration** → the 14 goldens of 0.2.0 are regenerated at the commit that holds the
  relabeled seed (the first of these commits), by two separate runs of `generate --dry-run`
  whose outputs are compared byte for byte before run A's are placed under
  `engine/generate/tests/goldens/`. The second part of this addendum records the engine commit
  they name, the two runs, the gate on the committed tree, the recheck and the push.

**The rulings applied (second part of the addendum).**
- **Commit 5** `c899b0732db8b7a1a104822772a428f534a651cd`: the rulings recorded verbatim (ADR-0011
  *Amendments*, STATE, this addendum's first part); D5's FLAW-0006 mode enum extended with
  `step-above-base` in ADR-0011 *Amendments*; `seeds/horses.json` relabeled
  (`mode: "step-above-base"`, `failing_case: "n + 1 = 2"` kept; steps, fragments and witness
  untouched); the refusal case in `tests/properties.rs` and the crate README following; the three
  horses goldens, whose configs named the old mode, removed and the goldens test's 0.2.0 count
  pinned at 11 so that the commit's tree stays green.
- **Regeneration at commit 5**, with the binary rebuilt from its tree: two separate runs of
  `generate --dry-run` for all 14 cases of 0.2.0 (`evidence/goldens-run2.py`; run A
  2026-10-28T19:28:06Z–19:30:19Z, run B 19:30:19Z–19:32:14Z; every case exit 0, fidelity PASS;
  the provenance check passed against commit 5 before each run). `evidence/goldens-copy2.py`
  compared the runs file by file — every `proof.md`, `Proof.lean` and `manifest.json`
  byte-identical, every observation equal, one engine commit — and replaced the 0.2.0 cases
  under `engine/generate/tests/goldens/` with run A's outputs
  (`evidence/goldens-regeneration-report.json`, SHA-256 `cb8aba70…d2ad`, 9,130 bytes).
  - **The engine commit the regenerated goldens name: `c899b0732db8b7a1a104822772a428f534a651cd`.**
  - The horses proof bytes are unchanged from the first production (`Proof.lean` `9e198c7c…`,
    `proof.md` `d516f093…`); its manifests, config hashes and artifact ids changed
    (`22c3147d9d48` informal, `05964cd1eb74` formal, `7b76255e38a5` pair).
  - The other eleven cases differ from the first production only in the engine commit they
    name (22 files, one line each).
  - The receipts are under the external root's `goldens-2a\` and `goldens-2b\`; their sorted
    `receipt.json` SHA-256 lists hash to `a79a05b9…f0ee` and `872b85fe…0e92`.
- **Commit 6** (this one): the regenerated goldens, the goldens test expecting 14 again, the
  goldens README, the packet's regeneration record and evidence, STATE, and this part.

CHECKS for this addendum:
- No leakage: `contact/` was not opened, and no source was added.
- No proof byte, label, register status or 0.1.0 golden changed; the 0.2.0 manifests changed
  under Al's ruling, which authorizes the regeneration.
- No verdict on any paper (SD3). **Memory writes:** none.
ESCALATIONS (superseding the block's):
- **This pull request:** review and merge with a merge commit, so that `c899b07`, the 0.2.0
  goldens' `engine_commit`, stays a real commit on `main`.
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction, stop for review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root, not in the tree it certifies.

SESSION: 2026-10-28 UTC (2026-10-28 America/New_York) (Session 42 — a fresh conversation launched
from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone; SD15 disclosure
first, by digest (ADR-0005 *Amendments* 1–3); not an examination — Gate S2 has nothing to
examine; examined fetched `origin/main` `7a05e75ed5ed872f92eb4453d353616c2f1d1dbf`, tree
`c95ea6fb1c3fb8733d90539874719a610674c832`, in the original checkout `C:\Users\LEET\Syndeom` on
`main`, synchronized; branch `claude/s2-template-0.3.0`; header completed after ADR-0008 step 5 —
it was written provisionally with the disclosure, which is unchanged; the session-end SHA is the
commit carrying this block, recorded in its pull request)

### Entry 1 — SD15 memory disclosure, by digest (ADR-0005 D1–D4 as amended by Amendments 1–3)

Appended by shell append (`cat <scratchpad file> >> PROGRAM/HANDOFF.md`) without reading HANDOFF's
existing contents. Executor: Claude Code on Claude Fable 5.1 (model id `claude-fable-5-1`). The
launching message was the §6 Session-N resume prompt verbatim and nothing else; the conversation
context carried no program state. Launched from `C:\Users\LEET` (not a git work tree): the harness
loaded the `C--Users-LEET` memory index into context as a system reminder and nothing else from the
memory store; no gitStatus snapshot was attached.

Actions before this append, in order, and nothing else: (1) `cat PROGRAM/SYNDEOM-PROGRAM.md`, read
in full (500 lines); (2) `cat PROGRAM/DECISIONS/ADR-0005.md`, read in full through Amendment 3;
(3) a hashing script over the memory directory only
(`C:\Users\LEET\.claude\projects\C--Users-LEET\memory\`), run at 2026-10-28T19:53:27Z; (4) `cat` of
`project_syndeom.md`. No other repository file, no listing of the checkout's directories, no Git
command, no build. Memory was read, not written; no memory write will be made this session.

**Whole index (`MEMORY.md`), recorded as a fact (ADR-0005 Amendment 2):**
- SHA-256 `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`; 15,420 bytes;
  110 lines (`wc -l` and awk `NR` agree; final byte is LF); 0 lines carry CR; `file(1)`: UTF-8 text.
- Identical to the ADR-0005 Amendment 2 baseline (Session 37) and to Session 39's recorded facts.

**D3 check — case-insensitive search of the whole index for `syndeom`, `kober`, `friedman`**
(awk `tolower($0) ~ /syndeom|kober|friedman/`; the FRIEDMAN sibling's index lines spell the repo
FR13DMAN but name FRIEDMAN in text, so they match): matched line numbers **85, 86, 88, 89, 106,
107** — 6 lines (85–86 KOBER, 88–89 FRIEDMAN, 106–107 SYNDEOM). Every matched line is among the
lines whose digest is compared below; none is withheld.

**D2 — withheld by count:** 104 index lines concern unrelated programs and are withheld: line
numbers 1–84, 87, 90–105, 108–110. None of them names SYNDEOM, KOBER or FRIEDMAN (the search above
matched only the six lines listed).

**Matched index lines, digest (the Amendment 3 byte form: each matched line with its LF terminator,
concatenated in index order):**
- SHA-256 `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`; 6 lines; 1,198 bytes;
  0 CR; 30 non-ASCII bytes.
- Equal to the Amendment 2 baseline, and so to the digest of the bytes HANDOFF Session 35 Entry 1
  reproduced (HANDOFF lines 6672–6677 as recorded in ADR-0005 Amendment 2; verified by hashing in
  Session 37 Entry 2). The matched line numbers are unchanged; no renumbering.

**`project_syndeom.md` (this program's memory file), digest:**
- SHA-256 `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`; 10,957 bytes;
  135 lines (final byte LF); 0 CR.
- Equal to the Amendment 2 baseline, and so to the digest of the bytes Session 35 Entry 1
  reproduced (HANDOFF lines 6683–6817 per ADR-0005 Amendment 2).
- Read in full this session: it holds the repo location, the sibling pointers and machine/harness
  hazards only — no phase, gate status, CLAIM or FLAW status, file contents, commit SHA as a state
  marker, next action or escalation (SD15; D4).

**Disclosure mode:** both compared digests equal the digests of bytes that an earlier HANDOFF
disclosure reproduced, so this disclosure is **by digest** (ADR-0005 Amendment 1, refined by
Amendment 2, both readings confirmed as Al's by Amendment 3). Nothing is reproduced. Cited entry:
**HANDOFF Session 35, Entry 1** (the verbatim reproduction), via Session 37 Entry 2's hash
verification and the baseline ADR-0005 Amendment 2 records. This citation rests on ADR-0005's
record, not on a read of HANDOFF; a verification against HANDOFF's own bytes follows after
orientation as a later entry, so that this disclosure stays first.

**D1(c) — other recalled memory:** the harness loaded no memory other than the index. As a fact: of
the 42 files in the memory directory, only `MEMORY.md` (2 lines) and `project_syndeom.md` (8 lines)
contain the string `syndeom`, case-insensitively.

**D4:** no reproduced memory holds program state (nothing was reproduced; the SYNDEOM file was
re-read and found clean); no memory is written this session. Whether this session is an
examination is determined after orientation from STATE's NEXT-SINGLE-ACTION; SD15's
conversation-context condition is met — launched with the §6 prompt verbatim and nothing carrying
program state.

### Entry 2 — the disclosure's citation, verified after the append; the header completed

- **Cited entry:** HANDOFF Session 35, Entry 1 (heading at line 6664, "Entry 1 — SD15 memory
  disclosure (ADR-0005 D1–D4; ADR-0009)"). Checked in the working file, whose first 699,254 bytes
  are the HANDOFF blob of `main` `7a05e75` (blob SHA-256 `f8ac981d…cd49`; unchanged through this
  branch's first three commits):
  - lines 6672–6677 (1,198 bytes) hash to `0418e621…534c` and are byte-identical to index lines
    85, 86, 88, 89, 106 and 107;
  - lines 6683–6817 (10,957 bytes) hash to `ce2ce6de…2cc0` and are byte-identical to
    `project_syndeom.md` on disk.

  Neither digest is new, so nothing is owed verbatim. The matched lines kept their numbers.
- **The append.** Entry 1 with its provisional header was 4,714 bytes, SHA-256 `65355e6f…b775`,
  appended by `cat >>` at 2026-10-28T19:55:47Z (the hashing ran at 19:53:27Z). HANDOFF grew from
  699,254 to 703,968 bytes, exactly that much, and the blob is an exact prefix of the working file.
- **The header.** A script replaced the one provisional header line (`## Session — 2026-10-28 UTC
  (provisional header, completed after orientation)`) with the completed `SESSION:` header and
  asserted that the disclosure body, from `### Entry 1` to the end of the append, was unchanged,
  byte for byte.
- **Constitution and ADR bytes (ADR-0008 step 5).** As read in step 0 and as in the snapshot at
  `origin/main` `7a05e75`: the constitution `110f5360…479c` (38,958 B, ADR-0009's amended
  digest), ADR-0005 `b08e543a…ce11` (15,291 B, with *Amendments* 1–3) and ADR-0008
  `ccdfa9d7…e9ff` (18,589 B).
- **One deviation from ADR-0008 step 0, recorded.** When verifying the append (after disclosing,
  before the preflight), the session hashed HANDOFF's last 68 lines — enough — and also printed
  the twelve lines before them and the file's `## ` headings. Those twelve lines are earlier
  HANDOFF content (the end of Session 41's addendum), read before step 5 completed. The read came
  after the disclosure, so the examination clause is untouched, and the session was not an
  examination in any case (Entry 3). Cost: none. For a successor: verify an append by hashing
  the tail alone.

### Entry 3 — the launch

A fresh conversation from `C:\Users\LEET`, launched with the §6 Session-N resume prompt verbatim
and alone (no owner note). By eye the prompt matches §6's text line for line. The harness
attached no Git snapshot (`C:\Users\LEET` is not a work tree) and loaded the `C--Users-LEET`
index. Executor model: Claude Fable 5.1.

**Not an examination.** Gate S2 has nothing to examine. The session certifies nothing.

### Entry 4 — `s2_template` 0.3.0, built (S2 ordering step 4, continued)

All on branch `claude/s2-template-0.3.0`, from `main` `7a05e75`:
- **Commit 1** `59b505a320447f58c811ec394103f64ac9381a93`: the 0.3.0 producer — lemma steps in
  the plan (`LemmaHeader`, `check_lemmas`; the formal writer emits lemma declarations before the
  main one; `FormalRendering.declarations`), the audit tail over every constant the file defined
  (`AUDIT_TAIL`, `parse_used_constants`, `used_closure`, `Observation.used_constants`), check T
  for FLAW-0007 `unused` and FLAW-0009, `manifests_in`, insertion material (`inserts`,
  `from: null`), the seed `n_lt_two_pow`, the witness module `NLtTwoPow.lean` (`Syndeom.lean`,
  `Audit.lean`), the tests and the crate README. `Cargo.lock` unchanged.
- **Commit 2** `5a313ddf8de6a870c2f41c9d65d8cb87a1595b2e`: FINDING-011 — `verify` refuses a
  manifest whose `main_declaration` is not the producer's, with a CLI test.
- **Commit 3** `f0173a285a2d3797368827c1c82661019b3ec9f6`: the 9 goldens of 0.3.0, produced by
  `generate --dry-run` at commit 2 by two separate runs (2026-10-28T20:32:24Z–20:34:40Z) compared
  byte for byte, and the goldens README.
- **Commit 4** (this one): the packet `analysis/s2-generate-3/`, STATE, and this close.

The evidence, the security review and readings 21–28 are in the packet's README. Every receipt
is under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-030-20261028T2005Z\` (the seven probes,
the lake build log, `cargo audit`, the three productions — `goldens-a`/`-b` at commit 1,
`goldens-2a` refused by the provenance check, `goldens-3a`/`-b` at commit 2, whose sorted
`receipt.json` SHA-256 lists hash to `df4aee9d…c473` and `27cd18a5…b8c4`), and the orientation
receipt under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1956Z\`.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN. S2.1–S2.4 are in progress: this is the third
implementation session (ADR-0011 D11). Gate S1 is CLOSED (Session 29).
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipt `orient-receipt-1.json`, SHA-256
`240276ee…f61f`, under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T1956Z\`):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-28T19:56:30Z–19:56:31Z, exit 0.
- **Snapshot.** `main` at `7a05e75e…`, tree `c95ea6fb…`, **synchronized**: ahead 0, behind 0. The
  only change was the disclosure append (`allowed_present`; `unrelated: []`). No worktree.
- **Step 5.** The constitution (`110f5360…`), ADR-0005 (`b08e543a…`) and ADR-0008 (`ccdfa9d7…`),
  as read in step 0, equal the snapshot's blobs.
- **Step 6.** Read in this order: STATE (the Updated bullets of Sessions 39–41, the memory
  policy, the Gate S2 table, blockers, standing escalations, NEXT); HANDOFF Session 41 in full
  (Entries 1–4, the close and the addendum); CLAIMS: 0001–0005 VERIFIED, 0006 CONDITIONAL, 0007
  VERIFIED; FLAWS `0.1.0` (14 entries; FLAW-0007, FLAW-0009 and the summary table in full);
  ADR-0008, ADR-0011 in full with its amendments; the crate's sources, tests, seeds, goldens
  layout and READMEs, the witness modules, `Audit.lean`, the gate script, the manifest crate's
  parsers and self-check, implementation packet 2 and its production scripts; PR 27 on GitHub
  (merged 19:49:50Z; no review, no later comment; no open pull request or issue). They agree with
  one another and with the tree.
- **ORIENT gate GREEN**, 19:58:16Z–19:59:07Z (log `gate-orient.log`, SHA-256 `28fc78e5…3d47`,
  17,329 bytes, stdout and stderr merged): 8,933 Lean jobs; eight axiom lines within the standard
  three; fmt, clippy and every test pass; every tracked text file LF; the registers and the
  freeze register pass.
- **NEXT survived ORIENT.** PR 27 was merged by Al with the merge commit `7a05e75` at
  2026-10-28T19:49:50Z; `af28cc96` and `c899b073` are ancestors of `main`. No later ruling exists,
  so nothing was recorded before the build.
ADVANCED:
- **`s2_template` 0.3.0** (S2.1–S2.4 in progress): lemma steps (D2), the used-constants audit
  tail and check T, the first tooling-caught classes confirmed live — FLAW-0009 `axiom` (gap and
  fatal) and FLAW-0007 `unused` — on the seed `n_lt_two_pow`; nine goldens from two
  byte-identical runs; 27 tests; clippy and audit clean.
- **The gate's axiom audit** now covers ten declarations.
- **FINDING-011** (High) found by the D10 review and closed in the same pull request.
BELIEF DELTAS:
- **Lean lists its own auxiliary constants** (`…lemma_1._proof_1_1`, from `omega`) among a
  theorem's used-constants, so a tail that enumerates only the declarations the Rust side knows
  would miss a use routed through one; the tail now enumerates the environment's second-stage
  constant map. Evidence: `nltp_unused.out`, `nltp_tail_map2.out`.
- **`n < 2 ^ n` and `n + 1 ≤ 2 ^ n` are definitionally equal in Lean**, so `exact lemma n`
  closes the conclusion with no conversion lemma. Evidence: `nltp_faithful`, exit 0.
- **The probes compile in 3–4 s** with the single import `Mathlib.Data.Nat.Notation`; a formal
  case of the production takes about 9 s (two compiles).
- **`verify` was injectable** through the manifest's `main_declaration` since 0.1.0: the bytes
  check (`Proof.lean` equals what the producer emits) did not cover the name the audit file
  interpolates. Evidence: the CLI test's `injected` case.
CONFUSIONS:
- **A `cat -v | grep -F '^M'` check for CR matches a literal caret followed by `M`**, as in
  `g^{…}` next to an escaped byte: two "CR lines" in HANDOFF were a false positive
  (`git ls-files --eol` says `i/lf`). Cost: one grep. Use `git ls-files --eol` or `od`.
- **The goldens test's pin lives in the input set**, so a pin changed in the working tree makes
  every production refuse (`engine_commit does not identify … goldens.rs`). Cost: one refused
  run (kept as `goldens-2a`). The order is: commit the producer with the pin at the old count,
  produce, then pin with the goldens.
- **The copy script's summary print assumed an observation for every case**; informal-only
  cases have none. Cost: one re-run of the placement. Fixed in the script, with the probe
  comparison's off-by-one (it kept the blank line's LF).
- **ADR-0008 step 0's "nothing else of the checkout before step 5"** was crossed by twelve lines
  of earlier HANDOFF read while verifying the append (Entry 2). Hash the tail only.
HONESTY CHECK:
- No register status changed. The packet's measurements are implementation evidence, not CLAIMS.
- The goldens' observations are what the harness measured at `5a313ddf…`; nothing was
  hand-edited. The first production at `59b505a3…` yielded the same proof bytes and ids and is
  kept as a record, superseded only so that the goldens name the commit holding FINDING-011's fix.
- FLAW-0009 `truth: false` is recorded as fatal with the axiom stating a false lemma that the
  witness module refutes; the informal file is the faithful one because the class manifests in
  the formal file only (FLAWS.md's column, applied). A reader expecting an informal trace of the
  smuggled axiom will not find one.
- The property tests' predicted control observation now lists `[propext, Quot.sound]`, the
  observed form; no artifact byte depends on it.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened; no `groundtruth/` item was read for
design; the seed comes from FLAWS.md 0.1.0 only (D6, ruling 1); the Lean core and Mathlib names
used are library code at the pinned toolchain; no real AI-generated proof was used. Verdict: no
leakage.
DETERMINISM CHECK:
- Every golden regenerates byte for byte from its config, seed and recorded observation under
  the version its manifest names; two productions at one commit agree file by file; the RNG
  vectors are pinned.
- No wall-clock value enters an artifact; receipts carry durations only.
- Nothing entered CLAIMS, so there is nothing to quarantine.
MANIFEST CHECK:
- Every golden manifest passed `Manifest::check` with its bytes before it was written: `origin`
  synthetic, `derivation` by-construction; the FLAW-0007 `unused` artifacts are
  `proof_status: valid` and `correct: true` (cosmetic only); the FLAW-0009 artifacts `invalid`,
  `correct: false`, with `lean_axioms_standard` commanded and achieved `false`.
- Nothing synthetic is labeled real.
- "Proven" was applied to nothing. The witnesses are `lean-kernel` references audited by the
  gate, not CLAIMS.
DRIFT CHECK:
- No verdict on any paper (SD3), and no understanding claims (SD17).
- No detector was chosen or tuned, and there was no referee contact (SD13).
- No S3 work, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. After this pull request merges, with any ruling recorded first,
`s2_template` 0.4.0: the padding module (FLAW-0013; D2's `pad`, D5, Options 8) on the seed
`sqrt2_irrational` (D6), with the padded control and the FLAWS.md example as goldens.
ESCALATIONS:
- **This pull request:** review and merge with a merge commit, so that `5a313ddf…`, the 0.3.0
  goldens' `engine_commit`, stays a real commit on `main`.
- **Readings 21–28** in `analysis/s2-generate-3/README.md`, and **FINDING-011** (High; fixed in
  this pull request; present since 0.1.0).
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. The 0.3.0 scope STATE's NEXT named is built, and the pull request
is open for Al's review. Repo green: the gate on the committed tree is recorded in the pull
request and under the external root, not in the tree it certifies.
**Memory writes:** none. A write would change a memory digest and make the next disclosure owe
verbatim reproduction.

SESSION 42 ADDENDUM — AL'S RULINGS ON PR 28 (2026-10-28 UTC, received in this session's
conversation after PR #28 opened; recorded at Al's direction in a new commit on the same branch,
no force-push; transcribed verbatim from Al's message as rendered to the executor; copy
`owner-rulings-pr28.txt` under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-030-20261028T2005Z\`,
764 bytes, SHA-256 `798ec70baacbd1d7716cadcbeac39213815878afc7c766123709e8194368e8ed`):

~~~text
Rulings on PR #28 / s2_template 0.3.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 21, 22, 23, 24, 25, 26, 27 and 28 are accepted as read.

On reading 24, record also: FLAWS.md's FLAW-0009 recipe ("an axiom of the same statement; dial true | false") is read as same statement for truth: true, and for truth: false a false statement from which the main theorem still follows, with a witness module proving its negation; the record carries the replaced lemma and the inserted axiom so the modality mismatch in a pair is explicit in the ground truth. FLAWS.md's text is left as it is.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

Before applying the rulings, a fresh fetch at 2026-10-28T21:05:07Z showed `origin/main` still at
`7a05e75` and the branch at `416fa8e`, in sync with its remote.

How each ruling is applied:
- **Readings 21–28 accepted** → recorded in ADR-0011 *Amendments*, STATE and the packet.
- **Reading 24, recorded as ruled** → ADR-0011 *Amendments* (the D5 FLAW-0009 reading: the same
  statement for `truth: true`; for `truth: false` a false statement from which the main theorem
  still follows, its negation proved by the witness module; the record carrying the replaced
  lemma and the inserted axiom). The 0.3.0 formal record already carries both — `replaced` holds
  the faithful lemma's declaration lines and `inserted` the axiom line — so no byte changes, no
  seed or golden regenerates, FLAWS.md is untouched, and the producer version stays 0.3.0.
- **Commit 5** (this one): the records — ADR-0011 *Amendments*, STATE (*Standing rulings*,
  *Standing escalations*, the Updated bullet, the Gate S2 row), the packet README, and this
  addendum. The gate on the committed tree, the step-7 recheck and the push are recorded in the
  pull request and under the external root.

CHECKS for this addendum:
- No leakage: `contact/` was not opened, and no source was added.
- No proof byte, label, register status or golden changed; FLAWS.md unchanged.
- No verdict on any paper (SD3). **Memory writes:** none.
ESCALATIONS (superseding the block's):
- **This pull request:** review and merge with a merge commit, so that `5a313ddf…`, the 0.3.0
  goldens' `engine_commit`, stays a real commit on `main`.
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction, stop for review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root, not in the tree it certifies.

SESSION: 2026-10-28 UTC (2026-10-28 America/New_York) (Session 43 — a fresh conversation launched
from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone; SD15 disclosure
first, by digest (ADR-0005 *Amendments* 1–3); not an examination — Gate S2 has nothing to
examine; examined fetched `origin/main` `4cd1f7724f5d8ee1097f0afbc6d1ceafb0220f62`, tree
`f9c8e1d03df23ef34cf1db417c4e43318062ac60`, in the original checkout `C:\Users\LEET\Syndeom` on
`main`, synchronized; branch `claude/s2-template-0.4.0`; header completed after ADR-0008 step 5 —
it was written provisionally with the disclosure, which is unchanged; the session-end SHA is the
commit carrying this block, recorded in its pull request)

### Entry 1 — SD15 memory disclosure, by digest (ADR-0005 D1–D4 as amended by Amendments 1–3)

Appended 2026-10-28 UTC by shell append (`cat >>`), without reading HANDOFF's existing contents. Digests computed at 2026-10-28T21:17:43Z.

**Launch.** This session was launched with the §6 Session-N resume prompt verbatim and nothing else: no owner note, and no program state in the launching message. The conversation context is fresh; no SYNDEOM work preceded the prompt in it. Executor model: Claude Fable 5.1. The harness recalled into context exactly one memory, the machine-wide executor memory index `MEMORY.md`; no other memory file's content was loaded.

**Actions before this append, in order.** (1) One shell command reading `PROGRAM/SYNDEOM-PROGRAM.md` and `PROGRAM/DECISIONS/ADR-0005.md` in full; the harness persisted the oversized output to its own session file, which the executor then read in full (the same bytes, not a further repository read). (2) One script that hashed the memory index, the matched index lines and `project_syndeom.md`, scanned every other file in the memory directory for the program's name, and printed the UTC time; a stray redirect in the same call to an undefined path failed with "Permission denied" and wrote nothing. No other repository file was read, no directory listing was taken, no Git history or status was run, and nothing was built. No memory was written before this append, and none will be written during this session.

**Whole index, as a fact.** `C:\Users\LEET\.claude\projects\C--Users-LEET\memory\MEMORY.md`: SHA-256 `cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`, 15,420 bytes, 110 lines, 0 CR, 334 non-ASCII bytes, final byte LF. Unchanged from the baseline recorded in ADR-0005 Amendment 2 (Session 37).

**Name search (D3).** Case-insensitive search of every index line for `syndeom`, `kober`, `friedman` and, additionally, `fr13dman` (FRIEDMAN's repository name), by `awk '{l=tolower($0); if (l ~ /syndeom|kober|friedman|fr13dman/) print NR}'`. Matched line numbers: 85, 86, 88, 89, 106, 107 (six lines). Per name: syndeom → 106, 107; kober → 85, 86, 89, 106; friedman → 88, 89, 106; fr13dman → 88, 89, 107. Every matched line is among the six lines whose digest is recorded below; no withheld line names the program or a sibling.

**Matched index lines.** Lines 85, 86, 88, 89, 106 and 107, each with its LF terminator, concatenated in index order: 6 lines, 1,198 bytes, 0 CR, 30 non-ASCII bytes, SHA-256 `0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`. Equal to the baseline in ADR-0005 Amendment 2, which equals the digest of HANDOFF lines 6672–6677 as reproduced by Session 35, Entry 1. The line numbers are unchanged; no renumbering occurred.

**Withheld (D2).** 104 index lines, occupying line numbers 1–84, 87, 90–105 and 108–110. They concern unrelated private programs and are withheld by count because this repository is public.

**This program's memory file.** `C:\Users\LEET\.claude\projects\C--Users-LEET\memory\project_syndeom.md`: SHA-256 `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, 10,957 bytes, 135 lines, 0 CR, 71 non-ASCII bytes, final byte LF. Equal to the baseline in ADR-0005 Amendment 2, which equals the digest of HANDOFF lines 6683–6817 as reproduced by Session 35, Entry 1.

**Other recalled memory (D1 c).** None. A case-insensitive scan of every other `.md` file in the memory directory for `syndeom` matched nothing.

**Disclosure.** Both digests that Amendment 2 names, the matched index lines and `project_syndeom.md`, equal the digests of bytes that HANDOFF Session 35, Entry 1 reproduced verbatim. The disclosure is therefore by digest, and nothing is reproduced here. Citation: HANDOFF Session 35, Entry 1, taken from ADR-0005 Amendment 2's baseline. Because this entry is appended without reading HANDOFF, the direct check that hashes those HANDOFF lines is deferred to a later entry of this block, as Session 37's Entry 2 did.

**Pass criterion (D4).** The index line for this program states in its own text that no program state is recorded in it or in the linked file, and both files' bytes are unchanged since Session 35's reproduction. Whether this session attempts a gate's cold-start test is unknown at this point, STATE being unread; the disclosure is made regardless (SD15 as amended by ADR-0009). The launch satisfies SD15's conversation-context corollary: fresh context, the §6 prompt verbatim, nothing carrying program state.

### Entry 2 — the disclosure's citation, verified after the append; the header completed

- **Cited entry:** HANDOFF Session 35, Entry 1 (the verbatim reproduction). Checked in the
  working file after ADR-0008 step 5, whose first 720,162 bytes are the HANDOFF blob of `main`
  `4cd1f77` (unchanged through this branch's first two commits):
  - lines 6672–6677 (1,198 bytes) hash to `0418e621…534c`, the digest of index lines 85, 86, 88,
    89, 106 and 107 with their LF terminators;
  - lines 6683–6817 (10,957 bytes) hash to `ce2ce6de…2cc0`, the digest of `project_syndeom.md`
    on disk.

  Neither digest is new, so nothing is owed verbatim. The matched lines kept their numbers.
- **The append.** Entry 1 with its provisional header was 4,603 bytes, SHA-256 `626f5be8…9a23`,
  appended by `cat >>` between 2026-10-28T21:17:43Z (the hashing) and 21:19:54Z (the preflight's
  start). HANDOFF grew from 720,162 to 724,765 bytes, exactly that much, and the blob is an exact
  prefix of the working file.
- **The header.** A script replaced the one provisional header line (`## SESSION: 2026-10-28 UTC
  (provisional header, completed after orientation)`) with the completed `SESSION:` header and
  asserted that the disclosure body, from `### Entry 1` to the end of the append, was unchanged,
  byte for byte; its digest and length are in the script's output under the external root.
- **Constitution and ADR bytes (ADR-0008 step 5).** As read in step 0 and as in the snapshot at
  `origin/main` `4cd1f77`: the constitution `110f5360…479c` (38,958 B, ADR-0009's amended
  digest), ADR-0005 `b08e543a…ce11` (15,291 B, with *Amendments* 1–3) and ADR-0008
  `ccdfa9d7…e9ff` (18,589 B).
- **Between the disclosure and step 5** the session read ADR-0008 (allowed) and the executor's
  own `project_syndeom.md` (memory, not the checkout), and ran the preflight. Nothing else of
  the checkout was read before step 5 completed; the append was verified afterwards by hashing
  the tail alone, as Session 42's note for a successor asked.

### Entry 3 — the launch

A fresh conversation from `C:\Users\LEET`, launched with the §6 Session-N resume prompt verbatim
and alone (no owner note). By eye the prompt matches §6's text line for line. The harness
attached no Git snapshot (`C:\Users\LEET` is not a work tree) and loaded the `C--Users-LEET`
index. Executor model: Claude Fable 5.1.

**Not an examination.** Gate S2 has nothing to examine. The session certifies nothing.

### Entry 4 — `s2_template` 0.4.0, built (S2 ordering step 4, continued)

All on branch `claude/s2-template-0.4.0`, from `main` `4cd1f77`:
- **Commit 1** `b54e2f16aa3c153258bf6ece67798bd270993cbb`: the 0.4.0 producer — padding steps in
  the plan (`PaddingKind`, `PaddingHeader`, `Step.padding`, `padding_lines`; the formal writer
  emits them as `theorem` declarations before the main declaration), the padding module `pad`
  (a FLAW-0013 spec with dials `kinds`, `count`, `placement` and a positive `padding_ratio`;
  paragraphs and, under `drawn`, positions drawn from the `padding` stream; one instance per
  paragraph), the padded records and `padding_ratio` in `assemble` (check T shared with
  FLAW-0007 `unused` as `unused_in_file`), the seed `sqrt2_irrational` with seven padding
  paragraphs, the witness module `Sqrt2Irrational.lean` (`Syndeom.lean`, `Audit.lean` eleven
  lines), the tests and the crate README. `Cargo.lock` unchanged.
- **Commit 2** `cc868a3cef8648059df71ed888493765214fe04a`: the 8 goldens of 0.4.0, produced by
  `generate --dry-run` at commit 1 by two separate runs (2026-10-28T21:51:09Z–21:54:28Z)
  compared byte for byte, the goldens README and the count pinned in `tests/goldens.rs`.
- **Commit 3** (this one): the packet `analysis/s2-generate-4/`, ADR-0011 *Amendments*, STATE
  and this close.

The evidence, the security review and readings 29–38 are in the packet's README. Every receipt
is under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-040-20261028T2136Z\` (the three probes,
the lake build log, `cargo audit`, the ratio probe, the two productions `goldens-4a`/`-4b`,
whose sorted `receipt.json` SHA-256 lists hash to `4ab0cf60…8f25` and `a8ecdfd7…6aaa`, the
production report `207f5a8e…a971`, and the ORIENT gate log), and the orientation receipt under
`C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T2119Z\`.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN. S2.1–S2.4 are in progress: this is the fourth
implementation session (ADR-0011 D11). Gate S1 is CLOSED (Session 29).
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipt `orient-1.json` under
`C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T2119Z\`):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-28T21:19:55Z–21:19:59Z, exit 0.
- **Snapshot.** `main` at `4cd1f772…`, tree `f9c8e1d0…`, **synchronized**: ahead 0, behind 0. The
  only change was the disclosure append (`allowed_present`; `unrelated: []`). No worktree.
- **Step 5.** The constitution (`110f5360…`), ADR-0005 (`b08e543a…`) and ADR-0008 (`ccdfa9d7…`),
  as read in step 0, equal the snapshot's blobs.
- **Step 6.** Read in this order: STATE (the Updated bullets of Sessions 37–42, the memory
  policy, the standing rulings, the Gate S2 table, blockers, standing escalations, NEXT); HANDOFF
  Session 42 in full (Entries 1–4, the close and the addendum); CLAIMS: 0001–0005 VERIFIED, 0006
  CONDITIONAL, 0007 VERIFIED; FLAWS `0.1.0` (14 entries; FLAW-0007, FLAW-0011, FLAW-0013,
  FLAW-0014 and the summary table in full); ADR-0008, ADR-0011 in full with its amendments; the
  crate's sources, tests, seeds, goldens layout and READMEs, the witness modules, `Audit.lean`,
  the gate script, the manifest crate, implementation packet 3 and its production scripts; the
  pinned core and Mathlib sources for the lemma names the new seed uses; PR 28 on GitHub
  (merged by Al 21:12:20Z with the merge commit `4cd1f77`; no review, no later comment; no open
  pull request or issue). They agree with one another and with the tree.
- **ORIENT gate GREEN**, 21:21:03Z–21:22:00Z (log `gate-orient.log`, SHA-256 `583dec7e…a787`,
  17,471 bytes, stdout and stderr merged): 8,934 Lean jobs; ten axiom lines within the standard
  three; fmt, clippy and every test pass; every tracked text file LF; the registers and the
  freeze register pass.
- **NEXT survived ORIENT.** PR 28 was merged by Al with the merge commit `4cd1f77` at
  2026-10-28T21:12:20Z; `5a313ddf` and `caa4db2c` are ancestors of `main`. No later ruling
  exists, so nothing was recorded before the build.
ADVANCED:
- **`s2_template` 0.4.0** (S2.1–S2.4 in progress): the padding module — FLAW-0013 realized for
  the first time, with the FLAWS.md example's shape (seven paragraphs before the five-step
  argument) and a lightly padded control; eight goldens from two byte-identical runs; check T
  on padding declarations; `padding_ratio` measured; 29 tests; clippy and audit clean.
- **The ninth seed**, `sqrt2_irrational`, the first with padding material; the gate's axiom
  audit now covers eleven declarations.
- **FINDING-008** (packet 1) closed by the design: the ratio never enters a generation decision.
BELIEF DELTAS:
- **`rw [Nat.pow_mod, hp]` closes `1 ^ 2 % 2 = 1` by `rfl`** with Mathlib imported, so no
  `decide` or `simp` was needed in the parity arguments. Evidence: `s2i_faithful.out`, exit 0
  on the first compile.
- **Core `Nat` divisibility unfolds for the anonymous constructor and `obtain`:**
  `⟨q ^ 2, hpq⟩ : 2 ∣ p ^ 2` and `obtain ⟨k, hk⟩ := hp` give `hk : p = 2 * k` directly, and
  `omega` reads `2 ∣ p ^ 2` with `p ^ 2` as an atom. Evidence: the three probes.
- **`Mathlib.Tactic.Ring` costs about 8 s per compile** against 3–4 s with
  `Mathlib.Data.Nat.Notation` alone; a formal case of the production takes about 18 s (two
  compiles). Evidence: the probe and production summaries.
- **The `padding` stream at seed 7 orders the FLAWS.md example's paragraphs 3, 4, 1, 2, 7, 5, 6**
  — the stream's order, not the material's — and places the single drawn re-derivation
  (paragraph 6) before step 2 of the argument. Evidence: the goldens.
CONFUSIONS:
- **The constitution's `cat` exceeded the harness's display limit** and was persisted to a file,
  which the session read in full (Entry 1). Cost: none, but a successor should expect it.
- **A redirect before its directory variable was defined** failed with "Permission denied" on
  `/sd15_hash.sh` and wrote nothing (Entry 1). Cost: none. Define the variable first.
- **Mathlib's `Tactic/Ring.lean` at this pin uses the module system** (`module`, `public
  import`), so `grep '^import'` found nothing and an `&&` chain stopped there. Cost: one
  re-run. Grep `public import` too.
- **Clippy's `type_complexity`** refused `Vec<(Step, Option<(u64, u64, PaddingKind)>)>`; a named
  `Marker` struct replaced the tuple. Cost: one rebuild.
- **STATE's "*Current build verification*" is a cross-reference, not a section:** earlier
  Updated bullets cite it, but no heading exists. This session wrote the gate facts inline.
  Cost: one grep.
HONESTY CHECK:
- No register status changed. The packet's measurements are implementation evidence, not CLAIMS.
- The padded goldens command two-decimal ratios (0.28, 0.71; 0.31, 0.61) within `abs 0.05` of
  the achieved values, not the exact values; the packet and the goldens README say so.
- The padded goldens' formal files are not byte-identical to one probe: their declarations are
  the padded probe's blocks in drawn order, which the production report checked; the control
  golden is the faithful probe without its tail.
- The ratio probe's four artifacts carry FAIL verdicts by design (ratio 1 commanded) and are
  kept under the external root as the record, never as goldens.
- FINDING-014 was found in this session's own code and is recorded as backlog rather than fixed:
  a fix after the production would change the engine's inputs, and the goldens name
  `b54e2f16`. It is in NEXT for the next revision.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened; no `groundtruth/` item was read for
design; the seed comes from FLAWS.md 0.1.0 only (D6, ruling 1); the Lean core and Mathlib names
used are library code at the pinned toolchain; no real AI-generated proof was used. Verdict: no
leakage.
DETERMINISM CHECK:
- Every golden regenerates byte for byte from its config, seed and recorded observation under
  the version its manifest names; two productions at one commit agree file by file; the padding
  draw is the `padding` stream of `syndeom-rng/1`.
- No wall-clock value enters an artifact; receipts carry durations only.
- Nothing entered CLAIMS, so there is nothing to quarantine.
MANIFEST CHECK:
- Every golden manifest passed `Manifest::check` with its bytes before it was written: `origin`
  synthetic, `derivation` by-construction; the padded artifacts are `proof_status: valid` and
  `correct: true` with their FLAW-0013 records cosmetic (`NO` informal, `YES-TOOLING` formal).
- Nothing synthetic is labeled real.
- "Proven" was applied to nothing. The witness is a `lean-kernel` reference audited by the gate,
  not a CLAIM.
DRIFT CHECK:
- No verdict on any paper (SD3), and no understanding claims (SD17): padding is measured as a
  token ratio, a verification cost, never as pedagogical harm.
- No detector was chosen or tuned, and there was no referee contact (SD13).
- No S3 work, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. After this pull request merges, with any ruling recorded first,
`s2_template` 0.5.0: FLAW-0004 (the hedge lexicon, `hidden_step`; the aligned tactic step
omitted) on `sqrt2_irrational`'s step 3 and FLAW-0011 as a commanded composite (`embedded`)
realizing FLAWS.md's FLAW-0011 example on the same seed, with FINDING-014's `check_seed` rule.
ESCALATIONS:
- **This pull request:** review and merge with a merge commit, so that `b54e2f16…`, the 0.4.0
  goldens' `engine_commit`, stays a real commit on `main`.
- **Readings 29–38** in `analysis/s2-generate-4/README.md` (ADR-0011 *Amendments*), in
  particular 30 (how padding is commanded: `count` as the generation decision, the ratio
  measured) and 31 (`placement`), and **FINDING-014** (informational, backlog).
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. The 0.4.0 scope STATE's NEXT named is built, and the pull request
is open for Al's review. Repo green: the gate on the committed tree is recorded in the pull
request and under the external root, not in the tree it certifies.
**Memory writes:** none. A write would change a memory digest and make the next disclosure owe
verbatim reproduction.

SESSION 43 ADDENDUM — AL'S RULINGS ON PR 29 (2026-10-28 UTC, received in this session's
conversation after PR #29 opened; recorded at Al's direction in a new commit on the same branch,
no force-push; transcribed verbatim from Al's message as rendered to the executor; copy
`owner-rulings-pr29.txt` under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-040-20261028T2136Z\`,
696 bytes, SHA-256 `b9e05c65bbcdb85d7ec9c0ef7ef139c05101bbf7e7a6ea97b6abd34562f8fec7`):

~~~text
Rulings on PR #29 / s2_template 0.4.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 29 through 38 are accepted as read.

On reading 30, record also: the integer count is the generation decision because D4 forbids a floating-point value there; padding_ratio is a commanded statistic measured from the emitted file and checked at abs 0.05, and a ratio the count cannot reach is a FAIL verdict. A ratio-targeted mode, if a later version wants one, must choose its count by exact integer comparison of token quotients.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

Before applying the rulings, a fresh fetch at 2026-10-28T22:52:00Z (receipt `recheck-3.json`
under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T2119Z\`) showed `origin/main` still at
`4cd1f77` and the branch at `3aa87f7`, in sync with its remote; PR 29 had no review and no
comment.

How each ruling is applied:
- **Readings 29–38 accepted** → recorded in ADR-0011 *Amendments*, STATE and the packet.
- **Reading 30, recorded as ruled** → ADR-0011 *Amendments* (the D4/D8 reading: the integer
  `count` is the generation decision because D4 forbids a floating-point value there;
  `padding_ratio` is a commanded statistic measured from the emitted file and checked at
  `abs 0.05`; a ratio the count cannot reach is a FAIL verdict; a ratio-targeted mode, if a later
  version wants one, must choose its count by exact integer comparison of token quotients). 0.4.0
  already does what the ruling records, so no byte changes, no seed or golden regenerates, and
  the producer version stays 0.4.0.
- **Commit 4** (this one): the records — ADR-0011 *Amendments*, STATE (*Standing rulings*,
  *Standing escalations*, the Updated bullet), the packet README, and this addendum. The gate on
  the committed tree, the step-7 recheck and the push are recorded in the pull request and
  under the external root.

CHECKS for this addendum:
- No leakage: `contact/` was not opened, and no source was added.
- No proof byte, label, register status or golden changed; FLAWS.md unchanged.
- No verdict on any paper (SD3). **Memory writes:** none.
ESCALATIONS (superseding the block's):
- **This pull request:** review and merge with a merge commit, so that `b54e2f16…`, the 0.4.0
  goldens' `engine_commit`, stays a real commit on `main`.
- **FINDING-014** (informational, backlog; in NEXT for 0.5.0).
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction, stop for review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root, not in the tree it certifies.

SESSION: 2026-10-28 UTC (2026-10-28 America/New_York) (Session 44 — a fresh conversation launched
from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone; SD15 disclosure
first, by digest (ADR-0005 *Amendments* 1–3); not an examination — Gate S2 has nothing to
examine; examined fetched `origin/main` `fbcc6c4e682e6cbaaf14d8e42a28cfcd5d2f6cb0`, tree
`13d4e586a7974dcebe72353a9bafe8f774044139`, in the original checkout `C:\Users\LEET\Syndeom` on
`main`, synchronized; branch `claude/s2-template-0.5.0`; header completed after ADR-0008 step 5 —
it was written provisionally with the disclosure, which is unchanged; the session-end SHA is the
commit carrying this block, recorded in its pull request)

### Entry 1 — SD15 memory disclosure, by digest (ADR-0005 D1–D4 as amended by Amendments 1–3)

**Appended:** 2026-10-28 UTC, by shell append (`cat <scratchpad file> >> PROGRAM/HANDOFF.md`, CR
stripped), without reading HANDOFF's existing contents. The entry text was composed in the session
scratchpad. Digests were computed at 2026-10-28T23:13:02Z.

**Executor and context:** Claude Code on Claude Fable 5.1 (`claude-fable-5-1`); a fresh conversation
context with no prior turns; working directory `C:\Users\LEET`, which is not a git work tree, so the
harness loaded the `C--Users-LEET` memory index.

**Launching prompt:** the §6 Session-N resume prompt, verbatim. The pasted prompt's bytes equal the
13-line block extracted from the constitution between the code fences under "Session N — resume":
SHA-256 `0015172624685c5025a9d2a76e19e6f4a4b7093e64d085f9d505f3bff6f23dd3` for both, exact byte
diff empty. No owner note was attached; nothing else in the launching message carried program state.

**Actions before this append, in order:** (1) `cat PROGRAM/SYNDEOM-PROGRAM.md`, whose 38 KB output
the harness persisted to a tool-results file; (2) `cat` of that persisted copy, persisted again;
(3) the constitution and ADR-0005 read in full with the Read tool; (4) `date -u` and a scratchpad
script that hashed the memory index and `project_syndeom.md`, ran the name search, listed the file
names in the memory store, and printed `project_syndeom.md` in full; (5) a second script for the
leetspeak sibling name and the prompt comparison. No other repository file was read, no listing of
the checkout's directories, no Git command, no build. No memory was written.

**Recalled memory loaded by the harness:** the index only,
`C:\Users\LEET\.claude\projects\C--Users-LEET\memory\MEMORY.md`. No other memory file was loaded or
recalled. `project_syndeom.md` was read by the executor for this disclosure (D1 (b)).

**Whole index, as a fact (Amendment 2):** SHA-256
`cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`, 15,420 bytes, 110 lines, 0 CR,
334 non-ASCII bytes. Unchanged from the Amendment 2 baseline.

**Name search (D3):** case-insensitive scan of every index line for `syndeom`, `kober`, `friedman`
(awk `tolower`, exit code 0). Matched line numbers: **85, 86, 88, 89, 106, 107** (6 lines). A second
scan for the leetspeak form `fr13dman` matched lines 88, 89 and 107 only, all within the six. No
withheld line names this program or a sibling.

**Matched index lines (D1 (a), by digest):** the six lines, each with its LF terminator, concatenated
in index order: 1,198 bytes, 0 CR, 30 non-ASCII bytes, SHA-256
`0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`. Equal to the Amendment 2
baseline, which equals the digest of HANDOFF lines 6672–6677 reproduced verbatim by Session 35,
Entry 1. Line numbers unchanged from the baseline (no renumbering). Not reproduced here (Amendment 1).

**`project_syndeom.md` (D1 (b), by digest):** SHA-256
`ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, 10,957 bytes, 135 lines, 0 CR,
71 non-ASCII bytes; frontmatter `modified: 2026-10-26T19:18:39.389Z`. Equal to the Amendment 2
baseline, which equals the digest of HANDOFF lines 6683–6817 from the same Session 35 entry. Read in
full this session: it holds the repo location and remote, sibling pointers, and machine and harness
hazards; it names no phase, gate, CLAIM or FLAW status, commit SHA as a state marker, next action,
or escalation (D4). Not reproduced here (Amendment 1).

**Withheld (D2):** 104 index lines, by count, occupying line numbers 1–84, 87, 90–105 and 108–110.
They concern unrelated programs, or are blank lines or section headings; the D3 search matched none
of them.

**Other recalled memory naming this program (D1 (c)):** none.

**Citation (Amendment 1):** Session 35, Entry 1 (HANDOFF lines 6672–6677 and 6683–6817), via the
Amendment 2 baseline computed in Session 37. Verification by re-hashing those HANDOFF lines is owed
after orientation, when HANDOFF may be read, and is recorded in Entry 2 of this block.

**Exam eligibility (SD15, D4):** no reproduced-by-digest memory holds program state; the conversation
context is fresh; the prompt is the §6 text verbatim. Whether this session attempts a gate's
cold-start test is decided at ASSESS and PLAN. No memory writes are planned this session.

### Entry 2 — the disclosure's citation, verified after the append; the header completed

- **Cited entry:** HANDOFF Session 35, Entry 1 (the verbatim reproduction). Checked in the
  working file after ADR-0008 step 5, whose first 741,428 bytes are the HANDOFF blob of `main`
  `fbcc6c4` (unchanged through this branch's first two commits):
  - lines 6672–6677 (1,198 bytes) hash to `0418e621…534c`, the digest of index lines 85, 86, 88,
    89, 106 and 107 with their LF terminators;
  - lines 6683–6817 (10,957 bytes, 135 lines) hash to `ce2ce6de…2cc0`, the digest of
    `project_syndeom.md` on disk.

  Neither digest is new, so nothing is owed verbatim. The matched lines kept their numbers.
- **The append.** Entry 1 with its provisional header was 4,502 bytes (LF, SHA-256
  `3947e995043be011aeca3a9f17e1c0d70b197da4ebf68e9e460ea5503540654e`), appended by `cat >>` at
  2026-10-28T23:15:28Z. HANDOFF grew from 741,428 to 745,930 bytes, exactly that much, and the
  blob is an exact prefix of the working file.
- **The header.** A script replaced the one provisional header line (`## Session N (number
  provisional) — 2026-10-28 UTC (provisional header, completed after orientation)`) with the
  completed `SESSION:` header and asserted that the disclosure body, from `### Entry 1` to the
  end of the append, was unchanged, byte for byte; its digest and length are in the script's
  output under the external root (`handoff-header-completion.txt`).
- **Constitution and ADR bytes (ADR-0008 step 5).** As read in step 0 and as in the snapshot at
  `origin/main` `fbcc6c4`: the constitution `110f5360…479c` (38,958 B, ADR-0009's amended
  digest), ADR-0005 `b08e543a…ce11` (15,291 B, with *Amendments* 1–3) and ADR-0008
  `ccdfa9d7…e9ff` (18,589 B).
- **Between the disclosure and step 5 — one deviation, recorded.** The session read ADR-0008
  (allowed) and ran the preflight. But before the preflight, to verify the append, it printed
  HANDOFF's last 72 numbered lines, which included the six closing lines of Session 43's
  addendum (9285–9290: the superseding escalations and the stop reason) — program state that
  ADR-0008 step 0 says is not read before step 5 completes. The disclosure itself preceded every
  read of the checkout, so SD15's order is intact; the deviation is procedural, nothing was acted
  on from those six lines before step 5, and the fix for a successor is under CONFUSIONS: verify
  an append by hashing the tail's own bytes, never by printing lines before it.

### Entry 3 — the launch

A fresh conversation from `C:\Users\LEET`, launched with the §6 Session-N resume prompt verbatim
and alone (no owner note). The pasted prompt equals the constitution's §6 block byte for byte
(Entry 1). The harness attached no Git snapshot (`C:\Users\LEET` is not a work tree) and loaded
the `C--Users-LEET` index. Executor model: Claude Fable 5.1 (`claude-fable-5-1`).

**Not an examination.** Gate S2 has nothing to examine. The session certifies nothing.

### Entry 4 — `s2_template` 0.5.0, built (S2 ordering step 4, continued)

All on branch `claude/s2-template-0.5.0`, from `main` `fbcc6c4`:
- **Commit 1** `7be41123c698a07b9e56cab2aa2a1e9d257944b8`: the 0.5.0 producer — FLAW-0004 (D5:
  the material's `hidden` names the bare claim, the justification and the omitted tactic lines;
  the hedge is drawn from the fixed five-entry lexicon on the stream `injector/FLAW-0004/1`
  after the material and prefixed to the claim; the proof block of the aligned `have` is
  omitted, never `sorry`, so the kernel reports an unsolved goal at the site, check K; `N/A` for
  an informal-only target; the D5 informal check), FLAW-0011 as a commanded composite (D5:
  `embedded`, one FlawSpec, on a true statement; the composite's record at step 0 and at the
  main declaration's header line, severity and `lean_catchable` inherited, confirmed by the
  truth witness and every embedded check; counts per instance), the `sqrt2_irrational` seed's
  FLAW-0004 `routine-true` material at step 3 (FLAWS.md's FLAW-0011 example), the FINDING-014
  rule in `check_seed`, the formal rendering's `main_line`, the tests (composite cases in
  `every_case`, the record count per instance, two new tests, the seed-rule tests) and the crate
  README. `Cargo.lock` unchanged.
- **Commit 2** `4edc3f73f43e32d615cd7ba01093215f133c3e08`: the 4 goldens of 0.5.0 — FLAW-0004
  (informal, pair) and the composite (informal, pair) — produced by `generate --dry-run` with
  seed 7 at commit 1 by two separate runs (2026-10-28T23:41:58Z–23:42:55Z) compared byte for
  byte, the goldens README and the count pinned in `tests/goldens.rs` (47 cases).
- **Commit 3** (this one): the packet `analysis/s2-generate-5/`, ADR-0011 *Amendments*, STATE
  and this close.

The evidence, the security review and readings 39–51 are in the packet's README. Every receipt
is under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-050-20261028T2330Z\` (the two probes,
`cargo audit`, the two productions `goldens-5a`/`-5b`, whose sorted `receipt.json` SHA-256 lists
hash to `40c5d738…8494` and `91087721…cc8a`, the production report `784b1762…dec39`, and the
ORIENT gate log), and the orientation receipt under
`C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T2316Z\`.

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN. S2.1–S2.4 are in progress: this is the fifth
implementation session (ADR-0011 D11). Gate S1 is CLOSED (Session 29).
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipt `live-orient-1.json` under
`C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T2316Z\`):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-28T23:16:09Z–23:16:10Z, exit 0.
- **Snapshot.** `main` at `fbcc6c4e…`, tree `13d4e586…`, **synchronized**: ahead 0, behind 0. The
  only change was the disclosure append (`allowed_present`; `unrelated: []`). No worktree.
- **Step 5.** The constitution (`110f5360…`), ADR-0005 (`b08e543a…`) and ADR-0008 (`ccdfa9d7…`),
  as read in step 0, equal the snapshot's blobs.
- **Step 6.** Read in this order: STATE (the Updated bullets of Sessions 37–43, the memory
  policy, the Gate S2 table, blockers, the standing escalations' headings and the 0.4.0 item,
  the PR 29 rulings, NEXT); HANDOFF Session 43 in full (Entries 1–4, the close and the
  addendum); CLAIMS: 0001–0005 VERIFIED, 0006 CONDITIONAL, 0007 VERIFIED; FLAWS `0.1.0` (14
  entries; the vocabulary, FLAW-0004, FLAW-0011, FLAW-0013 and the summary table in full; later
  FLAW-0003, 0007, 0008, 0010 and 0012 for NEXT); ADR-0008 and ADR-0011 (D2–D12 and the
  amendments) in full; the crate's sources, tests, seeds, goldens layout and READMEs, the sqrt2
  witness module, `Audit.lean`, the manifest crate's record and check rules, implementation
  packet 4 and its production scripts; PR 29 on GitHub (merged by Al 23:06:26Z with the merge
  commit `fbcc6c4`; no review, no later comment; no open pull request or issue). They agree with
  one another and with the tree.
- **ORIENT gate GREEN**, 23:16:57Z–23:17:47Z (log `gate-orient.log`, SHA-256
  `b6fe53cc87df4f4e88348ce4803338cc6455b824547654dd1702159f5cebaf65`, 17,508 bytes, stdout and
  stderr merged): 8,935 Lean jobs; eleven axiom lines within the standard three; fmt, clippy and
  every test pass; every tracked text file LF; the registers and the 65-row freeze register pass.
- **NEXT survived ORIENT.** PR 29 was merged by Al with the merge commit `fbcc6c4` at
  2026-10-28T23:06:26Z; `b54e2f16` and `cc868a3c` are ancestors of `main`. No later ruling
  exists, so nothing was recorded before the build.
ADVANCED:
- **`s2_template` 0.5.0** (S2.1–S2.4 in progress): FLAW-0004 realized for the first time — the
  hedge lexicon, the justification deleted, the proof block omitted — and FLAW-0011 as a
  commanded composite, both as FLAWS.md's FLAW-0011 example on `sqrt2_irrational`; four goldens
  from two byte-identical runs; the D5 informal check for FLAW-0004 and the composite's check;
  31 tests; clippy and audit clean.
- **FINDING-014** (packet 4) closed by `check_seed`.
BELIEF DELTAS:
- **A `have … := by` whose tactic block is empty does not absorb the next line.** Session-start
  me expected Lean 4's `by` block to swallow a following line at the same column and cascade;
  the probe shows one `unsolved goals` error at the `by` (7:21), `hp` bound, and every later
  line elaborating. Evidence: `s2i_hedged_p1.out`.
- **Lean reports an unsolved tactic goal at the `by` token's line**, not at the last tactic or
  the next line: both probes locate the error at 7:21. Evidence: the two probe outputs.
- **At seed 7 the `injector/FLAW-0004/1` stream draws hedge 1** after the material draw; forty
  seeds draw more than one entry. Evidence: the goldens and the new test.
- **The manifest crate admits step 0 as an informal location and any declared declaration's
  line as a formal one**, so D5's composite placement needs no schema change. Evidence:
  `engine/manifest/src/lib.rs`, the D3 checks, and the goldens' self-checks.
CONFUSIONS:
- **The constitution's `cat` exceeded the harness's display limit twice** (the persisted copy is
  persisted again when `cat`); the Read tool was needed. Cost: two calls. Read the constitution
  with the Read tool at once.
- **The append was verified by printing lines before it** (Entry 2's deviation). Cost: six lines
  of program state read before step 5. Fix: verify an append by its own length and digest —
  `tail -c <appended bytes> | sha256sum` — never by printing context lines.
- **A closure returning `String` in `assert!(mutate(&|m| … .remove(0)))`** failed to type-check
  (`()` expected); braces and a semicolon fix it. Cost: one rebuild.
- **Clippy's `unnecessary_get_then_check`** refuses `map.get(k).is_none()`; write
  `!map.contains_key(k)`. Cost: one rebuild.
HONESTY CHECK:
- No register status changed. The packet's measurements are implementation evidence, not CLAIMS.
- The FLAWS.md FLAW-0011 example, as built, is a *valid* proof under D7 (reading 43); the
  packet, the goldens README, the crate README and STATE say so plainly, and the alternative
  readings are listed for Al rather than resolved by the executor.
- "The statement's truth witness builds" is not re-run by the producer (reading 45,
  FINDING-015); the packet says what is and is not checked.
- The probe `p2` was compiled and recorded although not built into the producer, so the choice
  between the two realizations is visible.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened; no `groundtruth/` item was read for
design; the material comes from FLAWS.md 0.1.0 only (D6, ruling 1); the Lean names used are the
seed's own at the pinned toolchain; no real AI-generated proof was used. Verdict: no leakage.
DETERMINISM CHECK:
- Every golden regenerates byte for byte from its config, seed and recorded observation under
  the version its manifest names; two productions at one commit agree file by file; the hedge
  is a draw of `syndeom-rng/1`'s `injector/FLAW-0004/1` stream, recorded as `hedge`.
- No wall-clock value enters an artifact; receipts carry durations only.
- Nothing entered CLAIMS, so there is nothing to quarantine.
MANIFEST CHECK:
- Every golden manifest passed `Manifest::check` with its bytes before it was written: `origin`
  synthetic, `derivation` by-construction; the FLAW-0004 and composite artifacts are
  `proof_status: valid` and `correct: true` with their records cosmetic (`YES-KERNEL` in a pair,
  `N/A` informal-only).
- Nothing synthetic is labeled real.
- "Proven" was applied to nothing. The witness is a `lean-kernel` reference audited by the gate,
  not a CLAIM.
DRIFT CHECK:
- No verdict on any paper (SD3), and no understanding claims (SD17): the hedge is a labeled gap,
  measured by whether a detector sees it, never by readability.
- No detector was chosen or tuned, and there was no referee contact (SD13).
- No S3 work, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. After this pull request merges, with any ruling recorded first,
`s2_template` 0.6.0: FLAW-0010 `a` (the fictional-reference lexicon, `shape`, `claim`) on
`sqrt2_irrational`'s step 3 and FLAW-0003 (`direct`, `cycle`; the plan's premises) on the new
seed `bounded_monotone_converges`.
ESCALATIONS:
- **This pull request:** review and merge with a merge commit, so that `7be41123…`, the 0.5.0
  goldens' `engine_commit`, stays a real commit on `main`.
- **Readings 39–51** in `analysis/s2-generate-5/README.md` (ADR-0011 *Amendments*), in
  particular 43 (a composite of a cosmetic instance is a valid, correct proof under D7, against
  FLAW-0011's recipe), 39 (the hedge drawn) and 46 (one embedded instance; FLAW-0013 inside a
  composite is Al's), and **FINDING-015** (informational, backlog).
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. The 0.5.0 scope STATE's NEXT named is built, and the pull request
is open for Al's review. Repo green: the gate on the committed tree is recorded in the pull
request and under the external root, not in the tree it certifies.
**Memory writes:** none. A write would change a memory digest and make the next disclosure owe
verbatim reproduction.

SESSION 44 ADDENDUM — AL'S RULINGS ON PR 30 (received 2026-10-29 UTC, 2026-10-28
America/New_York, in this session's conversation after PR #30 opened; recorded at Al's direction
in new commits on the same branch, no force-push; transcribed verbatim from Al's message as
rendered to the executor; copy `owner-rulings-pr30.txt` under
`C:\Users\LEET\Codex\Temp\syndeom-s2-template-050-20261028T2330Z\`, 1,248 bytes, SHA-256
`8907df8358260ff8da2ce8d2b7db9b30b63516816a122b0793063983ed3be618`):

~~~text
Rulings on PR #30 / s2_template 0.5.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as new commits on the branch; no force-push):

1. Readings 39, 40, 41, 42, 44, 45, 47, 48, 49, 50 and 51 are accepted as read.

2. Reading 43: D7 stands. FLAW-0011 requires at least one embedded instance of severity gap or fatal; a composite whose embedded instances are all cosmetic is not an invalid proof and is refused, with a test. Under D7, FLAWS.md's FLAW-0011 example embeds a cosmetic instance and therefore realizes FLAW-0004 routine-true, which the 0.5.0 goldens already hold; record that as an erratum note against FLAWS.md 0.1.0 and leave FLAWS.md's text as it is. Replace the two FLAW-0011 goldens with a composite embedding one gap-or-fatal instance from a seed's existing material (your choice), informal and pair, from two byte-identical runs, and regenerate all 0.5.0 goldens at the fix commit so they name one engine_commit; the FLAW-0004 proof bytes must not change.

3. Reading 46: a composite embeds instances of FLAW-0001 through FLAW-0010 only, by FLAW-0011's definition; FLAW-0012 and FLAW-0013 are never embedded.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

Before applying the rulings, a fresh fetch at 2026-10-29T00:05:32Z (receipt `recheck-3.json`
under `C:\Users\LEET\Codex\Temp\syndeom-orient-20261028T2316Z\`) showed `origin/main` still at
`fbcc6c4` and the branch at `074e577`, in sync with its remote; PR 30 had no review and no
comment on GitHub.

How each ruling is applied:
- **Ruling 1** → readings 39, 40, 41, 42, 44, 45, 47, 48, 49, 50 and 51 recorded as accepted in
  ADR-0011 *Amendments*, STATE and the packet.
- **Ruling 2 (reading 43 replaced)** → commit 4 `1c0c5fcdf1ffd375b109f272461a63c49330e007`:
  `compose` refuses an embedded instance whose severity is not gap or fatal, with the test
  `d5_flaw_0011_is_a_commanded_composite_of_one_embedded_instance` (the cosmetic FLAW-0004
  composite, a cosmetic FLAW-0007 `unused` composite and a repairable FLAW-0001 `local`
  composite all refused; the FLAW-0004 instance alone still accepted); the two cosmetic
  composite goldens removed; `every_case` embeds only gap-or-fatal materials of FLAW-0001
  through FLAW-0010; the crate README. The erratum note against FLAWS.md 0.1.0 is recorded in
  ADR-0011 *Amendments*, STATE and the packet; FLAWS.md is unchanged. Commit 5 `5afc00d7809fa34a0c163e08c6d8e353d374bcae`: the
  0.5.0 goldens regenerated at `1c0c5fcd…` by two runs (2026-10-29T00:11:35Z–00:12:38Z,
  `evidence/goldens-run5r.py`) compared byte for byte (`evidence/goldens-copy5r.py`) —
  FLAW-0004 (informal, pair) with proof bytes, ids and ground truth unchanged, checked against
  the goldens committed at `4edc3f7` exported under the external root, and the new composite of
  `continuity_is_not_uniform`'s FLAW-0002 `forall-exists` gap (informal, pair; the executor's
  choice: a true statement with a kernel-caught gap whose 0.2.0 files the composite leaves
  byte-identical, the pair failing at line 6) — with the goldens README and the count (47 cases
  again). `evidence/goldens-regeneration-report.json` has SHA-256
  `5e0d258b5cb3c6cb1cab78e7646ac28e2466e36b3336575afa16f8089a3239f6` (7,397 bytes); the sorted
  `receipt.json` SHA-256 lists of the two runs hash to
  `bbe9fdb30289e0dfffedcd899a76701e94bb5190d298ec71e2c28d67c4d66aa8` and
  `5244afe6f4fe2abc1d7609b2f5058565bd2880a21001118e44cf066d4df8e652`; the receipts are under
  `goldens-5ra\` and `goldens-5rb\` of the external root.
- **Ruling 3 (reading 46 replaced)** → commit 4: `compose` accepts FLAW-0001 through FLAW-0010
  only (FLAW-0011, 0012, 0013 and 0014 refused, with a test).
- **Commit 6** (this one): the records — ADR-0011 *Amendments*, STATE (*Standing rulings*,
  *Standing escalations*, the Updated bullet, the Gate S2 rows, NEXT), the packet README and
  this addendum. The gate on the committed tree, the step-7 recheck and the push are recorded
  in the pull request and under the external root.

CHECKS for this addendum:
- No leakage: `contact/` was not opened, and no source was added.
- No seed, no FLAW-0004 proof byte, no label and no register status changed; FLAWS.md unchanged;
  the composite's proof bytes are the 0.2.0 FLAW-0002 goldens'.
- Determinism: the four 0.5.0 goldens regenerate from (config, seed, observation) at one engine
  commit; two runs agreed file by file.
- No verdict on any paper (SD3). **Memory writes:** none.
ESCALATIONS (superseding the block's):
- **This pull request:** review and merge with a merge commit, so that `1c0c5fcd…`, the 0.5.0
  goldens' `engine_commit`, stays a real commit on `main`.
- **FINDING-015** (informational, backlog).
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction, stop for review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root, not in the tree it certifies.

---

SESSION: 2026-10-29 UTC (2026-10-28 America/New_York) (Session 45 — a fresh conversation launched
from `C:\Users\LEET` with the §6 Session-N resume prompt verbatim and alone; SD15 disclosure
first, by digest (ADR-0005 *Amendments* 1–3); not an examination — Gate S2 has nothing to
examine; examined fetched `origin/main` `854a4b4520583c1343d64fd55fe392f0a1fe8a2f`, tree
`f2357f53ce78d1acc1d3aef8c872f264914e54bf`, in the original checkout `C:\Users\LEET\Syndeom` on
`main`, synchronized; branch `claude/s2-template-0.6.0`; header completed after ADR-0008 step 5 —
it was written provisionally with the disclosure, which is unchanged; the session-end SHA is the
commit carrying this block, recorded in its pull request)

Appended at 2026-10-29T00:30:50Z (UTC) by shell redirection; the block header above is provisional and is not part of the disclosure.

### Entry 1 — SD15 memory disclosure, by digest (ADR-0005 D1–D4 as amended by Amendments 1–3)

Appended by shell redirection (`>> PROGRAM/HANDOFF.md`) before any other repository read, directory
listing, Git command, or build. Read before this append, in order: `PROGRAM/SYNDEOM-PROGRAM.md` in
full; `PROGRAM/DECISIONS/ADR-0005.md` in full; then the executor memory index and the SYNDEOM memory
file, both read in full and hashed. HANDOFF's existing contents were not read before this append; only
its byte size was taken before and after, to confirm the append. The launching message was the §6
Session-N resume prompt verbatim and nothing else. Local calendar date at launch: 2026-10-28
(America/New_York); the append time stamped above is UTC, so the provisional header carries 2026-10-29.

**Harness-loaded memory.** The harness loaded one memory into context: the index
`C:/Users/LEET/.claude/projects/C--Users-LEET/memory/MEMORY.md` (the auto-memory index). No other
memory file was loaded by the harness; D1(c) is empty. No memory was written in this session before
this append.

**Whole index, as a fact (ADR-0005 Amendment 2).** SHA-256
`cd9438425b3bcc6b81967543e1b9c5387ca1ebdd14a6ba0082a7af398b8ffc69`, 15,420 bytes, 110 lines, 0 CR,
334 non-ASCII bytes, final byte LF. Equal to the whole-index digest recorded in ADR-0005 Amendment 2
(unchanged since Session 37's Entry 1).

**Name search (D3).** Case-insensitive search of the whole index for `syndeom`, `kober`, and
`friedman`, line by line: matched line numbers 85, 86, 88, 89, 106, 107 (6 lines). Supplementary
case-insensitive search for the leetspeak sibling name `fr13dman`: lines 88, 89, 107, all within the
matched set, so it adds no line. Every matched line is in the set hashed below; no withheld line names
this program or a sibling.

**Matched index lines.** Byte form per ADR-0005 Amendment 3: each matched line with its LF terminator,
concatenated in index order. 6 lines, 1,198 bytes, 0 CR, 30 non-ASCII bytes, SHA-256
`0418e62182f345e565cee9465dd61ce41685c4b15799e52cfcf966ca7b87534c`. Equal to the baseline in
ADR-0005 Amendment 2: the digest of HANDOFF lines 6672–6677, which Session 35's Entry 1 reproduced.
The line numbers are the same as in that baseline; there is no renumbering.

**Withheld (D2).** 104 index lines concern unrelated programs and are withheld by count. They occupy
lines 1–84, 87, 90–105, and 108–110.

**SYNDEOM memory file.** `C:/Users/LEET/.claude/projects/C--Users-LEET/memory/project_syndeom.md`:
SHA-256 `ce2ce6deeb3db123c7110a4ac0fc8ac5ccc33c8b528a1b4511acc950ff7a2cc0`, 10,957 bytes, 135 lines,
0 CR, 71 non-ASCII bytes, final byte LF. Equal to the baseline in ADR-0005 Amendment 2: the digest of
HANDOFF lines 6683–6817, which Session 35's Entry 1 reproduced.

**Disclosure.** Both compared digests (the matched index lines and the SYNDEOM memory file) equal the
digests of bytes that an earlier HANDOFF disclosure reproduced, so this disclosure is by digest
(ADR-0005 Amendment 1, comparison refined by Amendment 2, readings confirmed as Al's by Amendment 3).
The content is not reproduced. Citation: HANDOFF Session 35, Entry 1 — taken here from ADR-0005
Amendment 2's baseline record, because HANDOFF was not read before this append. A verification that
hashes those HANDOFF lines follows after orientation, as a later entry of this block, so that this
disclosure stays first.

**Pass criterion (D4).** As read in full this session, the SYNDEOM memory file holds the repo
location, the remote, the sibling pointers, the standing rule to work only from the repo, the location
of a second checkout and of the external evidence roots, and machine and harness hazards. It names no
phase, gate status, CLAIM or FLAW status, commit SHA as a state marker, next action, or escalation.
The six matched index lines hold the same kind of content (repo pointers and the standing rule). No
memory disclosed by digest holds program state. Withheld lines are outside the program by D3.

**Cold status.** This session was launched with the §6 Session-N resume prompt verbatim; it does not
run in the conversation context that built any state under test. Whether this session attempts a
gate's cold-start test is decided after orientation; the disclosure is made regardless, as SD15
(amended by ADR-0009) requires. If this session is an exam, no memory is written in it.

### Entry 2 — the disclosure's citation, verified after the append; the header completed

- **Cited entry:** HANDOFF Session 35, Entry 1 (the verbatim reproduction). Checked in the
  working file after ADR-0008 step 5, whose first 765,381 bytes are the HANDOFF blob of `main`
  `854a4b4` (unchanged through this branch's first two commits):
  - lines 6672–6677 (1,198 bytes) hash to `0418e621…534c`, the digest of index lines 85, 86, 88,
    89, 106 and 107 with their LF terminators;
  - lines 6683–6817 (10,957 bytes, 135 lines) hash to `ce2ce6de…2cc0`, the digest of
    `project_syndeom.md` on disk.

  Neither digest is new, so nothing is owed verbatim. The matched lines kept their numbers.
- **The append.** Entry 1 with its provisional header and append-time line was 4,608 bytes (LF,
  SHA-256 `aaaa6136ec47e4a4b4b1f59fce744ef85dc9f41cd44a195bcb29d7dd2fd97a6d`), appended by `>>`
  at 2026-10-29T00:30:50Z; its body (4,389 bytes, SHA-256
  `7646fb34b5695106749735571da75123007eeb474f6d61814d900bbb6a0b8f8e`) was composed in the
  session scratchpad and is retained as `disclosure-body.md` under the external root. HANDOFF
  grew from 765,381 to 769,989 bytes, exactly that much. The append was verified by HANDOFF's
  byte size before and after, and by hashing the tail's own bytes — no line before it was
  printed.
- **The header.** A script replaced the one provisional header line (`## Session N — 2026-10-29
  UTC (provisional header, completed after orientation)`) with the completed `SESSION:` header
  and asserted that the disclosure — the append-time line and the body, to the end of the
  append — was unchanged, byte for byte, and that the committed prefix was unchanged; its output
  is `handoff-header-completion.txt` under the external root.
- **Constitution and ADR bytes (ADR-0008 step 5).** As read in step 0 and as in the snapshot at
  `origin/main` `854a4b4`: the constitution `110f5360…479c` (38,958 B, ADR-0009's amended
  digest), ADR-0005 `b08e543a…ce11` (15,291 B, with *Amendments* 1–3) and ADR-0008
  `ccdfa9d7…e9ff` (18,589 B).
- **Between the disclosure and step 5 — nothing else of the checkout was read.** After the
  disclosure the session read ADR-0008 and ran the preflight with `--allow-dirty
  PROGRAM/HANDOFF.md`; the receipt lists that path under `allowed_present` and nothing under
  `unrelated`. (A harness note, as Session 44 recorded: the constitution's `cat` output exceeded
  the display limit and was persisted to a tool-results file, which the session then read with
  the Read tool; the bytes read were the checkout's.)

### Entry 3 — the launch

A fresh conversation from `C:\Users\LEET`, launched with the §6 Session-N resume prompt verbatim
and alone (no owner note). The harness attached no Git snapshot (`C:\Users\LEET` is not a work
tree) and loaded the `C--Users-LEET` index. Executor model: Claude Fable 5.1
(`claude-fable-5-1`). The local calendar date at launch was 2026-10-28 (America/New_York); the
UTC date, which the header carries, 2026-10-29.

**Not an examination.** Gate S2 has nothing to examine. The session certifies nothing.

### Entry 4 — `s2_template` 0.6.0, built (S2 ordering step 4, continued)

All on branch `claude/s2-template-0.6.0`, from `main` `854a4b4`:
- **Commit 1** `ef41a10ce16db34c1ebe3ea66a456df97df83be1`: the 0.6.0 producer — the plan's
  `premises` (D2: `{cites, phrase}` per step, rendering to nothing; `premise_edges` and
  `premise_cycle`); FLAW-0010 `a` (D5: the material's `cited` — the bare claim and the
  justification the reference stands in for — with the one material dial `claim` and the
  config's `shape`; the reference `[<author> <year>, <venue>, <result>]` drawn from the fixed
  lexicon of obviously fictional tokens on the stream `injector/FLAW-0010/1` after the material,
  in the order author, year, venue, result, and prefixed as `By <reference>` to the bare claim;
  the record spanning the reference, `replaced` the justification, the indices, `reference`,
  `asserted`, `justification` and the originals recorded; the formal file unchanged, `N/A`,
  informal-only; `formal`, `b`, `c` and `claim: false` refused; the check rebuilding the
  reference from the recorded indices, requiring every token in the lexicon, the step
  `By <reference>` and the claim, and the faithful step holding the justification once);
  FLAW-0003 (D5: insertion material at a `theorem` lemma, fatal — `direct`, `form`
  `restated-goal` | `contrapositive`, Lemma 1 proved by `exact
  syndeom_s2_template_bounded_monotone_converges`, a forward reference the kernel rejects at the
  lemma's declaration, check K; `cycle`, `cycle_length` 2, Lemma 1 citing the theorem and the
  key step citing Lemma 1, informal-only, `NO`, the formal target refused, the record carrying
  the cycle `[lemma_1, main]` and the citation phrases' spans, checked on the plan's premise
  graph and the rendered steps; the formal file a faithful proof with the lemma proved from
  Mathlib); `check_seed`'s rules for both materials (`check_cited_material`,
  `check_circularity_material`, premises on cycle material only); the seed
  `bounded_monotone_converges` (FLAWS.md's FLAW-0003 example, two aligned steps on the
  supremum, three materials) with its witness module `lean/Syndeom/Seeds/BoundedMonotoneConverges.lean`
  (imported by `Syndeom.lean`, audited by `Audit.lean`, now twelve lines); the `sqrt2_irrational`
  FLAW-0010 material at step 3 (the FLAW-0004 material's bare claim and justification reused,
  every fragment faithful); `Version::V0_6_0`; the tests (two new, `d6`, `d1`, `every_case`,
  `d10_locations`); the crate README. `Cargo.lock` unchanged.
- **Commit 2** `45bdbe3c03d046ce89ac92f723b86d199462d7ba`: the 12 goldens of 0.6.0 —
  FLAW-0010 `journal` (informal, pair) and, on the new seed, the control (informal, formal),
  `direct` `restated-goal` and `contrapositive` (informal, formal, pair each) and `cycle`
  (informal, pair) — produced by `generate --dry-run` with seed 7 at commit 1 by two separate
  runs (2026-10-29T01:02:06Z–01:06:34Z) compared byte for byte, every formal file equal to its
  hand-written probe (or, for the FLAW-0010 pair, to the faithful control), the goldens README
  and the count pinned in `tests/goldens.rs` (59 cases).
- **Commit 3** (this one): the packet `analysis/s2-generate-6/`, ADR-0011 *Amendments*, STATE
  and this close.

The evidence, the security review and readings 52–65 are in the packet's README. Every receipt
is under `C:\Users\LEET\Codex\Temp\syndeom-s2-template-060-20261029T0029Z\` (the five probes,
`cargo audit`, the two productions `goldens-6a`/`-6b`, whose sorted `receipt.json` SHA-256 lists
hash to `82f2cbda…c8c8` and `6183f15a…43bd`, the production report `5dd0943f…771f`, the
orientation receipt, the ORIENT gate log, the disclosure body and the header-completion record).

PHASE / GATE: S2 — GENERATOR / Gate S2 OPEN. S2.1–S2.4 are in progress: this is the sixth
implementation session (ADR-0011 D11). Gate S1 is CLOSED (Session 29).
ORIENT (ADR-0008 D1, steps 1–5 by the preflight; receipt `orient-receipt-1.json` under
`C:\Users\LEET\Codex\Temp\syndeom-s2-template-060-20261029T0029Z\`):
- **Identity.** `C:/Users/LEET/Syndeom`; remote `https://github.com/Quigles1337/Syndeom.git`;
  default branch `main`, from `git ls-remote --symref`.
- **Fetch.** 2026-10-29T00:31:28Z, exit 0.
- **Snapshot.** `main` at `854a4b45…`, tree `f2357f53…`, **synchronized**: ahead 0, behind 0. The
  only change was the disclosure append (`allowed_present`; `unrelated: []`). No worktree.
- **Step 5.** The constitution (`110f5360…`), ADR-0005 (`b08e543a…`) and ADR-0008 (`ccdfa9d7…`),
  as read in step 0, equal the snapshot's blobs.
- **Step 6.** Read in this order: STATE (the Updated bullets of Sessions 37–44, the memory
  policy, the standing rulings, the Gate S2 table, blockers, the standing escalations, NEXT);
  HANDOFF Session 44 in full (Entries 1–4, the close and the addendum); CLAIMS: 0001–0005
  VERIFIED, 0006 CONDITIONAL, 0007 VERIFIED; FLAWS `0.1.0` (14 entries, in full); ADR-0008 and
  ADR-0011 (D1–D12 and every amendment) in full; the crate's sources, tests, seeds, goldens
  layout and READMEs, the seed witness modules, `Audit.lean`, the manifest crate's record and
  check rules, implementation packet 5 and its production scripts; PR 30 on GitHub (merged by
  Al 2026-10-29T00:25:46Z with the merge commit `854a4b4`, parents `fbcc6c4` and `dbce6d9`; no
  review, no comment but the executor's own; no open pull request or issue). They agree with one
  another and with the tree.
- **ORIENT gate GREEN**, 2026-10-29T00:32:24Z–00:33:14Z (log `gate-orient.log`, SHA-256
  `c849e4081f9b6d33937dada5683688a180591415e0495ad25f02b11078cd6487`, 17,548 bytes, stdout and
  stderr merged): 8,935 Lean jobs; eleven axiom lines within the standard three; fmt, clippy and
  every test pass; every tracked text file LF; the registers and the 65-row freeze register pass.
- **NEXT survived ORIENT.** PR 30 was merged by Al with a merge commit; `1c0c5fcd` and
  `5afc00d7` are ancestors of `main`. No later ruling exists, so nothing was recorded before the
  build.
ADVANCED:
- **`s2_template` 0.6.0** (S2.1–S2.4 in progress): FLAW-0010 `a` realized for the first time —
  the fictional-reference lexicon, the drawn reference, the justification replaced — and
  FLAW-0003 in both forms, `direct` (the forward reference the kernel rejects) and `cycle` (the
  premise graph), on the new seed `bounded_monotone_converges`; the plan's `premises` (D2); 12
  goldens from two byte-identical runs; 33 tests; clippy and audit clean. Ten of the twelve
  FLAWS.md example seeds now exist; FLAW-0008 and FLAW-0012 remain.
BELIEF DELTAS:
- **A Lean `theorem` whose tactic proof fails still enters the environment, so a later `exact`
  of it elaborates without a second error.** Session-start me expected the forward reference to
  cascade into the main declaration; both `direct` probes report exactly one error, at the
  lemma's proof line, and the main declaration elaborates. Evidence: `s2i_direct_restated.out`,
  `s2i_direct_contrapositive.out`.
- **Lean 4.34.1 prints `error(lean.unknownIdentifier):`**, and the harness's `first_error_line`
  (keyed on `: error`) still parses it: the goldens' observations record line 5. Evidence: the
  probe outputs and `case.json` of the `direct` cases.
- **The main declaration's name is a prefix of its lemma's name**, so a substring test for "the
  fragment references the main declaration" is wrong; a whole-identifier match
  (`mentions_identifier`) is needed. Evidence: `d6` failed on the cycle material before the fix.
- **`tendsto_atTop_ciSup` at the pin needs `Mathlib.Topology.Order.MonotoneConvergence` and, for
  `OrderTopology ℝ`, `Mathlib.Topology.MetricSpace.Pseudo.Lemmas`**; the pair compiles in about
  9 s per probe. Evidence: the five probes.
CONFUSIONS:
- **Appending ~450 lines of Rust tests through a Bash heredoc failed to parse** (the harness's
  long-heredoc hazard, already in memory). Cost: one call. Fix: write the text with the Write
  tool and `cat` it onto the file.
- **Pinning the 0.6.0 golden count before the goldens existed** turned commit 1's tree red (the
  count assertion); packet 5 had put the count in the goldens commit. Cost: one test run. Fix:
  the `GOLDENS_<version>` constant belongs with the goldens.
- **A local variable named like a new helper** (`flawed_step`) shadowed it; one build. Fix: name
  helpers distinctly from the locals around their call sites (`step_of_flawed`).
- **`original_fragments` was expected among the informal dials**; it lives in the formal dials.
  Cost: one test run.
- **The constitution's `cat` exceeded the harness's display limit** (as Session 44 recorded) and
  was read from the persisted copy with the Read tool. Cost: one call.
HONESTY CHECK:
- No register status changed. The packet's measurements are implementation evidence, not CLAIMS.
- The `cycle`'s formal file is *not* the control's bytes: Lemma 1 is declared and proved from
  Mathlib, because D2 renders a `Lemma N.` step as a declaration; the packet, both READMEs and
  the ADR amendment say so (reading 61) instead of calling it "the faithful proof" literally.
- The `cycle` check is structural; no Lean observation can confirm or fail it; said plainly
  (reading 63).
- "Obviously fictional" is the executor's judgment of the lexicon's tokens (reading 52), not a
  verified property; the tokens are listed in full so Al can judge.
- FINDING-016 is recorded; FINDING-015 stands and now covers the new seed.
LEAKAGE CHECK: checked, not assumed. `contact/` was not opened; no `groundtruth/` item was read for
design; the seed and the materials come from FLAWS.md 0.1.0 only (D6, ruling 1); the Lean names
used are Mathlib's at the pinned commit (`tendsto_atTop_ciSup`, `le_ciSup`); no real AI-generated
proof was used. Verdict: no leakage.
DETERMINISM CHECK:
- Every golden regenerates byte for byte from its config, seed and recorded observation under
  the version its manifest names; two productions at one commit agree file by file; the
  reference is a draw of `syndeom-rng/1`'s `injector/FLAW-0010/1` stream, recorded as the four
  lexicon indices and rebuilt by the check.
- No wall-clock value enters an artifact; receipts carry durations only.
- Nothing entered CLAIMS, so there is nothing to quarantine.
MANIFEST CHECK:
- Every golden manifest passed `Manifest::check` with its bytes before it was written: `origin`
  synthetic, `derivation` by-construction; every flawed artifact `proof_status: invalid`,
  `correct: false`, of a true statement; the FLAW-0010 records `N/A`, the `cycle` records `NO`,
  the `direct` records `YES-KERNEL` in both files of a pair.
- Nothing synthetic is labeled real.
- "Proven" was applied to nothing. The new witness is a `lean-kernel` reference audited by the
  gate, not a CLAIM.
DRIFT CHECK:
- No verdict on any paper (SD3), and no understanding claims (SD17): the fictional reference and
  the circular lemma are labeled gaps, measured by whether a detector sees them, never by
  readability.
- No detector was chosen or tuned, and there was no referee contact (SD13).
- No S3 work, and the SD8 quarantine is untouched.
NEXT-SINGLE-ACTION: STATE's NEXT. After this pull request merges, with any ruling recorded first,
`s2_template` 0.7.0: FLAW-0008 on the new seed `bolzano_weierstrass` (the five transformations;
pairs only; the check that both statements build and differ by exactly the recorded
transformation) and FLAW-0012 on the new seed `bounded_continuous_sup` (the nearby false
statement, the transplanted proof failing at the hidden use site, the negation witness).
ESCALATIONS:
- **This pull request:** review and merge with a merge commit, so that `ef41a10c…`, the 0.6.0
  goldens' `engine_commit`, stays a real commit on `main`.
- **Readings 52–65** in `analysis/s2-generate-6/README.md` (ADR-0011 *Amendments*), in
  particular 61 (the `cycle`'s formal file declares and proves the lemma rather than keeping the
  control's bytes), 57 (FLAW-0010 `N/A` for a pair, its `lean_build` `pass`), 55 (the record
  spans the reference) and 52 (the reference's form and the lexicon); **FINDING-016**
  (informational, backlog) and FINDING-015 (standing).
- Unchanged: human-referee participants (SD13).
STOP-REASON: natural boundary. The 0.6.0 scope STATE's NEXT named is built, and the pull request
is open for Al's review. Repo green: the gate on the committed tree is recorded in the pull
request and under the external root, not in the tree it certifies.
**Memory writes:** none. A write would change a memory digest and make the next disclosure owe
verbatim reproduction.

SESSION 45 ADDENDUM — AL'S RULINGS ON PR 31 (received 2026-10-29 UTC, 2026-10-28
America/New_York, in this session's conversation after PR #31 opened; recorded at Al's direction
in a new commit on the same branch, no force-push; transcribed verbatim from Al's message as
rendered to the executor; copy `owner-rulings-pr31.txt` under
`C:\Users\LEET\Codex\Temp\syndeom-s2-template-060-20261029T0029Z\\`, 793 bytes, SHA-256
`fe937f3eae7cb0d4f250df5e001e76bd922ae4c3fe40b15e8f35523f52f8748a`):

~~~text
Rulings on PR #31 / s2_template 0.6.0 (record verbatim in ADR-0011 Amendments, STATE and this session's HANDOFF block, as a new commit on the branch; no force-push):

Readings 52 through 65 are accepted as read.

On reading 52, record also: FLAW-0010 a's references are placeholder-shaped by D5's design, so S3 reads detection rates for that class as rates against obviously fictional references; a plausible-shaped lexicon, if one is ever wanted, is a later producer version and never attaches a real name to a fictional work.

On reading 61, record also: the cycle pair's formal file is a valid proof by construction; the instance is informal-only, and the pair is invalid under D7's either-file rule.

Run the gate on the committed tree, fetch fresh before pushing, and stop for my review.
~~~

Before recording the rulings, the checkout — left clean on `main` at the session's close — was
switched back to `claude/s2-template-0.6.0` at `7f0c85ff…`, in sync with its remote; PR 31 had
no review and no comment on GitHub.

How each ruling is applied:
- **Readings 52–65 accepted as read** → recorded in ADR-0011 *Amendments*, STATE (*Standing
  rulings*, *Standing escalations*, the Updated bullet, NEXT) and here. Nothing regenerated.
- **Reading 52's note** → recorded: FLAW-0010 `a`'s references are placeholder-shaped by D5's
  design; S3 reads the class's detection rates as rates against obviously fictional references;
  a plausible-shaped lexicon, if ever wanted, is a later producer version and never attaches a
  real name to a fictional work. 0.6.0's lexicon stands; no byte changes.
- **Reading 61's note** → recorded: the cycle pair's formal file is a valid proof by
  construction; the instance is informal-only; the pair is invalid under D7's either-file rule,
  which is how 0.6.0 already records it (`proof_status: invalid`, one informal record,
  `lean_build` commanded `pass`). No seed, golden or check changes; the version stays 0.6.0.
- **The record commit** (this one): ADR-0011 *Amendments*, STATE and this addendum. The gate on
  the committed tree, the step-7 recheck and the push are recorded in the pull request and under
  the external root.

CHECKS for this addendum:
- No leakage: `contact/` was not opened, and no source was added.
- No seed, no proof byte, no label and no register status changed; no golden rewritten.
- Determinism: unchanged — nothing was generated.
- No verdict on any paper (SD3). **Memory writes:** none.
ESCALATIONS (superseding the block's):
- **This pull request:** merge with a merge commit, so that `ef41a10c…`, the 0.6.0 goldens'
  `engine_commit`, stays a real commit on `main`.
- **FINDING-016** (informational, backlog); FINDING-015 standing.
- Unchanged: human-referee participants (SD13).
STOP-REASON: Al's direction, stop for review. Repo green: the gate on the committed tree is
recorded in the pull request and under the external root, not in the tree it certifies.
