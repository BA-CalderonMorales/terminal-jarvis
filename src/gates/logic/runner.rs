use crate::gates::logic::loader::{load, Gate};
use crate::gates::structs::state::selected;
use crate::{context, security};
use std::io::{Read, Write};
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
    if !security::command_on_path(&gate.binary) {
        eprintln!(
            "warning: optional gate '{}' is enabled but '{}' is not on PATH; {} Run `terminal-jarvis gate disable` to stop the warning, or install the scanner to start scanning.",
            gate.name, gate.binary, gate.install_hint
        );
        return Ok(());
    }
    eprintln!("running security gate '{}' ...", gate.name);
    let (code, output) = run(gate)?;
    if code == 0 {
        eprintln!("security gate '{}' passed", gate.name);
        return Ok(());
    }
    Err(format!(
        "security gate '{}' blocked harness execution (exit {code})\n{}",
        gate.name,
        block_summary(&output)
    ))
}

fn block_summary(output: &str) -> String {
    let lines = output
        .lines()
        .filter(|line| !line.contains(" INFO ") && !line.trim().is_empty())
        .collect::<Vec<_>>();
    let tail = lines.into_iter().rev().take(6).collect::<Vec<_>>();
    let mut lines = tail.into_iter().rev().collect::<Vec<_>>();
    if lines.is_empty() {
        lines.push(output.lines().next_back().unwrap_or("scan failed"));
    }
    lines.join("\n")
}

pub fn run(gate: &Gate) -> Result<(i32, String), String> {
    if !security::command_on_path(&gate.binary) {
        return Err(format!(
            "optional gate '{}' is enabled but '{}' is not on PATH. {}",
            gate.name, gate.binary, gate.install_hint
        ));
    }
    let mut child = Command::new(&gate.binary)
        .args(&gate.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to run gate '{}': {error}", gate.name))?;
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let stdout_reader = std::thread::spawn(move || tee(&mut stdout));
    let stderr_reader = std::thread::spawn(move || tee(&mut stderr));
    let status = child
        .wait()
        .map_err(|error| format!("gate scan failed: {error}"))?;
    let joined = [stdout_reader.join(), stderr_reader.join()]
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .join("\n");
    Ok((status.code().unwrap_or(1), joined.trim().to_string()))
}

fn tee(pipe: &mut dyn Read) -> String {
    let mut captured = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match pipe.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => {
                let _ = std::io::stderr().write_all(&chunk[..read]);
                let _ = std::io::stderr().flush();
                captured.extend_from_slice(&chunk[..read]);
            }
        }
    }
    String::from_utf8_lossy(&captured).trim().to_string()
}
#[cfg(test)]
#[path = "../tests/runner.rs"]
mod gates_runner_tests;
