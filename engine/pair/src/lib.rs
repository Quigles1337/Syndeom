//! ADR-0007: one reviewed statement/Mathlib argument, never a general fidelity judge.
#![forbid(unsafe_code)]

pub mod output;
pub mod provenance;
pub mod strict_json;

use serde_json::{Value, json};
use std::collections::BTreeMap;
use syndeom_manifest::*;

pub const CORRECTION_REVISION: &str = "d88fad2bb7b339c56bd9964167d3e0acddc8ee4e";
pub const MAIN_BASELINE: &str = "1a8388b03017e4044cad01ac10e7e89f8fb5e38f";
pub const MATHLIB: &str = "d13f23b723b8a846827a245b89c10fc7d3f11612";
pub const LEAN: &str = "leanprover/lean4:v4.34.1";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ARCHIVE_HASH: &str = "094625fa10ffde723ba3a3ae47ff75db2b1994b8f96f26f6eb562dcbf653ede0";
pub const CHILD_HASH: &str = "62fe0c816a6b0ef928f3e4c2fb7e2bd2e215171ebf6b6130202e851e1e9664a3";
pub const ALIAS: &[u8] = include_bytes!("../../../analysis/s1-2/Proof.lean");
const ARCHIVE: &[u8] = include_bytes!("../../../analysis/s1-2/archive-config.proposed.json");
const CHILD: &[u8] = include_bytes!("../../../analysis/s1-2/canonical-config.proposed.json");
const DECL: &str = "theorem infinite_setOfPred_prime : { p | Prime p }.Infinite :=";
const BODY: &str = "  Set.infinite_of_not_bddAbove not_bddAbove_setOfPred_prime";
const ALIAS_DECL: &str =
    "theorem syndeom_s1_2_judson_primes : Set.Infinite {p : ℕ | Nat.Prime p} :=";
pub type Sources = BTreeMap<String, Vec<u8>>;
pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Archive,
    Canonical,
}

impl Mode {
    pub fn generator(self) -> &'static str {
        match self {
            Self::Archive => "s1_pair_archive",
            Self::Canonical => "s1_pair",
        }
    }
}

pub fn require(ok: bool, message: &str) -> Result<()> {
    if ok { Ok(()) } else { Err(message.into()) }
}

pub fn reviewed_config(mode: Mode) -> Result<Value> {
    let (bytes, expected) = match mode {
        Mode::Archive => (ARCHIVE, ARCHIVE_HASH),
        Mode::Canonical => (CHILD, CHILD_HASH),
    };
    let config = strict_json::parse(bytes)?;
    require(
        config_hash(&config).map_err(|e| e.to_string())? == expected,
        "compiled configuration differs from the corrected reviewed contract",
    )?;
    Ok(config)
}

/// Exact full-object equality also rejects unknown/missing fields at any depth.
pub fn validate_config(config: &Value, mode: Mode) -> Result<()> {
    require(
        *config == reviewed_config(mode)?,
        "unsupported/unreviewed configuration, metadata, source, declaration, type or recipe",
    )
}

pub fn context_specs() -> Result<Vec<Value>> {
    Ok(
        reviewed_config(Mode::Canonical)?["recipe"]["required_external_context"]
            .as_array()
            .ok_or("missing reviewed context")?
            .clone(),
    )
}

pub fn validate_sources(sources: &Sources) -> Result<()> {
    let specs = context_specs()?;
    require(
        sources.len() == specs.len(),
        "missing or unsupported context source",
    )?;
    for spec in specs {
        let name = spec["file"].as_str().ok_or("invalid source file")?;
        let bytes = sources
            .get(name)
            .ok_or_else(|| format!("missing source: {name}"))?;
        require(
            content_hash(bytes) == spec["sha256"].as_str().ok_or("missing hash")?
                && Some(bytes.len() as u64) == spec["n_bytes"].as_u64(),
            &format!("changed/unreviewed source bytes: {name}"),
        )?;
    }
    source_map_check(
        &sources["PrimeFin.lean"],
        ALIAS,
        &reviewed_config(Mode::Archive)?,
        &reviewed_config(Mode::Canonical)?,
    )
}

