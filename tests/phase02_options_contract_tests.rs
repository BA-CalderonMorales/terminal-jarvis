use terminal_jarvis::cli::args::{parse_cli, Action, OutputMode};

#[test]
fn globals_work_anywhere_before_the_child_boundary() {
    let parsed = parse_cli([
        "tj",
        "run",
        "--no-input",
        "fixture",
        "headless",
        "--plain",
        "--confirm=headless:fixture",
        "--",
        "--json",
        "--no-color",
    ])
    .unwrap();
    assert_eq!(parsed.options.output, OutputMode::Plain);
    assert!(parsed.options.no_input);
    assert_eq!(parsed.options.confirm.as_deref(), Some("headless:fixture"));
    assert_eq!(
        parsed.action,
        Action::Run(vec![
            "fixture".into(),
            "headless".into(),
            "--json".into(),
            "--no-color".into(),
        ])
    );
}

#[test]
fn boundary_is_rejected_for_non_child_commands() {
    let error = parse_cli(["tj", "list", "--", "--json"]).unwrap_err();
    assert!(error.contains("valid only with run or direct"));
}

#[test]
fn direct_boundary_preserves_global_looking_child_flags() {
    let parsed = parse_cli(["tj", "fixture", "--", "--plain", "--json"]).unwrap();
    assert_eq!(parsed.options.output, OutputMode::Rich);
    assert_eq!(
        parsed.action,
        Action::Direct {
            harness: "fixture".into(),
            extra: vec!["--plain".into(), "--json".into()],
        }
    );
}

#[test]
fn run_child_flags_require_an_explicit_boundary() {
    let run = parse_cli(["tj", "run", "fixture", "headless", "--bogus"]).unwrap_err();
    assert!(run.contains("--bogus"));
}

#[test]
fn direct_child_flags_require_an_explicit_boundary() {
    let direct = parse_cli(["tj", "fixture", "--bogus"]).unwrap_err();
    assert!(direct.contains("--bogus"));
}

#[test]
fn compatibility_child_help_does_not_require_a_boundary() {
    assert!(parse_cli(["tj", "run", "fixture", "--help"]).is_ok());
    assert!(parse_cli(["tj", "fixture", "--help"]).is_ok());
}

#[test]
fn output_modes_conflict_across_option_positions() {
    let error = parse_cli(["tj", "--plain", "list", "--json"]).unwrap_err();
    assert!(error.contains("mutually exclusive"));
}
