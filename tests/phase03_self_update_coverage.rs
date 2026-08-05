#![cfg(unix)]

mod phase02_fixture;

use phase02_fixture::Fixture;
use std::process::Command;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_PHASE02_MARKER\"\n";

fn run_self_update(args: &[&str], distribution: Option<&str>) -> std::process::Output {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"));
    cmd.args(args);
    if let Some(dist) = distribution {
        cmd.env("TERMINAL_JARVIS_DISTRIBUTION", dist);
    }
    cmd.env(
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
    .env("TJ_PHASE02_MARKER", fixture.marker_path())
    .env("TJ_PHASE02_GATE_MARKER", fixture.gate_marker_path());
    cmd.output().expect("terminal-jarvis runs")
}

#[test]
fn self_update_preview_cargo_channel() {
    let output = run_self_update(&["--plain", "self-update", "--dry-run"], Some("cargo"));
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan:"));
    assert!(stdout.contains("cargo install terminal-jarvis"));
}

#[test]
fn self_update_preview_homebrew_channel() {
    let output = run_self_update(&["--plain", "self-update", "--dry-run"], Some("homebrew"));
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan:"));
    assert!(stdout.contains("brew upgrade terminal-jarvis"));
}

#[test]
fn self_update_preview_npm_channel() {
    let output = run_self_update(&["--plain", "self-update", "--dry-run"], Some("npm"));
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan:"));
    assert!(stdout.contains("npm install -g terminal-jarvis@latest"));
}

#[test]
fn self_update_preview_source_channel() {
    let output = run_self_update(&["--plain", "self-update", "--dry-run"], Some("source"));
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan:"));
    assert!(stdout.contains("cargo install terminal-jarvis"));
}

#[test]
fn self_update_preview_direct_channel_manual() {
    let output = run_self_update(&["--plain", "self-update", "--dry-run"], Some("direct"));
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan:"));
    assert!(stdout.contains("download and checksum-verify"));
}

#[test]
fn self_update_preview_unknown_channel_manual() {
    let output = run_self_update(&["--plain", "self-update", "--dry-run"], Some("custom"));
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan:"));
    assert!(stdout.contains("identify the install channel"));
}

#[test]
fn self_update_alias_works() {
    let output = run_self_update(&["--plain", "--update", "--dry-run"], Some("cargo"));
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan:"));
    assert!(stdout.contains("cargo install terminal-jarvis"));
}
