use super::*;
use crate::contracts::Capability;

fn flat(input: &[&str]) -> Vec<String> {
    input.iter().map(|s| s.to_string()).collect()
}

fn g(words: &[&str], expected: Action) {
    assert_eq!(parse(flat(words)), Ok(expected), "green for {words:?}");
}

fn plan(harness: Option<&str>, capability: Capability) -> Action {
    Action::Plan {
        harness: harness.map(String::from),
        capability,
    }
}

#[test]
fn every_action_variant_roundtrips_green() {
    g(&["tj", "help"], Action::Help);
    g(&["tj", "list"], Action::List);
    g(&["tj", "check"], Action::Check);
    g(&["tj", "current"], Action::Current);
    g(&["tj", "version"], Action::Version { verbose: false });
    g(&["tj", "--info"], Action::Version { verbose: true });
    g(&["tj", "use", "opencode"], Action::Use("opencode".into()));
    g(
        &["tj", "show", "opencode"],
        Action::Show(Some("opencode".to_string())),
    );
    g(
        &["tj", "plan", "headless"],
        plan(None, Capability::Headless),
    );
    g(
        &["tj", "plan", "opencode", "update"],
        plan(Some("opencode"), Capability::Update),
    );
    g(&["tj", "run"], Action::Run(vec![]));
    g(&["tj", "run", "x"], Action::Run(vec!["x".into()]));
    g(
        &["tj", "opencode", "a", "b"],
        Action::Direct {
            harness: "opencode".into(),
            extra: vec!["a".into(), "b".into()],
        },
    );
    g(
        &["tj", "install", "opencode"],
        Action::Install(Some("opencode".to_string())),
    );
    g(
        &["tj", "self-update"],
        Action::SelfUpdate { dry_run: false },
    );
    g(&["tj", "update"], Action::Update(None));
    g(
        &["tj", "update", "opencode"],
        Action::Update(Some("opencode".into())),
    );
    g(&["tj", "auth", "set"], Action::Auth(vec!["set".into()]));
    g(
        &["tj", "config", "show"],
        Action::Config(vec!["show".into()]),
    );
    g(
        &["tj", "cache", "clear"],
        Action::Cache(vec!["clear".into()]),
    );
    g(&["tj", "security"], Action::Security(vec![]));
    g(
        &["tj", "gate", "status"],
        Action::Gate(vec!["status".into()]),
    );
    g(&["tj", "templates"], Action::Legacy("templates".into()));
}
