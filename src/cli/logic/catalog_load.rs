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

#[cfg(test)]
mod tests {
    use super::*;

    fn failure_of(kind: std::io::ErrorKind) -> error::Failure {
        catalog_error(
            Path::new("/opt/catalog.toml"),
            std::io::Error::new(kind, "boom"),
        )
    }

    #[test]
    fn permission_denied_maps_to_permission_error() {
        assert_eq!(
            failure_of(std::io::ErrorKind::PermissionDenied),
            error::Failure::state(
                "catalog_permission_denied",
                "harness catalog is not readable at catalog.toml",
                "reinstall terminal-jarvis or set TERMINAL_JARVIS_CATALOG to a valid catalog",
            )
        );
    }

    #[test]
    fn invalid_data_maps_to_invalid_catalog() {
        assert_eq!(
            failure_of(std::io::ErrorKind::InvalidData),
            error::Failure::state(
                "catalog_invalid",
                "harness catalog is invalid: boom",
                "reinstall terminal-jarvis or set TERMINAL_JARVIS_CATALOG to a valid catalog",
            )
        );
    }

    #[test]
    fn not_found_and_other_kinds_are_distinct() {
        assert_eq!(
            failure_of(std::io::ErrorKind::NotFound),
            error::Failure::state(
                "catalog_missing",
                "harness catalog is missing at catalog.toml",
                "reinstall terminal-jarvis or set TERMINAL_JARVIS_CATALOG to a valid catalog",
            )
        );
        assert_eq!(
            failure_of(std::io::ErrorKind::WouldBlock),
            error::Failure::state(
                "catalog_unreadable",
                "failed to load harness catalog at catalog.toml: boom",
                "reinstall terminal-jarvis or set TERMINAL_JARVIS_CATALOG to a valid catalog",
            )
        );
    }
}
