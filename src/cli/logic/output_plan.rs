use super::output_truth;
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

pub fn fields(
    harness: &Harness,
    plan: &CapabilityPlan,
    command: &CommandPlan,
) -> Vec<(&'static str, String)> {
    vec![
        ("SUMMARY", plan.summary.clone()),
        ("SUPPORT", plan.support.as_str().to_string()),
        ("EVIDENCE", plan.evidence.as_str().to_string()),
        ("EFFECT", output_truth::effect(plan)),
        ("PLATFORMS", output_truth::platforms(plan)),
        ("EXECUTABLE", plan.executable.clone()),
        ("SOURCE", plan.source.clone()),
        ("VERIFIED AT", output_truth::freshness(plan)),
        ("COMMAND", command.render()),
        ("ENVIRONMENT", harness.setup_hint()),
        ("INTENT", intent(harness, plan)),
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
