# Orientation repair and held-harvest preservation (Session 23)

**Recorded:** 2026-10-21 UTC. **Event:** `s1-orientation-repair-2026-10-21`. An informed working
session on branch `claude/s1-orientation-repair` from `main`
`d8c0d9cbcef0af5249cbf439808b74d8828bd69d` (tree `f98e7832b3f8c4371e7fae7c6deeffff737f3c84`),
fetched and verified at 2026-10-21T23:30Z with no local-only or remote-only commits. Gate S1
remains **OPEN**; this event conducts no cold-start examination and does not close S1. Session 22
(HANDOFF) remains ineligible as the examination, and its record stands as an unsuccessful attempt.

The owner's three decisions of 2026-10-21 are transcribed verbatim in
[OWNER-DECISIONS.md](OWNER-DECISIONS.md); decision 2 is implemented as
[ADR-0008](../../PROGRAM/DECISIONS/ADR-0008.md). They are recorded and not to be requested again.

## A. Held evidence preserved

Session 22's retrieval evidence for Route A candidates 28–30 (the five items of the held branch)
and for candidate 31 (opened, undecided) lived only in that session's scratchpad. It is now
copied, with every file's SHA-256 computed before the copy and verified after, to the durable
external root

```
C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\held-evidence\scratchpad-b5392462-cdc6-4d18-b175-6ca1c8052991\
```

147 files, 17,824,213 bytes. The manifest (`manifest.json` beside that directory; copy
[evidence/held-evidence-manifest.json](evidence/held-evidence-manifest.json), SHA-256
`d3ea78799dd3e4bbe660368660c17473065768e5ba0b27c7b8c34c89660b87ce`, 30,261 bytes) pins each
file and associates every retrieval receipt the examination helper printed (URL, UTC access time,
size, SHA-256) with the saved bytes, where they exist. The originals keep their existing
disposition: arXiv non-exclusive licence, `external`, never committed to this public repository.

