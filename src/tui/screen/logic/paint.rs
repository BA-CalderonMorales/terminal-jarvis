//! Paint: composes the whole viewport frame -- bordered title, live status
//! row, body canvas windowed to the zone, prompt row, and a bottom border
//! that carries the hint plus a scroll badge like an old DOS status bar.
//! Pure string work.

use super::canvas::{self};
use super::sanitize;
use super::scroll;
use super::structs::Size;

/// One composed frame. `body` arrives pre-styled; fitting happens here.
pub struct Draft {
    pub title: String,
    pub status: String,
    pub body: Vec<String>,
    pub prompt: String,
    pub hint: String,
    pub offset: usize,
}

pub fn frame(size: Size, draft: &Draft) -> String {
    let inner = size.inner_cols();
    let mut rows = Vec::with_capacity(size.rows);
    rows.push(bordered(
        &format!("═ {} ", draft.title),
        inner,
        '╔',
        '╗',
        '═',
    ));
    let status = sanitize::keep_color(&draft.status);
    rows.push(pad(&canvas::clip_line(&status, inner), inner));
    rows.push(seam(inner));
    for line in scroll::window(&draft.body, draft.offset, size.body_rows()) {
        rows.push(pad(
            &canvas::clip_line(&sanitize::keep_color(line), inner),
            inner,
        ));
    }
    while rows.len() < size.rows - 3 {
        rows.push(pad("", inner));
    }
    rows.push(seam(inner));
    rows.push(pad(
        &canvas::clip_line(&sanitize::keep_color(&draft.prompt), inner),
        inner,
    ));
    let hint = format!("─ {} ", canvas::hint_clip(&draft.hint, inner));
    let badge = format!(
        "{} ",
        scroll::badge(draft.offset, draft.body.len(), size.body_rows())
    );
    rows.push(bordered_lr(&hint, &badge, inner, '╰', '╯', '─'));
    format!("\x1b[H\x1b[2J{}", rows.join("\n"))
}

fn pad(line: &str, inner: usize) -> String {
    format!(
        "│{line}{}│",
        " ".repeat(inner.saturating_sub(canvas::visible_width(line)))
    )
}

fn seam(inner: usize) -> String {
    format!("├{}┤", "─".repeat(inner))
}

fn bordered(label: &str, inner: usize, left: char, right: char, rule: char) -> String {
    bordered_lr(label, "", inner, left, right, rule)
}

/// A rule with a label on the left and an optional badge on the right:
/// `╰─ hint ─────── ↑ 12 ─╯`.
fn bordered_lr(label: &str, tail: &str, inner: usize, lch: char, rch: char, rule: char) -> String {
    let used = canvas::visible_width(label) + canvas::visible_width(tail);
    let fill = inner.saturating_sub(used);
    format!("{lch}{label}{}{tail}{rch}", rule.to_string().repeat(fill))
}

/// Appends the cursor park sequence: one cell past the prompt prefix on its
/// row, ready for terminal echo.
pub fn parked(mut painted: String, size: Size, prompt_cells: usize) -> String {
    let row = size.rows.saturating_sub(1);
    let col = prompt_cells.max(1);
    painted.push_str(&format!("\x1b[{row};{col}H"));
    painted
}

#[cfg(test)]
#[path = "../../tests/screen_paint.rs"]
mod tests;
