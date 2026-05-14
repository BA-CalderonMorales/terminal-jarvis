// Tools Execution Engine Domain
// Handles tool execution, session continuation, and argument processing
//
// REFACTORED: Inline per-tool auth prompting replaced with unified AuthPreflight.
// Tool-specific environment setup extracted to tools_environment.rs.

use anyhow::Result;
use std::io::Write;
use std::process::Command;

use super::handlers::tool_registry;
use super::tools_command_mapping::get_cli_command;
use super::tools_detection::resolve_tool_path;
use super::tools_environment::{apply_aider_headless_args, apply_tool_environment};
use super::tools_process_management::{
    prepare_opencode_terminal_state, run_opencode_with_clean_exit, run_tool_intercepting_sigint,
};
use super::tools_startup_guidance::show_tool_startup_guidance;
use crate::auth_manager::auth_preflight::AuthPreflight;
use crate::auth_manager::AuthManager;

struct AuthEnvironmentGuard {
    enabled: bool,
}

impl AuthEnvironmentGuard {
    fn prepare(enabled: bool) -> Result<Self> {
        if enabled {
            AuthManager::prepare_auth_safe_environment()?;
        }
        Ok(Self { enabled })
    }
}

impl Drop for AuthEnvironmentGuard {
    fn drop(&mut self) {
        if self.enabled {
            let _ = AuthManager::restore_environment();
        }
    }
}

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
    let explicit_auth_setup_args = args.iter().any(|arg| {
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

    explicit_auth_setup_args
}

/// Run a tool with arguments (single execution without continuation logic)
pub async fn run_tool_once(display_name: &str, args: &[String]) -> Result<()> {
    let cli_command = get_cli_command(display_name);

    let executable_path = match resolve_tool_path(&cli_command) {
        Some(path) => path,
        None => {
            return Err(anyhow::anyhow!(
                "Tool '{display_name}' is not installed. Use 'terminal-jarvis install {display_name}' to install it."
            ));
        }
    };

    // --- UNIFIED AUTH: Check and prompt if needed ---
    let auth_result = AuthPreflight::check(display_name);
    if !auth_result.is_ready && auth_result.auth_mode != "none" {
        // Try to prompt for missing credentials (unified flow for all tools)
        let _ = AuthPreflight::prompt_for_missing(display_name, &auth_result);
    }

    // Export any saved credentials for this session so tools don't re-prompt
    let _ = AuthManager::export_saved_env_vars();

    let registry = tool_registry();
    let handler = registry.get(display_name);

    // Prepare authentication-safe environment and warn about browser opening.
    // Some provider tools need the host auth environment unchanged.
    let _auth_env_guard =
        AuthEnvironmentGuard::prepare(!handler.is_some_and(|h| h.uses_host_auth_environment()))?;
    AuthManager::warn_if_browser_likely(display_name)?;

    // Provide minimal guidance before tool startup (only shows tips if API keys missing)
    show_tool_startup_guidance(display_name)?;

    // --- TERMINAL PREPARATION ---
    if display_name == "opencode" {
        prepare_opencode_terminal_state()?;
    } else {
        print!("\x1b[2K\r"); // Clear current line
        print!("\x1b[?25h"); // Show cursor
        std::io::stdout().flush().unwrap_or_default();
    }

    let mut cmd = Command::new(&executable_path);

    // --- INJECT SAVED CREDENTIALS ---
    AuthPreflight::inject_credentials(&mut cmd, display_name)?;

    // --- APPLY TOOL-SPECIFIC ENVIRONMENT ---
    apply_tool_environment(&mut cmd, display_name, args)?;

    // --- TOOL HANDLER PRE-EXECUTION ---
    if let Some(handler) = handler {
        handler.pre_execution(&mut cmd)?;
    }

    // --- TOOL-SPECIFIC ARGUMENT HANDLING ---
    apply_tool_args(&mut cmd, display_name, args);

    // --- STDIO: inherit all streams for interactive tools ---
    cmd.stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    // --- EXECUTE ---
    let status = if display_name == "opencode" {
        run_opencode_with_clean_exit(cmd)?
    } else if display_name == "aider" {
        run_tool_intercepting_sigint(cmd)?
    } else {
        cmd.status()
            .map_err(|e| anyhow::anyhow!("Failed to execute {cli_command}: {e}"))?
    };

    // --- EXIT CODE HANDLING ---
    if !status.success() {
        return handle_tool_failure(display_name, &executable_path, args, status);
    }

    Ok(())
}

/// Apply tool-specific argument modifications to the Command.
fn apply_tool_args(cmd: &mut Command, display_name: &str, args: &[String]) {
    match display_name {
        "opencode" => {
            cmd.args(normalize_opencode_args(args));
        }
        "codex" => {
            // All argument combinations pass through directly
            cmd.args(args);
        }
        "aider" => {
            // Apply headless args (--no-pretty, --no-fancy-input, --no-multiline)
            apply_aider_headless_args(cmd, args);
            cmd.args(args);
        }
        _ => {
            cmd.args(args);
        }
    }
}

/// Normalize opencode args:
/// - no args => launch TUI
/// - explicit subcommands/flags/paths => passthrough
/// - free-form input => prefix with "run"
fn normalize_opencode_args(args: &[String]) -> Vec<String> {
    if args.is_empty() {
        return vec![];
    }

    let first = args[0].as_str();

    // Preserve explicit subcommands and common flags
    let passthrough_subcommands = [
        "run", "auth", "config", "models", "mcp", "status", "update", "help", "version",
    ];
    let is_passthrough = first.starts_with('-')
        || passthrough_subcommands
            .iter()
            .any(|subcommand| first.eq_ignore_ascii_case(subcommand));

    if is_passthrough || (args.len() == 1 && (first == "." || std::path::Path::new(first).is_dir()))
    {
        return args.to_vec();
    }

    let mut normalized = Vec::with_capacity(args.len() + 1);
    normalized.push("run".to_string());
    normalized.extend(args.to_vec());
    normalized
}

/// Handle tool exit with non-zero status code.
fn handle_tool_failure(
    display_name: &str,
    executable_path: &str,
    args: &[String],
    status: std::process::ExitStatus,
) -> Result<()> {
    if display_name == "aider" {
        // For aider (especially uv-installed), treat any non-zero as graceful termination
        let exit_code = status
            .code()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "signal".to_string());
        println!("\nAider session ended (exit: {exit_code}). Returning to Terminal Jarvis...");
        return Ok(());
    }

    if display_name == "goose" && args.is_empty() {
        // If Goose fails with no args, run `goose configure` to help user set up provider
        println!(
            "{}",
            crate::theme::theme_global_config::current_theme()
                .primary("Goose requires a provider. Launching 'goose configure'...\n")
        );
        let mut configure_cmd = Command::new(executable_path);
        configure_cmd
            .arg("configure")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());

        let _ = run_tool_intercepting_sigint(configure_cmd);
        println!("Returning to Terminal Jarvis...");
        return Ok(());
    }

    Err(anyhow::anyhow!(
        "Tool '{}' exited with error code: {:?}",
        display_name,
        status.code()
    ))
}

#[cfg(test)]
mod tests {
    use super::normalize_opencode_args;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn opencode_no_args_remains_empty() {
        assert!(normalize_opencode_args(&[]).is_empty());
    }

    #[test]
    fn opencode_explicit_run_subcommand_is_not_prefixed_twice() {
        let args = strings(&["run", "--help"]);
        assert_eq!(normalize_opencode_args(&args), args);
    }

    #[test]
    fn opencode_flags_pass_through() {
        let args = strings(&["--help"]);
        assert_eq!(normalize_opencode_args(&args), args);
    }

    #[test]
    fn opencode_known_subcommand_passes_through() {
        let args = strings(&["status"]);
        assert_eq!(normalize_opencode_args(&args), args);
    }

    #[test]
    fn opencode_directory_path_passes_through() {
        let args = strings(&["."]);
        assert_eq!(normalize_opencode_args(&args), args);
    }

    #[test]
    fn opencode_free_form_input_is_prefixed_with_run() {
        let args = strings(&["write tests for auth flow"]);
        assert_eq!(
            normalize_opencode_args(&args),
            strings(&["run", "write tests for auth flow"])
        );
    }
}
