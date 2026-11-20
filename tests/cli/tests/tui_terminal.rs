#![cfg(unix)]
use crate::logic::cli_driver::Fixture;
use crate::logic::pty;
use crate::logic::tui::{launch, EXIT, FRAME, SERIAL};

#[test]
fn interactive_forms_give_the_child_a_real_terminal() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    for command in [
        b"run fixture\n".as_slice(),
        b"run fixture ui\n",
        b"fixture --help\n",
    ] {
        let fixture = Fixture::new(
            "expected",
            "expected",
            concat!(
                "#!/bin/sh\n",
                "if [ -t 0 ] && [ -t 1 ]; then result=terminal; else result=piped; fi\n",
                "printf '%s' \"$result\" > \"$TJ_FIXTURE_MARKER\"\n",
                "echo terminal-child-finished\n",
            ),
        );
        fixture.run(&["--plain", "use", "fixture"]);
        let (status, _) = pty::run_pty_probe(
            launch(&fixture, &[]),
            &[
                (b"", Some(FRAME)),
                (command, None),
                (b"", Some(b"terminal-child-finished".as_slice())),
                EXIT,
            ],
        );
        assert_eq!(status.code(), Some(0));
        assert_eq!(
            std::fs::read_to_string(fixture.marker_path()).unwrap(),
            "terminal"
        );
    }
}
