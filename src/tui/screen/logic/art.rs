//! Art: the fixed retro chrome. A double-rule brand block opens the body,
//! then the quick-key cheat sheet -- decoration that teaches the tool is
//! value, not theatrics. Pure so tests can pin the shape.

pub const BRAND: &[&str] = &[
    "╔═══════════════════════════════╗",
    "║ T E R M I N A L   J A R V I S ║",
    "╚═══════════════════════════════╝",
];

/// Default body: brand block, identity, then the keys a first-run user
/// needs. Pure so tests can pin the shape; `active` names the harness.
pub fn welcome(active: &str, ready: usize, total: usize) -> Vec<String> {
    let mut lines: Vec<String> = BRAND.iter().map(|row| row.to_string()).collect();
    lines.push(String::new());
    lines.push(format!(
        "context command center · active [{active}] · fleet readiness {ready}/{total}"
    ));
    lines.push(String::new());
    lines.extend(keys().lines().map(String::from));
    lines
}

fn keys() -> String {
    let rows = [
        ("list", "numbered fleet picker"),
        ("status", "readiness dashboard"),
        ("<number>", "instant switch"),
        ("plan <h> <cap>", "preview before running"),
        ("/debug", "raw view · help full table · exit leaves"),
    ];
    rows.iter()
        .map(|(key, purpose)| format!("  {key:<15} {purpose}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
#[path = "../../tests/screen_art.rs"]
mod tests;
