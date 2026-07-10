use super::super::{style, table};
use crate::gates::Gate;
#[cfg(test)]
#[path = "gate_output_test.rs"]
mod tests;
pub fn disabled_status(available: &str) -> String {
    if style::plain() {
        return format!("gate: disabled\navailable: {available}\n");
    }
    table::fields(
        "Security Gate",
        &[
            ("STATUS", "disabled".to_string()),
            ("AVAILABLE", available.to_string()),
        ],
    )
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
    table::fields(
        "Security Gate",
        &[
            ("GATE", gate.name.clone()),
            ("SOURCE", source.to_string()),
            ("BINARY", binary.to_string()),
            (
                "COMMAND",
                format!("{} {}", gate.binary, gate.args.join(" ")),
            ),
        ],
    )
}
pub fn list(available: &[Gate]) -> String {
    if style::plain() {
        return available
            .iter()
            .map(|gate| format!("{} - {}\n", gate.name, gate.description))
            .collect();
    }
    let rows = available
        .iter()
        .map(|gate| {
            vec![
                gate.name.clone(),
                gate.display.clone(),
                gate.description.clone(),
            ]
        })
        .collect::<Vec<_>>();
    table::render(
        "Available Security Gates",
        &["NAME", "DISPLAY", "DESCRIPTION"],
        &rows,
    )
}

pub fn enabled(name: &str) -> String {
    if style::plain() {
        return format!("gate '{name}' enabled; harness commands will scan before execution\n");
    }
    format!(
        "{}\n{}",
        style::success("Security gate enabled"),
        table::fields(
            "Security Gate",
            &[("GATE", name.to_string()), ("STATUS", "active".to_string())],
        )
    )
}

pub fn disabled() -> String {
    if style::plain() {
        return "gate: disabled\n".to_string();
    }
    format!(
        "{}\n{}",
        style::success("Security gate disabled"),
        table::fields("Security Gate", &[("STATUS", "disabled".to_string())])
    )
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