/// Narrow literal source review: one-based inclusive source interval, one-based alias line.
/// This checks the actual declaration/body slice independently of a caller's config hash.
pub fn source_map_check(raw: &[u8], alias: &[u8], archive: &Value, child: &Value) -> Result<()> {
    let lines: Vec<_> = std::str::from_utf8(raw)
        .map_err(|e| e.to_string())?
        .split('\n')
        .collect();
    let mapping = &child["recipe"]["source_map"]["formal"];
    let interval = mapping["source_lines_one_based"]
        .as_array()
        .ok_or("invalid source interval")?;
    require(interval.len() == 2, "invalid source interval length")?;
    let first = interval[0].as_u64().ok_or("invalid source start")?;
    let last = interval[1].as_u64().ok_or("invalid source end")?;
    require(
        first > 0 && first <= last && last <= lines.len() as u64,
        "source interval out of bounds",
    )?;
    require(
        lines[(first - 1) as usize..last as usize] == [DECL, BODY],
        "source interval omits or changes the reviewed declaration/proof body",
    )?;
    require(
        first == 27
            && last == 28
            && archive["files"]["formal"]["locator"] == "Nat.infinite_setOfPred_prime, lines 27-28"
            && mapping["source_file"] == "PrimeFin.lean"
            && mapping["source_declaration"] == "Nat.infinite_setOfPred_prime",
        "wrong reviewed source identity/locator",
    )?;
    let target: Vec<_> = std::str::from_utf8(alias)
        .map_err(|e| e.to_string())?
        .split('\n')
        .collect();
    require(
        mapping["canonical_line"] == 3
            && mapping["canonical_declaration"] == "syndeom_s1_2_judson_primes"
            && target.get(2..4) == Some(&[ALIAS_DECL, "  Nat.infinite_setOfPred_prime"][..]),
        "wrong canonical declaration location/type/proof",
    )?;
    Ok(())
}

pub fn canonical_bytes() -> Result<(Vec<u8>, Vec<u8>)> {
    let child = reviewed_config(Mode::Canonical)?;
    let recipe = &child["recipe"];
    let informal = format!(
        "# {}\n\n**Theorem.** {}\n\n**Proof.**\n\n∎\n",
        recipe["title"].as_str().ok_or("missing title")?,
        recipe["statement"].as_str().ok_or("missing statement")?
    )
    .into_bytes();
    let formal = ALIAS.to_vec();
    require(
        content_hash(&informal) == recipe["informal_sha256"]
            && content_hash(&formal) == recipe["formal_sha256"],
        "canonical template byte drift",
    )?;
    let i = parse_informal_md(&informal).map_err(|e| e.to_string())?;
    let f = parse_lean(&formal).map_err(|e| e.to_string())?;
    require(
        i.n_steps == 0
            && i.n_lemmas == 0
            && i.steps == ["There exist an infinite number of primes."]
            && f.declarations == [(3, "syndeom_s1_2_judson_primes".into())]
            && f.imports == ["Mathlib.Data.Nat.PrimeFin"],
        "reviewed canonical shape drift",
    )?;
    Ok((informal, formal))
}

