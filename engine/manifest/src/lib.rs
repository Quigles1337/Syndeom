//! SYNDEOM proof-artifact manifests — the SD1 carrier, as decided in ADR-0001 (amended Session 4).
//!
//! `engine/` is the canonical tier (SD10): only what this crate computes may enter CLAIMS.md as
//! MEASURED or REPRODUCED. Every rule implemented here names its ADR-0001 clause so a successor can
//! check code against decision without guessing.
//!
//! - D1: I-JSON; on-disk form pretty-printed with JCS-ordered keys and one trailing LF
//!   ([`to_disk_json`]).
//! - D2: `artifact_id` is a pure function of (producer, config, seed) ([`artifact_id`]).
//! - D3: the byte formats `syndeom-informal-md/1` ([`parse_informal_md`]), `syndeom-lean/1`
//!   ([`parse_lean`]) and `raw/1` (no parser: bytes as received).
//! - D4: the schema ([`Manifest`]), its consistency rules ([`Manifest::check`]), the derived
//!   canonical artifact convention ([`Manifest::check_derived`]), and the fidelity rule
//!   ([`statistic_verdict`], [`fidelity`]). Amended 2026-10-13 (Al's ruling (ii) on ADR-0003):
//!   `statement_status.value` may be `null` with `method: "undetermined"` under
//!   [`MANIFEST_SCHEMA`] (`syndeom-manifest/2`); [`MANIFEST_SCHEMA_V1`] remains accepted.
//! - D5: SHA-256 over RFC 8785 JCS bytes for `config_hash` and `prompt_hash` ([`config_hash`],
//!   [`prompt_hash`]); SHA-256 over exact bytes for `content_hash` ([`content_hash`]).
//! - D6: FRIEDMAN ADR-0004 binary64 arithmetic, `<=` inclusive, no fused operations.
//! - D7: the SD6 clause for LLM-assisted producers ([`Llm`]).

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

/// ADR-0001 D4 as amended 2026-10-13: the current schema string, written by every producer from
/// that amendment on. It differs from `/1` only in admitting a null `statement_status.value` with
/// `method: "undetermined"` (and the rule `null ⇒ proof_status invalid`).
pub const MANIFEST_SCHEMA: &str = "syndeom-manifest/2";
/// ADR-0001 D4 as ratified: the pre-amendment schema string. Still accepted (the D8 goldens); a
/// `/1` manifest with a null statement truth fails its self-check.
pub const MANIFEST_SCHEMA_V1: &str = "syndeom-manifest/1";
/// Every schema string this crate understands (SD14: consumers pin the string they understand).
pub const MANIFEST_SCHEMAS: &[&str] = &[MANIFEST_SCHEMA_V1, MANIFEST_SCHEMA];
/// ADR-0001 D3: the informal byte format.
pub const FORMAT_INFORMAL_MD: &str = "syndeom-informal-md/1";
/// ADR-0001 D3: the formal byte format.
pub const FORMAT_LEAN: &str = "syndeom-lean/1";
/// ADR-0001 D3: the archival format for ingested files (bytes as received).
pub const FORMAT_RAW: &str = "raw/1";
/// ADR-0001 D2: the informal file name for canonical-format files.
pub const FILE_INFORMAL: &str = "proof.md";
/// ADR-0001 D2: the formal file name for canonical-format files.
pub const FILE_FORMAL: &str = "Proof.lean";
/// ADR-0001 D3: the informal terminator line (U+220E).
pub const TERMINATOR: &str = "\u{220E}";
/// ADR-0001 D3: marker of the statement paragraph (step 0).
pub const THEOREM_MARKER: &str = "**Theorem.** ";
/// ADR-0001 D3: the proof heading line.
pub const PROOF_LINE: &str = "**Proof.**";
/// ADR-0001 D3: `rng` value of an ingest or derivation producer.
pub const RNG_NONE: &str = "none";
/// ADR-0001 D7: `rng` value of an LLM-assisted producer.
pub const RNG_LLM: &str = "llm";
/// ADR-0001 D3: top-level Lean declaration keywords counted by `n_declarations`.
pub const LEAN_DECLARATION_KEYWORDS: &[&str] = &[
    "theorem",
    "lemma",
    "def",
    "axiom",
    "instance",
    "abbrev",
    "structure",
    "inductive",
    "opaque",
    "example",
];

/// serde helper: the field must be present; `null` is allowed and maps to `None`.
fn required_option<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(d)
}

/// ADR-0001 D4 — `origin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    #[serde(rename = "synthetic")]
    Synthetic,
    #[serde(rename = "ground-truth")]
    GroundTruth,
    #[serde(rename = "contact")]
    Contact,
}

impl Origin {
    /// ADR-0001 D2: the directory root an artifact of this origin lives under.
    pub fn root(self) -> &'static str {
        match self {
            Self::Synthetic => "corpora",
            Self::GroundTruth => "groundtruth",
            Self::Contact => "contact",
        }
    }
}

/// ADR-0001 D4 — top-level manifest object. Unknown fields are rejected (I-JSON strictness, and
/// in particular no timestamp field is ever accepted).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub manifest_schema: String,
    pub artifact_id: String,
    pub origin: Origin,
    pub generator: Generator,
    pub engine_commit: String,
    #[serde(deserialize_with = "required_option")]
    pub seed: Option<u64>,
    pub config: Value,
    pub config_hash: String,
    #[serde(deserialize_with = "required_option")]
    pub toolchain: Option<Toolchain>,
    pub artifact: Artifact,
    #[serde(deserialize_with = "required_option")]
    pub ground_truth: Option<GroundTruth>,
    pub commanded: BTreeMap<String, Commanded>,
    pub achieved: BTreeMap<String, Achieved>,
    pub fidelity: Fidelity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// ADR-0001 D4 — `generator`: the versioned deterministic producer of the artifact directory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Generator {
    pub id: String,
    pub version: String,
    pub rng: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub llm: Option<Llm>,
}

