//! Artificial mechanical fixtures only. No fixture is a real admitted item or mathematical
//! evidence, and none is written into groundtruth/.
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::json;
use syndeom_derive::*;
use syndeom_manifest::*;

const COMMIT: &str = "0000000000000000000000000000000000000000"; // mechanical provenance only
const SOURCE: &str = "\\newcommand{\\A}{A}\n\\begin{document}\nDefinitions here.\nPrerequisite here.\n\
\\begin{proposition}\\label{fixture}\nα = α.\n\\end{proposition}\n\\begin{proof}\n\
Lemma 1. α = α.\n\nUse α.\n\\end{proof}\n\\begin{thebibliography}{1}\n\
\\bibitem{example} Mechanical reference.\n\\end{thebibliography}\n\\end{document}\n";

fn slice(source: &[u8], start: usize, end: usize) -> Slice {
    Slice {
        range: [start, end],
        sha256: content_hash(&source[start..end]),
    }
}

fn find(text: &str) -> Slice {
    let start = SOURCE.find(text).unwrap();
    slice(SOURCE.as_bytes(), start, start + text.len())
}

fn gzip(bytes: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(bytes).unwrap();
    encoder.finish().unwrap()
}

fn fixture() -> (Vec<u8>, Vec<u8>, Recipe) {
    let raw = gzip(SOURCE.as_bytes());
    let config = json!({"sources":["S-0001"], "item":"ARTIFICIAL MECHANICAL FIXTURE, not a corpus item",
        "licence":"Apache-2.0 fixture authored for this test", "status":{"kind":"erratum", "reference":"S-0001"}});
    let hash = config_hash(&config).unwrap();
    let parent = Manifest {
        manifest_schema: MANIFEST_SCHEMA.into(),
        artifact_id: artifact_id("fixture", "0.1.0", &hash, None),
        origin: Origin::GroundTruth, // exercises real-item SHAPE only
        generator: Generator {
            id: "fixture".into(),
            version: "0.1.0".into(),
            rng: RNG_NONE.into(),
            llm: None,
        },
        engine_commit: COMMIT.into(),
        seed: None,
        config,
        config_hash: hash,
        toolchain: None,
        artifact: Artifact {
            kind: Kind::Informal,
            files: Files {
                informal: Some(InformalFile {
                    file: "fixture.gz".into(),
                    format: FORMAT_RAW.into(),
                    storage: Storage::External,
                    content_hash: content_hash(&raw),
                    n_bytes: raw.len() as u64,
                    n_tokens: None,
                    n_steps: None,
                    n_lemmas: None,
                }),
                formal: None,
            },
        },
        ground_truth: Some(GroundTruth {
            derivation: Derivation::DocumentedBySource,
            statement_status: StatementStatus {
                value: Some(true),
                method: StatementMethod::SourceDocument,
                reference: "S-0001 mechanical status only".into(),
            },
            proof_status: ProofStatus::Invalid,
            correct: false,
            taxonomy_version: "0.1.0".into(),
            flaws: vec![Flaw {
                instance: 1,
                class: "FLAW-0005".into(),
                sub_variant: None,
                location: Location::Raw(RawLocation {
                    artifact: FileKey::Informal,
                    locator: "fixture statement".into(),
                }),
                severity: Severity::Gap,
                lean_catchable: LeanCatchable::YesKernel,
                dials: json!({}),
            }],
        }),
        commanded: BTreeMap::new(),
        achieved: BTreeMap::new(),
        fidelity: fidelity(&BTreeMap::new(), &BTreeMap::new()),
        notes: None,
    };
    let parent_bytes = to_disk_json(&parent).unwrap().into_bytes();
    let recipe = Recipe {
        method: METHOD.into(),
        transformation: "ascii-whitespace/1".into(),
        parent_manifest_sha256: content_hash(&parent_bytes),
        context_sha256: content_hash(SOURCE.as_bytes()),
        environment: "proposition".into(),
        label: "fixture".into(),
        title: "Mechanical fixture".into(),
        statement: find("α = α."),
        proof: vec![find("Lemma 1. α = α."), find("Use α.")],
        context: vec![
            Context {
                role: "preamble".into(),
                slice: slice(
                    SOURCE.as_bytes(),
                    0,
                    SOURCE.find("\\begin{document}").unwrap(),
                ),
            },
            Context {
                role: "definitions".into(),
                slice: find("Definitions here."),
            },
            Context {
                role: "prerequisite".into(),
                slice: find("Prerequisite here."),
            },
            Context {
                role: "bibliography".into(),
                slice: slice(
                    SOURCE.as_bytes(),
                    SOURCE.find("\\begin{thebibliography}").unwrap(),
                    SOURCE.find("\\end{thebibliography}").unwrap() + "\\end{thebibliography}".len(),
                ),
            },
        ],
        mappings: vec![Mapping {
            instance: 1,
            step: 0,
            span: [0, 6],
            text_sha256: content_hash("α = α.".as_bytes()),
        }],
    };
    (parent_bytes, raw, recipe)
}

