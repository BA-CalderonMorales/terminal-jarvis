#[path = "phase03_cli_fixture/mod.rs"]
pub mod fixture;

use fixture::{assert_json_document, text, Fixture, State};

#[test]
fn every_named_surface_rejects_representative_unconsumed_input() {
    let fixture = Fixture::new(State::Unknown);
    let cases: &[&[&str]] = &[
        &["help", "list", "extra"],
        &["help", "unknown"],
        &["version", "extra"],
        &["version", "--wat"],
        &["list", "extra"],
        &["check", "extra"],
        &["current", "extra"],
        &["use"],
        &["use", "fixture", "extra"],
        &["show"],
        &["show", "fixture", "extra"],
        &["plan"],
        &["plan", "fixture", "not-a-capability"],
        &["plan", "fixture", "headless", "extra"],
        &["run", "fixture", "--wat"],
        &["install"],
        &["install", "fixture", "extra"],
        &["update", "fixture", "extra"],
        &["auth", "set", "fixture", "extra"],
        &["config", "unknown"],
        &["cache", "unknown"],
        &["security", "status", "extra"],
        &["gate", "enable", "trivy", "extra"],
        &["experimental"],
        &["experimental", "wrong"],
        &["experimental", "dashboard", "extra"],
        &["templates", "extra"],
        &["db", "extra"],
        &["list", "--", "child"],
        &["--"],
        &["--wat"],
    ];
    for args in cases {
        let mut full = vec!["--plain"];
        full.extend_from_slice(args);
        let output = fixture.run(&full);
        assert_eq!(output.status.code(), Some(2), "args: {args:?}");
        assert!(output.stdout.is_empty(), "args: {args:?}");
        let error = text(&output.stderr);
        assert!(error.starts_with("error: "), "args: {args:?}: {error}");
        assert!(!error.contains("\u{1b}["));
    }
}

#[test]
fn option_validation_rejects_conflicts_duplicates_and_wrong_scope() {
    let fixture = Fixture::new(State::Unknown);
    let cases: &[&[&str]] = &[
        &["--plain", "list", "--json"],
        &["--confirm=bad", "install", "fixture"],
        &[
            "--confirm=download:fixture",
            "install",
            "fixture",
            "--confirm=download:fixture",
        ],
        &["--verbose", "list"],
        &["--dry-run", "list"],
        &["--json", "run", "fixture", "headless"],
    ];
    for args in cases {
        let output = fixture.run(args);
        assert_eq!(output.status.code(), Some(2), "args: {args:?}");
    }
}

#[test]
fn json_parse_errors_are_single_documents_on_stdout() {
    let fixture = Fixture::new(State::Unknown);
    for args in [
        &["--json", "list", "extra"][..],
        &["--plain", "list", "--json"],
        &["--json", "run", "fixture", "--wat"],
    ] {
        let body = assert_json_document(&fixture.run(args), 2);
        assert!(body.contains("\"command\":\"parse\""));
        assert!(body.contains("\"code\":\"usage\""));
    }
}
