//! Graveyard: the retired-tool ledger. A retired harness stays in the catalog
//! so its binary, install channel, and removal path remain inspectable; this
//! surface is where "why did it stop working, and what replaced it?" gets a
//! dated answer instead of a silent failure.
//!
//! A vendor announces a shutdown before it happens, and that announcement is
//! the news -- so the ledger separates the two states. Printing "retired
//! 2026-12-31" on a reader whose calendar says November asks them to
//! disbelieve the tool.

use super::super::value_block;
use super::super::{output_fields as fields, style};
use crate::cli::structs::value_block::{Group, Row};
use crate::contracts::{Harness, Sunset};

#[path = "output_graveyard_words.rs"]
pub(super) mod words;
use words::{binary_state, date_label, headline, plain};

fn retired(harnesses: &[Harness]) -> Vec<&Harness> {
    harnesses
        .iter()
        .filter(|harness| harness.is_sunset())
        .collect()
}

fn sunset(harness: &Harness) -> &Sunset {
    harness.sunset.as_ref().expect("filtered to sunset")
}

/// The one fact the detail card owes a retired tool: when, and what to use
/// instead. Absent for a maintained harness -- there is nothing to say.
pub fn retirement(harness: &Harness) -> Option<Row> {
    let record = harness.sunset.as_ref()?;
    Some(Row::new(
        "retirement",
        format!(
            "{} {} · replaced by {}",
            record.verb(&crate::catalog::today_utc()),
            record.at,
            record.successor()
        ),
    ))
}

pub fn graveyard(harnesses: &[Harness]) -> String {
    let today = crate::catalog::today_utc();
    let shown = retired(harnesses);
    if style::plain() {
        return plain(&shown, &today);
    }
    if shown.is_empty() {
        return format!(
            "{}\n",
            style::success("No retired harnesses: every catalog harness is maintained.")
        );
    }
    let width = fields::width();
    let mut lines = vec![style::heading(&headline(&shown, harnesses.len(), &today))];
    for harness in shown {
        let record = sunset(harness);
        lines.push(String::new());
        lines.push(format!("{} ({})", harness.display, harness.name));
        lines.extend(fields::section(String::new(), &record.note, width));
        lines.extend(value_block::card(
            &[Group::new(
                "the retirement",
                vec![
                    Row::new(date_label(record, &today), &record.at),
                    Row::new("replaced by", record.successor()),
                    Row::new("binary", binary_state(harness)),
                ],
            )],
            width,
        ));
    }
    format!("{}\n", lines.join("\n"))
}
