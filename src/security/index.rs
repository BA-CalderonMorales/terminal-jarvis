//! Public face of the security domain: executable and environment checks.

#[path = "logic/mod.rs"]
mod logic;

pub use crate::security::logic::checks::{command_on_path, missing_env, resolve_on_path};
pub use crate::security::logic::package_check::{check as package_check, Verdict};
