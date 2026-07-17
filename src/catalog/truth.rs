use crate::contracts::{CapabilityPlan, EvidenceMode, SupportState};
use std::collections::BTreeSet;

use super::freshness;

const PLATFORMS: [&str; 5] = [
    "linux-x64-gnu",
    "linux-arm64-gnu",
    "macos-x64",
    "macos-arm64",
    "windows-x64-msvc",
];

pub fn validate(harness: &str, plan: &CapabilityPlan, errors: &mut Vec<String>) {
    let prefix = format!("{harness}:{}", plan.capability);
    required(&prefix, "executable", &plan.executable, errors);
    required(&prefix, "source", &plan.source, errors);
    if plan.executable != plan.command.command {
        errors.push(format!("{prefix} executable must match command"));
    }
    if !freshness::valid_utc(&plan.verified_at) {
        errors.push(format!("{prefix} verified_at must be a UTC timestamp"));
    }
    if matches!(
        plan.support,
        SupportState::Verified | SupportState::Expected | SupportState::Manual
    ) && freshness::status(plan) != "fresh"
    {
        errors.push(format!(
            "{prefix} executable support evidence must be fresh"
        ));
    }
    validate_evidence(&prefix, plan, errors);
    validate_platforms(&prefix, plan, errors);
    super::command_truth::validate(&prefix, plan, errors);
    super::effect_truth::validate(&prefix, plan, errors);
}

fn validate_evidence(prefix: &str, plan: &CapabilityPlan, errors: &mut Vec<String>) {
    let expected = match plan.support {
        SupportState::Verified => EvidenceMode::DisposableReal,
        SupportState::Manual => EvidenceMode::Manual,
        SupportState::Unsupported => EvidenceMode::Unsupported,
        SupportState::Expected
        | SupportState::Stub
        | SupportState::Disabled
        | SupportState::Unknown => EvidenceMode::Deterministic,
    };
    if plan.evidence != expected {
        errors.push(format!("{prefix} support and evidence contradict"));
    }
}

fn validate_platforms(prefix: &str, plan: &CapabilityPlan, errors: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    for platform in &plan.platforms {
        if !PLATFORMS.contains(&platform.as_str()) {
            errors.push(format!("{prefix} has unknown platform {platform}"));
        } else if !seen.insert(platform) {
            errors.push(format!("{prefix} has duplicate platform {platform}"));
        }
    }
    let claims_support = matches!(
        plan.support,
        SupportState::Verified | SupportState::Expected | SupportState::Manual
    );
    if claims_support && plan.platforms.is_empty() {
        errors.push(format!("{prefix} support claim has no platform"));
    }
    if matches!(
        plan.support,
        SupportState::Unsupported | SupportState::Unknown
    ) && !plan.platforms.is_empty()
    {
        errors.push(format!("{prefix} guarded state cannot claim a platform"));
    }
}

fn required(prefix: &str, name: &str, value: &str, errors: &mut Vec<String>) {
    if value.trim().is_empty() {
        errors.push(format!("{prefix} has an empty {name}"));
    }
}
