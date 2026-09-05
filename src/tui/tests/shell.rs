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
        Resolved::Run(action, _) => Some(action),
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
    assert!(matches!(resolve("/use", &[]), Resolved::Error(_)));
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
    assert_eq!(
        action_of("/use codex", &[]),
        Some(args::Action::Use("codex".into()))
    );
    assert!(matches!(
        action_of("/bogus", &[]),
        Some(args::Action::Direct { .. })
    ));
}

#[test]
fn bare_numbers_and_names_select_the_harness() {
    let harnesses = [harness("alpha"), harness("beta")];
    let beta = Some(args::Action::Use("beta".into()));
    assert_eq!(action_of("2", &harnesses), beta);
    assert_eq!(action_of("beta", &harnesses), beta);
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
        Some(args::Action::Show(Some("opencode".to_string())))
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

#[test]
fn theme_lines_resolve_to_the_theme_surface() {
    for line in ["/theme", "theme"] {
        assert!(matches!(resolve(line, &[]), Resolved::Theme(None)));
    }
    for line in ["/theme moss", "theme moss"] {
        assert!(matches!(resolve(line, &[]), Resolved::Theme(Some(_))));
    }
}
