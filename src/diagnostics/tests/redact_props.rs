use super::*;
use std::path::PathBuf;

fn clean_is_idempotent(value: String) -> bool {
    clean(&clean(&value)) == clean(&value)
}

fn clean_replaces_control_characters(value: String) -> bool {
    if value.chars().any(|c| c.is_control()) {
        !value.is_empty() && clean(&value).chars().all(|c| !c.is_control())
    } else {
        true
    }
}

fn segment_roundtrips_identifiers(value: String) -> bool {
    let valid = !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'));
    valid == (segment(&value) == value)
}

fn prefixed_never_leaks_home(path: String) -> bool {
    let home = PathBuf::from("/home/user");
    let full = PathBuf::from(format!("/home/user/{path}"));
    let redacted = Redactor::new(Some(&home), None).full(&full);
    !redacted.contains("/home/user/") && (redacted == "~" || redacted.starts_with("~/"))
}

#[test]
fn redact_properties() {
    quickcheck::quickcheck(clean_is_idempotent as fn(String) -> bool);
    quickcheck::quickcheck(clean_replaces_control_characters as fn(String) -> bool);
    quickcheck::quickcheck(segment_roundtrips_identifiers as fn(String) -> bool);
    quickcheck::quickcheck(prefixed_never_leaks_home as fn(String) -> bool);
}
