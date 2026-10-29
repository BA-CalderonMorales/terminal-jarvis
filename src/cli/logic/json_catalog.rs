//! CatalogJson: the structured data behind `--json` for the catalog surface.
//! An agent deciding which harness to run should not have to re-parse the
//! human summary, so each document carries the same `text` a person sees plus
//! the fields behind it. A command that cannot resolve its harness returns
//! `None` and lets the ordinary failure envelope explain why.

use super::{dispatch_support, json, output};
use super::{json_capability, json_shape};
use crate::cli::args;
use crate::contracts::{Capability, Harness};
use std::path::Path;

pub fn document(action: &args::Action, harnesses: &[Harness], home: &Path) -> Option<String> {
    match action {
        args::Action::List => Some(list(harnesses)),
        args::Action::Show(name) => named(name.clone(), home, harnesses).map(show),
        args::Action::Plan {
            harness,
            capability,
        } => named(harness.clone(), home, harnesses).map(|found| plan(found, *capability)),
        _ => None,
    }
}

fn named<'a>(
    explicit: Option<String>,
    home: &Path,
    harnesses: &'a [Harness],
) -> Option<&'a Harness> {
    let selected = dispatch_support::selected_name(explicit, home).ok()?;
    dispatch_support::find(harnesses, &selected).ok()
}

fn list(harnesses: &[Harness]) -> String {
    let text = output::list(harnesses);
    let rows = harnesses
        .iter()
        .map(json_shape::summary)
        .collect::<Vec<_>>()
        .join(",");
    json::envelope(
        "list",
        0,
        &format!(
            "{{\"text\":\"{}\",\"harnesses\":[{rows}]}}",
            json::escape(&text)
        ),
    )
}

fn show(harness: &Harness) -> String {
    let text = output::show(harness);
    json::envelope(
        "show",
        0,
        &format!(
            "{{\"text\":\"{}\",\"harness\":{}}}",
            json::escape(&text),
            json_shape::detail(harness)
        ),
    )
}

fn plan(harness: &Harness, capability: Capability) -> String {
    let text = output::plan(harness, capability);
    let target = format!(
        "\"harness\":\"{}\",\"capability\":\"{}\"",
        json::escape(&harness.name),
        capability.as_str()
    );
    let body = match harness.plan(capability) {
        Some(plan) => format!("{target},{}", json_capability::plan(plan)),
        None => target,
    };
    json::envelope(
        "plan",
        0,
        &format!("{{\"text\":\"{}\",{body}}}", json::escape(&text)),
    )
}

#[cfg(test)]
#[path = "../tests/json_catalog_test.rs"]
mod tests;
