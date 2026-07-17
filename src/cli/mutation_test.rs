use super::super::experimental;
use super::{parse_cli, OutputMode};

#[test]
fn presentation_flags_work_before_or_after_the_command() {
    for args in [
        ["tj", "--plain", "--no-color", "list"],
        ["tj", "list", "--plain", "--no-color"],
    ] {
        let parsed = parse_cli(args).unwrap();
        assert_eq!(parsed.options.output, OutputMode::Plain);
        assert!(parsed.options.no_color);
    }
    assert!(parse_cli(["tj", "--plain", "--json", "list"]).is_err());
}

#[test]
fn experimental_rejects_unknown_actions() {
    let words = ["unknown".to_string()];
    let error = experimental::run(&words, &[], std::path::Path::new("/missing")).unwrap_err();
    assert_eq!(error, "usage: terminal-jarvis experimental dashboard");
}
