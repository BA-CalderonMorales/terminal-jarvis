#![cfg(unix)]

use crate::logic::cli_driver::Fixture;
use std::process::Command;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";

const TOKEN: &str = "self-update:terminal-jarvis";

#[test]
fn self_update_intent_dry_run_bypasses_confirmation() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "self-update", "--dry-run"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("terminal-jarvis update plan:"));
}

#[test]
fn self_update_intent_confirm_token_match_with_no_input_succeeds() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "self-update", "--no-input", "--confirm", TOKEN])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
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

    // self-update with --no-input --confirm=TOKEN should succeed (exit 0)
    // Note: actual execution may fail if cargo is not available, but intent check should pass
    // The intent check passes, then it tries to run cargo which may fail
    // We check that it didn't fail at the intent check level (exit 5)
    assert_ne!(output.status.code(), Some(5));
}

#[test]
fn self_update_intent_confirm_token_mismatch_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args([
            "--plain",
            "self-update",
            "--no-input",
            "--confirm=wrong:token",
        ])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
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

    assert_eq!(output.status.code(), Some(5));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(
        "noninteractive self-update requires --no-input --confirm=self-update:terminal-jarvis"
    ));
}

#[test]
fn self_update_intent_confirm_missing_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "self-update", "--no-input"])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
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

    assert_eq!(output.status.code(), Some(5));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(
        "noninteractive self-update requires --no-input --confirm=self-update:terminal-jarvis"
    ));
}

#[test]
fn self_update_intent_allow_dangerous_rejected() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "self-update", "--allow-dangerous", "--dry-run"])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
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

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--allow-dangerous is not valid for self-update"));
}

#[test]
fn self_update_intent_no_input_without_confirm_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "self-update", "--no-input"])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
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

    assert_eq!(output.status.code(), Some(5));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(
        "noninteractive self-update requires --no-input --confirm=self-update:terminal-jarvis"
    ));
}

#[test]
fn self_update_intent_no_terminal_without_confirm_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    // When stdin is not a terminal and no --no-input, should fail
    let output = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "self-update"])
        .env("TERMINAL_JARVIS_DISTRIBUTION", "source")
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

    // In non-interactive mode (no terminal), should fail with intent error
    assert_eq!(output.status.code(), Some(5));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(
        "noninteractive self-update requires --no-input --confirm=self-update:terminal-jarvis"
    ));
}
