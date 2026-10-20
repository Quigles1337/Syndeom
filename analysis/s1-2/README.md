# S1.2 — Positive-pair procedure and first candidate review

**Decision:** the first candidate is feasible for the narrow producer/admission slice in
[ADR-0007](../../PROGRAM/DECISIONS/ADR-0007.md). This packet is development analysis,
not an admitted artifact, a completed S1.2 corpus, or a PROVEN promotion.
The reviewed schema already supports the representation; no schema amendment is proposed.

Branch: `codex/s1-2-positive-pair-procedure`. Base: main
`8adcb4865abd3f6c3ac882b70563d82caca6207c`, tree
`a1b2d003950972741a9768408554a9764a973f26`. This is an informed session. One mathematical
candidate was examined; none was rejected and no alternative candidate was investigated.
The candidate has been exposed during development; it is not unseen evaluation data.

**Post-merge locator correction (2026-10-20):** PR 3's original 28–29 formal interval
omitted the declaration. Current links/maps/configs use **27–28, one-based inclusive**,
checked against the exact pinned Git blob, and canonical declaration line 3. The corrected
archive identity was computed first with the Rust library, then propagated to the child
before computing its identity. The table below contains those corrected prospective IDs.
[Correction evidence](correction/README.md) is separate; the original `evidence/` receipts
and transcripts still describe the original configurations and are byte-for-byte retained.

## Independent primary source and rights

Thomas W. Judson, *Abstract Algebra: Theory and Applications*, **Annual Edition 2025**,
dated **August 1, 2025**, **Theorem 2.2.6 (Euclid)**, subsection **2.2.2 Prime Numbers**,
printed **p.28**, physical **PDF page 41** (one-based). The selected statement is:

> There exist an infinite number of primes.

