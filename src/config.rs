#![allow(dead_code)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

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
