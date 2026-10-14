//! Golden and conformance tests for `syndeom-manifest`.
//!
//! Every expected value here is (a) an ADR-0001 D8 golden — hand-computed over fixed bytes in
//! Session 3 and cross-checked with `sha256sum`; (b) a FRIEDMAN ADR-0004 conformance vector adopted
//! by ADR-0001 D6; or (c) a vector printed in RFC 8785 itself (S-0007: §3.2.2 sample input, §3.2.4
//! expected bytes, Appendix B number table). If this crate ever disagrees with them, the crate is
//! wrong until proven otherwise.

use std::collections::BTreeMap;

use serde_json::{Value, json};
use syndeom_manifest::*;

// ---- ADR-0001 D8 goldens -------------------------------------------------------------------

const GOLDEN_A: &[u8] = include_bytes!("fixtures/proof.md");
const GOLDEN_B: &[u8] = include_bytes!("fixtures/Proof.lean");
const A_HASH: &str = "0ac164476b6cedcd3c68194d8e65d3a602c0a7aa97e9e30f133602600712a6a1";
const B_HASH: &str = "c568a98188da217920cecec74ac4a44da5fb1321653e61852f25d6817e11458e";
const A_CONFIG_AUTHORED: &str = r#"{"theorem": "nat_le_sq", "style": "two_cases", "target": "informal", "flaws": [], "padding_ratio": 0}"#;
const A_CONFIG_JCS: &[u8] =
    br#"{"flaws":[],"padding_ratio":0,"style":"two_cases","target":"informal","theorem":"nat_le_sq"}"#;
const A_CONFIG_HASH: &str = "e938458d924bdd4e917b9939acf8de226895129a703e30e8dca8eb82fc714e5f";
const A_ID: &str = "template-v0.1.0-e938458d924b-s7";
const B_CONFIG_AUTHORED: &str = r#"{"theorem": "nat_le_sq", "target": "formal", "flaws": []}"#;
const B_CONFIG_JCS: &[u8] = br#"{"flaws":[],"target":"formal","theorem":"nat_le_sq"}"#;
const B_CONFIG_HASH: &str = "447e6407a01b0ba7796e15a37b6f9099fd76b909662432cded1101f8b2b0cbb2";
const B_ID: &str = "template-v0.1.0-447e6407a01b-s7";

fn adr_example_block() -> &'static str {
    let adr = include_str!("../../../PROGRAM/DECISIONS/ADR-0001.md");
    let start = adr.find("```json\n").expect("ADR-0001 has a ```json block") + "```json\n".len();
    let end = start + adr[start..].find("\n```").expect("json block is closed");
    &adr[start..end]
}

fn hex_to_bytes(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|h| u8::from_str_radix(h, 16).unwrap())
        .collect()
}

#[test]
fn d8_golden_a_bytes_and_hash() {
    assert_eq!(GOLDEN_A.len(), 279);
    assert_eq!(GOLDEN_A.iter().filter(|&&b| b == b'\n').count(), 12);
    assert_eq!(content_hash(GOLDEN_A), A_HASH);
    // the only non-ASCII code points: U+2014 on the Step 1 line, U+220E on the last line
    let text = std::str::from_utf8(GOLDEN_A).unwrap();
    let non_ascii: Vec<char> = text.chars().filter(|c| !c.is_ascii()).collect();
    assert_eq!(non_ascii, ['\u{2014}', '\u{220E}']);
}

#[test]
fn d8_golden_a_segmentation() {
    let st = parse_informal_md(GOLDEN_A).unwrap();
    assert_eq!(
        (st.n_bytes, st.n_steps, st.n_lemmas, st.n_tokens),
        (279, 2, 0, 44)
    );
    assert_eq!(st.steps.len(), 3);
    assert_eq!(st.steps[0], "For every natural number $n$, $n \\le n^2$.");
    let cps: Vec<usize> = st.steps.iter().map(|s| s.chars().count()).collect();
    let bytes: Vec<usize> = st.steps.iter().map(|s| s.len()).collect();
    let toks: Vec<u64> = st.steps.iter().map(|s| count_tokens(s)).collect();
    assert_eq!(cps, [42, 61, 103]);
    assert_eq!(bytes, [42, 63, 103]);
    assert_eq!(toks, [8, 14, 22]);
    assert!(st.steps[2].contains('\n'), "step 2 wraps over two lines");
    assert!(!st.steps[2].ends_with('\n'));
}

