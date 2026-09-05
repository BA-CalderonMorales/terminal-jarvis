//! Art: the first-boot primer. The header owns identity; the body greets
//! with the live fleet state, then a short, dimmed command primer —
//! centered by the paint pass — that yields to real content after the
//! first command.

/// The default body: a centered greeting over a left-aligned command
/// block, in the order a new user reaches for them. Pure so tests can pin
/// the shape.
pub fn welcome(active: &str, ready: usize, total: usize) -> Vec<String> {
    let commands = [
        "  home              the command center",
        "  status            readiness dashboard",
        "  list              numbered fleet picker",
        "  <number|harness>  instant switch",
        "  plan <h> <cap>    preview before running",
        "  install <h>       add a harness to the fleet",
        "  show <h>          harness details",
        "  debug             raw view · help full table",
        "  exit              leave the command center",
    ];
    let greeting = [
        format!("Welcome back -- {active} is at the helm."),
        String::new(),
        format!("{ready} of {total} harnesses are ready to run; the rest are one install away."),
        String::new(),
    ];
    // One shared axis: greetings center over the block, commands stay
    // flush-left as a unit, and the paint pass centers the assembly.
    let widest = greeting
        .iter()
        .map(String::as_str)
        .chain(commands.iter().copied())
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let centered = |line: &str| {
        let pad = (widest.saturating_sub(line.chars().count())) / 2;
        format!("{}{line}", " ".repeat(pad))
    };
    let mut lines: Vec<String> = greeting.iter().map(|line| centered(line)).collect();
    lines.extend(commands.iter().map(|line| (*line).to_string()));
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
