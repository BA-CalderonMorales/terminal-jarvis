use terminal_jarvis::cli::args::{parse_cli, Action, Options};

#[test]
fn boundary_tokens_are_forwarded_and_never_reparsed_as_globals() {
    let parsed = parse_cli([
        "tj",
        "run",
        "fixture",
        "headless",
        "--",
        "--json",
        "--plain",
        "--no-color",
        "--verbose",
    ])
    .unwrap();
    assert_eq!(parsed.options, Options::default());
    assert_eq!(
        parsed.action,
        Action::Run(vec![
            "fixture".into(),
            "headless".into(),
            "--json".into(),
            "--plain".into(),
            "--no-color".into(),
            "--verbose".into(),
        ])
    );
}

#[test]
fn direct_boundary_preserves_global_looking_child_tokens() {
    let parsed = parse_cli(["tj", "fixture", "--", "--json", "--dry-run"]).unwrap();
    assert_eq!(parsed.options, Options::default());
    assert_eq!(
        parsed.action,
        Action::Direct {
            harness: "fixture".into(),
            extra: vec!["--json".into(), "--dry-run".into()],
        }
    );
}
