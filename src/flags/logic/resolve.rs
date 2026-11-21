//! Resolution, highest layer first: environment, settings file, default.

use super::settings_parse::Choices;
use crate::flags::constants::fixed::ENV_PREFIX;
use crate::flags::structs::flag::{Flag, Resolved, Source};

/// `TJ_FLAG_<NAME>`: the name upper-cased with `-` as `_`.
pub fn env_name(flag: &str) -> String {
    format!(
        "{ENV_PREFIX}{}",
        flag.to_ascii_uppercase().replace('-', "_")
    )
}

/// `1/0/on/off/true/false`, any case; anything else is ignored.
pub fn parse_switch(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "on" | "true" => Some(true),
        "0" | "off" | "false" => Some(false),
        _ => None,
    }
}

/// Pure resolution: `lookup` is the environment, injected for testing.
pub fn resolve(
    flag: &Flag,
    choices: &Choices,
    lookup: &dyn Fn(&str) -> Option<String>,
) -> Resolved {
    let from_env = lookup(&env_name(&flag.name)).and_then(|value| parse_switch(&value));
    let (enabled, source) = match (from_env, choices.get(&flag.name)) {
        (Some(value), _) => (value, Source::Env),
        (None, Some(value)) => (*value, Source::Settings),
        (None, None) => (flag.default, Source::Default),
    };
    Resolved {
        flag: flag.clone(),
        enabled,
        source,
    }
}
