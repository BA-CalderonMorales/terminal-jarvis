//! The code-facing API: `flags::enabled("name")` and the full listing.

use super::{registry, resolve::resolve, settings};
use crate::flags::structs::flag::Resolved;
use std::path::Path;

fn env_lookup(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

/// The declared flags with their effective values, sorted by name.
pub fn resolve_all(home: &Path) -> Result<Vec<Resolved>, String> {
    let flags = registry::load(&crate::context::flags_root())?;
    let choices = settings::load(home);
    Ok(flags
        .iter()
        .map(|flag| resolve(flag, &choices, &env_lookup))
        .collect())
}

/// Whether `name` is on: environment, then settings file, then its default.
/// An undeclared name is a bug at the call site: false, and loud in debug.
pub fn enabled(name: &str) -> bool {
    let known = resolve_all(&crate::context::default_home())
        .ok()
        .and_then(|all| all.into_iter().find(|entry| entry.flag.name == name));
    debug_assert!(known.is_some(), "flag '{name}' is not declared in flags/");
    known.is_some_and(|entry| entry.enabled)
}
