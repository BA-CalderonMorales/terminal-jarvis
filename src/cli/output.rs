use crate::contracts::{Capability, Harness};
use crate::{context::Session, runtime, security};

pub fn help() -> &'static str {
    super::help::text()
}

pub fn list(harnesses: &[Harness]) -> String {
    let mut out = String::new();
    for harness in harnesses {
        out.push_str(&format!("{} - {}\n", harness.name, harness.description));
    }
    out
}

pub fn current(session: Option<Session>) -> String {
    session
        .map(|session| format!("active harness = {}\n", session.active_harness))
        .unwrap_or_else(|| "active harness = none\n".to_string())
}

pub fn show(harness: &Harness) -> String {
    let mut out = format!(
        "{} ({})\n{}\n",
        harness.display, harness.name, harness.description
    );
    out.push_str(&format!("setup: {}\n", harness.setup_hint()));
    out.push_str("agent loop:\n");
    for plan in runtime::planned_steps(harness) {
        out.push_str(&format!("  {}: {}\n", plan.capability, plan.summary));
    }
    out
}

pub fn plan(harness: &Harness, capability: Capability) -> String {
    let plan = harness
        .plan(capability)
        .expect("validated harness capability");
    format!(
        "{}:{}\n{}\ncommand: {}\nenv: {}\n",
        harness.name,
        capability,
        plan.summary,
        plan.command.render(),
        harness.setup_hint()
    )
}

pub fn checks(harnesses: &[Harness]) -> String {
    let mut out = String::new();
    for harness in harnesses {
        let binary = if security::command_on_path(&harness.binary) {
            "found"
        } else {
            "missing"
        };
        let env = env_status(harness, &security::missing_env(harness));
        out.push_str(&format!("{} binary={} env={}\n", harness.name, binary, env));
    }
    out
}

pub fn is_harness_ready(h: &Harness) -> bool {
    security::command_on_path(&h.binary) && security::missing_env(h).is_empty()
}

pub fn status(harnesses: &[Harness]) -> String {
    summary(harnesses, "status")
}

pub fn audit(harnesses: &[Harness]) -> String {
    summary(harnesses, "audit summary")
}

fn summary(harnesses: &[Harness], label: &str) -> String {
    let mut out = checks(harnesses);
    let ready = harnesses.iter().filter(|h| is_harness_ready(h)).count();
    out.push_str(&format!(
        "\n{}: {}/{} harnesses ready\n",
        label,
        ready,
        harnesses.len()
    ));
    out
}

fn env_status(harness: &Harness, missing: &[String]) -> String {
    if missing.is_empty() {
        return "ready".to_string();
    }
    match harness.env_mode {
        crate::contracts::EnvMode::Any => format!("missing one of {}", missing.join(", ")),
        crate::contracts::EnvMode::All => format!("missing {}", missing.join(", ")),
        crate::contracts::EnvMode::None => "ready".to_string(),
    }
}

#[cfg(test)]
#[path = "output_test.rs"]
mod tests;
