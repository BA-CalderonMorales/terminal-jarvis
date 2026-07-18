use super::*;
use crate::contracts::{Capability, CapabilityPlan, EnvMode, Harness};

fn cap(c: Capability) -> CapabilityPlan {
    crate::cli::test_support::plan(c, c.as_str(), vec![])
}
fn harness(name: &str) -> Harness {
    Harness {
        name: name.to_string(),
        display: name.to_string(),
        description: String::new(),
        binary: name.to_string(),
        env_mode: EnvMode::None,
        env: vec![],
        capabilities: Capability::ALL.iter().map(|c| cap(*c)).collect(),
    }
}

#[test]
fn update_summary_lists_harnesses() {
    let mut guarded = harness("opencode");
    let out = update_summary(&[guarded.clone()]);
    assert!(out.contains("opencode"));
    assert!(out.contains("support=unknown evidence=deterministic command=withheld"));
    assert!(!out.contains("opencode: update"));

    let update = guarded
        .capabilities
        .iter_mut()
        .find(|plan| plan.capability == Capability::Update)
        .unwrap();
    update.support = crate::contracts::SupportState::Verified;
    update.command.command = "fixture-update".to_string();
    let verified = update_summary(&[guarded]);
    assert!(verified.contains("fixture-update"));
}
#[test]
fn auth_routes() {
    let hs = [harness("opencode")];
    assert!(auth(&[], &hs).is_ok());
    assert!(auth(&["manage".to_string()], &hs).is_ok());
    assert!(auth(&["help".to_string(), "opencode".to_string()], &hs).is_ok());
    assert!(auth(&["set".to_string(), "opencode".to_string()], &hs).is_ok());
    assert!(auth(&["opencode".to_string()], &hs).is_ok());
    assert!(auth(&["unknown".to_string()], &hs).is_err());
    assert!(auth(&["help".to_string(), "unknown".to_string()], &hs).is_err());
    assert!(auth(&["a".to_string(), "b".to_string(), "c".to_string()], &hs).is_err());
}
#[test]
fn config_routes() {
    let p = std::path::Path::new("/cat");
    let h = std::path::Path::new("/home");
    assert!(config(&[], p, h, None).is_ok());
    assert!(config(&["show".to_string()], p, h, None).is_ok());
    let out = config(&["path".to_string()], p, h, None).unwrap();
    assert!(out.contains("/cat") && out.contains("/home"));
    let reset = config(&["reset".to_string()], p, h, None).unwrap();
    assert!(reset.contains("not automatic"));
    assert!(config(&["bogus".to_string()], p, h, None).is_err());
}
