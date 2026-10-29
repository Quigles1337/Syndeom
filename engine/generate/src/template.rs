//! `s2_template` (ADR-0011 D1, D6): hand-authored seed theorems with aligned informal and formal
//! steps. `prepare` runs `plan` → `inject` → `pad` → `render`; `assemble` runs `measure` →
//! `manifest` once the Lean harness has observed the formal file (D2's pipeline). A config
//! commands either one injected instance or padding, never both (D5: two whole-argument
//! variants cannot be made disjoint, and a padded flawed argument is a later version).
//!
//! The crate carries seven producer versions (ADR-0011 *Amendments*, Al's rulings 3 and 5 on PR
//! 26): `0.1.0`, whose three seeds and per-record counts stay regenerable as the goldens name them;
//! `0.2.0`, with four more seeds and counts per instance; `0.3.0`, with lemma steps (D2), the T
//! check (D5, D8: FLAW-0007 `unused` and FLAW-0009) and the `n_lt_two_pow` seed; `0.4.0`, with
//! the padding module (D2 `pad`, D5 FLAW-0013, Options 8) and the `sqrt2_irrational` seed, which
//! carries padding material; `0.5.0`, with FLAW-0004 (the hedge lexicon, `hidden_step`; the
//! aligned tactic step omitted), realized on `sqrt2_irrational` as FLAWS.md's FLAW-0011 example,
//! and FLAW-0011 as a commanded composite (`embedded`) of one gap-or-fatal instance of FLAW-0001
//! through FLAW-0010 (Al's rulings 2 and 3 on PR 30); `0.6.0`, with FLAW-0010 `a` (the
//! fictional-reference lexicon, `shape`, `claim`; the formal file unchanged) on
//! `sqrt2_irrational` and FLAW-0003 (`direct`: the goal or its contrapositive as a lemma proved
//! by a forward reference to the theorem, check K; `cycle`: the lemma and the theorem citing
//! each other, checked on the plan's premise graph) on the seed `bounded_monotone_converges`;
//! and `0.7.0`, the version `generate` emits, with FLAW-0008 (`transformation`: the formal
//! statement drifts by exactly one recorded edit while the informal file stays faithful; the
//! drifted file builds, `NO`) on the seed `bolzano_weierstrass` and FLAW-0012
//! (`dropped_hypothesis`: a false statement's convincing argument, the true-case proof
//! transplanted, failing at the hidden use site, which carries a FLAW-0005 `violated` record)
//! on the seed `bounded_continuous_sup`.
//!
//! A seed's flaw material is a whole-argument variant of one style: the flawed step list, each
//! step naming the faithful step it replaces (`from`), the flaw site, and the anchor text at the
//! site. The injector selects the material the spec names, applies it to the plan, and writes the
//! record's dials with every original text it replaced or removed (D5). A seed whose statement is
//! false has no faithful argument and no control; its material is *intrinsic*: the style's steps
//! are the convincing argument as the FLAWS.md example writes it, nothing is replaced, and the
//! record carries the seed's counterexample (0.2.0; a reading flagged for Al in the amendment).
//! A seed's padding material (0.4.0) is a list of paragraphs, each true irrelevant material as a
//! whole step with the unused declaration that matches it in the formal file. FLAW-0004 material
//! (0.5.0) is replacement material whose site step is the bare claim: the injector draws a hedge
//! from the fixed lexicon and prefixes it, records the justification the faithful step held and
//! the tactic lines omitted from the aligned fragment, and the kernel reports the unsolved goal at
//! the site. A FLAW-0011 composite (0.5.0) applies one embedded spec of severity gap or fatal to
//! a true statement, so that the argument is invalid, and adds its own record at the statement.
//! FLAW-0010 material (0.6.0) is replacement material whose site step is the bare claim: the
//! injector draws a reference from the fixed lexicon of obviously fictional tokens and prefixes
//! `By <reference>`, records the justification the reference stands in for, and leaves the formal
//! file unchanged. FLAW-0003 material (0.6.0) inserts the lemma that carries the circularity;
//! `cycle` material carries `premises` (D2), from which the record's cycle and citation spans are
//! computed and checked. FLAW-0008 material (0.7.0) is replacement material of the *statement*
//! (site 0): its `drift` names the drifted binders and claim and the one edit that carries the
//! drift, and its steps are the drifted statement's own correct proof with every informal text
//! faithful; the record is formal-only, at the main declaration's header line, and is confirmed
//! when the file builds, the faithful statement has its kernel-checked witness, and the two
//! statement texts differ by exactly the transformation's edit. FLAW-0012 material (0.7.0) is
//! intrinsic material of a false statement whose `use` names the theorem applied at the hidden
//! use site and the hypothesis it lacks: commanding FLAW-0012 writes the FLAW-0005 `violated`
//! instance at that site (check K) and the FLAW-0012 instance at the statement, confirmed when
//! the use site's records pass and the seed's negation witness is kernel-checked.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value, json};
use syndeom_manifest::{
    Achieved, Artifact, Commanded, Derivation, FILE_FORMAL, FILE_INFORMAL, FORMAT_INFORMAL_MD,
    FORMAT_LEAN, FileBytes, Files, Flaw, FormalFile, FormalLocation, FormalTag, Generator,
    GroundTruth, InformalFile, InformalLocation, InformalTag, Kind, LeanCatchable, Location,
    MANIFEST_SCHEMA, Manifest, Origin, ProofStatus, Severity, StatementMethod, StatementStatus,
    Storage, Tolerance, Toolchain, artifact_id, config_hash, content_hash, count_tokens, fidelity,
    is_lemma_step, parse_informal_md, parse_lean, span_text,
};

use crate::harness::{Observation, used_closure};
use crate::plan::{
    FormalRendering, LemmaHeader, LemmaKind, PaddingHeader, PaddingKind, Plan, Premise, Step,
    THEOREM_PREMISE, header_line, lemma_lines, padding_lines, premise_cycle, render_formal,
    render_informal,
};
use crate::rng::{RNG_ID, Stream};

pub const GENERATOR_ID: &str = "s2_template";
/// The producer version `generate` emits (ADR-0011 D1; ruling 5 on PR 26).
pub const GENERATOR_VERSION: &str = "0.7.0";
/// The FLAWS.md version every record is read against (ADR-0011 D7).
pub const TAXONOMY_VERSION: &str = "0.1.0";

/// A producer version this crate regenerates. `generate` emits [`Version::CURRENT`]; the goldens
/// select the version their manifests name, and `regenerate`/`verify` accept any of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    V0_1_0,
    V0_2_0,
    V0_3_0,
    V0_4_0,
    V0_5_0,
    V0_6_0,
    V0_7_0,
}

impl Version {
    pub const CURRENT: Version = Version::V0_7_0;

    pub fn parse(s: &str) -> Option<Version> {
        match s {
            "0.1.0" => Some(Version::V0_1_0),
            "0.2.0" => Some(Version::V0_2_0),
            "0.3.0" => Some(Version::V0_3_0),
            "0.4.0" => Some(Version::V0_4_0),
            "0.5.0" => Some(Version::V0_5_0),
            "0.6.0" => Some(Version::V0_6_0),
            "0.7.0" => Some(Version::V0_7_0),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Version::V0_1_0 => "0.1.0",
            Version::V0_2_0 => "0.2.0",
            Version::V0_3_0 => "0.3.0",
            Version::V0_4_0 => "0.4.0",
            Version::V0_5_0 => "0.5.0",
            Version::V0_6_0 => "0.6.0",
            Version::V0_7_0 => "0.7.0",
        }
    }

    /// How many of [`SEED_RECORDS`] the version compiles in: the v0.1 set, then the v0.2
    /// additions, then the v0.3 addition, then the v0.4 addition (0.5.0 adds no seed), then the
    /// v0.6 addition, then the two v0.7 additions.
    fn seed_count(self) -> usize {
        match self {
            Version::V0_1_0 => 3,
            Version::V0_2_0 => 7,
            Version::V0_3_0 => 8,
            Version::V0_4_0 | Version::V0_5_0 => 9,
            Version::V0_6_0 => 10,
            Version::V0_7_0 => 12,
        }
    }

    /// From 0.5.0 the injector realizes FLAW-0004 (the hedge lexicon and `hidden_step`) and
    /// FLAW-0011 as a commanded composite; from 0.6.0 FLAW-0010 `a` (the fictional-reference
    /// lexicon) and FLAW-0003 (`direct` and `cycle`); from 0.7.0 FLAW-0008 (the statement
    /// drift) and FLAW-0012 (the dropped hypothesis); earlier versions refuse those specs.
    fn realizes(self, class: &str) -> bool {
        match class {
            "FLAW-0004" | "FLAW-0011" => self >= Version::V0_5_0,
            "FLAW-0003" | "FLAW-0010" => self >= Version::V0_6_0,
            "FLAW-0008" | "FLAW-0012" => self >= Version::V0_7_0,
            _ => true,
        }
    }

    /// Ruling 3 on PR 26: from 0.2.0 the flaw counts are per instance; 0.1.0 counted records.
    fn counts_instances(self) -> bool {
        self != Version::V0_1_0
    }

    /// From 0.4.0 the padding module realizes FLAW-0013; earlier versions refuse it.
    fn pads(self) -> bool {
        self >= Version::V0_4_0
    }
}

/// The seed records, compiled in so the bytes are a function of the engine commit (ADR-0011
/// D6). The first three are 0.1.0's; a version compiles in a prefix of this list.
const SEED_RECORDS: &[&str] = &[
    include_str!("../seeds/nat_le_sq.json"),
    include_str!("../seeds/n_sq_mod_four.json"),
    include_str!("../seeds/sq_plus_one_ge_two_x.json"),
    include_str!("../seeds/continuity_is_not_uniform.json"),
    include_str!("../seeds/x_over_abs_x.json"),
    include_str!("../seeds/rolle_abs.json"),
    include_str!("../seeds/horses.json"),
    include_str!("../seeds/n_lt_two_pow.json"),
    include_str!("../seeds/sqrt2_irrational.json"),
    include_str!("../seeds/bounded_monotone_converges.json"),
    include_str!("../seeds/bolzano_weierstrass.json"),
    include_str!("../seeds/bounded_continuous_sup.json"),
];

/// Dial keys the injector or the padding module writes into a record; a spec may not use them.
/// `claim` is FLAW-0004's recorded bare claim and, as the one exception, FLAW-0010's own
/// `true | false` dial (ADR-0011 D5), which the config sets.
const RESERVED_DIALS: &[&str] = &[
    "site",
    "inserted",
    "replaced",
    "original_steps",
    "original_fragments",
    "counterexample",
    "declaration",
    "ratio",
    "kind",
    "paragraph",
    "hedge",
    "hedge_text",
    "justification",
    "claim",
    "omitted",
    "embedded_instances",
    "author",
    "year",
    "venue",
    "result",
    "reference",
    "asserted",
    "cycle",
    "citations",
    "faithful_statement",
    "drifted_statement",
    "edit",
    "use_site",
    "use_instance",
];

fn required_option<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(d)
}

/// A seed theorem record (ADR-0011 D6).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub id: String,
    /// Where the material comes from (Al's ruling 1 of 2026-10-27): never `groundtruth/`.
    pub source: String,
    pub licence: String,
    pub title: String,
    pub statement: String,
    pub truth: StatementStatus,
    /// A seed whose statement is false names its counterexample in words (ADR-0011 D7); the
    /// Lean declaration `truth.reference` proves the negation. Absent for a true statement.
    #[serde(default)]
    pub counterexample: Option<String>,
    pub imports: Vec<String>,
    pub binders: String,
    pub claim: String,
    pub styles: Vec<Style>,
    /// FLAW-0013 padding material (0.4.0; ADR-0011 D5, D6), usable with every style of the
    /// seed: true irrelevant paragraphs, each with its unused declaration. Empty for a seed
    /// without padding material, on which a positive `padding_ratio` is refused.
    #[serde(default)]
    pub padding: Vec<Paragraph>,
}

/// One paragraph of FLAW-0013 padding material (0.4.0): its kind, its informal text (a whole
/// step, never a `Lemma N.` step), and the matching unused declaration in the formal file — a
/// `theorem` with this name and claim, proved by `fragment`, used by nothing.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Paragraph {
    pub kind: PaddingKind,
    pub text: String,
    pub name: String,
    pub claim: String,
    pub fragment: Vec<String>,
}

impl Paragraph {
    fn step(&self) -> Step {
        Step {
            text: self.text.clone(),
            fragment: self.fragment.clone(),
            lemma: None,
            padding: Some(PaddingHeader {
                kind: self.kind,
                name: self.name.clone(),
                claim: self.claim.clone(),
            }),
            premises: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Style {
    pub id: String,
    pub steps: Vec<Step>,
    pub flaws: Vec<Material>,
}

/// One realizable flaw instance of a style: the spec it answers and the flawed argument.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    pub class: String,
    #[serde(deserialize_with = "required_option")]
    pub sub_variant: Option<String>,
    pub severity: Severity,
    pub lean_catchable: LeanCatchable,
    pub dials: Value,
    /// The flaw site: a step index of the argument, 1-based.
    pub site: u64,
    /// The changed text inside the site step; `null` means the whole step.
    #[serde(deserialize_with = "required_option")]
    pub anchor: Option<Anchor>,
    /// `true` for a seed whose statement is false: the style's steps are the convincing argument
    /// itself, `steps` is empty, and the record replaces nothing.
    #[serde(default)]
    pub intrinsic: bool,
    /// `true` for material that inserts a step (0.3.0; FLAW-0007 `unused`): `site` is then the
    /// 1-based index of the inserted step in `steps`, the one step that stands in for no
    /// faithful step (`from: null`).
    #[serde(default)]
    pub inserts: bool,
    /// FLAW-0004 material (0.5.0): the hidden step's claim, justification and omitted tactic
    /// lines; present iff the class is FLAW-0004.
    #[serde(default)]
    pub hidden: Option<HiddenStep>,
    /// FLAW-0010 material (0.6.0): the cited step's bare claim and the justification the
    /// fictional reference stands in for; present iff the class is FLAW-0010.
    #[serde(default)]
    pub cited: Option<CitedStep>,
    /// FLAW-0008 material (0.7.0): the seed's drifted statement for the material's
    /// transformation and the one edit that carries the drift; present iff the class is
    /// FLAW-0008, whose `site` is 0, the statement.
    #[serde(default)]
    pub drift: Option<Drift>,
    /// FLAW-0012 material (0.7.0): the theorem applied at the hidden use site (`site`) and the
    /// hypothesis the dropped one supplied there; present iff the class is FLAW-0012.
    #[serde(default, rename = "use")]
    pub hidden_use: Option<HiddenUse>,
    pub steps: Vec<FlawedStep>,
}

/// FLAW-0008's `transformation` dial (ADR-0011 D5; 0.7.0), FLAWS.md's recipe: exactly one of
/// these is applied to the faithful formalization.
pub const TRANSFORMATIONS: &[&str] = &[
    "add-hypothesis",
    "restrict-type",
    "flip-strictness",
    "weaken-conclusion",
    "shadow-definition",
];

/// FLAW-0008 material at the statement (0.7.0; ADR-0011 D5): the drifted statement's binders and
/// claim, which replace the faithful ones in the formal file (the informal claim is unchanged),
/// and the one edit that carries the drift — `replaced`, the faithful text, and `inserted`, what
/// stands in its place. The transformation fixes how the edit applies ([`check_drift`]). The
/// material's steps are the drifted statement's own correct proof, every informal text faithful.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Drift {
    pub binders: String,
    pub claim: String,
    pub replaced: String,
    pub inserted: String,
}

/// FLAW-0012 material's hidden use site (0.7.0; ADR-0011 D5): the theorem the convincing argument
/// applies there and the hypothesis the dropped one supplied — the FLAW-0005 `violated` record
/// written at the site when FLAW-0012 is commanded.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HiddenUse {
    pub theorem: String,
    pub hypothesis: String,
}

