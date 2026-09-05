use crate::gates::logic::heartbeat::{Heartbeat, Scan};
use crate::gates::logic::loader::Gate;
use crate::security;
use std::io::{Read, Write};
use std::process::{Command, ExitStatus, Stdio};
use std::time::Instant;

/// The scan's numeric outcome: the real exit code, or 128 + the signal on
/// unix when a scan was killed outright.
#[cfg(unix)]
fn exit_code(status: &ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    status.code().unwrap_or(128 + status.signal().unwrap_or(9))
}

/// The scan's numeric outcome on platforms without a signal model.
#[cfg(not(unix))]
fn exit_code(status: &ExitStatus) -> i32 {
    status.code().unwrap_or(128)
}

/// Copies a child gate's pipe to stderr (live, tee-style) while capturing
/// the full bytes for the caller; narrate controls the live copy.
pub fn tee(pipe: &mut dyn Read, narrate: bool) -> String {
    let mut captured = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match pipe.read(&mut chunk) {
            Ok(0) => break,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => break,
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

/// Spawns a gate scan and waits for it (deadline-bounded), streaming live
/// when asked and redrawing a heartbeat; Ctrl+C SIGKILLs via the tracker.
pub fn run(gate: &Gate, narrate: bool) -> Result<Scan, String> {
    if !security::command_on_path(&gate.binary) {
        return Err(format!(
            "optional gate '{}' is enabled but '{}' is not on PATH. {}",
            gate.name, gate.binary, gate.install_hint
        ));
    }
    let mut child = Command::new(security::resolved(&gate.binary).as_ref())
        .args(&gate.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to run gate '{}': {error}", gate.name))?;
    let (mut stdout, mut stderr) = (child.stdout.take().unwrap(), child.stderr.take().unwrap());
    super::interrupt::track(child.id() as i32);
    let stdout_reader = std::thread::spawn(move || tee(&mut stdout, narrate));
    let stderr_reader = std::thread::spawn(move || tee(&mut stderr, narrate));
    let started = Instant::now();
    let mut heartbeat =
        (!narrate).then(|| Heartbeat::start(&format!("security scan ({}) ...", gate.name)));
    let limit = super::deadline::timeout_secs();
    let (status, timed_out) = super::deadline::wait(&mut child, limit)
        .map_err(|error| format!("gate scan failed: {error}"))?;
    let elapsed = started.elapsed();
    super::interrupt::track(0);
    let fired = heartbeat.as_ref().is_some_and(|tick| tick.fired());
    if let Some(tick) = &mut heartbeat {
        tick.stop();
    }
    let joined =
        super::interrupt::bounded_join(vec![stdout_reader, stderr_reader], timed_out).join("\n");
    let mut output = joined.trim().to_string();
    if timed_out {
        if !output.is_empty() {
            output.push('\n');
        }
        let name = &gate.name;
        output.push_str(&format!(
            "security gate '{name}' timed out after {limit}s and was killed"
        ));
    }
    Ok(Scan {
        code: exit_code(&status),
        output,
        heartbeat: fired,
        elapsed,
    })
}
