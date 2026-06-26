use terminal_jarvis::cli::args::{parse, Action};
use terminal_jarvis::contracts::Capability;

#[test]
fn empty_args_show_help() {
    assert_eq!(parse(["tj"]).unwrap(), Action::Help);
}

#[test]
fn parses_plan_with_explicit_harness() {
    assert_eq!(
        parse(["tj", "plan", "codex", "headless"]).unwrap(),
        Action::Plan {
            harness: Some("codex".to_string()),
            capability: Capability::Headless,
        }
    );
}

#[test]
fn parses_plan_for_active_harness() {
    assert_eq!(
        parse(["tj", "plan", "models"]).unwrap(),
        Action::Plan {
            harness: None,
            capability: Capability::Models,
        }
    );
}

#[test]
fn parses_run_with_extra_args() {
    assert_eq!(
        parse(["tj", "run", "codex", "headless", "fix", "tests"]).unwrap(),
        Action::Run {
            harness: "codex".to_string(),
            capability: Capability::Headless,
            extra: vec!["fix".to_string(), "tests".to_string()],
        }
    );
}

#[test]
fn rejects_unknown_capability() {
    let error = parse(["tj", "plan", "codex", "launch"]).unwrap_err();
    assert!(error.contains("unknown capability"));
}

#[test]
fn parses_every_core_capability() {
    for capability in Capability::ALL {
        assert_eq!(
            Capability::parse(capability.as_str()),
            Some(capability),
            "capability parse failed for {capability}"
        );
    }
}
