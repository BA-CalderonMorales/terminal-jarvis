// Tools Execution Engine Domain
// Handles tool execution, session continuation, and argument processing

use anyhow::Result;
use std::process::Command;
use std::io::Write;
use std::io::IsTerminal;
use std::collections::HashMap;

use super::tools_command_mapping::get_cli_command;
use super::tools_detection::check_tool_installed;
use super::tools_process_management::{
    prepare_opencode_terminal_state, run_opencode_with_clean_exit, run_tool_intercepting_sigint,
};
use super::tools_startup_guidance::show_tool_startup_guidance;
use crate::auth_manager::AuthManager;
use inquire::{Select, Text};

// Heuristic validators for API keys we handle
fn looks_like_gemini_api_key(key: &str) -> bool {
    // Gemini/Google API keys typically start with "AIza" (Google) or sometimes "AI" and are long
    let k = key.trim();
    (k.starts_with("AIza") || k.starts_with("AI"))
        && k.len() >= 25
        && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
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

    // Export any saved credentials for this session so tools don't re-prompt
    let _ = AuthManager::export_saved_env_vars();

    // Prepare authentication-safe environment and warn about browser opening
    // Special-case: Goose + gemini provider can misbehave when DISPLAY/GUI vars are altered.
    // Skip environment mutations for Goose to let provider tools run with the host env.
    if display_name != "goose" {
        AuthManager::prepare_auth_safe_environment()?;
    }
    AuthManager::warn_if_browser_likely(display_name)?;

    // Provide T.JARVIS-themed guidance before tool startup
    show_tool_startup_guidance(display_name)?;

    // Pause for confirmation before launching the external CLI tool (interactive only)
    pause_for_enter_if_interactive();

    // Special terminal preparation for opencode to ensure proper input focus
    if display_name == "opencode" {
        prepare_opencode_terminal_state()?;
    } else {
        // Clear any remaining progress indicators and ensure clean terminal state for other tools
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
        let is_headless =
            std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err();
        let is_codespaces = std::env::var("CODESPACES")
            .map(|v| v == "true")
            .unwrap_or(false)
            || std::env::var("GITHUB_CODESPACES").is_ok()
            || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
        let should_disable_fancy = is_headless || is_codespaces;
        if should_disable_fancy
            && !args
                .iter()
                .any(|arg| arg.contains("help") || arg.contains("version"))
        {
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
        if is_codespaces
            && std::env::var("OPENROUTER_API_KEY").is_err()
            && std::env::var("OPENAI_API_KEY").is_err()
            && std::env::var("ANTHROPIC_API_KEY").is_err()
        {
            // Try to hydrate from saved credentials to avoid prompting
            if let Ok(saved) = AuthManager::get_tool_credentials("aider") {
                for (k, v) in saved {
                    if std::env::var(&k).is_err() {
                        cmd.env(&k, &v);
                    }
                }
            }

            // Still no keys? Offer a lightweight one-time prompt (then persist)
            let no_provider_keys = std::env::var("OPENROUTER_API_KEY").is_err()
                && std::env::var("OPENAI_API_KEY").is_err()
                && std::env::var("ANTHROPIC_API_KEY").is_err();
            if no_provider_keys {
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
                    // Heuristic: choose which env var to set
                    // - sk-or-*: OpenRouter
                    // - sk-ant* or contains "anthropic": Anthropic
                    // - otherwise: OpenAI
                    let mut map = HashMap::new();
                    if trimmed.starts_with("sk-or-") {
                        cmd.env("OPENROUTER_API_KEY", &trimmed);
                        map.insert("OPENROUTER_API_KEY".to_string(), trimmed);
                    } else if trimmed.starts_with("sk-ant") || trimmed.to_lowercase().contains("anthropic") {
                        cmd.env("ANTHROPIC_API_KEY", &trimmed);
                        map.insert("ANTHROPIC_API_KEY".to_string(), trimmed);
                    } else {
                        cmd.env("OPENAI_API_KEY", &trimmed);
                        map.insert("OPENAI_API_KEY".to_string(), trimmed);
                    }
                    let _ = AuthManager::save_tool_credentials("aider", &map);
                }
            }
            }
        }
        cmd.args(args);
    } else if display_name == "goose" {
        // Goose typically uses 'goose configure' for provider setup. In Codespaces, prefer API keys.
        // Keep interactive behavior; just pass args directly.
        let is_codespaces = std::env::var("CODESPACES")
            .map(|v| v == "true")
            .unwrap_or(false)
            || std::env::var("GITHUB_CODESPACES").is_ok()
            || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
        // Prepare a minimal, whitelisted environment for Goose child
        let mut current_env: Vec<(String, String)> = std::env::vars().collect();
        let whitelist = [
            "PATH",
            "HOME",
            "USER",
            "SHELL",
            "TERM",
            "LANG",
            "LC_ALL",
            "LC_CTYPE",
            "COLUMNS",
            "LINES",
            "PWD",
            "XDG_RUNTIME_DIR",
            "XDG_CACHE_HOME",
            "XDG_CONFIG_HOME",
            // Networking/proxy and certs
            "HTTP_PROXY",
            "HTTPS_PROXY",
            "NO_PROXY",
            "http_proxy",
            "https_proxy",
            "no_proxy",
            "SSL_CERT_FILE",
            "SSL_CERT_DIR",
            "CURL_CA_BUNDLE",
            // Provider and keys
            "GOOGLE_API_KEY",
            "GEMINI_API_KEY",
            "OPENAI_API_KEY",
            "ANTHROPIC_API_KEY",
            // Goose hints if present
            "GOOSE_PROVIDER",
            "GOOSE_MODEL",
        ];
        let whitelist_set: std::collections::HashSet<&str> = whitelist.iter().copied().collect();
        cmd.env_clear();
        for (k, v) in current_env.drain(..) {
            if whitelist_set.contains(k.as_str()) {
                cmd.env(&k, &v);
            }
        }
        // Hydrate from saved creds (may add keys not in whitelist by design)
        if let Ok(saved) = AuthManager::get_tool_credentials("goose") {
            for (k, v) in saved {
                cmd.env(&k, &v);
            }
        }

        // Also hydrate Gemini saved credentials to support Goose's gemini provider seamlessly
        if let Ok(saved_gemini) = AuthManager::get_tool_credentials("gemini") {
            for (k, v) in saved_gemini {
                if k == "GOOGLE_API_KEY" || k == "GEMINI_API_KEY" {
                    cmd.env(&k, &v);
                }
            }
        }

        // Bridge Gemini env vars: some stacks expect GOOGLE_API_KEY, others GEMINI_API_KEY
        // If exactly one is set in the parent env, mirror it into the other for the child
        let google_key = std::env::var("GOOGLE_API_KEY").ok();
        let gemini_key = std::env::var("GEMINI_API_KEY").ok();
        match (google_key.as_deref(), gemini_key.as_deref()) {
            (Some(g), None) => {
                cmd.env("GEMINI_API_KEY", g);
            }
            (None, Some(gm)) => {
                cmd.env("GOOGLE_API_KEY", gm);
            }
            _ => {}
        }

        // Ensure browser/no-GUI overrides are not present in the child env
        for var in [
            "HEADLESS",
            "DISABLE_GUI",
            "INTERACTIVE",
            "FORCE_INTERACTIVE",
            "NO_BROWSER",
            "GEMINI_NO_BROWSER",
            "OAUTH_NO_BROWSER",
            "GOOGLE_APPLICATION_CREDENTIALS_NO_BROWSER",
            "BROWSER",
        ] {
            cmd.env_remove(var);
        }

        // Preflight validation: If a Gemini key is present but clearly invalid (OAuth token or wrong shape), prompt to fix
        let candidate_key = std::env::var("GOOGLE_API_KEY")
            .ok()
            .or_else(|| std::env::var("GEMINI_API_KEY").ok())
            .or_else(|| {
                // Fall back to saved creds if not in env
                AuthManager::get_tool_credentials("goose")
                    .ok()
                    .and_then(|m| m.get("GOOGLE_API_KEY").cloned().or_else(|| m.get("GEMINI_API_KEY").cloned()))
                    .or_else(|| {
                        AuthManager::get_tool_credentials("gemini")
                            .ok()
                            .and_then(|m| m.get("GOOGLE_API_KEY").cloned().or_else(|| m.get("GEMINI_API_KEY").cloned()))
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
                if std::io::stdin().is_terminal() {
                    if let Ok(input) = Text::new("Enter a valid GOOGLE_API_KEY (leave blank to cancel):")
                        .with_placeholder("AIza... or AI...")
                        .prompt()
                    {
                        let new_key = input.trim().to_string();
                        if !new_key.is_empty() {
                            if looks_like_gemini_api_key(&new_key) {
                                cmd.env("GOOGLE_API_KEY", &new_key);
                                cmd.env("GEMINI_API_KEY", &new_key);
                                let mut map = HashMap::new();
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

        let has_any_key = std::env::var("OPENAI_API_KEY").is_ok()
            || std::env::var("ANTHROPIC_API_KEY").is_ok()
            || std::env::var("GEMINI_API_KEY").is_ok()
            || std::env::var("GOOGLE_API_KEY").is_ok();
        if is_codespaces && !has_any_key {
            println!(
                "{}",
                crate::theme::theme_global_config::current_theme().accent(
                    "Tip: Set OPENAI_API_KEY, ANTHROPIC_API_KEY, or GEMINI_API_KEY for Goose."
                )
            );
            // Inline prompt (optional): pick provider and capture key for this session
            let providers = vec!["OpenAI", "Anthropic", "Gemini", "Skip"];
            if let Ok(choice) = Select::new(
                "Select a provider to set an API key (or Skip):",
                providers.clone(),
            )
            .prompt()
            {
                match choice {
                    "OpenAI" => {
                        if let Ok(key) = Text::new("Enter OPENAI_API_KEY (leave blank to skip):")
                            .with_placeholder("skips if empty")
                            .prompt()
                        {
                            let trimmed = key.trim();
                            if !trimmed.is_empty() {
                                cmd.env("OPENAI_API_KEY", trimmed);
                                let mut map = HashMap::new();
                                map.insert("OPENAI_API_KEY".to_string(), trimmed.to_string());
                                let _ = AuthManager::save_tool_credentials("goose", &map);
                            }
                        }
                    }
                    "Anthropic" => {
                        if let Ok(key) = Text::new("Enter ANTHROPIC_API_KEY (leave blank to skip):")
                            .with_placeholder("skips if empty")
                            .prompt()
                        {
                            let trimmed = key.trim();
                            if !trimmed.is_empty() {
                                cmd.env("ANTHROPIC_API_KEY", trimmed);
                                let mut map = HashMap::new();
                                map.insert("ANTHROPIC_API_KEY".to_string(), trimmed.to_string());
                                let _ = AuthManager::save_tool_credentials("goose", &map);
                            }
                        }
                    }
                    "Gemini" => {
                        if let Ok(key) = Text::new("Enter GOOGLE_API_KEY (leave blank to skip):")
                            .with_placeholder("skips if empty")
                            .prompt()
                        {
                            let trimmed = key.trim();
                            if !trimmed.is_empty() {
                                // Goose's gemini-cli provider expects GOOGLE_API_KEY; also set GEMINI_API_KEY for compatibility
                                cmd.env("GOOGLE_API_KEY", trimmed);
                                cmd.env("GEMINI_API_KEY", trimmed);
                                let mut map = HashMap::new();
                                map.insert("GOOGLE_API_KEY".to_string(), trimmed.to_string());
                                map.insert("GEMINI_API_KEY".to_string(), trimmed.to_string());
                                let _ = AuthManager::save_tool_credentials("goose", &map);
                            }
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
        let is_codespaces = std::env::var("CODESPACES")
            .map(|v| v == "true")
            .unwrap_or(false)
            || std::env::var("GITHUB_CODESPACES").is_ok()
            || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
        let headless =
            std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err();

        // Hydrate from saved creds to avoid prompt where possible
        if let Ok(saved) = AuthManager::get_tool_credentials("qwen") {
            for (k, v) in saved {
                if std::env::var(&k).is_err() {
                    cmd.env(&k, &v);
                }
            }
        }

        if (is_codespaces || headless) && {
            let no_keys = std::env::var("OPENAI_API_KEY").is_err()
                && std::env::var("ANTHROPIC_API_KEY").is_err()
                && std::env::var("GEMINI_API_KEY").is_err()
                && std::env::var("GOOGLE_API_KEY").is_err();
            no_keys
        } {
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
                    let mut map = HashMap::new();
                    map.insert("OPENAI_API_KEY".to_string(), trimmed.to_string());
                    let _ = AuthManager::save_tool_credentials("qwen", &map);
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
        // Use direct status() to avoid interfering with Goose's own signal handling/provider subprocesses
        cmd.status()
            .map_err(|e| anyhow::anyhow!("Failed to execute {}: {}", cli_command, e))?
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
                    crate::theme::theme_global_config::current_theme()
                        .primary("Goose requires a provider. Launching 'goose configure'...\n",)
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

/// Prompt the user to press Enter before launching the tool, only if running in an interactive TTY.
fn pause_for_enter_if_interactive() {
    if std::io::stdin().is_terminal() {
        let theme = crate::theme::theme_global_config::current_theme();
        println!("\n{}", theme.accent("Press Enter to continue..."));
        let _ = std::io::stdin().read_line(&mut String::new());
        let _ = std::io::stdout().flush();
    }
}
