use super::{card, Group, Row};

fn group(heading: &str, rows: Vec<Row>) -> Group {
    Group::new(heading, rows)
}

#[test]
fn a_short_value_keeps_its_label_beside_it() {
    let lines = card(
        &[group(
            "details",
            vec![Row::new("binary", "codex"), Row::new("setup", "no key")],
        )],
        80,
    );
    assert_eq!(
        lines,
        vec![
            "  details".to_string(),
            "    binary   codex".to_string(),
            "    setup    no key".to_string(),
        ]
    );
}

#[test]
fn a_long_value_takes_its_own_lines_hanging_under_its_label() {
    let lines = card(
        &[group(
            "capabilities",
            vec![Row::new(
                "update",
                "Update the npm-installed Codex package without interactive auth.",
            )],
        )],
        48,
    );
    assert_eq!(
        lines,
        vec![
            "  capabilities".to_string(),
            "    update".to_string(),
            "      Update the npm-installed Codex package".to_string(),
            "      without interactive auth.".to_string(),
        ],
        "a value that does not fit is wrapped, never clipped mid-word"
    );
}

#[test]
fn groups_are_separated_by_a_blank_line_and_nest_under_their_heading() {
    let lines = card(
        &[
            group("details", vec![Row::new("binary", "codex")]),
            group("support", vec![Row::new("verified", "1")]),
        ],
        80,
    );
    assert_eq!(
        lines,
        vec![
            "  details".to_string(),
            "    binary   codex".to_string(),
            String::new(),
            "  support".to_string(),
            "    verified 1".to_string(),
        ]
    );
}

#[test]
fn an_empty_group_renders_nothing_at_all() {
    let lines = card(&[group("support", vec![])], 80);
    assert!(lines.is_empty(), "a group with no rows has nothing to say");
}

#[test]
fn a_narrow_terminal_never_paints_a_row_past_its_width() {
    let lines = card(
        &[group(
            "details",
            vec![Row::new("binary", "a-very-long-binary-name-indeed")],
        )],
        20,
    );
    for line in &lines {
        assert!(
            line.chars().count() <= 20,
            "a row must not overflow the frame: {line:?}"
        );
    }
}
