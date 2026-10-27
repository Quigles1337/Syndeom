//! Fail-closed effective inventory for the single owner-accepted first-parent case.
#![forbid(unsafe_code)]

#[allow(dead_code)]
#[path = "../../../manifest/examples/support/coverage.rs"]
mod historical;

use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use syndeom_manifest::{FileBytes, Manifest, Storage, content_hash, from_json_str};

pub type Blobs = BTreeMap<String, Vec<u8>>;
pub type Result<T> = std::result::Result<T, String>;
pub const BASE: &str = "37f1d598bb79a42eea6c5203d8a48d5d7b6ab29f";
pub const OLD: &str = "s1_ingest-v0.1.0-63e23a952084";
pub const NEW: &str = "s1_ingest-v0.1.0-15ab865466d6";
pub const PACKET: &str = "analysis/s1-first-parent-integration";
pub const MAP: &str = "groundtruth/amendment-map.json";
pub const WITNESS: &str = "analysis/s1-first-parent-status/WITNESS.md";
pub const WITNESS_HASH: &str = "c88af9c0d8b1ac2a65e8bdb634d41fbf3a8bbc1f31c168f81e3caff9e8573917";
const STATUS_REFERENCE: &str = "analysis/s1-first-parent-status/WITNESS.md; SHA-256 c88af9c0d8b1ac2a65e8bdb634d41fbf3a8bbc1f31c168f81e3caff9e8573917; sections 2-3; Codex GPT-6, 2026-10-20";
const DECISION: &str = "analysis/s1-first-parent-integration/OWNER-DECISION.md";
const FREEZE: &str =
    include_str!("../../../../analysis/s1-first-parent-integration/preservation-inputs.json");
const EXPECTED_MAP: &str = include_str!("../../../../groundtruth/amendment-map.json");
const COMPILED: &[(&str, &[u8])] = &[
    (
        "engine/ingest/examples/support/effective.rs",
        include_bytes!("effective.rs"),
    ),
    (
        "engine/ingest/examples/s1_effective_inventory.rs",
        include_bytes!("../s1_effective_inventory.rs"),
    ),
    (
        "engine/manifest/examples/support/coverage.rs",
        include_bytes!("../../../manifest/examples/support/coverage.rs"),
    ),
    (
        "engine/manifest/src/lib.rs",
        include_bytes!("../../../manifest/src/lib.rs"),
    ),
    (
        "engine/ingest/Cargo.toml",
        include_bytes!("../../Cargo.toml"),
    ),
    (
        "engine/manifest/Cargo.toml",
        include_bytes!("../../../manifest/Cargo.toml"),
    ),
    ("engine/Cargo.toml", include_bytes!("../../../Cargo.toml")),
    ("engine/Cargo.lock", include_bytes!("../../../Cargo.lock")),
    (
        "rust-toolchain.toml",
        include_bytes!("../../../../rust-toolchain.toml"),
    ),
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenFile {
    pub sha256: String,
    pub n_bytes: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Frozen {
    pub base_revision: String,
    pub base_tree: String,
    pub files: BTreeMap<String, FrozenFile>,
}

pub fn frozen() -> Result<Frozen> {
    let f: Frozen = serde_json::from_str(FREEZE).map_err(|e| e.to_string())?;
    if f.base_revision != BASE || f.base_tree != "aadce1d6332ad3a43f7f31b6779ee52083287215" {
        return Err("wrong preservation baseline".into());
    }
    Ok(f)
}

pub fn bytes<'a>(blobs: &'a Blobs, path: &str) -> Result<&'a [u8]> {
    blobs
        .get(path)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("missing input {path}"))
}

fn text<'a>(blobs: &'a Blobs, path: &str) -> Result<&'a str> {
    std::str::from_utf8(bytes(blobs, path)?).map_err(|e| e.to_string())
}

