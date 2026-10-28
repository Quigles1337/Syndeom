//! Fail-closed inventory and coverage of the first later prefix (ADR-0013): the accepted
//! first-parent integration's inputs unchanged, plus five item directories frozen at the commit
//! that added them. The pinned tools `s1_effective_inventory` and `s1_4_coverage` are not changed;
//! their modules are read here, as they stand.
#![forbid(unsafe_code)]

#[allow(dead_code)]
#[path = "effective.rs"]
mod effective;

// `effective.rs` loads the same pinned module privately, and its bytes may not change, so this
// tool loads its own copy.
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "../../../manifest/examples/support/coverage.rs"]
mod historical;

pub use effective::{
    BASE, Blobs, DISPOSITIONS, Frozen, FrozenFile, MAP, NEW, OLD, Result, bytes, frozen, input_set,
    manifest_path,
};
pub use historical::json_bytes;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use syndeom_manifest::{FileBytes, Manifest, Storage, content_hash, from_json_str};

pub const EVENT: &str = "first-later-prefix-inventory-2026-10-28";
pub const PACKET: &str = "analysis/s1-later-prefix-1";
pub const FREEZE_POINT: &str = "d2569d61588368a60bbf7558fb9f0703f96e3a5b";
pub const PREFIX_INPUTS: &str = "analysis/s1-later-prefix-1/config/prefix-inputs.json";
pub const OVERLAY: &str = "analysis/s1-later-prefix-1/config/dispositions.json";
pub const BINS: &str = "analysis/s1-4/config/bins.json";
pub const HARVEST_LOG: &str = "groundtruth/HARVEST-LOG.md";
pub const DECISION: &str = "analysis/s1-first-parent-integration/OWNER-DECISION.md";
const BASELINE: &str = "analysis/s1-first-parent-integration/preservation-inputs.json";
const PAIR_VERIFICATION: &str = "analysis/s1-2/implementation/VERIFICATION.md";
const LAST_CANDIDATE: (usize, &str) = (30, "1404.4599");
const PREFIX: &str =
    include_str!("../../../../analysis/s1-later-prefix-1/config/prefix-inputs.json");
const EXPECTED_OVERLAY: &str =
    include_str!("../../../../analysis/s1-later-prefix-1/config/dispositions.json");
const EXPECTED_BASELINE: &str =
    include_str!("../../../../analysis/s1-first-parent-integration/preservation-inputs.json");
const COMPILED: &[(&str, &[u8])] = &[
    (
        "engine/ingest/examples/support/later_prefix.rs",
        include_bytes!("later_prefix.rs"),
    ),
    (
        "engine/ingest/examples/s1_later_prefix.rs",
        include_bytes!("../s1_later_prefix.rs"),
    ),
    (
        "engine/ingest/examples/support/effective.rs",
        include_bytes!("effective.rs"),
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

fn text<'a>(blobs: &'a Blobs, path: &str) -> Result<&'a str> {
    std::str::from_utf8(bytes(blobs, path)?).map_err(|e| e.to_string())
}

/// ADR-0013 D2 — the prefix's pins: its item directories' files and the harvest log, as they
/// stood at the freeze point (Al's ruling Q-H7 of 2026-10-28).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prefix {
    pub event_id: String,
    pub freeze_point: String,
    pub freeze_point_tree: String,
    pub item_directories: Vec<String>,
    pub files: BTreeMap<String, FrozenFile>,
    pub harvest_log: FrozenFile,
}

pub fn prefix() -> Result<Prefix> {
    let p: Prefix = serde_json::from_str(PREFIX).map_err(|e| e.to_string())?;
    let dirs: BTreeSet<&str> = p
        .files
        .keys()
        .filter_map(|f| f.strip_prefix("groundtruth/")?.split_once('/'))
        .map(|(dir, _)| dir)
        .collect();
    let complete = p.item_directories.iter().all(|d| {
        ["manifest.json", "RECORD.md"]
            .iter()
            .all(|name| p.files.contains_key(&format!("groundtruth/{d}/{name}")))
    });
    if p.event_id != EVENT
        || p.freeze_point != FREEZE_POINT
        || p.freeze_point_tree != "1e2fafad7e21fc69d0ebbc315a96b0e065dd9bd7"
        || p.item_directories.len() != 5
        || dirs
            != p.item_directories
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
        || p.files.len() != 2 * p.item_directories.len()
        || !complete
    {
        return Err("wrong later-prefix pins".into());
    }
    Ok(p)
}

