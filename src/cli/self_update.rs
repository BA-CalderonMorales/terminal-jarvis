use super::{style, table};
use std::process::{Command, Stdio};

#[path = "self_update_route.rs"]
mod route;

#[cfg(test)]
use route::{homebrew_path, wrapper_path};

pub fn run(dry_run: bool) -> Result<(i32, String), String> {
    if dry_run {
        return Ok((0, preview()));
    }
    match route::selected() {
        route::Route::Command { command, args, .. } => run_cmd(command, args),
        route::Route::Manual { guidance, .. } => Err(guidance.to_string()),
    }
}

pub fn preview() -> String {
    match route::selected() {
        route::Route::Command { command, args, .. } => dry_run_output(command, args),
        route::Route::Manual { guidance, .. } => guidance_output(guidance),
    }
}

pub fn route_name() -> &'static str {
    route::name()
}

fn run_cmd(cmd: &str, args: &[&str]) -> Result<(i32, String), String> {
    let mut command = Command::new(cmd);
    command.args(args).stderr(Stdio::piped());
    let output = command.output().map_err(|e| {
        format!(
            "failed to run '{}': {}; install {} or update manually",
            cmd, e, cmd
        )
    })?;
    let code = output.status.code().unwrap_or(1);
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if code == 0 {
        Ok((0, success_output(cmd)))
    } else {
        Err(format!(
            "'{} {}' exited with {code}{}",
            cmd,
            args.join(" "),
            if stderr.is_empty() {
                String::new()
            } else {
                format!(": {stderr}")
            }
        ))
    }
}

fn dry_run_output(command: &str, args: &[&str]) -> String {
    let value = format!("{command} {}", args.join(" "));
    if style::plain() {
        return format!("terminal-jarvis update plan: {value}\n");
    }
    table::fields("Self-Update Plan", &[("COMMAND", value)])
}

fn guidance_output(guidance: &str) -> String {
    if style::plain() {
        return format!("terminal-jarvis update plan: {guidance}\n");
    }
    table::fields("Self-Update Plan", &[("NEXT ACTION", guidance.to_string())])
}

fn success_output(command: &str) -> String {
    if style::plain() {
        return format!("terminal-jarvis updated via {command}\n");
    }
    format!(
        "{}\n{}",
        style::success("Terminal Jarvis updated"),
        table::fields("Self-Update", &[("METHOD", command.to_string())])
    )
}

#[cfg(test)]
#[path = "self_update_test.rs"]
mod tests;
