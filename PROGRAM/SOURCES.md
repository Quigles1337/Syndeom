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
