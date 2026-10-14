# RECORD — Route A item 9 (candidate 12, first documented result): arXiv:1107.0863 v3, Lemma 12 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1107.0863`; original = v3 (26 March 2013);
  correcting version = v4 (22 April 2026, inside the ADR-0003 D8 window), whose appendix the
  authors say also appears as a journal correction (DOI quoted in S-0039); examined Session 15,
  2026-10-14 (access 03:53:05Z). Two authors; Metric Geometry (math.MG). The same candidate yields
  a second item (Theorem 1), recorded in its own directory.
- **Status document (SD12; D2.2):** the v4 revision comment ("Previous version had an error in
  Lemma 12, affecting Theorem 1 …") with v4 itself, whose appendix states: "The error in [CH] is
  in the proof of [Lem. 12], which is false as stated; Figure A shows a counterexample. We correct
  the lemma below, using the extra hypothesis." S-entries: S-0038 (the original, v3), S-0039 (the
  corrected version, its comment and appendix).
- **Provenance and digests:** abs v3 41,872 B `7c136ca3…6144`; abs v4 42,134 B `3338cd9a…eb1d`;
  e-print v3 4,152,829 B `13e63ce7…1727` (gzipped tar with figures) — the original, **not
  committed**; e-print v4 4,287,074 B `489ab215…7915`. Full digests in S-0038 and S-0039.
- **The claim of record (quoted from v3):** Lemma 12 (TeX label `lem:normal`): "Any induced cycle
  C of Υ₀(U) of odd length n = 2k+1 contains at least one normal hyperplane H_i." Its proof opens
  "By Lemma [trianglefree], n > 3. Suppose that no hyperplane in C is normal. Then the hyperplanes
  of C can be partitioned into two sets ℐ and 𝒪, where ℐ is the set of H_i for which both
  neighbours in C are incoming, and 𝒪 is th[e set …]" and continues for one paragraph.
- **The notice's locating and describing sentences (quoted):** "an error in Lemma 12"; "The error
  in [CH] is in the proof of [Lem. 12], which is false as stated; Figure A shows a
  counterexample"; "Proposition A replaces Proposition 11 and Lemma 12 of [CH]".
- **Diff summary (executor's reading of the TeX diff v3 → v4):** the original text is unchanged
  apart from a moved paragraph in Section 5 and packaging; the correction is an appended appendix
  that restates Theorem 1 with the 5-cycle hypothesis, gives the counterexample figure to Lemma 12
  (a 5-cycle H₁ → H₂ → H₃ → H₄, H₁ → H₀ → H₄ in Υ₀(U) with the root in U), proves that 5-cycles in
  the crossing graph correspond to 5-cycles in vertex links, replaces Proposition 11 and Lemma 12
  by "Proposition A: if Γ_#(X) has no 5-cycle then Υ₀(U) is bipartite", and reruns the Section 7
  argument with that proposition.
- **Location (D3, `raw/1` locator):** "Lemma 12, statement and proof"; the notice locates the
  error to the proof without naming the step, and the executor does not supply one: the lemma's
  falsity is established by the authors' counterexample, which is what the class below needs.
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition), lean_catchable YES-KERNEL.** The statement is false — the authors say so and give
  a counterexample (a 5-cycle of Υ₀(U) with no normal hyperplane) — and the offered proof is a
  one-paragraph argument of the kind that proves the corrected proposition under the extra
  hypothesis: the entry's definition ("the statement is false (a counterexample exists) and the
  proof mirrors the argument for a nearby true statement"). The failing step of the proof is not
  named by the notice and is not recorded; a use-site record (FLAW-0005 or FLAW-0004) would be
  the executor's guess, so none is made. Lean-catchable: a sorry-free, standard-axiom proof of a
  false statement cannot exist (`YES-KERNEL` per the entry).
  Other classes considered: FLAW-0014 (incomplete case analysis) — the proof partitions the
  cycle's hyperplanes into two sets and argues on them; whether the omitted possibility is a
  missing case or a false inference is exactly what the notice does not say; not recorded.
- **Statement status (D3):** `value: false`, `method: "counterexample"`, `reference` = the
  appendix's Figure A as described (S-0039) — the status document disproves the statement with a
  counterexample, so the counterexample method applies.
- **Proof status:** `invalid` (S1.1 by construction; the authors document the error in the proof).
- **Human-authored (D2.6):** two human authors; no AI-assistance mention (the scanner's "llm"
  hit is inside the citation key `BallmanSwiatkowski`, checked); the error was found after two
  named mathematicians showed the authors examples (quoted in S-0039).
- **Licence (D5, SD16):** v3 and v4 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1107.0863v3.tar.gz`, 4,152,829 B, `13e63ce7…1727`). Recorded in S-0038 and
  S-0039 before ingestion.
- **What this item is and is not:** a real, human-authored, published paper whose authors
  document a false lemma with a counterexample and publish a correction. Nothing here assesses
  the paper's other results (the appendix says they are unaffected) or the corrected theorem
  (SD3); the claim of record is Lemma 12 as stated in v3.
- **Derived canonical artifact (S1.3):** not yet produced. Note for S1.3: the e-print is 4 MB of
  figures around one TeX file; the lemma and its proof are one paragraph each.
- **Written by:** the executor (Claude Fable 5.1), Session 15; a reading of public documents,
  not a verdict on the paper (SD3).
