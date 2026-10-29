use super::*;
use crate::tui::input::Key;

fn fleet() -> Vec<String> {
    vec!["claude-code".to_string(), "codex".to_string()]
}

#[test]
fn a_unique_command_prefix_expands_to_the_whole_word() {
    assert_eq!(line("instal", &[]), Some("install".to_string()));
    assert_eq!(line("ga", &[]), Some("gate".to_string()));
}

#[test]
fn a_harness_argument_completes_from_the_fleet() {
    assert_eq!(
        line("install cla", &fleet()),
        Some("install claude-code".to_string())
    );
    assert_eq!(line("run co", &fleet()), Some("run codex".to_string()));
}

#[test]
fn ambiguity_stops_at_the_shared_prefix() {
    let names = vec!["claude-code".to_string(), "claude-api".to_string()];
    assert_eq!(
        line("use claude-", &names),
        None,
        "the token already reaches the shared prefix"
    );
    assert_eq!(
        line("use cla", &names),
        Some("use claude-".to_string()),
        "candidates agree past the typed token"
    );
}

#[test]
fn a_word_that_is_already_whole_or_unknown_is_left_alone() {
    assert_eq!(line("install", &fleet()), None);
    assert_eq!(line("install nope", &fleet()), None);
    assert_eq!(line("", &fleet()), None);
    assert_eq!(line("install ", &fleet()), None);
}

#[test]
fn earlier_tokens_are_never_rewritten() {
    assert_eq!(
        line("converse 3 co code", &fleet()),
        Some("converse 3 co codex".to_string()),
        "only the token under the cursor grows"
    );
}

#[test]
fn apply_rewrites_the_editor_and_reports_whether_it_moved() {
    let mut editor = Editor::default();
    for character in "instal".chars() {
        editor.feed(Key::Char(character));
    }
    assert!(apply(&mut editor, &fleet()));
    assert_eq!(editor.line(), "install");
    assert!(!apply(&mut editor, &fleet()), "a second Tab adds nothing");
}
