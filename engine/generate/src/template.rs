//! `s2_template` (ADR-0011 D1, D6): hand-authored seed theorems with aligned informal and formal
//! steps. `prepare` runs `plan` → `inject` → `pad` → `render`; `assemble` runs `measure` →
//! `manifest` once the Lean harness has observed the formal file (D2's pipeline). A config
//! commands either one injected instance or padding, never both (D5: two whole-argument
//! variants cannot be made disjoint, and a padded flawed argument is a later version).
//!
//! The crate carries four producer versions (ADR-0011 *Amendments*, Al's rulings 3 and 5 on PR
//! 26): `0.1.0`, whose three seeds and per-record counts stay regenerable as the goldens name them;
//! `0.2.0`, with four more seeds and counts per instance; `0.3.0`, with lemma steps (D2), the T
//! check (D5, D8: FLAW-0007 `unused` and FLAW-0009) and the `n_lt_two_pow` seed; and `0.4.0`,
//! the version `generate` emits, with the padding module (D2 `pad`, D5 FLAW-0013, Options 8) and
//! the `sqrt2_irrational` seed, which carries padding material.
//!
//! A seed's flaw material is a whole-argument variant of one style: the flawed step list, each
//! step naming the faithful step it replaces (`from`), the flaw site, and the anchor text at the
//! site. The injector selects the material the spec names, applies it to the plan, and writes the
//! record's dials with every original text it replaced or removed (D5). A seed whose statement is
//! false has no faithful argument and no control; its material is *intrinsic*: the style's steps
//! are the convincing argument as the FLAWS.md example writes it, nothing is replaced, and the
//! record carries the seed's counterexample (0.2.0; a reading flagged for Al in the amendment).
//! A seed's padding material (0.4.0) is a list of paragraphs, each true irrelevant material as a
//! whole step with the unused declaration that matches it in the formal file.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value, json};
use syndeom_manifest::{
    Achieved, Artifact, Commanded, Derivation, FILE_FORMAL, FILE_INFORMAL, FORMAT_INFORMAL_MD,
    FORMAT_LEAN, FileBytes, Files, Flaw, FormalFile, FormalLocation, FormalTag, Generator,
    GroundTruth, InformalFile, InformalLocation, InformalTag, Kind, LeanCatchable, Location,
    MANIFEST_SCHEMA, Manifest, Origin, ProofStatus, Severity, StatementStatus, Storage, Tolerance,
    Toolchain, artifact_id, config_hash, content_hash, count_tokens, fidelity, is_lemma_step,
    parse_informal_md, parse_lean, span_text,
};

use crate::harness::{Observation, used_closure};
use crate::plan::{
    FormalRendering, LemmaHeader, PaddingHeader, PaddingKind, Plan, Step, lemma_lines,
    padding_lines, render_formal, render_informal,
};
use crate::rng::{RNG_ID, Stream};

pub const GENERATOR_ID: &str = "s2_template";
/// The producer version `generate` emits (ADR-0011 D1; ruling 5 on PR 26).
pub const GENERATOR_VERSION: &str = "0.4.0";
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
}

impl Version {
    pub const CURRENT: Version = Version::V0_4_0;

    pub fn parse(s: &str) -> Option<Version> {
        match s {
            "0.1.0" => Some(Version::V0_1_0),
            "0.2.0" => Some(Version::V0_2_0),
            "0.3.0" => Some(Version::V0_3_0),
            "0.4.0" => Some(Version::V0_4_0),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Version::V0_1_0 => "0.1.0",
            Version::V0_2_0 => "0.2.0",
            Version::V0_3_0 => "0.3.0",
            Version::V0_4_0 => "0.4.0",
        }
    }

