use crate::cli::logic::{output_plan, style, table};
use crate::contracts::{Capability, CommandPlan, Harness};

pub fn list(harnesses: &[Harness]) -> String {
    if style::plain() {
        return harnesses
            .iter()
            .map(|harness| {
                format!(
                    "{} support={} - {}\n",
                    harness.name,
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
                harness.name.clone(),
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
    table::fields(
        &format!("Plan: {} {capability}", harness.name),
        &output_plan::fields(harness, plan, &command),
    )
}
