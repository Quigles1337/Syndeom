//! ADR-0006: deterministic replay of reviewed UTF-8 statement/proof slices in a single
//! gzip TeX source. The complete decompressed source is retained as external context.
//! This is a transcription procedure, not a TeX interpreter or a status adjudicator.
#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use syndeom_manifest::*;

pub const GENERATOR: &str = "s1_derive";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const METHOD: &str = "reviewed-tex-slices/1";
const MAX_SOURCE: u64 = 16 * 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Slice {
    /// Half-open byte offsets in the complete decompressed source.
    pub range: [usize; 2],
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub role: String,
    pub slice: Slice,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mapping {
    pub instance: u64,
    pub step: u64,
    /// Half-open Unicode-scalar positions in the marker-free canonical step.
    pub span: [u64; 2],
    pub text_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    pub method: String,
    pub parent_manifest_sha256: String,
    pub context_sha256: String,
    pub environment: String,
    pub label: String,
    pub title: String,
    pub transformation: String,
    pub statement: Slice,
    pub proof: Vec<Slice>,
    /// Reviewer's context checklist; the output retains ALL source bytes, not just these.
    pub context: Vec<Context>,
    pub mappings: Vec<Mapping>,
}

pub struct Product {
    pub manifest: Manifest,
    pub proof: Vec<u8>,
    /// Opaque bytes: the original bibliography need not be UTF-8.
    pub context: Vec<u8>,
    /// Public metadata only: offsets, digests and canonical locations, no source excerpts.
    pub source_map: Value,
}

fn require(ok: bool, message: &str) -> Result<(), String> {
    if ok { Ok(()) } else { Err(message.into()) }
}

fn checked_slice<'a>(source: &'a [u8], spec: &Slice) -> Result<&'a [u8], String> {
    let [s, e] = spec.range;
    let bytes = source
        .get(s..e)
        .filter(|b| !b.is_empty())
        .ok_or("empty or out-of-bounds source slice")?;
    require(
        content_hash(bytes) == spec.sha256,
        "source slice hash mismatch",
    )?;
    Ok(bytes)
}

fn unique(source: &[u8], marker: &str) -> Result<usize, String> {
    let positions: Vec<_> = source
        .windows(marker.len())
        .enumerate()
        .filter_map(|(i, b)| (b == marker.as_bytes()).then_some(i))
        .collect();
    require(
        positions.len() == 1,
        &format!("ambiguous/missing boundary: {marker}"),
    )?;
    Ok(positions[0])
}

fn trim_range(source: &[u8], mut s: usize, mut e: usize) -> [usize; 2] {
    while s < e && source[s].is_ascii_whitespace() {
        s += 1;
    }
    while s < e && source[e - 1].is_ascii_whitespace() {
        e -= 1;
    }
    [s, e]
}

/// Exact permitted transformation: trim and collapse ASCII spaces/tabs/CR/LF only.
/// Non-whitespace Unicode scalars, TeX commands, mathematics and errors are unchanged.
fn normalize(bytes: &[u8]) -> Result<String, String> {
    let text =
        std::str::from_utf8(bytes).map_err(|e| format!("selected text is not UTF-8: {e}"))?;
    require(
        !text
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t')),
        "unsupported control character",
    )?;
    for forbidden in [
        "%",
        "\\\\",
        "\\par",
        "\\verb",
        "\\input",
        "\\include",
        "\\catcode",
        "\\newcommand",
        "\\renewcommand",
        "\\def",
        "\\write",
        "\\read",
        "\\csname",
        "\\begin",
        "\\end",
        "\\label",
        "\\\n",
        "\\\r",
        "\\\t",
    ] {
        require(
            !text.contains(forbidden),
            &format!("unsupported selected-text construct: {forbidden:?}"),
        )?;
    }
    let trimmed = text.trim_matches([' ', '\t', '\r', '\n']);
    require(
        !trimmed
            .lines()
            .any(|line| line.trim_matches([' ', '\t', '\r']).is_empty()),
        "a step cannot flatten a paragraph boundary",
    )?;
    let normalized = trimmed
        .split([' ', '\t', '\r', '\n'])
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    require(!normalized.is_empty(), "empty canonical step")?;
    Ok(normalized)
}

