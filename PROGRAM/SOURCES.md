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

---

## S-0019 — Apache License, Version 2.0 — the repository licence text (Al's ruling R1 of 2026-10-13)

- **State:** INGESTED (the text is committed verbatim at the repo root as `LICENSE`, applying the
  licence to this repository under the boundary stated in README.md § Licence).
- **As named:** Al's ruling R1 of 2026-10-13 (ADR-0000 Addendum 2): SPDX `Apache-2.0`, "retrieved
  this session from its canonical source and recorded with URL, access date and SHA-256 per SD11
  (do not reproduce it from memory)".
- **Canonical host — unreachable this session (reachability, not existence):**
  `https://www.apache.org/licenses/LICENSE-2.0.txt` and `https://apache.org/licenses/LICENSE-2.0.txt`.
  curl at 2026-10-13T22:36:27Z: `Failed to connect to www.apache.org port 443 after 21115 ms:
  Could not connect to server`; again IPv4-forced on both hosts at 22:38:32Z (same error); the
  host root at 22:44Z (connect failed); the harness fetch tool at about 22:40Z: `connect
  ECONNREFUSED 151.101.2.132:443`. DNS resolved (`151.101.2.132`, `2a04:4e42::644`). A later
  session diffs the committed text against the ASF file when the host answers (whitespace
  normalization as below); nothing in S1 depends on it.
- **Retrieved — the SPDX License List's text for the identifier `Apache-2.0`:**
  `https://spdx.org/licenses/Apache-2.0.txt`, accessed 2026-10-13T22:38:32Z (curl, HTTP 200,
  `Content-Type: text/plain`, `Last-Modified: Wed, 29 Apr 2020 17:19:07 GMT`, ETag
  `"ba963850f6731c74878fe839d227e675"`): 10,279 bytes, 0 CR, no trailing newline, 72 lines
  (paragraph-per-line layout), SHA-256
  `c274f80372d90c012937370f0e1f15087d22e308ef98b27cea5dc0d2d088366c`. Header reads "Apache License
  / Version 2.0, January 2004 / http://www.apache.org/licenses/".
- **Cross-check — an independent copy:** GitHub Licenses API `https://api.github.com/licenses/apache-2.0`
  (`key: apache-2.0`, `spdx_id: Apache-2.0`, `name: Apache License 2.0`; GitHub sources it from
  `choosealicense.com/licenses/apache-2.0/`), accessed 2026-10-13T22:44:34Z via `gh api`
  (authenticated): `body` 11,357 bytes UTF-8, 0 CR, trailing LF, centred-header layout, SHA-256
  `c71d239df91726fc519c6eb72d318ec65820627232b2f796219e87dcf35d0ab4`. The two texts are
  **identical after whitespace normalization** (`tr -s ' \t\r\n' ' '` on each, trim, `diff`: empty);
  they differ only in line wrapping and the centred header.
- **Committed form:** `LICENSE` = the SPDX bytes plus one trailing LF, because the gate requires
  every tracked text file to end with LF (`scripts/check_registers.py`): 10,280 bytes, SHA-256
  `074e6e32c86a4c0ef8b3ed25b721ca23aca83df277cd88106ef7177c354615ff`. Re-verify the retrieved
  bytes from the committed file with `head -c 10279 LICENSE | sha256sum` → `c274f803…366c`.
- **Licence (of this material):** the Apache License 2.0 is itself the licence; the text is
  included to apply it (R1). What it covers and does not cover: README.md § Licence (Al's
  boundary: not `sources/`, `groundtruth/`, `contact/` third-party materials, each under its own
  recorded licence; not separately controlled DARQ Labs research materials, which are marked where
  they appear and remain proprietary).
- **Also set:** `license = "Apache-2.0"` in `engine/manifest/Cargo.toml` `[package]` (the only
  package manifest; `engine/Cargo.toml` is a bare `[workspace]`). No NOTICE file: neither sibling
  carries one (KOBER `f94bec2`, FRIEDMAN `7c3e5dd`, read 2026-10-13).
- **2026-10-14 (Session 11), one attempt as ordered:** `https://www.apache.org/licenses/LICENSE-2.0.txt`
  at 01:06:29Z — `Connection timed out after 20003 milliseconds`. The diff note is carried
  unchanged.

---

## S-0020 — Mann and Whitney (1947), "On a Test of Whether one of Two Random Variables is Stochastically Larger than the Other" (ADR-0004 D5: the null-hypothesis variance of U)

- **State:** VERIFIED (the publisher's PDF retrieved this session — through the Internet Archive's
  capture of the Project Euclid file, because Project Euclid itself served a bot challenge — and
  read on pp. 50, 51 and 55; digest-only, no copy committed).
- **As named:** Al's ruling R2 (v) of 2026-10-13: N_min is derived "from the null-hypothesis
  variance of the AUC (Mann–Whitney) statistic, with the primary source for that variance retrieved
  and cited this session per SD11, or left UNVERIFIED with a note". The executor identified the
  candidate paper; its bibliographic record was confirmed from Crossref before anything was read.
- **Citation (from the first page, confirmed by Crossref):** H. B. Mann and D. R. Whitney (Ohio
  State University), "On a Test of Whether one of Two Random Variables is Stochastically Larger than
  the Other", *The Annals of Mathematical Statistics* 18(1), pp. 50–60, March 1947, Institute of
  Mathematical Statistics. DOI `10.1214/aoms/1177730491`.
- **Primary host, this session:** `https://doi.org/10.1214/aoms/1177730491` → HTTP 302 →
  `projecteuclid.org/euclid.aoms/1177730491` (22:36:49Z). The Project Euclid landing page, the
  `.full` page and the `.pdf` URL each returned HTTP 200 with an Incapsula challenge page
  (1,054–1,167 bytes; "Request unsuccessful. Incapsula incident ID …") to curl with a browser UA at
  22:36:49Z and 22:40Z; the harness fetch tool received empty content from both. Crossref
  `https://api.crossref.org/works/10.1214/aoms/1177730491` at 22:44:36Z: title, authors, container
  title, volume 18, issue 1, pages 50-60, published-print 1947-03, publisher as cited; `license: null`.
- **Retrieved copy:** the Internet Archive's capture of the Project Euclid PDF —
  `http://web.archive.org/web/20260927082551id_/https://projecteuclid.org/journals/annals-of-mathematical-statistics/volume-18/issue-1/On-a-Test-of-Whether-one-of-Two-Random-Variables/10.1214/aoms/1177730491.pdf`
  (capture 2026-09-27T08:25:51Z, found through `archive.org/wayback/available` at 22:44:36Z),
  accessed 2026-10-13T22:46:04Z (curl, HTTP 200, `application/pdf`): 779,952 bytes, PDF-1.3,
  11 pages, a JSTOR scan without a text layer (p. 50 carries the footer "Institute of Mathematical
  Statistics is collaborating with JSTOR to digitize, preserve, and extend access to The Annals of
  Mathematical Statistics"), SHA-256 `7647e2a29f7d11a12ecafa31e7474cb48349a8e71f140effdf24e014b80e0788`.
  Pages were rendered to images (poppler 26.02.0) and read: 50, 51, 52–54 (Table I), 55.
- **Content used (ADR-0004 D5 and its Method):** §1 and §2 assume continuous distributions (so
  P(x_i = y_j) = 0 — no ties). §3 "The U test": "Let U count the number of times a y precedes an
  x"; U = mn + m(m+1)/2 − T with T Wilcoxon's rank sum. §4 "The distribution of U": the recurrence
  (1) for p_nm(U); eq. (2), p. 51: E_nm(U) = nm/2 ("Since the probability of the ith 1 preceding
  the jth 0 is ½"); p. 55, with u = U − nm/2: eq. (5) E_nm(u²) = nm(n + m + 1)/12, shown by
  substitution to solve the recurrence (3) with initial conditions (4), "hence it follows by
  mathematical induction". Eq. (5) is the null-hypothesis variance of U that ADR-0004 fixes as the
  basis of N_min. The fourth moment (8) and the limit-normality argument were not used.
- **Licence:** not stated on the retrieved copy; Crossref `license: null`; the publisher's page was
  unreadable this session. **Not redistributable on the record available; no copy committed**
  (digest-only). Nothing needs redistributing: ADR-0004 cites the equation numbers and reproduces
  the variance by an independent derivation.
- **Bears on:** ADR-0004 only. Not a CLAIM; nothing about any manuscript.

---

## S-0021 — "Open Letter about the Mathathon", a coalition of current and former Caltech mathematicians, *Proofs and Prompts*, 10 September 2026 (position paper §0 disclosure)

- **State:** VERIFIED (page retrieved and read this session; digest-only).
- **As named:** Al (Session 11): published 10 September 2026, received 8 September; 771
  signatories at publication; states that Anthropic and OpenAI would supply participants with two
  million dollars in AI credits. Each point is on the page, quoted below.
- **Primary:** <https://proofsandprompts.com/2026/09/10/open-letter-about-the-mathathon/>,
  accessed 2026-10-14T01:06:24Z (curl, HTTP 200, `text/html`), 221,399 bytes, SHA-256
  `f21fbc51771eae5588108fd3eb88796dd77072d0eeef08a03d66d90d9679aa6f`.
- **Page metadata:** `og:title` "Open Letter about the Mathathon"; `og:site_name` "Proofs and
  Prompts"; `article:published_time` 2026-09-10T10:21:20+00:00; `article:modified_time`
  2026-09-29T21:51:19+00:00; byline "A coalition of current and former Caltech mathematicians";
  filed under "Initiatives"; tags "Community · Initiative · Open letter".
- **Content (quoted from the page):** a note above the letter: "The Mathathon organizers have
  published a response, available here" (linking
  `https://proofsandprompts.com/2026/09/10/a-response-from-mathathon/`). The letter opens: "On
  October 30th, 2026, Caltech will begin hosting the Caltech Mathathon, a math hackathon in which
  participants are encouraged to solve open research problems by prompting LLMs. Two prominent AI
  companies, Anthropic and OpenAI, will supply participants with 2 million dollars in AI credits.
  This event is likely to have destructive impacts for the mathematical community." It states
  five headed concerns — "The creation of slop mathematics and the exacerbation of a market-driven
  arms race", "Intrusion of corporate interest", "Conditions unlikely to foster mathematical
  understanding", "Damaging misinformation about research mathematics", "Failure to care for
  young mathematicians" — then "we call on the organizers to suspend this event" and, in closing,
  "we ask that the organizers give up this harmful enterprise entirely." A footnote dated
  "September 10th, 2026" says the Mathathon website changed after the letter circulated and links
  "a version of the website accessed September 8th, 2026, which included the line 'What is the
  role of a mathematician when AI can solve conjectures faster?'". Below the letter: "At the time
  of publication, there are 771 signatories. A full list of signatories is available here."
  (linking a Google document); eligibility to sign — members of the Caltech community or of the
  mathematical research community at PhD level or higher; and "Received 8 September 2026." The
  page's reader comments are not the source and were not transcribed.
- **Licence:** none stated on the page (no licence or copyright notice in the retrieved HTML);
  **copy not committed** (digest-only).
- **Bears on:** the position paper's §0 disclosure; no CLAIM.

---

## S-0022 — "Joint Statement about Mathathon", a coalition of Caltech mathematicians, *Proofs and Prompts*, 28 September 2026, cross-posted on Terence Tao's blog (position paper §0 disclosure)

- **State:** VERIFIED (both pages retrieved and read this session; digest-only).
- **As named:** Al (Session 11): 28 September 2026, cross-posted the same day on Terence Tao's
  blog; states that the redesigned event no longer receives sponsorships from developers of
  proprietary AI models. Each point is on the pages, quoted below.
- **Primary:** <https://proofsandprompts.com/2026/09/28/joint-statement-about-mathathon/>,
  accessed 2026-10-14T01:06:25Z (curl, HTTP 200, `text/html`), 189,416 bytes, SHA-256
  `bc7a0c9610fe6cdf10481c0088bbfae39b97a787b4c5ec1381b7d2abc49ba22d`. Metadata: `og:title`
  "Joint Statement about Mathathon"; `article:published_time` 2026-09-28T16:44:23+00:00;
  `article:modified_time` 2026-09-29T21:49:42+00:00; byline "A coalition of Caltech
  Mathematicians", with a footnote "This is a joint statement by everyone named in footnotes 2-4":
  the original organizers of Mathathon, two coauthors of the Open Letter (S-0021), and other
  members of the Caltech community, each group named on the page; opening line "Also
  cross-posted at Terry Tao's blog."
- **Content (quoted from the primary page):** "Mathathon is centered around one question: how
  can we use AI to augment human understanding of mathematics?" "This past week, the original
  organizers of Mathathon, two coauthors of Open Letter about the Mathathon, and other members
  of the Caltech community engaged in a conversation to redesign this event." "Thus the new
  theme of Mathathon is Old Problems, New Proofs." Participants pick a problem with an unintuitive
  solution, learn for 40 hours, present, then "take two months to develop an alternative proof or
  exposition" and submit "an explainer and a GitHub repository" holding "LLM chat histories, code
  used to produce visualizations, and more". "Mathathon treats AI as one of many tools available
  to mathematicians. We no longer receive sponsorships from developers of proprietary AI models.
  Participants are given a cash grant to purchase any tool they need, whether that be LLM credits,
  HPC access, or pen and paper. Proprietary models are allowed, but we encourage the use of
  open-source tools." "The new event will take place on November 13–15. It is co-organized with
  the Foundation for Science and AI Research (SAIR), which will provide compute, host workshops,
  and offer prizes to teams that use open-source models." "Our lead donor is XTX Markets, an
  algorithmic trading firm". Acknowledgements thank Terence Tao and Andrew Wu for reviewing the
  statement; "All of the views above are solely our own and do not reflect those of Caltech or any
  other organizations we belong to." Reader comments not transcribed.
- **Cross-post (secondary, the S-0003 pattern):** Terence Tao, "Joint Statement about Mathathon",
  *What's new*, "28 September, 2026 in advertising, guest blog, math.GM | Tags: AI, Caltech,
  Mathathon, SAIR | by Terence Tao",
  <https://terrytao.wordpress.com/2026/09/28/joint-statement-about-mathathon/>, accessed
  2026-10-14T01:06:26Z (HTTP 200), 251,467 bytes, SHA-256
  `71af973573c58cc60e7ff0a8e86e69763416d17eb4706420a98961ca027f8e16`;
  `article:published_time` 2026-09-28T18:33:48+00:00. Its bracketed note: "[This is a guest post
  by a coalition of Caltech mathematicians: the original organizers of Mathathon (…), two
  coauthors of the Open Letter about the Mathathon (…), and other members of the Caltech
  community (…). This blog post was initially written in a different file format and converted
  using AI. It is also crossposted at Proofs and Prompts — T.]" The body matches the primary.
  Unlike S-0003, the primary page is itself dated; the cross-post corroborates the date rather
  than supplying it.
- **Licence:** none stated on either page; **copies not committed** (digest-only).
- **Bears on:** the position paper's §0 disclosure; no CLAIM.

---

## S-0023 — Mathathon, "Old Problems, New Proofs" — the event's current page (position paper §0 disclosure)

- **State:** VERIFIED (page retrieved and read this session; digest-only). The page is undated;
  the access time is the date of record for this copy.
- **As named:** Al (Session 11): the Mathathon's current page, Round 1 on 13–15 November 2026.
  On the page as quoted below.
- **Primary:** <https://mathathonchallenge.com/>, accessed 2026-10-14T01:06:28Z (curl, HTTP 200,
  `text/html`), 5,656 bytes, SHA-256
  `96e4af8497458214b56e6c30fc623b7a5a298d817b073475c60a623fc5707955`. `<title>` "Mathathon: Old
  Problems, New Proofs"; no other metadata; no date.
- **Content (quoted from the page):** "Mathathon is centered around one question: how can we use
  AI to augment human understanding of mathematics?" Two rounds: "Round 1: Onsite, 40 hours. Teams
  will make preliminary progress and give live talks before judges and other contestants." and
  "Round 2: Remote, 2 months. Teams will produce an online explainer and a GitHub repository."
  "We are hosting Round 1 on November 13th to November 15th at Caltech." FAQ: "Mathathon is a
  registered student organization at Caltech. Its decisions are made by its organizing committee,
  which consists of undergraduate and graduate students currently enrolled at Caltech. The
  committee does not receive monetary compensation from Mathathon or its sponsors." Organizers
  and faculty advisors (Sergei Gukov, Terence Tao, Tony Yue Yu) are named; "The Foundation of
  Science and AI Research (SAIR) administers grants and travel reimbursements on behalf of
  Mathathon."; applicants above 18 with research or competition experience, PhD holders may apply
  as judges; flights covered for participants and judges; "We treat AI as one of many tools
  available to mathematicians. Proprietary models are allowed, but we encourage the use of
  open-source tools." A contact e-mail on the page is not transcribed.
