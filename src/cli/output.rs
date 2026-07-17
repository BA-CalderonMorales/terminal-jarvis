#[path = "output_catalog.rs"]
mod catalog;
#[path = "output_diagnostics.rs"]
mod diagnostics;
#[path = "output_readiness.rs"]
mod readiness;
#[path = "output_summary.rs"]
mod summary;

use super::{style, table};
use crate::contracts::Harness;
use crate::{context::Session, security};

pub use catalog::{list, plan, plan_with_extra, show};
pub use diagnostics::diagnostics;
pub use readiness::is_harness_ready;
pub use summary::{audit, status};

pub fn help() -> String {
    super::help::text()
}

pub fn current(session: Option<Session>) -> String {
    let active = session
        .map(|session| session.active_harness)
        .unwrap_or_else(|| "none".to_string());
    if style::plain() {
        return format!("active harness = {active}\n");
    }
    table::fields("Active Harness", &[("HARNESS", active)])
}

pub fn selected(name: &str) -> String {
    if style::plain() {
        return format!("active harness = {name}\n");
    }
    format!(
        "{}\n{}",
        style::success("Active harness updated"),
        table::fields("Active Harness", &[("HARNESS", name.to_string())])
    )
}

pub fn checks(harnesses: &[Harness]) -> String {
    if style::plain() {
        return plain_checks(harnesses);
    }
    let rows = harnesses
        .iter()
        .map(|harness| {
            let binary = if security::command_on_path(&harness.binary) {
                "found"
            } else {
                "missing"
            };
            vec![
                harness.name.clone(),
                binary.to_string(),
                env_status(harness, &security::missing_env(harness)),
            ]
        })
        .collect::<Vec<_>>();
    table::render(
        "Harness Readiness",
        &["HARNESS", "BINARY", "ENVIRONMENT"],
        &rows,
    )
}

fn plain_checks(harnesses: &[Harness]) -> String {
    let mut out = String::new();
    for harness in harnesses {
        let binary = if security::command_on_path(&harness.binary) {
            "found"
        } else {
            "missing"
        };
        let env = env_status(harness, &security::missing_env(harness));
        out.push_str(&format!("{} binary={} env={}\n", harness.name, binary, env));
    }
    out
}

fn env_status(harness: &Harness, missing: &[String]) -> String {
    if missing.is_empty() {
        return "ready".to_string();
    }
    match harness.env_mode {
        crate::contracts::EnvMode::Any => format!("missing one of {}", missing.join(", ")),
        crate::contracts::EnvMode::All => format!("missing {}", missing.join(", ")),
        crate::contracts::EnvMode::None => "ready".to_string(),
    }
}

#[cfg(test)]
#[path = "output_test.rs"]
mod tests;
