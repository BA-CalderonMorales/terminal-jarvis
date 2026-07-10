use crate::context::Session;
use crate::contracts::Harness;
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use super::cache::handle as cache;
use super::compat_support::auth_status;
#[path = "compat_config.rs"]
mod config_output;
#[path = "compat_output.rs"]
mod output;

pub fn update_summary(harnesses: &[Harness]) -> String {
    output::updates(VERSION, harnesses)
}

pub fn auth(words: &[String], harnesses: &[Harness]) -> Result<String, String> {
    match words {
        [] => Ok(output::auth_notice(VERSION)),
        [action] if action == "manage" => Ok(output::auth_notice(VERSION)),
        [action, name] if action == "help" => auth_for(name, harnesses),
        [action, name] if action == "set" => auth_set_for(name, harnesses),
        [name] => auth_for(name, harnesses),
        _ => Err("usage: terminal-jarvis auth [help|set] <harness>".to_string()),
    }
}

pub fn config(
    words: &[String],
    catalog_root: &Path,
    home: &Path,
    session: Option<Session>,
) -> Result<String, String> {
    match words {
        [] => Ok(config_output::show(catalog_root, home, session)),
        [action] if action == "show" => Ok(config_output::show(catalog_root, home, session)),
        [action] if action == "path" => Ok(config_output::paths(catalog_root, home)),
        [action] if action == "reset" => Ok(config_output::reset(VERSION)),
        _ => Err("usage: terminal-jarvis config [show|path|reset]".to_string()),
    }
}

pub fn legacy(command: &str) -> String {
    config_output::legacy(command)
}

fn auth_for(name: &str, harnesses: &[Harness]) -> Result<String, String> {
    auth_detail(
        name,
        harnesses,
        &format!("credential storage is not active in v{VERSION}; export env vars in your shell"),
    )
}

fn auth_set_for(name: &str, harnesses: &[Harness]) -> Result<String, String> {
    auth_detail(name, harnesses, "terminal-jarvis does not persist credentials; nothing was stored. Export the env vars in your shell")
}

fn auth_detail(name: &str, harnesses: &[Harness], note: &str) -> Result<String, String> {
    let harness = harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| format!("unknown harness '{name}'"))?;
    Ok(output::auth_detail(harness, &auth_status(harness), note))
}

#[cfg(test)]
#[path = "compat_test.rs"]
mod tests;
