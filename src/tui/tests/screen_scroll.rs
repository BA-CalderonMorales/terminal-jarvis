//! Scroll: the window math contract — offsets stay in bounds, moves clamp,
//! windows slice the newest lines, and the badge names what is hidden.

use super::*;
use crate::tui::input::Move;

#[test]
fn max_offset_is_the_hidden_history() {
    assert_eq!(max_offset(50, 10), 40);
    assert_eq!(max_offset(10, 10), 0);
    assert_eq!(max_offset(5, 10), 0, "oversized terminals pin to zero");
}

#[test]
fn clamp_never_escapes_the_bounds() {
    assert_eq!(clamp(5, 50, 10), 5);
    assert_eq!(clamp(999, 50, 10), 40);
    assert_eq!(clamp(0, 0, 10), 0);
}

#[test]
fn one_move_steps_one_line_and_clamps_both_ways() {
    assert_eq!(step(5, Move::Older, 50, 10), 6);
    assert_eq!(step(40, Move::Older, 50, 10), 40, "older clamps at the top");
    assert_eq!(step(5, Move::Newer, 50, 10), 4);
    assert_eq!(
        step(0, Move::Newer, 50, 10),
        0,
        "newer clamps at the bottom"
    );
}

#[test]
fn pages_move_by_the_visible_height_minus_two() {
    assert_eq!(step(0, Move::PageOlder, 50, 10), 8);
    assert_eq!(step(8, Move::PageNewer, 50, 10), 0);
    assert_eq!(step(2, Move::PageNewer, 50, 10), 0, "pages clamp");
}

#[test]
fn top_and_bottom_jump_to_the_ends() {
    assert_eq!(step(0, Move::Top, 50, 10), 40);
    assert_eq!(step(40, Move::Bottom, 50, 10), 0);
}

#[test]
fn window_shows_the_newest_lines_above_the_offset() {
    let lines: Vec<String> = (0..10).map(|i| format!("line {i}")).collect();
    assert_eq!(window(&lines, 0, 4), &lines[6..10]);
    assert_eq!(window(&lines, 6, 4), &lines[0..4]);
    assert!(window(&lines, 99, 4).len() == 4, "clamped, never panics");
}

#[test]
fn badge_names_the_hidden_sides_only() {
    assert_eq!(badge(0, 50, 10), "↑ 40");
    assert_eq!(badge(40, 50, 10), "↓ 40");
    assert_eq!(badge(20, 50, 10), "↑ 20 · ↓ 20");
    assert_eq!(badge(0, 4, 10), "", "nothing hidden, no badge");
}
