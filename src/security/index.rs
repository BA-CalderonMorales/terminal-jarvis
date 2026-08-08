//! Public face of the security domain: executable and environment checks.

mod logic;

pub use crate::security::logic::checks::{command_on_path, missing_env};