#[test]
fn d8_golden_a_span_example() {
    let st = parse_informal_md(GOLDEN_A).unwrap();
    let step1 = &st.steps[1];
    assert_eq!(
        span_text(step1, [35, 60]).as_deref(),
        Some("there is nothing to prove")
    );
    // the same substring in bytes starts at 37 and ends at 62, because of the em dash before it
    let byte_start = step1.find("there is nothing to prove").unwrap();
    assert_eq!((byte_start, byte_start + 25), (37, 62));
    // omission point just before "that is", immediately after the wrapped line's U+000A
    let step2 = &st.steps[2];
    assert_eq!(span_text(step2, [82, 82]).as_deref(), Some(""));
    assert_eq!(step2.chars().nth(81), Some('\n'));
    assert_eq!(
        step2.chars().skip(82).take(7).collect::<String>(),
        "that is"
    );
    // bounds
    assert_eq!(
        span_text(step1, [0, 61]).map(|s| s.chars().count()),
        Some(61)
    );
    assert_eq!(span_text(step1, [0, 62]), None);
    assert_eq!(span_text(step1, [5, 4]), None);
}

#[test]
fn d8_golden_a_config_and_id() {
    let config: Value = serde_json::from_str(A_CONFIG_AUTHORED).unwrap();
    let jcs = jcs_bytes(&config).unwrap();
    assert_eq!(jcs, A_CONFIG_JCS);
    assert_eq!(jcs.len(), 92);
    assert_eq!(config_hash(&config).unwrap(), A_CONFIG_HASH);
    assert_eq!(
        artifact_id("template", "0.1.0", A_CONFIG_HASH, Some(7)),
        A_ID
    );
}

#[test]
fn d8_golden_b_bytes_counts_config_and_id() {
    assert_eq!(GOLDEN_B.len(), 168);
    assert_eq!(content_hash(GOLDEN_B), B_HASH);
    let st = parse_lean(GOLDEN_B).unwrap();
    assert_eq!(
        (st.n_bytes, st.n_lines, st.n_tokens, st.n_declarations),
        (168, 5, 29, 1)
    );
    assert_eq!(
        st.declarations,
        vec![(4, "syndeom_golden_nat_le_sq".to_string())]
    );
    assert_eq!(st.imports, vec!["Mathlib".to_string()]);
    let config: Value = serde_json::from_str(B_CONFIG_AUTHORED).unwrap();
    let jcs = jcs_bytes(&config).unwrap();
    assert_eq!(jcs, B_CONFIG_JCS);
    assert_eq!(jcs.len(), 52);
    assert_eq!(config_hash(&config).unwrap(), B_CONFIG_HASH);
    assert_eq!(
        artifact_id("template", "0.1.0", B_CONFIG_HASH, Some(7)),
        B_ID
    );
}

/// The Lean module the gate builds is the D8 fixture, byte for byte (ADR-0002).
#[test]
fn d8_lean_project_module_is_the_golden_b_bytes() {
    let module = include_bytes!("../../../lean/Syndeom/Fixtures/GoldenB.lean");
    assert_eq!(&module[..], GOLDEN_B);
}

#[test]
fn d8_prompt_hash_example() {
    let prompt =
        json!({"system": "You write proofs.", "user": "Prove that n <= n^2.", "version": 1});
    assert_eq!(
        String::from_utf8(jcs_bytes(&prompt).unwrap()).unwrap(),
        r#"{"system":"You write proofs.","user":"Prove that n <= n^2.","version":1}"#
    );
    assert_eq!(
        prompt_hash(&prompt).unwrap(),
        "2f5dd887ab1ad85a7ad83907a6a1d306874630ff4cb17e4ceeeac59b2ba11825"
    );
}

