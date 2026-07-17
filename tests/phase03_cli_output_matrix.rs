#[path = "phase03_cli_fixture/mod.rs"]
pub mod fixture;

use fixture::{assert_json_document, assert_width, text, Fixture, State};

fn rich(fixture: &Fixture, columns: Option<&str>, args: &[&str]) -> String {
    let mut command = fixture.command();
    command.args(args);
    if let Some(value) = columns {
        command.env("COLUMNS", value);
    } else {
        command.env_remove("COLUMNS");
    }
    let output = command.output().expect("CLI runs");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    text(&output.stdout)
}

#[test]
fn non_tty_output_modes_and_color_disablers_are_deterministic() {
    let fixture = Fixture::new(State::Unknown);
    let plain = fixture.run(&["--plain", "list"]);
    assert_eq!(plain.status.code(), Some(0));
    let plain = text(&plain.stdout);
    assert!(plain.contains("fixture support="));
    assert!(!plain.contains('+') && !plain.contains("\u{1b}["));

    let baseline = rich(&fixture, None, &["list"]);
    assert!(baseline.contains("Available Harnesses\n+"));
    assert!(!baseline.contains("\u{1b}["));
    for (name, value) in [("NO_COLOR", "1"), ("TERM", "dumb"), ("CI", "1")] {
        let mut command = fixture.command();
        let output = command.env(name, value).arg("list").output().unwrap();
        assert_eq!(text(&output.stdout), baseline, "environment: {name}");
        assert!(output.stderr.is_empty());
    }
    assert_eq!(rich(&fixture, None, &["--no-color", "list"]), baseline);
    let json = assert_json_document(&fixture.run(&["--json", "list"]), 0);
    assert!(json.contains("\"command\":\"list\""));
}

#[test]
fn rich_layout_obeys_all_frozen_width_cells() {
    let fixture = Fixture::new(State::Unknown);
    for (columns, expected) in [
        (Some("40"), 40),
        (Some("80"), 80),
        (Some("100"), 100),
        (Some("120"), 120),
        (Some("invalid"), 100),
        (None, 100),
    ] {
        let body = rich(&fixture, columns, &["list"]);
        assert_width(&body, expected);
        let border = body
            .lines()
            .filter(|line| line.starts_with('+'))
            .map(str::len)
            .max()
            .unwrap();
        assert_eq!(border, expected, "COLUMNS={columns:?}");
    }
}

#[test]
fn long_paths_unicode_combining_marks_emoji_and_unbroken_fields_wrap() {
    let fixture = Fixture::new(State::Unknown);
    for (args, columns) in [
        (&["show", "fixture"][..], "80"),
        (&["plan", "fixture", "headless"][..], "40"),
        (&["config", "path"][..], "40"),
    ] {
        let body = rich(&fixture, Some(columns), args);
        assert_width(&body, columns.parse().unwrap());
        if args[0] != "config" {
            assert!(body.contains("漢字") && body.contains("é") && body.contains('🚀'));
        }
        if args[0] == "config" {
            let compact = body.replace(['|', ' ', '\n'], "");
            assert!(compact.contains(fixture.root().to_string_lossy().as_ref()));
        }
    }
}
