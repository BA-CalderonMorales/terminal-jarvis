#![cfg(unix)]

mod phase02_fixture;

use phase02_fixture::Fixture;
use std::time::{Duration, Instant};

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_PHASE02_MARKER\"\n";

#[test]
fn unknown_lifecycle_row_fails_closed_before_spawn() {
    let fixture = Fixture::new("unknown", "expected", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "install",
        "fixture",
        "--no-input",
        "--confirm=download:fixture",
    ]);
    assert_eq!(output.status.code(), Some(4));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown"));
}

#[test]
fn noninteractive_lifecycle_requires_exact_bound_intent() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let started = Instant::now();
    let missing = fixture.run(&["--plain", "install", "fixture"]);
    assert!(started.elapsed() < Duration::from_secs(1));
    assert_eq!(missing.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let wrong = fixture.run(&[
        "--plain",
        "install",
        "fixture",
        "--no-input",
        "--confirm=update:fixture",
    ]);
    assert_eq!(wrong.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let diagnostic = String::from_utf8_lossy(&wrong.stderr);
    assert!(diagnostic.contains("--confirm=download:fixture"));
}

#[test]
fn lifecycle_dry_run_previews_without_spawning() {
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
fn exact_bound_intent_allows_an_executable_lifecycle_operation() {
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