    /// How many of [`SEED_RECORDS`] the version compiles in: the v0.1 set, then the v0.2
    /// additions, then the v0.3 addition, then the v0.4 addition.
    fn seed_count(self) -> usize {
        match self {
            Version::V0_1_0 => 3,
            Version::V0_2_0 => 7,
            Version::V0_3_0 => 8,
            Version::V0_4_0 => 9,
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
];

/// Dial keys the injector or the padding module writes into a record; a spec may not use them.
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
    pub steps: Vec<FlawedStep>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Anchor {
    /// The faithful text at the site; `null` only in intrinsic material.
    #[serde(deserialize_with = "required_option")]
    pub replaced: Option<String>,
    pub inserted: String,
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
                if m.site < 1 || m.site as usize > style.steps.len() {
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
        let f = f
            .as_object()
            .ok_or(format!("config.flaws[{i}] is an object"))?;
        exact_keys(
            f,
            &["class", "sub_variant", "severity", "site", "dials"],
            &format!("config.flaws[{i}]"),
        )?;
        let site = match &f["site"] {
            Value::String(s) if s == "auto" => None,
            v => Some(v.as_u64().filter(|k| *k >= 1).ok_or(format!(
                "config.flaws[{i}].site is \"auto\" or a step index"
            ))?),
        };
        let dials = f["dials"]
            .as_object()
            .ok_or(format!("config.flaws[{i}].dials is an object"))?
            .clone();
        if let Some(k) = RESERVED_DIALS.iter().find(|k| dials.contains_key(**k)) {
            return Err(format!(
                "config.flaws[{i}].dials may not set {k:?}: the injector records it"
            ));
        }
        flaws.push(FlawSpec {
            class: f["class"]
                .as_str()
                .ok_or(format!("config.flaws[{i}].class is a string"))?
                .to_string(),
            sub_variant: match &f["sub_variant"] {
                Value::Null => None,
                Value::String(s) => Some(s.clone()),
                _ => return Err(format!("config.flaws[{i}].sub_variant is a string or null")),
            },
            severity: parse_severity(&f["severity"])?,
            site,
            dials,
        });
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
        _ => false,
    };
    if refused {
        return Err(format!(
            "{} cannot be realized with target {target:?}",
            spec.class
        ));
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

/// Everything `prepare` produces: the config as given, the plan after injection and padding,
/// the rendered bytes, the injected instance (none for a control or a padded control) and the
/// padding insertions (none unless FLAW-0013 was commanded).
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
    let mut plan = faithful_plan(&seed, style);
    let (injection, padding) = match cfg.flaws.first() {
        None => (None, Vec::new()),
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
        ),
        Some(spec) => (
            Some(inject(
                &mut plan, &seed, style, spec, cfg.target, seed_value,
            )?),
            Vec::new(),
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
    let dials = Value::Object(spec.dials.clone());
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
    let flawed_site = &material.steps[site - 1];
    let flawed_step = Step {
        text: flawed_site.text.clone(),
        fragment: flawed_site.fragment.clone(),
        lemma: flawed_site.lemma.clone(),
        padding: None,
    };
    // Informal: the anchor's text, or the whole step; an inserted step replaces nothing.
    let faithful_site = (!material.inserts).then(|| &faithful[(material.site - 1) as usize]);
    let (inserted, replaced) = match (&material.anchor, faithful_site) {
        (Some(a), _) => (
            a.inserted.clone(),
            Some(
                a.replaced
                    .clone()
                    .ok_or("material that is not intrinsic replaces text at its anchor")?,
            ),
        ),
        (None, Some(f)) => (flawed_site.text.clone(), Some(f.text.clone())),
        (None, None) => (flawed_site.text.clone(), None),
    };
    let informal_dials = with(vec![
        ("inserted", json!(inserted)),
        ("replaced", json!(replaced)),
        ("original_steps", Value::Object(original_steps)),
    ]);
    // Formal: a main-proof step's fragment, or a lemma step's declaration lines as the writer
    // emits them (0.3.0), with the declaration's name recorded.
    let (formal_inserted, formal_replaced) = match (&flawed_step.lemma, faithful_site) {
        (Some(_), Some(f)) => (lemma_lines(&flawed_step)?, Some(lemma_lines(f)?)),
        (Some(_), None) => (lemma_lines(&flawed_step)?, None),
        (None, Some(f)) => (flawed_step.fragment.clone(), Some(f.fragment.clone())),
        (None, None) => (flawed_step.fragment.clone(), None),
    };
    let mut formal_extra = vec![
        ("inserted", json!(formal_inserted)),
        ("replaced", json!(formal_replaced)),
        ("original_fragments", Value::Object(original_fragments)),
    ];
    if let Some(l) = &flawed_step.lemma {
        formal_extra.push(("declaration", json!(l.name)));
    }
    let formal_dials = with(formal_extra);
    plan.steps = material
        .steps
        .iter()
        .map(|s| Step {
            text: s.text.clone(),
            fragment: s.fragment.clone(),
            lemma: s.lemma.clone(),
            padding: None,
        })
        .collect();
    Ok(Injection {
        instance: 1,
        class: material.class.clone(),
        sub_variant: material.sub_variant.clone(),
        severity: material.severity,
        lean_catchable: material.lean_catchable,
        site,
        informal_inserted: inserted,
        informal_dials,
        formal_inserted,
        formal_dials,
    })
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
            let confirmed = span_text(text, span).as_deref() == Some(&inj.informal_inserted)
                && (inj.informal_dials["replaced"].is_null()
                    || inj.informal_dials["replaced"] != json!(inj.informal_inserted));
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
        if let (Some(f), Some(obs), true) = (&p.formal, observation, in_formal) {
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
