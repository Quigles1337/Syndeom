# RECORD — Route A item 31 (candidate 27): arXiv:1403.4485 v3, Proposition 5.1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1403.4485`; original = v3 (7 April 2015; published as
  Int. Math. Res. Not. 2015, 13379–13405, DOI 10.1093/imrn/rnv090, per the abstract page; v2 of
  2014 and v3's own "proof of Proposition 7.2 corrected" are outside the ADR-0003 D8 window and
  not harvested); correcting version = v4 (12 June 2023, inside the window); examined Session 20,
  2026-10-14 (access 21:28:34Z). One author; Algebraic Topology (math.AT).
- **Status document (SD12; D2.2):** the v4 revision comment — "20 pages; incorrect degree shift
  in Proposition 5.1 corrected, some typos fixed" — with v4 itself, whose acknowledgement adds
  "and particularly Sergio Chaves for pointing out previously incorrect degrees shifts in
  Proposition 5.1". S-entries: S-0064 (the original, v3), S-0065 (the corrected version and its
  comment).
- **Provenance and digests:** abs v3 41,241 B `46854f52…47f1`; abs v4 41,177 B `476f7125…3472`;
  e-print v3 24,763 B `52d5f949…f851` (one gzipped TeX file) — the original, **not committed**;
  e-print v4 25,056 B `126db358…bfc5`. Full digests in S-0064 and S-0065.
- **The claim of record (quoted from v3, Section 5 "The equilateral case", Proposition 5.1, TeX
  label `thm:HTX-equilateral`):** "Let r = 2m+1 and ℓ = (1,…,1) ∈ ℝ^r. Then H_T^*(X_{a,b}(ℓ)) ≅
  ⊕_{|J|<m} R[|J|d_V] ⊕ 𝒦_m[m d_V] ⊕ 𝒦_{m+2}[(m+1)d_V − d_U + 1] ⊕ ⊕_{|J|>m+1} R[|J|d_V − d_U −
  1]. In particular, syzord H_T^*(X_{a,b}(ℓ)) = m." (R the polynomial ring of the torus T, 𝒦_k
  the Koszul syzygy modules, [·] the degree shift; the big polygon space X_{a,b}(ℓ) and d_U, d_V
  as the paper defines them.) The proof computes the kernel and cokernel of a map ι^T_* in a
  short exact sequence, identifies the restriction of ι^T_* with a Koszul differential — "is
  essentially the map δ̃_{r−j} from (eq:koszul-diff-homology)" — obtains "ker ι^T_* ≅ ⊕_{|J|<m}
  R[−|J|d_V − d_U] ⊕ 𝒦_{m+2}[−m d_V − d_U + 2]" with "The degree shifts follow from
  (eq:dim-VJ-WJ) and the convention that the Koszul syzygies are generated in degree 0", shows the
  extension splits, and reads off the syzygy order.
- **The notice's locating and describing sentences (quoted):** "incorrect degree shift in
  Proposition 5.1 corrected"; "previously incorrect degrees shifts in Proposition 5.1".
- **Diff summary (executor's reading of the TeX diff v3 → v4, 272 lines):** in the statement the
  one summand 𝒦_{m+2}[(m+1)d_V − d_U + 1] becomes 𝒦_{m+2}[(m+2)d_V − 2d_U − 1]; everything else
  in the statement, including the conclusion syzord = m, is unchanged. In the proof the degree
  bookkeeping is redone: the restriction of ι^T_* is "essentially the map δ̃_j …, up to a degree
  shift by |W_J| − |e_J| = −(j+1)d_U, compare (eq:dim-VJ-WJ). (Note that we grade homology
  negatively.) Set s̄ = −(m+1)d_U and s = s̄ − 2b(m+1) = −(m+1)d_V."; the Koszul summand of the
  kernel becomes 𝒦_{m+2}[s̄ − 2b(m−1)] = 𝒦_{m+2}[−(m−1)d_V − 2d_U]; the sentence about shifts
  following from the convention is dropped. The splitting argument and the syzygy conclusion are
  unchanged; the Section 4 lemma feeding this ("up to a degree shift by −2b(r−k)") gets explicit
  shifts; the remaining hunks are preamble, a citation and typography. The AI-assistance
  scanner's hit "llm" is the bibliography entry "Stillman" (Grayson–Stillman, Macaulay2) — not
  an AI mention.
- **Location (D3, `raw/1` locators):** instance 1 — "Proposition 5.1, statement: the summand
  𝒦_{m+2}[(m+1)d_V − d_U + 1]"; instance 2 — "Proposition 5.1, proof: the degree bookkeeping
  (the unstated shift of the Koszul identification and the kernel's 𝒦_{m+2}[−m d_V − d_U + 2])".
- **Class argument (D3):** **FLAW-0001 (sign or constant error), sub-variant propagated, severity
  fatal, lean_catchable YES-KERNEL** for the statement: a wrong shift constant in a graded
  isomorphism is a false statement (a non-zero graded module is not isomorphic to a different
  shift of itself), and the author calls the shift "incorrect"; the error propagates from the
  proof's bookkeeping to the stated conclusion — the entry's propagated variant, as for item 4.
  **FLAW-0001, sub-variant local, severity repairable, lean_catchable YES-KERNEL** for the
  proof: the shift of the Koszul identification is left implicit and the kernel's summand carries
  the resulting wrong constant, while the structural argument (the splitting, the syzygy order) is
  right and unchanged — the fix supplies the constants and nothing else.
  Other classes considered: FLAW-0012 — the statement is false, but the entry's "convincing proof
  of a nearby true statement" shape is not the author's description, which is a wrong constant;
  FLAW-0004 — the shift claim is justified (by the convention sentence), just wrongly.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0065`
  ("incorrect degree shift … corrected"). No `undetermined` alternative is named: the author
  replaces one constant by another in the same statement, so the v3 statement is false if the v4
  one is true, as for item 4; the record is not added to the escalated list of
  "corrected"-wording items, whose doubt is whether the old statement was false or merely
  unproved.
- **Proof status:** `invalid` (S1.1 by construction; the author corrects the proof's shifts).
- **Human-authored (D2.6):** one human author; no AI-assistance mention (the scanner's one hit
  checked, a citation key); the error was pointed out by a named mathematician, per the
  acknowledgement.
- **Licence (D5, SD16):** v3 and v4 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1403.4485v3.tex.gz`, 24,763 B, `52d5f949…f851`). Recorded in S-0064 and
  S-0065 before ingestion.
- **What this item is and is not:** a real, human-authored, published paper whose author, eight
  years on, corrects a degree shift in one summand of a computed equivariant cohomology module
  after a named reader pointed it out; the structural conclusion (the syzygy order) is unchanged.
  Nothing here assesses the rest of the paper (SD3); the claim of record is Proposition 5.1 as
  stated in v3.
- **Derived canonical artifact (S1.3):** not yet produced; the proposition and its proof are
  about sixty source lines and depend on the paper's Lemma `thm:extension-HTX`, Proposition
  `thm:iota-V-W` and the Koszul-complex conventions of Section 4.
- **Written by:** the executor (Claude Fable 5.1), Session 20; a reading of public documents,
  not a verdict on the paper (SD3).
