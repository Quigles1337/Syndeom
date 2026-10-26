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
