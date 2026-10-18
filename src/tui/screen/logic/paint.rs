//! Paint: composes the viewport as three minimal zones -- one merged header
//! line, a dim rule, then the body -- and a prompt line with the hint and
//! scroll badge right-aligned. Repaints overwrite in place: cursor home, no
//! erase, so keystrokes never flicker.

use super::layout;
use super::sanitize;
use super::scroll;
use super::structs::Size;
use super::theme;

/// One composed frame. `body` arrives pre-styled; fitting happens here.
pub struct Draft {
    pub header: String,
    pub cwd: String,
    pub tagline: String,
    pub body: Vec<String>,
    pub prompt: String,
    pub offset: usize,
    pub hint: String,
    /// Anchor the body to the bottom, just above the prompt: a decision
    /// belongs next to the cursor, not in the middle of the frame.
    pub bottom: bool,
}

pub fn frame(size: Size, draft: &Draft) -> String {
    let inner = size.inner_cols();
    // Compact terminals spend every row on content and give up the rule.
    let compact = size.cols < 100 || size.rows < 20;
    let body_rows = size.rows.saturating_sub(if compact { 2 } else { 3 }).max(1);
    let mut rows = vec![layout::header(draft, inner, body_rows, compact)];
    if !compact {
        rows.push(layout::rule(inner));
    }
    let window = scroll::window(&draft.body, draft.offset, body_rows);
    let block = body_block(window, inner);
    let pad_top = if draft.bottom {
        body_rows.saturating_sub(block.len())
    } else {
        (body_rows - block.len()) / 2
    };
    for _ in 0..pad_top {
        rows.push(layout::pad("", inner));
    }
    for row in block {
        rows.push(row);
    }
    while rows.len() < size.rows - 1 {
        rows.push(layout::pad("", inner));
    }
    rows.push(layout::prompt(draft, inner, body_rows));
    // Raw mode disables OPOST: paint with explicit returns or rows stair-step.
    format!("\x1b[H{}", rows.join("\r\n"))
}

/// The body window as painted rows: a centered BLOCK clipped to the frame.
/// A line that names its own role keeps it; only unstyled text takes the dim.
fn body_block(window: &[String], inner: usize) -> Vec<String> {
    let text: Vec<String> = window
        .iter()
        .map(|line| sanitize::keep_color(line))
        .map(|line| crate::tui::screen::clip_line(&line, inner))
        .collect();
    let width = text
        .iter()
        .map(|line| crate::tui::screen::visible_width(line))
        .max()
        .unwrap_or(0);
    let margin = inner.saturating_sub(width) / 2;
    text.iter()
        .map(|line| {
            let padded = layout::pad(&format!("{}{line}", " ".repeat(margin)), inner);
            if line.contains('\x1b') {
                padded
            } else {
                theme::dim(&padded)
            }
        })
        .collect()
}

pub fn parked(mut painted: String, size: Size, prompt_cells: usize) -> String {
    let row = size.rows;
    let col = prompt_cells.max(1);
    painted.push_str(&format!("\x1b[{row};{col}H"));
    painted
}

#[cfg(test)]
#[path = "../../tests/screen_paint.rs"]
mod tests;

#[cfg(test)]
#[path = "../../tests/screen_narrow.rs"]
mod narrow_tests;

#[cfg(test)]
#[path = "../../tests/screen_parked.rs"]
mod parked_tests;
