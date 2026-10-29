//! The session surface: which harness is active. `use` and `current` are one
//! fact read two ways, so they share a file instead of widening `output.rs`.

use super::super::{style, table};
use crate::context::Session;

pub fn help() -> String {
    super::super::help::text()
}

pub fn current(session: Option<Session>) -> String {
    let active = session.map(|s| s.active_harness).unwrap_or("none".into());
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
