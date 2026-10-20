//! Frozen-prefix inventory using the existing manifest validator, with explicit review overlays.
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use syndeom_manifest::{
    FileBytes, Manifest, Origin, ProofStatus, Storage, content_hash, from_json_str,
};

pub type Result<T> = std::result::Result<T, String>;
type Blobs = BTreeMap<String, Vec<u8>>;
const VERSION: &str = "s1-4-coverage/1";
const CONFIG: &str = "analysis/s1-4/config";
const CORE: &[(&str, &[u8])] = &[
    (
        "engine/manifest/examples/s1_4_coverage.rs",
        include_bytes!("../s1_4_coverage.rs"),
    ),
    (
        "engine/manifest/examples/support/coverage.rs",
        include_bytes!("coverage.rs"),
    ),
    (
        "engine/manifest/src/lib.rs",
        include_bytes!("../../src/lib.rs"),
    ),
];

pub fn json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn git(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output.stdout)
}

fn revision(repo: &Path, rev: &str) -> Result<()> {
    if rev.len() != 40
        || !rev
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("revision must be a full lowercase 40-hex commit".into());
    }
    let resolved = git(repo, &["rev-parse", &format!("{rev}^{{commit}}")])?;
    if String::from_utf8_lossy(&resolved).trim() != rev {
        return Err("revision is not a commit".into());
    }
    Ok(())
}

fn tracked(repo: &Path, rev: &str) -> Result<Vec<String>> {
    let bytes = git(repo, &["ls-tree", "-r", "--name-only", "-z", rev])?;
    bytes
        .split(|b| *b == 0)
        .filter(|b| !b.is_empty())
        .map(|b| String::from_utf8(b.to_vec()).map_err(|e| e.to_string()))
        .collect()
}

fn blob(repo: &Path, rev: &str, path: &str) -> Result<Vec<u8>> {
    git(repo, &["show", &format!("{rev}:{path}")])
}

fn corpus_path(path: &str) -> bool {
    path.starts_with("groundtruth/")
        || path.starts_with("analysis/s1-2/")
        || path.starts_with("analysis/s1-3/")
        || matches!(
            path,
            "PROGRAM/STATE.md"
                | "PROGRAM/HANDOFF.md"
                | "PROGRAM/SOURCES.md"
                | "PROGRAM/FLAWS.md"
                | "PROGRAM/SYNDEOM-PROGRAM.md"
                | "PROGRAM/DECISIONS/ADR-0000.md"
                | "PROGRAM/DECISIONS/ADR-0001.md"
                | "PROGRAM/DECISIONS/ADR-0003.md"
                | "PROGRAM/DECISIONS/ADR-0004.md"
                | "PROGRAM/DECISIONS/ADR-0006.md"
                | "PROGRAM/DECISIONS/ADR-0007.md"
        )
}

fn snapshot(repo: &Path, rev: &str) -> Result<Blobs> {
    revision(repo, rev)?;
    tracked(repo, rev)?
        .into_iter()
        .filter(|p| corpus_path(p))
        .map(|p| Ok((p.clone(), blob(repo, rev, &p)?)))
        .collect()
}

pub fn hashes(blobs: &Blobs) -> BTreeMap<String, String> {
    blobs
        .iter()
        .map(|(p, b)| (p.clone(), content_hash(b)))
        .collect()
}

pub fn verify_frozen(
    actual: &BTreeMap<String, String>,
    expected: &BTreeMap<String, String>,
) -> Result<()> {
    if actual != expected {
        return Err("frozen input set/hash mismatch (added, missing or changed input)".into());
    }
    Ok(())
}

fn text<'a>(blobs: &'a Blobs, path: &str) -> Result<&'a str> {
    std::str::from_utf8(
        blobs
            .get(path)
            .ok_or_else(|| format!("missing input {path}"))?,
    )
    .map_err(|e| e.to_string())
}

