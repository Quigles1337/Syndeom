# Lemma 1.1 status-support witness

**2026-10-20 UTC · checked mathematical witness; corpus amendment proposed.**
Scope: Hisanobu Shinya, *On a new approach to the Riemann hypothesis*,
arXiv:0706.0357v70, Lemma 1.1 and its displayed character expansion only.
This review does not assess any other result or the Riemann hypothesis.

The candidate argument was supplied in the owner's task prompt. The executor, **Codex
(GPT-6), this status-resolution session**, checked it against the retrieved source and
standard identities in two algebraically different ways. Those checks are one executor's
work, not independent human review, blind review or a detector evaluation. This case and
both witnesses are development-exposed.

## 1. Exact source correspondence

The source archives were freshly retrieved on 2026-10-20 and compared byte-for-byte with
the retained S1.3 copies. The v70 archive also matches the old committed original.
`evidence/retrieval.json` records URL, access time, HTTP status, byte length and SHA-256;
`evidence/correspondence.json` records the sole TeX member and its exact-byte hash.
Line numbers below are one-based lines in that member, counted with `splitlines()`;
no source bytes were normalized before hashing. Full v71/source HTML copies stay external.

| Version / object | Bytes | SHA-256 |
|---|---:|---|
| [v70 archive](https://arxiv.org/src/0706.0357v70) | 8,236 | `5987d8106ae69669b10c876a0915d5ad52ce44a814ea7fe63600c1737642cca8` |
| v70 `On_a_new_approach_to_the_Riemann_hypothesis.tex` | 29,544 | `17d3f102b355618c6aec9daa0187b35c2d3c4b8a78c62c9a38bb97a7bcb00db1` |
| [v71 archive](https://arxiv.org/src/0706.0357v71) | 8,750 | `fffd0fdc3a868deb642102a1f72f23cd92ee89bf118c92a4b992cb5a6fdfd263` |
| v71 `On_a_new_approach_to_the_Riemann_hypothesis_2.tex` | 30,890 | `2bdf0dc0e10601f547e196442f78a377e100cab369bc6fc1b5ef862cd48c81f4` |

| Passage | v70 TeX lines | v71 TeX lines | Correspondence |
|---|---|---|---|
| Definition of M and allowed p | 58–64 | 69–75 | Same series and rational interval |
| Lemma 1.1 and A | 72–82 | 83–95 | v71 subtracts a prime-power sum on the left |
| Displayed character expansion | 85–98 | 98–111 | v71 explicitly restricts to gcd(k,b)=1 |
| Use inside the Mangoldt series | 114–125 | 127–145 | v71 separates prime divisors of b |

The definitions and v70 claim, transcribed as mathematical expressions, are

\[
M(s,p)=\sum_{n\ge1}\Lambda(n)e^{-2\pi i pn}n^{-s},\qquad
A(a,b;\chi)=\frac1{\phi(b)}\sum_{x=1}^b\overline{\chi(x)}e^{-2\pi i ax/b},
\]
\[
M(s,a/b)=-\sum_{\chi\bmod b} A(a,b;\chi)\frac{L'}L(s,\chi).
\]

The source quantifies over complex s and rational 0<a/b<1. We use only
**a=1, b=2, s=2**: a and b are positive coprime integers, p=1/2 is allowed,
and Re(s)>1 is an ordinary domain of absolute convergence. Nothing here relies on
analytic continuation or on interpreting either side at a pole.

The [v71 author notice](https://arxiv.org/abs/0706.0357v71) says only
**“To fix an error in Lemma 1.1.”** Its changed formula and restriction locate the
correction. The notice does not explicitly declare the v70 statement false or supply
our parameter choice and residual. Fresh abstract HTML hashes differ from the older
`analysis/s1-3/retrieval.json` HTML hashes; that is recorded as a new dynamic-page
retrieval, not a failed archive match or an overwrite of the old receipts.

## 2. Witness A: the displayed proof step

A Dirichlet character modulo 2 is the extension by zero of a homomorphism on the unit
group of Z/2Z. That group contains only 1. A homomorphism sends 1 to 1, so there is
exactly one character, chi0: it is 1 on odd integers and 0 on even integers. Also
phi(2)=1. These properties agree with [DLMF §27.8, E1–E5](https://dlmf.nist.gov/27.8);
its orthogonality formula E6 includes the coprimality condition.

Complex conjugation fixes both 0 and 1. Thus, using the source's inclusive x=1,2 range,

\[
A(1,2;\chi_0)=\overline{1}e^{-\pi i}+\overline{0}e^{-2\pi i}=-1.
\]

At **k=2**, v70 lines 88–96 specialize to

\[
\underbrace{e^{-2\pi i\,1\,2/2}}_{1}
\stackrel{\text{v70}}{=}
\underbrace{\sum_{\chi\bmod2}\chi(2)A(1,2;\chi)}_{0\cdot(-1)=0}.
\]

This is a finite counterexample to that unrestricted equality. It needs no series
interchange. Every character vanishes at the nonunit residue, whereas the exponential
does not. It supports the recorded failed-hypothesis application (FLAW-0005).
A failed proof step alone would not establish a counterexample to the lemma's conclusion;
that is the separate task in §3.

## 3. Witness B: a nonzero residual for the lemma

### Convergence and nonvanishing

The [Mangoldt definition, DLMF §27.2 E14](https://dlmf.nist.gov/27.2#E14), gives
Lambda(1)=0, Lambda(p^ell)=log p for a prime p and ell>=1, and zero otherwise.
Consequently 0<=Lambda(n)<=log n for n>=1. For every epsilon>0,
`sum(log(n)/n^(1+epsilon))` converges by the integral test: its tail is bounded by
an integral of `log(x)/x^(1+epsilon)`, which is finite. Exponential factors at p=1/2
have modulus one. Therefore M(2,1/2), and its even and odd subseries, converge absolutely.
The same bound applies to all series used below on each closed half-plane
Re(s)>=1+epsilon. It justifies partitions, regrouping by prime powers and locally
uniform analytic manipulations near s=2; we can take epsilon=1/2 there.

The ordinary [Dirichlet L-series and Euler product, DLMF §25.15 E1/E2](https://dlmf.nist.gov/25.15),
are valid for Re(s)>1. Here

\[
L(2,\chi_0)=\sum_{n\ \mathrm{odd}}n^{-2}>0
\]

(the n=1 term is 1). The logarithmic derivative at 2 is therefore defined.
More generally the absolutely convergent logarithmic Euler product on Re(s)>1
shows nonvanishing there. For chi0 modulo 2 that product omits the prime 2:

\[
\log L(s,\chi_0)=\sum_{p\ne2}\sum_{m\ge1}\frac{p^{-ms}}m.
\]

To differentiate, bound the absolute derivative series for sigma=Re(s)>=1+epsilon by

\[
\sum_p\sum_{m\ge1}(\log p)p^{-m\sigma}
=\sum_p\frac{(\log p)p^{-\sigma}}{1-p^{-\sigma}}
\le\frac1{1-2^{-1-\epsilon}}\sum_{n\ge2}\frac{\log n}{n^{1+\epsilon}}<\infty.
\]

The undifferentiated logarithmic series is locally uniformly absolutely convergent too.
The Euler product is its exponential; differentiation yields

\[
\frac{L'}L(s,\chi_0)
=-\sum_{p\ne2}\sum_{m\ge1}(\log p)p^{-ms}
=-\sum_{n\ \mathrm{odd}}\Lambda(n)n^{-s}.
\]

The last regrouping uses unique prime factorization, the Mangoldt definition and
absolute convergence. The minus sign comes from differentiating p^(-ms)/m.
No corrected v71 identity is assumed.

### Direct parity derivation

Set T=sum(n>=1) Lambda(n)/n^2, E the even part and O the odd part; thus T=E+O.
At p=1/2, exp(-pi*i*n)=(-1)^n. It follows that

\[
M(2,1/2)=E-O,\qquad (L'/L)(2,\chi_0)=-O.
\]

An even integer has nonzero Mangoldt value precisely when it is 2^ell for ell>=1:
an even prime power must have prime base 2. There is no ell=0 contribution, because
Lambda(1)=0 and 1 is odd. Thus

\[
E=(\log2)\sum_{\ell=1}^{\infty}4^{-\ell}
=(\log2)\frac{1/4}{1-1/4}=\frac{\log2}{3}>0.
\]

This is the convergent infinite geometric series, with ratio 1/4 and first term 1/4.
Since A=-1 and there is one character, the claimed v70 right side is exactly L'/L.
The counterexample is therefore

\[
\boxed{\ M(2,1/2)-\left[-\sum_{\chi\bmod2}A(1,2;\chi)(L'/L)(2,\chi)\right]
=M(2,1/2)-(L'/L)(2,\chi_0)=\frac{\log2}{3}>0.\ }
\]

The residual is a positive real number, also nonzero under the injective embedding
into C. This does not use a numerical approximation or infer noncancellation from
magnitudes of arbitrary complex summands: at these parameters the omitted phases
are individually 1 and the surviving terms are positive real numbers.

### Second derivation: the missing Euler factor

Independently split the absolutely convergent zeta series into odd and even integers:

\[
L(s,\chi_0)=\sum_{n\ \mathrm{odd}}n^{-s}=(1-2^{-s})\zeta(s),\qquad\Re s>1.
\]

At 2 both factors are positive. Differentiating in that open half-plane gives

\[
(L'/L)(s,\chi_0)=\zeta'(s)/\zeta(s)
+\frac{(\log2)2^{-s}}{1-2^{-s}}.
\]

In particular the extra term at s=2 is **+log(2)/3**, not its negative.
The [Mangoldt logarithmic derivative, DLMF §27.4 E12](https://dlmf.nist.gov/27.4#E12),
valid for Re(s)>1, says T=-zeta'(2)/zeta(2). Absolute convergence also gives
M=2E-T. Therefore

\[
M(2,1/2)=2\log(2)/3+\zeta'(2)/\zeta(2),\qquad
(L'/L)(2,\chi_0)=\log(2)/3+\zeta'(2)/\zeta(2).
\]

Subtracting again gives log(2)/3. This checks the sign and factor using the zeta
identity rather than the character Euler product. It shares the elementary parity
and geometric evaluation; it is not an independent human review.

### v71 correspondence cross-check only

At b=2 the only prime divisor q is 2. For ell>=1, q^ell/b=2^(ell-1) is an integer,
so every exp(-2*pi*i*a*q^ell/b) equals 1 at a=1. The term subtracted in v71
lines 86–87 is exactly E=log(2)/3. This agrees with the independently derived residual.
We make no assertion about the whole v71 lemma for every s or any later theorem.

## 4. Attribution and actual check scope

| Evidence / role | What it establishes here | What it does not establish |
|---|---|---|
| Shinya's v71 notice and source change; S-0025 | Author-documented error at the named lemma; changed formula and coprimality restriction | Explicit author declaration of falsity; authorship of our witnesses |
| Codex GPT-6, this session, direct check | Parameter legality, unique character, A, finite 1 versus 0, absolute convergence, nonvanishing, Mangoldt support and residual | Independent human review or approval to amend the corpus |
| Same executor, second check | Missing Euler factor and zeta derivative independently reproduce the residual and sign | A blind or institutionally independent review |
| Lean kernel, pinned project, Witness.lean | Actual character uniqueness; exact phases and A; finite-step inequality; infinite geometric HasSum and positivity | Full formal definition/correspondence for source M, L, or their analytic bridge |
| Rust ingest + manifest validation | Consistent candidate schema, actual bytes, config hash, ID, duplicated metadata and replay | Mathematical truth, rights, admission or acceptance of the proposed ruling |
| Owner | Supplied task and candidate derivation; merged PR 5 | Has not yet accepted this ruling or reviewed the new derivation |

`Witness.lean` defines phase(k)=exp(-2*pi*i*k/2) and coefficient as the two source
summands divided by Nat.totient 2, with complex `star` as conjugation. The theorem
`character_unique` compares actual Mathlib `DirichletCharacter C 2` objects;
`finite_step_fails` proves phase(2) differs from chi(2)*coefficient(chi) for every such
character. The reduction of the source's all-character sum to this unique summand
is recorded in §2, not implemented as a separate Fintype sum in Lean.
`residual_series` proves HasSum of log(2)*(1/4)^(n+1) to log(2)/3, with n>=0
corresponding to ell>=1. `residual_positive` proves positivity.

**Unformalized bridge obligations:** define the source's M and Dirichlet L in Lean;
prove their convergence, parity split, Mangoldt reindexing and derivative identity;
identify their residual with the supported geometric series; express the all-character
sum and the complete source quantification. Those obligations are justified in writing
above, but not claimed kernel-checked. No assumption of the disputed equality or whole
v71 identity appears in the Lean file. No CLAIM is added or promoted to PROVEN.

Actual compiler/axiom logs, including failed prototypes, are retained in `evidence/`.
The final Lean check is explicit; `scripts/check.sh` alone does not compile this analysis
file. See `VERIFICATION.md` for commands, outcomes and scope. No detector, numerical
power estimate, new canonical item or harvest run is part of this review.
