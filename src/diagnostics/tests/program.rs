use super::*;
use crate::diagnostics::RuntimeInput;
use std::fs;
use std::path::{Path, PathBuf};

fn exe(directory: &Path, name: &str) -> PathBuf {
    let binary = directory.join(name);
    fs::write(&binary, "").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binary).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&binary, perms).unwrap();
    }
    binary
}
fn temp_dir(tag: &str) -> PathBuf {
    let directory = std::env::temp_dir().join(format!("tj_program_{}_{}", std::process::id(), tag));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).unwrap();
    directory
}

fn input(current: PathBuf, dirs: &[&Path]) -> DiagnosticInput {
    let mut local = DiagnosticInput::local(
        Path::new("/tmp/tj-catalog"),
        Path::new("/tmp/tj-home"),
        None,
        &[],
        RuntimeInput {
            gate: "/tmp/tj-gate".into(),
            stdout_tty: false,
            stderr_tty: false,
            color: false,
            width: 0,
            update_route: String::new(),
            checksum: String::new(),
            probes: false,
        },
    );
    local.executable = Some(current);
    local
        .environment
        .insert("PATH", std::env::join_paths(dirs).unwrap());
    local
}

fn run(current: &Path, dirs: &[&Path]) -> (Record, Record, bool) {
    collect(
        &input(current.to_path_buf(), dirs),
        &Redactor::new(None, None),
    )
}
#[test]
fn collect_marks_path_states() {
    let scratch = temp_dir("scratch");
    let (bin, one, two) = (temp_dir("bin"), temp_dir("one"), temp_dir("two"));
    for dir in [&one, &two, &scratch] {
        exe(dir, "tj");
    }
    let current = exe(&scratch, "tj");

    let (_, path, _) = run(current.as_path(), &[bin.as_path()]);
    assert_eq!(path.code, Code::Ready);
    assert_eq!(path.value, "direct");
    let (_, path, ok) = run(current.as_path(), &[one.as_path()]);
    assert_eq!(path.code, Code::Conflicting);
    assert_eq!(path.value, "shadowed");
    assert!(!ok);
    let (_, path, ok) = run(current.as_path(), &[one.as_path(), two.as_path()]);
    assert_eq!(path.code, Code::Conflicting);
    assert_eq!(path.value, "shadowed (2 matches)");
    assert!(!ok);
    for dir in [&scratch, &bin, &one, &two] {
        let _ = fs::remove_dir_all(dir);
    }
}

#[test]
fn shadowed_record_excludes_the_current_binary() {
    let (scratch, other_dir) = (temp_dir("record"), temp_dir("other"));
    let (current, other) = (exe(&scratch, "tj-current"), exe(&other_dir, "tj-other"));
    let resolved = super::super::resolve::Resolution {
        code: Code::Conflicting,
        path: Some(current.clone()),
        paths: vec![current.clone(), other.clone()],
        matches: 2,
    };
    let record = shadowed_record(&Redactor::new(None, None), &resolved, &current);
    assert_eq!(record.value, "shadowed (2 matches)");
    let expected = format!(
        "remove stale or shadowing PATH entries: {}",
        other.to_string_lossy()
    );
    assert_eq!(record.action.as_deref(), Some(expected.as_str()));
    for dir in [&scratch, &other_dir] {
        let _ = fs::remove_dir_all(dir);
    }
}