/// A statement as the two parts of the header the formal writer emits.
fn statement_json(binders: &str, claim: &str) -> Value {
    json!({"binders": binders, "claim": claim})
}

/// D5's FLAW-0008 check, "the drifted statement text differs from the faithful one by exactly
/// the recorded transformation": the one edit `replaced` → `inserted` applied to the faithful
/// binders and claim yields the drifted ones, in the way the transformation fixes —
/// `add-hypothesis`: one binder `(<name> : <type>)` appended after the replaced binder text,
/// the claim unchanged; `restrict-type`: every occurrence of one whitespace-free type token
/// replaced by another, in the binders and the claim; `flip-strictness`: one occurrence of a
/// text replaced by the same text with exactly one relation symbol flipped between `≤` and `<`
/// (or `≥` and `>`); `weaken-conclusion`: one occurrence inside the claim replaced, the binders
/// unchanged; `shadow-definition`: one occurrence inside the claim replaced by
/// `let <name> := <definition>; <body>` whose body uses the name, the binders unchanged. Every
/// transformation changes the statement.
pub fn check_drift(
    transformation: &str,
    faithful: (&str, &str),
    drifted: (&str, &str),
    replaced: &str,
    inserted: &str,
) -> Result<(), String> {
    let (fb, fc) = faithful;
    let (db, dc) = drifted;
    if replaced.is_empty() || inserted.is_empty() || replaced == inserted {
        return Err("the edit replaces one non-empty text by a different non-empty text".into());
    }
    if (fb, fc) == (db, dc) {
        return Err("the drifted statement differs from the faithful one".into());
    }
    let once = |hay: &str| hay.matches(replaced).count() == 1;
    let ok = match transformation {
        "add-hypothesis" => {
            let added = inserted
                .strip_prefix(replaced)
                .map(str::trim_start)
                .unwrap_or("");
            dc == fc
                && once(fb)
                && db == fb.replacen(replaced, inserted, 1)
                && added.starts_with('(')
                && added.ends_with(')')
                && added.contains(" : ")
        }
        "restrict-type" => {
            !replaced.contains(char::is_whitespace)
                && !inserted.contains(char::is_whitespace)
                && (fb.contains(replaced) || fc.contains(replaced))
                && db == fb.replace(replaced, inserted)
                && dc == fc.replace(replaced, inserted)
        }
        "flip-strictness" => {
            let f = format!("{fb}\n{fc}");
            let d = format!("{db}\n{dc}");
            let flips: Vec<(char, char)> = replaced
                .chars()
                .zip(inserted.chars())
                .filter(|(a, b)| a != b)
                .collect();
            once(&f)
                && d == f.replacen(replaced, inserted, 1)
                && replaced.chars().count() == inserted.chars().count()
                && matches!(
                    flips.as_slice(),
                    [('≤', '<') | ('<', '≤') | ('≥', '>') | ('>', '≥')]
                )
        }
        "weaken-conclusion" => db == fb && once(fc) && dc == fc.replacen(replaced, inserted, 1),
        "shadow-definition" => {
            let shadow = inserted
                .strip_prefix("let ")
                .and_then(|rest| rest.split_once(" := "))
                .and_then(|(name, rest)| {
                    rest.split_once("; ")
                        .map(|(definition, body)| (name, definition, body))
                });
            db == fb
                && once(fc)
                && dc == fc.replacen(replaced, inserted, 1)
                && shadow.is_some_and(|(name, definition, body)| {
                    !name.is_empty()
                        && !name.contains(char::is_whitespace)
                        && !definition.is_empty()
                        && body.contains(name)
                })
        }
        _ => {
            return Err(format!(
                "FLAW-0008's transformation is one of {TRANSFORMATIONS:?}, not {transformation:?}"
            ));
        }
    };
    if ok {
        Ok(())
    } else {
        Err(format!(
            "the drifted statement does not differ from the faithful one by exactly the {transformation} edit"
        ))
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Anchor {
    /// The faithful text at the site; `null` only in intrinsic material.
    #[serde(deserialize_with = "required_option")]
    pub replaced: Option<String>,
    pub inserted: String,
}

/// FLAW-0004's fixed hedge lexicon (ADR-0011 D5; 0.5.0). The record's `hedge` dial is an index
/// into it, drawn from the instance's stream after the material; `hedge_text` is the entry.
pub const HEDGES: &[&str] = &[
    "Clearly",
    "It is easy to see that",
    "Obviously",
    "By a standard argument",
    "One checks that",
];

/// FLAW-0004's `hidden_step` dial and the severity it fixes (ADR-0011 D5).
fn hidden_step_severity(hidden_step: &str) -> Option<Severity> {
    match hidden_step {
        "routine-true" => Some(Severity::Cosmetic),
        "nontrivial-true" => Some(Severity::Gap),
        "false" => Some(Severity::Fatal),
        _ => None,
    }
}

/// FLAW-0004 material at a site (0.5.0; ADR-0011 D5): the step's bare claim, to which the
/// injector prefixes the drawn hedge; the justification the recipe deletes from the faithful
/// step, which holds it exactly once; and the aligned tactic lines it omits from the faithful
/// fragment — a contiguous block, never replaced by `sorry`, so the kernel reports an unsolved
/// goal at the site.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HiddenStep {
    pub claim: String,
    pub justification: String,
    pub omitted: Vec<String>,
}

/// FLAW-0010 `a`'s fixed lexicon of obviously fictional tokens (ADR-0011 D5; 0.6.0): never a
/// real author's name, venue or identifier. A reference is `[<author> <year>, <venue>, <result>]`
/// — the shape of FLAWS.md's example `[Placeholder 19XX, Theorem 4.2]` with the venue the spec's
/// `shape` selects — and every token of it is a token of one of these entries.
pub const CITATION_AUTHORS: &[&str] = &["Placeholder", "Nonesuch", "Fictitious", "Hypothetical"];
pub const CITATION_YEARS: &[&str] = &["19XX", "20XX"];
pub const CITATION_VENUES_JOURNAL: &[&str] = &[
    "Journal of Nonexistent Mathematics",
    "Annals of Imaginary Results",
    "Fictitious Mathematical Quarterly",
];
pub const CITATION_VENUES_BOOK: &[&str] = &[
    "Treatise on Nonesuch Lemmas",
    "Elements of Imaginary Analysis",
    "Handbook of Fictitious Theorems",
];
pub const CITATION_VENUES_PREPRINT: &[&str] = &[
    "preprint XXXX.XXXXX",
    "preprint YYYY.YYYYY",
    "preprint ZZZZ.ZZZZZ",
];
pub const CITATION_RESULTS: &[&str] = &[
    "Theorem 4.2",
    "Lemma 3.1",
    "Proposition 2.5",
    "Corollary 1.7",
];
/// FLAW-0010's `shape` dial (ADR-0011 D5).
pub const CITATION_SHAPES: &[&str] = &["journal", "book", "preprint"];

/// The venue list a `shape` draws from.
pub fn citation_venues(shape: &str) -> Option<&'static [&'static str]> {
    match shape {
        "journal" => Some(CITATION_VENUES_JOURNAL),
        "book" => Some(CITATION_VENUES_BOOK),
        "preprint" => Some(CITATION_VENUES_PREPRINT),
        _ => None,
    }
}

/// The reference the lexicon indices name, or `None` if the shape is unknown or an index is out
/// of range.
pub fn citation_reference(
    shape: &str,
    author: usize,
    year: usize,
    venue: usize,
    result: usize,
) -> Option<String> {
    Some(format!(
        "[{} {}, {}, {}]",
        CITATION_AUTHORS.get(author)?,
        CITATION_YEARS.get(year)?,
        citation_venues(shape)?.get(venue)?,
        CITATION_RESULTS.get(result)?
    ))
}

/// D5's check: every token of `reference` — split on whitespace, with brackets and commas
/// stripped — is a token of some lexicon entry, and there is at least one.
pub fn citation_tokens_in_lexicon(reference: &str) -> bool {
    let lexicon: BTreeSet<&str> = CITATION_AUTHORS
        .iter()
        .chain(CITATION_YEARS)
        .chain(CITATION_VENUES_JOURNAL)
        .chain(CITATION_VENUES_BOOK)
        .chain(CITATION_VENUES_PREPRINT)
        .chain(CITATION_RESULTS)
        .flat_map(|e| e.split_whitespace())
        .collect();
    let tokens: Vec<&str> = reference
        .split_whitespace()
        .map(|t| t.trim_matches(|c| matches!(c, '[' | ']' | ',')))
        .filter(|t| !t.is_empty())
        .collect();
    !tokens.is_empty() && tokens.iter().all(|t| lexicon.contains(t))
}

/// A drawn citation (0.6.0): the lexicon indices and the reference they name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Citation {
    pub author: usize,
    pub year: usize,
    pub venue: usize,
    pub result: usize,
    pub reference: String,
}

/// Draw a citation from `stream`, after the material draw: the author, the year, the venue (from
/// the shape's list) and the result, in that order. The order is part of the producer's rng
/// definition (ADR-0011 D4).
pub fn draw_citation(stream: &mut Stream, shape: &str) -> Result<Citation, String> {
    let venues = citation_venues(shape).ok_or(format!(
        "FLAW-0010's shape is one of {CITATION_SHAPES:?}, not {shape:?}"
    ))?;
    let author = stream.below(CITATION_AUTHORS.len() as u64) as usize;
    let year = stream.below(CITATION_YEARS.len() as u64) as usize;
    let venue = stream.below(venues.len() as u64) as usize;
    let result = stream.below(CITATION_RESULTS.len() as u64) as usize;
    let reference = citation_reference(shape, author, year, venue, result)
        .ok_or("a drawn lexicon index is in range")?;
    Ok(Citation {
        author,
        year,
        venue,
        result,
        reference,
    })
}

/// FLAW-0010's `claim` dial and the severity it fixes (ADR-0011 D5): the cited step is true
/// (gap) or false (fatal).
fn citation_claim_severity(claim: bool) -> Severity {
    if claim {
        Severity::Gap
    } else {
        Severity::Fatal
    }
}

/// FLAW-0010 `a` material at a site (0.6.0; ADR-0011 D5): the step's bare claim, to which the
/// injector prefixes `By <reference>`, and the justification the reference stands in for, which
/// the faithful step holds exactly once. The formal fragment is the faithful one: the class has
/// no formal manifestation.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CitedStep {
    pub claim: String,
    pub justification: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlawedStep {
    /// The faithful step this one stands in for, 1-based; `null` for an inserted step.
    #[serde(deserialize_with = "required_option")]
    pub from: Option<u64>,
    pub text: String,
    pub fragment: Vec<String>,
    /// The lemma header of a lemma step (0.3.0).
    #[serde(default)]
    pub lemma: Option<LemmaHeader>,
    /// The premises the step cites (0.6.0; FLAW-0003 `cycle` material only).
    #[serde(default)]
    pub premises: Vec<Premise>,
}

/// A flawed step as a step of the plan.
fn step_of_flawed(s: &FlawedStep) -> Step {
    Step {
        text: s.text.clone(),
        fragment: s.fragment.clone(),
        lemma: s.lemma.clone(),
        padding: None,
        premises: s.premises.clone(),
    }
}

/// Every seed record a producer version compiles in.
pub fn seeds(version: Version) -> Vec<Seed> {
    SEED_RECORDS[..version.seed_count()]
        .iter()
        .map(|s| serde_json::from_str(s).expect("a compiled-in seed record parses"))
        .collect()
}