/// ADR-0001 D7 — the SD6 clause for LLM-assisted producers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Llm {
    pub model_id: String,
    pub model_version: String,
    pub prompt_hash: String,
    pub sampling: Value,
    pub output_content_hash: String,
    pub adr: String,
}

/// ADR-0001 D4 — `toolchain`, present iff a formal file is present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Toolchain {
    pub lean: String,
    pub mathlib: String,
}

/// ADR-0001 D4 — `artifact.kind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Informal,
    Formal,
    Pair,
}

/// ADR-0001 D4 — `artifact`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub kind: Kind,
    pub files: Files,
}

/// ADR-0001 D4 — `artifact.files`; key presence is governed by `kind`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Files {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub informal: Option<InformalFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formal: Option<FormalFile>,
}

/// ADR-0001 D2 — `storage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Storage {
    Committed,
    Regenerable,
    External,
}

/// ADR-0001 D4 — the informal file record. Counts are `null` iff `format == "raw/1"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InformalFile {
    pub file: String,
    pub format: String,
    pub storage: Storage,
    pub content_hash: String,
    pub n_bytes: u64,
    #[serde(deserialize_with = "required_option")]
    pub n_tokens: Option<u64>,
    #[serde(deserialize_with = "required_option")]
    pub n_steps: Option<u64>,
    #[serde(deserialize_with = "required_option")]
    pub n_lemmas: Option<u64>,
}

/// ADR-0001 D4 — the formal file record. Counts are `null` iff `format == "raw/1"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormalFile {
    pub file: String,
    pub format: String,
    pub storage: Storage,
    pub content_hash: String,
    pub n_bytes: u64,
    #[serde(deserialize_with = "required_option")]
    pub n_tokens: Option<u64>,
    #[serde(deserialize_with = "required_option")]
    pub n_lines: Option<u64>,
    #[serde(deserialize_with = "required_option")]
    pub n_declarations: Option<u64>,
    #[serde(deserialize_with = "required_option")]
    pub main_declaration: Option<String>,
}

/// ADR-0001 D4 — `ground_truth.derivation` (SD12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Derivation {
    #[serde(rename = "by-construction")]
    ByConstruction,
    #[serde(rename = "documented-by-source")]
    DocumentedBySource,
    #[serde(rename = "formalized-in-mathlib")]
    FormalizedInMathlib,
}

/// ADR-0001 D4 — how `statement_status.value` was established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatementMethod {
    #[serde(rename = "mathlib")]
    Mathlib,
    #[serde(rename = "lean-kernel")]
    LeanKernel,
    #[serde(rename = "source-document")]
    SourceDocument,
    #[serde(rename = "counterexample")]
    Counterexample,
    /// 2026-10-13 amendment (`syndeom-manifest/2`): the status document named by `reference`
    /// documents the proof as invalid and leaves the statement's truth open. Used iff `value` is
    /// `null`.
    #[serde(rename = "undetermined")]
    Undetermined,
}

/// ADR-0001 D4 — `ground_truth.statement_status`. `value` is `null` iff `method` is
/// `"undetermined"` (2026-10-13 amendment; `/2` only); the key is always present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatementStatus {
    #[serde(deserialize_with = "required_option")]
    pub value: Option<bool>,
    pub method: StatementMethod,
    pub reference: String,
}

/// ADR-0001 D4 — `ground_truth.proof_status`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProofStatus {
    Valid,
    Invalid,
}

/// FLAWS.md severity vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Cosmetic,
    Repairable,
    Gap,
    Fatal,
}

/// FLAWS.md Lean-catchable vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeanCatchable {
    #[serde(rename = "YES-KERNEL")]
    YesKernel,
    #[serde(rename = "YES-TOOLING")]
    YesTooling,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "N/A")]
    NotApplicable,
}

/// Which file a location points into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileKey {
    Informal,
    Formal,
}

/// Tag type: the literal string `"informal"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InformalTag {
    #[serde(rename = "informal")]
    Informal,
}

/// Tag type: the literal string `"formal"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormalTag {
    #[serde(rename = "formal")]
    Formal,
}

/// ADR-0001 D3 — informal location: step index and code-point span `[s, e)`; `s == e` is an
/// omission point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InformalLocation {
    pub artifact: InformalTag,
    pub step: u64,
    pub span: [u64; 2],
}

/// ADR-0001 D3 — formal location: `<declaration>:<line>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormalLocation {
    pub artifact: FormalTag,
    pub declaration: String,
    pub line: u64,
}

/// ADR-0001 D3 — location inside a `raw/1` file: the documenting source's own locator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawLocation {
    pub artifact: FileKey,
    pub locator: String,
}

/// ADR-0001 D3 — the three location forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Location {
    Informal(InformalLocation),
    Formal(FormalLocation),
    Raw(RawLocation),
}

impl Location {
    /// The file this location points into.
    pub fn file_key(&self) -> FileKey {
        match self {
            Self::Informal(_) => FileKey::Informal,
            Self::Formal(_) => FileKey::Formal,
            Self::Raw(r) => r.artifact,
        }
    }

    /// True for the two canonical (D3) forms, false for a `raw/1` locator.
    pub fn is_canonical(&self) -> bool {
        !matches!(self, Self::Raw(_))
    }
}

/// ADR-0001 D4 — one `ground_truth.flaws[]` record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Flaw {
    pub instance: u64,
    pub class: String,
    #[serde(deserialize_with = "required_option")]
    pub sub_variant: Option<String>,
    pub location: Location,
    pub severity: Severity,
    pub lean_catchable: LeanCatchable,
    pub dials: Value,
}

