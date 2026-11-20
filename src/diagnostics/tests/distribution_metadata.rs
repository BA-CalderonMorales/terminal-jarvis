use super::*;
use crate::diagnostics::{Environment, RuntimeInput};
use std::path::Path;

fn with(environment: Environment) -> (Record, bool) {
    let runtime = RuntimeInput::local(false, false, false, 80, "source");
    let mut input = DiagnosticInput::local(Path::new("/c"), Path::new("/h"), None, &[], runtime);
    input.environment = environment;
    collect(&input)
}

#[test]
fn absent_metadata_is_a_source_build() {
    let (record, ok) = with(Environment::default());
    assert!(ok);
    assert_eq!(
        (record.severity, record.value.as_str()),
        (Severity::Info, "source build")
    );
}

/// Set but blank means someone meant to declare a channel; say so instead of
/// calling it a healthy source build.
#[test]
fn blank_metadata_warns_instead_of_reading_as_a_source_build() {
    let mut environment = Environment::default();
    environment.insert("TERMINAL_JARVIS_DISTRIBUTION", "  ");
    let (record, _) = with(environment);
    assert_eq!(
        (record.code, record.severity),
        (Code::Empty, Severity::Warning)
    );
}
