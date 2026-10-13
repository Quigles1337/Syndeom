# SOURCES — citations and data provenance (SD11, SD16)

An entry here records what the program *has been told exists* and what it *has actually retrieved*.
Every entry carries one of these states:

- **UNVERIFIED** — named (by the genesis prompt, a sweep, or Al) but not yet retrieved; nothing
  beyond the naming is recorded; a note for Al says what is needed.
- **VERIFIED** — retrieved this-session and read from a primary source, with full citation, URL,
  access date, licence terms, and SHA-256 of any stored copy.
- **SURVEYED** — existence, coverage, and licence noted; nothing ingested.
- **INGESTED** — data imported into the repo under the recorded licence, with audit trail.

Rules: a citation exists only after retrieval and verification this-session (SD11). Licence terms are
recorded *before* any data ingestion (SD16); ambiguity → escalate to Al, do not ingest. A stored copy
is committed to the repo only when its licence permits redistribution (this repo is public);
otherwise the entry records URL + access date + SHA-256 of the local copy and notes that the copy is
not committed (FRIEDMAN's storage rule, adopted). Stored copies live under `sources/`. If the
session has no web access, everything stays UNVERIFIED. Never fabricate a citation.

Any founding sweep report placed under `PROGRAM/` is a **map, not evidence**: it may suggest what to
retrieve; nothing enters this register from it.

Retrieval notes (2026-10-12): `curl` with a browser user-agent retrieved openai.com, agmai.org,
ahmath.org, mathandai.org, leidendeclaration.ai, terrytao.wordpress.com, arxiv.org and Zenodo
record pages (HTTP 200); the harness fetch tool received HTTP 403 from openai.com; both tools
received HTTP 403 from ias.edu (Cloudflare interstitial); the Zenodo REST API returned 403 to curl
while record pages returned 200. GitHub content was read through the authenticated GitHub API.
Every SHA-256 below is over the exact bytes retrieved at the stated time; web pages are dynamic, so a
page digest identifies *this* copy, not the page for all time.

---

## S-0001 — OpenAI, "Sharing AI progress in mathematics" (6 October 2026) and the repository `openai/math` (CLAIM-0001; the S5 contact corpus)

- **State:** VERIFIED (post and repository read first-hand). Nothing INGESTED — ingestion of this
  corpus is S5 work under `contact/` (SD8).
- **As named:** genesis prompt, "the Oct 6 2026 OpenAI manuscript release and its verification
  counts"; Al (Session 2): the post title and the repository `openai/math`, with the instruction to
  read the licence from the repository and derive counts from its contents listing and Lean
  catalogue, recording the unit with every number. Both confirmed below.
- **The post.** OpenAI, "Sharing AI progress in mathematics", labelled "Research · Publication",
  dated "October 6, 2026" on the page, author line "OpenAI".
  URL <https://openai.com/index/sharing-ai-progress-in-mathematics/>. Retrieved 2026-10-12T21:58:52Z
  by curl (HTTP 200, 387,243 bytes, SHA-256
  `51142fc55618c2f01ca4d7620afdf5c58d07f518e51543f141ab160ec150a3b1`); the fetch tool got HTTP
  403, so the page was read from the downloaded bytes. Licence: none stated; **not committed**.
  Read first-hand: "releasing a broad range of new mathematical results produced by an internal
  frontier model"; consultation with "the independent Advisory Group on Mathematics and Artificial
  Intelligence at the Institute for Advanced Study" and reliance on "their advice and public
  recommendations" (the page links `https://agmai.org/` and `https://agmai.org/general-sep29/`);
  publication "in a GitHub repository, with protocols for paper revisions and citations" (links
  `https://github.com/openai/math`); "formalizations of many of the proofs in Lean"; "10 summaries
  of the model's reasoning, estimations of compute spent in terms of Pro usage on ChatGPT, and
  statistics about the number of attempted problems"; "The average result used the equivalent
  compute of roughly three hours of ChatGPT Pro thinking." **The post states no manuscript, family,
  or verification counts.**
- **The repository.** <https://github.com/openai/math>, public, created 2026-10-06T21:47:02Z,
  default branch `main`, read through the GitHub API between 2026-10-12T21:57Z and 22:04Z at tip
  `1581d4cbdd69a0faf34b2089614fe2a814316a6a` (2026-10-12T21:55:24Z). Full history (five commits):
  `adc7f1241b` "Initial commit" 2026-10-06T21:58:50Z; `301488868b` "Update manuscripts and Lean
  formalizations" 2026-10-08T05:03:50Z, merged as PR #1 `fd4aeeb2ee` 2026-10-08T05:20:01Z (1,185
  files changed, +324,499/−17,340; PR body: "Updates manuscripts, the catalog and overview, and
  withdrawal notices and archive links. Adds Lean formalizations and supporting results … **Lean
  builds were not run for this synchronization.**"); `2710f995ca` and `1581d4cbdd` (PR #2, locks PR
  conversations) 2026-10-12. Top level: `CONTENTS.md`, `LICENSE`, `README.md`, `history.md`,
  `overview.pdf`, `overview.tex`, `lean/`, `preprints/`, `reasoning_traces/`, `.github/`.
- **Licence:** **Apache License 2.0** — read from the repository's own `LICENSE` (git blob
  `261eeb9e9f8b2b4b0d119366dda99c6fd7d35c64`, 11,357 bytes), from `lean/LICENSE` (Apache 2.0
  header), and from `lean/formalization.yaml` (`project.license: "Apache-2.0"`). Redistribution is
  permitted with attribution; nothing is copied into this repository before S5 (SD8).
- **Files read, with digests** (git blob id; SHA-256 over the decoded bytes):
  - `README.md` 4,190 bytes — blob `50feb63d396138f30dc1ff1e0af121d0263bf5a3`, SHA-256
    `5e8878ee04866f52ec942852552b9c28db278f65691c7ab49a87c9e93a9199fd`
  - `history.md` 2,281 bytes — blob `693874f398905b1bc59e01b898b223ae9956bfc2`, SHA-256
    `41c2a2d470014b3628695c7aabf5fd0c93f6f687621fca172062f22b59ed6a1e` (see S-0006)
  - `CONTENTS.md` 638,727 bytes at the tip — blob `2c68c086fff36a5806c936ad1659cf1a8d0b6dd9`,
    SHA-256 `cc7c0b8a1cdedd845a777e77cf682657308503daf8346c1349e06e9259d063c2`; at the initial
    commit blob `8148d2c9077e611d5665a2ded93da68e5b608f0f` (headline and counts read, bytes not
    hashed)
  - `lean/formalization.yaml` 79,482 bytes — blob `275eee8cfab7ce49d3a5e88f02cbb8f28fe6fee6`,
    SHA-256 `5a8a164259591208886cd018416e61552665265ec9e20cff636f9d51117c2278`
  - `lean/README.md`, `lean/ComparatorChallenges/README.md`, `lean/lean-toolchain`,
    `lean/lakefile.lean`, `lean/lake-manifest.json` — read, not hashed.
