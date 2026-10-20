# RECORD — canonical representation of Route A item 19, Proposition 3.1

- **Date/reviewer:** 2026-10-20 UTC; Codex executor, informed source-fidelity reading.
- **Parent / item of record:** `s1_ingest-v0.1.0-00f28e252823`, arXiv:1209.2022v2,
  Proposition 3.1. This child is one representation, not another independently admitted sample.
- **Child:** `s1_derive-v0.1.0-c08f9e3ec092`; manifest `/2`, informal format `/1`.
- **Producer:** `s1_derive` 0.1.0, `reviewed-tex-slices/1` (ADR-0006),
  engine commit `49e47237924eddf0c39c24aefad0eb9aa02ee8b3`, committed before this manifest
  was generated. No LLM call or sampling occurs in the producer; RNG `none`, seed `null`.
- **Recipe:** `groundtruth/derivations/1209.2022v2-proposition3.1.json`.
  It contains reviewed ranges/hashes, not proof text.
- **Rights:** S-0052/S-0053, arXiv non-exclusive distribution grant; both proof and complete
  context are **external**. The proof and complete source-context files are not committed here;
  the manifest retains the parent's existing status-reference metadata. Retain the author's attribution,
  version and references when inspecting the external files; do not redistribute them through
  this repository. Reproduction retrieves from the primary source.

## Status evidence and exact inheritance

The executor reread both versions' primary abstract pages and source this session.
S-0053's replacement remark explicitly supplies a counterexample attributed to Tian Lan;
the revision comment removes Proposition 3.1 and revises the dependent theorem's proof.
The parent addendum and `analysis/s1-3/retrieval.json` identify this fresh reading.
The original statement and its offered proof, including the error, are the extraction target.
The corrected argument is not substituted. This is documentary status evidence, not an
independent formal verification of the counterexample or a broader verdict on the paper.

The child's origin, derivation, statement status (`false`, `counterexample`, exact reference),
proof status (`invalid`), correctness (`false`), taxonomy (0.1.0), and all flaw metadata are
copied from the parent. The existing class argument is in the parent RECORD. Only the location
is re-expressed. The producer checks exact inherited metadata as well as the ratified
parent/child validator. Its empty-statistics fidelity `PASS` does not certify mathematics.

## Source-fidelity review

The source is the original v2 gzip, 7,300 bytes:
`21f63ae00385cda7216c843cfc4730e994f1fe0c40f645bb49bb323a2456d0ed`.
The complete decompressed `context.tex` is 20,953 bytes:
`25ba26029915d23c5591361c337c20d4b5fc667c6dd704af08933a490b0ecc4f`.
Byte ranges below are half-open in that context. Lines are inspection aids; byte ranges
and hashes in `source-map.json` are authoritative.

| Role | Source bytes | Source lines | Canonical disposition / reviewed context |
|---|---|---|---|
| Statement | `[12073,12152)` | 388–389 | Entire original proposition body becomes step 0; 79 Unicode scalars after ASCII whitespace collapse |
| Offered proof | `[12186,12783)` | 393–402 | Entire original one-paragraph proof becomes step 1; 597 scalars; no inference is supplied or removed |
| Preamble | `[0,4736)` | 1–266 | Complete, unchanged in external context, including definitions of `\C` and `\id` |
| Definitions | `[8514,12015)` | 309–386 | Unitary category/fusion conventions and center/unitary-center definitions, unchanged |
| Prerequisite | `[9163,9431)` | 323–328 | Original Remark 2.1, `remark polar`, unchanged; no strengthened conclusion |
| Bibliography | `[17966,20937)` | 523–604 | Complete source bibliography, including `GHR` and `Mueg2`; opaque original bytes, no replacement decoding |

The executor compared original statement/proof slices against both produced copies.
Every non-whitespace scalar and its order are identical. The only changes are the prescribed
ASCII whitespace collapse, canonical title/statement/proof/step markers and final terminator.
The source's macros, citation options/keys, `remark polar` reference, mathematical expressions,
and purported unitary-arrow argument remain literal. The original definitions and remark are
available in full context. The source's bibliography contains non-UTF-8 bytes outside the
selected text; they remain byte-identical in `context.tex`.

The source invokes external equivalence results. Their citation keys and bibliography are
retained; those works were not retrieved or independently verified in this slice, and no such
verification is claimed. The artifact is source-faithful canonical text with required context,
not a standalone rendered TeX proof. Source testimony identifies the proposition-wide error;
the parent's narrower proposed mechanism remains explicitly the executor's interpretation.

## Flaw-location re-expression

There is one parent flaw: instance 1, FLAW-0012, null sub-variant, fatal, YES-KERNEL, empty dials.
Its raw locator names Proposition 3.1's statement and offered proof, without a source-identified
failing proof step. The exact original locator is retained in `source-map.json`.

The canonical anchor is **step 0, span `[0,79)`**, the entire statement, with text SHA-256
`4e6f5d7d5c99f1c3495c9a45b8e4de5d3411c9dbc6f8e1890f6b21e182ff298d`.
Step markers are excluded from span counting. This represents the statement-wide composite
flaw and makes no new claim about a particular proof inference. The associated offered proof
is preserved whole as step 1, source range `[12186,12783)`. The single-location schema cannot
encode a cross-step range; choosing the full statement as anchor avoids inventing a narrower
failure site or splitting one source flaw into additional flaw instances.

## Reproduction and verification

From a checkout containing the evidence:

```text
python -I -X utf8 scripts/reproduce_s1_3.py --work-dir <new-directory-outside-repo>
```

The committed receipt is `analysis/s1-3/real-result.json`. It records the producer and checkout
commits separately, the exact reproducer hash, executed argument arrays, primary retrieval,
exit codes and all output hashes. Both Rust runs passed `Manifest::check` with actual proof
bytes and `Manifest::check_derived` against the real raw parent. Repeated output was
byte-identical: 770-byte proof, 75 whitespace tokens, one proof step, zero lemma steps.
These are syntax-defined counts, not mathematical proof-length or correctness measurements.

Proof SHA-256: `79929ff56666b0f80283625f683e3d5598e521010ab7426dd3071aa6525bbc5d`.
Manifest SHA-256: `7ce4a924866a5ce58f9f670d74e6b5cbd39192d8013e8856b7f1eb1c6d09d97b`.
Source-map SHA-256: `7c0db9ca74344405b8b2e7d5454f9256914fca0d24bce44324a3e5bc50d70517`.
Full gate results and mechanical checks are kept separately under `analysis/s1-3/`.