pub fn json(blobs: &Blobs, path: &str) -> Result<Value> {
    serde_json::from_slice(bytes(blobs, path)?).map_err(|e| format!("{path}: {e}"))
}

/// The baseline files the reproduction reads by name (ADR-0012), besides the item directories.
pub const DISPOSITIONS: &str = "analysis/s1-4/config/dispositions.json";
pub const CANDIDATE_CONFIG: &str = "analysis/s1-first-parent-status/candidate-config.json";

/// The accepted candidate's packet directory, whose manifest and source bytes are read.
pub fn candidate_dir() -> String {
    format!("analysis/s1-first-parent-status/candidate/{NEW}/")
}

/// An accepted item directory is a baseline directory under `groundtruth/` holding a manifest.
fn item_dirs(frozen: &Frozen) -> BTreeSet<&str> {
    frozen
        .files
        .keys()
        .filter_map(|p| {
            p.strip_prefix("groundtruth/")?
                .strip_suffix("/manifest.json")
        })
        .collect()
}

/// ADR-0012 D1 — the input set: every baseline file the reproduction reads, in baseline order.
/// The taxonomy; every file of every accepted item directory (manifest, record, committed bytes,
/// source map); the dispositions overlay; and the accepted-candidate packet files that
/// `validate_case` compares. Files the reproduction reads that are not in the baseline — the
/// amendment map, the owner decision and the effective representation's directory — are bound by
/// compiled constants and hashes instead. Nothing else in the baseline is byte-checked.
pub fn input_set(frozen: &Frozen) -> Vec<&str> {
    let items = item_dirs(frozen);
    let candidate = candidate_dir();
    frozen
        .files
        .keys()
        .map(String::as_str)
        .filter(|path| {
            matches!(
                *path,
                "PROGRAM/FLAWS.md" | DISPOSITIONS | WITNESS | CANDIDATE_CONFIG
            ) || path.starts_with(&candidate)
                || path
                    .strip_prefix("groundtruth/")
                    .and_then(|rest| rest.split_once('/'))
                    .is_some_and(|(dir, _)| items.contains(dir))
        })
        .collect()
}

/// ADR-0012: the input set is byte-identical to the accepted baseline, and nothing was added
/// under `groundtruth/` besides the amendment map and the effective representation, because the
/// census enumerates that directory. A change anywhere else in the tree is not this tool's
/// concern: it neither stops the reproduction nor fails the gate.
pub fn preserve(blobs: &Blobs, frozen: &Frozen) -> Result<Value> {
    let inputs = input_set(frozen);
    for path in &inputs {
        let pin = &frozen.files[*path];
        let current = bytes(blobs, path)?;
        if current.len() != pin.n_bytes || content_hash(current) != pin.sha256 {
            return Err(format!("historical bytes changed: {path}"));
        }
    }
    let effective = format!("groundtruth/{NEW}/");
    for path in blobs.keys().filter(|p| !frozen.files.contains_key(*p)) {
        if path.starts_with("groundtruth/") && path != MAP && !path.starts_with(&effective) {
            return Err(format!("unexpected addition in preserved scope: {path}"));
        }
    }
    let scope = |prefix: &str| inputs.iter().filter(|p| p.starts_with(prefix)).count();
    let item_files = scope("groundtruth/");
    let packet_files = scope("analysis/s1-first-parent-status/");
    Ok(
        json!({"baseline_revision": BASE, "baseline_tree": frozen.base_tree,
        "input_set_rule": "ADR-0012: the taxonomy, every accepted ground-truth item directory, the dispositions overlay and the accepted-candidate packet files the reproduction reads; baseline files outside this set are not byte-checked",
        "exact_files_checked": inputs.len(),
        "baseline_files": frozen.files.len(),
        "baseline_files_outside_input_set": frozen.files.len() - inputs.len(),
        "groundtruth_item_files_checked": item_files,
        "accepted_packet_files_checked": packet_files,
        "inputs_checked": inputs,
        "all_historical_groundtruth_objects": "every accepted item directory byte-identical; no added members",
        "pair_producer_and_dependency_pins": "hashed at the producer revision into provenance.json; not compared with the checkout (ADR-0012)"}),
    )
}

