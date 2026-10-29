//! JsonCapability: one capability's truth as structure. `show` publishes the
//! whole record; `plan` publishes the same truth plus the argv a caller would
//! execute, so an agent can decide on evidence instead of on prose.

use super::json;
use crate::contracts::CapabilityPlan;

pub fn capability(plan: &CapabilityPlan) -> String {
    format!(
        "{{\"capability\":\"{}\",\"summary\":\"{}\",{},\"command\":[{}],\"command_line\":\"{}\",\"executable\":\"{}\",\"source\":\"{}\",\"verified_at\":\"{}\",\"package\":{}}}",
        plan.capability.as_str(),
        json::escape(&plan.summary),
        truth(plan),
        words(plan),
        json::escape(&plan.command.render()),
        json::escape(&plan.executable),
        json::escape(&plan.source),
        json::escape(&plan.verified_at),
        package(plan)
    )
}

pub fn plan(plan: &CapabilityPlan) -> String {
    format!(
        "{},\"command\":[{}],\"command_line\":\"{}\",\"executable\":\"{}\"",
        truth(plan),
        words(plan),
        json::escape(&plan.command.render()),
        json::escape(&plan.executable)
    )
}

fn truth(plan: &CapabilityPlan) -> String {
    let platforms = plan
        .platforms
        .iter()
        .map(|platform| format!("\"{}\"", json::escape(platform)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "\"support\":\"{}\",\"evidence\":\"{}\",\"effect\":\"{}\",\"network\":{},\"interaction\":\"{}\",\"platforms\":[{platforms}]",
        plan.support.as_str(),
        plan.evidence.as_str(),
        plan.effect.as_str(),
        plan.network,
        plan.interaction.as_str()
    )
}

fn words(plan: &CapabilityPlan) -> String {
    std::iter::once(&plan.command.command)
        .chain(plan.command.args.iter())
        .map(|word| format!("\"{}\"", json::escape(word)))
        .collect::<Vec<_>>()
        .join(",")
}

fn package(plan: &CapabilityPlan) -> String {
    match &plan.package {
        Some(package) => format!("\"{}\"", json::escape(package)),
        None => "null".to_string(),
    }
}
