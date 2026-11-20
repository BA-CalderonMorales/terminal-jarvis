use super::inspect::Kind;
use super::redact::Redactor;
use super::{Code, DiagnosticInput, Record, Severity};

pub fn collect(input: &DiagnosticInput, redact: &Redactor<'_>) -> (Vec<Record>, bool) {
    // The embedded gate catalog is a sentinel path, not a directory on disk.
    let gate_code = if input.runtime.gate == std::path::Path::new(crate::context::EMBEDDED_GATES) {
        Code::Ready
    } else {
        super::inspect::path(&input.runtime.gate, Kind::Directory, input.now, None)
    };
    let route_ok = matches!(
        input.runtime.update_route.as_str(),
        "cargo" | "npm" | "homebrew" | "direct"
    );
    let checksum_ok = matches!(
        input.runtime.checksum.as_str(),
        "verified" | "cache-integrity-verified" | "not-applicable" | "unknown"
    );
    let width_ok = (40..=120).contains(&input.runtime.width);
    let mut records = vec![
        fact(
            "presentation.stdout-tty",
            input.runtime.stdout_tty.to_string(),
        ),
        fact(
            "presentation.stderr-tty",
            input.runtime.stderr_tty.to_string(),
        ),
        fact(
            "presentation.color",
            if input.runtime.color {
                "enabled"
            } else {
                "disabled"
            },
        ),
        status(
            "presentation.width",
            width_ok,
            input.runtime.width.to_string(),
            "set COLUMNS to a value from 40 through 120",
        ),
        status_code(
            "state.gates",
            gate_code,
            redact.full(&input.runtime.gate),
            "restore the packaged gate catalog",
        ),
        status_code(
            "update.route",
            if route_ok { Code::Ready } else { Code::Unknown },
            &input.runtime.update_route,
            "identify the install channel before updating",
        ),
        status(
            "distribution.checksum",
            checksum_ok,
            &input.runtime.checksum,
            "repair or refresh the distribution cache",
        ),
    ];
    if input.runtime.checksum == "unknown" {
        let checksum = records.last_mut().unwrap();
        checksum.code = Code::Unknown;
        checksum.severity = Severity::Warning;
    }
    // An unknown channel only matters to `update`, which refuses with guidance.
    if let Some(route) = records
        .iter_mut()
        .find(|r| r.key == "update.route" && !route_ok)
    {
        route.severity = Severity::Warning;
    }
    (records, gate_code == Code::Ready && checksum_ok && width_ok)
}

fn fact(key: &str, value: impl Into<String>) -> Record {
    Record::new(key, Code::Ready, Severity::Info, value)
}

fn status(key: &str, ok: bool, value: impl Into<String>, action: &str) -> Record {
    if ok {
        fact(key, value)
    } else {
        Record::new(key, Code::Malformed, Severity::Error, value).action(action)
    }
}

fn status_code(key: &str, code: Code, value: impl Into<String>, action: &str) -> Record {
    if code == Code::Ready {
        fact(key, value)
    } else {
        Record::new(key, code, Severity::Error, value).action(action)
    }
}

#[cfg(test)]
#[path = "../tests/runtime_records.rs"]
mod tests;
