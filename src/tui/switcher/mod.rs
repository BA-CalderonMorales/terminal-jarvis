//! Switcher: the numbered harness picker. A bare number maps straight to the
//! same `use` action the headless cli parses, so flipping tools is a single
//! keystroke. Each row carries the dashboard's readiness verdict, so the
//! picker answers "what can I run now?" without a second command, and a
//! filter narrows 25 rows while the numbers keep their picker positions.

use crate::cli::args;
use crate::contracts::Harness;
use std::collections::BTreeSet;

#[path = "filter.rs"]
mod filter;
#[path = "rows.rs"]
mod rows;

pub fn select(input: &str, harnesses: &[Harness]) -> Option<args::Action> {
    let index = input.parse::<usize>().ok()?;
    if index == 0 {
        return None;
    }
    harnesses
        .get(index - 1)
        .map(|harness| args::Action::Use(harness.name.clone()))
}

pub fn pick(
    harnesses: &[Harness],
    active: Option<&str>,
    ready: &BTreeSet<String>,
    filter: Option<&str>,
    inner: usize,
) -> String {
    let needle = filter.unwrap_or("").trim().to_lowercase();
    let shown: Vec<(usize, &Harness)> = harnesses
        .iter()
        .enumerate()
        .filter(|(_, harness)| filter::matches(harness, &needle))
        .collect();
    let name_width = shown
        .iter()
        .map(|(_, harness)| harness.name.chars().count())
        .max()
        .unwrap_or(0);
    let marker = if active.is_some() { rows::MARKER } else { 0 };
    let fixed = 2 + 2 + name_width + 2 + rows::CHIP + 2 + marker;
    let room = inner
        .checked_sub(fixed)
        .filter(|room| *room >= rows::MIN_DESCRIPTION);
    let mut lines: Vec<String> = shown
        .iter()
        .map(|(offset, harness)| {
            let chip = if ready.contains(&harness.name) {
                "[ready]"
            } else {
                "[install]"
            };
            rows::row(
                offset + 1,
                name_width,
                harness,
                chip,
                room,
                active == Some(harness.name.as_str()),
            )
        })
        .collect();
    lines.push(filter::tip(
        &needle,
        shown.len(),
        harnesses.len(),
        filter.is_some(),
    ));
    let mut body = lines.join("\n");
    body.push('\n');
    body
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
#[path = "../tests/select_filter.rs"]
mod filter_tests;
#[cfg(test)]
#[path = "../tests/select_props.rs"]
mod props;
#[cfg(test)]
#[path = "../tests/select.rs"]
mod tests;