- **Counts derived from the repository itself, units explicit:**

| Quantity | Value | Unit | Where |
|---|---|---|---|
| Manuscripts at the initial commit (2026-10-06) | 722 | manuscripts (distinct preprint directories linked) | `CONTENTS.md@adc7f1241b` headline "722 manuscripts covering 372 result families" and link count; `README.md@adc7f1241b` "722 manuscripts organized into 372 families" |
| Result families | 372 | families (result headers numbered 001–377, 372 distinct) | `CONTENTS.md`, both commits |
| Manuscripts at the tip (after 2026-10-08) | 719 | manuscripts | `CONTENTS.md@1581d4cbdd` headline and link count; `README.md` "719 manuscripts organized into 372 families" |
| Preprint directories at the tip | 749 | directories | contents API; 30 more than 719, consistent with the 3 withdrawn plus 27 revised/updated manuscripts retaining prior editions per `history.md` (executor inference, not checked directory by directory) |
| Top-line results formalized | 300 / 719 ≈ 42% | numerator phrased "top-line results", denominator = manuscripts | `history.md` ("This brings the total percentage of top-line results formalized to 300 / 719 = ~42%"); `README.md` "~42% top-line results formalized" |
| Papers with a formalized main result | 173 | papers (`sources` entries) | `lean/formalization.yaml` schema version v0.4 |
| Related formalizations | 231 entries (200 named declarations) | entries | `lean/formalization.yaml` `related_formalizations` |
| Families with a Lean scope note | 242 | families | `lean/docs/` (242 files; 242 `(lean/docs/NNN.md)` links in `CONTENTS.md`) |
| Reasoning summaries | 10 | PDFs (11 files including one `.tex`) | `reasoning_traces/`; README table of 10 families |
| Problems posed | ≈ 4,000 | problems | `README.md` "approximately 4,000 problems" |
| Compute per result | ≈ 3 hours | ChatGPT Pro thinking | `README.md`; the post |
| Review status of the formalizations | "unchecked" | — | `lean/formalization.yaml` `review: status: unchecked`; `automation: methods: - method: agent` |

  The three formalization quantities (300 top-line results; 173 catalogued papers; 242 families with
  scope notes) are different units and the repository does not reconcile them; SYNDEOM records all
  three and reconciles none.
- **Corpus toolchain (recorded for the Q8 consequence; acted on at S5 ingest):**
  `lean/lean-toolchain` = `leanprover/lean4:v4.34.1`; `lakefile.lean` sets `fixedToolchain :=
  true`; `lake-manifest.json` (manifest version 1.2.0, package `OAI`) pins Mathlib at
  `d13f23b723b8a846827a245b89c10fc7d3f11612` and roughly forty further git dependencies (among them
  checkdecls, doc-gen4, PrimeCert, LeanArchitect, leancert, PrimeNumberTheoremAnd, carleson,
  StrongPNT, SphereEversion, ClassFieldTheory, and a dozen `lana-agents/*` repositories). The
  verification instructions name `comparator`, `landrun`, `lean4export`, `lake update` and `lake
  exe cache get`. The recursive tree at the tip is truncated by the API at 58,728 entries (57,285
  `.lean` files).
- **Not evidence:** secondary coverage returned by web search (dealroom, emergent, gigazine,
  unite.ai, interestingengineering, futurism, …) was not read for facts and is not cited.

## S-0002 — AGMAI, "Responsible Release of AI-Generated Mathematics" (29 September 2026) (CLAIM-0002)

- **State:** VERIFIED (web page and PDF retrieved and read; licence unstated → copies not committed)
- **As named:** genesis prompt "the AGMAI guidelines"; Al (Session 2): AGMAI = Advisory Group on
  Mathematics and Artificial Intelligence, hosted at the Institute for Advanced Study; the title
  and date above. Expansion, title, and date confirmed verbatim by the primary.
- **Issuing body:** Advisory Group on Mathematics and Artificial Intelligence, <https://agmai.org/>
  (index page retrieved 2026-10-12T22:01:29Z, HTTP 200, 70,597 bytes, SHA-256
  `8b17d0f550d0c34b119d1a2fc6a74f6de61ecbdf8ba401cedf3b6638f539b64c`). Site header: "Advisory Group
  on Mathematics and Artificial Intelligence / Institute for Advanced Study", linking
  <https://www.ias.edu/nccr/projects#agmai>. Members listed: François Charles (ENS-PSL), Camillo De
  Lellis (IAS, GSSI), Timothy Gowers (Collège de France, Cambridge), Martin Hairer (EPFL, Imperial
  College London), Nikhil Srivastava (Berkeley, Simons Institute), Ulrike Tillmann (Oxford), Ravi
  Vakil (Stanford), Edward Witten (IAS), Melanie Matchett Wood (Harvard). Stated: "This group
  operates independently of any AI company and members do not accept payment for this work." The
  index also carries "On OpenAI's Release of Mathematical Results", October 6, 2026.
