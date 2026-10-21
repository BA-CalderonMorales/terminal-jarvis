//! Size bounds: usable() gates viewport mode, body_rows/inner_cols keep
//! chrome and content inside any geometry.

use super::*;

#[test]
fn min_bounds_gate_the_viewport() {
    assert!(!Size { cols: 19, rows: 24 }.usable());
    assert!(!Size { cols: 120, rows: 9 }.usable());
    assert!(Size {
        cols: Size::MIN_COLS,
        rows: Size::MIN_ROWS
    }
    .usable());
}

#[test]
fn the_column_floor_keeps_a_narrow_terminal_on_the_frame() {
    // 20 columns still composes a frame (`Continue? [Y/n]` is 15 cells), and
    // the frame clips its own rows, so the terminal never wraps one.
    assert_eq!(Size::MIN_COLS, 20);
}

#[test]
fn zones_never_go_negative_at_the_floor() {
    let floor = Size {
        cols: Size::MIN_COLS,
        rows: Size::MIN_ROWS,
    };
    assert_eq!(floor.body_rows(), 4);
    assert_eq!(floor.inner_cols(), Size::MIN_COLS - 2);
    let huge = Size {
        cols: 400,
        rows: 200,
    };
    assert_eq!(huge.body_rows(), 194);
}
