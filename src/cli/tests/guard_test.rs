use super::*;

fn harnesses() -> Vec<Harness> {
    vec![Harness {
        name: "opencode".to_string(),
        display: "opencode".to_string(),
        description: String::new(),
        binary: "opencode".to_string(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }]
}

#[test]
fn explicit_capability_requires_known_harness_and_parsable_capability() {
    let hs = harnesses();
    assert!(!explicit_capability(&[], &hs));
    assert!(!explicit_capability(
        &["other".into(), "install".into()],
        &hs
    ));
    assert!(!explicit_capability(
        &["opencode".into(), "bogus".into()],
        &hs
    ));
    assert!(explicit_capability(
        &["opencode".into(), "download".into()],
        &hs
    ));
}

#[test]
fn resolve_error_distinguishes_active_harness_state_from_unknown() {
    assert_eq!(
        resolve_error("no active harness for this run".into()).exit_code,
        3
    );
    assert_eq!(
        resolve_error("active harness has no confirmed session".into()).exit_code,
        3
    );
    assert_eq!(resolve_error("wow such failure".into()).exit_code, 4);
}
