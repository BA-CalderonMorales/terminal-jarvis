use crate::contracts::CapabilityPlan;
use std::io;
use std::process::Command;

pub fn run_command(plan: &CapabilityPlan, extra: &[String]) -> io::Result<(i32, String)> {
    let mut command = Command::new(&plan.command.command);
    command.args(&plan.command.args).args(extra);
    let output = command.output()?;
    let code = output.status.code().unwrap_or(1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let captured = [stdout, stderr]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    Ok((code, captured))
}