/// The illustrative manifest printed in ADR-0001 D8 must parse, self-check against Golden A's
/// bytes, and survive a disk round-trip unchanged. The ADR is read from the repo so the two can
/// never drift apart silently.
#[test]
fn d8_example_manifest_from_adr_text() {
    let block = adr_example_block();
    let manifest = from_json_str(block).expect("ADR-0001 example manifest parses");
    // the D8 example predates the 2026-10-13 amendment and stays a valid /1 manifest
    assert_eq!(manifest.manifest_schema, MANIFEST_SCHEMA_V1);
    assert_eq!(
        manifest
            .ground_truth
            .as_ref()
            .unwrap()
            .statement_status
            .value,
        Some(true)
    );
    assert_eq!(manifest.artifact_id, A_ID);
    assert_eq!(manifest.config_hash, A_CONFIG_HASH);
    assert_eq!(manifest.seed, Some(7));
    assert_eq!(manifest.origin, Origin::Synthetic);
    assert_eq!(manifest.artifact.kind, Kind::Informal);
    let informal = manifest.artifact.files.informal.as_ref().unwrap();
    assert_eq!(informal.content_hash, A_HASH);
    assert_eq!(
        (
            informal.n_bytes,
            informal.n_tokens,
            informal.n_steps,
            informal.n_lemmas
        ),
        (279, Some(44), Some(2), Some(0))
    );
    assert!(manifest.artifact.files.formal.is_none() && manifest.toolchain.is_none());
    let gt = manifest.ground_truth.as_ref().unwrap();
    assert!(gt.correct && gt.flaws.is_empty() && gt.taxonomy_version == "0.1.0");
    manifest
        .check(FileBytes {
            informal: Some(GOLDEN_A),
            formal: None,
        })
        .unwrap_or_else(|problems| panic!("ADR-0001 example fails its own rules: {problems:#?}"));

    // D1 on-disk form: parses back to the same manifest, ends with exactly one LF, no CR.
    let disk = to_disk_json(&manifest).unwrap();
    assert!(disk.ends_with('\n') && !disk.ends_with("\n\n"));
    assert!(!disk.contains('\r'));
    assert_eq!(from_json_str(&disk).unwrap(), manifest);
    assert_eq!(to_disk_json(&from_json_str(&disk).unwrap()).unwrap(), disk);
    // and the ADR's block is itself in JCS key order at the top level (D1 presentation rule)
    let top: Vec<&str> = block
        .lines()
        .filter_map(|l| l.strip_prefix("  \"").and_then(|r| r.split('"').next()))
        .collect();
    let mut sorted = top.clone();
    sorted.sort_unstable();
    assert_eq!(
        top, sorted,
        "ADR-0001 example manifest top-level keys are not in JCS order"
    );
    // the D6 arithmetic the ADR spells out for this example
    assert_eq!(manifest.fidelity.verdict, Verdict::Pass);
    assert_eq!(manifest.fidelity.per_statistic.len(), 4);
}

// ---- ADR-0001 D2 / D4 rules -------------------------------------------------------------------

#[test]
fn d2_artifact_id_has_two_forms() {
    assert_eq!(
        artifact_id("ingest_errata", "0.1.0", A_CONFIG_HASH, None),
        "ingest_errata-v0.1.0-e938458d924b"
    );
    assert_eq!(
        artifact_id("template", "0.1.0", A_CONFIG_HASH, Some(0)),
        "template-v0.1.0-e938458d924b-s0"
    );
}

#[test]
fn d4_timestamp_and_unknown_fields_are_rejected() {
    let with_extra = adr_example_block().replacen(
        "\"seed\": 7",
        "\"seed\": 7,\n  \"created\": \"2026-10-12\"",
        1,
    );
    assert!(
        from_json_str(&with_extra).is_err(),
        "a timestamp field must not be accepted (D4)"
    );
}

#[test]
fn d4_nullable_fields_must_be_present() {
    // removing the `toolchain` key entirely is an error: null is allowed, absence is not
    let block = adr_example_block();
    let without = block.replacen(",\n  \"toolchain\": null", "", 1);
    assert_ne!(block, without);
    assert!(
        from_json_str(&without).is_err(),
        "toolchain must be present (as null) (D4)"
    );
}

#[test]
fn d4_check_reports_every_discrepancy_not_just_the_first() {
    let mut m = from_json_str(adr_example_block()).unwrap();
    m.seed = Some(8); // artifact_id no longer derives
    m.ground_truth.as_mut().unwrap().correct = false; // not the derived bit
    m.fidelity.verdict = Verdict::Fail; // verdict wrong
    let problems = m
        .check(FileBytes {
            informal: Some(GOLDEN_A),
            formal: None,
        })
        .unwrap_err();
    assert!(
        problems.iter().any(|p| p.starts_with("D2:")),
        "{problems:#?}"
    );
    assert!(
        problems.iter().any(|p| p.starts_with("D4/SD1:")),
        "{problems:#?}"
    );
    assert!(
        problems.iter().any(|p| p.starts_with("D4/S2.2:")),
        "{problems:#?}"
    );
    assert_eq!(problems.len(), 3, "{problems:#?}");
}

#[test]
fn d4_contact_artifacts_carry_no_ground_truth_and_no_seed() {
    let mut m = from_json_str(adr_example_block()).unwrap();
    m.origin = Origin::Contact;
    let problems = m
        .check(FileBytes {
            informal: Some(GOLDEN_A),
            formal: None,
        })
        .unwrap_err();
    assert!(
        problems.iter().any(|p| p.starts_with("D4/SD3:")),
        "{problems:#?}"
    );
    assert!(
        problems.iter().any(|p| p.contains("seed must be null")),
        "{problems:#?}"
    );
    assert!(
        problems.iter().any(|p| p.contains("rng \"none\"")),
        "{problems:#?}"
    );
}

