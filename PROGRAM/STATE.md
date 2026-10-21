# STATE

- **Updated:** 2026-10-21 UTC (Session 22) — a session launched with the §6 resume prompt ran
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
- **Phase:** S1 — GROUND-TRUTH CORPUS (real proofs as unit tests)
- **Gate:** **S1 OPEN — ready for the separate cold-start examination after this
  reconciliation is on main.** S1.1 has evidence for the admitted unskipped prefix,
  not an exhausted harvest (ADR-0003 D8). PR 4 admitted one positive statement-fidelity
  pair; PR 5 merged the frozen S1.4 report; PR 6 archived the witness/proposal; the
  owner accepted its exact ruling and PR 7 activated verified integration on main.
  The criterion-by-criterion readiness assessment above carries all qualifications.
  This is an informed session and cannot perform or certify the §2/SD15 examination.
  Harvest resume at candidate 28 / `1403.6317` remains saved for a later harvest.
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
  MNEMOSYNE material in it is proprietary and outside the Apache-2.0 release; the program is named
  with its owner's authorization, Al, 2026-10-13). **SD11 banner: advisory, not evidence** —
  nothing enters any register from it; its two "recorded counterexamples" are not in the Mnemosyne
  repository (its header says so); its §6/§7 finding is Mnemosyne change control, not SYNDEOM's.
  Its only standing effect here is R4 (candidate S2 seed family, standing rulings below).
- **Default branch:** `main` — discovered from the clone of the empty remote and the REST API
  (ADR-0000 D3); confirmed by Al (Q2). Remote: `origin` = `https://github.com/Quigles1337/Syndeom`
  (**public**; created by Al 2026-10-12; Actions enabled; **`LICENSE` = Apache-2.0 since Session 7**
  — Q3 resolved by Al 2026-10-13, ADR-0000 Addendum 2 R1; the release boundary is stated in
  README.md § Licence; text provenance S-0019).
  **ORIENT note (SD7):** `git symbolic-ref refs/remotes/origin/HEAD` resolves to `main`; if it ever
  fails, fallback `git remote show origin`, repair `git remote set-head origin -a`.
  **Fetch first (Session 22, 2026-10-21):** run `git fetch origin` before §4.1's `git status` /
  `git log`, then compare `main` with `origin/main` (`git status -sb`; `git log main..origin/main`;
  `gh pr list --state merged`): a working copy's remote ref is a claim until fetched, and on
  2026-10-21 a §6-launched session reconstructed a 24-commit-stale state through §4.1 without
  noticing and ran the superseded harvest NEXT. A diverged local commit goes to a branch, never a
  force-push.
- **S1.3 orientation verification (2026-10-20):** existing Lean/Rust code passed the gate,
  with documentation reconciliation in progress; 29 existing tests passed, axioms `[propext]`.
  The new crate was enabled only after this gate. Its focused producer checks now pass
  37 tests (24 manifest + 5 ingest + 8 derivation), fmt and clippy. Logs: `analysis/s1-3/`.
  Real generation/reproduction passed from the producer commit. The final staged artifact/evidence
  tree passed the full gate; `analysis/s1-3/gate-final.txt` records exit 0 and GATE: GREEN.
- **Current build verification:** clean merged main
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
  **amended twice: ADR-0004 (2026-10-13, Session 7) — §2 S3.2, fooling length as equivalence to
  chance with a design-derived N_min; ADR-0005 (2026-10-13, Session 10) — SD15, count-only
  disclosure of unrelated memory index lines in a cold-start exam.** Rulings of record: the
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
  ADR-0007 first positive-pair procedure and candidate — written 2026-10-20 (S1.2).
  **ADR-0008 is the next free number.**
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

