#![cfg(unix)]

mod phase02_fixture;

use phase02_fixture::Fixture;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_PHASE02_MARKER\"\n";

#[test]
fn dangerous_execution_requires_separate_opt_in_and_exact_intent() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let no_opt_in = fixture.run(&[
        "--plain",
        "run",
        "fixture",
        "yolo",
        "--no-input",
        "--confirm=yolo:fixture",
    ]);
    assert_eq!(no_opt_in.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let no_intent = fixture.run(&[
        "--plain",
        "run",
        "fixture",
        "yolo",
        "--no-input",
        "--allow-dangerous",
    ]);
    assert_eq!(no_intent.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
}

#[test]
fn exact_dangerous_opt_ins_allow_an_executable_operation() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "run",
        "fixture",
        "yolo",
        "--no-input",
        "--allow-dangerous",
        "--confirm=yolo:fixture",
    ]);
    assert_eq!(output.status.code(), Some(0));
    assert!(fixture.spawned());
    assert!(fixture.gate_spawned());
}

#[test]
fn dangerous_dry_run_previews_without_spawning() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "run", "fixture", "yolo", "--dry-run"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    assert!(String::from_utf8_lossy(&output.stdout).contains("fixture-child"));
}

#[test]
fn disabled_dangerous_row_wins_over_all_execution_opt_ins() {
    let fixture = Fixture::new("expected", "disabled", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "run",
        "fixture",
        "yolo",
        "--no-input",
        "--allow-dangerous",
        "--confirm=yolo:fixture",
    ]);
    assert_eq!(output.status.code(), Some(4));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    assert!(String::from_utf8_lossy(&output.stderr).contains("disabled"));
}