#[test]
fn mechanical_golden_and_independent_regeneration() {
    let (parent_bytes, raw, recipe) = fixture();
    let a = derive(&parent_bytes, &raw, &recipe, COMMIT).unwrap();
    // Fresh input decoding and a second independent derivation; no reuse of the first Product.
    let round_trip: Recipe = serde_json::from_slice(&serde_json::to_vec(&recipe).unwrap()).unwrap();
    let b = derive(&parent_bytes.clone(), &raw.clone(), &round_trip, COMMIT).unwrap();
    let expected = "# Mechanical fixture\n\n**Theorem.** α = α.\n\n**Proof.**\n\n\
**Step 1.** Lemma 1. α = α.\n\n**Step 2.** Use α.\n\n∎\n";
    assert_eq!(a.proof, expected.as_bytes());
    assert_eq!(a.proof, b.proof);
    assert_eq!(a.context, SOURCE.as_bytes());
    assert_eq!(a.context, b.context);
    assert_eq!(a.source_map, b.source_map);
    let manifest = to_disk_json(&a.manifest).unwrap();
    assert_eq!(manifest, to_disk_json(&b.manifest).unwrap());
    assert_eq!(a.manifest, from_json_str(&manifest).unwrap());
    let stats = parse_informal_md(&a.proof).unwrap();
    assert_eq!((stats.n_steps, stats.n_lemmas, stats.n_tokens), (2, 1, 10));
    assert_eq!(span_text(&stats.steps[0], [0, 6]), Some("α = α.".into()));
    // Fixed goldens inserted from an independent SHA-256/JCS calculation in the test evidence.
    assert_eq!(
        content_hash(&a.proof),
        "90da82a426d54dad8a6772d2d75ea500eb249fd79e598ba22561a1e2cc54d15b"
    );
    assert_eq!(a.manifest.artifact_id, "s1_derive-v0.1.0-9a493194d950");
    assert_eq!(
        content_hash(manifest.as_bytes()),
        "e740bfc62c7d86598f9fa0adacd3ffe2820c6008e5e6b8d559fc432660fe0c2f"
    );
}

#[test]
fn refuses_changed_source_and_parent_bytes() {
    let (parent, mut raw, recipe) = fixture();
    raw[10] ^= 1;
    assert!(derive(&parent, &raw, &recipe, COMMIT).is_err());
    let (mut parent, raw, recipe) = fixture();
    parent.push(b'\n');
    assert!(derive(&parent, &raw, &recipe, COMMIT).is_err());
}

#[test]
fn refuses_invalid_location_mapping() {
    let (parent, raw, mut recipe) = fixture();
    for (step, span) in [(9, [0, 6]), (0, [0, 8]), (0, [0, 0]), (0, [1, 6])] {
        recipe.mappings[0].step = step;
        recipe.mappings[0].span = span;
        assert!(derive(&parent, &raw, &recipe, COMMIT).is_err());
    }
    recipe.mappings.clear();
    assert!(derive(&parent, &raw, &recipe, COMMIT).is_err());
}

#[test]
fn refuses_incompatible_inherited_metadata() {
    let (parent_bytes, raw, recipe) = fixture();
    let parent = from_json_str(std::str::from_utf8(&parent_bytes).unwrap()).unwrap();
    let product = derive(&parent_bytes, &raw, &recipe, COMMIT).unwrap();
    for field in ["truth", "instance", "dials", "licence"] {
        let mut child = product.manifest.clone();
        let gt = child.ground_truth.as_mut().unwrap();
        match field {
            "truth" => gt.statement_status.value = Some(false),
            "instance" => gt.flaws[0].instance = 2,
            "dials" => gt.flaws[0].dials = json!({"changed":true}),
            "licence" => child.config["licence"] = json!("changed"),
            _ => unreachable!(),
        }
        assert!(validate_child(&child, &parent, &product.proof).is_err());
    }
}

