use crate::contracts::{Capability, EnvMode, Harness};
use std::collections::BTreeSet;

pub fn validate(harnesses: &[Harness]) -> Vec<String> {
    let mut errors = Vec::new();
    let mut names = BTreeSet::new();
    if harnesses.is_empty() {
        errors.push("harness catalog is empty".to_string());
    }
    for harness in harnesses {
        if !names.insert(harness.name.as_str()) {
            errors.push(format!("duplicate harness '{}'", harness.name));
        }
        if harness.binary.trim().is_empty() {
            errors.push(format!("{} has an empty binary", harness.name));
        }
        if harness.env_mode == EnvMode::None && !harness.env.is_empty() {
            errors.push(format!("{} has env vars with env_mode none", harness.name));
        }
        validate_env(&harness.name, &harness.env, &mut errors);
        if !harness.has_all_capabilities() {
            errors.push(format!("{} is missing a core capability", harness.name));
        }
        validate_plans(harness, &mut errors);
    }
    errors
}

fn validate_plans(harness: &Harness, errors: &mut Vec<String>) {
    for plan in &harness.capabilities {
        if plan.command.command.trim().is_empty() {
            errors.push(format!(
                "{}:{} has an empty command",
                harness.name, plan.capability
            ));
        }
        if plan.capability == Capability::Update && has_interactive_word(&plan.command.render()) {
            errors.push(format!("{} update command looks interactive", harness.name));
        }
        if plan.capability == Capability::Yolo && !mentions_danger(&plan.summary) {
            errors.push(format!("{} yolo summary must mention danger", harness.name));
        }
    }
}

fn validate_env(harness: &str, names: &[String], errors: &mut Vec<String>) {
    for name in names {
        if !name
            .chars()
            .all(|char| char.is_ascii_uppercase() || char == '_')
        {
            errors.push(format!("{harness} has invalid env {name}"));
        }
    }
}

fn has_interactive_word(command: &str) -> bool {
    ["login", "auth", "configure", "wizard", "prompt"]
        .iter()
        .any(|word| command.to_ascii_lowercase().contains(word))
}

fn mentions_danger(summary: &str) -> bool {
    summary.to_ascii_lowercase().contains("danger")
}
