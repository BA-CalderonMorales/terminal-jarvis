/// Prefix of the per-flag environment override: `TJ_FLAG_<NAME>`.
pub const ENV_PREFIX: &str = "TJ_FLAG_";
/// The user's choices live in this file inside the Terminal Jarvis home.
pub const SETTINGS_FILE: &str = "settings.toml";
pub const SETTINGS_TABLE: &str = "flags";
pub const MAX_NAME: usize = 40;
pub const MAX_DESCRIPTION: usize = 100;
