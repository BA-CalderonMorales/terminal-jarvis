//! Paint: composes the viewport as three minimal zones -- one merged header
//! line, a dim rule, then the body -- and a prompt line with the hint and
//! scroll badge right-aligned. Repaints overwrite in place: cursor home, no
//! erase, so keystrokes never flicker. Pure string work.

use super::canvas;
use super::sanitize;
use super::scroll;
use super::structs::Size;

/// One composed frame. `body` arrives pre-styled; fitting happens here.
pub struct Draft {
    pub header: String,
    pub cwd: String,
    pub body: Vec<String>,
    pub prompt: String,
    pub offset: usize,
    pub hint: String,
}

pub fn frame(size: Size, draft: &Draft) -> String {
    let inner = size.inner_cols();
    let body_rows = size.rows.saturating_sub(3).max(1);
    let mut rows = vec![header(draft, inner, body_rows), rule(inner)];
    for line in scroll::window(&draft.body, draft.offset, body_rows) {
        rows.push(pad(&sanitize::keep_color(line), inner));
    }
    while rows.len() < size.rows - 1 {
        rows.push(pad("", inner));
    }
    rows.push(prompt(draft, inner, body_rows));
    format!("\x1b[H{}", rows.join("\n"))
}

/// The merged header line: identity, active harness, and the readiness
/// verdict survive a narrow terminal; the working directory dies first.
fn header(draft: &Draft, inner: usize, body_rows: usize) -> String {
    let badge = scroll::badge(draft.offset, draft.body.len(), body_rows);
    let badge = if badge.is_empty() {
        badge
    } else {
        format!(" {badge}")
    };
    let core = sanitize::keep_color(&draft.header);
    let cwd = format!(" · {}", sanitize::keep_color(&draft.cwd));
    let wide = format!("{core}{cwd}");
    let line = if canvas::visible_width(&wide) + canvas::visible_width(&badge) <= inner {
        wide
    } else {
        core
    };
    fit(&line, inner)
}

fn prompt(draft: &Draft, inner: usize, body_rows: usize) -> String {
    let left = sanitize::keep_color(&draft.prompt);
    let badge = scroll::badge(draft.offset, draft.body.len(), body_rows);
    let hint = sanitize::keep_color(&draft.hint);
    let right = match (badge.is_empty(), hint.is_empty()) {
        (true, true) => String::new(),
        (true, false) => hint,
        (false, true) => badge.clone(),
        (false, false) => format!("{badge} · {hint}"),
    };
    let left_width = canvas::visible_width(&left);
    let right_width = canvas::visible_width(&right);
    let badge_width = canvas::visible_width(&badge);
    let fits = left_width + right_width + 2 <= inner;
    let badge_fits = badge.is_empty() || left_width + badge_width + 2 <= inner;
    let left = if !fits {
        canvas::clip_line(&left, inner.saturating_sub(1))
    } else {
        let gap = inner - left_width - right_width;
        format!("{left}{}", " ".repeat(gap.saturating_sub(1)))
    };
    let right = if fits {
        crate::cli::style::dim(&right)
    } else if badge_fits {
        let gap = inner - left_width - badge_width;
        let padded = format!(
            "{left}{}{}",
            " ".repeat(gap.saturating_sub(1)),
            crate::cli::style::dim(&badge)
        );
        return padded;
    } else {
        return left;
    };
    format!("{left}{right}")
}

fn rule(inner: usize) -> String {
    crate::cli::style::dim(&"─".repeat(inner))
}

fn pad(line: &str, inner: usize) -> String {
    let width = canvas::visible_width(line);
    let fill = inner.saturating_sub(width);
    format!("{line}{}", " ".repeat(fill))
}

/// Clamps a line to `inner` cells, marking the cut with an ellipsis.
fn fit(line: &str, inner: usize) -> String {
    if canvas::visible_width(line) <= inner {
        return line.to_string();
    }
    let mut kept: String = canvas::clip_line(line, inner.saturating_sub(1));
    kept.push('…');
    kept
}

/// Appends the cursor park sequence: one cell past the prompt prefix on its
/// final row, so the session's next write lands after the visible prompt.
pub fn parked(mut painted: String, size: Size, prompt_cells: usize) -> String {
    let row = size.rows.saturating_sub(1);
    let col = prompt_cells.max(1);
    painted.push_str(&format!("\x1b[{row};{col}H"));
    painted
}

#[cfg(test)]
#[path = "../../tests/screen_paint.rs"]
mod tests;
