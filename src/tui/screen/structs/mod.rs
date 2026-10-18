//! Screen data shapes.

/// Terminal geometry in cells.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub cols: usize,
    pub rows: usize,
}

impl Size {
    /// Below these bounds the viewport frame cannot hold its chrome plus a
    /// useful body. 40 columns is the floor: the frame composes and clips
    /// there (its rows are `cols - 2` wide), so a narrow terminal never
    /// makes the terminal itself wrap a row onto the next line.
    pub const MIN_COLS: usize = 40;
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