#[test]
fn d4_a_false_statement_cannot_have_a_valid_proof_and_valid_proofs_carry_cosmetic_flaws_only() {
    let mut m = from_json_str(adr_example_block()).unwrap();
    {
        let gt = m.ground_truth.as_mut().unwrap();
        gt.statement_status.value = Some(false);
        gt.correct = false;
    }
    let problems = m
        .check(FileBytes {
            informal: Some(GOLDEN_A),
            formal: None,
        })
        .unwrap_err();
    assert!(
        problems
            .iter()
            .any(|p| p.contains("false statement cannot have a valid proof")),
        "{problems:#?}"
    );

    let mut m = from_json_str(adr_example_block()).unwrap();
    m.ground_truth.as_mut().unwrap().flaws.push(Flaw {
        instance: 1,
        class: "FLAW-0005".into(),
        sub_variant: None,
        location: Location::Informal(InformalLocation {
            artifact: InformalTag::Informal,
            step: 2,
            span: [82, 82],
        }),
        severity: Severity::Gap,
        lean_catchable: LeanCatchable::YesKernel,
        dials: json!({}),
    });
    let problems = m
        .check(FileBytes {
            informal: Some(GOLDEN_A),
            formal: None,
        })
        .unwrap_err();
    assert_eq!(
        problems,
        vec!["D4: a valid proof may carry cosmetic flaws only".to_string()]
    );
}

/// ADR-0001 as amended 2026-10-13 (Al's ruling (ii) on ADR-0003): a null statement truth is
/// admitted under `syndeom-manifest/2` exactly with `method: "undetermined"`, forces
/// `proof_status: "invalid"` and `correct: false`, and is rejected under `/1`.
#[test]
fn d4_amendment_2026_10_13_null_statement_truth() {
    let bytes = b"%PDF-1.7 (stand-in bytes)\n";
    let config = json!({
        "sources": ["S-0001"], "item": "paper-y, Lemma 3", "licence": "CC-BY-4.0",
        "status": {"kind": "erratum", "reference": "S-0001"}
    });
    let h = config_hash(&config).unwrap();
    let mut v = json!({
        "manifest_schema": MANIFEST_SCHEMA,
        "artifact_id": artifact_id("s1_ingest", "0.1.0", &h, None),
        "origin": "ground-truth",
        "generator": {"id": "s1_ingest", "version": "0.1.0", "rng": "none"},
        "engine_commit": "0000000000000000000000000000000000000000",
        "seed": null,
        "config": config,
        "config_hash": h,
        "toolchain": null,
        "artifact": {"kind": "informal", "files": {"informal": {
            "file": "original.pdf", "format": FORMAT_RAW, "storage": "external",
            "content_hash": content_hash(bytes), "n_bytes": bytes.len(),
            "n_tokens": null, "n_steps": null, "n_lemmas": null
        }}},
        "ground_truth": {
            "derivation": "documented-by-source",
            "statement_status": {"value": null, "method": "undetermined", "reference": "S-0001"},
            "proof_status": "invalid",
            "correct": false,
            "flaws": [{
                "instance": 1, "class": "FLAW-0005", "sub_variant": null,
                "location": {"artifact": "informal", "locator": "Lemma 3, proof"},
                "severity": "fatal", "lean_catchable": "YES-KERNEL", "dials": {}
            }],
            "taxonomy_version": "0.1.0"
        },
        "commanded": {},
        "achieved": {},
        "fidelity": {"per_statistic": {}, "verdict": "PASS"}
    });
    let files = FileBytes {
        informal: Some(bytes),
        formal: None,
    };
    let ok: Manifest = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(ok.manifest_schema, "syndeom-manifest/2");
    assert_eq!(
        ok.ground_truth.as_ref().unwrap().statement_status.value,
        None
    );
    ok.check(files)
        .unwrap_or_else(|p| panic!("null truth under /2 must pass: {p:#?}"));
    // the key must be present: absence is not null
    let mut absent = v.clone();
    absent["ground_truth"]["statement_status"]
        .as_object_mut()
        .unwrap()
        .remove("value");
    assert!(serde_json::from_value::<Manifest>(absent).is_err());
    // /1 does not admit it
    v["manifest_schema"] = json!(MANIFEST_SCHEMA_V1);
    let old: Manifest = serde_json::from_value(v.clone()).unwrap();
    let problems = old.check(files).unwrap_err();
    assert_eq!(problems.len(), 1, "{problems:#?}");
    assert!(problems[0].contains("does not admit a null statement truth"));
    v["manifest_schema"] = json!(MANIFEST_SCHEMA);
    // null iff undetermined, both ways
    let mut m: Manifest = serde_json::from_value(v.clone()).unwrap();
    m.ground_truth.as_mut().unwrap().statement_status.method = StatementMethod::SourceDocument;
    let problems = m.check(files).unwrap_err();
    assert_eq!(problems.len(), 1, "{problems:#?}");
    assert!(problems[0].contains("null iff method"));
    let mut m: Manifest = serde_json::from_value(v.clone()).unwrap();
    m.ground_truth.as_mut().unwrap().statement_status.value = Some(true);
    let problems = m.check(files).unwrap_err();
    assert_eq!(problems.len(), 1, "{problems:#?}");
    assert!(problems[0].contains("null iff method"));
    // null forces invalid, and correct stays derived false
    let mut m: Manifest = serde_json::from_value(v.clone()).unwrap();
    {
        let gt = m.ground_truth.as_mut().unwrap();
        gt.proof_status = ProofStatus::Valid;
        gt.flaws.clear();
    }
    let problems = m.check(files).unwrap_err();
    assert!(
        problems
            .iter()
            .any(|p| p.contains("requires proof_status invalid")),
        "{problems:#?}"
    );
    let mut m: Manifest = serde_json::from_value(v).unwrap();
    m.ground_truth.as_mut().unwrap().correct = true;
    let problems = m.check(files).unwrap_err();
    assert_eq!(
        problems,
        vec!["D4/SD1: correct is true, derived false".to_string()]
    );
}

