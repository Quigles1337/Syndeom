//! `syndeom-ingest` — the deterministic ingest producer `s1_ingest` 0.1.0 (ADR-0003 D6).
//!
//! Reads a per-item config (ADR-0003 D4 fields) and the original's bytes, computes the hashes and
//! counts with `syndeom-manifest`, writes `<root>/groundtruth/<artifact_id>/manifest.json` as a
//! `syndeom-manifest/2` manifest with `origin: "ground-truth"`, and refuses to write anything that
//! fails ADR-0001 D4's consistency rules (as amended 2026-10-13) or ADR-0003's admission shape.
//! It decides nothing: every value it writes is an input or a hash.
//!
//! ```text
//! syndeom-ingest --config <item.json> --bytes <original> --root <repo root>
//!                --engine-commit <40-hex git SHA> [--dry-run]
//! ```
//!
//! Exit codes: 0 written (or identical to what exists); 1 usage or I/O error; 2 the manifest
//! fails its self-check or the config fails ADR-0003's shape; 3 the target directory holds a
//! different manifest or different bytes under the same `artifact_id` (ADR-0001 D2 invariant).

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use syndeom_manifest::{
    Artifact, Derivation, FORMAT_RAW, FileBytes, FileKey, Files, Flaw, Generator, GroundTruth,
    InformalFile, Kind, LeanCatchable, Location, MANIFEST_SCHEMA, Manifest, Origin, ProofStatus,
    RNG_NONE, RawLocation, Severity, StatementStatus, Storage, artifact_id, config_hash,
    content_hash, fidelity, to_disk_json,
};

/// ADR-0003 D6: the producer's identity. Any change to what this binary writes bumps `VERSION`.
pub const GENERATOR_ID: &str = "s1_ingest";
pub const VERSION: &str = "0.1.0";
/// `PROGRAM/FLAWS.md` version the flaw records are read against (S0.5; changed only by ADR).
pub const TAXONOMY_VERSION: &str = "0.1.0";
/// The classes FLAWS.md 0.1.0 defines (FLAW-0001..FLAW-0014). A taxonomy bump updates this list
/// and `TAXONOMY_VERSION` together.
pub const FLAW_CLASSES_0_1_0: u32 = 14;

fn required_option<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(d)
}

/// ADR-0003 D4 — the per-item config file. Every key except `notes` becomes `manifest.config`
/// verbatim; `notes` becomes `manifest.notes` (never load-bearing, so it does not enter the id).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ItemConfig {
    sources: Vec<String>,
    item: String,
    licence: String,
    status: StatusInput,
    statement_status: StatementStatus,
    proof_status: ProofStatus,
    flaws: Vec<FlawInput>,
    file: FileInput,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatusInput {
    kind: String,
    reference: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FlawInput {
    instance: u64,
    class: String,
    #[serde(deserialize_with = "required_option")]
    sub_variant: Option<String>,
    /// ADR-0001 D3 `raw/1` location: the documenting source's own locator, transcribed.
    locator: String,
    severity: Severity,
    lean_catchable: LeanCatchable,
    dials: Value,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileInput {
    name: String,
    storage: Storage,
}

struct Args {
    config: PathBuf,
    bytes: PathBuf,
    root: PathBuf,
    engine_commit: String,
    dry_run: bool,
}

const USAGE: &str = "usage: syndeom-ingest --config <item.json> --bytes <original> --root <repo root> --engine-commit <40-hex> [--dry-run]";

fn parse_args() -> Result<Args, String> {
    let mut config = None;
    let mut bytes = None;
    let mut root = None;
    let mut engine_commit = None;
    let mut dry_run = false;
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--config" => config = it.next().map(PathBuf::from),
            "--bytes" => bytes = it.next().map(PathBuf::from),
            "--root" => root = it.next().map(PathBuf::from),
            "--engine-commit" => engine_commit = it.next(),
            "--dry-run" => dry_run = true,
            other => return Err(format!("unknown argument {other:?}\n{USAGE}")),
        }
    }
    let (Some(config), Some(bytes), Some(root), Some(engine_commit)) =
        (config, bytes, root, engine_commit)
    else {
        return Err(USAGE.to_string());
    };
    if engine_commit.len() != 40
        || !engine_commit
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
    {
        return Err("--engine-commit must be a 40-character lowercase hex git SHA (SD6)".into());
    }
    Ok(Args {
        config,
        bytes,
        root,
        engine_commit,
        dry_run,
    })
}

