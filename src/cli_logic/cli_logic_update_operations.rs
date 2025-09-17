//! Tool Update Operations
//!
//! Handles updating AI coding tools using the modular configuration system.
//! Uses InstallationManager for tool definitions instead of legacy TOML config.

use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use anyhow::{anyhow, Result};
use std::time::Duration;
use tokio::process::Command as AsyncCommand;
use tokio::task::JoinSet;

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

/// Update all packages concurrently with clean, non-animated output and summary
async fn update_all_packages() -> Result<()> {
    let tools = InstallationManager::get_tool_names();
    if tools.is_empty() {
        ProgressUtils::info_message("No tools to update");
        return Ok(());
    }

    let total = tools.len();
    println!("\nUpdating tools concurrently ({total})...");
    println!("----------------------------------------------------------------");
    let mut join_set: JoinSet<(String, Result<()>)> = JoinSet::new();

    for (i, tool) in tools.iter().cloned().enumerate() {
        println!("* [START] Updating {tool} ({}/{})", i + 1, total);
        join_set.spawn(async move {
            // Small stagger to create a natural startup cadence
            tokio::time::sleep(Duration::from_millis((i as u64) * 50)).await;
            let res = update_tool_using_install_manager(&tool).await;
            (tool, res)
        });
    }

    let mut had_errors = false;
    let mut results: Vec<(String, bool, String)> = Vec::with_capacity(total);

    while let Some(join_res) = join_set.join_next().await {
        match join_res {
            Ok((tool, Ok(()))) => {
                println!("* [OK]    {tool} updated successfully");
                results.push((tool, true, String::from("updated")));
            }
            Ok((tool, Err(e))) => {
                had_errors = true;
                println!("* [FAIL]  {tool} — {}", e);
                results.push((tool, false, e.to_string()));
            }
            Err(e) => {
                had_errors = true;
                println!("* [FAIL]  update task join error — {}", e);
            }
        }
    }

    // Final summary table (ASCII only)
    println!("\nUpdate summary:");
    println!("----------------+--------+----------------------------------------");
    println!("{:<16}| {:<6} | DETAILS", "TOOL", "STATUS");
    println!("----------------+--------+----------------------------------------");
    // Sort results by tool name for deterministic output
    results.sort_by(|a, b| a.0.cmp(&b.0));
    for (tool, ok, info) in results {
        let status = if ok { "OK" } else { "FAIL" };
        let max_details = 56usize; // keep table within ~80 cols
        let details = if info.len() > max_details {
            format!("{}…", &info[..max_details])
        } else {
            info
        };
        println!("{:<16}| {:<6} | {}", tool, status, details);
    }
    println!("----------------+--------+----------------------------------------\n");

    if had_errors {
        println!("Some packages failed to update. See details above.");
    } else {
        println!("All packages updated successfully.");
    }

    Ok(())
}

/// Update a tool using the InstallationManager configuration
async fn update_tool_using_install_manager(tool_name: &str) -> Result<()> {
    let install_commands = InstallationManager::get_install_commands();

    let install_info = install_commands
        .get(tool_name)
        .ok_or_else(|| anyhow!("Tool '{}' not found in configuration", tool_name))?;

    // Convert install command to update command
    let update_command =
        if install_info.command == "npm" && install_info.args.contains(&"install".to_string()) {
            // Convert npm install to npm update and remove version specifiers
            let mut update_args = Vec::new();
            for arg in &install_info.args {
                if arg == "install" {
                    update_args.push("update".to_string());
                } else if arg.contains("@latest") {
                    // Remove @latest from package names
                    // Examples:
                    // @qwen-code/qwen-code@latest -> @qwen-code/qwen-code
                    // opencode-ai@latest -> opencode-ai
                    let package_name = arg.replace("@latest", "");
                    update_args.push(package_name);
                } else {
                    update_args.push(arg.clone());
                }
            }
            format!("{} {}", install_info.command, update_args.join(" "))
        } else {
            // For non-npm commands, use the install command as-is
            format!("{} {}", install_info.command, install_info.args.join(" "))
        };

    execute_command(&update_command).await
}

/// Execute a shell command
async fn execute_command(command: &str) -> Result<()> {
    let mut parts = command.split_whitespace();
    let cmd = parts.next().ok_or_else(|| anyhow!("Empty command"))?;
    let args: Vec<&str> = parts.collect();

    // For npm global updates, try with sudo if available to handle permission issues
    let status = if cmd == "npm" && args.contains(&"-g") {
        // Check if sudo is available
        let sudo_available = std::process::Command::new("which")
            .arg("sudo")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if sudo_available {
            let mut sudo_cmd = AsyncCommand::new("sudo");
            sudo_cmd.arg(cmd);
            sudo_cmd.args(&args);
            sudo_cmd.stdout(std::process::Stdio::null());
            sudo_cmd.stderr(std::process::Stdio::null());
            sudo_cmd.status().await?
        } else {
            // Fallback to regular command if sudo isn't available
            let mut regular_cmd = AsyncCommand::new(cmd);
            regular_cmd.args(&args);
            regular_cmd.stdout(std::process::Stdio::null());
            regular_cmd.stderr(std::process::Stdio::null());
            regular_cmd.status().await?
        }
    } else {
        // Non-global npm commands or other commands
        let mut regular_cmd = AsyncCommand::new(cmd);
        regular_cmd.args(&args);
        regular_cmd.stdout(std::process::Stdio::null());
        regular_cmd.stderr(std::process::Stdio::null());
        regular_cmd.status().await?
    };

    if !status.success() {
        return Err(anyhow!(
            "Command '{}' failed with exit code: {}",
            command,
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}
