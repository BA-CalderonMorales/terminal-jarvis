// Configuration Structures - Core data structures for configuration management
//
// This module defines the main configuration structures used throughout
// Terminal Jarvis, including tool configs, template configs, and API configs.

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

        tools.insert(
            "codex".to_string(),
            ToolConfig {
                enabled: true,
                auto_update: true,
                install_command: Some("npm install -g @openai/codex".to_string()),
                update_command: Some("npm update -g @openai/codex".to_string()),
            },
        );

        tools.insert(
            "crush".to_string(),
            ToolConfig {
                enabled: true,
                auto_update: true,
                install_command: Some("npm install -g @charmland/crush".to_string()),
                update_command: Some("npm update -g @charmland/crush".to_string()),
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