#[test]
fn d3_informal_span_bounds_are_checked_against_the_bytes() {
    let mut m = from_json_str(adr_example_block()).unwrap();
    {
        let gt = m.ground_truth.as_mut().unwrap();
        gt.proof_status = ProofStatus::Invalid;
        gt.correct = false;
        gt.flaws.push(Flaw {
            instance: 1,
            class: "FLAW-0004".into(),
            sub_variant: None,
            location: Location::Informal(InformalLocation {
                artifact: InformalTag::Informal,
                step: 1,
                span: [35, 62],
            }),
            severity: Severity::Gap,
            lean_catchable: LeanCatchable::YesKernel,
            dials: json!({"hedge": "clearly"}),
        });
    }
    let problems = m
        .check(FileBytes {
            informal: Some(GOLDEN_A),
            formal: None,
        })
        .unwrap_err();
    assert_eq!(problems.len(), 1, "{problems:#?}");
    assert!(
        problems[0].contains("span [35, 62] is outside step 1 (61 code points)"),
        "{problems:#?}"
    );
}

#[test]
fn d3_informal_format_rejections() {
    use FormatError::*;
    let ok = parse_informal_md(GOLDEN_A).is_ok();
    assert!(ok);
    let s = std::str::from_utf8(GOLDEN_A).unwrap();
    assert_eq!(parse_informal_md(b"\xff\n"), Err(NotUtf8));
    assert_eq!(
        parse_informal_md("\u{FEFF}# x\n".as_bytes()),
        Err(ByteOrderMark)
    );
    assert_eq!(
        parse_informal_md(s.replace('\n', "\r\n").as_bytes()),
        Err(CarriageReturn { offset: 10 })
    );
    assert_eq!(
        parse_informal_md(s.trim_end_matches('\n').as_bytes()),
        Err(MissingFinalNewline)
    );
    assert_eq!(
        parse_informal_md(format!("{s}\n").as_bytes()),
        Err(TrailingBlankLine)
    );
    assert_eq!(
        parse_informal_md(s.replacen("**Proof.**\n", "**Proof.** \n", 1).as_bytes()),
        Err(TrailingSpace { line: 5 })
    );
    assert_eq!(
        parse_informal_md(s.replacen("# Golden A", "Golden A", 1).as_bytes()),
        Err(BadTitle)
    );
    assert_eq!(
        parse_informal_md(s.replacen("**Theorem.** ", "**Claim.** ", 1).as_bytes()),
        Err(MissingTheorem)
    );
    assert_eq!(
        parse_informal_md(s.replacen("**Proof.**\n", "**Proof**\n", 1).as_bytes()),
        Err(MissingProofLine)
    );
    assert_eq!(
        parse_informal_md(s.replacen("**Step 2.** ", "**Step 3.** ", 1).as_bytes()),
        Err(BadStepMarker { index: 2 })
    );
    assert_eq!(
        parse_informal_md(s.replacen("\n\u{220E}\n", "\nQED\n", 1).as_bytes()),
        Err(MissingTerminator)
    );
    assert_eq!(
        parse_informal_md(
            s.replacen("\n\n**Proof.**", "\n\n\n**Proof.**", 1)
                .as_bytes()
        ),
        Err(BlankLine { line: 5 })
    );
    // lemma counting
    assert!(is_lemma_step("Lemma 1. Every x is y."));
    assert!(is_lemma_step("Lemma 12. x"));
    assert!(!is_lemma_step("Lemma 01. x"));
    assert!(!is_lemma_step("Lemma. x"));
    assert!(!is_lemma_step("By Lemma 1, x"));
}