fn expected_manifest(mode: Mode, commit: &str) -> Result<Manifest> {
    require(
        commit.len() == 40
            && commit
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "engine_commit must be full lowercase 40-hex",
    )?;
    let config = reviewed_config(mode)?;
    let hash = config_hash(&config).map_err(|e| e.to_string())?;
    let archive = reviewed_config(Mode::Archive)?;
    let files = if mode == Mode::Archive {
        Files {
            informal: Some(InformalFile {
                file: "aata-20250801.pdf".into(),
                format: FORMAT_RAW.into(),
                storage: Storage::External,
                content_hash: archive["files"]["informal"]["sha256"]
                    .as_str()
                    .unwrap()
                    .into(),
                n_bytes: archive["files"]["informal"]["n_bytes"].as_u64().unwrap(),
                n_tokens: None,
                n_steps: None,
                n_lemmas: None,
            }),
            formal: Some(FormalFile {
                file: "PrimeFin.lean".into(),
                format: FORMAT_RAW.into(),
                storage: Storage::External,
                content_hash: archive["files"]["formal"]["sha256"]
                    .as_str()
                    .unwrap()
                    .into(),
                n_bytes: archive["files"]["formal"]["n_bytes"].as_u64().unwrap(),
                n_tokens: None,
                n_lines: None,
                n_declarations: None,
                main_declaration: None,
            }),
        }
    } else {
        let (informal, formal) = canonical_bytes()?;
        let i = parse_informal_md(&informal).map_err(|e| e.to_string())?;
        let f = parse_lean(&formal).map_err(|e| e.to_string())?;
        Files {
            informal: Some(InformalFile {
                file: FILE_INFORMAL.into(),
                format: FORMAT_INFORMAL_MD.into(),
                storage: Storage::External,
                content_hash: content_hash(&informal),
                n_bytes: i.n_bytes,
                n_tokens: Some(i.n_tokens),
                n_steps: Some(i.n_steps),
                n_lemmas: Some(i.n_lemmas),
            }),
            formal: Some(FormalFile {
                file: FILE_FORMAL.into(),
                format: FORMAT_LEAN.into(),
                storage: Storage::Committed,
                content_hash: content_hash(&formal),
                n_bytes: f.n_bytes,
                n_tokens: Some(f.n_tokens),
                n_lines: Some(f.n_lines),
                n_declarations: Some(f.n_declarations),
                main_declaration: Some("syndeom_s1_2_judson_primes".into()),
            }),
        }
    };
    let commanded = BTreeMap::new();
    let achieved = BTreeMap::new();
    Ok(Manifest {
        manifest_schema: MANIFEST_SCHEMA.into(),
        artifact_id: artifact_id(mode.generator(), VERSION, &hash, None),
        origin: Origin::GroundTruth,
        generator: Generator { id: mode.generator().into(), version: VERSION.into(), rng: RNG_NONE.into(), llm: None },
        engine_commit: commit.into(), seed: None,
        ground_truth: Some(serde_json::from_value(config["ground_truth"].clone()).map_err(|e| e.to_string())?),
        toolchain: Some(serde_json::from_value(config["toolchain"].clone()).map_err(|e| e.to_string())?),
        config, config_hash: hash, artifact: Artifact { kind: Kind::Pair, files },
        fidelity: fidelity(&commanded, &achieved), commanded, achieved,
        notes: Some("ADR-0007 reviewed Judson statement / Mathlib offered argument. Zero selected informal proof steps. Empty-statistics PASS is vacuous for semantics. Branch generation is not accepted-main admission or PROVEN promotion.".into()),
    })
}

pub fn parse_manifest(bytes: &[u8]) -> Result<Manifest> {
    serde_json::from_value(strict_json::parse(bytes)?).map_err(|e| e.to_string())
}

pub fn validate_parent_record(parent: &Manifest, commit: &str) -> Result<()> {
    parent
        .check(FileBytes::default())
        .map_err(|e| e.join("\n"))?;
    require(
        *parent == expected_manifest(Mode::Archive, commit)?,
        "wrong actual parent identity/config/metadata/toolchain/producer; require reviewed raw pair",
    )
}

pub fn validate_archive(parent: &Manifest, sources: &Sources, commit: &str) -> Result<()> {
    validate_sources(sources)?;
    validate_parent_record(parent, commit)?;
    parent
        .check(FileBytes {
            informal: Some(&sources["aata-20250801.pdf"]),
            formal: Some(&sources["PrimeFin.lean"]),
        })
        .map_err(|e| e.join("\n"))
}

