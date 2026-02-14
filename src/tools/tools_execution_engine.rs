// Tools Execution Engine Domain
// Handles tool execution, session continuation, and argument processing
//
// REFACTORED: Inline per-tool auth prompting replaced with unified AuthPreflight.
// Tool-specific environment setup extracted to tools_environment.rs.

use anyhow::Result;
use std::io::Write;
use std::process::Command;

use super::tools_command_mapping::get_cli_command;
use super::tools_detection::check_tool_installed;
use super::tools_environment::{apply_aider_headless_args, apply_tool_environment};
use super::tools_process_management::{
    prepare_opencode_terminal_state, run_opencode_with_clean_exit, run_tool_intercepting_sigint,
};
use super::tools_startup_guidance::show_tool_startup_guidance;
use crate::auth_manager::auth_preflight::AuthPreflight;
use crate::auth_manager::AuthManager;

// Heuristic validators for Gemini API keys (used for goose preflight validation)
fn looks_like_gemini_api_key(key: &str) -> bool {
    let k = key.trim();
    (k.starts_with("AIza") || k.starts_with("AI"))
        && k.len() >= 25
        && k.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn looks_like_oauth_token(token: &str) -> bool {
    let t = token.trim();
    t.starts_with("4/") || t.starts_with("ya29.")
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

    if !check_tool_installed(cli_command) {
        return Err(anyhow::anyhow!(
            "Tool '{display_name}' is not installed. Use 'terminal-jarvis install {display_name}' to install it."
        ));
    }

    // --- UNIFIED AUTH: Check and prompt if needed ---
    let auth_result = AuthPreflight::check(display_name);
    if !auth_result.is_ready && auth_result.auth_mode != "none" {
        // Try to prompt for missing credentials (unified flow for all tools)
        let _ = AuthPreflight::prompt_for_missing(display_name, &auth_result);
    }

    // Export any saved credentials for this session so tools don't re-prompt
    let _ = AuthManager::export_saved_env_vars();

    // Prepare authentication-safe environment and warn about browser opening
    // Skip environment mutations for Goose to let provider tools run with the host env.
    if display_name != "goose" {
        AuthManager::prepare_auth_safe_environment()?;
    }
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

    let mut cmd = Command::new(cli_command);

    // --- INJECT SAVED CREDENTIALS ---
    AuthPreflight::inject_credentials(&mut cmd, display_name)?;

    // --- APPLY TOOL-SPECIFIC ENVIRONMENT ---
    apply_tool_environment(&mut cmd, display_name, args)?;

    // --- GOOSE: Gemini key validation and credential hydration ---
    if display_name == "goose" {
        hydrate_goose_credentials(&mut cmd)?;
        validate_goose_gemini_key(&mut cmd)?;
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

    // Restore environment after tool execution
    AuthManager::restore_environment()?;

    // --- EXIT CODE HANDLING ---
    if !status.success() {
        return handle_tool_failure(display_name, cli_command, args, status);
    }

    Ok(())
}

/// Apply tool-specific argument modifications to the Command.
fn apply_tool_args(cmd: &mut Command, display_name: &str, args: &[String]) {
    match display_name {
        "opencode" => {
            if args.is_empty() {
                // No arguments - start pure TUI mode
            } else if args.len() == 1 && (args[0] == "." || std::path::Path::new(&args[0]).is_dir())
            {
                cmd.args(args);
            } else {
                cmd.arg("run");
                cmd.args(args);
            }
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

/// Hydrate Goose credentials from saved store (goose + gemini tools).
fn hydrate_goose_credentials(cmd: &mut Command) -> Result<()> {
    if let Ok(saved) = AuthManager::get_tool_credentials("goose") {
        for (k, v) in saved {
            cmd.env(&k, &v);
        }
    }
    // Also hydrate Gemini saved credentials for Goose's gemini provider
    if let Ok(saved_gemini) = AuthManager::get_tool_credentials("gemini") {
        for (k, v) in saved_gemini {
            if k == "GOOGLE_API_KEY" || k == "GEMINI_API_KEY" {
                cmd.env(&k, &v);
            }
        }
    }
    Ok(())
}

/// Validate Goose's Gemini key if present -- reject OAuth tokens.
fn validate_goose_gemini_key(cmd: &mut Command) -> Result<()> {
    let candidate_key = std::env::var("GOOGLE_API_KEY")
        .ok()
        .or_else(|| std::env::var("GEMINI_API_KEY").ok())
        .or_else(|| {
            AuthManager::get_tool_credentials("goose")
                .ok()
                .and_then(|m| {
                    m.get("GOOGLE_API_KEY")
                        .cloned()
                        .or_else(|| m.get("GEMINI_API_KEY").cloned())
                })
                .or_else(|| {
                    AuthManager::get_tool_credentials("gemini")
                        .ok()
                        .and_then(|m| {
                            m.get("GOOGLE_API_KEY")
                                .cloned()
                                .or_else(|| m.get("GEMINI_API_KEY").cloned())
                        })
                })
        });

    if let Some(k) = candidate_key {
        if looks_like_oauth_token(&k) || !looks_like_gemini_api_key(&k) {
            let theme = crate::theme::theme_global_config::current_theme();
            println!(
                "{}",
                theme.primary("Gemini provider requires a valid API key, not an OAuth token.")
            );
            println!(
                "{}",
                theme.secondary(
                    "Get a key from Google AI Studio and set GOOGLE_API_KEY (or GEMINI_API_KEY).\nDocs: https://ai.google.dev/gemini-api/docs/api-key"
                )
            );

            if std::io::IsTerminal::is_terminal(&std::io::stdin()) {
                if let Ok(input) =
                    inquire::Password::new("Enter a valid GOOGLE_API_KEY (leave blank to cancel):")
                        .without_confirmation()
                        .prompt()
                {
                    let new_key = input.trim().to_string();
                    if !new_key.is_empty() {
                        if looks_like_gemini_api_key(&new_key) {
                            cmd.env("GOOGLE_API_KEY", &new_key);
                            cmd.env("GEMINI_API_KEY", &new_key);
                            let mut map = std::collections::HashMap::new();
                            map.insert("GOOGLE_API_KEY".to_string(), new_key.clone());
                            map.insert("GEMINI_API_KEY".to_string(), new_key);
                            let _ = AuthManager::save_tool_credentials("gemini", &map);
                            let _ = AuthManager::save_tool_credentials("goose", &map);
                        } else {
                            return Err(anyhow::anyhow!(
                                "The provided key does not look like a valid Gemini API key."
                            ));
                        }
                    } else {
                        return Err(anyhow::anyhow!(
                            "Invalid Gemini credentials detected. Update your GOOGLE_API_KEY and try again."
                        ));
                    }
                }
            } else {
                return Err(anyhow::anyhow!(
                    "Invalid Gemini credentials detected. Update your GOOGLE_API_KEY and try again."
                ));
            }
        }
    }

    Ok(())
}

/// Handle tool exit with non-zero status code.
fn handle_tool_failure(
    display_name: &str,
    cli_command: &str,
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
        let mut configure_cmd = Command::new(cli_command);
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
