//! Art: the first-boot primer. The header owns identity; the body is a
//! short, dimmed command primer — centered by the paint pass — that yields
//! to real content after the first command.

/// The default body: the keys a first-run user needs. Pure so tests can pin
/// the shape; `active` names the harness.
pub fn welcome(_active: &str, _ready: usize, _total: usize) -> Vec<String> {
    vec![
        "  list            numbered fleet picker".to_string(),
        "  status          readiness dashboard".to_string(),
        "  <number>        instant switch".to_string(),
        "  plan <h> <cap>  preview before running".to_string(),
        "  /debug          raw view · help full table".to_string(),
        "  exit            leave the command center".to_string(),
    ]
}

/// The header tagline: purpose plus the live fleet state, right-aligned on
/// the header row.
pub fn tagline(active: &str, ready: usize, total: usize) -> String {
    format!("context command center · active [{active}] · fleet readiness {ready}/{total}")
}

#[cfg(test)]
#[path = "../../tests/screen_art.rs"]
mod tests;