/// The shape rules a seed record must satisfy (ADR-0011 D6, D7): a true statement has no
/// intrinsic material and no counterexample; a false one has only intrinsic material, a
/// counterexample, and a negation witness.
pub fn check_seed(seed: &Seed) -> Result<(), String> {
    let false_statement = match seed.truth.value {
        Some(true) => false,
        Some(false) => true,
        None => {
            return Err(format!(
                "seed {}: a seed's truth is by construction",
                seed.id
            ));
        }
    };
    if false_statement != seed.counterexample.is_some() {
        return Err(format!(
            "seed {}: a counterexample is named iff the statement is false",
            seed.id
        ));
    }
    for style in &seed.styles {
        for m in &style.flaws {
            if m.intrinsic != false_statement {
                return Err(format!(
                    "seed {}: material is intrinsic iff the statement is false",
                    seed.id
                ));
            }
            if m.intrinsic && !m.steps.is_empty() {
                return Err(format!(
                    "seed {}: intrinsic material lists no steps",
                    seed.id
                ));
            }
            if m.inserts {
                if m.intrinsic || m.anchor.is_some() {
                    return Err(format!(
                        "seed {}: insertion material is neither intrinsic nor anchored",
                        seed.id
                    ));
                }
                if m.site < 1 || m.site as usize > m.steps.len() {
                    return Err(format!(
                        "seed {}: an insertion's site is one of its own steps",
                        seed.id
                    ));
                }
                for (i, s) in m.steps.iter().enumerate() {
                    if s.from.is_none() != (i + 1 == m.site as usize) {
                        return Err(format!(
                            "seed {}: exactly the inserted step stands in for no faithful step",
                            seed.id
                        ));
                    }
                }
            } else {
                // FLAW-0008 material (0.7.0) sits at the statement, site 0 (check_drift_material)
                if m.drift.is_none() && (m.site < 1 || m.site as usize > style.steps.len()) {
                    return Err(format!("seed {}: a material's site is a step", seed.id));
                }
                if m.steps.iter().any(|s| s.from.is_none()) {
                    return Err(format!(
                        "seed {}: replacement material names the faithful step each step stands in for",
                        seed.id
                    ));
                }
            }
            if let Some(a) = &m.anchor
                && a.replaced.is_some() == m.intrinsic
            {
                return Err(format!(
                    "seed {}: an anchor replaces text iff the material is not intrinsic",
                    seed.id
                ));
            }
            check_hidden_material(seed, style, m)?;
            check_cited_material(seed, style, m)?;
            check_circularity_material(seed, style, m)?;
            check_drift_material(seed, style, m)?;
            check_dropped_hypothesis_material(seed, style, m)?;
        }
        // FINDING-014 (packet 4): a seed's own steps never carry a padding header; only the
        // padding module sets one (a flawed step cannot carry one, by its shape).
        if style.steps.iter().any(|s| s.padding.is_some()) {
            return Err(format!(
                "seed {}: a style step never carries a padding header",
                seed.id
            ));
        }
    }
    // Padding material (0.4.0): a faithful argument to pad, whole non-lemma steps, proved
    // declarations whose names are distinct from one another, from every lemma the seed
    // declares and from the main declaration.
    if !seed.padding.is_empty() && false_statement {
        return Err(format!(
            "seed {}: padding material belongs to a true statement's faithful argument",
            seed.id
        ));
    }
    let mut taken: BTreeSet<String> = BTreeSet::new();
    taken.insert(main_declaration(&seed.id));
    for style in &seed.styles {
        for s in &style.steps {
            if let Some(l) = &s.lemma {
                taken.insert(l.name.clone());
            }
        }
        for m in &style.flaws {
            for s in &m.steps {
                if let Some(l) = &s.lemma {
                    taken.insert(l.name.clone());
                }
            }
        }
    }
    for para in &seed.padding {
        if is_lemma_step(&para.text) {
            return Err(format!(
                "seed {}: a padding paragraph is not a lemma step",
                seed.id
            ));
        }
        if para.fragment.is_empty() {
            return Err(format!(
                "seed {}: a padding declaration has proof lines",
                seed.id
            ));
        }
        if !taken.insert(para.name.clone()) {
            return Err(format!(
                "seed {}: padding declaration {} collides with another declaration of the seed",
                seed.id, para.name
            ));
        }
    }
    Ok(())
}

/// FLAW-0004 material (0.5.0): `hidden` is present iff the class is FLAW-0004; the material is
/// replacement material at a main-proof step without an anchor, `YES-KERNEL`; its one dial
/// `hidden_step` fixes the severity; the material's site step is the bare claim; the faithful
/// site step holds the justification exactly once and the claim does not; the omitted block
/// occurs exactly once in the faithful fragment, and removing it yields the material's fragment.
fn check_hidden_material(seed: &Seed, style: &Style, m: &Material) -> Result<(), String> {
    let err =
        |what: &str| Err::<(), String>(format!("seed {}: FLAW-0004 material {what}", seed.id));
    let Some(hs) = &m.hidden else {
        if m.class == "FLAW-0004" {
            return err("carries `hidden`");
        }
        return Ok(());
    };
    if m.class != "FLAW-0004" {
        return err("is the only material that carries `hidden`");
    }
    if m.intrinsic || m.inserts || m.anchor.is_some() {
        return err("is replacement material without an anchor");
    }
    if m.lean_catchable != LeanCatchable::YesKernel {
        return err(
            "is YES-KERNEL in a formal file (N/A is the injector's, for an informal-only target)",
        );
    }
    let hidden_step = m
        .dials
        .as_object()
        .filter(|d| d.len() == 1)
        .and_then(|d| d.get("hidden_step"))
        .and_then(Value::as_str);
    let Some(hidden_step) = hidden_step else {
        return err("has exactly the dial hidden_step");
    };
    if hidden_step_severity(hidden_step) != Some(m.severity) {
        return err("has the severity its hidden_step fixes");
    }
    let Some(faithful) = (m.site >= 1)
        .then(|| style.steps.get(m.site as usize - 1))
        .flatten()
    else {
        return err("sits at a step of the style");
    };
    let Some(flawed) = m.steps.iter().find(|s| s.from == Some(m.site)) else {
        return err("keeps its site step");
    };
    if faithful.lemma.is_some() || flawed.lemma.is_some() {
        return err("sits at a main-proof step in 0.5.0");
    }
    if flawed.text != hs.claim {
        return err(
            "has its site step's text equal to the bare claim (the injector prefixes the hedge)",
        );
    }
    if hs.justification.is_empty() || faithful.text.matches(hs.justification.as_str()).count() != 1
    {
        return err("names a justification the faithful step holds exactly once");
    }
    if hs.claim.contains(hs.justification.as_str()) {
        return err("deletes the justification from the claim");
    }
    if hs.omitted.is_empty() {
        return err("omits at least one tactic line");
    }
    let n = hs.omitted.len();
    let positions: Vec<usize> = faithful
        .fragment
        .windows(n)
        .enumerate()
        .filter(|(_, w)| *w == hs.omitted.as_slice())
        .map(|(i, _)| i)
        .collect();
    if positions.len() != 1 {
        return err("omits a block that occurs exactly once in the faithful fragment");
    }
    let mut without = faithful.fragment.clone();
    without.drain(positions[0]..positions[0] + n);
    if without != flawed.fragment {
        return err(
            "has its site fragment equal to the faithful fragment without the omitted block",
        );
    }
    Ok(())
}

/// FLAW-0010 material (0.6.0): `cited` is present iff the class is FLAW-0010; the material is
/// sub-variant `a`, replacement material at a main-proof step without an anchor, `N/A`; its one
/// dial `claim` fixes the severity; the site step's text is the bare claim; the faithful site step
/// holds the justification exactly once and the claim does not; every fragment and every other
/// step's text are the faithful ones (the formal file is unchanged).
fn check_cited_material(seed: &Seed, style: &Style, m: &Material) -> Result<(), String> {
    let err =
        |what: &str| Err::<(), String>(format!("seed {}: FLAW-0010 material {what}", seed.id));
    let Some(cs) = &m.cited else {
        if m.class == "FLAW-0010" {
            return err("carries `cited`");
        }
        return Ok(());
    };
    if m.class != "FLAW-0010" {
        return err("is the only material that carries `cited`");
    }
    if m.sub_variant.as_deref() != Some("a") {
        return err("is sub-variant `a` (b and c are deferred, ADR-0011 Options 5)");
    }
    if m.intrinsic || m.inserts || m.anchor.is_some() {
        return err("is replacement material without an anchor");
    }
    if m.lean_catchable != LeanCatchable::NotApplicable {
        return err("is N/A: the formal file is unchanged");
    }
    let claim = m
        .dials
        .as_object()
        .filter(|d| d.len() == 1)
        .and_then(|d| d.get("claim"))
        .and_then(Value::as_bool);
    let Some(claim) = claim else {
        return err("has exactly the dial claim, true or false");
    };
    if citation_claim_severity(claim) != m.severity {
        return err("has the severity its claim fixes (true: gap; false: fatal)");
    }
    if m.steps.len() != style.steps.len() {
        return err("keeps every faithful step");
    }
    for (j, f) in style.steps.iter().enumerate() {
        let k = (j + 1) as u64;
        let Some(s) = m.steps.iter().find(|s| s.from == Some(k)) else {
            return err("keeps every faithful step");
        };
        if s.fragment != f.fragment || s.lemma != f.lemma || !s.premises.is_empty() {
            return err("leaves every formal fragment unchanged");
        }
        if k == m.site {
            if f.lemma.is_some() {
                return err("sits at a main-proof step in 0.6.0");
            }
            if s.text != cs.claim {
                return err(
                    "has its site step's text equal to the bare claim (the injector prefixes the reference)",
                );
            }
            if cs.justification.is_empty() || f.text.matches(cs.justification.as_str()).count() != 1
            {
                return err("names a justification the faithful step holds exactly once");
            }
            if cs.claim.contains(cs.justification.as_str()) {
                return err("replaces the justification, so the claim does not hold it");
            }
        } else if s.text != f.text {
            return err("changes the site step only");
        }
    }
    Ok(())
}

/// True iff `name` occurs in `line` as a whole Lean identifier: not preceded or followed by an
/// identifier character (so `foo` does not match inside `foo_lemma_1` or `Foo.foo`).
fn mentions_identifier(line: &str, name: &str) -> bool {
    let ident = |c: char| c.is_alphanumeric() || matches!(c, '_' | '.' | '\'' | '!' | '?');
    line.match_indices(name).any(|(at, _)| {
        let before = line[..at].chars().next_back().is_none_or(|c| !ident(c));
        let after = line[at + name.len()..]
            .chars()
            .next()
            .is_none_or(|c| !ident(c));
        before && after
    })
}

/// FLAW-0003 material (0.6.0; ADR-0011 D5): insertion material whose site is a `theorem` lemma,
/// fatal. `direct` is `YES-KERNEL` with exactly the dial `form` (`restated-goal` or
/// `contrapositive`), its lemma proved by a forward reference to the main declaration, and
/// carries no premises (its check is K). `cycle` is `NO` with exactly the dial `cycle_length`
/// (2 in 0.6.0), never references the main declaration formally, and carries premises whose
/// phrases each occur exactly once in their step, citing the theorem or a lemma of the material,
/// and whose graph contains the cycle through the lemma. No other material carries premises.
fn check_circularity_material(seed: &Seed, style: &Style, m: &Material) -> Result<(), String> {
    let err =
        |what: &str| Err::<(), String>(format!("seed {}: FLAW-0003 material {what}", seed.id));
    if m.class != "FLAW-0003" {
        if m.steps.iter().any(|s| !s.premises.is_empty()) {
            return Err(format!(
                "seed {}: premises belong to FLAW-0003 cycle material in 0.6.0",
                seed.id
            ));
        }
        return Ok(());
    }
    if m.intrinsic || !m.inserts || m.anchor.is_some() {
        return err("inserts the lemma that carries the circularity");
    }
    if m.severity != Severity::Fatal {
        return err("is fatal");
    }
    let Some(site) = (m.site >= 1)
        .then(|| m.steps.get(m.site as usize - 1))
        .flatten()
    else {
        return err("sits at one of its own steps");
    };
    let Some(lemma) = &site.lemma else {
        return err("has a lemma at its site");
    };
    if lemma.kind != LemmaKind::Theorem {
        return err("declares its lemma as a theorem");
    }
    let main = main_declaration(&seed.id);
    // the main declaration's name as a whole identifier (the lemma's name extends it)
    let references_main = |s: &FlawedStep| s.fragment.iter().any(|l| mentions_identifier(l, &main));
    match m.sub_variant.as_deref() {
        Some("direct") => {
            if m.lean_catchable != LeanCatchable::YesKernel {
                return err(
                    "`direct` is YES-KERNEL: the lemma's proof references the main declaration forward",
                );
            }
            let form = m
                .dials
                .as_object()
                .filter(|d| d.len() == 1)
                .and_then(|d| d.get("form"))
                .and_then(Value::as_str);
            if !matches!(form, Some("restated-goal" | "contrapositive")) {
                return err("`direct` has exactly the dial form, restated-goal or contrapositive");
            }
            if !references_main(site) {
                return err(
                    "`direct` proves its lemma by the main declaration, a forward reference",
                );
            }
            if m.steps.iter().any(|s| !s.premises.is_empty()) {
                return err("`direct` carries no premises in 0.6.0; its check is K");
            }
        }
        Some("cycle") => {
            if m.lean_catchable != LeanCatchable::No {
                return err("`cycle` is NO: the formal file keeps a faithful proof");
            }
            let n = m
                .dials
                .as_object()
                .filter(|d| d.len() == 1)
                .and_then(|d| d.get("cycle_length"))
                .and_then(Value::as_u64);
            if n != Some(2) {
                return err("`cycle` has exactly the dial cycle_length, 2 in 0.6.0");
            }
            if m.steps.iter().any(references_main) {
                return err("`cycle` never references the main declaration formally");
            }
            for s in &m.steps {
                for premise in &s.premises {
                    if s.text.matches(premise.phrase.as_str()).count() != 1 {
                        return err("has each citation phrase exactly once in its step");
                    }
                    let known = premise.cites == THEOREM_PREMISE
                        || m.steps
                            .iter()
                            .any(|t| t.lemma.as_ref().is_some_and(|l| l.name == premise.cites));
                    if !known {
                        return err("cites the theorem or a lemma of the material");
                    }
                }
            }
            let mut plan = faithful_plan(seed, style);
            plan.steps = m.steps.iter().map(step_of_flawed).collect();
            if premise_cycle(&plan, &lemma.name, 2).is_none() {
                return err("has a premise graph with the cycle through its lemma");
            }
        }
        _ => return err("is `direct` or `cycle`"),
    }
    Ok(())
}

/// FLAW-0008 material (0.7.0; ADR-0011 D5): `drift` is present iff the class is FLAW-0008; the
/// material is replacement material of the statement — site 0, no anchor, neither intrinsic nor
/// an insertion — `NO`, of severity gap or fatal as the seed marks it, with exactly the dial
/// `transformation`; the drifted binders and claim differ from the faithful ones by exactly that
/// transformation's edit ([`check_drift`]); and every faithful step is kept with its informal
/// text, its lemma header and no premises (the informal file is the faithful one), the
/// fragments being the drifted statement's own proof.
fn check_drift_material(seed: &Seed, style: &Style, m: &Material) -> Result<(), String> {
    let err =
        |what: &str| Err::<(), String>(format!("seed {}: FLAW-0008 material {what}", seed.id));
    let Some(d) = &m.drift else {
        if m.class == "FLAW-0008" {
            return err("carries `drift`");
        }
        return Ok(());
    };
    if m.class != "FLAW-0008" {
        return err("is the only material that carries `drift`");
    }
    if m.intrinsic || m.inserts || m.anchor.is_some() {
        return err("is replacement material of the statement, without an anchor");
    }
    if m.site != 0 {
        return err("sits at the statement, site 0");
    }
    if m.lean_catchable != LeanCatchable::No {
        return err("is NO: the drifted file builds (Lean checks the statement it is given)");
    }
    if !matches!(m.severity, Severity::Gap | Severity::Fatal) {
        return err("is gap or fatal, as the seed marks it");
    }
    let transformation = m
        .dials
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("transformation"))
        .and_then(Value::as_str);
    let Some(transformation) = transformation.filter(|t| TRANSFORMATIONS.contains(t)) else {
        return err("has exactly the dial transformation, one of D5's five");
    };
    check_drift(
        transformation,
        (&seed.binders, &seed.claim),
        (&d.binders, &d.claim),
        &d.replaced,
        &d.inserted,
    )
    .map_err(|e| format!("seed {}: FLAW-0008 material: {e}", seed.id))?;
    if m.steps.len() != style.steps.len() {
        return err("keeps every faithful step");
    }
    for (j, f) in style.steps.iter().enumerate() {
        let k = (j + 1) as u64;
        let Some(s) = m.steps.iter().find(|s| s.from == Some(k)) else {
            return err("keeps every faithful step");
        };
        if s.text != f.text || s.lemma != f.lemma || !s.premises.is_empty() {
            return err(
                "leaves every informal step unchanged: the informal file is the faithful one",
            );
        }
    }
    Ok(())
}

