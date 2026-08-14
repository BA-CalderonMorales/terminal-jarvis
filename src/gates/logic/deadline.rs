//! A deadline wait for a gate scan child. Headless automation must never
//! block forever on a hung scanner (a stuck mount can idle a walk for
//! minutes): after the limit the child is killed and reaped, and the caller
//! reports the timeout. The limit is tunable via
//! TERMINAL_JARVIS_GATE_TIMEOUT_SECS; the default keeps legitimate slow-mount
//! scans (measured minutes) safe.

use std::process::{Child, ExitStatus};
use std::time::{Duration, Instant};

pub fn timeout_secs() -> u64 {
    std::env::var("TERMINAL_JARVIS_GATE_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(300)
}

/// Waits for the child up to `secs`. Returns the exit status and whether the
/// deadline fired (the child was killed with SIGKILL and reaped).
pub fn wait(child: &mut Child, secs: u64) -> std::io::Result<(ExitStatus, bool)> {
    let deadline = Instant::now() + Duration::from_secs(secs);
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok((status, false));
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
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
    fn reaps_a_fast_child_without_deadline() {
        let mut child = Command::new("true").spawn().unwrap();
        let (status, timed_out) = wait(&mut child, 5).unwrap();
        assert!(status.success());
        assert!(!timed_out);
    }

    #[cfg(unix)]
    #[test]
    fn kills_and_reaps_a_child_that_outlives_the_deadline() {
        let mut child = Command::new("sh").arg("-c").arg("sleep 30").spawn().unwrap();
        let (status, timed_out) = wait(&mut child, 0).unwrap();
        assert!(!status.success());
        assert!(timed_out);
    }
}
