# RECORD — Route A item 19 (candidate 21, first documented result): arXiv:1209.2022 v2, Proposition 3.1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1209.2022`; original = v2 (23 May 2013; v1 is the
  2012 first version); correcting version = v3 (6 December 2024, inside the ADR-0003 D8 window);
  examined Session 18, 2026-10-14 (access 19:51:37Z). One author; Quantum Algebra (math.QA); a
  seven-page note with no journal reference on arXiv, though v3's remark speaks of "the
  published version of this work" (not identified, not retrieved). The same candidate yields a
  second item (Theorem 3.2), recorded in its own directory.
- **Status document (SD12; D2.2):** the v3 revision comment — "In this version, Proposition 3.1,
  which discussed the equality of the unitary center and the ordinary center, has been removed.
  While these centers are equivalent as braided fusion categories, the unitary center is
  generally a non-replete subcategory. The proof of Theorem 3.2 has been revised to avoid
  reliance on this proposition" — with v3 itself: its acknowledgement ("grateful to Tian Lan for
  identifying an error in Proposition 3.1 of the previous version") and the unnumbered remark
  that stands where the proposition stood (quoted in full in S-0053). S-entries: S-0052 (the
  original, v2), S-0053 (the corrected version, its comment and remark).
- **Provenance and digests:** abs v2 38,028 B `78837805…be31`; abs v3 38,328 B `093d4818…8de5`;
  e-print v2 7,300 B `21f63ae0…60ed` (one gzipped TeX file) — the original, **not committed**;
  e-print v3 7,668 B `59b91928…89a7`. Full digests in S-0052 and S-0053.
- **The claim of record (quoted from v2, Subsection 3.1, Proposition 3.1, TeX label `centro
  igual a centro unitario`):** with the unitary center defined just before it — "If 𝒞 is a
  unitary fusion category the unitary center 𝒵*(𝒞) is defined as the full tensor subcategory of
  𝒵(𝒞), where (X, c_{−,X}) ∈ 𝒵*(𝒞) if and only if c_{W,X}: W ⊗ X → X ⊗ W is unitary for all
  W ∈ 𝒞" — the proposition reads "Let 𝒞 be a unitary fusion category then 𝒵*(𝒞) = 𝒵(𝒞)." Its
  proof, whole: "Let (X, c_{−,X}) be an object in 𝒵(𝒞). By [GHR, Proposition 5.24] or [Mueg2,
  Theorem 6.4], the inclusion functor 𝒵*(𝒞) ⊆ 𝒵(𝒞) is a tensor equivalence. Therefore, there is
  an object (Y, c_{−,Y}) in 𝒵*(𝒞) and an isomorphism f: (X, c_{−,X}) → (Y, c_{−,Y}) in 𝒵(𝒞). By
  Remark 2.1 there exists a unitary arrow u: (X, c_{−,X}) → (Y, c_{−,Y}). Hence, for every
  W ∈ 𝒞, c_{W,X} = (u ⊗ id_W)* ∘ c_{W,Y}(id_W ⊗ u), so c_{W,X} is a unitary arrow and
  (X, c_{−,X}) ∈ 𝒵*(𝒞)." Remark 2.1 is the polar decomposition in a C*-category: an isomorphism
  f: X → Y factors as f = ua with a self-adjoint and u unitary.
- **The notice's locating and describing sentences (quoted):** "Proposition 3.1 … has been
  removed. While these centers are equivalent as braided fusion categories, the unitary center is
  generally a non-replete subcategory"; "an error in Proposition 3.1 of the previous version";
  "in general, 𝒵*(𝒞) ≠ 𝒵(𝒞); in other words, 𝒵*(𝒞) is not a replete subcategory of 𝒵(𝒞)";
  "Tian Lan provided a counterexample using the category Hilb_G of finite-dimensional Hilbert
  spaces graded over G, where we have URep(G) ⊊ Rep(G) ⊂ 𝒵(Hilb_G) and URep(G) ⊂ 𝒵*(Hilb_G), yet
  the inclusion URep(G) ⊊ Rep(G) is not a replete subcategory".
