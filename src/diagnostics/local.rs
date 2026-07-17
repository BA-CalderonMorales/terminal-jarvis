use super::{DiagnosticInput, Environment, HarnessInput, PlatformInput};
use crate::contracts::Harness;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

impl PlatformInput {
    pub fn local() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            libc: crate::platform::libc().to_string(),
            wsl: crate::platform::wsl().to_string(),
        }
    }
}

impl From<&Harness> for HarnessInput {
    fn from(harness: &Harness) -> Self {
        let support = harness
            .capabilities
            .iter()
            .filter(|plan| !plan.executable.trim().is_empty())
            .map(|plan| {
                (
                    plan.capability.as_str().to_string(),
                    plan.support.as_str().to_string(),
                    executable_support(plan),
                )
            })
            .collect();
        Self {
            name: harness.name.clone(),
            binary: harness.binary.clone(),
            env_mode: harness.env_mode,
            env: harness.env.clone(),
            support,
        }
    }
}

fn executable_support(plan: &crate::contracts::CapabilityPlan) -> bool {
    matches!(
        plan.support,
        crate::contracts::SupportState::Verified | crate::contracts::SupportState::Expected
    ) && crate::catalog::freshness_status(plan) == "fresh"
        && crate::platform::id()
            .is_some_and(|platform| plan.platforms.iter().any(|candidate| candidate == platform))
}

impl DiagnosticInput {
    pub fn local(
        catalog: &Path,
        home: &Path,
        active: Option<String>,
        harnesses: &[Harness],
        runtime: super::RuntimeInput,
    ) -> Self {
        let environment = Environment::process();
        let home_prefix =
            prefix(&environment, "HOME").or_else(|| prefix(&environment, "USERPROFILE"));
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            executable: std::env::current_exe().ok(),
            catalog: catalog.to_path_buf(),
            home: home.to_path_buf(),
            config: home.join("session.toml"),
            home_prefix,
            temp_prefix: Some(std::env::temp_dir()),
            active_harness: active,
            harnesses: harnesses.iter().map(HarnessInput::from).collect(),
            platform: PlatformInput::local(),
            environment,
            runtime,
            now: SystemTime::now(),
            stale_after: Duration::from_secs(30 * 24 * 60 * 60),
        }
    }
}

fn prefix(environment: &Environment, name: &str) -> Option<PathBuf> {
    environment
        .text(name)
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
}
