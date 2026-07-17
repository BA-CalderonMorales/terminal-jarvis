use super::{compat, dispatch_support, error};
use crate::contracts::Harness;
use std::path::Path;

pub fn auth(words: &[String], harnesses: &[Harness]) -> error::Result<(i32, String)> {
    if words.first().is_some_and(|word| word == "set") {
        return Err(error::Failure::unavailable(
            "guidance_only",
            "auth set does not persist credentials",
            "export the harness credential variables in your shell",
        ));
    }
    compat::auth(words, harnesses)
        .map(|body| (0, body))
        .map_err(dispatch_support::unavailable_error)
}

pub fn config(words: &[String], catalog: &Path, home: &Path) -> error::Result<(i32, String)> {
    if words.first().is_some_and(|word| word == "reset") {
        return Err(error::Failure::unavailable(
            "guidance_only",
            "config reset is guidance-only; no configuration was changed",
            "review and remove TERMINAL_JARVIS_HOME manually",
        ));
    }
    compat::config(words, catalog, home, dispatch_support::session(home)?)
        .map(|body| (0, body))
        .map_err(dispatch_support::state_error)
}

pub fn cache(words: &[String]) -> error::Result<(i32, String)> {
    if matches!(words.first().map(String::as_str), Some("clear" | "refresh")) {
        return Err(error::Failure::unavailable(
            "guidance_only",
            "cache maintenance is guidance-only; no cache was changed",
            "review `terminal-jarvis cache status` and remove the cache manually",
        ));
    }
    compat::cache(words)
        .map(|body| (0, body))
        .map_err(dispatch_support::state_error)
}
