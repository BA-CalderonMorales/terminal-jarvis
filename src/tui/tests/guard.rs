use super::*;

#[test]
fn guard_requires_both_streams_to_be_terminals() {
    assert!(guard(false, true, true).is_ok());
    assert!(guard(false, false, true).is_err());
    assert!(guard(false, true, false).is_err());
    assert!(guard(false, false, false).is_err());
}

#[test]
fn guard_rejects_plain_mode_on_a_terminal() {
    let error = guard(true, true, true).unwrap_err();
    assert!(error.contains("--plain"), "{error}");
}

#[test]
fn guard_prefers_the_terminal_error_over_plain() {
    let error = guard(true, false, false).unwrap_err();
    assert!(error.contains("interactive terminal"), "{error}");
}