#[test]
fn d3_lean_format_rejections_and_declaration_names() {
    use FormatError::*;
    let s = std::str::from_utf8(GOLDEN_B).unwrap();
    assert_eq!(
        parse_lean(
            s.replacen("import Mathlib\n", "import Mathlib\nimport Aesop\n", 1)
                .as_bytes()
        ),
        Err(InvalidImport {
            line: 2,
            module: "Aesop".into()
        })
    );
    assert!(
        parse_lean(
            s.replacen("import Mathlib\n", "import Mathlib.Tactic\n", 1)
                .as_bytes()
        )
        .is_ok()
    );
    assert_eq!(
        parse_lean(s.replace('\n', "\r\n").as_bytes()),
        Err(CarriageReturn { offset: 14 })
    );
    assert_eq!(parse_lean(b"import Mathlib\n\n"), Err(TrailingBlankLine));
    let st = parse_lean(
        "import Mathlib\n\naxiom helper : \u{2200} n : \u{2115}, n < 2 ^ n\n\nlemma foo {a : \u{2115}} : a = a := rfl\n"
            .as_bytes(),
    )
    .unwrap();
    assert_eq!(
        st.declarations,
        vec![(3, "helper".to_string()), (5, "foo".to_string())]
    );
    assert_eq!(st.n_lines, 5);
}

// ---- derived canonical artifacts (ADR-0001 D4, Al's amendment) --------------------------------

fn real_item(generator_id: &str, config: Value, informal: Value, location: Value) -> Manifest {
    let h = config_hash(&config).unwrap();
    let id = artifact_id(generator_id, "0.1.0", &h, None);
    let v = json!({
        "manifest_schema": MANIFEST_SCHEMA,
        "artifact_id": id,
        "origin": "ground-truth",
        "generator": {"id": generator_id, "version": "0.1.0", "rng": "none"},
        "engine_commit": "0000000000000000000000000000000000000000",
        "seed": null,
        "config": config,
        "config_hash": h,
        "toolchain": null,
        "artifact": {"kind": "informal", "files": {"informal": informal}},
        "ground_truth": {
            "derivation": "documented-by-source",
            "statement_status": {"value": true, "method": "source-document", "reference": "S-0001"},
            "proof_status": "invalid",
            "correct": false,
            "flaws": [{
                "instance": 1, "class": "FLAW-0005", "sub_variant": null, "location": location,
                "severity": "gap", "lean_catchable": "YES-KERNEL", "dials": {}
            }],
            "taxonomy_version": "0.1.0"
        },
        "commanded": {},
        "achieved": {},
        "fidelity": {"per_statistic": {}, "verdict": "PASS"}
    });
    serde_json::from_value(v).unwrap()
}

#[test]
fn d4_derived_canonical_artifact_against_its_raw_parent() {
    let pdf = b"%PDF-1.7 (stand-in bytes)\n";
    let base = json!({
        "sources": ["S-0001"], "item": "paper-x, Theorem 2", "licence": "CC0-1.0",
        "status": {"kind": "erratum", "reference": "S-0001"}
    });
    let parent = real_item(
        "ingest_errata",
        base.clone(),
        json!({
            "file": "proof.pdf", "format": FORMAT_RAW, "storage": "external",
            "content_hash": content_hash(pdf), "n_bytes": pdf.len(),
            "n_tokens": null, "n_steps": null, "n_lemmas": null
        }),
        json!({"artifact": "informal", "locator": "Theorem 2, p. 4, second display"}),
    );
    parent
        .check(FileBytes {
            informal: Some(pdf),
            formal: None,
        })
        .unwrap_or_else(|p| panic!("raw/1 parent fails: {p:#?}"));

    let mut child_cfg = base.clone();
    child_cfg["derived_from"] = json!(parent.artifact_id);
    child_cfg["method"] = json!("manual transcription, argued in the S1.3 record");
    let child = real_item(
        "derive_informal_md",
        child_cfg,
        json!({
            "file": FILE_INFORMAL, "format": FORMAT_INFORMAL_MD, "storage": "committed",
            "content_hash": A_HASH, "n_bytes": 279, "n_tokens": 44, "n_steps": 2, "n_lemmas": 0
        }),
        json!({"artifact": "informal", "step": 2, "span": [82, 82]}),
    );
    child
        .check(FileBytes {
            informal: Some(GOLDEN_A),
            formal: None,
        })
        .unwrap_or_else(|p| panic!("derived child fails its own rules: {p:#?}"));
    child
        .check_derived(&parent)
        .unwrap_or_else(|p| panic!("derived child fails against its parent: {p:#?}"));

    // the parent is not derived from anything; a child naming the wrong parent fails
    assert!(parent.check_derived(&child).is_err());
    let mut wrong = child.clone();
    wrong.config["derived_from"] = json!("ingest_errata-v0.1.0-000000000000");
    let problems = wrong.check_derived(&parent).unwrap_err();
    assert!(
        problems[0].starts_with("D4: config.derived_from"),
        "{problems:#?}"
    );
    // a raw-form location on a derived artifact is rejected (locations must be re-expressed)
    let mut raw_loc = child.clone();
    raw_loc.ground_truth.as_mut().unwrap().flaws[0].location = Location::Raw(RawLocation {
        artifact: FileKey::Informal,
        locator: "p. 4".into(),
    });
    assert!(
        raw_loc
            .check_derived(&parent)
            .unwrap_err()
            .iter()
            .any(|p| p.contains("not in canonical D3 form"))
    );
    // a synthetic artifact can never be derived_from
    let mut synthetic = from_json_str(adr_example_block()).unwrap();
    synthetic.config["derived_from"] = json!(parent.artifact_id);
    assert!(
        synthetic
            .check(FileBytes::default())
            .unwrap_err()
            .iter()
            .any(|p| p.contains("cannot be derived_from"))
    );
}