#[derive(Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Mapping {
    event_id: String,
    relation: String,
    accepted_packet_revision: String,
    owner_decision_date: String,
    owner_decision_timezone: String,
    owner_decision_path: String,
    owner_decision_sha256: String,
    logical_item_key: String,
    superseded_artifact_id: String,
    effective_artifact_id: String,
    activation: String,
}

pub fn mapping(blobs: &Blobs) -> Result<Mapping> {
    // Deserialize the struct directly: duplicate keys, extra fields and arrays are errors.
    let m: Mapping = serde_json::from_slice(bytes(blobs, MAP)?)
        .map_err(|e| format!("amendment mapping: {e}"))?;
    let expected: Mapping = serde_json::from_str(EXPECTED_MAP).map_err(|e| e.to_string())?;
    if m != expected
        || m.relation != "raw-metadata-amendment"
        || m.accepted_packet_revision != BASE
        || m.logical_item_key != OLD
        || m.superseded_artifact_id != OLD
        || m.effective_artifact_id != NEW
        || m.owner_decision_path != DECISION
        || content_hash(bytes(blobs, DECISION)?) != m.owner_decision_sha256
    {
        return Err(
            "inconsistent or unsupported one-case amendment mapping/owner-decision binding".into(),
        );
    }
    Ok(m)
}

pub fn manifest_path(id: &str) -> String {
    format!("groundtruth/{id}/manifest.json")
}

pub fn validate_case(blobs: &Blobs) -> Result<()> {
    mapping(blobs)?;
    if !text(blobs, "PROGRAM/DECISIONS/ADR-0003.md")?
        .contains("### 2026-10-20 America/New_York — accepted first-parent D3/SD12 exception")
        || !text(blobs, "PROGRAM/HANDOFF.md")?.contains(
            "OWNER DECISION EVENT: 2026-10-20 America/New_York — accepted first-parent amendment",
        )
    {
        return Err("accepted owner ruling must be recorded in ADR and append-only history".into());
    }
    if content_hash(bytes(blobs, WITNESS)?) != WITNESS_HASH {
        return Err("written witness hash mismatch".into());
    }
    let old = json(blobs, &manifest_path(OLD))?;
    let new = json(blobs, &manifest_path(NEW))?;
    let status = json!({"value":false,"method":"counterexample","reference":STATUS_REFERENCE});
    if new["config"]["statement_status"] != status
        || new["ground_truth"]["statement_status"] != status
    {
        return Err("ambiguous or unbound duplicated statement status".into());
    }
    if new["artifact_id"] != NEW
        || old["artifact_id"] != OLD
        || new["config_hash"] != "15ab865466d67dfd18ed487b4e51a3dc1719b07a00b89b3627e92dc111c95364"
        || old["config_hash"] != "63e23a9520843809c8f6b1d538d2ab2f1c3609dd5c6889b3779ea417a3567013"
        || old["engine_commit"] != "30e2a4aa06368b8a2ba1cc1a8164c4d22229a0cd"
        || new["engine_commit"] != "1a76d9ac21cd5d2e4311f9164178327c0ca128bb"
        || new["config"].get("derived_from").is_some()
        || old["config"].get("derived_from").is_some()
    {
        return Err("case identity/producing revision/raw relation mismatch".into());
    }
    let mut amended_config = old["config"].clone();
    amended_config["statement_status"] = status;
    if new["config"] != amended_config || new["config"]["status"]["reference"] != "S-0025" {
        return Err("amendment changes more than the accepted truth attribution".into());
    }
    let mut ingest_config = json(
        blobs,
        "analysis/s1-first-parent-status/candidate-config.json",
    )?;
    let notes = ingest_config
        .as_object_mut()
        .ok_or("candidate config must be an object")?
        .remove("notes")
        .ok_or("candidate notes missing")?;
    let retained = format!("analysis/s1-first-parent-status/candidate/{NEW}");
    if bytes(blobs, &manifest_path(NEW))? != bytes(blobs, &format!("{retained}/manifest.json"))?
        || content_hash(bytes(blobs, &manifest_path(NEW))?)
            != "2fbb965ffe92a8d0f102081adc43d84d56182fba5f4fee2bbd7c8dbfd080f8be"
        || ingest_config != new["config"]
        || notes != new["notes"]
    {
        return Err("admitted candidate differs from exact accepted packet/config".into());
    }
    let name = "original-0706.0357v70.tar.gz";
    let new_bytes = bytes(blobs, &format!("groundtruth/{NEW}/{name}"))?;
    if new_bytes != bytes(blobs, &format!("groundtruth/{OLD}/{name}"))?
        || new_bytes != bytes(blobs, &format!("{retained}/{name}"))?
        || new_bytes.len() != 8236
        || content_hash(new_bytes)
            != "5987d8106ae69669b10c876a0915d5ad52ce44a814ea7fe63600c1737642cca8"
    {
        return Err("admitted source bytes mismatch".into());
    }
    Ok(())
}

