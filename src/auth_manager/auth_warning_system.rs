// Authentication Warning System - User notification for browser-related issues
//
// This module handles warning users about potential browser opening issues
// and provides guidance for avoiding authentication problems.

use anyhow::Result;

/// Warning system for authentication-related issues
pub struct WarningSystem;

impl WarningSystem {
    /// Check if a tool is likely to open a browser and warn user
    pub fn warn_if_browser_likely(tool: &str) -> Result<()> {
        use crate::auth_manager::auth_api_key_management::ApiKeyManager;
        use crate::auth_manager::auth_environment_detection::EnvironmentDetector;

        if EnvironmentDetector::should_prevent_browser_opening()
            && !ApiKeyManager::check_api_keys_for_tool(tool)
        {
            eprintln!("WARNING: {tool} may attempt to open a browser for authentication.");
            eprintln!("  This can cause issues in terminal/cloud environments.");
            eprintln!("  Consider setting API keys to avoid browser authentication.");
            // Intentionally avoid printing dynamic help content to stderr to prevent
            // potential cleartext-logging flags. Provide a static pointer instead.
            eprintln!("  Help: Run 'terminal-jarvis auth-help {tool}' for detailed setup");
            eprintln!();
        }

        Ok(())
    }

    /// Provide a comprehensive warning about environment issues
    #[allow(dead_code)]
    pub fn warn_about_environment_issues() {
        use crate::auth_manager::auth_environment_detection::EnvironmentDetector;

        if EnvironmentDetector::should_prevent_browser_opening() {
            eprintln!("NOTICE: Terminal Jarvis detected a restricted environment.");

            if EnvironmentDetector::is_ci_environment() {
                eprintln!("  - CI environment detected: Browser opening will be prevented");
            }
            if EnvironmentDetector::is_cloud_environment() {
                eprintln!("  - Cloud development environment detected");
            }
            if EnvironmentDetector::is_container_environment() {
                eprintln!("  - Container environment detected");
            }

            eprintln!("  Tools will be configured to avoid browser authentication when possible.");
            eprintln!();
        }
    }

    /// Show available authentication methods for a tool
    #[allow(dead_code)]
    pub fn show_auth_methods(tool: &str) {
        use crate::auth_manager::auth_api_key_management::ApiKeyManager;

        let env_vars = ApiKeyManager::get_supported_env_vars(tool);
        if !env_vars.is_empty() {
            eprintln!("Available authentication methods for {tool}:");
            eprintln!("  Environment variables: {}", env_vars.join(", "));
            eprintln!("  Help: Run 'terminal-jarvis auth-help {tool}' for detailed setup");
            eprintln!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_warning_system() {
        // This is mainly testing that the functions don't panic
        // Real testing would need to capture stdout/stderr
        WarningSystem::warn_if_browser_likely("gemini").unwrap();
        WarningSystem::warn_about_environment_issues();
        WarningSystem::show_auth_methods("claude");
    }

    #[test]
    fn test_warning_conditions() {
        // Test that warning logic is sound
        use crate::auth_manager::auth_api_key_management::ApiKeyManager;
        use crate::auth_manager::auth_environment_detection::EnvironmentDetector;

        // Store original environment
        let original_ci = env::var("CI").ok();
        let original_api_key = env::var("GOOGLE_API_KEY").ok();

        // Set up warning condition (CI environment, no API key)
        env::set_var("CI", "true");
        env::remove_var("GOOGLE_API_KEY");

        let should_warn = EnvironmentDetector::should_prevent_browser_opening()
            && !ApiKeyManager::check_api_keys_for_tool("gemini");
        assert!(should_warn);

        // Clean up
        match original_ci {
            Some(val) => env::set_var("CI", val),
            None => env::remove_var("CI"),
        }
        match original_api_key {
            Some(val) => env::set_var("GOOGLE_API_KEY", val),
            None => env::remove_var("GOOGLE_API_KEY"),
        }
    }
}