- **Licence:** none stated; **copy not committed** (digest-only).
- **Bears on:** the position paper's §0 disclosure; no CLAIM. The page changes over time
  (S-0021's footnote records an earlier version); this digest identifies the 2026-10-14 copy only.

---

## S-0024 — Hisanobu Shinya, "On a new approach to the Riemann hypothesis", arXiv:0706.0357 **v70** (the original of Route A item 1)

- **State:** INGESTED (the v70 e-print bytes are committed under `groundtruth/` as the item's
  `raw/1` original; licence permits, below). Retrieved and read this session.
- **How found:** Route A (ADR-0003 D1), candidate 1 of the finder's ascending-id stream
  (HARVEST-LOG run 1); examined Session 11, 2026-10-14.
- **Citation (from the abstract page):** Hisanobu Shinya, "On a new approach to the Riemann
  hypothesis", arXiv:0706.0357, General Mathematics (math.GM); v1 4 June 2007; **v70 submitted
  7 March 2026** (comment: "13 pages. In this revision, the author tried to clarify what he thinks
  is a key feature of his preprint"); 71 versions in all.
- **Primary:** abstract page <https://arxiv.org/abs/0706.0357v70>, accessed 2026-10-14T01:57:34Z
  (HTTP 200), 49,164 bytes, SHA-256
  `4a1cd5dc8663424fbb6c582cd708404d34e312cce12f2d743f6a525e64209013`; e-print
  <https://arxiv.org/e-print/0706.0357v70> (`application/gzip`), 8,236 bytes, SHA-256
  `5987d8106ae69669b10c876a0915d5ad52ce44a814ea7fe63600c1737642cca8` — a gzipped tar holding one
  file, `On_a_new_approach_to_the_Riemann_hypothesis.tex`.
- **Content used:** Lemma 1.1 — its statement and proof — is the item's claim of record (ADR-0003
  D6); the locating sentences are quoted in the item's `RECORD.md`, nothing more is reproduced
  outside the committed copy.
- **Licence:** Creative Commons Attribution 4.0 (`http://creativecommons.org/licenses/by/4.0/`,
  the licence link on the v70 abstract page). **Copy committed** as
  `groundtruth/<artifact_id>/original-0706.0357v70.tar.gz` (ADR-0003 D5 as amended: CC BY is in
  the committable set), with attribution here and in `RECORD.md`.
- **Bears on:** S1.1 (`groundtruth/` item); no CLAIM.

---

## S-0025 — Hisanobu Shinya, "On a new approach to the Riemann hypothesis", arXiv:0706.0357 **v71** — the correcting version and its comment (the status document of Route A item 1)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0024; **v71 submitted 7 June 2026**, comment: "11 pages. To fix an error in
  Lemma 1.1". This comment, with the corrected version it belongs to, is the status document SD12
  requires (ADR-0003 D1 Route A, D2.2).
- **Primary:** abstract page <https://arxiv.org/abs/0706.0357v71>, accessed 2026-10-14T01:57:34Z
  (HTTP 200), 49,049 bytes, SHA-256
  `1dbf356c8d01e9a60e66a7db043d332655697deaf3dab5694e7c35e1b5774e29`; e-print
  <https://arxiv.org/e-print/0706.0357v71> (`application/gzip`), 8,750 bytes, SHA-256
  `fffd0fdc3a868deb642102a1f72f23cd92ee89bf118c92a4b992cb5a6fdfd263` — a gzipped tar holding
  `On_a_new_approach_to_the_Riemann_hypothesis_2.tex`.
- **Content used:** the corrected Lemma 1.1: its statement now reads, in the executor's
  transcription of the TeX, "M(s, a/b) − Σ_{q|b} Σ_{l≥1} (log q) q^{−sl} e^{−2πi a q^l/b} = −Σ_χ
  A(a, b; χ) (L′/L)(s, χ), where the sum Σ_{q|b} runs through all the prime divisors of b"; its
  proof now applies the orthogonality formula "for (k, b) = 1" and says the formula "is applicable
  to all n's which are powers of primes other than of q|b". The diff v70 → v71 is otherwise
  typographical and expository (a new introductory paragraph, a new bibliography entry).
- **Licence:** Creative Commons Attribution 4.0 on the abstract page; copy not committed (the
  item's committed bytes are the original, S-0024).
- **Bears on:** S1.1 item 1 (its status document); no CLAIM.

---

## S-0026 — Roman Bezrukavnikov and Michael Finkelberg, "Equivariant Satake category and Kostant-Whittaker reduction", arXiv:0707.3799 **v2** (the original of Route A item 2)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 2 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 11, 2026-10-14.
- **Citation (from the abstract page):** Roman Bezrukavnikov, Michael Finkelberg, "Equivariant
  Satake category and Kostant-Whittaker reduction", arXiv:0707.3799, Representation Theory
  (math.RT); v1 25 July 2007; **v2 submitted 18 February 2008** (comment: "31 pages, to appear in
  Moscow Math J; some typos corrected"); four versions in all.
- **Primary:** abstract page <https://arxiv.org/abs/0707.3799v2>, accessed 2026-10-14T01:58:05Z
  (HTTP 200), 40,691 bytes, SHA-256
  `d37bcb174d90c18aad1e65ab0a38ae0849c6931c05bf45fcf475be5b0f187699`; e-print
  <https://arxiv.org/e-print/0707.3799v2> (`application/gzip`), 40,240 bytes, SHA-256
  `3f9593e18a00f30685f66f1e4d920fbf2a4427e94b23beb2032ccd91d47d3053` — a single gzipped TeX file.
- **Content used:** Theorem 1 b) (TeX label `normocon`) — statement and proof — is the item's
  claim of record; the locating sentences are quoted in `RECORD.md`.
- **Licence:** the abstract page's licence link is `http://arxiv.org/licenses/assumed-1991-2003/`,
  arXiv's "Assumed non-exclusive license to distribute" (page retrieved 2026-10-14T02:00:31Z,
  1,988 bytes, SHA-256 `dfe5bdbd72f269aaf1cbbb2fcae418267571b6df6fe6c9e139dc73918b37fe13`): "It
  was assumed that … a license equivalent to http://arxiv.org/licenses/nonexclusive-distrib/1.0/
  was granted by the act of submission." Equivalent to the arXiv non-exclusive licence, which
  ADR-0003 D5 lists as `external`: **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` item); no CLAIM.

---

## S-0027 — Bezrukavnikov and Finkelberg, arXiv:0707.3799 **v3** — the correcting version and its comment (the status document of Route A item 2)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0026; **v3 submitted 25 February 2026**, comment: "This version corrects a
  mistake in the proof of Theorem 1b) and a typo in Proposition 7. All the statements are
  unchanged from the previous version. We thank Jakub Löwut for pointing out the issue in Theorem
  1b)". (v4, 9 September 2026, adds "v4: section 5.2, and a typo in section 3.3 corrected"; v4 is
  not the notice for Theorem 1 b) and was not retrieved.) This comment, with the corrected
  version it belongs to, is the status document SD12 requires.
- **Primary:** abstract page <https://arxiv.org/abs/0707.3799v3>, accessed 2026-10-14T01:58:05Z
  (HTTP 200), 41,001 bytes, SHA-256
  `d2444a7842fd6e12e5519e085030c8f8554ee93b2960b7955dc6f68b06dd6e1b`; e-print
  <https://arxiv.org/e-print/0707.3799v3> (`application/gzip`), 41,505 bytes, SHA-256
  `0818ab63cef2ca7a4a3760ccbee0e1c78175be383b4367c950df9e46cbf65a00` — a gzipped tar holding
  `qsatake_final_corrected.tex`.
- **Content used:** the v3 proof of Theorem 1 b) carries the footnote "The proof of this part of
  the Theorem in the published version contained a mistake, here we present a corrected proof.
  We thank Jakub Löwit for pointing out the issue." and replaces the v2 paragraph with an
  explicit construction of a W-equivariant identification of the connected components of the
  affine Grassmannian (via the coweight-lattice action and an affine blow-up), ending "It is
  easy to see that the resulting isomorphism is independent of the choice of λ and hence it is
  W-equivariant." The statement of Theorem 1 is unchanged apart from notation (`Gr` → `Gr_G`,
  `π_1(G)^` → `π_1(G)`). The rest of the diff is notational (`\epf` → `\hfill $\Box$`) and the
  Proposition 7 typo the comment names.
- **Licence:** Creative Commons Attribution 4.0 on the v3 abstract page; copy not committed.
- **Bears on:** S1.1 item 2 (its status document); no CLAIM.

---

## S-0028 — Michael Björklund, "Ergodic Theorems for Homogeneous Dilations", arXiv:0902.0680 **v1** (the original of Route A item 3)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 4 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 13, 2026-10-14.
- **Citation (from the abstract page):** Michael Björklund, "Ergodic Theorems for Homogeneous
  Dilations", arXiv:0902.0680, Dynamical Systems (math.DS); **v1 submitted 4 February 2009**
  (comment "11 pages"); v2 of 29 June 2011 is marked withdrawn on the submission history; v3 of
  30 September 2026 is the corrected version (S-0029).
- **Primary:** abstract page <https://arxiv.org/abs/0902.0680v1>, accessed 2026-10-14T02:52:35Z
  (HTTP 200), 39,446 bytes, SHA-256
  `4d1555841da366c97ecfb8143c315608dfe78feea970a39e2bb45b88f27cb0a4`; e-print
  <https://arxiv.org/e-print/0902.0680v1> (`application/gzip`), 11,058 bytes, SHA-256
  `06c7ac8d54ddf9e5aec3caed1dc604e8bc170081efb5ad00de42b50bed4dcbac` — a single gzipped TeX file.
- **Content used:** the lemma with TeX label `density` in the section "Pointwise Ergodic
  Theorems" (the dense-subspace lemma: the span L of smoothed functions is dense in L¹₀(X) and the
  dilated averages of any f ∈ L converge to 0 almost everywhere when ν is a Rajchman measure) and
  its proof — the item's claim of record; locating sentences quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v1 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` item); no CLAIM.

---

## S-0029 — Björklund, arXiv:0902.0680 **v3** with the v2 withdrawal — the correcting version, its comment and its "Erratum on a previous version" (the status document of Route A item 3)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0028; **v3 submitted 30 September 2026** (inside the ADR-0003 D8 window, its
  last day), comment: "10 pages. Revised and corrected version of the paper withdrawn in June
  2011. The gap in the dense-subspace argument for pointwise convergence has been repaired by a
  new proof. This version treats the Fourier-dimension theorem; the earlier results on general
  Rajchman measures and polynomial curves are not included". v2 (29 June 2011) is marked
  "(withdrawn)" in the submission history; its abstract page carries the comment "11 pages" and no
  licence link.
- **Primary:** abstract page <https://arxiv.org/abs/0902.0680v3>, accessed 2026-10-14T02:55:06Z
  (HTTP 200), 39,907 bytes, SHA-256
  `ed5a9f433c1549ce8808a4f4d3787fad48e2e6274f84fd61abd0257c8ec2c5ba`; v2 abstract page
  <https://arxiv.org/abs/0902.0680v2>, 39,139 bytes, SHA-256
  `a6a765b22c74577c8963f9e8b0035da5d21257d3b31be37d80c43c8e6fe183d7` (no e-print exists for the
  withdrawn version: HTTP 404); e-print <https://arxiv.org/e-print/0902.0680v3>
  (`application/gzip`), 12,220 bytes, SHA-256
  `193b6443d5c928fb621fcd5f3c97a4721fe3ca7e7f9c9b1c4787673a57cb0048` — a gzipped tar holding
  `ErgodicTheoremsHomogeneousDilations_v15.tex`.
- **Content used:** the v3 section "Erratum on a previous version": "An earlier version of this
  work (arXiv:0902.0680) was withdrawn in June 2011 after the author found a gap in the
  dense-subspace argument for pointwise convergence. The present paper should be read as a
  correction to the Fourier-dimension theorem in that version and gives a new proof of that
  result. The broader statements there on Rajchman measures under homogeneous dilations and on
  polynomial curves are not addressed here." v3's new dense class (its section "A dense class with
  pointwise convergence") is "localized away from frequency zero" and differs from v1's.
- **Licence:** Creative Commons Attribution 4.0 on the v3 abstract page; copy not committed.
- **Bears on:** S1.1 item 3 (its status document); no CLAIM.

---

## S-0030 — A. Andrada, M. L. Barberis and I. G. Dotti, "Classification of abelian complex structures on 6-dimensional Lie algebras", arXiv:0908.3213 **v4** (the original of Route A item 4)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 5 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 13, 2026-10-14.
- **Citation (from the abstract page):** A. Andrada, M. L. Barberis, I. G. Dotti,
  "Classification of abelian complex structures on 6-dimensional Lie algebras", arXiv:0908.3213,
  Rings and Algebras (math.RA) and Differential Geometry (math.DG); v1 21 August 2009; **v4
  submitted 19 June 2012** (comment: "The proof of Theorem 3.2 and the statement of Theorem 3.4
  have been fixed. One remark added" — a 2012 correction, outside the window and not this item's
  notice); five versions in all.
- **Primary:** abstract page <https://arxiv.org/abs/0908.3213v4>, accessed 2026-10-14T02:52:55Z
  (HTTP 200), 40,761 bytes, SHA-256
  `6bcb8355c262a68c209009892f875a7a19362f188bb7dfb552a43aa3869e4c37`; e-print
  <https://arxiv.org/e-print/0908.3213v4> (`application/gzip`), 34,662 bytes, SHA-256
  `30978332dd33e6fb3a8d3d2cf94aa8ebe20f8a7eeabe6e1c7b6e28d62502c9d5` — a gzipped tar holding
  `dim6_arxiv3.tex`.
- **Content used:** Theorem 4.4 — its statement (the listed Lie algebras with abelian complex
  structures, assertions (1) and (2) among them) and its proof — is the item's claim of record;
  the locating lines are quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v4 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` item); no CLAIM.

---

## S-0031 — Andrada, Barberis and Dotti, arXiv:0908.3213 **v5** — the correcting version and its comment (the status document of Route A item 4)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0030; **v5 submitted 29 July 2024**, comment: "v4: The proof of Theorem 3.2
  and the statement of Theorem 3.4 have been fixed, and one remark was added. v5: The statement of
  Theorem 4.4 (assertions (1) and (2)) was fixed and the proof was modified accordingly. Some
  arguments were clarified". The v5 part of the comment, with v5 itself, is this item's status
  document; the v4 part records the 2012 correction, outside the window.
- **Primary:** abstract page <https://arxiv.org/abs/0908.3213v5>, accessed 2026-10-14T02:53:13Z
  (HTTP 200), 40,827 bytes, SHA-256
  `e69afae7e7fdc149be711f68a6206b29bb620fdfd0e79b8af0d94c76831cf877`; e-print
  <https://arxiv.org/e-print/0908.3213v5> (`application/gzip`), 71,750 bytes, SHA-256
  `16f9d1c2f512d78f21e5e3e02eb637149db4369175a10e9384a4aa710b1795f5` — a gzipped tar holding
  `dim6_arxiv_5.tex` and a figure.
- **Content used:** in Theorem 4.4's statement, assertions (1) `(aff(C) × R², J)` and (2)
  `(s₁, J₁)`: the complex structure changes from `Jf₁ = f₂, Je₁ = −e₂ + e₃, Je₂ = e₁ + e₄, Je₃ = e₄`
  (v4) to `Jf₁ = f₂, Je₁ = e₂ − e₃, Je₂ = −e₁ − e₄, Je₃ = −e₄` (v5); in the proof, the
  normalization with TeX label `base` changes its factor from `1/√(a²+b²)` to `1/(a²+b²)`, with
  the consequence `[f̃₁, f̃₂] = e₃/(a²+b²)` and a rescaled `ẽ₃`; the case (ii) argument is
  rewritten. The rest of the diff is typographical (spacing macros, a figure format, `\qedhere`).
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v5 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 4 (its status document); no CLAIM.

---

## S-0032 — Jarod Alper, "Adequate moduli spaces and geometrically reductive group schemes", arXiv:1005.2398 **v2** (the original of Route A items 5 and 6)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The items' `raw/1` file records are `external`.
- **How found:** Route A, candidate 8 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 14, 2026-10-14. One candidate, two documented results → two items sharing this
  original and S-0033.
- **Citation (from the abstract page):** Jarod Alper, "Adequate moduli spaces and geometrically
  reductive group schemes", arXiv:1005.2398, Algebraic Geometry (math.AG); v1 13 May 2010; **v2
  submitted 1 September 2018** (comment "minor errors corrected"); v3 22 March 2026 (S-0033). The
  v3 footnotes refer to "the published version of this paper"; no journal reference is on the
  abstract page.
- **Primary:** abstract page <https://arxiv.org/abs/1005.2398v2>, accessed 2026-10-14T03:37:02Z
  (HTTP 200), 38,222 bytes, SHA-256
  `f15feae8f7cf6e2b48a15e0b72fefbebc9e4dd7515869efed05c9ed3f3ed981a`; e-print
  <https://arxiv.org/e-print/1005.2398v2> (`application/gzip`), 42,669 bytes, SHA-256
  `a651e35acdba731abb10e2049b96dbe7ab02ecdd6e941d7541da8ed36011cd39` — a gzipped tar holding
  `adequate-website-8-31-18.tex`.
- **Content used:** Proposition 3.3.5 (TeX label `proposition-adequate-equiv`) with its proof,
  and the lemma on quasi-coherent algebras that follows it in Section 3.3 with its one-line proof —
  the two claims of record; locating sentences quoted in the two `RECORD.md` files.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v2 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` items 5 and 6); no CLAIM.

---

## S-0033 — Alper, arXiv:1005.2398 **v3** — the correcting version, its comment and its footnotes (the status document of Route A items 5 and 6)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0032; **v3 submitted 22 March 2026**, comment: "several minor errors
  corrected, including the addition of a finitely presented hypothesis to Proposition 3.3.5".
- **Primary:** abstract page <https://arxiv.org/abs/1005.2398v3>, accessed 2026-10-14T03:37:02Z
  (HTTP 200), 38,226 bytes, SHA-256
  `72e0c3d7971f75ffcfe5034c2c18c6371e1609d656f78f6fa99bcd8ad8a48c51`; e-print
  <https://arxiv.org/e-print/1005.2398v3> (`application/gzip`), 43,530 bytes, SHA-256
  `442625543857b906985081a39d93f128577dcc5d7c0314d02e63766ef5d80112` — a gzipped tar holding
  `adequate-jan9-2026.tex`.
- **Content used:** Proposition 3.3.5 now reads "Let A → B be a finitely presented homomorphism of
  rings" with the footnote "The published version of this paper had a version of this statement
  without the finitely presented hypothesis, which was incorrect. Namely, when A → B is not
  finitely presented, the impli[cation fails for] ℤ → ℤ[{x_p}_{p prime}]/(p x_p, x_p^p)" (the
  footnote's counterexample; its middle clause is elided here and quoted in full in the item's
  record); the proof of (2) ⇒ (1) now derives the isomorphism in characteristic 0 from the
  finitely presented hypothesis via the remark with TeX label `rmk-adequate-homeomorphism`, and
  the implication (1) ⇒ (3) is re-argued. The following lemma gains the same hypothesis with the
  footnote "As with Proposition 3.3.5, the published version did not assume that A → B is finitely
  presented and was incorrect." The rest of the diff: an introduction sentence, a remark's
  wording, a `char(κ) = 0` → `char(κ) = p` fix in a later lemma's second clause (not named by the
  comment beyond "several minor errors", so not harvested — D2.2), a bibliography path.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page;
  copy not committed.
- **Bears on:** S1.1 items 5 and 6 (their status document); no CLAIM.

---

## S-0034 — Ben Green, Terence Tao and Tamar Ziegler, "An inverse theorem for the Gowers U^{s+1}[N]-norm", arXiv:1009.3998 **v4** (the original of Route A item 7)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 9 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 14, 2026-10-14.
- **Citation (from the abstract page):** Ben Green, Terence Tao, Tamar Ziegler, "An inverse
  theorem for the Gowers U^{s+1}[N]-norm", arXiv:1009.3998, Combinatorics (math.CO); v1 21
  September 2010; **v4 submitted 22 April 2024** (comment "116 pages. Submitted"); v5 23 April
  2026 (S-0035). The v5 comment: "Appeared as Annals of Math. 176 (2012), 1231--1372."
