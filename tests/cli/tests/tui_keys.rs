#![cfg(unix)] // PTY-driven: see tui_acceptance.rs for why this is Unix-only.

use crate::logic::frames::{frames, rows};
use crate::logic::pty;
use crate::logic::tui::{booted, launch, EXIT, FRAME, SERIAL};

/// The last frame the tui painted after `steps`, each separated in time so the
/// escape resolver sees a bare Esc rather than an Alt combo.
fn frame_after(steps: &[&[u8]]) -> Vec<String> {
    let fixture = booted();
    let mut stages: Vec<(&[u8], Option<&[u8]>)> = vec![(b"", Some(FRAME))];
    for step in steps {
        stages.push((step, None));
        stages.push((b"", None));
    }
    stages.push(EXIT);
    let (_, bytes) = pty::run_pty_probe(
        launch(
            &fixture,
            &[("TERM", "xterm-256color"), ("TJ_DEBUG_SIZE", "80x24")],
        ),
        &stages,
    );
    let painted = frames(&bytes);
    rows(painted.last().expect("a final frame"))
}

fn prompt(screen: &[String]) -> String {
    screen
        .iter()
        .find(|row| row.contains("[>_]::"))
        .cloned()
        .unwrap_or_default()
}

/// Esc is the vim mode switch, so it must be exactly one keypress: a second
/// press in quick succession must not swallow the first (the escape resolver
/// used to treat `Esc Esc` as an Alt combo and eat both bytes).
#[test]
fn esc_reaches_normal_mode_and_a_double_press_is_not_swallowed() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    for input in [&b"\x1b"[..], &b"\x1b\x1b"[..]] {
        let screen = frame_after(&[input]);
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
    let screen = frame_after(&[b"\x1bq"]);
    assert!(
        screen
            .iter()
            .any(|row| row.contains("[harness:fixture]: q")),
        "the combo key was eaten: {screen:?}"
    );
}

/// Normal mode must have a way back. It was a trap: nothing typed there
/// reached the line and no key returned to Insert, so the prompt was dead
/// until the session restarted.
#[test]
fn normal_mode_has_a_way_back_to_insert() {
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());
    for way_out in [&b"i"[..], &b"\x1b"[..]] {
        let screen = frame_after(&[b"\x1b", way_out, b"hello"]);
        assert!(
            prompt(&screen).contains("hello"),
            "{way_out:?} did not leave Normal mode: {:?}",
            prompt(&screen)
        );
    }
    // and the frame says how, instead of leaving the user to guess
    let normal = frame_after(&[b"\x1b"]);
    assert!(
        normal.iter().any(|row| row.contains("esc or i to type")),
        "Normal mode does not say the way out: {normal:?}"
    );
}
