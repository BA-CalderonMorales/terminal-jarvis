use terminal_jarvis::contracts::{Capability, CapabilityPlan, CommandPlan, EnvMode, Harness};
use terminal_jarvis::{catalog, security};

fn plan(capability: Capability, summary: &str, command: &str) -> CapabilityPlan {
    CapabilityPlan {
        capability,
        summary: summary.to_string(),
        command: CommandPlan::new(command.to_string(), Vec::new()),
    }
}

fn harness(name: &str, mode: EnvMode, env: Vec<String>) -> Harness {
    Harness {
        name: name.to_string(),
        display: name.to_string(),
        description: name.to_string(),
        binary: "sh".to_string(),
        env_mode: mode,
        env,
        capabilities: Capability::ALL
            .iter()
            .map(|capability| plan(*capability, "Dangerous test plan", "sh"))
            .collect(),
    }
}

#[test]
fn validation_reports_contract_errors() {
    let bad = Harness {
        name: "bad".to_string(),
        display: "bad".to_string(),
        description: "bad".to_string(),
        binary: String::new(),
        env_mode: EnvMode::None,
        env: vec!["bad-env".to_string()],
        capabilities: vec![
            plan(Capability::Update, "update", "login"),
            plan(Capability::Yolo, "fast mode", "sh"),
        ],
    };
    let errors = catalog::validate(&[bad.clone(), bad]).join("\n");
    assert!(errors.contains("duplicate harness"));
    assert!(errors.contains("empty binary"));
    assert!(errors.contains("env vars with env_mode none"));
    assert!(errors.contains("invalid env"));
    assert!(errors.contains("missing a core capability"));
    assert!(errors.contains("update command looks interactive"));
    assert!(errors.contains("yolo summary must mention danger"));
}

#[test]
fn security_checks_cover_path_and_env_modes() {
    let current = std::env::current_exe().unwrap();
    assert!(security::command_on_path(current.to_str().unwrap()));
    assert!(!security::command_on_path(
        "/definitely/not/terminal-jarvis"
    ));
    assert!(security::missing_env(&harness("none", EnvMode::None, Vec::new())).is_empty());
    assert!(security::missing_env(&harness("any", EnvMode::Any, vec!["PATH".into()])).is_empty());
    let missing = security::missing_env(&harness(
        "all",
        EnvMode::All,
        vec!["PATH".into(), "__TERMINAL_JARVIS_MISSING_ENV__".into()],
    ));
    assert_eq!(missing, vec!["__TERMINAL_JARVIS_MISSING_ENV__"]);
}

#[test]
fn setup_hints_cover_all_env_modes() {
    assert_eq!(
        harness("none", EnvMode::None, Vec::new()).setup_hint(),
        "no API key required"
    );
    assert_eq!(
        harness("all", EnvMode::All, vec!["A".into(), "B".into()]).setup_hint(),
        "set all of: A, B"
    );
    assert!(EnvMode::parse("sometimes").is_err());
}