fn decode(raw: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = flate2::bufread::GzDecoder::new(raw);
    let mut out = Vec::new();
    decoder
        .by_ref()
        .take(MAX_SOURCE + 1)
        .read_to_end(&mut out)
        .map_err(|e| format!("invalid gzip: {e}"))?;
    require(
        out.len() as u64 <= MAX_SOURCE,
        "source exceeds 16 MiB decoded limit",
    )?;
    require(
        decoder.into_inner().is_empty(),
        "gzip must have one member and no trailing bytes",
    )?;
    require(
        !out.contains(&0),
        "source contains NUL (tar and binary containers are unsupported)",
    )?;
    Ok(out)
}

fn boundaries(source: &[u8], recipe: &Recipe) -> Result<(), String> {
    require(
        matches!(
            recipe.environment.as_str(),
            "proposition" | "theorem" | "lemma"
        ),
        "unsupported environment",
    )?;
    require(
        !recipe.label.is_empty() && !recipe.label.contains(['{', '}', '\n', '\r']),
        "invalid source label",
    )?;
    let label = format!("\\label{{{}}}", recipe.label);
    unique(source, &label)?;
    let start_marker = format!("\\begin{{{}}}{label}", recipe.environment);
    let start = unique(source, &start_marker)? + start_marker.len();
    let end_marker = format!("\\end{{{}}}", recipe.environment);
    let rest = &source[start..];
    let end = rest
        .windows(end_marker.len())
        .position(|b| b == end_marker.as_bytes())
        .ok_or("missing statement end")?
        + start;
    require(
        recipe.statement.range == trim_range(source, start, end),
        "statement slice must cover the entire selected statement",
    )?;
    let mut p = end + end_marker.len();
    while source.get(p).is_some_and(u8::is_ascii_whitespace) {
        p += 1;
    }
    let begin_proof = b"\\begin{proof}";
    require(
        source[p..].starts_with(begin_proof),
        "proof must immediately follow statement",
    )?;
    p += begin_proof.len();
    let end_proof = b"\\end{proof}";
    let q = source[p..]
        .windows(end_proof.len())
        .position(|b| b == end_proof)
        .ok_or("missing proof end")?
        + p;
    require(!recipe.proof.is_empty(), "at least one proof step required")?;
    for (i, step) in recipe.proof.iter().enumerate() {
        let [s, e] = step.range;
        require(
            p <= s && s < e && e <= q,
            "unordered/overlapping/outside-proof step",
        )?;
        require(
            source[p..s].iter().all(u8::is_ascii_whitespace),
            "omitted proof text",
        )?;
        if i > 0 {
            require(
                source[p..s].iter().filter(|b| **b == b'\n').count() >= 2,
                "steps must be separated by an original paragraph boundary",
            )?;
        }
        require(
            trim_range(source, s, e) == [s, e],
            "step offsets must trim boundary whitespace",
        )?;
        p = e;
    }
    require(
        source[p..q].iter().all(u8::is_ascii_whitespace),
        "omitted proof tail",
    )?;
    // All context is kept, including byte encodings not present in the selected UTF-8 slices.
    let doc = unique(source, "\\begin{document}")?;
    let bib = unique(source, "\\begin{thebibliography}")?;
    let bib_end = unique(source, "\\end{thebibliography}")? + "\\end{thebibliography}".len();
    let mut roles = BTreeSet::new();
    for context in &recipe.context {
        checked_slice(source, &context.slice)?;
        require(
            roles.insert(context.role.as_str()),
            "duplicate context role",
        )?;
        match context.role.as_str() {
            "preamble" => require(context.slice.range == [0, doc], "incomplete preamble")?,
            "bibliography" => require(
                context.slice.range == [bib, bib_end],
                "incomplete bibliography",
            )?,
            "definitions" | "prerequisite" => {}
            _ => return Err("unknown context role".into()),
        }
    }
    require(
        roles == BTreeSet::from(["preamble", "definitions", "prerequisite", "bibliography"]),
        "missing required context role",
    )?;
    Ok(())
}

