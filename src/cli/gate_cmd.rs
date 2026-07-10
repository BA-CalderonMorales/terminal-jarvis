use crate::{context, gates, security};
use std::path::Path;

#[path = "gate_output.rs"]
mod output;

pub fn handle(words: &[String], home: &Path) -> Result<(i32, String), String> {
    let available = gates::load(&context::gates_root()).map_err(|error| error.to_string())?;
    match words {
        [] => status(&available, home).map(|body| (0, body)),
        [action] if action == "status" => status(&available, home).map(|body| (0, body)),
        [action] if action == "list" => Ok((0, output::list(&available))),
        [action] if action == "enable" => enable(&available, home, "trivy"),
        [action, name] if action == "enable" => enable(&available, home, name),
        [action] if action == "disable" => {
            gates::disable(home).map_err(|error| error.to_string())?;
            Ok((0, output::disabled()))
        }
        [action] if action == "run" => run(find(&available, "trivy")?),
        [action, name] if action == "run" => run(find(&available, name)?),
        _ => Err(
            "usage: terminal-jarvis gate [status|list|enable [trivy]|disable|run [trivy]]"
                .to_string(),
        ),
    }
}

fn status(available: &[gates::Gate], home: &Path) -> Result<String, String> {
    let Some(selection) = gates::selected(home).map_err(|error| error.to_string())? else {
        return Ok(output::disabled_status(&names(available)));
    };
    let gate = find(available, &selection.name)?;
    let binary = if security::command_on_path(&gate.binary) {
        "found"
    } else {
        "missing"
    };
    Ok(output::configured(gate, selection.source, binary))
}

fn enable(available: &[gates::Gate], home: &Path, name: &str) -> Result<(i32, String), String> {
    let gate = find(available, name)?;
    gates::enable(home, &gate.name).map_err(|error| error.to_string())?;
    Ok((0, output::enabled(&gate.name)))
}

fn run(gate: &gates::Gate) -> Result<(i32, String), String> {
    let (code, body) = gates::run(gate)?;
    Ok((code, output::run_result(&gate.name, code, &body)))
}

fn find<'a>(available: &'a [gates::Gate], name: &str) -> Result<&'a gates::Gate, String> {
    available
        .iter()
        .find(|gate| gate.name == name)
        .ok_or_else(|| format!("unknown gate '{name}'"))
}

fn names(available: &[gates::Gate]) -> String {
    available
        .iter()
        .map(|gate| gate.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}