- **The document:** "Responsible Release of AI-Generated Mathematics", September 29, 2026.
  Web page <https://agmai.org/general-sep29/> (retrieved 2026-10-12T22:03:36Z, HTTP 200, 80,290
  bytes, SHA-256 `0bdfff1f80ec4a760defac4e44d5511cf742ee9ff009ab1145ef3f65f6b3aa71`). PDF
  <https://agmai.org/wp-content/uploads/2026/09/recommendations.pdf> (retrieved 22:06:44Z, HTTP 200,
  119,758 bytes, SHA-256 `25c621e9a4609dc43538db3bcd493c9208e541d267998e9983017274ed03f130`).
  Structure read first-hand: an opening statement that the group does not endorse testing advanced
  mathematical problems on proprietary models and asks labs to stop; §1 Background (three
  principles); §2 Responsible release — 2.A papers a human understands, 2.B papers not yet
  understood by anybody; Step I initial release (five points: improve the written version with LLM
  help; deposit in scholarly repositories; make public the model name, the prompts, a summarized
  chain of thought; "As far as possible, a proof released by an AI lab should be formalized", with
  formalization artefacts meeting community standards including a comparator challenge file and a
  `formalization.yaml`; document how AI was used); Step II supporting human understanding (three
  points); §3 Ensuring broad access. Survey: "over 600 replies" (index: "over six hundred survey
  responses").
- **IAS affiliation:** the linked IAS project page returned HTTP 403 (Cloudflare "Just a moment…")
  to curl and to the fetch tool on 2026-10-12 (5,451 bytes, SHA-256
  `08c1650b105dd51a3d23958f0c106bf8ba92f7ffb86d50f632d3b4cf11c4a744`). The affiliation is
  therefore verified from the group's own site, not from IAS's.
- **Licence:** none stated on the pages or seen in the PDF as read; **copies not committed**.

## S-0003 — Association for Human Mathematics, "AHM Statement on OpenAI's October 6 Release of Mathematical Documents" (CLAIM-0003)

- **State:** VERIFIED (primary page retrieved and read; Tao's repost retrieved for the date)
- **As named:** genesis prompt "the AHM statement"; Al (Session 2): AHM = Association for Human
  Mathematics; dated 7 October 2026 on its statements page; republished the same day as a guest post
  on Terence Tao's blog; signed by its Communications Working Group; the AHM page is primary.
  Expansion, signature, and repost confirmed; **the date is not on the primary page** (below).
- **Primary:** <https://www.ahmath.org/statements> (retrieved 2026-10-12T22:03:36Z, HTTP 200,
  141,571 bytes, SHA-256 `c4b8a4b6a489afa00913ab4d804cd8b38dd067cf0e73f7d04d68f2a0234c10d1`).
  Title "AHM Statement on OpenAI's October 6 Release of Mathematical Documents"; signature
  "Association for Human Mathematics / Communications Working Group". The page carries no date;
  the statement opens "Yesterday, on October 6th, 2026, OpenAI – which is currently defending
  lawsuits against accusations of illegal plagiarism, copyright infringement, and trademark
  dilution – released a repository of manuscripts purporting to contain solutions to a number of
  high-profile problems in mathematics." Four paragraphs; includes "Releasing over 700 files at
  once is not a demonstration of scholarship, but a demonstration of power." and "We urge
  mathematicians to discontinue their work with OpenAI and to return to a vision of science that
  centers human understanding." The page also links member comments submitted to AGMAI.
- **Date carrier (secondary):** Terence Tao, "AHM Statement on OpenAI's October 6 Release of
  Mathematical Documents", *What's new*, 7 October, 2026, "guest blog": "[This is a guest post by
  the Association for Human Mathematics, reposted from their statements page …]", linking
  `https://www.ahmath.org/` and `https://www.ahmath.org/statements`.
  <https://terrytao.wordpress.com/2026/10/07/ahm-statement-on-openais-october-6-release-of-mathematical-documents/>
  (retrieved 22:03:36Z, HTTP 200, 368,984 bytes, SHA-256
  `82b5f8653fbf72f7e2cd821a60fd990777c86879746b4e79e42a3605fc0b6239`). So "7 October" is the
  repost date and the day after the event the statement describes.
- **Licence:** none stated on either page; **copies not committed**.

## S-0004 — "A Severe Misalignment of AI in Mathematics" (11 September 2026) (CLAIM-0004)

- **State:** INGESTED (the Zenodo-deposited PDF, CC0) + VERIFIED (page, record, repost)
- **As named:** genesis prompt "the Fields Medalist letter"; Al (Session 2): the title, 11 September
  2026, 25 Fields Medalists, mathandai.org, same-day post on Tao's blog; record the signatory count
  as of the access date. All confirmed; the count as of access is 28 (below).
- **Primary page:** <https://mathandai.org/> (page title "Declaration — Math and AI"; retrieved
  2026-10-12T21:58:52Z, HTTP 200, 10,921 bytes, SHA-256
  `e5224dceccf4b29b3735644e1d6ae588be8e704920be986c383815fe1c4a047e`). "A Severe Misalignment of
  AI in Mathematics", "Published 11 September 2026 · DOI 10.5281/zenodo.22737750"; offered in de,
  en, es, fr, it, ja, ru, zh. **Signatories listed as of 2026-10-12: 28**, each labelled
  "(Fields Medal YYYY)": Artur Avila, Manjul Bhargava, Caucher Birkar, Pierre Deligne, Yu Deng,
  Simon Donaldson, Vladimir Drinfeld, Hugo Duminil-Copin, Alessio Figalli, Martin Hairer, June Huh,
  Maxim Kontsevich, Elon Lindenstrauss, Pierre-Louis Lions, Grigory Margulis, James Maynard, Curtis
  McMullen, Shigefumi Mori, David Mumford, Ngô Bảo Châu, Andrei Okounkov, Peter Scholze, Stanislav
  Smirnov, Terence Tao, Maryna Viazovska, Cédric Villani, Wendelin Werner, Efim Zelmanov.
  Endorsements are a separate list: <https://mathandai.org/endorsers.php> shows "8,310 endorsers"
  as of 2026-10-12T22:06:44Z (66,358 bytes, SHA-256
  `b566e9bcbee4d1bd60c316a67c066b9ab1db7b0ca9e78938a1849e2dc1fcd3da`).
- **Zenodo record:** DOI 10.5281/zenodo.22737750 resolves to record 22737751 (version v1):
  "Published September 11, 2026"; 25 authors/creators (the 28 above minus Drinfeld, Margulis,
  Mumford); **licence Creative Commons Zero v1.0 Universal**; file `mathandai.pdf`, 29.3 kB,
  MD5 `de86427726663561af64af8642370282`. Record page retrieved 22:03:36Z (132,140 bytes,
  SHA-256 `a7b5e700bce301b95e93f3510d2adebd940753f7c2837e8d2cba019b3bde63e2`).
- **Stored copy (CC0):** `sources/mathandai-severe-misalignment-2026-09-11-zenodo-22737751.pdf`,
  29,254 bytes, SHA-256 `32e3afcefd9ac5947b965ec7811ac5df05d95513b77877cb2f023f1c31ac2fa5`; its
  MD5 reproduces the record's `de86427726663561af64af8642370282`.
- **Same-day repost (secondary):** Terence Tao, "A Severe Misalignment of AI in Mathematics",
  *What's new*, 11 September, 2026: "I am proud to be among the list of 25 initial signatories —
  all Fields Medallists — to the declaration below…".
  <https://terrytao.wordpress.com/2026/09/11/a-severe-misalignment-of-ai-in-mathematics/>
  (retrieved 22:03:36Z, HTTP 200, 238,628 bytes, SHA-256
  `cdf4e2b330af3f6b01620c4a667050889b1533e74f7e3be6055f072f686ab31b`). Consistent with the
  25-author deposit; Al's "25 Fields Medalists" are the initial signatories.
- **Licence:** page — none stated; deposit — CC0 1.0 (stored copy committed under it).

## S-0005 — "Leiden Declaration on Artificial Intelligence and Mathematics" (2 June 2026) (CLAIM-0005)

- **State:** INGESTED (the Zenodo-deposited PDF, CC BY-NC 4.0) + VERIFIED (page, record, arXiv)
- **As named:** genesis prompt "the Leiden Declaration"; Al (Session 2): the title, 2 June 2026,
  leidendeclaration.ai. Confirmed.
