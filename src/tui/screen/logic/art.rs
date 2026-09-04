//! Art: the first-boot primer. The header owns identity; the body is a
//! short, dimmed command primer — centered by the paint pass — that yields
//! to real content after the first command.

/// The default body: the commands in the order a new user reaches for
/// them. Pure so tests can pin the shape.
pub fn welcome(_active: &str, _ready: usize, _total: usize) -> Vec<String> {
    vec![
        "  home              the command center".to_string(),
        "  status            readiness dashboard".to_string(),
        "  list              numbered fleet picker".to_string(),
        "  <number|harness>  instant switch".to_string(),
        "  plan <h> <cap>    preview before running".to_string(),
        "  install <h>       add a harness to the fleet".to_string(),
        "  show <h>          harness details".to_string(),
        "  debug             raw view · help full table".to_string(),
        "  exit              leave the command center".to_string(),
    ]
}

/// The header tagline: purpose plus the live fleet state, right-aligned on
/// the header row.
pub fn tagline(_active: &str, _ready: usize, _total: usize) -> String {
    "context command center".to_string()
}

#[cfg(test)]
#[path = "../../tests/screen_art.rs"]
mod tests;
