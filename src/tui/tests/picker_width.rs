use super::super::fixture::{fleet, ready};
use super::*;

#[test]
fn cell_budgets_include_the_cursor_chip_ellipsis_and_current_marker() {
    for description in ["a long description ".repeat(20), "漢字の説明 ".repeat(20)] {
        let mut harnesses = fleet();
        harnesses[0].description = description;
        let picker = Picker::new(&harnesses, Some("alpha".into()), ready(&["alpha"]), None);
        for inner in [18, 28, 38, 58, 78, 98] {
            let rows = picker.rows(inner);
            assert!(rows[0].contains("(current)"));
            for row in rows {
                assert!(
                    crate::tui::screen::visible_width(&row) <= inner,
                    "{inner}: {row}"
                );
            }
        }
    }
}
