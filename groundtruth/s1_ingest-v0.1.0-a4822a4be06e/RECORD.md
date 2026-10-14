# RECORD — Route A item 16 (candidate 17, second documented result): arXiv:1206.1893 v6, Proposition 35 "Martingale concentration" (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** as item 15 (arXiv `1206.1893`, v6 → v7, Session 17); the same
  original bytes and the same S-entries (S-0046, S-0047); a different claim of record.
- **The claim of record (quoted from v6, Section 4, Proposition 35, TeX label `azuma`):**
  "(Martingale concentration) Let ξ₁, …, ξₙ be independent complex random variables of mean zero
  and |ξ_i| = n^{o(1)} with overwhelming probability for all i. Let α₁, …, αₙ > 0 be positive real
  numbers, and for each i = 1, …, n, let c_i(ξ₁, …, ξ_{i−1}) be a complex random variable
  depending only on ξ₁, …, ξ_{i−1} obeying the bound |c_i(ξ₁, …, ξ_{i−1})| ≤ α_i with overwhelming
  probability. Define Y := Σ_{i=1}^n c_i(ξ₁, …, ξ_{i−1}) ξ_i. Then |Y| ≪ n^{o(1)} (Σ_{i=1}^n
  α_i²)^{1/2} with overwhelming probability." Introduced as "the following useful corollary"
  (of Proposition 34). The proof (three sentences): C_i(ξ) = |c_i(ξ₁, …, ξ_{i−1}) ξ_i|; by the
  assumptions C_i(ξ) ≤ n^{o(1)} α_i with overwhelming probability; "Now apply Proposition 34 with
  a suitable choice of parameter λ = n^{o(1)}."
- **Status document (SD12; D2.2):** the v7 comment ("indicates a workaround so as not to impact
  the remainder of the paper") and the v7 remark: "for the purposes of establishing Proposition
  35 (which is the only place in this paper where Proposition 34 is used), it is better to
  proceed as follows. Firstly, one should impose an additional mild moment hypothesis such as
  𝔼ξ_i² = O(n^{O(1)}), which is true in all applications of interest. Then one can define Y′ to
  be the same as Y but with each ξ_i replaced by a mean zero modification ξ′_i … and c_i …
  similarly replaced by c′_i … that is bounded by α_i almost surely and agrees with c_i with
  overwhelming probability." Proposition 35 is applied at eight places later in the paper (the
  executor's count of `\ref{azuma}` in v6).
- **Location (D3, `raw/1` locator):** "Proposition 35, proof, the last sentence: the application
  of Proposition 34".
- **Class argument (D3):** **FLAW-0005 (theorem applied without its hypotheses), severity gap,
  lean_catchable YES-KERNEL.** The proof's only non-trivial step invokes Proposition 34, which the
  authors state is incorrect; read as in item 13, the invoked result itself is the failed
  premise. Gap, not fatal: the authors do not say Proposition 35 is false — they say how to
  establish it under an added hypothesis — so the truth is open and the damage is to the
  argument. Other classes considered: FLAW-0012 — not recorded, the document does not call the
  statement false; FLAW-0011 — not recorded, the document does not re-prove the statement as
  written (the workaround adds a hypothesis).
- **Statement status (D3; ADR-0001 as amended 2026-10-13):** `value: null`,
  `method: "undetermined"`, `reference: S-0047` — the statement as written is neither re-proved
  nor disproved: the workaround establishes it "under an additional mild moment hypothesis …
  which is true in all applications of interest". The third undetermined-truth item in the
  corpus (after items 3 and 13); `correct` is `false` by derivation.
- **Proof status:** `invalid` (S1.1 by construction; the proof rests on the incorrect
  proposition, as the authors say).
- **Human-authored, licence, provenance:** as item 15 (S-0046, S-0047; `external`).
- **What this item is and is not:** the same paper's second documented result, recorded as its
  own item because it is a distinct claim of record whose status differs from Proposition 34's
  (undetermined rather than false) and because it is the result the rest of the paper uses; the
  dependence between items 15 and 16 (one note, one incorrect proposition) is for the S3
  calibration ADR's effective-N rule (ADR-0004 D6), and the S3 obligation for undetermined items
  (STATE standing rulings) applies to this one. Nothing here assesses the paper's main theorems
  (SD3).
- **Derived canonical artifact (S1.3):** not yet produced.
- **Written by:** the executor (Claude Fable 5.1), Session 17; a reading of public documents,
  not a verdict on the paper (SD3).
