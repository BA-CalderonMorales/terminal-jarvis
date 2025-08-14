use anyhow::Result;
use std::env;

/// Authentication management utilities for preventing unwanted browser opening
pub struct AuthManager;

impl AuthManager {
    /// Check if we're running in an environment where browser opening should be prevented
    pub fn should_prevent_browser_opening() -> bool {
        // Prevent browser opening in CI environments
        if env::var("CI").is_ok() {
            return true;
        }

        // Prevent browser opening if no DISPLAY is set (headless environments)
        if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() {
            return true;
        }

        // Prevent browser opening in cloud development environments
        if env::var("CODESPACES").is_ok()
            || env::var("GITPOD_WORKSPACE_ID").is_ok()
            || env::var("CLOUD_SHELL").is_ok()
        {
            return true;
        }

        // Check for terminal-specific environments that can't handle browser opening
        if let Ok(term) = env::var("TERM") {
            if term == "dumb" || term.contains("screen") {
                return true;
            }
        }

        // Check if we're running in SSH session
        if env::var("SSH_CONNECTION").is_ok() || env::var("SSH_CLIENT").is_ok() {
            return true;
        }

        // Check if running in a container
        if std::path::Path::new("/.dockerenv").exists() {
            return true;
        }

        false
    }

    /// Set environment variables to prevent browser opening for tools that support it
    pub fn set_no_browser_env_vars() -> Result<()> {
        // Set common no-browser flags
        env::set_var("NO_BROWSER", "1");
        env::set_var("BROWSER", "echo 'Browser prevented by Terminal Jarvis:'"); // Override browser command

        // OAuth and authentication prevention
        env::set_var("OAUTH_NO_BROWSER", "1");
        env::set_var("GOOGLE_APPLICATION_CREDENTIALS_NO_BROWSER", "1");

        // Tool-specific browser prevention
        env::set_var("GEMINI_NO_BROWSER", "1");
        env::set_var("QWEN_NO_BROWSER", "1");
        env::set_var("CLAUDE_NO_BROWSER", "1");
        env::set_var("CODEX_NO_BROWSER", "1");

        // Disable GUI-related features that might trigger browser opening
        env::set_var("DISABLE_GUI", "1");
        env::set_var("HEADLESS", "1");

        // Override common browser environment variables
        if let Ok(original_display) = env::var("DISPLAY") {
            // Temporarily disable DISPLAY to prevent GUI applications
            env::set_var("ORIGINAL_DISPLAY", original_display);
            env::set_var("DISPLAY", "");
        }

        Ok(())
    }

    /// Restore original environment after tool execution
    pub fn restore_environment() -> Result<()> {
        // Restore DISPLAY if it was temporarily disabled
        if let Ok(original_display) = env::var("ORIGINAL_DISPLAY") {
            env::set_var("DISPLAY", original_display);
            env::remove_var("ORIGINAL_DISPLAY");
        }

        // Restore CI flag if we're actually in CI
        if std::path::Path::new("/.dockerenv").exists()
            || env::var("GITHUB_ACTIONS").is_ok()
            || env::var("CODESPACES").is_ok()
        {
            env::set_var("CI", "true");
        }

        // Clean up temporary interactive flags
        env::remove_var("INTERACTIVE");
        env::remove_var("FORCE_INTERACTIVE");

        Ok(())
    }

    /// Check if required API keys are set for a tool
    #[allow(dead_code)]
    pub fn check_api_keys_for_tool(tool: &str) -> bool {
        match tool {
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
            _ => true, // Assume other tools don't need API keys or handle auth differently
        }
    }

    /// Provide helpful error messages for missing API keys
    #[allow(dead_code)]
    pub fn get_api_key_help_message(tool: &str) -> String {
        match tool {
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
            _ => {
                format!("Tool '{tool}' may require authentication. Please check its documentation.")
            }
        }
    }

    /// Setup authentication prevention wrapper for running tools
    pub fn prepare_auth_safe_environment() -> Result<()> {
        if Self::should_prevent_browser_opening() {
            Self::set_no_browser_env_vars()?;

            // Make tools think they're in a fully interactive environment
            // but prevent browser opening
            env::set_var("TERM", "xterm-256color"); // Ensure interactive terminal
            env::set_var("COLUMNS", "120"); // Set terminal width
            env::set_var("LINES", "30"); // Set terminal height

            // Keep DISPLAY empty to prevent browser opening
            env::set_var("DISPLAY", "");
            env::set_var("XDG_CURRENT_DESKTOP", "");

            // But don't make the environment look completely headless
            env::remove_var("CI"); // Remove CI flag temporarily

            // Ensure tools stay interactive
            env::set_var("INTERACTIVE", "1");
            env::set_var("FORCE_INTERACTIVE", "1");
        }

        Ok(())
    }