| Retrieved bytes (saved) | Size | SHA-256 |
|---|---:|---|
| arXiv:1403.6317 v1 e-print (the held item 32's original; tar) | 126,877 | `97b28cf24aa4cc4bb6e810ceb12aebad481df0840cb5a3389b5eca1c2e599bce` |
| arXiv:1403.6317 v3 e-print (its correcting version) | 125,994 | `3e95b88f70673c6d42813c7e9f43a62a272060078a0ef01b726e06df15c1f2ed` |
| arXiv:1403.6317 v2 abstract page (withdrawal stub) | 39,708 | `81d0d559b9f39506e35193889d29923874cc69ff6fa309d00128d43a66f8f907` |
| arXiv:1403.8075 v9 e-print (the held items 33–35's original; tar) | 136,883 | `c171e3381684cb051dcf77333de7e51d75e08ca92580b9dc14d24d73e6999270` |
| arXiv:1403.8075 v10 abstract page (withdrawal stub) | 39,877 | `b01ae085d31ce38869d53d8f6fab91ff5878b8f7865c6960f9af87148478d42f` |
| arXiv:1404.4599 v4 e-print (the held item 36's original; gzip) | 48,382 | `2e326a3380ac4068d70de4685e3b306eb693677716fa6c920d8deb747f923cd2` |
| arXiv:1806.08664 v6 e-print (the later paper the item-36 notice names; gzip) | 59,896 | `02f6c59b3e992c6c2f6831cab991e1d7b23ae84299b4a9b3d3101b8d38247ac7` |
| arXiv:1406.7174 v2 e-print (candidate 31, original; gzip) | 114,892 | `df580e82d3a93aab1daa809d46f8e49898905f3608f28543bc35eb4e783ed921` |
| arXiv:1406.7174 v3 e-print (candidate 31, correcting version; gzip) | 115,786 | `f8f5aefcb24e37e0051705dc4a87f5d26d823376a60595a97cca912357539c62` |

Every saved e-print matches the hash and size its receipt recorded
(`retrieval_receipts[*].saved_sha256_matches` in the manifest). Also preserved: the helper
outputs (`exam/*/examine.txt`, `abs-only.txt`), the TeX diffs (1403.6317 v1→v3, 645 lines;
1406.7174 v2→v3, 72 lines), the extracted sources, the five item configs (`items/item-32..36.json`,
whose JCS hashes gave the branch's artifact ids), the regenerated candidate list
(`route-a/candidates.tsv`, SHA-256 `e164c0b1693c7a6cdf242dcca853db28517f8c210d17af7ba82d2446dd752d48`,
identical to run 1) with the 73 cached API pages and finder state, Session 22's gate logs and
drafts.

**Specifically unavailable** — receipts exist (size and SHA-256 printed at retrieval and recorded
in the helper outputs or, for the first nine, also in the held branch's source entries), but the
helper never saved the bytes, and no newly downloaded copy is substituted for them:

| Abstract page | Size | SHA-256 (receipt) |
|---|---:|---|
| arXiv:1403.6317 v1 | 39,781 | `1e5d5c9673d90d6a20c07fa2481bd50349ad6d2f1bad07a0c290e1c8e13628fd` |
| arXiv:1403.6317 v3 | 38,733 | `5b32eedd88fe1b0a4972c956bbc5742f210c61e84a066065b387301eaa6da531` |
| arXiv:1403.8075 v9 | 41,098 | `1cd0cb15c6e65b322aed751bda1ee6bb96e80628085a3b2d922e327be1635d77` |
| arXiv:1403.8075 v11 | 40,895 | `5726f969962deed8b5d8f840cda7fe80e497486521c1b698b93c64cb874f397b` |
| arXiv:1404.4599 v4 | 41,567 | `84cb82628e4efab4b00b65555e29be87738382e6117c24d317903f40afda559d` |
| arXiv:1404.4599 v5 | 40,117 | `4f36fad483db20aec042a381cf0fa315f28c0e2979e0d9b7d8b23bc9464cd8ac` |
| arXiv:1406.7174 v2 | 44,368 | `4b30657699c162e320ba20ce3a5cc029e4fe29d8644f0e84fc254d90ca8fdcc0` |
| arXiv:1406.7174 v3 | 44,365 | `abc7321d4efed96de76d2c79dc26e006ec8d908329292b31477617d14129e9e0` |
| arXiv:1806.08664 (current version page, accessed 21:53:27Z) | 40,695 | `ffccecdf45d9751e7885081aa8d0c10ad341b2109646fdaebc90ffe521fac30a` |

The 1806.08664 page receipt and the second fetches of the 1403.6317 v1/v3, 1403.8075 v9 and
1404.4599 v4/v5 pages (journal fields; each byte-identical to the first fetch) exist only in the
session's console output as transcribed into the held branch's source entries; no file holds
them.

**Disposition (owner decision 1).** Branch `claude/s1-harvest-candidates-28-30` is preserved at
its original recorded commit `c3a3d369689d56bab3678f519904087b313daf4e`; its manifests
(`engine_commit` `8e222a49a64273c9cefc35bca5ae15a0a05fb9ee`), producer provenance, records, source
entries and harvest-log rows are untouched. The five labels are not approved; no future source
identifiers are reserved; accepted `main`'s harvest position stays at candidate 27 /
`1403.4485`; nothing is merged or cherry-picked. Admission review and integration come after Gate
S1 closes. Candidate 31 (`1406.7174`) remains undecided; its partial-inspection files are pinned
above.

## B. The accepted orientation addendum

[ADR-0008](../../PROGRAM/DECISIONS/ADR-0008.md) holds the complete amended §4.1 procedure (D1),
the preflight's contract (D2), the examination accounting (D3), the verification (D4) and what
stays unchanged (D5). The constitution's bytes are unchanged (SHA-256
`89c198b0f1ca7b44f96c9d13816409f65acfb0b8eb3e119cf37003ff15d48102`, 37,609 bytes, the value
pinned in `analysis/s1-first-parent-integration/preservation-inputs.json`). SD15, ADR-0005 and
the §6 launch prompt are unchanged. STATE links the ADR from its orientation bullet and its NEXT.

Implementation: [`scripts/orient_preflight.py`](../../scripts/orient_preflight.py) (orient /
recheck; JSON receipt `syndeom-orient-preflight/1`; exit 0/2/3/4/1 as the ADR tabulates) and
[`scripts/orient_preflight_cases.py`](../../scripts/orient_preflight_cases.py) (offline cases).
Both are standard-library Python with no result authority; `scripts/check.sh` does not call them,
so the historical build gate gains no network dependency.

## C. Failure-mode verification (actual run)

```
python -I -X utf8 scripts/orient_preflight_cases.py --work-dir C:\Users\LEET\Codex\Temp\syndeom-orientation-repair-20261021\orientation-cases
```

2026-10-21, git 2.49.0.windows.1, Python 3.11.9, exit 0, **all cases passed**. Every command and
outcome is in [evidence/orient-preflight-cases.json](evidence/orient-preflight-cases.json)
(SHA-256 `b9b638a8467f4ac152aa0fbbb76488427098fec7c2a059a2f401f5d42e44450e`); the temporary
repositories remain under the external root.

| Case | Checks | Result |
|---|---:|---|
| 1 stale tracking branch appears synchronized (`git status -sb` shows no behind count; cached `origin/main` equals HEAD while the remote tip differs) | 3 | pass |
| 2–3 fetch exposes the newer snapshot; the stale checkout is not treated as current (exit 3, HEAD unchanged, remote-only commit listed, receipt complete) | 7 | pass |
| 4 clean behind-only synchronization fast-forwards (exit 0, HEAD = fetched tip, clean) | 4 | pass |
| 5 ahead/diverged commit, uncommitted edit and untracked file preserved under `--sync` (exit 3, nothing changed); `--worktree` yields an isolated checkout at the fetched tip | 12 | pass |
| 6 fetch failure stops (exit 2, no snapshot recorded, HEAD unchanged) | 4 | pass |
| 7 target movement detected before publication (`recheck` exit 4 with the new tip and commits, nothing published; exit 0 against the current tip) | 6 | pass |
| 8 an examination's HANDOFF append counts as unrelated without an allowance (exit 3) and as the declared change with `--allow-dirty` (exit 0); the text survives | 4 | pass |

An offline fixture establishes nothing about the live remote. The live receipts of this session's
own checkout are separate: [evidence/live-orient-1-plain.json](evidence/live-orient-1-plain.json)
(exit 3: HEAD synchronized at the fetched `origin/main`
`d8c0d9cbcef0af5249cbf439808b74d8828bd69d`, tree `f98e7832b3f8c4371e7fae7c6deeffff737f3c84`,
but the two new scripts were unrelated untracked changes; nothing was changed) and
[evidence/live-orient-2-allowed-new-scripts.json](evidence/live-orient-2-allowed-new-scripts.json)
(exit 0, `SNAPSHOT-ESTABLISHED`, the same SHAs). The pre-publication recheck of the repair branch
is recorded in the pull request.

## D. Preservation, reproduction and gate

- **Accepted corpus unchanged:** 35 archived representations, 32 effective logical items; every
  corpus object, source record, accepted decision, frozen S1.4 output and bin, witness and
  integration packet, receipt, dependency pin and all 20 frozen pair inputs are byte-identical to
  the accepted baseline (verified by the unchanged Rust consumer below). No held item is merged,
  no id regenerated, no source number allocated, no harvest log advanced, no label changed.
- **Conditional projections, not inventory:** if all five held items were admitted unchanged, the
  errata/retraction-only total would be 36; the whole-corpus projection would be 37 effective
  logical items and 40 archived representations, absent other changes.
- **Implementation commit** `d4dd5da956a02bbb4895dc6f6e15d0e16bcf9f23` (tree `4a8155dfecfbe422781a832e6550b257c55c84f6`): the
  full gate ran on this exact staged tree, detached with a log, 2026-10-21T23:44:48Z–23:55:37Z, exit 0 / `GATE: GREEN` (Lean 8,926 jobs, golden axioms `[propext]`, fmt/clippy, 82 Rust tests); 455 tracked files;
  registers Q 17 / SD 17 / FLAW 14 / CLAIM 7 / S- 72. Receipt and log:
  [evidence/gate-implementation.json](evidence/gate-implementation.json),
  [evidence/gate-implementation.log](evidence/gate-implementation.log).
- **Rust reproduction on the clean committed implementation tree** (`analysis/s1-first-parent-integration/reproduce.py`,
  the unchanged consumer `s1_effective_inventory`): **passed** at 2026-10-21T23:58:50Z–23:58:54Z — two Rust runs exit 0 with byte-identical outputs, the overwrite attempt refused (exit 1) with outputs unchanged; inventory SHA-256 `b626d1b53f50a23ee5eae9e99fc6ea387bb83f51c807d843d68a29dd1651b803`, identical to the retained integration inventory (35 archived representations / 32 effective logical items unchanged); provenance bound to `d4dd5da956a02bbb4895dc6f6e15d0e16bcf9f23` / `4a8155dfecfbe422781a832e6550b257c55c84f6`. Receipt copied to
  [evidence/reproduction-implementation-receipt.json](evidence/reproduction-implementation-receipt.json);
  outputs under the external root `…\reproduction-d4dd5da956a0\`. The consumer refuses a dirty
  checkout, so an earlier attempt on the uncommitted tree (`…\reproduction-base-d8c0d9c\`, exit 1,
  "commit all inputs and use a clean checkout") is retained as a failed development run, not a
  certificate.
- **Recording commit** (this README, HANDOFF Session 23, the STATE build bullet): its own full
  gate and Rust reproduction are recorded in the pull request and under the external root, outside
  the tree they certify. No earlier gate substitutes for them.

## E. Unrelated local work observed and preserved

A second checkout of this repository on the same machine, `C:\Users\LEET\Documents\ChatGPT\Syndeom`,
is on a local branch `codex/s1-cold-start-attempt` at `af43198324f414687040ef214a3ecb3e71c97e58`
with three commits on top of `26c2555` that are not on `origin` (a non-counting cold-start attempt
record, a root `AGENTS.md` placing the SD15 disclosure before orientation, and a second
disclosure-first attempt record). It was read only to avoid contradiction and was not fetched,
merged, modified or reset; its disposition is the owner's. ADR-0008 agrees with its ordering
(disclosure before any orientation command) and does not depend on it.
