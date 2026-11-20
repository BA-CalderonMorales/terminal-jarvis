use super::capability::Capability;
use super::command::CommandPlan;
use super::effect::{Effect, Interaction};
use super::environment::EnvMode;
use super::sunset::Sunset;
use super::support::{EvidenceMode, SupportState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityPlan {
    pub capability: Capability,
    pub summary: String,
    pub command: CommandPlan,
    pub support: SupportState,
    pub evidence: EvidenceMode,
    pub effect: Effect,
    pub network: bool,
    pub interaction: Interaction,
    pub platforms: Vec<String>,
    pub executable: String,
    pub source: String,
    pub verified_at: String,
    pub package: Option<String>,
    /// The tool's own uninstaller, when its docs provide one. Removal prefers
    /// this over a guessed package-manager inverse, so a script install can
    /// still be removed by the channel that owns it.
    pub uninstall: Option<CommandPlan>,
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
    /// Present once the upstream project retired the tool. A sunset harness
    /// stays in the catalog so its install channel and removal stay legible,
    /// but it is not part of the maintained fleet.
    pub sunset: Option<Sunset>,
}

impl Harness {
    pub fn plan(&self, capability: Capability) -> Option<&CapabilityPlan> {
        self.capabilities
            .iter()
            .find(|plan| plan.capability == capability)
    }

    /// Whether upstream has announced or completed a retirement. Both states
    /// keep the tool out of the maintained fleet; only the wording differs,
    /// and `Sunset::verb`/`adjective` own that.
    pub fn is_sunset(&self) -> bool {
        self.sunset.is_some()
    }

    /// The verb for this tool's retirement, or `None` while it is maintained.
    pub fn retirement_verb(&self, today: &str) -> Option<&'static str> {
        self.sunset.as_ref().map(|record| record.verb(today))
    }

    /// The adjective for this tool's retirement, or `None` while maintained.
    pub fn retirement_adjective(&self, today: &str) -> Option<&'static str> {
        self.sunset.as_ref().map(|record| record.adjective(today))
    }

    pub fn has_all_capabilities(&self) -> bool {
        Capability::ALL
            .iter()
            .all(|capability| self.plan(*capability).is_some())
    }

    pub fn setup_hint(&self) -> String {
        match (self.env_mode, self.env.is_empty()) {
            (EnvMode::None, _) | (_, true) => "no API key required".to_string(),
            (EnvMode::Optional, false) => format!("optional: {}", self.env.join(", ")),
            (EnvMode::Any, false) => format!("set one of: {}", self.env.join(", ")),
            (EnvMode::All, false) => format!("set all of: {}", self.env.join(", ")),
        }
    }
}
