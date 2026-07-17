use terminal_jarvis::cli::args::{parse_cli, Action, Options, OutputMode, Parsed};

fn inserted(base: &[&str], flag: &str, index: usize) -> Parsed {
    let mut words = vec!["terminal-jarvis".to_string()];
    words.extend(base.iter().map(|word| (*word).to_string()));
    words.insert(index + 1, flag.to_string());
    parse_cli(words).unwrap_or_else(|error| panic!("{base:?} + {flag}@{index}: {error}"))
}

#[test]
fn presentation_options_work_at_every_token_boundary() {
    let base = ["show", "fixture"];
    for (flag, mode, no_color) in [
        ("--plain", OutputMode::Plain, false),
        ("--json", OutputMode::Json, false),
        ("--no-color", OutputMode::Rich, true),
    ] {
        for index in 0..=base.len() {
            let parsed = inserted(&base, flag, index);
            assert_eq!(parsed.action, Action::Show("fixture".to_string()));
            assert_eq!(parsed.options.output, mode);
            assert_eq!(parsed.options.no_color, no_color);
        }
    }
}

#[test]
fn scoped_options_work_at_every_valid_token_boundary() {
    for index in 0..=1 {
        assert!(inserted(&["check"], "--verbose", index).options.verbose);
        assert!(inserted(&["version"], "--verbose", index).options.verbose);
    }
    let base = ["run", "fixture", "yolo"];
    for index in 0..=base.len() {
        assert!(inserted(&base, "--dry-run", index).options.dry_run);
        assert!(inserted(&base, "--no-input", index).options.no_input);
        assert!(
            inserted(&base, "--allow-dangerous", index)
                .options
                .allow_dangerous
        );
        assert_eq!(
            inserted(&base, "--confirm=yolo:fixture", index)
                .options
                .confirm
                .as_deref(),
            Some("yolo:fixture")
        );
    }
}

#[test]
fn combined_globals_are_order_independent_before_the_boundary() {
    let first = parse_cli([
        "tj",
        "--plain",
        "run",
        "--dry-run",
        "fixture",
        "--no-input",
        "yolo",
        "--confirm=yolo:fixture",
        "--allow-dangerous",
        "--no-color",
    ])
    .unwrap();
    assert_eq!(
        first.action,
        Action::Run(vec!["fixture".into(), "yolo".into()])
    );
    assert_eq!(
        first.options,
        Options {
            output: OutputMode::Plain,
            no_color: true,
            verbose: false,
            dry_run: true,
            no_input: true,
            confirm: Some("yolo:fixture".into()),
            allow_dangerous: true,
        }
    );
}
