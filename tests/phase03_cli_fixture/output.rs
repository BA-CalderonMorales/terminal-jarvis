#[path = "width.rs"]
mod width;

use std::process::Output;

pub fn text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).expect("CLI output is UTF-8")
}

pub fn assert_json_document(output: &Output, code: i32) -> String {
    assert_eq!(output.status.code(), Some(code));
    assert!(output.stderr.is_empty());
    let body = text(&output.stdout);
    assert!(body.starts_with("{\"schema_version\":1,"));
    assert!(body.ends_with("}\n"));
    assert_eq!(body.bytes().filter(|byte| *byte == b'\n').count(), 1);
    assert!(body.contains(&format!("\"exit_code\":{code}")));
    assert!(!body.contains("\u{1b}["));
    body
}

pub fn assert_width(body: &str, limit: usize) {
    for line in body.lines() {
        let width = width::display_width(line);
        assert!(width <= limit, "line width {width} exceeds {limit}: {line}");
    }
}
