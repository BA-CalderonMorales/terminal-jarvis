#![cfg(unix)]

#[path = "phase03_cli_fixture/mod.rs"]
pub mod fixture;

use fixture::{run_pty, Fixture, State};
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
