use super::fixture::*;
use super::*;

#[test]
fn arrows_move_the_cursor_and_clamp_at_the_edges() {
    let harnesses = fleet();
    let mut picker = picker(&harnesses);
    picker.step(Key::Up);
    assert_eq!(picker.cursor, 0);
    for _ in 0..3 {
        picker.step(Key::Down);
    }
    assert_eq!(picker.cursor, 2);
    assert_eq!(
        picker.selected().map(|harness| harness.name.as_str()),
        Some("gamma")
    );
    picker.step(Key::PageUp);
    assert_eq!(picker.cursor, 0);
}

#[test]
fn typing_narrows_live_and_resets_the_cursor() {
    let harnesses = fleet();
    let mut picker = picker(&harnesses);
    picker.step(Key::Down);
    for character in "eta".chars() {
        picker.step(Key::Char(character));
    }
    assert_eq!(picker.filter, "eta");
    assert_eq!(picker.cursor, 0);
    assert_eq!(picker.shown().len(), 1);
    assert_eq!(select_index(&mut picker, Key::Enter), Some(1));
}

#[test]
fn a_digit_jumps_by_picker_position_only_while_unfiltered() {
    let harnesses = fleet();
    let mut picker = picker(&harnesses);
    assert_eq!(select_index(&mut picker, Key::Char('3')), Some(2));
    picker.step(Key::Char('a'));
    assert!(matches!(picker.step(Key::Char('2')), Step::Continue));
    assert_eq!(picker.filter, "a2");
}

#[test]
fn backspace_and_clear_edit_the_filter() {
    let harnesses = fleet();
    let mut picker = picker(&harnesses);
    picker.step(Key::Char('b'));
    picker.step(Key::Char('e'));
    picker.step(Key::Backspace);
    assert_eq!(picker.filter, "b");
    picker.step(Key::ClearLine);
    assert!(picker.filter.is_empty());
    assert_eq!(picker.shown().len(), 3);
}

#[test]
fn escape_and_eof_close_and_an_empty_enter_continues() {
    let harnesses = fleet();
    let mut picker = picker(&harnesses);
    for character in "zzz".chars() {
        picker.step(Key::Char(character));
    }
    assert!(matches!(picker.step(Key::Enter), Step::Continue));
    assert!(matches!(picker.step(Key::Escape), Step::Close));
    assert!(matches!(picker.step(Key::Dead), Step::Close));
    assert!(matches!(picker.step(Key::Interrupt), Step::Close));
}

#[test]
fn filter_helper_reads_the_text_after_the_verb() {
    assert_eq!(filter("list"), None);
    assert_eq!(filter("list   "), None);
    assert_eq!(filter("list code"), Some("code".to_string()));
}
