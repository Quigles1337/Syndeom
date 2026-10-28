//! The Lean harness (ADR-0011 D8): `lake env lean <absolute path>` from `lean/`, one process at a
//! time, then — only when the file builds — the same bytes with an audit tail compiled the same
//! way. Parsing is separated from running so the parsers are tested without Lean.
//!
//! The audit tail prints `#print axioms` of the main declaration and, for every declaration of
//! the file, its direct used-constants (`Lean.ConstantInfo.getUsedConstantsAsSet` in a `#eval`
//! over `MetaM`). The Rust side closes the used-constants relation over the file's own
//! declarations and evaluates the class's condition — check T (FLAW-0007 `unused`, FLAW-0009;
//! `s2_template` 0.3.0).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};

/// The standard three (SD2(ii)).
pub const STANDARD_AXIOMS: &[&str] = &["propext", "Classical.choice", "Quot.sound"];

/// What the harness saw for one formal file: the build's exit code, the line of the first
/// reported error (if any), the main declaration's axioms when the file built and the audit tail
/// printed them, and the used-constants listing the tail printed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub exit_code: i32,
    pub first_error_line: Option<u64>,
    pub axioms: Option<Vec<String>>,
    /// Per declaration of the file, its direct used-constants as the audit tail printed them,
    /// each list sorted and deduplicated (0.3.0). `None` when no audit ran or the tail printed no
    /// listing; absent from the observations of earlier producer versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_constants: Option<BTreeMap<String, Vec<String>>>,
}

impl Observation {
    /// `lean_axioms_standard`: `None` when no axiom print was observed.
    pub fn axioms_standard(&self) -> Option<bool> {
        self.axioms
            .as_ref()
            .map(|a| a.iter().all(|x| STANDARD_AXIOMS.contains(&x.as_str())))
    }
}

/// The audit tail (ADR-0011 D8): the axiom print of the main declaration, then one line per
/// constant the file itself defined — the declarations and Lean's auxiliary constants
/// (`<name>._proof_N`), the second stage of the environment's constant map — with its direct
/// used-constants.
pub const AUDIT_TAIL: &str = "open Lean in
#eval show Lean.Meta.MetaM Unit from do
  let env ← Lean.getEnv
  for (n, ci) in env.constants.map₂.toList do
    IO.println s!\"used-constants {n}: {ci.getUsedConstantsAsSet.toList}\"
";

/// The audit file: the artifact's exact bytes followed by a blank line, the axiom print of the
/// main declaration, and the used-constants listing of every constant the file defined.
pub fn audit_bytes(artifact: &[u8], main_declaration: &str) -> Vec<u8> {
    let mut out = artifact.to_vec();
    out.extend_from_slice(format!("\n#print axioms {main_declaration}\n").as_bytes());
    out.extend_from_slice(AUDIT_TAIL.as_bytes());
    out
}

/// Check T's closure (ADR-0011 D5, D8): the constants of the file reachable from `main` through
/// the listing — `main` itself, the file's constants it uses directly, theirs, and so on. Names
/// the listing does not define (imported constants) are not followed and not included.
pub fn used_closure(used: &BTreeMap<String, Vec<String>>, main: &str) -> BTreeSet<String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut stack = vec![main.to_string()];
    while let Some(d) = stack.pop() {
        if !seen.insert(d.clone()) {
            continue;
        }
        if let Some(list) = used.get(&d) {
            for n in list {
                if used.contains_key(n) && !seen.contains(n) {
                    stack.push(n.clone());
                }
            }
        }
    }
    seen
}

/// The line of the first `<file>:<line>:<col>: error` message about `file` itself (the path
/// exactly as it was passed to Lean, which prints it back unchanged), in output order. An error
/// reported against any other file is not this file's first error.
pub fn first_error_line(output: &str, file: &str) -> Option<u64> {
    output.lines().find_map(|l| {
        let at = l.find(": error")?;
        let mut parts = l[..at].rsplitn(3, ':');
        let _col: u64 = parts.next()?.parse().ok()?;
        let line: u64 = parts.next()?.parse().ok()?;
        (parts.next()? == file).then_some(line)
    })
}

/// The axioms `#print axioms <name>` reported, or `None` if no such line is present.
pub fn parse_axioms(output: &str, name: &str) -> Option<Vec<String>> {
    let quoted = format!("'{name}'");
    output.lines().find_map(|l| {
        let rest = l.strip_prefix(&quoted)?;
        if rest == " does not depend on any axioms" {
            return Some(Vec::new());
        }
        let list = rest
            .strip_prefix(" depends on axioms: [")?
            .strip_suffix(']')?;
        Some(list.split(", ").map(str::to_string).collect())
    })
}

/// The used-constants listing the audit tail printed — `used-constants <name>: [a, b, c]`, one
/// line per declaration — each list sorted and deduplicated; `None` if no such line is present.
pub fn parse_used_constants(output: &str) -> Option<BTreeMap<String, Vec<String>>> {
    let mut map = BTreeMap::new();
    for l in output.lines() {
        let Some(rest) = l.strip_prefix("used-constants ") else {
            continue;
        };
        let Some((name, list)) = rest.split_once(": ") else {
            continue;
        };
        let Some(inner) = list.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
            continue;
        };
        let mut names: Vec<String> = inner
            .split(", ")
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();
        names.sort();
        names.dedup();
        map.insert(name.to_string(), names);
    }
    (!map.is_empty()).then_some(map)
}

/// One Lean process as it ran.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LeanRun {
    pub argv: Vec<String>,
    pub cwd: String,
    pub exit_code: i32,
    pub duration_ms: u128,
    pub stdout: String,
    pub stderr: String,
}

/// Run `lake env lean <file>` from `lean_dir` and wait for it.
pub fn run_lean(lean_dir: &Path, file: &Path) -> std::io::Result<LeanRun> {
    let file_arg = file.to_string_lossy().replace('\\', "/");
    let started = Instant::now();
    let out = Command::new("lake")
        .args(["env", "lean", &file_arg])
        .current_dir(lean_dir)
        .output()?;
    Ok(LeanRun {
        argv: vec!["lake".into(), "env".into(), "lean".into(), file_arg],
        cwd: lean_dir.to_string_lossy().replace('\\', "/"),
        exit_code: out.status.code().unwrap_or(-1),
        duration_ms: started.elapsed().as_millis(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    })
}

/// The observation from a build run and, when the build passed, its audit run.
pub fn observe(build: &LeanRun, audit: Option<&LeanRun>, main_declaration: &str) -> Observation {
    let build_out = format!("{}\n{}", build.stdout, build.stderr);
    let audit_out = audit
        .filter(|a| a.exit_code == 0)
        .map(|a| format!("{}\n{}", a.stdout, a.stderr));
    Observation {
        exit_code: build.exit_code,
        first_error_line: build
            .argv
            .last()
            .and_then(|file| first_error_line(&build_out, file)),
        axioms: audit_out
            .as_deref()
            .and_then(|o| parse_axioms(o, main_declaration)),
        used_constants: audit_out.as_deref().and_then(parse_used_constants),
    }
}
