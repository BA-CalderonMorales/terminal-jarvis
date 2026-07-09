use crate::context::Session;
use crate::contracts::{Capability, Harness};
use std::path::Path;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use super::cache::handle as cache;
use super::compat_support::{auth_notice, auth_status, config_show};

pub fn update_summary(harnesses: &[Harness]) -> String {
    let mut out = format!("updates are per harness in v{VERSION}\n");
    out.push_str("run `terminal-jarvis update <harness>` to execute one update\n");
    for harness in harnesses {
        let plan = harness.plan(Capability::Update).expect("validated update");
        out.push_str(&format!("{}: {}\n", harness.name, plan.command.render()));
    }
    out
}

pub fn auth(words: &[String], harnesses: &[Harness]) -> Result<String, String> {
    match words {
        [] => Ok(auth_notice()),
        [action] if action == "manage" => Ok(auth_notice()),
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
        [] => Ok(config_show(catalog_root, home, session)),
        [action] if action == "show" => Ok(config_show(catalog_root, home, session)),
        [action] if action == "path" => Ok(format!(
            "home: {}\ncatalog: {}\n",
            home.display(),
            catalog_root.display()
        )),
        [action] if action == "reset" => Ok(format!(
            "config reset is not automatic in v{VERSION}; remove TERMINAL_JARVIS_HOME after review\n"
        )),
        _ => Err("usage: terminal-jarvis config [show|path|reset]".to_string()),
    }
}

pub fn legacy(command: &str) -> String {
    format!(
        "{command} was removed with the v0.1 catalog rewrite.\n\
         Use harness commands instead: list, show, plan, run, install, update, auth, security.\n"
    )
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
    Ok(format!(
        "auth for {} ({})\nsetup: {}\nstatus: {}\n{}\n",
        harness.display,
        harness.name,
        harness.setup_hint(),
        auth_status(harness),
        note
    ))
}

#[cfg(test)]
#[path = "compat_test.rs"]
mod tests;
