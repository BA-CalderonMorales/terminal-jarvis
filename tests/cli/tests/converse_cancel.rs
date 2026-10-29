#![cfg(unix)]
use crate::logic::cli_driver::Fixture;
use crate::logic::tui::{launch, EXIT, FRAME, SERIAL};
use crate::logic::{pty, screen};

#[test]
fn cancelling_a_conversation_then_an_install_uses_the_same_input_owner() {
    let _serial = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let fixture = Fixture::new(
        "expected",
        "expected",
        concat!(
            "#!/bin/sh\n",
            "if [ \"$1\" = --version ]; then echo 1.0; exit 0; fi\n",
            "if [ $# -gt 0 ]; then echo conversation-child >&2; else echo install-child >&2; fi\n",
            "sleep 60\n"
        ),
    );
    fixture.run(&["--plain", "use", "fixture"]);
    let (_, bytes) = pty::run_pty_probe(
        launch(&fixture, &[("TJ_DEBUG_SIZE", "80x24")]),
        &[
            (b"", Some(FRAME)),
            (b"converse 1 fixture fixture topic\n", None),
            (b"", Some(b"again to accept")),
            (b"converse 1 fixture fixture topic\n", None),
            (b"", Some(b"conversation-child")),
            (b"\x03", None),
            (b"", Some(b"converse ended early")),
            (b"install fixture\n", None),
            (b"y\n", Some(b"Continue with download:fixture? [Y/n]")),
            (b"", Some(b"install-child")),
            (b"\x1b", None),
            (b"", Some(b"cancelled")),
            EXIT,
        ],
    );
    let painted = screen::render(&bytes);
    assert!(painted.contains("cancelled"));
}
