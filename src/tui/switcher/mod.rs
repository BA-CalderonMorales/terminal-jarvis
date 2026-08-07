//! Switcher: the numbered harness picker. The welcome stays clean, but once
//! a tool list is visible -- `/list`, or the catalog table -- a bare number
//! maps straight to the same `use` action the headless cli would parse, so
//! flipping between tools stays a single keystroke. `pick` renders the list
//! as numbered, single-line rows with the current agent marked -- a picker,
//! not a dump.

use crate::cli::args;
use crate::contracts::Harness;

pub fn select(input: &str, harnesses: &[Harness]) -> Option<args::Action> {
    let index = input.parse::<usize>().ok()?;
    if index == 0 {
        return None;
    }
    harnesses
        .get(index - 1)
        .map(|harness| args::Action::Use(harness.name.clone()))
}

pub fn pick(harnesses: &[Harness], active: Option<&str>) -> String {
    let width = harnesses
        .iter()
        .map(|harness| harness.name.chars().count())
        .max()
        .unwrap_or(0);
    let mut lines: Vec<String> = harnesses
        .iter()
        .enumerate()
        .map(|(index, harness)| row(index + 1, width, harness, active))
        .collect();
    let tip = "pick a number to switch agents, or type 'home' to go back".to_string();
    lines.push(crate::cli::style::dim(&tip));
    let mut body = lines.join("\n");
    body.push('\n');
    body
}

fn row(index: usize, width: usize, harness: &Harness, active: Option<&str>) -> String {
    let marker = if active == Some(harness.name.as_str()) {
        format!("  {}", crate::cli::style::dim("(current)"))
    } else {
        String::new()
    };
    format!(
        "{index:>2}  {:<width$}  {}{marker}",
        harness.name,
        clip(&harness.description, 66)
    )
}

pub fn clip(description: &str, limit: usize) -> String {
    let mut remaining = limit;
    let mut clipped = String::new();
    for character in description.chars() {
        if remaining == 0 {
            clipped.extend(['.', '.', '.']);
            break;
        }
        clipped.push(character);
        remaining -= 1;
    }
    clipped
}

#[cfg(test)]
#[path = "../tests/select_props.rs"]
mod props;
#[cfg(test)]
#[path = "../tests/select.rs"]
mod tests;