This quotation comes from the [author-hosted fixed-edition PDF](https://judsonbooks.org/aata-files/aata-20250801.pdf),
not a back-translation of Lean. The edition's copyright is 1997–2025 Thomas W. Judson and
Robert A. Beezer; Beezer is separately credited for Sage exercises. The executor selected
the pair, transcribed the statement, and wrote the alias wrapper; neither source author is
credited with these SYNDEOM operations or claimed to endorse them.

The official [book page](https://judsonbooks.org/abstract-algebra-theory-and-applications/)
was used to locate the dated PDF. Its 2026 link has an inconsistent "2025 edition" label,
so this review freezes the unambiguous 2025 PDF and its own title/copyright pages.
Earlier theorem numbering from search snippets is not used. The rendered page and the
copyright page were visually read; `pdftotext -layout` supplied searchable text.
The same-page prime definition and surrounding proof were read, with the roles below.
The integer-divisibility definition in section 2.2, printed p.26 / PDF page 39,
was also read in text and in the rendered page: divisibility means an integer multiple.

The PDF's copyright page (page 3) explicitly grants **GFDL-1.2-or-later**, without
invariant sections or cover texts. Appendix A (printed pp.461–467) reproduces GFDL 1.3;
its copying and modification conditions were read. Mathlib's actual pinned LICENSE is
**Apache-2.0**, with attribution in the source headers to Microsoft Corporation (2015),
Leonardo de Moura, Jeremy Avigad and Mario Carneiro. The existing S-0008 licence record
is preserved; fresh source/declaration entries are S-0067 and S-0068.

Keep the full source PDF, raw formal module and canonical informal file external.
This is an operational storage choice under the current contract, not a finding that the
licences forbid redistribution. It keeps the textbook and any derivative extract outside
the repository's Apache release. No rights/policy decision blocks this design. Republishing
a modified textbook extract later would require the applicable GFDL notices/licence and
modification conditions; it cannot silently inherit the repository's licence.

The complete retrieval ledger is [retrieval.json](retrieval.json), including requested/final
URLs, HTTP status, UTC timestamps and byte counts. The frozen inputs were retrieved between
04:50:52Z and 04:50:57Z on 2026-10-20. Raw retained copies are at:

```text
C:\Users\LEET\Codex\Projects\Syndeom-S1.2-work-2026-10-20\sources\
```

| Exact object | Bytes | SHA-256 |
|---|---:|---|
| `aata-20250801.pdf` | 2,207,438 | `4f74587b61861b4790d613f2adb64699f90fd1d07760204f1d8fc9fcaf63d14f` |
| `PrimeFin.lean` | 5,328 | `92a5630a19bff74ade4088f1253cf75b0ac8143c002e6b604ac0b77a39c9ecf6` |
| `Infinite.lean` | 1,671 | `ede54daa0f432b403dfd402d96c682fc892f371fcbec26fb003c3fece9aea3f5` |
| `PrimeDefs.lean` | 16,833 | `aebd695f444b13ff056ba32c43c88d31b327a5e65d2a3e0ed4784643c542a59c` |
| Mathlib `LICENSE` | 11,357 | `b40930bbcf80744c86c46a12bc9da056641d722716c378f5659b9e555ef833e1` |

The dynamic author landing page is retained for retrieval provenance but is not a replay
input. The old `abstract.ups.edu` routes were unavailable; an optional separate GNU-host
licence download timed out. These did not block primary evidence: the author's current
site supplies the complete fixed PDF and its licence appendix. The first six-file retrieval
was retained; a second successful ledger-producing retrieval has the same PDF/Mathlib
digests. Nothing was silently substituted for the frozen book.

## Exact declaration and semantic correspondence

Pin: **Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`**, with existing Lean
**`leanprover/lean4:v4.34.1`**, Rust **1.91.0**, and unchanged transitive lockfiles.
Select [`Nat.infinite_setOfPred_prime`](https://github.com/leanprover-community/mathlib4/blob/d13f23b723b8a846827a245b89c10fc7d3f11612/Mathlib/Data/Nat/PrimeFin.lean#L27),
lines 27–28. Its module directly imports `Mathlib.Basic.Countable.Defs`,
`Mathlib.Data.Nat.Factors`, `Mathlib.Data.Nat.Prime.Infinite`, and
`Mathlib.Data.Set.Finite.Lattice`. The candidate imports only `Mathlib.Data.Nat.PrimeFin`.
The deprecated alias `Nat.infinite_setOf_prime` is not used.

Lean actually printed the complete elaborated target type with `pp.all = true`:

```lean
Nat.infinite_setOfPred_prime :
  @Set.Infinite.{0} Nat (@Set.ofPred.{0} Nat fun (p : Nat) => Nat.Prime p)
```

There are **no theorem parameters, implicit binders, universe parameters, typeclass
assumptions or hypotheses** on this declaration. `Nat`, universe 0 and the natural-number
structures are fixed. The higher-universe/ordered-type parameters visible in a dependency
are instantiated to the standard natural-number instances, not extra source hypotheses.
[Proof.lean](Proof.lean) declares `syndeom_s1_2_judson_primes` with the same printed type.

| Source clause/convention | Pinned formal evidence | Correspondence judgment |
|---|---|---|
| Section 2.2.2, p.28 defines primes among integers greater than one | `Nat.Prime : Nat → Prop`; `Nat.prime_def` gives `2 ≤ p` | Positive integers greater than one correspond uniquely to naturals at least two, by the value-preserving inclusion into integers and its nonnegative inverse. Zero, one and negative integers are excluded on both readings. This is not a restriction from arbitrary integers to a smaller set of possible primes. |
| Section 2.2 (p.26) defines integer divisibility; section 2.2.2 (p.28) allows only positive divisors one and the number itself | `Nat.prime_def : ∀ {p : Nat}, Prime p ↔ 2 ≤ p ∧ ∀ m, m ∣ p → m = 1 ∨ m = p` | The definition is audited, not inferred from the identifier. Its divisor quantifier ranges over all naturals. For `p ≥ 2`, zero cannot divide `p`; positive natural divisors match positive integer divisors. No sign ambiguity or negative-prime convention remains. |
| Theorem asserts infinitude of primes, with no parameters | The exact closed type above is the set `{p : Nat \| Nat.Prime p}` being infinite | Membership includes precisely the reviewed prime property. There is no chosen list, bound, ambient field or additional assumption. |
| "Infinite number" excludes any finite enumeration | `Set.Infinite s := ¬s.Finite`; `Set.Finite s := Finite s` for the element subtype; `Mathlib/Basic/Finite/Defs.lean:185,205` | The finite subtype convention means an equivalence with some finite `Fin n`. It matches the ordinary inability to list all primes finitely, also explicit in the source's contradiction setup. No cardinality stronger than infinitude is claimed. |
| Quantifier scope | No top-level binders; `p` is bound inside `Set.ofPred`; the prime definition quantifies divisors only after selecting `p` | There is no exchanged `∀/∃`. The dependency's `∀ n, ∃ p, n ≤ p ∧ Prime p` is a proof route to the selected set-infinitude type, not a replacement source statement. |
| Implicit conventions and maps | Natural `LE`, divisibility and monoid instances appear explicitly in `#print Nat.prime_def`; no local redefinitions | `p > 1` becomes `2 ≤ p` for integers/naturals. The domain identification is explained above. There is no runtime Lean coercion in the final closed statement and no algebraic-generalization assumption. |
| Conclusion and scope | `Nat.infinite_setOfPred_prime` plus the explicit `Set.Infinite` definition | Same unconditional infinitude claim. No assertion about density, distribution, effective bounds, prime ideals or all integer irreducibles. |

These are source-specific judgments anchored in the printed definition and theorem.
No equivalence proof between two independently true propositions is offered as a substitute.
No residual statement ambiguity was found within this scope.

## Offered proof and actual axiom audit

The published source has a proof, but this pair **selects the statement only**.
Its canonical informal file contains zero proof steps. The supplied Mathlib proof is the
offered argument for that informal claim. The canonical wrapper is:

```lean
import Mathlib.Data.Nat.PrimeFin

theorem syndeom_s1_2_judson_primes : Set.Infinite {p : ℕ | Nat.Prime p} :=
  Nat.infinite_setOfPred_prime
```

The source's printed argument uses a product of an assumed complete list of primes.
Mathlib's inspected argument instead chooses a minimal prime factor of `n! + 1`, proves it
cannot lie at or below the bound used in the contradiction, obtains unboundedness, and
uses finiteness-implies-boundedness. They target the same reviewed claim. This is **not**
a claim that Mathlib formalizes each step of Judson's printed argument; that proof is retained
as source context and no source-proof validity label is inferred from the alias compiling.

The inspected dependency chain is:

| Declaration/body | Role | Transitive axiom constants actually reported |
|---|---|---|
| `syndeom_s1_2_judson_primes` | Direct exact-type reference to the selected theorem | `propext`, `Classical.choice`, `Quot.sound` |
| `Nat.infinite_setOfPred_prime` | `Set.infinite_of_not_bddAbove Nat.not_bddAbove_setOfPred_prime` | Same standard three |
| `Nat.not_bddAbove_setOfPred_prime` | Apply `Nat.exists_infinite_primes` at `n.succ` to refute an upper bound | Same standard three |
| `Nat.exists_infinite_primes` | `minFac (n! + 1)`, primality, factorial divisibility and impossibility of a prime dividing one | Same standard three |
| `Set.infinite_of_not_bddAbove` | Contrapositive of `Set.Finite.bddAbove` | Same standard three |
| `Nat.prime_def` | Divisor characterization used in semantic review | `propext` |

`#print axioms` traverses proof dependencies, rather than just scanning the wrapper text.
No `sorryAx`, `Lean.ofReduceBool` or nonstandard axiom occurs in these closures.
The full output (including printed bodies/types) is [evidence/candidate-audit.txt](evidence/candidate-audit.txt).
`pp.all` prints `Classical.choice.{u}` and `Quot.sound.{u}`; receipts keep that literal
output and record the corresponding constant names for the standard-three comparison.
This is candidate feasibility at the existing trust boundary, not corpus admission.

## Concrete representation and provenance

The intended real item is **one archival pair**: the entire Judson PDF as informal `raw/1`
and the entire pinned `PrimeFin.lean` Git blob as formal `raw/1`. The item/locators select
the theorem and offered proof within those larger objects. The original book's proof is
not relabeled as the offered proof; Mathlib authorship is not assigned to Judson.
The canonical pair is a representation of that archival item, with the statement
transcription and an executor-authored exact-type alias. No fake raw parent or erratum is used.

Full proposed runtime inputs are [archive-config.proposed.json](archive-config.proposed.json)
and [canonical-config.proposed.json](canonical-config.proposed.json). These are configuration
designs, **not full manifests**. The latter's `derived_from` is a prospective, deterministic
ID; there is no admitted parent with that name yet.

| Manifest field | Archival pair | Canonical pair |
|---|---|---|
| `manifest_schema`, `origin` | `syndeom-manifest/2`, `ground-truth` | Same |
| `generator` | Proposed `s1_pair_archive`, `0.1.0`, RNG `none` | Proposed `s1_pair`, `0.1.0`, RNG `none` |
| `seed` | `null` | `null` |
| `engine_commit` | Actual future producer-containing commit, verified against executable inputs | Actual future producer-containing commit; none fabricated here |
| `config` / `config_hash` | Complete proposed JSON / Rust JCS digest below | Complete proposed JSON / Rust JCS digest below |
| `artifact.kind` | `pair`, one ID and two files | `pair`, one ID and two files |
| Informal file | `aata-20250801.pdf`, `raw/1`, `external`; byte hash/size above; other counts null | `proof.md`, `syndeom-informal-md/1`, `external`; 118 bytes, 7 tokens, 0 steps, 0 lemmas |
| Formal file | `PrimeFin.lean`, `raw/1`, `external`; byte hash/size above; counts/main declaration null | `Proof.lean`, `syndeom-lean/1`, `committed`; 142 bytes, 14 tokens, 4 lines, 1 declaration; main `syndeom_s1_2_judson_primes` |
| `toolchain` | Existing Lean string and Mathlib 40-hex pin | Same |
| `config.status` | `kind: mathlib`; declaration/pin/source correspondence reference | Identical inherited status |
| `ground_truth` | Intended `formalized-in-mathlib`; true by `mathlib`; offered proof `valid`; `correct: true`; empty flaws; taxonomy `0.1.0` | Identical statuses, source item and ground truth |
| `commanded`, `achieved`, `fidelity` | `{}`, `{}`, `{per_statistic: {}, verdict: PASS}` | Same; this vacuous statistical PASS is not semantic review |
| `derived_from`, `method` | Genuine raw item of record | Actual parent ID, `reviewed-statement-mathlib-alias/1` |
| Attribution/context | Original authors/rights, full source objects, source licences | Separately credited transcription/alias; complete raw context still required |

The canonical statement is an exact sentence transcription. Number/Euclid label is retained
in source metadata/title; the required Markdown delimiters add no proof. The source map in
the recipe links printed theorem 2.2.6 to statement step 0, scalar span `[0,41)`, and
Mathlib lines 27–28 to wrapper declaration line 3. No fabricated PDF byte offsets are given.
Config hashes cover the frozen transcription, wrapper identity/type, all target/raw hashes,
statuses, source/attribution/storage decisions and toolchain. Local paths/timestamps are
receipt data, not generation inputs. Raw blob bytes remain distinct from the Windows
dependency checkout's CRLF bytes.

| Proposed input/output | SHA-256 / prospective ID |
|---|---|
| Canonical informal bytes | `56983d03aa658d0315659b88147f6277e7cb6b883e3ce5a6db099094b5949b95` |
| Canonical formal bytes | `038c70bf73cacf9eca975bd58ac6d64c23f3227474654d0c18ee2a93f5be09a2` |
| Archival config, Rust JCS | `094625fa10ffde723ba3a3ae47ff75db2b1994b8f96f26f6eb562dcbf653ede0` |
| Archival prospective ID, not admitted | `s1_pair_archive-v0.1.0-094625fa10ff` |
| Canonical config, Rust JCS | `62fe0c816a6b0ef928f3e4c2fb7e2bd2e215171ebf6b6130202e851e1e9664a3` |
| Canonical prospective ID, not admitted | `s1_pair-v0.1.0-62fe0c816a6b` |

The current `Manifest::check` accepts the pair shape, `status.kind = mathlib`,
`formalized-in-mathlib`, valid/true distinction and zero-step informal syntax.
The actual implementation was read, including `check_derived`; the latter does not enforce
every producer-specific parent-kind/config invariant, so ADR-0007 D6 adds explicit checks.
The existing S1.1 CLI deliberately requires invalid erratum/retraction proofs. The S1.3
CLI deliberately takes a raw informal gzip/TeX parent and nonempty proof slices.
Neither limitation is a schema blocker; neither CLI is loosened in this task.

Implement one later narrow Rust archive/canonical pair producer reusing the manifest library.
It will validate both actual files and the actual parent/child relationship, preserve
inherited metadata, refuse changed existing outputs, and bind its executable inputs to a
real producer commit before generation. Details and refusal tests are in ADR-0007 D5–D8.
This packet's Python orchestration and Rust pure-API probe do not implement that producer.

## Reproduction and executed checks

From a checkout with the existing pinned toolchains/cache, use a **new external directory**:

```powershell
python -I -X utf8 analysis/s1-2/reproduce.py --output C:\Users\LEET\Codex\Projects\Syndeom-S1.2-replay
```

The default retrieves the five frozen primary objects and checks their exact digests.
To use already retained bytes, add:

```powershell
--source-dir C:\Users\LEET\Codex\Projects\Syndeom-S1.2-work-2026-10-20\sources
```

The output directory must not exist and must be outside the repository. It receives source
copies, candidate bytes, the concatenated audit, per-command logs and a fresh receipt.
Existing historical evidence is never the output target. The helper records checkout
revision, index tree/status, its own hash and all packet input hashes. It verifies the
downloaded Mathlib objects against pinned Git blobs, and separately records an exact
LF-to-CRLF checkout conversion where present. This does not normalize the archival objects.

Commands actually executed by the successful replay on 2026-10-20:

```text
(cwd lean) lake env lean <fresh-output>/CandidateAudit.lean
(cwd engine) cargo build --locked -p syndeom-manifest --message-format=json
(cwd repository) rustc --edition=2024 analysis/s1-2/api_probe.rs --extern syndeom_manifest=<cargo-emitted-rlib> -L dependency=<target/debug/deps> -o <fresh-output>/api-probe.exe
<fresh-output>/api-probe.exe <fresh-output>/proof.md <fresh-output>/Proof.lean analysis/s1-2/archive-config.proposed.json analysis/s1-2/canonical-config.proposed.json
```

All four exited 0, ending **04:57:22.757990Z**. See
[evidence/rehearsal-03.json](evidence/rehearsal-03.json) for the exact Windows argument
arrays and [evidence/api-probe.txt](evidence/api-probe.txt) for counts/hashes and the
two format-refusal controls (CR in informal bytes, non-Mathlib import). The Rust probe
confirms the independent Python proposed-parent hash calculation; Rust is the canonical
hash/count authority. The compiled wrapper's type/axioms are checked separately from the
repository's pre-existing golden fixture.

Two **development harness failures** are retained honestly:

- [rehearsal-01](evidence/rehearsal-01.failed.json): exact working-file comparison stopped
  before Lean because of the clean Windows dependency checkout's CRLF conversion.
  Fixed by checking pinned Git blobs and recording the permitted checkout conversion.
- [rehearsal-02](evidence/rehearsal-02.failed.json): source checks and Lean passed, but
  standalone `rustc` could not find transitive dependencies because the harness supplied
  `target/debug` instead of `target/debug/deps` as its dependency search directory.
  [Compiler output](evidence/rehearsal-02-compiler.txt) retained; fixed in the harness.

Neither was a failed mathematical audit. The initial focused Lean run and both later Lean
runs returned 0 with identical audit output. No failure was replaced by an asserted success,
axiom, placeholder, producer SHA or rewritten old receipt.

**Not executed:** full candidate `Manifest::check`, `check_derived`, producer output/replay
tests or admission, because no production pair producer/manifest exists yet. Pure API
success is not advertised as full manifest validation. The repository gate and preservation
results are in [VERIFICATION.md](VERIFICATION.md); its Lean audit remains the existing golden.

## Remaining obligation and exact next action

No unresolved source/statement/rights decision was found for the chosen external-source
design. The implementation/admission obligation is still real: review this draft packet,
then implement only the narrow `syndeom-pair` archive/canonical producer of ADR-0007,
test and commit it first, freshly verify the frozen objects, generate a genuine raw pair
and canonical child from that actual producer SHA, run all byte/parent-child/kernel/axiom
and refusal checks, and review the admission RECORD before counting the first positive pair.

This packet leaves the corpus at **31 raw parents plus one canonical representation**.
The first-parent promotion hold is unresolved; harvest resumes at candidate 28,
`1403.6317`. Gate S1, S1.4 and unrelated standing questions remain open.
