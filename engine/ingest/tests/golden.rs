//! Golden tests for `syndeom-ingest` (`s1_ingest` 0.1.0, ADR-0003 D6).
//!
//! Fixed bytes and fixed configs; the expected config hashes and ids were computed independently
//! with Python (`json.dumps(sort_keys=True, separators=(",", ":"))` + SHA-256) in Session 10 and
//! the fixture digest with `sha256sum`; the manifest digests were pinned from the first run after
//! every field had been checked against those independent values. If this binary ever disagrees
//! with them, the binary is wrong until proven otherwise.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{Value, json};
use syndeom_manifest::{
    FileBytes, Location, MANIFEST_SCHEMA, Origin, ProofStatus, StatementMethod, Storage,
    content_hash, from_json_str, sha256_hex,
};

const FIXTURE: &[u8] = include_bytes!("fixtures/original.bin");
const FIXTURE_HASH: &str = "915d3ed6d9f7e388466bd03ea1f11f603fa57477fdf3724e6327537698ad1390";
const ITEM_A: &str = include_str!("fixtures/item-a.json");
const ITEM_B: &str = include_str!("fixtures/item-b.json");
const A_CONFIG_HASH: &str = "d2134e7fd2c3f7042d08f00f5a6536322e145f110722247532932a9337f8f0a7";
const A_ID: &str = "s1_ingest-v0.1.0-d2134e7fd2c3";
const B_CONFIG_HASH: &str = "4f5d4e93adaa08b8191c53d7ccfa2a57c4f49126822c84b9b9ce2a6e22ac59e0";
const B_ID: &str = "s1_ingest-v0.1.0-4f5d4e93adaa";
const A_MANIFEST_HASH: &str = "1f2cf7b9230a74d26f4aff5128140ad997a53d76ca562c1e06a39224bf951f63";
const B_MANIFEST_HASH: &str = "451bde2d114eacdde2b8b7aeb3404efdb6b594121e4835614a19075e609b8340";
const ZERO_COMMIT: &str = "0000000000000000000000000000000000000000";

struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new(tag: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "syndeom-ingest-golden-{}-{tag}",
            std::process::id()
        ));
        if root.exists() {
            std::fs::remove_dir_all(&root).unwrap();
        }
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("original.bin"), FIXTURE).unwrap();
        Self { root }
    }

    fn write_config(&self, name: &str, text: &str) -> PathBuf {
        let p = self.root.join(name);
        std::fs::write(&p, text).unwrap();
        p
    }

    fn run(&self, config: &Path, extra: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_syndeom-ingest"))
            .arg("--config")
            .arg(config)
            .arg("--bytes")
            .arg(self.root.join("original.bin"))
            .arg("--root")
            .arg(&self.root)
            .arg("--engine-commit")
            .arg(ZERO_COMMIT)
            .args(extra)
            .output()
            .expect("the binary runs")
    }

    fn item_dir(&self, id: &str) -> PathBuf {
        self.root.join(Origin::GroundTruth.root()).join(id)
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

#[test]
fn fixture_bytes_are_the_golden_bytes() {
    assert_eq!(FIXTURE.len(), 72);
    assert_eq!(content_hash(FIXTURE), FIXTURE_HASH);
}

#[test]
fn golden_a_boolean_truth_external_storage() {
    let sb = Sandbox::new("a");
    let cfg = sb.write_config("item-a.json", ITEM_A);
    let out = sb.run(&cfg, &[]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), A_ID);
    let dir = sb.item_dir(A_ID);
    assert!(
        !dir.join("original.bin").exists(),
        "external storage copies nothing"
    );
    let disk = std::fs::read(dir.join("manifest.json")).unwrap();
    assert_eq!(sha256_hex(&disk), A_MANIFEST_HASH);
    let text = std::str::from_utf8(&disk).unwrap();
    assert!(text.ends_with('\n') && !text.ends_with("\n\n") && !text.contains('\r'));
    let m = from_json_str(text).unwrap();
    m.check(FileBytes {
        informal: Some(FIXTURE),
        formal: None,
    })
    .unwrap_or_else(|p| panic!("golden A fails ADR-0001 D4: {p:#?}"));
    assert_eq!(m.manifest_schema, MANIFEST_SCHEMA);
    assert_eq!(m.artifact_id, A_ID);
    assert_eq!(m.config_hash, A_CONFIG_HASH);
    assert_eq!(m.origin, Origin::GroundTruth);
    assert_eq!(m.seed, None);
    assert!(m.toolchain.is_none());
    assert_eq!(
        (
            m.generator.id.as_str(),
            m.generator.version.as_str(),
            m.generator.rng.as_str()
        ),
        ("s1_ingest", "0.1.0", "none")
    );
    assert!(m.config.get("notes").is_none(), "notes never enter the id");
    assert_eq!(
        m.notes.as_deref(),
        Some("ADR-0003 D6 golden fixture A; S-0000 is a placeholder, not a register entry")
    );
    let f = m.artifact.files.informal.as_ref().unwrap();
    assert_eq!(
        (
            f.file.as_str(),
            f.format.as_str(),
            f.storage,
            f.content_hash.as_str(),
            f.n_bytes
        ),
        ("original.bin", "raw/1", Storage::External, FIXTURE_HASH, 72)
    );
    assert_eq!((f.n_tokens, f.n_steps, f.n_lemmas), (None, None, None));
    let gt = m.ground_truth.as_ref().unwrap();
    assert_eq!(gt.statement_status.value, Some(true));
    assert_eq!(gt.statement_status.method, StatementMethod::SourceDocument);
    assert_eq!(gt.proof_status, ProofStatus::Invalid);
    assert!(!gt.correct);
    assert_eq!(gt.taxonomy_version, "0.1.0");
    assert_eq!(gt.flaws.len(), 1);
    assert!(
        matches!(&gt.flaws[0].location, Location::Raw(r) if r.locator == "Lemma 2.1, proof, first display")
    );
    assert!(m.commanded.is_empty() && m.achieved.is_empty());
    // idempotent: a second run writes nothing new and succeeds
    let again = sb.run(&cfg, &[]);
    assert!(again.status.success(), "{}", stderr(&again));
    assert_eq!(std::fs::read(dir.join("manifest.json")).unwrap(), disk);
}