/// ADR-0001 D4 — `ground_truth` (absent, as `null`, for every contact artifact).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroundTruth {
    pub derivation: Derivation,
    pub statement_status: StatementStatus,
    pub proof_status: ProofStatus,
    pub correct: bool,
    pub flaws: Vec<Flaw>,
    pub taxonomy_version: String,
}

/// ADR-0001 D4 — one `commanded` entry (FRIEDMAN D4 shape).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Commanded {
    pub value: Value,
    pub tolerance: Tolerance,
}

/// ADR-0001 D4 — tolerance forms. Exactly one of `abs`, `rel`, `exact`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tolerance {
    Abs { abs: f64 },
    Rel { rel: f64 },
    Exact { exact: bool },
}

/// ADR-0001 D4 — one `achieved` entry; `reason` is required iff `value` is null (checked by
/// [`Manifest::check`], not by the type).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Achieved {
    #[serde(deserialize_with = "required_option")]
    pub value: Option<Value>,
    pub estimator: String,
    #[serde(deserialize_with = "required_option")]
    pub ci95: Option<[f64; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// ADR-0001 D4 — `fidelity`, the S2.2 verdict block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fidelity {
    pub per_statistic: BTreeMap<String, Verdict>,
    pub verdict: Verdict,
}

/// `"PASS"` / `"FAIL"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Verdict {
    Pass,
    Fail,
}

// ---- D5 hashing -------------------------------------------------------------------------------

/// Lowercase-hex SHA-256 of `bytes` (ADR-0001 D5: every hash in a manifest).
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// ADR-0001 D5: `content_hash` = SHA-256 over a file's exact bytes.
pub fn content_hash(bytes: &[u8]) -> String {
    sha256_hex(bytes)
}

/// ADR-0001 D5: the RFC 8785 (JCS) canonical bytes of a JSON value (S-0007).
pub fn jcs_bytes(value: &Value) -> serde_json::Result<Vec<u8>> {
    serde_json_canonicalizer::to_vec(value)
}

/// ADR-0001 D5: `config_hash` = SHA-256 over JCS(`config`).
pub fn config_hash(config: &Value) -> serde_json::Result<String> {
    Ok(sha256_hex(&jcs_bytes(config)?))
}

/// ADR-0001 D7: `prompt_hash` = SHA-256 over JCS of the full prompt object.
pub fn prompt_hash(prompt: &Value) -> serde_json::Result<String> {
    config_hash(prompt)
}

/// ADR-0001 D2: `<id>-v<version>-<config_hash[0:12]>-s<seed>`, or without the `-s<seed>` segment
/// when `seed` is `None` (ground-truth and contact producers).
pub fn artifact_id(
    generator_id: &str,
    generator_version: &str,
    config_hash: &str,
    seed: Option<u64>,
) -> String {
    let prefix: String = config_hash.chars().take(12).collect();
    match seed {
        Some(s) => format!("{generator_id}-v{generator_version}-{prefix}-s{s}"),
        None => format!("{generator_id}-v{generator_version}-{prefix}"),
    }
}

// ---- D3 byte formats --------------------------------------------------------------------------

/// Whitespace per ADR-0001 D3: U+0009, U+000A, U+000D, U+0020.
fn is_ws(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | ' ')
}

/// Number of maximal runs of non-whitespace code points (ADR-0001 D3 `n_tokens`).
pub fn count_tokens(text: &str) -> u64 {
    text.split(is_ws).filter(|t| !t.is_empty()).count() as u64
}

/// Common byte-level rules of the two canonical formats: UTF-8, no BOM, no U+000D, no U+0009,
/// non-empty, exactly one trailing U+000A, no trailing U+0020 on any line. Returns the lines.
fn common_text_rules(bytes: &[u8]) -> Result<(&str, Vec<&str>), FormatError> {
    let text = std::str::from_utf8(bytes).map_err(|_| FormatError::NotUtf8)?;
    if text.starts_with('\u{FEFF}') {
        return Err(FormatError::ByteOrderMark);
    }
    if let Some(offset) = bytes.iter().position(|&b| b == b'\r') {
        return Err(FormatError::CarriageReturn { offset });
    }
    if let Some(offset) = bytes.iter().position(|&b| b == b'\t') {
        return Err(FormatError::Tab { offset });
    }
    if text.is_empty() {
        return Err(FormatError::Empty);
    }
    if !text.ends_with('\n') {
        return Err(FormatError::MissingFinalNewline);
    }
    if text.ends_with("\n\n") {
        return Err(FormatError::TrailingBlankLine);
    }
    let lines: Vec<&str> = text[..text.len() - 1].split('\n').collect();
    for (i, l) in lines.iter().enumerate() {
        if l.ends_with(' ') {
            return Err(FormatError::TrailingSpace { line: i + 1 });
        }
    }
    Ok((text, lines))
}

