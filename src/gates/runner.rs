use super::{load, selected, Gate};
use crate::{context, security};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn preflight(home: &Path) -> Result<(), String> {
    let Some(selection) = selected(home).map_err(|error| error.to_string())? else {
        return Ok(());
    };
    let gates = load(&context::gates_root()).map_err(|error| error.to_string())?;
    let gate = gates
        .iter()
        .find(|gate| gate.name == selection.name)
        .ok_or_else(|| {
            format!(
                "enabled gate '{}' is not in the gate catalog",
                selection.name
            )
        })?;
    let (code, output) = run(gate)?;
    if code == 0 {
        return Ok(());
    }
    Err(format!(
        "security gate '{}' blocked harness execution (exit {code})\n{output}\nrun `terminal-jarvis gate status` for configuration",
        gate.name
    ))
}

pub fn run(gate: &Gate) -> Result<(i32, String), String> {
    if !security::command_on_path(&gate.binary) {
        return Err(format!(
            "optional gate '{}' is enabled but '{}' is not on PATH. {}",
            gate.name, gate.binary, gate.install_hint
        ));
    }
    let output = Command::new(&gate.binary)
        .args(&gate.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("failed to run gate '{}': {error}", gate.name))?;
    let body = [
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
        String::from_utf8_lossy(&output.stderr).trim().to_string(),
    ]
    .into_iter()
    .filter(|part| !part.is_empty())
    .collect::<Vec<_>>()
    .join("\n");
    Ok((output.status.code().unwrap_or(1), body))
}

#[cfg(test)]
#[path = "runner_test.rs"]
mod tests;
