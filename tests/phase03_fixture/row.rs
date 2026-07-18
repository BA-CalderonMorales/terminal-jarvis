use terminal_jarvis::contracts::{CapabilityPlan, Harness, SupportState};

pub struct Row {
    pub harness: String,
    pub capability: String,
    pub support: String,
    pub evidence: String,
    pub guard: String,
    pub argv: String,
    pub effect: String,
    pub platforms: String,
    pub executable: String,
    pub source: String,
    pub verified_at: String,
    pub summary: String,
    pub result: String,
}

impl Row {
    pub(super) fn from_plan(harness: &Harness, plan: &CapabilityPlan) -> Self {
        Self {
            harness: harness.name.clone(),
            capability: plan.capability.to_string(),
            support: plan.support.as_str().to_string(),
            evidence: plan.evidence.as_str().to_string(),
            guard: guard(plan.support).to_string(),
            argv: plan.command.render(),
            effect: format!(
                "{};network={};interaction={}",
                plan.effect.as_str(),
                plan.network,
                plan.interaction.as_str()
            ),
            platforms: if plan.platforms.is_empty() {
                "none".to_string()
            } else {
                plan.platforms.join(",")
            },
            executable: plan.executable.clone(),
            source: plan.source.clone(),
            verified_at: plan.verified_at.clone(),
            summary: plan.summary.clone(),
            result: "pass".to_string(),
        }
    }

    pub(super) fn tsv(&self, tested_ref: &str) -> String {
        let values = [
            "1",
            tested_ref,
            &self.harness,
            &self.capability,
            &self.support,
            &self.evidence,
            &self.guard,
            &self.argv,
            &self.effect,
            &self.platforms,
            &self.executable,
            &self.source,
            &self.verified_at,
            &self.summary,
            &self.result,
        ];
        values.map(clean).join("\t")
    }
}

fn guard(state: SupportState) -> &'static str {
    match state {
        SupportState::Verified | SupportState::Expected => "policy-and-platform-check",
        SupportState::Manual => "blocked:manual:exit-4",
        SupportState::Stub => "blocked:stub:exit-4",
        SupportState::Unsupported => "blocked:unsupported:exit-4",
        SupportState::Disabled => "blocked:disabled:exit-4",
        SupportState::Unknown => "blocked:unknown:exit-4",
    }
}

fn clean(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}
