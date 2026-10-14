# RECORD — Route A item 5 (candidate 8, first documented result): arXiv:1005.2398 v2, Proposition 3.3.5 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1005.2398`; original = v2 (1 September 2018);
  correcting version = v3 (22 March 2026, inside the ADR-0003 D8 window); examined Session 14,
  2026-10-14 (access 03:37:02Z). One author; Algebraic Geometry (math.AG). The same candidate
  yields a second item (the lemma that follows the proposition), recorded in its own directory.
- **Status document (SD12; D2.2):** the v3 revision comment — "several minor errors corrected,
  including the addition of a finitely presented hypothesis to Proposition 3.3.5" — with v3
  itself, whose Proposition 3.3.5 carries the footnote: "The published version of this paper had
  a version of this statement without the finitely presented hypothesis, which was incorrect.
  Namely, when A → B is not finitely presented, the implication [(2) ⇒ (1)] fails for
  ℤ → ℤ[{x_p}_{p prime}]/(p x_p, x_p^p)." (the clause in brackets is the executor's reading of the
  footnote's middle, which names the failing implication; the counterexample is the author's).
  S-entries: S-0032 (the original, v2), S-0033 (the corrected version, its comment and footnotes).
- **Provenance and digests:** abs v2 38,222 B `f15feae8…981a`; abs v3 38,226 B `72e0c3d7…8c51`;
  e-print v2 42,669 B `a651e35a…cd39` (gzipped tar) — the original, **not committed**; e-print
  v3 43,530 B `44262554…0112`. Full digests in S-0032 and S-0033.
- **The notice's locating and describing sentences (quoted):** "the addition of a finitely
  presented hypothesis to Proposition 3.3.5"; the footnote quoted above.
- **Diff summary (executor's reading of the TeX diff v2 → v3):** the proposition's hypothesis
  "Let A → B be a homomorphism of rings" becomes "Let A → B be a finitely presented homomorphism
  of rings"; the clause "If A → B is of finite type, then the above conditions are also
  equivalent to: (3) …" is commented out (finite presentation now being assumed throughout); the
  proof of (2) ⇒ (1) is rewritten — v2 derived "an isomorphism at all points with characteristic 0
  residue field by Lemma lemma-adequate-char0", v3 derives the isomorphism after tensoring with ℚ
  from universal adequacy and then uses finite presentation and the remark
  `rmk-adequate-homeomorphism` (local isomorphism ⇔ stalk isomorphism for locally finitely
  presented morphisms) to conclude; (1) ⇒ (3) is re-argued through a finite A-module Q = B/A.
  The introduction's summary sentence gains the hypothesis. Elsewhere the diff fixes unrelated
  small errors not named by the comment (not harvested, D2.2).
- **Location (D3, `raw/1` locators):** instance 1 — "Proposition 3.3.5, statement"; instance 2 —
  "Proposition 3.3.5, proof, the step (2) ⇒ (1)".
- **Class argument (D3):**
  - **Instance 1, FLAW-0012 (false theorem with convincing proof), severity fatal (by
    definition), lean_catchable YES-KERNEL.** The author's footnote supplies the counterexample
    and says the statement without the hypothesis "was incorrect"; the v2 proof is the argument
    for the finitely presented case with the hypothesis dropped — the entry's recipe in reverse
    ("drop or weaken one hypothesis to obtain a false statement with a known counterexample;
    transplant the true-case proof"). Lean-catchable: a sorry-free proof of a false statement
    cannot exist (`YES-KERNEL` per the entry).
  - **Instance 2, FLAW-0005 (theorem applied without its hypotheses), severity fatal,
    lean_catchable YES-KERNEL.** The v2 step (2) ⇒ (1) passes from "an isomorphism after
    tensoring with ℚ" to "an isomorphism at all points of characteristic 0" by a lemma whose
    argument needs the morphism to be locally of finite presentation (v3 makes the use explicit
    through the remark); for the counterexample the hypothesis is false and the conclusion fails
    — the entry's fatal case. This is the hidden use site FLAW-0012's entry predicts ("usually as
    FLAW-0005").
  - Other classes considered: FLAW-0008 — no formal pair; FLAW-0004 — the step is justified by a
    cited lemma, not by a hedge; the defect is the lemma's unmet hypothesis.
- **Statement status (D3):** `value: false`, `method: "counterexample"`, `reference` = the
  author's counterexample as quoted (S-0033) — the status document itself disproves the
  statement as written, so the counterexample method applies.
- **Proof status:** `invalid` (S1.1 by construction; the author documents the error).
- **Human-authored (D2.6):** one human author; no mention of AI assistance in the v3 sources
  (helper scan: none) or in any comment; the finder of the error is unstated.
- **Licence (D5, SD16):** v2 and v3 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1005.2398v2.tar.gz`, 42,669 B, `a651e35a…cd39`). Recorded in S-0032 and
  S-0033 before ingestion.
- **What this item is and is not:** a real, human-authored paper (the footnote refers to "the
  published version") whose author documents a missing hypothesis in one proposition with a
  counterexample. Nothing here assesses the paper's other results (SD3); the claim of record is
  Proposition 3.3.5 as stated in v2.
- **Derived canonical artifact (S1.3):** not yet produced; the location re-expression argument
  is added here when it is.
- **Written by:** the executor (Claude Fable 5.1), Session 14; a reading of public documents,
  not a verdict on the paper (SD3).
