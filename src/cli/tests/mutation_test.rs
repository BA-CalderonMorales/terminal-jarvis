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
