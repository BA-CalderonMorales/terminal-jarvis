use crate::{context, contracts::Harness};
use std::path::Path;

use super::{style, table};

pub fn run(words: &[String], harnesses: &[Harness], home: &Path) -> Result<String, String> {
    match words {
        [action] if action == "dashboard" => dashboard(harnesses, home),
        _ => Err("usage: terminal-jarvis experimental dashboard".to_string()),
    }
}

fn dashboard(harnesses: &[Harness], home: &Path) -> Result<String, String> {
    if std::env::var("TERMINAL_JARVIS_EXPERIMENTAL_UI").as_deref() != Ok("1") {
        return Err(
            "experimental dashboard is disabled; set TERMINAL_JARVIS_EXPERIMENTAL_UI=1".to_string(),
        );
    }
    let active = context::load(home)
        .map_err(|error| error.to_string())?
        .map(|session| session.active_harness)
        .unwrap_or_else(|| "none".to_string());
    let ready = harnesses
        .iter()
        .filter(|harness| super::output::is_harness_ready(harness))
        .count();
    if style::plain() {
        return Ok(format!(
            "Terminal Jarvis\nexperimental dashboard\nactive harness: {active}\nreadiness: {ready}/{} harnesses\nmode: headless command center\n",
            harnesses.len()
        ));
    }
    Ok(format!(
        "{}{}",
        style::banner("Terminal Jarvis", "Experimental dashboard"),
        table::fields(
            "Dashboard",
            &[
                ("ACTIVE HARNESS", active),
                (
                    "READINESS",
                    format!("{ready}/{} harnesses", harnesses.len())
                ),
                ("MODE", "headless command center".to_string()),
            ],
        )
    ))
}
