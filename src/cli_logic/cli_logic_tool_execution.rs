use crate::cli_logic::themed_components::themed_confirm;
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use crate::tools::ToolManager;
use anyhow::{anyhow, Result};
use dirs::home_dir;
use std::env::{join_paths, split_paths};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use tokio::process::Command as AsyncCommand;

fn resolve_npm_prefix(detected_prefix: Option<PathBuf>, home_dir: &Path) -> PathBuf {
    if let Some(prefix) = detected_prefix {
        if prefix.starts_with(home_dir) {
            return prefix;
        }
    }

    home_dir.join(".npm-global")
}

fn detect_npm_prefix() -> Option<PathBuf> {
    if let Ok(override_prefix) = std::env::var("TERMINAL_JARVIS_NPM_PREFIX_OVERRIDE") {
        return Some(PathBuf::from(override_prefix));
    }

    if let Ok(prefix) = std::env::var("NPM_CONFIG_PREFIX") {
        return Some(PathBuf::from(prefix));
    }

    let output = match StdCommand::new("npm")
        .args(["config", "get", "prefix"])
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            eprintln!("Warning: failed to read npm prefix: {}", err);
            return None;
        }
    };

    if !output.status.success() {
        eprintln!(
            "Warning: npm config get prefix exited with status {}",
            output.status
        );
        return None;
    }

    let prefix_lossy = String::from_utf8_lossy(&output.stdout);
    let prefix_trimmed = prefix_lossy.trim();
    if prefix_trimmed.is_empty() {
        None
    } else {
        Some(PathBuf::from(prefix_trimmed))
    }
}

fn fallback_path_from_exe() -> OsString {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(dir) = exe_path.parent() {
            if let Ok(joined) = join_paths([dir]) {
                return joined;
            }
        }
    }

    #[cfg(windows)]
    {
        join_paths(["C:\\Windows\\System32", "C:\\Windows"]).unwrap_or_else(|_| OsString::new())
    }

    #[cfg(not(windows))]
    {
        join_paths(["/usr/local/bin", "/usr/bin", "/bin"]).unwrap_or_else(|_| OsString::new())
    }
}

fn configure_npm_prefix_env() -> Result<(PathBuf, OsString)> {
    let home_dir = home_dir().ok_or_else(|| anyhow!("Unable to determine home directory"))?;
    let detected_prefix = detect_npm_prefix();
    let prefix = resolve_npm_prefix(detected_prefix, &home_dir);
    let bin_dir = prefix.join("bin");

    if prefix.starts_with(&home_dir) {
        if let Err(err) = fs::create_dir_all(&bin_dir) {
            eprintln!(
                "Warning: unable to prepare npm prefix directory {}: {}",
                bin_dir.display(),
                err
            );
            return Err(anyhow!(
                "Failed to create npm prefix directory {}",
                bin_dir.display()
            ));
        }
    }

    let (path_os, path_initially_missing) = match std::env::var_os("PATH") {
        Some(val) => (val, false),
        None => (fallback_path_from_exe(), true),
    };
    let current_paths: Vec<PathBuf> = split_paths(&path_os).collect();
    let mut final_path = path_os;
    if !current_paths.iter().any(|p| p == &bin_dir) {
        let mut updated_paths = Vec::with_capacity(current_paths.len() + 1);
        updated_paths.push(bin_dir.clone());
        updated_paths.extend(current_paths);
        if let Ok(joined) = join_paths(&updated_paths) {
            std::env::set_var("PATH", &joined);
            final_path = joined;
        } else {
            eprintln!(
                "Warning: unable to update PATH to include {}",
                bin_dir.display()
            );
        }
    } else if path_initially_missing {
        std::env::set_var("PATH", &final_path);
    }

    std::env::set_var("NPM_CONFIG_PREFIX", &prefix);
    Ok((prefix, final_path))
}