pub fn inventory(blobs: &Blobs) -> Result<Value> {
    let preservation = preserve(blobs, &frozen()?)?;
    validate_case(blobs)?;
    let classes = historical::taxonomy(text(blobs, "PROGRAM/FLAWS.md")?)?;
    let mut manifests: BTreeMap<String, Manifest> = BTreeMap::new();
    let mut byte_checks = Vec::new();
    for (path, data) in blobs
        .iter()
        .filter(|(p, _)| p.starts_with("groundtruth/") && p.ends_with("/manifest.json"))
    {
        let m = from_json_str(std::str::from_utf8(data).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if *path != manifest_path(&m.artifact_id) {
            return Err("manifest path/identity mismatch".into());
        }
        historical::validate_labels(&m, &classes)?;
        if let Some(status) = m.config.get("statement_status")
            && *status
                != serde_json::to_value(&m.ground_truth.as_ref().unwrap().statement_status)
                    .map_err(|e| e.to_string())?
        {
            return Err(format!("ambiguous duplicated status in {}", m.artifact_id));
        }
        let dir = format!("groundtruth/{}", m.artifact_id);
        bytes(blobs, &format!("{dir}/RECORD.md"))?;
        let mut available = BTreeMap::new();
        let files = &m.artifact.files;
        for (modality, name, storage) in [
            files
                .informal
                .as_ref()
                .map(|f| ("informal", &f.file, f.storage)),
            files
                .formal
                .as_ref()
                .map(|f| ("formal", &f.file, f.storage)),
        ]
        .into_iter()
        .flatten()
        {
            if name.contains(['/', '\\']) || name == "." || name == ".." {
                return Err("unsafe artifact file name".into());
            }
            let path = format!("{dir}/{name}");
            let present = blobs.get(&path);
            if storage == Storage::Committed && present.is_none() {
                return Err(format!("missing committed bytes {path}"));
            }
            if let Some(b) = present {
                available.insert(modality, b.as_slice());
            }
            byte_checks.push(json!({"artifact_id":m.artifact_id,"file":name,"actual_bytes_checked":present.is_some(),
                "scope":if present.is_some(){"Manifest::check against committed bytes"}else{"external/regenerable bytes absent; retained historical evidence only"}}));
        }
        m.check(FileBytes {
            informal: available.get("informal").copied(),
            formal: available.get("formal").copied(),
        })
        .map_err(|e| e.join("; "))?;
        if manifests.insert(m.artifact_id.clone(), m).is_some() {
            return Err("duplicate representation".into());
        }
    }
    let roots = historical::lineage(&manifests)?;
    let dispositions: historical::Dispositions =
        serde_json::from_slice(bytes(blobs, "analysis/s1-4/config/dispositions.json")?)
            .map_err(|e| e.to_string())?;
    let mut reviews = BTreeMap::new();
    for review in dispositions.reviews {
        if reviews.insert(review.root_id, review.issues).is_some() {
            return Err("duplicate disposition".into());
        }
    }
    let held = reviews
        .remove(OLD)
        .ok_or("missing historical first-parent hold")?;
    if held.len() != 1 || held[0].kind != "promotion-hold" {
        return Err("unexpected hold scope".into());
    }
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (id, root) in &roots {
        // Only raw NEW aliases OLD. A future canonical child needs its own reviewed integration.
        if (root == NEW || root == OLD) && id != NEW && id != OLD {
            return Err("unreviewed canonical child of amended item".into());
        }
        let key = if root == NEW { OLD } else { root };
        groups.entry(key.into()).or_default().push(id.clone());
    }
    let mut items = Vec::new();
    let mut effective_ids = BTreeSet::new();
    let mut truth = BTreeMap::<String, usize>::new();
    let mut qualified_truth = BTreeMap::<String, usize>::new();
    let mut unflagged = 0;
    let mut missing_canonical = 0;
    let mut proof_status = BTreeMap::<String, usize>::new();
    let mut flaw_instances = 0;
    let mut observed_classes = BTreeSet::new();
    for (key, representations) in &groups {
        let effective = if key == OLD { NEW } else { key };
        if !effective_ids.insert(effective.to_string())
            || !representations.iter().any(|id| id == effective)
        {
            return Err("missing/ambiguous effective representation".into());
        }
        let m = &manifests[effective];
        let gt = m.ground_truth.as_ref().unwrap();
        flaw_instances += gt.flaws.len();
        observed_classes.extend(gt.flaws.iter().map(|f| f.class.clone()));
        let issues = reviews.remove(key).unwrap_or_default();
        let (clear, truth_ok) = historical::qualify(&issues, gt.statement_status.value);
        unflagged += usize::from(clear);
        let value = match gt.statement_status.value {
            Some(true) => "true",
            Some(false) => "false",
            None => "undetermined",
        };
        *truth.entry(value.into()).or_default() += 1;
        if truth_ok {
            *qualified_truth.entry(value.into()).or_default() += 1;
        }
        *proof_status
            .entry(format!("{:?}", gt.proof_status).to_lowercase())
            .or_default() += 1;
        let canonical: Vec<_> = representations
            .iter()
            .filter(|id| manifests[*id].config.get("derived_from").is_some())
            .collect();
        missing_canonical += usize::from(canonical.is_empty());
        items.push(json!({"logical_item_key":key,"effective_artifact_id":effective,"archived_representations":representations,
            "statement_status":gt.statement_status,"author_status":m.config["status"],"proof_status":gt.proof_status,
            "effective_producing_revision":m.engine_commit,"issues":issues,
            "unflagged_documented_subset":clear,"truth_dimension_eligible":truth_ok,
            "canonical_representations":canonical,"status_support_hold":false,
            "canonical_selection_context_locator_review_required":key == OLD,
            "scoring_readiness":"not adjudicated by this metadata inventory; canonical artifacts and frozen battery still required"}));
    }
    if !reviews.is_empty()
        || manifests.len() != 35
        || groups.len() != 32
        || effective_ids.len() != 32
    {
        return Err("unexpected inventory/disposition population; one amendment must yield 35 representations and 32 logical items".into());
    }
    let raw = manifests
        .values()
        .filter(|m| m.config.get("derived_from").is_none())
        .count();
    if raw != 33 || groups[OLD] != vec![NEW.to_string(), OLD.to_string()] {
        return Err("amended item counted ambiguously".into());
    }
    Ok(
        json!({"event_id":"first-parent-accepted-integration-2026-10-20","mapping":json(blobs,MAP)?,
        "activation_scope":"this verified tree only; main changes only on review and merge",
        "counts":{"archived_representations":manifests.len(),"archived_raw_representations":raw,"canonical_representations":manifests.len()-raw,
        "effective_logical_items":groups.len(),"unflagged_documented_subset":unflagged,"statement_status":truth,
        "qualified_truth_dimension":qualified_truth,"proof_status":proof_status,"logical_items_without_canonical":missing_canonical,
        "flaw_instances":flaw_instances,"observed_flaw_classes":observed_classes.len(),"zero_coverage_classes":classes.len()-observed_classes.len()},
        "items":items,"actual_byte_checks":byte_checks,"preservation":preservation,
        "historical_s1_4":"unchanged; its bins, counts and receipts describe its original frozen prefix",
        "scope_notes":dispositions.scope_notes,
        "formalization_boundary":"owner accepts the attributed written witness; full analytic source-to-Lean bridge not formalized; no PROVEN claim",
        "gate_s1":"OPEN; informed integration session; later separate genuine cold-start examination"}),
    )
}

fn git(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let o = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| e.to_string())?;
    if !o.status.success() {
        return Err(format!(
            "git {args:?}: {}",
            String::from_utf8_lossy(&o.stderr)
        ));
    }
    Ok(o.stdout)
}

