//! The Lean harness (ADR-0011 D8): `lake env lean <absolute path>` from `lean/`, one process at a
//! time, then — only when the file builds — the same bytes with an audit tail compiled the same
//! way. Parsing is separated from running so the parsers are tested without Lean.
//!
//! v0.1 audit tail: `#print axioms` of the main declaration. The used-constants listing that the
//! T check needs (FLAW-0007, FLAW-0009, FLAW-0013) arrives with the first producer version that
//! realizes one of those classes; no v0.1 seed does.

use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};

/// The standard three (SD2(ii)).
pub const STANDARD_AXIOMS: &[&str] = &["propext", "Classical.choice", "Quot.sound"];

/// What the harness saw for one formal file: the build's exit code, the line of the first
/// reported error (if any), and the main declaration's axioms when the file built and the audit
/// tail printed them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub exit_code: i32,
    pub first_error_line: Option<u64>,
    pub axioms: Option<Vec<String>>,
}

impl Observation {
    /// `lean_axioms_standard`: `None` when no axiom print was observed.
    pub fn axioms_standard(&self) -> Option<bool> {
        self.axioms
            .as_ref()
            .map(|a| a.iter().all(|x| STANDARD_AXIOMS.contains(&x.as_str())))
    }
}

/// The audit file: the artifact's exact bytes followed by a blank line and the axiom print.
pub fn audit_bytes(artifact: &[u8], main_declaration: &str) -> Vec<u8> {
    let mut out = artifact.to_vec();
    out.extend_from_slice(format!("\n#print axioms {main_declaration}\n").as_bytes());
    out
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
    let axioms = audit.and_then(|a| {
        (a.exit_code == 0)
            .then(|| parse_axioms(&format!("{}\n{}", a.stdout, a.stderr), main_declaration))
            .flatten()
    });
    Observation {
        exit_code: build.exit_code,
        first_error_line: build
            .argv
            .last()
            .and_then(|file| first_error_line(&build_out, file)),
        axioms,
    }
}
