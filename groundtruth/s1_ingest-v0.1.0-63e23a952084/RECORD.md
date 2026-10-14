# RECORD — Route A item 1: arXiv:0706.0357 v70, Lemma 1.1 (ADR-0003 D4; the S1.3 record for this item)

The `artifact_id` is the name of this directory and is in `manifest.json`.

- **Route and identifiers:** Route A (arXiv revision comments); arXiv `0706.0357`; original =
  v70 (7 March 2026); correcting version = v71 (7 June 2026, inside the ADR-0003 D8 window);
  examined Session 11, 2026-10-14 (access 01:57:34Z). Single author; General Mathematics (math.GM);
  71 versions since 4 June 2007.
- **Status document (SD12; D2.2):** the v71 revision comment — "11 pages. To fix an error in
  Lemma 1.1" — together with the corrected version v71 itself. S-entries: S-0024 (the original,
  v70), S-0025 (the corrected version and its comment).
- **Provenance and digests:** abs v70 49,164 B `4a1cd5dc…9013`; abs v71 49,049 B `1dbf356c…4e29`;
  e-print v70 8,236 B `5987d810…cca8` (gzipped tar, one `.tex`) — the committed original; e-print
  v71 8,750 B `fffd0fdc…d263`. Full digests in S-0024 and S-0025.
- **The notice's locating and describing sentence (quoted):** "To fix an error in Lemma 1.1."
- **Diff summary (executor's reading of the TeX diff v70 → v71):** (1) The *statement* of Lemma 1.1
  changes: v70 asserts, for all s ∈ ℂ and rational 0 < a/b < 1, the identity M(s, a/b) =
  −Σ_χ A(a, b; χ)(L′/L)(s, χ); v71 subtracts from the left side the term
  Σ_{q|b} Σ_{l≥1} (log q) q^{−sl} e^{−2πi a q^l/b}, the sum running over the prime divisors q of
  b. (2) The *proof* changes at exactly the corresponding place: v70 applied the orthogonality
  relation for Dirichlet characters to every n ≥ 1 in the sum defining M(s, a/b); v71 states the
  relation "for (k, b) = 1" and adds "the formula is applicable to all n's which are powers of
  primes other than of q|b". (3) Everything else in the diff is expository (a new introductory
  paragraph on zero-density estimates, one new bibliography entry), typographical, or a
  renaming of a dummy variable (n, m → x, a). No other result's statement changes.
- **Location (D3, `raw/1` locators):** instance 1 — "Lemma 1.1, statement"; instance 2 —
  "Lemma 1.1, proof, the application of the character orthogonality formula to every n".
- **Class argument (D3):**
  - **Instance 2, FLAW-0005 (theorem applied without its hypotheses), severity fatal,
    lean_catchable YES-KERNEL.** The v70 proof invokes the orthogonality relation of Dirichlet
    characters modulo b — valid only for arguments coprime to b — on every integer n ≥ 1, including
    the prime powers n = q^l with q | b, for which every character value χ(n) is zero. The entry's
    definition clause: "A named theorem is invoked on an object for which one of its hypotheses is
    … false (fatal)". Fatal because the hypothesis fails for infinitely many terms and the stated
    conclusion is false (next instance). Lean-catchable: a faithful formalization cannot supply the
    coprimality hypothesis for those n (`YES-KERNEL` per the entry).
  - **Instance 1, FLAW-0012 (false theorem with convincing proof), severity fatal (by definition),
    lean_catchable YES-KERNEL.** The v70 statement of Lemma 1.1 omits the term the v71 statement
    adds; that term is Σ over the prime divisors q of b of Σ_{l≥1} (log q) q^{−sl} e^{−2πi a q^l/b},
    which is not identically zero for any b ≥ 2 (each summand has modulus (log q) q^{−σ l} > 0 and
    the inner sums are absolutely convergent geometric-type series for σ > 0 — the executor's
    check of the corrected statement, not a new claim). So the v70 statement is false as stated
    and the v70 proof "mirrors the argument for a nearby true statement, hiding the point where a
    dropped hypothesis is used" — the entry's definition — with the hidden use site being
    instance 2, exactly as the entry says ("the flaw surfaces at the hypothesis's use site,
    usually as FLAW-0005"). Lean-catchable: a sorry-free proof of the v70 statement cannot exist
    (`YES-KERNEL` per the entry).
  - Other classes considered: FLAW-0001 (sign or constant error) — no: nothing is mis-signed; a
    whole family of terms is missing. FLAW-0004 (unjustified step) — no: the step is justified,
    by a formula applied outside its hypothesis.
- **Statement status (D3):** `value: false`, `method: "source-document"`, `reference: S-0025` —
  the status document (the corrected version) replaces the statement with a different identity;
  the executor's reading of the difference is above. The notice itself says only "an error in
  Lemma 1.1"; the falsity of the v70 statement is what the corrected statement documents.
- **Proof status:** `invalid` (S1.1 by construction; the author documents the error in the proof
  and the statement of Lemma 1.1).
- **Human-authored (D2.6):** single human author; no mention of AI assistance in the v71 sources
  (helper scan: none) or in any comment; the error was found by the author (no other finder is
  named).
- **Licence (D5, SD16):** v70 and v71 both carry `http://creativecommons.org/licenses/by/4.0/` on
  their abstract pages → the original's storage is `committed`
  (`original-0706.0357v70.tar.gz`, the e-print bytes as received); recorded in S-0024 and S-0025
  before ingestion.
- **What this item is and is not:** a real, human-authored preprint whose author documents an
  error in the statement and proof of its Lemma 1.1. Nothing here assesses the preprint's main
  claim, its approach, or any other result (SD3); the item's claim of record is Lemma 1.1 alone.
  The paper's 71 revisions and its math.GM classification played no part in the verdict: the
  no-skipping rule (D8) admits what the criteria admit.
- **Derived canonical artifact (S1.3):** not yet produced; the location re-expression argument is
  added here when it is. The affected result is short (one lemma, a half-page proof in the
  single TeX file).
- **Written by:** the executor (Claude Fable 5.1), Session 11; a reading of public documents, not
  a verdict on the paper (SD3).
