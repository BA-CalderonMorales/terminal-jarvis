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
                theme.primary("=== Authentication Advisory: Claude ===")
            );
            println!();
            println!(
                "{}",
                theme.secondary("- Requires ANTHROPIC_API_KEY for API access.")
            );
            println!(
                "{}",
                theme.accent("- Set: export ANTHROPIC_API_KEY=your_api_key")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
        "gemini" => {
            println!(
                "{}",
                theme.primary("=== Authentication Advisory: Gemini ===")
            );
            println!();
            println!("{}", theme.secondary("- Requires Google API credentials."));
            println!(
                "{}",
                theme.accent("- Set: export GOOGLE_API_KEY=your_api_key")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
        "qwen" => {
            println!("{}", theme.primary("=== Setup Advisory: Qwen ==="));
            println!();
            println!(
                "{}",
                theme.secondary("- May prompt for initial configuration.")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
        "opencode" => {
            println!("{}", theme.primary("=== Startup Advisory: OpenCode ==="));
            println!();
            println!(
                "{}",
                theme.secondary("- Starting interactive environment...")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
        "llxprt" => {
            println!("{}", theme.primary("=== Startup Advisory: LLxprt ==="));
            println!();
            println!(
                "{}",
                theme.secondary("- Preparing interactive interface...")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
        "codex" => {
            println!(
                "{}",
                theme.primary("=== Authentication Advisory: Codex ===")
            );
            println!();
            println!(
                "{}",
                theme.secondary("- Requires OPENAI_API_KEY for model access.")
            );
            println!(
                "{}",
                theme.accent("- Set: export OPENAI_API_KEY=your_api_key")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
        "crush" => {
            println!("{}", theme.primary("=== Startup Advisory: Crush ==="));
            println!();
            println!(
                "{}",
                theme.secondary("- Initializing development environment...")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
        "aider" => {
            println!(
                "{}",
                theme.primary("=== Authentication Advisory: Aider ===")
            );
            println!();
            println!("{}", theme.secondary("- Use an API key for model access."));
            println!(
                "{}",
                theme.accent("- OPENROUTER_API_KEY (recommended) or OPENAI/ANTHROPIC")
            );
            println!("{}", theme.accent("- Docs: https://aider.chat/docs/"));
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
            // Minimal OAuth section only if relevant
            if is_codespaces {
                println!("{}", theme.primary("=== OAuth Limitation (Codespaces) ==="));
                println!();
                println!(
                    "{}",
                    theme.secondary("- OAuth callbacks may not complete in Codespaces.")
                );
                println!(
                    "{}",
                    theme.accent(
                        "- Prefer API keys. Example: export OPENROUTER_API_KEY=your_api_key"
                    )
                );
                println!();
                println!("{}", theme.primary("==================================="));
                println!();
            } else if is_devcontainer {
                println!("{}", theme.primary("=== OAuth Tip (Dev Container) ==="));
                println!();
                println!(
                    "{}",
                    theme.secondary("- Forward the callback port (e.g., 8484) if OAuth is used.")
                );
                println!();
                println!("{}", theme.primary("==================================="));
                println!();
            }
        }
        "amp" => {
            println!("{}", theme.primary("=== Startup Advisory: Amp ==="));
            println!();
            println!("{}", theme.secondary("- Initializing interface..."));
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
        "goose" => {
            println!(
                "{}",
                theme.primary("=== Authentication Advisory: Goose ===")
            );
            println!();
            println!(
                "{}",
                theme.secondary("- Requires provider/API key configuration.")
            );
            println!(
                "{}",
                theme.accent("- Run: goose configure (choose provider, set key)")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
            // Codespaces advisory similar to Aider, recommending API keys over OAuth/browser flows
            let is_codespaces = std::env::var("CODESPACES")
                .map(|v| v == "true")
                .unwrap_or(false)
                || std::env::var("GITHUB_CODESPACES").is_ok()
                || std::env::var("GITHUB_CODESPACES_PORT_FORWARDING_DOMAIN").is_ok();
            if is_codespaces {
                println!("{}", theme.primary("=== OAuth Limitation (Codespaces) ==="));
                println!();
                println!(
                    "{}",
                    theme.secondary("- Browser/OAuth callbacks may not complete.")
                );
                println!(
                    "{}",
                    theme.accent("- Prefer API keys. Example: export OPENAI_API_KEY=your_api_key")
                );
                println!();
                println!("{}", theme.primary("==================================="));
                println!();
            }
        }
        _ => {
            // For any future tools, show generic guidance
            println!(
                "{}",
                theme.primary(&format!("=== Startup Advisory: {} ===", display_name))
            );
            println!();
            println!(
                "{}",
                theme.secondary("- Follow any setup prompts if needed.")
            );
            println!();
            println!("{}", theme.primary("==================================="));
            println!();
        }
    }

    Ok(())
}
