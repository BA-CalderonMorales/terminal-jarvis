use super::{style, table};
use crate::diagnostics::Report;

pub fn diagnostics(report: &Report) -> String {
    if style::plain() {
        return report.plain();
    }
    let rows = report
        .records
        .iter()
        .map(|record| {
            vec![
                record.key.clone(),
                record.code.as_str().to_string(),
                record.severity.as_str().to_string(),
                record.value.clone(),
                record.action.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    table::render(
        "Terminal Jarvis Diagnostics",
        &["CHECK", "STATE", "SEVERITY", "VALUE", "NEXT ACTION"],
        &rows,
    )
}
