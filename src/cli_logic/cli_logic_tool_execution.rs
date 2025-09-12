use crate::cli_logic::cli_logic_utilities::get_themed_render_config;
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use crate::tools::ToolManager;
use anyhow::{anyhow, Result};
use inquire::Confirm;
use tokio::process::Command as AsyncCommand;

/// Handle running a specific AI coding tool with arguments
pub async fn handle_run_tool(tool: &str, args: &[String]) -> Result<()> {
    // Check if NPM is available first
    if !InstallationManager::check_npm_available() {
        ProgressUtils::warning_message("Node.js runtime environment not detected");
        println!("  Most AI coding tools are distributed via the NPM ecosystem.");
        println!("  Please install Node.js to continue: https://nodejs.org/");
        return Err(anyhow!("Node.js runtime required"));
    }

    // Check if tool is installed with progress
    let check_progress = ProgressContext::new(&format!("Checking {tool} availability"));
    let cli_command = ToolManager::get_cli_command(tool);

    // Add a small delay to show the progress indicator
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    if !ToolManager::check_tool_installed(cli_command) {
        check_progress.finish_error(&format!("Tool '{tool}' is not installed"));

        let should_install = match Confirm::new(&format!("Install '{tool}' now?"))
            .with_render_config(get_themed_render_config())
            .with_default(true)
            .prompt()
        {
            Ok(result) => result,
            Err(_) => {
                // User interrupted - treat as "no"
                return Err(anyhow!("Installation cancelled"));
            }
        };

        if should_install {
            handle_install_tool(tool).await?;
            ProgressUtils::success_message("Installation complete!");
        } else {
            return Err(anyhow!("Tool '{}' is required but not installed", tool));
        }
    } else {
        check_progress.finish_success(&format!("{tool} is available"));
    }

    // Show startup progress for the tool
    let args_display = if args.is_empty() {
        "no arguments".to_string()
    } else {
        format!("arguments: {}", args.join(" "))
    };

    let startup_progress = ProgressContext::new(&format!("Launching {tool}"));
    startup_progress.update_message(&format!("Launching {tool} with {args_display}"));

    // Add a brief delay to show startup progress
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
    startup_progress.finish_success(&format!("Starting {tool}"));

    // Special handling for opencode - ensure clean terminal state
    if tool == "opencode" {
        prepare_opencode_terminal_state().await?;
    }

    ToolManager::run_tool(tool, args).await
}

/// Handle installing a specific AI coding tool
pub async fn handle_install_tool(tool: &str) -> Result<()> {
    let install_cmd = InstallationManager::get_install_command(tool)
        .ok_or_else(|| anyhow!("Tool '{}' not found in installation registry", tool))?;

    // Check NPM availability with progress
    if install_cmd.requires_npm {
        let npm_check = ProgressContext::new("Checking NPM availability");

        if !InstallationManager::check_npm_available() {
            npm_check.finish_error("Node.js ecosystem not detected");
            println!("  Please install Node.js and NPM first: https://nodejs.org/");
            return Err(anyhow!(
                "NPM is required to install {} but is not available",
                tool
            ));
        }

        npm_check.finish_success("NPM is available");
    }

    // Create installation progress
    let progress = ProgressContext::new(&format!("Installing {tool}"));
    progress.update_message(&format!(
        "Installing {tool} using: {} {}",
        install_cmd.command,
        install_cmd.args.join(" ")
    ));

    // For NPM packages, simulate realistic installation progress
    if install_cmd.requires_npm {
        ProgressUtils::simulate_installation_progress(&progress.spinner, tool).await;
    }

    let mut cmd = AsyncCommand::new(install_cmd.command);
    cmd.args(&install_cmd.args);

    // For NPM global installs, use sudo if available to handle permission issues
    let status = if install_cmd.requires_npm && install_cmd.args.contains(&"-g") {
        // Check if sudo is available
        let sudo_available = std::process::Command::new("which")
            .arg("sudo")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if sudo_available {
            let mut sudo_cmd = AsyncCommand::new("sudo");
            sudo_cmd.arg(install_cmd.command);
            sudo_cmd.args(&install_cmd.args);
            sudo_cmd.stdout(std::process::Stdio::null());
            sudo_cmd.stderr(std::process::Stdio::null());
            sudo_cmd.status().await?
        } else {
            // Fallback to regular command if sudo isn't available
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());
            cmd.status().await?
        }
    } else {
        // Non-global NPM installs or non-NPM commands
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());
        cmd.status().await?
    };

    if status.success() {
        progress.finish_success(&format!("{tool} installed successfully!"));

        // Verify installation with progress - add delay for PATH updates
        let verify_progress = ProgressContext::new(&format!("Verifying {tool} installation"));

        // Wait a bit for PATH updates and system to recognize new binary
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        ProgressUtils::simulate_verification_progress(&verify_progress.spinner, tool).await;

        let cli_command = ToolManager::get_cli_command(tool);
        if ToolManager::check_tool_installed(cli_command) {
            verify_progress.finish_success(&format!("{tool} is ready to use"));
        } else {
            verify_progress.finish_error(&format!("{tool} installation could not be verified"));

            // For opencode, provide additional guidance
            if tool == "opencode" {
                ProgressUtils::warning_message(
                    "OpenCode requires shell environment refresh to update PATH",
                );
                ProgressUtils::info_message(
                    "Quick fix: Run 'source ~/.bashrc' or restart your terminal",
                );
            }
        }

        Ok(())
    } else {
        progress.finish_error(&format!("Failed to install {tool}"));
        Err(anyhow!("Failed to install {}", tool))
    }
}

/// Prepare terminal state specifically for opencode to ensure proper input focus
async fn prepare_opencode_terminal_state() -> Result<()> {
    use std::io::Write;

    // Force flush any remaining output and reset terminal
    print!("\x1b[2J\x1b[H\x1b[?25h"); // Clear screen, home cursor, show cursor
    std::io::stdout().flush().unwrap_or_default();
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    Ok(())
}
