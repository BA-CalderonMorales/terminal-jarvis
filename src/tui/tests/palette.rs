use super::*;

#[test]
fn slash_lines_parse_with_the_cli_grammar() {
    assert_eq!(parse("list"), Ok(args::Action::List));
    assert_eq!(parse("status"), Ok(args::Action::Check));
    assert_eq!(parse("use codex"), Ok(args::Action::Use("codex".into())));
    assert_eq!(
        parse("opencode headless fix this"),
        Ok(args::Action::Direct {
            harness: "opencode".into(),
            extra: vec!["headless".into(), "fix".into(), "this".into()],
        })
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
        parse("install opencode"),
        Ok(args::Action::Install(Some("opencode".to_string())))
    );
    assert_eq!(
        parse("update opencode"),
        Ok(args::Action::Update(Some("opencode".into())))
    );
    assert_eq!(
        parse("version"),
        Ok(args::Action::Version { verbose: false })
    );
}
