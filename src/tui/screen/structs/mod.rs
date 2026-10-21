//! Screen data shapes.

/// Terminal geometry in cells.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub cols: usize,
    pub rows: usize,
}

impl Size {
    /// Below these bounds the viewport frame cannot hold its chrome plus a
    /// useful body. The column floor is deliberately low: the frame composes
    /// and clips at any width (`cols - 2` rows), so a narrow terminal stays on
    /// the frame instead of falling back to chat mode -- where the terminal
    /// wraps the install log and walks the prompt up the screen.
    pub const MIN_COLS: usize = 20;
    pub const MIN_ROWS: usize = 10;

    pub fn usable(self) -> bool {
        self.cols >= Self::MIN_COLS && self.rows >= Self::MIN_ROWS
    }

    /// Body-zone height: everything minus top border, status, separator,
    /// separator, prompt, bottom border.
    pub fn body_rows(self) -> usize {
        self.rows.saturating_sub(6).max(1)
    }

    /// Inner width between the side borders.
    pub fn inner_cols(self) -> usize {
        self.cols.saturating_sub(2)
    }
}

#[cfg(test)]
#[path = "../../tests/screen_size.rs"]
mod tests;
