#[path = "phase03_cli_fixture/mod.rs"]
pub mod fixture;

use fixture::{text, Fixture, State};

fn success(fixture: &Fixture, args: &[&str]) -> String {
    let output = fixture.run(args);
    assert_eq!(output.status.code(), Some(0), "args: {args:?}");
    assert!(output.stderr.is_empty(), "args: {args:?}");
    text(&output.stdout)
}

#[test]
fn top_level_help_forms_are_identical() {
    let fixture = Fixture::new(State::Unknown);
    let expected = success(&fixture, &["--plain"]);
    for args in [
        &["--plain", "help"][..],
        &["--plain", "--help"],
        &["--plain", "-h"],
    ] {
        assert_eq!(success(&fixture, args), expected, "args: {args:?}");
    }
}

#[test]
fn every_named_command_has_matching_help_forms() {
    let fixture = Fixture::new(State::Unknown);
    let commands = [
        "help",
        "version",
        "list",
        "check",
        "current",
        "use",
        "show",
        "plan",
        "run",
        "install",
        "update",
        "auth",
        "config",
        "cache",
        "security",
        "gate",
        "experimental",
        "templates",
        "db",
    ];
    for command in commands {
        let expected = success(&fixture, &["--plain", "help", command]);
        for flag in ["--help", "-h"] {
            let actual = success(&fixture, &["--plain", command, flag]);
            assert_eq!(actual, expected, "command: {command}, flag: {flag}");
        }
    }
}

#[test]
fn alias_help_resolves_to_the_canonical_command() {
    let fixture = Fixture::new(State::Unknown);
    for (alias, canonical) in [("tools", "list"), ("status", "check"), ("info", "show")] {
        let expected = success(&fixture, &["--plain", "help", canonical]);
        assert_eq!(success(&fixture, &["--plain", "help", alias]), expected);
        assert_eq!(success(&fixture, &["--plain", alias, "--help"]), expected);
        assert_eq!(success(&fixture, &["--plain", alias, "-h"]), expected);
    }
}
