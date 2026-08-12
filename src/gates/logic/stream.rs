use crate::gates::logic::loader::Gate;
use crate::security;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

/// Copies a child gate's pipe to stderr (live, tee-style) while capturing
/// the full bytes for the caller. The stderr copy is success-path narration:
/// it happens only when `narrate` is on, so a quiet tui never sees the raw
/// scan stream, but the capture (and therefore the block summary) is intact.
pub fn tee(pipe: &mut dyn Read, narrate: bool) -> String {
    let mut captured = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match pipe.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(read) => {
                if narrate {
                    let _ = std::io::stderr().write_all(&chunk[..read]);
                    let _ = std::io::stderr().flush();
                }
                captured.extend_from_slice(&chunk[..read]);
            }
        }
    }
    String::from_utf8_lossy(&captured).trim().to_string()
}

/// Spawns a gate scan and waits for it, streaming output live when asked.
/// The pid is handed to the interrupt tracker so Ctrl+C can SIGKILL a stuck
/// scanner; a signal-killed scan surfaces as exit >= 128.
pub fn run(gate: &Gate, narrate: bool) -> Result<(i32, String), String> {
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
    let (mut stdout, mut stderr) = (child.stdout.take().unwrap(), child.stderr.take().unwrap());
    super::interrupt::track(child.id() as i32);
    let stdout_reader = std::thread::spawn(move || tee(&mut stdout, narrate));
    let stderr_reader = std::thread::spawn(move || tee(&mut stderr, narrate));
    let status = child
        .wait()
        .map_err(|error| format!("gate scan failed: {error}"))?;
    super::interrupt::track(0);
    let joined = [stdout_reader.join(), stderr_reader.join()]
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .join("\n");
    let code = status.code().unwrap_or(128 + status.signal().unwrap_or(9));
    Ok((code, joined.trim().to_string()))
}

/// The meaningful tail of a failed scan report: drops trivy's INFO chatter
/// (both spaced and `?`-separated log formats) and keeps the signal lines.
pub fn block_summary(output: &str) -> String {
    let tail = output
        .lines()
        .filter(|line| !noise(line) && !line.trim().is_empty())
        .rev()
        .take(6)
        .collect::<Vec<_>>();
    let mut lines = tail.into_iter().rev().collect::<Vec<_>>();
    if lines.is_empty() {
        lines.push(output.lines().next_back().unwrap_or("scan failed"));
    }
    lines.join("\n")
}

fn noise(line: &str) -> bool {
    line.contains(" INFO ") || line.contains("?INFO?")
}
