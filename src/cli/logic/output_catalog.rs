use crate::cli::logic::{output_fields as fields, output_plan, style, table, value_block};
use crate::contracts::{Capability, CommandPlan, Harness};

pub fn list(harnesses: &[Harness]) -> String {
    if style::plain() {
        return harnesses
            .iter()
            .map(|harness| {
                format!(
                    "{} support={} - {}\n",
                    label(harness),
                    crate::cli::logic::output_truth::support_summary(harness),
                    harness.description
                )
            })
            .collect();
    }
    let rows = harnesses
        .iter()
        .map(|harness| {
            vec![
                label(harness),
                crate::cli::logic::output_truth::support_summary(harness),
                harness.description.clone(),
            ]
        })
        .collect::<Vec<_>>();
    table::render(
        "Available Harnesses",
        &["NAME", "SUPPORT", "DESCRIPTION"],
        &rows,
    )
}

fn label(harness: &Harness) -> String {
    if harness.is_sunset() {
        format!("{} (sunset)", harness.name)
    } else {
        harness.name.clone()
    }
}

pub fn plan(harness: &Harness, capability: Capability) -> String {
    plan_with_extra(harness, capability, &[])
}

pub fn plan_with_extra(harness: &Harness, capability: Capability, extra: &[String]) -> String {
    let plan = harness
        .plan(capability)
        .expect("validated harness capability");
    let mut command = CommandPlan::new(plan.command.command.clone(), plan.command.args.clone());
    command.args.extend_from_slice(extra);
    if style::plain() {
        return output_plan::plain(harness, plan, &command);
    }
    let width = fields::width();
    let mut lines = vec![format!("Plan: {} {capability}", harness.name)];
    lines.extend(value_block::card(
        &output_plan::groups(harness, plan, &command),
        width,
    ));
    lines.join("\n")
}
