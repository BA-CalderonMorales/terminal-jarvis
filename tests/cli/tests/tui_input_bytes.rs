#![cfg(unix)]
use crate::logic::frames::{frames, rows};
use crate::logic::pty;
use crate::logic::tui::{booted, launch, EXIT, FRAME, SERIAL};

fn prompt_after(bytes: &[u8]) -> String {
    let fixture = booted();
    let (status, output) = pty::run_pty_probe(
        launch(&fixture, &[]),
        &[(b"", Some(FRAME)), (bytes, None), EXIT],
    );
    assert!(status.success(), "terminal input crashed the application");
    let painted = frames(&output);
    rows(painted.last().unwrap())
        .into_iter()
        .find(|line| line.contains("[>_]::"))
        .unwrap()
}

#[test]
fn real_page_keys_consume_the_terminator_without_typing_it() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    for bytes in [b"\x1b[5~x", b"\x1b[6~x"] {
        let prompt = prompt_after(bytes);
        assert!(prompt.trim_end().ends_with(": x"), "{prompt}");
    }
}

#[test]
fn real_tab_completes_ascii_and_unicode_input() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    for (bytes, expected) in [
        (b"instal\t".as_slice(), ": install"),
        (b"run\xc2\xa0fi\t".as_slice(), ": run\u{00a0}fixture"),
    ] {
        let prompt = prompt_after(bytes);
        assert!(prompt.trim_end().ends_with(expected), "{prompt}");
    }
}
