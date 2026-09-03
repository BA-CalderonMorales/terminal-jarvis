//! Security: PATH resolution and the harness environment contract.

#[path = "logic/mod.rs"]
mod logic;

pub use crate::security::logic::checks::missing_env;
pub use crate::security::logic::package_check::{check as package_check, Verdict};
pub use crate::security::logic::path::{candidates, command_on_path, resolve_on_path};
pub use crate::security::logic::resolve::resolved;
