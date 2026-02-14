// Authentication Preflight - Unified data-driven auth checking and prompting
//
// Replaces hardcoded per-tool match arms in auth_api_key_management.rs
// with TOML-config-driven logic. All auth info comes from config/tools/*.toml.

use anyhow::Result;
use std::collections::HashMap;

use super::auth_credentials_store::CredentialsStore;
use crate::tools::tools_config::{get_tool_config_loader, AuthProvider};

/// Result of an authentication preflight check for a single tool
#[derive(Debug, Clone)]
pub struct AuthPreflightResult {
    pub tool_name: String,
    pub is_ready: bool,
    pub missing_vars: Vec<String>,
    pub set_vars: Vec<String>,
    pub auth_mode: String,
    pub setup_url: Option<String>,
    pub auth_instructions: Option<String>,
    pub cli_auth_command: Option<String>,
    pub providers: Vec<AuthProvider>,
    pub default_env_var: Option<String>,
    pub key_format_hint: Option<String>,
}

/// Zero-sized struct providing static auth preflight utilities.
/// All auth information is driven by TOML config -- no hardcoded per-tool logic.
pub struct AuthPreflight;

impl AuthPreflight {
    /// Check whether authentication is satisfied for the given tool.
    ///
    /// Resolution order for each env var:
    ///   1. std::env::var (current process environment)
    ///   2. CredentialsStore persisted values
    ///
    /// auth_mode semantics:
    ///   "any"  (default) -- at least one env var must be set
    ///   "all"            -- every env var must be set
    ///   "none"           -- tool requires no auth; always ready
    pub fn check(tool_name: &str) -> AuthPreflightResult {
        let loader = get_tool_config_loader();

        let Some(tool_def) = loader.get_tool_definition(tool_name) else {
            // Unknown tool -- assume ready (matches existing _ => true default)
            return AuthPreflightResult {
                tool_name: tool_name.to_string(),
                is_ready: true,
                missing_vars: Vec::new(),
                set_vars: Vec::new(),
                auth_mode: "none".to_string(),
                setup_url: None,
                auth_instructions: None,
                cli_auth_command: None,
                providers: Vec::new(),
                default_env_var: None,
                key_format_hint: None,
            };
        };

        let auth = &tool_def.auth;
        let auth_mode = auth.auth_mode.as_deref().unwrap_or("any").to_string();

        // "none" mode -- always ready
        if auth_mode == "none" {
            return AuthPreflightResult {
                tool_name: tool_name.to_string(),
                is_ready: true,
                missing_vars: Vec::new(),
                set_vars: auth.env_vars.clone(),
                auth_mode,
                setup_url: Some(auth.setup_url.clone()),
                auth_instructions: auth.auth_instructions.clone(),
                cli_auth_command: auth.cli_auth_command.clone(),
                providers: auth.providers.clone(),
                default_env_var: auth.default_env_var.clone(),
                key_format_hint: auth.key_format_hint.clone(),
            };
        }

        // Load saved credentials once
        let saved = CredentialsStore::get_tool_env_vars(tool_name).unwrap_or_default();

        let mut missing_vars = Vec::new();
        let mut set_vars = Vec::new();

        for var in &auth.env_vars {
            if std::env::var(var).is_ok() || saved.contains_key(var) {
                set_vars.push(var.clone());
            } else {
                missing_vars.push(var.clone());
            }
        }

        let is_ready = match auth_mode.as_str() {
            "all" => missing_vars.is_empty(),
            _ => !set_vars.is_empty() || auth.env_vars.is_empty(),
        };

        AuthPreflightResult {
            tool_name: tool_name.to_string(),
            is_ready,
            missing_vars,
            set_vars,
            auth_mode,
            setup_url: Some(auth.setup_url.clone()),
            auth_instructions: auth.auth_instructions.clone(),
            cli_auth_command: auth.cli_auth_command.clone(),
            providers: auth.providers.clone(),
            default_env_var: auth.default_env_var.clone(),
            key_format_hint: auth.key_format_hint.clone(),
        }
    }

