# S1 readiness after verified main activation

**Assessment began:** 2026-10-21 UTC (2026-10-20 America/New_York).
**Scope:** informed reconciliation of the admitted prefix at main
`1c56028532ea135603dbad5423d5555313bfd9ec`, tree
`14b451866491dc4a2b866ec0b1d726d792385916`. The [activation event](README.md)
separates the owner's acceptance, verified integration and PR 7's actual merge.

**Ready to attempt the separate cold-start examination once this reconciliation is
on main. Gate S1 remains OPEN.** No additional substantive S1 exit criterion is
identified as unmet for this admitted prefix under the rules below. This is a
readiness assessment, not the examination or its result. The required exact final
reconciliation-tree gate is recorded externally and in its PR; an earlier gate does
not certify a changed final tree.

## Criteria, evidence and limits

The governing [constitution §2](../../PROGRAM/SYNDEOM-PROGRAM.md) says:
“`groundtruth/` manifested; every item sourced, licensed, and status-documented; class
coverage table with the power statement; repo green; cold-start test.” Its S1.1–S1.4
requirements are mapped individually here. The prefix permission and reporting
qualifications come from [ADR-0003 D8/D9](../../PROGRAM/DECISIONS/ADR-0003.md).

| Requirement and governing rule | Evidence and present disposition | Limitation carried forward |
|---|---|---|
| S1.1: original, notice, located error and written class argument; SD12/SD16; ADR-0003 D2–D8 | The [harvest log](../../groundtruth/HARVEST-LOG.md), per-item RECORDs and [SOURCES](../../PROGRAM/SOURCES.md) document the admitted, unskipped Route A prefix through candidate 27 / `1403.4485`: 21 admitted candidates yielding 31 logical errata/retraction items. The accepted first-parent exception is effective on verified main. | The 2,472-candidate harvest is not exhausted. Candidate 28 / `1403.6317` is the saved resume point, not the current next action. D8 explicitly permits Gate S1 on a prefix; no new harvest is required for this assessment. |
| S1.2: informal statement and pinned Mathlib formalization as positive statement-fidelity ground truth | One positive pair was admitted by the owner's PR 4 merge. [ADR-0007](../../PROGRAM/DECISIONS/ADR-0007.md), the [implementation packet](../s1-2/implementation/README.md) and both pair RECORDs retain source correspondence, actual generation, both-file/lineage checks, emitted type/axiom audit and replay. | Its selected informal statement has seven tokens and **zero proof steps**. It is not a positive-length informal proof control. The 14-token Lean alias is not a measurement of its transitive Mathlib proof. No independent GitHub reviewer approval, blind validation or general semantic-fidelity reliability is inferred. |
| S1.3: real items manifested under the origin extension; SD1/SD6/SD10; Gate S1's manifested-corpus condition | The existing Rust consumer reproduced [the retained effective inventory](../s1-first-parent-integration/report/inventory.json) exactly on merged main: **35 archived representations (33 raw, two canonical), 32 effective logical items**. It validates identities, status agreement, witness/owner binding, available committed bytes, lineage, mapping and preservation. [Fresh receipt](evidence/main-reproduction/receipt.json) and [provenance](evidence/main-reproduction/provenance.json) identify the merge. | Both raws for the amended lemma remain historical representations of one logical item. Thirty logical items lack canonical representations. Under ADR-0001 D4 and ADR-0003 D6, canonical selection, source context and locator review are prerequisites for items later scored; canonicalizing the entire prefix is not an S1 exit requirement. |
| Every item sourced, licensed and status-documented; SD12/SD16; accepted ADR-0003 case exception | Source entries, per-item records and the frozen [source-group matrix](../s1-4/report/report.md) retain the admission/licence trail. The [owner decision](../s1-first-parent-integration/OWNER-DECISION.md), witness and [effective map](../../groundtruth/amendment-map.json) pass the existing Rust integration conditions on main. S-0025 remains author error/locator evidence; the separately attributed written counterexample supplies this case's statement status. | The seven older truth and seven severity questions keep their documented nonblocking dispositions and exclusions below. External files retain historical retrieval/producer evidence; this inventory does not freshly retrieve them. Their bytes must be obtained and hash-verified before future scoring. No new rights, truth labels or source-verification transitions are inferred. |
| S1.4 and ADR-0003 D9: per-class coverage, lengths, zero classes and interim power statement | The complete [frozen S1.4 report](../s1-4/report/report.md), bins and receipts remain exact for corpus `e6a7cae4392dd1565ee57f9fb727cc01e453687d` and tool `4e46564ac62bc2bd7a5bef54265e9b2dd26772bb`. It has all 14 class rows plus a total, severity/catchability, length bins and missingness, zero classes, dispositions and the power statement. The later effective inventory separately records the accepted status amendment: 46 flaw instances, five observed classes, nine zero classes; unflagged subset 18. | The frozen report's 34 representations and 17-item unflagged subset describe its earlier snapshot; they are not current counts. Its first-write bins and historical results are not regenerated. Zero coverage is a finding: S4 cannot validate those classes on this prefix. Neither filling every class nor extending the harvest is a Gate S1 requirement. |
| Repo green; SD9 and ADR-0002 D6 | The tested PR head and merge share the exact tree. The main-push workflow succeeded, and this session ran the full local gate on clean merged main. The [event](README.md) links the actual commands, receipts and checks. The exact final committed reconciliation tree receives a separate full gate; its receipt/log remain outside that tree and in the PR. | CI is advisory. The local gate is authoritative. Build/test success establishes the specified checks, not the mathematical completeness of the analytic bridge or detector reliability. A later changed tree needs its own gate. |
| Cold-start examination; constitution §2, SD15, §4.1 and §6; ADR-0005 | **OPEN — the remaining gate examination.** A genuinely fresh session must reconstruct state from the repository alone in one pass, verify the gate and assess closure. | This informed conversation and any full-history continuation are not cold. Launch only with §6's Session-N resume prompt verbatim and alone, with no state-bearing preamble, handoff or additional task text. Honor SD15's disclosure and no-memory-write requirements. This assessment cannot certify a future PASS. |