/// ADR-0003 shape rules the crate's self-check does not know: the S1.1 status kinds, the storage
/// values a real item may use, and the flaw classes FLAWS.md 0.1.0 defines.
fn check_shape(cfg: &ItemConfig) -> Vec<String> {
    let mut problems = Vec::new();
    let is_source_id =
        |s: &str| s.len() == 6 && s.starts_with("S-") && s[2..].chars().all(|c| c.is_ascii_digit());
    if cfg.sources.is_empty() || !cfg.sources.iter().all(|s| is_source_id(s)) {
        problems.push(
            "ADR-0003 D4: sources is a non-empty list of S-NNNN ids recorded before ingestion (SD16)"
                .into(),
        );
    }
    if cfg.item.trim().is_empty() {
        problems.push(
            "ADR-0003 D4: item identifies the original (arXiv id and version, or DOI)".into(),
        );
    }
    if cfg.licence.trim().is_empty() {
        problems.push("ADR-0003 D5: licence is recorded before ingestion (SD16); empty".into());
    }
    if !matches!(cfg.status.kind.as_str(), "erratum" | "retraction") {
        problems.push(format!(
            "ADR-0003 D4: status.kind must be erratum or retraction for an S1.1 item, got {:?}",
            cfg.status.kind
        ));
    }
    if cfg.status.reference.is_empty() {
        problems.push("ADR-0003 D4: status.reference names the notice (an S-entry)".into());
    }
    if cfg.proof_status != ProofStatus::Invalid {
        problems.push(
            "ADR-0003 D3: proof_status is invalid for every S1.1 item by construction".into(),
        );
    }
    if cfg.file.storage == Storage::Regenerable {
        problems.push("ADR-0001 D2: regenerable storage is for synthetic artifacts only".into());
    }
    if cfg.file.name.is_empty()
        || cfg.file.name.contains('/')
        || cfg.file.name.contains('\\')
        || cfg.file.name == "manifest.json"
    {
        problems.push(format!(
            "ADR-0001 D2: file.name must be a bare file name other than manifest.json, got {:?}",
            cfg.file.name
        ));
    }
    if cfg.flaws.is_empty() {
        problems.push("ADR-0003 D2: an admitted item carries at least one flaw record".into());
    }
    for (i, fl) in cfg.flaws.iter().enumerate() {
        let n = fl
            .class
            .strip_prefix("FLAW-")
            .filter(|d| d.len() == 4)
            .and_then(|d| d.parse::<u32>().ok());
        if !n.is_some_and(|n| (1..=FLAW_CLASSES_0_1_0).contains(&n)) {
            problems.push(format!(
                "ADR-0003 D3: flaws[{i}].class {:?} is not a class of FLAWS.md {TAXONOMY_VERSION}",
                fl.class
            ));
        }
        if fl.locator.trim().is_empty() {
            problems.push(format!(
                "ADR-0003 D6: flaws[{i}].locator is the status document's own locator; empty"
            ));
        }
    }
    problems
}

