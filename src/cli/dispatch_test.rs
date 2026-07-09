use super::*;
use crate::contracts::{Capability, CapabilityPlan, CommandPlan, EnvMode, Harness};

fn cap(c: Capability) -> CapabilityPlan {
    CapabilityPlan {
        capability: c,
        summary: c.as_str().to_string(),
        command: CommandPlan::new(
            "sh".to_string(),
            vec!["-c".to_string(), "exit 0".to_string()],
        ),
    }
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
fn paths() -> (&'static std::path::Path, &'static std::path::Path) {
    (std::path::Path::new("/cat"), std::path::Path::new("/home"))
}

#[test]
fn list_check_help_legacy() {
    let hs = [harness("opencode")];
    let (p, h) = paths();
    assert_eq!(dispatch(Action::List, &hs, p, h).unwrap().0, 0);
    assert_eq!(dispatch(Action::Check, &hs, p, h).unwrap().0, 0);
    assert_eq!(dispatch(Action::Help, &hs, p, h).unwrap().0, 0);
    let out = dispatch(Action::Legacy("templates".to_string()), &hs, p, h)
        .unwrap()
        .1;
    assert!(out.contains("removed"));
}
#[test]
fn security_routes() {
    let hs = [harness("opencode")];
    let (p, h) = paths();
    assert!(dispatch(Action::Security(vec![]), &hs, p, h).is_ok());
    assert!(dispatch(Action::Security(vec!["status".to_string()]), &hs, p, h).is_ok());
    assert!(dispatch(Action::Security(vec!["audit".to_string()]), &hs, p, h).is_ok());
    let out = dispatch(Action::Security(vec!["opencode".to_string()]), &hs, p, h)
        .unwrap()
        .1;
    assert!(out.contains("opencode"));
    assert!(dispatch(
        Action::Security(vec!["a".to_string(), "b".to_string()]),
        &hs,
        p,
        h
    )
    .is_err());
}
#[test]
fn auth_update_install() {
    let hs = [harness("opencode")];
    let (p, h) = paths();
    assert!(dispatch(Action::Auth(vec![]), &hs, p, h).is_ok());
    let (_, up) = dispatch(Action::Update(None), &hs, p, h).unwrap();
    assert!(up.contains("opencode"));
    assert!(dispatch(Action::Install("opencode".to_string()), &hs, p, h).is_ok());
    assert!(dispatch(Action::Update(Some("opencode".to_string())), &hs, p, h).is_ok());
}
#[test]
fn direct_and_cache() {
    let hs = [harness("opencode")];
    let (p, h) = paths();
    assert!(dispatch(
        Action::Direct {
            harness: "opencode".to_string(),
            extra: vec![]
        },
        &hs,
        p,
        h
    )
    .is_ok());
    assert!(dispatch(Action::Cache(vec![]), &hs, p, h).is_ok());
}
