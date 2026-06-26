use super::{Capability, CommandPlan, EnvMode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityPlan {
    pub capability: Capability,
    pub summary: String,
    pub command: CommandPlan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Harness {
    pub name: String,
    pub display: String,
    pub description: String,
    pub binary: String,
    pub env_mode: EnvMode,
    pub env: Vec<String>,
    pub capabilities: Vec<CapabilityPlan>,
}

impl Harness {
    pub fn plan(&self, capability: Capability) -> Option<&CapabilityPlan> {
        self.capabilities
            .iter()
            .find(|plan| plan.capability == capability)
    }

    pub fn has_all_capabilities(&self) -> bool {
        Capability::ALL
            .iter()
            .all(|capability| self.plan(*capability).is_some())
    }

    pub fn setup_hint(&self) -> String {
        match (self.env_mode, self.env.is_empty()) {
            (EnvMode::None, _) | (_, true) => "no API key required".to_string(),
            (EnvMode::Any, false) => format!("set one of: {}", self.env.join(", ")),
            (EnvMode::All, false) => format!("set all of: {}", self.env.join(", ")),
        }
    }
}
