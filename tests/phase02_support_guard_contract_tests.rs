#![cfg(unix)]

mod phase02_fixture;

use phase02_fixture::Fixture;
use std::fs;

const MARKER_SCRIPT: &str = "#!/bin/sh\n: > \"$TJ_PHASE02_MARKER\"\n";

fn assert_guarded(support: &str) {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    set_field(&fixture, "headless", "support", &format!("\"{support}\""));
    match support {
        "manual" => {
            set_field(&fixture, "headless", "evidence", "\"manual\"");
            set_field(&fixture, "headless", "interaction", "\"interactive\"");
        }
        "stub" => set_field(&fixture, "headless", "args", "[\"--help\"]"),
        "unsupported" => {
            set_field(&fixture, "headless", "evidence", "\"unsupported\"");
            set_field(&fixture, "headless", "platforms", "[]");
        }
        other => panic!("unexpected support state {other}"),
    }
    let output = fixture.run(&["--plain", "run", "fixture", "headless"]);
    assert_eq!(output.status.code(), Some(4));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    assert!(String::from_utf8_lossy(&output.stderr).contains(support));
}

#[test]
fn non_executable_support_states_fail_before_gate_or_child() {
    for support in ["manual", "stub", "unsupported"] {
        assert_guarded(support);
    }
}

#[test]
fn stale_executable_claim_fails_closed_before_gate_or_child() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    set_field(
        &fixture,
        "download",
        "verified_at",
        "\"2025-01-01T00:00:00Z\"",
    );
    let output = fixture.run(&["--plain", "install", "fixture", "--dry-run"]);
    assert_ne!(output.status.code(), Some(0));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    assert!(String::from_utf8_lossy(&output.stderr).contains("fresh"));
}

#[test]
fn incompatible_platform_fails_before_gate_or_child() {
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    set_field(&fixture, "download", "platforms", "[\"windows-x64-msvc\"]");
    let output = fixture.run(&["--plain", "install", "fixture", "--dry-run"]);
    assert_eq!(output.status.code(), Some(4));
    assert!(!fixture.spawned());
    assert!(!fixture.gate_spawned());
    assert!(String::from_utf8_lossy(&output.stderr).contains("platform"));
}

fn set_field(fixture: &Fixture, capability: &str, key: &str, value: &str) {
    let path = fixture
        .root
        .join("catalog/fixture")
        .join(capability)
        .join("index.toml");
    let prefix = format!("{key} = ");
    let source = fs::read_to_string(&path).unwrap();
    assert!(source.lines().any(|line| line.starts_with(&prefix)));
    let updated = source
        .lines()
        .map(|line| {
            if line.starts_with(&prefix) {
                format!("{prefix}{value}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(path, format!("{updated}\n")).unwrap();
}
