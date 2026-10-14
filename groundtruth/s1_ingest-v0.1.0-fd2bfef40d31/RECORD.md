# RECORD — Route A item 13 (candidate 15, second documented result): arXiv:1201.3327 v2, the former Theorem 5.9 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 12 (arXiv `1201.3327`, v2 → v3, Session 16); the same
  original bytes and the same S-entries (S-0042, S-0043); a different claim of record.
- **The claim of record (from v2, Section 5):** the former Theorem 5.9 — "Let E be an elliptic
  curve defined over the number field K with non-archimedean absolute value v, and let α ∈ ℚ̄.
  Then K^{nr,v}(α) has the Bogomolov property relative to ĥ_E if and only if there is a
  w ∈ M_{K(α)}, w | v, such that E/K(α) has bad reduction at w." — introduced by "We can
  give a complete classification of finite extensions of K^{nr,v} having the Bogomolov property
  relative to ĥ_E" and proved by "If we combine this lemma with the preceding results we obtain".
- **Status document (SD12; D2.2):** the v3 comment ("This false lemma and the former Theorem 5.9
  have been deleted in this version") and the v3 note "In contrast to the statement given in the
  published article, I am not able to classify all these field extensions. This is due the fact
  that the claimed Lemma 5.8 in the previous version of this article is not true"; the theorem is
  replaced by a weaker Proposition the author calls "the trivial consequence of the preceding
  results".
- **Location (D3, `raw/1` locator):** "the former Theorem 5.9, proof: the combination of Lemma
  5.8 with the preceding results".
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity gap,
  lean_catchable YES-KERNEL.** The proof applies the former Lemma 5.8, which is false, so the step
  that combines it "with the preceding results" invokes a result that does not hold — a
  hypothesis-level failure of the argument rather than a sign or an omission; the entry's clause
  "a named theorem is invoked on an object for which one of its hypotheses is … false" is read
  here with the lemma itself as the failed premise. Gap, not fatal: the author withdraws the
  classification but does not say the theorem is false — "I am not able to classify all these
  field extensions" — so the truth is open and the damage is to the argument.
  Other classes considered: FLAW-0012 — not recorded, because the document does not say the
  theorem is false; FLAW-0011 — not recorded, because it does not say it is true.
- **Statement status (D3; ADR-0001 as amended 2026-10-13):** `value: null`,
  `method: "undetermined"`, `reference: S-0043` — the theorem is neither re-proved nor disproved:
  deleted and replaced by a weaker statement. The second undetermined-truth item in the corpus
  (after item 3); `correct` is `false` by derivation.
- **Proof status:** `invalid` (S1.1 by construction; the proof rested on the false lemma).
- **Human-authored, licence, provenance:** as item 12 (S-0042, S-0043; `external`).
- **What this item is and is not:** the same paper's second documented result, recorded as its
  own item because it is a distinct claim of record whose status differs from the lemma's
  (undetermined rather than false); the dependence between items 12 and 13 (one notice, one
  false lemma) is for the S3 calibration ADR's effective-N rule (ADR-0004 D6), and the S3
  obligation for undetermined items (STATE standing rulings) applies to this one.
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 16; a reading of public documents,
  not a verdict on the paper (SD3).
