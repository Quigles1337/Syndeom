# RECORD — Route A item 2: arXiv:0707.3799 v2, Theorem 1 b) (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A (arXiv revision comments); arXiv `0707.3799`; original =
  v2 (18 February 2008); correcting version = v3 (25 February 2026, inside the ADR-0003 D8 window;
  a later v4 of 9 September 2026 corrects other things and is not the notice for this item);
  examined Session 11, 2026-10-14 (access 01:58:05Z). Two authors; Representation Theory (math.RT).
- **Status document (SD12; D2.2):** the v3 revision comment — "This version corrects a mistake in
  the proof of Theorem 1b) and a typo in Proposition 7. All the statements are unchanged from the
  previous version. We thank Jakub Löwut for pointing out the issue in Theorem 1b)" — together
  with the corrected version v3, whose proof of Theorem 1 b) carries the footnote "The proof of
  this part of the Theorem in the published version contained a mistake, here we present a
  corrected proof." S-entries: S-0026 (the original, v2), S-0027 (the corrected version and its
  comment).
- **Provenance and digests:** abs v2 40,691 B `d37bcb17…7699`; abs v3 41,001 B `d2444a78…dd6b`;
  e-print v2 40,240 B `3f9593e1…3053` (one gzipped TeX file) — the original, **not committed**;
  e-print v3 41,505 B `0818ab63…5a00`. Full digests in S-0026 and S-0027.
- **The notice's locating and describing sentences (quoted):** "corrects a mistake in the proof of
  Theorem 1b)"; "All the statements are unchanged from the previous version"; footnote: "The
  proof of this part of the Theorem in the published version contained a mistake, here we present
  a corrected proof."
- **Diff summary (executor's reading of the TeX diff v2 → v3):** the whole proof of part b) of
  Theorem 1 (TeX label `normocon`: the equivariant cohomology of the affine Grassmannian for a
  group G that is not necessarily simply connected) is replaced. The v2 proof, one paragraph,
  argued: Gr_G is a union of connected components indexed by characters of π₁(G), each isomorphic
  to the Grassmannian of the simply connected cover; "the isomorphisms of various connected
  components are T×G_m-equivariant. Hence" the T×G_m-equivariant cohomology of Gr_G equals the
  product of the components' cohomologies; "Finally", the G_O⋊G_m-equivariant cohomology is the
  W-invariant part. The v3 proof describes the component cohomology as functions on an affine
  blow-up with a natural W-action, builds the identification between components explicitly as the
  shift by a coweight λ composed with the inverse of λ's action on the blow-up, and argues that the
  resulting isomorphism "is independent of the choice of λ and hence it is W-equivariant" before
  taking W-invariants. The statement of Theorem 1 is unchanged except for notation. The rest of the
  diff is notational (`\epf` → `\hfill $\Box$` throughout), a definition expanded in words, and
  the Proposition 7 typo the comment names.
- **Location (D3, `raw/1` locators):** instance 1 — "Theorem 1 b), proof (the paragraph
  beginning 'b) Let G̃ stand for the simply connected cover of G')"; instance 2 — "Theorem 1 b),
  proof, the steps 'Hence … is equal to the product … Finally … coincides with the W-invariants'".
- **Class argument (D3):**
  - **Instance 2, FLAW-0005 (theorem applied without its hypotheses), severity gap,
    lean_catchable YES-KERNEL.** The final step of the v2 proof takes W-invariants of the
    T×G_m-equivariant cohomology *as decomposed* into a product over components; that requires the
    decomposition — the identifications of the components with the neutral one — to be compatible
    with the W-action. The v2 text establishes only T×G_m-equivariance of those identifications
    and then uses the decomposition as if W acted componentwise through them. The corrected proof
    supplies exactly the missing hypothesis: an identification that is W-equivariant. The entry's
    clause: "A named theorem is invoked on an object for which one of its hypotheses is … not
    established (gap)". Gap, not fatal: the hypothesis is true for the identification the corrected
    proof constructs, and the statement stands (the authors: "All the statements are unchanged").
    Lean-catchable: the equivariance argument cannot be supplied for the step as written
    (`YES-KERNEL` per the entry).
  - **Instance 1, FLAW-0011 (true theorem with invalid proof; composite), severity gap
    (inherits), lean_catchable per the embedded flaw (YES-KERNEL).** The statement is documented
    true by the authors (unchanged in v3, re-proved there), and the offered v2 proof carries the
    flaw above — the entry's definition.
  - Other classes considered: FLAW-0004 (unjustified step, "Hence") — close, but the step is not
    merely unjustified: it relies on a specific property (W-compatibility of the identification)
    that the corrected proof had to construct, which is FLAW-0005's shape. FLAW-0003
    (circularity) — no. FLAW-0008 (statement drift) — no: the statement did not change.
- **Statement status (D3):** `value: true`, `method: "source-document"`, `reference: S-0027` —
  "All the statements are unchanged from the previous version", and v3 re-proves part b).
- **Proof status:** `invalid` (S1.1 by construction; the authors document the mistake in the v2
  proof of Theorem 1 b)).
- **Human-authored (D2.6):** two human authors; no mention of AI assistance in the v3 sources
  (helper scan: none) or in any comment; the error was pointed out by a named third party in the
  authors' acknowledgement (quoted above).
- **Licence (D5, SD16):** v2 carries `http://arxiv.org/licenses/assumed-1991-2003/` on its
  abstract page — arXiv's assumed non-exclusive licence to distribute, equivalent to the
  non-exclusive-distrib licence — so the original's storage is **`external`**: no copy is
  committed; the manifest pins the bytes by hash (`original-0707.3799v2.tex.gz`, 40,240 B,
  `3f9593e1…3053`). v3 carries CC BY 4.0. Recorded in S-0026 and S-0027 before ingestion.
- **What this item is and is not:** a real, human-authored paper (the comment says "to appear in
  Moscow Math J"; the v3 footnote refers to "the published version") whose authors document a
  mistake in one proof and re-prove the unchanged statement. Nothing here assesses any other part
  of the paper (SD3); the claim of record is Theorem 1 b) alone.
- **Derived canonical artifact (S1.3):** not yet produced; the location re-expression argument is
  added here when it is. Note for S1.3: the source is a single TeX file with macros (`\Gr`, `\GO`,
  `\Gm`, `\t`, …) that the transcription must expand or carry.
- **Written by:** the executor (Claude Fable 5.1), Session 11; a reading of public documents, not
  a verdict on the paper (SD3).
