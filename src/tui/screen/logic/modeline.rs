//! Modeline: the prompt row -- indicator left, hint and scroll badge
//! right-aligned. On narrow terminals (inner <= 60) the hint degrades to
//! its shortest useful form: the text before the first " | " or " · "
//! clause, so "active: x | list, status, ..." reads as just "active: x"
//! instead of vanishing or cutting mid-clause.

use super::canvas;
use super::sanitize;
use super::scroll;
use super::theme;
use super::Draft;

pub fn prompt(draft: &Draft, inner: usize, body_rows: usize) -> String {
    let left = sanitize::keep_color(&draft.prompt);
    let badge = scroll::badge(draft.offset, draft.body.len(), body_rows);
    let hint = sanitize::keep_color(&draft.hint);
    let hint = if inner <= 60 {
        shortest(&hint).to_string()
    } else {
        hint
    };
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
        format!("{left}{}", " ".repeat(inner - left_width - right_width - 1))
    };
    let right = if fits {
        theme::dim(&right)
    } else if badge_fits {
        let gap = inner - left_width - badge_width;
        return format!(
            "{left}{}{}",
            " ".repeat(gap.saturating_sub(1)),
            theme::accent(&badge)
        );
    } else {
        return left;
    };
    format!("{left}{right}")
}

/// The shortest useful form of a hint: its first clause.
fn shortest(hint: &str) -> &str {
    hint.split(" | ")
        .next()
        .unwrap_or(hint)
        .split(" · ")
        .next()
        .unwrap_or(hint)
}

pub fn rule(inner: usize) -> String {
    theme::dim(&"─".repeat(inner))
}

pub fn pad(line: &str, inner: usize) -> String {
    let width = canvas::visible_width(line);
    let fill = inner.saturating_sub(width);
    format!("{line}{}", " ".repeat(fill))
}
