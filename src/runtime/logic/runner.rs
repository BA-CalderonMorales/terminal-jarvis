use crate::contracts::CapabilityPlan;
use std::io;
use std::process::{Command, Stdio};

pub fn run_command(plan: &CapabilityPlan, extra: &[String]) -> io::Result<i32> {
    let mut command = Command::new(&plan.command.command);
    command.args(&plan.command.args).args(extra);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    command.status().map(status_code)
}

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
