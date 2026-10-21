# RECORD — Route A item 33 (candidate 29): arXiv:1403.8075 v9, Proposition 4.1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A; arXiv `1403.8075`; original = v9 (23 March 2023, the last
  version with an e-print; the paper's eleven versions run from March 2014; no journal reference
  on the abstract page); correcting versions = v10 (27 December 2025) and v11 (9 January 2026),
  both withdrawals of the whole paper, inside the window; examined Session 21, 2026-10-21 (access
  21:48:05Z–21:52:06Z). One author; General Mathematics (math.GM).
- **Status document (SD12; D2.2):** the two withdrawal comments — v10: "Withdrawn: Section 4
  contains a critical logical error. In Proposition 4.1 the argument incorrectly treats primality
  verification as equivalent to checking leading digits in multiple base representations, and the
  subsequent claim that fp(1),...,fp(n) are independent is unjustified. These invalidate the
  probabilistic step leading to Theorems 4.3 -- 4.4"; v11: "Withdrawn due to a logical error in
  Section 4. In Proposition 4.1, the argument incorrectly treats primality verification as
  equivalent to checking leading digits in multiple base representations, and the subsequent
  independence assumption for fp(1),...,fp(n) is unjustified. This invalidates the derivation of
  Theorems 4.3-4.4". There is no corrected version; the comments are read against the v9 text (as
  for candidate 18). S-entries: S-0068 (the original, v9), S-0069 (the withdrawal comments);
  shared with items 34 and 35.
- **Provenance and digests:** abs v9 41,098 B `1cd0cb15…5d77`; abs v10 39,877 B
  `b01ae085…d42f`; abs v11 40,895 B `5726f969…397b`; e-print v9 136,883 B `c171e338…9270` (a tar
  bundle: `rev_math1.tex`, 595 lines, with `.bbl` and `.synctex`) — the original, **not
  committed**. Full digests in S-0068 and S-0069.
- **The claim of record (quoted from v9, Section 4 "Distribution of prime numbers"; the counter
  is per section and shared, so Proposition 4.1 is the section's first numbered environment):**
  "The function f_p has the property of an infinite number of steps." — with f_p : ℕ → {0, 1} the
  indicator of primality and the property Definition 2.7's: "a monotonically increasing natural
  number sequence {n_i} exists such that for any algorithm, the number of steps required to
  verify whether f(n_i) = 0 or f(n_i) = 1 tends to infinity when i → ∞". Its proof: "Let {p_i} be
  a number sequence, where p_i is the ith prime number. If we can verify whether f_p(p_i) = 0 or
  1 when p_i tends to infinity, we can ascertain this parameter from reading finite cells of p_i
  described in some number representations. However, if we cannot verify that the first digit of
  the p_j-ary representation of p_i is not 0 for a sufficient number of j < i, we cannot verify
  whether p_i is prime. Thus, the input to an algorithm in a certain number of finite steps must
  cover an arbitrarily large number of combinations of whether the first digit of the p_j-ary
  number is 0 or not when i → ∞. Therefore, we require an arbitrarily large number of inputs to
  be calculated in a certain number of finite steps to verify whether p_i is prime when i is
  arbitrarily high. The above reasoning is valid for any algorithm since verifying any number to
  be prime is equivalent to verifying the first digit of the p_j-ary number is not 0 for certain
  p_js. However, the input is finite, which leads to a contradiction. Therefore, we must
  implement an arbitrarily large number of steps for arbitrarily large p_i. Such a subsequence
  satisfies Definition 2.7 for f_p. This result leads to the proposition."
- **The notice's locating and describing sentences (quoted):** "In Proposition 4.1 the argument
  incorrectly treats primality verification as equivalent to checking leading digits in multiple
  base representations" (v10); "In Proposition 4.1, the argument incorrectly treats primality
  verification as equivalent to checking leading digits in multiple base representations" (v11).
- **Confirmation against the text (executor's reading; there is no diff, the correcting versions
  being stubs):** the sentence the notice describes is the proof's "verifying any number to be
  prime is equivalent to verifying the first digit of the p_j-ary number is not 0 for certain
  p_js", on which "The above reasoning is valid for any algorithm" rests; it is asserted, not
  argued. The proposition is used once, in the paragraph that follows it ("we can deduce from
  Proposition 4.1 and Remark 2.4 that lim ω_{p_i} ≠ ∅ and lim ω_{p_i} ≠ Ω"), to treat f_p as a
  random variable — the "probabilistic step" the notice says is invalidated (items 34 and 35).
  The AI-assistance scan of v9: no hit.
- **Location (D3, `raw/1` locator):** "Proposition 4.1, proof, the sentence 'The above reasoning
  is valid for any algorithm since verifying any number to be prime is equivalent to verifying
  the first digit of the p_j-ary number is not 0 for certain p_js'".
- **Class argument (D3):** **FLAW-0004 (unjustified step), dial `false`, severity fatal,
  lean_catchable YES-KERNEL.** The proof's decisive step is an equivalence asserted without
  justification — the entry's "non-trivial step carries no justification" — and the author now
  states that the equivalence is incorrect, which is the entry's `false` dial and its fatal end
  ("the gap must be filled; when the hidden step is false it cannot be"). Severity fatal also by
  D3: the paper is withdrawn. Other classes considered: FLAW-0012 — the notice faults the
  argument and does not say the proposition is false, so the composite does not apply; FLAW-0005
  — no named theorem is invoked at the step; FLAW-0010 — nothing is cited.
- **Statement status (D3):** `value: null`, `method: "undetermined"`, `reference: S-0069` — the
  withdrawal says the argument is wrong, and nothing re-proves or disproves the proposition (the
  eleventh use of ruling (ii)).
- **Proof status:** `invalid` (S1.1 by construction; the author withdraws the argument).
- **Human-authored (D2.6):** one human author; no AI-assistance mention in the v9 source
  (scanner clean); the finder of the error is not stated. math.GM is the General Mathematics
  listing; ADR-0003 D2 does not select by category, and the candidate was read with the care
  STATE asked for: a preprint with named results and offered proofs, withdrawn by its author with
  a comment describing a mathematical error.
- **Licence (D5, SD16):** v9 carries `http://arxiv.org/licenses/nonexclusive-distrib/1.0/`; the
  withdrawal stubs carry no licence link → the original's storage is **`external`**: no copy
  committed; the manifest pins the bytes by hash (`original-1403.8075v9.tar.gz`, 136,883 B,
  `c171e338…9270`). Recorded in S-0068 and S-0069 before ingestion.
- **What this item is and is not:** a real, human-authored preprint, revised over nine years and
  withdrawn by its author with a comment that names the wrong step; the paper's abstract claims
  consequences for the Riemann hypothesis, which this record neither restates nor assesses (SD3).
  Lemma 4.2 and the results of Sections 2 and 3 are not named by the notice and are not items
  (D2.3). The claim of record is Proposition 4.1 as stated in v9.
- **Derived canonical artifact (S1.3):** not yet produced; the proposition and its proof are
  eleven source lines, self-contained but for Definition 2.7.
- **Written by:** the executor (Claude Fable 5.1), Session 21; a reading of public documents,
  not a verdict on the paper (SD3).
