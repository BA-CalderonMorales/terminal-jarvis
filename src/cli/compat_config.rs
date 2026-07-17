use super::super::{style, table};
use crate::context::Session;
use std::path::Path;

pub fn show(catalog_root: &Path, home: &Path, session: Option<Session>) -> String {
    let active = session
        .map(|session| session.active_harness)
        .unwrap_or_else(|| "none".to_string());
    if style::plain() {
        return format!(
            "home: {}\ncatalog: {}\nactive harness: {}\n",
            home.display(),
            catalog_root.display(),
            active
        );
    }
    table::fields(
        "Configuration",
        &[
            ("HOME", home.display().to_string()),
            ("CATALOG", catalog_root.display().to_string()),
            ("ACTIVE HARNESS", active),
        ],
    )
}

pub fn paths(catalog_root: &Path, home: &Path) -> String {
    if style::plain() {
        return format!(
            "home: {}\ncatalog: {}\n",
            home.display(),
            catalog_root.display()
        );
    }
    table::fields(
        "Configuration Paths",
        &[
            ("HOME", home.display().to_string()),
            ("CATALOG", catalog_root.display().to_string()),
        ],
    )
}

pub fn reset(version: &str) -> String {
    if style::plain() {
        return format!(
            "config reset is not automatic in v{version}; remove TERMINAL_JARVIS_HOME after review\n"
        );
    }
    let note = format!(
        "Config reset is not automatic in v{version}; remove TERMINAL_JARVIS_HOME after review"
    );
    format!(
        "{}\n{}",
        style::warning("Configuration was not changed."),
        table::fields("Configuration Reset", &[("NEXT STEP", note)])
    )
}