fn pinned(blobs: &Blobs, path: &str, pin: &FrozenFile) -> Result<()> {
    let current = bytes(blobs, path)?;
    if current.len() != pin.n_bytes || content_hash(current) != pin.sha256 {
        return Err(format!("historical bytes changed: {path}"));
    }
    Ok(())
}

/// The harvest log as it stood at the freeze point. The file is append-only, so its pinned
/// prefix is read and a later harvest's rows are not.
pub fn harvest_log<'a>(blobs: &'a Blobs, p: &Prefix) -> Result<&'a str> {
    let log = bytes(blobs, HARVEST_LOG)?;
    let head = log
        .get(..p.harvest_log.n_bytes)
        .ok_or("harvest log shorter than its pinned prefix")?;
    if content_hash(head) != p.harvest_log.sha256 {
        return Err(format!("historical bytes changed: {HARVEST_LOG}"));
    }
    std::str::from_utf8(head).map_err(|e| e.to_string())
}

/// ADR-0013 D3 — ADR-0012's input set and the frozen bins, byte-identical to the accepted
/// baseline; the prefix's item files byte-identical to the freeze point; the harvest log's pinned
/// prefix; and nothing else added under `groundtruth/` besides the amendment map and the effective
/// representation. A change anywhere else neither stops the inventory nor fails the gate.
pub fn preserve(blobs: &Blobs, frozen: &Frozen, p: &Prefix) -> Result<Value> {
    let inputs = input_set(frozen);
    for path in inputs.iter().copied().chain([BINS]) {
        let pin = frozen
            .files
            .get(path)
            .ok_or_else(|| format!("not a baseline file: {path}"))?;
        pinned(blobs, path, pin)?;
    }
    for (path, pin) in &p.files {
        if frozen.files.contains_key(path) {
            return Err(format!(
                "prefix file already in the accepted baseline: {path}"
            ));
        }
        pinned(blobs, path, pin)?;
    }
    harvest_log(blobs, p)?;
    let effective = format!("groundtruth/{NEW}/");
    for path in blobs
        .keys()
        .filter(|k| !frozen.files.contains_key(*k) && !p.files.contains_key(*k))
    {
        if path.starts_with("groundtruth/") && path != MAP && !path.starts_with(&effective) {
            return Err(format!("unexpected addition in preserved scope: {path}"));
        }
    }
    Ok(
        json!({"baseline_revision": BASE, "baseline_tree": frozen.base_tree,
        "prefix_freeze_point": p.freeze_point, "prefix_freeze_point_tree": p.freeze_point_tree,
        "input_set_rule": "ADR-0013 D3: ADR-0012's input set and the frozen S1.4 bins at the accepted baseline; the prefix's item files at the freeze point; the harvest log's prefix as it stood at the freeze point; additions under groundtruth/ refused besides the amendment map and the effective representation",
        "baseline_files_checked": inputs.len() + 1,
        "prefix_item_directories": p.item_directories,
        "prefix_files_checked": p.files.len(),
        "harvest_log_prefix_bytes": p.harvest_log.n_bytes,
        "accepted_corpus_and_packets": "the first-parent integration's inputs byte-identical; its tool, packet and retained report unchanged and not run"}),
    )
}

/// ADR-0013 D4 — the prefix's review overlay. Every prefix root is listed explicitly; the frozen
/// S1.4 overlay still governs the accepted corpus's roots and is not edited.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrefixReview {
    pub root_id: String,
    pub issues: Vec<historical::Issue>,
    pub basis: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Overlay {
    pub schema: String,
    pub event_id: String,
    pub base_overlay: String,
    pub base_overlay_sha256: String,
    pub reviews: Vec<PrefixReview>,
    pub scope_notes: Vec<String>,
}

