//! StorageRecords: the two records `storage` builds besides its main tuple --
//! the wrapper cache and a generic path check. The severity table lives here
//! because both need it.

use super::super::inspect::Kind;
use super::super::redact::Redactor;
use super::super::{Code, DiagnosticInput, Record, Severity, ValueState};
use std::path::Path;

pub(super) fn cache(input: &DiagnosticInput, redact: &Redactor<'_>) -> Record {
    let state = input.environment.state("TERMINAL_JARVIS_CACHE");
    if state != ValueState::Present {
        if crate::context::distribution::channel() != Some("npm") {
            return Record::new("state.cache", Code::Ready, Severity::Info, "not-applicable");
        }
        return Record::new(
            "state.cache",
            state.code(),
            Severity::Warning,
            state.as_str(),
        );
    }
    let Some(value) = input.environment.text("TERMINAL_JARVIS_CACHE") else {
        return Record::new(
            "state.cache",
            Code::Malformed,
            Severity::Warning,
            "malformed",
        );
    };
    let path = Path::new(value);
    let code = super::super::inspect::writable(path, Kind::Any, input.now, Some(input.stale_after));
    let record = Record::new("state.cache", code, severity(code), redact.full(path));
    if matches!(code, Code::Stale | Code::Malformed | Code::PermissionDenied) {
        record.action("refresh or remove the local wrapper cache")
    } else {
        record
    }
}

pub(super) fn path_record(
    key: &str,
    path: &Path,
    kind: Kind,
    input: &DiagnosticInput,
    redact: &Redactor<'_>,
    stale: bool,
) -> Record {
    let limit = stale.then_some(input.stale_after);
    let code = super::super::inspect::path(path, kind, input.now, limit);
    Record::new(key, code, severity(code), redact.full(path))
}

pub(super) fn severity(code: Code) -> Severity {
    match code {
        Code::Ready => Severity::Info,
        Code::Missing | Code::Empty | Code::Stale | Code::Unknown => Severity::Warning,
        _ => Severity::Error,
    }
}
