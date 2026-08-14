//! A deadline wait for a gate scan child. Headless automation must never
//! block forever on a hung scanner (a stuck mount can idle a walk for
//! minutes): after the limit the child is killed and reaped, and the caller
//! reports the timeout. The limit is tunable via
//! TERMINAL_JARVIS_GATE_TIMEOUT_SECS (clamped to [0, 86400]); the default
//! keeps legitimate slow-mount scans (measured minutes) safe.

use std::process::{Child, ExitStatus};
use std::time::{Duration, Instant};

pub fn timeout_secs() -> u64 {
    std::env::var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .map(|secs| secs.min(86400))
        .unwrap_or(300)
}

/// Waits for the child up to `secs`. Returns the exit status and whether the
/// deadline fired (the child was killed with SIGKILL and reaped). A child
/// that exits between the deadline check and the kill reports itself as
/// finished, not timed out.
pub fn wait(child: &mut Child, secs: u64) -> std::io::Result<(ExitStatus, bool)> {
    let deadline = Instant::now() + Duration::from_secs(secs);
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok((status, false));
        }
        if Instant::now() >= deadline && child.kill().is_ok() {
            let status = child.wait()?;
            return Ok((status, true));
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn timeout_secs_defaults_and_clamps() {
        let previous = std::env::var_os("TERMINAL_JARVIS_GATE_TIMEOUT_SECS");
        for (value, expected) in [
            (None, 300),
            (Some("1"), 1),
            (Some(" 30 "), 30),
            (Some("abc"), 300),
            (Some("-5"), 300),
            (Some("86401"), 86400),
            (Some("999999999999999999999999"), 300),
            (Some("0"), 0),
        ] {
            match value {
                Some(value) => std::env::set_var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS", value),
                None => std::env::remove_var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS"),
            }
            assert_eq!(timeout_secs(), expected, "value={value:?}");
        }
        match previous {
            Some(value) => std::env::set_var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS", value),
            None => std::env::remove_var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn reaps_a_fast_child_without_deadline() {
        let mut child = Command::new("true").spawn().unwrap();
        let (status, timed_out) = wait(&mut child, 5).unwrap();
        assert!(status.success());
        assert!(!timed_out);
    }

    #[cfg(unix)]
    #[test]
    fn kills_and_reaps_a_child_that_outlives_the_deadline() {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("sleep 30")
            .spawn()
            .unwrap();
        let (status, timed_out) = wait(&mut child, 0).unwrap();
        assert!(!status.success());
        assert!(timed_out);
    }

    #[cfg(unix)]
    #[test]
    fn an_already_exited_child_is_not_reported_timed_out() {
        let mut child = Command::new("true").spawn().unwrap();
        std::thread::sleep(Duration::from_millis(50));
        let (status, timed_out) = wait(&mut child, 0).unwrap();
        assert!(status.success());
        assert!(!timed_out, "a child that finished must not read as a kill");
    }
}