// ---- ADR-0001 D6 = FRIEDMAN ADR-0004 conformance vectors -------------------------------------

fn cmd(value: Value, tolerance: Tolerance) -> Commanded {
    Commanded { value, tolerance }
}

fn ach(value: Option<Value>) -> Achieved {
    Achieved {
        value,
        estimator: "test".into(),
        ci95: None,
        reason: None,
    }
}

#[test]
fn d6_adr0004_vectors() {
    let cases: &[(f64, f64, Tolerance, Verdict)] = &[
        (-1.0, -1.125, Tolerance::Abs { abs: 0.125 }, Verdict::Pass),
        (-1.0, -1.1, Tolerance::Abs { abs: 0.1 }, Verdict::Fail),
        (-1.0, -1.11, Tolerance::Abs { abs: 0.1 }, Verdict::Fail),
        (-2.0, -2.25, Tolerance::Rel { rel: 0.125 }, Verdict::Pass),
        (-2.0, -2.2, Tolerance::Rel { rel: 0.125 }, Verdict::Pass),
        (-2.0, -2.2, Tolerance::Rel { rel: 0.1 }, Verdict::Fail),
        (-2.0, -2.26, Tolerance::Rel { rel: 0.125 }, Verdict::Fail),
    ];
    for (c, a, tol, want) in cases {
        let got = statistic_verdict(&cmd(json!(c), tol.clone()), Some(&ach(Some(json!(a)))));
        assert_eq!(got, *want, "c={c} a={a} tol={tol:?}");
    }
    // null / unmeasured / non-numeric -> FAIL; exact is structural; ci95 never enters
    let abs = cmd(json!(-1.0), Tolerance::Abs { abs: 0.1 });
    assert_eq!(statistic_verdict(&abs, Some(&ach(None))), Verdict::Fail);
    assert_eq!(statistic_verdict(&abs, None), Verdict::Fail);
    assert_eq!(
        statistic_verdict(&abs, Some(&ach(Some(json!("x"))))),
        Verdict::Fail
    );
    let exact = cmd(json!(0), Tolerance::Exact { exact: true });
    assert_eq!(
        statistic_verdict(&exact, Some(&ach(Some(json!(0))))),
        Verdict::Pass
    );
    assert_eq!(
        statistic_verdict(&exact, Some(&ach(Some(json!(0.0))))),
        Verdict::Fail,
        "integer 0 and number 0.0 differ under exact (D4)"
    );
    let wide = Achieved {
        ci95: Some([-9.0, 9.0]),
        ..ach(Some(json!(-1.5)))
    };
    assert_eq!(statistic_verdict(&abs, Some(&wide)), Verdict::Fail);
    // vacuous fidelity
    let f = fidelity(&BTreeMap::new(), &BTreeMap::new());
    assert_eq!(f.verdict, Verdict::Pass);
    assert!(f.per_statistic.is_empty());
}

// ---- RFC 8785 conformance (S-0007) ----------------------------------------------------------

/// §3.2.2 sample input, exactly as printed in the RFC (escapes are JSON escapes, not Rust ones).
const RFC_SAMPLE_INPUT: &str = r#"{
       "numbers": [333333333.33333329, 1E30, 4.50,
                   2e-3, 0.000000000000000000000000001],
       "string": "\u20ac$\u000F\u000aA'\u0042\u0022\u005c\\\"\/",
       "literals": [null, true, false]
     }"#;

