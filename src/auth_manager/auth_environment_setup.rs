// Authentication Environment Setup - Environment variable management
//
// This module handles setting up and restoring environment variables to prevent
// browser opening while maintaining tool functionality.

use anyhow::Result;
use std::env;

/// Environment setup utilities for browser prevention
pub struct EnvironmentSetup;

impl EnvironmentSetup {
    /// Set environment variables to prevent browser opening for tools that support it
    pub fn set_no_browser_env_vars() -> Result<()> {
    // Set common no-browser flags
    env::set_var("NO_BROWSER", "1");
    // Save original BROWSER if present and override with a safe no-op
    if let Ok(orig) = env::var("BROWSER") {
        env::set_var("ORIGINAL_BROWSER", orig);
    }
    // Using a simple no-op avoids shell quoting issues when invoked via sh -c
    env::set_var("BROWSER", "true");

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
        }
        env::set_var("DISPLAY", "");

        Ok(())
    }

    /// Restore original environment after tool execution
    pub fn restore_environment() -> Result<()> {
        // Restore DISPLAY if it was temporarily disabled
        if let Ok(original_display) = env::var("ORIGINAL_DISPLAY") {
            env::set_var("DISPLAY", original_display);
            env::remove_var("ORIGINAL_DISPLAY");
        } else if env::var("DISPLAY").ok().as_deref() == Some("") {
            env::remove_var("DISPLAY");
        }

        // Restore or unset BROWSER
        if let Ok(orig) = env::var("ORIGINAL_BROWSER") {
            env::set_var("BROWSER", orig);
            env::remove_var("ORIGINAL_BROWSER");
        } else if env::var("BROWSER").ok().as_deref() == Some("true") {
            env::remove_var("BROWSER");
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
        // Remove no-browser flags and GUI suppressors
        for v in [
            "NO_BROWSER",
            "OAUTH_NO_BROWSER",
            "GOOGLE_APPLICATION_CREDENTIALS_NO_BROWSER",
            "GEMINI_NO_BROWSER",
            "QWEN_NO_BROWSER",
            "CLAUDE_NO_BROWSER",
            "CODEX_NO_BROWSER",
            "DISABLE_GUI",
            "HEADLESS",
        ] {
            env::remove_var(v);
        }

        Ok(())
    }

    /// Setup authentication prevention wrapper for running tools
    pub fn prepare_auth_safe_environment() -> Result<()> {
        use crate::auth_manager::auth_environment_detection::EnvironmentDetector;

        if EnvironmentDetector::should_prevent_browser_opening() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_setup_and_restore() {
        let original_display = env::var("DISPLAY").ok();
        let original_ci = env::var("CI").ok();

        // Test setting no-browser environment
        EnvironmentSetup::set_no_browser_env_vars().unwrap();

        assert_eq!(env::var("NO_BROWSER").unwrap(), "1");
        assert_eq!(env::var("DISABLE_GUI").unwrap(), "1");
        assert_eq!(env::var("HEADLESS").unwrap(), "1");

        // Test restore
        EnvironmentSetup::restore_environment().unwrap();

        // Check that temporary flags are cleaned up
        assert!(env::var("INTERACTIVE").is_err());
        assert!(env::var("FORCE_INTERACTIVE").is_err());

        // Restore original environment
        match original_display {
            Some(val) => env::set_var("DISPLAY", val),
            None => env::remove_var("DISPLAY"),
        }
        match original_ci {
            Some(val) => env::set_var("CI", val),
            None => env::remove_var("CI"),
        }
    }
}
