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
    use crate::context::ParseError;
    crate::context::parse_session(data).map_err(|error| match error {
        ParseError::Malformed => Code::Malformed,
        ParseError::Conflicting => Code::Conflicting,
    })
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

#[cfg(test)]
#[path = "../tests/config.rs"]
mod tests;