/// Handle running a specific AI coding tool with arguments
pub async fn handle_run_tool(tool: &str, args: &[String]) -> Result<()> {
    // Get install command to check dependencies
    let install_cmd = InstallationManager::get_install_command(tool)
        .ok_or_else(|| anyhow!("Tool {} not found in configuration", tool))?;

    // Check appropriate dependencies based on installation method
    if install_cmd.requires_npm && !InstallationManager::check_npm_available() {
        ProgressUtils::warning_message("Node.js runtime environment not detected");
        println!("  Tool {} requires NPM but it's not available.", tool);
        println!("  Please install Node.js to continue: https://nodejs.org/");
        return Err(anyhow!("Node.js runtime required"));
    }

    if install_cmd.command == "curl" && !InstallationManager::check_curl_available() {
        ProgressUtils::warning_message("curl not found");
        println!("  Tool {} requires curl but it's not available.", tool);
        println!("  Please install curl to continue.");
        return Err(anyhow!("curl required"));
    }

    if install_cmd.command == "uv" && !InstallationManager::check_uv_available() {
        ProgressUtils::warning_message("uv not found");
        println!("  Tool {} requires uv but it's not available.", tool);
        println!(
            "  Please install uv from https://docs.astral.sh/uv/getting-started/installation/"
        );
        return Err(anyhow!("uv required"));
    }

    // Check if tool is installed with progress
    let check_progress = ProgressContext::new(&format!("Checking {tool} availability"));
    let cli_command = ToolManager::get_cli_command(tool);

    // Add a small delay to show the progress indicator
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    if !ToolManager::check_tool_installed(cli_command) {
        check_progress.finish_error(&format!("Tool '{tool}' is not installed"));

        let should_install = match themed_confirm(&format!("Install '{tool}' now?"))
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

    // Save last-used tool for quick access
    let _ = crate::cli_logic::cli_logic_first_run::save_last_used_tool(tool);

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

    let mut npm_prefix: Option<(PathBuf, OsString)> = None;

    // Check dependencies based on installation method
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
        npm_prefix = Some(configure_npm_prefix_env()?);
    }

    if install_cmd.command == "curl" {
        let curl_check = ProgressContext::new("Checking curl availability");

        if !InstallationManager::check_curl_available() {
            curl_check.finish_error("curl not found");
            println!("  Please install curl first (usually available by default on most systems)");
            return Err(anyhow!(
                "curl is required to install {} but is not available",
                tool
            ));
        }

        curl_check.finish_success("curl is available");
    }

    if install_cmd.command == "uv" {
        let uv_check = ProgressContext::new("Checking uv availability");

        if !InstallationManager::check_uv_available() {
            uv_check.finish_error("uv not found");
            println!("  Please install uv first: https://docs.astral.sh/uv/getting-started/installation/");
            return Err(anyhow!(
                "uv is required to install {} but is not available",
                tool
            ));
        }

        uv_check.finish_success("uv is available");
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

    let mut cmd = AsyncCommand::new(&install_cmd.command);
    cmd.args(&install_cmd.args);
    if let Some((prefix, path)) = &npm_prefix {
        cmd.env("NPM_CONFIG_PREFIX", prefix);
        cmd.env("PATH", path);
    }

    // Handle special installation types
    let status = if let Some(pipe_to) = &install_cmd.pipe_to {
        // Handle curl-based installations that pipe to bash (e.g., goose)
        let curl_output = AsyncCommand::new(&install_cmd.command)
            .args(&install_cmd.args)
            .output()
            .await?;

        if !curl_output.status.success() {
            return Err(anyhow::anyhow!("Failed to download installation script"));
        }

        let mut bash_cmd = AsyncCommand::new(pipe_to);
        bash_cmd.stdin(std::process::Stdio::piped());
        bash_cmd.stdout(std::process::Stdio::null());
        bash_cmd.stderr(std::process::Stdio::null());

        let mut child = bash_cmd.spawn()?;
        if let Some(stdin) = child.stdin.as_mut() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(&curl_output.stdout).await?;
        }
        child.wait().await?
    } else if install_cmd.requires_npm && install_cmd.args.contains(&"-g".to_string()) {
        // Issue #37: DO NOT use sudo for NPM global installs
        // When npm is installed via NVM, it's not in sudo's PATH, causing silent failures.
        // NVM sets up permissions correctly so sudo is not needed for global installs.
        // Show stderr so users can see any error messages for debugging.
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::inherit()); // Show errors to user
        cmd.status().await?
    } else {
        // Regular command execution (uv, cargo, etc.)
        // Show stderr so users can see any error messages for debugging
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::inherit()); // Show errors to user
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

/// Quick launch mode: immediately launch the last-used tool without prompts
/// This implements Issue #26's goal of minimal steps to launch
pub async fn handle_quick_launch() -> Result<()> {
    use crate::cli_logic::cli_logic_first_run::{get_last_used_tool, get_last_used_tool_async};

    // Try database first (async), fall back to file-based
    let last_tool = get_last_used_tool_async().await.or_else(get_last_used_tool);

    match last_tool {
        Some(tool) => {
            ProgressUtils::info_message(&format!("Quick launch: {}", tool));
            handle_run_tool(&tool, &[]).await
        }
        None => {
            ProgressUtils::warning_message("No last-used tool found");
            println!("  Use 'terminal-jarvis <tool>' to launch a tool directly");
            println!("  Available tools: claude, gemini, qwen, opencode, codex, aider, amp, goose, crush, llxprt");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_npm_prefix;
    use std::path::PathBuf;

    #[test]
    fn resolve_npm_prefix_keeps_home_prefix() {
        let home_dir = PathBuf::from("/home/user");
        let prefix = PathBuf::from("/home/user/.nvm");
        let resolved = resolve_npm_prefix(Some(prefix.clone()), &home_dir);
        assert_eq!(resolved, prefix);
    }

    #[test]
    fn resolve_npm_prefix_falls_back_for_system_prefix() {
        let home_dir = PathBuf::from("/home/user");
        let prefix = PathBuf::from("/usr/local");
        let resolved = resolve_npm_prefix(Some(prefix), &home_dir);
        assert_eq!(resolved, home_dir.join(".npm-global"));
    }
}