/// A violation of a canonical byte format (ADR-0001 D3). Lines are 1-based.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    NotUtf8,
    ByteOrderMark,
    CarriageReturn {
        offset: usize,
    },
    Tab {
        offset: usize,
    },
    Empty,
    MissingFinalNewline,
    TrailingBlankLine,
    TrailingSpace {
        line: usize,
    },
    /// informal: the title line is not `# <title>`
    BadTitle,
    /// informal: a blank line where a paragraph was expected, or two blank lines in a row
    BlankLine {
        line: usize,
    },
    /// informal: the statement paragraph does not begin with the theorem marker
    MissingTheorem,
    /// informal: the `**Proof.**` line is missing or not alone
    MissingProofLine,
    /// informal: the file does not end with the terminator line
    MissingTerminator,
    /// informal: step paragraph `index` (1-based among steps) does not begin with `**Step index.** `
    BadStepMarker {
        index: usize,
    },
    /// lean: an import that is not `Mathlib` or `Mathlib.*`
    InvalidImport {
        line: usize,
        module: String,
    },
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUtf8 => write!(f, "not valid UTF-8"),
            Self::ByteOrderMark => write!(f, "starts with a byte order mark"),
            Self::CarriageReturn { offset } => write!(f, "U+000D at byte offset {offset}"),
            Self::Tab { offset } => write!(f, "U+0009 at byte offset {offset}"),
            Self::Empty => write!(f, "file is empty"),
            Self::MissingFinalNewline => write!(f, "last line is not terminated by U+000A"),
            Self::TrailingBlankLine => write!(f, "file ends with a blank line"),
            Self::TrailingSpace { line } => write!(f, "line {line} ends with U+0020"),
            Self::BadTitle => write!(f, "line 1 is not `# <title>`"),
            Self::BlankLine { line } => write!(f, "unexpected blank line at line {line}"),
            Self::MissingTheorem => write!(f, "statement paragraph lacks `{THEOREM_MARKER}`"),
            Self::MissingProofLine => write!(f, "`{PROOF_LINE}` line missing or not alone"),
            Self::MissingTerminator => write!(f, "file does not end with the `{TERMINATOR}` line"),
            Self::BadStepMarker { index } => write!(f, "step {index} lacks `**Step {index}.** `"),
            Self::InvalidImport { line, module } => {
                write!(f, "line {line} imports {module:?}, not Mathlib")
            }
        }
    }
}

impl std::error::Error for FormatError {}

/// Segmentation of a `syndeom-informal-md/1` document (ADR-0001 D3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InformalStats {
    pub n_bytes: u64,
    pub n_tokens: u64,
    pub n_steps: u64,
    pub n_lemmas: u64,
    /// `steps[0]` is the statement text; `steps[k]` is step k's text (marker removed).
    pub steps: Vec<String>,
}

/// True iff `text` begins with `Lemma <positive integer>.` (ADR-0001 D3 `n_lemmas`).
pub fn is_lemma_step(text: &str) -> bool {
    let Some(rest) = text.strip_prefix("Lemma ") else {
        return false;
    };
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    !digits.is_empty() && !digits.starts_with('0') && rest[digits.len()..].starts_with('.')
}

/// ADR-0001 D3: validate `syndeom-informal-md/1` bytes and segment them into steps.
pub fn parse_informal_md(bytes: &[u8]) -> Result<InformalStats, FormatError> {
    let (_, lines) = common_text_rules(bytes)?;
    // paragraphs = maximal runs of non-blank lines, separated by exactly one blank line
    let mut paragraphs: Vec<Vec<&str>> = Vec::new();
    let mut current: Vec<&str> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if l.is_empty() {
            if current.is_empty() {
                return Err(FormatError::BlankLine { line: i + 1 });
            }
            paragraphs.push(std::mem::take(&mut current));
        } else {
            current.push(l);
        }
    }
    if !current.is_empty() {
        paragraphs.push(current);
    }
    if paragraphs.len() < 4 {
        return Err(FormatError::MissingTerminator);
    }
    let title = &paragraphs[0];
    if title.len() != 1 || !title[0].starts_with("# ") || title[0].len() <= 2 {
        return Err(FormatError::BadTitle);
    }
    let statement = paragraphs[1].join("\n");
    let Some(statement_text) = statement.strip_prefix(THEOREM_MARKER) else {
        return Err(FormatError::MissingTheorem);
    };
    if paragraphs[2] != [PROOF_LINE] {
        return Err(FormatError::MissingProofLine);
    }
    let last = paragraphs.len() - 1;
    if paragraphs[last] != [TERMINATOR] {
        return Err(FormatError::MissingTerminator);
    }
    let mut steps = vec![statement_text.to_string()];
    for (index, p) in paragraphs[3..last].iter().enumerate() {
        let index = index + 1;
        let text = p.join("\n");
        let marker = format!("**Step {index}.** ");
        let Some(step_text) = text.strip_prefix(&marker) else {
            return Err(FormatError::BadStepMarker { index });
        };
        steps.push(step_text.to_string());
    }
    let n_steps = (steps.len() - 1) as u64;
    let n_lemmas = steps.iter().skip(1).filter(|t| is_lemma_step(t)).count() as u64;
    let n_tokens = steps.iter().map(|t| count_tokens(t)).sum();
    Ok(InformalStats {
        n_bytes: bytes.len() as u64,
        n_tokens,
        n_steps,
        n_lemmas,
        steps,
    })
}

/// ADR-0001 D3: the code points of `step_text` addressed by `span` (`[s, e)`), or `None` if the
/// span is out of bounds. Spans index Unicode scalar values of the UTF-8-decoded bytes, with no
/// normalization and no grapheme clustering; an empty span (`s == e`) returns `Some("")`.
pub fn span_text(step_text: &str, span: [u64; 2]) -> Option<String> {
    let [s, e] = span;
    let len = step_text.chars().count() as u64;
    if s > e || e > len {
        return None;
    }
    Some(
        step_text
            .chars()
            .skip(s as usize)
            .take((e - s) as usize)
            .collect(),
    )
}

/// Counts of a `syndeom-lean/1` file (ADR-0001 D3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeanStats {
    pub n_bytes: u64,
    pub n_tokens: u64,
    pub n_lines: u64,
    pub n_declarations: u64,
    /// `(line, name)` of every top-level declaration, 1-based lines, in file order.
    pub declarations: Vec<(u64, String)>,
    /// imported modules, in file order
    pub imports: Vec<String>,
}

