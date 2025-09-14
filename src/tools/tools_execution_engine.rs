// Tools Execution Engine Domain
// Handles tool execution, session continuation, and argument processing

use anyhow::Result;
use std::process::Command;

use super::tools_command_mapping::get_cli_command;
use super::tools_detection::check_tool_installed;
use super::tools_process_management::{
    prepare_opencode_terminal_state, run_opencode_with_clean_exit, run_tool_intercepting_sigint,
};
use super::tools_startup_guidance::show_tool_startup_guidance;
use crate::auth_manager::AuthManager;
use inquire::{Select, Text};

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
        // Strategic aider handling - reduce terminal control and ensure Ctrl+C only stops child
        cmd.env("PYTHONUNBUFFERED", "1");
        cmd.env("AIDER_NO_BROWSER", "1"); // prevent auto opening browser; still prints URL
        // Reduce fancy terminal features from prompt_toolkit ONLY in headless/Codespaces
        let is_headless = std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err();
        let is_codespaces = std::env::var("CODESPACES").map(|v| v == "true").unwrap_or(false)
            || std::env::var("GITHUB_CODESPACES").is_ok()
            || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
        let should_disable_fancy = is_headless || is_codespaces;
        if should_disable_fancy && !args.iter().any(|arg| arg.contains("help") || arg.contains("version")) {
            if !args.iter().any(|arg| arg.contains("no-pretty")) {
                cmd.arg("--no-pretty");
            }
            if !args.iter().any(|arg| arg.contains("no-fancy-input")) {
                cmd.arg("--no-fancy-input");
            }
            if !args.iter().any(|arg| arg.contains("no-multiline")) {
                cmd.arg("--no-multiline");
            }
        }

        // If running in Codespaces (or a cloud env) where OAuth callback won't work,
        // and no API key is present, offer to set an API key for this session only.
        let no_provider_keys = std::env::var("OPENROUTER_API_KEY").is_err()
            && std::env::var("OPENAI_API_KEY").is_err()
            && std::env::var("ANTHROPIC_API_KEY").is_err();

        if is_codespaces && no_provider_keys {
            println!(
                "{}",
                crate::theme::theme_global_config::current_theme()
                    .accent("OpenRouter API keys: https://openrouter.ai/settings/keys")
            );
            // Lightweight inline prompt; user can press Enter to skip
            if let Ok(input) = Text::new("Enter an API key for Aider (recommended: OPENROUTER_API_KEY). Leave blank to skip:")
                .with_placeholder("skips if empty")
                .prompt()
            {
                let trimmed = input.trim().to_string();
                if !trimmed.is_empty() {
                    // Prefer OpenRouter key when provided directly
                    cmd.env("OPENROUTER_API_KEY", trimmed);
                }
            }
        }
        cmd.args(args);
    } else if display_name == "goose" {
        // Goose typically uses 'goose configure' for provider setup. In Codespaces, prefer API keys.
        // Keep interactive behavior; just pass args directly.
        let is_codespaces = std::env::var("CODESPACES").map(|v| v == "true").unwrap_or(false)
            || std::env::var("GITHUB_CODESPACES").is_ok()
            || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
        let has_any_key = std::env::var("OPENAI_API_KEY").is_ok()
            || std::env::var("ANTHROPIC_API_KEY").is_ok()
            || std::env::var("GEMINI_API_KEY").is_ok();
        if is_codespaces && !has_any_key {
            println!(
                "{}",
                crate::theme::theme_global_config::current_theme()
                    .accent("Tip: Set OPENAI_API_KEY, ANTHROPIC_API_KEY, or GEMINI_API_KEY for Goose.")
            );
            // Inline prompt (optional): pick provider and capture key for this session
            let providers = vec!["OpenAI", "Anthropic", "Gemini", "Skip"]; 
            if let Ok(choice) = Select::new("Select a provider to set an API key (or Skip):", providers.clone()).prompt() {
                match choice {
                    "OpenAI" => {
                        if let Ok(key) = Text::new("Enter OPENAI_API_KEY (leave blank to skip):").with_placeholder("skips if empty").prompt() {
                            let trimmed = key.trim();
                            if !trimmed.is_empty() { cmd.env("OPENAI_API_KEY", trimmed); }
                        }
                    }
                    "Anthropic" => {
                        if let Ok(key) = Text::new("Enter ANTHROPIC_API_KEY (leave blank to skip):").with_placeholder("skips if empty").prompt() {
                            let trimmed = key.trim();
                            if !trimmed.is_empty() { cmd.env("ANTHROPIC_API_KEY", trimmed); }
                        }
                    }
                    "Gemini" => {
                        if let Ok(key) = Text::new("Enter GEMINI_API_KEY (leave blank to skip):").with_placeholder("skips if empty").prompt() {
                            let trimmed = key.trim();
                            if !trimmed.is_empty() { cmd.env("GEMINI_API_KEY", trimmed); }
                        }
                    }
                    _ => {}
                }
            }
            println!(
                "{}",
                crate::theme::theme_global_config::current_theme().secondary(
                    "Match your model to the provider you configured (e.g., gemini-* => GEMINI_API_KEY, claude-* => ANTHROPIC_API_KEY, gpt-* => OPENAI_API_KEY).",
                )
            );
        }
        cmd.args(args);
    } else if display_name == "qwen" {
        // Reduce auth flicker for Qwen in headless/Codespaces by preferring API key path
        let is_codespaces = std::env::var("CODESPACES").map(|v| v == "true").unwrap_or(false)
            || std::env::var("GITHUB_CODESPACES").is_ok()
            || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
        let headless = std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err();
        let no_keys = std::env::var("OPENAI_API_KEY").is_err()
            && std::env::var("ANTHROPIC_API_KEY").is_err()
            && std::env::var("GEMINI_API_KEY").is_err();

        if (is_codespaces || headless) && no_keys {
            println!(
                "{}",
                crate::theme::theme_global_config::current_theme()
                    .accent("Qwen tip: Set OPENAI_API_KEY to avoid interactive auth flicker.")
            );
            if let Ok(input) = Text::new("Enter OPENAI_API_KEY for Qwen (leave blank to skip):")
                .with_placeholder("skips if empty")
                .prompt()
            {
                let trimmed = input.trim();
                if !trimmed.is_empty() {
                    cmd.env("OPENAI_API_KEY", trimmed);
                }
            }
        }
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

    // Special handling for tools with known issues
    let status = if display_name == "opencode" {
        run_opencode_with_clean_exit(cmd)?
    } else if display_name == "aider" {
        // Run aider while intercepting Ctrl+C so only the child is terminated
        run_tool_intercepting_sigint(cmd)?
    } else if display_name == "goose" {
        // Ensure Ctrl+C during any provider config or prompts does not kill Terminal Jarvis
        run_tool_intercepting_sigint(cmd)?
    } else {
        // Use direct status() for tools to ensure proper signal handling
        // This allows Ctrl+C and other signals to work properly and exit gracefully
        cmd.status()
            .map_err(|e| anyhow::anyhow!("Failed to execute {}: {}", cli_command, e))?
    };

    // Restore environment after tool execution
    AuthManager::restore_environment()?;

    // Strategic exit code handling for tools with known issues
    if !status.success() {
        if display_name == "aider" {
            // For aider (especially uv-installed), treat any non-zero as graceful termination
            let exit_code = status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "signal".to_string());
            println!(
                "\nAider session ended (exit: {}). Returning to Terminal Jarvis...",
                exit_code
            );
            return Ok(());
        } else if display_name == "goose" {
            // If Goose fails with no args, it's commonly due to missing provider configuration.
            // Proactively run `goose configure` to help the user set it up, then return to menu.
            if args.is_empty() {
                println!(
                    "{}",
                    crate::theme::theme_global_config::current_theme().primary(
                        "Goose requires a provider. Launching 'goose configure'...\n",
                    )
                );
                let mut configure_cmd = Command::new(cli_command);
                configure_cmd
                    .arg("configure")
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit());

                // Reuse SIGINT-safe runner so Ctrl+C doesn't kill TJ
                let _ = run_tool_intercepting_sigint(configure_cmd);
                println!("Returning to Terminal Jarvis...");
                return Ok(());
            }
        }

        return Err(anyhow::anyhow!(
            "Tool '{}' exited with error code: {:?}",
            display_name,
            status.code()
        ));
    }

    Ok(())
}