/// FLAW-0012 material (0.7.0; ADR-0011 D5): `use` is present iff the class is FLAW-0012; the
/// material is intrinsic (the convincing argument of a false statement, as the example writes
/// it), without a sub-variant, fatal and `YES-KERNEL`, with exactly the dial
/// `dropped_hypothesis`; `use` names the theorem applied at the hidden use site and the
/// hypothesis it lacks; and the site is a main-proof step with a formal fragment, where check K
/// expects the transplanted proof to fail.
fn check_dropped_hypothesis_material(
    seed: &Seed,
    style: &Style,
    m: &Material,
) -> Result<(), String> {
    let err =
        |what: &str| Err::<(), String>(format!("seed {}: FLAW-0012 material {what}", seed.id));
    let Some(u) = &m.hidden_use else {
        if m.class == "FLAW-0012" {
            return err("carries `use`");
        }
        return Ok(());
    };
    if m.class != "FLAW-0012" {
        return err("is the only material that carries `use`");
    }
    if !m.intrinsic || m.inserts {
        return err("is intrinsic: the convincing argument of a false statement, as written");
    }
    if m.sub_variant.is_some() {
        return err("has no sub-variant");
    }
    if m.severity != Severity::Fatal || m.lean_catchable != LeanCatchable::YesKernel {
        return err("is fatal and YES-KERNEL: the transplanted proof fails at the hidden use site");
    }
    let dropped = m
        .dials
        .as_object()
        .filter(|o| o.len() == 1)
        .and_then(|o| o.get("dropped_hypothesis"))
        .and_then(Value::as_str);
    if dropped.is_none_or(str::is_empty) {
        return err(
            "has exactly the dial dropped_hypothesis, naming the hypothesis the seed dropped",
        );
    }
    if u.theorem.is_empty() || u.hypothesis.is_empty() {
        return err("names the theorem applied at the hidden use site and the hypothesis it lacks");
    }
    let site = (m.site >= 1)
        .then(|| style.steps.get(m.site as usize - 1))
        .flatten();
    if site.is_none_or(|s| s.fragment.is_empty() || s.lemma.is_some()) {
        return err(
            "has its hidden use site at a main-proof step with a formal fragment (check K)",
        );
    }
    Ok(())
}

/// `artifact.kind` requested by the config.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Informal,
    Formal,
    Pair,
}

impl Target {
    fn informal(self) -> bool {
        self != Self::Formal
    }
    fn formal(self) -> bool {
        self != Self::Informal
    }
    fn kind(self) -> Kind {
        match self {
            Self::Informal => Kind::Informal,
            Self::Formal => Kind::Formal,
            Self::Pair => Kind::Pair,
        }
    }
}

/// One `FlawSpec` (ADR-0011 D3); `site: None` is `"auto"`.
#[derive(Debug, Clone, PartialEq)]
pub struct FlawSpec {
    pub class: String,
    pub sub_variant: Option<String>,
    pub severity: Severity,
    pub site: Option<u64>,
    pub dials: Map<String, Value>,
}

/// A parsed `s2_template` config (ADR-0011 D3).
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub theorem: String,
    pub style: String,
    pub target: Target,
    pub flaws: Vec<FlawSpec>,
    pub padding_ratio: f64,
}

fn exact_keys(obj: &Map<String, Value>, keys: &[&str], what: &str) -> Result<(), String> {
    let have: BTreeSet<&str> = obj.keys().map(String::as_str).collect();
    let want: BTreeSet<&str> = keys.iter().copied().collect();
    if have != want {
        return Err(format!(
            "{what} must have exactly the keys {want:?} (no defaults elided), has {have:?}"
        ));
    }
    Ok(())
}

fn parse_severity(v: &Value) -> Result<Severity, String> {
    serde_json::from_value(v.clone()).map_err(|_| format!("unknown severity {v}"))
}

/// One `FlawSpec` (ADR-0011 D3) as a config gives it or, from 0.5.0, as FLAW-0011's `embedded`
/// list gives it; `what` names it in a refusal.
fn parse_spec(f: &Value, what: &str) -> Result<FlawSpec, String> {
    let f = f.as_object().ok_or(format!("{what} is an object"))?;
    exact_keys(
        f,
        &["class", "sub_variant", "severity", "site", "dials"],
        what,
    )?;
    let site = match &f["site"] {
        Value::String(s) if s == "auto" => None,
        v => Some(
            v.as_u64()
                .filter(|k| *k >= 1)
                .ok_or(format!("{what}.site is \"auto\" or a step index"))?,
        ),
    };
    let class = f["class"]
        .as_str()
        .ok_or(format!("{what}.class is a string"))?
        .to_string();
    let dials = f["dials"]
        .as_object()
        .ok_or(format!("{what}.dials is an object"))?
        .clone();
    // FLAW-0010's `claim` (true | false) is the config's own dial (ADR-0011 D5); for every other
    // class `claim` is a name the injector writes.
    if let Some(k) = RESERVED_DIALS
        .iter()
        .find(|k| dials.contains_key(**k) && !(class == "FLAW-0010" && **k == "claim"))
    {
        return Err(format!(
            "{what}.dials may not set {k:?}: the injector records it"
        ));
    }
    Ok(FlawSpec {
        class,
        sub_variant: match &f["sub_variant"] {
            Value::Null => None,
            Value::String(s) => Some(s.clone()),
            _ => return Err(format!("{what}.sub_variant is a string or null")),
        },
        severity: parse_severity(&f["severity"])?,
        site,
        dials,
    })
}

/// Parse and shape-check a config; every refusal is an exit-2 refusal.
pub fn parse_config(config: &Value) -> Result<Config, String> {
    let obj = config.as_object().ok_or("the config is a JSON object")?;
    exact_keys(
        obj,
        &["theorem", "style", "target", "flaws", "padding_ratio"],
        "an s2_template config",
    )?;
    let string = |k: &str| {
        obj[k]
            .as_str()
            .map(str::to_string)
            .ok_or(format!("config.{k} is a string"))
    };
    let target = match obj["target"].as_str() {
        Some("informal") => Target::Informal,
        Some("formal") => Target::Formal,
        Some("pair") => Target::Pair,
        _ => return Err("config.target is \"informal\", \"formal\" or \"pair\"".into()),
    };
    let padding_ratio = obj["padding_ratio"]
        .as_f64()
        .filter(|r| r.is_finite() && *r >= 0.0)
        .ok_or("config.padding_ratio is a non-negative number")?;
    let mut flaws = Vec::new();
    for (i, f) in obj["flaws"]
        .as_array()
        .ok_or("config.flaws is an array")?
        .iter()
        .enumerate()
    {
        flaws.push(parse_spec(f, &format!("config.flaws[{i}]"))?);
    }
    Ok(Config {
        theorem: string("theorem")?,
        style: string("style")?,
        target,
        flaws,
        padding_ratio,
    })
}

fn severity_rank(s: Severity) -> u8 {
    match s {
        Severity::Cosmetic => 0,
        Severity::Repairable => 1,
        Severity::Gap => 2,
        Severity::Fatal => 3,
    }
}

/// FLAWS.md 0.1.0 summary table: each class's severity range.
fn severity_range(class: &str) -> Option<(Severity, Severity)> {
    use Severity::*;
    Some(match class {
        "FLAW-0001" => (Repairable, Fatal),
        "FLAW-0002" | "FLAW-0005" | "FLAW-0006" | "FLAW-0008" | "FLAW-0009" | "FLAW-0010"
        | "FLAW-0014" => (Gap, Fatal),
        "FLAW-0003" | "FLAW-0012" => (Fatal, Fatal),
        "FLAW-0004" | "FLAW-0007" | "FLAW-0011" => (Cosmetic, Fatal),
        "FLAW-0013" => (Cosmetic, Cosmetic),
        _ => return None,
    })
}

/// ADR-0011 D3: the class, target and severity refusals that do not depend on the seed.
fn check_spec(spec: &FlawSpec, target: Target) -> Result<(), String> {
    let (lo, hi) = severity_range(&spec.class).ok_or(format!(
        "{} is not a class of FLAWS.md {TAXONOMY_VERSION}",
        spec.class
    ))?;
    let r = severity_rank(spec.severity);
    if r < severity_rank(lo) || r > severity_rank(hi) {
        return Err(format!(
            "severity {:?} is outside {}'s range",
            spec.severity, spec.class
        ));
    }
    let refused = match (spec.class.as_str(), target) {
        ("FLAW-0008", Target::Pair) => false,
        ("FLAW-0008", _) => true,
        ("FLAW-0009", Target::Informal) => true,
        ("FLAW-0004" | "FLAW-0010", Target::Formal) => true,
        // FLAW-0003 `cycle` is informal-only (0.6.0): declaration order forbids a formal cycle,
        // so a formal-only artifact has nothing to carry it
        ("FLAW-0003", Target::Formal) if spec.sub_variant.as_deref() == Some("cycle") => true,
        _ => false,
    };
    if refused {
        return Err(format!(
            "{} cannot be realized with target {target:?}",
            spec.class
        ));
    }
    if spec.class == "FLAW-0010" {
        match spec.sub_variant.as_deref() {
            Some("a") => {}
            Some("b" | "c") => {
                return Err(
                    "FLAW-0010 sub-variants b and c need real references and are deferred (ADR-0011 Options 5); v0.1 realizes a only"
                        .into(),
                );
            }
            _ => return Err("FLAW-0010's sub_variant is \"a\" in v0.1".into()),
        }
    }
    Ok(())
}

/// The applied instance, before rendering fixes its locations.
#[derive(Debug, Clone, PartialEq)]
pub struct Injection {
    pub instance: u64,
    pub class: String,
    pub sub_variant: Option<String>,
    pub severity: Severity,
    pub lean_catchable: LeanCatchable,
    /// The site in the flawed plan, 1-based.
    pub site: usize,
    pub informal_inserted: String,
    pub informal_dials: Value,
    pub formal_inserted: Vec<String>,
    pub formal_dials: Value,
}

/// One inserted padding paragraph (0.4.0, FLAW-0013), before rendering fixes its locations.
#[derive(Debug, Clone, PartialEq)]
pub struct PaddingInsertion {
    pub instance: u64,
    pub kind: PaddingKind,
    /// The paragraph's 1-based index in the seed's padding material.
    pub paragraph: u64,
    /// The inserted step's 1-based index in the padded plan.
    pub site: usize,
    pub text: String,
    pub declaration: String,
    pub informal_dials: Value,
    pub formal_dials: Value,
}

/// FLAW-0011 as a commanded composite (0.5.0; ADR-0011 D5): the record that names its embedded
/// instance(s), inherits the most severe one's severity and `lean_catchable`, and addresses the
/// statement — step 0 with span `[0, len)` in the informal file, the main declaration at its
/// header line in the formal file (D5, notes to the table).
#[derive(Debug, Clone, PartialEq)]
pub struct Composite {
    pub instance: u64,
    pub severity: Severity,
    pub lean_catchable: LeanCatchable,
    /// The embedded specs exactly as the config gave them.
    pub embedded: Value,
    pub embedded_instances: Vec<u64>,
}

/// FLAW-0012 as a commanded class (0.7.0; ADR-0011 D5): the record at the statement — step 0
/// with span `[0, len)` in the informal file, the main declaration at its header line in the
/// formal file — fatal and `YES-KERNEL`, numbered after the FLAW-0005 `violated` instance the
/// injector writes at the hidden use site, and confirmed when that instance's records pass
/// (check K at the use site) and the seed's negation witness is kernel-checked (D7).
#[derive(Debug, Clone, PartialEq)]
pub struct FalseTheorem {
    pub instance: u64,
    /// The spec's dial: the hypothesis the seed dropped to reach the false statement.
    pub dropped_hypothesis: String,
    /// The hidden use site, 1-based, and the instance recorded there.
    pub use_site: usize,
    pub use_instance: u64,
    pub counterexample: String,
}

/// Everything `prepare` produces: the config as given, the plan after injection and padding,
/// the rendered bytes, the injected instance (none for a control or a padded control; the
/// embedded instance of a composite; the use-site instance of a FLAW-0012 artifact), the padding
/// insertions (none unless FLAW-0013 was commanded), the composite (none unless FLAW-0011 was
/// commanded) and the false theorem's own instance (none unless FLAW-0012 was commanded).
#[derive(Debug, Clone, PartialEq)]
pub struct Prepared {
    pub version: Version,
    pub config: Value,
    /// `config_hash` (ADR-0001 D5), computed once; a config that cannot be canonicalized is
    /// refused by `prepare`.
    pub config_hash: String,
    pub seed: u64,
    pub target: Target,
    pub plan: Plan,
    pub informal: Option<Vec<u8>>,
    pub formal: Option<FormalRendering>,
    pub injection: Option<Injection>,
    pub padding: Vec<PaddingInsertion>,
    pub composite: Option<Composite>,
    pub false_theorem: Option<FalseTheorem>,
}

impl Prepared {
    pub fn artifact_id(&self) -> String {
        artifact_id(
            GENERATOR_ID,
            self.version.as_str(),
            &self.config_hash,
            Some(self.seed),
        )
    }
}

/// The main declaration's name (ADR-0011 D2).
pub fn main_declaration(theorem: &str) -> String {
    format!("syndeom_s2_template_{theorem}")
}

