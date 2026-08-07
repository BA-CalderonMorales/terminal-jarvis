use super::*;

#[test]
fn field_escapes_controls_and_backslashes() {
    assert_eq!(field("a\u{7}b\\c"), "a?b\\\\c");
    assert_eq!(field("tab\tend"), "tab\\tend");
}

#[test]
fn field_keeps_plain_and_wide_text() {
    assert_eq!(field("plain 界"), "plain 界");
}
