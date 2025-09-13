//! Tool Update Operations
//! 
//! Handles updating AI coding tools using the modular configuration system.
//! Uses InstallationManager for tool definitions instead of legacy TOML config.

use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use anyhow::{anyhow, Result};
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

/// Update all packages with progress tracking and error handling
async fn update_all_packages() -> Result<()> {
    let overall_progress = ProgressContext::new("Updating all packages");
    let tools = InstallationManager::get_tool_names();
    let mut had_errors = false;

    for (index, tool) in tools.iter().enumerate() {
        overall_progress.update_message(&format!(
            "Updating {tool} ({}/{})...",
            index + 1,
            tools.len()
        ));

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        if let Err(e) = update_tool_using_install_manager(tool).await {
            ProgressUtils::error_message(&format!("Failed to update {tool}: {e}"));
            had_errors = true;
        } else {
            ProgressUtils::success_message(&format!("{tool} updated successfully"));
        }
    }

    if had_errors {
        overall_progress.finish_error("Some packages failed to update");
    } else {
        overall_progress.finish_success("All packages updated successfully");
    }

    Ok(())
}

/// Update a tool using the InstallationManager configuration
async fn update_tool_using_install_manager(tool_name: &str) -> Result<()> {
    let install_commands = InstallationManager::get_install_commands();
    
    let install_info = install_commands
        .get(tool_name)
        .ok_or_else(|| anyhow!("Tool '{}' not found in configuration", tool_name))?;
    
    println!("Updating {}...", tool_name);
    
    // Convert install command to update command
    let update_command = if install_info.command == "npm" && install_info.args.contains(&"install".to_string()) {
        // Convert npm install to npm update
        let mut update_args = install_info.args.clone();
        if let Some(pos) = update_args.iter().position(|x| x == "install") {
            update_args[pos] = "update".to_string();
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
