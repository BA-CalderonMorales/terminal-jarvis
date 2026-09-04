#![cfg(unix)] // PTY-driven (openpty); no Windows equivalent in-tree.
use crate::logic::tui::{booted, launch, EXIT, FRAME, SERIAL};
use crate::logic::{pty, screen};
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[test]
fn the_badge_shows_the_active_harness_and_version() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = booted();
    let (status, bytes) = pty::run_pty_probe(launch(&fixture, &[]), &[(b"", Some(FRAME)), EXIT]);
    assert_eq!(
        status.code(),
        Some(0),
        "stream: {:?}",
        screen::render(&bytes).displayed()
    );
    let screen = screen::render(&bytes);
    let badge = format!("[>_]::[tj:{VERSION}]::[harness:fixture]:");
    assert!(
        screen.contains(&badge),
        "badge missing in:\n{}",
        screen.lines().join("\n")
    );
    assert!(
        screen
            .lines()
            .iter()
            .any(|line| line.contains("Terminal Jarvis") && line.contains("ACTIVE")),
        "the status row belongs on the title line:\n{}",
        screen.lines().join("\n")
    );
}

#[test]
fn idle_ctrl_c_keeps_the_session_and_typing_clean() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = booted();
    let (status, bytes) = pty::run_pty_probe(
        launch(&fixture, &[]),
        &[
            (b"", Some(FRAME)),
            (b"\x03", None),
            (b"", Some(FRAME)),
            (b"status\n", None),
            (b"", Some(b"ready".as_slice())),
            EXIT,
        ],
    );
    assert_eq!(status.code(), Some(0), "idle Ctrl+C must not kill the tui");
    let screen = screen::render(&bytes);
    assert!(
        screen.contains("[>_]::[tj:"),
        "the prompt did not survive Ctrl+C:\n{}",
        screen.displayed()
    );
}
#[test]
fn clean_install_answers_with_a_verdict_and_adopts_the_harness() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = booted();
    let (_, bytes) = pty::run_pty_probe(
        launch(&fixture, &[]),
        &[
            (b"", Some(FRAME)),
            (b"install fixture\n", None),
            (
                b"y\n",
                Some(b"Continue with download:fixture? [y/N]".as_slice()),
            ),
            (b"", Some(b"installed fixture".as_slice())),
            EXIT,
        ],
    );
    let screen = screen::render(&bytes);
    assert!(
        screen.no_line_contains("running security gate"),
        "gate narration leaked into the clean view:\n{}",
        screen.lines().join("\n")
    );
    assert!(
        screen.no_line_contains("installing fixture: fixture-child"),
        "install command line leaked:\n{}",
        screen.lines().join("\n")
    );
    assert!(
        screen.contains("security scan (acceptance): passed"),
        "the scan's role is invisible in the clean view:\n{}",
        screen.lines().join("\n")
    );
    assert!(
        screen.contains("installed fixture"),
        "no verdict:\n{}",
        screen.lines().join("\n")
    );
    assert!(
        screen.contains("now active"),
        "install did not adopt the harness:\n{}",
        screen.lines().join("\n")
    );
}