/// The identifier that follows a declaration keyword: up to the first whitespace, `(`, `{`, `[`,
/// `:` or `⦃`.
fn declaration_name(after_keyword: &str) -> String {
    after_keyword
        .trim_start()
        .chars()
        .take_while(|c| !is_ws(*c) && !matches!(*c, '(' | '{' | '[' | ':' | '⦃'))
        .collect()
}

/// ADR-0001 D3: validate `syndeom-lean/1` bytes and count them.
pub fn parse_lean(bytes: &[u8]) -> Result<LeanStats, FormatError> {
    let (text, lines) = common_text_rules(bytes)?;
    let mut imports = Vec::new();
    let mut declarations = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if let Some(module) = l.strip_prefix("import ") {
            let module = module.trim();
            if module != "Mathlib" && !module.starts_with("Mathlib.") {
                return Err(FormatError::InvalidImport {
                    line: i + 1,
                    module: module.to_string(),
                });
            }
            imports.push(module.to_string());
            continue;
        }
        for kw in LEAN_DECLARATION_KEYWORDS {
            if let Some(rest) = l.strip_prefix(kw)
                && rest.starts_with(' ')
            {
                declarations.push(((i + 1) as u64, declaration_name(rest)));
                break;
            }
        }
    }
    Ok(LeanStats {
        n_bytes: bytes.len() as u64,
        n_tokens: count_tokens(text),
        n_lines: lines.len() as u64,
        n_declarations: declarations.len() as u64,
        declarations,
        imports,
    })
}

// ---- D4 / D6 fidelity ---------------------------------------------------------------------------

/// ADR-0001 D4 + D6 — the per-statistic verdict rule (FRIEDMAN ADR-0004 D1 arithmetic).
///
/// FAIL if the statistic was not measured or its value is `null`. `abs`/`rel` compare point
/// estimates in binary64 with `<=` and no fused operations (non-numeric values FAIL); `exact`
/// compares JSON values structurally. `ci95` never enters the verdict.
pub fn statistic_verdict(commanded: &Commanded, achieved: Option<&Achieved>) -> Verdict {
    let Some(achieved_value) = achieved.and_then(|a| a.value.as_ref()) else {
        return Verdict::Fail;
    };
    let pass = match commanded.tolerance {
        Tolerance::Exact { exact: true } => *achieved_value == commanded.value,
        Tolerance::Exact { exact: false } => false,
        Tolerance::Abs { abs } => match (commanded.value.as_f64(), achieved_value.as_f64()) {
            (Some(c), Some(a)) => (a - c).abs() <= abs,
            _ => false,
        },
        Tolerance::Rel { rel } => match (commanded.value.as_f64(), achieved_value.as_f64()) {
            (Some(c), Some(a)) => (a - c).abs() <= rel * c.abs(),
            _ => false,
        },
    };
    if pass { Verdict::Pass } else { Verdict::Fail }
}

/// ADR-0001 D4 — the fidelity block: one verdict per commanded key; overall PASS iff all PASS
/// (vacuously PASS when nothing is commanded).
pub fn fidelity(
    commanded: &BTreeMap<String, Commanded>,
    achieved: &BTreeMap<String, Achieved>,
) -> Fidelity {
    let per_statistic: BTreeMap<String, Verdict> = commanded
        .iter()
        .map(|(k, c)| (k.clone(), statistic_verdict(c, achieved.get(k))))
        .collect();
    let verdict = if per_statistic.values().all(|v| *v == Verdict::Pass) {
        Verdict::Pass
    } else {
        Verdict::Fail
    };
    Fidelity {
        per_statistic,
        verdict,
    }
}

// ---- D1 disk form -----------------------------------------------------------------------------

/// ADR-0001 D1 — the on-disk form: 2-space indent, keys in JCS order, single trailing LF.
///
/// Key order follows from `serde_json::Value`'s sorted map (this crate does not enable
/// `preserve_order`); for ASCII keys that order coincides with JCS §3.2.3.
pub fn to_disk_json(manifest: &Manifest) -> serde_json::Result<String> {
    let value = serde_json::to_value(manifest)?;
    let mut s = serde_json::to_string_pretty(&value)?;
    s.push('\n');
    Ok(s)
}

/// Parse a manifest from JSON text (D4; unknown fields rejected).
pub fn from_json_str(text: &str) -> serde_json::Result<Manifest> {
    serde_json::from_str(text)
}

// ---- D4 self-check ----------------------------------------------------------------------------

fn is_lower_hex(s: &str, len: usize) -> bool {
    s.len() == len
        && s.chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

fn is_flaw_class(s: &str) -> bool {
    s.len() == 9 && s.starts_with("FLAW-") && s[5..].chars().all(|c| c.is_ascii_digit())
}

fn is_source_id(v: &Value) -> bool {
    v.as_str().is_some_and(|s| {
        s.len() == 6 && s.starts_with("S-") && s[2..].chars().all(|c| c.is_ascii_digit())
    })
}

/// The bytes of the stored files, when the caller has them; `None` means "not available", in
/// which case the hash and count rules for that file are skipped (never treated as passed).
#[derive(Debug, Clone, Copy, Default)]
pub struct FileBytes<'a> {
    pub informal: Option<&'a [u8]>,
    pub formal: Option<&'a [u8]>,
}

