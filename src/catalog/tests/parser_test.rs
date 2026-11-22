use super::{list, Fields};

#[test]
fn list_preserves_commas_inside_quoted_values() {
    let mut fields = Fields::new();
    fields.insert(
        "args".to_string(),
        "[\"--scanners\", \"vuln,secret,misconfig\"]".to_string(),
    );
    assert_eq!(
        list(&fields, "args").unwrap(),
        ["--scanners", "vuln,secret,misconfig"]
    );
}

#[test]
fn a_hash_inside_quotes_is_text_and_a_trailing_comment_is_dropped() {
    let fields = super::parse("description = \"Enable the #1 scanner.\" # note\n").unwrap();
    assert_eq!(fields["description"], "\"Enable the #1 scanner.\"");
    let fields = super::parse("name = \"x\" # trailing\n# whole line\n").unwrap();
    assert_eq!(fields["name"], "\"x\"");
    assert_eq!(fields.len(), 1);
}
