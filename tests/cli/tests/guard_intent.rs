#![cfg(unix)]

use crate::logic::cli_driver::Fixture;
use std::io::{IsTerminal, Write};
use std::process::Command;
use std::process::Stdio;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";

#[test]
fn guard_intent_dry_run_previews_without_spawning() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "install", "fixture", "--dry-run"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let preview = String::from_utf8_lossy(&output.stdout);
    assert!(preview.contains("fixture-child"));
    assert!(preview.contains("download"));
}

#[test]
fn guard_intent_interactive_requires_terminal() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "fixture",
        "ui",
        "--no-input",
        "--confirm=ui:fixture",
    ]);
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.status.code(), Some(5), "exit code should be 5");
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("interactive capability requires a terminal"));
    assert!(stderr.contains("terminal-jarvis plan fixture ui"));
}

#[test]
fn guard_intent_dangerous_requires_explicit_and_allow_dangerous() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let no_allow_dangerous = fixture.run(&[
        "--plain",
        "run",
        "fixture",
        "yolo",
        "--no-input",
        "--confirm=yolo:fixture",
    ]);
    eprintln!(
        "no_allow_dangerous stdout: {}",
        String::from_utf8_lossy(&no_allow_dangerous.stdout)
    );
    eprintln!(
        "no_allow_dangerous stderr: {}",
        String::from_utf8_lossy(&no_allow_dangerous.stderr)
    );
    assert_eq!(no_allow_dangerous.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());

    let no_confirm = fixture.run(&[
        "--plain",
        "run",
        "fixture",
        "yolo",
        "--no-input",
        "--allow-dangerous",
    ]);
    eprintln!(
        "no_confirm stdout: {}",
        String::from_utf8_lossy(&no_confirm.stdout)
    );
    eprintln!(
        "no_confirm stderr: {}",
        String::from_utf8_lossy(&no_confirm.stderr)
    );
    assert_eq!(no_confirm.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
}

#[test]
fn guard_intent_dangerous_dry_run_previews_without_spawning() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "run", "fixture", "yolo", "--dry-run"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let preview = String::from_utf8_lossy(&output.stdout);
    assert!(preview.contains("fixture-child"));
    assert!(preview.contains("yolo"));
}

#[test]
fn guard_intent_confirm_token_match_with_no_input_succeeds() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "install",
        "fixture",
        "--no-input",
        "--confirm=download:fixture",
    ]);
    assert_eq!(output.status.code(), Some(0));
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}

#[test]
fn guard_intent_confirm_token_match_with_terminal_succeeds() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "fixture",
        "download",
        "--no-input",
        "--confirm=download:fixture",
    ]);
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());

    // Also test with install command (legacy style)
    let output2 = fixture.run(&[
        "--plain",
        "install",
        "fixture",
        "--no-input",
        "--confirm=download:fixture",
    ]);
    eprintln!(
        "install fixture stdout: {}",
        String::from_utf8_lossy(&output2.stdout)
    );
    eprintln!(
        "install fixture stderr: {}",
        String::from_utf8_lossy(&output2.stderr)
    );
    eprintln!("install fixture exit code: {:?}", output2.status.code());

    assert_eq!(output.status.code(), Some(0));
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}

#[test]
fn guard_intent_confirm_mismatch_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "install",
        "fixture",
        "--no-input",
        "--confirm=update:fixture",
    ]);
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());
    assert_eq!(output.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("noninteractive execution requires --no-input --confirm=download:fixture")
    );
    assert!(stderr.contains("review the plan, then pass --no-input --confirm=download:fixture"));
}

#[test]
fn guard_intent_confirm_missing_fails() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "install", "fixture", "--no-input"]);
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());
    assert_eq!(output.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("noninteractive execution requires --no-input --confirm=download:fixture")
    );
    assert!(stderr.contains("review the plan, then pass --no-input --confirm=download:fixture"));
}

#[test]
fn guard_intent_prompt_accepts_yes() {
    // This test requires stdin to be a terminal (interactive prompt path)
    if !std::io::stdin().is_terminal() {
        eprintln!("Skipping: stdin is not a terminal");
        return;
    }
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let mut child = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "install", "fixture"])
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
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("terminal-jarvis spawns");

    child.stdin.as_mut().unwrap().write_all(b"yes\n").unwrap();
    let output = child.wait_with_output().expect("wait");

    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());

    assert_eq!(output.status.code(), Some(0));
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}

#[test]
fn guard_intent_prompt_accepts_y() {
    // This test requires stdin to be a terminal (interactive prompt path)
    if !std::io::stdin().is_terminal() {
        eprintln!("Skipping: stdin is not a terminal");
        return;
    }
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let mut child = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "install", "fixture"])
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
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("terminal-jarvis spawns");

    child.stdin.as_mut().unwrap().write_all(b"y\n").unwrap();
    let output = child.wait_with_output().expect("wait");

    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());

    assert_eq!(output.status.code(), Some(0));
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}

#[test]
fn guard_intent_prompt_rejects_no() {
    // This test requires stdin to be a terminal (interactive prompt path)
    if !std::io::stdin().is_terminal() {
        eprintln!("Skipping: stdin is not a terminal");
        return;
    }
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let mut child = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "install", "fixture"])
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
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("terminal-jarvis spawns");

    child.stdin.as_mut().unwrap().write_all(b"no\n").unwrap();
    let output = child.wait_with_output().expect("wait");

    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());

    assert_eq!(output.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("confirmation_declined"));
}

#[test]
fn guard_intent_prompt_rejects_n() {
    // This test requires stdin to be a terminal (interactive prompt path)
    if !std::io::stdin().is_terminal() {
        eprintln!("Skipping: stdin is not a terminal");
        return;
    }
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let mut child = Command::new(env!("CARGO_BIN_EXE_terminal-jarvis"))
        .args(["--plain", "install", "fixture"])
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
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("terminal-jarvis spawns");

    child.stdin.as_mut().unwrap().write_all(b"n\n").unwrap();
    let output = child.wait_with_output().expect("wait");

    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    eprintln!("exit code: {:?}", output.status.code());

    assert_eq!(output.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("confirmation_declined"));
}
