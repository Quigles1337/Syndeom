# RECORD — Route A item 15 (candidate 17, first documented result): arXiv:1206.1893 v6, Proposition 34 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1206.1893`; original = v6 (16 March 2015, the
  published version — Annals of Probability, DOI 10.1214/13-AOP876 per the comment); correcting
  version = v7 (25 May 2024, inside the ADR-0003 D8 window); examined Session 17, 2026-10-14
  (access 08:15:08Z). Two authors; Probability (math.PR). The same candidate yields a second item
  (Proposition 35), recorded in its own directory.
- **Status document (SD12; D2.2):** the v7 comment — "This version notes an error in Proposition
  34, and indicates a workaround so as not to impact the remainder of the paper" — with v7's
  remark "(Note added after publication.) It has been pointed out to us by Heejune Sheen, and
  independently by Christian Borgs and Karissa Huang, that Proposition 34 is incorrect; the
  random variable Y′ constructed above does not, in fact, obey the hypotheses of Azuma's
  inequality. A version of the proposition can be salvaged by replacing C_i(ξ₁, …, ξ_i) with the
  larger quantity sup_η C_i(ξ₁, …, ξ_{i−1}) (and redefining the events B_i accordingly); …".
  S-entries: S-0046 (the original, v6), S-0047 (the corrected version, its comment and remark).
- **Provenance and digests:** abs v6 47,236 B `fc2a7818…9aa8`; abs v7 46,744 B `5ef405a7…10de`;
  e-print v6 327,296 B `8e7b0c7f…098c` (publisher's TeX, tar) — the original, **not committed**;
  e-print v7 1,642,248 B `a890683f…0d5c`. Full digests in S-0046 and S-0047.
- **The claim of record (quoted from v6, Section 4, Proposition 34, TeX label `azuma0`):** with
  Y = Y(ξ₁, …, ξₙ) "a random variable depending on independent atom variables ξ_i ∈ ℂ" and
  C_i(ξ) := |𝔼(Y | ξ₁, …, ξ_i) − 𝔼(Y | ξ₁, …, ξ_{i−1})| the martingale differences — "For any
  α_i ≥ 0, we have the inequality ℙ(|Y − 𝔼Y| ≥ λ√(Σ_{i=1}^n α_i²)) = O(exp(−Ω(λ²))) +
  Σ_{i=1}^n ℙ(C_i(ξ) ≥ α_i)." Introduced as "a simple variant of [VuNL], Lemma 3.1". The proof
  (eight lines): define B_i as the event that i is the first index with C_i(ξ) ≥ α_i; let Y′
  agree with Y off ⋃B_i and equal 𝔼_{B_i}Y on B_i; "It is clear that Y′ and Y has the same mean
  and ℙ(Y ≠ Y′) ≤ Σ ℙ(C_i(ξ) ≥ α_i). Moreover, Y′ satisfies the condition of Azuma's inequality,
  so ℙ(|Y′ − 𝔼Y′| ≥ λ√(Σα_i²)) ≪ exp(−Ω(λ²)) and the bound follows."
- **The notice's locating and describing sentences (quoted):** "Proposition 34 is incorrect; the
  random variable Y′ constructed above does not, in fact, obey the hypotheses of Azuma's
  inequality."
- **Diff summary (executor's reading of the TeX diff v6 → v7):** the publisher's TeX against the
  authors' TeX, 11,140 lines of formatting; the statement and proof of Proposition 34 (and of
  Proposition 35) are word-for-word the same in both versions; the remark after Proposition 35's
  proof is the one substantive addition and is what the comment announces. The publisher's
  markers `%pr34` and `%pr35` fix the numbering the comment uses.
- **Location (D3, `raw/1` locators):** instance 1 — "Proposition 34, statement"; instance 2 —
  "Proposition 34, proof, the sentence 'Moreover, Y′ satisfies the condition of Azuma's
  inequality'".
- **Class argument (D3):** **FLAW-0012 (false theorem with convincing proof), severity fatal (by
  definition), lean_catchable YES-KERNEL** for the statement: the authors say the proposition
  "is incorrect" and that a *different* proposition (with sup_η C_i in place of C_i) is what can
  be salvaged — the statement as written is not the one that holds; the proof is a short,
  standard-looking truncation argument, the entry's shape. **FLAW-0005 (theorem applied without
  its hypotheses), severity fatal, lean_catchable YES-KERNEL** for the proof: Azuma's inequality
  is invoked for Y′ and, by the authors' account, Y′ "does not, in fact, obey the hypotheses" —
  the entry's definition exactly, and the authors name the step. The same pairing as items 1, 5
  and 8 (a false statement whose proof fails by applying a result outside its hypotheses).
  Other classes considered: FLAW-0011 (true statement, broken proof) — not recorded, because the
  authors say the proposition is incorrect, not merely its proof; FLAW-0001 — no sign or
  constant is involved.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0047`
  ("Proposition 34 is incorrect"; a version "can be salvaged by replacing" the quantity in the
  statement). **Caveat, as for items 8, 10, 11 and 14:** "incorrect" is not the word "false",
  and the note gives no counterexample; the stricter `undetermined` reading is the alternative.
  The executor records `false` because the authors replace the statement's quantity to obtain
  one that holds.
- **Proof status:** `invalid` (S1.1 by construction; the authors locate the error in the proof).
- **Human-authored (D2.6):** two human authors; no AI-assistance mention (the scanner reports no
  hit on either version); the error was pointed out by three named mathematicians, per the
  authors.
- **Licence (D5, SD16):** v6 and v7 carry `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`
  → the original's storage is **`external`**: no copy committed; the manifest pins the bytes by
  hash (`original-1206.1893v6.tar.gz`, 327,296 B, `8e7b0c7f…098c`). Recorded in S-0046 and
  S-0047 before ingestion.
- **What this item is and is not:** a real, human-authored, published paper (Annals of
  Probability) whose authors add a post-publication note that an auxiliary concentration
  proposition is incorrect, say which hypothesis fails, and give a workaround for its one use.
  Nothing here assesses the paper's main results (SD3); the claim of record is Proposition 34 as
  stated in v6.
- **Derived canonical artifact (S1.3):** not yet produced; the proposition and its proof are a
  dozen source lines, self-contained given Azuma's inequality.
- **Written by:** the executor (Claude Fable 5.1), Session 17; a reading of public documents,
  not a verdict on the paper (SD3).