pub fn validate_child_record(
    child: &Manifest,
    parent: &Manifest,
    informal: &[u8],
    formal: &[u8],
    commit: &str,
) -> Result<()> {
    validate_parent_record(parent, commit)?;
    child
        .check(FileBytes {
            informal: Some(informal),
            formal: Some(formal),
        })
        .map_err(|e| e.join("\n"))?;
    child.check_derived(parent).map_err(|e| e.join("\n"))?;
    require(
        *child == expected_manifest(Mode::Canonical, commit)?,
        "unreviewed child manifest/config/recipe/metadata/producer",
    )?;
    for key in [
        "sources",
        "item",
        "licence",
        "status",
        "ground_truth",
        "toolchain",
    ] {
        require(
            child.config[key] == parent.config[key],
            "inherited metadata differs",
        )?;
    }
    require(
        child.ground_truth == parent.ground_truth && child.toolchain == parent.toolchain,
        "ground truth/toolchain changed",
    )?;
    let expected = canonical_bytes()?;
    require(
        informal == expected.0 && formal == expected.1,
        "unreviewed canonical bytes",
    )
}

pub struct Product {
    pub manifest: Manifest,
    pub source_map: Value,
    pub committed: Sources,
    pub external: Sources,
    pub parent_manifest_sha256: Option<String>,
}

pub fn archive(config: &Value, sources: &Sources, commit: &str) -> Result<Product> {
    validate_config(config, Mode::Archive)?;
    let manifest = expected_manifest(Mode::Archive, commit)?;
    validate_archive(&manifest, sources, commit)?;
    let source_map = json!({
        "artifact_id": manifest.artifact_id, "method": config["method"],
        "source_interval_convention": "one-based inclusive",
        "formal_source_lines": [27, 28], "formal_declaration": "Nat.infinite_setOfPred_prime",
        "sources": config["files"], "attribution": config["attribution"],
        "claim_role": config["claim_role"], "offered_argument": config["offered_argument"],
    });
    Ok(Product {
        manifest,
        source_map,
        committed: Sources::new(),
        external: sources.clone(),
        parent_manifest_sha256: None,
    })
}

pub fn canonical(
    config: &Value,
    parent_bytes: &[u8],
    sources: &Sources,
    commit: &str,
) -> Result<Product> {
    validate_config(config, Mode::Canonical)?;
    let parent = parse_manifest(parent_bytes)?;
    validate_archive(&parent, sources, commit)?;
    require(
        parent_bytes == to_disk_json(&parent).map_err(|e| e.to_string())?.as_bytes(),
        "parent manifest must retain deterministic on-disk bytes",
    )?;
    let manifest = expected_manifest(Mode::Canonical, commit)?;
    let (informal, formal) = canonical_bytes()?;
    validate_child_record(&manifest, &parent, &informal, &formal, commit)?;
    let parent_hash = content_hash(parent_bytes);
    let source_map = json!({
        "artifact_id": manifest.artifact_id, "derived_from": parent.artifact_id,
        "parent_manifest_sha256": parent_hash, "parent_config_sha256": parent.config_hash,
        "method": config["method"], "source_map": config["recipe"]["source_map"],
        "source_interval_convention": "one-based inclusive; canonical line one-based; statement scalar span zero-based half-open",
        "attribution": parent.config["attribution"],
        "required_external_context": config["recipe"]["required_external_context"],
    });
    Ok(Product {
        manifest,
        source_map,
        committed: BTreeMap::from([("Proof.lean".into(), formal)]),
        external: BTreeMap::from([("proof.md".into(), informal)]),
        parent_manifest_sha256: Some(parent_hash),
    })
}

#[cfg(test)]
#[path = "../tests/support/contract.rs"]
mod tests;
