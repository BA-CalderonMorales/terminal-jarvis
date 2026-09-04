//! Art: the first-boot primer. The header line carries identity; the body
//! starts as a short command primer and yields to real content after the
//! first command. Pure so tests can pin the shape.

/// The default body: one overview line, then the keys a first-run user
/// needs. Pure so tests can pin the shape; `active` names the harness.
pub fn welcome(active: &str, ready: usize, total: usize) -> Vec<String> {
    vec![
        format!("context command center · active [{active}] · fleet readiness {ready}/{total}"),
        String::new(),
        "  list            numbered fleet picker".to_string(),
        "  status          readiness dashboard".to_string(),
        "  <number>        instant switch".to_string(),
        "  plan <h> <cap>  preview before running".to_string(),
        "  /debug          raw view · help full table · exit leaves".to_string(),
    ]
}

#[cfg(test)]
#[path = "../../tests/screen_art.rs"]
mod tests;
