use super::{parse, record, Code, Severity};

#[test]
fn parse_reads_quoted_active_harness() {
    let data = "\n# comment\n\nactive_harness = \"xh\"\n";
    assert_eq!(parse(data), Ok(Some("xh".to_string())));
    assert_eq!(
        parse("active_harness = \"xh\"\n"),
        Ok(Some("xh".to_string()))
    );
}

#[test]
fn parse_treats_empty_input_as_no_session() {
    assert_eq!(parse(""), Ok(None));
    assert_eq!(parse("# comment\n\n"), Ok(None));
}

#[test]
fn parse_rejects_unquoted_or_foreign_keys() {
    assert_eq!(parse("active_harness = xh\n"), Err(Code::Malformed));
    assert_eq!(parse("other = \"xh\"\n"), Err(Code::Malformed));
    assert_eq!(parse("active_harness = \"\"\n"), Err(Code::Malformed));
}

#[test]
fn parse_rejects_conflicting_values() {
    assert_eq!(
        parse("active_harness = \"a\"\nactive_harness = \"b\"\n"),
        Err(Code::Conflicting)
    );
}

#[test]
fn record_maps_severity_by_code() {
    assert_eq!(record(Code::Ready, String::new()).severity, Severity::Info);
    assert_eq!(
        record(Code::Missing, String::new()).severity,
        Severity::Warning
    );
    assert_eq!(
        record(Code::Empty, String::new()).severity,
        Severity::Warning
    );
    assert_eq!(
        record(Code::Malformed, String::new()).severity,
        Severity::Error
    );
    assert_eq!(
        record(Code::Conflicting, String::new()).severity,
        Severity::Error
    );
    assert_eq!(
        record(Code::Unknown, String::new()).severity,
        Severity::Error
    );
}

#[test]
fn record_actions_only_carry_error_severity() {
    assert_eq!(record(Code::Ready, String::new()).action, None);
    assert_eq!(record(Code::Missing, String::new()).action, None);
    assert_eq!(
        record(Code::Conflicting, String::new()).action.as_deref(),
        Some("repair or remove the local session config")
    );
}
