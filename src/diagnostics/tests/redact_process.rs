use super::*;

#[test]
fn single_character_prefixes_are_not_redacted() {
    assert_eq!(replace_prefix("a/b", "a", "<x>"), "a/b");
}

#[test]
fn longer_prefixes_are_redacted_once() {
    assert_eq!(replace_prefix("$HOME/.local", "$HOME", "<h>"), "<h>/.local");
}
