use super::super::fixture::{fleet, long_fleet, picker, ready};
use super::*;
use crate::tui::input::Key;

#[test]
fn rows_carry_the_cursor_readiness_and_the_active_marker() {
    let harnesses = fleet();
    let mut picker = Picker::new(
        &harnesses,
        Some("gamma".to_string()),
        ready(&["beta"]),
        None,
    );
    picker.step(Key::Down);
    let rows = picker.rows(80);
    assert!(rows[0].contains("1  alpha"));
    assert!(rows[0].contains("[install]"));
    assert!(rows[1].starts_with('▸'));
    assert!(rows[1].contains("[ready]"));
    assert!(!rows[1].contains("(current)"));
    assert!(rows[2].contains("(current)"));
}

#[test]
fn narrow_frames_drop_the_description_before_the_verdict() {
    let harnesses = fleet();
    let picker = picker(&harnesses);
    let rows = picker.rows(26);
    assert!(rows[0].contains("[install]"));
    assert!(!rows[0].contains("probe"));
}

#[test]
fn the_cursor_row_names_the_selection_the_enter_key_would_take() {
    let harnesses = fleet();
    let mut picker = picker(&harnesses);
    picker.step(Key::Down);
    let rows = picker.rows(80);
    assert!(rows[1].starts_with('▸'));
    assert_eq!(
        picker.selected().map(|harness| harness.name.as_str()),
        Some("beta")
    );
}

#[test]
fn the_cursor_never_scrolls_out_of_the_painted_window() {
    let harnesses = long_fleet();
    let mut picker = picker(&harnesses);
    let rows = 12;
    for _ in 0..harnesses.len() {
        let offset = picker.offset(rows);
        let start = harnesses.len() - offset - rows.min(harnesses.len());
        assert!(
            (start..start + rows).contains(&picker.cursor),
            "cursor {} outside window {}..{} (offset {offset})",
            picker.cursor,
            start,
            start + rows
        );
        picker.step(Key::Down);
    }
}

#[test]
fn the_list_starts_at_the_top_until_the_cursor_reaches_the_edge() {
    let harnesses = long_fleet();
    let picker = picker(&harnesses);
    assert_eq!(
        picker.offset(12),
        25 - 12,
        "the first screen shows the fleet from the top"
    );
}
