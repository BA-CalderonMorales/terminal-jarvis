use super::Report;

pub fn data(report: &Report) -> String {
    let records = report
        .records
        .iter()
        .map(record)
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"ready_harnesses\":{},\"diagnostics\":[{records}]}}",
        report.ready_harnesses
    )
}

pub fn full(report: &Report) -> String {
    let error = if report.ok {
        "null".to_string()
    } else {
        "{\"code\":\"readiness-failed\",\"message\":\"diagnostics require remediation\"}"
            .to_string()
    };
    format!(
        "{{\"schema_version\":1,\"command\":\"check\",\"ok\":{},\"exit_code\":{},\"data\":{},\"error\":{error}}}",
        report.ok,
        report.exit_code(),
        data(report)
    )
}

fn record(record: &super::Record) -> String {
    let action = record
        .action
        .as_ref()
        .map(|value| quoted(value))
        .unwrap_or_else(|| "null".into());
    format!(
        "{{\"key\":{},\"code\":\"{}\",\"severity\":\"{}\",\"value\":{},\"action\":{action}}}",
        quoted(&record.key),
        record.code.as_str(),
        record.severity.as_str(),
        quoted(&record.value),
    )
}

fn quoted(value: &str) -> String {
    let mut out = String::from("\"");
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            value if value < ' ' => out.push_str(&format!("\\u{:04x}", value as u32)),
            value => out.push(value),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::logic::{Code, Record, Report, Severity};

    #[test]
    fn data_serializes_records_and_actions() {
        let mut first = Record::new("a", Code::Ready, Severity::Info, "b");
        first.action = Some("c".into());
        let report = Report {
            records: vec![
                first,
                Record::new("d", Code::Missing, Severity::Warning, "e"),
            ],
            ready_harnesses: 1,
            ok: true,
        };
        assert_eq!(
            data(&report),
            "{\"ready_harnesses\":1,\"diagnostics\":[{\"key\":\"a\",\"code\":\"ready\",\"severity\":\"info\",\"value\":\"b\",\"action\":\"c\"},{\"key\":\"d\",\"code\":\"missing\",\"severity\":\"warning\",\"value\":\"e\",\"action\":null}]}"
        );
    }

    #[test]
    fn quoted_escapes_and_keeps_other_chars() {
        assert_eq!(quoted("ab"), "\"ab\"");
        assert_eq!(quoted("a\"b"), "\"a\\\"b\"");
        assert_eq!(quoted("a\\b"), "\"a\\\\b\"");
        assert_eq!(quoted("a\nb"), "\"a\\nb\"");
        assert_eq!(quoted("a\rb"), "\"a\\rb\"");
        assert_eq!(quoted("a\tb"), "\"a\\tb\"");
        assert_eq!(quoted("a\u{1}b"), "\"a\\u0001b\"");
        assert_eq!(quoted("a b"), "\"a b\"");
        assert_eq!(quoted("a\u{1f}b"), "\"a\\u001fb\"");
        assert_eq!(quoted("a\u{7f}b"), "\"a\u{7f}b\"");
    }
}