pub fn taxonomy(text: &str) -> Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    for line in text.lines().filter_map(|l| l.strip_prefix("## FLAW-")) {
        let (number, name) = line
            .split_once(" — ")
            .ok_or("unexpected taxonomy heading")?;
        if number.len() != 4 || !number.bytes().all(|b| b.is_ascii_digit()) {
            return Err("invalid taxonomy ID".into());
        }
        if result
            .insert(format!("FLAW-{number}"), name.to_string())
            .is_some()
        {
            return Err("duplicate taxonomy ID".into());
        }
    }
    if result.is_empty() {
        return Err("no taxonomy classes".into());
    }
    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    pub kind: String,
    pub reason: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Review {
    pub root_id: String,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dispositions {
    pub schema: String,
    pub pair_root: String,
    pub merge_evidence: String,
    pub reviews: Vec<Review>,
    pub scope_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bins {
    pub schema: String,
    pub population: String,
    pub edge_rule: String,
    pub role_rule: String,
    pub boundaries: BTreeMap<String, Vec<u64>>,
}

pub fn octave_edges(values: &[u64]) -> Result<Vec<u64>> {
    let min = *values
        .iter()
        .min()
        .ok_or("cannot freeze bins for an empty population")?;
    let max = *values.iter().max().unwrap();
    if min == 0 {
        return Err("zero-token population requires an explicit separate zero bucket".into());
    }
    let mut edges = vec![min];
    while *edges.last().unwrap() <= max {
        edges.push(
            edges
                .last()
                .unwrap()
                .checked_mul(2)
                .ok_or("octave overflow")?,
        );
    }
    Ok(edges)
}

pub fn bin(value: u64, edges: &[u64]) -> String {
    if value < edges[0] {
        return "underflow".into();
    }
    for w in edges.windows(2) {
        if value < w[1] {
            return format!("[{}, {})", w[0], w[1]);
        }
    }
    "overflow".into()
}

fn blank_bins(edges: &[u64]) -> BTreeMap<String, usize> {
    let mut result: BTreeMap<String, usize> = edges
        .windows(2)
        .map(|w| (format!("[{}, {})", w[0], w[1]), 0))
        .collect();
    result.insert("underflow".into(), 0);
    result.insert("overflow".into(), 0);
    result
}

#[derive(Debug, Clone, Serialize)]
pub struct FileCheck {
    pub modality: String,
    pub file: String,
    pub storage: Storage,
    pub format: String,
    pub sha256: String,
    pub current_verification: String,
    pub prior_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Representation {
    pub artifact_id: String,
    pub root_id: String,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub representation: String,
    pub manifest_path: String,
    pub record_path: String,
    pub files: Vec<FileCheck>,
    pub schema_id_validation: String,
    pub parent_validation: String,
    pub manifest: Manifest,
}

pub fn lineage(manifests: &BTreeMap<String, Manifest>) -> Result<BTreeMap<String, String>> {
    let mut roots = BTreeMap::new();
    for (id, m) in manifests {
        let mut seen = BTreeSet::new();
        let mut current = m;
        loop {
            if !seen.insert(current.artifact_id.clone()) {
                return Err(format!("lineage cycle at {id}"));
            }
            match current.config.get("derived_from") {
                None => {
                    roots.insert(id.clone(), current.artifact_id.clone());
                    break;
                }
                Some(value) => {
                    let p = value.as_str().ok_or("non-string parent")?;
                    current = manifests
                        .get(p)
                        .ok_or_else(|| format!("missing parent {p}"))?;
                }
            }
        }
    }
    for m in manifests.values() {
        if let Some(parent) = m.config.get("derived_from").and_then(Value::as_str) {
            let p = &manifests[parent];
            m.check_derived(p).map_err(|e| e.join("; "))?;
            if p.config.get("derived_from").is_some() {
                return Err("parent must be raw item of record".into());
            }
            for field in ["sources", "item", "licence", "status"] {
                if m.config.get(field) != p.config.get(field) {
                    return Err(format!("inconsistent parent {field}"));
                }
            }
            if m.toolchain != p.toolchain {
                return Err("inconsistent parent toolchain".into());
            }
        }
    }
    Ok(roots)
}

fn load_inventory(
    blobs: &Blobs,
    classes: &BTreeMap<String, String>,
) -> Result<Vec<Representation>> {
    let mut manifests = BTreeMap::new();
    let mut paths = BTreeMap::new();
    for (path, bytes) in blobs
        .iter()
        .filter(|(p, _)| p.starts_with("groundtruth/") && p.ends_with("/manifest.json"))
    {
        let m = from_json_str(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)
            .map_err(|e| format!("{path}: {e}"))?;
        if path != &format!("groundtruth/{}/manifest.json", m.artifact_id) {
            return Err("manifest ID/path mismatch".into());
        }
        validate_labels(&m, classes)?;
        if manifests.insert(m.artifact_id.clone(), m.clone()).is_some() {
            return Err("duplicate artifact ID".into());
        }
        paths.insert(m.artifact_id.clone(), path.clone());
    }
    let roots = lineage(&manifests)?;
    let mut output = Vec::new();
    for (id, m) in &manifests {
        let directory = format!("groundtruth/{id}");
        let record = format!("{directory}/RECORD.md");
        text(blobs, &record)?;
        let mut file_checks = Vec::new();
        let mut available = BTreeMap::new();
        let files = &m.artifact.files;
        let records = [
            files
                .informal
                .as_ref()
                .map(|f| ("informal", &f.file, f.storage, &f.format, &f.content_hash)),
            files
                .formal
                .as_ref()
                .map(|f| ("formal", &f.file, f.storage, &f.format, &f.content_hash)),
        ];
        for (modality, file, storage, format, hash) in records.into_iter().flatten() {
            if file.contains(['/', '\\']) || file == "." || file == ".." {
                return Err("unsafe file name".into());
            }
            let path = format!("{directory}/{file}");
            let bytes = blobs.get(&path);
            if storage == Storage::Committed && bytes.is_none() {
                return Err(format!("committed file missing: {path}"));
            }
            if let Some(b) = bytes {
                available.insert(modality, b.as_slice());
            }
            let mut evidence = vec![record.clone(), "PROGRAM/SOURCES.md".into()];
            if m.generator.id.starts_with("s1_pair") {
                evidence.push(
                    "analysis/s1-2/implementation/evidence/production/verification-receipt.json"
                        .into(),
                );
            }
            if m.generator.id == "s1_derive" || roots[id] == "s1_ingest-v0.1.0-00f28e252823" {
                evidence.push("analysis/s1-3/real-result.json".into());
            }
            file_checks.push(FileCheck { modality: modality.into(), file: file.clone(), storage, format: format.clone(), sha256: hash.clone(),
                current_verification: if bytes.is_some() { "verified against pinned committed bytes by Manifest::check" } else { "not performed: external/regenerable bytes absent from pinned tracked corpus; prior evidence is not a new byte check" }.into(), prior_evidence: evidence });
        }
        m.check(FileBytes {
            informal: available.get("informal").copied(),
            formal: available.get("formal").copied(),
        })
        .map_err(|e| format!("{id}: {}", e.join("; ")))?;
        let parent = m
            .config
            .get("derived_from")
            .and_then(Value::as_str)
            .map(str::to_string);
        let all_raw = file_checks.iter().all(|f| f.format == "raw/1");
        if all_raw != parent.is_none() {
            return Err("expected raw roots and canonical children".into());
        }
        output.push(Representation { artifact_id: id.clone(), root_id: roots[id].clone(), parent_id: parent.clone(),
            children: manifests.values().filter(|child| child.config.get("derived_from").and_then(Value::as_str) == Some(id)).map(|c| c.artifact_id.clone()).collect(),
            representation: if all_raw { "raw" } else { "canonical" }.into(), manifest_path: paths[id].clone(), record_path: record, files: file_checks,
            schema_id_validation: "passed existing typed parser and Manifest::check; absent file bytes skipped".into(),
            parent_validation: if parent.is_some() { "passed check_derived plus sources/item/licence/status/toolchain equality; root raw, acyclic" } else { "root item of record" }.into(), manifest: m.clone() });
    }
    Ok(output)
}

pub fn validate_labels(m: &Manifest, classes: &BTreeMap<String, String>) -> Result<()> {
    m.check(FileBytes::default()).map_err(|e| e.join("; "))?;
    if m.origin != Origin::GroundTruth {
        return Err("only real ground-truth manifests accepted".into());
    }
    let gt = m.ground_truth.as_ref().ok_or("ground truth missing")?;
    if gt.taxonomy_version != "0.1.0" {
        return Err("unsupported taxonomy version".into());
    }
    for f in &gt.flaws {
        if !classes.contains_key(&f.class) {
            return Err(format!("unknown class {}", f.class));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub struct Candidate {
    pub index: usize,
    pub arxiv_id: String,
    pub verdict: String,
    pub artifact_ids: Vec<String>,
    pub suggested_classes: Vec<String>,
    pub evidence_line: usize,
    pub reason: String,
}

pub fn harvest(log: &str) -> Result<Vec<Candidate>> {
    let mut rows = Vec::new();
    let mut joined = Vec::new();
    let mut pending: Option<(usize, String)> = None;
    for (line_index, line) in log.lines().enumerate() {
        let starts_row = line
            .split('|')
            .nth(1)
            .is_some_and(|s| s.trim().parse::<usize>().is_ok());
        if starts_row {
            if pending.is_some() {
                return Err("unterminated harvest row".into());
            }
            pending = Some((line_index, line.to_string()));
        } else if let Some((_, content)) = &mut pending {
            content.push(' ');
            content.push_str(line.trim());
        }
        if line.trim_end().ends_with('|')
            && let Some(row) = pending.take()
        {
            joined.push(row);
        }
    }
    if pending.is_some() {
        return Err("unterminated harvest row".into());
    }
    for (line_index, line) in joined {
        let fields: Vec<&str> = line.split('|').map(str::trim).collect();
        if fields.len() < 7 {
            continue;
        }
        let Ok(index) = fields[1].parse::<usize>() else {
            continue;
        };
        if index != rows.len() + 1 {
            return Err(
                "unsupported repeated/out-of-order harvest disposition; review before freezing"
                    .into(),
            );
        }
        let verdict_text = fields[4].trim_matches('*');
        let verdict = [
            "found-but-unclassifiable",
            "pending-truth",
            "admitted",
            "rejected",
            "escalated",
            "pending",
        ]
        .into_iter()
        .find(|v| verdict_text == *v || verdict_text.starts_with(&format!("{v} ×")))
        .ok_or("unknown harvest verdict")?;
        let reason = fields[5..fields.len() - 1].join(" | ");
        let ids: BTreeSet<String> = reason
            .split('`')
            .filter(|s| s.starts_with("s1_") && !s.contains(char::is_whitespace))
            .map(str::to_string)
            .collect();
        // No semantic inference: neither frozen unclassifiable row assigns a class.
        if verdict == "found-but-unclassifiable" && reason.contains("FLAW-") {
            return Err(
                "unclassifiable class attribution needs an explicit reviewed mapping".into(),
            );
        }
        rows.push(Candidate {
            index,
            arxiv_id: fields[2].trim_matches('`').into(),
            verdict: verdict.into(),
            artifact_ids: ids.into_iter().collect(),
            suggested_classes: Vec::new(),
            evidence_line: line_index + 1,
            reason,
        });
    }
    if rows.is_empty() {
        return Err("empty harvest".into());
    }
    Ok(rows)
}

#[derive(Debug, Clone, Serialize)]
pub struct Length {
    pub artifact_id: String,
    pub modality: String,
    pub role: String,
    pub n_tokens: u64,
    pub n_steps: Option<u64>,
    pub n_lines: Option<u64>,
    pub n_declarations: Option<u64>,
}

pub fn lengths(m: &Manifest) -> Vec<Length> {
    let mut out = Vec::new();
    if let Some(f) = &m.artifact.files.informal
        && f.format == "syndeom-informal-md/1"
    {
        out.push(Length {
            artifact_id: m.artifact_id.clone(),
            modality: "informal".into(),
            role: if f.n_steps == Some(0) {
                "statement-only"
            } else {
                "proof-bearing"
            }
            .into(),
            n_tokens: f.n_tokens.unwrap(),
            n_steps: f.n_steps,
            n_lines: None,
            n_declarations: None,
        });
    }
    if let Some(f) = &m.artifact.files.formal
        && f.format == "syndeom-lean/1"
    {
        out.push(Length {
            artifact_id: m.artifact_id.clone(),
            modality: "formal".into(),
            role: if m.config.get("method").and_then(Value::as_str)
                == Some("reviewed-statement-mathlib-alias/1")
            {
                "theorem-alias"
            } else {
                "formal-file"
            }
            .into(),
            n_tokens: f.n_tokens.unwrap(),
            n_steps: None,
            n_lines: f.n_lines,
            n_declarations: f.n_declarations,
        });
    }
    out
}

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub root_id: String,
    pub representations: Vec<String>,
    pub sources: Vec<String>,
    pub documentary_family: String,
    pub admission: String,
    pub evidence: Vec<String>,
    pub issues: Vec<Issue>,
    pub unflagged_documented_subset: bool,
    pub truth_dimension_eligible: bool,
    pub scoring_ready: bool,
    pub development_exposure: String,
    pub lengths: Vec<Length>,
    pub root_manifest: Manifest,
}

pub fn qualify(issues: &[Issue], truth: Option<bool>) -> (bool, bool) {
    let unflagged = issues.is_empty();
    let truth_ok = truth.is_some()
        && !issues.iter().any(|i| {
            matches!(
                i.kind.as_str(),
                "promotion-hold" | "truth-review" | "pending-review"
            )
        });
    (unflagged, truth_ok)
}

fn items(
    reps: &[Representation],
    candidates: &[Candidate],
    dispositions: &Dispositions,
) -> Result<Vec<Item>> {
    if dispositions.schema != VERSION {
        return Err("unsupported dispositions schema".into());
    }
    let mut reviews = BTreeMap::new();
    for review in &dispositions.reviews {
        if reviews
            .insert(review.root_id.clone(), review.issues.clone())
            .is_some()
        {
            return Err("duplicate review root".into());
        }
        for issue in &review.issues {
            if ![
                "promotion-hold",
                "truth-review",
                "severity-review",
                "pending-review",
            ]
            .contains(&issue.kind.as_str())
                || issue.evidence.is_empty()
            {
                return Err("unknown or unsupported review issue".into());
            }
        }
    }
    let mut seen_admissions = BTreeSet::new();
    let mut out = Vec::new();
    for r in reps.iter().filter(|r| r.parent_id.is_none()) {
        let m = &r.manifest;
        let gt = m.ground_truth.as_ref().unwrap();
        let candidate = candidates
            .iter()
            .find(|c| c.verdict == "admitted" && c.artifact_ids.contains(&r.root_id));
        let admission = if let Some(c) = candidate {
            seen_admissions.insert(r.root_id.clone());
            format!(
                "historically admitted, Route A candidate {} ({})",
                c.index, c.arxiv_id
            )
        } else if r.root_id == dispositions.pair_root {
            "owner merged PR 4 admission packet; documented checks retained; no separate GitHub reviewer approval and no PROVEN promotion".into()
        } else {
            return Err(format!("no admission evidence for {}", r.root_id));
        };
        let issues = reviews.remove(&r.root_id).unwrap_or_default();
        let (unflagged, truth_eligible) = qualify(&issues, gt.statement_status.value);
        let sources: Vec<String> =
            serde_json::from_value(m.config["sources"].clone()).map_err(|e| e.to_string())?;
        let family = sources.join("+");
        let related: Vec<&Representation> =
            reps.iter().filter(|x| x.root_id == r.root_id).collect();
        let mut evidence = vec![r.record_path.clone(), "PROGRAM/SOURCES.md".into()];
        if let Some(c) = candidate {
            evidence.push(format!("groundtruth/HARVEST-LOG.md#L{}", c.evidence_line));
        } else {
            evidence.push(dispositions.merge_evidence.clone());
            evidence.push("analysis/s1-2/implementation/VERIFICATION.md".into());
        }
        out.push(Item { root_id: r.root_id.clone(), representations: related.iter().map(|r| r.artifact_id.clone()).collect(), sources, documentary_family: family, admission, evidence, issues,
            unflagged_documented_subset: unflagged, truth_dimension_eligible: truth_eligible, scoring_ready: false,
            development_exposure: if r.root_id == dispositions.pair_root || related.len() > 1 || r.root_id == "s1_ingest-v0.1.0-63e23a952084" { "known development/source-review exposure in repository session records and task context; not blinded" } else { "known harvest/executor review; additional chat/training exposure unknown, never classified as unseen" }.into(),
            lengths: related.iter().flat_map(|r| lengths(&r.manifest)).collect(), root_manifest: m.clone() });
    }
    if !reviews.is_empty() {
        return Err("review references unknown/non-root item".into());
    }
    let admitted: BTreeSet<String> = candidates
        .iter()
        .filter(|c| c.verdict == "admitted")
        .flat_map(|c| c.artifact_ids.iter().cloned())
        .collect();
    if seen_admissions != admitted {
        return Err("harvest admissions do not match corpus roots".into());
    }
    if !out.iter().any(|i| i.root_id == dispositions.pair_root) {
        return Err("merged pair root missing".into());
    }
    Ok(out)
}

fn bump(map: &mut BTreeMap<String, usize>, key: String) {
    *map.entry(key).or_default() += 1;
}
fn enum_name<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

#[derive(Debug, Serialize)]
pub struct CoverageRow {
    pub class: String,
    pub name: String,
    pub items: usize,
    pub flaw_instances: usize,
    pub severity_items: BTreeMap<String, usize>,
    pub severity_instances: BTreeMap<String, usize>,
    pub lean_catchable_items: BTreeMap<String, usize>,
    pub lean_catchable_instances: BTreeMap<String, usize>,
    pub truth: BTreeMap<String, usize>,
    pub truth_eligible: BTreeMap<String, usize>,
    pub proof_status: BTreeMap<String, usize>,
    pub correct_controls: usize,
    pub positive_length_informal_controls: usize,
    pub statement_only_controls: usize,
    pub canonical_roots: usize,
    pub missing_canonical_length: usize,
    pub informal_bins: BTreeMap<String, usize>,
    pub informal_proof_bins: BTreeMap<String, usize>,
    pub informal_statement_bins: BTreeMap<String, usize>,
    pub formal_bins: BTreeMap<String, usize>,
    pub steps: Value,
    pub proof_steps: Value,
    pub unclassifiable_attributed: usize,
}

fn zeros(names: &[&str]) -> BTreeMap<String, usize> {
    names.iter().map(|n| (n.to_string(), 0)).collect()
}
fn step_summary(mut values: Vec<u64>, denominator: usize) -> Value {
    values.sort_unstable();
    let n = values.len();
    let median = if n == 0 {
        None
    } else {
        Some((values[(n - 1) / 2] as f64 + values[n / 2] as f64) / 2.0)
    };
    json!({"n": n, "missing": denominator-n, "min":values.first(), "median":median, "max":values.last()})
}

pub fn coverage(
    items: &[Item],
    classes: &BTreeMap<String, String>,
    bins: &Bins,
) -> Vec<CoverageRow> {
    let mut rows = Vec::new();
    for (class, name) in classes
        .iter()
        .map(|(c, n)| (c.as_str(), n.as_str()))
        .chain(std::iter::once(("TOTAL", "Unique logical items")))
    {
        let selected: Vec<&Item> = items
            .iter()
            .filter(|i| {
                class == "TOTAL"
                    || i.root_manifest
                        .ground_truth
                        .as_ref()
                        .unwrap()
                        .flaws
                        .iter()
                        .any(|f| f.class == class)
            })
            .collect();
        let mut row = CoverageRow {
            class: class.into(),
            name: name.into(),
            items: selected.len(),
            flaw_instances: 0,
            severity_items: zeros(&["cosmetic", "repairable", "gap", "fatal"]),
            severity_instances: zeros(&["cosmetic", "repairable", "gap", "fatal"]),
            lean_catchable_items: zeros(&["YES-KERNEL", "YES-TOOLING", "NO", "N/A"]),
            lean_catchable_instances: zeros(&["YES-KERNEL", "YES-TOOLING", "NO", "N/A"]),
            truth: zeros(&["true", "false", "undetermined"]),
            truth_eligible: zeros(&["true", "false", "excluded"]),
            proof_status: zeros(&["valid", "invalid"]),
            correct_controls: 0,
            positive_length_informal_controls: 0,
            statement_only_controls: 0,
            canonical_roots: 0,
            missing_canonical_length: 0,
            informal_bins: blank_bins(&bins.boundaries["informal"]),
            informal_proof_bins: blank_bins(&bins.boundaries["informal"]),
            informal_statement_bins: blank_bins(&bins.boundaries["informal"]),
            formal_bins: blank_bins(&bins.boundaries["formal"]),
            steps: Value::Null,
            proof_steps: Value::Null,
            unclassifiable_attributed: 0,
        };
        let mut steps = Vec::new();
        let mut proof_steps = Vec::new();
        for item in &selected {
            let gt = item.root_manifest.ground_truth.as_ref().unwrap();
            let flaws: Vec<_> = gt
                .flaws
                .iter()
                .filter(|f| class == "TOTAL" || f.class == class)
                .collect();
            row.flaw_instances += flaws.len();
            let mut severities = BTreeSet::new();
            let mut catchabilities = BTreeSet::new();
            for f in flaws {
                let s = enum_name(&f.severity);
                let c = enum_name(&f.lean_catchable);
                severities.insert(s.clone());
                catchabilities.insert(c.clone());
                bump(&mut row.severity_instances, s);
                bump(&mut row.lean_catchable_instances, c);
            }
            for s in severities {
                bump(&mut row.severity_items, s);
            }
            for c in catchabilities {
                bump(&mut row.lean_catchable_items, c);
            }
            let truth = gt
                .statement_status
                .value
                .map_or("undetermined", |b| if b { "true" } else { "false" });
            bump(&mut row.truth, truth.into());
            bump(
                &mut row.truth_eligible,
                if item.truth_dimension_eligible {
                    truth
                } else {
                    "excluded"
                }
                .into(),
            );
            bump(&mut row.proof_status, enum_name(&gt.proof_status));
            if gt.correct && gt.proof_status == ProofStatus::Valid {
                row.correct_controls += 1;
            }
            if item.lengths.is_empty() {
                row.missing_canonical_length += 1;
            } else {
                row.canonical_roots += 1;
            }
            let informal: Vec<_> = item
                .lengths
                .iter()
                .filter(|l| l.modality == "informal")
                .collect();
            // The frozen corpus has one canonical file per modality/root. Refuse ambiguity in build().
            for l in &item.lengths {
                let b = bin(l.n_tokens, &bins.boundaries[&l.modality]);
                if l.modality == "informal" {
                    bump(&mut row.informal_bins, b.clone());
                    if l.role == "statement-only" {
                        bump(&mut row.informal_statement_bins, b);
                    } else {
                        bump(&mut row.informal_proof_bins, b);
                    }
                } else {
                    bump(&mut row.formal_bins, b);
                }
            }
            if let Some(l) = informal.first() {
                let n = l.n_steps.unwrap();
                steps.push(n);
                if n > 0 {
                    proof_steps.push(n);
                    if gt.correct {
                        row.positive_length_informal_controls += 1;
                    }
                } else if gt.correct {
                    row.statement_only_controls += 1;
                }
            }
        }
        row.steps = step_summary(steps, selected.len());
        row.proof_steps = step_summary(proof_steps, selected.len());
        rows.push(row);
    }
    rows
}

fn source_entries(register: &str, source_ids: &BTreeSet<String>) -> Result<Value> {
    let mut entries = BTreeMap::new();
    for chunk in register.split("\n## ").skip(1) {
        let Some(id) = chunk.split_whitespace().next() else {
            continue;
        };
        if !source_ids.contains(id) {
            continue;
        }
        let state = chunk
            .lines()
            .find_map(|l| l.strip_prefix("- **State:** "))
            .ok_or_else(|| format!("no source state for {id}"))?;
        entries.insert(id.to_string(), json!({"recorded_state":state,"entry_text":format!("## {chunk}"),"evidence":"PROGRAM/SOURCES.md","fresh_primary_retrieval_in_this_report":false}));
    }
    if entries.len() != source_ids.len() {
        return Err("source IDs missing from register".into());
    }
    Ok(json!(entries))
}

fn validate_bins(bins: &Bins, reps: &[Representation]) -> Result<()> {
    if bins.schema != VERSION
        || bins.edge_rule
            != "lower-inclusive upper-exclusive; retain empty bins; explicit underflow/overflow"
    {
        return Err("unsupported bin contract".into());
    }
    if bins
        .boundaries
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>()
        != ["formal", "informal"]
    {
        return Err("unexpected bin modalities".into());
    }
    for (modality, edges) in &bins.boundaries {
        if edges.len() < 2
            || edges[0] == 0
            || edges.windows(2).any(|w| w[0].checked_mul(2) != Some(w[1]))
        {
            return Err("bins must be fixed octaves".into());
        }
        let values: Vec<_> = reps
            .iter()
            .flat_map(|r| lengths(&r.manifest))
            .filter(|l| &l.modality == modality)
            .map(|l| l.n_tokens)
            .collect();
        if &octave_edges(&values)? != edges {
            return Err("frozen bins do not match the first-write pinned population".into());
        }
    }
    Ok(())
}

pub fn output_dir(out: &Path, files: BTreeMap<String, Vec<u8>>) -> Result<()> {
    fs::create_dir(out).map_err(|e| format!("new output directory required: {e}"))?;
    for (name, bytes) in files {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out.join(name))
            .map_err(|e| e.to_string())?;
        f.write_all(&bytes).map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn cli(args: Vec<String>) -> Result<()> {
    let usage = "from engine/: cargo run --locked -p syndeom-manifest --example s1_4_coverage -- freeze <repo> <corpus-commit> <new-dir> | report <repo> <corpus-commit> <tool-commit> <new-dir>";
    if args.is_empty() {
        return Err(usage.into());
    }
    match args[0].as_str() {
        "freeze" if args.len() == 4 => freeze(Path::new(&args[1]), &args[2], Path::new(&args[3])),
        "report" if args.len() == 5 => {
            report(Path::new(&args[1]), &args[2], &args[3], Path::new(&args[4]))
        }
        _ => Err(usage.into()),
    }
}

fn freeze(repo: &Path, corpus: &str, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("output already exists".into());
    }
    let blobs = snapshot(repo, corpus)?;
    let reps = load_inventory(&blobs, &taxonomy(text(&blobs, "PROGRAM/FLAWS.md")?)?)?;
    let mut boundaries = BTreeMap::new();
    for modality in ["informal", "formal"] {
        let values: Vec<u64> = reps
            .iter()
            .flat_map(|r| lengths(&r.manifest))
            .filter(|l| l.modality == modality)
            .map(|l| l.n_tokens)
            .collect();
        boundaries.insert(modality.into(), octave_edges(&values)?);
    }
    let bins=Bins {schema:VERSION.into(),population:format!("all canonical files at {corpus}, grouped by modality; includes statement-only positive control; one representation per root/modality"),edge_rule:"lower-inclusive upper-exclusive; retain empty bins; explicit underflow/overflow".into(),role_rule:"informal n_steps=0 is statement-only, >0 proof-bearing; formal reviewed-statement-mathlib-alias/1 is theorem-alias. Shared informal edges, separately counted roles; no pooling across modalities.".into(),boundaries};
    output_dir(
        out,
        BTreeMap::from([
            (
                "corpus-inputs.json".into(),
                json_bytes(
                    &json!({"schema":VERSION,"corpus_revision":corpus,"sha256":hashes(&blobs)}),
                )?,
            ),
            ("bins.json".into(), json_bytes(&bins)?),
        ]),
    )
}

fn tool_inputs(repo: &Path, tool: &str) -> Result<BTreeMap<String, String>> {
    revision(repo, tool)?;
    let names = String::from_utf8(blob(repo, tool, "analysis/s1-4/tool-inputs.txt")?)
        .map_err(|e| e.to_string())?;
    let mut expected = BTreeMap::new();
    let mut actual = BTreeMap::new();
    for name in names.lines() {
        if name.is_empty() || name.contains("..") || name.contains('\\') || name.starts_with('/') {
            return Err("unsafe tool input".into());
        }
        if expected
            .insert(name.to_string(), content_hash(&blob(repo, tool, name)?))
            .is_some()
        {
            return Err("duplicate tool input".into());
        }
        actual.insert(
            name.to_string(),
            content_hash(&fs::read(repo.join(name)).map_err(|e| format!("{name}: {e}"))?),
        );
    }
    verify_frozen(&actual, &expected)?;
    if !actual.contains_key("analysis/s1-4/tool-inputs.txt") {
        return Err("tool closure must include itself".into());
    }
    for (p, b) in CORE {
        if actual.get(*p) != Some(&content_hash(b)) {
            return Err(format!(
                "compiled source differs from declared tool revision: {p}"
            ));
        }
    }
    let listed = git(
        repo,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
            "--",
            "engine/manifest/examples",
            "analysis/s1-4/config",
        ],
    )?;
    for path in listed.split(|b| *b == 0).filter(|b| !b.is_empty()) {
        if !actual.contains_key(std::str::from_utf8(path).map_err(|e| e.to_string())?) {
            return Err("unexpected source/config input outside tool closure".into());
        }
    }
    Ok(actual)
}

fn pair_preservation(repo: &Path, corpus: &str) -> Result<Value> {
    let list = blob(repo, corpus, "engine/pair/input-files.txt")?;
    let mut inputs = BTreeMap::new();
    for name in std::str::from_utf8(&list)
        .map_err(|e| e.to_string())?
        .lines()
    {
        let original = blob(repo, corpus, name)?;
        if original != fs::read(repo.join(name)).map_err(|e| e.to_string())? {
            return Err(format!("historical pair input changed: {name}"));
        }
        inputs.insert(name.to_string(), content_hash(&original));
    }
    Ok(
        json!({"baseline":corpus,"unchanged_inputs":inputs,"result":"byte-for-byte preserved; build/replay execution has a separate receipt"}),
    )
}

fn report(repo: &Path, corpus: &str, tool: &str, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("output already exists".into());
    }
    let tool_hashes = tool_inputs(repo, tool)?;
    let blobs = snapshot(repo, corpus)?;
    let expected: Value = serde_json::from_slice(
        &fs::read(repo.join(format!("{CONFIG}/corpus-inputs.json"))).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if expected["schema"] != VERSION || expected["corpus_revision"] != corpus {
        return Err("wrong frozen corpus revision".into());
    }
    let expected_hashes =
        serde_json::from_value(expected["sha256"].clone()).map_err(|e| e.to_string())?;
    verify_frozen(&hashes(&blobs), &expected_hashes)?;
    let classes = taxonomy(text(&blobs, "PROGRAM/FLAWS.md")?)?;
    let reps = load_inventory(&blobs, &classes)?;
    let candidates = harvest(text(&blobs, "groundtruth/HARVEST-LOG.md")?)?;
    let bins: Bins = serde_json::from_slice(
        &fs::read(repo.join(format!("{CONFIG}/bins.json"))).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    validate_bins(&bins, &reps)?;
    let dispositions: Dispositions = serde_json::from_slice(
        &fs::read(repo.join(format!("{CONFIG}/dispositions.json"))).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    for review in &dispositions.reviews {
        for issue in &review.issues {
            for reference in &issue.evidence {
                let path = reference.split('#').next().unwrap();
                if !blobs.contains_key(path) {
                    return Err(format!(
                        "review evidence missing from pinned inputs: {reference}"
                    ));
                }
            }
        }
    }
    let merge: Value = serde_json::from_slice(
        &fs::read(repo.join(&dispositions.merge_evidence)).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    if merge["state"] != "MERGED"
        || merge["mergeCommit"]["oid"] != corpus
        || merge["number"] != 4
        || merge["mergedBy"]["login"] != "Quigles1337"
        || !merge["reviews"].as_array().is_some_and(Vec::is_empty)
    {
        return Err("merge evidence differs from reviewed admission disposition".into());
    }
    let items = items(&reps, &candidates, &dispositions)?;
    for item in &items {
        let modalities: Vec<_> = item.lengths.iter().map(|l| &l.modality).collect();
        if modalities.len() != modalities.iter().collect::<BTreeSet<_>>().len() {
            return Err("multiple canonical representations in one modality require an explicit selection policy".into());
        }
    }
    let rows = coverage(&items, &classes, &bins);
    let unflagged: Vec<_> = items
        .iter()
        .filter(|i| i.unflagged_documented_subset)
        .cloned()
        .collect();
    let qualified = coverage(&unflagged, &classes, &bins);
    let sources: BTreeSet<String> = items
        .iter()
        .flat_map(|i| i.sources.iter().cloned())
        .collect();
    let source_records = source_entries(text(&blobs, "PROGRAM/SOURCES.md")?, &sources)?;
    let mut families: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut raw_hashes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut shared_sources: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for i in &items {
        families
            .entry(i.documentary_family.clone())
            .or_default()
            .push(i.root_id.clone());
        for s in &i.sources {
            shared_sources
                .entry(s.clone())
                .or_default()
                .push(i.root_id.clone());
        }
        let m = &i.root_manifest;
        for hash in [
            m.artifact.files.informal.as_ref().map(|f| &f.content_hash),
            m.artifact.files.formal.as_ref().map(|f| &f.content_hash),
        ]
        .into_iter()
        .flatten()
        {
            raw_hashes
                .entry(hash.clone())
                .or_default()
                .push(i.root_id.clone());
        }
    }
    let mut candidate_counts = zeros(&[
        "admitted",
        "rejected",
        "found-but-unclassifiable",
        "pending",
        "pending-truth",
        "escalated",
    ]);
    for c in &candidates {
        bump(&mut candidate_counts, c.verdict.clone());
    }
    let zero: Vec<_> = rows
        .iter()
        .filter(|r| r.class != "TOTAL" && r.items == 0)
        .map(|r| json!({"class":r.class,"name":r.name}))
        .collect();
    let undetermined: Vec<_> = items
        .iter()
        .filter(|i| {
            i.root_manifest
                .ground_truth
                .as_ref()
                .unwrap()
                .statement_status
                .value
                .is_none()
        })
        .map(|i| &i.root_id)
        .collect();
    let mut issue_items: BTreeMap<String, Vec<String>> = [
        "promotion-hold",
        "truth-review",
        "severity-review",
        "pending-review",
    ]
    .into_iter()
    .map(|s| (s.to_string(), Vec::new()))
    .collect();
    for i in &items {
        for issue in &i.issues {
            issue_items
                .entry(issue.kind.clone())
                .or_default()
                .push(i.root_id.clone());
        }
    }
    let report_json = json!({"schema":VERSION,"corpus_revision":corpus,"tool_revision":tool,"taxonomy_version":"0.1.0","battery_version":null,"seed":null,"uncertainty":"descriptive census of pinned prefix; no sampling intervals or numeric power calculated",
        "representations":reps.len(),"roots":items.len(),"canonical_representations":reps.iter().filter(|r|r.parent_id.is_some()).count(),"source_id_array_groups":families.len(),
        "archive_coverage":rows,"unflagged_documented_subset_coverage":qualified,"subset_rule":"exclude roots with promotion-hold, truth-review, severity-review or pending-review; historical admission and labels unchanged. Not scoring readiness or a new admission policy.",
        "bins":bins,"zero_coverage":zero,"candidate_counts":candidate_counts,"harvest_candidates":candidates.len(),"prefix_through_arxiv_id":candidates.last().unwrap().arxiv_id,
        "undetermined_truth_items":undetermined,"issues_by_kind":issue_items,"scope_notes":dispositions.scope_notes,
        "qualification_counts":{"pending_item_review_union":items.iter().filter(|i|!i.issues.is_empty()).count(),"escalated_item_union":items.iter().filter(|i|i.issues.iter().any(|e|e.kind=="truth-review"||e.kind=="severity-review")).count(),"promotion_holds":items.iter().filter(|i|i.issues.iter().any(|e|e.kind=="promotion-hold")).count(),"unflagged_documented_subset":unflagged.len()},
        "interim_power":"Every real-item cell is expected underpowered for the equivalence test until S3 derives N_min and freezes the protocol; S4 plans recall against documented errors and a false-positive upper bound on correct proofs, not a fooling length."});
    let inventory = json!({"schema":VERSION,"representations":reps,"items":items,"candidates":candidates,"source_records":source_records,"documentary_families":families,"shared_source_ids":shared_sources,"raw_hash_groups":raw_hashes,
        "dependence_note":"Exact source-ID array groups, shared sources, raw hashes and lineage are observable dependencies, not independence certificates or effective N. Common theorem/author origins remain review obligations; Judson/Mathlib share Euclid's theorem."});
    let provenance = json!({"schema":VERSION,"corpus_revision":corpus,"tool_revision":tool,"tool_version":VERSION,"corpus_inputs_sha256":hashes(&blobs),"tool_inputs_sha256":tool_hashes,"pair_input_preservation":pair_preservation(repo,corpus)?,
        "byte_check_scope":"Pinned tracked corpus bytes only. External bytes not fetched for this inventory; retained source/producer evidence is linked separately. No Lean execution by this reporting tool.","git_tree":String::from_utf8_lossy(&git(repo,&["rev-parse",&format!("{corpus}^{{tree}}")])?).trim(),
        "command_from_engine":format!("cargo run --locked -p syndeom-manifest --example s1_4_coverage -- report <repo> {corpus} {tool} <new-output-directory>"),"volatile_run_data":"separate execution receipts; excluded from deterministic report artifacts"});
    let markdown = render(&report_json, &inventory)?;
    output_dir(
        out,
        BTreeMap::from([
            ("inventory.json".into(), json_bytes(&inventory)?),
            ("coverage.json".into(), json_bytes(&report_json)?),
            ("provenance.json".into(), json_bytes(&provenance)?),
            ("report.md".into(), markdown.into_bytes()),
        ]),
    )
}

fn compact_counts(value: &Value, keys: &[&str]) -> String {
    keys.iter()
        .map(|k| value[*k].to_string())
        .collect::<Vec<_>>()
        .join(" / ")
}

fn render(report: &Value, inventory: &Value) -> Result<String> {
    let mut out = format!(
        "# S1.4 — Corpus coverage and interim power\n\nCorpus `{}` · reporting implementation `{}`.\n\n**{} representations; {} logical items; {} canonical children.** Prefix through arXiv id {}.\n\nThis is an inventory of recorded source labels and reviewed qualifications, not a new mathematical verdict.\n\n",
        report["corpus_revision"].as_str().unwrap(),
        report["tool_revision"].as_str().unwrap(),
        report["representations"],
        report["roots"],
        report["canonical_representations"],
        report["prefix_through_arxiv_id"].as_str().unwrap()
    );
    out.push_str("## Counting contract\n\nA raw parent and its canonical child count once. Repeated instances of a class count once in that class's item column; instance totals remain separate. Items can appear in multiple class rows and multiple severity/catchability categories. TOTAL is the unique union, not the sum of rows. Correct controls with no flaw remain in TOTAL. True statements with invalid offered proofs are not correct controls. YES-KERNEL is a classification, not a claim that every item was run through Lean.\n\nThe historical table includes the explicit promotion hold and all older review questions. The unflagged subset excludes roots with any recorded hold, truth review, severity review or pending review; it neither rewrites admission nor certifies independent review or scoring readiness.\n\n");
    for (key, title) in [
        ("archive_coverage", "Historical admitted inventory"),
        (
            "unflagged_documented_subset_coverage",
            "Unflagged documented subset",
        ),
    ] {
        out.push_str(&format!("## {title}\n\nSeverity: cosmetic / repairable / gap / fatal. Catchability: YES-KERNEL / YES-TOOLING / NO / N/A. Truth: true / false / undetermined. Length bins and counts are canonical informal only; formal counts follow separately.\n\n| Class | Items | Instances | Severity items | Catchability items | Truth | Informal bins¹ | Steps n; min / median / max | Missing canonical | Unclassifiable attributed |\n|---|---:|---:|---|---|---|---|---|---:|---:|\n"));
        for r in report[key].as_array().ok_or("missing coverage")? {
            let steps = &r["steps"];
            let bins = report["bins"]["boundaries"]["informal"]
                .as_array()
                .unwrap()
                .windows(2)
                .map(|w| {
                    let key = format!("[{}, {})", w[0], w[1]);
                    format!("{key}: {}", r["informal_bins"][&key])
                })
                .collect::<Vec<_>>()
                .join("; ");
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {}; {} / {} / {} | {} | {} |\n",
                r["class"].as_str().unwrap(),
                r["items"],
                r["flaw_instances"],
                compact_counts(
                    &r["severity_items"],
                    &["cosmetic", "repairable", "gap", "fatal"]
                ),
                compact_counts(
                    &r["lean_catchable_items"],
                    &["YES-KERNEL", "YES-TOOLING", "NO", "N/A"]
                ),
                compact_counts(&r["truth"], &["true", "false", "undetermined"]),
                bins,
                steps["n"],
                steps["min"],
                steps["median"],
                steps["max"],
                r["missing_canonical_length"],
                r["unclassifiable_attributed"]
            ));
        }
        out.push_str("\n¹ Empty bins retained; boundaries are lower-inclusive and upper-exclusive. Explicit underflow/overflow and role-separated bins, severity/catchability instance counts, truth-eligible denominators, and missing step counts are in coverage.json. `null` means unavailable, never zero.\n\n");
    }
    out.push_str("## Zero coverage and interim power\n\nS4 cannot validate these classes on this prefix:\n\n");
    for z in report["zero_coverage"].as_array().unwrap() {
        out.push_str(&format!(
            "- **{} — {}**\n",
            z["class"].as_str().unwrap(),
            z["name"].as_str().unwrap()
        ));
    }
    out.push_str(&format!(
        "\n{}\n\n",
        report["interim_power"].as_str().unwrap()
    ));
    out.push_str("This is an interim scope statement, not a numerical power calculation. ADR-0004 requires the 95% bootstrap AUC interval wholly within [0.45, 0.55], with N_min derived for 90% power at true AUC 0.5. S3 must confirm the analytic starting value using the actual bootstrap, ties and dependence rules, including band-edge error and interval coverage, before curves. Raw item or source-family counts are not effective N. No detector scores, AUC, measured false positives, N_min, or fooling length are inferred here.\n\n");
    out.push_str("## Canonical length, role and controls\n\n| Root | Canonical representation | Modality / role | Tokens | Steps | Lines / declarations |\n|---|---|---|---:|---:|---|\n");
    for i in inventory["items"].as_array().unwrap() {
        for l in i["lengths"].as_array().unwrap() {
            out.push_str(&format!(
                "| {} | {} | {} / {} | {} | {} | {} / {} |\n",
                i["root_id"].as_str().unwrap(),
                l["artifact_id"].as_str().unwrap(),
                l["modality"].as_str().unwrap(),
                l["role"].as_str().unwrap(),
                l["n_tokens"],
                l["n_steps"],
                l["n_lines"],
                l["n_declarations"]
            ));
        }
    }
    let total = report["archive_coverage"]
        .as_array()
        .unwrap()
        .last()
        .unwrap();
    out.push_str(&format!("\n{} roots lack a canonical child/length. Correct controls: {}; positive-length informal proof controls: {}; statement-only controls: {}. Formal alias counts describe the wrapper, not the transitive Mathlib proof. Informal and formal tokens are never pooled. These sparse length-bearing representations, including the zero-step statement selection, cannot establish proof-reasoning or semantic-fidelity reliability. Only committed bytes are checked anew here; external canonical counts have linked prior generation evidence.\n\n",total["missing_canonical_length"],total["correct_controls"],total["positive_length_informal_controls"],total["statement_only_controls"]));
    out.push_str(&format!("Stored statement truth (true / false / undetermined): {}. Offered-proof validity (valid / invalid): {}. Qualified truth cells (true / false / excluded): {}. Proof-bearing informal step summary alone: `{}`. This last summary excludes statement-only selections without concealing their genuine zero above.\n\n",compact_counts(&total["truth"],&["true","false","undetermined"]),compact_counts(&total["proof_status"],&["valid","invalid"]),compact_counts(&total["truth_eligible"],&["true","false","excluded"]),total["proof_steps"]));
    out.push_str("## Qualifications and unresolved work\n\nHarvest candidates and corpus items use different denominators. Candidate dispositions (parsed from each logged row):\n\n");
    for (v, n) in report["candidate_counts"].as_object().unwrap() {
        out.push_str(&format!("- {v}: {n}\n"));
    }
    let unclassifiable = inventory["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["verdict"] == "found-but-unclassifiable")
        .map(|c| c["arxiv_id"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join(", ");
    out.push_str(&format!("\nCandidate pending / pending-truth / escalated counts: {}. Undetermined admitted statements are a separate category. The positive pair is a separate source route, not another arXiv candidate. Unclassifiable candidates {unclassifiable} remain unassigned; their notices do not support a class. No harvest is performed; resume remains candidate 28, 1403.6317.\n\nItem review union: {}; escalated item union: {}; promotion holds: {}; unflagged subset: {}. These counts overlap canonical missingness and truth missingness; they must not be added.\n\nRecorded item review issues:\n\n",compact_counts(&report["candidate_counts"],&["pending","pending-truth","escalated"]),report["qualification_counts"]["pending_item_review_union"],report["qualification_counts"]["escalated_item_union"],report["qualification_counts"]["promotion_holds"],report["qualification_counts"]["unflagged_documented_subset"]));
    for (kind, ids) in report["issues_by_kind"].as_object().unwrap() {
        out.push_str(&format!(
            "- {kind}: {} roots — {}.\n",
            ids.as_array().unwrap().len(),
            ids.as_array()
                .unwrap()
                .iter()
                .map(|i| format!("`{}`", i.as_str().unwrap()))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    out.push_str("\nThe first-parent promotion hold preserves its historical false/source-document label without treating it as uncontested or inferring a replacement. Review questions are documented qualifications, not silent label changes. The owner merged the positive pair; no separate GitHub reviewer approval is recorded and no CLAIM becomes PROVEN.\n\nUndetermined truth items are excluded from every truth-dependent true/false cell (including composites):\n\n");
    for id in report["undetermined_truth_items"].as_array().unwrap() {
        out.push_str(&format!("- `{}`\n", id.as_str().unwrap()));
    }
    out.push_str("\nSee ../OBLIGATIONS.md for the gate-by-gate evidence and gaps, including the seven truth and seven severity questions, unnamed dependent-result scope, byte retrieval and cold start. Gate S1 remains open; this informed session is not a cold-start examination.\n\n## Provenance and completeness\n\n");
    out.push_str(&format!("{} exact source-ID-array groups across {} roots are documentary groupings only. inventory.json also exposes shared source IDs, repeated raw hashes, lineage and per-item development exposure. Unknown exposure never means unseen or blinded. Common document/theorem/author origins can induce further dependence.\n\n",report["source_id_array_groups"],report["roots"]));
    out.push_str("| Source group | Logical items | Recorded licence / storage | Status and admission evidence | Open issues |\n|---|---:|---|---|---|\n");
    for (family, ids) in inventory["documentary_families"].as_object().unwrap() {
        let members: Vec<_> = inventory["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|i| i["documentary_family"] == *family)
            .collect();
        let first = members[0];
        let licence = first["root_manifest"]["config"]["licence"]
            .as_str()
            .unwrap()
            .replace('|', "/");
        let storage = first["root_manifest"]["artifact"]["files"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| format!("{k}: {}", v["storage"].as_str().unwrap()))
            .collect::<Vec<_>>()
            .join(", ");
        let issues: BTreeSet<_> = members
            .iter()
            .flat_map(|i| {
                i["issues"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|e| e["kind"].as_str().unwrap())
            })
            .collect();
        let links = members
            .iter()
            .map(|i| {
                let id = i["root_id"].as_str().unwrap();
                format!(
                    "[{}](../../../groundtruth/{id}/RECORD.md)",
                    id.rsplit('-').next().unwrap()
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        out.push_str(&format!("| {family} | {} | {licence}; {storage} | {links}; source entries in inventory.json | {} |\n",ids.as_array().unwrap().len(),if issues.is_empty(){"none recorded in reviewed disposition overlay".into()}else{issues.into_iter().collect::<Vec<_>>().join(", ")}));
    }
    out.push_str("\nEvery representation retains its typed manifest, stored status/truth/validity/correctness, complete flaw instances and locations, source references, byte-check scope and prior evidence links in inventory.json. The source register text is historical testimony, not fresh retrieval or a new rights opinion. Completeness of fields is distinct from mathematical assurance. provenance.json enumerates exact corpus/tool hashes and the preserved pair closure. Reproduce using ../README.md.\n");
    Ok(out)
}
