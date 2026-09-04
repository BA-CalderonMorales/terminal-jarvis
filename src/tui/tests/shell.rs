use super::*;

fn harness(name: &str) -> Harness {
    Harness {
        name: name.into(),
        display: name.into(),
        description: "probe".into(),
        binary: name.into(),
        env_mode: crate::contracts::EnvMode::None,
        env: vec![],
        capabilities: vec![],
    }
}

fn action_of(input: &str, harnesses: &[Harness]) -> Option<args::Action> {
    match resolve(input, harnesses) {
        Resolved::Run(action) => Some(action),
        _ => None,
    }
}

#[test]
fn empty_lines_and_exit_verbs_terminate_the_mapping() {
    assert!(matches!(resolve("", &[]), Resolved::Empty));
    assert!(matches!(resolve("  ", &[]), Resolved::Empty));
    for verb in ["/exit", "/quit", "exit", "quit"] {
        assert!(matches!(resolve(verb, &[]), Resolved::Exit), "{verb}");
    }
}

#[test]
fn bare_and_slashed_home_and_clear_route_to_the_welcome() {
    for verb in ["/home", "/clear", "home", "clear"] {
        assert!(matches!(resolve(verb, &[]), Resolved::Home), "{verb}");
    }
}

#[test]
fn slash_lines_map_through_the_cli_grammar() {
    assert_eq!(action_of("/list", &[]), Some(args::Action::List));
    assert_eq!(action_of("/status", &[]), Some(args::Action::Check));
    let used = Some(args::Action::Use("codex".into()));
    assert_eq!(action_of("/use codex", &[]), used);
    assert!(matches!(
        action_of("/bogus", &[]),
        Some(args::Action::Direct { .. })
    ));
}

#[test]
fn broken_slash_lines_report_errors() {
    assert!(matches!(resolve("/use", &[]), Resolved::Error(_)));
}

#[test]
fn bare_numbers_select_the_harness_at_that_position() {
    let harnesses = [harness("alpha"), harness("beta")];
    assert_eq!(
        action_of("2", &harnesses),
        Some(args::Action::Use("beta".into()))
    );
    assert!(matches!(resolve("9", &harnesses), Resolved::Error(_)));
}

#[test]
fn bare_command_words_work_without_the_slash() {
    assert_eq!(action_of("list", &[]), Some(args::Action::List));
    assert_eq!(action_of("status", &[]), Some(args::Action::Check));
    assert_eq!(
        action_of("use codex", &[]),
        Some(args::Action::Use("codex".into()))
    );
    assert_eq!(
        action_of("show opencode", &[]),
        Some(args::Action::Show("opencode".into()))
    );
}

#[test]
fn bare_harness_names_switch_to_that_harness() {
    let harnesses = [harness("alpha"), harness("beta")];
    assert_eq!(
        action_of("alpha", &harnesses),
        Some(args::Action::Use("alpha".into()))
    );
    assert_eq!(
        action_of("beta", &harnesses),
        Some(args::Action::Use("beta".into()))
    );
}

// bare free text -> Run: covered by `bare_unknown_words_run_the_active_agent`.
#[test]
fn help_text_lists_commands_and_the_gate_line() {
    let body = crate::tui::shell::help::text();
    assert!(body.contains("Commands"));
    assert!(body.contains("exit | quit"));
    assert!(body.contains("Trivy gate"));
    assert!(body.contains("install <harness>"));
}
