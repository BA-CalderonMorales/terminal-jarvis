use crate::context::Session;
use crate::contracts::{EnvMode, Harness};
use crate::security;
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn auth_status(harness: &Harness) -> String {
    let missing = security::missing_env(harness);
    if missing.is_empty() {
        return "ready".to_string();
    }
    match harness.env_mode {
        EnvMode::Any => format!("missing one of: {}", missing.join(", ")),
        EnvMode::All => format!("missing: {}", missing.join(", ")),
        EnvMode::None => "ready".to_string(),
    }
}

pub fn auth_notice() -> String {
    format!(
        "credential manager is not active in v{VERSION}\n\
         use `terminal-jarvis auth help <harness>` and export the listed env vars\n"
    )
}

pub fn config_show(catalog_root: &Path, home: &Path, session: Option<Session>) -> String {
    let active = session
        .map(|s| s.active_harness)
        .unwrap_or_else(|| "none".to_string());
    format!(
        "home: {}\ncatalog: {}\nactive harness: {}\n",
        home.display(),
        catalog_root.display(),
        active
    )
}
