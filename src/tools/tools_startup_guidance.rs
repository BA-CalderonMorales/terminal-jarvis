// Tools Startup Guidance Domain
// Handles tool-specific startup messages and user guidance

use anyhow::Result;

/// Show T.JARVIS-themed startup guidance for tools
pub fn show_tool_startup_guidance(display_name: &str) -> Result<()> {
    use crate::theme::theme_global_config;

    let theme = theme_global_config::current_theme();
    // Basic environment detection to tailor guidance
    let is_codespaces = std::env::var("CODESPACES")
        .map(|v| v == "true")
        .unwrap_or(false)
        || std::env::var("GITHUB_CODESPACES").is_ok()
        || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
    let is_devcontainer = std::path::Path::new("/.dockerenv").exists()
        || std::env::var("DEVCONTAINER").is_ok()
        || std::env::var("REMOTE_CONTAINERS").is_ok()
        || std::env::var("VSCODE_IPC_HOOK_CLI").is_ok();

    match display_name {
        "claude" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS AUTHENTICATION ADVISORY ─────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ Claude may require API key authentication on first use.    │")
            );
            println!(
                "{}",
                theme.primary("│ This is normal and expected for secure API access.         │")
            );
            println!(
                "{}",
                theme.accent("│ • Set: export ANTHROPIC_API_KEY=\"your-api-key\"             │")
            );
            println!(
                "{}",
                theme.accent("│ • Get your API key: https://console.anthropic.com/         │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
        "gemini" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS AUTHENTICATION ADVISORY ─────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ Gemini may require Google Cloud authentication setup.      │")
            );
            println!(
                "{}",
                theme.primary("│ This is normal for secure API access to Gemini models.     │")
            );
            println!(
                "{}",
                theme.accent("│ • Follow the authentication prompts if they appear         │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
        "qwen" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS SETUP ADVISORY ──────────────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ Qwen may require initial configuration on first use.       │")
            );
            println!(
                "{}",
                theme.primary("│ Follow any setup prompts that appear.                      │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
        "opencode" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS STARTUP ADVISORY ────────────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ OpenCode is initializing the interactive environment.      │")
            );
            println!(
                "{}",
                theme.primary("│ The input interface will be available momentarily.         │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
        "llxprt" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS STARTUP ADVISORY ────────────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ LLxprt Code is preparing the interactive interface.        │")
            );
            println!(
                "{}",
                theme.primary("│ Advanced code analysis capabilities will be available.     │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
        "codex" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS AUTHENTICATION ADVISORY ─────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ OpenAI Codex may require API key authentication.           │")
            );
            println!(
                "{}",
                theme.primary("│ This enables access to advanced code generation models.    │")
            );
            println!(
                "{}",
                theme.accent("│ • Set: export OPENAI_API_KEY=\"your-api-key\"                │")
            );
            println!(
                "{}",
                theme.accent("│ • Get your API key: https://platform.openai.com/           │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
        "crush" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS STARTUP ADVISORY ────────────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ Crush is initializing the development environment.         │")
            );
            println!(
                "{}",
                theme.primary("│ Advanced coding assistance will be available momentarily.  │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
        "aider" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS AUTHENTICATION ADVISORY ─────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ Aider may require API key setup for AI model access.       │")
            );
            println!(
                "{}",
                theme.primary("│ Follow the configuration prompts to get started.           │")
            );
            println!(
                "{}",
                theme.accent("│ • Supports OpenAI, Anthropic, OpenRouter, and more         │")
            );
            println!(
                "{}",
                theme.accent("│ • Documentation: https://aider.chat/docs/                  │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();

            // Additional OAuth guidance for containerized environments
            if is_codespaces {
                // In Codespaces, localhost callbacks in the local browser cannot reach the remote container
                println!(
                    "{}",
                    theme.secondary(
                        "┌─ T.JARVIS OAUTH LIMITATION (CODESPACES) ───────────────────┐"
                    )
                );
                println!(
                    "{}",
                    theme.primary(
                        "│ OAuth with localhost callback is not supported in Codespaces. │"
                    )
                );
                println!(
                    "{}",
                    theme.primary(
                        "│ Use API key authentication instead for OpenRouter/OpenAI/etc. │"
                    )
                );
                println!(
                    "{}",
                    theme.accent("│ • Example: export OPENROUTER_API_KEY=your_api_key           │")
                );
                println!(
                    "{}",
                    theme.accent(
                        "│ • Get an OpenRouter API key: https://openrouter.ai/settings/keys │"
                    )
                );
                println!(
                    "{}",
                    theme.accent("│ • Then run: aider --model openrouter/anthropic/claude-3.5   │")
                );
                println!(
                    "{}",
                    theme.accent(
                        "│   See: https://aider.chat/docs/troubleshooting/models-and-keys.html │"
                    )
                );
                println!(
                    "{}",
                    theme.secondary(
                        "└────────────────────────────────────────────────────────────┘"
                    )
                );
                println!();
            } else if is_devcontainer {
                // In local devcontainers, forwarding the callback port enables OAuth
                println!(
                    "{}",
                    theme.secondary(
                        "┌─ T.JARVIS OAUTH TIP (DEV CONTAINER) ───────────────────────┐"
                    )
                );
                println!(
                    "{}",
                    theme.primary("│ Forward port 8484 to your host in the VS Code Ports panel. │")
                );
                println!(
                    "{}",
                    theme.primary("│ Then open the printed URL and complete the login.          │")
                );
                println!(
                    "{}",
                    theme.accent("│ • Port to forward: 8484 (HTTP)                              │")
                );
                println!(
                    "{}",
                    theme.accent("│ • If OAuth still fails, set an API key as a fallback.       │")
                );
                println!(
                    "{}",
                    theme.secondary(
                        "└────────────────────────────────────────────────────────────┘"
                    )
                );
                println!();
            }
        }
        "amp" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS STARTUP ADVISORY ────────────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ Amp is initializing the AI-powered development interface.  │")
            );
            println!(
                "{}",
                theme.primary("│ Advanced code assistance will be available momentarily.    │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
        "goose" => {
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS AUTHENTICATION ADVISORY ─────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary("│ Goose may require provider/API key configuration.           │")
            );
            println!(
                "{}",
                theme.primary("│ Run 'goose configure' to select a provider and set keys.    │")
            );
            println!(
                "{}",
                theme.accent("│ • Providers: OpenAI, Anthropic, Gemini (and more)          │")
            );
            println!(
                "{}",
                theme.accent("│ • Docs: https://block.github.io/goose/docs/                │")
            );
            println!(
                "{}",
                theme.accent("│ • Provider setup: https://block.github.io/goose/docs/getting-started/providers │")
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
            // Codespaces advisory similar to Aider, recommending API keys over OAuth/browser flows
            let is_codespaces = std::env::var("CODESPACES")
                .map(|v| v == "true")
                .unwrap_or(false)
                || std::env::var("GITHUB_CODESPACES").is_ok()
                || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
            if is_codespaces {
                println!(
                    "{}",
                    theme.secondary(
                        "┌─ T.JARVIS OAUTH LIMITATION (CODESPACES) ───────────────────┐"
                    )
                );
                println!(
                    "{}",
                    theme.primary("│ Browser-based OAuth may not complete in Codespaces.        │")
                );
                println!(
                    "{}",
                    theme
                        .primary("│ Use direct API keys with 'goose configure' or env vars.     │")
                );
                println!(
                    "{}",
                    theme.accent("│ • OPENAI_API_KEY, ANTHROPIC_API_KEY, GEMINI_API_KEY        │")
                );
                println!(
                    "{}",
                    theme.accent("│ • Provider setup: https://block.github.io/goose/docs/getting-started/providers │")
                );
                println!(
                    "{}",
                    theme.secondary(
                        "└────────────────────────────────────────────────────────────┘"
                    )
                );
                println!();
            }
        }
        _ => {
            // For any future tools, show generic guidance
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS STARTUP ADVISORY ────────────────────────────────┐")
            );
            println!(
                "{}",
                theme.primary(&format!(
                    "│ Launching {} - Follow any setup prompts if needed.        │",
                    display_name
                ))
            );
            println!(
                "{}",
                theme.secondary("└────────────────────────────────────────────────────────────┘")
            );
            println!();
        }
    }

    Ok(())
}
