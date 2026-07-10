use super::super::{style, table};
use super::{checks, is_harness_ready};
use crate::contracts::Harness;

pub fn status(harnesses: &[Harness]) -> String {
    summary(harnesses, "status")
}

pub fn audit(harnesses: &[Harness]) -> String {
    summary(harnesses, "audit summary")
}

fn summary(harnesses: &[Harness], label: &str) -> String {
    let ready = harnesses
        .iter()
        .filter(|harness| is_harness_ready(harness))
        .count();
    if style::plain() {
        return format!(
            "{}\n{}: {ready}/{} harnesses ready\n",
            checks(harnesses),
            label,
            harnesses.len()
        );
    }
    let title = if label == "status" {
        "Security Status"
    } else {
        "Security Audit"
    };
    format!(
        "{}\n{}",
        checks(harnesses),
        table::fields(
            title,
            &[("READY", format!("{ready}/{} harnesses", harnesses.len()))]
        )
    )
}
