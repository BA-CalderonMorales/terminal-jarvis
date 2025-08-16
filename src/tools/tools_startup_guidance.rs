// Tools Startup Guidance Domain
// Handles tool-specific startup messages and user guidance

use anyhow::Result;

/// Show T.JARVIS-themed startup guidance for tools
pub fn show_tool_startup_guidance(display_name: &str) -> Result<()> {
    use crate::theme_config;

    let theme = theme_config::current_theme();

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
        _ => {
            // For other tools, show generic guidance
            println!(
                "{}",
                theme.secondary("┌─ T.JARVIS SYSTEM ──────────────────────────────────────────┐")
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
