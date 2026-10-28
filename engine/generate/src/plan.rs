//! The plan — the one shared interface behind the four S2.1 modules (ADR-0011 D2) — and its two
//! renderers. Both renderers are pure functions of the plan; only they produce bytes.

use serde::{Deserialize, Serialize};
use syndeom_manifest::{PROOF_LINE, StatementStatus, TERMINATOR, THEOREM_MARKER};

/// One step of the argument: its informal text (ADR-0001 D3, without the marker) and the aligned
/// Lean fragment, as tactic lines of the main proof with their relative indentation. A step may
/// have no fragment (its content is carried by a neighbour's tactic).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub text: String,
    pub fragment: Vec<String>,
}

/// A whole artifact before rendering (ADR-0011 D2). Lemmas and padding steps arrive with the
/// producer versions that realize FLAW-0007 and FLAW-0013; v0.1 seeds carry neither.
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub title: String,
    pub statement: String,
    pub imports: Vec<String>,
    /// An optional `/-- … -/` line above the main declaration (used by the D8 writer test).
    pub doc: Option<String>,
    pub main_declaration: String,
    pub binders: String,
    pub claim: String,
    pub truth: StatementStatus,
    pub steps: Vec<Step>,
}

/// The formal file and, per step (0-based index into `Plan::steps`), the 1-based line range of
/// its fragment in that file, `None` for a step without a fragment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalRendering {
    pub bytes: Vec<u8>,
    pub fragment_lines: Vec<Option<[u64; 2]>>,
}

fn check_text(what: &str, text: &str) -> Result<(), String> {
    if text.is_empty() {
        return Err(format!("{what} is empty"));
    }
    if text.contains(['\r', '\t']) {
        return Err(format!("{what} contains a CR or a tab"));
    }
    if text.contains("\n\n") || text.starts_with('\n') || text.ends_with('\n') {
        return Err(format!("{what} would split its paragraph"));
    }
    if text.split('\n').any(|l| l.ends_with(' ')) {
        return Err(format!("{what} has a trailing space"));
    }
    Ok(())
}

/// The `syndeom-informal-md/1` writer (ADR-0001 D3; ADR-0011 D2): title, statement, the proof
/// line, `**Step k.**` paragraphs numbered consecutively, the terminator, one trailing LF.
pub fn render_informal(plan: &Plan) -> Result<Vec<u8>, String> {
    check_text("title", &plan.title)?;
    if plan.title.contains('\n') {
        return Err("title spans lines".into());
    }
    check_text("statement", &plan.statement)?;
    if plan.steps.is_empty() {
        return Err("a proof has at least one step".into());
    }
    let mut out = format!(
        "# {}\n\n{THEOREM_MARKER}{}\n\n{PROOF_LINE}\n\n",
        plan.title, plan.statement
    );
    for (i, step) in plan.steps.iter().enumerate() {
        let k = i + 1;
        check_text(&format!("step {k}"), &step.text)?;
        out.push_str(&format!("**Step {k}.** {}\n\n", step.text));
    }
    out.push_str(TERMINATOR);
    out.push('\n');
    Ok(out.into_bytes())
}

/// The `syndeom-lean/1` writer (ADR-0011 D2): the imports, a blank line, the optional doc line,
/// the main declaration, and its proof as the steps' fragments in step order, indented two spaces.
pub fn render_formal(plan: &Plan) -> Result<FormalRendering, String> {
    if plan.imports.is_empty() {
        return Err("a formal file imports at least one Mathlib module".into());
    }
    let mut out = String::new();
    let mut line: u64 = 0;
    let mut push = |out: &mut String, text: &str| {
        out.push_str(text);
        out.push('\n');
        line += 1;
        line
    };
    for module in &plan.imports {
        if module != "Mathlib" && !module.starts_with("Mathlib.") {
            return Err(format!("import {module:?} is not Mathlib"));
        }
        push(&mut out, &format!("import {module}"));
    }
    push(&mut out, "");
    if let Some(doc) = &plan.doc {
        check_text("doc", doc)?;
        push(&mut out, &format!("/-- {doc} -/"));
    }
    push(
        &mut out,
        &format!(
            "theorem {} {} : {} := by",
            plan.main_declaration, plan.binders, plan.claim
        ),
    );
    let mut fragment_lines = Vec::with_capacity(plan.steps.len());
    let mut any = false;
    for (i, step) in plan.steps.iter().enumerate() {
        let mut range: Option<[u64; 2]> = None;
        for l in &step.fragment {
            if l.trim().is_empty() || l.ends_with(' ') || l.contains(['\r', '\t', '\n']) {
                return Err(format!("step {} has a malformed fragment line", i + 1));
            }
            let n = push(&mut out, &format!("  {l}"));
            range = Some([range.map_or(n, |r| r[0]), n]);
        }
        any |= range.is_some();
        fragment_lines.push(range);
    }
    if !any {
        return Err("the main proof has no tactic line".into());
    }
    Ok(FormalRendering {
        bytes: out.into_bytes(),
        fragment_lines,
    })
}
