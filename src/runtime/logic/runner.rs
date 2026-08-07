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

#[cfg(test)]
mod tests {
    use super::*;

    fn code_of(script: &str) -> i32 {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(script)
            .status()
            .unwrap();
        status_code(status)
    }

    #[test]
    fn maps_exit_codes_and_signal_terms() {
        assert_eq!(code_of("exit 0"), 0);
        assert_eq!(code_of("exit 7"), 7);
        assert_eq!(code_of("kill -TERM $$"), 143);
    }
}