- **Primary:** abstract page <https://arxiv.org/abs/1009.3998v4>, accessed 2026-10-14T03:39:23Z
  (HTTP 200), 42,105 bytes, SHA-256
  `31c74252a31ff82b6e61e4ea31cfb222a6b963f2ef7dc5d60071baa135a163da`; e-print
  <https://arxiv.org/e-print/1009.3998v4> (`application/gzip`), 395,608 bytes, SHA-256
  `f765dca2d73a9936a54356e61c9901d0c55f09069df7fefc7018161cf4ede3a2` — a gzipped tar holding
  `uk-inverse.tex` and auxiliary files.
- **Content used:** Lemma 13.2 and its proof (the paragraph "We now ℕ-filter G³ by defining
  (G³)_i …") — the item's claim of record; locating sentences quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v4 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` item 7); no CLAIM.

---

## S-0035 — Green, Tao and Ziegler, arXiv:1009.3998 **v5** — the correcting version, its comment and its footnote (the status document of Route A item 7)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0034; **v5 submitted 23 April 2026**, comment: "Appeared as Annals of Math.
  176 (2012), 1231--1372. Preprint is 116 pages. This version corrects a mistake in the proof of
  Lemma 13.2 (the wrong filtration was specified) and two small typos on page 28".
- **Primary:** abstract page <https://arxiv.org/abs/1009.3998v5>, accessed 2026-10-14T03:39:23Z
  (HTTP 200), 40,873 bytes, SHA-256
  `d6727daa30fa1682e1011156715ac7e8b538610094a26c4d92fafda294f5c03a`; e-print
  <https://arxiv.org/e-print/1009.3998v5> (`application/gzip`), 116,921 bytes, SHA-256
  `9da858fd065b9621b6cca3ee9bdabb3fbef91f71061df903a1bd127ca951594a` — a gzipped tar holding
  `uk-inverse.tex`.
- **Content used:** in the proof of Lemma 13.2 the definition of the ℕ-filtration of G³ is
  replaced (v4: generated by (G_{(i₁,…,i_s)})³ for i₁+…+i_s > i together with {(g₁g₂, g₁, g₂)} for
  i₁+…+i_s = i; v5: generated by the diagonal {(g, g, g) : g ∈ G_{(i₁,…,i_s)}} for i₁+…+i_s = i
  and i₁ = 0 together with {(g₁g₂, g₁, g₂)} for i₁+…+i_s = i and i₁ = 1) with the footnote "In
  the published version of the paper there is an error in the definition of this filtration,
  which was pointed out by Jan Fornal. We thank James Leng for indicating the appropriate
  correction."; the sentences that follow ("one verifies that this is a rational filtration of
  G³", "g̃ is polynomial with respect to this filtration") are unchanged. The "two small typos on
  page 28" are the degree-filtration example's quantifiers (h₁,…,h_m ∈ ℤ^k, n ∈ ℤ^k). The scanner's
  one "llm" hit in the sources is the substring of `\smallmatrix`; no AI-assistance mention.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v5 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 7 (its status document); no CLAIM.

---

## S-0036 — Jan Sieber, "Finding periodic orbits in state-dependent delay differential equations as roots of algebraic equations", arXiv:1010.2391 **v9** (the original of Route A item 8)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 10 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 14, 2026-10-14.
- **Citation (from the abstract page):** Jan Sieber, "Finding periodic orbits in state-dependent
  delay differential equations as roots of algebraic equations", arXiv:1010.2391, Dynamical
  Systems (math.DS); journal reference on the abstract page "Discrete and Continuous Dynamical
  Systems Series A 32(8), 2607-2651, 2012"; v1 12 October 2010; **v9 submitted 17 January 2020**
  (comment: "minor revision, correcting mistakes in formulation of Lemma 2.3 and A.5 (which are
  also present in the Journal paper): center of neighborhood must be in C¹, which is the case for
  the main theorem" — a 2020 correction, outside the window and not harvested); v10 5 October 2023
  (S-0037).
- **Primary:** abstract page <https://arxiv.org/abs/1010.2391v9>, accessed 2026-10-14T03:39:45Z
  (HTTP 200), 42,878 bytes, SHA-256
  `9af705af267910d26e95fd4338073122fe3022284d856d530454f0070b06d274`; e-print
  <https://arxiv.org/e-print/1010.2391v9> (`application/gzip`), 53,403 bytes, SHA-256
  `d64ff4f096e3c23364c08e67e7214f47c2705b2fe9bc8c013f55a9ed04f002b6` — a gzipped tar holding
  `S11arxiv.tex`.
- **Content used:** Lemma A.3 (TeX label `thm:flip`, "Local EC Lipschitz continuity") and its
  proof — the item's claim of record; locating sentences quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v9 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` item 8); no CLAIM.

---

## S-0037 — Sieber, arXiv:1010.2391 **v10** — the correcting version and its comment (the status document of Route A item 8)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0036; **v10 submitted 5 October 2023**, comment: "minor revision, correcting
  mistakes in formulation of Lemma A.3 (which are also present in the Journal paper): center x of
  neighborhood must be in C¹, which is the case for the main theorem. Further clarification in
  proof of Lemma A.3".
- **Primary:** abstract page <https://arxiv.org/abs/1010.2391v10>, accessed 2026-10-14T03:39:45Z
  (HTTP 200), 43,010 bytes, SHA-256
  `628736d96d518e6bf3cbf81d4fb0620043e6a890bb09bf9a556e1f9bd17a2dc1`; e-print
  <https://arxiv.org/e-print/1010.2391v10> (`application/gzip`), 53,362 bytes, SHA-256
  `853ca171d5e46d15cfa181ee8e457edd8b1f7494cc774566c446b4f316caac40` — a gzipped tar holding
  `S11arxiv.tex`.
- **Content used:** Lemma A.3's statement changes from "For all x ∈ D^{0,1} there exists a
  neighborhood U(x) ⊆ D^{0,1} and a constant K_x > 0 such that …" to "For all x ∈ D¹ …"; its
  proof is rewritten to work on a ball B¹_{δ/2}(x) ⊂ D¹ and to extend to the D^{0,1}-ball by
  density, in place of the v9 text "We prove the Lipschitz continuity first for y₁ and y₂ from a
  sufficiently small neighborhood U(x) ∩ D¹ ⊆ D¹ of x ∈ D¹ … Let x be an element of D¹." One
  norm subscript elsewhere is corrected. v10's abstract page carries CC BY 4.0.
- **Licence:** Creative Commons Attribution 4.0 on the v10 abstract page; copy not committed.
- **Bears on:** S1.1 item 8 (its status document); no CLAIM.

---

## S-0038 — Victor Chepoi and Mark F. Hagen, "On embeddings of CAT(0) cube complexes into products of trees via colouring their hyperplanes", arXiv:1107.0863 **v3** (the original of Route A items 9 and 10)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The items' `raw/1` file records are `external`.
- **How found:** Route A, candidate 12 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 15, 2026-10-14. One candidate, two documented results → two items sharing this
  original and S-0039.
- **Citation (from the abstract page):** Victor Chepoi, Mark F. Hagen, "On embeddings of CAT(0)
  cube complexes into products of trees via colouring their hyperplanes", arXiv:1107.0863, Metric
  Geometry (math.MG); v1 5 July 2011; **v3 submitted 26 March 2013** (comment: "Some small
  corrections; main change is a correction of the computation of the bounds in Theorem 1. Some
  figures repaired" — a 2013 revision, outside the window); v4 22 April 2026 (S-0039).
- **Primary:** abstract page <https://arxiv.org/abs/1107.0863v3>, accessed 2026-10-14T03:53:05Z
  (HTTP 200), 41,872 bytes, SHA-256
  `7c136ca32e84f97f4a3a7243196d2a154d6433c86f4e3ff4432a091f01fe6144`; e-print
  <https://arxiv.org/e-print/1107.0863v3> (`application/gzip`), 4,152,829 bytes, SHA-256
  `13e63ce74f6b6101a7fa3d29c9d466ec410a71eff57c0bc43d4665bdbd1c1727` — a gzipped tar holding
  `CAT_0__to_trees3_25Mar2013.tex` and figure files.
- **Content used:** Lemma 12 (TeX label `lem:normal`, Section 6.4) with its proof, and Theorem 1
  (TeX label `theorem1`) with its proof in Section 7 — the two claims of record; locating
  sentences quoted in the two `RECORD.md` files.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` items 9 and 10); no CLAIM.

---

## S-0039 — Chepoi and Hagen, arXiv:1107.0863 **v4** — the correcting version, its comment and its appendix "correction to …" (the status document of Route A items 9 and 10)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0038; **v4 submitted 22 April 2026**, comment: "Previous version had an
  error in Lemma 12, affecting Theorem 1. Current version has appendix correcting Theorem 1
  under additional hypothesis: no vertex has a 5-cycle in its link or, equivalently, the crossing
  graph has no 5-cycle. (4-cycles, and cycles larger than 5, are allowed.) Theorem 2, is
  unchanged. Appendix appears as journal correction: https://doi.org/10.1016/j.jctb.2026.04.001".
  The journal correction's DOI is as stated in the comment and was not retrieved separately.
- **Primary:** abstract page <https://arxiv.org/abs/1107.0863v4>, accessed 2026-10-14T03:53:05Z
  (HTTP 200), 42,134 bytes, SHA-256
  `3338cd9a57c806ddac22bab24a36abd77181f17544ce21e4fabbb62e0421eb1d`; e-print
  <https://arxiv.org/e-print/1107.0863v4> (`application/gzip`), 4,287,074 bytes, SHA-256
  `489ab21588b9d61ba5c13d7ce4bb3a68e2b5a4d7cf98d24b5def084781d97915` — a gzipped tar holding
  `CAT_0__to_trees4_17Apr2013.tex` (the original text plus the appendix) and figure files.
- **Content used:** the appendix "Appendix: correction to *On embeddings of CAT(0) cube complexes
  into products of trees via colouring their hyperplanes*" (authors Chepoi and Hagen): "Theorem
  1 of [CH] is incorrect in the stated generality due to an error in [Lem. 12]. We correct this
  under an extra hypothesis on X by proving:" Theorem A (Theorem 1 with the added hypothesis that
  no vertex link contains a 5-cycle, equivalently the crossing graph has no 5-cycle); "The only
  difference between Theorem A and [Thm. 1] is the addition of the hypothesis about 5-cycles";
  thanks to James Davies and Harry Petyt "for showing us examples that led us to find the error in
  our original argument; these will be in their forthcoming paper". Section "Summary of
  corrections": "The error in [CH] is in the proof of [Lem. 12], which is false as stated; Figure
  A shows a counterexample" (the figure: paths H₁ → H₂ → H₃ → H₄ and H₁ → H₀ → H₄ forming a
  5-cycle in Υ₀(U)); "Proposition A replaces Proposition 11 and Lemma 12"; Lemmas 8 and 9 hold as
  written; "The other results in [CH] are unaffected." The proof of Theorem A "proceeds exactly as
  in [Sec. 7], with one difference" — the colouring of Υ(U), now by the new Proposition A
  ("Suppose that Γ_#(X) has no 5-cycle. Then Υ₀(U) is bipartite"). The original text is
  otherwise unchanged apart from a moved paragraph and packaging. The scanner's one "llm" hit in
  the sources is the substring of a citation key (`BallmanSwiatkowski`); no AI-assistance mention.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v4 abstract page;
  copy not committed.
- **Bears on:** S1.1 items 9 and 10 (their status document); no CLAIM.

---

## S-0040 — Ursula Hamenstädt, "Symbolic dynamics for the Teichmueller flow", arXiv:1112.6107 **v2** (the original of Route A item 11)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 14 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 16, 2026-10-14.
- **Citation (from the abstract page):** Ursula Hamenstädt, "Symbolic dynamics for the
  Teichmueller flow", arXiv:1112.6107, Dynamical Systems (math.DS); v1 28 December 2011; **v2
  submitted 18 December 2023** (comment: "Article expanded and split into three parts. This is
  part 2. 32 p, 1 figure" — a revision, not a correction notice); v3 13 April 2025 (S-0041).
- **Primary:** abstract page <https://arxiv.org/abs/1112.6107v2>, accessed 2026-10-14T04:53:16Z
  (HTTP 200), 39,032 bytes, SHA-256
  `871136a430c430d9cabd57fdcd8e3de0ef432e4dbbff073b0d4950592b775742`; e-print
  <https://arxiv.org/e-print/1112.6107v2> (`application/gzip`), 46,945 bytes, SHA-256
  `f1bfb2c30d7a9c98c12cb83d929de33c853825a5c4b56e6a1042a3076fde4da4` — a gzipped tar holding
  `symbolic3.tex` and a figure.
- **Content used:** Theorem 1 (TeX label `coding`) — its statement, whose last clause is the
  documented error — and the paper's proof of it; locating sentences quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v2 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` item 11); no CLAIM.

---

## S-0041 — Hamenstädt, arXiv:1112.6107 **v3** — the correcting version and its comment (the status document of Route A item 11)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0040; **v3 submitted 13 April 2025**, comment: "Correction of the last
  statement in Theorem 1, proofs are not affected. Details added, writing improved. 35p".
- **Primary:** abstract page <https://arxiv.org/abs/1112.6107v3>, accessed 2026-10-14T04:53:16Z
  (HTTP 200), 38,962 bytes, SHA-256
  `1171502fa5f54cfa85dfd1e08c567cb8bd1ce2c0050d2d8cfaca9246379209a0`; e-print
  <https://arxiv.org/e-print/1112.6107v3> (`application/gzip`), 55,742 bytes, SHA-256
  `dd2e3d03795018fef1d251a1a9f5d5fe5064061389a339eb346b2f4a779234c2` — a gzipped tar holding
  `symbolic3.tex` and a figure.
- **Content used:** Theorem 1's last clause changes from "which maps the space of σ-invariant
  Borel probability measures on 𝒰 continuously onto ℳ_inv(𝒬)" (v2) to "which maps the space of
  Θ^t-invariant Borel probability measures on X continuously onto ℳ_inv(𝒬)" (v3), where
  (X, Θ^t) is the suspension of the shift σ over 𝒰 and Ξ: (X, Θ^t) → (𝒬, Φ^t) is the
  semi-conjugacy of the statement; the introduction now also calls Ξ "a (partial)
  semi-conjugacy … however it is not surjective". The rest of the 3,720-line diff is the
  "details added, writing improved" the comment announces (expository rewording, references,
  an acknowledgement to referees); no other result's statement is named by the comment. The
  scanner's "llm" hit in the sources is the TeX label `fillmore` (a lemma named for Fillmore),
  checked; no AI-assistance mention was found.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 11 (its status document); no CLAIM.

---

## S-0042 — Lukas Pottmeyer, "Heights of points with bounded ramification", arXiv:1201.3327 **v2** (the original of Route A items 12 and 13)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The items' `raw/1` file records are `external`.
- **How found:** Route A, candidate 15 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 16, 2026-10-14. One candidate, two documented results → two items sharing
  this original and S-0043.
- **Citation (from the abstract page):** Lukas Pottmeyer, "Heights of points with bounded
  ramification", arXiv:1201.3327, Number Theory (math.NT); v1 16 January 2012; **v2 submitted 18
  November 2013** (comment: "Major changes regarding the first version. E.g.: the title was
  changed; errors were corrected; based on a remark of Joseph Silverman Example 5.7 and Theorem
  5.9 were added" — a 2013 revision, outside the window); v3 5 May 2023 (S-0043). The v3 text
  refers to "the published version of this article".
- **Primary:** abstract page <https://arxiv.org/abs/1201.3327v2>, accessed 2026-10-14T04:53:37Z
  (HTTP 200), 42,130 bytes, SHA-256
  `0fb78c5db4a727df6bd3ba9c314d0f994ff88e339d18da149758424662e65168`; e-print
  <https://arxiv.org/e-print/1201.3327v2> (`application/gzip`), 16,122 bytes, SHA-256
  `f15a4756a72d4616ef5801d63440501756109f09620727733d4fb8af9a94db33` — a single gzipped TeX file.
- **Content used:** the former Lemma 5.8 (Section 5) with its proof, and the former Theorem 5.9
  with its proof ("If we combine this lemma with the preceding results we obtain …") — the two
  claims of record; statements quoted in the two `RECORD.md` files.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v2 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` items 12 and 13); no CLAIM.

---

## S-0043 — Pottmeyer, arXiv:1201.3327 **v3** — the correcting version, its comment and its in-text notes (the status document of Route A items 12 and 13)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0042; **v3 submitted 5 May 2023**, comment: "There was an error in the proof
  of the former Lemma 5.8. This false lemma and the former Theorem 5.9 have been deleted in this
  version".
- **Primary:** abstract page <https://arxiv.org/abs/1201.3327v3>, accessed 2026-10-14T04:53:37Z
  (HTTP 200), 41,990 bytes, SHA-256
  `0ae768a05760f7354e899679090784db79900ba87db4b6df4745bdc313354c3d`; e-print
  <https://arxiv.org/e-print/1201.3327v3> (`application/gzip`), 16,661 bytes, SHA-256
  `a23aec07c4ce09623068411872b428308b555748a8c43ccfa268652527ea6e7e` — a single gzipped TeX file.
- **Content used:** the acknowledgement gains "In the published version of this article there is
  an error in the proof of the former Lemma 5.8. This was noticed by Francesco Amoroso and Lea
  Terracini produced a counterexample to prove that the whole statement was false. I thank both
  of them for the chance to remove this long standing error."; the lemma and theorem are removed
  and replaced by the note "THE FOLLOWING IS NOT EQUIVALENT TO THE CONTENT OF THE PUBLISHED
  VERSION!" and the sentences "In contrast to the statement given in the published article, I
  am not able to classify all these field extensions. This is due the fact that the claimed
  Lemma 5.8 in the previous version of this article is not true - a counterexample was found by
  Francesco Amoroso and Lea Terracini.", followed by a weaker Proposition (TeX label
  `thm:finex`) described as "the trivial consequence of the preceding results"; the deleted
  proof and a remark remain in the source as comments. The counterexample itself is not in the
  document. The scanner's "llm" hit in the sources is the surname Ullmo in the acknowledgement.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page;
  copy not committed.
- **Bears on:** S1.1 items 12 and 13 (their status document); no CLAIM.

---

## S-0044 — César Massri, "Irreducibility of G-varieties defined by quadrics", arXiv:1202.5531 **v3** (the original of Route A item 14)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 16 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 17, 2026-10-14.
- **Citation (from the abstract page):** César Massri, "Irreducibility of G-varieties defined by
  quadrics", arXiv:1202.5531, Algebraic Geometry (math.AG); v1 24 February 2012, v2 3 May 2012,
  **v3 submitted 27 March 2013** (no comment); v4 23 August 2026 (S-0045). No journal reference
  or DOI on arXiv (API query this session): a preprint. Abstract: "Let 𝔤 be a complex semisimple
  Lie algebra, G a simply connected and connected Lie group with Lie algebra 𝔤 and V a finite
  dimensional representation. We prove that the zero locus of quadrics containing G.y is an
  irreducible variety in ℙV."
- **Primary:** abstract page <https://arxiv.org/abs/1202.5531v3>, accessed 2026-10-14T08:09:19Z
  (HTTP 200), 38,272 bytes, SHA-256
  `b28e0061b31121853f6e9e465cc576341ec4727d0add0f1e832a6f42806f0837`; e-print
  <https://arxiv.org/e-print/1202.5531v3> (`application/gzip`), 11,081 bytes, SHA-256
  `d99f8d19d17391a4285c1dddf0e6b98dab3852b120742a8135d0e979eb0e2cf9` — a single gzipped TeX file
  (`main.tex`).
- **Content used:** Theorem 6 (TeX label `irr-phi-cat`, the paper's main result) — its statement,
  standing hypotheses and five-step proof, Step 4 in particular — and the unnumbered statement of
  the result in the introduction; quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` item 14); no CLAIM.

---

## S-0045 — Massri, arXiv:1202.5531 **v4** — the correcting version and its comment (the status document of Route A item 14)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0044, but under the new title "Irreducibility of quadratic hulls of
  osculating varieties of closed orbits"; **v4 submitted 23 August 2026**, comment: "Major
  revision. The main irreducibility theorem in the previous version is false in the stated
  generality. The error occurs in Step 4 of the proof of Theorem 6. The present version replaces
  the incorrect general statement by a valid result for quadratic hulls of osculating varieties
  of closed orbits under suitable hypotheses".
- **Primary:** abstract page <https://arxiv.org/abs/1202.5531v4>, accessed 2026-10-14T08:09:19Z
  (HTTP 200), 40,372 bytes, SHA-256
  `8ab470eb3dfd5136f631db7ece414568b1b5eef6d01626c3bbf551194d982d99`; e-print
  <https://arxiv.org/e-print/1202.5531v4> (`application/gzip`), 16,787 bytes, SHA-256
  `b0b1ec31ec96234b0b921b167111f00878af478d76c80789170df8945e6f6b3a` — a gzipped tar holding
  `irred.tex`, `citas2.bib` and arXiv's `00README.json`.
- **Content used:** the comment (the only place the previous theorem and its error are named);
  the v4 text, a complete rewrite (2,003 source lines against 666; new title, abstract and
  bibliography) whose main theorem is "if d > 4p" the quadratic hull of the p-osculating variety
  of the closed highest-weight orbit X_{dλ} equals the osculating variety, "In particular, the
  quadratic hull is irreducible"; its introduction's reducible examples — Böhning's Z₃ ⊂ ℙS⁸V
  "has two irreducible components", and for the third osculating variety of the rational normal
  curve c₁₀, "Since 28 does not divide 64, the quadratic hull has another irreducible component
  of dimension 4. Thus even in the binary case the quadratic hull of an osculating variety may be
  reducible"; and "In general, 𝒬(Y) may be strictly larger than Y and need not be irreducible".
  The text does not refer to the previous version's theorem or to Step 4. The AI-assistance
  scanner reports no hit on either version.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v4 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 14 (its status document); no CLAIM.

---

## S-0046 — Terence Tao and Van Vu, "Random matrices: Universality of local spectral statistics of non-Hermitian matrices", arXiv:1206.1893 **v6** (the published version; the original of Route A items 15 and 16)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The items' `raw/1` file records are `external`.
- **How found:** Route A, candidate 17 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 17, 2026-10-14. One candidate, two documented results → two items sharing
  this original and S-0047.
- **Citation (from the abstract page):** Terence Tao and Van Vu, "Random matrices: Universality
  of local spectral statistics of non-Hermitian matrices", arXiv:1206.1893, Probability
  (math.PR); v1 9 June 2012 … **v6 submitted 16 March 2015** (comment: "Published in at
  <http://dx.doi.org/10.1214/13-AOP876> the Annals of Probability … by the Institute of
  Mathematical Statistics" — the published version, submitted via the VTEX proxy); v7 25 May
  2024 (S-0047).
- **Primary:** abstract page <https://arxiv.org/abs/1206.1893v6>, accessed 2026-10-14T08:15:08Z
  (HTTP 200; a first retrieval at 08:12:31Z fetched the same pages), 47,236 bytes, SHA-256
  `fc2a781846d0939c332c987235e890aeec0ce4328b9fc85de9726c21b5f79aa8`; e-print
  <https://arxiv.org/e-print/1206.1893v6> (`application/gzip`), 327,296 bytes, SHA-256
  `8e7b0c7fbae142df4a267b6999a18f8ad6a7ec62292754c865aa90b492a5098c` — a gzipped tar holding
  the publisher's TeX `aop876.tex` and figures.
- **Content used:** Section 4 "A concentration inequality": the setup (Y = Y(ξ₁, …, ξₙ) a random
  variable of independent atom variables; the martingale differences C_i(ξ) of equation `mdif`;
  Azuma's inequality as recalled), Proposition 34 (TeX label `azuma0`; the publisher's markers
  `%pr34`, `%pr35` fix the numbering) with its proof, and Proposition 35 "Martingale
  concentration" (label `azuma`) with its proof — the two claims of record; quoted in the two
  `RECORD.md` files.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v6 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hash above pins what was read.
- **Bears on:** S1.1 (`groundtruth/` items 15 and 16); no CLAIM.

---

## S-0047 — Tao and Vu, arXiv:1206.1893 **v7** — the correcting version, its comment and its "Note added after publication" (the status document of Route A items 15 and 16)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0046; **v7 submitted 25 May 2024**, comment: "Published in at [the DOI] the
  Annals of Probability … . This version notes an error in Proposition 34, and indicates a
  workaround so as not to impact the remainder of the paper".
- **Primary:** abstract page <https://arxiv.org/abs/1206.1893v7>, accessed 2026-10-14T08:15:08Z
  (HTTP 200), 46,744 bytes, SHA-256
  `5ef405a74cd3d259fbfe56a8808b4ed5d10b0914b90a6cb862bd191b53fd10de`; e-print
  <https://arxiv.org/e-print/1206.1893v7> (`application/gzip`), 1,642,248 bytes, SHA-256
  `a890683f825cce05456b41abbf571798a8a7d7f0029a7bf8556a5eec83e50d5c` — a gzipped tar holding
  the authors' TeX `iid11.tex` and figures.
- **Content used:** the remark added after the proof of Proposition 35: "(Note added after
  publication.) It has been pointed out to us by Heejune Sheen, and independently by Christian
  Borgs and Karissa Huang, that Proposition 34 is incorrect; the random variable Y′ constructed
  above does not, in fact, obey the hypotheses of Azuma's inequality. A version of the
  proposition can be salvaged by replacing C_i(ξ₁, …, ξ_i) with the larger quantity
  sup_η C_i(ξ₁, …, ξ_{i−1}) (and redefining the events B_i accordingly); however, for the
  purposes of establishing Proposition 35 (which is the only place in this paper where
  Proposition 34 is used), it is better to proceed as follows. Firstly, one should impose an
  additional mild moment hypothesis such as 𝔼ξ_i² = O(n^{O(1)}), which is true in all
  applications of interest. Then one can define Y′ to be the same as Y but with each ξ_i replaced
  by a mean zero modification ξ′_i of size O(n^{o(1)}) almost surely that agrees with ξ_i with
  overwhelming probability, and c_i(ξ₁, …, ξ_{i−1}) similarly replaced by c′_i(ξ′₁, …, ξ′_{i−1})
  that is bounded by α_i almost surely and agrees with c_i with overwhelming probability." (The
  proposition numbers are written as `\ref`s to `azuma0` and `azuma`.) The 11,140-line diff is
  the publisher's TeX against the authors' TeX; the statements and proofs of Propositions 34 and
  35 are the same in both versions, and the remark is the one addition the comment announces.
  The AI-assistance scanner reports no hit.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v7 abstract page;
  copy not committed.
