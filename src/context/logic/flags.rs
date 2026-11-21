use crate::context::constants::env as env_const;
use std::env;
use std::path::PathBuf;

/// The flag registry location: `TERMINAL_JARVIS_FLAGS` when set, otherwise
/// the registry embedded in the binary (flags never ship as loose files).
pub fn flags_root() -> PathBuf {
    match env::var_os(env_const::FLAGS).filter(|path| !path.is_empty()) {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(env_const::EMBEDDED_FLAGS),
    }
}

#[cfg(test)]
#[path = "../tests/flags.rs"]
mod context_flags_tests;
