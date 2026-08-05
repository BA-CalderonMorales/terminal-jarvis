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
