use super::redact::Redactor;
use super::{Code, DiagnosticInput, Record, Severity};
use std::fs;

pub struct ConfigResult {
    pub record: Record,
    pub active: Option<String>,
    pub valid: bool,
}

pub fn inspect(input: &DiagnosticInput, redact: &Redactor<'_>) -> ConfigResult {
    let path = &input.config;
    let value = redact.full(path);
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(error) => {
            let code = super::inspect::io_code(&error);
            let valid = matches!(code, Code::Missing);
            return ConfigResult {
                record: record(code, value),
                active: None,
                valid,
            };
        }
    };
    if data.trim().is_empty() {
        return ConfigResult {
            record: record(Code::Empty, value),
            active: None,
            valid: true,
        };
    }
    let parsed = parse(&data);
    let (code, active) = match parsed {
        Ok(active)
            if input
                .active_harness
                .as_ref()
                .is_some_and(|value| value != &active) =>
        {
            (Code::Conflicting, Some(active))
        }
        Ok(active) => (Code::Ready, Some(active)),
        Err(code) => (code, None),
    };
    ConfigResult {
        record: record(code, value),
        active,
        valid: code == Code::Ready,
    }
}

fn parse(data: &str) -> Result<String, Code> {
    let mut values = Vec::new();
    for line in data
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let (key, value) = line.split_once('=').ok_or(Code::Malformed)?;
        if key.trim() != "active_harness" {
            return Err(Code::Malformed);
        }
        let value = value
            .trim()
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .filter(|v| !v.trim().is_empty())
            .ok_or(Code::Malformed)?;
        values.push(value.to_string());
    }
    let Some(first) = values.first().cloned() else {
        return Err(Code::Malformed);
    };
    if values.iter().any(|value| value != &first) {
        Err(Code::Conflicting)
    } else {
        Ok(first)
    }
}

fn record(code: Code, value: String) -> Record {
    let severity = match code {
        Code::Ready => Severity::Info,
        Code::Missing | Code::Empty => Severity::Warning,
        _ => Severity::Error,
    };
    let record = Record::new("state.config", code, severity, value);
    if severity == Severity::Error {
        record.action("repair or remove the local session config")
    } else {
        record
    }
}