- **Primary page:** <https://leidendeclaration.ai/> (retrieved 2026-10-12T21:58:52Z, HTTP 200,
  71,363 bytes, SHA-256 `b5148e6aae51956bd0663783deb81db6c24c5e78cd1bb3be2f90e2e660e2ced4`).
  "Leiden Declaration on Artificial Intelligence and Mathematics", "2 June 2026 · DOI:
  10.5281/zenodo.20302944"; **"4322 Signatories" as of 2026-10-12**; working group of 16: Jarod
  Alper, Michael Barany, Alain Chavarri Villarello, Sander Dahmen, Walter Dean, Karthik Ganapathy,
  Michael Harris, David Holmes, Mateja Jamnik, Steven Kelk, Bryna Kra, Ursula Martin, Bartosz
  Naskręcki, Rodrigo Ochigame, Jim Portegies, Johannes Schmitt. Structure: values; potential
  threats; recommendations for individual mathematicians (eleven headed items, e.g. "Disclose tool
  use", "Retain the responsibility for correctness", "Affirm the humanity of authorship"), for
  mathematical organizations and not-for-profit funders (eight), for policymakers (four), and for
  commercial artificial intelligence. Links an IMU endorsement PDF (not retrieved).
- **Zenodo record 20302944** (version v1): "Published June 2, 2026"; 16 creators as above;
  **licence Creative Commons Attribution Non Commercial 4.0 International**; file
  `Leiden_Declaration_on_Artificial_Intelligence_and_Mathematics.pdf`, 199.3 kB, MD5
  `22f4a7d18c9ce00dda35def67a847068`; description "This declaration calls for action to address
  the challenges posed by the use of artificial intelligence within mathematics research." Record
  page retrieved 22:03:36Z (123,437 bytes, SHA-256
  `40ade054e6ad5cc3b725b2899bea9b3fec532cde04fec4fd6505761a8109b3b1`).
- **arXiv:** arXiv:2609.35334 [math.HO], v1 submitted 28 Sep 2026, v2 30 Sep 2026, the 16 authors
  above; licence arXiv non-exclusive distribution 1.0 (`licenses/nonexclusive-distrib/1.0/`), which
  grants no third-party redistribution — not stored. Abstract page retrieved 22:03:36Z (45,662
  bytes, SHA-256 `10f6edfe384734a4160e79e7ef04a12b61f4a3705d018ea26b2d460166940a6c`).
- **Stored copy (CC BY-NC 4.0; attribution: the 16 authors above):**
  `sources/leiden-declaration-2026-06-02-zenodo-20302944.pdf`, 199,338 bytes, SHA-256
  `f6c545ebfd69fbcb0fd981da8fc7189950ce56c7d608979a7fc0d97c3de27d22`; its MD5 reproduces the
  record's `22f4a7d18c9ce00dda35def67a847068`.
- **Licence:** page — none stated; deposit — CC BY-NC 4.0 (stored copy committed under it; this
  repository is non-commercial research infrastructure; re-examine at S5.4 publication packaging).

## S-0006 — withdrawals and revisions in `openai/math` dated 7 October 2026 (CLAIM-0007)

- **State:** VERIFIED (existence only) — an SD8 contact-phase item; content read only in S5.
- **As named:** Al (Session 2): "a reported correction or retraction of some manuscripts in the
  6 October release shortly after publication."
- **Primary:** `history.md` in `openai/math` (2,281 bytes; blob `693874f398905b1bc59e01b898b223ae9956bfc2`;
  SHA-256 `41c2a2d470014b3628695c7aabf5fd0c93f6f687621fca172062f22b59ed6a1e`), heading "October 7,
  2026", introduced by commit `301488868b` via PR #1 (merged 2026-10-08T05:20:01Z; the file is
  absent at `adc7f1241b`). Existence facts: sections "Withdrawals" (3 manuscripts withdrawn, with
  notices "explaining the gap and linking to the archived manuscripts"), "Fixes" (14 manuscripts
  revised; 13 further manuscripts updated to cite revised editions), "Additional Formalizations"
  (6 added plus 5 other additions; "300 / 719 = ~42%"). The catalogue's manuscript count fell from
  722 to 719 accordingly. README: "Corrections and revisions will be recorded as new versions, with
  previously released versions remaining accessible."
- **Withheld (SD8):** the names of the withdrawn and revised manuscripts and the nature of the gaps
  are stated in that file and were seen by the executor while establishing existence; they are not
  transcribed here, not committed, and informed no decision. They are read in S5.
- **Licence:** Apache-2.0 (repository); not copied.

---

## S-0007 — RFC 8785, "JSON Canonicalization Scheme (JCS)" (ADR-0001 D5)

- **State:** VERIFIED (the RFC text retrieved from the RFC Editor and read: §3.2.1–3.2.4 and
  Appendix B, the parts ADR-0001 D5 relies on).
- **As named:** FRIEDMAN ADR-0001 D5 and its own S-0004 (read at FRIEDMAN tip `7c3e5dd`); under
  SD14 the standard is adopted by reference, but SD11 requires SYNDEOM to retrieve it itself rather
  than inherit the sibling's citation — done here.
- **Citation (from the document header):** A. Rundgren, B. Jordan, S. Erdtman, "JSON
  Canonicalization Scheme (JCS)", RFC 8785, Independent Submission, Category: Informational, ISSN
  2070-1721, June 2020.
- **Primary:** `https://www.rfc-editor.org/rfc/rfc8785.txt`, accessed 2026-10-13T02:23:50Z (curl,
  HTTP 200), 41,879 bytes, SHA-256 `63d52294eb0e3f0014174288186d388b4ddbf2c67d1ce8af1d9726eb0c3ab240`.
