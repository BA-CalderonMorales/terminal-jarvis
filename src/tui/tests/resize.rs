//! Resize contract: the flag is level-triggered and consumed exactly once,
//! so a burst of SIGWINCHes collapses to one repaint per loop pass.

use super::{note, pending, take};

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