/// Calls both ratified validators and additionally preserves instance IDs, dials and order.
pub fn validate_child(child: &Manifest, parent: &Manifest, proof: &[u8]) -> Result<(), String> {
    child
        .check(FileBytes {
            informal: Some(proof),
            formal: None,
        })
        .map_err(|e| e.join("\n"))?;
    child.check_derived(parent).map_err(|e| e.join("\n"))?;
    for key in ["sources", "item", "licence", "status"] {
        require(
            child.config.get(key) == parent.config.get(key),
            "incompatible inherited config metadata",
        )?;
    }
    let mut restored = child
        .ground_truth
        .clone()
        .ok_or("missing child ground truth")?;
    let original = parent
        .ground_truth
        .as_ref()
        .ok_or("missing parent ground truth")?;
    require(
        restored.flaws.len() == original.flaws.len(),
        "flaw cardinality changed",
    )?;
    for (f, p) in restored.flaws.iter_mut().zip(&original.flaws) {
        f.location = p.location.clone();
    }
    require(
        &restored == original,
        "incompatible inherited flaw metadata",
    )
}

pub fn derive(
    parent_bytes: &[u8],
    raw: &[u8],
    recipe: &Recipe,
    commit: &str,
) -> Result<Product, String> {
    require(
        commit.len() == 40
            && commit
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "engine_commit must be a full lowercase Git commit",
    )?;
    require(
        recipe.method == METHOD && recipe.transformation == "ascii-whitespace/1",
        "unsupported method/transformation",
    )?;
    require(
        !recipe.title.is_empty()
            && !recipe.title.contains(['\r', '\n', '\t'])
            && !recipe.title.ends_with(' '),
        "invalid title",
    )?;
    require(
        content_hash(parent_bytes) == recipe.parent_manifest_sha256,
        "parent manifest hash mismatch",
    )?;
    let parent: Manifest =
        from_json_str(std::str::from_utf8(parent_bytes).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    parent
        .check(FileBytes {
            informal: Some(raw),
            formal: None,
        })
        .map_err(|e| e.join("\n"))?;
    require(
        parent.origin == Origin::GroundTruth
            && parent.artifact.kind == Kind::Informal
            && parent
                .artifact
                .files
                .informal
                .as_ref()
                .is_some_and(|f| f.format == FORMAT_RAW),
        "only admitted informal raw ground-truth parents are supported",
    )?;
    let context = decode(raw)?;
    require(
        content_hash(&context) == recipe.context_sha256,
        "decompressed context hash mismatch",
    )?;
    boundaries(&context, recipe)?;
    let mut steps = vec![normalize(checked_slice(&context, &recipe.statement)?)?];
    for spec in &recipe.proof {
        steps.push(normalize(checked_slice(&context, spec)?)?);
    }
    let mut proof = format!(
        "# {}\n\n**Theorem.** {}\n\n**Proof.**\n",
        recipe.title, steps[0]
    );
    for (i, text) in steps.iter().enumerate().skip(1) {
        proof.push_str(&format!("\n**Step {i}.** {text}\n"));
    }
    proof.push_str("\n∎\n");
    let proof = proof.into_bytes();
    let stats = parse_informal_md(&proof).map_err(|e| e.to_string())?;
    let mut gt = parent.ground_truth.clone().ok_or("missing ground truth")?;
    require(
        recipe.mappings.len() == gt.flaws.len(),
        "every flaw needs exactly one mapping",
    )?;
    let mut instances = BTreeSet::new();
    let mut mappings = Vec::new();
    for (flaw, mapping) in gt.flaws.iter_mut().zip(&recipe.mappings) {
        require(
            mapping.instance == flaw.instance && instances.insert(mapping.instance),
            "mapping instance/order mismatch",
        )?;
        let Location::Raw(raw_location) = &flaw.location else {
            return Err("parent location must be raw".into());
        };
        let text = stats
            .steps
            .get(usize::try_from(mapping.step).map_err(|e| e.to_string())?)
            .ok_or("mapping step out of bounds")?;
        let selected = span_text(text, mapping.span).ok_or("mapping span out of bounds")?;
        require(
            !selected.is_empty() && content_hash(selected.as_bytes()) == mapping.text_sha256,
            "mapping span is empty or has incorrect text hash",
        )?;
        let location = InformalLocation {
            artifact: InformalTag::Informal,
            step: mapping.step,
            span: mapping.span,
        };
        mappings.push(
            json!({"instance": flaw.instance, "raw_locator": raw_location.locator,
            "canonical": location, "text_sha256": mapping.text_sha256}),
        );
        flaw.location = Location::Informal(location);
    }
    let mut config = json!({
        "derived_from": parent.artifact_id, "method": METHOD, "recipe": recipe,
        "context": {"file": "context.tex", "storage": "external",
            "content_hash": content_hash(&context), "n_bytes": context.len()}
    });
    for key in ["sources", "item", "licence", "status"] {
        config[key] = parent.config[key].clone();
    }
    let hash = config_hash(&config).map_err(|e| e.to_string())?;
    let mut child = parent.clone();
    child.manifest_schema = MANIFEST_SCHEMA.into();
    child.artifact_id = artifact_id(GENERATOR, VERSION, &hash, None);
    child.generator = Generator {
        id: GENERATOR.into(),
        version: VERSION.into(),
        rng: RNG_NONE.into(),
        llm: None,
    };
    child.engine_commit = commit.into();
    child.seed = None;
    child.config = config;
    child.config_hash = hash;
    child.ground_truth = Some(gt);
    child.notes = Some(
        "ADR-0006 reviewed transcription; complete external context.tex is required. \
        Canonical syntax and inherited labels are not a mathematical verification. \
        This child is a representation of its raw parent, not an independent sample."
            .into(),
    );
    child.artifact.files.informal = Some(InformalFile {
        file: FILE_INFORMAL.into(),
        format: FORMAT_INFORMAL_MD.into(),
        storage: Storage::External,
        content_hash: content_hash(&proof),
        n_bytes: stats.n_bytes,
        n_tokens: Some(stats.n_tokens),
        n_steps: Some(stats.n_steps),
        n_lemmas: Some(stats.n_lemmas),
    });
    validate_child(&child, &parent, &proof)?;
    let source_map = json!({
        "method": METHOD, "artifact_id": child.artifact_id, "derived_from": parent.artifact_id,
        "source_hash": content_hash(raw), "context_hash": content_hash(&context),
        "offsets": "half-open decompressed source bytes; spans are Unicode scalars in marker-free steps",
        "statement": recipe.statement, "proof": recipe.proof, "context": recipe.context,
        "flaws": mappings
    });
    Ok(Product {
        manifest: child,
        proof,
        context,
        source_map,
    })
}

/// Preflight the complete output set; refuse changed existing bytes and never truncate a file.
/// Identical retry is permitted. A crash can leave incomplete files; these are conflicts,
/// never silently replaced. A set of complete files with missing siblings can be retried.
pub fn write_outputs(files: &[(&Path, &[u8])]) -> Result<(), String> {
    let mut paths = BTreeSet::new();
    for (path, bytes) in files {
        require(paths.insert(*path), "duplicate output path")?;
        if path.exists() {
            require(
                fs::read(path).map_err(|e| e.to_string())? == *bytes,
                &format!("same-ID/different-bytes conflict: {}", path.display()),
            )?;
        }
    }
    for (path, bytes) in files {
        fs::create_dir_all(path.parent().ok_or("output has no parent")?)
            .map_err(|e| e.to_string())?;
        match OpenOptions::new().write(true).create_new(true).open(path) {
            Ok(mut f) => f.write_all(bytes).map_err(|e| e.to_string())?,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => require(
                fs::read(path).map_err(|e| e.to_string())? == *bytes,
                "output changed after preflight",
            )?,
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(())
}
