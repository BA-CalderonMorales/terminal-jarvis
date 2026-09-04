//! Boot contract: the viewport only engages when geometry and termcap both
//! allow it; the session guard always restores the terminal.

use crate::tui::screen::{active, boot, Size};

#[test]
fn floor_geometry_is_usable_and_tiny_is_not() {
    let floor = Size {
        cols: Size::MIN_COLS,
        rows: Size::MIN_ROWS,
    };
    assert!(floor.usable());
    assert!(!Size { cols: 1, rows: 1 }.usable());
}

#[test]
fn boot_stays_off_without_a_sized_tty() {
    // The test harness's stdout is captured, never a winsize tty.
    assert!(boot().is_none());
    assert!(!active());
}
