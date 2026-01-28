// Tools Startup Guidance Domain
// Handles tool-specific startup messages and user guidance

use anyhow::Result;

/// Show minimal startup guidance for tools - only when critical info is needed
pub fn show_tool_startup_guidance(display_name: &str) -> Result<()> {
    use crate::theme::theme_global_config;

    let theme = theme_global_config::current_theme();

    // Only show guidance for tools that need API keys and don't have them set
    match display_name {
        "claude" => {
            if std::env::var("ANTHROPIC_API_KEY").is_err() {
                println!(
                    "{}",
                    theme.secondary("Tip: Set ANTHROPIC_API_KEY for Claude API access")
                );
            }
        }
        "gemini" => {
            if std::env::var("GOOGLE_API_KEY").is_err() && std::env::var("GEMINI_API_KEY").is_err()
            {
                println!(
                    "{}",
                    theme.secondary("Tip: Set GOOGLE_API_KEY for Gemini API access")
                );
            }
        }
        "codex" => {
            if std::env::var("OPENAI_API_KEY").is_err() {
                println!(
                    "{}",
                    theme.secondary("Tip: Set OPENAI_API_KEY for Codex API access")
                );
            }
        }
        "aider" => {
            let has_key = std::env::var("OPENROUTER_API_KEY").is_ok()
                || std::env::var("OPENAI_API_KEY").is_ok()
                || std::env::var("ANTHROPIC_API_KEY").is_ok();
            if !has_key {
                println!(
                    "{}",
                    theme.secondary("Tip: Set OPENROUTER_API_KEY for Aider (or OPENAI/ANTHROPIC)")
                );
            }
        }
        "goose" => {
            let has_key = std::env::var("OPENAI_API_KEY").is_ok()
                || std::env::var("ANTHROPIC_API_KEY").is_ok()
                || std::env::var("GOOGLE_API_KEY").is_ok();
            if !has_key {
                println!(
                    "{}",
                    theme.secondary("Tip: Run 'goose configure' to set up provider/API key")
                );
            }
        }
        "vibe" => {
            if std::env::var("MISTRAL_API_KEY").is_err() {
                println!(
                    "{}",
                    theme.secondary("Tip: Set MISTRAL_API_KEY for Mistral Vibe access")
                );
            }
        }
        "droid" => {
            if std::env::var("FACTORY_API_KEY").is_err() {
                println!(
                    "{}",
                    theme.secondary("Tip: Set FACTORY_API_KEY for Droid access")
                );
            }
        }
        "forge" => {
            if std::env::var("FORGE_API_KEY").is_err() {
                println!(
                    "{}",
                    theme.secondary("Tip: Set FORGE_API_KEY for Forge access")
                );
            }
        }
        "kilocode" => {
            if std::env::var("KILO_API_KEY").is_err() {
                println!(
                    "{}",
                    theme.secondary("Tip: Set KILO_API_KEY for Kilocode access")
                );
            }
        }
        "letta" => {
            if std::env::var("LETTA_API_KEY").is_err() {
                println!(
                    "{}",
                    theme.secondary("Tip: Set LETTA_API_KEY for Letta access")
                );
            }
        }
        "eca" => {
            if std::env::var("ECA_API_KEY").is_err() {
                println!("{}", theme.secondary("Tip: Set ECA_API_KEY for ECA access"));
            }
        }
        "cursor-agent" | "pi" => {
            if std::env::var("OPENAI_API_KEY").is_err()
                && std::env::var("ANTHROPIC_API_KEY").is_err()
            {
                println!(
                    "{}",
                    theme.secondary("Tip: Set OPENAI_API_KEY or ANTHROPIC_API_KEY")
                );
            }
        }
        // Tools that don't need API keys - no guidance needed
        _ => {}
    }

    Ok(())
}
