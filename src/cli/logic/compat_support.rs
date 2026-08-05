use crate::contracts::{EnvMode, Harness};
use crate::security;

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
