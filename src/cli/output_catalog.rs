use super::super::{style, table};
use crate::contracts::{Capability, Harness};
use crate::runtime;

pub fn list(harnesses: &[Harness]) -> String {
    if style::plain() {
        return harnesses
            .iter()
            .map(|harness| format!("{} - {}\n", harness.name, harness.description))
            .collect();
    }
    let rows = harnesses
        .iter()
        .map(|harness| {
            vec![
                harness.name.clone(),
                harness.display.clone(),
                harness.description.clone(),
            ]
        })
        .collect::<Vec<_>>();
    table::render(
        "Available Harnesses",
        &["NAME", "DISPLAY", "DESCRIPTION"],
        &rows,
    )
}

pub fn show(harness: &Harness) -> String {
    if style::plain() {
        return plain_show(harness);
    }
    let details = table::fields(
        &format!("{} ({})", harness.display, harness.name),
        &[
            ("DESCRIPTION", harness.description.clone()),
            ("BINARY", harness.binary.clone()),
            ("SETUP", harness.setup_hint()),
        ],
    );
    let rows = runtime::planned_steps(harness)
        .into_iter()
        .map(|plan| vec![plan.capability.to_string(), plan.summary.clone()])
        .collect::<Vec<_>>();
    format!(
        "{details}\n{}",
        table::render("Capabilities", &["CAPABILITY", "BEHAVIOR"], &rows)
    )
}

pub fn plan(harness: &Harness, capability: Capability) -> String {
    let plan = harness
        .plan(capability)
        .expect("validated harness capability");
    if style::plain() {
        return format!(
            "{}:{}\n{}\ncommand: {}\nenv: {}\n",
            harness.name,
            capability,
            plan.summary,
            plan.command.render(),
            harness.setup_hint()
        );
    }
    table::fields(
        &format!("Plan: {} {}", harness.name, capability),
        &[
            ("SUMMARY", plan.summary.clone()),
            ("COMMAND", plan.command.render()),
            ("ENVIRONMENT", harness.setup_hint()),
        ],
    )
}

fn plain_show(harness: &Harness) -> String {
    let mut out = format!(
        "{} ({})\n{}\nsetup: {}\nagent loop:\n",
        harness.display,
        harness.name,
        harness.description,
        harness.setup_hint()
    );
    for plan in runtime::planned_steps(harness) {
        out.push_str(&format!("  {}: {}\n", plan.capability, plan.summary));
    }
    out
}
