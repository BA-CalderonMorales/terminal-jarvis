//! Tool Update Operations
//!
//! Handles updating AI coding tools using the modular configuration system.
//! Uses InstallationManager for tool definitions instead of legacy TOML config.

use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use crate::tools::ToolManager;
use anyhow::{anyhow, Result};
use std::io::{self, Write};
use tokio::process::Command as AsyncCommand;

/// Handle updating packages - either a specific package or all packages
pub async fn handle_update_packages(package: Option<&str>) -> Result<()> {
    match package {
        Some(pkg) => update_single_package(pkg).await,
        None => update_all_packages().await,
    }
}

/// Update a specific package with progress tracking
async fn update_single_package(pkg: &str) -> Result<()> {
    let update_progress = ProgressContext::new(&format!("Updating {pkg}"));

    // Add a small delay to show the progress indicator
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    update_progress.update_message(&format!("Downloading latest version of {pkg}..."));

    let result = update_tool_using_install_manager(pkg).await;

    match result {
        Ok(_) => {
            update_progress.finish_success(&format!("{pkg} updated successfully"));
            Ok(())
        }
        Err(e) => {
            update_progress.finish_error(&format!("Failed to update {pkg}"));
            Err(e)
        }
    }
}

/// Update all installed packages with deterministic, non-animated feedback
async fn update_all_packages() -> Result<()> {
    let tools = selected_update_tools(ToolManager::get_installed_tools());
    if tools.is_empty() {
        ProgressUtils::info_message("No installed tools to update");
        return Ok(());
    }

    let total = tools.len();
    let headless = crate::cli_logic::cli_logic_headless::is_headless();

    if headless {
        println!("[INFO] Updating installed tools: {total} found");
    } else {
        println!("\nUpdating installed tools: {total} found");
    }

    let mut results = Vec::with_capacity(total);
    for (i, tool) in tools.iter().enumerate() {
        let position = i + 1;
        println!("[{position}/{total}] {tool} UPDATE start");
        let result = update_tool_using_install_manager(tool).await;
        match &result {
            Ok(()) => println!("[{position}/{total}] {tool} UPDATE ok"),
            Err(err) => println!(
                "[{position}/{total}] {tool} UPDATE failed: {}",
                truncate(&err.to_string(), 200)
            ),
        }
        results.push((tool.clone(), result));
        let _ = io::stdout().flush();
    }

    summarize_update_results(results)
}

fn selected_update_tools(mut installed_tools: Vec<String>) -> Vec<String> {
    installed_tools.sort();
    installed_tools.dedup();
    installed_tools
        .into_iter()
        .filter(|tool| InstallationManager::get_update_command(tool).is_some())
        .collect()
}

fn summarize_update_results(results: Vec<(String, Result<()>)>) -> Result<()> {
    let mut ok_count = 0;
    let mut failures = Vec::new();

    for (tool, result) in results {
        match result {
            Ok(()) => ok_count += 1,
            Err(err) => failures.push((tool, err.to_string())),
        }
    }

    let fail_count = failures.len();
    if fail_count == 0 {
        println!("[INFO] Update complete: {ok_count} OK, 0 FAIL");
        return Ok(());
    }

    eprintln!("[ERROR] Update complete: {ok_count} OK, {fail_count} FAIL");
    eprintln!("Failures:");
    for (tool, msg) in failures {
        eprintln!("- {tool}: {}", truncate(&msg, 200));
    }

    Err(anyhow!(
        "Update failed for {fail_count} tool{}",
        if fail_count == 1 { "" } else { "s" }
    ))
}

fn resolve_update_command(
    tool_name: &str,
) -> Result<crate::installation_arguments::InstallCommand> {
    InstallationManager::get_update_command(tool_name)
        .ok_or_else(|| anyhow!("Tool '{tool_name}' not found in configuration"))
}

fn command_display(command: &crate::installation_arguments::InstallCommand) -> String {
    if command.args.is_empty() {
        command.command.clone()
    } else {
        format!("{} {}", command.command, command.args.join(" "))
    }
}

fn command_failure_message(
    command: &crate::installation_arguments::InstallCommand,
    output: &std::process::Output,
) -> String {
    let code = output.status.code().unwrap_or(-1);
    let mut details = String::new();

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stderr = stderr.trim();
    if !stderr.is_empty() {
        details.push_str(stderr);
    }

    if details.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout = stdout.trim();
        if !stdout.is_empty() {
            details.push_str(stdout);
        }
    }

    if details.is_empty() {
        format!("{} exited {code}", command.command)
    } else {
        format!(
            "{} exited {code}: {}",
            command.command,
            truncate(&details, 200)
        )
    }
}

/// Truncate a string to a maximum length, appending ellipsis if needed.
/// Uses char-boundaries to avoid panicking on multi-byte UTF-8 codepoints.
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() > max {
        s.chars().take(max).collect::<String>() + "..."
    } else {
        s.to_string()
    }
}

/// Update a tool using the InstallationManager configuration
async fn update_tool_using_install_manager(tool_name: &str) -> Result<()> {
    let update_command = resolve_update_command(tool_name)?;
    execute_command(&update_command).await
}

/// Execute a shell command
async fn execute_command(command: &crate::installation_arguments::InstallCommand) -> Result<()> {
    if command.command.trim().is_empty() {
        return Err(anyhow!("Empty command"));
    }

    let output = if command.requires_sudo {
        let sudo_available = std::process::Command::new("which")
            .arg("sudo")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if sudo_available {
            let mut sudo_cmd = AsyncCommand::new("sudo");
            sudo_cmd.arg("-n");
            sudo_cmd.arg(&command.command);
            sudo_cmd.args(&command.args);
            sudo_cmd.stdin(std::process::Stdio::null());
            sudo_cmd.output().await?
        } else {
            let mut regular_cmd = AsyncCommand::new(&command.command);
            regular_cmd.args(&command.args);
            regular_cmd.stdin(std::process::Stdio::null());
            regular_cmd.output().await?
        }
    } else {
        let mut regular_cmd = AsyncCommand::new(&command.command);
        regular_cmd.args(&command.args);
        regular_cmd.stdin(std::process::Stdio::null());
        regular_cmd.output().await?
    };

    if !output.status.success() {
        return Err(anyhow!(
            "Command '{}' failed: {}",
            command_display(command),
            command_failure_message(command, &output)
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_update_tools_uses_installed_tools_only() {
        let selected = selected_update_tools(vec![
            "codex".to_string(),
            "claude".to_string(),
            "codex".to_string(),
            "__unknown__".to_string(),
        ]);

        assert_eq!(selected, vec!["claude".to_string(), "codex".to_string()]);
        assert!(selected.len() < InstallationManager::get_tool_names().len());
    }

    #[test]
    fn resolve_update_command_uses_tool_update_config() {
        let command = resolve_update_command("claude").expect("claude has update config");

        assert_eq!(command.command, "claude");
        assert_eq!(command.args, vec!["update".to_string()]);
        assert_ne!(command.command, "curl");
    }

    #[test]
    fn resolve_update_command_unknown_tool_fails_clearly() {
        let err = resolve_update_command("__missing_tool__").unwrap_err();

        assert!(
            err.to_string()
                .contains("Tool '__missing_tool__' not found in configuration"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn aggregate_update_failure_returns_error() {
        let result = summarize_update_results(vec![
            ("claude".to_string(), Ok(())),
            ("codex".to_string(), Err(anyhow!("npm exited 1"))),
        ]);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("1 tool"));
    }
}
