use super::config::ConfigResult;
use super::inspect::Kind;
use super::redact::Redactor;
use super::{Code, DiagnosticInput, Record, Severity, ValueState};
use std::path::Path;

pub fn collect(
    input: &DiagnosticInput,
    redact: &Redactor<'_>,
) -> (Vec<Record>, ConfigResult, bool) {
    let mut catalog = path_record(
        "state.catalog",
        &input.catalog,
        Kind::Directory,
        input,
        redact,
        false,
    );
    if catalog.code == Code::Ready && input.harnesses.is_empty() {
        catalog.code = Code::Empty;
        catalog.severity = Severity::Error;
    }
    if catalog.code != Code::Ready {
        catalog.action = Some("restore a valid harness catalog".into());
    }
    let home_code = super::inspect::writable(&input.home, Kind::Directory, input.now, None);
    let mut home = Record::new(
        "state.home",
        home_code,
        severity(home_code),
        redact.full(&input.home),
    );
    if home_code == Code::PermissionDenied {
        home.action = Some("make the Terminal Jarvis home writable".into());
    }
    let cache = cache(input, redact);
    let config = super::config::inspect(input, redact);
    let home_valid = !matches!(home.code, Code::Malformed | Code::PermissionDenied);
    let valid = catalog.code == Code::Ready && config.valid && home_valid;
    (
        vec![catalog, home, cache, config.record.clone()],
        config,
        valid,
    )
}

fn cache(input: &DiagnosticInput, redact: &Redactor<'_>) -> Record {
    let state = input.environment.state("TERMINAL_JARVIS_CACHE");
    if state != ValueState::Present {
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
    let code = super::inspect::writable(path, Kind::Any, input.now, Some(input.stale_after));
    let record = Record::new("state.cache", code, severity(code), redact.full(path));
    if matches!(code, Code::Stale | Code::Malformed | Code::PermissionDenied) {
        record.action("refresh or remove the local wrapper cache")
    } else {
        record
    }
}

fn path_record(
    key: &str,
    path: &Path,
    kind: Kind,
    input: &DiagnosticInput,
    redact: &Redactor<'_>,
    stale: bool,
) -> Record {
    let limit = stale.then_some(input.stale_after);
    let code = super::inspect::path(path, kind, input.now, limit);
    Record::new(key, code, severity(code), redact.full(path))
}

fn severity(code: Code) -> Severity {
    match code {
        Code::Ready => Severity::Info,
        Code::Missing | Code::Empty | Code::Stale | Code::Unknown => Severity::Warning,
        _ => Severity::Error,
    }
}
