use super::*;
use crate::contracts::Capability;

fn a(args: &[&str]) -> Action {
    parse(args.iter().map(|s| s.to_string())).unwrap()
}
fn e(args: &[&str]) -> Result<Action, String> {
    parse(args.iter().map(|s| s.to_string()))
}

#[test]
fn plan_run_install() {
    assert_eq!(
        a(&["tj", "plan", "update"]),
        Action::Plan {
            harness: None,
            capability: Capability::Update
        }
    );
    assert_eq!(
        a(&["tj", "plan", "opencode", "update"]),
        Action::Plan {
            harness: Some("opencode".to_string()),
            capability: Capability::Update
        }
    );
    assert!(e(&["tj", "plan"]).is_err());
    assert!(e(&["tj", "plan", "opencode", "bogus"]).is_err());
    assert_eq!(a(&["tj", "run"]), Action::Run(vec![]));
    assert_eq!(
        a(&["tj", "run", "opencode", "prompt"]),
        Action::Run(vec!["opencode".to_string(), "prompt".to_string()])
    );
    assert_eq!(
        a(&["tj", "install", "opencode"]),
        Action::Install("opencode".to_string())
    );
    assert!(e(&["tj", "install"]).is_err());
}
#[test]
fn update_auth_config_cache_security_legacy() {
    assert_eq!(a(&["tj", "update"]), Action::Update(None));
    assert_eq!(
        a(&["tj", "update", "opencode"]),
        Action::Update(Some("opencode".to_string()))
    );
    assert!(e(&["tj", "update", "a", "b"]).is_err());
    assert_eq!(
        a(&["tj", "--update"]),
        Action::SelfUpdate { dry_run: false }
    );
    assert_eq!(
        a(&["tj", "--update", "--dry-run"]),
        Action::SelfUpdate { dry_run: true }
    );
    assert_eq!(a(&["tj", "auth"]), Action::Auth(vec![]));
    assert_eq!(a(&["tj", "auth", "x"]), Action::Auth(vec!["x".to_string()]));
    assert_eq!(a(&["tj", "config"]), Action::Config(vec![]));
    assert_eq!(
        a(&["tj", "cache", "clear"]),
        Action::Cache(vec!["clear".to_string()])
    );
    assert_eq!(a(&["tj", "security"]), Action::Security(vec![]));
    assert_eq!(
        a(&["tj", "security", "status"]),
        Action::Security(vec!["status".to_string()])
    );
    assert_eq!(
        a(&["tj", "templates"]),
        Action::Legacy("templates".to_string())
    );
    assert_eq!(a(&["tj", "db"]), Action::Legacy("db".to_string()));
}
#[test]
fn version_and_update_reject_unexpected_trailing_args() {
    assert!(e(&["tj", "--version", "version"]).is_err());
    assert!(e(&["tj", "--update", "foo"]).is_err());
}

#[test]
fn help_routing_and_direct_and_flag() {
    for sub in [
        "list", "tools", "check", "status", "current", "use", "show", "info", "plan", "install",
        "update", "auth", "config", "cache", "security",
    ] {
        assert_eq!(a(&["tj", sub, "--help"]), Action::Help);
        assert_eq!(a(&["tj", sub, "-h"]), Action::Help);
    }
    assert_eq!(
        a(&["tj", "opencode", "do", "thing"]),
        Action::Direct {
            harness: "opencode".to_string(),
            extra: vec!["do".to_string(), "thing".to_string()]
        }
    );
    assert!(e(&["tj", "--bogus"]).is_err());
}
