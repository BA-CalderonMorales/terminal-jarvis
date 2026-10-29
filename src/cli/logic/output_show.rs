//! Show: the harness detail surface. The rich card is grouped -- the facts,
//! the support, then what the harness can do -- so a card reads as sections
//! instead of one wall of peers. `--plain` keeps the key=value contract.

use super::super::value_block;
use super::super::{output_fields as fields, style};
use crate::cli::structs::value_block::{Group, Row};
use crate::contracts::Harness;

/// One row per support state that occurs, in a fixed order.
fn support_rows(harness: &Harness) -> Vec<Row> {
    use crate::contracts::SupportState;
    let order = [
        SupportState::Verified,
        SupportState::Expected,
        SupportState::Manual,
        SupportState::Stub,
        SupportState::Unsupported,
        SupportState::Unknown,
        SupportState::Disabled,
    ];
    let mut counts = order.map(|state| (state, 0));
    for plan in &harness.capabilities {
        for entry in counts.iter_mut() {
            if entry.0 == plan.support {
                entry.1 += 1;
            }
        }
    }
    counts
        .iter()
        .filter(|(_, count)| *count > 0)
        .map(|(state, count)| Row::new(state.as_str(), count.to_string()))
        .collect()
}

/// One row per capability, under the capability's own name.
fn capability_rows(harness: &Harness) -> Vec<Row> {
    harness
        .capabilities
        .iter()
        .map(|plan| Row::new(&plan.capability.to_string(), plan.summary.clone()))
        .collect()
}

pub fn show(harness: &Harness) -> String {
    if style::plain() {
        let mut out = format!(
            "{} ({})\n{}\nbinary: {}\nsetup: {}\nsupport: {}\n",
            harness.display,
            harness.name,
            harness.description,
            harness.binary,
            harness.setup_hint(),
            crate::cli::logic::output_truth::support_summary(harness)
        );
        if let Some(sunset) = &harness.sunset {
            out.push_str(&format!(
                "retirement: retired {} · replaced by {}\n",
                sunset.at,
                sunset.successor()
            ));
        }
        for plan in &harness.capabilities {
            out.push_str(&crate::cli::logic::output_truth::plain_capability(plan));
        }
        return out;
    }
    let width = fields::width();
    let mut lines = vec![format!("{} ({})", harness.display, harness.name)];
    lines.extend(fields::section(String::new(), &harness.description, width));
    lines.push(String::new());
    lines.extend(value_block::card(
        &[
            Group::new(
                "the facts",
                [
                    Row::new("binary", &harness.binary),
                    Row::new("setup", harness.setup_hint()),
                ]
                .into_iter()
                .chain(super::graveyard::retirement(harness))
                .collect(),
            ),
            Group::new("the support", support_rows(harness)),
            Group::new("what it can do", capability_rows(harness)),
        ],
        width,
    ));
    lines.join("\n")
}
