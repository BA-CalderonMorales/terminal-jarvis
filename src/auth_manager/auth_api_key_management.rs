// Authentication API Key Management - API key detection and help messages
//
// This module handles API key detection for different tools and provides
// helpful error messages when authentication is required.

use std::env;

/// API key management utilities
pub struct ApiKeyManager;

impl ApiKeyManager {
    /// Check if required API keys are set for a tool
    pub fn check_api_keys_for_tool(tool: &str) -> bool {
        match tool {
            "aider" => {
                // Aider supports multiple providers; consider any of these sufficient
                env::var("OPENROUTER_API_KEY").is_ok()
                    || env::var("OPENAI_API_KEY").is_ok()
                    || env::var("ANTHROPIC_API_KEY").is_ok()
            }
            "gemini" => {
                env::var("GOOGLE_API_KEY").is_ok()
                    || env::var("GEMINI_API_KEY").is_ok()
                    || env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok()
            }
            "qwen" => {
                env::var("QWEN_CODE_API_KEY").is_ok() || env::var("DASHSCOPE_API_KEY").is_ok()
            }
            "claude" => env::var("ANTHROPIC_API_KEY").is_ok() || env::var("CLAUDE_API_KEY").is_ok(),
            "codex" => env::var("OPENAI_API_KEY").is_ok(),
            "vibe" => env::var("MISTRAL_API_KEY").is_ok(),
            "droid" => env::var("FACTORY_API_KEY").is_ok(),
            "forge" => env::var("FORGE_API_KEY").is_ok(),
            "cursor-agent" => {
                env::var("OPENAI_API_KEY").is_ok() || env::var("ANTHROPIC_API_KEY").is_ok()
            }
            "kilocode" => env::var("KILO_API_KEY").is_ok(),
            "letta" => env::var("LETTA_API_KEY").is_ok(),
            "pi" => env::var("OPENAI_API_KEY").is_ok() || env::var("ANTHROPIC_API_KEY").is_ok(),
            "code" => env::var("OPENAI_API_KEY").is_ok(),
            "eca" => env::var("ECA_API_KEY").is_ok(),
            _ => true, // Assume other tools don't need API keys or handle auth differently
        }
    }

    /// Provide helpful error messages for missing API keys
    pub fn get_api_key_help_message(tool: &str) -> String {
        match tool {
            "aider" => {
                "Aider supports multiple providers. Set one of these environment variables:\n\
         export OPENROUTER_API_KEY=\"your-api-key\"\n\
         export OPENAI_API_KEY=\"your-api-key\"\n\
         export ANTHROPIC_API_KEY=\"your-api-key\"\n\
         \n\
         OpenRouter is a good default that aggregates many models:\n\
         https://openrouter.ai/settings/keys\n\
         See: https://aider.chat/docs/troubleshooting/models-and-keys.html"
                    .to_string()
            }
            "gemini" => {
                "Gemini CLI requires authentication. Set one of these environment variables:\n\
         export GOOGLE_API_KEY=\"your-api-key\"\n\
         export GEMINI_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://makersuite.google.com/app/apikey"
                    .to_string()
            }
            "qwen" => {
                "Qwen Code requires authentication. Set one of these environment variables:\n\
         export QWEN_CODE_API_KEY=\"your-api-key\"\n\
         export DASHSCOPE_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://dashscope.console.aliyun.com/"
                    .to_string()
            }
            "claude" => {
                "Claude CLI requires authentication. Set one of these environment variables:\n\
         export ANTHROPIC_API_KEY=\"your-api-key\"\n\
         export CLAUDE_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://console.anthropic.com/"
                    .to_string()
            }
            "codex" => "OpenAI Codex CLI supports two authentication methods:\n\
         1. ChatGPT account (Plus/Pro/Team): Run 'codex' and select 'Sign in with ChatGPT'\n\
         2. OpenAI API key (usage-based billing):\n\
          export OPENAI_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://platform.openai.com/api-keys"
                .to_string(),
            "vibe" => {
                "Mistral Vibe requires authentication. Set the following environment variable:\n\
         export MISTRAL_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://console.mistral.ai/"
                    .to_string()
            }
            "droid" => {
                "Factory AI Droid requires authentication. Set the following environment variable:\n\
         export FACTORY_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://app.factory.ai/"
                    .to_string()
            }
            "forge" => {
                "Forge requires authentication. Set the following environment variable:\n\
         export FORGE_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://forgecode.dev"
                    .to_string()
            }
            "cursor-agent" => {
                "Cursor Agent requires authentication. Set one of these environment variables:\n\
         export OPENAI_API_KEY=\"your-api-key\"\n\
         export ANTHROPIC_API_KEY=\"your-api-key\""
                    .to_string()
            }
            "kilocode" => {
                "Kilocode requires authentication. Set the following environment variable:\n\
         export KILO_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://kilo.ai/settings"
                    .to_string()
            }
            "letta" => {
                "Letta requires authentication. Set the following environment variable:\n\
         export LETTA_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://letta.com"
                    .to_string()
            }
            "pi" => {
                "Pi requires authentication. Set one of these environment variables:\n\
         export OPENAI_API_KEY=\"your-api-key\"\n\
         export ANTHROPIC_API_KEY=\"your-api-key\""
                    .to_string()
            }
            "code" => {
                "Code requires authentication. Set the following environment variable:\n\
         export OPENAI_API_KEY=\"your-api-key\""
                    .to_string()
            }
            "eca" => {
                "ECA requires authentication. Set the following environment variable:\n\
         export ECA_API_KEY=\"your-api-key\"\n\
         \n\
         Get your API key from: https://eca.dev"
                    .to_string()
            }
            _ => {
                format!("Tool '{tool}' may require authentication. Please check its documentation.")
            }
        }
    }

