# RECORD — Route A item 11 (candidate 14): arXiv:1112.6107 v2, Theorem 1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1112.6107`; original = v2 (18 December 2023, itself
  a revision that split the 2011 paper into parts, not a correction notice); correcting version
  = v3 (13 April 2025, inside the ADR-0003 D8 window); examined Session 16, 2026-10-14 (access
  04:53:16Z). One author; Dynamical Systems (math.DS).
- **Status document (SD12; D2.2):** the v3 revision comment — "Correction of the last statement
  in Theorem 1, proofs are not affected. Details added, writing improved." — with v3 itself.
  S-entries: S-0040 (the original, v2), S-0041 (the corrected version and its comment).
- **Provenance and digests:** abs v2 39,032 B `871136a4…5742`; abs v3 38,962 B `1171502f…09a0`;
  e-print v2 46,945 B `f1bfb2c3…4da4` (gzipped tar with a figure) — the original, **not
  committed**; e-print v3 55,742 B `dd2e3d03…34c2`. Full digests in S-0040 and S-0041.
- **The claim of record (quoted from v2, Theorem 1, TeX label `coding`):** "Let 𝒬 be a component
  of a stratum of quadratic or abelian differentials. Then there exists a topologically
  transitive subshift of finite type (Ω, σ), a σ-invariant dense Borel set 𝒰 ⊂ Ω containing all
  normal sequences, a suspension (X, Θ^t) of σ over 𝒰, given by a positive bounded continuous
  roof function on 𝒰, and a finite-to-one semi-conjugacy Ξ: (X, Θ^t) → (𝒬, Φ^t) which maps the
  space of σ-invariant Borel probability measures on 𝒰 continuously onto ℳ_inv(𝒬)."
- **The notice's locating and describing sentence (quoted):** "Correction of the last statement
  in Theorem 1, proofs are not affected."
- **Diff summary (executor's reading of the TeX diff v2 → v3):** the last clause becomes "which
  maps the space of Θ^t-invariant Borel probability measures on X continuously onto ℳ_inv(𝒬)":
  the map Ξ is defined on the suspension X, so what it pushes forward are Θ^t-invariant measures
  on X, not σ-invariant measures on the base 𝒰; the introduction adds that Ξ is a partial
  semi-conjugacy, "not surjective". The remaining 3,700 diff lines are the announced expository
  improvements; no other result is named by the notice.
- **Location (D3, `raw/1` locator):** "Theorem 1, statement, last clause".
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition), lean_catchable YES-KERNEL.** The statement as written asserts a property of the
  wrong object (measures on 𝒰 under σ, rather than measures on X under Θ^t), and the paper's
  proof — unchanged in v3, by the author's account — establishes the corrected clause: a proof of
  the nearby true statement offered for a different one, the entry's shape. The misstatement is
  at the level of the conclusion, so there is no hidden use site to record. Lean-catchable: the
  conclusion as written is not what the argument produces (`YES-KERNEL` per the entry).
  Other classes considered: FLAW-0008 (statement drift) — the taxonomy reserves it for formal
  pairs, though this is the informal analogue of exactly that: the stated theorem differs from
  the proved one; FLAW-0001 — no sign or constant is involved.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0041` —
  read from "Correction of the last statement": a statement that needed correcting was
  incorrect as written. **Caveat:** the author does not say "false"; whether the v2 clause is
  false or merely ill-posed (Ξ acts on X, not on 𝒰) is not settled by the document, and if Al
  prefers the stricter reading the value would be `null` / `undetermined`. The executor records
  `false` because the corrected clause names a different space, so the original clause cannot be
  what the proofs establish.
- **Proof status:** `invalid` (S1.1 by construction: as an argument for the statement as written,
  the proof does not establish it).
- **Human-authored (D2.6):** one human author; no AI-assistance mention (the scanner's "llm" hit
  is the TeX label `fillmore`, checked, a false positive of the kind seen on candidates 9, 12 and
  15); the finder of the error is unstated.
- **Licence (D5, SD16):** v2 and v3 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1112.6107v2.tar.gz`, 46,945 B, `f1bfb2c3…4da4`). Recorded in S-0040 and
  S-0041 before ingestion.
- **What this item is and is not:** a real, human-authored paper whose author corrects a
  misstated conclusion of its main theorem with the proofs unchanged. Nothing here assesses the
  paper's results beyond that clause (SD3); the claim of record is Theorem 1 as stated in v2.
- **Derived canonical artifact (S1.3):** not yet produced. Note for S1.3: the proof of Theorem 1
  spans most of a 35-page paper; the derived artifact's scope is an S1.3 decision.
- **Written by:** the executor (Claude Fable 5.1), Session 16; a reading of public documents,
  not a verdict on the paper (SD3).