    /// Check if a tool is likely to open a browser and warn user
    pub fn warn_if_browser_likely(_tool: &str) -> Result<()> {
        // T.JARVIS startup guidance in tools.rs now handles all authentication messaging
        // This keeps our interface clean and professional
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_should_prevent_browser_opening() {
        // Store original environment
        let original_ci = env::var("CI").ok();
        let original_display = env::var("DISPLAY").ok();
        let original_wayland = env::var("WAYLAND_DISPLAY").ok();
        let original_codespaces = env::var("CODESPACES").ok();
        let original_ssh = env::var("SSH_CONNECTION").ok();
        let original_term = env::var("TERM").ok();

        // Test CI environment
        env::remove_var("DISPLAY");
        env::remove_var("WAYLAND_DISPLAY");
        env::remove_var("CODESPACES");
        env::remove_var("SSH_CONNECTION");
        env::set_var("TERM", "xterm");
        env::set_var("CI", "true");
        assert!(AuthManager::should_prevent_browser_opening());
        env::remove_var("CI");

        // Test headless environment
        env::remove_var("DISPLAY");
        env::remove_var("WAYLAND_DISPLAY");
        assert!(AuthManager::should_prevent_browser_opening());

        // Test GUI environment - clear all blocking factors
        env::remove_var("CI");
        env::remove_var("CODESPACES");
        env::remove_var("SSH_CONNECTION");
        env::remove_var("SSH_CLIENT");
        env::set_var("TERM", "xterm-256color");
        env::set_var("DISPLAY", ":0");
        // Only assert false if not in a container (which would still block browser opening)
        if !std::path::Path::new("/.dockerenv").exists() {
            assert!(!AuthManager::should_prevent_browser_opening());
        }
        env::remove_var("DISPLAY");

        // Test Codespaces
        env::set_var("CODESPACES", "true");
        assert!(AuthManager::should_prevent_browser_opening());
        env::remove_var("CODESPACES");

        // Restore original environment
        match original_ci {
            Some(val) => env::set_var("CI", val),
            None => env::remove_var("CI"),
        }
        match original_display {
            Some(val) => env::set_var("DISPLAY", val),
            None => env::remove_var("DISPLAY"),
        }
        match original_wayland {
            Some(val) => env::set_var("WAYLAND_DISPLAY", val),
            None => env::remove_var("WAYLAND_DISPLAY"),
        }
        match original_codespaces {
            Some(val) => env::set_var("CODESPACES", val),
            None => env::remove_var("CODESPACES"),
        }
        match original_ssh {
            Some(val) => env::set_var("SSH_CONNECTION", val),
            None => env::remove_var("SSH_CONNECTION"),
        }
        match original_term {
            Some(val) => env::set_var("TERM", val),
            None => env::remove_var("TERM"),
        }
    }

    #[test]
    fn test_api_key_detection() {
        // Test Gemini API key detection
        env::remove_var("GOOGLE_API_KEY");
        env::remove_var("GEMINI_API_KEY");
        assert!(!AuthManager::check_api_keys_for_tool("gemini"));

        env::set_var("GOOGLE_API_KEY", "test-key");
        assert!(AuthManager::check_api_keys_for_tool("gemini"));
        env::remove_var("GOOGLE_API_KEY");

        // Test Qwen API key detection
        env::remove_var("QWEN_CODE_API_KEY");
        env::remove_var("DASHSCOPE_API_KEY");
        assert!(!AuthManager::check_api_keys_for_tool("qwen"));

        env::set_var("QWEN_CODE_API_KEY", "test-key");
        assert!(AuthManager::check_api_keys_for_tool("qwen"));
        env::remove_var("QWEN_CODE_API_KEY");

        // Test Codex API key detection
        env::remove_var("OPENAI_API_KEY");
        assert!(!AuthManager::check_api_keys_for_tool("codex"));

        env::set_var("OPENAI_API_KEY", "test-key");
        assert!(AuthManager::check_api_keys_for_tool("codex"));
        env::remove_var("OPENAI_API_KEY");
    }

    #[test]
    fn test_help_messages() {
        let gemini_help = AuthManager::get_api_key_help_message("gemini");
        assert!(gemini_help.contains("GOOGLE_API_KEY"));
        assert!(gemini_help.contains("makersuite.google.com"));

        let qwen_help = AuthManager::get_api_key_help_message("qwen");
        assert!(qwen_help.contains("QWEN_CODE_API_KEY"));
        assert!(qwen_help.contains("dashscope.console.aliyun.com"));

        let codex_help = AuthManager::get_api_key_help_message("codex");
        assert!(codex_help.contains("OPENAI_API_KEY"));
        assert!(codex_help.contains("platform.openai.com"));
    }
}
