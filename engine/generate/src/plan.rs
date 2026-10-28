//! The plan — the one shared interface behind the four S2.1 modules (ADR-0011 D2) — and its two
//! renderers. Both renderers are pure functions of the plan; only they produce bytes.
//!
//! Lemma steps (0.3.0): a step whose text begins `Lemma N.` carries a [`LemmaHeader`] and is
//! rendered in the formal file as its own declaration before the main declaration, in step
//! order; in the informal file it is a step like any other (D2).

use serde::{Deserialize, Serialize};
use syndeom_manifest::{PROOF_LINE, StatementStatus, TERMINATOR, THEOREM_MARKER, is_lemma_step};

/// How a lemma step is declared in the formal file (0.3.0): a `theorem` whose proof is the step's
/// fragment, or an `axiom` with no proof (FLAW-0009's smuggled lemma).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LemmaKind {
    Theorem,
    Axiom,
}

/// A lemma step's declaration (ADR-0011 D2). The claim is a closed statement: the declaration has
/// no binders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LemmaHeader {
    pub kind: LemmaKind,
    pub name: String,
    pub claim: String,
}

/// One step of the argument: its informal text (ADR-0001 D3, without the marker) and the aligned
/// Lean fragment — tactic lines of the main proof with their relative indentation, or, for a
/// lemma step, the lemma's proof. A step may have no fragment (its content is carried by a
/// neighbour's tactic; an `axiom` lemma has none by construction).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub text: String,
    pub fragment: Vec<String>,
    /// Present iff the step is a lemma (0.3.0); absent in every 0.1.0 and 0.2.0 seed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lemma: Option<LemmaHeader>,
}

/// A whole artifact before rendering (ADR-0011 D2). Lemmas are steps with a [`LemmaHeader`]
/// (0.3.0); padding steps arrive with the producer version that realizes FLAW-0013.
#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub title: String,
    pub statement: String,
    pub imports: Vec<String>,
    /// An optional `/-- … -/` line above the main declaration (used by the D8 writer test).
    pub doc: Option<String>,
    pub main_declaration: String,
    /// The binders between the declaration's name and its colon; empty for a closed statement
    /// (an `s2_template` 0.2.0 seed may have none).
    pub binders: String,
    pub claim: String,
    pub truth: StatementStatus,
    pub steps: Vec<Step>,
}

/// The formal file and, per step (0-based index into `Plan::steps`), the 1-based line range of
/// its fragment in that file — for a lemma step, of its whole declaration — `None` for a step
/// without lines; and, per step, the declaration those lines belong to (the lemma's name, or the
/// main declaration).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalRendering {
    pub bytes: Vec<u8>,
    pub fragment_lines: Vec<Option<[u64; 2]>>,
    pub declarations: Vec<String>,
}

/// The file's own declarations in order of first appearance: every lemma, then the main
/// declaration (the names the audit tail lists; ADR-0011 D8).
pub fn own_declarations(rendering: &FormalRendering) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for d in &rendering.declarations {
        if !out.contains(d) {
            out.push(d.clone());
        }
    }
    out
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

/// ADR-0011 D2: lemma steps are numbered `Lemma 1.`, `Lemma 2.`, … in document order, only a step
/// with a lemma header begins that way, an `axiom` lemma has no proof lines, and a `theorem`
/// lemma has some.
pub fn check_lemmas(plan: &Plan) -> Result<(), String> {
    let mut n = 0;
    for (i, step) in plan.steps.iter().enumerate() {
        let k = i + 1;
        match &step.lemma {
            Some(l) => {
                n += 1;
                if !step.text.starts_with(&format!("Lemma {n}. ")) {
                    return Err(format!(
                        "step {k} is lemma {n} but its text does not begin \"Lemma {n}. \""
                    ));
                }
                check_text(&format!("step {k}'s lemma name"), &l.name)?;
                check_text(&format!("step {k}'s lemma claim"), &l.claim)?;
                if l.name.contains(char::is_whitespace) || l.claim.contains('\n') {
                    return Err(format!("step {k}: a lemma's name and claim are one line"));
                }
                match l.kind {
                    LemmaKind::Axiom if !step.fragment.is_empty() => {
                        return Err(format!("step {k}: an axiom has no proof lines"));
                    }
                    LemmaKind::Theorem if step.fragment.is_empty() => {
                        return Err(format!("step {k}: a theorem lemma has proof lines"));
                    }
                    _ => {}
                }
            }
            None if is_lemma_step(&step.text) => {
                return Err(format!(
                    "step {k} begins \"Lemma N.\" but has no lemma header"
                ));
            }
            None => {}
        }
    }
    Ok(())
}

