use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EnvMode, EvidenceMode, Harness, Interaction,
    SupportState,
};

use super::parser::{self, Fields};

const HARNESS_KEYS: [&str; 6] = [
    "name",
    "display",
    "description",
    "binary",
    "env_mode",
    "env",
];
const PLAN_KEYS: [&str; 12] = [
    "summary",
    "command",
    "args",
    "support",
    "evidence",
    "effect",
    "network",
    "interaction",
    "platforms",
    "executable",
    "source",
    "verified_at",
];

pub fn harness(fields: &Fields, capabilities: Vec<CapabilityPlan>) -> Result<Harness, String> {
    exact_keys(fields, &HARNESS_KEYS)?;
    Ok(Harness {
        name: parser::string(fields, "name")?,
        display: parser::string(fields, "display")?,
        description: parser::string(fields, "description")?,
        binary: parser::string(fields, "binary")?,
        env_mode: EnvMode::parse(&parser::string(fields, "env_mode")?)?,
        env: parser::list(fields, "env")?,
        capabilities,
    })
}

pub fn capability(fields: &Fields, capability: Capability) -> Result<CapabilityPlan, String> {
    exact_keys(fields, &PLAN_KEYS)?;
    let command = parser::string(fields, "command")?;
    Ok(CapabilityPlan {
        capability,
        summary: parser::string(fields, "summary")?,
        command: CommandPlan::new(command, parser::list(fields, "args")?),
        support: SupportState::parse(&parser::string(fields, "support")?)?,
        evidence: EvidenceMode::parse(&parser::string(fields, "evidence")?)?,
        effect: Effect::parse(&parser::string(fields, "effect")?)?,
        network: boolean(fields, "network")?,
        interaction: Interaction::parse(&parser::string(fields, "interaction")?)?,
        platforms: parser::list(fields, "platforms")?,
        executable: parser::string(fields, "executable")?,
        source: parser::string(fields, "source")?,
        verified_at: parser::string(fields, "verified_at")?,
    })
}

fn exact_keys(fields: &Fields, expected: &[&str]) -> Result<(), String> {
    let actual = fields.keys().map(String::as_str).collect::<Vec<_>>();
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "metadata keys must be exactly {}; found {}",
            expected.join(", "),
            actual.join(", ")
        ))
    }
}

fn boolean(fields: &Fields, key: &str) -> Result<bool, String> {
    match fields.get(key).map(String::as_str) {
        Some("true") => Ok(true),
        Some("false") => Ok(false),
        Some(value) => Err(format!("'{key}' must be true or false, got {value}")),
        None => Err(format!("missing '{key}'")),
    }
}

#[cfg(test)]
#[path = "metadata_test.rs"]
mod tests;
