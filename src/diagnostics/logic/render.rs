use super::Report;

pub fn plain(report: &Report) -> String {
    let mut out = String::new();
    for record in &report.records {
        out.push_str(&field(&record.key));
        out.push('\t');
        out.push_str(record.code.as_str());
        out.push('\t');
        out.push_str(record.severity.as_str());
        out.push('\t');
        out.push_str(&field(&record.value));
        out.push('\t');
        out.push_str(&field(record.action.as_deref().unwrap_or("")));
        out.push('\n');
    }
    out
}

fn field(value: &str) -> String {
    let mut out = String::new();
    for character in value.chars() {
        match character {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            value if value.is_control() => out.push('?'),
            value => out.push(value),
        }
    }
    out
}