/// The lines of a lemma step's declaration as the formal writer emits them: the header, then the
/// proof lines indented two spaces (none for an `axiom`).
pub fn lemma_lines(step: &Step) -> Result<Vec<String>, String> {
    let l = step.lemma.as_ref().ok_or("not a lemma step")?;
    let mut out = vec![match l.kind {
        LemmaKind::Theorem => format!("theorem {} : {} := by", l.name, l.claim),
        LemmaKind::Axiom => format!("axiom {} : {}", l.name, l.claim),
    }];
    out.extend(step.fragment.iter().map(|f| format!("  {f}")));
    Ok(out)
}

fn check_fragment(k: usize, step: &Step) -> Result<(), String> {
    for l in &step.fragment {
        if l.trim().is_empty() || l.ends_with(' ') || l.contains(['\r', '\t', '\n']) {
            return Err(format!("step {k} has a malformed fragment line"));
        }
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
    check_lemmas(plan)?;
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

/// The `syndeom-lean/1` writer (ADR-0011 D2): the imports, a blank line, every lemma step's
/// declaration in step order (each followed by a blank line), the optional doc line, the main
/// declaration, and its proof as the other steps' fragments in step order, indented two spaces.
/// A declaration without binders is written `theorem <name> : <claim> := by` (no double space);
/// every 0.1.0 seed has binders, so no 0.1.0 byte depends on this case. No 0.1.0 or 0.2.0 seed
/// has a lemma step, so no byte of theirs depends on the lemma rendering.
pub fn render_formal(plan: &Plan) -> Result<FormalRendering, String> {
    if plan.imports.is_empty() {
        return Err("a formal file imports at least one Mathlib module".into());
    }
    check_lemmas(plan)?;
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
    let mut fragment_lines: Vec<Option<[u64; 2]>> = vec![None; plan.steps.len()];
    let mut declarations = vec![plan.main_declaration.clone(); plan.steps.len()];
    for (i, step) in plan.steps.iter().enumerate() {
        let Some(l) = &step.lemma else {
            continue;
        };
        check_fragment(i + 1, step)?;
        let mut range: Option<[u64; 2]> = None;
        for text in lemma_lines(step)? {
            let n = push(&mut out, &text);
            range = Some([range.map_or(n, |r| r[0]), n]);
        }
        push(&mut out, "");
        fragment_lines[i] = range;
        declarations[i] = l.name.clone();
    }
    if let Some(doc) = &plan.doc {
        check_text("doc", doc)?;
        push(&mut out, &format!("/-- {doc} -/"));
    }
    let header = if plan.binders.is_empty() {
        format!("theorem {} : {} := by", plan.main_declaration, plan.claim)
    } else {
        format!(
            "theorem {} {} : {} := by",
            plan.main_declaration, plan.binders, plan.claim
        )
    };
    push(&mut out, &header);
    let mut any = false;
    for (i, step) in plan.steps.iter().enumerate() {
        if step.lemma.is_some() {
            continue;
        }
        check_fragment(i + 1, step)?;
        let mut range: Option<[u64; 2]> = None;
        for l in &step.fragment {
            let n = push(&mut out, &format!("  {l}"));
            range = Some([range.map_or(n, |r| r[0]), n]);
        }
        any |= range.is_some();
        fragment_lines[i] = range;
    }
    if !any {
        return Err("the main proof has no tactic line".into());
    }
    Ok(FormalRendering {
        bytes: out.into_bytes(),
        fragment_lines,
        declarations,
    })
}
