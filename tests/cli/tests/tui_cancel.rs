#![cfg(unix)] // PTY-driven: see tui_acceptance.rs for why this is Unix-only.

use crate::logic::cli_driver::Fixture;
use crate::logic::tui::{launch, EXIT, FRAME, SERIAL};
use crate::logic::{pty, screen};

#[test]
fn ctrl_c_cancels_a_running_install_and_returns_the_prompt() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    // A slow fixture install: the stream is still running when Ctrl+C lands.
    let fixture = Fixture::new(
        "expected",
        "expected",
        "#!/bin/sh\necho working\nsleep 60\n",
    );
    fixture.run(&["--plain", "use", "fixture"]);
    let (_, bytes) = pty::run_pty_probe(
        launch(
            &fixture,
            &[("TERM", "xterm-256color"), ("TJ_DEBUG_SIZE", "80x24")],
        ),
        &[
            (b"", Some(FRAME)),
            (b"install fixture\n", None),
            (
                b"y\n",
                Some(b"Continue with download:fixture? [Y/n]".as_slice()),
            ),
            (b"", Some(b"working".as_slice())),
            (b"\x03", None),
            (b"", Some(b"cancelled".as_slice())),
            EXIT,
        ],
    );
    let screen = screen::render(&bytes);
    assert!(
        screen.contains("cancelled"),
        "cancel was not reported:\n{}",
        screen.displayed()
    );
    assert!(
        screen.contains("[>_]::[tj:"),
        "the prompt did not come back:\n{}",
        screen.displayed()
    );
    assert!(
        screen.contains("esc or ctrl-c"),
        "the cancel affordance was never shown:\n{}",
        screen.displayed()
    );
}
