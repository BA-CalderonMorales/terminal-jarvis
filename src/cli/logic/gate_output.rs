//! GateOutput: the gate screens in the human-line style of `show` --
//! identity lines, grouped label/value fields, cell-aware wrapping.
//! `--plain` stays line-oriented for automation.

use crate::cli::logic::{output_fields as fields, style};
use crate::gates::Gate;

pub fn disabled_status(available: &str) -> String {
    if style::plain() {
        return format!("gate: disabled\navailable: {available}\n");
    }
    let width = fields::width();
    let mut lines = vec!["Security gate".to_string(), String::new()];
    lines.extend(fields::field("status", "disabled", width));
    lines.extend(fields::field("available", available, width));
    lines.join("\n")
}

pub fn configured(gate: &Gate, source: &str, binary: &str) -> String {
    if style::plain() {
        return format!(
            "gate: {} ({source})\nbinary: {binary}\ncommand: {} {}\n",
            gate.name,
            gate.binary,
            gate.args.join(" ")
        );
    }
    let width = fields::width();
    let mut lines = vec![format!("{} ({})", gate.display, gate.name)];
    lines.extend(fields::section(String::new(), &gate.description, width));
    lines.push(String::new());
    lines.extend(fields::field("status", "enabled", width));
    lines.extend(fields::field("source", source, width));
    lines.extend(fields::field("binary", binary, width));
    lines.extend(fields::field(
        "command",
        &format!("{} {}", gate.binary, gate.args.join(" ")),
        width,
    ));
    lines.join("\n")
}

pub fn list(available: &[Gate]) -> String {
    if style::plain() {
        return available
            .iter()
            .map(|gate| format!("{} - {}\n", gate.name, gate.description))
            .collect();
    }
    let width = fields::width();
    let sections: Vec<Vec<String>> = available
        .iter()
        .map(|gate| {
            let mut lines = vec![format!("{} ({})", gate.display, gate.name)];
            lines.extend(fields::section(String::new(), &gate.description, width));
            lines
        })
        .collect();
    sections
        .iter()
        .map(|section| section.join("\n"))
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn enabled(name: &str) -> String {
    if style::plain() {
        return format!("gate '{name}' enabled; harness commands will scan before execution\n");
    }
    let width = fields::width();
    let mut lines = vec![style::success("Security gate enabled"), String::new()];
    lines.extend(fields::field("gate", name, width));
    lines.extend(fields::field("status", "active", width));
    lines.join("\n")
}

pub fn disabled() -> String {
    if style::plain() {
        return "gate: disabled\n".to_string();
    }
    let width = fields::width();
    let mut lines = vec![style::success("Security gate disabled"), String::new()];
    lines.extend(fields::field("status", "disabled", width));
    lines.join("\n")
}

pub fn run_result(name: &str, code: i32, body: &str) -> String {
    let label = if code == 0 { "passed" } else { "blocked" };
    if style::plain() {
        return format!("gate '{name}' {label}\n{body}\n");
    }
    let title = if code == 0 {
        style::success(&format!("Security gate '{name}' passed"))
    } else {
        style::warning(&format!("Security gate '{name}' blocked execution"))
    };
    format!("{title}\n\n{body}\n")
}