fn build_manifest(
    cfg: &ItemConfig,
    config_value: Value,
    bytes: &[u8],
    engine_commit: &str,
) -> Result<Manifest, String> {
    let h =
        config_hash(&config_value).map_err(|e| format!("config cannot be canonicalized: {e}"))?;
    let id = artifact_id(GENERATOR_ID, VERSION, &h, None);
    let flaws = cfg
        .flaws
        .iter()
        .map(|f| Flaw {
            instance: f.instance,
            class: f.class.clone(),
            sub_variant: f.sub_variant.clone(),
            location: Location::Raw(RawLocation {
                artifact: FileKey::Informal,
                locator: f.locator.clone(),
            }),
            severity: f.severity,
            lean_catchable: f.lean_catchable,
            dials: f.dials.clone(),
        })
        .collect();
    let correct =
        cfg.statement_status.value == Some(true) && cfg.proof_status == ProofStatus::Valid;
    let commanded = BTreeMap::new();
    let achieved = BTreeMap::new();
    let fidelity = fidelity(&commanded, &achieved);
    Ok(Manifest {
        manifest_schema: MANIFEST_SCHEMA.to_string(),
        artifact_id: id,
        origin: Origin::GroundTruth,
        generator: Generator {
            id: GENERATOR_ID.to_string(),
            version: VERSION.to_string(),
            rng: RNG_NONE.to_string(),
            llm: None,
        },
        engine_commit: engine_commit.to_string(),
        seed: None,
        config: config_value,
        config_hash: h,
        toolchain: None,
        artifact: Artifact {
            kind: Kind::Informal,
            files: Files {
                informal: Some(InformalFile {
                    file: cfg.file.name.clone(),
                    format: FORMAT_RAW.to_string(),
                    storage: cfg.file.storage,
                    content_hash: content_hash(bytes),
                    n_bytes: bytes.len() as u64,
                    n_tokens: None,
                    n_steps: None,
                    n_lemmas: None,
                }),
                formal: None,
            },
        },
        ground_truth: Some(GroundTruth {
            derivation: Derivation::DocumentedBySource,
            statement_status: cfg.statement_status.clone(),
            proof_status: cfg.proof_status,
            correct,
            flaws,
            taxonomy_version: TAXONOMY_VERSION.to_string(),
        }),
        commanded,
        achieved,
        fidelity,
        notes: cfg.notes.clone(),
    })
}

/// Write `content` to `path` unless an identical file is already there; a different file is the
/// ADR-0001 D2 violation (same id, different bytes) and is refused.
fn write_or_refuse(path: &Path, content: &[u8]) -> Result<(), (u8, String)> {
    if path.exists() {
        let existing =
            std::fs::read(path).map_err(|e| (1, format!("read {}: {e}", path.display())))?;
        if existing == content {
            return Ok(());
        }
        return Err((
            3,
            format!(
                "{} exists with different bytes (ADR-0001 D2: one id, one byte sequence); not overwritten",
                path.display()
            ),
        ));
    }
    std::fs::write(path, content).map_err(|e| (1, format!("write {}: {e}", path.display())))
}

fn run() -> Result<(), (u8, String)> {
    let args = parse_args().map_err(|e| (1, e))?;
    let config_text = std::fs::read_to_string(&args.config)
        .map_err(|e| (1, format!("read {}: {e}", args.config.display())))?;
    let cfg: ItemConfig = serde_json::from_str(&config_text)
        .map_err(|e| (2, format!("config does not match ADR-0003 D4: {e}")))?;
    let mut config_value: Map<String, Value> = serde_json::from_str(&config_text)
        .map_err(|e| (2, format!("config is not a JSON object: {e}")))?;
    config_value.remove("notes");
    let config_value = Value::Object(config_value);

    let shape = check_shape(&cfg);
    if !shape.is_empty() {
        return Err((2, format!("refused:\n  {}", shape.join("\n  "))));
    }
    let bytes = std::fs::read(&args.bytes)
        .map_err(|e| (1, format!("read {}: {e}", args.bytes.display())))?;
    let manifest =
        build_manifest(&cfg, config_value, &bytes, &args.engine_commit).map_err(|e| (2, e))?;
    if let Err(problems) = manifest.check(FileBytes {
        informal: Some(&bytes),
        formal: None,
    }) {
        return Err((
            2,
            format!(
                "refused: the manifest fails ADR-0001 D4:\n  {}",
                problems.join("\n  ")
            ),
        ));
    }
    let disk = to_disk_json(&manifest).map_err(|e| (1, format!("serialize: {e}")))?;
    if args.dry_run {
        print!("{disk}");
        return Ok(());
    }
    let dir = args
        .root
        .join(Origin::GroundTruth.root())
        .join(&manifest.artifact_id);
    std::fs::create_dir_all(&dir).map_err(|e| (1, format!("mkdir {}: {e}", dir.display())))?;
    if cfg.file.storage == Storage::Committed {
        write_or_refuse(&dir.join(&cfg.file.name), &bytes)?;
    }
    write_or_refuse(&dir.join("manifest.json"), disk.as_bytes())?;
    println!("{}", manifest.artifact_id);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, msg)) => {
            eprintln!("syndeom-ingest: {msg}");
            ExitCode::from(code)
        }
    }
}
