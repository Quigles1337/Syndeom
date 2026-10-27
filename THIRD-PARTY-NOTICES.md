# Third-party notices

SYNDEOM's original software and technical documentation are licensed under the PolyForm
Noncommercial License 1.0.0 ([`LICENSE`](LICENSE)). This file lists every file in the repository
that is not under that licence, with its licence and attribution.

- The source register [`PROGRAM/SOURCES.md`](PROGRAM/SOURCES.md) holds each work's full citation,
  retrieval record and SHA-256. The S-number in each row points there.
- The list follows a file-by-file licence audit of `main`
  `368bcfb5c05c23536a32bfc15b9537173fb0fef7` (2026-10-27). The owner's rulings of that date are
  recorded in [ADR-0010](PROGRAM/DECISIONS/ADR-0010.md), *Amendments*.

## SYNDEOM's own records, and the quotations inside them

SYNDEOM's own record files are covered by the repository licence. They include:

- the ground-truth manifests, `RECORD.md` files, source maps and derivation recipes under
  `groundtruth/`;
- the evidence, receipts and logs under `analysis/`;
- the registers and decisions under `PROGRAM/`.

The short quotations inside those files remain their authors'. Examples are the sentences that
locate an error in a paper, or a theorem statement quoted from a book.

- The licence of each quoted work is recorded in its S-entry. For a ground-truth item it is also
  in the item's `manifest.json` (`config.licence`).
- Where a manifest records an original as not redistributable, the original is not in this
  repository. Only its locator and SHA-256 are.

## Unmodified third-party copies

Each file is the exact file retrieved from its primary source, unmodified. Its SHA-256 is in the
S-entry.

### Creative Commons Attribution 4.0 International (`http://creativecommons.org/licenses/by/4.0/`)

The full author lists are on each work's arXiv abstract page.

| File | Work and attribution | Source |
|---|---|---|
| `sources/arxiv-2501.03124v5-prmbench.pdf` | Mingyang Song and four co-authors (last: Yu Cheng), "PRMBench: A Fine-grained and Challenging Benchmark for Process-Level Reward Models", arXiv:2501.03124v5 | S-0010 |
| `sources/arxiv-2510.13744v1-hard2verify.pdf` | Shrey Pandit, Austin Xu, Xuan-Phi Nguyen, Yifei Ming, Caiming Xiong, Shafiq Joty, "Hard2Verify: A Step-Level Verification Benchmark for Open-Ended Frontier Math", arXiv:2510.13744v1 | S-0011 |
| `sources/arxiv-2607.11849v2-advancedmathbench.pdf` | Lingkai Kong and thirteen co-authors (last: Kai Chen), "AdvancedMathBench: A Benchmark Suite for Advanced Mathematical Proof Generation and Verification", arXiv:2607.11849v2 | S-0012 |
| `sources/arxiv-2605.12524v1-proofgrid.pdf` | Konstantine Arkoudas, Serafim Batzoglou, "Stress-Testing the Reasoning Competence of LLMs With Proofs Under Minimal Formalism", arXiv:2605.12524v1 | S-0013 |
| `sources/arxiv-2605.10141v1-formalrewardbench.pdf` | Zeynel A. Uluşan and three co-authors (last: Gözde Gül Şahin), "FormalRewardBench: A Benchmark for Formal Theorem Proving Reward Models", arXiv:2605.10141v1 | S-0014 |
| `sources/arxiv-2605.20531v3-pseudo-formalization.pdf` | Slim Barkallah and four co-authors (last: Tengyu Ma), "Pseudo-Formalization for Automatic Proof Verification", arXiv:2605.20531v3 | S-0015 |
| `sources/arxiv-2606.29493v1-faults-in-our-formal-benchmarking.pdf` | Pawan Sasanka Ammanamanchi and two co-authors (last: Stella Biderman), "Faults in Our Formal Benchmarking: Dataset Defects and Evaluation Failures in Lean Theorem Proving", arXiv:2606.29493v1 | S-0016 |
| `groundtruth/s1_ingest-v0.1.0-15ab865466d6/original-0706.0357v70.tar.gz`, `groundtruth/s1_ingest-v0.1.0-63e23a952084/original-0706.0357v70.tar.gz`, `analysis/s1-first-parent-status/candidate/s1_ingest-v0.1.0-15ab865466d6/original-0706.0357v70.tar.gz` (identical bytes) | Hisanobu Shinya, "On a new approach to the Riemann hypothesis", arXiv:0706.0357v70, the e-print source | S-0024 |

