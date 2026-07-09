use crate::contracts::CapabilityPlan;
use std::io;
use std::process::{Command, Stdio};

pub fn run_command(plan: &CapabilityPlan, extra: &[String]) -> io::Result<(i32, String)> {
    let mut command = Command::new(&plan.command.command);
    command.args(&plan.command.args).args(extra);
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::piped());
    let output = command.output()?;
    let code = output.status.code().unwrap_or(1);
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if code == 0 {
        Ok((0, String::new()))
    } else {
        Ok((code, stderr))
    }
}
