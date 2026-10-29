#[path = "output_catalog.rs"]
mod catalog;
#[path = "output_show.rs"]
mod detail;
#[path = "output_diagnostics.rs"]
mod diagnostics;
#[path = "output_graveyard.rs"]
mod graveyard;
#[path = "output_readiness.rs"]
mod readiness;
#[path = "output_session.rs"]
mod session;
#[path = "output_summary.rs"]
mod summary;

use super::{style, table};
use crate::contracts::EnvMode;
use crate::contracts::Harness;
use crate::security;

pub use catalog::{list, plan, plan_with_extra};
pub use detail::show;
pub use diagnostics::diagnostics;
pub use graveyard::graveyard;
pub use readiness::is_harness_ready;
pub use session::{current, help, selected};
pub use summary::{audit, status};

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
        EnvMode::Any => format!("missing one of {}", missing.join(", ")),
        EnvMode::All => format!("missing {}", missing.join(", ")),
        EnvMode::None | EnvMode::Optional => "ready".to_string(),
    }
}

#[cfg(test)]
#[path = "../tests/output_test.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/output_graveyard_test.rs"]
mod graveyard_tests;
