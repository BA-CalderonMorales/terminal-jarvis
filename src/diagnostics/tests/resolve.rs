use super::*;
use crate::diagnostics::{Environment, PlatformInput, RuntimeInput};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

static COUNTER: AtomicU64 = AtomicU64::new(0);
fn dummy_input() -> DiagnosticInput {
    DiagnosticInput {
        version: "0.1.13".into(),
        executable: None,
        catalog: PathBuf::from("/tmp/catalog"),
        home: PathBuf::from("/tmp/home"),
        config: PathBuf::from("/tmp/config"),
        home_prefix: None,
        temp_prefix: None,
        active_harness: None,
        harnesses: vec![],
        platform: PlatformInput {
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
            libc: crate::context::platform::libc().into(),
            wsl: crate::context::platform::wsl().into(),
        },
        environment: Environment::process(),
        runtime: RuntimeInput {
            gate: PathBuf::from("/tmp/gate"),
            stdout_tty: true,
            stderr_tty: true,
            color: false,
            width: 80,
            update_route: "source".into(),
            checksum: "".into(),
            probes: true,
        },
        now: SystemTime::now(),
        stale_after: Duration::from_secs(300),
    }
}

fn temp_executable() -> std::path::PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let test_file = std::env::temp_dir().join(format!("resolve_test_binary_{n}"));
    std::fs::write(&test_file, "").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&test_file).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&test_file, perms).unwrap();
    }
    test_file
}

fn assert_ready(result: &Resolution) {
    assert_eq!(result.code, Code::Ready);
    assert_eq!(result.matches, 1);
    assert!(result.path.is_some());
}

#[test]
fn direct_with_existing_executable_file() {
    let test_file = temp_executable();
    assert_ready(&direct(&test_file));
    let _ = std::fs::remove_file(&test_file);
}

#[test]
fn direct_rejects_missing_paths_and_directories() {
    let missing = direct(Path::new("/nonexistent/path/to/file"));
    assert_eq!(missing.code, Code::Missing);
    assert_eq!(missing.matches, 0);
    let directory = direct(&std::env::temp_dir());
    assert_eq!(directory.code, Code::Malformed);
}

#[test]
fn binary_rejects_empty_and_unknown_names() {
    let empty = binary("", &dummy_input());
    assert_eq!(empty.code, Code::Malformed);
    let missing = binary("nonexistent_command_12345", &dummy_input());
    assert_eq!(missing.code, Code::Missing);
    assert_eq!(missing.matches, 0);
}

#[test]
fn binary_with_path_separators() {
    let test_file = temp_executable();
    let result = binary(&test_file.to_string_lossy(), &dummy_input());
    assert_ready(&result);
    let _ = std::fs::remove_file(&test_file);
}

#[test]
fn pathext_skips_empty_segments() {
    let mut input = dummy_input();
    input.platform.os = "windows".into();
    input.environment.insert("PATHEXT", ".COM;;.EXE;");
    assert_eq!(candidates("tool", &input), vec!["tool.COM", "tool.EXE"]);
}
