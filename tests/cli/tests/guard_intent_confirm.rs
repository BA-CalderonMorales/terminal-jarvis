#![cfg(unix)]

use crate::logic::cli_driver::Fixture;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\n";

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