## Gate S1 exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| S1.1 Errata and retractions: per item the original, the notice, the flaw's location, its FLAWS.md class argued in writing; derivation `documented-by-source`; licence per SD16 | **evidenced for the admitted unskipped prefix; harvest not exhausted — prefix through arXiv id `1403.4485` (candidates 1–27 of 2,472; Sessions 11, 13, 14, 15, 16, 17, 18, 19, 20): 27 examined, 21 admitted yielding 31 items, 4 rejected (two D2.1 — mis-stated quoted theorems with no proof offered; two D8 — one correcting version after the window, one cumulative comment whose only correction predates it), 2 found-but-unclassifiable (withdrawal stubs naming no describable error, one of them superseded by a paper that is silent on the mistake); `groundtruth/HARVEST-LOG.md` is the record of every candidate; harvest resume remains candidate 28** | `PROGRAM/DECISIONS/ADR-0003.md` (Session 9; Al's rulings on (i)–(v) recorded Session 10): D1 routes (arXiv comments, Crossref update relations, zbMATH index; AI-generated excluded), D2 admission, D3 class mapping (undetermined truth admitted under `syndeom-manifest/2`), D4 record (`manifest.json`, bytes, `RECORD.md`, S-entries, `groundtruth/HARVEST-LOG.md`), D5 licences (NC/ND `external`), D6 `raw/1` parent + derived artifact + the producer, D7 log, D8 window, no skipping, prefix gate, D9 the S1.4 table; routes live-checked 2026-10-13. **Producer:** `engine/ingest/` → `syndeom-ingest` (`s1_ingest` 0.1.0, Session 10): writes a `/2` manifest from a config and bytes, refuses on any ADR-0001 D4 or ADR-0003 shape violation (exit 2) and on a same-id byte conflict (exit 3); goldens over fixed bytes (fixture `915d3ed6…1390`, 72 bytes) and two fixed configs — boolean truth (`s1_ingest-v0.1.0-d2134e7fd2c3`, manifest `1f2cf7b9…1f63`) and the null case (`s1_ingest-v0.1.0-4f5d4e93adaa`, manifest `451bde2d…8340`) — config hashes cross-checked in Python. Orientation reconciliation (2026-10-20): the log and 31 admitted items exist; the former empty-slots sentence was stale. |
| S1.2 Informal–formal pairs with the mathlib commit pinned (positive statement-fidelity pairs; drifted pairs are synthetic and live in `corpora/`, SD12) | **one positive pair admitted by owner merge PR 4** | ADR-0007; both pair RECORD post-merge dispositions; actual producer/both-file/lineage/type/axiom/replay evidence in `analysis/s1-2/implementation/`; merge metadata in `analysis/s1-4/config/merge-evidence.json`. No separate GitHub reviewer approval, no PROVEN promotion; statement-only informal selection, zero steps. |
| S1.3 Manifests for real items (`origin: ground-truth`; `raw/1` parent plus derived canonical artifact per ADR-0001 D4; nothing synthetic labeled real) | **35 archived representations (33 raw + two canonical), 32 effective logical items; 30 without canonical lengths. First-parent status support discharged on verified main by PR 7; fresh Rust reproduction and preservation evidence in `analysis/s1-post-merge-readiness/`.** | `s1_ingest-v0.1.0-63e23a952084/` (arXiv 0706.0357 v70, Lemma 1.1; FLAW-0012 + FLAW-0005; old raw retained; stable logical key resolves to accepted false/counterexample representation `s1_ingest-v0.1.0-15ab865466d6`; canonical selection/context/locator review still required; original committed, CC BY 4.0); `s1_ingest-v0.1.0-744652f4b95f/` (arXiv 0707.3799 v2, Theorem 1 b); FLAW-0011 + FLAW-0005; statement true; `external`); `s1_ingest-v0.1.0-5ba6c0f07557/` (arXiv 0902.0680 v1, the dense-subspace lemma; FLAW-0005 gap; **statement undetermined — `value: null`, the first use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-a8ac73896464/` (arXiv 0908.3213 v4, Theorem 4.4; FLAW-0001 propagated + local; statement false; `external`); `s1_ingest-v0.1.0-be3b17e73425/` and `s1_ingest-v0.1.0-72415457dce4/` (arXiv 1005.2398 v2, Proposition 3.3.5 and the lemma following it; FLAW-0012 + FLAW-0005 each; statements false — the proposition by the author's counterexample; `external`); `s1_ingest-v0.1.0-e0c199d3c60a/` (arXiv 1009.3998 v4, Lemma 13.2; FLAW-0011 + FLAW-0001 local; statement true; `external`); `s1_ingest-v0.1.0-448d7aefa486/` (arXiv 1010.2391 v9, Lemma A.3; FLAW-0012 + FLAW-0005; statement false; `external`); `s1_ingest-v0.1.0-1c4c9a7747d0/` and `s1_ingest-v0.1.0-2d93a266558a/` (arXiv 1107.0863 v3, Lemma 12 and Theorem 1; FLAW-0012 each; the lemma false by the authors' counterexample figure, the theorem by source document; `external`); `s1_ingest-v0.1.0-e3cbdfa7ad1c/` (arXiv 1112.6107 v2, Theorem 1; FLAW-0012; statement false by source document — the misstated last clause, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-22ae000cb879/` and `s1_ingest-v0.1.0-fd2bfef40d31/` (arXiv 1201.3327 v2, the former Lemma 5.8 and the former Theorem 5.9; FLAW-0012 fatal and FLAW-0005 gap; the lemma false by source document, **the theorem undetermined — `value: null`, the second use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-dc1e44f39bb9/` (arXiv 1202.5531 v3, Theorem 6; FLAW-0012; statement false by source document, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-68ed1abfecb4/` and `s1_ingest-v0.1.0-a4822a4be06e/` (arXiv 1206.1893 v6, Propositions 34 and 35; FLAW-0012 + FLAW-0005, and FLAW-0005 gap; the first false by source document, **the second undetermined — `value: null`, the third use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-da3a4784c2be/` (arXiv 1206.2228 v1, Theorem 1; FLAW-0012; statement false by source document; **`status.kind` `retraction`, the first**; `external`); `s1_ingest-v0.1.0-685b874cf579/` (arXiv 1206.5522 v5, Proposition 5.3; FLAW-0012 + FLAW-0005; statement false by source document — the hypotheses corrected, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-00f28e252823/` and `s1_ingest-v0.1.0-a93badadec27/` (arXiv 1209.2022 v2, Proposition 3.1 and Theorem 3.2; FLAW-0012, and FLAW-0011 + FLAW-0005 gap; the proposition false by the printed counterexample, the theorem true by source document; `external`); `s1_ingest-v0.1.0-cafb56cb27fb/` (arXiv 1209.4040 v3, Theorem 4.5; FLAW-0005 gap; **undetermined — `value: null`, the fourth use of ruling (ii)**; `external`); `s1_ingest-v0.1.0-45c3ee8319d5/`, `s1_ingest-v0.1.0-486ef22f632b/` and `s1_ingest-v0.1.0-dc0e0b0d5b87/` (arXiv 1210.6709 v1, Theorems 1.6, 2.2 and 2.1; FLAW-0005 fatal, fatal and gap; **all three undetermined — `value: null`, the fifth to seventh uses of ruling (ii)**; `external`); `s1_ingest-v0.1.0-40e6c0da8821/` (arXiv 1303.3650 v2, Theorem 4.6; FLAW-0012 + FLAW-0005; statement false by source document, `undetermined` named as the alternative; `external`); `s1_ingest-v0.1.0-003a35498654/`, `s1_ingest-v0.1.0-0833f61c748c/`, `s1_ingest-v0.1.0-82b4e66d0016/` and `s1_ingest-v0.1.0-315546e87259/` (arXiv 1304.5623 v4, Proposition 3.5, Theorem 3.6 (4), Theorem 4.3 and Theorem 5.3; FLAW-0004 gap, FLAW-0005 gap, FLAW-0012 + FLAW-0004 + FLAW-0004, FLAW-0005 gap; **three undetermined — `value: null`, the eighth to tenth uses of ruling (ii)**, Theorem 4.3 false by the erratum's example; `external`); `s1_ingest-v0.1.0-f634c259799f/` (arXiv 1312.1045 v1, Theorem A.1; FLAW-0011 + FLAW-0005 gap; statement true by source document; `external`); `s1_ingest-v0.1.0-6ae03b18a1bd/` (arXiv 1403.4485 v3, Proposition 5.1; FLAW-0001 propagated + local; statement false by source document; `external`) — all under `groundtruth/`, each with `manifest.json` (`syndeom-manifest/2`, `engine_commit` of the producer's commit, self-checked by the producer) and `RECORD.md` (the D3 class argument); S-0024..S-0065 |
| S1.4 Coverage and power: per flaw class, how many real items at what proof lengths; zero-coverage classes named as a finding | **merged in PR 5; complete for pinned prefix, qualified, reproducible and frozen** | `analysis/s1-4/report/report.md`, inventory/coverage/provenance JSON, frozen bins, two fresh byte-identical runs. Five observed classes, nine zero, 32 historical roots / 17 unflagged roots. Every real-item cell expected underpowered; no numerical power or detector run. `analysis/s1-4/OBLIGATIONS.md` is the historical ledger. The later effective inventory records 35 representations / 32 logical items / 18 unflagged; current requirements and limits are mapped in `analysis/s1-post-merge-readiness/READINESS.md`. |
| Licences recorded for anything touched (SD16) | **recorded; existing storage choices preserved** | Per-source completeness matrix and source records in `analysis/s1-4/`; SOURCES historical prefix unchanged. No new external source ingested or rights inferred. First-parent status support is discharged on verified main under the accepted scoped ruling. External bytes retain historical evidence and must be retrieved/reverified before scoring; the older dispositions and exclusions remain unchanged. |
| Repo green under the gate | **standing** | Build status above |
| Cold-start test (§2, SD15) | **OPEN — next examination** | Separate genuinely fresh session launched with the constitution's §6 Session-N resume prompt verbatim and alone after this reconciliation is on main. `analysis/s1-post-merge-readiness/READINESS.md` records readiness, not an examination result. |

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

- **Gate S1 remains OPEN for the genuine cold-start examination** (constitution §2,
  SD15, §4.1 and §6). No additional substantive S1 exit criterion is identified as
  unmet for the admitted prefix in `analysis/s1-post-merge-readiness/READINESS.md`.
  The first-parent status-support hold is discharged on verified main. The frozen
  S1.4 findings and older nonblocking questions/dispositions/exclusions stand.
  Canonical selection/context/locator review, missing canonical lengths and external-byte
  verification remain future scoring prerequisites, not an obligation to canonicalize
  the entire prefix or fill every flaw class before S1. This informed session cannot
  certify the examination. Temporary reconciliation review status lives in PR metadata.

## Standing escalations and dispositions

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
- **Later harvest held off main — Al's decision (Session 22, 2026-10-21):** branch
  `claude/s1-harvest-candidates-28-30` (commit `c3a3d36` on base `8e222a4`, pushed) holds Route A
  candidates 28–30, examined in order and all admitted — five items: arXiv 1403.6317 Theorem 1
  (FLAW-0012, false by source document); 1403.8075 Proposition 4.1 (FLAW-0004), Theorems 4.3 and
  4.4 (FLAW-0005), all undetermined; 1404.4599 Proposition 2.22 (FLAW-0005, undetermined) — produced
  by a session that had not fetched and so ran the paused harvest instead of the examination. It
  is **not mergeable as it stands**: its S-0066..S-0072 collide with main's (they become
  the seven numbers after S-0072, 0073–0079), so every config hash, artifact id, record, log row and STATE row must be
  regenerated at the integration commit (the originals' bytes are pinned by SHA-256 in its
  S-entries and manifests); main's Rust inventory consumers fail closed on any manifest beyond the
  35 archived representations, so a later prefix needs its own snapshot and coverage run under
  ADR-0003 D8/D9 and never rewrites the Gate S1 table or the frozen S1.4 packet. Options for Al:
  (a) integrate after Gate S1 closes, as the first later prefix under its own record (the
  executor recommends this); (b) integrate before the examination with a new snapshot and an S1.4
  re-run on 36 items; (c) discard and re-harvest later from the log. Until Al rules, main's
  harvest resume point stays candidate 28 / `1403.6317`; the branch's HARVEST-LOG rows, records
  and HANDOFF Session 22 hold the full account, including candidate 31 opened but undecided.

- **Q3 — RESOLVED 2026-10-13 (Session 7).** Al: SPDX `Apache-2.0` for SYNDEOM's original software
  and technical documentation, with the release boundary recorded in ADR-0000 Addendum 2 R1 and
  README.md § Licence. `LICENSE` committed (S-0019), `license` field set on `syndeom-manifest`.
  Nothing awaited. One note for Al, not a blocker: the Apache Software Foundation's own host
  (`www.apache.org`) refused connections from both tools throughout Session 7, so the committed
  text is the SPDX License List's rendering, cross-checked whitespace-for-whitespace against an
  independent copy (S-0019); a later session can diff it against the ASF file when the host answers.
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

**Once this reconciliation is on main, a separate genuinely fresh session should
attempt Gate S1's cold-start examination under §2/SD15 using the constitution's
§6 Session-N resume prompt verbatim and alone.** Launch without a state-bearing
preamble, handoff or extra task text; a full-history continuation or subagent is
not cold. The session must honor SD15 disclosure, reconstruct from the repo via
§4.1, run the required checks and determine whether every criterion supports closure.
This readiness assessment is not a PASS; Gate S1 remains OPEN until that examination.
The examination session runs `git fetch origin` before §4.1's commands and reconstructs from
the fetched `origin/main`, recording its SHA: Session 22 showed that an unfetched working copy
reproduces a stale state and passes §4.1 silently.

The seven truth/seven severity/unnamed-result questions keep their existing
dispositions and exclusions. Missing lengths and external-byte verification remain
scoring prerequisites; canonical production needs selection, context and locator
review. The frozen S1.4 packet and bins remain historical. Harvest stays paused
through candidate 27 / `1403.4485`, with candidate 28 / `1403.6317` saved for later; a later
harvest of candidates 28–30 exists on branch `claude/s1-harvest-candidates-28-30`, held for Al's
disposition (dispositions above) and not part of the state under examination.
