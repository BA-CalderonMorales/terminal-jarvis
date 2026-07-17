#[path = "phase03_cli_fixture/mod.rs"]
pub mod fixture;

use fixture::{text, Fixture, State};
use std::process::Output;

fn assert_data(output: Output, code: i32) {
    assert_eq!(output.status.code(), Some(code));
    assert!(!output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}

fn assert_error(output: Output, code: i32) {
    assert_eq!(output.status.code(), Some(code));
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

fn same(left: Output, right: Output) {
    assert_eq!(left.status.code(), right.status.code());
    assert_eq!(left.stdout, right.stdout);
    assert_eq!(left.stderr, right.stderr);
}

#[test]
fn every_canonical_surface_has_a_deterministic_safe_outcome() {
    let fixture = Fixture::new(State::Unknown);
    for args in [
        &["--plain", "version"][..],
        &["--plain", "list"],
        &["--plain", "current"],
        &["--plain", "show", "fixture"],
        &["--plain", "plan", "fixture", "headless"],
        &["--plain", "update"],
        &["--plain", "auth"],
        &["--plain", "config", "show"],
        &["--plain", "cache", "status"],
        &["--plain", "security", "status"],
        &["--plain", "gate", "status"],
        &["--plain", "--update", "--dry-run"],
    ] {
        assert_data(fixture.run(args), 0);
    }
    assert_data(fixture.run(&["--plain", "check"]), 4);
    assert_data(fixture.run(&["--plain", "use", "fixture"]), 0);
    assert!(text(&fixture.run(&["--plain", "current"]).stdout).contains("fixture"));
    for args in [
        &["--plain", "run", "fixture", "headless"][..],
        &["--plain", "install", "fixture", "--dry-run"],
        &["--plain", "update", "fixture", "--dry-run"],
        &["--plain", "experimental", "dashboard"],
        &["--plain", "fixture", "--help"],
        &["--plain", "templates"],
        &["--plain", "db"],
    ] {
        assert_error(fixture.run(args), 4);
    }
}

#[test]
fn compatibility_aliases_match_their_canonical_routes() {
    let fixture = Fixture::new(State::Unknown);
    same(
        fixture.run(&["--plain", "tools"]),
        fixture.run(&["--plain", "list"]),
    );
    same(
        fixture.run(&["--plain", "status"]),
        fixture.run(&["--plain", "check"]),
    );
    same(
        fixture.run(&["--plain", "info", "fixture"]),
        fixture.run(&["--plain", "show", "fixture"]),
    );
    same(
        fixture.run(&["--plain", "install", "fixture", "--dry-run"]),
        fixture.run(&["--plain", "run", "fixture", "download", "--dry-run"]),
    );
    same(
        fixture.run(&["--plain", "update", "fixture", "--dry-run"]),
        fixture.run(&["--plain", "run", "fixture", "update", "--dry-run"]),
    );
    same(
        fixture.run(&["--plain", "fixture", "--help"]),
        fixture.run(&["--plain", "run", "fixture", "ui", "--", "--help"]),
    );
}
