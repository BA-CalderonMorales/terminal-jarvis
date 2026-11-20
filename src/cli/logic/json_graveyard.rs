//! GraveyardJson: the structured ledger behind `graveyard --json`. An agent
//! deciding whether a tool is still maintained should not parse prose, so the
//! retirement date, the successor, and whether the binary is still on PATH
//! each become a field beside the rendered text.

use super::{json, output};
use crate::contracts::{Harness, SunsetState};

pub fn document(harnesses: &[Harness]) -> String {
    let text = output::graveyard(harnesses);
    let today = crate::catalog::today_utc();
    let rows = harnesses
        .iter()
        .filter(|harness| harness.is_sunset())
        .map(|harness| entry(harness, &today))
        .collect::<Vec<_>>()
        .join(",");
    json::envelope(
        "graveyard",
        0,
        &format!(
            "{{\"text\":\"{}\",\"total\":{},\"retired\":[{rows}]}}",
            json::escape(&text),
            harnesses.len()
        ),
    )
}

fn entry(harness: &Harness, today: &str) -> String {
    let sunset = harness.sunset.as_ref().expect("filtered to sunset");
    let state = match sunset.state(today) {
        SunsetState::Done => "retired",
        SunsetState::Scheduled => "scheduled",
    };
    let replaced_by = match &sunset.replaced_by {
        Some(tool) => format!("\"{}\"", json::escape(tool)),
        None => "null".to_string(),
    };
    format!(
        "{{\"name\":\"{}\",\"state\":\"{state}\",\"display\":\"{}\",\"binary\":\"{}\",\"binary_on_path\":{},\"sunset\":{{\"at\":\"{}\",\"note\":\"{}\",\"replaced_by\":{replaced_by}}}}}",
        json::escape(&harness.name),
        json::escape(&harness.display),
        json::escape(&harness.binary),
        crate::security::command_on_path(&harness.binary),
        json::escape(&sunset.at),
        json::escape(&sunset.note)
    )
}
