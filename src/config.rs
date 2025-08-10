#![allow(dead_code)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCache {
    pub version_info: String,
    pub cached_at: u64,   // Unix timestamp
    pub ttl_seconds: u64, // Time to live in seconds
}

impl VersionCache {
    pub fn new(version_info: String, ttl_seconds: u64) -> Self {
        let cached_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        Self {
            version_info,
            cached_at,
            ttl_seconds,
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        now - self.cached_at > self.ttl_seconds
    }

    pub fn remaining_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        (self.cached_at + self.ttl_seconds).saturating_sub(now)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub tools: HashMap<String, ToolConfig>,
    pub templates: TemplateConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    pub enabled: bool,
    pub auto_update: bool,
    pub install_command: Option<String>,
    pub update_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub repository: Option<String>,
    pub auto_sync: bool,
    pub local_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        let mut tools = HashMap::new();

        // Default tool configurations
        tools.insert(
            "claude-code".to_string(),
            ToolConfig {
                enabled: true,
                auto_update: true,
                install_command: Some("npm install -g @anthropic-ai/claude-code".to_string()),
                update_command: Some("npm update -g @anthropic-ai/claude-code".to_string()),
            },
        );

        tools.insert(
            "gemini-cli".to_string(),
            ToolConfig {
                enabled: true,
                auto_update: false,
                install_command: Some("npm install -g @google/gemini-cli".to_string()),
                update_command: Some("npm update -g @google/gemini-cli".to_string()),
            },
        );

        tools.insert(
            "qwen-code".to_string(),
            ToolConfig {
                enabled: true,
                auto_update: true,
                install_command: Some("npm install -g @qwen-code/qwen-code@latest".to_string()),
                update_command: Some("npm update -g @qwen-code/qwen-code".to_string()),
            },
        );

        tools.insert(
            "opencode".to_string(),
            ToolConfig {
                enabled: true,
                auto_update: true,
                install_command: Some("npm install -g opencode-ai@latest".to_string()),
                update_command: Some("npm update -g opencode-ai".to_string()),
            },
        );

        tools.insert(
            "llxprt-code".to_string(),
            ToolConfig {
                enabled: true,
                auto_update: true,
                install_command: Some("npm install -g @vybestack/llxprt-code".to_string()),
                update_command: Some("npm update -g @vybestack/llxprt-code".to_string()),
            },
        );

        Self {
            tools,
            templates: TemplateConfig {
                repository: None,
                auto_sync: true,
                local_path: None,
            },
            api: ApiConfig {
                base_url: "https://api.terminal-jarvis.dev".to_string(),
                timeout_seconds: 30,
                max_retries: 3,
            },
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        let config_paths = vec![
            dirs::config_dir().map(|p| p.join("terminal-jarvis").join("config.toml")),
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
                match std::fs::read_to_string(&path) {
                    Ok(content) => {
                        // Try to parse as partial TOML first, then fallback to full Config
                        match toml::from_str::<Config>(&content) {
                            Ok(user_config) => {
                                // Merge user config with defaults (user settings override defaults)
                                for (tool_name, tool_config) in user_config.tools {
                                    config.tools.insert(tool_name, tool_config);
                                }

                                // Update other settings if they exist in user config
                                config.templates = user_config.templates;
                                config.api = user_config.api;

                                // Ensure all defaults are still present
                                config.ensure_default_tools();
                                return Ok(config);
                            }
                            Err(e) => {
                                eprintln!(
                                    "Warning: Failed to parse config file {}: {}",
                                    path.display(),
                                    e
                                );
                                eprintln!("Using default configuration");
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to read config file {}: {}",
                            path.display(),
                            e
                        );
                        continue;
                    }
                }
            }
        }

        // Return default config if no file found (ensure defaults are present)
        config.ensure_default_tools();
        Ok(config)
    }

    /// Save configuration to the user config directory
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

    /// Get tool configuration
    pub fn get_tool_config(&self, tool: &str) -> Option<&ToolConfig> {
        self.tools.get(tool)
    }

    /// Check if a tool is enabled
    pub fn is_tool_enabled(&self, tool: &str) -> bool {
        self.tools.get(tool).map(|c| c.enabled).unwrap_or(false)
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

/// Configuration manager for handling config files and caching
pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
            .join("terminal-jarvis");

        // Create config directory if it doesn't exist
        std::fs::create_dir_all(&config_dir)?;

        Ok(Self { config_dir })
    }

    /// Get the path to the version cache file
    pub fn get_version_cache_path(&self) -> PathBuf {
        self.config_dir.join("version_cache.toml")
    }

    /// Load version cache from disk
    pub fn load_version_cache(&self) -> Result<Option<VersionCache>> {
        let cache_path = self.get_version_cache_path();

        if !cache_path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&cache_path)?;
        let cache: VersionCache = toml::from_str(&content)?;

        if cache.is_expired() {
            // Clean up expired cache
            let _ = std::fs::remove_file(&cache_path);
            return Ok(None);
        }

        Ok(Some(cache))
    }

    /// Save version cache to disk
    pub fn save_version_cache(&self, cache: &VersionCache) -> Result<()> {
        let cache_path = self.get_version_cache_path();
        let content = toml::to_string_pretty(cache)?;
        std::fs::write(&cache_path, content)?;
        Ok(())
    }

    /// Clear version cache
    pub fn clear_version_cache(&self) -> Result<()> {
        let cache_path = self.get_version_cache_path();
        if cache_path.exists() {
            std::fs::remove_file(&cache_path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_version_cache_creation() {
        let cache = VersionCache::new("1.0.0 (@stable)".to_string(), 3600);
        assert_eq!(cache.version_info, "1.0.0 (@stable)");
        assert_eq!(cache.ttl_seconds, 3600);
        assert!(!cache.is_expired()); // Should not be expired immediately
    }

    #[test]
    fn test_version_cache_expiration() {
        let mut cache = VersionCache::new("1.0.0".to_string(), 1);
        cache.cached_at = 0; // Force expiration by setting very old timestamp
        assert!(cache.is_expired());
    }

    #[test]
    fn test_version_cache_remaining_seconds() {
        let cache = VersionCache::new("1.0.0".to_string(), 3600);
        let remaining = cache.remaining_seconds();
        // Should be close to 3600, allowing for a few seconds of execution time
        assert!(remaining > 3590 && remaining <= 3600);
    }

    #[test]
    fn test_config_manager_version_cache_file_operations() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new()?;
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };

        let cache = VersionCache::new("1.2.3 (@stable, beta)".to_string(), 3600);

        // Test save
        config_manager.save_version_cache(&cache)?;

        // Test load
        let loaded_cache = config_manager.load_version_cache()?.unwrap();
        assert_eq!(loaded_cache.version_info, "1.2.3 (@stable, beta)");
        assert_eq!(loaded_cache.ttl_seconds, 3600);

        // Test clear
        config_manager.clear_version_cache()?;
        let after_clear = config_manager.load_version_cache()?;
        assert!(after_clear.is_none());

        Ok(())
    }

    #[test]
    fn test_expired_cache_cleanup() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };

        // Create an expired cache
        let mut expired_cache = VersionCache::new("1.0.0".to_string(), 1);
        expired_cache.cached_at = 0; // Make it ancient

        config_manager.save_version_cache(&expired_cache)?;

        // When we try to load, it should return None and clean up the file
        let loaded = config_manager.load_version_cache()?;
        assert!(loaded.is_none());

        // The cache file should no longer exist
        assert!(!config_manager.get_version_cache_path().exists());

        Ok(())
    }
}
