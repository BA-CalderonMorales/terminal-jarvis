// Authentication Manager Entry Point - Main AuthManager coordination
//
// This module provides the main AuthManager interface that coordinates
// between different authentication domains for a unified API.

use anyhow::Result;
use std::collections::HashMap;

use super::auth_credentials_store::CredentialsStore;
use super::auth_preflight;

/// Main authentication manager that coordinates all authentication functionality
pub struct AuthManager;

impl AuthManager {
    /// Check if we're running in an environment where browser opening should be prevented
    #[allow(dead_code)]
    pub fn should_prevent_browser_opening() -> bool {
        use crate::auth_manager::auth_environment_detection::EnvironmentDetector;
        EnvironmentDetector::should_prevent_browser_opening()
    }

    /// Set environment variables to prevent browser opening for tools that support it
    #[allow(dead_code)]
    pub fn set_no_browser_env_vars() -> Result<()> {
        use crate::auth_manager::auth_environment_setup::EnvironmentSetup;
        EnvironmentSetup::set_no_browser_env_vars()
    }

    /// Restore original environment after tool execution
    #[allow(dead_code)]
    pub fn restore_environment() -> Result<()> {
        use crate::auth_manager::auth_environment_setup::EnvironmentSetup;
        EnvironmentSetup::restore_environment()
    }

    /// Check if required API keys are set for a tool
    #[allow(dead_code)]
    pub fn check_api_keys_for_tool(tool: &str) -> bool {
        use crate::auth_manager::auth_api_key_management::ApiKeyManager;
        ApiKeyManager::check_api_keys_for_tool(tool)
    }

    /// Provide helpful error messages for missing API keys
    #[allow(dead_code)]
    pub fn get_api_key_help_message(tool: &str) -> String {
        use crate::auth_manager::auth_api_key_management::ApiKeyManager;
        ApiKeyManager::get_api_key_help_message(tool)
    }

    /// Setup authentication prevention wrapper for running tools
    #[allow(dead_code)]
    pub fn prepare_auth_safe_environment() -> Result<()> {
        use crate::auth_manager::auth_environment_setup::EnvironmentSetup;
        EnvironmentSetup::prepare_auth_safe_environment()
    }

    /// Check if a tool is likely to open a browser and warn user
    #[allow(dead_code)]
    pub fn warn_if_browser_likely(tool: &str) -> Result<()> {
        use crate::auth_manager::auth_warning_system::WarningSystem;
        WarningSystem::warn_if_browser_likely(tool)
    }

    /// Load saved credentials from store and export as env vars (session-scoped)
    pub fn export_saved_env_vars() -> Result<()> {
        let creds = CredentialsStore::load()?;
        for (_tool, vars) in creds.tools.iter() {
            for (k, v) in vars {
                std::env::set_var(k, v);
            }
        }
        Ok(())
    }

    /// Persist credentials for a specific tool
    pub fn save_tool_credentials(
        tool: &str,
        vars: &std::collections::HashMap<String, String>,
    ) -> Result<()> {
        CredentialsStore::upsert_tool_env_vars(tool, vars)
    }

    /// Fetch saved credentials for a specific tool
    pub fn get_tool_credentials(tool: &str) -> Result<std::collections::HashMap<String, String>> {
        CredentialsStore::get_tool_env_vars(tool)
    }

    /// Delete specific credentials for a tool (if keys empty, removes the tool entirely)
    pub fn delete_tool_credentials(tool: &str, keys: &[String]) -> Result<()> {
        CredentialsStore::delete_tool_env_vars(tool, keys)
    }

    /// Clear all saved credentials for all tools
    pub fn clear_all_credentials() -> Result<()> {
        CredentialsStore::clear_all()
    }

    /// Data-driven auth preflight check using TOML config
    pub fn check_auth_preflight(tool: &str) -> auth_preflight::AuthPreflightResult {
        auth_preflight::AuthPreflight::check(tool)
    }

    /// Interactively prompt the user for missing auth credentials
    pub fn prompt_for_missing_auth(
        tool: &str,
        result: &auth_preflight::AuthPreflightResult,
    ) -> Result<HashMap<String, String>> {
        auth_preflight::AuthPreflight::prompt_for_missing(tool, result)
    }
}
