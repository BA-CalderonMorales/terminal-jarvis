//! LiveScroll: navigation keys move the shared offset so turns and the
//! prompt scroll one view -- j/k row-step, g/G the ends, paging by bubble,
//! and anything else is not a scroll. The offset counts lines hidden
//! below the window: zero is the newest row, max is the oldest.

use super::navigate;
use crate::tui::input::Key;

fn body() -> Vec<String> {
    (0..60).map(|row| format!("row {row}")).collect()
}

#[test]
fn j_and_k_step_rows_while_g_and_g_jump_the_ends() {
    let body = body();
    let mut offset = 40usize;
    assert!(navigate(&Key::Char('j'), &mut offset, &body, 20));
    assert_eq!(offset, 39, "j walks toward the newest row");
    assert!(navigate(&Key::Char('k'), &mut offset, &body, 20));
    assert_eq!(offset, 40, "k walks back into history, clamped");
    assert!(navigate(&Key::Char('g'), &mut offset, &body, 20));
    assert_eq!(offset, 0);
    assert!(navigate(&Key::Char('G'), &mut offset, &body, 20));
    assert_eq!(offset, 40);
}

#[test]
fn paging_moves_bubble_to_bubble_and_arrows_step() {
    let mut body = body();
    body[20] = "╭─ hermes".into();
    body[30] = "╭─ opencode".into();
    let mut offset = 20usize;
    assert!(navigate(&Key::Down, &mut offset, &body, 20));
    assert_eq!(offset, 19);
    assert!(navigate(&Key::PageDown, &mut offset, &body, 20));
    assert_eq!(offset, 20, "the hermes bubble start");
    assert!(navigate(&Key::PageDown, &mut offset, &body, 20));
    assert_eq!(offset, 30, "the next bubble start wins");
    assert!(navigate(&Key::PageUp, &mut offset, &body, 20));
    assert_eq!(offset, 20);
}

#[test]
fn keys_that_are_not_scrolls_never_move_the_view() {
    let body = body();
    let mut offset = 10usize;
    for key in [Key::Char('i'), Key::Escape, Key::Enter, Key::Char('x')] {
        assert!(!navigate(&key, &mut offset, &body, 20));
    }
    assert_eq!(offset, 10);
}
