use super::redact::Redactor;
use super::{Code, DiagnosticInput, Record, Severity};
use std::fs;

pub fn collect(input: &DiagnosticInput, redact: &Redactor<'_>) -> (Record, Record, bool) {
    let Some(current) = input.executable.as_deref() else {
        return (
            Record::new("tj.executable", Code::Missing, Severity::Error, "unknown"),
            Record::new("tj.path", Code::Unknown, Severity::Warning, "unknown"),
            false,
        );
    };
    let direct = super::resolve::direct(current);
    let executable = Record::new(
        "tj.executable",
        direct.code,
        if direct.code == Code::Ready {
            Severity::Info
        } else {
            Severity::Error
        },
        redact.full(current),
    );
    let Some(name) = current.file_name().and_then(|name| name.to_str()) else {
        return (
            executable,
            Record::new("tj.path", Code::Malformed, Severity::Error, "unknown"),
            false,
        );
    };
    let resolved = super::resolve::binary(name, input);
    let shadowed = resolved.matches > 1
        || resolved
            .path
            .as_ref()
            .is_some_and(|path| !same(path, current));
    let path = if shadowed {
        shadowed_record(redact, &resolved, current)
    } else {
        Record::new(
            "tj.path",
            Code::Ready,
            Severity::Info,
            if resolved.matches == 0 {
                "direct"
            } else {
                "resolved"
            },
        )
    };
    (executable, path, direct.code == Code::Ready && !shadowed)
}

fn shadowed_record(
    redact: &Redactor<'_>,
    resolved: &super::resolve::Resolution,
    current: &std::path::Path,
) -> Record {
    let value = if resolved.matches > 1 {
        format!("shadowed ({} matches)", resolved.matches)
    } else {
        "shadowed".to_string()
    };
    let others = resolved
        .paths
        .iter()
        .filter(|path| !same(path, current))
        .take(3)
        .map(|path| redact.full(path))
        .collect::<Vec<_>>()
        .join(", ");
    let action = if others.is_empty() {
        "remove stale or shadowing PATH entries".to_string()
    } else {
        format!("remove stale or shadowing PATH entries: {others}")
    };
    Record::new("tj.path", Code::Conflicting, Severity::Error, value).action(action)
}

fn same(left: &std::path::Path, right: &std::path::Path) -> bool {
    fs::canonicalize(left)
        .ok()
        .zip(fs::canonicalize(right).ok())
        .is_some_and(|(a, b)| a == b)
}

#[cfg(test)]
#[path = "../tests/program.rs"]
mod tests;