- **Bears on:** S1.1 items 15 and 16 (their status document); no CLAIM.

---

## S-0048 — Michael Beeson, "Triangle Tiling V: Tilings by a tile with integer sides", arXiv:1206.2228 **v1** (the original of Route A item 17)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 18 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 17, 2026-10-14.
- **Citation (from the abstract page):** Michael Beeson, "Triangle Tiling V: Tilings by a tile
  with integer sides", arXiv:1206.2228, Metric Geometry (math.MG); **v1 submitted 5 June 2012**
  (no comment); v2 27 May 2024, withdrawn (S-0049). A 46-page preprint.
- **Primary:** abstract page <https://arxiv.org/abs/1206.2228v1>, accessed 2026-10-14T08:15:30Z
  (HTTP 200), 42,150 bytes, SHA-256
  `56a770d5fa2e0d1c693631166865951ffa7e253f82ed11b704930903abf980e0`; e-print
  <https://arxiv.org/e-print/1206.2228v1> (`application/gzip`), 44,667 bytes, SHA-256
  `781abaf3afeac885a2a25b41aea02a56b7ac68ce39730d532f3b2caf4ae7126e` — a gzipped tar holding
  `TriangleTiling5.tex` and `TriangleTiling5.bbl`; and, to resolve the notice's page-and-line
  reference, the PDF <https://arxiv.org/pdf/1206.2228v1>, accessed 2026-10-14 at about 08:17Z
  (HTTP 200, `application/pdf`), 349,941 bytes, SHA-256
  `3d11b49d09399e8399e74b7df9b8f0392c094bbddfdc8712d8cccb6f4d24907a`, 46 pages.
- **Content used:** Theorem 1 (TeX label `theorem:everyrowhasazero`, the first theorem
  environment) — its statement and its proof, whose kernel argument begins at the top of page 8
  of the PDF; quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v1 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` item 17); no CLAIM.

---

## S-0049 — Beeson, arXiv:1206.2228 **v2** — the withdrawal and its comment (the status document of Route A item 17)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0048; **v2 submitted 27 May 2024, withdrawn** (1 KB; no e-print, HTTP 404),
  comment: "Theorem 1 is wrong; tilings described in later papers contradict it. Line 4, page 8,
  is the error in the alleged proof. This paper is not used in any of my other papers on
  triangle tiling".
- **Primary:** abstract page <https://arxiv.org/abs/1206.2228v2>, accessed 2026-10-14T08:15:30Z
  (HTTP 200), 42,101 bytes, SHA-256
  `88d290bc8aae5abb86e27865e3cf3922ab6156f127f5fd65c121d3d7a08ab3fe`. The withdrawn version has
  no e-print and its abstract page shows no licence link.
- **Content used:** the comment, the whole of the status document: it names the result, states
  that it is wrong, gives the ground (contradicting tilings in the author's later papers — not
  in this document and not named), and locates the error to a line of the proof. The later
  papers were not retrieved: the item's truth is recorded by the document method on the author's
  statement.
- **Licence:** none stated for the stub; nothing retrievable to commit.
- **Bears on:** S1.1 item 17 (its status document); no CLAIM.

---

## S-0050 — David Ayala and John Francis, "Factorization homology of topological manifolds", arXiv:1206.5522 **v5** (the original of Route A item 18)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 19 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 18, 2026-10-14.
- **Citation (from the abstract page):** David Ayala, John Francis, "Factorization homology of
  topological manifolds", arXiv:1206.5522, Algebraic Topology (math.AT; cross-listed math.GT,
  math.QA); v1 24 June 2012, v2 29 August 2013, v3 22 April 2014, v4 13 May 2015, **v5 submitted
  18 August 2015** (comment "37 pages. Accepted for publication by Journal of Topology"); v6
  23 June 2024 (S-0051). Journal reference on the abstract page: J. Topol. 8 (2015), no. 4,
  1045–1084; related DOI 10.1112/jtopol/jtv028. The published version was not retrieved; the
  original of record is v5, the accepted version.
- **Primary:** abstract page <https://arxiv.org/abs/1206.5522v5>, accessed 2026-10-14T19:45:32Z
  (HTTP 200), 43,810 bytes, SHA-256
  `64a931cffd932d0b932075cd361ca2a30c1ff4e90090dab1fa0b93adcb114859`; e-print
  <https://arxiv.org/e-print/1206.5522v5> (`application/gzip`), 43,241 bytes, SHA-256
  `dc5828b3f1403bfb32140ecbc6687c634ef712456003882373a551eede869a4f` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** Proposition 5.3 (Subsection 5.1, "Factorization homology with coefficients in
  commutative algebras"; unlabelled in v5 — the third numbered environment of Section 5, after
  Proposition 5.1 `tensor` and an unlabelled Corollary 5.2, so the comment's "Proposition 5.3" is
  this one) — its statement and proof, including the convergence step that cites Dwyer [Dw] and
  Hilton–Mislin–Roitberg–Steiner [HMRS]; quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v5 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` item 18); no CLAIM.

---

## S-0051 — Ayala and Francis, arXiv:1206.5522 **v6** — the correcting version and its comment (the status document of Route A item 18)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0050; **v6 submitted 23 June 2024**, comment: "37 pages. Corrected
  Proposition 5.3".
- **Primary:** abstract page <https://arxiv.org/abs/1206.5522v6>, accessed 2026-10-14T19:45:32Z
  (HTTP 200), 43,703 bytes, SHA-256
  `0ee83fd6aa33e29473f00b09ee3bc931b4b516826eba64dd22c8efccf341dbbf` (re-fetched at about
  19:49Z to read the journal fields; byte-identical); e-print
  <https://arxiv.org/e-print/1206.5522v6> (`application/gzip`), 43,658 bytes, SHA-256
  `7a5c8411a01861b5b3177765c3f247dfcd53b8e6c7f9755cf4e982819b0129c4` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** the comment; the v5 → v6 TeX diff (61 lines, the whole of it): the
  acknowledgement's added sentence "We thank Amabel Wilson and Theo Johnson–Freyd for
  correcting the hypotheses in the statement of Proposition 5.3"; the restated Proposition 5.3
  (now labelled `prop.5.3`), whose hypotheses "Let M be an n-manifold, and let X be a nilpotent
  n-connective space of finite type over R such that π_n X is finite" replace v5's "Let M be an
  n-manifold. Let X be a nilpotent space whose first n homotopy groups are finite, |π_i X| < ∞
  for i ≤ n, and let R be any commutative ring" (the conclusion is unchanged); and its proof's
  two changed steps — the convergence step, which now states Dwyer's criterion as requiring a
  connected base and a nilpotent action of π₁ of the base on the homotopy of the fibre and
  derives both from n-connectivity (a semi-direct-product computation of π₁(X^{S^k})), and the
  finiteness step, which now reads "Since π_n X is finite … Since they are additionally finite
  CW complexes and X is finite type, the homology groups … are finite rank over R". The
  AI-assistance scanner reports no hit on either version.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v6 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 18 (its status document); no CLAIM.

---

## S-0052 — César Galindo, "On braided and ribbon unitary fusion categories", arXiv:1209.2022 **v2** (the original of Route A items 19 and 20)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The items' `raw/1` file records are `external`.
- **How found:** Route A, candidate 21 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 18, 2026-10-14.
- **Citation (from the abstract page):** César Galindo, "On braided and ribbon unitary fusion
  categories", arXiv:1209.2022, Quantum Algebra (math.QA); v1 10 September 2012, **v2 submitted
  23 May 2013** (comment "The introduction have been improved"); v3 6 December 2024 (S-0053).
  No journal reference or DOI on the abstract page; v3's remark speaks of "the published version
  of this work", which was not identified or retrieved. A seven-page note. Abstract (v2): "We
  prove that every braiding over a unitary fusion category is unitary and every unitary braided
  fusion category admits a unique unitary ribbon structure."