#[test]
fn refuses_missing_context_omissions_and_unsupported_transformations() {
    let (parent, raw, recipe) = fixture();
    let mut r = recipe.clone();
    r.context.pop();
    assert!(derive(&parent, &raw, &r, COMMIT).is_err());
    let mut r = recipe.clone();
    r.context[0].slice.range[1] -= 1;
    assert!(derive(&parent, &raw, &r, COMMIT).is_err());
    let mut r = recipe.clone();
    r.proof.remove(0);
    assert!(derive(&parent, &raw, &r, COMMIT).is_err());
    let mut r = recipe.clone();
    r.proof[1] = r.proof[0].clone();
    assert!(derive(&parent, &raw, &r, COMMIT).is_err());
    let mut r = recipe.clone();
    let [s, e] = r.proof[0].range;
    // Dividing a word is not a source paragraph boundary, even with valid slice hashes.
    r.proof[0] = slice(SOURCE.as_bytes(), s, s + 2);
    r.proof.insert(1, slice(SOURCE.as_bytes(), s + 2, e));
    assert!(derive(&parent, &raw, &r, COMMIT).is_err());
    let mut r = recipe;
    r.transformation = "repair-proof".into();
    assert!(derive(&parent, &raw, &r, COMMIT).is_err());
}

fn mutated_source(source: &[u8]) -> (Vec<u8>, Vec<u8>, Recipe) {
    let (parent_bytes, _, mut r) = fixture();
    let mut parent = from_json_str(std::str::from_utf8(&parent_bytes).unwrap()).unwrap();
    let raw = gzip(source);
    let file = parent.artifact.files.informal.as_mut().unwrap();
    file.content_hash = content_hash(&raw);
    file.n_bytes = raw.len() as u64;
    let parent = to_disk_json(&parent).unwrap().into_bytes();
    r.parent_manifest_sha256 = content_hash(&parent);
    r.context_sha256 = content_hash(source);
    (parent, raw, r)
}

#[test]
fn refuses_ambiguous_boundaries_and_non_utf8_selected_text() {
    let duplicate = format!("{SOURCE}\\label{{fixture}}\n");
    let (parent, raw, recipe) = mutated_source(duplicate.as_bytes());
    assert!(
        derive(&parent, &raw, &recipe, COMMIT)
            .err()
            .unwrap()
            .contains("ambiguous")
    );
    let mut source = SOURCE.as_bytes().to_vec();
    let s = SOURCE.find('α').unwrap();
    source[s] = 0xff;
    let (parent, raw, mut recipe) = mutated_source(&source);
    recipe.statement.sha256 =
        content_hash(&source[recipe.statement.range[0]..recipe.statement.range[1]]);
    assert!(
        derive(&parent, &raw, &recipe, COMMIT)
            .err()
            .unwrap()
            .contains("UTF-8")
    );
}

#[test]
fn refuses_comments_and_multiple_gzip_members() {
    let mut source = SOURCE.as_bytes().to_vec();
    let s = SOURCE.find("Lemma 1.").unwrap();
    source[s] = b'%';
    let (parent, raw, mut recipe) = mutated_source(&source);
    recipe.proof[0].sha256 =
        content_hash(&source[recipe.proof[0].range[0]..recipe.proof[0].range[1]]);
    assert!(
        derive(&parent, &raw, &recipe, COMMIT)
            .err()
            .unwrap()
            .contains("unsupported")
    );
    let (parent_bytes, mut raw, mut recipe) = fixture();
    raw.extend(gzip(b"second member"));
    let mut parent = from_json_str(std::str::from_utf8(&parent_bytes).unwrap()).unwrap();
    let f = parent.artifact.files.informal.as_mut().unwrap();
    f.content_hash = content_hash(&raw);
    f.n_bytes = raw.len() as u64;
    let parent = to_disk_json(&parent).unwrap().into_bytes();
    recipe.parent_manifest_sha256 = content_hash(&parent);
    assert!(
        derive(&parent, &raw, &recipe, COMMIT)
            .err()
            .unwrap()
            .contains("one member")
    );
}

#[test]
fn collision_preflight_preserves_all_existing_outputs() {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "syndeom-derive-test-{}-{}",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::create_dir(&dir).unwrap();
    let first = dir.join("manifest.json");
    let second = dir.join("proof.md");
    std::fs::write(&second, b"retained").unwrap();
    assert!(write_outputs(&[(&first, b"new"), (&second, b"conflict")]).is_err());
    assert!(!first.exists());
    assert_eq!(std::fs::read(&second).unwrap(), b"retained");
    write_outputs(&[(&first, b"new"), (&second, b"retained")]).unwrap();
    write_outputs(&[(&first, b"new"), (&second, b"retained")]).unwrap();
    std::fs::remove_file(first).unwrap();
    std::fs::remove_file(second).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
