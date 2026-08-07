//! Public face of the security domain: executable and environment checks.

pub use crate::security::logic::checks::{command_on_path, missing_env};
