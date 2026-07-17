#![cfg(unix)]

mod phase02_fixture;

use phase02_fixture::Fixture;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_PHASE02_MARKER\"\n";

#[test]
fn interactive_capability_rejects_non_terminal_execution_before_spawn() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&[
        "--plain",
        "run",
        "fixture",
        "ui",
        "--no-input",
        "--confirm=ui:fixture",
    ]);
    assert_eq!(output.status.code(), Some(5));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(diagnostic.contains("interactive capability requires a terminal"));
}

#[test]
fn interactive_dry_run_previews_without_a_terminal_or_spawn() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    let output = fixture.run(&["--plain", "run", "fixture", "ui", "--dry-run"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    let preview = String::from_utf8_lossy(&output.stdout);
    assert!(preview.contains("fixture-child"));
    assert!(preview.contains("ui"));
}
