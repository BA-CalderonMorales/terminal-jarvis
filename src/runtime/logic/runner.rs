use crate::contracts::CapabilityPlan;
use std::io;
use std::process::{Command, Stdio};

pub fn run_command(plan: &CapabilityPlan, extra: &[String]) -> io::Result<i32> {
    let mut command = Command::new(resolved_binary(&plan.command.command));
    command.args(&plan.command.args).args(extra);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    #[cfg(unix)]
    reset_sigint_in_child(&mut command);
    command.status().map(status_code)
}

/// On Windows, `Command::new` does not expand `PATHEXT`, so a bare name like
/// `opencode` never resolves to `opencode.CMD` and spawning fails with
/// `NotFound` even though the harness is installed. Resolve the real
/// candidate first; fall back to the original name (letting the spawn fail
/// naturally with `NotFound`) when nothing on `PATH` matches.
#[cfg(windows)]
fn resolved_binary(command: &str) -> String {
    crate::security::resolve_on_path(command).unwrap_or_else(|| command.to_string())
}

#[cfg(not(windows))]
fn resolved_binary(command: &str) -> String {
    command.to_string()
}

#[cfg(unix)]
fn reset_sigint_in_child(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    unsafe {
        command.pre_exec(|| {
            signal(SIGINT, SIG_DFL);
            Ok(())
        });
    }
}

#[cfg(unix)]
extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
}

#[cfg(unix)]
const SIGINT: i32 = 2;
#[cfg(all(unix, test))]
const SIG_IGN: usize = 1;
#[cfg(unix)]
const SIG_DFL: usize = 0;

fn status_code(status: std::process::ExitStatus) -> i32 {
    status.code().unwrap_or_else(|| signal_code(&status))
}

#[cfg(unix)]
fn signal_code(status: &std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    status.signal().map_or(1, |signal| 128 + signal)
}

#[cfg(not(unix))]
fn signal_code(_status: &std::process::ExitStatus) -> i32 {
    1
}

// Exercises `sh`/POSIX signal semantics (`kill -TERM $$`, the 128+signal
// exit-code convention `signal_code` only computes on `#[cfg(unix)]`) that
// have no Windows equivalent, so the whole module is Unix-only.
#[cfg(all(test, unix))]
#[path = "../tests/runner_test.rs"]
mod tests;
