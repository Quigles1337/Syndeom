# Proposed disposition: one raw evidence-attribution amendment

**2026-10-20 UTC — proposed, not effective.** Owner acceptance is not recorded.
The complete generated candidate is retained under `candidate/`, outside the live
`groundtruth/` tree. Neither this proposal nor a merge of its evidence alone releases
the canonical-promotion hold.

## Finding and contract boundary

The [written witness](WITNESS.md) checks two separate statements: the v70 displayed
character expansion gives 1 versus 0 at a=1,b=2,k=2; the lemma's residual at
a=1,b=2,s=2 is log(2)/3>0. This repairs the mathematical noncancellation gap in the
old RECORD. The author notice still supplies only the named correction, not an
explicit declaration of falsity or this counterexample.

The existing contracts support retaining and reviewing this evidence, but do not
unambiguously authorize making it the effective corpus status:

| Contract | Permission or constraint | Consequence here |
|---|---|---|
| ADR-0001 D4 | `counterexample` may reference a checkable counterexample itself | The proposed status is structurally expressible and validates |
| ADR-0003 D2.2/D2.3 | Author/publisher status document establishes and locates an error | S-0025 still supplies that evidence for this existing item |
| ADR-0003 D3 | Statement status comes from the status document; the counterexample case describes one supplied by that notice | Our executor-written witness is a different evidence origin |
| SD12 and ADR-0003 D4 | Real-item derivation is `documented-by-source`; the status-establishing document must be recorded | Retaining that enum cannot silently attribute our reasoning to the author |
| ADR-0000 Addendum 3(v), ADR-0003 rationale | No LLM admission classifier; executor document reading is the recorded workflow | These mathematical checks do not constitute an automatic admission decision; owner must address this case explicitly |
| Appended first-parent hold | Allows a separately checked, clearly attributed counterexample and an explicit amendment trail | Evidence work is permitted; the hold does not supply a general raw-record amendment mechanism |
| ADR-0001 D2/D5 | Configuration determines hash and ID | Changing method/reference requires a different ID; the old object remains immutable |

The schema validator is not a policy adjudicator. No existing decision is amended by
this packet. The two same-executor checks are disclosed; no independent human or
blind review is claimed. The owner can reject the proposal without losing the evidence.

## Exact proposed owner ruling

The owner may accept or reject the following case-specific ruling as one decision:

> For the already archived Lemma 1.1 of arXiv:0706.0357v70 only, accept the separately
> attributed written counterexample in this packet as support for
> `statement_status = {value: false, method: counterexample, reference: <the exact
> WITNESS.md reference in candidate-config.json>}`. S-0025 remains the author-documented
> error/locator and `config.status.reference`; it is not the source of this witness.
> Authorize this narrow exception to ADR-0003 D3's notice-supplied truth evidence,
> with `documented-by-source` retained solely for the source-documented admission and
> offered-proof error under SD12. Its statement-truth support is explicitly the
> referenced executor-written counterexample. This does not authorize an LLM classifier,
> generalize to other items, or turn executor inference into author testimony.
>
> Approve the generated raw candidate `s1_ingest-v0.1.0-15ab865466d6` as a metadata
> replacement proposal for `s1_ingest-v0.1.0-63e23a952084`, subject to a separate verified
> integration commit. Preserve the original object and its historical record. Record
> the old-to-new relation in `amendment-map.json` as one logical item, with the old ID
> remaining its stable logical-item key. Exactly one representation may supply effective
> status for that item in a subsequent report. Neither is a canonical child of the other.
>
> On integration, record this ruling and its date in an ADR and the append-only record,
> validate the admitted bytes and mapping, and make subsequent consumers use the new
> effective representation without double counting. Release the status-support hold
> only after these requirements are satisfied. Canonical production still requires
> its own authorized selection, context and locator review. Preserve the frozen S1.4
> packet and publish any effective-status change as a later event.

This text expressly puts the D3/SD12 interpretation and the ratified admission boundary
before the owner. Approval must identify this ruling and candidate; a passing build,
sidecar, discussion of the argument or evidence-only merge is not implicit approval.
If rejected, both the historical metadata and current hold remain, with the witness
retained as analysis. No blanket false-to-undetermined change is proposed.

## Concrete identity and minimal mapping

| Field | Archived | Proposed |
|---|---|---|
| Artifact ID | `s1_ingest-v0.1.0-63e23a952084` | `s1_ingest-v0.1.0-15ab865466d6` |
| Statement value | false | false |
| Statement method | source-document | counterexample |
| Statement reference | S-0025 | WITNESS.md path + exact SHA-256 + executor/session |
| Author status reference | S-0025 | S-0025 |
| Actual producing revision | `30e2a4aa06368b8a2ba1cc1a8164c4d22229a0cd` | `1a76d9ac21cd5d2e4311f9164178327c0ca128bb` |
| Proof status / correct | invalid / false | invalid / false |
| Source bytes, rights, flaws, severity, catchability | Original values | Identical |
| Derivation | documented-by-source | Same enum; interpretation requires the ruling above |
| `derived_from` | Absent | Absent |
| Corpus effect now | Historical effective record, qualified truth/promotion held | Zero: proposed output outside live corpus |

Only `statement_status.method` and `.reference` change inside the configuration.
The ingest producer copies the new status to `ground_truth.statement_status`; both
agree. `notes` records attribution and the pending decision but is outside the config
hash. The new producing revision and config-derived ID/hash change legitimately.
`evidence/candidate-run-01/receipt.json` and the Rust validator record the complete
hashes and actual-byte checks. No old manifest/config is rewritten.

`amendment-map.json` is the complete proposed relation, not a general supersession
schema. Currently the live corpus remains **34 representations / 32 logical items**.
If a later integration retains both raw representations, it will have 35 archived
representations but still **32 effective logical items**, and this pair contributes
exactly one. Consumers that merely count raw directories cannot be used for the
amended effective inventory until they honor the explicit relation; this is an
acceptance requirement, not functionality implemented here.

## Disposition by obligation

| Obligation | State at this commit | Next condition |
|---|---|---|
| Finite proof-step witness | Checked in writing and with scoped Lean support | Owner may review the disclosed source correspondence |
| Lemma residual / noncancellation | Checked written derivation by two methods; partial kernel support | Analytic bridge is not fully formalized; no PROVEN claim |
| Author attribution | Resolved in this review: notice, diff and executor witness separated | Preserve that distinction in any admitted amendment |
| Old embedded metadata | Unchanged and still held | Owner accepts exact ruling; separate integration validates replacement |
| Candidate metadata / identity | Generated, validated, replayed; proposed only | Approval and integration, not a sidecar alone |
| Canonical eligibility | Held | Ruling and consistent effective metadata first; later canonical-selection review |
| S1.4 / first-write bins | Frozen byte-for-byte | Any later status event/report must be separately identified |
| Gate S1 | Open | Dispose of the substantive hold; green integration; later genuine cold-start examination |

The seven truth questions, seven severity questions and unnamed-dependent-results
question retain their historical nonblocking classification and exclusions. They are
not fourteen new prerequisites for this case. Thirty roots' missing canonical lengths
remain scoring prerequisites, not a requirement to canonicalize the entire prefix.
Harvest remains through `1403.4485`, with candidate 28 / `1403.6317` saved next.
