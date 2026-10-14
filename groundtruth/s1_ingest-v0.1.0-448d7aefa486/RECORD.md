# RECORD — Route A item 8 (candidate 10): arXiv:1010.2391 v9, Lemma A.3 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1010.2391`; original = v9 (17 January 2020);
  correcting version = v10 (5 October 2023, inside the ADR-0003 D8 window); examined Session 14,
  2026-10-14 (access 03:39:45Z). One author; Dynamical Systems (math.DS); journal reference on the
  abstract page: Discrete and Continuous Dynamical Systems Series A 32(8), 2607–2651, 2012. **Not
  this item:** v9 itself corrected Lemmas 2.3 and A.5 in January 2020 — outside the window (D8
  precision), not harvested.
- **Status document (SD12; D2.2):** the v10 revision comment — "minor revision, correcting
  mistakes in formulation of Lemma A.3 (which are also present in the Journal paper): center x of
  neighborhood must be in C¹, which is the case for the main theorem. Further clarification in
  proof of Lemma A.3" — with v10 itself. S-entries: S-0036 (the original, v9), S-0037 (the
  corrected version and its comment).
- **Provenance and digests:** abs v9 42,878 B `9af705af…d274`; abs v10 43,010 B `628736d9…2dc1`;
  e-print v9 53,403 B `d64ff4f0…02b6` (gzipped tar) — the original, **not committed**; e-print
  v10 53,362 B `853ca171…ac40`. Full digests in S-0036 and S-0037.
- **The notice's locating and describing sentences (quoted):** "correcting mistakes in
  formulation of Lemma A.3 …: center x of neighborhood must be in C¹".
- **Diff summary (executor's reading of the TeX diff v9 → v10):** Lemma A.3 (TeX label `thm:flip`,
  "Local EC Lipschitz continuity") changes its statement from "For all x ∈ D^{0,1} there exists a
  neighborhood U(x) ⊆ D^{0,1} and a constant K_x > 0 such that for all y₁ and y₂ ∈ U(x) …" to "For
  all x ∈ D¹ there exists …". The v9 proof opened "We prove the Lipschitz continuity first for y₁
  and y₂ from a sufficiently small neighborhood U(x) ∩ D¹ ⊆ D¹ of x ∈ D¹. … Let x be an element
  of D¹." — assuming x ∈ D¹ although the statement quantified over x ∈ D^{0,1} — and extended by
  density; v10 rewrites the proof to work on a ball B¹_{δ/2}(x) ⊂ D¹ and to extend to the
  D^{0,1}-ball by density, consistently with the new hypothesis. One norm subscript elsewhere is
  corrected (not a documented error, not used).
- **Location (D3, `raw/1` locators):** instance 1 — "Lemma A.3, statement"; instance 2 — "Lemma
  A.3, proof, its opening and the continuity-of-the-mean-value argument".
- **Class argument (D3):**
  - **Instance 1, FLAW-0012 (false theorem with convincing proof), severity fatal (by
    definition), lean_catchable YES-KERNEL.** The author says the formulation was mistaken and
    that the center "must be in C¹"; the v9 statement claims the lemma for every x in the larger
    space D^{0,1}, and the v9 proof is the argument for x ∈ D¹ — the entry's shape, "the proof
    mirrors the argument for a nearby true statement, hiding the point where a dropped
    hypothesis is used" (here the hypothesis is not even hidden: the proof writes "Let x be an
    element of D¹" against a statement that does not grant it). Lean-catchable: a sorry-free proof
    of the v9 statement cannot exist if it is false (`YES-KERNEL` per the entry).
  - **Instance 2, FLAW-0005 (theorem applied without its hypotheses), severity fatal,
    lean_catchable YES-KERNEL.** The proof's mean-value/continuity argument is applied at a center
    x that the statement does not require to be C¹ — the hypothesis is not granted by the
    statement and, per the author, is required; the entry's fatal case (hypothesis false for the
    stated generality).
  - Other classes considered: FLAW-0008 — no formal pair. FLAW-0004 — the step is not hedged; it
    assumes a hypothesis.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0037` —
  the author corrects the formulation and says the C¹ condition "must" hold; read as the author's
  statement that the v9 formulation is incorrect. **Caveat:** the notice gives no counterexample;
  if Al prefers the stricter reading (the general statement withdrawn but not disproved), the
  value would be `null` / `undetermined` instead — stated here so the choice is visible.
- **Proof status:** `invalid` (S1.1 by construction; the author documents the mistake).
- **Human-authored (D2.6):** one human author; no mention of AI assistance in the v10 sources
  (helper scan: none) or in any comment; the finder of the error is unstated.
- **Licence (D5, SD16):** v9 carries `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` →
  the original's storage is **`external`**: no copy committed; the manifest pins the bytes by hash
  (`original-1010.2391v9.tar.gz`, 53,403 B, `d64ff4f0…02b6`). v10 carries CC BY 4.0. Recorded in
  S-0036 and S-0037 before ingestion.
- **What this item is and is not:** a real, human-authored, published paper whose author
  documents a mis-formulated appendix lemma and fixes it. Nothing here assesses the paper's main
  theorem (which, per the author, satisfies the corrected hypothesis) or any other result (SD3);
  the claim of record is Lemma A.3 as stated in v9.
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 14; a reading of public documents,
  not a verdict on the paper (SD3).
