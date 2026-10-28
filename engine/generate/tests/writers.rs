//! The writers agree with ADR-0001 D8's fixtures (ADR-0011 D10): Golden A's 279 bytes and Golden
//! B's 168 bytes are reproduced from the `nat_le_sq` seed, style `two_cases`. The fixtures' title,
//! declaration name and doc line are fixture names, not seed material, so the test sets them; every
//! other byte comes from the seed record and the renderers.

use syndeom_generate::plan::{render_formal, render_informal};
use syndeom_generate::template::{Version, faithful_plan, seeds};

const GOLDEN_A: &[u8] = include_bytes!("../../manifest/tests/fixtures/proof.md");
const GOLDEN_B: &[u8] = include_bytes!("../../manifest/tests/fixtures/Proof.lean");

fn nat_le_sq() -> syndeom_generate::plan::Plan {
    let seed = seeds(Version::V0_1_0)
        .into_iter()
        .find(|s| s.id == "nat_le_sq")
        .unwrap();
    let style = seed.styles.iter().find(|s| s.id == "two_cases").unwrap();
    faithful_plan(&seed, style)
}

#[test]
fn golden_a_from_the_informal_writer() {
    let mut plan = nat_le_sq();
    plan.title = "Golden A".into();
    let bytes = render_informal(&plan).unwrap();
    assert_eq!(bytes.len(), 279);
    assert_eq!(bytes, GOLDEN_A);
}

#[test]
fn golden_b_from_the_formal_writer() {
    let mut plan = nat_le_sq();
    plan.main_declaration = "syndeom_golden_nat_le_sq".into();
    plan.doc = Some("Golden B: the formal variant of `nat_le_sq`.".into());
    let rendered = render_formal(&plan).unwrap();
    assert_eq!(rendered.bytes.len(), 168);
    assert_eq!(rendered.bytes, GOLDEN_B);
    // step 1 carries no fragment; step 2's tactic is line 5
    assert_eq!(rendered.fragment_lines, vec![None, Some([5, 5])]);
}

#[test]
fn a_closed_statement_is_written_without_a_double_space() {
    // 0.2.0: a seed may have no binders (`rolle_abs`); the header then has one space before the
    // colon. Every 0.1.0 seed has binders, so no 0.1.0 byte depends on this.
    let mut plan = nat_le_sq();
    plan.binders = String::new();
    plan.claim = "True".into();
    let rendered = render_formal(&plan).unwrap();
    let text = String::from_utf8(rendered.bytes).unwrap();
    assert!(text.contains("theorem syndeom_s2_template_nat_le_sq : True := by\n"));
    assert!(!text.contains("  :"));
}
