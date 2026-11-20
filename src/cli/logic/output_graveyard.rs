//! Graveyard: the retired-tool ledger. A retired harness stays in the catalog
//! so its binary, install channel, and removal path remain inspectable; this
//! surface is where "why did it stop working, and what replaced it?" gets a
//! dated answer instead of a silent failure.

use super::super::value_block;
use super::super::{output_fields as fields, style};
use crate::cli::structs::value_block::{Group, Row};
use crate::contracts::Harness;

fn binary_state(harness: &Harness) -> &'static str {
    if crate::security::command_on_path(&harness.binary) {
        "still on PATH; uninstall removes it"
    } else {
        "not installed"
    }
}

fn retired(harnesses: &[Harness]) -> Vec<&Harness> {
    harnesses
        .iter()
        .filter(|harness| harness.is_sunset())
        .collect()
}

/// `<name> retired=<date> replaced_by=<tool> binary=<state> note=<why>`, one
/// line per retired tool -- the same key=value contract `show --plain` keeps.
fn plain(shown: &[&Harness]) -> String {
    let mut out = String::new();
    for harness in shown {
        let sunset = harness.sunset.as_ref().expect("filtered to sunset");
        out.push_str(&format!(
            "{} retired={} replaced_by={} binary={} note={}\n",
            harness.name,
            sunset.at,
            sunset.successor(),
            binary_state(harness),
            sunset.note
        ));
    }
    out
}

/// The one fact the detail card owes a retired tool: when, and what to use
/// instead. Absent for a maintained harness -- there is nothing to say.
pub fn retirement(harness: &Harness) -> Option<Row> {
    let sunset = harness.sunset.as_ref()?;
    Some(Row::new(
        "retirement",
        format!("retired {} · replaced by {}", sunset.at, sunset.successor()),
    ))
}

pub fn graveyard(harnesses: &[Harness]) -> String {
    let shown = retired(harnesses);
    if style::plain() {
        return plain(&shown);
    }
    if shown.is_empty() {
        return format!(
            "{}\n",
            style::success("No retired harnesses: every catalog harness is maintained.")
        );
    }
    let width = fields::width();
    let mut lines = vec![style::heading(&format!(
        "Graveyard · {} of {} retired",
        shown.len(),
        harnesses.len()
    ))];
    for harness in shown {
        let sunset = harness.sunset.as_ref().expect("filtered to sunset");
        lines.push(String::new());
        lines.push(format!("{} ({})", harness.display, harness.name));
        lines.extend(fields::section(String::new(), &sunset.note, width));
        lines.extend(value_block::card(
            &[Group::new(
                "the retirement",
                vec![
                    Row::new("retired", &sunset.at),
                    Row::new("replaced by", sunset.successor()),
                    Row::new("binary", binary_state(harness)),
                ],
            )],
            width,
        ));
    }
    format!("{}\n", lines.join("\n"))
}