- **Primary:** abstract page <https://arxiv.org/abs/1209.2022v2>, accessed 2026-10-14T19:51:37Z
  (HTTP 200), 38,028 bytes, SHA-256
  `788378057c63d7645c1696b4cf801eb3e7317cc8ddf268a5e8b101dfe8b9be31`; e-print
  <https://arxiv.org/e-print/1209.2022v2> (`application/gzip`), 7,300 bytes, SHA-256
  `21f63ae00385cda7216c843cfc4730e994f1fe0c40f645bb49bb323a2456d0ed` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** Subsection 3.1, "The center of a unitary fusion category": the definition of
  the unitary center 𝒵*(𝒞); Proposition 3.1 (TeX label `centro igual a centro unitario`) with
  its proof; Theorem 3.2 (TeX label `braids are unitary`) with its proof; and Remark 2.1
  (`remark polar`, the polar decomposition), which the proof of Proposition 3.1 invokes. The
  theorem counter is per section and shared by propositions, lemmas, remarks and definitions;
  Proposition 3.1 and Theorem 3.2 are the first two numbered environments of Section 3, so the
  comment's numbering is v2's. Quoted in the two `RECORD.md` files.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v2 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` items 19 and 20); no CLAIM.

---

## S-0053 — Galindo, arXiv:1209.2022 **v3** — the correcting version, its comment and its remark (the status document of Route A items 19 and 20)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0052; **v3 submitted 6 December 2024**, comment: "In this version,
  Proposition 3.1, which discussed the equality of the unitary center and the ordinary center,
  has been removed. While these centers are equivalent as braided fusion categories, the unitary
  center is generally a non-replete subcategory. The proof of Theorem 3.2 has been revised to
  avoid reliance on this proposition".
- **Primary:** abstract page <https://arxiv.org/abs/1209.2022v3>, accessed 2026-10-14T19:51:37Z
  (HTTP 200), 38,328 bytes, SHA-256
  `093d48181d3dd3b6590de732217d07016c19f2501c1956546ec523806f108de5` (re-fetched at about
  19:54Z to read the journal fields, which are absent; byte-identical); e-print
  <https://arxiv.org/e-print/1209.2022v3> (`application/gzip`), 7,668 bytes, SHA-256
  `59b919282e273cadb2a12baa8a48b90cdb79468ce3beb6931cd0c37bc2fb89a7` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** the comment; the v2 → v3 TeX diff (410 lines, most of them reflowed prose
  and a reformatted bibliography): the acknowledgement's added sentence "The author is also
  grateful to Tian Lan for identifying an error in Proposition 3.1 of the previous version"; the
  unnumbered remark that stands where Proposition 3.1 stood — "In the published version of this
  work, it was stated that the inclusion functor 𝒵*(𝒞) ⊆ 𝒵(𝒞), being an equivalence as
  established in [GHR, Proposition 5.24] and [Mueg2, Theorem 6.4], implied that 𝒵*(𝒞) = 𝒵(𝒞).
  However, thanks to an observation by Tian Lan, we now understand that, in general,
  𝒵*(𝒞) ≠ 𝒵(𝒞); in other words, 𝒵*(𝒞) is not a replete subcategory of 𝒵(𝒞). To illustrate
  this distinction, consider the following case, as pointed out by Tian Lan: URep(G), the
  category of unitary representations of a finite group G, and Rep_ℂ(G), the category of all
  complex representations of G. Although these two categories are equivalent as fusion
  categories, they are not identical in terms of their objects. Indeed, Tian Lan provided a
  counterexample using the category Hilb_G of finite-dimensional Hilbert spaces graded over G,
  where we have URep(G) ⊊ Rep(G) ⊂ 𝒵(Hilb_G) and URep(G) ⊂ 𝒵*(Hilb_G), yet the inclusion
  URep(G) ⊊ Rep(G) is not a replete subcategory. This nuance arises because every complex
  representation is unitarizable (i.e., isomorphic to a unitary representation)."; and the new
  proof of Theorem 3.2 (its statement unchanged), which works with the object (X, c_{−,X}) of
  𝒵(𝒞), takes an equivalent object (Y, τ_{−,Y}) of 𝒵*(𝒞), identifies τ_{−,Y} with c_{−,Y} by
  naturality, and applies the polar decomposition to the isomorphism f: X → Y in 𝒞 only. The
  AI-assistance scanner reports no hit on either version.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page;
  copy not committed.
- **Bears on:** S1.1 items 19 and 20 (their status document); no CLAIM.

---

## S-0054 — Katrin Wehrheim, "Fredholm notions in scale calculus and Hamiltonian Floer theory", arXiv:1209.4040 **v3** (the original of Route A item 21)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 22 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 19, 2026-10-14.
- **Citation (from the abstract page):** Katrin Wehrheim, "Fredholm notions in scale calculus
  and Hamiltonian Floer theory", arXiv:1209.4040, Symplectic Geometry (math.SG); v1 18 September
  2012; v2 14 January 2013 ("v2: corrections in some nonlinear estimates"); **v3 submitted
  25 October 2016** ("v3: complete revision - giving full details and correcting substantial
  mistakes in Floer application") — v2's and v3's own corrections are outside the ADR-0003 D8
  window and are not harvested; v4 10 September 2025 (S-0055). No journal reference or DOI on
  the abstract page: a preprint. One author.
- **Primary:** abstract page <https://arxiv.org/abs/1209.4040v3>, accessed 2026-10-14T20:14:48Z
  (HTTP 200), 39,385 bytes, SHA-256
  `dcfc69f6e526b4ad635dab868746d62d63a0744f8457d5d622a4e88e89968854`; e-print
  <https://arxiv.org/e-print/1209.4040v3> (`application/gzip`), 43,630 bytes, SHA-256
  `6a907dc3753a3a56f8ccf28dbc58a8ce848fea2d48c1728a1f1979f7fa7eabd0` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** Section 4, "Fredholm property for nonlinear maps": Definition 4.3 (TeX label
  `def:scfred`, "sc-Fredholm at 0 with respect to the splitting"), in particular its clause (iii)
  "weakly regularizing, i.e. ker D_{E′}Φ(r,0) ⊂ E′_1"; Theorem 4.5 (label `thm`, the theorem the
  comment calls the main theorem) and its proof, in particular the injectivity step quoted in
  `RECORD.md`; Section 5's Lemma `le3` (the Floer application's verification of clause (iii)),
  read for the diff. The theorem counter is per section and shared: Section 4 holds Definition
  4.1 (`def:HWZfred`), Remark 4.2, Definition 4.3, Remark 4.4, Theorem 4.5, so the comment's
  numbers are v3's.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` item 21); no CLAIM.

---

## S-0055 — Wehrheim, arXiv:1209.4040 **v4** — the correcting version and its comment (the status document of Route A item 21)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0054; **v4 submitted 10 September 2025**, comment: "v4:The notion of
  "weakly regularizing" (in Def.4.3) was strengthened to fix a gap in the proof of the main
  theorem (Thm.4.5)".
- **Primary:** abstract page <https://arxiv.org/abs/1209.4040v4>, accessed 2026-10-14T20:14:48Z
  (HTTP 200), 39,270 bytes, SHA-256
  `80202a75a436c6a1340531f5228cc720a4b9885dd677b97fbc9613f761c065b7` (re-fetched at about
  20:17Z to read the journal fields, which are absent; byte-identical); e-print
  <https://arxiv.org/e-print/1209.4040v4> (`application/gzip`), 46,286 bytes, SHA-256
  `5f59f9ad14ea8e594accbbfdee865fdbf5083873dc23cefa382bcf8c0e9213ed` — a gzipped tar holding
  `fredholm-arxiv.tex` and arXiv's `00README.json`.
- **Content used:** the comment; the v3 → v4 TeX diff (1,183 lines, most of them reference
  updates to the polyfold book, a new appendix "Brief Guide to Polyfold Theory", and
  commented-out `\todo` notes left in the source that describe the edits): Definition 4.3 (iii)
  "weakly regularizing, i.e. ker D_{E′}Φ(r,0) ⊂ E′_1" becomes "these operators are weakly
  regularizing in the sense that for all |r| < ε_0 and e ∈ E′_0 we have the implication
  D_{E′}Φ(r,0) e ∈ F_1 ⟹ e ∈ E′_1", with the source note "Here the notion of "weakly
  regularizing" was strengthened -- previously it was ker D_{E′}Φ(r,0) ⊂ E′_1"; Theorem 4.5's
  statement gains the words "in the sense of Definition 4.3" and is otherwise unchanged; its
  proof's injectivity step is rewritten (source note: "Here we clarified the use of the
  (strengthened) weak regularization property and added proof that D_{E′}Φ(r^ν,0) is uniformly
  bounded"); Lemma `le3` of Section 5 asserts and proves the strengthened property for the
  linearized Floer operators (source note: "Updated the weak regularization claim"). The
  AI-assistance scanner reports no hit on either version.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v4 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 21 (its status document); no CLAIM.

---

## S-0056 — Aleksandra Kwiatkowska, "Large conjugacy classes, projective Fraïssé limits, and the pseudo-arc", arXiv:1210.6709 **v1** (the original of Route A items 22, 23 and 24)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The items' `raw/1` file records are `external`.
- **How found:** Route A, candidate 23 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 19, 2026-10-14.
- **Citation (from the abstract page):** Aleksandra Kwiatkowska, "Large conjugacy classes,
  projective Fraïssé limits, and the pseudo-arc", arXiv:1210.6709, Logic (math.LO; cross-listed
  math.GN); **v1 submitted 25 October 2012** (comment "18 pages"); v2 20 April 2023 under the
  new title "Large orbits, projective Fraïssé limits, and the pseudo-arc" (S-0057). No journal
  reference or DOI on the abstract page; not retrieved. One author. Abstract (v1): "We show that
  the automorphism group, Aut(ℙ), of a projective Fraïssé limit ℙ, whose natural quotient is the
  pseudo-arc, has a comeager conjugacy class. This generalizes an unpublished result of
  Oppenheim that Aut(ℙ) (and consequently, the group of all homeomorphisms of the pseudo-arc)
  has a dense conjugacy class. We also present a simple proof of the result of Oppenheim."
- **Primary:** abstract page <https://arxiv.org/abs/1210.6709v1>, accessed 2026-10-14T20:15:06Z
  (HTTP 200), 38,596 bytes, SHA-256
  `a50b9233f8615a5bd65c5330a523879a8890943bfa3dec6e509985774d99f9e3`; e-print
  <https://arxiv.org/e-print/1210.6709v1> (`application/gzip`), 19,732 bytes, SHA-256
  `3c24927fdca7fbbf289f27839f7308a2b3dcdef23084d80608c7f94d13a5bb34` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** Theorem 1.6 (TeX label `abcd`, the main result, restated as Theorem 3.1,
  label `maint`) and its Section 3 proof (Propositions 3.2 `ap` and 3.3 `comeager`, Lemma 3.4
  `ddd`); Section 2's Theorem 2.1 (`pseu`, attributed "[Oppenheim]") and Theorem 2.2 (`aut`,
  "[Oppenheim]") with the paper's own proofs (Propositions 2.5 `jpp` and 2.6 `dense`, Lemmas
  2.12 `slawek` and 2.13 `pse`, and the class 𝓕 of finite structures (A, s^A) induced by
  automorphisms of ℙ); the acknowledgement crediting a proof of Lemma 2.12 to Sławomir Solecki.
  Numbering: the theorem counter is per section and shared by all environments; **v1 has no
  Theorem 1.7** — Section 1 ends with Theorem 1.6 — so the v2 comment's "Theorem 1.7" is v2's
  numbering, in which a new Proposition 1.3 shifts the main theorem to 1.7.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v1 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` items 22, 23 and 24); no CLAIM.

---

## S-0057 — Kwiatkowska, arXiv:1210.6709 **v2** — the correcting version, its comment and its remark (the status document of Route A items 22, 23 and 24)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0056, under the new title "Large orbits, projective Fraïssé limits, and the
  pseudo-arc"; **v2 submitted 20 April 2023**, comment: "Statement of Theorem 1.7 is corrected".
- **Primary:** abstract page <https://arxiv.org/abs/1210.6709v2>, accessed 2026-10-14T20:15:06Z
  (HTTP 200), 38,086 bytes, SHA-256
  `decce1a9e007287ed845267a8d8cecc56ee3150b89759c631ea4f53353d71556` (re-fetched at about
  20:17Z to read the journal fields, which are absent; byte-identical); e-print
  <https://arxiv.org/e-print/1210.6709v2> (`application/gzip`), 20,021 bytes, SHA-256
  `a52fadc29937333acb29a5fb6b759309e11c1c5271c4b389d177298e061b8288` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** the comment; the new title and abstract ("We show that the conjugacy action
  of the automorphism group, Aut(ℙ), of the projective Fraïssé limit ℙ, whose natural quotient
  is the pseudo-arc, on the set of involutions of ℙ, has a comeager orbit."); Theorem 1.7
  (label `abcd`): "The action Aut(ℙ) ↷ I(ℙ) has a comeager orbit." (I(ℙ) the set of involutions
  of ℙ, "a closed invariant subspace for the conjugacy action"); Section 2's Theorem 2.1 ("The
  action H(P) ↷ I(P) has a dense orbit.") and Theorem 2.2 ("The action Aut(ℙ) ↷ I(ℙ) has a dense
  orbit."), now without the "[Oppenheim]" attributions; and Remark 2.3, the whole of it: "We do
  not know if Aut(ℙ), has a dense conjugacy class or if H(P) has a dense conjugacy class. All
  existing "proofs" seem to have a flow." [sic]. The v1 → v2 TeX diff (1,047 lines): the class
  𝓕 is redefined with f ∈ I(ℙ) in place of f ∈ Aut(ℙ); "symmetric" is added to the hypotheses
  of Lemma `slawek` ("surjective, symmetric, and connected") and to the characterisation in
  Lemma `pse`; a new Proposition `lifttoinv` ("Let A ∈ 𝓖, let s^A be the antidiagonal of A, and
  let φ: ℙ → A be an epimorphism. Then there is h ∈ I(ℙ) such that φ: (ℙ,h) → (A,s^A) is an
  epimorphism") with Lemma `switch` replaces v1's lifting argument in Lemma `pse` and is used in
  Proposition `jpp`; a new Lemma `denseI(p)` and a new proof of Theorem 2.1; Section 3 is
  rewritten for the action on I(ℙ) with the same propositions. The AI-assistance scanner
  reports no hit on either version.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v2 abstract page;
  copy not committed.
- **Bears on:** S1.1 items 22, 23 and 24 (their status document); no CLAIM.

---

## S-0058 — Lei Fu and Daqing Wan, "A Class of Incomplete Character Sums", arXiv:1303.3650 **v2** (the original of Route A item 25)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 24 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 19, 2026-10-14.
- **Citation (from the abstract page):** Lei Fu, Daqing Wan, "A Class of Incomplete Character
  Sums", arXiv:1303.3650, Algebraic Geometry (math.AG; cross-listed math.NT); v1 15 March 2013;
  **v2 submitted 19 March 2014** (comment "Following the suggestion of the referee, we use
  tensor induction to study a class of incomplete character sums. Originally we use transfer,
  which is a special case of tensor induction, and which only works for rank one sheaves. The
  paper is to appear in Quarterly Journal of Mathematics"); v3 6 January 2025 (S-0059). No
  journal reference or DOI on the abstract page; the published version was not retrieved. Two
  authors.
- **Primary:** abstract page <https://arxiv.org/abs/1303.3650v2>, accessed 2026-10-14T20:15:25Z
  (HTTP 200), 38,292 bytes, SHA-256
  `a44522338986c9fd28c2034909c4e2ba684ba6d8ebf4a413caece8d21ab02729`; e-print
  <https://arxiv.org/e-print/1303.3650v2> (`application/gzip`), 11,478 bytes, SHA-256
  `7361c0027ddd151f1895f774bd24304643a781b529f9f4c659d41199d3d5f9b9` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** Section 4, "Character Sums on Curves": Theorem 4.6 — the sixth numbered
  environment of the section (Proposition 4.1, Theorem 4.2, Lemma 4.3, Remark 4.4, Theorem 4.5,
  Theorem 4.6; the counter is per section and shared) — its statement and proof; the
  introduction's conventions ("p is a fixed prime number, 𝔽_p is a finite field with p
  elements, and 𝔽 is an algebraic closure of 𝔽_p. For any power q of p, let 𝔽_q be the subfield
  of 𝔽 with q elements"). Quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v2 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` item 25); no CLAIM.

---

## S-0059 — Fu and Wan, arXiv:1303.3650 **v3** — the correcting version and its comment (the status document of Route A item 25)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0058; **v3 submitted 6 January 2025**, comment: "José Alves Oliveira and
  Lucas Reis point out an error in the old version of Theorem 4.6. The assumption that Tr(g) is
  not of the form r^q-r is corrected to that Tr(g) is not of the form r^p-r".
- **Primary:** abstract page <https://arxiv.org/abs/1303.3650v3>, accessed 2026-10-14T20:15:25Z
  (HTTP 200), 38,113 bytes, SHA-256
  `f1dd52c50155cdf5811534989f52b1a0579471e8e1e8fdf384fb9f192baccce3` (re-fetched at about
  20:17Z to read the journal fields, which are absent; byte-identical); e-print
  <https://arxiv.org/e-print/1303.3650v3> (`application/gzip`), 11,527 bytes, SHA-256
  `56343607e895ca241882168dd7b5cdb1a2ed24bee2589d2f2cf19c85e89e3164` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** the comment; the v2 → v3 TeX diff (40 lines, the whole of it): the title
  footnote gains "We would like to thank José Alves Oliveira and Lucas Reis for pointing out an
  error in the old version of Theorem 4.6"; in Theorem 4.6's statement "is not of the form
  r(t)^q−r(t) in 𝔽(t)" becomes "r(t)^p−r(t)"; in its proof the sentence "Since
  Tr_{𝔽_{q^d}/𝔽_q}(g) is not of the form r(t)^q−r(t) in 𝔽(t), the sheaf 𝓛_{ψ,Tr(g)} is
  geometrically nonconstant" changes in the same way; otherwise one notation change in Section 3
  ("tran" to "⊗-Ind") and a document-class option. The AI-assistance scanner reports no hit on
  either version.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 25 (its status document); no CLAIM.

---

