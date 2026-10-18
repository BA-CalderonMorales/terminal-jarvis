//! StreamRows: one classified child row, styled for the frame. The runner
//! tags each line with a level; here the level becomes a compact label in its
//! role and the message becomes body text, so a run's output reads as a
//! contained log rather than a wall of equal-weight lines.

use crate::tui::screen::{paint, Role};
use std::time::{Duration, Instant};

/// The level labels the runner emits, longest first. A row without one is not
/// a classified log line -- a gate note, an advisory -- and passes through
/// untouched, so nothing is ever dropped or mislabelled.
const LEVELS: [(&str, Role); 3] = [
    ("ERROR", Role::Err),
    ("WARN ", Role::Warn),
    ("INFO ", Role::Dim),
];

/// Styles one row: `  info   message`, the level in its role and the message
/// as body text. The timestamp the runner used to prefix is gone.
pub fn render(row: &str) -> String {
    for (level, role) in LEVELS {
        if let Some(message) = row.strip_prefix(level) {
            let label = level.trim_end().to_ascii_lowercase();
            return format!(
                "  {}  {}",
                paint(role, &format!("{label:<5}")),
                paint(Role::Value, message.trim_start())
            );
        }
    }
    row.to_string()
}

/// The burst-head throttle for live rows. The guard chain paints the plan and
/// the consent question as a few rows and then blocks on a key, so the head of
/// every burst paints in full; sustained output is throttled to one repaint
/// per window. A throttled question would leave the user staring at the old
/// screen with nothing to answer.
pub struct Throttle {
    painted_at: Option<Instant>,
    rows: usize,
}

const HEAD_ROWS: usize = 5;
const WINDOW: Duration = Duration::from_millis(180);

impl Throttle {
    pub fn new() -> Self {
        Self {
            painted_at: None,
            rows: 0,
        }
    }

    /// Records `rows` new rows and reports whether to repaint now.
    pub fn due(&mut self, rows: usize) -> bool {
        self.rows += rows;
        let now = Instant::now();
        let elapsed = self
            .painted_at
            .map(|at| now.duration_since(at) > WINDOW)
            .unwrap_or(true);
        if elapsed || self.rows <= HEAD_ROWS {
            self.painted_at = Some(now);
            self.rows = 0;
            return true;
        }
        false
    }
}

#[cfg(test)]
#[path = "../tests/stream_rows.rs"]
mod tests;
