#[path = "output_plan_words.rs"]
mod words;
use super::output_truth;
use crate::cli::structs::value_block::{Group, Row};
use crate::contracts::{CapabilityPlan, CommandPlan, Effect, Harness};

pub fn retirement(harness: &Harness, today: &str) -> Option<String> {
    words::retirement(harness, today)
}

#[cfg(test)]
pub fn plain(harness: &Harness, plan: &CapabilityPlan, command: &CommandPlan) -> String {
    plain_on(harness, plan, command, &crate::catalog::today_utc())
}

pub fn plain_on(
    harness: &Harness,
    plan: &CapabilityPlan,
    command: &CommandPlan,
    today: &str,
) -> String {
    let retirement = words::retirement(harness, today)
        .map(|line| format!("retirement: {line}\n"))
        .unwrap_or_default();
    format!(
        "{}:{}\nsummary: {}\nsupport: {}\nevidence: {}\neffect: {}\nplatforms: {}\nexecutable: {}\nsource: {}\nverified_at: {}\ncommand: {}\n{retirement}env: {}\nintent: {}\n",
        harness.name,
        plan.capability,
        plan.summary,
        plan.support.as_str(),
        plan.evidence.as_str(),
        output_truth::effect(plan),
        output_truth::platforms(plan),
        plan.executable,
        plan.source,
        output_truth::freshness(plan),
        command.render(),
        harness.setup_hint(),
        intent(harness, plan)
    )
}

/// The plan as a card: the facts about the capability, then the command that
/// would run it.
#[cfg(test)]
pub fn groups(harness: &Harness, plan: &CapabilityPlan, command: &CommandPlan) -> Vec<Group> {
    groups_on(harness, plan, command, &crate::catalog::today_utc())
}

pub fn groups_on(
    harness: &Harness,
    plan: &CapabilityPlan,
    command: &CommandPlan,
    today: &str,
) -> Vec<Group> {
    // Before the flags: nobody should install a departing tool unread.
    let mut run = vec![Row::new("command", command.render())];
    if let Some(line) = words::retirement(harness, today) {
        run.push(Row::new("retirement", line));
    }
    run.push(Row::new("environment", harness.setup_hint()));
    run.push(Row::new("intent", intent(harness, plan)));
    vec![
        Group::new(
            "the facts",
            vec![
                Row::new("summary", plan.summary.clone()),
                Row::new("support", plan.support.as_str()),
                Row::new("evidence", plan.evidence.as_str()),
                Row::new("effect", output_truth::effect(plan)),
                Row::new("platforms", output_truth::platforms(plan)),
                Row::new("executable", plan.executable.clone()),
                Row::new("source", plan.source.clone()),
                Row::new("verified", output_truth::freshness(plan)),
            ],
        ),
        Group::new("the command", run),
    ]
}

fn intent(harness: &Harness, plan: &CapabilityPlan) -> String {
    match plan.effect {
        Effect::ReadOnly => "none".to_string(),
        Effect::StateChanging => {
            format!("--no-input --confirm={}:{}", plan.capability, harness.name)
        }
        Effect::Dangerous => format!(
            "--allow-dangerous --no-input --confirm={}:{}",
            plan.capability, harness.name
        ),
    }
}

#[cfg(test)]
#[path = "../tests/output_plan_test.rs"]
mod tests;
