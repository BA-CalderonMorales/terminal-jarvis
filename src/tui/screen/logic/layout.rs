//! Layout: the frame's row builders -- header, rule, body rows, prompt.

use super::canvas::{self, fit};
use super::sanitize;
use super::scroll;
use super::theme;
use super::Draft;

/// The header row: compact centers+greys it; roomy yields tagline, then cwd.
pub fn header(draft: &Draft, inner: usize, body_rows: usize, compact: bool) -> String {
    let badge = scroll::badge(draft.offset, draft.body.len(), body_rows);
    let badge = if badge.is_empty() {
        badge
    } else {
        format!(" {badge}")
    };
    let core = sanitize::keep_color(&draft.header);
    if compact {
        // Small terminals: the chrome centers and greys out so content owns
        // the screen; tagline and cwd yield entirely.
        let width = canvas::visible_width(&core);
        let centered = format!("{}{core}", " ".repeat(inner.saturating_sub(width) / 2));
        return fit(&theme::dim(&pad(&centered, inner)), inner);
    }
    let cwd = format!(" · {}", sanitize::keep_color(&draft.cwd));
    let tagline = format!("  {}", sanitize::keep_color(&draft.tagline));
    let with_cwd = format!("{core}{cwd}");
    let with_tagline = format!("{core}{tagline}");
    let line = if canvas::visible_width(&with_tagline) + canvas::visible_width(&badge) <= inner {
        with_tagline
    } else if canvas::visible_width(&with_cwd) + canvas::visible_width(&badge) <= inner {
        with_cwd
    } else {
        core
    };
    fit(&line, inner)
}

pub fn prompt(draft: &Draft, inner: usize, body_rows: usize) -> String {
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

pub fn rule(inner: usize) -> String {
    theme::dim(&"─".repeat(inner))
}

pub fn pad(line: &str, inner: usize) -> String {
    let width = canvas::visible_width(line);
    let fill = inner.saturating_sub(width);
    format!("{line}{}", " ".repeat(fill))
}
