// Configuration File Operations - File loading, saving, and merging logic
//
// DEPRECATED: This module provides TOML-based configuration loading.
// The primary source is now the database (via src/db/).
//
// This TOML loader is kept as a FALLBACK for:
// 1. Initial import of configs into the database
// 2. Environments where the database hasn't been initialized
// 3. User-level config overrides (terminal-jarvis.toml)
//
// For new code, prefer using database repositories.

use crate::config::config_structures::{Config, ToolConfig};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Default per-user configuration path.
pub fn default_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("terminal-jarvis").join("config.toml"))
}

/// Custom configuration path persisted in user preferences, if configured.
pub fn custom_config_path() -> Option<PathBuf> {
    crate::cli_logic::cli_logic_first_run::get_custom_config_path()
}

/// Active configuration path and whether it comes from a custom preference.
pub fn active_config_path() -> (PathBuf, bool) {
    if let Some(path) = custom_config_path() {
        return (path, true);
    }

    (
        default_config_path().unwrap_or_else(|| PathBuf::from("config.toml")),
        false,
    )
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        let custom_path = custom_config_path();
        let config_paths = vec![
            custom_path.clone(),
            default_config_path(),
            Some(PathBuf::from("./terminal-jarvis.toml")),
            Some(PathBuf::from("./terminal-jarvis.toml.example")),
            // Add NPM package config path - look relative to binary location
            std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|p| p.join("../config/default.toml")))
                .filter(|p| p.exists()),
        ];

        // Start with default configuration
        let mut config = Config::default();

        // Try to load user configuration and merge it
        for path in config_paths.into_iter().flatten() {
            if path.exists() {
                match Self::load_from_path(&path, &mut config) {
                    Ok(()) => return Ok(config),
                    Err(e) => {
                        if custom_path.as_ref() == Some(&path) {
                            return Err(e).with_context(|| {
                                format!(
                                    "Failed to load custom configuration file {}",
                                    path.display()
                                )
                            });
                        }
                        eprintln!(
                            "Warning: Failed to load config file {}: {}",
                            path.display(),
                            e
                        );
                        eprintln!("Trying fallback configuration locations");
                    }
                };
            } else if custom_path.as_ref() == Some(&path) {
                return Err(anyhow::anyhow!(
                    "Custom configuration file does not exist: {}",
                    path.display()
                ));
            }
        }

        // Return default config if no file found (ensure defaults are present)
        config.ensure_default_tools();
        Ok(config)
    }

    /// Validate and merge a configuration file into the provided config.
    pub fn load_from_path(path: &Path, config: &mut Config) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let user_config = toml::from_str::<Config>(&content)?;

        for (tool_name, tool_config) in user_config.tools {
            config.tools.insert(tool_name, tool_config);
        }

        config.templates = user_config.templates;
        config.api = user_config.api;
        config.ensure_default_tools();

        Ok(())
    }

    /// Save configuration to the user config directory
    #[allow(dead_code)]
    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
            .join("terminal-jarvis");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;

        Ok(())
    }

    /// Ensure all default tools are present in the configuration
    pub fn ensure_default_tools(&mut self) {
        let default_config = Config::default();

        // Add any missing default tools
        for (tool_name, tool_config) in default_config.tools {
            self.tools.entry(tool_name).or_insert(tool_config);
        }
    }
}

impl Config {
    /// Get tool configuration
    #[allow(dead_code)]
    pub fn get_tool_config(&self, tool: &str) -> Option<&ToolConfig> {
        self.tools.get(tool)
    }

    /// Check if a tool is enabled
    #[allow(dead_code)]
    pub fn is_tool_enabled(&self, tool: &str) -> bool {
        self.tools.get(tool).map(|c| c.enabled).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    struct EnvVarGuard {
        originals: Vec<(&'static str, Option<OsString>)>,
    }

    impl EnvVarGuard {
        fn capture(keys: &[&'static str]) -> Self {
            Self {
                originals: keys
                    .iter()
                    .map(|key| (*key, std::env::var_os(key)))
                    .collect(),
            }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            for (key, value) in &self.originals {
                match value {
                    Some(val) => std::env::set_var(key, val),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    #[test]
    fn custom_config_load_failure_reports_custom_path_without_fallback_message() {
        let _lock = crate::cli_logic::cli_logic_first_run::TEST_ENV_LOCK
            .lock()
            .unwrap();
        let _env = EnvVarGuard::capture(&["HOME"]);
        let temp_home = tempfile::tempdir().unwrap();
        std::env::set_var("HOME", temp_home.path());

        let custom_path = temp_home.path().join("bad-config.toml");
        std::fs::write(&custom_path, "not = [valid").unwrap();
        crate::cli_logic::cli_logic_first_run::save_custom_config_path(&custom_path).unwrap();

        let err = Config::load().unwrap_err();
        let message = err.to_string();

        assert!(
            message.contains("Failed to load custom configuration file"),
            "custom config load error should identify the custom config path: {message}"
        );
        assert!(
            message.contains(&custom_path.display().to_string()),
            "custom config load error should include the custom path: {message}"
        );
        assert!(
            !message.contains("Using default configuration"),
            "custom config load error should not imply fallback to defaults: {message}"
        );
    }
}