pub fn overlay(blobs: &Blobs, p: &Prefix) -> Result<Overlay> {
    let o: Overlay =
        serde_json::from_slice(bytes(blobs, OVERLAY)?).map_err(|e| format!("{OVERLAY}: {e}"))?;
    let listed: BTreeSet<&str> = o.reviews.iter().map(|r| r.root_id.as_str()).collect();
    if o.schema != "s1-later-prefix-overlay/1"
        || o.event_id != EVENT
        || o.base_overlay != DISPOSITIONS
        || o.base_overlay_sha256 != content_hash(bytes(blobs, DISPOSITIONS)?)
        || listed.len() != o.reviews.len()
        || listed
            != p.item_directories
                .iter()
                .map(String::as_str)
                .collect::<BTreeSet<_>>()
    {
        return Err("the prefix overlay must list each prefix root once, and only those".into());
    }
    for review in &o.reviews {
        let kinds = [
            "promotion-hold",
            "truth-review",
            "severity-review",
            "pending-review",
        ];
        if review.basis.is_empty()
            || review
                .issues
                .iter()
                .any(|i| !kinds.contains(&i.kind.as_str()) || i.evidence.is_empty())
        {
            return Err("unknown or unsupported prefix review".into());
        }
        for reference in review
            .basis
            .iter()
            .chain(review.issues.iter().flat_map(|i| i.evidence.iter()))
        {
            bytes(blobs, reference.split('#').next().unwrap())
                .map_err(|_| format!("review evidence missing: {reference}"))?;
        }
    }
    Ok(o)
}

fn bump(map: &mut BTreeMap<String, usize>, key: &str) {
    *map.entry(key.to_string()).or_default() += 1;
}

/// The frozen S1.4 bins stay the bins (ADR-0003 D9: fixed at the first S1.4 write, never
/// changed). This event adds no canonical length, so the population's octave grid must still be
/// the frozen one; a later event that adds lengths decides what that means in its own ADR.
fn validate_bins(bins: &historical::Bins, manifests: &BTreeMap<String, Manifest>) -> Result<()> {
    if bins.schema != "s1-4-coverage/1"
        || bins.edge_rule
            != "lower-inclusive upper-exclusive; retain empty bins; explicit underflow/overflow"
        || bins
            .boundaries
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            != ["formal", "informal"]
    {
        return Err("unsupported bin contract".into());
    }
    for (modality, edges) in &bins.boundaries {
        let values: Vec<u64> = manifests
            .values()
            .flat_map(historical::lengths)
            .filter(|l| &l.modality == modality)
            .map(|l| l.n_tokens)
            .collect();
        if &historical::octave_edges(&values)? != edges {
            return Err("the population's lengths no longer fit the frozen bins".into());
        }
    }
    Ok(())
}

pub struct Event {
    pub inventory: Value,
    pub coverage: Value,
}

