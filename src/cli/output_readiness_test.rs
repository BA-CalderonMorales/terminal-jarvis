use super::*;
use crate::contracts::{Capability, EnvMode};

fn harness(state: SupportState, verified_at: &str, platform: &str) -> Harness {
    let path = std::env::temp_dir().join(format!(
        "tj-readiness-{}-{}",
        std::process::id(),
        state.as_str()
    ));
    std::fs::write(&path, "#!/bin/sh\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let binary = path.to_string_lossy().to_string();
    let mut plan = crate::cli::test_support::plan(Capability::Version, &binary, vec![]);
    plan.support = state;
    plan.verified_at = verified_at.into();
    plan.platforms = vec![platform.into()];
    Harness {
        name: "fixture".into(),
        display: "Fixture".into(),
        description: String::new(),
        binary,
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: vec![plan],
    }
}

#[test]
fn support_freshness_and_platform_are_part_of_readiness() {
    let platform = crate::platform::id().unwrap();
    let other = if platform == "windows-x64-msvc" {
        "linux-x64-gnu"
    } else {
        "windows-x64-msvc"
    };
    assert!(!is_harness_ready(&harness(
        SupportState::Unknown,
        "2026-07-17T04:59:27Z",
        platform,
    )));
    assert!(!is_harness_ready(&harness(
        SupportState::Expected,
        "2020-01-01T00:00:00Z",
        platform,
    )));
    assert!(!is_harness_ready(&harness(
        SupportState::Expected,
        "2026-07-17T04:59:27Z",
        other,
    )));
}
