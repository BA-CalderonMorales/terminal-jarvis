use crate::cli_logic::themed_components::themed_confirm;
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use crate::tools::ToolManager;
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use tokio::process::Command as AsyncCommand;

/// Check if the npm global prefix directory is writable by the current user.
/// Returns true for NVM users or when npm is configured with user-writable paths.
fn is_npm_global_writable() -> bool {
    let output = std::process::Command::new("npm")
        .args(["prefix", "-g"])
        .output();

    let Ok(out) = output else { return false };
    if !out.status.success() {
        return false;
    }

    let prefix = PathBuf::from(String::from_utf8_lossy(&out.stdout).trim());
    let node_modules = prefix.join("lib").join("node_modules");
    let lib_dir = prefix.join("lib");

    // Check writable directory in order of preference: node_modules > lib > prefix
    if node_modules.exists() {
        is_directory_writable(&node_modules)
    } else if lib_dir.exists() {
        is_directory_writable(&lib_dir)
    } else {
        is_directory_writable(&prefix)
    }
}

/// Check if a directory is writable by the current user using Unix permissions.
fn is_directory_writable(path: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;

    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };

    let mode = metadata.mode();
    let uid = unsafe { libc::getuid() };
    let gid = unsafe { libc::getgid() };
    let file_uid = metadata.uid();
    let file_gid = metadata.gid();

    let is_owner_writable = uid == file_uid && (mode & 0o200) != 0;
    let is_group_writable = gid == file_gid && (mode & 0o020) != 0;
    let is_other_writable = (mode & 0o002) != 0;

    is_owner_writable || is_group_writable || is_other_writable
}

/// Get the user-local npm prefix directory for global installs.
/// Uses ~/.local/share/npm-global following XDG conventions.
fn get_user_npm_prefix() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local/share/npm-global")
}

/// Ensure the user npm prefix directory exists and return the prefix path.
fn ensure_user_npm_prefix() -> Result<PathBuf> {
    let prefix = get_user_npm_prefix();
    std::fs::create_dir_all(prefix.join("bin"))?;
    Ok(prefix)
}

/// Handle running a specific AI coding tool with arguments
pub async fn handle_run_tool(tool: &str, args: &[String]) -> Result<()> {
    // Get install command to check dependencies
    let install_cmd = InstallationManager::get_install_command(tool)
        .ok_or_else(|| anyhow!("Tool {tool} not found in configuration"))?;

    // Check appropriate dependencies based on installation method
    if install_cmd.requires_npm && !InstallationManager::check_npm_available() {
        ProgressUtils::warning_message("Node.js runtime environment not detected");
        println!("  Tool {tool} requires NPM but it's not available.");
        println!("  Please install Node.js to continue: https://nodejs.org/");
        return Err(anyhow!("Node.js runtime required"));
    }

    if install_cmd.command == "curl" && !InstallationManager::check_curl_available() {
        ProgressUtils::warning_message("curl not found");
        println!("  Tool {tool} requires curl but it's not available.");
        println!("  Please install curl to continue.");
        return Err(anyhow!("curl required"));
    }

    if install_cmd.command == "uv" && !InstallationManager::check_uv_available() {
        ProgressUtils::warning_message("uv not found");
        println!("  Tool {tool} requires uv but it's not available.");
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
            return Err(anyhow!("Tool '{tool}' is required but not installed"));
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
        .ok_or_else(|| anyhow!("Tool '{tool}' not found in installation registry"))?;

    // Check dependencies based on installation method
    if install_cmd.requires_npm {
        let npm_check = ProgressContext::new("Checking NPM availability");

        if !InstallationManager::check_npm_available() {
            npm_check.finish_error("Node.js ecosystem not detected");
            println!("  Please install Node.js and NPM first: https://nodejs.org/");
            return Err(anyhow!(
                "NPM is required to install {tool} but is not available"
            ));
        }

        npm_check.finish_success("NPM is available");
    }

    if install_cmd.command == "curl" {
        let curl_check = ProgressContext::new("Checking curl availability");

        if !InstallationManager::check_curl_available() {
            curl_check.finish_error("curl not found");
            println!("  Please install curl first (usually available by default on most systems)");
            return Err(anyhow!(
                "curl is required to install {tool} but is not available"
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
                "uv is required to install {tool} but is not available"
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

    // Track if we installed to user directory (for PATH guidance)
    let mut used_user_prefix: Option<PathBuf> = None;

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
        // Issue #37 & #39: Handle NPM global installs without sudo
        // NVM users have writable npm prefix; system npm users need user-local prefix
        let is_writable = is_npm_global_writable();

        if !is_writable {
            let user_prefix = ensure_user_npm_prefix()?;
            let user_bin = user_prefix.join("bin");

            used_user_prefix = Some(user_prefix.clone());

            let mut args_with_prefix = install_cmd.args.clone();
            args_with_prefix.extend(["--prefix".to_string(), user_prefix.display().to_string()]);

            progress.info_inline(&format!(
                "Installing to user directory: {}",
                user_prefix.display()
            ));

            let current_path = std::env::var("PATH").unwrap_or_default();
            let new_path = format!("{}:{}", user_bin.display(), current_path);

            let mut user_cmd = AsyncCommand::new(&install_cmd.command);
            user_cmd.args(&args_with_prefix);
            user_cmd.stdout(std::process::Stdio::null());
            user_cmd.stderr(std::process::Stdio::inherit());
            user_cmd.env("PATH", &new_path);

            std::env::set_var("PATH", &new_path);
            user_cmd.status().await?
        } else {
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::inherit());
            cmd.status().await?
        }
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

            // Provide PATH guidance for user-prefix installations
            if let Some(prefix) = &used_user_prefix {
                let bin_path = prefix.join("bin").to_string_lossy().to_string();
                ProgressUtils::info_message(
                    "To use this tool in future sessions, add to your shell profile:",
                );
                println!("  export PATH=\"{bin_path}:$PATH\"");
            }
        } else {
            verify_progress.finish_error(&format!("{tool} installation could not be verified"));

            // Provide PATH guidance based on installation type
            if let Some(prefix) = &used_user_prefix {
                let bin_path = prefix.join("bin").to_string_lossy().to_string();
                ProgressUtils::warning_message(
                    "Tool installed but not found in PATH. Add to your shell profile:",
                );
                println!("  export PATH=\"{bin_path}:$PATH\"");
                ProgressUtils::info_message("Then restart your terminal or run: source ~/.bashrc");
            } else if tool == "opencode" {
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
        Err(anyhow!("Failed to install {tool}"))
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
            ProgressUtils::info_message(&format!("Quick launch: {tool}"));
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
