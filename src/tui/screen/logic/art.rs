//! Art: the first-boot primer. The header owns identity; the body greets
//! with the live fleet state, then a short, dimmed command primer —
//! centered by the paint pass — that yields to real content after the
//! first command.

/// The default body: a greeting with the live fleet state, then the
/// commands in the order a new user reaches for them. Pure so tests can
/// pin the shape.
pub fn welcome(active: &str, ready: usize, total: usize) -> Vec<String> {
    let mut lines = vec![
        format!("welcome back -- {active} is at the helm."),
        format!("{ready} of {total} harnesses are ready to run; the rest are one install away."),
        String::new(),
    ];
    lines.extend([
        "  home              the command center".to_string(),
        "  status            readiness dashboard".to_string(),
        "  list              numbered fleet picker".to_string(),
        "  <number|harness>  instant switch".to_string(),
        "  plan <h> <cap>    preview before running".to_string(),
        "  install <h>       add a harness to the fleet".to_string(),
        "  show <h>          harness details".to_string(),
        "  debug             raw view · help full table".to_string(),
        "  exit              leave the command center".to_string(),
    ]);
    lines
}

/// The header tagline: purpose plus the live fleet state, right-aligned on
/// the header row.
pub fn tagline(_active: &str, _ready: usize, _total: usize) -> String {
    "context command center".to_string()
}

#[cfg(test)]
#[path = "../../tests/screen_art.rs"]
mod tests;
