use super::error;
use std::path::Path;

pub fn catalog_error(path: &Path, cause: std::io::Error) -> error::Failure {
    let safe_path = crate::diagnostics::redact_process_path(path);
    let kind = cause.kind();
    let cause = crate::diagnostics::redact_process_text(&cause.to_string())
        .replace(&path.to_string_lossy().to_string(), &safe_path);
    let (code, message) = match kind {
        std::io::ErrorKind::NotFound => (
            "catalog_missing",
            format!("harness catalog is missing at {safe_path}"),
        ),
        std::io::ErrorKind::PermissionDenied => (
            "catalog_permission_denied",
            format!("harness catalog is not readable at {safe_path}"),
        ),
        std::io::ErrorKind::InvalidData => (
            "catalog_invalid",
            format!("harness catalog is invalid: {cause}"),
        ),
        _ => (
            "catalog_unreadable",
            format!("failed to load harness catalog at {safe_path}: {cause}"),
        ),
    };
    error::Failure::state(
        code,
        message,
        "reinstall terminal-jarvis or set TERMINAL_JARVIS_CATALOG to a valid catalog",
    )
}
