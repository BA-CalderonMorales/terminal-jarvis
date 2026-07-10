use super::{style, table};
use std::process::{Command, Stdio};

pub fn run(dry_run: bool) -> Result<(i32, String), String> {
    let (command, args) = update_command();
    if dry_run {
        return Ok((0, dry_run_output(command, args)));
    }
    run_cmd(command, args)
}

fn update_command() -> (&'static str, &'static [&'static str]) {
    if wrapper_path().is_some() {
        return ("npm", &["install", "-g", "terminal-jarvis@latest"]);
    }
    let raw = std::env::var("TERMINAL_JARVIS_DISTRIBUTION").unwrap_or_default();
    match raw.as_str() {
        "github-release" | "github-release-cache" => {
            return ("npm", &["install", "-g", "terminal-jarvis@latest"])
        }
        "source" | "env" => return ("cargo", &["install", "terminal-jarvis"]),
        _ => {}
    }
    let path = std::env::current_exe()
        .ok()
        .map(|binary| binary.to_string_lossy().to_string())
        .unwrap_or_default();
    if path.contains("homebrew") || path.contains("Cellar") {
        return ("brew", &["upgrade", "terminal-jarvis"]);
    }
    ("cargo", &["install", "terminal-jarvis"])
}

fn wrapper_path() -> Option<std::path::PathBuf> {
    let wrapper = std::env::var("TERMINAL_JARVIS_WRAPPER").ok()?;
    let pkg = std::path::Path::new(&wrapper)
        .parent()
        .and_then(std::path::Path::parent)?
        .join("package.json");
    pkg.exists().then_some(pkg)
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