/// The plan of a seed's style before injection: the faithful argument of a true statement, or
/// the convincing argument of a false one.
pub fn faithful_plan(seed: &Seed, style: &Style) -> Plan {
    Plan {
        title: seed.title.clone(),
        statement: seed.statement.clone(),
        imports: seed.imports.clone(),
        doc: None,
        main_declaration: main_declaration(&seed.id),
        binders: seed.binders.clone(),
        claim: seed.claim.clone(),
        truth: seed.truth.clone(),
        steps: style.steps.clone(),
    }
}

/// `plan` → `inject` → `pad` → `render` for one (config, seed) under the current producer
/// version.
pub fn prepare(config: &Value, seed_value: u64) -> Result<Prepared, String> {
    prepare_with(Version::CURRENT, config, seed_value)
}

/// `plan` → `inject` → `pad` → `render` for one (config, seed) under a named producer version.
pub fn prepare_with(version: Version, config: &Value, seed_value: u64) -> Result<Prepared, String> {
    let cfg = parse_config(config)?;
    let hash =
        config_hash(config).map_err(|e| format!("the config cannot be canonicalized: {e}"))?;
    let seed = seeds(version)
        .into_iter()
        .find(|s| s.id == cfg.theorem)
        .ok_or(format!(
            "no seed theorem {:?} in s2_template {}",
            cfg.theorem,
            version.as_str()
        ))?;
    check_seed(&seed)?;
    let style = seed
        .styles
        .iter()
        .find(|s| s.id == cfg.style)
        .ok_or(format!("seed {} has no style {:?}", seed.id, cfg.style))?;
    // Padding (0.4.0): commanded by a FLAW-0013 spec together with a positive `padding_ratio`,
    // and by nothing else; earlier versions refuse both.
    let pads = cfg.flaws.iter().any(|s| s.class == "FLAW-0013");
    if !version.pads() {
        if cfg.padding_ratio != 0.0 {
            return Err(format!(
                "seed {} carries no FLAW-0013 padding material in s2_template {}",
                seed.id,
                version.as_str()
            ));
        }
        if pads {
            return Err(format!(
                "FLAW-0013 is not realized by s2_template {}",
                version.as_str()
            ));
        }
    } else if (cfg.padding_ratio > 0.0) != pads {
        return Err(
            "padding is commanded by a FLAW-0013 spec with a positive padding_ratio, and by nothing else"
                .into(),
        );
    }
    if cfg.flaws.len() > 1 {
        return Err(format!(
            "s2_template {} materials are whole-argument variants, so two instances cannot be made disjoint (ADR-0011 D5); a padded flawed argument is a later version",
            version.as_str()
        ));
    }
    if seed.truth.value == Some(false) && (cfg.flaws.is_empty() || pads) {
        return Err(format!(
            "seed {} states a false theorem (its witness proves the negation), so it has no control and nothing to pad: a flaw must be commanded",
            seed.id
        ));
    }
    if let Some(spec) = cfg.flaws.first()
        && !version.realizes(&spec.class)
    {
        return Err(format!(
            "{} is not realized by s2_template {}",
            spec.class,
            version.as_str()
        ));
    }
    let mut plan = faithful_plan(&seed, style);
    let (injection, padding, composite, false_theorem) = match cfg.flaws.first() {
        None => (None, Vec::new(), None, None),
        Some(spec) if spec.class == "FLAW-0013" => (
            None,
            pad(
                &mut plan,
                &seed,
                spec,
                cfg.padding_ratio,
                cfg.target,
                seed_value,
            )?,
            None,
            None,
        ),
        Some(spec) if spec.class == "FLAW-0011" => {
            let (inj, comp) = compose(
                &mut plan, &seed, style, spec, cfg.target, seed_value, version,
            )?;
            (Some(inj), Vec::new(), Some(comp), None)
        }
        Some(spec) if spec.class == "FLAW-0012" => {
            let (inj, ft) = drop_hypothesis(&seed, style, spec, cfg.target, seed_value)?;
            (Some(inj), Vec::new(), None, Some(ft))
        }
        Some(spec) => (
            Some(inject(
                &mut plan, &seed, style, spec, cfg.target, seed_value,
            )?),
            Vec::new(),
            None,
            None,
        ),
    };
    let informal = cfg
        .target
        .informal()
        .then(|| render_informal(&plan))
        .transpose()?;
    let formal = cfg
        .target
        .formal()
        .then(|| render_formal(&plan))
        .transpose()?;
    Ok(Prepared {
        version,
        config: config.clone(),
        config_hash: hash,
        seed: seed_value,
        target: cfg.target,
        plan,
        informal,
        formal,
        injection,
        padding,
        composite,
        false_theorem,
    })
}

/// Where the padding module puts the paragraphs (the `placement` dial, 0.4.0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Placement {
    /// Every paragraph before the first step of the argument, in drawn order: the FLAWS.md
    /// example's "preceded by".
    Before,
    /// Each paragraph at a position drawn from the `padding` stream: before one step of the
    /// plan as it then stands, so the padding interleaves the argument.
    Drawn,
}

/// The padding module (ADR-0011 D2 `pad`, D5 FLAW-0013, Options 8; 0.4.0). Whole steps of true
/// irrelevant material from the seed's padding material, each with its unused declaration, are
/// inserted into the plan. The spec's dials are `kinds` (which kinds are eligible), `count`
/// (how many paragraphs; an integer, as D4 requires of a generation decision) and `placement`
/// (`before` or `drawn`). The paragraphs are drawn without replacement from the `padding`
/// stream, then, under `drawn`, each position is drawn from the same stream. One record per
/// paragraph, each its own instance; the config's `padding_ratio` is recorded as the `ratio`
/// dial and compared only after rendering, in `measure` (D4, D8).
fn pad(
    plan: &mut Plan,
    seed: &Seed,
    spec: &FlawSpec,
    ratio: f64,
    target: Target,
    seed_value: u64,
) -> Result<Vec<PaddingInsertion>, String> {
    check_spec(spec, target)?;
    if spec.sub_variant.is_some() {
        return Err("FLAW-0013 has no sub-variant".into());
    }
    if spec.site.is_some() {
        return Err(
            "FLAW-0013's positions are drawn from the padding stream, so its site is \"auto\""
                .into(),
        );
    }
    exact_keys(
        &spec.dials,
        &["kinds", "count", "placement"],
        "FLAW-0013's dials",
    )?;
    let kinds: Vec<PaddingKind> = spec.dials["kinds"]
        .as_array()
        .ok_or("FLAW-0013's kinds is an array")?
        .iter()
        .map(|k| serde_json::from_value(k.clone()).map_err(|_| format!("unknown padding kind {k}")))
        .collect::<Result<_, _>>()?;
    if kinds.is_empty() {
        return Err("FLAW-0013's kinds names at least one kind".into());
    }
    if kinds.iter().collect::<BTreeSet<_>>().len() != kinds.len() {
        return Err("FLAW-0013's kinds names each kind once".into());
    }
    let count = spec.dials["count"]
        .as_u64()
        .filter(|c| *c >= 1)
        .ok_or("FLAW-0013's count is a positive integer")?;
    let placement = match spec.dials["placement"].as_str() {
        Some("before") => Placement::Before,
        Some("drawn") => Placement::Drawn,
        _ => return Err("FLAW-0013's placement is \"before\" or \"drawn\"".into()),
    };
    if seed.padding.is_empty() {
        return Err(format!(
            "seed {} carries no FLAW-0013 padding material",
            seed.id
        ));
    }
    let mut remaining: Vec<(u64, &Paragraph)> = seed
        .padding
        .iter()
        .enumerate()
        .filter(|(_, p)| kinds.contains(&p.kind))
        .map(|(i, p)| ((i + 1) as u64, p))
        .collect();
    if (remaining.len() as u64) < count {
        return Err(format!(
            "seed {} carries {} padding paragraph(s) of the kinds named, fewer than count {count}",
            seed.id,
            remaining.len()
        ));
    }
    /// Which inserted paragraph a step of the padded plan is, if it is one.
    struct Marker {
        instance: u64,
        paragraph: u64,
        kind: PaddingKind,
    }
    let mut stream = Stream::new(seed_value, "padding");
    let mut steps: Vec<(Step, Option<Marker>)> =
        plan.steps.iter().cloned().map(|s| (s, None)).collect();
    for instance in 1..=count {
        let pick = stream.below(remaining.len() as u64) as usize;
        let (paragraph, para) = remaining.remove(pick);
        let position = match placement {
            Placement::Before => (instance - 1) as usize,
            Placement::Drawn => stream.below(steps.len() as u64) as usize,
        };
        let marker = Marker {
            instance,
            paragraph,
            kind: para.kind,
        };
        steps.insert(position, (para.step(), Some(marker)));
    }
    plan.steps = steps.iter().map(|(s, _)| s.clone()).collect();
    let mut out: Vec<PaddingInsertion> = Vec::new();
    for (k, (step, marker)) in steps.iter().enumerate() {
        let Some(Marker {
            instance,
            paragraph,
            kind,
        }) = marker
        else {
            continue;
        };
        let site = k + 1;
        let declaration = step
            .padding
            .as_ref()
            .map(|p| p.name.clone())
            .ok_or("a padding step carries its header")?;
        let with = |extra: Vec<(&str, Value)>| {
            let mut m = spec.dials.clone();
            m.insert("ratio".into(), json!(ratio));
            m.insert("site".into(), json!(site));
            m.insert("kind".into(), json!(kind.as_str()));
            m.insert("paragraph".into(), json!(paragraph));
            for (key, v) in extra {
                m.insert(key.into(), v);
            }
            Value::Object(m)
        };
        out.push(PaddingInsertion {
            instance: *instance,
            kind: *kind,
            paragraph: *paragraph,
            site,
            text: step.text.clone(),
            declaration: declaration.clone(),
            informal_dials: with(vec![
                ("inserted", json!(step.text)),
                ("replaced", Value::Null),
            ]),
            formal_dials: with(vec![
                ("inserted", json!(padding_lines(step)?)),
                ("replaced", Value::Null),
                ("declaration", json!(declaration)),
            ]),
        });
    }
    out.sort_by_key(|i| i.instance);
    Ok(out)
}

