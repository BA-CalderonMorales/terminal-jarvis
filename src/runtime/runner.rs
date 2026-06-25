use crate::contracts::CapabilityPlan;
use std::io;
use std::process::Command;

pub fn run_command(plan: &CapabilityPlan, extra: &[String]) -> io::Result<i32> {
    let mut command = Command::new(&plan.command.command);
    command.args(&plan.command.args).args(extra);
    let status = command.status()?;
    Ok(status.code().unwrap_or(1))
}
