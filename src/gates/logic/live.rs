//! Live: the pure progress-line rules for a running gate scan -- the fixed
//! width every tick is padded to, and when a tick fires. Drawing is pure,
//! property-tested.

use std::time::Duration;

pub const TICK: Duration = Duration::from_secs(5);
pub const TAIL: &str = " · scanning workspace; this can take a minute or more";

/// The live line for an elapsed scan time: CR, fixed-width seconds, padding.
pub fn live_line(prefix: &str, secs: u64) -> String {
    let body = format!("{prefix} {secs:>4}s{TAIL}");
    format!(
        "\r{body}{}",
        " ".repeat(live_width(prefix).saturating_sub(body.len()))
    )
}

/// The stable width every live line is padded to, so ticks leave no residue.
pub fn live_width(prefix: &str) -> usize {
    prefix.len() + 1 + 4 + 1 + TAIL.len()
}

pub fn should_tick(elapsed_secs: u64) -> bool {
    elapsed_secs >= TICK.as_secs() && elapsed_secs % TICK.as_secs() == 0
}