/// Read Git objects through one process; no shell interpolation or working-file fallback.
fn snapshot(repo: &Path, revision: &str) -> Result<Blobs> {
    let listing = git(repo, &["ls-tree", "-r", "-z", revision])?;
    let mut child = Command::new("git")
        .args(["cat-file", "--batch"])
        .current_dir(repo)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut input = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap());
    let mut blobs = Blobs::new();
    for record in listing.split(|b| *b == 0).filter(|r| !r.is_empty()) {
        let record = std::str::from_utf8(record).map_err(|e| e.to_string())?;
        let (meta, path) = record.split_once('\t').ok_or("bad git listing")?;
        let fields: Vec<_> = meta.split_whitespace().collect();
        if fields.len() != 3 || fields[1] != "blob" || !matches!(fields[0], "100644" | "100755") {
            return Err("unsupported Git object type/mode".into());
        }
        writeln!(input, "{}", fields[2]).map_err(|e| e.to_string())?;
        input.flush().map_err(|e| e.to_string())?;
        let mut header = String::new();
        output.read_line(&mut header).map_err(|e| e.to_string())?;
        let fields: Vec<_> = header.split_whitespace().collect();
        if fields.len() != 3 || fields[1] != "blob" {
            return Err("invalid git blob response".into());
        }
        let len: usize = fields[2].parse().map_err(|_| "bad blob length")?;
        let mut bytes = vec![0; len + 1];
        output.read_exact(&mut bytes).map_err(|e| e.to_string())?;
        if bytes.pop() != Some(b'\n') {
            return Err("missing blob delimiter".into());
        }
        blobs.insert(path.into(), bytes);
    }
    drop(input);
    if !child.wait().map_err(|e| e.to_string())?.success() {
        return Err("git object reader failed".into());
    }
    Ok(blobs)
}

