# RECORD — Route A item 29 (candidate 25, fourth documented result): arXiv:1304.5623 v4, Theorem 5.3 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 26 (arXiv `1304.5623`, v4 → v5, Session 20); the same
  original bytes and the same S-entries (S-0060, S-0061); a different claim of record.
- **The claim of record (quoted from v4, Section 5 "Supersingular K3 surfaces are unirational",
  Theorem 5.3, TeX label `thm:main`):** "Supersingular K3 surfaces in characteristic p ≥ 5 are
  unirational." — the paper's title and main theorem ("the conjecture of Artin, Rudakov,
  Shafarevich, and Shioda", as the sentence before it says). The proof, whole: "In odd
  characteristic, supersingular Kummer surfaces are unirational by [Shioda, Theorem 1.1]. The
  assertion then follows from Theorem 5.1." Theorem 5.1 (`thm:correspondences`) asserts purely
  inseparable isogenies between any two supersingular K3 surfaces in characteristic p ≥ 5 and
  between any one and the Kummer surface Km(E × E); its proof applies Corollary 4.6 (an isogeny of
  height 2 lowering the Artin invariant), which rests on Theorem 4.5 and, through it, on Assertion
  (4) of Theorem 3.6.
- **Status document (SD12; D2.2, D2.3):** the v5 comment — "due to a mistake in Proposition 3.5,
  the unirationality of supersingular K3 surfaces remains a conjecture" — the footnote ("our
  results merely show that supersingular K3 surfaces are related by sequences of very special
  correspondences, which is not sufficient to deduce their unirationality. The unirationality
  conjecture thus remains a conjecture.") and the erratum: "the main result, the unirationality of
  supersingular K3 surfaces Theorem 5.3, remains a conjecture"; "The strategy of establishing
  Theorem 5.3 consisted in showing that any two supersingular K3 surfaces in characteristic p ≥ 5
  can be connected by one-dimensional families of the type as constructed in Section 3 and then,
  that unirationality is preserved in these families. This reduced the unirationality conjecture
  to the showing that there exists at least one unirational K3 surface in each characteristic,
  where one can use Kummer surfaces, whose unirationality was established by Shioda. Now, in
  order to show that unirationality is preserved in these one-dimensional families, one needs
  Assertion (4) of Theorem 3.6 … Without this, one can only conclude that any two supersingular
  K3 surfaces in characteristic p ≥ 5 are connected by a sequence of very special degree p
  correspondences …, which is not sufficient for unirationality."
- **Location (D3, `raw/1` locator):** "Theorem 5.3, proof: 'The assertion then follows from
  Theorem 5.1'" — the invocation whose premise the erratum withdraws, through the chain Theorem
  5.1 → Corollary 4.6 → Theorem 4.5 → Theorem 3.6 (4) → Proposition 3.5.
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity gap,
  lean_catchable YES-KERNEL.** The two-sentence proof's only non-trivial step invokes Theorem
  5.1, whose purely inseparable isogenies the erratum says are not established (they need
  Assertion (4) of Theorem 3.6, item 27); read as in items 13, 16, 20, 24 and 27, the invoked
  result is the failed premise. Gap, not fatal: nothing is shown false — the author's word is
  "remains a conjecture" — and the reduction to Theorem 5.1 is itself untouched; D3's consequence
  clause ("withdrawn → fatal") is read as in the sibling items (the escalation of HANDOFF Session
  19 stands).
  Other classes considered: FLAW-0012 — not recorded, the statement is a conjecture, not a
  falsity; FLAW-0003 — no circularity.
- **Statement status (D3; ADR-0001 as amended):** `value: null`, `method: "undetermined"`,
  `reference: S-0061` ("remains a conjecture"). The tenth undetermined-truth item; `correct` is
  `false` by derivation.
- **Proof status:** `invalid` (S1.1 by construction; the author withdraws the result to a
  conjecture).
- **Human-authored, licence, provenance:** as item 26 (S-0060, S-0061; `external`).
- **What this item is and is not:** the same paper's fourth documented result — the main theorem
  of a published paper, withdrawn to a conjecture by its author's published erratum. Theorem 5.1
  (the intermediate link), Theorem 4.5 and Theorem 5.5 are affected by the same chain but are not
  named by the erratum and are not recorded (D2.3; SD3); the record notes that the footnote's
  "related by sequences of very special correspondences" describes what replaces Theorem 5.1's
  content. Nothing here assesses the later literature on the conjecture, including the Lieblich
  proof the introduction mentions, which the erratum does not discuss (SD3). The dependence
  between items 26–29 is for the S3 calibration ADR's effective-N rule (ADR-0004 D6).
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 20; a reading of public documents,
  not a verdict on the paper (SD3).