    /// Return the list of env vars required by a tool (from TOML config).
    pub fn get_required_env_vars(tool_name: &str) -> Vec<String> {
        let loader = get_tool_config_loader();
        loader
            .get_tool_definition(tool_name)
            .map(|t| t.auth.env_vars.clone())
            .unwrap_or_default()
    }

    /// Build a human-readable help message from TOML config data.
    pub fn get_help_message(tool_name: &str) -> String {
        let loader = get_tool_config_loader();
        let Some(tool_def) = loader.get_tool_definition(tool_name) else {
            return format!(
                "Tool '{}' may require authentication. Please check its documentation.",
                tool_name
            );
        };

        let auth = &tool_def.auth;
        let mut msg = format!("{} requires authentication.", tool_def.display_name);

        // Provider-specific info
        if !auth.providers.is_empty() {
            msg.push_str(" Choose a provider and set the corresponding variable:\n");
            for p in &auth.providers {
                msg.push_str(&format!(
                    "  - {}: export {}=\"your-key\"",
                    p.name, p.env_var
                ));
                if let Some(url) = &p.setup_url {
                    msg.push_str(&format!("\n    Get from: {}", url));
                }
                msg.push('\n');
            }
        } else if !auth.env_vars.is_empty() {
            if auth.env_vars.len() == 1 {
                msg.push_str(&format!(
                    " Set the following environment variable:\n  export {}=\"your-key\"",
                    auth.env_vars[0]
                ));
            } else {
                msg.push_str(" Set one of these environment variables:\n");
                for var in &auth.env_vars {
                    msg.push_str(&format!("  export {}=\"your-key\"\n", var));
                }
            }
        }

        // Setup URL
        if !auth.setup_url.is_empty() {
            msg.push_str(&format!("\nGet your key from: {}", auth.setup_url));
        }

        // Auth instructions
        if let Some(instructions) = &auth.auth_instructions {
            msg.push_str(&format!("\n{}", instructions));
        }

        // CLI auth command
        if let Some(cmd) = &auth.cli_auth_command {
            msg.push_str(&format!("\nAlternatively, run: {}", cmd));
        }

        msg
    }

    /// Validate a key against a glob-like format hint.
    ///
    /// Hint format: "prefix*" means the key must start with "prefix".
    /// An empty or absent hint accepts any non-empty string.
    pub fn validate_key_format(key: &str, hint: &str) -> bool {
        if hint.is_empty() {
            return !key.is_empty();
        }
        let prefix = hint.trim_end_matches('*');
        if prefix.is_empty() {
            return !key.is_empty();
        }
        key.starts_with(prefix)
    }