### Creative Commons Zero v1.0 Universal

| File | Work and attribution | Source |
|---|---|---|
| `sources/mathandai-severe-misalignment-2026-09-11-zenodo-22737751.pdf` | "A Severe Misalignment of AI in Mathematics", 11 September 2026, by the 25 creators listed on Zenodo record 22737751 (DOI 10.5281/zenodo.22737750) | S-0004 |

### Creative Commons Attribution Non Commercial 4.0 International

Non-commercial use only.

| File | Work and attribution | Source |
|---|---|---|
| `sources/leiden-declaration-2026-06-02-zenodo-20302944.pdf` | Jarod Alper, Michael Barany, Alain Chavarri Villarello, Sander Dahmen, Walter Dean, Karthik Ganapathy, Michael Harris, David Holmes, Mateja Jamnik, Steven Kelk, Bryna Kra, Ursula Martin, Bartosz Naskręcki, Rodrigo Ochigame, Jim Portegies, Johannes Schmitt, "Leiden Declaration on Artificial Intelligence and Mathematics", 2 June 2026, Zenodo record 20302944 (DOI 10.5281/zenodo.20302944) | S-0005 |

This file is **not included in the Gate S1 registration deposit** (the owner's ruling of
2026-10-27). S-0005 keeps its locator and SHA-256, so anyone can fetch and verify the original.

## Apache License 2.0

The licence text is [`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt).

| File | What it is, and attribution | Record |
|---|---|---|
| `groundtruth/s1_pair-v0.1.0-62fe0c816a6b/Proof.lean`, `analysis/s1-2/Proof.lean` (identical bytes) | The S1.2 pair's SYNDEOM alias wrapper. The pair's manifests record it as Apache-2.0. It names the Mathlib declaration `Nat.infinite_setOfPred_prime` (Mathlib, Apache-2.0). | ADR-0010 D3; S-0068 |
| `analysis/s1-2/evidence/candidate-audit.txt`, `analysis/s1-2/correction/evidence/candidate-audit.txt`, `analysis/s1-2/implementation/evidence/production/logs/emitted-type-axiom-audit.txt` (identical bytes) | Lean's printed types and bodies of Mathlib declarations, from Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` on Lean `v4.34.1`. Mathlib and the Lean 4 toolchain are Apache-2.0. The selected module's header credits Microsoft Corporation (2015) and Leonardo de Moura, Jeremy Avigad and Mario Carneiro. Each other printed declaration keeps its own module's credits. | S-0068; S-0008 |

## The licence texts

| File | What it is | Source |
|---|---|---|
| `LICENSE` | The Required Notice line (Copyright 2026 Alexander David Zalewski), then the PolyForm Noncommercial License 1.0.0 text by the PolyForm Project's contributors, unmodified. The project's README: "Each contributor licenses you to do everything with PolyForm licenses that would otherwise infringe that contributor's copyright in it." | S-0073 |
| `LICENSES/Apache-2.0.txt` | The Apache License 2.0 text (The Apache Software Foundation), unmodified. It is the same text, byte for byte, that was this repository's `LICENSE` through `c9372b1`. | S-0019 |

## Not licensed for redistribution

| File | What it is | Record |
|---|---|---|
| `PROGRAM/ADVISORY-2026-10-13-MNEMOSYNE-CASES.md` | Contains material from DARQ MNEMOSYNE, separately controlled DARQ Labs research recorded as `PROPRIETARY_ALL_RIGHTS_RESERVED`. The body was produced by ChatGPT (OpenAI) at the owner's request and relayed verbatim, with a verification record by Claude (Anthropic). It is outside SYNDEOM's licence, and no redistribution permission is recorded. | ADR-0000 Addendum 2 R1(b); ADR-0010 D2(b) |

This file is **not included in the Gate S1 registration deposit** (the owner's ruling of
2026-10-27).

## Not in this repository

The build-time dependencies are fetched from their pins and are not copied here. Their licences
are recorded in the S-entries named below.

- The Lean 4 toolchain and Mathlib with its Lake dependencies: pinned in `lean/lean-toolchain` and
  `lean/lake-manifest.json`; licences in S-0008 and S-0068.
- The Rust crates: pinned in `engine/Cargo.lock`; licences in S-0008 and S-0066.

## Earlier versions

Every commit through `c9372b1355a6d687dec6287f908639ac670db659` was distributed under the Apache
License 2.0, whose text is `LICENSES/Apache-2.0.txt`. Copies obtained under it remain under it
(ADR-0010 D3).
