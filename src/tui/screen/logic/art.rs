//! Art: the fixed retro chrome. Block-glyph monogram, one name row, and the
//! quick-key cheat sheet that doubles as the default body -- decoration that
//! teaches the tool is value, not theatrics.

pub const MONOGRAM: &[&str] = &["╔╗╔═╗╔═╗", "║║║╣ ╚═╝", "╝╚╚═╝╚═╝"];

pub const NAME: &str = "T E R M I N A L   J A R V I S";

/// Default body: monogram, identity, then the keys a first-run user needs.
/// Pure so tests can pin the shape; `active` names the current harness.
pub fn welcome(active: &str, ready: usize, total: usize) -> Vec<String> {
    let mut lines: Vec<String> = MONOGRAM.iter().map(|row| row.to_string()).collect();
    lines.push(String::new());
    lines.push(format!("{NAME}  ·  context command center"));
    lines.push(format!(
        "active [{active}]  fleet readiness {ready}/{total}"
    ));
    lines.push(String::new());
    lines.push(" list      numbered fleet picker".into());
    lines.push(" status    readiness dashboard".into());
    lines.push(" <number>  instant switch".into());
    lines.push(" plan <harness> <capability>   preview before running".into());
    lines.push(" /debug    raw view · help full table · exit leaves".into());
    lines
}

#[cfg(test)]
#[path = "../../tests/screen_art.rs"]
mod tests;
