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
