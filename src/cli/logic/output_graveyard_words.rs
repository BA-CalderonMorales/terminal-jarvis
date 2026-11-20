//! GraveyardWords: the tense and the column labels. A retirement is either
//! announced or done, and the ledger has to say which without the reader
//! checking a calendar against a date printed in past tense.

use crate::contracts::{Harness, Sunset, SunsetState};

pub fn binary_state(harness: &Harness) -> &'static str {
    if crate::security::command_on_path(&harness.binary) {
        "still on PATH; uninstall removes it"
    } else {
        "not installed"
    }
}

/// The card's date label: `retires` while the date is still ahead.
pub fn date_label(record: &Sunset, today: &str) -> &'static str {
    match record.state(today) {
        SunsetState::Done => "retired",
        SunsetState::Scheduled => "retires",
    }
}

/// The heading counts both states rather than calling an announcement a
/// retirement: "2 of 25 retired" in November about a December shutdown is a
/// small lie that undermines every other count on the surface.
pub fn headline(shown: &[&Harness], total: usize, today: &str) -> String {
    let scheduled = shown
        .iter()
        .filter(|harness| {
            harness
                .sunset
                .as_ref()
                .is_some_and(|record| record.state(today) == SunsetState::Scheduled)
        })
        .count();
    let retired = shown.len() - scheduled;
    let tail = match (retired, scheduled) {
        (0, scheduled) => format!("{scheduled} retiring"),
        (retired, 0) => format!("{retired} retired"),
        (retired, scheduled) => format!("{retired} retired · {scheduled} retiring"),
    };
    format!("Graveyard · {tail} of {total}")
}

/// `<name> state=<retired|scheduled> retired=<date> replaced_by=<tool>
/// binary=<state> note=<why>`, one line per retired tool -- the same key=value
/// contract `show --plain` keeps.
///
/// `state` is not decoration: `retired=` carries the date the vendor named,
/// and for a tool with an announced shutdown that date is still in the future.
/// A script reading only the date would treat a working tool as dead, so the
/// tense becomes a field.
pub fn plain(shown: &[&Harness], today: &str) -> String {
    let mut out = String::new();
    for (harness, record) in shown
        .iter()
        .filter_map(|harness| harness.sunset.as_ref().map(|record| (*harness, record)))
    {
        let state = match record.state(today) {
            SunsetState::Done => "retired",
            SunsetState::Scheduled => "scheduled",
        };
        out.push_str(&format!(
            "{} state={} retired={} replaced_by={} binary={} note={}\n",
            harness.name,
            state,
            record.at,
            record.successor(),
            binary_state(harness),
            record.note
        ));
    }
    out
}

#[cfg(test)]
#[path = "../tests/output_graveyard_test.rs"]
mod tests;
