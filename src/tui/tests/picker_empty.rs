//! Empty picker filter (the `tui-pages` flag): one guidance line when the
//! filter matches nothing; main's empty paint when the flag is off.

use super::super::fixture::{fleet, ready};
use super::super::Picker;
use super::painted;
use crate::contracts::Harness;
use crate::tui::pages_env;
use crate::tui::screen::visible_width;

fn no_match(harnesses: &[Harness]) -> Picker<'_> {
    Picker::new(harnesses, None, ready(&["beta"]), Some("zzz-no-agent"))
}

#[test]
fn an_empty_filter_paints_no_rows_when_the_flag_is_off() {
    let _env = pages_env::set(None);
    let harnesses = fleet();
    assert!(painted(&no_match(&harnesses), 60).is_empty());
}

#[test]
fn an_empty_filter_says_why_and_how_to_leave_when_the_flag_is_on() {
    let _env = pages_env::set(Some("1"));
    let harnesses = fleet();
    let rows = painted(&no_match(&harnesses), 60);
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert!(rows[0].contains("No agents match this filter"));
    assert!(rows[1].contains("Ctrl+U clears") && rows[1].contains("Esc closes"));
    assert!(rows.iter().all(|row| visible_width(row) <= 60));
}

#[test]
fn a_matching_filter_paints_the_picker_rows_with_the_flag_on() {
    let _env = pages_env::set(Some("1"));
    let harnesses = fleet();
    let picker = Picker::new(&harnesses, None, ready(&[]), Some("alpha"));
    assert_eq!(painted(&picker, 80), picker.rows(80));
}
