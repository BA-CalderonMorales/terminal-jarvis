//! Paint: composes the whole viewport frame -- bordered title, live status
//! row, body canvas clipped to the zone, prompt row, and a bottom border
//! that carries the hint like an old DOS status bar. Pure string work.

use super::canvas::{self};
use super::sanitize;
use super::structs::Size;

/// One composed frame. `body` arrives pre-styled; fitting happens here.
pub struct Draft {
    pub title: String,
    pub status: String,
    pub body: Vec<String>,
    pub prompt: String,
    pub hint: String,
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
    for line in canvas::fit_rows(
        &draft
            .body
            .iter()
            .map(|l| canvas::clip_line(&sanitize::keep_color(l), inner))
            .collect::<Vec<_>>(),
        size.body_rows(),
    ) {
        rows.push(pad(&line, inner));
    }
    while rows.len() < size.rows - 3 {
        rows.push(pad("", inner));
    }
    rows.push(seam(inner));
    rows.push(pad(
        &canvas::clip_line(&sanitize::keep_color(&draft.prompt), inner),
        inner,
    ));
    let hint = hint_clip(&draft.hint, inner);
    rows.push(bordered(&format!("─ {hint} "), inner, '╰', '╯', '─'));
    format!("\x1b[H\x1b[2J{}", rows.join("\n"))
}

fn hint_clip(hint: &str, inner: usize) -> String {
    let budget = inner.saturating_sub(4);
    hint.chars().take(budget).collect()
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
    let used = canvas::visible_width(label);
    let fill = inner.saturating_sub(used);
    format!("{left}{label}{}{right}", rule.to_string().repeat(fill))
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