fn clean(repo: &Path) -> Result<()> {
    if !git(repo, &["status", "--porcelain=v1", "--untracked-files=all"])?.is_empty() {
        return Err(
            "commit all inputs and use a clean checkout; output must be outside the repository"
                .into(),
        );
    }
    Ok(())
}

pub fn cli() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err(
            "usage: s1_effective_inventory <repo> <new-output-directory-outside-repo>".into(),
        );
    }
    let repo = fs::canonicalize(&args[0]).map_err(|e| e.to_string())?;
    let out = Path::new(&args[1]);
    if out.exists() {
        return Err("output already exists; refusing overwrite".into());
    }
    let parent = fs::canonicalize(out.parent().ok_or("output parent missing")?)
        .map_err(|e| e.to_string())?;
    if parent.starts_with(&repo) {
        return Err("output must be outside repository".into());
    }
    clean(&repo)?;
    let head = String::from_utf8(git(&repo, &["rev-parse", "HEAD"])?)
        .map_err(|e| e.to_string())?
        .trim()
        .to_string();
    let tree = String::from_utf8(git(&repo, &["rev-parse", "HEAD^{tree}"])?)
        .map_err(|e| e.to_string())?
        .trim()
        .to_string();
    let frozen = frozen()?;
    let baseline = snapshot(&repo, BASE)?;
    if baseline.len() != frozen.files.len()
        || baseline.iter().any(|(p, b)| {
            frozen
                .files
                .get(p)
                .is_none_or(|f| f.n_bytes != b.len() || f.sha256 != content_hash(b))
        })
    {
        return Err("preservation input list does not bind the complete accepted Git tree".into());
    }
    let current = snapshot(&repo, &head)?;
    for (path, compiled) in COMPILED {
        if bytes(&current, path)? != *compiled
            || fs::read(repo.join(path)).map_err(|e| e.to_string())? != *compiled
        {
            return Err(format!("compiled/input source mismatch {path}"));
        }
    }
    if bytes(&current, MAP)? != EXPECTED_MAP.as_bytes()
        || bytes(&current, &format!("{PACKET}/preservation-inputs.json"))? != FREEZE.as_bytes()
    {
        return Err("compiled mapping/preservation pins mismatch".into());
    }
    let result = inventory(&current)?;
    // ADR-0012: the pair producer's inputs are hashed at the producer revision for the record.
    // The checkout's bytes are not compared: the inventory does not read these files, and the
    // pair's replay is pinned to the revision range STATE records (ADR-0010).
    let pair_revision = "eb1b96d5b8084ceb1de734fffc90c9b480ec32bf";
    let list = git(
        &repo,
        &[
            "show",
            &format!("{pair_revision}:engine/pair/input-files.txt"),
        ],
    )?;
    let mut pair_inputs = BTreeMap::new();
    for path in std::str::from_utf8(&list)
        .map_err(|e| e.to_string())?
        .lines()
    {
        let original = git(&repo, &["show", &format!("{pair_revision}:{path}")])?;
        pair_inputs.insert(path.to_string(), content_hash(&original));
    }
    if pair_inputs.len() != 20 {
        return Err("unexpected frozen pair input membership".into());
    }
    let provenance = json!({"checkout_revision":head,"checkout_tree":tree,"accepted_packet_revision":BASE,
        "tool_source_sha256":COMPILED.iter().map(|(p,b)|(*p,content_hash(b))).collect::<BTreeMap<_,_>>(),
        "mapping_sha256":content_hash(EXPECTED_MAP.as_bytes()),"preservation_inputs_sha256":content_hash(FREEZE.as_bytes()),
        "taxonomy_version":"0.1.0","battery_version":null,"seed":null,"uncertainty":"deterministic census of pinned metadata; not a statistical estimate",
        "pair_producer_revision":pair_revision,"pair_input_hashes":pair_inputs,
        "pair_input_check":"hashes of the producer revision's objects; the checkout's bytes are not compared (ADR-0012)",
        "no_network_or_witness_reinvestigation":true});
    clean(&repo)?;
    if String::from_utf8(git(&repo, &["rev-parse", "HEAD"])?)
        .map_err(|e| e.to_string())?
        .trim()
        != head
    {
        return Err("checkout changed during validation".into());
    }
    historical::output_dir(
        out,
        BTreeMap::from([
            (
                "inventory.json".to_string(),
                historical::json_bytes(&result)?,
            ),
            (
                "provenance.json".to_string(),
                historical::json_bytes(&provenance)?,
            ),
        ]),
    )?;
    println!(
        "PASS: 35 archived representations / 32 effective logical items; historical bytes preserved; {OLD} -> {NEW}"
    );
    Ok(())
}
