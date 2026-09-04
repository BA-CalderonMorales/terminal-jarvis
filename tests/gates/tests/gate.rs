use crate::structs::catalog;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(unix)]
const NOOP_SCRIPT: &str = "#!/bin/sh\n";
#[cfg(not(unix))]
const NOOP_SCRIPT: &str = "@echo off\r\n";

/// `terminal-jarvis run` resolves a harness binary via `PATHEXT` on Windows
/// (see `security::checks::resolve_on_path`), which only matches
/// extension-suffixed files, so the fixture needs a `.cmd` there.
#[cfg(unix)]
fn script_filename(name: &str) -> String {
    name.to_string()
}

#[cfg(not(unix))]
fn script_filename(name: &str) -> String {
    format!("{name}.cmd")
}

#[cfg(unix)]
fn make_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = std::fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(path, permissions).unwrap();
}

#[cfg(not(unix))]
fn make_executable(_path: &std::path::Path) {}

static TEMP_ID: AtomicUsize = AtomicUsize::new(0);

fn home() -> String {
    std::env::temp_dir()
        .join(format!(
            "terminal-jarvis-gate-{}-{}",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        ))
        .to_string_lossy()
        .to_string()
}

fn tj(args: &[&str], home: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .arg("--plain")
        .args(args)
        .env("TERMINAL_JARVIS_HOME", home)
        .output()
        .expect("terminal-jarvis runs")
}

#[test]
fn gate_is_disabled_by_default_and_can_be_enabled() {
    let home = home();
    let status = tj(&["gate", "status"], &home);
    assert!(status.status.success());
    assert!(String::from_utf8_lossy(&status.stdout).contains("gate: disabled"));
    assert!(tj(&["gate", "enable", "trivy"], &home).status.success());
    let enabled = tj(&["gate", "status"], &home);
    assert!(String::from_utf8_lossy(&enabled.stdout).contains("gate: trivy (config)"));
}

#[test]
fn enabled_missing_trivy_warns_and_runs_the_harness() {
    let root = home();
    let home = format!("{root}/home");
    let catalog_root = std::path::Path::new(&root).join("catalog");
    let bin = std::path::Path::new(&root).join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let child = bin.join(script_filename("fixture-child"));
    std::fs::write(&child, NOOP_SCRIPT).unwrap();
    make_executable(&child);
    catalog::write(&catalog_root, "expected", "expected");
    assert!(tj(&["gate", "enable", "trivy"], &home).status.success());
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "run", "fixture", "headless"])
        .env("TERMINAL_JARVIS_HOME", home)
        .env("TERMINAL_JARVIS_CATALOG", catalog_root)
        .env("PATH", bin)
        .output()
        .expect("terminal-jarvis runs");
    assert_eq!(output.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("warning: optional gate 'trivy' is enabled"));
    assert!(stderr.contains("trivy.dev/docs/latest/getting-started/installation"));
}
