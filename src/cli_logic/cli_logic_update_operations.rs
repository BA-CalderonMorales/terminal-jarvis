//! Tool Update Operations
//!
//! Handles updating AI coding tools using the modular configuration system.
//! Uses InstallationManager for tool definitions instead of legacy TOML config.

use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{self, Write};
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

/// Update all packages concurrently with minimal, non-animated feedback
async fn update_all_packages() -> Result<()> {
    let tools = InstallationManager::get_tool_names();
    if tools.is_empty() {
        ProgressUtils::info_message("No tools to update");
        return Ok(());
    }

    let total = tools.len();

    println!("\nUpdating {} tools concurrently...", total);

    // Compute fixed column widths for consistent alignment
    let idx_width = total.to_string().len().max(2); // width for the left index
    let name_width = tools.iter().map(|t| t.len()).max().unwrap_or(0).max(8);

    // Map tool -> index for quick updates
    let mut index_map: HashMap<String, usize> = HashMap::new();

    // Print all lines upfront with DOWNLOADING state
    for (i, tool) in tools.iter().enumerate() {
        index_map.insert(tool.clone(), i);
        println!(
            "[{:>idx$}/{}]  {:<name$}  DOWNLOADING",
            i + 1,
            total,
            tool,
            idx = idx_width,
            name = name_width
        );
    }
    let _ = io::stdout().flush();

    // Helper to update a single printed line in-place
    let update_line = |tool_index: usize, status: &str| {
        let lines_up = total - tool_index; // from one line below the last entry
                                           // Build the updated line text
        let line_text = format!(
            "[{:>idx$}/{}]  {:<name$}  {}",
            tool_index + 1,
            total,
            tools[tool_index],
            status,
            idx = idx_width,
            name = name_width
        );
        // Move up, clear line, write, move back down
        print!("\x1b[{}A", lines_up);
        print!("\r\x1b[2K{}", line_text);
        print!("\x1b[{}B\r", lines_up);
        let _ = io::stdout().flush();
    };

    // Spawn all updates after printing lines to ensure they show immediately
    let mut join_set: JoinSet<(String, Result<()>)> = JoinSet::new();
    for (i, tool) in tools.iter().cloned().enumerate() {
        join_set.spawn(async move {
            tokio::time::sleep(Duration::from_millis((i as u64) * 50)).await;
            let res = update_tool_using_install_manager(&tool).await;
            (tool, res)
        });
    }

    let mut ok_count: usize = 0;
    let mut fail_count: usize = 0;
    let mut failures: Vec<(String, String)> = Vec::new();

    while let Some(join_res) = join_set.join_next().await {
        match join_res {
            Ok((tool, Ok(()))) => {
                ok_count += 1;
                if let Some(&idx) = index_map.get(&tool) {
                    update_line(idx, "DONE");
                }
            }
            Ok((tool, Err(e))) => {
                fail_count += 1;
                let msg = truncate(&e.to_string(), 80);
                if let Some(&idx) = index_map.get(&tool) {
                    update_line(idx, "FAILED");
                }
                failures.push((tool, msg));
            }
            Err(e) => {
                fail_count += 1;
                let msg = truncate(&format!("join error: {}", e), 80);
                // We cannot map to a specific line, so append failure list only
                failures.push(("<task>".to_string(), msg));
            }
        }
    }

    // Compact final status
    println!("\nUpdate complete: {} OK, {} FAIL", ok_count, fail_count);
    if !failures.is_empty() {
        println!("Failures:");
        for (tool, msg) in failures {
            println!("- {}: {}", tool, msg);
        }
    }
    // Ensure we end on a clean new line and flush, so follow-up prompts work reliably
    let _ = io::stdout().flush();

    Ok(())
}

/// Truncate a string to a maximum length, appending ellipsis if needed
fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max])
    } else {
        s.to_string()
    }
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
            // Non-interactive sudo to avoid blocking on password prompts
            sudo_cmd.arg("-n");
            sudo_cmd.arg(cmd);
            sudo_cmd.args(&args);
            // Ensure child doesn't read stdin; we suppress all IO
            sudo_cmd.stdin(std::process::Stdio::null());
            sudo_cmd.stdout(std::process::Stdio::null());
            sudo_cmd.stderr(std::process::Stdio::null());
            let status = sudo_cmd.status().await?;
            if status.success() {
                status
            } else {
                // Fallback to running without sudo to avoid interactive password prompts
                let mut regular_cmd = AsyncCommand::new(cmd);
                regular_cmd.args(&args);
                regular_cmd.stdin(std::process::Stdio::null());
                regular_cmd.stdout(std::process::Stdio::null());
                regular_cmd.stderr(std::process::Stdio::null());
                regular_cmd.status().await?
            }
        } else {
            // Fallback to regular command if sudo isn't available
            let mut regular_cmd = AsyncCommand::new(cmd);
            regular_cmd.args(&args);
            regular_cmd.stdin(std::process::Stdio::null());
            regular_cmd.stdout(std::process::Stdio::null());
            regular_cmd.stderr(std::process::Stdio::null());
            regular_cmd.status().await?
        }
    } else {
        // Non-global npm commands or other commands
        let mut regular_cmd = AsyncCommand::new(cmd);
        regular_cmd.args(&args);
        regular_cmd.stdin(std::process::Stdio::null());
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