    /// Get all supported API key environment variables for a tool
    #[allow(dead_code)]
    pub fn get_supported_env_vars(tool: &str) -> Vec<&'static str> {
        match tool {
            "aider" => vec!["OPENROUTER_API_KEY", "OPENAI_API_KEY", "ANTHROPIC_API_KEY"],
            "gemini" => vec![
                "GOOGLE_API_KEY",
                "GEMINI_API_KEY",
                "GOOGLE_APPLICATION_CREDENTIALS",
            ],
            "qwen" => vec!["QWEN_CODE_API_KEY", "DASHSCOPE_API_KEY"],
            "claude" => vec!["ANTHROPIC_API_KEY", "CLAUDE_API_KEY"],
            "codex" => vec!["OPENAI_API_KEY"],
            "vibe" => vec!["MISTRAL_API_KEY"],
            "droid" => vec!["FACTORY_API_KEY"],
            "forge" => vec!["FORGE_API_KEY"],
            "cursor-agent" => vec!["OPENAI_API_KEY", "ANTHROPIC_API_KEY"],
            "kilocode" => vec!["KILO_API_KEY"],
            "letta" => vec!["LETTA_API_KEY"],
            "pi" => vec!["OPENAI_API_KEY", "ANTHROPIC_API_KEY"],
            "code" => vec!["OPENAI_API_KEY"],
            "eca" => vec!["ECA_API_KEY"],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_detection() {
        // Test Gemini API key detection
        env::remove_var("GOOGLE_API_KEY");
        env::remove_var("GEMINI_API_KEY");
        assert!(!ApiKeyManager::check_api_keys_for_tool("gemini"));

        env::set_var("GOOGLE_API_KEY", "test-key");
        assert!(ApiKeyManager::check_api_keys_for_tool("gemini"));
        env::remove_var("GOOGLE_API_KEY");

        // Test Qwen API key detection
        env::remove_var("QWEN_CODE_API_KEY");
        env::remove_var("DASHSCOPE_API_KEY");
        assert!(!ApiKeyManager::check_api_keys_for_tool("qwen"));

        env::set_var("QWEN_CODE_API_KEY", "test-key");
        assert!(ApiKeyManager::check_api_keys_for_tool("qwen"));
        env::remove_var("QWEN_CODE_API_KEY");

        // Test Codex API key detection
        env::remove_var("OPENAI_API_KEY");
        assert!(!ApiKeyManager::check_api_keys_for_tool("codex"));

        env::set_var("OPENAI_API_KEY", "test-key");
        assert!(ApiKeyManager::check_api_keys_for_tool("codex"));
        env::remove_var("OPENAI_API_KEY");
    }

    #[test]
    fn test_help_messages() {
        let gemini_help = ApiKeyManager::get_api_key_help_message("gemini");
        assert!(gemini_help.contains("GOOGLE_API_KEY"));
        assert!(gemini_help.contains("makersuite.google.com"));

        let qwen_help = ApiKeyManager::get_api_key_help_message("qwen");
        assert!(qwen_help.contains("QWEN_CODE_API_KEY"));
        assert!(qwen_help.contains("dashscope.console.aliyun.com"));

        let codex_help = ApiKeyManager::get_api_key_help_message("codex");
        assert!(codex_help.contains("OPENAI_API_KEY"));
        assert!(codex_help.contains("platform.openai.com"));
    }

    #[test]
    fn test_supported_env_vars() {
        let gemini_vars = ApiKeyManager::get_supported_env_vars("gemini");
        assert!(gemini_vars.contains(&"GOOGLE_API_KEY"));
        assert!(gemini_vars.contains(&"GEMINI_API_KEY"));

        let qwen_vars = ApiKeyManager::get_supported_env_vars("qwen");
        assert!(qwen_vars.contains(&"QWEN_CODE_API_KEY"));
        assert!(qwen_vars.contains(&"DASHSCOPE_API_KEY"));

        let unknown_vars = ApiKeyManager::get_supported_env_vars("unknown_tool");
        assert!(unknown_vars.is_empty());
    }
}
