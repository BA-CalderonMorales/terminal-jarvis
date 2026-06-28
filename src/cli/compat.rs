use crate::context::Session;
use crate::contracts::{Capability, EnvMode, Harness};
use crate::security;
use std::path::Path;

pub use super::cache::handle as cache;

pub fn update_summary(harnesses: &[Harness]) -> String {
    let mut out = "updates are per harness in v0.1.2\n".to_string();
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
        [action, name] if action == "help" || action == "set" => auth_for(name, harnesses),
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
        [action] if action == "reset" => Ok(
            "config reset is not automatic in v0.1.2; remove TERMINAL_JARVIS_HOME after review\n"
                .to_string(),
        ),
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
    let harness = harnesses
        .iter()
        .find(|harness| harness.name == name)
        .ok_or_else(|| format!("unknown harness '{name}'"))?;
    Ok(format!(
        "auth for {} ({})\nsetup: {}\nstatus: {}\ncredential storage is not active in v0.1.2; export env vars in your shell\n",
        harness.display,
        harness.name,
        harness.setup_hint(),
        auth_status(harness)
    ))
}

fn auth_status(harness: &Harness) -> String {
    let missing = security::missing_env(harness);
    if missing.is_empty() {
        return "ready".to_string();
    }
    match harness.env_mode {
        EnvMode::Any => format!("missing one of: {}", missing.join(", ")),
        EnvMode::All => format!("missing: {}", missing.join(", ")),
        EnvMode::None => "ready".to_string(),
    }
}

fn auth_notice() -> String {
    "credential manager is not active in v0.1.2\n\
     use `terminal-jarvis auth help <harness>` and export the listed env vars\n"
        .to_string()
}

fn config_show(catalog_root: &Path, home: &Path, session: Option<Session>) -> String {
    let active = session
        .map(|session| session.active_harness)
        .unwrap_or_else(|| "none".to_string());
    format!(
        "home: {}\ncatalog: {}\nactive harness: {}\n",
        home.display(),
        catalog_root.display(),
        active
    )
}
