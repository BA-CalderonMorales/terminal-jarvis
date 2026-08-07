use super::*;
use crate::diagnostics::{Environment, PlatformInput, RuntimeInput};
use std::path::Path;
use std::time::{Duration, SystemTime};

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

#[test]
fn direct_with_existing_executable_file() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_executable");
    std::fs::write(&test_file, "").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&test_file).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&test_file, perms).unwrap();
    }

    let result = direct(&test_file);
    assert_eq!(result.code, Code::Ready);
    assert_eq!(result.matches, 1);
    assert!(result.path.is_some());

    let _ = std::fs::remove_file(&test_file);
}

#[test]
fn direct_with_nonexistent_file() {
    let result = direct(Path::new("/nonexistent/path/to/file"));
    assert_eq!(result.code, Code::Missing);
    assert_eq!(result.matches, 0);
}

#[test]
fn direct_with_directory() {
    let temp_dir = std::env::temp_dir();
    let result = direct(&temp_dir);
    assert_eq!(result.code, Code::Malformed);
}

#[test]
fn binary_empty_name() {
    let result = binary("", &dummy_input());
    assert_eq!(result.code, Code::Malformed);
}

#[test]
fn binary_with_path_separators() {
    let result = binary("/usr/bin/echo", &dummy_input());
    assert_eq!(result.code, Code::Ready);
    assert!(result.path.is_some());
}

#[test]
fn binary_not_found() {
    let result = binary("nonexistent_command_12345", &dummy_input());
    assert_eq!(result.code, Code::Missing);
    assert_eq!(result.matches, 0);
}

#[test]
fn pathext_skips_empty_segments() {
    let mut input = dummy_input();
    input.platform.os = "windows".into();
    input.environment.insert("PATHEXT", ".COM;;.EXE;");
    assert_eq!(candidates("tool", &input), vec!["tool.COM", "tool.EXE"]);
}
