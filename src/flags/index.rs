//! Public face of the flags domain: feature flags declared as data.
//!
//! Code asks `flags::enabled("<name>")`; the registry lives in
//! `flags/<name>/index.toml` (embedded at build time, or a directory named by
//! `TERMINAL_JARVIS_FLAGS`); users choose through `settings.toml`, the
//! `settings` command, or `TJ_FLAG_<NAME>`. See docs/flags.md.

#[path = "constants/mod.rs"]
mod constants;
#[path = "logic/mod.rs"]
mod logic;
#[path = "structs/mod.rs"]
mod structs;

pub use crate::flags::logic::enabled::{enabled, resolve_all};
pub use crate::flags::logic::registry::{build, load};
pub use crate::flags::logic::resolve::{env_name, parse_switch, resolve};
pub use crate::flags::logic::settings::{load as load_choices, path as settings_path, save};
pub use crate::flags::logic::settings_parse::{parse as parse_choices, Choices};
pub use crate::flags::structs::flag::{Flag, Resolved, Source, Stage};

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
