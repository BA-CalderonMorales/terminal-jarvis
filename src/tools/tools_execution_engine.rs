// Tools Execution Engine Domain
// Handles tool execution, session continuation, and argument processing

use anyhow::Result;
use std::process::Command;

use super::tools_command_mapping::get_cli_command;
use super::tools_detection::check_tool_installed;
use super::tools_process_management::{
    prepare_opencode_terminal_state, run_opencode_with_clean_exit,
};
use super::tools_startup_guidance::show_tool_startup_guidance;
use crate::auth_manager::AuthManager;

/// Run a tool with arguments - automatically handles session continuation for internal commands
pub async fn run_tool(display_name: &str, args: &[String]) -> Result<()> {
    let start_time = std::time::Instant::now();

    // Run the tool normally first
    let result = run_tool_once(display_name, args).await;
    let execution_time = start_time.elapsed();

    match result {
        Ok(()) => {
            // Tool completed successfully - check if this looks like an internal command
            // that should continue the session rather than exit to menu
            if should_continue_session(display_name, args, execution_time) {
                println!("Internal command completed - continuing {display_name} session...");
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

                // Restart the tool without arguments to continue the interactive session
                run_tool_once(display_name, &[]).await
            } else {
                // Normal completion
                Ok(())
            }
        }
        Err(e) => {
            // Just propagate errors normally - let Terminal Jarvis handle the post-tool flow
            Err(e)
        }
    }
}

/// Check if a tool should continue its session after completing
fn should_continue_session(
    _display_name: &str,
    args: &[String],
    _execution_time: std::time::Duration,
) -> bool {
    // First check: if this is explicitly an exit command, never continue
    let is_exit_command = args.iter().any(|arg| {
        matches!(
            arg.as_str(),
            "/exit" | "/quit" | "/bye" | "--exit" | "--quit" | "exit" | "quit" | "bye"
        )
    });

    if is_exit_command {
        return false; // Exit commands should never continue sessions
    }

    // ONLY continue sessions for explicit authentication/setup commands
    // This prevents false positives from user exits, normal completions, etc.
    let explicit_auth_setup_args = args.iter().any(|arg| {
        // Very specific commands that indicate setup/auth workflows
        matches!(
            arg.as_str(),
            "/auth"
                | "/login"
                | "--auth"
                | "--login"
                | "/setup"
                | "--setup"
                | "/config"
                | "--config"
        ) || arg.contains("authenticate")
            || arg.contains("oauth")
    });

    // Only continue if it's an explicit auth/setup command
    // Remove the "quick completion + problematic tool" logic as it causes false positives
    explicit_auth_setup_args
}

/// Run a tool with arguments (single execution without continuation logic)
pub async fn run_tool_once(display_name: &str, args: &[String]) -> Result<()> {
    let cli_command = get_cli_command(display_name);

    if !check_tool_installed(cli_command) {
        return Err(anyhow::anyhow!(
            "Tool '{}' is not installed. Use 'terminal-jarvis install {}' to install it.",
            display_name,
            display_name
        ));
    }

    // Prepare authentication-safe environment and warn about browser opening
    AuthManager::prepare_auth_safe_environment()?;
    AuthManager::warn_if_browser_likely(display_name)?;

    // Provide T.JARVIS-themed guidance before tool startup
    show_tool_startup_guidance(display_name)?;

    // Special terminal preparation for opencode to ensure proper input focus
    if display_name == "opencode" {
        prepare_opencode_terminal_state()?;
    } else {
        // Clear any remaining progress indicators and ensure clean terminal state for other tools
        use std::io::Write;
        print!("\x1b[2K\r"); // Clear current line
        print!("\x1b[?25h"); // Show cursor
        std::io::stdout().flush().unwrap_or_default();
    }

    let mut cmd = Command::new(cli_command);

    // Special handling for opencode which has different command structure
    if display_name == "opencode" {
        if args.is_empty() {
            // No arguments - start pure TUI mode without analyzing any directory
            // This allows opencode to start in interactive mode without token limits
            // Users can then specify what they want to work on interactively
        } else if args.len() == 1 && (args[0] == "." || std::path::Path::new(&args[0]).is_dir()) {
            // Single directory argument - pass it directly for project analysis
            cmd.args(args);
        } else {
            // Multiple arguments or non-directory arguments - use 'run' subcommand
            cmd.arg("run");
            cmd.args(args);
        }
    } else if display_name == "codex" {
        if args.is_empty() {
            // No arguments - start interactive TUI mode
            // This allows users to interact with codex directly
        } else if args.len() == 1 && !args[0].starts_with("--") {
            // Single prompt argument - pass directly for interactive mode with initial prompt
            cmd.args(args);
        } else {
            // Multiple arguments or flags - pass them directly
            // Codex CLI handles various combinations of arguments and flags
            cmd.args(args);
        }
    } else if display_name == "aider" {
        // For aider, pass arguments directly without modification
        // Let aider handle its own terminal control and input modes
        cmd.args(args);
    } else if display_name == "llxprt" {
        // For llxprt, when no arguments are provided, it opens the interactive TUI
        // This is expected behavior and should work seamlessly
        cmd.args(args);
    } else {
        // For other tools, pass arguments directly
        cmd.args(args);
    }

    // For interactive tools, we MUST inherit all stdio streams
    // This is critical for tools like claude-code that use Ink/React components
    cmd.stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    // Special handling for opencode to prevent panic on exit
    let status = if display_name == "opencode" {
        run_opencode_with_clean_exit(cmd)?
    } else {
        // Use direct status() for tools to ensure proper signal handling
        // This allows Ctrl+C and other signals to work properly and exit gracefully
        cmd.status()
            .map_err(|e| anyhow::anyhow!("Failed to execute {}: {}", cli_command, e))?
    };

    // Restore environment after tool execution
    AuthManager::restore_environment()?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Tool '{}' exited with error code: {:?}",
            display_name,
            status.code()
        ));
    }

    Ok(())
}
