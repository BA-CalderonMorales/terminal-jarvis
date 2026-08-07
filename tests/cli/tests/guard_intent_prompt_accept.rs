#![cfg(unix)]

use crate::logic::cli_driver::Fixture;
use std::io::{IsTerminal, Write};
use std::process::Command;
use std::process::Stdio;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";

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
