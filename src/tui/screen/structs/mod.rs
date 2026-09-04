//! Screen data shapes.

/// Terminal geometry in cells.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub cols: usize,
    pub rows: usize,
}

impl Size {
    /// Below these bounds the viewport frame cannot hold its four fixed
    /// chrome rows plus a useful body; the tui falls back to chat mode.
    pub const MIN_COLS: usize = 50;
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