/// The injector (ADR-0011 D5): choose the material, rewrite the plan, write the dials.
fn inject(
    plan: &mut Plan,
    seed: &Seed,
    style: &Style,
    spec: &FlawSpec,
    target: Target,
    seed_value: u64,
) -> Result<Injection, String> {
    check_spec(spec, target)?;
    // FLAW-0010 (0.6.0): `shape` selects the venue list and is not a property of the material,
    // which is matched on the remaining dial (`claim`).
    let shape = if spec.class == "FLAW-0010" {
        exact_keys(&spec.dials, &["shape", "claim"], "FLAW-0010's dials")?;
        let shape = spec.dials["shape"]
            .as_str()
            .filter(|s| CITATION_SHAPES.contains(s))
            .ok_or(format!("FLAW-0010's shape is one of {CITATION_SHAPES:?}"))?;
        if !spec.dials["claim"].is_boolean() {
            return Err("FLAW-0010's claim is true or false".into());
        }
        Some(shape.to_string())
    } else {
        None
    };
    let mut matched = spec.dials.clone();
    if shape.is_some() {
        matched.remove("shape");
    }
    let dials = Value::Object(matched);
    let eligible: Vec<&Material> = style
        .flaws
        .iter()
        .filter(|m| {
            m.class == spec.class
                && m.sub_variant == spec.sub_variant
                && m.severity == spec.severity
                && m.dials == dials
                && spec.site.is_none_or(|k| k == m.site)
        })
        .collect();
    if eligible.is_empty() {
        return Err(format!(
            "style {} has no material for {} {:?} {:?} with dials {dials} at site {:?}",
            style.id, spec.class, spec.sub_variant, spec.severity, spec.site
        ));
    }
    // `auto` and an explicit site both draw from the instance's stream, so the draw sequence
    // does not depend on how the site was given.
    let mut stream = Stream::new(seed_value, &format!("injector/{}/1", spec.class));
    let material = eligible[stream.below(eligible.len() as u64) as usize];
    let with = |extra: Vec<(&str, Value)>| {
        let mut m = spec.dials.clone();
        m.insert("site".into(), json!(material.site));
        for (k, v) in extra {
            m.insert(k.into(), v);
        }
        Value::Object(m)
    };

    if let Some(drift) = &material.drift {
        // FLAW-0008 (0.7.0; D5): the statement drifts. The informal file is the faithful one; the
        // formal file states the seed's drifted statement with its own correct proof. The record
        // is formal-only (`manifests_in`), at the main declaration's header line, and carries
        // both statements and the one edit that carries the drift, so the check can rebuild it.
        let faithful_header = header_line(&plan.main_declaration, &plan.binders, &plan.claim);
        let drifted_header = header_line(&plan.main_declaration, &drift.binders, &drift.claim);
        let mut original_fragments = Map::new();
        for (j, f) in style.steps.iter().enumerate() {
            let k = (j + 1) as u64;
            let changed = material
                .steps
                .iter()
                .find(|s| s.from == Some(k))
                .is_none_or(|s| s.fragment != f.fragment);
            if changed {
                original_fragments.insert(k.to_string(), json!(f.fragment));
            }
        }
        let dials = with(vec![
            ("inserted", json!([drifted_header])),
            ("replaced", json!([faithful_header])),
            (
                "faithful_statement",
                statement_json(&plan.binders, &plan.claim),
            ),
            (
                "drifted_statement",
                statement_json(&drift.binders, &drift.claim),
            ),
            (
                "edit",
                json!({"replaced": drift.replaced, "inserted": drift.inserted}),
            ),
            ("original_fragments", Value::Object(original_fragments)),
            ("declaration", json!(plan.main_declaration)),
        ]);
        plan.binders = drift.binders.clone();
        plan.claim = drift.claim.clone();
        plan.steps = material.steps.iter().map(step_of_flawed).collect();
        return Ok(Injection {
            instance: 1,
            class: material.class.clone(),
            sub_variant: material.sub_variant.clone(),
            severity: material.severity,
            lean_catchable: material.lean_catchable,
            site: 0,
            informal_inserted: drifted_header.clone(),
            informal_dials: dials.clone(),
            formal_inserted: vec![drifted_header],
            formal_dials: dials,
        });
    }

    if material.intrinsic {
        // A false statement's convincing argument: the plan is the style's; nothing is replaced.
        let site = material.site as usize;
        let site_step = &style.steps[site - 1];
        let inserted = match &material.anchor {
            Some(a) => a.inserted.clone(),
            None => site_step.text.clone(),
        };
        let counterexample = json!(seed.counterexample);
        let informal_dials = with(vec![
            ("inserted", json!(inserted)),
            ("replaced", Value::Null),
            ("counterexample", counterexample.clone()),
        ]);
        let formal_dials = with(vec![
            ("inserted", json!(site_step.fragment)),
            ("replaced", Value::Null),
            ("counterexample", counterexample),
        ]);
        return Ok(Injection {
            instance: 1,
            class: material.class.clone(),
            sub_variant: material.sub_variant.clone(),
            severity: material.severity,
            lean_catchable: material.lean_catchable,
            site,
            informal_inserted: inserted,
            informal_dials,
            formal_inserted: site_step.fragment.clone(),
            formal_dials,
        });
    }

    let faithful = &style.steps;
    // The flaw site in the flawed argument: for replacement material, the step standing in for
    // the faithful site; for insertion material (0.3.0, FLAW-0007 `unused`), the inserted step
    // itself, which stands in for nothing.
    let site = if material.inserts {
        material.site as usize
    } else {
        material
            .steps
            .iter()
            .position(|s| s.from == Some(material.site))
            .ok_or("the material's site is not among its steps")?
            + 1
    };
    let mut original_steps = Map::new();
    let mut original_fragments = Map::new();
    for (j, f) in faithful.iter().enumerate() {
        let k = (j + 1) as u64;
        match material.steps.iter().find(|s| s.from == Some(k)) {
            None => {
                original_steps.insert(k.to_string(), json!(f.text));
                original_fragments.insert(k.to_string(), json!(f.fragment));
            }
            Some(s) => {
                if s.text != f.text {
                    original_steps.insert(k.to_string(), json!(f.text));
                }
                if s.fragment != f.fragment || s.lemma != f.lemma {
                    original_fragments.insert(k.to_string(), json!(f.fragment));
                }
            }
        }
    }
    // FLAW-0004 (0.5.0): the hedge is drawn from the instance's stream after the material and
    // prefixed to the material's bare claim; the deleted justification, the claim and the
    // omitted tactic lines are recorded. An informal-only target has no formal manifestation,
    // so the instance is `N/A` there (D5).
    let hidden = material.hidden.as_ref();
    let hedge = hidden.map(|_| stream.below(HEDGES.len() as u64) as usize);
    // FLAW-0010 (0.6.0): the reference is drawn from the instance's stream after the material —
    // author, year, venue (the shape's list), result — and prefixed as `By <reference>` to the
    // material's bare claim; the justification it stands in for is recorded. The formal file is
    // unchanged, so the instance is `N/A` for every target.
    let cited = material.cited.as_ref();
    let citation = match (cited, &shape) {
        (Some(_), Some(shape)) => Some(draw_citation(&mut stream, shape)?),
        (Some(_), None) => return Err("FLAW-0010 material is commanded with a shape".into()),
        _ => None,
    };
    let flawed_site = &material.steps[site - 1];
    let site_text = match (hidden, hedge, cited, &citation) {
        (Some(hs), Some(i), _, _) => format!("{} {}", HEDGES[i], hs.claim),
        (_, _, Some(cs), Some(c)) => format!("By {} {}", c.reference, cs.claim),
        _ => flawed_site.text.clone(),
    };
    let flawed_step = Step {
        text: site_text.clone(),
        fragment: flawed_site.fragment.clone(),
        lemma: flawed_site.lemma.clone(),
        padding: None,
        premises: flawed_site.premises.clone(),
    };
    let hedged = |formal: bool| -> Vec<(&str, Value)> {
        match (hidden, hedge) {
            (Some(hs), Some(i)) => {
                let mut v = vec![("hedge", json!(i)), ("hedge_text", json!(HEDGES[i]))];
                if formal {
                    v.push(("omitted", json!(hs.omitted)));
                } else {
                    v.push(("justification", json!(hs.justification)));
                    v.push(("claim", json!(hs.claim)));
                }
                v
            }
            _ => Vec::new(),
        }
    };
    let cited_dials = || -> Vec<(&str, Value)> {
        match (cited, &citation) {
            (Some(cs), Some(c)) => vec![
                ("author", json!(c.author)),
                ("year", json!(c.year)),
                ("venue", json!(c.venue)),
                ("result", json!(c.result)),
                ("reference", json!(c.reference)),
                ("asserted", json!(cs.claim)),
                ("justification", json!(cs.justification)),
            ],
            _ => Vec::new(),
        }
    };
    // FLAW-0003 `cycle` (0.6.0): the citation phrases' spans in the flawed steps and the cycle of
    // the plan's premise graph through the inserted lemma, recorded for the D5 check.
    let mut cycle_extra: Vec<(&str, Value)> = Vec::new();
    if material.class == "FLAW-0003" && material.sub_variant.as_deref() == Some("cycle") {
        let mut citations = Vec::new();
        for (j, s) in material.steps.iter().enumerate() {
            for premise in &s.premises {
                let span = unique_span(&s.text, &premise.phrase)
                    .ok_or("a citation phrase occurs exactly once in its step")?;
                citations.push(json!({"step": j + 1, "span": span, "phrase": premise.phrase}));
            }
        }
        let n = spec.dials["cycle_length"]
            .as_u64()
            .ok_or("FLAW-0003 cycle's cycle_length is an integer")? as usize;
        let mut probe = plan.clone();
        probe.steps = material.steps.iter().map(step_of_flawed).collect();
        let lemma = flawed_site
            .lemma
            .as_ref()
            .ok_or("the cycle's site is a lemma")?;
        let cycle = premise_cycle(&probe, &lemma.name, n)
            .ok_or("the plan's premise graph contains the commanded cycle")?;
        cycle_extra = vec![("cycle", json!(cycle)), ("citations", json!(citations))];
    }
    // Informal: the anchor's text, or the whole step; an inserted step replaces nothing; a
    // fictional reference (0.6.0) stands in for the justification it replaced.
    let faithful_site = (!material.inserts).then(|| &faithful[(material.site - 1) as usize]);
    let (inserted, replaced) = match (&material.anchor, faithful_site, cited, &citation) {
        (_, _, Some(cs), Some(c)) => (c.reference.clone(), Some(cs.justification.clone())),
        (Some(a), _, _, _) => (
            a.inserted.clone(),
            Some(
                a.replaced
                    .clone()
                    .ok_or("material that is not intrinsic replaces text at its anchor")?,
            ),
        ),
        (None, Some(f), _, _) => (site_text.clone(), Some(f.text.clone())),
        (None, None, _, _) => (site_text.clone(), None),
    };
    let mut informal_extra = hedged(false);
    informal_extra.extend(cited_dials());
    informal_extra.extend(cycle_extra);
    informal_extra.extend([
        ("inserted", json!(inserted)),
        ("replaced", json!(replaced)),
        ("original_steps", Value::Object(original_steps)),
    ]);
    let informal_dials = with(informal_extra);
    // Formal: a main-proof step's fragment, or a lemma step's declaration lines as the writer
    // emits them (0.3.0), with the declaration's name recorded.
    let (formal_inserted, formal_replaced) = match (&flawed_step.lemma, faithful_site) {
        (Some(_), Some(f)) => (lemma_lines(&flawed_step)?, Some(lemma_lines(f)?)),
        (Some(_), None) => (lemma_lines(&flawed_step)?, None),
        (None, Some(f)) => (flawed_step.fragment.clone(), Some(f.fragment.clone())),
        (None, None) => (flawed_step.fragment.clone(), None),
    };
    let mut formal_extra = hedged(true);
    formal_extra.extend([
        ("inserted", json!(formal_inserted)),
        ("replaced", json!(formal_replaced)),
        ("original_fragments", Value::Object(original_fragments)),
    ]);
    if let Some(l) = &flawed_step.lemma {
        formal_extra.push(("declaration", json!(l.name)));
    }
    let formal_dials = with(formal_extra);
    plan.steps = material
        .steps
        .iter()
        .enumerate()
        .map(|(j, s)| Step {
            text: if j + 1 == site {
                site_text.clone()
            } else {
                s.text.clone()
            },
            fragment: s.fragment.clone(),
            lemma: s.lemma.clone(),
            padding: None,
            premises: s.premises.clone(),
        })
        .collect();
    let lean_catchable = if hidden.is_some() && target == Target::Informal {
        LeanCatchable::NotApplicable
    } else {
        material.lean_catchable
    };
    Ok(Injection {
        instance: 1,
        class: material.class.clone(),
        sub_variant: material.sub_variant.clone(),
        severity: material.severity,
        lean_catchable,
        site,
        informal_inserted: inserted,
        informal_dials,
        formal_inserted,
        formal_dials,
    })
}

/// FLAW-0011 as a commanded composite (ADR-0011 D5; 0.5.0): `embedded` lists the FlawSpecs to
/// apply to a seed whose statement is true — one in 0.5.0, since the materials are
/// whole-argument variants (D5's disjointness rule) — of FLAW-0001 through FLAW-0010 only
/// (FLAW-0011's definition; Al's ruling 3 on PR 30) and of severity gap or fatal, so that the
/// composite is an invalid proof under D7 (Al's ruling 2 on PR 30: a composite whose embedded
/// instances are all cosmetic is refused). The composite's severity and `lean_catchable` are
/// its most severe embedded instance's; its own record addresses the statement and is numbered
/// after the embedded instance.
fn compose(
    plan: &mut Plan,
    seed: &Seed,
    style: &Style,
    spec: &FlawSpec,
    target: Target,
    seed_value: u64,
    version: Version,
) -> Result<(Injection, Composite), String> {
    check_spec(spec, target)?;
    if spec.sub_variant.is_some() {
        return Err("FLAW-0011 has no sub-variant".into());
    }
    if spec.site.is_some() {
        return Err("FLAW-0011 addresses the statement (step 0), so its site is \"auto\"".into());
    }
    exact_keys(&spec.dials, &["embedded"], "FLAW-0011's dials")?;
    let embedded = spec.dials["embedded"]
        .as_array()
        .ok_or("FLAW-0011's embedded is an array of FlawSpecs")?;
    if embedded.is_empty() {
        return Err("FLAW-0011 embeds at least one instance".into());
    }
    if embedded.len() > 1 {
        return Err(format!(
            "s2_template {} materials are whole-argument variants, so a composite embeds one instance (ADR-0011 D5)",
            version.as_str()
        ));
    }
    let inner = parse_spec(&embedded[0], "FLAW-0011's embedded[0]")?;
    let number = inner
        .class
        .strip_prefix("FLAW-")
        .and_then(|n| n.parse::<u32>().ok());
    if !matches!(number, Some(1..=10)) {
        return Err(format!(
            "FLAW-0011 embeds an instance of FLAW-0001 through FLAW-0010 (FLAWS.md's definition; Al's ruling 3 on PR 30), not {}",
            inner.class
        ));
    }
    if !matches!(inner.severity, Severity::Gap | Severity::Fatal) {
        return Err(format!(
            "FLAW-0011 requires an embedded instance of severity gap or fatal; a composite of {:?} instances is not an invalid proof (D7; Al's ruling 2 on PR 30)",
            inner.severity
        ));
    }
    if !version.realizes(&inner.class) {
        return Err(format!(
            "{} is not realized by s2_template {}",
            inner.class,
            version.as_str()
        ));
    }
    if seed.truth.value != Some(true) {
        return Err(format!(
            "seed {} states a false theorem; FLAW-0011 is a true theorem with an invalid proof (a false one is FLAW-0012)",
            seed.id
        ));
    }
    if inner.severity != spec.severity {
        return Err(format!(
            "FLAW-0011's severity inherits its most severe embedded instance: {:?}, not {:?}",
            inner.severity, spec.severity
        ));
    }
    let inj = inject(plan, seed, style, &inner, target, seed_value)?;
    let composite = Composite {
        instance: inj.instance + 1,
        severity: spec.severity,
        lean_catchable: inj.lean_catchable,
        embedded: spec.dials["embedded"].clone(),
        embedded_instances: vec![inj.instance],
    };
    Ok((inj, composite))
}

/// FLAW-0012 as a commanded class (ADR-0011 D5; 0.7.0): on a seed whose statement is false, the
/// material the one dial `dropped_hypothesis` names — intrinsic, the convincing argument as the
/// example writes it — yields two instances: the FLAW-0005 `violated` instance at the hidden use
/// site (instance 1, fatal, `YES-KERNEL`, confirmed by check K there and by the span check in
/// the informal file; its dials the theorem and the hypothesis the seed's `use` names, `mode`
/// `violated`, the site's text, nothing replaced, and the counterexample), and the FLAW-0012
/// instance at the statement (instance 2), whose records `assemble` writes from the returned
/// [`FalseTheorem`]. The material is drawn on the stream `injector/FLAW-0012/1`.
fn drop_hypothesis(
    seed: &Seed,
    style: &Style,
    spec: &FlawSpec,
    target: Target,
    seed_value: u64,
) -> Result<(Injection, FalseTheorem), String> {
    check_spec(spec, target)?;
    if spec.sub_variant.is_some() {
        return Err("FLAW-0012 has no sub-variant".into());
    }
    exact_keys(&spec.dials, &["dropped_hypothesis"], "FLAW-0012's dials")?;
    let dropped = spec.dials["dropped_hypothesis"]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or("FLAW-0012's dropped_hypothesis is a non-empty string")?
        .to_string();
    if seed.truth.value != Some(false) {
        return Err(format!(
            "seed {} states a true theorem; FLAW-0012 is a false theorem with a convincing proof (a true theorem with an invalid proof is FLAW-0011)",
            seed.id
        ));
    }
    let dials = Value::Object(spec.dials.clone());
    let eligible: Vec<&Material> = style
        .flaws
        .iter()
        .filter(|m| {
            m.class == "FLAW-0012"
                && m.hidden_use.is_some()
                && m.severity == spec.severity
                && m.dials == dials
                && spec.site.is_none_or(|k| k == m.site)
        })
        .collect();
    if eligible.is_empty() {
        return Err(format!(
            "style {} has no FLAW-0012 material with dials {dials} at site {:?}",
            style.id, spec.site
        ));
    }
    let mut stream = Stream::new(seed_value, "injector/FLAW-0012/1");
    let material = eligible[stream.below(eligible.len() as u64) as usize];
    let hidden = material
        .hidden_use
        .as_ref()
        .ok_or("FLAW-0012 material carries `use`")?;
    let counterexample = seed
        .counterexample
        .clone()
        .ok_or("a false statement names its counterexample")?;
    // The hidden use site: the FLAW-0005 `violated` instance, as the seed's convincing argument
    // writes it (intrinsic: nothing is replaced).
    let site = material.site as usize;
    let site_step = style
        .steps
        .get(site.wrapping_sub(1))
        .ok_or("the hidden use site is a step")?;
    let inserted = match &material.anchor {
        Some(a) => a.inserted.clone(),
        None => site_step.text.clone(),
    };
    let use_dials = |inserted: Value| {
        json!({
            "theorem": hidden.theorem,
            "hypothesis": hidden.hypothesis,
            "mode": "violated",
            "site": site,
            "inserted": inserted,
            "replaced": null,
            "counterexample": counterexample,
        })
    };
    let injection = Injection {
        instance: 1,
        class: "FLAW-0005".into(),
        sub_variant: None,
        severity: Severity::Fatal,
        lean_catchable: LeanCatchable::YesKernel,
        site,
        informal_inserted: inserted.clone(),
        informal_dials: use_dials(json!(inserted)),
        formal_inserted: site_step.fragment.clone(),
        formal_dials: use_dials(json!(site_step.fragment)),
    };
    let false_theorem = FalseTheorem {
        instance: 2,
        dropped_hypothesis: dropped,
        use_site: site,
        use_instance: 1,
        counterexample,
    };
    Ok((injection, false_theorem))
}

