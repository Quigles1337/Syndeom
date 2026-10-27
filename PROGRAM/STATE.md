# STATE

- **Updated:** 2026-10-27 UTC (Session 34 — a session launched from `C:\Users\LEET` with the §6
  Session-N resume prompt verbatim; SD15 disclosure first (HANDOFF Session 34, appended without
  reading the file, under a provisional header completed after ADR-0008 step 5); **not an
  examination** — Gate S2 has nothing to examine, which the session could establish only after
  orienting; it certifies nothing): **ADR-0011, the first generator family's design, written —
  decision only, no code (S2.1; STATE's NEXT as it stood).**
  - **Orientation:** the ADR-0008 preflight found the checkout `C:\Users\LEET\Syndeom` on branch
    `claude/s2-citation-cff` at `8ae94316f327194da86e3ce64b58d2573987e325`, **behind** fetched
    `origin/main` `a741ebf956be0bd87ebf1b606f0b9c6df1853043` (**PR 19's merge**, by Al; the same
    tree `a5e7df5488530c560ab7ec9eb608037f92a03820`, which Session 33 gated GREEN on `8ae9431`),
    with the disclosure append the only change. Not on the default branch, so no fast-forward:
    the preflight created the isolated worktree `C:\Users\LEET\Syndeom-wt-20261027` at the fetched
    tip (exit 0, `SNAPSHOT-ESTABLISHED-IN-WORKTREE`; receipts `orient-1-worktree-refused.json` and
    `orient-2-worktree.json` under `C:\Users\LEET\Codex\Temp\syndeom-adr-0011-20261027\`), and the
    disclosure was carried into it as the first action there (ADR-0008 D3). The original
    checkout's append stands uncommitted (Standing escalations).
  - **ADR-0011** (`PROGRAM/DECISIONS/ADR-0011.md`, proposed): one plan data structure as the shared
    interface behind the four S2.1 modules; two producers, `s2_template` (hand-authored seed
    theorems — the FLAWS.md examples — with aligned informal and formal proofs) and `s2_grammar`
    (derivation grammars with a commanded length); `syndeom-rng/1`, specified in the ADR, no new
    dependency; one dial per FLAWS.md `0.1.0` recipe with its kernel or tooling confirmation
    check; commanded and achieved keys with tolerances; `syndeom-manifest/2` manifests with
    `regenerable` storage; goldens from the FLAWS.md examples; `cargo audit` outside the offline
    gate; the crate `engine/generate/` under PolyForm, implemented only after the byte-check
    narrowing. **D12 flags for Al, undecided:** whether the R4 Mnemosyne-derived seed family
    enters, and its licence line under R1(b).
  - **Unchanged:** registers' statuses, manifests, frozen packets, the accepted corpus, the
    constitution, the licence and the tag. **NEXT** moves to ordering step 2, the narrowing.
- **Previous update (Session 33):** 2026-10-27 UTC (Session 33 — an informed working session, not an examination,
  continuing Session 32's conversation at Al's direction; not launched with the §6 prompt, so no
  SD15 disclosure): **`CITATION.cff` added, and its refresh joins the deposit procedure, before
  the tag (Al's ruling of 2026-10-27, Al's rulings on PR 19's two calls and Al's ruling on the
  open point, verbatim in ADR-0010 *Amendments* and HANDOFF Session 33).**
  - **Orientation:** the ADR-0008 preflight (receipt
    `C:\Users\LEET\Codex\Temp\syndeom-citation-cff-20261027\orient-1-sync.json`)
    fast-forwarded the clean `main` to fetched `origin/main`
    `99bdf0362e12d7d7c405566396d7a1a7f01c04a1`, tree `c063cc6589fc4f5d7605690ae7103cbd87993f5c`.
    That is **PR 18's merge** (2026-10-27T06:05:00Z, by Al). Its tree equals the PR's head tree,
    which Session 32 gated GREEN.
  - **`CITATION.cff`** (Citation File Format 1.2.0, schema-valid): `doi` is the concept DOI
    `10.5281/zenodo.23682780`; the version DOI `10.5281/zenodo.23682781` is listed under
    `identifiers` as the Gate S1 design registration; `version`, `date-released` and `title`
    match the latest deposit. It has no `commit` field (Al's ruling on PR 19).
  - **The deposit procedure** now refreshes `CITATION.cff` in the commit that is tagged, after the
    version DOI is reserved in the Zenodo draft. The tagged file is the reference: Zenodo is
    corrected to match it, and the records commit after the deposit confirms the match (Standing
    escalations, first bullet; ADR-0010 *Amendments*).
  - **Unchanged:**
    - the licence, the preservation tool, the README and the `gate-s1-registration` tag;
    - registers' statuses, manifests, frozen packets and the accepted corpus;
    - NEXT.
- **Previous update (Session 32):** 2026-10-27 UTC (Session 32 — an informed working session, not an examination,
  continuing Session 31's conversation at Al's direction; not launched with the §6 prompt, so no
  SD15 disclosure): **the Gate S1 design registration is published on Zenodo, and the records
  now say so (Al's instruction and rulings of 2026-10-27, quoted in HANDOFF Session 32).**
  - **Orientation:** the ADR-0008 preflight (receipt
    `C:\Users\LEET\Codex\Temp\syndeom-post-deposit-records-20261027\orient-1-sync.json`)
    fast-forwarded the clean `main` to fetched `origin/main`
    `2018d5e79ea23fa200e560d628bec5e91edf0956`, tree `3d7889def475b2b2f30d8acfd3dee31589a06a6e`.
    That is **PR 17's merge** (2026-10-27T02:48:36Z, by Al). Its tree equals the PR's head tree,
    which Session 31 gated GREEN.
  - **The deposit is done** (Standing escalations, first bullet): the annotated tag
    `gate-s1-registration` on `2018d5e`, the archive built from it, and Zenodo record 23682781
    (version DOI `10.5281/zenodo.23682781`; concept DOI `10.5281/zenodo.23682780`). Every
    identifier was confirmed from Zenodo's API before it was written.
  - **README:** the passages that presented Gate S1 as open now cite its closure (this file,
    HANDOFF Session 29, PR 15) and NEXT below; a DOI badge and a Citation section were added.
  - **Corrected here:** the Standing-escalations bullet on Session 29's pull request now records
    PR 15's merge.
  - **Unchanged:**
    - the licence, the preservation tool, and the `gate-s1-registration` tag;
    - registers' statuses, manifests, frozen packets and the accepted corpus;
    - NEXT.
- **Previous update (Session 31):** 2026-10-27 UTC (Session 31 — an informed working session, not an examination,
  continuing Session 30's conversation at Al's direction; not launched with the §6 prompt, so no
  SD15 disclosure): **third-party notices and the Apache-2.0 text, ahead of the Gate S1
  registration deposit (Al's rulings of 2026-10-27; ADR-0010 *Amendments*).**
  - **Orientation:** the ADR-0008 preflight (receipt
    `C:\Users\LEET\Codex\Temp\syndeom-registration-licence-20261027\orient-1-sync.json`)
    fast-forwarded the clean `main` to fetched `origin/main`
    `368bcfb5c05c23536a32bfc15b9537173fb0fef7`, tree `d33402ca2bfa0efcee577b2855b0741c07cdbe6e`.
    That is **PR 16's merge** (2026-10-27T02:09:06Z, by Al). Its tree equals the PR's head tree,
    which Session 30 gated GREEN.
  - **The audit**, file by file against the SD16 records at `368bcfb`, found:
    - one file whose recorded rights permit no redistribution (the advisory note, R1(b));
    - one CC BY-NC 4.0 copy (S-0005);
    - Apache-2.0 files, with no Apache-2.0 text in the tree;
    - 30 ground-truth items whose derived records had no licence recorded for themselves.
  - **Added:**
    - `LICENSES/Apache-2.0.txt`: S-0019's text, fetched again from SPDX. Its SHA-256 equals
      S-0019's record, and it is byte-identical to `LICENSE` through `c9372b1`.
    - `THIRD-PARTY-NOTICES.md`: every file not under PolyForm, with its licence and attribution.
    - A pointer in README § Licence, and an S-0019 addendum (no new S-number).
  - **The registration deposit** (rulings 1, 2 and 4) follows this pull request's merge (Standing
    escalations):
    - the tag `gate-s1-registration` on the merge commit;
    - a zip from the tag that excludes the advisory note and the Leiden PDF.
  - **Unchanged:**
    - the licence (ADR-0010 D1–D7) and the preservation tool;
    - registers' statuses, manifests, frozen packets and the accepted corpus;
    - NEXT.
- **Previous update (Session 30):** 2026-10-27 UTC (Session 30 — an informed working session, not an examination,
  continuing the conversation of Session 29 at Al's direction): **ADR-0010, the relicense to the
  PolyForm Noncommercial License 1.0.0, accepted by Al on 2026-10-26 with rulings C1–C5.**
  - **Orientation:** the ADR-0008 preflight (receipt
    `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\preflight-orient.json`) fast-forwarded
    the clean `main` to fetched `origin/main` `c9372b1355a6d687dec6287f908639ac670db659`, tree
    `4caa81f0c56416a02830d19d4a11eb45c9c4a9f4`. That is **PR 15's merge** (2026-10-27T00:25:39Z,
    by Al), whose tree is the one Session 29 gated on its committed rulings commit.
  - **The licence:** `LICENSE` now holds the Required Notice line and the verbatim PolyForm text
    (S-0073, SHA-256 `c0ea4a89…fabbe5`, equal to Al's stated digest). Copyright is held personally
    by Alexander David Zalewski; commercial licensing is at adz@alphx.io. The four crates'
    `license` fields are `PolyForm-Noncommercial-1.0.0`, and README § Licence carries a
    plain-language summary.
  - **The preservation check** gained three narrow exceptions (ADR-0010 D6), with six narrowness
    tests in the gate. The S1.3 reproduction at `3cd5558` still yields the retained inventory
    byte-identical; at HEAD it passes, differing only in three `preservation` fields (bullet
    below).
  - **Replay of the admitted S1.2 pair from HEAD ends here.** It runs on `e6a7cae…`..`c9372b1`
    (procedure below; run once at `c9372b1` in a clean clone: PASS, byte-identical to the
    original production). The 18 committed receipts are unchanged.
  - **Unchanged:** historical "Apache" statements in frozen files, and the pair's recorded wrapper
    licence (C4, C5).
  - **Ordering:** the S2 generator decision is now **ADR-0011**, and the byte-check narrowing
    lands before any S2 code (NEXT).
  - **Unchanged:** registers' statuses, manifests, frozen packets and the accepted corpus.
- **Previous update (Session 29):** 2026-10-26 UTC (Session 29 — **the Gate S1 cold-start examination: PASS; Gate S1
  CLOSED; phase S2 opens**, effective on `main` when Al merges this session's pull request). A
  fresh conversation with no prior turns, launched from `C:\Users\LEET` (not a git work tree; no
  Git snapshot attached) with the §6 Session-N resume prompt in its ADR-0009 wording, verbatim and
  alone. It opened as ADR-0009 prescribes: SD15 disclosure first, appended to HANDOFF without
  reading it (2026-10-26T23:14:39Z), under a provisional header completed after the preflight
  (HANDOFF Session 29). Its ADR-0008 preflight found the checkout **synchronized** with fetched
  `origin/main` `e029c80e1be149bef8fe6ceb76eed59c235ed8b4`, tree
  `67023777965d5747c5e3e5fe3f655825f658c046` (receipt
  `C:\Users\LEET\Codex\Temp\syndeom-s1-cold-start-exam-20261026\preflight-orient.json`). **PR 14
  (Session 28's record) merged** at 2026-10-26T22:35:49Z (18:35:49 America/New_York), merged by
  Al, creating that main. Its parents are the PR's base `0afbbd1…` and its head
  `fea670843d85212148be5995d244a97c83829d4e`, and its tree equals the head's, which Session 28's
  staged-tree gate certified GREEN (receipt
  `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\gate-staged.json`). CI
  pull_request run 40413519657 and push run 40413650650 both succeeded. Two later events are
  recorded only in that external root and are reconciled here:
  - the launch-directory hazard line Session 28 proposed was appended to the executor's SYNDEOM
    memory file on Al's approval, after the merge (`memory-change.txt`; Memory policy below);
  - four merged branches were deleted from `origin` on Al's instruction
    (`deleted-branches-manifest.txt`); the held branch was kept.

  The examination re-derived the program state from the repository in one pass. The ORIENT gate
  was GREEN, and the S1.3 reproduction on a fresh clone printed `PASS: 35 archived
  representations / 32 effective logical items`, differing from the retained inventory only in
  `exact_files_checked` 370 → 369, as this file predicted. Every Gate S1 criterion has evidence
  (table below). Three SD15 judgments were stated for Al's audit; **Al upheld all three on
  2026-10-26** (Standing escalations, verbatim), so the PASS is not conditional. Registers, frozen
  packets and the accepted corpus are unchanged.
- **Previous update (Session 28):** 2026-10-26 UTC (Session 28 — a fresh conversation launched with the §6 Session-N
  resume prompt in its ADR-0009 wording, verbatim and alone, but **inside the checkout**
  `C:\Users\LEET\Syndeom`. Before its first action the harness attached a Git status and history
  snapshot (branch, `git status`, the last five commit subjects) to the conversation, which SD15
  counts as program state read before disclosing, so it is **not an examination**. It certifies
  nothing and is recorded as a non-exam session at Al's direction of 2026-10-26.) It opened as
  ADR-0009 prescribes: SD15 disclosure first, appended to HANDOFF without reading it, under a
  provisional header completed after the preflight (HANDOFF Session 28). Its ADR-0008 preflight
  found the checkout **synchronized** with fetched `origin/main`
  `0afbbd17cdc5e9e93691612a95bd5950e3243e10`, tree `27106131f522357184171e19d0e0e51ba2f30d54`
  (receipt `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\preflight-orient.json`).
  **PR 13 (Session 27's record) merged** at 2026-10-26T21:42:45Z (17:42:45 America/New_York),
  merged by Al, creating that main. Its parents are the PR's base `6e198e0…` and its head
  `fbadee37b5148e25cd803e6df01d5346dd9e05b9`, and its tree equals the head's, which Session 27
  gated GREEN (receipt `C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\gate-staged.json`,
  raw log re-hashed and matching). CI pull_request run 40403469499 and push run 40406649416 both
  succeeded. **Launch-directory finding** (Memory policy below): a session launched inside the
  checkout receives that snapshot before its first action, so it can never be an examination.
  The examination is launched from outside the checkout, from `C:\Users\LEET`, as earlier
  sessions were. Registers, frozen packets and the accepted corpus (35 archived representations,
  32 effective logical items) are unchanged; Gate S1 stays OPEN.
- **Previous update (Session 27):** 2026-10-26 UTC (Session 27 — launched with the §6 Session-N resume prompt in its
  ADR-0009 wording, but inside the conversation that built Session 26: **not cold** under SD15's
  conversation-context corollary, so **not an examination**, and it certifies nothing; recorded
  as a non-cold §6 launch, not as a Gate S1 attempt). It opened exactly as ADR-0009 prescribes —
  SD15 disclosure first, appended to HANDOFF without reading it, under a provisional header
  completed after the preflight (HANDOFF Session 27) — and its ADR-0008 preflight found the
  checkout **synchronized** with fetched `origin/main`
  `6e198e066c128976d55d0194f278cc645baf9249`, tree `4ad8422654cb56d9ffe68a717100df5af240a22e`
  (receipt `C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\preflight-orient.json`). **PR 12
  (ADR-0009) merged** at 2026-10-26T21:03:28Z (17:03:28 America/New_York), merged by Al, creating
  that main; its parents are the PR's base `3cd5558…` and its head
  `61cc974ca67bb47d0ac7aaa5f2a9b6c219969b19`, and its tree equals the head's, which Session 26
  gated GREEN (receipt `C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\gate-final.json`, raw
  log re-hashed and matching). CI pull_request run 40397221741 and push run 40401087386 both
  succeeded. ADR-0009, the amended SD15 and §6 Session-N prompt, and the reproduction tool's
  one-line fix are on main. Registers, frozen packets and the accepted corpus (35 archived
  representations, 32 effective logical items) are unchanged; Gate S1 stays OPEN.
- **Previous update (Session 26):** 2026-10-26 UTC (Session 26 — an informed working session launched with Al's task
  text, not the §6 prompt; not an examination, and it certifies nothing about Gate S1). **PR 11
  (Session 25's record) merged** at 2026-10-26T19:55:55Z (15:55:55 America/New_York; merge
  committer stamp 19:55:54Z), merged by Al, creating main
  `3cd5558223294571dd85164eb0d2730cb1911637`. Its parents are the PR's base `5356412…` and its
  head `e1c81505017bd772dc4cb9caf36aaaf41cd90717`, and its tree
  `125b1ed56d8dcab8074119eb51fc3467dbb09dda` equals the head's, which Session 25 gated GREEN
  (receipt `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\gate-staged.json`, raw log
  re-hashed and matching). CI pull_request run 40389787616 and push run 40391129353 both
  succeeded. **ADR-0009 accepted by Al (2026-10-26) and applied:** every session launched with
  the §6 Session-N resume prompt now makes the SD15 disclosure first, whether or not it attempts
  a gate test, before any action but reading the constitution and ADR-0005 in full and reading and
  hashing the memory; it appends the disclosure to HANDOFF without reading that file, under a
  provisional block header completed after orientation; a session that reads program state before
  disclosing cannot be an examination. The §6 prompt's first sentence now says so, and the prompt
  points to ADR-0008 for orientation. ADR-0008 D1 step 0 follows (bullet below). The
  constitution's SHA-256 is now `110f5360af60075a8e799dce0dc235d1ef8785bd40b0b34c38fa53ddb1d5479c`
  (38,958 bytes; previously `89c198b0…8102`). The Session-25 escalation is decided. The first
  staged-tree gate failed (4 S1.3 reproduction tests rejected the amended constitution); by Al's
  option-1 ruling the reproduction tool now skips the constitution, which changes only by ADR, as
  it skips README and STATE (ADR-0009 D5; one line). **S1.3 reproduces at both revisions:** at
  `3cd5558` the retained `inventory.json` is byte-identical; at HEAD the reproduction passes and
  differs only in `preservation.exact_files_checked`, 370 → 369 (bullet below). Registers, frozen
  packets and the accepted corpus (35 archived representations, 32 effective logical items) are
  unchanged; Gate S1 stays OPEN.
- **Previous update (Session 25):** 2026-10-26 UTC (Session 25 — a fresh conversation launched with the §6 Session-N
  resume prompt; it attempted the Gate S1 cold-start examination and is **ineligible as an
  examination**: before appending its SD15 disclosure it ran orientation commands (including a
  fetch) and read STATE, HANDOFF entries, Git history and ADR-0008, contrary to ADR-0008 D1 step 0
  and SD15's "before any other action". The disclosure was then appended late, in full, and
  stands (HANDOFF Session 25). Its ADR-0008 preflight then found the checkout **synchronized**
  with fetched `origin/main` `535641227f408095f36a92000e808ed5b65e135c`, tree
  `1fdeda3eca982fbaec04ab2b1f8fdaaeb82c70ea` (receipt
  `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\preflight-orient.json`). **PR 10 (the
  Session-24 cold-start preparation) merged** at 2026-10-26T19:28:28Z (15:28:28
  America/New_York; merge committer stamp 19:28:27Z), merged by Al, creating that main. Its tree
  exactly equals tested head `3f0171a7dc36f90154739b5bbafce11da8f37dc2` (`git diff --quiet`
  exit 0; full gate GREEN on that tree 2026-10-26T19:22:04Z–19:22:58Z, receipt
  `C:\Users\LEET\Codex\Temp\syndeom-cold-start-prep-20261026\gate-commit-3f0171a.json`); its first
  parent is the PR's base `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10`, so main has no intervening
  commits. Tested-head CI (run 40386340630) and main-push CI (run 40387061357) both succeeded.
  `git ls-remote --heads origin` now lists only `main` and the held
  `claude/s1-harvest-candidates-28-30` at `c3a3d36`. The external root's
  `deleted-branches-manifest.txt` (written 19:29Z) lists ten merged branches removed after the
  merge, the cleanup Session 24's escalation records as authorized by Al. Registers,
  frozen packets, the accepted corpus (35 archived representations, 32 effective logical items)
  and the constitution's bytes are unchanged; Gate S1 stays OPEN; the reason step 0 was easy to
  miss is escalated (Standing escalations, first bullet).
- **Previous update (Session 24):** 2026-10-26 UTC (an informed preparation session, not the Gate S1 examination,
  opened per ADR-0008 D1 steps 1–5 from fetched `origin/main`
  `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10`, tree `b5fe517fb4fa58846df83187aaa11ee7c2954762`):
  PR 9 (the Session-23 orientation repair) merged at **2026-10-22T00:07:26Z** (2026-10-21
  20:07:26 America/New_York), creating main `ffb63a6cf30ff7a78cdf9b99944cb17433a1bf10`. Its tree
  `b5fe517fb4fa58846df83187aaa11ee7c2954762` exactly equals tested head
  `5e24d37d8a1882d182a27076ba03905588513542` (full gate GREEN on that tree,
  2026-10-22T00:00:49Z–00:04:39Z; receipt in PR 9 and under
  `C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\final-gate-5e24d37d8a18\`); its
  first parent is the PR's base `d8c0d9cbcef0af5249cbf439808b74d8828bd69d`, so fetched main has
  no intervening commits. Tested-head and main-push CI both succeeded. Stale statements are
  corrected: Blockers; the checkout note in NEXT-SINGLE-ACTION; the conditional "after/once this
  reconciliation is on main" wording in the Gate line, the Gate S1 table's cold-start row and
  NEXT; and Current build verification (the PR 8 readiness bullet is kept as historical). The
  session is recorded in HANDOFF Session 24. Registers, frozen packets, the constitution's bytes
  and the accepted corpus (35 archived representations, 32 effective logical items) are
  unchanged; Gate S1 stays OPEN and the next action is still the separate genuine examination,
  opened per ADR-0008 D1.
- **Previous update (Session 23):** 2026-10-21 UTC (the orientation repair under Al's decisions of
  2026-10-21, an informed working session on branch `claude/s1-orientation-repair` from `main`
  `d8c0d9cbcef0af5249cbf439808b74d8828bd69d`): **ADR-0008**, the accepted addendum governing
  §4.1 (fetch and identify the examined snapshot before reconstructing state; preflight
  `scripts/orient_preflight.py` with receipts; offline cases all passed), is the governing
  orientation procedure (bullet below). Al's decision 1 preserves the held harvest branch at
  `c3a3d36` and defers its admission review until after Gate S1 closes; decision 3 keeps the
  existing dispositions. Session 22's retrieval evidence for candidates 28–31 is preserved
  externally with full hashes; the accepted corpus (35 archived representations, 32 effective
  logical items), registers, frozen packets and the constitution's bytes are unchanged. Gate S1
  stays OPEN; the next action is the separate genuine examination, opened per ADR-0008 D1.
  Event: `analysis/s1-orientation-repair/`.
- **Previous update (Session 22):** 2026-10-21 UTC — a session launched with the §6 resume prompt ran
  the paused harvest from an unfetched working copy (its `origin/main` ref was 24 commits stale,
  so §4.1 reconstructed the Session-20 state); its candidates 28–30, all admitted (five items),
  are held on branch `claude/s1-harvest-candidates-28-30` for Al's disposition and are **not on
  main**. Main's corpus, registers, frozen S1.4 packet and readiness packet are unchanged; the
  ORIENT note now says to fetch first; Gate S1 stays OPEN for the separate cold-start
  examination, which remains the next action (HANDOFF Session 22).
- **Previous update (PR 8 readiness):** 2026-10-21 UTC — PR 7's actual main activation and S1 readiness reconciled.
  Owner acceptance remains dated 2026-10-20 America/New_York, at the exact ruling
  `37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f`; the hash-bound decision and ADR-0003
  are unchanged. PR 7 merged at **2026-10-21T03:33:35Z** (2026-10-20 23:33:35
  America/New_York), creating main `1c56028532ea135603dbad5423d5555313bfd9ec`.
  Its tree `14b451866491dc4a2b866ec0b1d726d792385916` exactly equals tested head
  `a337375dbae49150a1a40fa46f6d1417835aadd2`; fetched main has no intervening commits.
  The existing Rust consumer passed on clean merged main and reproduced the retained
  inventory bytes. Logical key `s1_ingest-v0.1.0-63e23a952084` selects exactly one
  effective status from `s1_ingest-v0.1.0-15ab865466d6`: 35 archived representations
  / 32 effective logical items, unflagged subset 18. **Status-support hold discharged
  on verified main.** Both raw objects and original producing revisions remain intact.
  S-0025 is author error/locator evidence; the accepted written counterexample is
  separately attributed. The full analytic bridge remains unformalized.
  [Activation and fresh evidence](../analysis/s1-post-merge-readiness/README.md);
  [current effective inventory](../analysis/s1-first-parent-integration/report/inventory.json);
  [S1 readiness assessment](../analysis/s1-post-merge-readiness/READINESS.md).
  No additional substantive S1 exit criterion is identified as unmet for this admitted
  prefix; Gate S1 stays OPEN pending the separate genuine cold-start examination.
- **Previous update (first-parent proposal, before PR 6 merge):** 2026-10-20 UTC — PR 5 owner merge reconciled; first-parent mathematical
  status-support witness checked, metadata amendment proposed, canonical promotion held.
  PR 5 merged at 20:41:46Z to main `1a76d9ac21cd5d2e4311f9164178327c0ca128bb`, tree
  `3b9028c0502add53ac646108132d89ba5f1e5666` (same as PR head
  `618271daf7b8b091382f800826dbe03156f4d803`); initial worktree clean, no open PRs.
  At a=1,b=2,k=2 the displayed expansion gives 1 versus 0; at s=2 the original lemma's
  residual is log(2)/3>0, derived with convergence/nonvanishing and checked again via
  the missing Euler factor. Author testimony, executor reasoning and partial Lean
  support are distinct. New packet: `analysis/s1-first-parent-status/`.
  Actual Rust ingest generated and validated candidate `s1_ingest-v0.1.0-15ab865466d6`
  twice outside the live corpus. Its false/counterexample reference replaces the old
  source-document attribution prospectively; both embedded statuses agree. Owner ruling
  and integration remain pending. Old metadata, all S1.4 bytes and 34 representations /
  32 logical items remain unchanged. No canonical production or Gate S1 closure.
- **Previous update (S1.4 before PR 5 merge):** 2026-10-20 UTC — post-merge reconciliation and S1.4 completed for the
  existing prefix, with Gate S1 open. Owner merged PR 4 at 18:10:13Z; actual base/corpus
  `e6a7cae4392dd1565ee57f9fb727cc01e453687d`, tree
  `2e4d454048ac916a0047bd908ae3051ba1b981af`; initial worktree clean, no open PRs.
  Reporting implementation `4e46564ac62bc2bd7a5bef54265e9b2dd26772bb` was committed before
  final generation. Rust recomputed 34 representations / 32 roots / two canonical children,
  five observed classes / nine zero classes, 30 roots missing canonical length, one valid
  positive pair and 22 documentary source-array groups. Four report artifacts replayed
  byte-identically; fresh pair build and 11-file historical replay passed with all 20
  producer inputs unchanged. Evidence: `analysis/s1-4/`. Source labels are unchanged;
  the first-parent hold, seven truth and seven severity questions remain explicit.
- **Previous update (S1.2 before PR 4 merge):** 2026-10-20 UTC (S1.2 producer and proposed admission under ADR-0007.
  Source-map correction `d88fad2bb7b339c56bd9964167d3e0acddc8ee4e` precedes tested producer
  `eb1b96d5b8084ceb1de734fffc90c9b480ec32bf`. Actual generation, both-file and parent/child
  validation, emitted Lean/type/axiom audit, 11-file fresh replay and CLI refusal checks passed.
  Branch inventory: 32 raw + two canonical, proposing one additional independent positive item;
  accepted-main baseline remains 31 raw + one canonical, zero admitted S1.2 pairs.
  Draft review and admission remain pending; Gate S1 stays open. Evidence:
  `analysis/s1-2/implementation/`. First-parent hold and harvest resume are unchanged.)
- **Previous update (S1.2 procedure):** 2026-10-20 UTC (S1.2 procedure and first candidate review: ADR-0007;
  Judson 2025 Theorem 2.2.6 paired prospectively with `Nat.infinite_setOfPred_prime`
  at the existing Mathlib pin. Primary source/rights and statement correspondence reviewed;
  focused Lean/axiom and Rust pure-API checks passed. No pair admitted or producer built;
  31 raw parents plus one canonical representation, the separate promotion hold and harvest
  resume point are unchanged. Gate S1 remains open. Draft review packet: `analysis/s1-2/`.)
- **Previous update (S1.3):** 2026-10-20 UTC (S1.3 first canonical derivation: `syndeom-derive` 0.1.0;
  parent `s1_ingest-v0.1.0-00f28e252823`, child `s1_derive-v0.1.0-c08f9e3ec092`;
  producer committed first at `49e47237924eddf0c39c24aefad0eb9aa02ee8b3`, real generation
  and independent regeneration passed. Source text remains external; one separate parent
  promotion hold recorded. No harvest advancement. S1.2/S1.4 and Gate S1 remain open.)
- **Previous update (historical):** 2026-10-14 UTC (Session 20 — the ninth Route A harvest: candidates 25–27
  examined in order, all three admitted yielding 6 items — four from one published erratum (a
  main theorem withdrawn to a conjecture, the proposition and the assertion its proof needed,
  and a moduli theorem shown false by the erratum's own example), one true theorem whose proof
  invoked a comparison principle outside its hypotheses, one wrong degree shift; prefix through
  `1403.4485`; `groundtruth/` holds thirty-one `raw/1` items. Session 19 — candidates 22–24: all
  three admitted yielding 5 items (four undetermined). Session 18 — candidates 19–21: 2 admitted
  (3 items), 1 rejected (D8, a cumulative comment). Session 17 — candidates 16–18: all three
  admitted yielding 4 items (the third undetermined statement truth; the first
  `retraction`-kind item). Session 16 — candidates 13–15: 2 admitted (3
  items), 1 found-but-unclassifiable. Session 15 — candidates 11–12: 1 admitted (2 items, a
  journal correction with a counterexample figure), 1 found-but-unclassifiable. Session 14 — candidates 7–10: 3
  admitted (4 items), 1 rejected; the examination helper steps over withdrawn versions. Session 13 — candidates 3–6: 2 admitted
  (one with the first undetermined statement truth), 2 rejected. Session 12 recorded Al's D8
  precision. Session 11 — the first Route A harvest: finder built and
  run (2,472 candidates), candidates 1–2 examined in order and both admitted and ingested
  (`groundtruth/` now holds two `raw/1` items with records); S-0021..S-0023 registered for the
  position paper's §0 disclosure; memory hazard added. Session 10 recorded Al's rulings of
  2026-10-13 (ADR-0001 `/2`, ADR-0003 resolved, ADR-0005) and built the producer; Session 9
  decided ADR-0003; Session 8 closed R3; Session 7 recorded the earlier rulings)
- **Phase:** S2 — GENERATOR (entered 2026-10-26, Session 29, at Gate S1's closure; effective on
  `main` at the merge of Session 29's pull request). S1 — GROUND-TRUTH CORPUS is closed on the
  admitted prefix.
- **Gate:** **S2 OPEN** (Gate S2 exit criteria below; nothing evidenced yet).
  **Gate S1 CLOSED 2026-10-26 (Session 29):** the cold-start examination met every launch
  condition. It was a fresh conversation launched from `C:\Users\LEET` with the §6 Session-N
  prompt (ADR-0009 wording) verbatim and alone, and the SD15 disclosure came first. It re-derived
  the program state from the repository in one pass without guessing, and found the gate GREEN at
  `e029c80`; result **PASS**; record in HANDOFF Session 29, whose first entry is the disclosure.
  The PASS rests on three SD15 judgments stated for Al's audit (Standing escalations). Earlier
  attempts stay as recorded: Sessions 22 and 25 ineligible; Session 27 not cold; Session 28 not an
  examination. S1.1 closed on the admitted unskipped prefix through candidate 27 / `1403.4485`,
  not an exhausted harvest (ADR-0003 D8, Al's ruling (iii)). PR 4 admitted one positive
  statement-fidelity pair; PR 5 merged the frozen S1.4 report; PR 6 archived the
  witness/proposal; the owner accepted its exact ruling and PR 7 activated verified integration on
  main. The criterion-by-criterion readiness assessment above carries all qualifications. Harvest
  resume at candidate 28 / `1403.6317` remains saved; any later harvest is a later prefix under
  its own gate record and never rewrites the Gate S1 table (ADR-0003 D8). The closed S1 table is
  retained below as the record.
  **Gate S0 CLOSED 2026-10-13
  (Session 6):** the cold-start exam, run in a fresh conversation launched with the §6 prompt
  verbatim and alone, re-derived the program state from the repo in one pass without guessing and
  found the gate GREEN at `2b7c573`; result **PASS**; record in HANDOFF Session 6, whose first
  entry is the SD15 memory disclosure. **Al confirmed the PASS on 2026-10-13 (Session 10, ADR-0000
  Addendum 3): the closure is not provisional; no re-run.** The closed S0 table is retained below
  as the record.
- **Founding sweep:** `PROGRAM/SWEEP-2026-10.md` (41,060 bytes, SHA-256
  `f4bfb2adcdf7c6714101ad6ddc2fdcf528e98e7c993accb8a5dbe04a8115b03e`, ferried by Al in Session 3
  and verified against Al's stated digest before placement; recomputed in Session 6). It is a
  **map, not evidence** (SD11 banner on line 3): nothing enters any register from it; where it
  disagrees with the S0.1 register, the register stands (its own ferry note says so). It was
  redacted under SD8 before the ferry. Its ten ranked sources were retrieved and read in Session 5
  (CLAIM-0006 examination, done); its runners-up remain unretrieved (carried note below).
- **Advisory note:** `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md` (27,238 bytes, LF, SHA-256
  `f3e40b684a8b437fa4f0ba9d059d23d6ae61ce88ce8bca84ceb49fd7c181fa2c`, ferried by Al under ruling
  R3 of 2026-10-13 and verified against the stated digest before the commit, Session 8). A
  ChatGPT-produced design discussion of 2026-10-13 relayed verbatim under a provenance header,
  with Claude's verification record and its rights status under the R1 boundary (the DARQ
  MNEMOSYNE material in it is proprietary and outside SYNDEOM's release — R1(b), carried forward by
  ADR-0010; the program is named
  with its owner's authorization, Al, 2026-10-13). **SD11 banner: advisory, not evidence** —
  nothing enters any register from it; its two "recorded counterexamples" are not in the Mnemosyne
  repository (its header says so); its §6/§7 finding is Mnemosyne change control, not SYNDEOM's.
  Its only standing effect here is R4 (candidate S2 seed family, standing rulings below).
- **Default branch:** `main` — discovered from the clone of the empty remote and the REST API
  (ADR-0000 D3); confirmed by Al (Q2). Remote: `origin` = `https://github.com/Quigles1337/Syndeom`
  (**public**; created by Al 2026-10-12; Actions enabled).
  **`LICENSE` = PolyForm Noncommercial 1.0.0 since ADR-0010 (2026-10-26)**, preceded by
  `Required Notice: Copyright 2026 Alexander David Zalewski
  (https://github.com/Quigles1337/Syndeom)`:
  - copyright held personally by Alexander David Zalewski;
  - commercial licensing at adz@alphx.io;
  - text provenance S-0073;
  - the four crates' `license` fields set to `PolyForm-Noncommercial-1.0.0`.

  It was Apache-2.0 from Session 7 through `c9372b1` (Q3, ADR-0000 Addendum 2 R1; text S-0019),
  and copies obtained under Apache-2.0 remain under it. The release boundary (third-party
  material; DARQ Labs proprietary material) is carried forward and stated in README.md §
  Licence. **Since Session 31 (Al's rulings of 2026-10-27; ADR-0010 *Amendments*):**
  - `THIRD-PARTY-NOTICES.md` lists every file not under PolyForm, with its licence and
    attribution;
  - `LICENSES/Apache-2.0.txt` holds the Apache-2.0 text (S-0019 addendum) for the files under it.
  **ORIENT note (SD7):** `git symbolic-ref refs/remotes/origin/HEAD` resolves to `main`; if it ever
  fails, fallback `git remote show origin`, repair `git remote set-head origin -a`.
  **Fetch first (Session 22, 2026-10-21):** run `git fetch origin` before §4.1's `git status` /
  `git log`, then compare `main` with `origin/main` (`git status -sb`; `git log main..origin/main`;
  `gh pr list --state merged`): a working copy's remote ref is a claim until fetched, and on
  2026-10-21 a §6-launched session reconstructed a 24-commit-stale state through §4.1 without
  noticing and ran the superseded harvest NEXT. A diverged local commit goes to a branch, never a
  force-push. (Historical note; the binding procedure is ADR-0008, next bullet.)
- **GOVERNING ORIENTATION AMENDMENT — `PROGRAM/DECISIONS/ADR-0008.md` (accepted by Al,
  2026-10-21; decision 2 in `analysis/s1-orientation-repair/OWNER-DECISIONS.md`).** Before any
  program state is reconstructed or any NEXT is executed: verify the repository and remote
  identity; `git fetch --prune origin` (a failed fetch stops the session — no cached NEXT);
  discover the default branch from the remote; record the fetched tip's full SHA and tree, the
  checked-out SHA, tree and branch, the working-tree status and the ahead/behind commit lists;
  select the snapshot — a synchronized clean checkout is it, a clean behind-only checkout of the
  default branch is fast-forwarded (ff-only) and re-verified, anything ahead, diverged or dirty is
  left untouched and examined from an isolated worktree at the fetched tip; reconstruct only from
  that checkout; re-fetch and compare before every push or PR, and stop if the target moved.
  Never force-push, reset, discard or blind-merge. `python -I -X utf8
  scripts/orient_preflight.py orient [--sync | --worktree DIR] [--allow-dirty PROGRAM/HANDOFF.md]
  --receipt FILE` performs and records it (exit 0 snapshot established; 2 fetch failed; 3
  checkout not current; `recheck --baseline SHA` exits 4 when the target moved); the gate does
  not call it. For every session launched with the §6 Session-N prompt (ADR-0009, amending
  step 0), SD15's disclosure (ADR-0005) precedes the preflight, appended to HANDOFF without
  reading that file, under a provisional block header completed after step 5; its HANDOFF append
  is the one declared change, and it is never erased or rewritten; a synchronized clean workspace
  may be prepared by an informed session beforehand, but the fresh executor still performs its
  own disclosure and its own preflight and records the fetched SHA. ADR-0009 (2026-10-26)
  amended SD15, the §6 Session-N prompt and ADR-0008 step 0: the constitution's SHA-256 is now
  `110f5360af60075a8e799dce0dc235d1ef8785bd40b0b34c38fa53ddb1d5479c` (38,958 bytes), and the
  preservation baseline `89c198b0…8102` is historical. ADR-0005 is unchanged.
- **S1.3 REPRODUCTION AFTER ADR-0009 — two facts (ADR-0009 D5; Al's option-1 ruling of
  2026-10-26).** The effective-inventory reproduction (`cargo run --locked --manifest-path
  engine/Cargo.toml -p syndeom-ingest --example s1_effective_inventory -- <clean clone> <new
  output directory outside it>`; packet `analysis/s1-first-parent-integration/`) checks its
  accepted baseline byte for byte at the clone's HEAD. ADR-0009 amended the constitution, and the
  tool's `preserve()` now skips `PROGRAM/SYNDEOM-PROGRAM.md` beside `README.md` and
  `PROGRAM/STATE.md`, because the constitution changes only by ADR and never belonged in a
  data-reproduction byte check (source `engine/ingest/examples/support/effective.rs` SHA-256
  `516bd44e…9775` → `92a816bc…b30c`; nothing else in the tool changed). **(1) At main
  `3cd5558223294571dd85164eb0d2730cb1911637`** (that revision's tool) the reproduction prints
  `PASS: 35 archived representations / 32 effective logical items` and the retained
  `inventory.json` is reproduced **byte-identical** (SHA-256 `b626d1b5…1b803`). **(2) At HEAD**
  (ADR-0009's tree and later, unless a later change says otherwise) it prints the same `PASS`,
  and its `inventory.json` differs from the retained one **only** in
  `preservation.exact_files_checked`, **370 → 369** — the constitution is no longer counted among
  the exact files; every count and item is unchanged (SHA-256 `df3a2c7f…a7f5`). An examiner
  should expect exactly that one difference at HEAD; it is not a broken S1.3. Evidence, in
  scratch clones: R1 at `3cd5558` (`repro.log`) and R5 on ADR-0009's staged tree
  (`repro-R5.log`), both under `C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\`. The gate's
  `first_parent_amendment` tests, whose fixture reads the same baseline from the working tree,
  pass 15 of 15 with the fix; without it four of them failed (the first staged-tree gate, same
  root, `gate-staged.json`).
- **S1.3 REPRODUCTION AFTER ADR-0010 (2026-10-26, Session 30).** The tool gained three narrow
  exceptions (ADR-0010 D6): `LICENSE` is skipped, `PROGRAM/SOURCES.md` is append-only, and the
  four crate manifests may differ only in their `license` line. Tool source SHA-256: `92a816bc…b30c`
  → `49e6cd6d…1a67`. The gate test `engine/ingest/tests/relicense_exceptions.rs` (6 tests) shows
  each exception is narrow.
  - **At `3cd5558`, with that revision's tool,** the retained `inventory.json` is still reproduced
    byte-identical (`b626d1b5…1b803`).
  - **At HEAD** the reproduction prints the same `PASS: 35 archived representations / 32
    effective logical items`. Its `inventory.json` differs from the retained one **only** in
    `preservation`:
    - `exact_files_checked` **363** (370 retained, 369 after ADR-0009);
    - `append_only_prefixes_checked` gains `PROGRAM/SOURCES.md`;
    - the new field `license_line_only_files_checked` lists the four manifests.

    Every count and item is unchanged.
  - In `provenance.json`, `tool_source_sha256` changes for `effective.rs`,
    `engine/ingest/Cargo.toml` and `engine/manifest/Cargo.toml`.

  An examiner should expect exactly these differences. Measurements and logs are under
  `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\`.
- **S1.2 PAIR REPLAY — where it runs (ADR-0010 *Consequences*).**
  - **Replay from HEAD ended at ADR-0010.** The four crate manifests' `license` line changed, and
    `engine/pair/Cargo.toml` and `engine/manifest/Cargo.toml` are two of the producer's 20 pinned
    inputs. A `syndeom-pair` built at HEAD therefore embeds their new digests, and
    `verify_producer` refuses producer revision `eb1b96d`.
  - **Where replay runs:** any `main` revision from `e6a7cae4392dd1565ee57f9fb727cc01e453687d`
    (PR 4's merge, the first with the helper) through `c9372b1355a6d687dec6287f908639ac670db659`
    (the last before ADR-0010). Each descends from `eb1b96d` and holds its 20 inputs
    byte-identical (`git diff --quiet eb1b96d c9372b1 -- $(cat engine/pair/input-files.txt)` exits
    0). A checkout of `eb1b96d` itself holds the producer but not the helper, which was added in
    `0b30d78`.
  - **Procedure:**
    1. Clone `origin` into a new directory and `git checkout --detach
       c9372b1355a6d687dec6287f908639ac670db659` (or any revision in that range).
    2. In its `lean/`, establish the pinned Mathlib build (`lake exe cache get`, then `lake
       build`). The producer reads `lean/.lake/packages/mathlib` at the pinned revision and
       compiles the emitted file.
    3. From the checkout root, run `python -I -X utf8 analysis/s1-2/implementation/reproduce.py
       --root <that checkout> --producer eb1b96d5b8084ceb1de734fffc90c9b480ec32bf --output <new
       path outside the checkout>`. Omit `--artifact-root`. Add `--source-dir
       <retained-five-object-directory>` only if the fixed upstream objects are unavailable; the
       original retained set is at
       `C:\Users\LEET\Codex\Projects\Syndeom-S1.2-implementation-work-2026-10-20\production-run-01\sources`.

    The helper builds the producer, verifies its inputs, generates into an isolated root,
    compiles and audits the emitted Lean file, replays into a second fresh root and runs the
    labelled controls (`analysis/s1-2/implementation/README.md`, "Reproduce into a new
    destination").
  - The 18 committed receipts that record the producer's input closure — 16 under
    `analysis/s1-2/implementation/evidence/production/receipts/` and 2 under
    `analysis/s1-4/evidence/` — are unaffected.
  - **Run once, exactly as written, at Al's direction (Session 30, 2026-10-27): PASS. No step
    needed correction.** Evidence is under the external root
    `C:\Users\LEET\Codex\Temp\syndeom-pair-replay-20261026\`: the procedure as written, the
    wrapper, the raw log `replay.raw.log` (SHA-256 `42383a2a…c04e`) and `replay-out/`.
    1. Fresh clone of `origin`, detached at `c9372b1`, tree `4caa81f0…`, clean (01:16:53Z–01:16:56Z).
    2. `lake exe cache get` (01:16:56Z–01:24:19Z) and `lake build` (01:24:19Z–01:30:05Z), both
       exit 0, about 13 min with the machine's Mathlib cache already populated.
    3. `reproduce.py` (01:30:05Z–01:31:14Z), exit 0.

    Its `verification-receipt.json` (SHA-256 `37d7037e…cc42`) reads `PASS: generation,
    actual-byte validation, emitted Lean audit, replay and controls; draft admission only`:
    - checkout `c9372b1…`, producer `eb1b96d…`; rustc 1.91.0; Lean 4.34.1;
    - all five source objects fetched over HTTP 200 and hash-verified (`--source-dir` not
      needed);
    - 33 commands, each with its expected exit; all 18 refusal controls refused (exit 2);
    - emitted-file audit PASS, standard axioms only;
    - fresh replay byte-identical (11 files).

    The 11 file digests equal those of the original production receipt
    (`analysis/s1-2/implementation/evidence/production/verification-receipt.json`), and the
    emitted `Proof.lean` equals the original (`038c70bf…`). The five generated metadata files
    (both manifests, both source maps, the emitted `Proof.lean`) are byte-identical to the
    admitted pair committed under `groundtruth/`. Replay from HEAD was not attempted;
    ADR-0010's statement that it is refused rests on the producer's code.
- **S1.3 orientation verification (2026-10-20):** existing Lean/Rust code passed the gate,
  with documentation reconciliation in progress; 29 existing tests passed, axioms `[propext]`.
  The new crate was enabled only after this gate. Its focused producer checks now pass
  37 tests (24 manifest + 5 ingest + 8 derivation), fmt and clippy. Logs: `analysis/s1-3/`.
  Real generation/reproduction passed from the producer commit. The final staged artifact/evidence
  tree passed the full gate; `analysis/s1-3/gate-final.txt` records exit 0 and GATE: GREEN.
- **Current build verification (Session 34):** ORIENT gate on the examined worktree at fetched
  `origin/main` `a741ebf956be0bd87ebf1b606f0b9c6df1853043` (tree
  `a5e7df5488530c560ab7ec9eb608037f92a03820`) with one declared change, the carried SD15
  disclosure appended to `PROGRAM/HANDOFF.md`: full gate, detached with a log, 2026-10-27T19:30:55Z–19:41:28Z, exit 0 / `GATE: GREEN` — Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 88 Rust tests, 467 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 73 (raw log SHA-256 `245980cdba01e84de7541471091b0b74a3a9b3b4b55156ec0a0364e06778067e`). Receipt and raw log are under the
  external root `C:\Users\LEET\Codex\Temp\syndeom-adr-0011-20261027\` (`gate-orient.raw.log`,
  `gate-orient.json`). The Rust reproduction was not rerun: this session changes only `PROGRAM/`
  text. The commit that adds ADR-0011, this bullet and HANDOFF Session 34 receives its own full
  gate on its committed tree, with a clean working tree, recorded in its pull request and under
  the same root, not in the tree it certifies.
- **Session 33 build verification:** no separate ORIENT gate on main `99bdf03`. Its
  tree `c063cc6589fc4f5d7605690ae7103cbd87993f5c` is the tree Session 32 gated GREEN on PR 18's
  head `d1e4d75`:
  - run 2026-10-27T05:46:42Z–05:57:09Z, recorded in `gate-committed.json`;
  - raw log SHA-256 `b37f667f…c603`, under
    `C:\Users\LEET\Codex\Temp\syndeom-post-deposit-records-20261027\`.

  Each of this pull request's commits (adding `CITATION.cff`, then recording Al's rulings on it)
  receives a full gate on its committed tree, with a clean working tree. Those gates are recorded
  in the pull request and under `C:\Users\LEET\Codex\Temp\syndeom-citation-cff-20261027\`, not in
  the tree they certify.
- **Session 32 build verification:** no separate ORIENT gate on main `2018d5e`. Its
  tree `3d7889def475b2b2f30d8acfd3dee31589a06a6e` is the tree Session 31 gated GREEN on PR 17's
  head `a6e2038`:
  - run 2026-10-27T02:35:08Z–02:41:56Z, recorded in `gate-staged.json`;
  - raw log SHA-256 `833fde02…d951`, under
    `C:\Users\LEET\Codex\Temp\syndeom-registration-licence-20261027\`.

  The commit recording the deposit receives a full gate on its committed tree, with a clean
  working tree. That gate is recorded in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-post-deposit-records-20261027\`, not in the tree it
  certifies.
- **Session 31 build verification:** no separate ORIENT gate on main `368bcfb`. Its
  tree `d33402ca2bfa0efcee577b2855b0741c07cdbe6e` is the tree Session 30 gated GREEN on PR 16's
  head `0470684`:
  - run 2026-10-27T01:32:57Z–01:42:06Z, recorded in `gate-committed-2.json`;
  - raw log SHA-256 `dadddea6…7594`, under `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\`.

  The commit adding the notices receives a full gate on the exact tree it commits. That gate is
  recorded in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-registration-licence-20261027\`, not in the tree it
  certifies.
- **Session 30 build verification:** no separate ORIENT gate on main `c9372b1`. Its
  tree `4caa81f0c56416a02830d19d4a11eb45c9c4a9f4` is the tree Session 29 gated GREEN on its
  committed rulings commit `6b3dbda` (2026-10-27T00:00:33Z–00:08:28Z; `gate-committed.json`,
  raw log SHA-256 `437716ec…58f7`, under
  `C:\Users\LEET\Codex\Temp\syndeom-s1-cold-start-exam-20261026\`). During the work, `cargo
  test` for the preservation suites (15 + 6 tests), `cargo clippy -D warnings` and `cargo fmt
  --check` passed. The commit that adds ADR-0010 receives a full gate on its committed tree,
  recorded in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-relicense-20261026\`, not in the tree it certifies.
- **Session 29 build verification:** ORIENT gate on main
  `e029c80e1be149bef8fe6ceb76eed59c235ed8b4` (tree `67023777965d5747c5e3e5fe3f655825f658c046`)
  with one declared change, the unstaged SD15 disclosure appended to `PROGRAM/HANDOFF.md`: full
  gate, detached with a log, 2026-10-26T23:16:58Z–23:25:02Z, exit 0 / `GATE: GREEN` — Lean 8,926
  jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests, 462 tracked files, Q 17 / SD 17 /
  FLAW 14 / CLAIM 7 / S- 72. Receipt and raw log (SHA-256
  `04c556afd76e90e3aacf940de3376087e9394c2832ab15118f581f37fb620943`) are under the external
  root `C:\Users\LEET\Codex\Temp\syndeom-s1-cold-start-exam-20261026\` (`gate-orient.json`,
  `gate-orient.raw.log`). The S1.3 effective-inventory reproduction ran on a fresh clone of
  `origin` at `e029c80` (2026-10-26T23:25:45Z–23:25:50Z) and printed the expected `PASS`. Its
  `inventory.json` (SHA-256 `df3a2c7f…a7f5`) differs from the retained one only in
  `preservation.exact_files_checked` 370 → 369 (`repro-head/`, `repro-head.log` under the same
  root). The commit that adds this bullet and HANDOFF Session 29 receives its own full gate on
  its exact staged tree, recorded in its pull request and under the same root, not in the tree it
  certifies.
- **Session 28 build verification:** no separate ORIENT gate on main `0afbbd1`. Its
  tree `27106131f522357184171e19d0e0e51ba2f30d54` is the tree Session 27's staged-tree gate
  certified (2026-10-26T21:09:57Z–21:19:09Z, GREEN; `gate-staged.json`, raw log SHA-256
  `d6fd8942…79d1` re-hashed and matching, under
  `C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\`). The commit that adds this bullet and
  HANDOFF Session 28 receives its own full gate on its exact staged tree. That gate is recorded
  in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\`, not in the tree it
  certifies. No Rust reproduction was rerun: this session changes only `PROGRAM/STATE.md` and
  `PROGRAM/HANDOFF.md`.
- **Session 27 build verification:** no separate ORIENT gate on main `6e198e0`: its
  tree `4ad8422654cb56d9ffe68a717100df5af240a22e` is the tree Session 26's final staged-tree gate
  certified (2026-10-26T20:34:44Z–20:35:33Z, GREEN; `gate-final.json`, raw log SHA-256
  `b25710de…c96e` re-hashed and matching, under
  `C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\`). The commit that adds this bullet and
  HANDOFF Session 27 receives its own full gate on its exact staged tree, recorded in its pull
  request and under `C:\Users\LEET\Codex\Temp\syndeom-sec6-launch-20261026\`, not in the tree it
  certifies. No Rust reproduction was rerun: this session changes only `PROGRAM/STATE.md` and
  `PROGRAM/HANDOFF.md`.
- **Session 26 build verification:** no separate ORIENT gate on main `3cd5558`: its
  tree `125b1ed56d8dcab8074119eb51fc3467dbb09dda` is the tree Session 25's final staged-tree gate
  certified (2026-10-26T19:44:55Z–19:45:36Z, GREEN; `gate-staged.json`, raw log SHA-256
  `b003ccee…054b` re-hashed and matching, under
  `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\`). The commit that adds ADR-0009,
  the amendments, this bullet and HANDOFF Session 26 receives its own full gate on its exact
  staged tree, recorded in its pull request and under
  `C:\Users\LEET\Codex\Temp\syndeom-adr-0009-20261026\`, not in the tree it certifies. The first
  staged-tree gate (2026-10-26T20:16:14Z–20:16:51Z, nothing committed) was RED: four
  `first_parent_amendment` tests rejected the amended constitution (`gate-staged.json`); the
  tool's one-line fix (ADR-0009 D5) followed Al's ruling. The S1.3 reproduction was rerun at
  `3cd5558` and on the amended tree (bullet near the top of this file); no other Rust
  reproduction was rerun, because only `PROGRAM/` text and that one tool line change.
- **Session 25 build verification:** main `535641227f408095f36a92000e808ed5b65e135c`
  (tree `1fdeda3eca982fbaec04ab2b1f8fdaaeb82c70ea`) with one declared change, the unstaged SD15
  disclosure appended to `PROGRAM/HANDOFF.md`: full gate, detached with a log,
  2026-10-26T19:36:57Z–19:41:40Z, exit 0 / `GATE: GREEN` — Lean 8,926 jobs, golden axioms
  `[propext]`, fmt/clippy, 82 Rust tests, 461 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 /
  S- 72. Receipt and raw log (SHA-256
  `a97599f8c585071fb9c7a1fa3773f27133a1ab0840d4603a4c47816b831fe2ff`) under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-s1-exam-attempt-20261026\` (`gate-orient.json`,
  `gate-orient.raw.log`). PR 10's second commit `3f0171a`, whose tree main carries, was gated
  GREEN by Session 24 (previous update above). The commit that adds this bullet and HANDOFF
  Session 25 receives its own full gate, recorded in its pull request and under the same
  external root, not in the tree it certifies. The Rust reproduction was not rerun: this session
  changes only `PROGRAM/STATE.md` and `PROGRAM/HANDOFF.md`.
- **PR 10 build verification:** PR 10's first commit
  `0801ecb52d4ec839bed90291527b11ca8c3e1281`, tree `78a30035d7137eb0a52ab200b3ba25406331c855`
  (main `ffb63a6` plus this session's first STATE edits): full gate on that exact staged tree,
  committed unchanged, detached with a log, 2026-10-26T19:02:21Z–19:12:29Z, exit 0 /
  `GATE: GREEN` — Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests, 461
  tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 72. Receipt and raw log (SHA-256
  `112787b8c0c5fca28b40926171e090c6f9f13788f0cee04dc39fb8e0b643bf13`) under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-cold-start-prep-20261026\` (`gate-staged.json`,
  `gate-staged.raw.log`). The Rust reproduction was not rerun: PR 10 changes only
  `PROGRAM/STATE.md` and `PROGRAM/HANDOFF.md`. The commit that adds this bullet and HANDOFF
  Session 24 receives its own full gate, recorded in PR 10 and under the same external root,
  not in the tree it certifies.
- **Historical build verification (PR 8 readiness):** clean merged main
  `1c56028532ea135603dbad5423d5555313bfd9ec`, tree
  `14b451866491dc4a2b866ec0b1d726d792385916`, full local gate **GREEN** at
  `2026-10-21T04:00:41.304180+00:00`: Lean 8,926 jobs, golden axioms `[propext]`,
  fmt/clippy, all 82 Rust tests, 428-file LF/BOM/register checks, exit 0.
  Actual [receipt](../analysis/s1-post-merge-readiness/evidence/main-gate.json) and
  [log](../analysis/s1-post-merge-readiness/evidence/main-gate.log).
  PR 7's tested head has the identical tree; both tested-head and main-push CI succeeded.
  Fresh existing-consumer reproduction passed at 2026-10-21T03:50:28.787156+00:00:
  inventory byte-identical to retained output, provenance bound to the merge, overwrite
  refused unchanged. Rust rechecked all accepted-baseline preservation conditions,
  including 35 S1.4 files, 47 witness/proposal files and 20 original pair inputs.
  The final reconciliation commit receives its own full gate and reproduction;
  exact head/tree, actual results and external receipts are recorded in its associated
  PR, outside the tree certified. No earlier gate substitutes for that final result.
  See [event and verification convention](../analysis/s1-post-merge-readiness/README.md).
- **Session 23 build verification (2026-10-21):** implementation commit
  `d4dd5da956a02bbb4895dc6f6e15d0e16bcf9f23` (tree `4a8155dfecfbe422781a832e6550b257c55c84f6`):
  full gate on that exact staged tree, detached with a log, 23:44:48Z–23:55:37Z, exit 0 /
  `GATE: GREEN` — Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests, 455
  tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 72; the unchanged Rust reproduction on the
  clean committed tree passed (23:58:50Z–23:58:54Z; inventory SHA-256 `b626d1b5…1b803` identical
  to the retained integration inventory; 35 archived representations / 32 effective logical
  items unchanged). Receipts and logs: `analysis/s1-orientation-repair/evidence/`. The recording
  commit that adds them with the event README and HANDOFF Session 23 receives its own gate and
  reproduction, recorded in its pull request and under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\`, not in the tree it certifies.
- **Session 22 build verification (2026-10-21):** clean `main` `26c2555` (PR 8's merge) full gate,
  detached with a log, 22:47:14Z–22:48:06Z: Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy,
  82 Rust tests, 447 tracked files, Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 72, exit 0 —
  `GATE: GREEN`; and again on the final tree of the Session-22 reconciliation commit before it was
  committed (detached run with a log immediately before the commit; its times are left out of this file to avoid a self-referential edit, `GATE: GREEN`).
- **Historical build verification (first-parent proposal):** clean-base `bash scripts/check.sh` at
  `1a76d9ac21cd5d2e4311f9164178327c0ca128bb`, tree
  `3b9028c0502add53ac646108132d89ba5f1e5666`, passed 2026-10-20T21:44:58Z:
  Lean build, golden axioms `[propext]`, fmt/clippy, all 67 Rust tests, 328 tracked files
  and registers. Both actual orientation runs are retained in the new packet; the second
  was an unnecessary retry while a sandbox process listing hid the first running process.
  The exact packet Lean file separately compiles with all eight audited theorems using
  `[propext, Classical.choice, Quot.sound]`; no complete analytic bridge is claimed.
  Candidate validation and two fresh generations passed. The final clean committed tree
  receives its own full gate, focused Lean audit and preservation check; actual revision,
  tree, results and external receipts belong in this task's draft PR, without a self-referential
  recording commit. Details and retained development failures: new packet `VERIFICATION.md`.
- **Historical build verification (S1.4):** full `bash scripts/check.sh` on the initial reporting implementation
  `9375af283e748e915ffc6e31df12f1bb81122e1d`, tree
  `96cf29f99656b039561170cc9356d0839f97a6ef`, ended **2026-10-20T20:23:14.733002+00:00**:
  exit 0 / GATE: GREEN; Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy clean,
  67 Rust tests including 16 new reporting integration tests; 301 tracked files and
  register checks passed. Receipt/log: `analysis/s1-4/evidence/gate-implementation.json`
  and `.txt`. Review then separated known statement-role exclusions from genuinely
  missing proof lengths; corrected tool `4e46564ac62bc2bd7a5bef54265e9b2dd26772bb`
  passed all 16 focused tests and clippy before two fresh final report runs.
  The final report/documentation commit is gated separately; its exact clean
  commit/tree and actual outcome are retained externally and in this task's draft PR,
  without editing the checked tree to embed its own hash. The earlier orientation run
  reached the Lean build/audit but failed fmt while untracked reporting drafts were being
  edited; it is retained as a failed development run, never a clean-base certificate.
- **Historical build-status record (before S1.4):** the gate is `bash scripts/check.sh` (ADR-0002 D6): `lake build` + axiom audit
  of the golden fixture + `cargo fmt --check` + `cargo clippy -D warnings` + `cargo test` + LF check
  + register cross-reference check; it prints `GATE: GREEN` on success. **Last verified GREEN:
  2026-10-20 06:28:55Z (S1.2 pre-generation producer tree)** — Lean 8,926 jobs,
  golden axioms `[propext]`, fmt/clippy clean, 51 tests, 203 tracked files,
  Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 68, exit 0. Exact unchanged tested tree
  `ad9cbee9a1939ddf3e0deb200a8afbe8db642d53` became producer
  `eb1b96d5b8084ceb1de734fffc90c9b480ec32bf` before generation.
  Logs/receipt: `analysis/s1-2/implementation/evidence/producer-checks/producer-gate-01.*`.
  Fresh actual emitted-file audit and full production verification passed at 06:34:11Z.
  The subsequent exact artifact/evidence commit gate is retained in an external receipt
  and the new draft PR without modifying that checked tree. **Previous GREEN:
  2026-10-20 05:17:18Z (S1.2 staged procedure/candidate packet)** — Lean 8,926 jobs,
  golden axioms `[propext]`, fmt/clippy clean, 37 tests, 172 tracked files;
  Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 68, exit 0. Tested index tree
  `1034c92f7724e9f86f590c3ae344c1d3291c9c71`; new log/receipt:
  `analysis/s1-2/evidence/gate-staged-packet.txt` / `gate-staged-packet.json`.
  Final recording-only edits are checked before commit; the final committed-tree gate
  is recorded in the unmerged draft PR without modifying the checked tree. **Previous GREEN:
  2026-10-20 04:58:19Z (S1.2 orientation, documentation drafting in progress)** — Lean 8,926
  jobs, golden axioms `[propext]`, fmt/clippy clean, 37 tests, 150 tracked files checked;
  registers Q 17, SD 17, FLAW 14, CLAIM 7, S- 68; exit 0. This run began on main `8adcb48`
  and finished with source/doc drafts; it is not a certification of the final packet tree.
  Receipt/log: `analysis/s1-2/evidence/gate-orientation.json` and `gate-orientation.txt`.
  The separate S1.2 candidate audit uses the standard three axioms; it is not the golden
  audit and does not admit a pair. **Previous GREEN:
  2026-10-20 04:26:20Z (PR 2 corrected documentation tree)** — Lean 8,926 jobs, golden axioms
  `[propext]`, fmt/clippy clean, 37 tests passed (24 manifest + 5 ingest + 8 derivation),
  149 tracked files checked; Q 17, SD 17, FLAW 14, CLAIM 7, S- 66; exit 0. Log:
  `analysis/s1-3/gate-pr2-review.txt`. HANDOFF retains main's exact historical prefix, followed
  by the original S1.3 block; its last NEXT and this file's single NEXT both name S1.2.
  The final recording commit is gated again before integration; PR 2 and the integration
  merge message record that exact checked revision/tree and result without rewriting the
  checked files. **Previous GREEN:
  2026-10-20 03:47:27Z (S1.3 staged artifact/evidence tree)** — Lean 8,926 jobs, golden axioms
  `[propext]`, fmt/clippy clean, 37 tests passed (24 manifest + 5 ingest + 8 derivation),
  147 tracked files checked, registers coherent (Q 17, SD 17, FLAW 14, CLAIM 7, S- 66),
  exit 0. Log: `analysis/s1-3/gate-final.txt`. Final recording-only edits also pass byte/register
  and diff checks before commit. **Previous GREEN:
  2026-10-20 03:10:57Z (Session 21, documentation branch)** — local gate after populating the
  pinned Mathlib cache: Lean 8,926 jobs, axioms `[propext]`, Rust fmt and clippy clean, 29/29
  tests, 122 tracked files checked, registers coherent; exit 0. Final recording edits also passed
  the byte/register and diff checks before the documentation commit.
  **Previous GREEN: 2026-10-14 21:48:44Z–21:50:35Z (Session 20, detached run with a log) on the Session-20 harvest
  tree with every edit staged, and again on the final tree before the harvest commit** — Lean `Build completed successfully (8926 jobs)`,
  axioms `[propext]`; Rust fmt and clippy clean, 29/29 tests (24 manifest + 5 ingest); 112 text files `i/lf` + 10 binaries;
  registers coherent (Q 17/17, SD 17/17, FLAW 14/14, CLAIM 7/7, S- 65/65). Earlier GREEN runs:
  Session 4 (before the S0.6 commit), Session 5 (at `132788c`), Session 6 (at `2b7c573` and on its
  own tree), Session 7 (22:54:28Z–23:04:29Z, cold OS cache), Session 8 (23:21:33Z–23:31:44Z and
  23:32:45Z–23:33:19Z), Session 9 (23:35:52Z–23:36:23Z and 23:36:55Z–23:37:25Z) and Session 10's
  rulings commit (2026-10-14 00:07:19Z–00:17:54Z) and producer commit (00:43:55Z–00:52:27Z and
  00:54:48Z–00:55:45Z, detached) Session 11 (02:07:11Z–02:18:18Z and 02:19:15Z–02:21:08Z,
  detached) Session 12 (02:34:42Z–02:45:22Z and 02:46:06Z–02:46:35Z, detached) Session 13
  (03:01:46Z–03:02:24Z and 03:03:06Z–03:03:41Z, detached) Session 14 (03:47:34Z–03:48:31Z and
  03:49:23Z–03:49:58Z, detached) Session 15 (03:52:24Z–04:03:12Z cold, 03:58:38Z–04:03:12Z and
  the final-tree run, detached), Session 16 (04:52:05Z–05:02:27Z cold, 05:07:44Z–05:09:07Z and
  05:09:56Z–05:10:25Z, detached), Session 17 (08:08:51Z–08:18:18Z cold, 08:30:22Z–08:30:47Z
  and 08:30:57Z–08:31:23Z, detached), Session 18 (19:44:55Z–19:55:18Z cold, 20:05:35Z–20:09:55Z
  and 20:10:27Z–20:11:44Z, detached) and Session 19 (20:14:35Z–20:15:39Z, 20:35:33Z–20:43:22Z
  and 20:43:48Z–20:49:47Z, detached), each on its own tree before its commit. A run the harness
  killed for system memory (00:21:40Z, in the axiom audit) is not a GREEN run; since then the
  gate is run detached with a log (HANDOFF Session 10). **Standing instruction:** update this line at every GREEN run a HANDOFF entry
  records, so it never lags HANDOFF. **Timing:** about 30 s with a warm `lean/.lake` *and* a warm
  OS file cache; **about 10 min after a reboot even with `lean/.lake` warm** (Session 7: the axiom
  audit, `lake env lean Audit.lean`, loaded Mathlib's oleans from a cold disk cache, lean.exe
  above 1.4 GB — not a hang; run the gate in the background with a log); about 15 min on a fresh
  machine (toolchain install, nine clones, 8,908-file cache download) — a `lake build` that prints
  nothing for minutes under a pipe is the download, not a hang. The advisory CI
  (`.github/workflows/check.yml`) runs the same script and is never a required check; it
  completed `success` for `132788c` and `2b7c573`.
- **Toolchain (ADR-0002, Session 4):** Lean `leanprover/lean4:v4.34.1` (`lean/lean-toolchain`),
  which is Mathlib's own `lean-toolchain` at the pinned Mathlib commit
  `d13f23b723b8a846827a245b89c10fc7d3f11612` (tag `v4.34.1`; `lean/lakefile.toml` +
  `lean/lake-manifest.json`, eight transitive Lake deps locked) — the Q8 rule, chosen as the newest
  Mathlib release tag with a stable Lean (a load-bearing divergence from KOBER's v4.33.1/no-Mathlib).
  Rust `1.91.0` with clippy + rustfmt (`rust-toolchain.toml` at the root), workspace
  `engine/Cargo.toml`, crates `syndeom-manifest`, `syndeom-ingest`, `syndeom-derive`, `syndeom-pair`.
  `engine/Cargo.lock` lists 34 packages: the new local pair crate and the 26 unchanged baseline entries (24 dependencies
  plus two workspace crates), six new decompression dependencies (S-0066), and `syndeom-derive`.
  The former 25-entry sentence predated the ingest crate. JCS
  serializer `serde_json_canonicalizer 0.3.2`. CI advisory on `actions/*@v5`. Licences of
  everything pinned: S-0008. Machine inventory as of 2026-10-12 in `PROGRAM/SIBLING-MAP-2026-10.md`
  §8. For the record (S5, not a reason for any pin — SD8): the contact corpus `openai/math` happens
  to pin the same Lean and Mathlib commit (S-0001); the ADR-0002 criterion was fixed before that
  was compared.
- **Constitution:** `PROGRAM/SYNDEOM-PROGRAM.md` v1.0, ratified by Al 2026-10-12 (Session 2);
  **amended three times: ADR-0004 (2026-10-13, Session 7) — §2 S3.2, fooling length as equivalence to
  chance with a design-derived N_min; ADR-0005 (2026-10-13, Session 10) — SD15, count-only
  disclosure of unrelated memory index lines in a cold-start exam; ADR-0009 (2026-10-26, Session
  26) — SD15 and the §6 Session-N prompt, memory disclosure first in every session launched with
  that prompt (SHA-256 now `110f5360…479c`).** Rulings of record: the
  ADR-0000 addendum (Session 2, Q1–Q17), Addendum 2 (Al's rulings of 2026-10-13, R1–R4) and
  Addendum 3 (Al's rulings of 2026-10-13 on ADR-0003 (i)–(v) and the SD15 audit). ADR-only; the
  constitution's version string is unchanged because no rule assigns version numbers to
  amendments (noted in ADR-0004).
- **Taxonomy:** `PROGRAM/FLAWS.md` 0.1.0 ratified (FLAW-0001..0014).
- **Registers (unchanged in Session 6):** CLAIM-0001..0005 and 0007 VERIFIED (0007 existence
  only); CLAIM-0006 **CONDITIONAL** in a narrowed form (unfalsified as of 2026-10-13; bounding
  sources S-0009..S-0018; a claim of absence cannot become VERIFIED by retrieval; the scoped
  novelty statement is in CLAIMS and lifts §0.6 only in that scope). S-0001..S-0018 VERIFIED;
  INGESTED: S-0004, S-0005 (declarations, CC0 and CC BY-NC 4.0) and S-0010..S-0016 (seven CC BY 4.0
  papers under `sources/`, ~16 MB; all nine stored PDFs re-hashed to their register digests in
  Session 6); S-0007 RFC 8785, S-0009 and S-0018 (arXiv non-exclusive licence) digest-only; S-0008
  the toolchain licence record; S-0017 the `leanprover/comparator` repository. Later entries:
  S-0019 (the LICENSE text, INGESTED), S-0020 (Mann–Whitney 1947, digest-only), S-0021..S-0023
  (the Mathathon open letter, joint statement and event page — the position paper's §0
  disclosure, no CLAIM, digest-only, Session 11), S-0024..S-0027 (the Route A items'
  originals and notices, Session 11; S-0024 INGESTED under `groundtruth/`, the rest digest-only),
  S-0028..S-0031 (Route A items 3 and 4, Session 13, all digest-only), S-0032..S-0037 (Route A
  items 5–8, Session 14, all digest-only), S-0038..S-0039 (Route A items 9 and 10, Session 15,
  digest-only), S-0040..S-0043 (Route A items 11–13, Session 16, digest-only),
  S-0044..S-0049 (Route A items 14–17, Session 17, digest-only);
  S-0050..S-0053 (Route A items 18–20, Session 18, digest-only);
  S-0054..S-0059 (Route A items 21–25, Session 19, digest-only);
  S-0060..S-0065 (Route A items 26–31, Session 20, digest-only).
- **Manifest schema:** `syndeom-manifest/2` since 2026-10-13 (ADR-0001 amended, Session 10:
  `statement_status.value` admits `null` with `method: "undetermined"`, `null ⇒ proof_status
  invalid`; `/1` remains accepted and names the pre-amendment schema — the D8 goldens), ADR-0001
  (Session 3; ratified Session 4). Formats
  `syndeom-informal-md/1`, `syndeom-lean/1`, `raw/1` (archival); ids
  `<generator.id>-v<version>-<config_hash[0:12]>[-s<seed>]`; JCS + SHA-256; FRIEDMAN ADR-0004
  arithmetic by reference; contact artifacts carry `ground_truth: null`; real items get a derived
  canonical artifact per D4 (iii).

## Memory policy

SD15 adopts FRIEDMAN ADR-0003 D1–D3 and the conversation-context corollary; the corollary is
**ratified for SYNDEOM** (Q13, ADR-0000 addendum), cited from FRIEDMAN, not duplicated. The
executor's memory store holds, for SYNDEOM, the repo location and machine hazards only. Exam
sessions write no memory; the Session-6 exam reproduced its recalled memory verbatim as the first
entry of its HANDOFF block and recorded the memory file's SHA-256 there. **Disclosure convention
(ADR-0005, 2026-10-13):** an exam reproduces verbatim every index line naming SYNDEOM or a
sibling and the SYNDEOM memory file in full with its digest, and withholds unrelated programs'
index lines by count with a recorded name-search check, because this repository is public. The
memory file was trimmed in Session 10 on Al's order (the paragraph paraphrasing §0.3, §0.4 and
SD17 removed; hazards kept); its digest is in HANDOFF Session 10 beside the Session-6 one.
**Disclosure timing (ADR-0009, 2026-10-26):** every session launched with the §6 Session-N prompt
makes this disclosure first, whether or not it attempts a gate test, appended to HANDOFF without
reading that file; a session that reads program state before disclosing cannot be an examination;
such a session writes memory only once it has established that it is not an examination.
**Launch directory (Session 28, 2026-10-26; recorded at Al's direction):** whenever Claude Code
is launched inside a git work tree, it attaches a Git status and history snapshot to the
conversation before the first action: the current branch, `git status` and the last five commit
subjects. A session launched inside the checkout `C:\Users\LEET\Syndeom` therefore holds program
state before it can disclose (SD15's "Git history or status"), so it **can never be an
examination**, whatever its prompt, memory or disclosure order.
The launch directory also selects the memory store the harness loads:
- inside the checkout, the key `C--Users-LEET-Syndeom`, whose memory directory is empty, so no
  index is loaded;
- from `C:\Users\LEET`, the machine-wide index that ADR-0005's disclosure handles.

`C:\Users\LEET` is not a git work tree (`git rev-parse` exits 128 there). Sessions 6, 7, 25 and
27 record the harness loading that index at start, so they were launched from it, and none
records a Git snapshot. The examination is launched from `C:\Users\LEET`, outside the checkout.
Its disclosure records what the harness attached before the first action. No constitution or ADR
text changes: this applies SD15 as written.
**Hazard line added after PR 14 (2026-10-26; Al's approval):** the launch-directory hazard
Session 28 proposed was appended to the executor's SYNDEOM memory file after PR 14 merged. The file
went from SHA-256 `163c7b06…2787` to `ce2ce6de…2cc0`; the old bytes are an exact prefix, and the
index is unchanged at `cd943842…fc69`. The record of the write is Session 28's external root
(`memory-change.txt`). Session 29 disclosed the new file in full, diffed it against Session 28's
reproduced copy (the only change is the appended five lines), and judged it a harness hazard, not
program state, subject to Al's audit (Standing escalations). Session 29, launched from
`C:\Users\LEET`, confirms the finding from the other side: the environment block read "Is a git
repository: false", no Git snapshot was attached, and the `C--Users-LEET` index loaded.

## Standing rulings (recorded; their ADRs come later)

- **S2 generator families:** the first family is deterministic — seeded Rust templates and
  grammars. Any LLM-assisted generator is a separate family requiring its own ADR under SD6 and is
  excluded from `battery-freeze-v1` unless that ADR exists (determinism; leakage risk that an LLM
  generator was trained on the contact corpus). ADR at S2.
- **S5 toolchains:** contact corpora carry their own Mathlib pins; S5 ingest records them and may
  need per-corpus toolchains. Note, not solve, until ingest (Q8). First data point recorded in
  S-0001.
- **ADR numbers:** ADR-0001 manifest schema — written (Session 3), ratified by Al (Session 4) with
  its *Amendments*; ADR-0002 toolchain pins, Lean project, Rust workspace, JCS crate, storage
  default, gate and CI — written (Session 4, S0.6), D8 records what was measured; **ADR-0003
  errata-and-retraction harvest procedure for `groundtruth/` (S1.1) — written Session 9**, D1–D9,
  five choices (i)–(v) ruled by Al 2026-10-13 (Session 10; Amendments); ADR-0004 constitution
  amendment to §2 S3.2 (Al's ruling R2 of 2026-10-13) — written Session 7; ADR-0005 constitution
  amendment to SD15 (count-only exam disclosure; Al's SD15 audit ruling of 2026-10-13) — written
  Session 10; ADR-0006 reviewed TeX derivation — written 2026-10-20 (S1.3);
  ADR-0007 first positive-pair procedure and candidate — written 2026-10-20 (S1.2);
  ADR-0008 orientation synchronization and snapshot selection, the accepted addendum governing
  §4.1 — written and accepted 2026-10-21 (Session 23); ADR-0009 constitution amendment to SD15
  and the §6 Session-N prompt (disclosure first) — written and accepted 2026-10-26 (Session 26);
  ADR-0010 relicense to PolyForm Noncommercial 1.0.0, with three preservation-check exceptions —
  written and accepted 2026-10-26 (Session 30); **ADR-0011 the first generator family (S2.1) —
  written Session 34 (2026-10-27), proposed, decision only, D12 flagged for Al; ADR-0012 is the
  next free number, reserved by NEXT for the byte-check narrowing.**
- **S3 calibration ADR obligation for undetermined items (Al's ruling (ii), 2026-10-13):**
  items whose `statement_status.value` is `null` (`method: "undetermined"`, ADR-0001 as amended)
  **count in proof-flaw cells and are excluded from every cell that uses the truth dimension,
  the FLAW-0011 and FLAW-0012 composites included.** Stated in the S3 calibration ADR before any
  curve, beside the ADR-0004 obligations above.
- **S3 calibration ADR obligations (ADR-0004, 2026-10-13):** before any curve is computed it must
  (1) compute N_min by the method ADR-0004 fixes — the smallest balanced cell at which the
  equivalence test (95% bootstrap CI of the AUC inside [0.45, 0.55]) has 90% power at true AUC
  0.5, from the null-hypothesis variance of the Mann–Whitney statistic — and confirm the number by
  simulation against the bootstrap procedure actually used; (2) state how effective N is computed
  for items derived from one seed theorem (for example a cluster bootstrap by seed theorem).
  Cells that cannot reach N_min — human-referee cells under SD13, detectors needing a per-item
  Lean build across the full grid — are reported **underpowered** by design.
- **S2 seed family (Al's ruling R4 of 2026-10-13; ADR at S2):** a Mnemosyne-derived seed family —
  the static-reduction theorem DM-FM-THM-001 and the labelled variants the advisory note proposes —
  is a **candidate** hand-authored golden-fixture family for S2.4, beside the FLAWS.md examples. A
  pipeline test, not calibration data (one theorem, six variants). Its licence line is decided at
  S2 under the R1 release boundary (ADR-0000 Addendum 2). Nothing built yet. The program is named
  with its program owner's authorization (Al, 2026-10-13).
- **Real items and detectors (ADR-0001 D4, ratified Session 4):** `raw/1` is archival; detectors
  consume only `syndeom-informal-md/1` and `syndeom-lean/1`; every S1/S5 item to be scored gets a
  derived canonical artifact (deterministic producer, `config.derived_from` + `method`, parent's
  ground truth with locations re-expressed and argued in the S1.3 record); the `raw/1` parent
  remains the item of record.

## Carried from S0 (notes, not actions)

- **Sweep runners-up unretrieved:** IMProofBench, Open Proof Corpus, FormalAlign, Beyond
  Compilation, ShadowBench, Sadasivan et al., First Proof batches, Greiffenhagen 2024. They stay
  outside the register until a later action retrieves them; the narrowed CLAIM-0006 scope says so.
  Retrieve one only when an S1 item or a later claim needs it; Gate S0's closure did not close
  this.

## Gate S2 exit criteria — OPEN (opened 2026-10-26, Session 29)

| Criterion | Status | Evidence |
|---|---|---|
| S2.1 Modules behind one shared interface: informal generator, Lean 4 formal-variant generator, flaw injector (every FLAWS.md recipe as a dial: presence, location, severity), padding module (FLAW-0013); first family deterministic (standing ruling) | **ADR-0011 proposed (Session 34, 2026-10-27; decision only, no code); Al's rulings on its D12 awaited; the byte-check narrowing precedes any S2 code** | `PROGRAM/DECISIONS/ADR-0011.md` |
| S2.2 Commanded and achieved statistics (tokens, steps, lemmas, flaws per class, severity, padding ratio); per-artifact fidelity verdict | **open** | — |
| S2.3 Manifests and determinism for every artifact (SD1, SD6); `corpus_id` invariant tested | **open** | — |
| S2.4 Verification: property tests; goldens against the FLAWS.md examples; `cargo clippy` deny-warnings; `cargo audit` | **open** | — |
| Fidelity sweep report (commanded vs achieved flaw presence across the parameter grid, tolerances stated) | **open** | — |
| Goldens green; lint and audit clean | **open** | — |
| Cold-start test (§2, SD15) | **open** | — |

## Gate S1 exit criteria — CLOSED 2026-10-26 (Session 29)

| Criterion | Status | Evidence |
|---|---|---|
| S1.1 Errata and retractions: per item the original, the notice, the flaw's location, its FLAWS.md class argued in writing; derivation `documented-by-source`; licence per SD16 | **evidenced for the admitted unskipped prefix; harvest not exhausted — prefix through arXiv id `1403.4485` (candidates 1–27 of 2,472; Sessions 11, 13, 14, 15, 16, 17, 18, 19, 20): 27 examined, 21 admitted yielding 31 items, 4 rejected (two D2.1 — mis-stated quoted theorems with no proof offered; two D8 — one correcting version after the window, one cumulative comment whose only correction predates it), 2 found-but-unclassifiable (withdrawal stubs naming no describable error, one of them superseded by a paper that is silent on the mistake); `groundtruth/HARVEST-LOG.md` is the record of every candidate; harvest resume remains candidate 28** | `PROGRAM/DECISIONS/ADR-0003.md` (Session 9; Al's rulings on (i)–(v) recorded Session 10): D1 routes (arXiv comments, Crossref update relations, zbMATH index; AI-generated excluded), D2 admission, D3 class mapping (undetermined truth admitted under `syndeom-manifest/2`), D4 record (`manifest.json`, bytes, `RECORD.md`, S-entries, `groundtruth/HARVEST-LOG.md`), D5 licences (NC/ND `external`), D6 `raw/1` parent + derived artifact + the producer, D7 log, D8 window, no skipping, prefix gate, D9 the S1.4 table; routes live-checked 2026-10-13. **Producer:** `engine/ingest/` → `syndeom-ingest` (`s1_ingest` 0.1.0, Session 10): writes a `/2` manifest from a config and bytes, refuses on any ADR-0001 D4 or ADR-0003 shape violation (exit 2) and on a same-id byte conflict (exit 3); goldens over fixed bytes (fixture `915d3ed6…1390`, 72 bytes) and two fixed configs — boolean truth (`s1_ingest-v0.1.0-d2134e7fd2c3`, manifest `1f2cf7b9…1f63`) and the null case (`s1_ingest-v0.1.0-4f5d4e93adaa`, manifest `451bde2d…8340`) — config hashes cross-checked in Python. Orientation reconciliation (2026-10-20): the log and 31 admitted items exist; the former empty-slots sentence was stale. |
| S1.2 Informal–formal pairs with the mathlib commit pinned (positive statement-fidelity pairs; drifted pairs are synthetic and live in `corpora/`, SD12) | **one positive pair admitted by owner merge PR 4** | ADR-0007; both pair RECORD post-merge dispositions; actual producer/both-file/lineage/type/axiom/replay evidence in `analysis/s1-2/implementation/`; merge metadata in `analysis/s1-4/config/merge-evidence.json`. No separate GitHub reviewer approval, no PROVEN promotion; statement-only informal selection, zero steps. |
| S1.3 Manifests for real items (`origin: ground-truth`; `raw/1` parent plus derived canonical artifact per ADR-0001 D4; nothing synthetic labeled real) | **35 archived representations (33 raw + two canonical), 32 effective logical items; 30 without canonical lengths. First-parent status support discharged on verified main by PR 7; fresh Rust reproduction and preservation evidence in `analysis/s1-post-merge-readiness/`. After ADR-0009 the reproduction gives the retained inventory byte-identical at main `3cd5558`, and at HEAD a PASS differing only in `preservation.exact_files_checked` 370 → 369 (bullet near the top of this file).** | `s1_ingest-v0.1.0-63e23a952084/` (arXiv 0706.0357 v70, Lemma 1.1; FLAW-0012 + FLAW-0005; old raw retained; stable logical key resolves to accepted false/counterexample representation `s1_ingest-v0.1.0-15ab865466d6`; canonical selection/context/locator review still required; original committed, CC BY 4.0); `s1_ingest-v0.1.0-744652f4b95f/` (arXiv 0707.3799 v2, Theorem 1 b); FLAW-0011 + FLAW-0005; statement true; `external`); `s1_ingest-v0.1.0-5ba6c0f07557/` (arXiv 0902.0680 v1, the dense-subspace lemma; FLAW-0005 gap; **statement undetermined — `value: null`, the first use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-a8ac73896464/` (arXiv 0908.3213 v4, Theorem 4.4; FLAW-0001 propagated + local; statement false; `external`); `s1_ingest-v0.1.0-be3b17e73425/` and `s1_ingest-v0.1.0-72415457dce4/` (arXiv 1005.2398 v2, Proposition 3.3.5 and the lemma following it; FLAW-0012 + FLAW-0005 each; statements false — the proposition by the author's counterexample; `external`); `s1_ingest-v0.1.0-e0c199d3c60a/` (arXiv 1009.3998 v4, Lemma 13.2; FLAW-0011 + FLAW-0001 local; statement true; `external`); `s1_ingest-v0.1.0-448d7aefa486/` (arXiv 1010.2391 v9, Lemma A.3; FLAW-0012 + FLAW-0005; statement false; `external`); `s1_ingest-v0.1.0-1c4c9a7747d0/` and `s1_ingest-v0.1.0-2d93a266558a/` (arXiv 1107.0863 v3, Lemma 12 and Theorem 1; FLAW-0012 each; the lemma false by the authors' counterexample figure, the theorem by source document; `external`); `s1_ingest-v0.1.0-e3cbdfa7ad1c/` (arXiv 1112.6107 v2, Theorem 1; FLAW-0012; statement false by source document — the misstated last clause, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-22ae000cb879/` and `s1_ingest-v0.1.0-fd2bfef40d31/` (arXiv 1201.3327 v2, the former Lemma 5.8 and the former Theorem 5.9; FLAW-0012 fatal and FLAW-0005 gap; the lemma false by source document, **the theorem undetermined — `value: null`, the second use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-dc1e44f39bb9/` (arXiv 1202.5531 v3, Theorem 6; FLAW-0012; statement false by source document, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-68ed1abfecb4/` and `s1_ingest-v0.1.0-a4822a4be06e/` (arXiv 1206.1893 v6, Propositions 34 and 35; FLAW-0012 + FLAW-0005, and FLAW-0005 gap; the first false by source document, **the second undetermined — `value: null`, the third use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-da3a4784c2be/` (arXiv 1206.2228 v1, Theorem 1; FLAW-0012; statement false by source document; **`status.kind` `retraction`, the first**; `external`); `s1_ingest-v0.1.0-685b874cf579/` (arXiv 1206.5522 v5, Proposition 5.3; FLAW-0012 + FLAW-0005; statement false by source document — the hypotheses corrected, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-00f28e252823/` and `s1_ingest-v0.1.0-a93badadec27/` (arXiv 1209.2022 v2, Proposition 3.1 and Theorem 3.2; FLAW-0012, and FLAW-0011 + FLAW-0005 gap; the proposition false by the printed counterexample, the theorem true by source document; `external`); `s1_ingest-v0.1.0-cafb56cb27fb/` (arXiv 1209.4040 v3, Theorem 4.5; FLAW-0005 gap; **undetermined — `value: null`, the fourth use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-45c3ee8319d5/`, `s1_ingest-v0.1.0-486ef22f632b/` and `s1_ingest-v0.1.0-dc0e0b0d5b87/` (arXiv 1210.6709 v1, Theorems 1.6, 2.2 and 2.1; FLAW-0005 fatal, fatal and gap; **all three undetermined — `value: null`, the fifth to seventh uses of ruling (ii)**; `external`); `s1_ingest-v0.1.0-40e6c0da8821/` (arXiv 1303.3650 v2, Theorem 4.6; FLAW-0012 + FLAW-0005; statement false by source document, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-003a35498654/`, `s1_ingest-v0.1.0-0833f61c748c/`, `s1_ingest-v0.1.0-82b4e66d0016/` and `s1_ingest-v0.1.0-315546e87259/` (arXiv 1304.5623 v4, Proposition 3.5, Theorem 3.6 (4), Theorem 4.3 and Theorem 5.3; FLAW-0004 gap, FLAW-0005 gap, FLAW-0012 + FLAW-0004 + FLAW-0004, FLAW-0005 gap; **three undetermined — `value: null`, the eighth to tenth uses of ruling (ii)**, Theorem 4.3 false by the erratum's example; `external`); `s1_ingest-v0.1.0-f634c259799f/` (arXiv 1312.1045 v1, Theorem A.1; FLAW-0011 + FLAW-0005 gap; statement true by source document; `external`); `s1_ingest-v0.1.0-6ae03b18a1bd/` (arXiv 1403.4485 v3, Proposition 5.1; FLAW-0001 propagated + local; statement false by source document; `external`) — all under `groundtruth/`, each with `manifest.json` (`syndeom-manifest/2`, `engine_commit` of the producer's commit, self-checked by the producer) and `RECORD.md` (the D3 class argument); S-0024..S-0065 |
| S1.4 Coverage and power: per flaw class, how many real items at what proof lengths; zero-coverage classes named as a finding | **merged in PR 5; complete for pinned prefix, qualified, reproducible and frozen** | `analysis/s1-4/report/report.md`, inventory/coverage/provenance JSON, frozen bins, two fresh byte-identical runs. Five observed classes, nine zero, 32 historical roots / 17 unflagged roots. Every real-item cell expected underpowered; no numerical power or detector run. `analysis/s1-4/OBLIGATIONS.md` is the historical ledger. The later effective inventory records 35 representations / 32 logical items / 18 unflagged; current requirements and limits are mapped in `analysis/s1-post-merge-readiness/READINESS.md`. |
| Licences recorded for anything touched (SD16) | **recorded; existing storage choices preserved** | Per-source completeness matrix and source records in `analysis/s1-4/`; SOURCES historical prefix unchanged. No new external source ingested or rights inferred. First-parent status support is discharged on verified main under the accepted scoped ruling. External bytes retain historical evidence and must be retrieved/reverified before scoring; the older dispositions and exclusions remain unchanged. |
| Repo green under the gate | **done (and standing)** | ORIENT gate GREEN at `e029c80` plus the declared disclosure append, 2026-10-26T23:16:58Z–23:25:02Z (Current build verification above); the closing commit's staged-tree gate is recorded in its pull request |
| Cold-start test (§2, SD15) | **done — PASS (Session 29, 2026-10-26)**; earlier: Sessions 22 and 25 ineligible attempts; Session 27 not cold; Session 28 not an examination | HANDOFF Session 29, whose first entry is the SD15 disclosure (index and memory file reproduced per ADR-0005; memory judged to hold no program state, for Al's audit). The session was a fresh conversation launched from `C:\Users\LEET` (no Git snapshot) with the §6 Session-N prompt (ADR-0009 wording) verbatim and alone. The ADR-0008 preflight found it synchronized at fetched `origin/main` `e029c80e1be149bef8fe6ceb76eed59c235ed8b4`, tree `67023777…`. It re-derived in one pass from the repository: phase and gate; S1.1 (harvest-log tally: 27 rows, 21 admitted yielding 31 items, 4 rejected, 2 found-but-unclassifiable; last `1403.4485`; ADR-0003 D8 prefix rule); S1.2; S1.3 (fresh-clone Rust reproduction `PASS: 35 / 32`, only `exact_files_checked` 370 → 369); S1.4 (prefix sentence, zero coverage, power statement); registers, pins, escalations and the next action. It compared these with this file: agreement, with three reconciliations of events after this file was written (PR 14's merge; the post-merge memory line and branch deletions recorded in Session 28's external root; the frozen packets' historical `gate_s1` fields). Gate GREEN (row above). The PASS rests on three SD15 judgments stated for Al's audit (Standing escalations). |

## Gate S0 exit criteria — CLOSED 2026-10-13 (Session 6)

| Criterion | Status | Evidence |
|---|---|---|
| S0.1 CLAIM-0001..0005 (and 0007) resolved to VERIFIED or honestly UNVERIFIED with consequences | **done** | all six VERIFIED 2026-10-12 from primary sources (CLAIMS.md transitions; SOURCES.md S-0001..S-0006 with URLs, access times, licences, digests). Consequences stated: S5 corpus location = `github.com/openai/math` (Apache-2.0); counts carry units and three unreconciled formalization units; the AHM page is undated; IAS page unretrievable (403); signatory counts recorded as of access date |
| S0.2 Founding sweep placed with the SD11 banner | **done** | `PROGRAM/SWEEP-2026-10.md`, SHA-256 `f4bfb2ad…b03e` = Al's stated digest (verified before the move, Session 3; recomputed Session 6); banner present; redacted under SD8 before ferry; reconciliation note defers to the S0.1 register |
| S0.3 Constitution ratified by Al; DRAFT marker removed | **done** | `c803c96` `[S0][ADR-0000] ratify the constitution and record Al's rulings`; ADR-0000 addendum |
| S0.4 Manifest schema ADR (`syndeom-manifest/1` extending `friedman-manifest/1`) | **done** | `PROGRAM/DECISIONS/ADR-0001.md` (Session 3; ratified Session 4): D1–D9 under Q10 (structural) and Q11 (fields); FRIEDMAN ADR-0001/0004 and crate re-read at pinned tip `7c3e5dd`; two goldens over fixed bytes (Golden A `0ac16447…a6a1`, Golden B `c568a981…458e`); RFC 8785 retrieved as S-0007; the five flagged choices ruled and the derived-artifact convention added (*Amendments*) |
| S0.5 Flaw taxonomy v0.1 ratified | **done** | `PROGRAM/FLAWS.md` 0.1.0, 14 entries, ratified (Q12; changelog) at `c803c96` |
| S0.6 Scaffold (`lean/` with Mathlib, `engine/`, `lab/`, `corpora/`, `groundtruth/`, `contact/`, `analysis/`, `sources/`, gate, CI) | **done** | `PROGRAM/DECISIONS/ADR-0002.md` (Session 4): Lean `v4.34.1` = Mathlib's `lean-toolchain` at pinned `d13f23b7…` (tag `v4.34.1`), `lake-manifest.json` locked; `engine/` workspace with `syndeom-manifest` (ADR-0001 D1–D7, 23 tests incl. D8 goldens, ADR-0004 vectors, RFC 8785 vectors); `scripts/check.sh` + `scripts/check_registers.py`; advisory CI on `actions/*@v5`; READMEs incl. `contact/` SD8 rules; licences S-0008; D8 measured: Golden B builds, axioms `[propext]` |
| Licences recorded for anything touched | **done** | S-0001 Apache-2.0 (nothing copied); S-0002, S-0003 licence unstated (nothing copied); S-0004 CC0 and S-0005 CC BY-NC 4.0 (PDFs copied under those terms); S-0008 for every toolchain pin; S-0010..S-0016 CC BY 4.0 (PDFs copied); S-0009, S-0018 digest-only |
| Repo green under the gate | **done (and standing)** | `bash scripts/check.sh` → `GATE: GREEN` at `2b7c573` in Session 6 and on the Session-6 tree (Build status above); GREEN at every earlier recorded run |
| CLAIM-0006 examined (deferred S0 action, Al's ruling Session 3) | **done** | Session 5: the ten ranked sources retrieved from primaries and read (S-0009..S-0018; seven CC BY PDFs under `sources/`, two digest-only, one repository); **no source falsifies the claim as worded**; CLAIM-0006 narrowed and moved UNKNOWN → CONDITIONAL (unfalsified as of 2026-10-13, bounding sources listed); the scoped novelty statement recorded in CLAIMS |
| Cold-start test (§2, SD15) | **done — PASS** | Session 6, a fresh conversation launched with the §6 prompt verbatim and alone: recalled memory reproduced verbatim as the first HANDOFF entry (no program state found; two borderline lines flagged for Al's audit); program state re-derived in one pass from the repo — phase, every S0 criterion with its evidence, all CLAIM and SOURCE statuses with CLAIM-0006's reason, the Lean/Mathlib/Rust pins from the files on disk, the manifest schema from ADR-0001, Q3 and SD13 as the open escalations, the next action — and compared against STATE.md: agreement on every item; two precision reconciliations (the Build-status line lagged HANDOFF by one session; "24 crates" = dependencies, 25 lock packages with the workspace crate) made as the session's first task. Gate GREEN at `2b7c573` (03:45:45Z–03:46:17Z). HANDOFF Session 6; commit `[S0][INFRA] gate S0 cold-start exam: PASS at 2b7c573` |

## Blockers

- **None for S2's first action.** Gate S1's last open criterion, the cold-start
  examination, passed in Session 29 (2026-10-26). The closure takes effect on `main` at
  the merge of that session's pull request and rests on the SD15 judgments under Standing
  escalations. The paragraph below is the S1 blocker as it stood before the examination,
  kept as the record.
- *(Historical, before Session 29)* **Gate S1 remains OPEN for the genuine cold-start examination** (constitution §2,
  SD15, §4.1 and §6). No additional substantive S1 exit criterion is identified as
  unmet for the admitted prefix in `analysis/s1-post-merge-readiness/READINESS.md`.
  The first-parent status-support hold is discharged on verified main. The frozen
  S1.4 findings and older nonblocking questions/dispositions/exclusions stand.
  Canonical selection/context/locator review, missing canonical lengths and external-byte
  verification remain future scoring prerequisites, not an obligation to canonicalize
  the entire prefix or fill every flaw class before S1. This informed session cannot
  certify the examination. The readiness reconciliation (PR 8), the orientation repair
  (PR 9), the cold-start preparation (PR 10), Session 25's record (PR 11), ADR-0009 (PR 12) and
  Session 27's record (PR 13) are merged; their review records are those merged pull requests.
  Session 25, launched as the examination,
  disclosed late and is ineligible; Al decided the step-0 escalation by ADR-0009 (2026-10-26),
  which puts the disclosure first in the §6 prompt itself (Standing escalations, first bullet).
  Session 28, a fresh §6 launch inside the checkout, received the harness's Git snapshot before
  its first action and is not an examination; the examination is launched from `C:\Users\LEET`
  (Memory policy).

## Standing escalations and dispositions

- **ADR-0011 — awaiting Al (Session 34, 2026-10-27).** The pull request carrying ADR-0011 (branch
  `claude/s2-adr-0011-generator`) awaits review; Al merges. **D12, flagged and not decided:**
  (h1) whether the R4 Mnemosyne-derived seed family (`DM-FM-THM-001` and the advisory note's
  variants) enters as an `s2_template` seed and candidate golden family; (h2) its licence line
  under R1(b) — the ADR lists three options with their costs and recommends none. Three
  non-blocking clarifications are listed there for Al's eyes (D7's `lean-kernel` reference form
  for grammar instances; the FLAW-0010 (b)/(c) deferral; the gate's audit generalization after
  the narrowing). Al's ruling of 2026-10-26 (the SD15 audit bullet below) defers the seed-family
  ruling until the generator ADR's draft is in front of Al; it now is.
- **Two checkouts to tidy (Session 34; Al's call, not the executor's).** (1) `C:\Users\LEET\Syndeom`
  is on branch `claude/s2-citation-cff` at `8ae9431` (the tree `main` carries) with this
  session's SD15 disclosure appended to `PROGRAM/HANDOFF.md`, uncommitted. ADR-0008 D3 says that
  append is left as it stands and is never erased by the session, so the executor did not touch
  it; a later session's preflight will classify that checkout behind-and-dirty and go to a
  worktree again unless the append is discarded or committed by Al. (2) The worktree
  `C:\Users\LEET\Syndeom-wt-20261027` holds this session's branch and can be removed with
  `git worktree remove` once the pull request is merged.
- **Gate S1 registration deposit — DONE: published on Zenodo on 2026-10-27** (Al's rulings of
  2026-10-27, ADR-0010 *Amendments*; HANDOFF Session 32). Nothing awaited.
  - **The tag.** `gate-s1-registration`, annotated, with the message "SYNDEOM design registration
    at Gate S1 closure" (tag object `a42e7c09a275157127d89cc931d35742422c0c7f`, 2026-10-27T02:51:14Z).
    It is on PR 17's merge `2018d5e79ea23fa200e560d628bec5e91edf0956`, tree
    `3d7889def475b2b2f30d8acfd3dee31589a06a6e`, and was pushed. No GitHub release.
  - **The archive.** `Syndeom-gate-s1-registration.zip`, built with `git archive --format=zip
    --prefix=Syndeom-gate-s1-registration/` from the tag, outside the repository:
    - 13,370,992 bytes; SHA-256 `95f7d1b7d37aeb604b633b902d8f5e13ab46f5cfd5b582d8d64a299f52039cfc`;
      MD5 `f78dab5eafc6e95f595b1f38b537c9f9`;
    - 464 files: the tag's 466 less exactly the two excluded ones, each byte-identical to its blob;
    - excluded: `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md` (proprietary, R1(b)) and
      `sources/leiden-declaration-2026-06-02-zenodo-20302944.pdf` (CC BY-NC 4.0; S-0005 keeps its
      locator and SHA-256). Both stay in the repository.

    Receipt and verification: `C:\Users\LEET\Codex\Temp\syndeom-gate-s1-registration-20261027\`.
  - **The deposit.** Al published it as Zenodo record 23682781
    (`https://zenodo.org/records/23682781`) on 2026-10-27, licence `polyform-noncommercial-1.0.0`:
    - version DOI (this registration) `10.5281/zenodo.23682781`;
    - concept DOI (all versions) `10.5281/zenodo.23682780`;
    - the file as Zenodo reports it: `Syndeom-gate-s1-registration.zip`, 13,370,992 bytes,
      `md5:f78dab5eafc6e95f595b1f38b537c9f9`.

    Session 32 confirmed each value from Zenodo's API. It also downloaded the published file,
    which is byte-identical to the built archive. README § Citation carries Zenodo's citation and
    BibTeX.
  - **Every later deposit refreshes `CITATION.cff` before its tag** (Al's ruling of 2026-10-27,
    Al's rulings on PR 19's two calls and Al's ruling on the open point, Session 33; ADR-0010
    *Amendments*):
    1. Al reserves the version DOI in the Zenodo draft before the tag. The draft's publication
       date is set to the planned publication date.
    2. The commit that is tagged refreshes the file:
       - the reserved version DOI is added under `identifiers`, with a description naming the
         deposit; none is replaced;
       - `version`, `date-released` (the planned publication date) and `title` become the new
         deposit's;
       - `doi` stays the concept DOI `10.5281/zenodo.23682780`.

       Each archive therefore cites itself.
    3. The draft whose reserved DOI has been written into a tagged commit must be the draft that
       is published.
    4. The tagged file is the reference, because the archive cannot change and Zenodo's metadata
       can. After the deposit:
       - a published value that differs from the tagged file is corrected on Zenodo to match it;
       - if publication slips past the planned date, Zenodo's publication date stays at the
         file's `date-released`, and the actual publication time stays in Zenodo's own record;
       - the records commit confirms the match, instead of refreshing the file, and notes any
         correction made on Zenodo;
       - if a value cannot be brought into line on Zenodo, never retag or rebuild: that records
         commit records the discrepancy and corrects `main`'s `CITATION.cff`.

    The file has no `commit` field: it can never hold the hash of the commit that contains it.
    `version` names the tag, and the README and the Zenodo record give the full hash.
- **SD15 audit request — Gate S1 examination (Session 29, 2026-10-26) — DECIDED by Al on
  2026-10-26: all three judgments upheld; the PASS is not conditional.** Al's rulings,
  transcribed verbatim from Al's message in the Session 29 conversation (also in HANDOFF
  Session 29):

  ~~~text
  Rulings on your three judgments: I uphold all three.
  1. The memory holds no program state, including the new hazard line, which records harness behavior only.
  2. The test -f existence check is not a read of program state: the prompt names the file, and checking that it exists reveals nothing more.
  3. Recording the other attached context after the disclosure is fine: none of it is memory, and none of it names this program or a sibling.

  Order of work: keep NEXT at ADR-0010 per the phase order, with the held harvest review scheduled after it. I will rule on the Mnemosyne seed family when ADR-0010's draft is in front of me.
  ~~~

  The request as Session 29 wrote it: Al, please confirm or overrule three judgments on which
  the PASS rests (HANDOFF Session 29):
  1. no reproduced memory holds program state, including the launch-directory hazard appended
     after PR 14 and the state-adjacent hazards the disclosure names;
  2. the `test -f` existence check in the disclosure's append command is not a read of program
     state;
  3. recording the non-memory harness context (account e-mail, withheld; the tool catalogue; the
     environment block) after the disclosure rather than in it does not affect eligibility.

  If any is overruled, the examination becomes CONDITIONAL PASS. Gate S1's closure is then
  provisional until a cold re-run, and memory is changed only in a non-exam session.
- **Session 29's pull request merged as PR 15** at 2026-10-27T00:25:39Z (2026-10-26 20:25:39
  America/New_York), by Al, creating `main` `c9372b1355a6d687dec6287f908639ac670db659`. Its
  parents are `e029c80…` and the PR's head `6b3dbda5983ed3c3e41ab5d96a2863a043855a88`
  (`claude/s1-session-29-cold-start-exam`), and its tree `4caa81f0…` equals the head's. Gate
  S1's closure is in effect on `main`.
- **Merged branches deleted (Al's instruction, 2026-10-26, after Session 28):**
  `claude/s1-adr-0009-disclosure-first`, `claude/s1-session-25-exam-attempt`,
  `claude/s1-session-27-sec6-launch` and `claude/s1-session-28-launch-directory`. Their tips are
  recorded with restore commands in `deleted-branches-manifest.txt` under
  `C:\Users\LEET\Codex\Temp\syndeom-session-28-launch-dir-20261026\`. Session 29's
  `git ls-remote --heads origin` lists only `main` and the held branch. Session 28's escalation on
  those branches is closed.
- **Examination step 0 — DECIDED by Al on 2026-10-26: ADR-0009, accepted with rulings 1–4 and
  additions 5–6 (transcribed verbatim in the ADR).** Options (a) and (b) together: SD15 and the §6
  Session-N prompt are amended so that every session launched with that prompt discloses first,
  whether or not it attempts a gate test; the prompt's first sentence instructs the disclosure,
  says to append it to HANDOFF without reading that file, allows a provisional block header, and
  points to ADR-0008 for orientation; ADR-0008 D1 step 0 applies to every such session. A session
  that reads program state before disclosing (any repository file but the constitution and
  ADR-0005, a directory listing, Git history or status, or a build) cannot be an examination.
  Not retroactive: Sessions 22 and 25 stay ineligible. The constitution's preservation hash
  `89c198b0…8102` is historical; the amended SHA-256 is `110f5360…479c`. Records of past
  baselines (HANDOFF entries, ADR-0008's header, the frozen packets under `analysis/`) are left
  byte-identical (ADR-0009 D4). The escalation as Session 25 wrote it is in HANDOFF Session 25 and
  in PR 11's STATE.
- **S1.3 reproduction tool: the full narrowing of the byte checks is the next tool revision (Al,
  2026-10-26; ADR-0009 D5, ADR-0010 D6–D7).** The tool should limit its byte checks to the
  inputs it depends on, so that an amendment of a file it does not use neither stops it running
  at HEAD nor fails the gate's `first_parent_amendment` tests. It now carries four case-by-case
  exceptions:
  - the constitution skipped (ADR-0009 D5);
  - `LICENSE` skipped (ADR-0010);
  - `PROGRAM/SOURCES.md` append-only (ADR-0010);
  - the four crate manifests' `license` line (ADR-0010).

  Per Al (2026-10-26), with three exceptions added at once, the full narrowing is the right
  next tool revision rather than a fourth exception. It **lands before any S2 code, including
  ADR-0011's crate**: S2's first code commit edits `engine/Cargo.toml` and `engine/Cargo.lock`,
  which are frozen exact files and two of the 20 pinned pair-producer inputs, so the current
  check would turn the gate RED. Still exact-checked today: every other non-input file in the
  baseline, ADR-0005 and SIBLING-MAP among them (an edit to ADR-0005 failed the reproduction in
  Session 26's probe). **It is the next action (Session 34): NEXT-SINGLE-ACTION below, recorded
  as ADR-0012.**

- **First-parent status support — accepted, verified and activated on main:**
  Al accepted the exact ruling on 2026-10-20 America/New_York; appended ADR-0003 and
  `analysis/s1-first-parent-integration/OWNER-DECISION.md` record it. The original raw
  `s1_ingest-v0.1.0-63e23a952084` stays immutable and remains the logical key; its
  effective status comes only from `s1_ingest-v0.1.0-15ab865466d6`. Rust validation,
  historical preservation and the full gate passed. This discharges the status-support
  hold on main after PR 7's actual merge at 2026-10-21T03:33:35Z. Fresh merged-main
  verification and the event are in `analysis/s1-post-merge-readiness/`. Do not request
  the same ruling again. Canonical selection/context/locator review remains separate.
- **Carried questions restored from HANDOFF Session 20:** seven false-versus-undetermined
  readings (items 8, 10, 11, 14, 15, 18, 25); seven gap-versus-fatal severity readings
  (items 13, 16, 21, 24, 26, 27, 29); whether unnamed downstream results under candidate 25
  meet ADR-0003 D2.3. They were recorded as nonblocking and remain undecided. Exact IDs,
  evidence and analysis consequences: `analysis/s1-4/config/dispositions.json` and
  `analysis/s1-4/OBLIGATIONS.md`. Nothing is relabeled or newly harvested.
- **Held branch now eligible for review (Session 29):** decision 1 below deferred the held
  branch's admission review "until after Gate S1 closes". With Gate S1 closed (at the merge of
  Session 29's pull request), the review is eligible. **Order decided by Al on 2026-10-26
  (verbatim in the SD15 audit bullet above): NEXT stays at ADR-0010 per the phase order, with
  the held harvest review scheduled after it.** ADR-0010 in that ruling is the generator
  decision, renumbered ADR-0011 by ADR-0010. Nothing on the branch is approved by this
  eligibility. Its integration would add `groundtruth/` members and SOURCES entries, which the
  current preservation check refuses ("unexpected addition in preserved scope"), so in practice
  it also follows the byte-check narrowing (bullet above).
- **Later harvest held off main — DECIDED by Al on 2026-10-21 (decision 1, transcribed in
  `analysis/s1-orientation-repair/OWNER-DECISIONS.md`): preserve branch
  `claude/s1-harvest-candidates-28-30` at its original recorded commit
  `c3a3d369689d56bab3678f519904087b313daf4e`; defer its admission review and integration until
  after Gate S1 closes. This preserves the work and approves nothing: the five proposed labels
  are not approved, no future source identifiers are reserved, accepted main's harvest position
  stays at candidate 27 / `1403.4485` (candidate 28 / `1403.6317` saved), and merging or
  cherry-picking the stale branch is not authorized. The branch's manifests, producer provenance
  (`engine_commit` `8e222a4`), records and rows stay as recorded; Session 22's retrieval
  evidence (originals, saved pages, receipts, diffs, configs; 147 files, 17,824,213 bytes) is
  preserved with full SHA-256 pins under the external root
  `C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\held-evidence\`, manifest
  copied to `analysis/s1-orientation-repair/evidence/held-evidence-manifest.json`; ten abstract
  pages exist only as hash-and-size receipts (listed there). Candidate 31 remains undecided.
  Do not request this decision again. The integration conditions recorded in Session 22 remain
  the description of the later review. — Session 22's original record of the hold: branch
  `claude/s1-harvest-candidates-28-30` (commit `c3a3d36` on base `8e222a4`, pushed) holds Route A
  candidates 28–30, examined in order and all admitted — five items: arXiv 1403.6317 Theorem 1
  (FLAW-0012, false by source document); 1403.8075 Proposition 4.1 (FLAW-0004), Theorems 4.3 and
  4.4 (FLAW-0005), all undetermined; 1404.4599 Proposition 2.22 (FLAW-0005, undetermined) — produced
  by a session that had not fetched and so ran the paused harvest instead of the examination. It
  is **not mergeable as it stands**: its S-0066..S-0072 collide with main's (they become
  the seven numbers after main's last entry — 0073–0079 when this was written; **0074–0080 since
ADR-0010 registered S-0073**), so every config hash, artifact id, record, log row and STATE row must be
  regenerated at the integration commit (the originals' bytes are pinned by SHA-256 in its
  S-entries and manifests); main's Rust inventory consumers fail closed on any manifest beyond the
  35 archived representations, so a later prefix needs its own snapshot and coverage run under
  ADR-0003 D8/D9 and never rewrites the Gate S1 table or the frozen S1.4 packet. Options for Al:
  (a) integrate after Gate S1 closes, as the first later prefix under its own record (the
  executor recommends this); (b) integrate before the examination with a new snapshot and an S1.4
  re-run on 36 items; (c) discard and re-harvest later from the log. Until Al rules, main's
  harvest resume point stays candidate 28 / `1403.6317`; the branch's HARVEST-LOG rows, records
  and HANDOFF Session 22 hold the full account, including candidate 31 opened but undecided.
- **Older questions — RETAINED by Al on 2026-10-21 (decision 3):** the seven truth questions,
  seven severity questions and the unnamed-result question keep their existing dispositions
  (`analysis/s1-4/config/dispositions.json`, `analysis/s1-4/OBLIGATIONS.md`). The questions the
  held branch introduced (an eighth truth reading, two further severity readings, further unnamed
  results, its item 36's "overcome in a later paper" wording) remain pending that branch's later
  admission review. Do not request this decision again.
- **Conditional projections, not inventory (Session 23):** the accepted population remains 35
  archived representations and 32 effective logical items. If all five held items were admitted
  unchanged, the errata/retraction-only total would be 36 items; the whole-corpus projection
  would be 37 effective logical items and 40 archived representations, absent other changes.
  These are projections conditional on a review that has not happened, not measured results.

- **Q3 — RESOLVED 2026-10-13 (Session 7); the licence choice SUPERSEDED 2026-10-26 by ADR-0010**
  (PolyForm Noncommercial 1.0.0; the release boundary carried forward; earlier Apache-2.0 grants
  stand). The record as written: Al: SPDX `Apache-2.0` for SYNDEOM's original software
  and technical documentation, with the release boundary recorded in ADR-0000 Addendum 2 R1 and
  README.md § Licence. `LICENSE` committed (S-0019), `license` field set on `syndeom-manifest`.
  Nothing awaited. One note for Al, not a blocker: the Apache Software Foundation's own host
  (`www.apache.org`) refused connections from both tools throughout Session 7, so the committed
  text is the SPDX License List's rendering, cross-checked whitespace-for-whitespace against an
  independent copy (S-0019); a later session can diff it against the ASF file when the host answers.
  Session 31 (2026-10-27): the host still refused connections. The same SPDX text, fetched again
  and equal to S-0019's digest, is now `LICENSES/Apache-2.0.txt` (S-0019 addendum).
- **Advisory note ferry (Al's ruling R3 of 2026-10-13) — RESOLVED 2026-10-13 (Session 8).** In
  Session 7 the file was absent and a mid-session paste hashed to 25,652 bytes /
  `0ee1eda0…baa80` (lossy chat rendering), so the executor stopped and escalated. Al then placed
  the file at the path; Session 8 verified it byte-exact against the stated digest and committed
  it as ferried, body unedited (Advisory note bullet above; ADR-0000 Addendum 2 R3). Nothing
  awaited. Standing form for any future ferry: a path to bytes on disk, never a paste.
- **ADR-0003 flagged choices (i)–(v) — RESOLVED 2026-10-13 (Session 10; ADR-0000 Addendum 3).**
  (i), (ii), (v) ratified as written; (iii) ratified with the gate clarified (Gate S1 may close on
  a prefix of the Route A stream); (iv) overruled (NC and ND originals are `external`,
  digest-only). ADR-0001 amended for (ii) under `syndeom-manifest/2`; the S3 obligation is in the
  standing rulings. Nothing awaited.
- **SD15 audit request (Session 6) — CONFIRMED 2026-10-13 (Session 10).** The exam's PASS stands;
  Gate S0's closure is not provisional; no re-run. The memory paraphrase line was trimmed (digest
  in HANDOFF Session 10); the count-only convention is now SD15 by ADR-0005. Nothing awaited.
- **Q14 / Q15 — resolved in S0.** The sweep was ferried and placed (Session 3); no retrieval
  failed outright in S0.1 (the IAS page, HTTP 403, is corroborative only). Nothing awaited.
- Human-referee participants are Al's decision whenever S4.3 arrives (SD13); any output about a
  specific real manuscript beyond SD3 is escalated.

## Preserved harvest resume procedure (not this session's priority)

**S1.1 — the tenth Route A harvest session, resuming from the log's last candidate** (ADR-0003
D7, D8 as amended: a prefix is enough). Procedure, executable cold:
(a) read `groundtruth/HARVEST-LOG.md`: run 1's query, window 2023-01-01..2026-09-30, ascending
arXiv id, **last candidate processed `1403.4485` (candidate 27 of 2,472)**; the next candidate is
`1403.6317` (v3 of 2024-10-15, math.SG, "Comparison of spectral invariants in Lagrangian and
Hamiltonian Floer theory", comment "We removed the wrong theorem from the first version, which was
brought to our attention by R. Leclercq" — identify the removed theorem from the diff, and read
v2's comment and date first: if v2 removed it, the correction may lie outside the window), then
`1403.8075` (v11 of 2026-01-09, math.GM, "Algorithm, probability, and prime numbers", a withdrawal
whose comment reads "Withdrawn due to a logical error in Section 4. In Proposition 4.1, the
argument incorrectly treats primality verification as equivalent to checking leading digits in
multiple base representations, and the subsequent independence assumption for fp(1),...,fp(n) is
unjustified. This invalidates the derivation of Theorems 4.3-4.4" — the helper fails on a withdrawn
version's missing e-print, decide from the abstract pages and v10 as for candidate 18; math.GM is
general mathematics, so read D2.1 and D2.6 with care), then `1404.4599` (v5 of 2024-01-12,
math.CO, comment beginning "The construction of finite n-acyclic groupoids in Section 2.4 is
flawed and I know of no direct repair: completion turns out to be incompatible with restriction to
proper subsets of the generator set, so that the induction towards Proposition 2.22 does not
stabilise as claimed. This problem has been overcome in arxiv:1806.08664 …"). A cumulative revision
comment can carry correction language from a version outside the window (candidate 20): read every
version's comment on its own abstract page before deciding D8. A comment's theorem number may
follow the corrected version's numbering (candidate 23): resolve the number in both versions
before locating. A remark or erratum in the corrected version that names further results is part
of the status document and yields further items (candidate 23, three; candidate 25, four from one
published erratum naming results at several depths of one dependency chain); links the erratum
does not name are not items (D2.3), and the records say which.
If a correcting version is itself a withdrawal (no e-print), the helper prints the abstract data
and fails on the source fetch (candidates 11, 13 and 18): decide from the abstract pages and the
original (candidate 18 was admitted that way, its comment describing and locating the error), or
add a `--no-sources` fallback first; a page-and-line reference is resolved against the arXiv PDF
(`pdftotext -layout`, as for candidate 18); the helper takes the correcting version as a bare
number (`4`, not `v4`);
(b) regenerate the candidate list — `python -I -X utf8 lab/route_a_candidates.py 2023-01-01
2026-09-30 <scratchpad>/route-a` (pages are cached under that directory only for the session
that fetched them; a fresh run re-fetches 57 ranges in about 30 minutes) — and confirm its
digest against the log's run record or record the new digest as run 2 with the difference stated
(arXiv adds and revises papers; the examined prefix is unaffected because ids are assigned at
submission and the finder sorts by id);
(c) examine candidates from `1403.6317` onward **in ascending id, skipping none**, each with
`python -I -X utf8 lab/route_a_examine.py <id> <correcting version> <scratchpad>/exam/<id>`
(both versions' abstract pages for licence and submission history, both e-prints, the TeX diff,
the AI-assistance scan) against ADR-0003 D2's seven criteria; log every verdict with its reason;
(d) for each admitted item, in this order: the S-entries for original and notice in SOURCES.md
(licence per D5 as amended: CC0 / CC BY / CC BY-SA / public domain → `committed`, NC, ND,
arXiv non-exclusive or assumed → `external`); the per-item config (the thirty-one Session-11/13/14/15/16/17/18/19/20 configs
are the pattern, including the null-truth one and the two-items-from-one-notice cases: ADR-0003 D4 fields, `statement_status` `null` + `"undetermined"` when the notice
leaves the truth open); then from `engine/`: `cargo run -q -p syndeom-ingest -- --config
<item.json> --bytes <original bytes> --root .. --engine-commit $(git rev-parse HEAD)`; then
`RECORD.md` into the item directory (the thirty-one Session-11/13/14/15/16/17/18/19/20 records are the pattern); the `.gz`
originals are binary in `.gitattributes` and the register checker;
(e) stop at the session's budget before examining the next candidate; update the log's counts
and last-candidate line, this file's Gate S1 table (S1.1, S1.3 rows), and set NEXT to the next
prefix. Grey zones are `escalated` and listed under standing escalations; nothing ambiguous is
ingested. Run the gate detached with a log before the commit (expect `GATE: GREEN`); commit
`[S1][ADR-0003] harvest Route A prefix through arXiv id <last id processed>: <n> admitted`.


## NEXT-SINGLE-ACTION

**S2 ordering step 2 — the full narrowing of the S1.3 effective-inventory reproduction tool's
byte checks, decided and recorded as ADR-0012.** It lands before any S2 code, including
ADR-0011's crate (ADR-0010 D7; Al, 2026-10-26). The session decides and implements the tool
revision only: it writes no generator code and does not integrate the held branch.

What it does, under Al's direction of 2026-10-26 (Standing escalations, "S1.3 reproduction tool"):
1. In `engine/ingest/examples/support/effective.rs`, limit `preserve()` and the pair-input check
   (`pair_input_unchanged()`) to the inputs the reproduction depends on, so that an amendment of
   a file it does not use — a constitution amendment, an ADR, SIBLING-MAP, `scripts/check.sh`,
   `lean/Audit.lean` — neither stops it running at HEAD nor fails the gate's
   `first_parent_amendment` tests, while any byte change to an input it depends on still fails.
   Read the input set from the tool, its tests and its packets (SD7), never from a summary;
   `analysis/s1-first-parent-integration/preservation-inputs.json` is compiled into the
   reproduction and checked against itself.
2. Decide in ADR-0012 how the narrowed check treats `engine/Cargo.toml` and `engine/Cargo.lock`:
   S2's first code commit edits both, and both are two of the 20 pinned pair-producer inputs
   (replay from HEAD already ended at ADR-0010; the replay revision range is recorded in this
   file). The current `license`-line exception and the three other case-by-case exceptions are
   replaced by the narrowed rule or kept, as ADR-0012 argues.
3. Keep the `relicense_exceptions` (6) and `first_parent_amendment` (15) suites green and add
   tests that show the new boundary: an edit outside the input set passes; an edit inside it
   fails.
4. Record, as ADR-0009 D5 and ADR-0010 D6 did: the tool's source SHA-256 before and after; that
   the retained `inventory.json` is reproduced byte-identical at `3cd5558` with that revision's
   tool; and exactly which `preservation` fields of the output change at HEAD, with everything
   else unchanged.
5. Write ADR-0012 (context, options, decision, the tests, the measurements); update this file's
   "S1.3 reproduction tool" escalation, the ADR-numbers line and this NEXT.

Bounds: the frozen packets, the registers, the constitution, the licence, the tag and the
accepted corpus are untouched; the held branch stays held; no generator code. If ADR-0011's pull
request is still unmerged when this action is picked up, this step may proceed — it does not
depend on ADR-0011 — and the ADR-0011 rulings stay awaited (Standing escalations). Open the
session per ADR-0008 D1, preceded by the SD15 disclosure if it is launched with the §6 prompt;
note the two checkouts under Standing escalations. Commit `[S2][ADR-0012] narrow the S1.3
reproduction's byte checks to its inputs` on a branch, gate the committed tree, open a pull
request; Al merges. Then step 3, the held harvest's admission review (Al's order), and only then
step 4, ADR-0011's crate.

The harvest itself stays paused through candidate 27 / `1403.4485`, with candidate 28 /
`1403.6317` saved. The seven truth questions, the seven severity questions and the
unnamed-result question keep their existing dispositions. Missing canonical lengths and
external-byte verification remain prerequisites for scoring real items (S4), not for S2.
