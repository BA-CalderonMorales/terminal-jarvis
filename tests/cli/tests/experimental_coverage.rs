#![cfg(unix)]

use crate::logic::cli_driver::Fixture;
use std::process::Command;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";

#[test]
fn experimental_dashboard_without_env_var_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "experimental", "dashboard"]);
    assert_eq!(output.status.code(), Some(4));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("experimental dashboard is disabled"));
    assert!(stderr.contains("TERMINAL_JARVIS_EXPERIMENTAL_UI=1"));
}

#[test]
fn experimental_dashboard_with_env_var_succeeds() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "experimental", "dashboard"])
        .env("TERMINAL_JARVIS_EXPERIMENTAL_UI", "1")
        .env(
            "PATH",
            format!(
                "{}:{}",
                fixture.root.join("bin").display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .env("TERMINAL_JARVIS_CATALOG", fixture.root.join("catalog"))
        .env("TERMINAL_JARVIS_GATE", "acceptance")
        .env("TERMINAL_JARVIS_GATES", fixture.root.join("gates"))
        .env("TERMINAL_JARVIS_HOME", fixture.root.join("home"))
        .env("TJ_FIXTURE_MARKER", fixture.marker_path())
        .env("TJ_FIXTURE_GATE_MARKER", fixture.gate_marker_path())
        .output()
        .expect("terminal-jarvis runs");

    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("experimental dashboard"));
    assert!(stdout.contains("active harness:"));
    assert!(stdout.contains("readiness:"));
}

#[test]
fn experimental_unknown_subcommand_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "experimental", "unknown"]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("usage: terminal-jarvis experimental dashboard"));
}

#[test]
fn experimental_no_args_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "experimental"]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("usage: terminal-jarvis experimental dashboard"));
}