- **Diff summary (executor's reading of the TeX diff v2 → v3, 410 lines):** most of the diff is
  reflowed prose, light copy-editing and a reformatted bibliography. The substantive changes are
  the acknowledgement sentence, the deletion of Proposition 3.1 and its proof, the remark in its
  place, and a new proof of Theorem 3.2 (its statement unchanged; see item 20). The definition of
  the unitary center is unchanged.
- **Location (D3, `raw/1` locator):** "Proposition 3.1, statement and proof"; the notice locates
  the error to the proposition and establishes that the statement is false, without naming the
  failing step.
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition; the result is withdrawn), lean_catchable YES-KERNEL.** The statement is false — the
  author says so and prints a counterexample attributed to Tian Lan (a non-unitary complex
  representation of G is an object of 𝒵(Hilb_G) whose half-braiding is not unitary, though it is
  isomorphic to one whose is) — and the offered proof is a four-sentence argument from an
  equivalence of categories and a polar decomposition, the shape the entry describes: the nearby
  true statement is that the inclusion 𝒵*(𝒞) ⊆ 𝒵(𝒞) is an equivalence, which is what the cited
  results give and what the comment affirms ("equivalent as braided fusion categories").
  **Executor's reading, labelled as such and not the author's:** the step "By Remark 2.1 there
  exists a unitary arrow u: (X, c_{−,X}) → (Y, c_{−,Y})" takes the unitary factor of the polar
  decomposition of f, which the remark gives as a morphism X → Y in 𝒞, to be a morphism of the
  center — compatible with the half-braidings — which nothing establishes; the final identity
  needs exactly that compatibility. The v3 proof of Theorem 3.2 avoids the point because there
  the half-braidings are a braiding c of 𝒞, whose naturality makes every morphism of 𝒞
  compatible with them. The document does not name this step.
  Other classes considered: FLAW-0005 (Remark 2.1 applied for a conclusion it does not have) is
  the executor's reading of the mechanism and is not recorded, because the notice does not state
  it; FLAW-0010 — the cited equivalence results exist and are reaffirmed in v3; FLAW-0008
  (statement drift between "equivalent" and "equal") describes the gap between the true and the
  false statement, not a flaw record of its own in the 0.1.0 taxonomy's informal sense.
- **Statement status (D3):** `value: false`, `method: "counterexample"`, `reference` = the v3
  remark's counterexample Hilb_G with URep(G) ⊊ Rep(G) (S-0053) — the status document disproves
  the statement with a counterexample, so the counterexample method applies (as for item 9).
- **Proof status:** `invalid` (S1.1 by construction; the author removes the proposition).
- **Human-authored (D2.6):** one human author; no AI-assistance mention (the scanner reports no
  hit on either version); the error was identified by a named mathematician, per the author.
- **Licence (D5, SD16):** v2 and v3 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1209.2022v2.tex.gz`, 7,300 B, `21f63ae0…60ed`). Recorded in S-0052 and S-0053
  before ingestion.
- **What this item is and is not:** a real, human-authored note whose author, eleven years on,
  removes a proposition, explains with a counterexample why it is false, and re-proves the
  theorem that depended on it. Nothing here assesses the note's two theorems (SD3); the claim of
  record is Proposition 3.1 as stated in v2.
- **Derived canonical artifact (S1.3):** not yet produced; the proposition and its proof are a
  dozen source lines and depend on the definitions of the center and the unitary center, Remark
  2.1, and the cited equivalence [GHR, Prop. 5.24] / [Mueg2, Thm. 6.4].
- **Written by:** the executor (Claude Fable 5.1), Session 18; a reading of public documents,
  not a verdict on the paper (SD3).


## S1.3 source and status review — 2026-10-20 UTC

Freshly retrieved S-0052/S-0053 abstract pages and e-prints match the historical hashes;
URLs, times, HTTP status and complete digests are in `analysis/s1-3/retrieval.json`.
The executor read the original affected statement and entire offered proof, its preamble,
unitary-fusion and center definitions, Remark 2.1 and bibliography; also the v3 revision
comment, acknowledgement and replacement remark. Unlike a notice that merely says a result
was corrected, the v3 remark explicitly supplies a counterexample attributed to Tian Lan.
It supports inheritance of the existing false/counterexample and invalid-proof statuses.
This is a source-status reading (SD3), not an independent proof of the counterexample.
The existing distinction between source testimony and the executor's proposed failure
mechanism remains intact; no narrower proof-step flaw is added.

This parent is suitable for one engineering slice because its original statement and proof
have short, unambiguous source boundaries and documented status. It is not a new admission
or representative calibration sample. Rights remain external/digest-only under SD16 and
ADR-0003 D5: the non-exclusive distribution grant is to arXiv, not this repository.
No proof or complete source-context file is added to Git; existing attribution/status/locator
metadata is retained. The complete decompressed source,
including non-UTF-8 bibliography bytes, is required external context under ADR-0006.
The public recipe is `groundtruth/derivations/1209.2022v2-proposition3.1.json`.
The resulting child and completed verification are recorded in the later evidence commit.

The post-producer run completed: child `s1_derive-v0.1.0-c08f9e3ec092`, engine commit
`49e47237924eddf0c39c24aefad0eb9aa02ee8b3`. Its RECORD and source map document exact source
ranges, retained context and the statement-wide step-0 `[0,79)` flaw anchor. Both actual-byte
and parent/child checks passed on two identical independent regenerations. Receipt and
commands: `analysis/s1-3/real-result.json`; full evidence index: `analysis/s1-3/README.md`.