## S-0060 — Christian Liedtke, "Supersingular K3 Surfaces are Unirational", arXiv:1304.5623 **v4** (the original of Route A items 26, 27, 28 and 29)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The items' `raw/1` file records are `external`.
- **How found:** Route A, candidate 25 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 20, 2026-10-14.
- **Citation (from the abstract page):** Christian Liedtke, "Supersingular K3 Surfaces are
  Unirational", arXiv:1304.5623, Algebraic Geometry (math.AG; cross-listed math.NT); v1 20 April
  2013, v2 4 June 2013, v3 13 March 2014, **v4 submitted 16 September 2014** (comment "31 pages;
  many details added, final version"); v5 8 February 2023 (S-0061). Journal reference on the
  abstract page: "Invent. Math. 200, 979-1014 (2015) and Invent. Math. 223 (2021), 1407-1409" —
  the paper and its published erratum; neither was retrieved. One author.
- **Primary:** abstract page <https://arxiv.org/abs/1304.5623v4>, accessed 2026-10-14T21:27:57Z
  (HTTP 200), 39,635 bytes, SHA-256
  `d67c3a08b304975da5f52721c017dd0afe7d51ee9eda0302f95f675e9597bacc`; e-print
  <https://arxiv.org/e-print/1304.5623v4> (`application/gzip`), 33,391 bytes, SHA-256
  `86a989c4aa282dc0d1f265abf3c5a862d53fba59248e214cd16339508f504f25` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** Section 3 "Continuous Families of Torsors": Proposition 3.5 (TeX label
  `prop:inseparable multisection`) with its proof, Theorem 3.6 (`thm:moving torsors general`)
  with its proof; Section 4 "Moduli spaces": Theorem 4.3 (`thm: bundle structure`) with its
  proof (Steps 1–6), Remark 4.4 (`rem: crystal geometry`); Section 5 "Supersingular K3 surfaces
  are unirational": Theorem 5.1 (`thm:correspondences`) and Theorem 5.3 (`thm:main`) with their
  proofs; Corollary 4.6 (`cor: ogus`) read for the dependency chain. Numbering: all theorem-like
  environments share one counter per section; Section 3 runs Definition 3.1, Proposition 3.2,
  Lemma 3.3, Proposition 3.4, Proposition 3.5, Theorem 3.6, …; Section 4 (from source line 1681)
  Definition 4.1, Definition 4.2, Theorem 4.3, Remark 4.4, Theorem 4.5, Corollary 4.6, Remark
  4.7; Section 5 (from line 2217) Theorem 5.1, Remark 5.2, Theorem 5.3, … — so the erratum's
  numbers are v4's.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v4 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` items 26–29); no CLAIM.

---

## S-0061 — Liedtke, arXiv:1304.5623 **v5** — the correcting version, its comment and its erratum appendix (the status document of Route A items 26, 27, 28 and 29)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0060; **v5 submitted 8 February 2023**, comment: "due to a mistake in
  Proposition 3.5, the unirationality of supersingular K3 surfaces remains a conjecture: see the
  appendix for erratum". The abstract page also lists the published erratum, Invent. Math. 223
  (2021), 1407–1409 (not retrieved).
- **Primary:** abstract page <https://arxiv.org/abs/1304.5623v5>, accessed 2026-10-14T21:27:57Z
  (HTTP 200), 39,394 bytes, SHA-256
  `97fabea89d23cfaa7c0f905734d60ab76dc2f730bde69061cfd6a2745c4dab4e` (re-fetched at about
  21:30Z to read the journal fields; byte-identical); e-print
  <https://arxiv.org/e-print/1304.5623v5> (`application/gzip`), 34,307 bytes, SHA-256
  `531f8badcfa87dca9fb3ae1023ae22760e2064dfad973feb92c920b66e4d80b4` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** the comment; the new footnote on the introduction ("Due to a mistake in
  Proposition 3.5, our results merely show that supersingular K3 surfaces are related by
  sequences of very special correspondences, which is not sufficient to deduce their
  unirationality. The unirationality conjecture thus remains a conjecture. See the appendix for
  erratum."); and the appendix "Erratum" (source lines 2396–2514), the whole of it, in
  particular: "As was pointed out by Bragg and Lieblich in [BL], the proof of Proposition 3.5
  contains a mistake, which has the effect that the main result, the unirationality of
  supersingular K3 surfaces Theorem 5.3, remains a conjecture."; under "Degree p multisections",
  "More precisely, the proof only shows the following:" followed by a weakened proposition whose
  conclusion is a degree-p multisection "finite and flat of degree p" that "arises from a family
  of A_ξ[p]-torsors", and "In particular, contrary to what was claimed in Proposition 3.5, one
  may not be able to choose the degree p multisection D_ξ to be radicial, that is, purely
  inseparable, over ξ ×_k S."; "Now, in order to show that unirationality is preserved in these
  one-dimensional families, one needs Assertion (4) of Theorem 3.6, whose proof requires that one
  can find degree p multisections in Proposition 3.5 that are radicial over the base. Without
  this, one can only conclude that any two supersingular K3 surfaces in characteristic p ≥ 5 are
  connected by a sequence of very special degree p correspondences (as in Assertion (3) of
  Theorem 3.6), which is not sufficient for unirationality."; under "Fibration structure",
  "Moreover, the proof of Theorem 4.3 contains a mistake. In fact, the proof only shows the
  following:" followed by a weakened theorem (a dominant rational map ϖ_N with a section, defined
  on the open set of crystals of Artin invariant σ_0+1, whose geometric generic fibre has
  normalization ℙ¹; "(In general, ϖ_N may not be generically smooth.)"), then "In the proof, the
  map is only defined on geometric points and extension to a morphism would require more
  arguments. In particular, the computations in Step 6 of the proof only show that the
  normalisation of the geometric generic fibre is isomorphic to ℙ¹." and "In particular, the
  table Remark 4.4 has to be corrected as follows: for σ_0(N) = 3, the moduli space 𝓜_N consists
  geometrically of two copies of the Fermat hypersurface of degree (p+1) in ℙ³. In this case, ϖ_N
  arises geometrically by choosing a pencil of planes through a line of this surface and then,
  one obtains a fibration over ℙ¹, whose geometric generic fibre is a singular rational curve
  that is not smooth. This gives an example, where the geometric generic fibre of ϖ_N is not
  smooth, and in particular, not isomorphic to ℙ¹." The v4 → v5 TeX diff (732 lines): the
  footnote, the appendix, a rearranged introduction (the ℙ¹-bundle theorem's "In particular,
  𝓜_N and 𝓜_{N_+} are iterated ℙ¹-bundles over Spec 𝔽_{p²}" dropped; a sentence on Lieblich's
  later proof moved), a shortened acknowledgement, and prose edits; the bodies of Sections 3–5
  are otherwise unchanged. The AI-assistance scanner reports no hit on either version.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v5 abstract page;
  copy not committed.
- **Bears on:** S1.1 items 26–29 (their status document); no CLAIM.

---

## S-0062 — Andrea Davini and Maxime Zavidovique, "On the (non) existence of viscosity solutions of multi-time Hamilton-Jacobi equations", arXiv:1312.1045 **v1** (the original of Route A item 30)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 26 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 20, 2026-10-14.
- **Citation (from the abstract page):** Andrea Davini, Maxime Zavidovique, "On the (non)
  existence of viscosity solutions of multi--time Hamilton--Jacobi equations", arXiv:1312.1045,
  Analysis of PDEs (math.AP; cross-listed math.SG); **v1 submitted 4 December 2013** (comment
  "15 pages"); v2 5 April 2023 (S-0063). Journal reference on the abstract page: J. Differential
  Equations 258 (2015), no. 2, 362–378, DOI 10.1016/j.jde.2014.09.015 (the published version was
  not retrieved). Two authors.
- **Primary:** abstract page <https://arxiv.org/abs/1312.1045v1>, accessed 2026-10-14T21:28:15Z
  (HTTP 200), 39,895 bytes, SHA-256
  `44a2f4614410b57ed78098d58a6ca429b721b768a2ced113d5f498ae1b6995dc`; e-print
  <https://arxiv.org/e-print/1312.1045v1> (`application/gzip`), 17,242 bytes, SHA-256
  `99711e20943672c1941370802d21f5537e37aef2773250780125e8ffd0a38483` — a gzipped tar holding
  `NonCommutingNov2013.tex` and `NonCommutingNov2013.bbl`.
- **Content used:** the Appendix: Theorem A.1 (TeX label `teo appendix`) with its proof (Perron's
  method), Proposition A.2 (`prop appendix comparison`, the comparison principle) and Remark A.3
  (`oss appendix comparison`), and the introduction's sentence "In the appendix we give the proof
  of a folklore theorem on existence and uniqueness of viscosity solutions". Numbering: the
  appendix is an unnumbered `\section*{Appendix}` whose environments carry the prefix A;
  Theorem A.1 is the first, Proposition A.2 the second, so the comment's numbers are v1's.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v1 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` item 30); no CLAIM.

---

## S-0063 — Davini and Zavidovique, arXiv:1312.1045 **v2** — the correcting version and its comment (the status document of Route A item 30)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0062; **v2 submitted 5 April 2023**, comment: "We have revised the Appendix
  by slightly changing the statement (and the proof) of Proposition A.2 (Comparison Principle).
  We needed Proposition A.2 in this modified form to fill a gap in the proof of Theorem A.1. We
  have also introduced some simplifications in the proof of Proposition A.2".
- **Primary:** abstract page <https://arxiv.org/abs/1312.1045v2>, accessed 2026-10-14T21:28:15Z
  (HTTP 200), 40,329 bytes, SHA-256
  `2a72ecf0abfb69b7c0db50160cb89adea2912684e88bd5ee13c3ea5d11da5821` (re-fetched at about
  21:30Z to read the journal fields; byte-identical); e-print
  <https://arxiv.org/e-print/1312.1045v2> (`application/gzip`), 17,971 bytes, SHA-256
  `3cf8c9755ab2c2f5aa0e18b41de206ba6960ee4eaa1efb8a3be2502e0a2b04b3` — a gzipped tar holding
  `NonCommuting_revised_Nov2022.tex` and its `.bbl`.
- **Content used:** the comment; the v1 → v2 TeX diff (467 lines): Theorem A.1's statement is
  unchanged word for word; Proposition A.2 now reads "Let v and u be, respectively, an upper and
  lower semicontinuous function on [0,+∞)×ℝⁿ satisfying condition (C) for a pair of constants
  α̂ > 0 and M_α̂ > 0. Let us assume that v is a subsolution and u is a supersolution of (A), and
  that either one of the following conditions hold: (a) for every α > 0, there exists M_α > 0 such
  that (C) holds with f := v; (b) v is continuous on [0,+∞)×ℝⁿ. Then sup(v − u) ≤
  sup_{ℝⁿ}(v(0,·) − u(0,·))", with condition (C) "f(t,x) ≥ f(0,x) − (α + M_α t) for all (t,x)",
  in place of v1's hypothesis that both (v_0) and (u_0) hold for every α > 0; its proof is
  reworked (a penalisation η/(T−t) on a finite time horizon in place of η(t²+s²), and a new case
  (b)); the proof of Theorem A.1 now states the bounds "u_0(x) − Mt ≤ u_*(t,x) ≤ u*(t,x) ≤
  u_0(x) + Mt on [0,+∞)×ℝⁿ" before invoking Proposition A.2 for u* ≤ u_*, and for the Lipschitz
  bound in time introduces u_h(t,x) := u(t+h,x), verifies "u_h satisfies condition (C) with
  α := 2Mh and M_α := M", and applies Proposition A.2 "to the continuous solutions u_h, u"; the
  equation (A)'s domain "(0,+∞)×(0,+∞)×ℝⁿ" is corrected to "(0,+∞)×ℝⁿ"; the semicontinuous
  envelopes are written with inf/sup over balls; the concluding speculation of Section 5 is
  rewritten. The AI-assistance scanner reports no hit on either version.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v2 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 30 (its status document); no CLAIM.

---

## S-0064 — Matthias Franz, "Big polygon spaces", arXiv:1403.4485 **v3** (the original of Route A item 31)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 27 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 20, 2026-10-14.
- **Citation (from the abstract page):** Matthias Franz, "Big polygon spaces", arXiv:1403.4485,
  Algebraic Topology (math.AT); v1 18 March 2014; v2 21 October 2014; **v3 submitted 7 April
  2015** (comment "20 pages; proof of Proposition 7.2 corrected, minor changes" — a 2015
  correction outside the ADR-0003 D8 window, not harvested); v4 12 June 2023 (S-0065). Journal
  reference on the abstract page: Int. Math. Res. Not. 2015 (2015), 13379–13405, DOI
  10.1093/imrn/rnv090 (the published version was not retrieved). One author.
- **Primary:** abstract page <https://arxiv.org/abs/1403.4485v3>, accessed 2026-10-14T21:28:34Z
  (HTTP 200), 41,241 bytes, SHA-256
  `46854f524747e82349ddbf30d4075d13d53482afe36e34ba4bc471a4c37e47f1`; e-print
  <https://arxiv.org/e-print/1403.4485v3> (`application/gzip`), 24,763 bytes, SHA-256
  `52d5f949d9ec84eedbaa3d6fc83484a4149332a83646efb88d7d69e8c370f851` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** Section 5 "The equilateral case": Proposition 5.1 (TeX label
  `thm:HTX-equilateral`, the section's first numbered environment; the counter is per section
  and shared) — its statement and proof, in particular the computation of ker ι^T_* and
  coker ι^T_* and their degree shifts; quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Bears on:** S1.1 (`groundtruth/` item 31); no CLAIM.

---

## S-0065 — Franz, arXiv:1403.4485 **v4** — the correcting version and its comment (the status document of Route A item 31)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0064; **v4 submitted 12 June 2023**, comment: "20 pages; incorrect degree
  shift in Proposition 5.1 corrected, some typos fixed".
- **Primary:** abstract page <https://arxiv.org/abs/1403.4485v4>, accessed 2026-10-14T21:28:34Z
  (HTTP 200), 41,177 bytes, SHA-256
  `476f7125e32b057f99cc10b6eb2cda0ec51d7d4c814f9d8c644f71fbe9453472` (re-fetched at about
  21:30Z to read the journal fields; byte-identical); e-print
  <https://arxiv.org/e-print/1403.4485v4> (`application/gzip`), 25,056 bytes, SHA-256
  `126db358c0ecd841159fd5e944866c891a7989dca2e7b59e7b4d88603021bfc5` — a single gzipped TeX
  file (`main.tex`).
- **Content used:** the comment; the v3 → v4 TeX diff (272 lines): the acknowledgement's new
  clause "and particularly Sergio Chaves for pointing out previously incorrect degrees shifts in
  Proposition 5.1"; in Proposition 5.1's statement the summand "𝒦_{m+2}[(m+1)d_V − d_U + 1]"
  becomes "𝒦_{m+2}[(m+2)d_V − 2d_U − 1]" (the other summands and the conclusion "syzord
  H_T^*(X_{a,b}(ℓ)) = m" unchanged); in its proof the degree bookkeeping is redone — the
  restriction of ι^T_* is "essentially the map δ̃_j …, up to a degree shift by |W_J| − |e_J| =
  −(j+1)d_U", with "s̄ = −(m+1)d_U and s = s̄ − 2b(m+1) = −(m+1)d_V" introduced, the kernel's
  Koszul summand becomes "𝒦_{m+2}[s̄ − 2b(m−1)] = 𝒦_{m+2}[−(m−1)d_V − 2d_U]" where v3 had
  "𝒦_{m+2}[−m d_V − d_U + 2]", and v3's sentence "The degree shifts follow from (eq:dim-VJ-WJ)
  and the convention that the Koszul syzygies are generated in degree 0" is dropped; in Lemma
  4.x's proof "up to a degree shift by −2b(r−k)" is replaced by explicit shifts; the remaining
  hunks are typographical (hyperref/breakurl preamble, a citation to Puppe 2018, index fixes).
  The AI-assistance scanner's one hit, "llm", is inside the bibliography entry "Stillman"
  (Grayson–Stillman, Macaulay2) — checked, not an AI-assistance mention.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v4 abstract page;
  copy not committed.
- **Bears on:** S1.1 item 31 (its status document); no CLAIM.


---

### S1.3 fresh retrieval addendum — 2026-10-20 UTC (S-0024/S-0025, S-0052/S-0053)

This adds a new reading; all historical access records above remain unchanged. The executor
retrieved and read both abstract pages and gzip sources for arXiv:0706.0357v70/v71 and
arXiv:1209.2022v2/v3 at 03:13:13–03:13:17Z. All eight responses were HTTP 200.
The complete URL/final-URL/time/size/SHA-256 ledger is `analysis/s1-3/retrieval.json`.
The downloaded bytes match the historical source digests. Restricted originals remain outside
the repository. The first source's notice and changed identity support the precise promotion
hold appended to its RECORD; no replacement status is inferred. The selected Galindo parent
has direct counterexample testimony in v3; its existing status is retained, with the
executor's interpretation still distinguished from the author's statement.

The selected original's complete decompressed source is 20,953 bytes, SHA-256
`25ba26029915d23c5591361c337c20d4b5fc667c6dd704af08933a490b0ecc4f`.
Its proof slices are UTF-8; the bibliography contains legacy bytes. It is preserved as opaque
external context rather than decoded with replacement. The selected byte ranges and hashes
are recorded in the recipe; no corrected proof is substituted.

The arXiv non-exclusive licence text was freshly read at
<https://arxiv.org/licenses/nonexclusive-distrib/1.0/license.html>, 03:25:46Z, 1,415 bytes,
SHA-256 `1991765e2079370736d4dedf217375799ad19aab6700ebdf42ae3dd84690067a`.
It grants distribution to arXiv; it supplies no additional repository redistribution grant.
The existing external-only decision is retained for source and canonical child/context.


---

## S-0066 — flate2 1.1.10 and the locked Rust decompression dependency closure

- **State:** VERIFIED (package metadata and relevant licence texts retrieved/read 2026-10-20).
- **Primary:** <https://crates.io/api/v1/crates/flate2/1.1.10>, accessed 03:25:45Z;
  2,328 bytes, SHA-256 `6a50d4e078faf0d553c693acfba054682c86ed3d9b6c11c0cbbc0733971b4839`.
  Upstream <https://github.com/rust-lang/flate2-rs>; versioned API documentation
  <https://docs.rs/flate2/1.1.10/flate2/bufread/struct.GzDecoder.html>.
- **Content used:** one-member gzip decoding with the buffered decoder, which exposes unconsumed
  input so trailing data is refused. Default Rust backend, no C library selection. The new
  producer pins `=1.1.10`; the existing Rust 1.91.0 satisfies its declared minimum 1.67.0.
- **Locked closure / licences:** flate2 1.1.10 and crc32fast 1.5.2 (MIT OR Apache-2.0),
  miniz_oxide 0.9.1 (MIT OR Zlib OR Apache-2.0), adler2 2.0.1 (0BSD OR MIT OR Apache-2.0),
  simd-adler32 0.3.10 (MIT), zlib-rs 0.6.8 (Zlib; locked optional backend, not selected by
  this producer's default features). All are compatible with repository software distribution;
  their notices remain in their upstream packages. Package manifests are primary records;
  checksums, declared licences, repositories and manifest hashes are in
  `analysis/s1-3/dependencies.json`. The selected Rust backend compiles flate2, crc32fast,
  miniz_oxide, adler2 and simd-adler32. Cargo may download the locked optional package as well.
- **Pin review:** all 26 baseline Cargo.lock packages are identical, including their dependency
  arrays and checksums. Six registry packages plus the new workspace crate produce 33 entries.
  No Lean, Mathlib, existing Rust dependency, compiler or manifest-schema pin changes.
- **Bears on:** ADR-0006 / S1.3 implementation only; no CLAIM or mathematical status.

---

## S-0067 — Judson, AATA Annual Edition 2025, Euclid's infinitude of primes

- **State:** VERIFIED (primary publication retrieved and selected passage/definitions/rights
  read 2026-10-20 UTC; external bytes; S1.2 development candidate only, not corpus admission).
- **Author/title/edition:** Thomas W. Judson, *Abstract Algebra: Theory and Applications*,
  Annual Edition 2025, dated August 1, 2025. The title page separately credits Robert A.
  Beezer for Sage exercises and Antonio Behn for Spanish translation. The copyright page
  names Thomas W. Judson and Robert A. Beezer, 1997–2025.
- **Primary:** <https://judsonbooks.org/aata-files/aata-20250801.pdf>, HTTP 200,
  accessed **2026-10-20T04:50:55.721358Z**, 2,207,438 bytes, SHA-256
  `4f74587b61861b4790d613f2adb64699f90fd1d07760204f1d8fc9fcaf63d14f`.
  The author's [book page](https://judsonbooks.org/abstract-algebra-theory-and-applications/)
  links this fixed edition; its dynamic discovery-page bytes are separately recorded.
- **Exact locator/content used:** Theorem **2.2.6 (Euclid)**, subsection **2.2.2 Prime Numbers**,
  printed **p.28**, physical PDF **page 41** (one-based). The statement says:
  "There exist an infinite number of primes." The same-page definition restricts primes to
  integers greater than one whose positive divisors are one and themselves. The printed
  proof is present and was read for context; it is not the offered argument selected for
  this pair. The candidate selects the statement, with zero informal proof steps, and offers
  the Mathlib proof identified by S-0068. Executor transcription/pairing is separately credited.
- **Licence actually read:** copyright page (PDF page 3) grants GFDL version 1.2 or later,
  with no invariant sections and no front/back cover texts. Appendix A, printed pp.461–467,
  reproduces GFDL 1.3, including copying/modification conditions. The grant is not Apache-2.0.
  Complete source and canonical informal bytes remain external by choice; this record does
  not claim the licence prohibits redistribution. Public metadata/review contains a short
  attributed mathematical quotation, not the book's proof. A separate request to distribute
  a modified extract must satisfy that licence and preserve attribution.
- **Evidence:** `analysis/s1-2/retrieval.json`, candidate correspondence and external byte
  locations in `analysis/s1-2/README.md`. The page and copyright notice were rendered and
  visually read, alongside `pdftotext -layout`. Earlier inaccessible old-host URLs and an
  optional GNU-host licence fetch are retrieval-route failures, not a failed candidate;
  the primary book itself supplies the grant and complete licence.
- **Bears on:** ADR-0007 and prospective S1.2 pairing only; development-exposed; no CLAIM.

## S-0068 — Pinned Mathlib prime-infinitude declaration and source licence

- **State:** VERIFIED (pinned primary blobs freshly retrieved/read 2026-10-20 UTC;
  focused Lean inspection executed; no corpus admission or PROVEN promotion).
- **Primary declaration:** `Nat.infinite_setOfPred_prime` in
  `Mathlib/Data/Nat/PrimeFin.lean`, lines 28–29 at commit
  `d13f23b723b8a846827a245b89c10fc7d3f11612`.
  [Immutable source](https://github.com/leanprover-community/mathlib4/blob/d13f23b723b8a846827a245b89c10fc7d3f11612/Mathlib/Data/Nat/PrimeFin.lean#L28).
  Exact raw URL, access time and companion-source digests: `analysis/s1-2/retrieval.json`.
  Module fetched HTTP 200 at **2026-10-20T04:50:56.017343Z**, 5,328 bytes, SHA-256
  `92a5630a19bff74ade4088f1253cf75b0ac8143c002e6b604ac0b77a39c9ecf6`.
- **Content used:** the exact set-infinitude theorem, `Nat.Prime` / `Nat.prime_def`,
  the unbounded-primes proof in `Mathlib/Data/Nat/Prime/Infinite.lean`,
  `Set.Finite` / `Set.Infinite` and `Set.infinite_of_not_bddAbove`.
  The full elaborated type has no free theorem parameters, implicit binders or typeclass
  assumptions. The selected import is `Mathlib.Data.Nat.PrimeFin`; the existing alias
  `Nat.infinite_setOf_prime` is deprecated and is not selected.
- **Audit:** `lake env lean <CandidateAudit.lean>` under the pinned project exited 0.
  The selected theorem, SYNDEOM wrapper and principal infinitude dependencies use only
  `propext`, `Classical.choice`, `Quot.sound`; `Nat.prime_def` uses `propext`.
  `pp.all` prints universes on two axiom names; these are the same standard constants.
  Full type/dependency output and separate receipts are in `analysis/s1-2/`.
- **Licence actually read:** pinned `LICENSE`, freshly retrieved at
  **2026-10-20T04:50:56.859508Z**, 11,357 bytes, SHA-256
  `b40930bbcf80744c86c46a12bc9da056641d722716c378f5659b9e555ef833e1`,
  Apache-2.0; headers credit copyright 2015 Microsoft Corporation and authors Leonardo
  de Moura, Jeremy Avigad, Mario Carneiro. Source and notices remain external with the
  pinned dependency; the small SYNDEOM alias references the existing declaration.
  S-0008 remains the historical toolchain-licence record.
- **Byte identity:** downloaded raw bytes match the pinned Git blobs. This Windows
  dependency checkout has CRLF working files; their distinct hashes and exact LF/CRLF
  relation are recorded, not substituted for the source-object digests.
- **Bears on:** ADR-0007 / S1.2 source correspondence and candidate feasibility only;
  this does not certify every statement/proof in Mathlib or the Judson book.


### Locator correction for S-0068 — 2026-10-20, after merged PR 3

The historical S-0068 entry above says lines 28–29 and links to `#L28`.
The exact pinned 5,328-byte Git blob places the declaration at **line 27** and its proof
at **line 28**; line 29 is blank. Correct interval: **27–28, one-based inclusive**.
[Correct declaration anchor](https://github.com/leanprover-community/mathlib4/blob/d13f23b723b8a846827a245b89c10fc7d3f11612/Mathlib/Data/Nat/PrimeFin.lean#L27).
The source hash, declaration/type/argument, licence and VERIFIED source status are unchanged.
`analysis/s1-2/correction/` records Rust-computed corrected config identities, a fresh
successful rehearsal and negative controls that reject the old interval even with valid
source bytes. The original entry and all original receipts remain historical evidence.


### S-0067 / S-0068 implementation and proposed-admission follow-up — 2026-10-20

The source statuses above remain unchanged. After the S-0068 locator correction at
`d88fad2bb7b339c56bd9964167d3e0acddc8ee4e`, the first narrow pair producer was committed at
`eb1b96d5b8084ceb1de734fffc90c9b480ec32bf` and all five fixed objects were freshly retrieved
and hash/length verified. Source lines remain 27–28 inclusive; canonical declaration line 3.
The claim, raw source objects, source/alias attribution and canonical bytes are unchanged.
Production receipt: `analysis/s1-2/implementation/evidence/production/verification-receipt.json`.

Actual proposed archive `s1_pair_archive-v0.1.0-094625fa10ff` and child `s1_pair-v0.1.0-62fe0c816a6b` have
both-file validation, stronger parent/child checks, emitted-file full-type/axiom audit and
11-file byte-identical replay evidence. The new RECORDs preserve the independent publication,
Mathlib offered argument and SYNDEOM transcription roles. Zero informal proof steps;
empty-statistics fidelity PASS is vacuous. Full raw/licensing context and informal output
remain external. Draft admission review is pending; no source label or CLAIM is promoted.


### S-0067 / S-0068 post-merge disposition — 2026-10-20 UTC

The owner merged the pair implementation/admission packet in PR 4 at 18:10:13Z,
main `e6a7cae4392dd1565ee57f9fb727cc01e453687d`, with the same tree as PR head
`0b30d782163ed961262dbf70328dc1d16cf58746`. The archived and canonical pair RECORDs
append that disposition. The earlier proposed/pending text remains historical evidence.
S-0067 and S-0068 retain their recorded VERIFIED states, attribution, licence and external
storage decisions. No source status, historical locator correction or earlier receipt is
rewritten; no fresh primary retrieval or independent reviewer is claimed here.

The S1.4 compatibility check uses retained exact source bytes, a fresh pair build and new
archive/canonical output destinations. All 11 deterministic outputs match the originals;
this is byte/provenance replay, not a newly executed emitted-file Lean audit. The source
correspondence and earlier audit remain the scientific evidence. The pair is one logical
positive item with zero informal proof steps, not an independent population sample or
a PROVEN promotion. `analysis/s1-4/` records the scope and remaining obligations.


### S-0024 / S-0025 status-support review — 2026-10-20 UTC

The original source entries and storage decisions stand. Fresh archives match the
retained S1.3 bytes: v70 8,236 bytes / SHA-256
`5987d8106ae69669b10c876a0915d5ad52ce44a814ea7fe63600c1737642cca8`;
v71 8,750 bytes / SHA-256
`fffd0fdc3a868deb642102a1f72f23cd92ee89bf118c92a4b992cb5a6fdfd263`.
Both version pages still identify CC BY 4.0. v70 remains committed; v71 remains
external/digest-only. The fresh dynamic abstract pages have new hashes, retained
separately from the older HTML receipts. Actual URLs, access times, lengths/hashes,
archive equality and exact TeX member locators are in
`analysis/s1-first-parent-status/evidence/retrieval.json`, `correspondence.json` and
`WITNESS.md`. New external retrieval directory:
`C:\Users\LEET\Codex\Temp\syndeom-first-parent-20261020\retrieval-01`.

Codex (GPT-6), this review session, read the original definitions, Lemma 1.1 and its
proof, and the corresponding v71 correction. The author notice says only “To fix an
error in Lemma 1.1.” The finite 1-versus-0 witness and analytic log(2)/3 residual are
separately attributed executor reasoning; neither is retroactively author testimony.
No source status changes and no CLAIM promotion. The old parent remains held while
the exact case-specific ruling and validated replacement metadata are proposed.

## S-0069 — NIST DLMF §27.8, Dirichlet characters

- **State:** VERIFIED (primary page freshly retrieved and relevant definitions/equations
  read this session, 2026-10-20 UTC).
- **Primary:** NIST Digital Library of Mathematical Functions,
  [§27.8](https://dlmf.nist.gov/27.8), E1–E6: character normalization, periodicity,
  vanishing on nonunits, principal character and coprimality in orthogonality.
- **Access:** 2026-10-20T21:36:43.746745Z; HTTP 200; 68,162 bytes; SHA-256
  `34f7100585b68af0dbc868e51038f22c4c91c6efb482bf1481f7cbc9ce3d54e5`.
- **Rights/storage:** no redistribution permission asserted here; digest-only in the
  repository, complete HTML retained externally in the directory above. No full source
  copy ingested. Supports standard identities, not author testimony about the v70 lemma.

## S-0070 — NIST DLMF §25.15, Dirichlet L-functions

- **State:** VERIFIED (primary page freshly retrieved and relevant identities/domain
  read this session, 2026-10-20 UTC).
- **Primary:** [§25.15](https://dlmf.nist.gov/25.15), E1/E2 and nonvanishing discussion:
  L-series and Euler product for Re(s)>1. The witness also gives direct nonvanishing
  at s=2 from the positive odd-integer series.
- **Access:** 2026-10-20T21:36:44.495977Z; HTTP 200; 100,871 bytes; SHA-256
  `ca746b3571f06cdd2a53530059cdfc6d1c12cc66b7457ec30023212e0b90a54f`.
- **Rights/storage:** no redistribution permission asserted; external HTML, digest-only
  in the repository. Standard analytic support, not a new ground-truth item.

## S-0071 — NIST DLMF §27.4, multiplicative functions and Dirichlet series

- **State:** VERIFIED (primary page freshly retrieved and relevant equation/domain
  read this session, 2026-10-20 UTC).
- **Primary:** [§27.4 E12](https://dlmf.nist.gov/27.4#E12), the Mangoldt series equals
  -zeta'/zeta for Re(s)>1; used in the second derivation of the residual.
- **Access:** 2026-10-20T21:36:45.205913Z; HTTP 200; 86,532 bytes; SHA-256
  `9765f56ffb3b2b79125b516780c55654243230fe81296b348d1427afdb86c208`.
- **Rights/storage:** no redistribution permission asserted; external HTML, digest-only
  in the repository. The executor's application is attributed in WITNESS.md.

## S-0072 — NIST DLMF §27.2, Mangoldt function

- **State:** VERIFIED (primary page freshly retrieved and definition read this session,
  2026-10-20 UTC).
- **Primary:** [§27.2 E14](https://dlmf.nist.gov/27.2#E14), Mangoldt's prime-power
  definition, including zero otherwise. Used to identify the even support as powers
  2^ell with ell>=1 and to bound the convergent series.
- **Access:** 2026-10-20T21:36:45.889969Z; HTTP 200; 160,061 bytes; SHA-256
  `6256c868078ac904ff6e0e55027c84d8aad63d438fd339957689980e26b6d510`.
- **Rights/storage:** no redistribution permission asserted; external HTML, digest-only
  in the repository. No source text is newly ingested into the live corpus.

## S-0073 — PolyForm Noncommercial License 1.0.0 — the repository licence text (ADR-0010; Al's decision of 2026-10-26)

- **State:** INGESTED (the text is committed verbatim in `LICENSE` at the repo root, after one line
  `Required Notice: Copyright 2026 Alexander David Zalewski (https://github.com/Quigles1337/Syndeom)`
  and one blank line). It applies to SYNDEOM's original software and technical documentation
  under the scope ADR-0010 states.
- **As named:** Al's decision of 2026-10-26 (ADR-0010), with the source URL and the SHA-256 the
  text must have. Al: stop if the digest differs.
- **Source:**
  `https://raw.githubusercontent.com/polyformproject/polyform-licenses/1.0.0/PolyForm-Noncommercial-1.0.0.md`
  (the PolyForm Project's repository at tag `1.0.0`; the text names its canonical URL
  `https://polyformproject.org/licenses/noncommercial/1.0.0`).
- **Access:** 2026-10-27T00:48:02Z, HTTP 200, 4,563 bytes, LF, SHA-256
  `c0ea4a896d2c8c394b29f9427589996db826cd501c512279ff0ed3ef48fabbe5`, equal to the digest Al
  stated. A pre-check at 2026-10-27T00:34:21Z returned the same bytes. In `LICENSE` (4,662
  bytes), the bytes after the notice line and the blank line re-hash to the same digest.
- **Terms of the licence text itself:** the same repository's `README.md` at tag `1.0.0`
  (retrieved 2026-10-27T00:50:21Z, HTTP 200, 2,639 bytes, SHA-256
  `bb69919c361a2666d23b9c816e6ecfe3bb193fc0138abf4c76f8413b34d74fca`), section "License":
  "Each contributor licenses you to do everything with PolyForm licenses that would otherwise
  infringe that contributor's copyright in it. If you make changes to a PolyForm license, you
  must remove all mention of "PolyForm" and polyformproject.org, as well." The text is used
  unchanged.
- **SPDX:** `PolyForm-Noncommercial-1.0.0`. The SPDX License List entry
  `https://spdx.org/licenses/PolyForm-Noncommercial-1.0.0.json` (HTTP 200 at
  2026-10-27T00:50:21Z) gives that `licenseId` and the name "PolyForm Noncommercial License
  1.0.0". The four crates' `license` fields use this identifier.
- **Relation to S-0019:** S-0019's Apache-2.0 text was `LICENSE` from Session 7 through `main`
  `c9372b1355a6d687dec6287f908639ac670db659`. This entry records its successor from the commit
  that accepts ADR-0010. S-0019 is unchanged: the register is append-only under ADR-0010 D6, and
  copies obtained under Apache-2.0 remain under it (ADR-0010 D3).

### S-0019 addendum — 2026-10-27 UTC (Session 31): the same text, re-fetched, committed as `LICENSES/Apache-2.0.txt`

- **As named:** Al's ruling 3 of 2026-10-27 (ADR-0010 *Amendments*): add the Apache-2.0 text
  verbatim as `LICENSES/Apache-2.0.txt`, fetched from an official source, with its checksum
  verified against this entry. `THIRD-PARTY-NOTICES.md` lists the files it applies to.
- **Canonical ASF host — still unreachable:** curl to `https://www.apache.org/licenses/LICENSE-2.0.txt`
  at 2026-10-27T02:26:03Z: `Failed to connect to www.apache.org port 443 after 21132 ms: Could
  not connect to server`; to `https://apache.org/licenses/LICENSE-2.0.txt` at 02:26:34Z: `Failed
  to connect to apache.org port 443 after 21098 ms: Could not connect to server`. The diff note
  above is carried unchanged.
- **Retrieved — the SPDX License List's text:** `https://spdx.org/licenses/Apache-2.0.txt`,
  accessed 2026-10-27T02:26:03Z (curl, HTTP 200, `Content-Type: text/plain`, `Last-Modified: Wed,
  29 Apr 2020 17:19:07 GMT`, ETag `"ba963850f6731c74878fe839d227e675"`, both as recorded above):
  10,279 bytes, 0 CR, no trailing newline, 72 lines, SHA-256
  `c274f80372d90c012937370f0e1f15087d22e308ef98b27cea5dc0d2d088366c`, **equal to the digest
  recorded above.**
- **Committed form:** the retrieved bytes plus one trailing LF, as above: 10,280 bytes, SHA-256
  `074e6e32c86a4c0ef8b3ed25b721ca23aca83df277cd88106ef7177c354615ff`. Compared with `cmp`, it is
  byte-identical to `LICENSE` at `c9372b1355a6d687dec6287f908639ac670db659`.
- **Licence (of this material):** the Apache License 2.0 is itself the licence. The text is
  included to apply it to the Apache-2.0 files that `THIRD-PARTY-NOTICES.md` lists.
- **Numbering:** this addendum takes no new S-number, so the held branch's planned renumbering
  (STATE) does not move. The entry's recorded state is unchanged.
- **Evidence:** `C:\Users\LEET\Codex\Temp\syndeom-registration-licence-20261027\retrieval\`.

---

## S-0074 — Jovana Đuretić, Jelena Katić and Darko Milinković, "Comparison of spectral invariants in Lagrangian and Hamiltonian Floer theory", arXiv:1403.6317 **v1** (the original of Route A item 32)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 28 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 22, 2026-10-21.
- **Citation (from the abstract page):** Jovana Đuretić, Jelena Katić, Darko Milinković,
  "Comparison of spectral invariants in Lagrangian and Hamiltonian Floer theory", arXiv:1403.6317,
  Symplectic Geometry (math.SG); **v1 submitted 25 March 2014** (124 KB; no comment); v2 28 March
  2014 (1 KB, withdrawn — S-0075); v3 15 October 2024 (S-0075). No journal reference or DOI on the
  abstract page (the v3 source file is named `DKM_filomat_v3.tex`). Three authors.
- **Primary:** abstract page <https://arxiv.org/abs/1403.6317v1>, accessed 2026-10-21T21:46:38Z
  (HTTP 200), 39,781 bytes, SHA-256
  `1e5d5c9673d90d6a20c07fa2481bd50349ad6d2f1bad07a0c290e1c8e13628fd` (re-fetched at 21:47:21Z
  and 21:51:53Z, byte-identical); e-print <https://arxiv.org/e-print/1403.6317v1>
  (`application/gzip`), accessed in the examination run of 21:47:21Z, 126,877 bytes, SHA-256
  `97b28cf24aa4cc4bb6e810ceb12aebad481df0840cb5a3389b5eca1c2e599bce` — a tar bundle whose TeX
  file is `Djuretic_Katic_Milinkovic.tex` (851 lines).
- **Content used:** the introduction's Theorem 1 (TeX label `thm:HD`; every theorem-like
  environment of the paper is numbered on one shared counter, so Theorem 1 is the first), its
  hypotheses and conclusion; Subsection 2.4 "Hofer's distance" (Definitions 5 and 6 —
  quasi-autonomous, admissible — Schwarz's theorem cited as Theorem 7, the Lagrangian Hofer
  distance Δ); Section 3 "Proof of Theorem 1" (the chain (i)–(vii)); Theorems 10 and 11 and
  Corollary 12; quoted in `RECORD.md`.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v1 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Integrated:** Session 38 (2026-10-28), carried from the held branch at `c3a3d36`, where
  this entry was S-0066 (`analysis/s1-held-harvest-review/REVIEW.md` §4 step 2; Al's
  ruling Q-H6 of 2026-10-28). "This session" above is Session 22 (2026-10-21): the retrieval
  dates, access times and digests are Session 22's, and nothing was re-fetched.
- **Bears on:** S1.1 (`groundtruth/` item 32); no CLAIM.

---

## S-0075 — Đuretić, Katić and Milinković, arXiv:1403.6317 **v3** — the correcting version and its comment, with the v2 withdrawal comment (the status document of Route A item 32)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed).
- **Citation:** as S-0074; **v2 submitted 28 March 2014** (1 KB, withdrawn), comment: "This paper
  has been withdrawn by the authors due to an error in the assumptions of the Theorem 1"; **v3
  submitted 15 October 2024**, comment: "We removed the wrong theorem from the first version,
  which was brought to our attention by R. Leclercq".
- **Primary:** abstract page <https://arxiv.org/abs/1403.6317v3>, accessed 2026-10-21T21:46:38Z
  (HTTP 200), 38,733 bytes, SHA-256
  `5b32eedd88fe1b0a4972c956bbc5742f210c61e84a066065b387301eaa6da531` (re-fetched at 21:47:21Z
  and 21:51:56Z, byte-identical); abstract page of the withdrawn v2
  <https://arxiv.org/abs/1403.6317v2>, accessed 2026-10-21T21:47:40Z (HTTP 200), 39,708 bytes,
  SHA-256 `81d0d559b9f39506e35193889d29923874cc69ff6fa309d00128d43a66f8f907` (a withdrawal stub:
  no e-print, no licence link); e-print <https://arxiv.org/e-print/1403.6317v3>
  (`application/gzip`), accessed in the examination run of 21:47:21Z, 125,994 bytes, SHA-256
  `3e95b88f70673c6d42813c7e9f43a62a272060078a0ef01b726e06df15c1f2ed` — a tar bundle whose TeX
  file is `DKM_filomat_v3.tex` (798 lines).
- **Content used:** the two comments; the v1 → v3 TeX diff (645 lines): Theorem 1 (`thm:HD`), the
  sentence announcing its proof, Section 3 "Proof of Theorem 1" with its seven-step chain,
  Subsection 2.4 "Hofer's distance" (the definitions of quasi-autonomous and admissible
  Hamiltonians, Schwarz's theorem, the distance Δ) and Corollary 12 are removed; the abstract's
  sentence "From this result, we derive a corollary considering comparison of Hofer's distance in
  periodic orbits and Lagrangian case" and the keyword "Hofer's distance" are dropped; Theorem 3
  (the Morse-homology product inequality) becomes a remark with its derivation; the invariants are
  reduced from four (ρ, η for periodic orbits; c, σ for the Lagrangian case) to two (ρ, σ)
  normalised through the PSS isomorphisms, with a new theorem that σ(·, H) − σ(·, K) is a
  constant C(H, K) whenever φ¹_H(L) = φ¹_K(L) replacing v1's sentence "In [L], Leclercq showed
  that c(β) does not depend on H, but only on L and φ¹_H(L)"; the introduction adds "We would
  like to thank Rémi Leclerq for pointing to us an error in the previous version of the paper."
  and a referee acknowledgement. The AI-assistance scan of the v3 sources: no hit.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v3 abstract page;
  copy not committed.
- **Integrated:** Session 38 (2026-10-28), carried from the held branch at `c3a3d36`, where
  this entry was S-0067 (`analysis/s1-held-harvest-review/REVIEW.md` §4 step 2; Al's
  ruling Q-H6 of 2026-10-28). "This session" above is Session 22 (2026-10-21): the retrieval
  dates, access times and digests are Session 22's, and nothing was re-fetched.
- **Bears on:** S1.1 item 32 (its status document); no CLAIM.

---

## S-0076 — Yasuo Nishii, "Algorithm, probability, and prime numbers", arXiv:1403.8075 **v9** (the original of Route A items 33–35)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The items' `raw/1` file records are `external`.
- **How found:** Route A, candidate 29 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 22, 2026-10-21.
- **Citation (from the abstract page):** Yasuo Nishii, "Algorithm, probability, and prime
  numbers", arXiv:1403.8075, General Mathematics (math.GM); v1 23 March 2014; v2–v5 April–May
  2014; v6 20 December 2016; v7 30 September 2018; v8 8 December 2020; **v9 submitted 23 March
  2023** (134 KB; comment "Revision of the contents over the whole range of the paper" — a
  revision inside the window that reports no error and is not a notice); v10 27 December 2025
  and v11 9 January 2026, both withdrawn (S-0077). No journal reference or DOI on the abstract
  page. One author.
- **Primary:** abstract page <https://arxiv.org/abs/1403.8075v9>, accessed 2026-10-21T21:48:05Z
  (HTTP 200), 41,098 bytes, SHA-256
  `1cd0cb15c6e65b322aed751bda1ee6bb96e80628085a3b2d922e327be1635d77` (re-fetched at about
  21:52:03Z, byte-identical); e-print <https://arxiv.org/e-print/1403.8075v9>
  (`application/gzip`), accessed 2026-10-21T21:52:06Z, 136,883 bytes, SHA-256
  `c171e3381684cb051dcf77333de7e51d75e08ca92580b9dc14d24d73e6999270` — a tar bundle
  (`rev_math1.tex`, 595 lines; `rev_math1.bbl`; `rev_math1.synctex`).
- **Content used:** Section 2 (Definition 2.7, the "property of an infinite number of steps";
  Remark 2.4); Section 3's standing independence assumption and Theorem 3.4; Section 4
  "Distribution of prime numbers": Proposition 4.1 and its proof, the paragraph following it (the
  independence claim and its footnote), Lemma 4.2, Theorems 4.3 and 4.4 and their proofs (the
  counter is per section and shared, so these are the first to fourth numbered environments of
  Section 4); quoted in the three `RECORD.md` files. The AI-assistance scan of the v9 sources: no
  hit.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v9 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Integrated:** Session 38 (2026-10-28), carried from the held branch at `c3a3d36`, where
  this entry was S-0068 (`analysis/s1-held-harvest-review/REVIEW.md` §4 step 2; Al's
  ruling Q-H6 of 2026-10-28). "This session" above is Session 22 (2026-10-21): the retrieval
  dates, access times and digests are Session 22's, and nothing was re-fetched.
- **Bears on:** S1.1 (`groundtruth/` items 33, 34 and 35); no CLAIM.

---

## S-0077 — Nishii, arXiv:1403.8075 **v10** and **v11** — the withdrawal comments (the status document of Route A items 33–35)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed; the
  withdrawn versions have no e-print).
- **Citation:** as S-0076; **v10 submitted 27 December 2025** (1 KB, withdrawn), comment:
  "Withdrawn: Section 4 contains a critical logical error. In Proposition 4.1 the argument
  incorrectly treats primality verification as equivalent to checking leading digits in multiple
  base representations, and the subsequent claim that fp(1),...,fp(n) are independent is
  unjustified. These invalidate the probabilistic step leading to Theorems 4.3 -- 4.4"; **v11
  submitted 9 January 2026** (1 KB, withdrawn), comment: "Withdrawn due to a logical error in
  Section 4. In Proposition 4.1, the argument incorrectly treats primality verification as
  equivalent to checking leading digits in multiple base representations, and the subsequent
  independence assumption for fp(1),...,fp(n) is unjustified. This invalidates the derivation of
  Theorems 4.3-4.4".
- **Primary:** abstract page <https://arxiv.org/abs/1403.8075v10>, accessed
  2026-10-21T21:52:00Z (HTTP 200), 39,877 bytes, SHA-256
  `b01ae085d31ce38869d53d8f6fab91ff5878b8f7865c6960f9af87148478d42f`; abstract page
  <https://arxiv.org/abs/1403.8075v11>, accessed 2026-10-21T21:48:05Z (HTTP 200), 40,895 bytes,
  SHA-256 `5726f969962deed8b5d8f840cda7fe80e497486521c1b698b93c64cb874f397b`. Neither stub page
  carries a licence link; the comments are the whole document.
- **Content used:** the two comments, read against the v9 text (S-0076): the equivalence
  sentence in the proof of Proposition 4.1, the independence sentence of the paragraph following
  it, and the applications of Theorem 3.4 in the proofs of Theorems 4.3 and 4.4.
- **Licence:** no licence link on the withdrawal stubs; nothing copied beyond the comments quoted
  above.
- **Integrated:** Session 38 (2026-10-28), carried from the held branch at `c3a3d36`, where
  this entry was S-0069 (`analysis/s1-held-harvest-review/REVIEW.md` §4 step 2; Al's
  ruling Q-H6 of 2026-10-28). "This session" above is Session 22 (2026-10-21): the retrieval
  dates, access times and digests are Session 22's, and nothing was re-fetched.
- **Bears on:** S1.1 items 33, 34 and 35 (their status document); no CLAIM.

---

## S-0078 — Martin Otto, "Finite Groupoids, Finite Coverings and Symmetries in Finite Structures", arXiv:1404.4599 **v4** (the original of Route A item 36)

- **State:** VERIFIED (retrieved and read this session). **Digest-only; no copy committed**
  (licence below). The item's `raw/1` file record is `external`.
- **How found:** Route A, candidate 30 of the finder's ascending-id stream (HARVEST-LOG run 1);
  examined Session 22, 2026-10-21.
- **Citation (from the abstract page):** Martin Otto, "Finite Groupoids, Finite Coverings and
  Symmetries in Finite Structures", arXiv:1404.4599, Combinatorics (math.CO), cross-listed Logic
  in Computer Science (cs.LO) and Logic (math.LO); MSC 05E18, 05C65, 05C25, 05C38, 20F05, 57M12;
  v1 17 April 2014; v2 14 July 2014; v3 24 March 2015; **v4 submitted 15 October 2015** (47 KB),
  comment: "This paper extends and supersedes earlier expositions in LICS 2013 and
  arXiv:1211.5656. Version (v2) of this paper fixes a false claim in Lemma 2.9 of the original
  version. Version (v4) eliminates confusion around "covering of A" vs "realisation of H(A)"
  (Definition 3.14 and adaptation of Lemma 3.16) and corrects a mistake in the "excursion" on
  Herwig's thm in section 4.4" — the v2 (2014) and v4 (2015) corrections it reports are outside
  the ADR-0003 D8 window and not harvested; v5 12 January 2024, withdrawn (S-0079). No journal
  reference or DOI on the abstract page. One author.
- **Primary:** abstract page <https://arxiv.org/abs/1404.4599v4>, accessed 2026-10-21T21:52:08Z
  (HTTP 200), 41,567 bytes, SHA-256
  `84cb82628e4efab4b00b65555e29be87738382e6117c24d317903f40afda559d` (re-fetched at 22:02:25Z to
  read the journal fields, byte-identical); e-print <https://arxiv.org/e-print/1404.4599v4>
  (`application/gzip`), accessed in the examination run of 21:52:08Z, 48,382 bytes, SHA-256
  `2e326a3380ac4068d70de4685e3b306eb693677716fa6c920d8deb747f923cd2` — a single gzipped TeX file
  (`main.tex`, 4,733 lines).
- **Content used:** Section 2 "Highly acyclic finite groupoids" (TeX label `groupoidsec`): Lemma
  2.9 (`completionandsubgroupslem`, completion and α-reducts); Section 2.4 "Eliminating short
  coset cycles" — Definitions 2.17–2.19, Remark 2.20, Lemma 2.21 (`amalgamationlem`) with its
  proof, Proposition 2.22 (`Nacycgroupoidprop`) with its proof, Observation 2.23 (the counter
  `theo` is per section and shared by every environment, so the numbers were confirmed by
  counting the environments of Section 2); quoted in `RECORD.md`. The AI-assistance scan of the v4
  source: no hit.
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the v4 abstract page
  (ADR-0003 D5: `external`): **no copy committed**; the hashes above pin what was read.
- **Integrated:** Session 38 (2026-10-28), carried from the held branch at `c3a3d36`, where
  this entry was S-0070 (`analysis/s1-held-harvest-review/REVIEW.md` §4 step 2; Al's
  ruling Q-H6 of 2026-10-28). "This session" above is Session 22 (2026-10-21): the retrieval
  dates, access times and digests are Session 22's, and nothing was re-fetched.
- **Bears on:** S1.1 (`groundtruth/` item 36); no CLAIM.

---

## S-0079 — Otto, arXiv:1404.4599 **v5** — the withdrawal comment (the status document of Route A item 36)

- **State:** VERIFIED (retrieved and read this session; digest-only, no copy committed; the
  withdrawn version has no e-print).
- **Citation:** as S-0078; **v5 submitted 12 January 2024** (1 KB, withdrawn), comment: "The
  construction of finite n-acyclic groupoids in Section 2.4 is flawed and I know of no direct
  repair: completion turns out to be incompatible with restriction to proper subsets of the
  generator set, so that the induction towards Proposition 2.22 does not stabilise as claimed.
  This problem has been overcome in arXiv:1806.08664. Also compare arXiv:1709.00031 and
  arXiv:2208.03273".
- **Primary:** abstract page <https://arxiv.org/abs/1404.4599v5>, accessed 2026-10-21T21:52:08Z
  (HTTP 200), 40,117 bytes, SHA-256
  `4f36fad483db20aec042a381cf0fa315f28c0e2979e0d9b7d8b23bc9464cd8ac` (re-fetched at 22:02:29Z,
  byte-identical); no licence link on the stub page; the e-print request fails (the examination
  helper stopped there, as for candidates 11, 13 and 18).
- **Content used:** the comment, read against the v4 text (S-0078) and against the author's later
  account in S-0080. The two "compare" references (arXiv:1709.00031, arXiv:2208.03273) were not
  retrieved.
- **Licence:** none stated on the stub; nothing copied beyond the comment quoted above.
- **Integrated:** Session 38 (2026-10-28), carried from the held branch at `c3a3d36`, where
  this entry was S-0071 (`analysis/s1-held-harvest-review/REVIEW.md` §4 step 2; Al's
  ruling Q-H6 of 2026-10-28). "This session" above is Session 22 (2026-10-21): the retrieval
  dates, access times and digests are Session 22's, and nothing was re-fetched.
- **Bears on:** S1.1 item 36 (its status document); no CLAIM.

---

## S-0080 — Martin Otto, "Acyclicity in finite groups and groupoids", arXiv:1806.08664 **v6** — the paper the S-0079 notice names as overcoming the problem, with the author's account of the flaw

- **State:** VERIFIED (retrieved and read in part this session; digest-only, no copy committed).
  Supplementary to the status document of item 36 — the author's own later account of the flaw
  and of who found it; it is not the notice.
- **Citation (from the abstract page):** Martin Otto, "Acyclicity in finite groups and
  groupoids", arXiv:1806.08664, Combinatorics (math.CO), cross-listed Discrete Mathematics
  (cs.DM) and Group Theory (math.GR); v1 22 June 2018; v2 27 December 2018 (withdrawn); v3 5
  February 2020; v4 2 July 2021; v5 12 October 2022; **v6 submitted 15 February 2024**, comment:
  "An error in (v1) invalidated a more direct reduction of the groupoidal case to the group case.
  The current version further corrects and expands some of the core technical arguments, and
  introduces some core concepts more systematically (partly in dedicated new sections). See
  acknowledgements for a more detailed account; (v5/6) contain minor corrections and updates". No
  journal reference or DOI on the abstract page. One author.
- **Primary:** abstract page <https://arxiv.org/abs/1806.08664> (the current version, v6),
  accessed 2026-10-21T21:53:27Z (HTTP 200), 40,695 bytes, SHA-256
  `ffccecdf45d9751e7885081aa8d0c10ad341b2109646fdaebc90ffe521fac30a`; e-print
  <https://arxiv.org/e-print/1806.08664v6> (`application/gzip`), accessed 2026-10-21T21:54:01Z,
  59,896 bytes, SHA-256 `02f6c59b3e992c6c2f6831cab991e1d7b23ae84299b4a9b3d3101b8d38247ac7` — a
  single gzipped TeX file (`main.tex`, 5,129 lines).
- **Content used:** the abstract ("We expound a concise construction of finite groups and
  groupoids whose Cayley graphs satisfy graded acyclicity requirements …"); the introduction's
  "The present exposition not only serves to correct a serious mistake in the construction of the
  relevant groupoids that was sketched in [lics2013]" (with [lics2013] = M. Otto, "Groupoids,
  hypergraphs and symmetries in finite models", LICS 2013 — the exposition that arXiv:1404.4599
  "extends and supersedes" per its own comment); the footnote "Indeed the proposal of a naive
  completion operation accounts for the major flaw in [lics2013]" beside "A major difference is
  the absence of a simple completion operation for I-graphs"; the conclusion's "a conceptual
  improvement and correction of the proposed constructions for groupoids from [lics2013]"; and
  the acknowledgements: "The proposed construction of N-acyclic groupoids in [lics2013], however,
  contained a serious flaw that was also carried in the arXiv preprints. This mistake from
  [lics2013] was finally found out in 2019 by Julian Bitterlich …"; "I am deeply indebted to
  Julian Bitterlich for identifying and clarifying the gap in my earlier attempts at the
  construction of finite N-acyclic groupoids." Whether this paper proves Proposition 2.22 of
  arXiv:1404.4599 as there stated was not established: its concepts are reworked, and the item
  records the statement's truth as undetermined (`RECORD.md`).
- **Licence:** `http://arxiv.org/licenses/nonexclusive-distrib/1.0/` on the abstract page; copy
  not committed.
- **Integrated:** Session 38 (2026-10-28), carried from the held branch at `c3a3d36`, where
  this entry was S-0072 (`analysis/s1-held-harvest-review/REVIEW.md` §4 step 2; Al's
  ruling Q-H6 of 2026-10-28). "This session" above is Session 22 (2026-10-21): the retrieval
  dates, access times and digests are Session 22's, and nothing was re-fetched.
- **Bears on:** S1.1 item 36 (the record's truth discussion and the finder of the error); no
  CLAIM.
