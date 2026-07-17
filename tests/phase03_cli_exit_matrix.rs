#[path = "phase03_cli_fixture/mod.rs"]
pub mod fixture;

use fixture::{assert_json_document, text, Fixture, State};

fn assert_plain_error(fixture: &Fixture, args: &[&str], code: i32, needle: &str) {
    let output = fixture.run(args);
    assert_eq!(output.status.code(), Some(code), "args: {args:?}");
    assert!(output.stdout.is_empty(), "args: {args:?}");
    let error = text(&output.stderr);
    assert!(error.contains(needle), "args: {args:?}: {error}");
    assert!(!error.contains("\u{1b}["));
}

#[test]
fn terminal_jarvis_exit_classes_use_the_contract_streams() {
    let unknown = Fixture::new(State::Unknown);
    assert_plain_error(&unknown, &["--plain", "list", "extra"], 2, "error:");

    let mut missing = unknown.command();
    let output = missing
        .args(["--plain", "list"])
        .env(
            "TERMINAL_JARVIS_CATALOG",
            unknown.root().join("missing-catalog"),
        )
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty() && !output.stderr.is_empty());

    assert_plain_error(&unknown, &["--plain", "templates"], 4, "was removed");
    let expected = Fixture::new(State::Expected);
    assert_plain_error(
        &expected,
        &["--plain", "install", "fixture"],
        5,
        "noninteractive execution",
    );
}

#[test]
fn handled_json_exit_classes_emit_one_document_and_no_stderr() {
    let fixture = Fixture::new(State::Unknown);
    assert_json_document(&fixture.run(&["--json", "list", "extra"]), 2);

    let mut missing = fixture.command();
    let state = missing
        .args(["--json", "list"])
        .env(
            "TERMINAL_JARVIS_CATALOG",
            fixture.root().join("missing-catalog"),
        )
        .output()
        .unwrap();
    assert_json_document(&state, 3);
    assert_json_document(&fixture.run(&["--json", "templates"]), 4);
    assert_json_document(
        &fixture.run(&["--json", "gate", "enable", "missing-gate"]),
        5,
    );
    let check = assert_json_document(&fixture.run(&["--json", "check"]), 4);
    assert!(check.contains("\"command\":\"check\""));
}

#[test]
fn missing_child_preserves_exit_127_on_stderr() {
    let fixture = Fixture::new(State::Expected);
    assert_plain_error(
        &fixture,
        &["--plain", "run", "fixture", "headless"],
        127,
        "was not found on PATH",
    );
}

#[cfg(unix)]
#[test]
fn non_executable_child_preserves_exit_126_on_stderr() {
    let fixture = Fixture::new(State::Expected);
    fixture.child(false);
    assert_plain_error(
        &fixture,
        &["--plain", "run", "fixture", "headless"],
        126,
        "is not executable",
    );
}
