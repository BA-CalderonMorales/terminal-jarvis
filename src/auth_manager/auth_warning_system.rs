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
            // Minimal one-line preamble (stderr). Styled sections are handled by tool startup guidance.
            eprintln!(
                "Notice: {tool} may try to open a browser. Prefer API keys. Run 'terminal-jarvis auth help {tool}'."
            );
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
            eprintln!("  Help: Run 'terminal-jarvis auth help {tool}' for detailed setup");
            eprintln!();
        }
    }
}

// Removed unused formatting helper after simplifying advisories

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::ffi::OsString;

    struct EnvVarGuard {
        originals: Vec<(&'static str, Option<OsString>)>,
    }

    impl EnvVarGuard {
        fn capture(keys: &[&'static str]) -> Self {
            Self {
                originals: keys.iter().map(|key| (*key, env::var_os(key))).collect(),
            }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            for (key, value) in &self.originals {
                match value {
                    Some(val) => env::set_var(key, val),
                    None => env::remove_var(key),
                }
            }
        }
    }

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
        let _guard = crate::cli_logic::cli_logic_first_run::TEST_ENV_LOCK
            .lock()
            .unwrap();
        let _env = EnvVarGuard::capture(&[
            "CI",
            "GOOGLE_API_KEY",
            "GEMINI_API_KEY",
            "XDG_CONFIG_HOME",
            "APPDATA",
        ]);

        // Test that warning logic is sound
        use crate::auth_manager::auth_api_key_management::ApiKeyManager;
        use crate::auth_manager::auth_environment_detection::EnvironmentDetector;

        // Store original environment
        let temp_config = tempfile::tempdir().unwrap();

        // Set up warning condition (CI environment, no API key or saved credential)
        env::set_var("CI", "true");
        env::remove_var("GOOGLE_API_KEY");
        env::remove_var("GEMINI_API_KEY");
        env::set_var("XDG_CONFIG_HOME", temp_config.path());
        env::set_var("APPDATA", temp_config.path());

        let should_warn = EnvironmentDetector::should_prevent_browser_opening()
            && !ApiKeyManager::check_api_keys_for_tool("gemini");
        assert!(should_warn);
    }
}
