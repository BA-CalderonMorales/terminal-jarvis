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
