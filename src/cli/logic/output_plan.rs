use super::output_truth;
use crate::cli::structs::value_block::{Group, Row};
use crate::contracts::{CapabilityPlan, CommandPlan, Effect, Harness};

pub fn plain(harness: &Harness, plan: &CapabilityPlan, command: &CommandPlan) -> String {
    format!(
        "{}:{}\nsummary: {}\nsupport: {}\nevidence: {}\neffect: {}\nplatforms: {}\nexecutable: {}\nsource: {}\nverified_at: {}\ncommand: {}\nenv: {}\nintent: {}\n",
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
pub fn groups(harness: &Harness, plan: &CapabilityPlan, command: &CommandPlan) -> Vec<Group> {
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
        Group::new(
            "the command",
            vec![
                Row::new("command", command.render()),
                Row::new("environment", harness.setup_hint()),
                Row::new("intent", intent(harness, plan)),
            ],
        ),
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
