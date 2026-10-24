#![cfg(unix)] // PTY-driven: see tui_acceptance.rs for why this is Unix-only.

use crate::logic::cli_driver::Fixture;
use crate::logic::frames::{frames, rows};
use crate::logic::golden;
use crate::logic::pty;
use crate::logic::tui::{launch, EXIT, FRAME, MARKER_SCRIPT, SERIAL};

/// One screen, painted and settled: the last frame the tui drew.
///
/// The probe waits for a marker *before* it writes the stage's input, so the
/// boot marker is its own stage and the command marker follows the command.
fn screen(fixture: &Fixture, command: &str, marker: &[u8]) -> Vec<String> {
    let command = format!("{command}\n");
    let (_, bytes) = pty::run_pty_probe(
        launch(
            fixture,
            &[("TERM", "xterm-256color"), ("TJ_DEBUG_SIZE", "80x24")],
        ),
        &[
            (b"", Some(FRAME)),
            (command.as_bytes(), None),
            (b"", Some(marker)),
            (b"", None),
            EXIT,
        ],
    );
    let painted = frames(&bytes);
    rows(painted.last().expect("a final frame"))
}

/// The screens a person sees, pinned as text.
///
/// A layout is a decision, so it gets a committed frame the way the site gets a
/// screenshot: the diff says what moved, and changing a frame has to be
/// deliberate. Re-record with `TJ_UPDATE_GOLDEN=1`.
#[test]
fn the_screens_are_pinned() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = Fixture::new("expected", "expected", MARKER_SCRIPT);
    fixture.run(&["--plain", "use", "fixture"]);
    let root = fixture.root.display().to_string();

    golden::check(
        "home",
        &screen(&fixture, "home", b"the command center"),
        &root,
    );
    golden::check(
        "list",
        &screen(&fixture, "list", b"Available Harnesses"),
        &root,
    );
    golden::check("show", &screen(&fixture, "show fixture", b"binary"), &root);
    golden::check(
        "plan",
        &screen(&fixture, "plan fixture headless", b"Plan: fixture"),
        &root,
    );
    golden::check(
        "status",
        &screen(&fixture, "status", b"one install away"),
        &root,
    );
}
