//! Resize contract: the flag is level-triggered and consumed exactly once,
//! so a burst of SIGWINCHes collapses to one repaint per loop pass; and a
//! frame clears exactly once per size change, so a shrink leaves no tail.

use super::{note, pending, settle_at, take};

#[test]
fn a_resize_is_pending_until_it_is_taken() {
    let _ = take();
    assert!(!pending(), "no resize outstanding");
    note();
    assert!(pending(), "the event is level-triggered");
    assert!(take(), "the first take consumes it");
    assert!(!take(), "the second take sees nothing");
    assert!(!pending(), "and it stays consumed");
}

#[test]
fn a_frame_clears_once_per_size_change_and_never_twice() {
    let mut last = None;
    let first = settle_at("frame".into(), &mut last, 80, 24);
    assert!(
        first.starts_with("\x1b[2J"),
        "the first frame clears: the alternate screen may still hold a previous session: {first}"
    );
    let same = settle_at("frame".into(), &mut last, 80, 24);
    assert_eq!(same, "frame", "a steady repaint overwrites in place");
    let shrunk = settle_at("frame".into(), &mut last, 60, 24);
    assert_eq!(
        shrunk, "\x1b[2Jframe",
        "a shrink clears first, so the last two columns of the wider frame go"
    );
    let settled = settle_at("frame".into(), &mut last, 60, 24);
    assert_eq!(settled, "frame", "and only once");
    let grown = settle_at("frame".into(), &mut last, 100, 40);
    assert!(grown.starts_with("\x1b[2J"), "a grow clears too: {grown}");
}
