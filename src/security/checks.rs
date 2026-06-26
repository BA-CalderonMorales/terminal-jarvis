use crate::contracts::{EnvMode, Harness};
use std::env;
use std::path::Path;

pub fn command_on_path(command: &str) -> bool {
    if command.contains('/') {
        return Path::new(command).exists();
    }
    let Some(path) = env::var_os("PATH") else {
        return false;
    };
    env::split_paths(&path).any(|dir| dir.join(command).exists())
}

pub fn missing_env(harness: &Harness) -> Vec<String> {
    match harness.env_mode {
        EnvMode::None => Vec::new(),
        EnvMode::Any => {
            if harness.env.iter().any(|name| env::var_os(name).is_some()) {
                Vec::new()
            } else {
                harness.env.clone()
            }
        }
        EnvMode::All => harness
            .env
            .iter()
            .filter(|name| env::var_os(name).is_none())
            .cloned()
            .collect(),
    }
}
