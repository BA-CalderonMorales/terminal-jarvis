//! Scroll: window math for bodies taller than the body zone. The offset
//! counts lines hidden below the window -- zero means the newest lines are
//! on screen, growing means the user is reading history. Pure, so tests
//! can pin the bounds.

use crate::tui::input::Move;

/// Furthest an offset may climb for a body of `len` in `rows`.
pub fn max_offset(len: usize, rows: usize) -> usize {
    len.saturating_sub(rows)
}

/// Clamps any offset into `[0, max]`; oversized terminals pin to zero.
pub fn clamp(offset: usize, len: usize, rows: usize) -> usize {
    offset.min(max_offset(len, rows))
}

/// Applies one navigation move; the result never escapes the bounds.
pub fn step(offset: usize, toward: Move, len: usize, rows: usize) -> usize {
    let page = rows.saturating_sub(2).max(1);
    match toward {
        Move::Top => max_offset(len, rows),
        Move::Bottom => 0,
        Move::Older => clamp(offset.saturating_add(1), len, rows),
        Move::Newer => offset.saturating_sub(1),
        Move::PageOlder => clamp(offset.saturating_add(page), len, rows),
        Move::PageNewer => offset.saturating_sub(page),
    }
}

/// The visible slice: up to `rows` lines ending `offset` above the newest.
pub fn window(lines: &[String], offset: usize, rows: usize) -> &[String] {
    let end = lines.len() - clamp(offset, lines.len(), rows);
    &lines[end.saturating_sub(rows)..end]
}

/// Bottom-border badge: how many lines hide above and below the window.
pub fn badge(offset: usize, len: usize, rows: usize) -> String {
    let off = clamp(offset, len, rows);
    let above = max_offset(len, rows) - off;
    match (above, off) {
        (0, 0) => String::new(),
        (above, 0) => format!("↑ {above}"),
        (0, below) => format!("↓ {below}"),
        (above, below) => format!("↑ {above} · ↓ {below}"),
    }
}

#[cfg(test)]
#[path = "../../tests/screen_scroll.rs"]
mod tests;