## Existing qualifications, unchanged

- **First parent:** status-support hold discharged on verified main. Stable logical key
  `s1_ingest-v0.1.0-63e23a952084` has exactly one effective status, from
  `s1_ingest-v0.1.0-15ab865466d6`. This is a raw metadata amendment, not a canonical
  child. Both original producing revisions and both raw objects remain intact.
- **Seven truth questions:** items 8, 10, 11, 14, 15, 18 and 25 keep their recorded
  labels and truth-review flags. They remain in historical class counts and are
  excluded from qualified truth cells; no false-to-undetermined adjudication occurs.
- **Seven severity questions:** items 13, 16, 21, 24, 26, 27 and 29 retain their
  recorded severities and severity-review flags, and stay excluded from the
  conservative unflagged subset. Their existing nonblocking classification stands.
- **Unnamed-result question:** candidate 25's unnamed downstream Theorems 4.5, 5.1
  and 5.5 remain unadmitted under the recorded ADR-0003 D2.3 question. No items or
  guessed classes are added. Exact IDs/references and the prior dispositions remain
  in [the frozen overlay](../s1-4/config/dispositions.json) and historical
  [obligations ledger](../s1-4/OBLIGATIONS.md). Only that ledger's first-parent hold
  and pending-merge next action have been overtaken by the separately dated events.
- **Ten undetermined statements:** still proof-flaw examples; excluded from every
  truth-dependent cell, including composites. Review categories overlap other
  missingness; they are not additive populations. Current Rust counts are 5 true /
  17 false / 10 undetermined, truth-qualified 5 true / 10 false, and offered-proof
  status 1 valid / 31 invalid.

## Boundaries that do not become new S1 exit criteria

The accepted counterexample has scoped Lean support; the complete analytic
source-to-Lean M/L bridge remains unformalized. The written checks are same-executor,
not independent human or blind review. The scoped owner exception does not authorize
general executor-assigned truth labels. No CLAIM becomes PROVEN.

All real-item cells remain **expected underpowered** for the equivalence test until
S3 derives N_min and freezes its design. ADR-0004's analytic calculation, bootstrap/tie
simulation, band-edge and dependence/effective-N checks remain future obligations.
S4's intended scope is recall against documented errors and a false-positive upper
bound on correct proofs at supported lengths. There is no measured detector
reliability, numerical power, fooling length or blind evaluation here.

Missing canonical lengths and absent external bytes remain explicit prerequisites
for future scoring. No new canonical artifact, pair generation, producer, detector,
harvest, witness investigation or general amendment mechanism is needed for this
bounded reconciliation. The next action is the separate genuine examination, not
automatic S1 closure or S2 work.
