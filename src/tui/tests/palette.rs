use super::*;

fn action(line: &str) -> args::Action {
    parse(line).expect("parses").0
}

#[test]
fn slash_lines_parse_with_the_cli_grammar() {
    assert_eq!(action("list"), args::Action::List);
    assert_eq!(action("status"), args::Action::Check);
    assert_eq!(action("use codex"), args::Action::Use("codex".into()));
    assert_eq!(
        action("opencode headless fix this"),
        args::Action::Direct {
            harness: "opencode".into(),
            extra: vec!["headless".into(), "fix".into(), "this".into()],
        }
    );
}

#[test]
fn broken_slash_lines_report_usage() {
    let error = parse("use").unwrap_err();
    assert!(
        error.contains("/help"),
        "palette errors must point at /help: {error}"
    );
}

#[test]
fn invalid_flags_are_rejected_inside_slash_lines() {
    let error = parse("--bogus").unwrap_err();
    assert!(error.contains("unknown flag"), "{error}");
}

#[test]
fn lifecycle_verbs_map_to_their_actions() {
    assert_eq!(
        action("install opencode"),
        args::Action::Install(Some("opencode".to_string()))
    );
    assert_eq!(
        action("update opencode"),
        args::Action::Update(Some("opencode".into()))
    );
    assert_eq!(action("version"), args::Action::Version { verbose: false });
}

#[test]
fn typed_flags_travel_with_the_action() {
    // the headless escape hatch must survive the tui's parse: the same
    // --no-input / --confirm grammar the cli speaks, typed in-frame
    let (action, options) =
        parse("install copilot --no-input --confirm=download:copilot").expect("parses");
    assert_eq!(action, args::Action::Install(Some("copilot".into())));
    assert!(options.no_input);
    assert_eq!(options.confirm.as_deref(), Some("download:copilot"));
}