pub fn inventory(blobs: &Blobs) -> Result<Event> {
    let frozen = frozen()?;
    let p = prefix()?;
    let preservation = preserve(blobs, &frozen, &p)?;
    effective::validate_case(blobs)?;
    let o = overlay(blobs, &p)?;
    let classes = historical::taxonomy(text(blobs, "PROGRAM/FLAWS.md")?)?;
    let candidates = historical::harvest(harvest_log(blobs, &p)?)?;
    let mut manifests: BTreeMap<String, Manifest> = BTreeMap::new();
    let mut byte_checks = Vec::new();
    for (path, data) in blobs
        .iter()
        .filter(|(k, _)| k.starts_with("groundtruth/") && k.ends_with("/manifest.json"))
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
    let bins: historical::Bins =
        serde_json::from_slice(bytes(blobs, BINS)?).map_err(|e| e.to_string())?;
    validate_bins(&bins, &manifests)?;
    let base: historical::Dispositions =
        serde_json::from_slice(bytes(blobs, DISPOSITIONS)?).map_err(|e| e.to_string())?;
    let mut reviews = BTreeMap::new();
    for review in base.reviews {
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
    let prefix_roots: BTreeSet<&str> = p.item_directories.iter().map(String::as_str).collect();
    for review in &o.reviews {
        if reviews
            .insert(review.root_id.clone(), review.issues.clone())
            .is_some()
        {
            return Err("prefix root reviewed by both overlays".into());
        }
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
    let mut covered = Vec::new();
    let mut effective_ids = BTreeSet::new();
    let mut admitted_seen = BTreeSet::new();
    let mut truth = BTreeMap::<String, usize>::new();
    let mut qualified_truth = BTreeMap::<String, usize>::new();
    let mut proof_status = BTreeMap::<String, usize>::new();
    let mut unflagged = 0;
    let mut missing_canonical = 0;
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
        bump(&mut truth, value);
        if truth_ok {
            bump(&mut qualified_truth, value);
        }
        bump(
            &mut proof_status,
            &format!("{:?}", gt.proof_status).to_lowercase(),
        );
        let canonical: Vec<_> = representations
            .iter()
            .filter(|id| manifests[*id].config.get("derived_from").is_some())
            .collect();
        missing_canonical += usize::from(canonical.is_empty());
        let lengths: Vec<historical::Length> = representations
            .iter()
            .flat_map(|id| historical::lengths(&manifests[id]))
            .collect();
        let modalities: BTreeSet<&str> = lengths.iter().map(|l| l.modality.as_str()).collect();
        if modalities.len() != lengths.len() {
            return Err("multiple canonical representations in one modality require an explicit selection policy".into());
        }
        let in_prefix = prefix_roots.contains(key.as_str());
        let candidate = candidates
            .iter()
            .find(|c| c.verdict == "admitted" && c.artifact_ids.contains(key));
        let record = format!("groundtruth/{effective}/RECORD.md");
        let mut evidence = vec![record, "PROGRAM/SOURCES.md".to_string()];
        let admission = if let Some(c) = candidate {
            admitted_seen.insert(key.clone());
            evidence.push(format!("{HARVEST_LOG}#L{}", c.evidence_line));
            format!(
                "{}, Route A candidate {} ({})",
                if in_prefix {
                    "admitted in the first later prefix"
                } else {
                    "historically admitted"
                },
                c.index,
                c.arxiv_id
            )
        } else if *key == base.pair_root {
            evidence.push(base.merge_evidence.clone());
            evidence.push(PAIR_VERIFICATION.into());
            "owner merged PR 4 admission packet; documented checks retained; no separate GitHub reviewer approval and no PROVEN promotion".into()
        } else {
            return Err(format!("no admission evidence for {key}"));
        };
        let sources: Vec<String> =
            serde_json::from_value(m.config["sources"].clone()).map_err(|e| e.to_string())?;
        items.push(json!({"logical_item_key":key,"effective_artifact_id":effective,"archived_representations":representations,
            "event":if in_prefix {EVENT} else {"first-parent-accepted-integration-2026-10-20"},
            "statement_status":gt.statement_status,"author_status":m.config["status"],"proof_status":gt.proof_status,
            "effective_producing_revision":m.engine_commit,"admission":admission,"issues":issues,
            "unflagged_documented_subset":clear,"truth_dimension_eligible":truth_ok,
            "canonical_representations":canonical,"status_support_hold":false,
            "canonical_selection_context_locator_review_required":key == OLD,
            "scoring_readiness":"not adjudicated by this metadata inventory; canonical artifacts and frozen battery still required"}));
        covered.push(historical::Item {
            root_id: key.clone(),
            representations: representations.clone(),
            documentary_family: sources.join("+"),
            sources,
            admission,
            evidence,
            issues,
            unflagged_documented_subset: clear,
            truth_dimension_eligible: truth_ok,
            scoring_ready: false,
            development_exposure: if *key == base.pair_root || representations.len() > 1 {
                "known development/source-review exposure in repository session records and task context; not blinded"
            } else {
                "known harvest/executor review; additional chat/training exposure unknown, never classified as unseen"
            }
            .into(),
            lengths,
            root_manifest: m.clone(),
        });
    }
    let admitted: BTreeSet<String> = candidates
        .iter()
        .filter(|c| c.verdict == "admitted")
        .flat_map(|c| c.artifact_ids.iter().cloned())
        .collect();
    let raw = manifests
        .values()
        .filter(|m| m.config.get("derived_from").is_none())
        .count();
    let last = candidates.last().ok_or("empty harvest")?;
    if !reviews.is_empty()
        || manifests.len() != 40
        || groups.len() != 37
        || effective_ids.len() != 37
        || raw != 38
        || groups[OLD] != vec![NEW.to_string(), OLD.to_string()]
        || admitted_seen != admitted
        || !prefix_roots.iter().all(|r| admitted_seen.contains(*r))
        || (last.index, last.arxiv_id.as_str()) != LAST_CANDIDATE
    {
        return Err("unexpected inventory/disposition population; the first later prefix must yield 40 representations and 37 logical items through candidate 30".into());
    }
    let mut candidate_counts: BTreeMap<String, usize> = [
        "admitted",
        "rejected",
        "found-but-unclassifiable",
        "pending",
        "pending-truth",
        "escalated",
    ]
    .into_iter()
    .map(|v| (v.to_string(), 0))
    .collect();
    for c in &candidates {
        bump(&mut candidate_counts, &c.verdict);
    }
    let rows = historical::coverage(&covered, &classes, &bins);
    let subset: Vec<historical::Item> = covered
        .iter()
        .filter(|i| i.unflagged_documented_subset)
        .cloned()
        .collect();
    let qualified = historical::coverage(&subset, &classes, &bins);
    let zero: Vec<_> = rows
        .iter()
        .filter(|r| r.class != "TOTAL" && r.items == 0)
        .map(|r| json!({"class":r.class,"name":r.name}))
        .collect();
    let undetermined: Vec<_> = covered
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
    let mut issues_by_kind: BTreeMap<String, Vec<String>> = [
        "promotion-hold",
        "truth-review",
        "severity-review",
        "pending-review",
    ]
    .into_iter()
    .map(|k| (k.to_string(), Vec::new()))
    .collect();
    for i in &covered {
        for issue in &i.issues {
            issues_by_kind
                .entry(issue.kind.clone())
                .or_default()
                .push(i.root_id.clone());
        }
    }
    let mut scope_notes = base.scope_notes.clone();
    scope_notes.extend(o.scope_notes.iter().cloned());
    let inventory = json!({"event_id":EVENT,"mapping":effective::json(blobs,MAP)?,
        "activation_scope":"this verified tree only; main changes only on review and merge",
        "counts":{"archived_representations":manifests.len(),"archived_raw_representations":raw,"canonical_representations":manifests.len()-raw,
        "effective_logical_items":groups.len(),"prefix_logical_items":prefix_roots.len(),"unflagged_documented_subset":unflagged,"statement_status":truth,
        "qualified_truth_dimension":qualified_truth,"proof_status":proof_status,"logical_items_without_canonical":missing_canonical,
        "flaw_instances":flaw_instances,"observed_flaw_classes":observed_classes.len(),"zero_coverage_classes":classes.len()-observed_classes.len()},
        "items":items,"actual_byte_checks":byte_checks,"preservation":preservation,
        "historical_s1_4":"unchanged; its bins, counts and receipts describe its original frozen prefix",
        "accepted_inventory":"unchanged; the first-parent integration's report (35 / 32) describes the accepted corpus before this prefix",
        "scope_notes":scope_notes,
        "formalization_boundary":"owner accepts the attributed written witness; full analytic source-to-Lean bridge not formalized; no PROVEN claim",
        "gate_s1":"CLOSED 2026-10-26 (Session 29) on the earlier prefix; its table is not rewritten (ADR-0003 D8); this is the later prefix's own record"});
    let coverage = json!({"schema":"s1-later-prefix-coverage/1","event_id":EVENT,"taxonomy_version":"0.1.0","battery_version":null,"seed":null,
        "uncertainty":"descriptive census of the pinned prefix; no sampling intervals or numeric power calculated",
        "label_basis":"effective representations: the accepted first-parent amendment applied, so its logical item carries its effective status (ADR-0013 D5)",
        "representations":manifests.len(),"logical_items":groups.len(),"canonical_representations":manifests.len()-raw,
        "archive_coverage":rows,"unflagged_documented_subset_coverage":qualified,
        "subset_rule":"exclude logical items with promotion-hold, truth-review, severity-review or pending-review issues; the accepted corpus's issues from the frozen S1.4 overlay with the first-parent hold discharged by the accepted amendment, the prefix's from its own overlay. Not scoring readiness or a new admission policy.",
        "bins":bins,"zero_coverage":zero,"candidate_counts":candidate_counts,"harvest_candidates":candidates.len(),"prefix_through_arxiv_id":last.arxiv_id,
        "undetermined_truth_items":undetermined,"issues_by_kind":issues_by_kind,"scope_notes":scope_notes,
        "qualification_counts":{"pending_item_review_union":covered.iter().filter(|i|!i.issues.is_empty()).count(),"escalated_item_union":covered.iter().filter(|i|i.issues.iter().any(|e|e.kind=="truth-review"||e.kind=="severity-review")).count(),"promotion_holds":covered.iter().filter(|i|i.issues.iter().any(|e|e.kind=="promotion-hold")).count(),"unflagged_documented_subset":subset.len()},
        "interim_power":"Every real-item cell is expected underpowered for the equivalence test until S3 derives N_min and freezes the protocol; S4 plans recall against documented errors and a false-positive upper bound on correct proofs, not a fooling length.",
        "gate_s1_table":"not rewritten; the S1.4 frozen packet and the Gate S1 table describe the prefix through 1403.4485 (ADR-0003 D8)"});
    Ok(Event {
        inventory,
        coverage,
    })
}

fn counts(value: &Value, keys: &[&str]) -> String {
    keys.iter()
        .map(|k| value[*k].as_u64().unwrap_or(0).to_string())
        .collect::<Vec<_>>()
        .join("/")
}

/// The D9 table as Markdown, rendered from `coverage.json`'s values only.
pub fn render(coverage: &Value) -> Result<String> {
    let mut out = String::new();
    out.push_str(&format!(
        "# First later prefix — coverage (ADR-0003 D9; ADR-0013)\n\nEvent `{}`. Prefix through arXiv id {}: {} harvest candidates, {} archived representations, {} effective logical items. Taxonomy `0.1.0`; no battery; no seed. Labels are the effective representations' (the accepted first-parent amendment applied). Generated from `coverage.json` by the tool; the Gate S1 table and the frozen S1.4 packet are not rewritten.\n\n",
        coverage["event_id"].as_str().ok_or("event id")?,
        coverage["prefix_through_arxiv_id"].as_str().ok_or("prefix id")?,
        coverage["harvest_candidates"],
        coverage["representations"],
        coverage["logical_items"]
    ));
    for (title, key) in [
        ("Archive coverage", "archive_coverage"),
        (
            "Unflagged documented subset",
            "unflagged_documented_subset_coverage",
        ),
    ] {
        out.push_str(&format!("## {title}\n\n| Class | Name | Items | Instances | Severity items c/r/g/f | Lean-catchable items K/T/N/NA | Truth t/f/u | Truth-eligible t/f/excluded | Proof valid/invalid | Canonical roots | Missing canonical length | Informal steps n/min/median/max |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n"));
        for row in coverage[key].as_array().ok_or("coverage rows")? {
            let steps = &row["steps"];
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}/{}/{}/{} |\n",
                row["class"].as_str().unwrap_or(""),
                row["name"].as_str().unwrap_or(""),
                row["items"],
                row["flaw_instances"],
                counts(
                    &row["severity_items"],
                    &["cosmetic", "repairable", "gap", "fatal"]
                ),
                counts(
                    &row["lean_catchable_items"],
                    &["YES-KERNEL", "YES-TOOLING", "NO", "N/A"]
                ),
                counts(&row["truth"], &["true", "false", "undetermined"]),
                counts(&row["truth_eligible"], &["true", "false", "excluded"]),
                counts(&row["proof_status"], &["valid", "invalid"]),
                row["canonical_roots"],
                row["missing_canonical_length"],
                steps["n"],
                steps["min"],
                steps["median"],
                steps["max"]
            ));
        }
        out.push('\n');
    }
    let zero: Vec<String> = coverage["zero_coverage"]
        .as_array()
        .ok_or("zero coverage")?
        .iter()
        .map(|z| {
            format!(
                "{} ({})",
                z["class"].as_str().unwrap_or(""),
                z["name"].as_str().unwrap_or("")
            )
        })
        .collect();
    out.push_str(&format!(
        "## Findings\n\n- **Zero-coverage classes ({}):** {}. S4 cannot validate them.\n- **Power:** {}\n- **Harvest candidates:** {}.\n- **Undetermined-truth items ({}):** counted in the class rows; excluded from every cell that uses the truth dimension (STATE, standing rulings).\n- **Qualification:** {}.\n",
        zero.len(),
        zero.join(", "),
        coverage["interim_power"].as_str().unwrap_or(""),
        coverage["candidate_counts"],
        coverage["undetermined_truth_items"].as_array().map_or(0, Vec::len),
        coverage["qualification_counts"]
    ));
    Ok(out)
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