- **Content used:** §3.2.1 whitespace; §3.2.2.3 numbers ("MUST be serialized according to Section
  7.1.12.1 of [ECMA-262], including the 'Note 2' enhancement"; NaN/Infinity "MUST cause a compliant
  JCS implementation to terminate with an appropriate error"); §3.2.3 property sorting ("sorted
  recursively", array order "MUST NOT be changed", names compared in raw unescaped form); §3.2.4
  UTF-8 generation; Appendix B number-serialization samples (the engine's conformance vectors at
  S0.6, as in FRIEDMAN's crate tests).
- **Licence:** "Copyright (c) 2020 IETF Trust and the persons identified as the document authors.
  All rights reserved. This document is subject to BCP 78 and the IETF Trust's Legal Provisions
  Relating to IETF Documents (https://trustee.ietf.org/license-info)". Copy **not committed**
  (digest-only, FRIEDMAN's storage rule); nothing needs redistributing — the engine cites the RFC
  number.

---

## S-0008 — Toolchain and dependency licences at the ADR-0002 pins (SD16; recorded before ingestion)

- **State:** VERIFIED (every licence file read from the pinned checkout or crate metadata on
  2026-10-13 UTC, Session 4). Nothing from these projects is copied into this repository; they are
  dependencies resolved at build time from the pins in `lean/lean-toolchain`,
  `lean/lake-manifest.json`, and `engine/Cargo.lock`.
- **Lean 4 toolchain** `leanprover/lean4:v4.34.1` (release tag commit `5045d00…`; installed by
  elan from `releases.lean-lang.org`): `LICENSE` at the tag begins "Apache License 2.0 (Apache)".
- **Mathlib** `leanprover-community/mathlib4` at `d13f23b723b8a846827a245b89c10fc7d3f11612` (tag
  `v4.34.1`, committed 2026-09-24T20:34:32Z "chore: bump toolchain to v4.34.1"): `LICENSE` blob
  `8dada3edaf50dbc082c9a125058f25def75e625a`, 11,357 bytes, Apache License 2.0. Its
  `lean-toolchain` at that commit reads `leanprover/lean4:v4.34.1` (the source of the toolchain
  pin, Q8).
- **Mathlib's transitive Lake dependencies** (revisions from `lean/lake-manifest.json`, licence
  files read from `lean/.lake/packages/<name>/LICENSE`): `batteries` `f2effa3d803f` Apache-2.0;
  `aesop` `355695d523e4` Apache-2.0; `Qq` (quote4) `6a489d9af5d0` Apache-2.0; `proofwidgets`
  (ProofWidgets4) `106ff4fafc74` Apache-2.0; `importGraph` (import-graph) `e928b7254487`
  Apache-2.0; `LeanSearchClient` `ddf04cf3949f` Apache-2.0; `plausible` `118aa17ee846` Apache-2.0;
  `Cli` (leanprover/lean4-cli) `e92c9f15fdfa` MIT ("Copyright (c) 2021 mhuisi").
- **Rust crates** (`engine/Cargo.lock`, 24 crates, licences from `cargo metadata`): direct —
  `serde 1.0.229` MIT OR Apache-2.0, `serde_json 1.0.154` MIT OR Apache-2.0,
  `serde_json_canonicalizer 0.3.2` MIT, `sha2 0.11.1` MIT OR Apache-2.0, `hex 0.4.3` MIT OR
  Apache-2.0; transitive — `block-buffer 0.12.1`, `cfg-if 1.0.5`, `cpufeatures 0.3.1`,
  `crypto-common 0.2.2`, `digest 0.11.3`, `hybrid-array 0.4.15`, `itoa 1.0.18`, `libc 0.2.190`,
  `proc-macro2 1.0.107`, `quote 1.0.47`, `serde_core 1.0.229`, `serde_derive 1.0.229`,
  `syn 3.0.9`, `typenum 1.20.1` (all MIT OR Apache-2.0); `const-oid 0.10.2` Apache-2.0 OR MIT;
  `ryu-js 1.0.3` Apache-2.0 OR BSL-1.0; `memchr 2.8.3` Unlicense OR MIT; `zmij 1.0.23` MIT;
  `unicode-ident 1.0.27` (MIT OR Apache-2.0) AND Unicode-3.0.
- **Use:** build-time dependencies only; no copy committed; no ambiguity to escalate. Any change
  of pin re-reads the licences (a new transition here).

---

Retrieval notes (Session 5, 2026-10-13 UTC): the ten sources below are the ten the founding sweep
ranks in its Verdict (3) (S-0002, rank 9, was already VERIFIED and is cross-referenced, not
re-entered). Each arXiv paper was retrieved as its abstract page (`arxiv.org/abs/<id>`, HTTP 200,
03:23:11–13Z), its latest-version PDF (`arxiv.org/pdf/<id>`, HTTP 200, 03:23:39–46Z) and its
arXiv HTML rendering; title, authors, dates, version count and licence were read from the abstract
page's metadata; the PDF text was extracted with poppler `pdftotext` and read against CLAIM-0006 as
worded. Every SHA-256 is over the PDF bytes retrieved. Licences: seven papers carry CC BY 4.0 and
their PDFs are stored under `sources/` with attribution; two carry the arXiv perpetual
non-exclusive distribution licence, which does not permit redistribution, so they are digest-only.
"Bears on CLAIM-0006" states, in the source's own terms, what it reports along the two axes the
claim turns on — error/flaw class and proof length — and at what level.

---

## S-0009 — Li et al., "Evaluating Mathematical Reasoning of Large Language Models: A Focus on Error Identification and Correction" (sweep rank 1)

- **State:** VERIFIED (abstract page and PDF retrieved and read; digest-only).
- **Citation (from the arXiv metadata):** Xiaoyuan Li and five co-authors (last: Fuli Feng),
  arXiv:2406.00755 (v1 only; 2024-06-02); comments "ACL Findings 2024"; 45 pages.
- **Primary:** `https://arxiv.org/abs/2406.00755` (42,235 bytes, SHA-256 `129b84a879b01a75…`);
  `https://arxiv.org/pdf/2406.00755` (1,725,936 bytes, SHA-256
  `e90f5c40e8c8c89c5c41f5ecf558fcbd6a94b9eee7b6656edee244408fb684d6`).
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` — **not redistributable; no
  copy committed.**
- **Bears on CLAIM-0006:** Level: math word problems — GSM8K ("elementary-level") and MathQA
  ("GRE-level"), not proofs. Data: "we use GPT-4 to convert ground-truth solutions into wrong
  solutions containing specific error types" (Fig. 2) — nine error types (Table 1), "single-step
  and single-type errors" only. Method: eleven LLMs as examiners on four tasks (error presence,
  error step, error type, error correction). **By error type: yes** — Tables 2–4 and 15–16 report
  accuracy per error type ("calculation error is the most challenging to identify and correct, with
  an average accuracy of only 26.3%"). **By length: yes, for one type only** — Appendix E.1: "We
  examine the influence of step count on EP and EC. For calculation error, we select 50 solutions
  ranging from 2 to 9 steps to transform. It can be observed that the accuracy of identification and
  correction is not significantly affected by the number of steps in Figure 10 to 13." By error
  position: Figures 14–17 (8-step calculation-error problems, "also not significantly affected").
  **Verdict for the claim:** the only retrieved source crossing a flaw class with length, and it
  does so for one class, on word-problem solutions rewritten by GPT-4, not on mathematical proofs
  and not at research level. It does not falsify the claim as worded; it bounds the claim from
  below.

---

## S-0010 — Song et al., "PRMBench: A Fine-grained and Challenging Benchmark for Process-Level Reward Models" (sweep rank 2)

- **State:** INGESTED (PDF under `sources/`, CC BY 4.0) + VERIFIED.
- **Citation:** Mingyang Song and four co-authors (last: Yu Cheng), arXiv:2501.03124 (v5,
  2025-06-28; first 2025-01-06); comments "Accepted by ACL 2025 Main"; 48 pages.
- **Primary:** `https://arxiv.org/abs/2501.03124` (SHA-256 `130b26fe04669dab…`);
  `https://arxiv.org/pdf/2501.03124` (3,431,116 bytes, SHA-256
  `31b43eac0683ee01a97edf8370bb521281a6eee639985902b550b9fac5f7d5f8`).
- **Licence:** CC BY 4.0. **Stored copy:** `sources/arxiv-2501.03124v5-prmbench.pdf` (same
  bytes). Attribution: the authors above, arXiv:2501.03124, CC BY 4.0.
- **Bears on CLAIM-0006:** Level: "built upon PRM800K (Lightman et al., 2023)" — MATH-level
  problems; errors are **LLM-injected** into correct solutions ("leveraging state-of-the-art LLMs to
  introduce various types of errors"), 6,216 instances, 83,456 step labels, with a 92% human
  qualification rate on a sample. **By error class: yes** — three categories and nine
  sub-categories (non-redundancy, non-circular logic, empirical soundness, step consistency,
  domain consistency, confidence invariance, prerequisite sensitivity, deception resistance,
  multi-solution consistency; Table 2), scores per sub-category for 25 models (Table 3, Tables
  15–23). **By length: no** — Table 2 lists "Avg. Steps" per category as a statistic only; Figure 4
  reports "error-detection accuracy across different error steps" (error **position**), with
  "proprietary LLMs maintain stable performance across different error step positions" while PRMs
  improve as the position increases (Takeaway 2). **Verdict:** class axis only, at competition
  level; an injection precedent whose circularity, redundancy and prerequisite classes overlap
  FLAW-0003, FLAW-0013 and FLAW-0005. Does not falsify.

---

## S-0011 — Pandit et al., "Hard2Verify: A Step-Level Verification Benchmark for Open-Ended Frontier Math" (sweep rank 3)

- **State:** INGESTED (PDF under `sources/`, CC BY 4.0) + VERIFIED (arXiv and the ACL Anthology
  page).
- **Citation:** Shrey Pandit, Austin Xu, Xuan-Phi Nguyen, Yifei Ming, Caiming Xiong, Shafiq Joty
  (Salesforce AI Research), arXiv:2510.13744 (v1, 2025-10-15; "21 pages, 8 figures, 5 tables").
  Published in *Proceedings of the 64th Annual Meeting of the Association for Computational
  Linguistics (Volume 1: Long Papers)*, Anthology ID 2026.acl-long.1031, pages 22502–22517 (read
  from `https://aclanthology.org/2026.acl-long.1031/`, HTTP 200 at 03:27:03Z, 45,019 bytes,
  SHA-256 `50d6451963a557a5…`).
- **Primary:** `https://arxiv.org/abs/2510.13744` (SHA-256 `d3d24f9d28926f32…`);
  `https://arxiv.org/pdf/2510.13744` (506,133 bytes, SHA-256
  `a1fb744600ddb478a3e8102826f1d5b412874a0cfe61f986f31d646df5497ae2`).
- **Licence:** CC BY 4.0. **Stored copy:** `sources/arxiv-2510.13744v1-hard2verify.pdf`.
- **Bears on CLAIM-0006:** Level: "very recent, challenging, and open-ended math questions" from
  2024+ Olympiads (IMO shortlists, Putnam, EGMO, BMO, CMO, USAJMO, INMO, USAMO; 80 problems) —
  competition level, not research level. Data: **AI-generated proofs** — 200 responses from GPT-5
  (high), Gemini 2.5 Pro, Claude Sonnet 4 (Thinking) — with step-level human labels: 52 annotators,
  35 with graduate degrees, "over 500 hours of human labor"; "on average, a model response took 90
  minutes to grade and 63 minutes to review, with the longest response taking up to 4 hours"
  (Appendix D) — a cost datapoint for the S3.4 model. Tasks: Step-Level, Response-Level, ErrorID;
  29 verifiers; Balanced Accuracy / Balanced F1 (Table 2). **By error type: no** — the annotation
  guidelines name kinds (computational, logical, hand-waviness) but no result is broken down by
  them. **By length: no** — Figure 3 counts correct/incorrect labels **by step index** ("models …
  typically start to get derailed after a few steps"); Figure 6 breaks TPR/TNR down by generator;
  no accuracy-by-length. Finding: "Current verifiers are too generous, with TPR rate tending
  towards 1 and TNR tending toward 0." **Verdict:** frontier-level AI-generated ground truth with
  neither breakdown. Does not falsify.

---

## S-0012 — Kong et al., "AdvancedMathBench: A Benchmark Suite for Advanced Mathematical Proof Generation and Verification" (sweep rank 4)

- **State:** INGESTED (PDF under `sources/`, CC BY 4.0) + VERIFIED.
- **Citation:** Lingkai Kong and thirteen co-authors (last: Kai Chen; Shanghai AI Laboratory and
  others), arXiv:2607.11849 **v2** (2026-09-29; v1 2026-07-13); 26 pages. The version matters (the
  sweep's hint): the error-category analysis quoted below was read in v2.
- **Primary:** `https://arxiv.org/abs/2607.11849` (SHA-256 `bf96b6b571234d1c…`);
  `https://arxiv.org/pdf/2607.11849` (1,584,125 bytes, SHA-256
  `9c109ddce12cf343a7735b6b3103cf3def09ab1ad1e327220eb19bdfa47029ec`); HTML rendering
  `https://arxiv.org/html/2607.11849` read for one sentence the PDF extraction split.
- **Licence:** CC BY 4.0. **Stored copy:** `sources/arxiv-2607.11849v2-advancedmathbench.pdf`.
- **Bears on CLAIM-0006:** Level: undergraduate (UG) and doctoral qualifying-exam (QE) problems —
  ProverBench 245 problems; VerifierBench "888 model-generated proof trajectories paired with
  expert ground truth" (**AI-generated proofs, expert-labelled**). Verifier results: TPR, TNR,
  Balanced F1 (Table 1); "The best-performing model, DeepSeek-V4-Pro, achieves only 65.1
  Meta-Verification Balanced F1." **By error class: yes, partially** — a generation-error taxonomy
  of four groups and ten types (strategy-level, assumption, reasoning, structural; Appendix E.1);
  Table 2 gives the distribution of these categories among failed generations; for verification,
  "for false positives, we … classify the underlying missed defect using the error taxonomy"
  (E.3), Table 3/Table 9 give FN/FP distributions, and the HTML text states "Reasoning and
  structural errors together account for nearly 90% of the invalid proofs missed by both models."
  These are distributions of *missed* errors by category, not detection rates per category.
  **By length: no** — the only "length" in the paper is the models' output budget. **Verdict:**
  class-axis evidence at advanced (not research) level on AI-generated proofs; no length axis.
  Does not falsify.

---

## S-0013 — Arkoudas and Batzoglou, "Stress-Testing the Reasoning Competence of LLMs With Proofs Under Minimal Formalism" (ProofGrid; sweep rank 5)

- **State:** INGESTED (PDF under `sources/`, CC BY 4.0) + VERIFIED.
- **Citation:** Konstantine Arkoudas, Serafim Batzoglou, arXiv:2605.12524 (v1; submission history
  "[v1] Tue, 7 Apr 2026 01:19:41 UTC"); 166 pages.
- **Primary:** `https://arxiv.org/abs/2605.12524` (SHA-256 `16bdaf7bcd76ac4b…`);
  `https://arxiv.org/pdf/2605.12524` (7,149,612 bytes, SHA-256
  `3bfaf7b715255eb429660f1f25a0624e55a39af119ac9244997d484154245bbc`).
- **Licence:** CC BY 4.0. **Stored copy:** `sources/arxiv-2605.12524v1-proofgrid.pdf`.
- **Bears on CLAIM-0006:** Level: formal natural deduction in a minimal language (NDL) for
  propositional and first-order logic, equational logic — **not research mathematics**. Proof
  checking task PL1-PC: 300 proofs, "38 of the proofs were generated by a theorem prover written in
  Athena, while the rest were produced by Gemini-2.5-Pro, o3, GPT-4o, and Claude Sonnet 3.5. The
  proofs written by the theorem prover were correct by construction, so synthetic errors were
  randomly injected in some of them. The model-written proofs were not corrupted because they
  contain organic errors of their own" — so **partly AI-generated proofs with organic errors**;
  balanced correct/incorrect. **By length: yes** — "PL1-PC accuracies … as a function of proof
  length" (Figures 7–8, variable-width bins with n ≥ 30), likewise PL3-PC (Figure 32: "proof size
  is not a strong degrader of PL3-PC performance … flat NDL0 proofs with no more than a few hundred
  steps"), proof masking (Figures 11–12) and gap filling (Figures 17–18, "many hitting near-zero
  accuracy around 40 lines"). **A detector near chance at long length:** PL1-PC-c, "50 longer
  proofs … lengths ranging from 720 to 987" (the paper's proof-length unit; its sibling challenge
  set is described in lines), "9 of the 50" minimally corrupted with exactly one injected error;
  only the four strongest models evaluated; "only two of them manage to perform better than
  flipping a coin on the guess-only version of the task"; "even the top-performing model, Claude
  Opus 4.5, performs only 38% better (69-50/50) than random guessing" and below the 82%
  majority-class baseline (Table 6). **By error type: model-verdict types only** — Type-1 (false
  negative) to Type-5 (Figure 6), not classes of the flaw in the proof; the equational task notes
  two corruption kinds (contractum vs. equation) with a qualitative difficulty comparison, no curve.
  **Both axes jointly: no.** **Verdict:** the strongest length-axis precedent among the retrieved
  sources, including an empirical near-chance regime at long proofs, in formal logic, without a
  flaw-class breakdown. Does not falsify; bounds the length axis.

---

## S-0014 — Uluşan et al., "FormalRewardBench: A Benchmark for Formal Theorem Proving Reward Models" (sweep rank 6)

- **State:** INGESTED (PDF under `sources/`, CC BY 4.0) + VERIFIED.
- **Citation:** Zeynel A. Uluşan and three co-authors (last: Gözde Gül Şahin), arXiv:2605.10141
  (v1, 2026-05-11); 22 pages.
- **Primary:** `https://arxiv.org/abs/2605.10141` (SHA-256 `8ef8d60d47068590…`);
  `https://arxiv.org/pdf/2605.10141` (1,059,765 bytes, SHA-256
  `f85306eb2fbdb1a4286385ea69719c3bef38b5d4cdaea217304ee07f00b80827`).
- **Licence:** CC BY 4.0. **Stored copy:** `sources/arxiv-2605.10141v1-formalrewardbench.pdf`.
- **Bears on CLAIM-0006:** Level: Lean 4 proofs of miniF2F problems ("488 olympiad-level
  mathematical problems"). Data: 250 preference pairs; correct proofs from DeepSeek-Prover-V2-671B;
  incorrect variants by "five expert curated error injection strategies" generated with Claude
  Opus 4.5 (forced mistakes, minimal single-point variations, **verbose incorrect proofs**,
  natural-language justification, Python code injection) plus sampled failed prover outputs — all
  **AI-generated**, all failing Lean type checking by construction. **By error class: yes** —
  pairwise and pointwise accuracy per strategy (Tables 2, 5, 6): "S5 (Verbose Incorrect) and S4
  (Forced Mistakes) are hardest … Even the best model achieves only 60% on S5 and 50% on S4";
  "most models perform at or below random baseline" (conclusion). The S5 prompt asks for "a LONG,
  COMPLEX Lean 4 proof with many sophisticated steps and intermediate lemmas" — length used as an
  **adversarial strategy**, not as a measured axis. **By length: no curve.** **Verdict:** class
  axis only (one class is length-flavoured), formal, competition level. Does not falsify; the
  closest injection precedent for FLAW-0013-style camouflage in Lean.

---

## S-0015 — Barkallah et al., "Pseudo-Formalization for Automatic Proof Verification" (ArxivMathGradingBench, FirstProofGradingBench; sweep rank 7)

- **State:** INGESTED (PDF under `sources/`, CC BY 4.0) + VERIFIED.
- **Citation:** Slim Barkallah and four co-authors (last: Tengyu Ma), arXiv:2605.20531 **v3**
  (2026-10-01; v1 2026-05-19); "31 pages, code available"; the PDF has 57 pages.
- **Primary:** `https://arxiv.org/abs/2605.20531` (SHA-256 `545c194b28f76bbb…`);
  `https://arxiv.org/pdf/2605.20531` (1,240,326 bytes, SHA-256
  `a6c93783a7584ada33d91d42fedc4d477d240a37404eabba73322519a5067439`).
- **Licence:** CC BY 4.0. **Stored copy:** `sources/arxiv-2605.20531v3-pseudo-formalization.pdf`.
- **Bears on CLAIM-0006:** Level: competition (Hard2Verify, S-0011) **and research level**:
  ArxivMathGradingBench — arXiv math papers 2023–2025 "that contained errors later corrected by
  their authors", 100 candidates reduced to "the 35 papers we use for the final dataset" after
  licence and false-positive filtering, PDFs as verifier input (**human-authored** research proofs
  with real errors); FirstProofGradingBench — "the research-level AI proof attempts and expert
  referee reports released in the First Proof project's second batch", "24 ground-truth errors
  across the 16 proofs (the 3 correct solutions contain none)" (**AI-generated research-level
  proofs with referee-labelled errors**). Method: Pseudo-Formalization + Block Verification vs.
  LLM-as-judge; metrics precision and recall per step / per error / per proof vs. the number of
  parallel attempts k (Figures 1, 3, 4). Research-level result: "Even with k = 8 rollouts both
  verifiers leave roughly 40% of annotated ground-truth errors uncaught". **By error class: no** —
  "we only have coarse-grained labels of error locations, as opposed to details of what the errors
  are" (ArxivMathGradingBench). **By length: no accuracy curve** — Figure 4 (right) shows that
  block-verification context size "does not grow with the input proof size" (a cost property). The
  ArxivMathGradingBench pipeline ("a regex and then GPT 5.4 nano") is itself a precedent for S1.1
  errata harvesting. **Verdict:** the only retrieved source with research-level, AI-generated,
  human-labelled proof errors — and it reports neither breakdown. Does not falsify.

---

## S-0016 — Ammanamanchi et al., "Faults in Our Formal Benchmarking: Dataset Defects and Evaluation Failures in Lean Theorem Proving" (sweep rank 8, first part)

- **State:** INGESTED (PDF under `sources/`, CC BY 4.0) + VERIFIED.
- **Citation:** Pawan Sasanka Ammanamanchi and two co-authors (last: Stella Biderman),
  arXiv:2606.29493 (v1, 2026-06-28); comments "Accepted at ICML 2026"; 32 pages.
- **Primary:** `https://arxiv.org/abs/2606.29493` (SHA-256 `6faa48e6c27af52e…`);
  `https://arxiv.org/pdf/2606.29493` (1,138,932 bytes, SHA-256
  `4f71871a708b06c1631264eedb784009033114412a2c0fad217804e8713bbef2`).
- **Licence:** CC BY 4.0. **Stored copy:**
  `sources/arxiv-2606.29493v1-faults-in-our-formal-benchmarking.pdf`.
- **Bears on CLAIM-0006:** Not a proof-triage calibration; an audit of five Lean benchmarks
  (miniF2F, ProofNet, FormalMath, CombiBench, ProverBench) and forks: "4,833 findings, including
  398 mechanically certified issues such as counterexamples, vacuous theorems, and unsound axioms";
  a fault taxonomy (fidelity failures, evaluation loopholes, maintenance decay). Relevant to the
  SYNDEOM flaw classes: **statement fidelity** (FLAW-0008) — LLM-assisted semantic audit with
  per-category F1 (Table 7, Table 14: "specification errors (0.51) and formalization errors
  (0.18)"); **smuggled axioms / trusted-base expansion** (FLAW-0009) — "native decide expands the
  trusted computing base beyond the kernel by trusting the compiler/codegen result via the
  Lean.ofReduceBool axiom"; "A bug in Lean versions prior to 4.20.0 allowed the apply? tactic to
  report success without producing a theorem declaration that had passed ordinary kernel
  verification … This failure mode appeared in at least three proofs claimed by DeepSeek-Prover-V2
  … one miniF2F solution and two PutnamBench solutions from the 7B model (Appendix D)";
  recommendation: "verify #print axioms output". Notes that "The Lean community has since released
  comparator" (S-0017). **By length: no. By flaw class: per-category F1 for dataset-defect
  detection, not proof-error detection.** **Verdict:** does not bear on the length axis; it is the
  evidence base for FLAW-0008/FLAW-0009's realism and for the gate's axiom audit. Does not falsify.

---

## S-0017 — `leanprover/comparator` (sweep rank 8, second part)

- **State:** VERIFIED (repository metadata, README, LICENSE, `lean-toolchain`, `lakefile.toml`
  read through the GitHub API at 03:23:14Z; nothing copied).
- **Primary:** `https://github.com/leanprover/comparator`, public, created 2025-06-06, default
  branch `master`, tip `e3dcf3083fc53d44b27a4431fabf7d43181583f5` (2026-10-12T18:43:46Z); README
  blob `e1d67f66…` (9,960 bytes); `lean-toolchain` at tip `leanprover/lean4:v4.35.0`.
- **Licence:** Apache-2.0 (LICENSE file; GitHub-detected SPDX `Apache-2.0`).
- **What it is (README):** "a trustworthy judge for Lean proofs": given a `Challenge` module
  (theorem with `sorry`) and a `Solution` module, a JSON config with `theorem_names` and
  `permitted_axioms` (default `["propext", "Quot.sound", "Classical.choice"]`), it guarantees that
  the listed theorems "1. Prove the same statement as provided in `Challenge` 2. Use no more axioms
  than listed in `permitted_axioms` 3. Be accepted by the Lean kernel", with sandboxing via
  `landrun`, export via `lean4export`, and optional external kernels (e.g. nanoda) under stated
  assumptions. **Bears on CLAIM-0006:** no calibration content; it is the reference
  implementation for SYNDEOM's statement-fidelity and axiom-audit detectors (S3.1) and the
  artifact standard AGMAI names (S-0002). Does not bear on either axis.

---

## S-0018 — Zheng et al., "ProcessBench: Identifying Process Errors in Mathematical Reasoning" (sweep rank 10)

- **State:** VERIFIED (abstract page and PDF retrieved and read; digest-only).
- **Citation:** Chujie Zheng and eight co-authors (last: Junyang Lin), arXiv:2412.06559 (v4,
  2025-05-26; first 2024-12-09); comments "ACL 2025"; 16 pages.
- **Primary:** `https://arxiv.org/abs/2412.06559` (SHA-256 `a8f105df6f495a6f…`);
  `https://arxiv.org/pdf/2412.06559` (740,748 bytes, SHA-256
  `30a74dc4ed897e077a243b82326df8529c037db0352b0ac35817dd29262ca653`).
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` — **not redistributable; no
  copy committed.**
- **Bears on CLAIM-0006:** Level: GSM8K, MATH, OlympiadBench, Omni-MATH ("primarily focused on
  competition- and Olympiad-level math problems"). Data: 3,400 cases; **AI-generated solutions**
  (Qwen and LLaMA open-source models) with human-expert annotation of the earliest erroneous step
  (three-of-n agreement; "∼30% discard rate"). Task: output the index of the earliest error or −1;
  F1 of the two per-class accuracies, per subset (Table 3, Tables 5 and 7). **By error type: no** —
  the guidelines name mathematical, logical, conceptual errors, but labels are positions only.
  **By length: no** — statistics only: Table 2 "% ≥ n steps"; "incorrect solutions usually contain
  more steps than correct ones"; Figure 3 error-position distribution ("a large proportion of
  errors occur in the earlier steps"); performance declines with problem difficulty, not length.
  **Verdict:** the canonical first-error-step task definition; neither breakdown. Does not falsify.

---

## Cross-reference — sweep rank 9

AGMAI "Responsible Release of AI-Generated Mathematics" is S-0002 (VERIFIED, Session 2); the
7 October withdrawals are S-0006 (existence VERIFIED; content withheld under SD8). Neither is a
calibration; neither bears on CLAIM-0006's two axes. Not re-entered.
