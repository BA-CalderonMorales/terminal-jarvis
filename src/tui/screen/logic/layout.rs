//! Layout: the frame's row builders -- header, rule, body rows, prompt.
//! The header degrades by segment priority on narrow terminals: the
//! readiness verdict survives, cwd dies first, tagline next, then the
//! title and active-harness clause -- never a cut mid-segment.

use super::canvas::{self, fit};
use super::modeline;
use super::sanitize;
use super::scroll;
use super::theme;
use super::Draft;

/// The header row: compact centers+greys it; roomy yields tagline, then
/// cwd; at inner <= 40 whole header clauses drop until the verdict is
/// the last thing standing.
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
    let tagline = format!("  {}", sanitize::keep_color(&draft.tagline));
    let mut candidates = vec![format!("{core}{tagline}"), format!("{core}{cwd}")];
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

pub use modeline::{pad, prompt, rule};
