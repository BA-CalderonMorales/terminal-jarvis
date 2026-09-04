//! Checks: the harness environment contract. A harness may require env
//! vars it cannot see a value for; `missing_env` names what is absent.

use crate::contracts::{EnvMode, Harness};
use std::env;

pub fn missing_env(harness: &Harness) -> Vec<String> {
    match harness.env_mode {
        EnvMode::None | EnvMode::Optional => Vec::new(),
        EnvMode::Any => {
            if harness.env.iter().any(|name| nonempty_env(name)) {
                Vec::new()
            } else {
                harness.env.clone()
            }
        }
        EnvMode::All => harness
            .env
            .iter()
            .filter(|name| !nonempty_env(name))
            .cloned()
            .collect(),
    }
}

fn nonempty_env(name: &str) -> bool {
    env::var(name).is_ok_and(|value| !value.trim().is_empty())
}

#[cfg(test)]
#[path = "../tests/checks.rs"]
mod security_checks_tests;

#[cfg(test)]
#[path = "../tests/checks_props.rs"]
mod props;
