use super::*;
use crate::contracts::{Capability, CapabilityPlan, EnvMode, Harness};

fn cap(c: Capability) -> CapabilityPlan {
    crate::cli::logic::test_support::plan(c, "sh", vec!["-c".into(), "exit 0".into()])
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
    catalog: &Path,
    home: &Path,
) -> crate::cli::structs::error::Result<(i32, String)> {
    dispatch(action, &Options::default(), harnesses, catalog, home)
}

#[test]
fn face_dispatch_pins_exit_code_and_output() {
    let hs = [harness("opencode")];
    let (p, h) = paths();
    let (code, body) = crate::cli::dispatch(Action::List, &Options::default(), &hs, p, h)
        .expect("list dispatches cleanly");
    assert_eq!((code, body.contains("opencode")), (0, true));
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
    // bare update now targets the active harness: no session = explicit error
    assert!(d(Action::Update(None), &hs, p, h)
        .unwrap_err()
        .message
        .contains("no active harness"));
    assert!(d(Action::Install(Some("opencode".to_string())), &hs, p, h).is_err());
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