impl Manifest {
    /// Recompute everything recomputable and report every discrepancy against ADR-0001.
    ///
    /// Returns `Ok(())` when the manifest is internally consistent (and consistent with whatever
    /// file bytes were supplied); otherwise every violated clause, so a caller sees the whole
    /// picture rather than the first failure.
    pub fn check(&self, bytes: FileBytes<'_>) -> Result<(), Vec<String>> {
        let mut problems = Vec::new();
        let mut push = |s: String| problems.push(s);

        let schema_v1 = self.manifest_schema == MANIFEST_SCHEMA_V1;
        if !MANIFEST_SCHEMAS.contains(&self.manifest_schema.as_str()) {
            push(format!(
                "D4: manifest_schema is {:?}, expected one of {MANIFEST_SCHEMAS:?}",
                self.manifest_schema
            ));
        }
        if !is_lower_hex(&self.engine_commit, 40) {
            push("D4/SD6: engine_commit must be a 40-character lowercase hex git SHA".to_string());
        }
        // seed null iff origin != synthetic
        match (self.origin, self.seed) {
            (Origin::Synthetic, None) => {
                push("D4: synthetic artifacts carry an integer seed".into())
            }
            (Origin::GroundTruth | Origin::Contact, Some(_)) => {
                push("D4: seed must be null for ground-truth and contact artifacts".into());
            }
            _ => {}
        }
        // config hash and id
        match config_hash(&self.config) {
            Ok(h) if h == self.config_hash => {
                let id = artifact_id(&self.generator.id, &self.generator.version, &h, self.seed);
                if id != self.artifact_id {
                    push(format!(
                        "D2: artifact_id is {:?}, derived {id:?}",
                        self.artifact_id
                    ));
                }
            }
            Ok(h) => push(format!(
                "D5: config_hash is {}, recomputed {h}",
                self.config_hash
            )),
            Err(e) => push(format!("D5: config cannot be canonicalized: {e}")),
        }
        // producers of real items: rng none, required config keys
        if self.origin != Origin::Synthetic {
            if self.generator.rng != RNG_NONE {
                push(format!(
                    "D4: an ingest or derivation producer has rng {RNG_NONE:?}, got {:?}",
                    self.generator.rng
                ));
            }
            let cfg = self.config.as_object();
            let has = |k: &str| cfg.is_some_and(|o| o.contains_key(k));
            let sources_ok = cfg
                .and_then(|o| o.get("sources"))
                .and_then(Value::as_array)
                .is_some_and(|a| !a.is_empty() && a.iter().all(is_source_id));
            if !sources_ok {
                push("D4: config.sources must be a non-empty array of S-NNNN ids".into());
            }
            for k in ["item", "licence"] {
                if !has(k) {
                    push(format!("D4: config.{k} is required for real items"));
                }
            }
            let status_ok =
                cfg.and_then(|o| o.get("status"))
                    .and_then(Value::as_object)
                    .is_some_and(|s| {
                        s.get("kind").and_then(Value::as_str).is_some_and(|k| {
                            matches!(k, "erratum" | "retraction" | "mathlib" | "none")
                        }) && s.get("reference").is_some_and(Value::is_string)
                    });
            if !status_ok {
                push(
                    "D4: config.status must be {kind: erratum|retraction|mathlib|none, reference}"
                        .into(),
                );
            }
            if !self.commanded.is_empty() {
                push("D4: commanded must be empty for real items".into());
            }
        } else if let Some(cfg) = self.config.as_object()
            && cfg.contains_key("derived_from")
        {
            push("D4: a synthetic artifact cannot be derived_from a parent".into());
        }
        // D7: LLM clause
        if let Some(llm) = &self.generator.llm {
            if self.generator.rng != RNG_LLM {
                push(format!("D7: an LLM-assisted producer has rng {RNG_LLM:?}"));
            }
            let hashes: Vec<&str> = [
                self.artifact
                    .files
                    .informal
                    .as_ref()
                    .map(|f| f.content_hash.as_str()),
                self.artifact
                    .files
                    .formal
                    .as_ref()
                    .map(|f| f.content_hash.as_str()),
            ]
            .into_iter()
            .flatten()
            .collect();
            if !hashes.contains(&llm.output_content_hash.as_str()) {
                push("D7: output_content_hash re-verifies against no stored file; artifact uncounted".into());
            }
            if !is_lower_hex(&llm.prompt_hash, 64) {
                push("D7: prompt_hash must be a lowercase SHA-256 hex digest".into());
            }
        }
        // kind <-> files
        let (want_informal, want_formal) = match self.artifact.kind {
            Kind::Informal => (true, false),
            Kind::Formal => (false, true),
            Kind::Pair => (true, true),
        };
        if want_informal != self.artifact.files.informal.is_some() {
            push("D4: artifact.files.informal presence must match artifact.kind".into());
        }
        if want_formal != self.artifact.files.formal.is_some() {
            push("D4: artifact.files.formal presence must match artifact.kind".into());
        }
        if self.toolchain.is_some() != self.artifact.files.formal.is_some() {
            push("D4: toolchain is present iff a formal file is present".into());
        }
        // informal file record
        let mut informal_steps: Option<Vec<String>> = None;
        if let Some(f) = &self.artifact.files.informal {
            let raw = f.format == FORMAT_RAW;
            if !raw && f.format != FORMAT_INFORMAL_MD {
                push(format!("D3: unknown informal format {:?}", f.format));
            }
            if !raw && f.file != FILE_INFORMAL {
                push(format!(
                    "D2: canonical informal file must be {FILE_INFORMAL:?}"
                ));
            }
            if raw != f.n_tokens.is_none()
                || raw != f.n_steps.is_none()
                || raw != f.n_lemmas.is_none()
            {
                push("D4: informal counts are null iff format is raw/1".into());
            }
            if let Some(b) = bytes.informal {
                let ch = content_hash(b);
                if ch != f.content_hash {
                    push(format!(
                        "D5: informal content_hash is {}, recomputed {ch}",
                        f.content_hash
                    ));
                }
                if b.len() as u64 != f.n_bytes {
                    push(format!(
                        "D4: informal n_bytes stated {}, measured {}",
                        f.n_bytes,
                        b.len()
                    ));
                }
                if !raw {
                    match parse_informal_md(b) {
                        Ok(st) => {
                            if (Some(st.n_tokens), Some(st.n_steps), Some(st.n_lemmas))
                                != (f.n_tokens, f.n_steps, f.n_lemmas)
                            {
                                push(format!(
                                    "D4: informal counts stated (tokens {:?}, steps {:?}, lemmas {:?}), measured ({}, {}, {})",
                                    f.n_tokens,
                                    f.n_steps,
                                    f.n_lemmas,
                                    st.n_tokens,
                                    st.n_steps,
                                    st.n_lemmas
                                ));
                            }
                            informal_steps = Some(st.steps);
                        }
                        Err(e) => push(format!(
                            "D3: informal bytes violate {FORMAT_INFORMAL_MD}: {e}"
                        )),
                    }
                }
            }
        }
        // formal file record
        let mut lean_stats: Option<LeanStats> = None;
        if let Some(f) = &self.artifact.files.formal {
            let raw = f.format == FORMAT_RAW;
            if !raw && f.format != FORMAT_LEAN {
                push(format!("D3: unknown formal format {:?}", f.format));
            }
            if !raw && f.file != FILE_FORMAL {
                push(format!("D2: canonical formal file must be {FILE_FORMAL:?}"));
            }
            if raw != f.n_tokens.is_none()
                || raw != f.n_lines.is_none()
                || raw != f.n_declarations.is_none()
                || raw != f.main_declaration.is_none()
            {
                push("D4: formal counts and main_declaration are null iff format is raw/1".into());
            }
            if let Some(b) = bytes.formal {
                let ch = content_hash(b);
                if ch != f.content_hash {
                    push(format!(
                        "D5: formal content_hash is {}, recomputed {ch}",
                        f.content_hash
                    ));
                }
                if b.len() as u64 != f.n_bytes {
                    push(format!(
                        "D4: formal n_bytes stated {}, measured {}",
                        f.n_bytes,
                        b.len()
                    ));
                }
                if !raw {
                    match parse_lean(b) {
                        Ok(st) => {
                            if (Some(st.n_tokens), Some(st.n_lines), Some(st.n_declarations))
                                != (f.n_tokens, f.n_lines, f.n_declarations)
                            {
                                push(format!(
                                    "D4: formal counts stated (tokens {:?}, lines {:?}, declarations {:?}), measured ({}, {}, {})",
                                    f.n_tokens,
                                    f.n_lines,
                                    f.n_declarations,
                                    st.n_tokens,
                                    st.n_lines,
                                    st.n_declarations
                                ));
                            }
                            if let Some(main) = &f.main_declaration
                                && !st.declarations.iter().any(|(_, n)| n == main)
                            {
                                push(format!(
                                    "D4: main_declaration {main:?} is not declared in the file"
                                ));
                            }
                            lean_stats = Some(st);
                        }
                        Err(e) => push(format!("D3: formal bytes violate {FORMAT_LEAN}: {e}")),
                    }
                }
            }
        }
        // ground truth
        match (&self.ground_truth, self.origin) {
            (None, Origin::Contact) => {}
            (Some(_), Origin::Contact) => {
                push("D4/SD3: contact artifacts carry no ground truth".into())
            }
            (None, _) => push("D4: ground_truth is required unless origin is contact".into()),
            (Some(gt), origin) => {
                if (gt.derivation == Derivation::ByConstruction) != (origin == Origin::Synthetic) {
                    push("D4/SD12: derivation is by-construction iff origin is synthetic".into());
                }
                // 2026-10-13 amendment: null truth iff method undetermined; /2 only; implies invalid
                let truth = gt.statement_status.value;
                let undetermined = gt.statement_status.method == StatementMethod::Undetermined;
                if truth.is_none() != undetermined {
                    push(
                        "D4 (2026-10-13): statement_status.value is null iff method is \"undetermined\""
                            .into(),
                    );
                }
                if truth.is_none() && schema_v1 {
                    push(format!(
                        "D4 (2026-10-13): {MANIFEST_SCHEMA_V1} does not admit a null statement truth; write {MANIFEST_SCHEMA}"
                    ));
                }
                if truth.is_none() && gt.proof_status == ProofStatus::Valid {
                    push(
                        "D4 (2026-10-13): an undetermined statement truth requires proof_status invalid"
                            .into(),
                    );
                }
                let derived = truth == Some(true) && gt.proof_status == ProofStatus::Valid;
                if gt.correct != derived {
                    push(format!(
                        "D4/SD1: correct is {}, derived {derived}",
                        gt.correct
                    ));
                }
                if truth == Some(false) && gt.proof_status == ProofStatus::Valid {
                    push("D4: a false statement cannot have a valid proof".into());
                }
                if gt.proof_status == ProofStatus::Valid
                    && gt.flaws.iter().any(|fl| fl.severity != Severity::Cosmetic)
                {
                    push("D4: a valid proof may carry cosmetic flaws only".into());
                }
                if gt.taxonomy_version.is_empty() {
                    push("D4/SD1: taxonomy_version is required".into());
                }
                let mut seen: BTreeSet<(u64, FileKey)> = BTreeSet::new();
                for (i, fl) in gt.flaws.iter().enumerate() {
                    if fl.instance == 0 {
                        push(format!("D4: flaws[{i}].instance must be >= 1"));
                    }
                    if !is_flaw_class(&fl.class) {
                        push(format!(
                            "D4: flaws[{i}].class {:?} is not FLAW-NNNN",
                            fl.class
                        ));
                    }
                    let key = fl.location.file_key();
                    if !seen.insert((fl.instance, key)) {
                        push(format!(
                            "D4: flaws[{i}] repeats instance {} in the same file",
                            fl.instance
                        ));
                    }
                    let (present, raw) = match key {
                        FileKey::Informal => {
                            let f = self.artifact.files.informal.as_ref();
                            (f.is_some(), f.is_some_and(|f| f.format == FORMAT_RAW))
                        }
                        FileKey::Formal => {
                            let f = self.artifact.files.formal.as_ref();
                            (f.is_some(), f.is_some_and(|f| f.format == FORMAT_RAW))
                        }
                    };
                    if !present {
                        push(format!(
                            "D4: flaws[{i}].location points into an absent file"
                        ));
                    } else if raw == fl.location.is_canonical() {
                        push(format!(
                            "D3: flaws[{i}].location form does not match the file's format (raw/1 uses a locator; canonical formats use step/span or declaration/line)"
                        ));
                    }
                    match &fl.location {
                        Location::Informal(loc) => {
                            if let Some(steps) = &informal_steps {
                                match steps.get(loc.step as usize) {
                                    None => push(format!(
                                        "D3: flaws[{i}] step {} does not exist",
                                        loc.step
                                    )),
                                    Some(t) if span_text(t, loc.span).is_none() => push(format!(
                                        "D3: flaws[{i}] span {:?} is outside step {} ({} code points)",
                                        loc.span,
                                        loc.step,
                                        t.chars().count()
                                    )),
                                    Some(_) => {}
                                }
                            } else if loc.span[0] > loc.span[1] {
                                push(format!("D3: flaws[{i}] span start exceeds end"));
                            }
                        }
                        Location::Formal(loc) => {
                            if let Some(st) = &lean_stats {
                                if loc.line == 0 || loc.line > st.n_lines {
                                    push(format!(
                                        "D3: flaws[{i}] line {} is outside the file",
                                        loc.line
                                    ));
                                }
                                if !st.declarations.iter().any(|(_, n)| *n == loc.declaration) {
                                    push(format!(
                                        "D3: flaws[{i}] declaration {:?} is not declared in the file",
                                        loc.declaration
                                    ));
                                }
                            } else if loc.line == 0 {
                                push(format!("D3: flaws[{i}] line numbers are 1-based"));
                            }
                        }
                        Location::Raw(_) => {}
                    }
                }
            }
        }
        // achieved: reason iff null; fidelity recomputed
        for (k, a) in &self.achieved {
            if a.value.is_none() != a.reason.is_some() {
                push(format!(
                    "D4: achieved[{k:?}] must carry `reason` iff `value` is null"
                ));
            }
        }
        let expected = fidelity(&self.commanded, &self.achieved);
        if expected != self.fidelity {
            push(format!(
                "D4/S2.2: fidelity stated {:?}, recomputed {expected:?}",
                self.fidelity
            ));
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems)
        }
    }

    /// ADR-0001 D4 (derived canonical artifacts): check this manifest against its parent.
    ///
    /// The parent is the `raw/1` item of record; this manifest must name it in
    /// `config.derived_from`, share its origin, derivation and statuses, carry the same multiset of
    /// flaw (class, sub_variant, severity, lean_catchable), and express every location in a
    /// canonical D3 form. The written argument for the re-expression lives in the S1.3 record,
    /// not here.
    pub fn check_derived(&self, parent: &Manifest) -> Result<(), Vec<String>> {
        let mut problems = Vec::new();
        let derived_from = self.config.get("derived_from").and_then(Value::as_str);
        if derived_from != Some(parent.artifact_id.as_str()) {
            problems.push(format!(
                "D4: config.derived_from is {derived_from:?}, parent is {:?}",
                parent.artifact_id
            ));
        }
        if !self.config.get("method").is_some_and(Value::is_string) {
            problems.push("D4: config.method is required for a derived artifact".into());
        }
        if self.generator.rng != RNG_NONE || self.seed.is_some() {
            problems
                .push("D4: a derivation producer is deterministic (rng none, seed null)".into());
        }
        if self.origin != parent.origin {
            problems.push("D4: a derived artifact keeps its parent's origin".into());
        }
        let canonical = |f: &Files| {
            f.informal.as_ref().is_none_or(|x| x.format != FORMAT_RAW)
                && f.formal.as_ref().is_none_or(|x| x.format != FORMAT_RAW)
        };
        if !canonical(&self.artifact.files) {
            problems.push(
                "D4: a derived artifact's files are in canonical formats, never raw/1".into(),
            );
        }
        match (&self.ground_truth, &parent.ground_truth) {
            (None, None) => {}
            (Some(_), None) | (None, Some(_)) => {
                problems.push("D4: a derived artifact has ground truth iff its parent does".into());
            }
            (Some(d), Some(p)) => {
                if d.derivation != p.derivation
                    || d.statement_status != p.statement_status
                    || d.proof_status != p.proof_status
                    || d.correct != p.correct
                    || d.taxonomy_version != p.taxonomy_version
                {
                    problems.push(
                        "D4: derived ground truth differs from the parent's beyond locations"
                            .into(),
                    );
                }
                let sig = |gt: &GroundTruth| {
                    let mut v: Vec<(String, Option<String>, Severity, LeanCatchable)> = gt
                        .flaws
                        .iter()
                        .map(|f| {
                            (
                                f.class.clone(),
                                f.sub_variant.clone(),
                                f.severity,
                                f.lean_catchable,
                            )
                        })
                        .collect();
                    v.sort_by(|a, b| format!("{a:?}").cmp(&format!("{b:?}")));
                    v
                };
                if sig(d) != sig(p) {
                    problems.push("D4: derived flaw multiset differs from the parent's".into());
                }
                for (i, fl) in d.flaws.iter().enumerate() {
                    if !fl.location.is_canonical() {
                        problems.push(format!(
                            "D4: derived flaws[{i}].location is not in canonical D3 form"
                        ));
                    }
                }
            }
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems)
        }
    }
}
