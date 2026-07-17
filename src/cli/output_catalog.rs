use super::super::{output_plan, output_truth, style, table};
use crate::contracts::{Capability, CommandPlan, Harness};

pub fn list(harnesses: &[Harness]) -> String {
    if style::plain() {
        return harnesses
            .iter()
            .map(|harness| {
                format!(
                    "{} support={} - {}\n",
                    harness.name,
                    output_truth::support_summary(harness),
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
                output_truth::support_summary(harness),
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

pub fn show(harness: &Harness) -> String {
    if style::plain() {
        let mut out = format!(
            "{} ({})\n{}\nbinary: {}\nsetup: {}\nsupport: {}\n",
            harness.display,
            harness.name,
            harness.description,
            harness.binary,
            harness.setup_hint(),
            output_truth::support_summary(harness)
        );
        for plan in &harness.capabilities {
            out.push_str(&output_truth::plain_capability(plan));
        }
        return out;
    }
    let details = table::fields(
        &format!("{} ({})", harness.display, harness.name),
        &[
            ("DESCRIPTION", harness.description.clone()),
            ("BINARY", harness.binary.clone()),
            ("SETUP", harness.setup_hint()),
            ("SUPPORT", output_truth::support_summary(harness)),
        ],
    );
    let rows = harness
        .capabilities
        .iter()
        .map(output_truth::capability_row)
        .collect::<Vec<_>>();
    format!(
        "{details}\n{}",
        table::render(
            "Capability Truth",
            &[
                "CAPABILITY",
                "STATE",
                "EVIDENCE",
                "EFFECT",
                "PLATFORMS",
                "FRESHNESS"
            ],
            &rows
        )
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
