//! Keys: the editor contract — decoded keys map to edits, scroll moves,
//! submit, or session end; the buffer never splits a glyph.

use crate::tui::input::{Editor, Feed, Key, Move};

#[test]
fn chars_append_and_backspace_pops_whole_glyphs() {
    let mut editor = Editor::default();
    assert_eq!(editor.feed(Key::Char('h')), Feed::Edited);
    assert_eq!(editor.feed(Key::Char('i')), Feed::Edited);
    assert_eq!(editor.feed(Key::Backspace), Feed::Edited);
    assert_eq!(editor.feed(Key::Enter), Feed::Submit("h".into()));
}

#[test]
fn clear_line_empties_the_buffer_in_place() {
    let mut editor = Editor::default();
    for key in [Key::Char('a'), Key::Char('b'), Key::Char('c')] {
        editor.feed(key);
    }
    assert_eq!(editor.feed(Key::ClearLine), Feed::Edited);
    assert_eq!(editor.feed(Key::Enter), Feed::Submit(String::new()));
}

#[test]
fn navigation_keys_map_to_scroll_moves() {
    let mut editor = Editor::default();
    assert_eq!(editor.feed(Key::Up), Feed::Moved(Move::Older));
    assert_eq!(editor.feed(Key::Down), Feed::Moved(Move::Newer));
    assert_eq!(editor.feed(Key::PageUp), Feed::Moved(Move::PageOlder));
    assert_eq!(editor.feed(Key::PageDown), Feed::Moved(Move::PageNewer));
    assert_eq!(editor.feed(Key::Home), Feed::Moved(Move::Top));
    assert_eq!(editor.feed(Key::End), Feed::Moved(Move::Bottom));
}

#[test]
fn ignored_keys_never_touch_the_buffer() {
    let mut editor = Editor::default();
    editor.feed(Key::Char('x'));
    assert_eq!(editor.feed(Key::Ignored), Feed::Idle);
    assert_eq!(editor.feed(Key::Dead), Feed::Dead);
    assert_eq!(editor.feed(Key::Enter), Feed::Submit("x".into()));
}

#[test]
fn tail_view_keeps_the_newest_glyphs_inside_the_budget() {
    let mut editor = Editor::default();
    for key in ['a', 'b', 'c', 'd', 'e'].map(Key::Char) {
        editor.feed(key);
    }
    // A budget of 3 cells can only show "cde" -- the cursor-anchored tail.
    assert_eq!(editor.tail_view(0, 3), "cde");
    assert_eq!(editor.tail_view(0, 10), "abcde");
    // A wide prefix eats into the budget before any glyph renders.
    assert_eq!(editor.tail_view(8, 3), "");
}
