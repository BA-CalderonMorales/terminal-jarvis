use super::config::ConfigResult;
use super::inspect::Kind;
use super::redact::Redactor;
use super::{Code, DiagnosticInput, Record, Severity};
use std::path::Path;

#[path = "storage_records.rs"]
mod records;
use records::{cache, path_record, severity};

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
    if input.catalog == Path::new(crate::context::EMBEDDED_CATALOG) {
        catalog = Record::new("state.catalog", Code::Ready, Severity::Info, "embedded");
    }
    if catalog.code == Code::Ready && input.harnesses.is_empty() {
        catalog.code = Code::Empty;
        catalog.severity = Severity::Error;
    }
    if catalog.code != Code::Ready {
        catalog.action = Some("restore a valid harness catalog".into());
    }
    let home_code = super::inspect::writable(&input.home, Kind::Directory, input.now, None);
    // A config directory that does not exist yet -- or exists and is still
    // empty -- is the first-run state: TJ fills it on the first write. It only
    // needs attention when it cannot be written.
    let home_severity = match home_code {
        Code::Missing | Code::Ready | Code::Empty => Severity::Info,
        _ => severity(home_code),
    };
    let mut home = Record::new(
        "state.home",
        home_code,
        home_severity,
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
#[cfg(test)]
#[path = "../tests/storage.rs"]
mod tests;