/// §3.2.4 expected canonical bytes, exactly as printed in the RFC.
const RFC_SAMPLE_OUTPUT_HEX: &str = "7b 22 6c 69 74 65 72 61 6c 73 22 3a 5b 6e 75 6c 6c 2c 74 72 \
     75 65 2c 66 61 6c 73 65 5d 2c 22 6e 75 6d 62 65 72 73 22 3a \
     5b 33 33 33 33 33 33 33 33 33 2e 33 33 33 33 33 33 33 2c 31 \
     65 2b 33 30 2c 34 2e 35 2c 30 2e 30 30 32 2c 31 65 2d 32 37 \
     5d 2c 22 73 74 72 69 6e 67 22 3a 22 e2 82 ac 24 5c 75 30 30 \
     30 66 5c 6e 41 27 42 5c 22 5c 5c 5c 5c 5c 22 2f 22 7d";

#[test]
fn rfc8785_section_3_2_sample_canonicalizes_to_the_printed_bytes() {
    let value: Value = serde_json::from_str(RFC_SAMPLE_INPUT).unwrap();
    let got = jcs_bytes(&value).unwrap();
    let expected = hex_to_bytes(RFC_SAMPLE_OUTPUT_HEX);
    assert_eq!(
        got,
        expected,
        "\n got: {}\nwant: {}",
        String::from_utf8_lossy(&got),
        String::from_utf8_lossy(&expected)
    );
}

/// Appendix B, Table 1 — every finite row. NaN and Infinity rows are covered separately.
const RFC_APPENDIX_B: &[(u64, &str)] = &[
    (0x0000000000000000, "0"),
    (0x8000000000000000, "0"),
    (0x0000000000000001, "5e-324"),
    (0x8000000000000001, "-5e-324"),
    (0x7fefffffffffffff, "1.7976931348623157e+308"),
    (0xffefffffffffffff, "-1.7976931348623157e+308"),
    (0x4340000000000000, "9007199254740992"),
    (0xc340000000000000, "-9007199254740992"),
    (0x4430000000000000, "295147905179352830000"),
    (0x44b52d02c7e14af5, "9.999999999999997e+22"),
    (0x44b52d02c7e14af6, "1e+23"),
    (0x44b52d02c7e14af7, "1.0000000000000001e+23"),
    (0x444b1ae4d6e2ef4e, "999999999999999700000"),
    (0x444b1ae4d6e2ef4f, "999999999999999900000"),
    (0x444b1ae4d6e2ef50, "1e+21"),
    (0x3eb0c6f7a0b5ed8c, "9.999999999999997e-7"),
    (0x3eb0c6f7a0b5ed8d, "0.000001"),
    (0x41b3de4355555553, "333333333.3333332"),
    (0x41b3de4355555554, "333333333.33333325"),
    (0x41b3de4355555555, "333333333.3333333"),
    (0x41b3de4355555556, "333333333.3333334"),
    (0x41b3de4355555557, "333333333.33333343"),
    (0xbecbf647612f3696, "-0.0000033333333333333333"),
    (0x43143ff3c1cb0959, "1424953923781206.2"),
];

#[test]
fn rfc8785_appendix_b_number_serialization() {
    for &(bits, expected) in RFC_APPENDIX_B {
        let x = f64::from_bits(bits);
        let got = serde_json_canonicalizer::to_string(&x).unwrap();
        assert_eq!(got, expected, "bits {bits:016x} as f64");
        let v = Value::from(x);
        let got = String::from_utf8(jcs_bytes(&v).unwrap()).unwrap();
        assert_eq!(got, expected, "bits {bits:016x} as Value");
    }
}

#[test]
fn rfc8785_nan_and_infinity_are_errors_not_output() {
    assert!(serde_json::Number::from_f64(f64::NAN).is_none());
    assert!(serde_json::Number::from_f64(f64::INFINITY).is_none());
    assert!(serde_json::Number::from_f64(f64::NEG_INFINITY).is_none());
    assert!(serde_json_canonicalizer::to_string(&f64::NAN).is_err());
    assert!(serde_json_canonicalizer::to_string(&f64::INFINITY).is_err());
}

#[test]
fn rfc8785_property_sorting_is_by_utf16_code_units() {
    // §3.2.3: names compared in raw form by UTF-16 code units; a non-BMP key (surrogate pair
    // 0xD83D..) sorts BEFORE U+FF5E even though its code point is larger.
    let v = json!({"\u{ff5e}": 1, "\u{1f600}": 2, "\u{20ac}": 3, "a": 4, "": 5, "\r": 6, "1": 7, "A": 8});
    let got = String::from_utf8(jcs_bytes(&v).unwrap()).unwrap();
    assert_eq!(
        got,
        "{\"\":5,\"\\r\":6,\"1\":7,\"A\":8,\"a\":4,\"\u{20ac}\":3,\"\u{1f600}\":2,\"\u{ff5e}\":1}"
    );
}
