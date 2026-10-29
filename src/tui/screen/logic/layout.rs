//! Layout: the frame's row builders -- header, rule, body rows, prompt.
//! The header degrades by segment priority on narrow terminals: the
//! readiness verdict survives, the brand tagline dies first, then the cwd,
//! then the title and active-harness clause -- never a cut mid-segment.

use super::canvas::{self, fit};
use super::modeline;
use super::roles::Role;
use super::sanitize;
use super::scroll;
use super::theme;
use super::Draft;

/// The header row: compact centers+greys it; roomy yields the cwd, then the
/// brand tagline; at inner <= 40 whole header clauses drop until the verdict
/// is the last thing standing.
pub fn header(draft: &Draft, inner: usize, body_rows: usize, compact: bool) -> String {
    let badge = scroll::badge(draft.offset, draft.body.len(), body_rows);
    let badge = if badge.is_empty() {
        badge
    } else {
        format!(" {badge}")
    };
    let badge_width = canvas::visible_width(&badge);
    let core = sanitize::keep_color(&draft.header);
    let cwd = format!(" · {}", sanitize::keep_color(&draft.cwd));
    // The working directory is live signal and rides the header; the status
    // rule below owns the fleet truth, so the header never repeats it.
    let mut candidates = vec![format!("{core}{cwd}")];
    candidates.push(core.clone());
    if inner <= 40 {
        candidates.extend(degraded(&core));
    }
    for candidate in &candidates {
        if canvas::visible_width(candidate) + badge_width <= inner {
            return finish(candidate, inner, compact);
        }
    }
    let last = candidates.last().expect("candidates never empty");
    fit(&finish(last, inner, compact), inner)
}

/// The merged header's tail clauses, shortest first: the verdict clause
/// is the last " · " segment and outlives the title and active name.
fn degraded(core: &str) -> Vec<String> {
    let clauses: Vec<&str> = core.split(" · ").collect();
    (1..clauses.len())
        .map(|keep| clauses[clauses.len() - keep..].join(" · "))
        .collect()
}

/// Centers+greys the header in the compact frame; flush in the roomy one.
fn finish(line: &str, inner: usize, compact: bool) -> String {
    if !compact {
        return line.to_string();
    }
    let width = canvas::visible_width(line);
    let centered = format!("{}{line}", " ".repeat(inner.saturating_sub(width) / 2));
    theme::dim(&modeline::pad(&centered, inner))
}

/// The status rule under the header: the fleet truth, then a dim rule to the
/// frame edge. A narrow terminal keeps the words and drops the ink.
pub fn status_rule(tagline: &str, inner: usize) -> String {
    let text = sanitize::keep_color(tagline);
    let width = canvas::visible_width(&text);
    if width + 3 > inner {
        return fit(&text, inner);
    }
    format!(
        "{} {text} {}",
        theme::paint(Role::Rule, "─"),
        theme::paint(Role::Rule, &"─".repeat(inner - width - 3))
    )
}

pub use modeline::{pad, prompt};