#[test]
fn golden_b_undetermined_truth_committed_storage() {
    let sb = Sandbox::new("b");
    let cfg = sb.write_config("item-b.json", ITEM_B);
    let out = sb.run(&cfg, &[]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), B_ID);
    let dir = sb.item_dir(B_ID);
    assert_eq!(
        std::fs::read(dir.join("original.bin")).unwrap(),
        FIXTURE,
        "committed storage copies the bytes verbatim"
    );
    let disk = std::fs::read(dir.join("manifest.json")).unwrap();
    assert_eq!(sha256_hex(&disk), B_MANIFEST_HASH);
    let m = from_json_str(std::str::from_utf8(&disk).unwrap()).unwrap();
    m.check(FileBytes {
        informal: Some(FIXTURE),
        formal: None,
    })
    .unwrap_or_else(|p| panic!("golden B fails ADR-0001 D4: {p:#?}"));
    assert_eq!(m.artifact_id, B_ID);
    assert_eq!(m.config_hash, B_CONFIG_HASH);
    let gt = m.ground_truth.as_ref().unwrap();
    assert_eq!(gt.statement_status.value, None);
    assert_eq!(gt.statement_status.method, StatementMethod::Undetermined);
    assert_eq!(gt.proof_status, ProofStatus::Invalid);
    assert!(!gt.correct);
    assert_eq!(gt.flaws.len(), 2);
    assert_eq!(gt.flaws[0].sub_variant.as_deref(), Some("propagated"));
    assert_eq!(gt.flaws[1].sub_variant, None);
    // the null is written as a literal null, key present (ADR-0001 D4: absence is not null)
    let raw: Value = serde_json::from_slice(&disk).unwrap();
    assert_eq!(
        raw["ground_truth"]["statement_status"]["value"],
        Value::Null
    );
}

#[test]
fn dry_run_prints_the_manifest_and_writes_nothing() {
    let sb = Sandbox::new("dry");
    let cfg = sb.write_config("item-a.json", ITEM_A);
    let out = sb.run(&cfg, &["--dry-run"]);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(sha256_hex(&out.stdout), A_MANIFEST_HASH);
    assert!(!sb.root.join("groundtruth").exists());
}

fn with(text: &str, edit: impl FnOnce(&mut Value)) -> String {
    let mut v: Value = serde_json::from_str(text).unwrap();
    edit(&mut v);
    serde_json::to_string(&v).unwrap()
}

#[test]
fn refusals() {
    let sb = Sandbox::new("refuse");
    // exit 2: a valid proof on an S1.1 item (ADR-0003 D3) — also the null ⇒ invalid rule
    let cfg = sb.write_config(
        "valid.json",
        &with(ITEM_B, |v| v["proof_status"] = json!("valid")),
    );
    let out = sb.run(&cfg, &[]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(stderr(&out).contains("proof_status is invalid for every S1.1 item"));
    // exit 2: a class outside FLAWS.md 0.1.0
    let cfg = sb.write_config(
        "class.json",
        &with(ITEM_A, |v| v["flaws"][0]["class"] = json!("FLAW-0099")),
    );
    let out = sb.run(&cfg, &[]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(stderr(&out).contains("not a class of FLAWS.md 0.1.0"));
    // exit 2: regenerable storage on a real item
    let cfg = sb.write_config(
        "regen.json",
        &with(ITEM_A, |v| v["file"]["storage"] = json!("regenerable")),
    );
    let out = sb.run(&cfg, &[]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    // exit 2: a null truth with a non-undetermined method reaches the crate's D4 rules
    let cfg = sb.write_config(
        "method.json",
        &with(ITEM_B, |v| {
            v["statement_status"]["method"] = json!("source-document")
        }),
    );
    let out = sb.run(&cfg, &[]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(stderr(&out).contains("null iff method"));
    // exit 2: an unknown config key (deny_unknown_fields) and a wrong-schema-shaped file
    let cfg = sb.write_config(
        "extra.json",
        &with(ITEM_A, |v| v["created"] = json!("2026")),
    );
    let out = sb.run(&cfg, &[]);
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    // exit 1: a malformed engine commit
    let cfg = sb.write_config("ok.json", ITEM_A);
    let out = Command::new(env!("CARGO_BIN_EXE_syndeom-ingest"))
        .args(["--config"])
        .arg(&cfg)
        .arg("--bytes")
        .arg(sb.root.join("original.bin"))
        .arg("--root")
        .arg(&sb.root)
        .args(["--engine-commit", "abc"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1), "{}", stderr(&out));
    assert!(
        !sb.root.join("groundtruth").exists(),
        "nothing written so far"
    );
    // exit 3: the same id with different bytes already on disk (ADR-0001 D2)
    let out = sb.run(&cfg, &[]);
    assert!(out.status.success(), "{}", stderr(&out));
    let manifest_path = sb.item_dir(A_ID).join("manifest.json");
    std::fs::write(&manifest_path, b"{}\n").unwrap();
    let out = sb.run(&cfg, &[]);
    assert_eq!(out.status.code(), Some(3), "{}", stderr(&out));
    assert!(stderr(&out).contains("different bytes"));
    assert_eq!(
        std::fs::read(&manifest_path).unwrap(),
        b"{}\n",
        "not overwritten"
    );
}
