use crate::contracts::{
    Capability, CapabilityPlan, CommandPlan, Effect, EnvMode, EvidenceMode, Harness, Interaction,
    SupportState,
};

use crate::catalog::logic::parser::{self, Fields};
use crate::catalog::structs::keys;

pub fn harness(fields: &Fields, capabilities: Vec<CapabilityPlan>) -> Result<Harness, String> {
    keys::exact_fields(fields, keys::harness_keys())?;
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
    keys::plan_keys(fields)?;
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
        package: parser::optional_string(fields, "package")?,
    })
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
#[path = "../tests/metadata_test.rs"]
mod tests;