fn line(repo: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8(git(repo, args)?)
        .map_err(|e| e.to_string())?
        .trim()
        .to_string())
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
        return Err("usage: s1_later_prefix <repo> <new-output-directory-outside-repo>".into());
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
    let head = line(&repo, &["rev-parse", "HEAD"])?;
    let tree = line(&repo, &["rev-parse", "HEAD^{tree}"])?;
    let frozen = frozen()?;
    let baseline = snapshot(&repo, BASE)?;
    if baseline.len() != frozen.files.len()
        || baseline.iter().any(|(path, b)| {
            frozen
                .files
                .get(path)
                .is_none_or(|f| f.n_bytes != b.len() || f.sha256 != content_hash(b))
        })
    {
        return Err("preservation input list does not bind the complete accepted Git tree".into());
    }
    // ADR-0013 D2: the pins bind the five directories and the harvest log at the freeze point.
    let p = prefix()?;
    git(
        &repo,
        &["merge-base", "--is-ancestor", FREEZE_POINT, "HEAD"],
    )?;
    if line(&repo, &["rev-parse", &format!("{FREEZE_POINT}^{{tree}}")])? != p.freeze_point_tree {
        return Err("freeze point tree mismatch".into());
    }
    let at_freeze = snapshot(&repo, FREEZE_POINT)?;
    let in_prefix: BTreeMap<&String, &Vec<u8>> = at_freeze
        .iter()
        .filter(|(path, _)| {
            p.item_directories
                .iter()
                .any(|d| path.starts_with(&format!("groundtruth/{d}/")))
        })
        .collect();
    let log = bytes(&at_freeze, HARVEST_LOG)?;
    if in_prefix.len() != p.files.len()
        || in_prefix.iter().any(|(path, b)| {
            p.files
                .get(*path)
                .is_none_or(|f| f.n_bytes != b.len() || f.sha256 != content_hash(b))
        })
        || log.len() != p.harvest_log.n_bytes
        || content_hash(log) != p.harvest_log.sha256
    {
        return Err("prefix pins do not bind the freeze point's item directories and log".into());
    }
    let current = snapshot(&repo, &head)?;
    for (path, compiled) in COMPILED {
        if bytes(&current, path)? != *compiled
            || fs::read(repo.join(path)).map_err(|e| e.to_string())? != *compiled
        {
            return Err(format!("compiled/input source mismatch {path}"));
        }
    }
    if bytes(&current, PREFIX_INPUTS)? != PREFIX.as_bytes()
        || bytes(&current, OVERLAY)? != EXPECTED_OVERLAY.as_bytes()
        || bytes(&current, BASELINE)? != EXPECTED_BASELINE.as_bytes()
    {
        return Err("compiled prefix/overlay/preservation pins mismatch".into());
    }
    let event = inventory(&current)?;
    let report = render(&event.coverage)?;
    let provenance = json!({"event_id":EVENT,"checkout_revision":head,"checkout_tree":tree,"accepted_packet_revision":BASE,
        "prefix_freeze_point":FREEZE_POINT,"prefix_freeze_point_tree":p.freeze_point_tree,
        "tool_source_sha256":COMPILED.iter().map(|(path,b)|(*path,content_hash(b))).collect::<BTreeMap<_,_>>(),
        "prefix_inputs_sha256":content_hash(PREFIX.as_bytes()),"overlay_sha256":content_hash(EXPECTED_OVERLAY.as_bytes()),
        "preservation_inputs_sha256":content_hash(EXPECTED_BASELINE.as_bytes()),
        "taxonomy_version":"0.1.0","battery_version":null,"seed":null,"uncertainty":"deterministic census of pinned metadata; not a statistical estimate",
        "pinned_tools":"s1_effective_inventory and s1_4_coverage are unchanged and not run; their modules are compiled into this tool as they stand",
        "command_from_engine":"cargo run --locked -p syndeom-ingest --example s1_later_prefix -- <repo> <new-output-directory-outside-repo>",
        "no_network_or_witness_reinvestigation":true});
    clean(&repo)?;
    if line(&repo, &["rev-parse", "HEAD"])? != head {
        return Err("checkout changed during validation".into());
    }
    historical::output_dir(
        out,
        BTreeMap::from([
            ("inventory.json".to_string(), json_bytes(&event.inventory)?),
            ("coverage.json".to_string(), json_bytes(&event.coverage)?),
            ("report.md".to_string(), report.into_bytes()),
            ("provenance.json".to_string(), json_bytes(&provenance)?),
        ]),
    )?;
    println!(
        "PASS: 40 archived representations / 37 effective logical items; prefix through {} ({EVENT})",
        LAST_CANDIDATE.1
    );
    Ok(())
}
