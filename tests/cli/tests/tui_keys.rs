#![cfg(unix)] // PTY-driven: see tui_acceptance.rs for why this is Unix-only.

use crate::logic::frames::{frames, rows};
use crate::logic::pty;
use crate::logic::tui::{booted, launch, EXIT, FRAME, SERIAL};

/// The last frame the tui painted after `input`.
fn last_frame_rows(input: &[u8]) -> Vec<String> {
    let fixture = booted();
    let (_, bytes) = pty::run_pty_probe(
        launch(
            &fixture,
            &[("TERM", "xterm-256color"), ("TJ_DEBUG_SIZE", "80x24")],
        ),
        &[(b"", Some(FRAME)), (input, None), (b"", None), EXIT],
    );
    let painted = frames(&bytes);
    rows(painted.last().expect("a final frame"))
}

/// Esc is the vim mode switch, so it must be exactly one keypress: a second
/// press in quick succession must not swallow the first (the escape resolver
/// used to treat `Esc Esc` as an Alt combo and eat both bytes).
#[test]
fn esc_reaches_normal_mode_and_a_double_press_is_not_swallowed() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    for input in [&b"\x1b"[..], &b"\x1b\x1b"[..]] {
        let screen = last_frame_rows(input);
        assert!(
            screen.iter().any(|row| row.contains("-- NORMAL --")),
            "{input:?} did not reach Normal mode: {screen:?}"
        );
    }
}

/// An Alt combo replays its key: the Esc is ignored, but the key after it is
/// not -- the resolver's replay used to decode to `Ignored` and eat the key.
#[test]
fn an_alt_combo_replays_its_key() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    let screen = last_frame_rows(b"\x1bq");
    assert!(
        screen
            .iter()
            .any(|row| row.contains("[harness:fixture]: q")),
        "the combo key was eaten: {screen:?}"
    );
}
