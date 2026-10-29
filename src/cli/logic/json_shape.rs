//! JsonShape: the harness objects the catalog documents embed. Fields are
//! emitted in a fixed order so a document diffs cleanly, and the support
//! counts follow the dashboard's state order rather than hash order.

use super::json;
use super::json_capability::capability;
use crate::contracts::{Harness, SupportState};

const ORDER: [SupportState; 7] = [
    SupportState::Verified,
    SupportState::Expected,
    SupportState::Manual,
    SupportState::Stub,
    SupportState::Unsupported,
    SupportState::Unknown,
    SupportState::Disabled,
];

pub fn summary(harness: &Harness) -> String {
    format!(
        "{{{},{},\"capabilities\":[{}]}}",
        facts(harness),
        supports(harness),
        names(harness)
    )
}

pub fn detail(harness: &Harness) -> String {
    let capabilities = harness
        .capabilities
        .iter()
        .map(capability)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{{},\"setup\":\"{}\",{},\"capabilities\":[{capabilities}]}}",
        facts(harness),
        json::escape(&harness.setup_hint()),
        supports(harness)
    )
}

fn facts(harness: &Harness) -> String {
    format!(
        "\"name\":\"{}\",\"display\":\"{}\",\"description\":\"{}\",\"binary\":\"{}\"",
        json::escape(&harness.name),
        json::escape(&harness.display),
        json::escape(&harness.description),
        json::escape(&harness.binary)
    )
}

fn names(harness: &Harness) -> String {
    harness
        .capabilities
        .iter()
        .map(|plan| format!("\"{}\"", plan.capability.as_str()))
        .collect::<Vec<_>>()
        .join(",")
}

fn supports(harness: &Harness) -> String {
    let pairs = ORDER
        .iter()
        .filter_map(|state| {
            let count = harness
                .capabilities
                .iter()
                .filter(|plan| plan.support == *state)
                .count();
            (count > 0).then(|| format!("\"{}\":{count}", state.as_str()))
        })
        .collect::<Vec<_>>();
    format!("\"support\":{{{}}}", pairs.join(","))
}
