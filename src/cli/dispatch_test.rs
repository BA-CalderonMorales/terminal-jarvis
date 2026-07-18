use super::*;
use crate::contracts::{Capability, CapabilityPlan, EnvMode, Harness};

fn cap(c: Capability) -> CapabilityPlan {
    crate::cli::test_support::plan(c, "sh", vec!["-c".into(), "exit 0".into()])
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
fn d(
    action: Action,
    harnesses: &[Harness],
    catalog: &std::path::Path,
    home: &std::path::Path,
) -> crate::cli::error::Result<(i32, String)> {
    dispatch(
        action,
        &crate::cli::args::Options::default(),
        harnesses,
        catalog,
        home,
    )
}

#[test]
fn list_check_help_legacy() {
    let hs = [harness("opencode")];
    let (p, h) = paths();
    assert_eq!(d(Action::List, &hs, p, h).unwrap().0, 0);
    assert_eq!(d(Action::Help, &hs, p, h).unwrap().0, 0);
    assert_eq!(
        d(Action::Legacy("templates".to_string()), &hs, p, h)
            .unwrap_err()
            .exit_code,
        4
    );
}
#[test]
fn security_routes() {
    let hs = [harness("opencode")];
    let (p, h) = paths();
    assert!(d(Action::Security(vec![]), &hs, p, h).is_ok());
    assert!(d(Action::Security(vec!["status".to_string()]), &hs, p, h).is_ok());
    assert!(d(Action::Security(vec!["audit".to_string()]), &hs, p, h).is_ok());
    let out = d(Action::Security(vec!["opencode".to_string()]), &hs, p, h)
        .unwrap()
        .1;
    assert!(out.contains("opencode"));
    assert!(d(
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
    assert!(d(Action::Auth(vec![]), &hs, p, h).is_ok());
    let (_, up) = d(Action::Update(None), &hs, p, h).unwrap();
    assert!(up.contains("opencode"));
    assert!(d(Action::Install("opencode".to_string()), &hs, p, h).is_err());
    assert!(d(Action::Update(Some("opencode".to_string())), &hs, p, h).is_err());
}
#[test]
fn direct_and_cache() {
    let hs = [harness("opencode")];
    let (p, h) = paths();
    assert!(d(
        Action::Direct {
            harness: "opencode".to_string(),
            extra: vec![]
        },
        &hs,
        p,
        h
    )
    .is_err());
    assert!(d(Action::Cache(vec![]), &hs, p, h).is_ok());
}