    /// Interactively prompt the user for missing credentials.
    ///
    /// - If providers are configured, shows a Select to pick one, then prompts
    ///   for that provider's env_var.
    /// - Otherwise prompts for default_env_var first, then remaining missing vars.
    /// - Uses inquire::Password for all key inputs (never echoes secrets).
    /// - Optionally saves to CredentialsStore.
    /// - Sets env vars in the current process.
    pub fn prompt_for_missing(
        tool_name: &str,
        result: &AuthPreflightResult,
    ) -> Result<HashMap<String, String>> {
        use crate::cli_logic::themed_components::themed_confirm;

        let mut collected: HashMap<String, String> = HashMap::new();

        if !result.providers.is_empty() {
            // Provider selection flow
            let provider_names: Vec<String> =
                result.providers.iter().map(|p| p.name.clone()).collect();

            let selection =
                inquire::Select::new("Select an authentication provider:", provider_names)
                    .prompt()?;

            let provider = result
                .providers
                .iter()
                .find(|p| p.name == selection)
                .expect("selected provider must exist");

            let prompt_label = format!("Enter {} ({})", provider.env_var, provider.name);
            let value = inquire::Password::new(&prompt_label)
                .without_confirmation()
                .prompt()?;

            // Warn if format doesn't match (don't block)
            if let Some(hint) = &provider.key_hint {
                if !Self::validate_key_format(&value, hint) {
                    eprintln!(
                        "Warning: key does not match expected format '{}'. Continuing anyway.",
                        hint
                    );
                }
            }

            std::env::set_var(&provider.env_var, &value);
            collected.insert(provider.env_var.clone(), value);
        } else {
            // Standard flow: default_env_var first, then remaining missing
            let mut vars_to_prompt: Vec<String> = Vec::new();

            if let Some(default_var) = &result.default_env_var {
                if result.missing_vars.contains(default_var) {
                    vars_to_prompt.push(default_var.clone());
                }
            }

            for var in &result.missing_vars {
                if !vars_to_prompt.contains(var) {
                    vars_to_prompt.push(var.clone());
                }
            }

            let hint = result.key_format_hint.as_deref().unwrap_or("");

            for var in &vars_to_prompt {
                let prompt_label = format!("Enter {}", var);
                let value = inquire::Password::new(&prompt_label)
                    .without_confirmation()
                    .prompt()?;

                if !hint.is_empty() && !Self::validate_key_format(&value, hint) {
                    eprintln!(
                        "Warning: key does not match expected format '{}'. Continuing anyway.",
                        hint
                    );
                }

                std::env::set_var(var, &value);
                collected.insert(var.clone(), value);

                // In "any" mode, one key is enough
                if result.auth_mode == "any" {
                    break;
                }
            }
        }

        // Offer to save
        if !collected.is_empty() {
            let save = themed_confirm("Save credentials for future sessions?")
                .with_default(true)
                .prompt()
                .unwrap_or(false);

            if save {
                CredentialsStore::upsert_tool_env_vars(tool_name, &collected)?;
            }
        }

        Ok(collected)
    }

    /// Inject saved credentials into a Command's environment before spawning.
    ///
    /// Sources (in order of precedence -- later wins):
    ///   1. CredentialsStore persisted values
    ///   2. Current process env vars (already inherited by Command)
    pub fn inject_credentials(cmd: &mut std::process::Command, tool_name: &str) -> Result<()> {
        let saved = CredentialsStore::get_tool_env_vars(tool_name)?;
        for (key, value) in &saved {
            // Only inject if not already set in the process environment,
            // so explicit env overrides take precedence.
            if std::env::var(key).is_err() {
                cmd.env(key, value);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_key_format_prefix_match() {
        assert!(AuthPreflight::validate_key_format(
            "sk-ant-abc123",
            "sk-ant-*"
        ));
        assert!(!AuthPreflight::validate_key_format("pk-abc123", "sk-ant-*"));
        assert!(AuthPreflight::validate_key_format("AIzaSy123", "AIza*"));
        assert!(!AuthPreflight::validate_key_format("xxxx", "AIza*"));
    }

    #[test]
    fn test_validate_key_format_empty_hint() {
        // Empty hint accepts any non-empty string
        assert!(AuthPreflight::validate_key_format("anything", ""));
        assert!(!AuthPreflight::validate_key_format("", ""));
        // Hint that is just "*" accepts any non-empty string
        assert!(AuthPreflight::validate_key_format("anything", "*"));
    }

    #[test]
    fn test_check_returns_ready_for_unknown_tool() {
        let result = AuthPreflight::check("nonexistent_tool_xyz_42");
        assert!(result.is_ready);
        assert!(result.missing_vars.is_empty());
        assert_eq!(result.auth_mode, "none");
    }

    #[test]
    fn test_get_help_message_generates_text() {
        // For a known tool, should include env var names and setup URL
        let msg = AuthPreflight::get_help_message("claude");
        // If claude.toml is loaded, message will reference ANTHROPIC_API_KEY
        // If not loaded (e.g. CI without config dir), falls back to generic
        assert!(!msg.is_empty());

        // Unknown tool returns a generic message
        let generic = AuthPreflight::get_help_message("unknown_tool_xyz");
        assert!(generic.contains("may require authentication"));
    }

    #[test]
    fn test_get_required_env_vars_unknown_tool() {
        let vars = AuthPreflight::get_required_env_vars("nonexistent_tool_xyz_42");
        assert!(vars.is_empty());
    }
}
