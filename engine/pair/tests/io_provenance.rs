//! Isolated development fixtures; never corpus items or semantic fidelity measurements.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};
use syndeom_pair::{output, provenance};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Scratch {
    path: PathBuf,
    base: PathBuf,
}
impl Scratch {
    fn new() -> Self {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target/pair-tests");
        fs::create_dir_all(&base).unwrap();
        let base = fs::canonicalize(base).unwrap();
        let path = base.join(format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&path).unwrap();
        Self { path, base }
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        // Delete only this resolved, owned fixture directory inside the explicit test root.
        if let Ok(path) = fs::canonicalize(&self.path) {
            assert!(path != self.base && path.starts_with(&self.base));
            fs::remove_dir_all(path).unwrap();
        }
    }
}

#[test]
fn success_replay_preserves_existing_file_times_and_dry_run_writes_nothing() {
    let s = Scratch::new();
    let path = s.path.join("item/a.json");
    let plan = vec![(path.clone(), b"fixed\n".to_vec())];
    output::write(&plan, true).unwrap();
    assert!(!path.exists());
    output::write(&plan, false).unwrap();
    let time = fs::metadata(&path).unwrap().modified().unwrap();
    output::write(&plan, false).unwrap();
    assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), time);
    assert_eq!(fs::read(path).unwrap(), b"fixed\n");
}

#[test]
fn collision_on_last_output_prevents_all_writes_even_during_dry_run() {
    let s = Scratch::new();
    let first = s.path.join("new/a.json");
    let last = s.path.join("existing.txt");
    fs::write(&last, b"historical").unwrap();
    let plan = vec![
        (first.clone(), b"new".to_vec()),
        (last.clone(), b"different".to_vec()),
    ];
    for dry in [true, false] {
        assert!(output::write(&plan, dry).unwrap_err().contains("conflict"));
        assert!(!first.exists());
        assert_eq!(fs::read(&last).unwrap(), b"historical");
    }
}

#[test]
fn partial_file_refused_but_missing_siblings_can_be_completed() {
    let s = Scratch::new();
    let a = s.path.join("a");
    let b = s.path.join("b");
    let plan = vec![
        (a.clone(), b"complete".to_vec()),
        (b.clone(), b"second".to_vec()),
    ];
    fs::write(&a, b"partial").unwrap();
    assert!(output::write(&plan, false).is_err());
    assert!(!b.exists());
    // Fixture repair is explicit; producer never overwrites a partial file.
    fs::write(&a, b"complete").unwrap();
    output::write(&plan, false).unwrap();
    assert_eq!(fs::read(b).unwrap(), b"second");
}

#[test]
fn hardlink_case_alias_ancestor_and_dot_paths_refused() {
    let s = Scratch::new();
    let original = s.path.join("original");
    let alias = s.path.join("alias");
    fs::write(&original, b"same").unwrap();
    fs::hard_link(&original, &alias).unwrap();
    assert!(
        output::write(&vec![(alias, b"same".to_vec())], true)
            .unwrap_err()
            .contains("hard-linked")
    );
    assert!(
        output::preflight(&vec![
            (s.path.join("A"), vec![]),
            (s.path.join("a"), vec![])
        ])
        .is_err()
    );
    assert!(
        output::preflight(&vec![
            (s.path.join("x"), vec![]),
            (s.path.join("x/y"), vec![])
        ])
        .is_err()
    );
    // PathBuf::join normalizes ".." when the Windows base is a verbatim path.
    // Preserve the caller's literal spelling so this actually tests a dot component.
    let sep = std::path::MAIN_SEPARATOR;
    let dot_path = PathBuf::from(format!("{}{sep}x{sep}..{sep}escape", s.path.display()));
    assert!(output::safe_path(&dot_path).is_err());
    assert!(output::safe_path(&s.path.join("bad:stream")).is_err());
}

#[test]
fn redirected_directory_is_refused_without_target_writes() {
    let s = Scratch::new();
    let real = s.path.join("real");
    let alias = s.path.join("alias");
    fs::create_dir(&real).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&real, &alias).unwrap();
    #[cfg(windows)]
    {
        let result = Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(&alias)
            .arg(&real)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "junction fixture setup failed: {result:?}"
        );
    }
    assert!(
        output::write(&vec![(alias.join("new"), b"no".to_vec())], false)
            .unwrap_err()
            .contains("symlink/junction")
    );
    assert!(!real.join("new").exists());
}

fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    let p = Command::new("git")
        .arg("-c")
        .arg("core.autocrlf=false")
        .arg("-c")
        .arg("core.hooksPath=disabled-test-hooks")
        .arg("-c")
        .arg("user.name=SYNDEOM development fixture")
        .arg("-c")
        .arg("user.email=fixture@example.invalid")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(p.status.success(), "{p:?}");
    p.stdout
}

#[test]
fn provenance_checks_executable_git_and_checkout_and_propagates_subprocess_failure() {
    let s = Scratch::new();
    git(&s.path, &["init", "-q"]);
    git(
        &s.path,
        &[
            "commit",
            "--allow-empty",
            "-qm",
            "development fixture before producer",
        ],
    );
    let old = String::from_utf8(git(&s.path, &["rev-parse", "HEAD"]))
        .unwrap()
        .trim()
        .to_string();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    for &(name, _) in provenance::COMPILED_INPUTS {
        fs::create_dir_all(s.path.join(name).parent().unwrap()).unwrap();
        fs::copy(root.join(name), s.path.join(name)).unwrap();
    }
    git(&s.path, &["add", "."]);
    git(
        &s.path,
        &[
            "commit",
            "-qm",
            "development fixture with exact producer inputs",
        ],
    );
    let revision = String::from_utf8(git(&s.path, &["rev-parse", "HEAD"]))
        .unwrap()
        .trim()
        .to_string();
    provenance::verify_producer(&s.path, &revision, &mut vec![]).unwrap();
    let mut log = Vec::new();
    assert!(provenance::verify_producer(&s.path, &old, &mut log).is_err());
    assert!(log.iter().any(|r| r["success"] == false)); // actual git show subprocess refusal
    fs::write(
        s.path.join("engine/pair/src/lib.rs"),
        b"altered implementation",
    )
    .unwrap();
    assert!(
        provenance::verify_producer(&s.path, &revision, &mut vec![])
            .unwrap_err()
            .contains("mismatch")
    );
    // Explicit controlled subprocess failure, not a failed production generation.
    let mut failure_log = Vec::new();
    assert!(
        provenance::run_checked(
            "git",
            &["--syndeom-controlled-failure"],
            &s.path,
            &mut failure_log
        )
        .is_err()
    );
    assert_eq!(failure_log[0]["success"], false);
}
