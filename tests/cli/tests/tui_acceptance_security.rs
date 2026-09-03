#![cfg(unix)]
// PTY-driven: see tui_acceptance.rs for why this is Unix-only.

use crate::logic::tui::{booted, launch, EXIT, FRAME, SERIAL};
use crate::logic::{pty, screen};

#[test]
fn run_after_a_passed_scan_skips_the_scan_again() {
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
            (b"run fixture ui\n", None),
            (b"y\n", Some(b"Continue with ui:fixture? [y/N]".as_slice())),
            (b"", Some(b"exited 0".as_slice())),
            EXIT,
        ],
    );
    let screen = screen::render(&bytes);
    let scans = screen
        .lines()
        .iter()
        .filter(|line| line.contains("security scan (acceptance)"))
        .count();
    assert!(
        scalene(scans),
        "expected exactly one scan for install+run, saw {scans}:\n{}",
        screen.displayed()
    );
}

fn scalene(count: usize) -> bool {
    count == 1
}

#[test]
fn debug_toggle_restores_the_verbose_view_on_demand() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = booted();
    let (_, bytes) = pty::run_pty_probe(
        launch(&fixture, &[]),
        &[
            (b"", Some(FRAME)),
            (b"/debug on\n", None),
            (b"install fixture\n", Some(b"debug view on".as_slice())),
            (
                b"y\n",
                Some(b"Continue with download:fixture? [y/N]".as_slice()),
            ),
            (b"", Some(b"installing fixture: fixture-child".as_slice())),
            (b"/debug off\n", None),
            (b"", Some(b"debug view off".as_slice())),
            EXIT,
        ],
    );
    let screen = screen::render(&bytes);
    assert!(
        screen.contains("running security gate"),
        "debug view did not restore narration:\n{}",
        screen.displayed()
    );
}