/// FLAWS.md 0.1.0's *Manifests in* column, as ADR-0011 D5 realizes it: whether a class's record
/// appears in the informal file and in the formal file. FLAW-0009 and FLAW-0008 leave the
/// informal file unchanged; FLAW-0010 and FLAW-0003 `cycle` leave the formal file unchanged.
pub fn manifests_in(class: &str, sub_variant: Option<&str>) -> (bool, bool) {
    match (class, sub_variant) {
        ("FLAW-0003", Some("cycle")) | ("FLAW-0010", _) => (true, false),
        ("FLAW-0008", _) | ("FLAW-0009", _) => (false, true),
        _ => (true, true),
    }
}

/// The unique code-point span of `needle` in `hay`, or `None` unless it occurs exactly once.
fn unique_span(hay: &str, needle: &str) -> Option<[u64; 2]> {
    let mut found = hay.match_indices(needle);
    let (at, _) = found.next()?;
    if found.next().is_some() {
        return None;
    }
    let s = hay[..at].chars().count() as u64;
    Some([s, s + needle.chars().count() as u64])
}

fn exact<T: serde::Serialize>(v: T) -> Commanded {
    Commanded {
        value: json!(v),
        tolerance: Tolerance::Exact { exact: true },
    }
}

fn measured(value: Value, estimator: &str) -> Achieved {
    Achieved {
        value: Some(value),
        estimator: estimator.into(),
        ci95: None,
        reason: None,
    }
}

/// Check T for an unused declaration (ADR-0011 D5, D8; FLAW-0007 `unused` from 0.3.0, FLAW-0013
/// from 0.4.0): the build passed, the audit tail's listing names the declaration, and it lies
/// outside the transitive used-constants of the main declaration, closed over the file's own
/// constants (Lean's auxiliary `_proof_N` constants included). No listing confirms nothing.
fn unused_in_file(obs: &Observation, declaration: &str, main: &str) -> bool {
    obs.exit_code == 0
        && obs.used_constants.as_ref().is_some_and(|used| {
            used.contains_key(declaration) && !used_closure(used, main).contains(declaration)
        })
}

/// The flaw counts (ADR-0011 D8 as amended by ruling 3 on PR 26): per instance from 0.2.0, per
/// record under 0.1.0. Returns `(total, confirmed, per_class → (total, confirmed))`; an
/// instance is confirmed only when every one of its records is (Al's owner note of 2026-10-28).
fn counts(version: Version, records: &[(Flaw, bool)]) -> (u64, u64, BTreeMap<String, (u64, u64)>) {
    let mut per_class: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    let mut total = 0;
    let mut confirmed = 0;
    if version.counts_instances() {
        let mut instances: BTreeMap<u64, (String, bool)> = BTreeMap::new();
        for (r, ok) in records {
            let e = instances
                .entry(r.instance)
                .or_insert_with(|| (r.class.clone(), true));
            e.1 &= *ok;
        }
        for (class, ok) in instances.values() {
            let e = per_class.entry(class[5..].to_string()).or_default();
            e.0 += 1;
            e.1 += u64::from(*ok);
            total += 1;
            confirmed += u64::from(*ok);
        }
    } else {
        for (r, ok) in records {
            let e = per_class.entry(r.class[5..].to_string()).or_default();
            e.0 += 1;
            e.1 += u64::from(*ok);
            total += 1;
            confirmed += u64::from(*ok);
        }
    }
    (total, confirmed, per_class)
}

