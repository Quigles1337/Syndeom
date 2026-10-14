# RECORD — Route A item 4 (candidate 5): arXiv:0908.3213 v4, Theorem 4.4 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A (arXiv revision comments); arXiv `0908.3213`; original =
  v4 (19 June 2012); correcting version = v5 (29 July 2024, inside the ADR-0003 D8 window);
  examined Session 13, 2026-10-14 (access 02:52:55Z–02:55:06Z). Three authors; Rings and Algebras
  (math.RA) and Differential Geometry (math.DG). **Not this item:** v4 itself corrected Theorem 3.2's
  proof and Theorem 3.4's statement in 2012 — outside the window, so that correction is not
  harvested; the window bounds the correcting version's date (D8 precision).
- **Status document (SD12; D2.2):** the v5 revision comment — "v5: The statement of Theorem 4.4
  (assertions (1) and (2)) was fixed and the proof was modified accordingly. Some arguments were
  clarified" — together with the corrected version v5. S-entries: S-0030 (the original, v4),
  S-0031 (the corrected version and its comment).
- **Provenance and digests:** abs v4 40,761 B `6bcb8355…4c37`; abs v5 40,827 B `e69afae7…f877`;
  e-print v4 34,662 B `30978332…c9d5` (gzipped tar) — the original, **not committed**; e-print v5
  71,750 B `16f9d1c2…95f5`. Full digests in S-0030 and S-0031.
- **The notice's locating and describing sentence (quoted):** "The statement of Theorem 4.4
  (assertions (1) and (2)) was fixed and the proof was modified accordingly."
- **Diff summary (executor's reading of the TeX diff v4 → v5):** in Theorem 4.4's statement,
  assertions (1) `(aff(ℂ) × ℝ², J)` and (2) `(s₁, J₁)` list the complex structure
  `Jf₁ = f₂, Je₁ = −e₂ + e₃, Je₂ = e₁ + e₄, Je₃ = e₄` in v4 and `Jf₁ = f₂, Je₁ = e₂ − e₃,
  Je₂ = −e₁ − e₄, Je₃ = −e₄` in v5 (every sign on e₂, e₃, e₄ reversed); the brackets listed with
  them are unchanged. In the proof, the normalization displayed with TeX label `base` changes
  from `f̃₁ = (1/√(a²+b²))(a f₁ + b f₂)` to `f̃₁ = (1/(a²+b²))(a f₁ + b f₂)`, with the consequence
  that `[f̃₁, f̃₂] = e₃/(a²+b²)` (v4 had asserted `[f̃₁, f̃₂] = [f₁, f₂]`) and a rescaled `ẽ₃`; the
  case (ii) argument is rewritten. The rest of the diff is typographical.
- **Executor's confirming computation (recorded as such):** with the brackets Theorem 4.4 lists
  for aff(ℂ) × ℝ² (`[f₁,e₁] = [f₂,e₂] = e₁`, `[f₁,e₂] = −[f₂,e₁] = e₂`, e₃, e₄ central) and for s₁
  (the same plus `[f₁,f₂] = e₃`), both structures square to −1, but the v4 structure fails the
  abelian condition `[Jx, Jy] = [x, y]` on the pairs (f₁,e₁), (f₁,e₂), (f₂,e₁), (f₂,e₂) — for
  instance `[Jf₁, Je₁] = [f₂, −e₂ + e₃] = −e₁ ≠ e₁ = [f₁, e₁]` — while the v5 structure satisfies it
  on every pair (a six-line linear-algebra script, Session 13). So the v4 statement lists, as
  abelian complex structures, structures that are not abelian: the statement is false as written.
- **Location (D3, `raw/1` locators):** instance 1 — "Theorem 4.4, statement, assertions (1) and
  (2): the listed complex structures J"; instance 2 — "Theorem 4.4, proof, the normalization
  displayed with TeX label `base` and the bracket it feeds".
- **Class argument (D3):**
  - **Instance 1, FLAW-0001 (sign or constant error), sub-variant `propagated`, severity fatal,
    lean_catchable YES-KERNEL.** The entry's definition: "An arithmetic or algebraic step carries
    a wrong sign … **propagated** (the error changes what follows, up to the conclusion)". The sign
    errors sit in the conclusion itself — the listed J — and make the stated classification false
    (the computation above); fatal because "the conclusion is false" (the entry's fatal end).
    Lean-catchable: a faithful formalization of the abelian condition for the v4 structure does
    not close (`YES-KERNEL` per the entry).
  - **Instance 2, FLAW-0001, sub-variant `local`, severity repairable, lean_catchable
    YES-KERNEL.** The normalization constant in the proof is wrong (`1/√(a²+b²)` where
    `1/(a²+b²)` is needed for the asserted bracket), and the proof asserts the correct consequence
    anyway ("we may assume A = I … the Lie bracket becomes …") — the entry's **local** variant,
    "the wrong expression is written but the correct consequence is asserted anyway"; the fix is
    the rescaling v5 makes, mechanical and argument-preserving: repairable.
  - Other classes considered: FLAW-0012 (false theorem with convincing proof) — the statement is
    false, but the mechanism is a sign slip in the listed structures, not a dropped hypothesis
    hidden in a nearby true argument; FLAW-0001 propagated is the closer fit and the executor's
    computation shows the falsity directly. FLAW-0008 — no formal pair exists.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0031` —
  the status document replaces the listed structures ("was fixed"); the executor's computation
  above confirms that the v4 structures are not abelian, so the v4 statement is false as written.
- **Proof status:** `invalid` (S1.1 by construction; the authors document the error and modify
  the proof).
- **Human-authored (D2.6):** three human authors; no mention of AI assistance in the v5 sources
  (helper scan: none) or in any comment; the finder of this error is unstated (the 2012 Theorem
  3.4 error was attributed to a named colleague in the acknowledgement; that is the other
  correction).
- **Licence (D5, SD16):** v4 and v5 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-0908.3213v4.tar.gz`, 34,662 B, `30978332…c9d5`). Recorded in S-0030 and S-0031
  before ingestion.
- **What this item is and is not:** a real, human-authored classification paper whose authors
  document an error in one theorem's statement and proof and fix it twelve years after the
  original. Nothing here assesses the paper's other results (SD3); the claim of record is
  Theorem 4.4 as stated in v4.
- **Derived canonical artifact (S1.3):** not yet produced; the location re-expression argument is
  added here when it is. Note for S1.3: the statement is a long enumerated list; the derived
  artifact's step 0 will carry the whole statement and the proof's case analysis becomes the steps.
- **Written by:** the executor (Claude Fable 5.1), Session 13; a reading of public documents, not
  a verdict on the paper (SD3).
