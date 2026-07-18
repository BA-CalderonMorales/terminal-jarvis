#![cfg(unix)]

#[path = "phase03_cli_fixture/mod.rs"]
pub mod fixture;

use fixture::{run_pty, run_pty_input, text, Fixture, State};
use std::process::Command;

fn tty(mut command: Command, args: &[&str]) -> Vec<u8> {
    command.args(args);
    let (status, output) = run_pty(command);
    assert_eq!(status.code(), Some(0), "args: {args:?}");
    output
}

#[test]
fn tty_rich_mode_colors_and_every_colorless_mode_disables_ansi() {
    let fixture = Fixture::new(State::Unknown);
    let rich = tty(fixture.command(), &["list"]);
    assert!(rich.windows(2).any(|pair| pair == b"\x1b["));

    for args in [
        &["--plain", "list"][..],
        &["--json", "list"],
        &["--no-color", "list"],
    ] {
        let body = tty(fixture.command(), args);
        assert!(!body.windows(2).any(|pair| pair == b"\x1b["));
    }
    for (name, value) in [("NO_COLOR", "1"), ("TERM", "dumb")] {
        let mut command = fixture.command();
        command.env(name, value);
        let body = tty(command, &["list"]);
        assert!(!body.windows(2).any(|pair| pair == b"\x1b["));
    }
}

#[test]
fn ci_tty_follows_the_frozen_destination_stream_color_rule() {
    let fixture = Fixture::new(State::Unknown);
    let mut command = fixture.command();
    command.env("CI", "1");
    let body = tty(command, &["list"]);
    assert!(body.windows(2).any(|pair| pair == b"\x1b["));
}

fn lifecycle(input: &[u8]) -> (std::process::ExitStatus, String) {
    let fixture = Fixture::new(State::Expected);
    fixture.child(true);
    let mut command = fixture.command();
    command.args(["--plain", "install", "fixture"]);
    let (status, output) = run_pty_input(command, input);
    (status, text(&output))
}

#[test]
fn lifecycle_pty_confirmation_executes_the_visible_plan() {
    let (status, output) = lifecycle(b"yes\n");
    assert_eq!(status.code(), Some(0));
    assert!(output.contains("fixture:download"));
    assert!(output.contains("command: fixture-child"));
    assert!(output.contains("Continue with download:fixture? [y/N]"));
}

#[test]
fn lifecycle_pty_rejection_fails_closed() {
    let (status, output) = lifecycle(b"no\n");
    assert_eq!(status.code(), Some(5));
    assert!(output.contains("operation was not confirmed"));
    assert!(output.contains("review the plan and retry when ready"));
}

#[test]
fn lifecycle_pty_eof_cancellation_fails_closed() {
    let (status, output) = lifecycle(b"\x04");
    assert_eq!(status.code(), Some(5));
    assert!(output.contains("operation was not confirmed"));
}