/// `measure` → `manifest` (ADR-0011 D7–D9). `observation` is the harness's view of the formal
/// file and must be present iff the target has one.
pub fn assemble(
    p: &Prepared,
    engine_commit: &str,
    toolchain: Option<Toolchain>,
    observation: Option<&Observation>,
) -> Result<Manifest, String> {
    if p.formal.is_some() != observation.is_some() || p.formal.is_some() != toolchain.is_some() {
        return Err(
            "a formal file needs its harness observation and toolchain, and only it".into(),
        );
    }
    let informal_stats = p
        .informal
        .as_deref()
        .map(parse_informal_md)
        .transpose()
        .map_err(|e| format!("rendered informal bytes: {e}"))?;
    let formal_stats = p
        .formal
        .as_ref()
        .map(|f| parse_lean(&f.bytes))
        .transpose()
        .map_err(|e| format!("rendered formal bytes: {e}"))?;
    let main = p.plan.main_declaration.clone();

    // Records, each with its confirmation check (S2.2).
    let mut records: Vec<(Flaw, bool)> = Vec::new();
    if let Some(inj) = &p.injection {
        let (in_informal, in_formal) = manifests_in(&inj.class, inj.sub_variant.as_deref());
        if let (Some(st), true) = (&informal_stats, in_informal) {
            let text = &st.steps[inj.site];
            let span = unique_span(text, &inj.informal_inserted).ok_or(
                "the inserted text does not occur exactly once at the informal site".to_string(),
            )?;
            // The span holds exactly the recorded text; where something was replaced, the text
            // differs from it (an intrinsic record replaces nothing).
            let mut confirmed = span_text(text, span).as_deref() == Some(&inj.informal_inserted)
                && (inj.informal_dials["replaced"].is_null()
                    || inj.informal_dials["replaced"] != json!(inj.informal_inserted));
            // FLAW-0004 (0.5.0, D5): the span equals the hedge entry plus the step text minus
            // its recorded justification — the step is the drawn hedge, a space and the bare
            // claim; the faithful text held the justification exactly once; the step does not.
            if inj.class == "FLAW-0004" {
                let d = &inj.informal_dials;
                let hedge = d["hedge_text"].as_str().unwrap_or_default();
                let claim = d["claim"].as_str().unwrap_or_default();
                let justification = d["justification"].as_str().unwrap_or_default();
                confirmed = confirmed
                    && !justification.is_empty()
                    && *text == format!("{hedge} {claim}")
                    && d["replaced"]
                        .as_str()
                        .is_some_and(|r| r.matches(justification).count() == 1)
                    && !text.contains(justification);
            }
            // FLAW-0010 (0.6.0, D5): the span equals the reference the recorded lexicon indices
            // name, every token of it is in the lexicon, the step is `By <reference>` and the
            // bare claim, and the faithful step held the justification exactly once.
            if inj.class == "FLAW-0010" {
                let d = &inj.informal_dials;
                let index = |k: &str| d[k].as_u64().map(|x| x as usize);
                let rebuilt = match (
                    d["shape"].as_str(),
                    index("author"),
                    index("year"),
                    index("venue"),
                    index("result"),
                ) {
                    (Some(shape), Some(a), Some(y), Some(v), Some(r)) => {
                        citation_reference(shape, a, y, v, r)
                    }
                    _ => None,
                };
                let asserted = d["asserted"].as_str().unwrap_or_default();
                let justification = d["justification"].as_str().unwrap_or_default();
                let original = d["original_steps"][inj.site.to_string()]
                    .as_str()
                    .unwrap_or_default();
                confirmed = confirmed
                    && rebuilt.as_deref() == Some(inj.informal_inserted.as_str())
                    && citation_tokens_in_lexicon(&inj.informal_inserted)
                    && *text == format!("By {} {asserted}", inj.informal_inserted)
                    && !justification.is_empty()
                    && original.matches(justification).count() == 1
                    && !text.contains(justification);
            }
            // FLAW-0003 `cycle` (0.6.0, D5): the plan's premise graph contains the recorded
            // cycle through the inserted lemma, and each citation phrase is at its recorded span.
            if inj.class == "FLAW-0003" && inj.sub_variant.as_deref() == Some("cycle") {
                let d = &inj.informal_dials;
                let n = d["cycle_length"].as_u64().unwrap_or(0) as usize;
                let lemma = p
                    .plan
                    .steps
                    .get(inj.site - 1)
                    .and_then(|s| s.lemma.as_ref())
                    .map(|l| l.name.clone());
                let cycle_ok = lemma
                    .and_then(|l| premise_cycle(&p.plan, &l, n))
                    .is_some_and(|c| json!(c) == d["cycle"]);
                let citations_ok = d["citations"].as_array().is_some_and(|cs| {
                    !cs.is_empty()
                        && cs.iter().all(|c| {
                            let step = c["step"].as_u64().map(|k| k as usize);
                            let span = c["span"]
                                .as_array()
                                .and_then(|s| Some([s.first()?.as_u64()?, s.get(1)?.as_u64()?]));
                            match (step, span, c["phrase"].as_str()) {
                                (Some(k), Some(span), Some(phrase)) => {
                                    st.steps.get(k).is_some_and(|t| {
                                        span_text(t, span).as_deref() == Some(phrase)
                                            && t.matches(phrase).count() == 1
                                    })
                                }
                                _ => false,
                            }
                        })
                });
                confirmed = confirmed && cycle_ok && citations_ok;
            }
            records.push((
                Flaw {
                    instance: inj.instance,
                    class: inj.class.clone(),
                    sub_variant: inj.sub_variant.clone(),
                    location: Location::Informal(InformalLocation {
                        artifact: InformalTag::Informal,
                        step: inj.site as u64,
                        span,
                    }),
                    severity: inj.severity,
                    lean_catchable: inj.lean_catchable,
                    dials: inj.informal_dials.clone(),
                },
                confirmed,
            ));
        }
        if let (Some(f), Some(obs), true) = (&p.formal, observation, in_formal)
            && inj.site == 0
        {
            // FLAW-0008 (0.7.0, D5): the record addresses the statement — the main declaration
            // at its header line. Confirmed when the drifted file builds (its own correct
            // proof), the faithful statement has its kernel-checked witness (D7: the seed's
            // reference, a library module the gate builds and audits), the two recorded
            // statements differ by exactly the transformation's edit, the plan states the
            // drifted one, and the header line is the recorded one.
            let text = std::str::from_utf8(&f.bytes).map_err(|e| e.to_string())?;
            let header = text
                .lines()
                .nth(f.main_line as usize - 1)
                .ok_or("the main declaration's header line is in the formal file")?;
            let d = &inj.formal_dials;
            let statement = |v: &Value| -> Option<(String, String)> {
                Some((
                    v["binders"].as_str()?.to_string(),
                    v["claim"].as_str()?.to_string(),
                ))
            };
            let drift_ok = match (
                d["transformation"].as_str(),
                statement(&d["faithful_statement"]),
                statement(&d["drifted_statement"]),
                d["edit"]["replaced"].as_str(),
                d["edit"]["inserted"].as_str(),
            ) {
                (Some(t), Some((fb, fc)), Some((db, dc)), Some(r), Some(i)) => {
                    check_drift(t, (&fb, &fc), (&db, &dc), r, i).is_ok()
                        && (p.plan.binders.as_str(), p.plan.claim.as_str())
                            == (db.as_str(), dc.as_str())
                }
                _ => false,
            };
            let truth = &p.plan.truth;
            let faithful_builds = truth.value == Some(true)
                && matches!(
                    truth.method,
                    StatementMethod::LeanKernel | StatementMethod::Mathlib
                )
                && !truth.reference.is_empty();
            let confirmed = obs.exit_code == 0
                && faithful_builds
                && drift_ok
                && d["inserted"][0].as_str() == Some(header);
            records.push((
                Flaw {
                    instance: inj.instance,
                    class: inj.class.clone(),
                    sub_variant: inj.sub_variant.clone(),
                    location: Location::Formal(FormalLocation {
                        artifact: FormalTag::Formal,
                        declaration: main.clone(),
                        line: f.main_line,
                    }),
                    severity: inj.severity,
                    lean_catchable: inj.lean_catchable,
                    dials: inj.formal_dials.clone(),
                },
                confirmed,
            ));
        } else if let (Some(f), Some(obs), true) = (&p.formal, observation, in_formal) {
            let [lo, hi] =
                f.fragment_lines[inj.site - 1].ok_or("the flaw site has no formal fragment")?;
            let declaration = f.declarations[inj.site - 1].clone();
            // K: the build fails and the first error lies within the site's fragment (D5).
            // T (0.3.0): the build passes and the audit tail's listing satisfies the class's
            // condition — FLAW-0007 `unused`: the lemma's name is listed and lies outside the
            // transitive used-constants of the main declaration, closed over the file's own
            // constants; FLAW-0009: `#print axioms` lists the axiom's name. No other check is
            // implemented, so any other class is never confirmed here.
            let confirmed = match inj.lean_catchable {
                LeanCatchable::YesKernel => {
                    obs.exit_code != 0 && obs.first_error_line.is_some_and(|l| lo <= l && l <= hi)
                }
                LeanCatchable::YesTooling if obs.exit_code == 0 => {
                    match (inj.class.as_str(), inj.sub_variant.as_deref()) {
                        ("FLAW-0007", Some("unused")) => unused_in_file(obs, &declaration, &main),
                        ("FLAW-0009", _) => obs
                            .axioms
                            .as_ref()
                            .is_some_and(|a| a.contains(&declaration)),
                        _ => false,
                    }
                }
                _ => false,
            };
            records.push((
                Flaw {
                    instance: inj.instance,
                    class: inj.class.clone(),
                    sub_variant: inj.sub_variant.clone(),
                    location: Location::Formal(FormalLocation {
                        artifact: FormalTag::Formal,
                        declaration,
                        line: lo,
                    }),
                    severity: inj.severity,
                    lean_catchable: inj.lean_catchable,
                    dials: inj.formal_dials.clone(),
                },
                confirmed,
            ));
        }
    }
    // Padding records (0.4.0, FLAW-0013): one instance per inserted paragraph, in both files
    // (FLAWS.md: *Manifests in* both). The informal record spans the whole step, `[0, len)`,
    // and is confirmed when the step holds exactly the paragraph's text; the formal record is
    // the matching declaration at its first line, `YES-TOOLING`, confirmed by check T.
    let mut padded_tokens: u64 = 0;
    for ins in &p.padding {
        if let Some(st) = &informal_stats {
            let text = &st.steps[ins.site];
            let span = [0, text.chars().count() as u64];
            let confirmed = *text == ins.text;
            padded_tokens += count_tokens(text);
            records.push((
                Flaw {
                    instance: ins.instance,
                    class: "FLAW-0013".into(),
                    sub_variant: None,
                    location: Location::Informal(InformalLocation {
                        artifact: InformalTag::Informal,
                        step: ins.site as u64,
                        span,
                    }),
                    severity: Severity::Cosmetic,
                    lean_catchable: LeanCatchable::No,
                    dials: ins.informal_dials.clone(),
                },
                confirmed,
            ));
        }
        if let (Some(f), Some(obs)) = (&p.formal, observation) {
            let [lo, hi] = f.fragment_lines[ins.site - 1]
                .ok_or("a padding step has its declaration lines in the formal file")?;
            let declaration = f.declarations[ins.site - 1].clone();
            if informal_stats.is_none() {
                // a formal-only artifact: the padding's tokens are its declarations' lines
                let text = std::str::from_utf8(&f.bytes).map_err(|e| e.to_string())?;
                padded_tokens += text
                    .lines()
                    .skip(lo as usize - 1)
                    .take((hi - lo + 1) as usize)
                    .map(count_tokens)
                    .sum::<u64>();
            }
            let confirmed = unused_in_file(obs, &declaration, &main);
            records.push((
                Flaw {
                    instance: ins.instance,
                    class: "FLAW-0013".into(),
                    sub_variant: None,
                    location: Location::Formal(FormalLocation {
                        artifact: FormalTag::Formal,
                        declaration,
                        line: lo,
                    }),
                    severity: Severity::Cosmetic,
                    lean_catchable: LeanCatchable::YesTooling,
                    dials: ins.formal_dials.clone(),
                },
                confirmed,
            ));
        }
    }
    // FLAW-0011 as a commanded composite (0.5.0, D5): one record per file the artifact has, at
    // the statement — step 0 with span `[0, len)`, the main declaration at its header line —
    // confirmed when the statement's truth is by a kernel-checked witness or Mathlib (D7; the
    // witness is a library module the gate builds) and every embedded instance's records pass.
    if let Some(comp) = &p.composite {
        let embedded_records: Vec<&(Flaw, bool)> = records
            .iter()
            .filter(|(r, _)| comp.embedded_instances.contains(&r.instance))
            .collect();
        let embedded_ok =
            !embedded_records.is_empty() && embedded_records.iter().all(|(_, ok)| *ok);
        let truth = &p.plan.truth;
        let truth_ok = truth.value == Some(true)
            && matches!(
                truth.method,
                StatementMethod::LeanKernel | StatementMethod::Mathlib
            )
            && !truth.reference.is_empty();
        let confirmed = embedded_ok && truth_ok;
        let with = |extra: Vec<(&str, Value)>| {
            let mut m = Map::new();
            m.insert("embedded".into(), comp.embedded.clone());
            m.insert("site".into(), json!(0));
            m.insert("embedded_instances".into(), json!(comp.embedded_instances));
            for (key, v) in extra {
                m.insert(key.into(), v);
            }
            Value::Object(m)
        };
        if let Some(st) = &informal_stats {
            let text = &st.steps[0];
            records.push((
                Flaw {
                    instance: comp.instance,
                    class: "FLAW-0011".into(),
                    sub_variant: None,
                    location: Location::Informal(InformalLocation {
                        artifact: InformalTag::Informal,
                        step: 0,
                        span: [0, text.chars().count() as u64],
                    }),
                    severity: comp.severity,
                    lean_catchable: comp.lean_catchable,
                    dials: with(vec![("inserted", json!(text)), ("replaced", Value::Null)]),
                },
                confirmed,
            ));
        }
        if let (Some(f), Some(_)) = (&p.formal, observation) {
            let text = std::str::from_utf8(&f.bytes).map_err(|e| e.to_string())?;
            let header = text
                .lines()
                .nth(f.main_line as usize - 1)
                .ok_or("the main declaration's header line is in the formal file")?;
            records.push((
                Flaw {
                    instance: comp.instance,
                    class: "FLAW-0011".into(),
                    sub_variant: None,
                    location: Location::Formal(FormalLocation {
                        artifact: FormalTag::Formal,
                        declaration: main.clone(),
                        line: f.main_line,
                    }),
                    severity: comp.severity,
                    lean_catchable: comp.lean_catchable,
                    dials: with(vec![
                        ("inserted", json!([header])),
                        ("replaced", Value::Null),
                    ]),
                },
                confirmed,
            ));
        }
    }
    // FLAW-0012 as a commanded class (0.7.0, D5): one record per file the artifact has, at the
    // statement — step 0 with span `[0, len)`, the main declaration at its header line —
    // confirmed when every record of the use-site instance passes (check K at the hidden use
    // site; the span check in the informal file) and the seed's negation witness is
    // kernel-checked (D7; a library module the gate builds and audits).
    if let Some(ft) = &p.false_theorem {
        let use_records: Vec<&(Flaw, bool)> = records
            .iter()
            .filter(|(r, _)| r.instance == ft.use_instance)
            .collect();
        let use_ok = !use_records.is_empty() && use_records.iter().all(|(_, ok)| *ok);
        let truth = &p.plan.truth;
        let negation_ok = truth.value == Some(false)
            && truth.method == StatementMethod::LeanKernel
            && !truth.reference.is_empty();
        let confirmed = use_ok && negation_ok;
        let with = |inserted: Value| {
            json!({
                "dropped_hypothesis": ft.dropped_hypothesis,
                "site": 0,
                "use_site": ft.use_site,
                "use_instance": ft.use_instance,
                "counterexample": ft.counterexample,
                "inserted": inserted,
                "replaced": null,
            })
        };
        if let Some(st) = &informal_stats {
            let text = &st.steps[0];
            records.push((
                Flaw {
                    instance: ft.instance,
                    class: "FLAW-0012".into(),
                    sub_variant: None,
                    location: Location::Informal(InformalLocation {
                        artifact: InformalTag::Informal,
                        step: 0,
                        span: [0, text.chars().count() as u64],
                    }),
                    severity: Severity::Fatal,
                    lean_catchable: LeanCatchable::YesKernel,
                    dials: with(json!(text)),
                },
                confirmed,
            ));
        }
        if let (Some(f), Some(_)) = (&p.formal, observation) {
            let text = std::str::from_utf8(&f.bytes).map_err(|e| e.to_string())?;
            let header = text
                .lines()
                .nth(f.main_line as usize - 1)
                .ok_or("the main declaration's header line is in the formal file")?;
            records.push((
                Flaw {
                    instance: ft.instance,
                    class: "FLAW-0012".into(),
                    sub_variant: None,
                    location: Location::Formal(FormalLocation {
                        artifact: FormalTag::Formal,
                        declaration: main.clone(),
                        line: f.main_line,
                    }),
                    severity: Severity::Fatal,
                    lean_catchable: LeanCatchable::YesKernel,
                    dials: with(json!([header])),
                },
                confirmed,
            ));
        }
    }
    let (n_total, n_confirmed, per_class) = counts(p.version, &records);

    // Commanded (ADR-0011 D8).
    let mut commanded = BTreeMap::new();
    if p.informal.is_some() {
        let steps = p.plan.steps.len() as u64;
        let lemmas = p
            .plan
            .steps
            .iter()
            .filter(|s| is_lemma_step(&s.text))
            .count() as u64;
        commanded.insert("n_steps".to_string(), exact(steps));
        commanded.insert("n_lemmas".to_string(), exact(lemmas));
    }
    commanded.insert("n_flaws_total".to_string(), exact(n_total));
    for (nnnn, (n, _)) in &per_class {
        commanded.insert(format!("n_flaws_{nnnn}"), exact(*n));
    }
    commanded.insert("flaws_confirmed".to_string(), exact(n_total));
    let ratio_tolerance = if p.config["padding_ratio"].as_f64() == Some(0.0) {
        0.0
    } else {
        0.05
    };
    commanded.insert(
        "padding_ratio".to_string(),
        Commanded {
            value: json!(p.config["padding_ratio"].as_f64().unwrap_or(0.0)),
            tolerance: Tolerance::Abs {
                abs: ratio_tolerance,
            },
        },
    );
    if p.formal.is_some() {
        let kernel = records.iter().any(|(r, _)| {
            matches!(r.location, Location::Formal(_))
                && r.lean_catchable == LeanCatchable::YesKernel
        });
        commanded.insert(
            "lean_build".to_string(),
            exact(if kernel { "fail" } else { "pass" }),
        );
        if !kernel {
            let smuggled = records.iter().any(|(r, _)| r.class == "FLAW-0009");
            commanded.insert("lean_axioms_standard".to_string(), exact(!smuggled));
        }
    }

    // Achieved (ADR-0011 D8): parsers on the rendered bytes, the records, the harness.
    let mut achieved = BTreeMap::new();
    let (n_tokens_value, n_tokens) = match (&informal_stats, &formal_stats) {
        (Some(st), _) => (
            st.n_tokens,
            measured(json!(st.n_tokens), "informal_md_1_token_count"),
        ),
        (None, Some(st)) => (
            st.n_tokens,
            measured(json!(st.n_tokens), "lean_1_token_count"),
        ),
        (None, None) => return Err("an artifact has at least one file".into()),
    };
    achieved.insert("n_tokens".to_string(), n_tokens);
    if let Some(st) = &informal_stats {
        achieved.insert(
            "n_steps".to_string(),
            measured(json!(st.n_steps), "informal_md_1_step_count"),
        );
        achieved.insert(
            "n_lemmas".to_string(),
            measured(json!(st.n_lemmas), "informal_md_1_lemma_count"),
        );
    }
    achieved.insert(
        "n_flaws_total".to_string(),
        measured(json!(n_confirmed), "injected_instances_confirmed"),
    );
    for (nnnn, (_, ok)) in &per_class {
        achieved.insert(
            format!("n_flaws_{nnnn}"),
            measured(json!(ok), "injected_instances_confirmed"),
        );
    }
    achieved.insert(
        "flaws_confirmed".to_string(),
        measured(json!(n_confirmed), "injected_instances_confirmed"),
    );
    // `padding_ratio` (ADR-0001 D4): the tokens inside the FLAW-0013 spans — the padded steps of
    // the informal file, or the padding declarations of a formal-only file — over `n_tokens`, in
    // binary64 (D6); 0.0 when nothing was padded.
    let padding_ratio = if padded_tokens == 0 {
        0.0
    } else {
        padded_tokens as f64 / n_tokens_value as f64
    };
    achieved.insert(
        "padding_ratio".to_string(),
        measured(json!(padding_ratio), "flaw_0013_span_tokens_over_n_tokens"),
    );
    if let Some(obs) = observation {
        achieved.insert(
            "lean_build".to_string(),
            measured(
                json!(if obs.exit_code == 0 { "pass" } else { "fail" }),
                "lake_env_lean_exit",
            ),
        );
        achieved.insert(
            "lean_axioms_standard".to_string(),
            match obs.axioms_standard() {
                Some(b) => measured(json!(b), "print_axioms_subset_standard"),
                None => Achieved {
                    value: None,
                    estimator: "print_axioms_subset_standard".into(),
                    ci95: None,
                    reason: Some("the formal file did not build, so no axiom print exists".into()),
                },
            },
        );
    }

    // Ground truth by construction (ADR-0011 D7).
    let proof_status = if records
        .iter()
        .all(|(r, _)| r.severity == Severity::Cosmetic)
    {
        ProofStatus::Valid
    } else {
        ProofStatus::Invalid
    };
    let truth = p.plan.truth.clone();
    let correct = truth.value == Some(true) && proof_status == ProofStatus::Valid;

    let files = Files {
        informal: p
            .informal
            .as_ref()
            .zip(informal_stats.as_ref())
            .map(|(b, st)| InformalFile {
                file: FILE_INFORMAL.into(),
                format: FORMAT_INFORMAL_MD.into(),
                storage: Storage::Regenerable,
                content_hash: content_hash(b),
                n_bytes: st.n_bytes,
                n_tokens: Some(st.n_tokens),
                n_steps: Some(st.n_steps),
                n_lemmas: Some(st.n_lemmas),
            }),
        formal: p
            .formal
            .as_ref()
            .zip(formal_stats.as_ref())
            .map(|(f, st)| FormalFile {
                file: FILE_FORMAL.into(),
                format: FORMAT_LEAN.into(),
                storage: Storage::Regenerable,
                content_hash: content_hash(&f.bytes),
                n_bytes: st.n_bytes,
                n_tokens: Some(st.n_tokens),
                n_lines: Some(st.n_lines),
                n_declarations: Some(st.n_declarations),
                main_declaration: Some(main.clone()),
            }),
    };
    let fid = fidelity(&commanded, &achieved);
    let hash = p.config_hash.clone();
    let manifest = Manifest {
        manifest_schema: MANIFEST_SCHEMA.into(),
        artifact_id: p.artifact_id(),
        origin: Origin::Synthetic,
        generator: Generator {
            id: GENERATOR_ID.into(),
            version: p.version.as_str().into(),
            rng: RNG_ID.into(),
            llm: None,
        },
        engine_commit: engine_commit.into(),
        seed: Some(p.seed),
        config: p.config.clone(),
        config_hash: hash,
        toolchain,
        artifact: Artifact {
            kind: p.target.kind(),
            files,
        },
        ground_truth: Some(GroundTruth {
            derivation: Derivation::ByConstruction,
            statement_status: truth,
            proof_status,
            correct,
            flaws: records.into_iter().map(|(r, _)| r).collect(),
            taxonomy_version: TAXONOMY_VERSION.into(),
        }),
        commanded,
        achieved,
        fidelity: fid,
        notes: None,
    };
    manifest
        .check(FileBytes {
            informal: p.informal.as_deref(),
            formal: p.formal.as_ref().map(|f| f.bytes.as_slice()),
        })
        .map_err(|problems| format!("the manifest fails its self-check: {problems:?}"))?;
    Ok(manifest)
}
