#![cfg(unix)]

use crate::logic::cli_driver::Fixture;

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
