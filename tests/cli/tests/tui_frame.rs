#![cfg(unix)] // PTY-driven: see tui_acceptance.rs for why this is Unix-only.

use crate::logic::cli_driver::Fixture;
use crate::logic::frames::{frames, height, rows};
use crate::logic::pty;
use crate::logic::tui::{launch, EXIT, FRAME, SERIAL};

/// An install must never paint a frame taller than the terminal -- that scrolls
/// it and walks the prompt up the screen -- and a blank log line must stay
/// blank rather than becoming an `info` row that reads as content.
#[test]
fn an_install_keeps_the_frame_inside_the_terminal_and_blank_lines_blank() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let fixture = Fixture::new(
        "expected",
        "expected",
        "#!/bin/sh\n: > \"$TJ_FIXTURE_MARKER\"\necho 'first line'\necho\necho 'after a blank'\n",
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
            (b"", Some(b"installed fixture".as_slice())),
            (b"", None),
            EXIT,
        ],
    );

    let painted = frames(&bytes);
    assert!(painted.len() > 5, "saw only {} frames", painted.len());
    for (index, frame) in painted.iter().enumerate() {
        assert_eq!(
            height(frame),
            24,
            "frame {index} outgrew the 24-row terminal"
        );
    }

    let last = rows(painted.last().expect("a final frame"));
    let first = last
        .iter()
        .position(|row| row.contains("first line"))
        .unwrap_or_else(|| panic!("no log row in {last:?}"));
    let after = last
        .iter()
        .position(|row| row.contains("after a blank"))
        .unwrap_or_else(|| panic!("no second log row in {last:?}"));
    assert_eq!(
        after,
        first + 2,
        "expected one blank row between the lines: {last:?}"
    );
    assert!(
        last[first + 1].trim().is_empty(),
        "the blank line became a labelled row: {:?}",
        last[first + 1]
    );
}
