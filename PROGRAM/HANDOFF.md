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
