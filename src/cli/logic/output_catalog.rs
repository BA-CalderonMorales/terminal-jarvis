use crate::cli::logic::{output_fields as fields, output_plan, style, table, value_block};
use crate::contracts::{Capability, CommandPlan, Harness};

pub fn list(harnesses: &[Harness]) -> String {
    list_on(harnesses, &crate::catalog::today_utc())
}

/// `today` is a parameter so the status words and footer counts are pinned by
/// tests rather than by the clock.
pub fn list_on(harnesses: &[Harness], today: &str) -> String {
    if style::plain() {
        return harnesses
            .iter()
            .map(|harness| {
                format!(
                    "{} support={} status={} - {}\n",
                    harness.name,
                    crate::cli::logic::output_truth::support_summary(harness),
                    crate::cli::logic::output_truth::status(harness, today),
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
                crate::cli::logic::output_truth::status(harness, today).to_string(),
                harness.description.clone(),
            ]
        })
        .collect::<Vec<_>>();
    // NAME stays the bare name: repeating the state there wrapped onto a
    // second line that rendered as a nameless phantom row.
    let table = table::render(
        "Available Harnesses",
        &["NAME", "STATUS", "DESCRIPTION"],
        &rows,
    );
    format!("{table}{}", footer(harnesses, today))
}

/// Retired and retiring are counted apart: a tool that stops working next
/// year is not the same news as one that already has.
fn footer(harnesses: &[Harness], today: &str) -> String {
    let count = |word: &str| {
        harnesses
            .iter()
            .filter(|harness| harness.retirement_adjective(today) == Some(word))
            .count()
    };
    let (retired, retiring) = (count("retired"), count("retiring"));
    format!(
        "\n{} harnesses · {retired} retired · {retiring} retiring\n",
        harnesses.len()
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
    let width = fields::width();
    let mut lines = vec![format!("Plan: {} {capability}", harness.name)];
    lines.extend(value_block::card(
        &output_plan::groups(harness, plan, &command),
        width,
    ));
    lines.join("\n")
}
