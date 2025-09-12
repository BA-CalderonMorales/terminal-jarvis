// AI Tools Registry - TOML-based configuration for AI coding tools
//
// This module provides a centralized system for loading AI tool configurations
// from the ai-tools-registry.toml file, enabling better maintainability and
// easier database integration in the future.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete AI Tools Registry loaded from TOML
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiToolsRegistry {
    pub metadata: RegistryMetadata,
    pub tools: HashMap<String, ToolDefinition>,
}

/// Registry metadata information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegistryMetadata {
    pub version: String,
    pub description: String,
    pub last_updated: String,
}

/// Complete tool definition with all configuration sections
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolDefinition {
    pub display_name: String,
    pub config_key: String,
    pub description: String,
    pub homepage: String,
    pub documentation: String,
    pub cli_command: String,
    pub requires_npm: bool,
    pub requires_sudo: bool,
    pub install: CommandDefinition,
    pub update: CommandDefinition,
    pub auth: AuthDefinition,
}

/// Command definition for install/update operations
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandDefinition {
    pub command: String,
    pub args: Vec<String>,
}

/// Authentication configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthDefinition {
    pub env_vars: Vec<String>,
    pub setup_url: String,
    pub browser_auth: bool,
}

/// Installation command metadata for compatibility with existing code
#[derive(Debug, Clone)]
pub struct InstallCommand {
    pub command: String,
    pub args: Vec<String>,
    pub description: String,
    pub requires_npm: bool,
    pub requires_sudo: bool,
}

/// Manager for AI Tools Registry operations
pub struct AiToolsRegistryManager {
    registry: AiToolsRegistry,
}

impl AiToolsRegistryManager {
    /// Load the AI Tools Registry from the TOML file
    pub fn new() -> Result<Self> {
        let registry_content = include_str!("../ai-tools-registry.toml");
        let registry: AiToolsRegistry = toml::from_str(registry_content)
            .map_err(|e| anyhow!("Failed to parse ai-tools-registry.toml: {}", e))?;

        Ok(Self { registry })
    }

    /// Get all available tool names
    pub fn get_tool_names(&self) -> Vec<String> {
        self.registry.tools.keys().cloned().collect()
    }

    /// Get installation command for a specific tool
    pub fn get_install_command(&self, tool: &str) -> Option<InstallCommand> {
        self.registry.tools.get(tool).map(|tool_def| InstallCommand {
            command: tool_def.install.command.clone(),
            args: tool_def.install.args.clone(),
            description: tool_def.description.clone(),
            requires_npm: tool_def.requires_npm,
            requires_sudo: tool_def.requires_sudo,
        })
    }

    /// Get update command for a specific tool
    pub fn get_update_command(&self, tool: &str) -> Option<InstallCommand> {
        self.registry.tools.get(tool).map(|tool_def| InstallCommand {
            command: tool_def.update.command.clone(),
            args: tool_def.update.args.clone(),
            description: tool_def.description.clone(),
            requires_npm: tool_def.requires_npm,
            requires_sudo: tool_def.requires_sudo,
        })
    }

    /// Get tool definition for detailed information
    pub fn get_tool_definition(&self, tool: &str) -> Option<&ToolDefinition> {
        self.registry.tools.get(tool)
    }

    /// Get display name to config key mapping
    pub fn get_display_name_to_config_mapping(&self) -> HashMap<String, String> {
        self.registry
            .tools
            .iter()
            .map(|(_, tool_def)| (tool_def.display_name.clone(), tool_def.config_key.clone()))
            .collect()
    }

    /// Get CLI command for a tool
    pub fn get_cli_command(&self, tool: &str) -> Option<String> {
        self.registry
            .tools
            .get(tool)
            .map(|tool_def| tool_def.cli_command.clone())
    }

    /// Get authentication information for a tool
    pub fn get_auth_info(&self, tool: &str) -> Option<&AuthDefinition> {
        self.registry.tools.get(tool).map(|tool_def| &tool_def.auth)
    }

    /// Check if a tool requires sudo for installation
    pub fn requires_sudo(&self, tool: &str) -> bool {
        self.registry
            .tools
            .get(tool)
            .map(|tool_def| tool_def.requires_sudo)
            .unwrap_or(false)
    }

    /// Get all tools that require NPM
    pub fn get_npm_tools(&self) -> Vec<String> {
        self.registry
            .tools
            .iter()
            .filter(|(_, tool_def)| tool_def.requires_npm)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Get registry metadata
    pub fn get_metadata(&self) -> &RegistryMetadata {
        &self.registry.metadata
    }
}

impl Default for AiToolsRegistryManager {
    fn default() -> Self {
        Self::new().expect("Failed to load AI Tools Registry")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_loading() {
        let manager = AiToolsRegistryManager::new().expect("Should load registry");
        let tool_names = manager.get_tool_names();
        
        assert!(!tool_names.is_empty());
        assert!(tool_names.contains(&"claude".to_string()));
        assert!(tool_names.contains(&"gemini".to_string()));
        assert!(tool_names.contains(&"qwen".to_string()));
    }

    #[test]
    fn test_install_command_retrieval() {
        let manager = AiToolsRegistryManager::new().expect("Should load registry");
        
        let claude_cmd = manager.get_install_command("claude").expect("Should find Claude");
        assert_eq!(claude_cmd.command, "npm");
        assert!(claude_cmd.args.contains(&"install".to_string()));
        assert!(claude_cmd.args.contains(&"-g".to_string()));
        assert!(claude_cmd.requires_npm);
        assert!(claude_cmd.requires_sudo);
    }

    #[test]
    fn test_display_name_mapping() {
        let manager = AiToolsRegistryManager::new().expect("Should load registry");
        let mapping = manager.get_display_name_to_config_mapping();
        
        assert_eq!(mapping.get("claude"), Some(&"claude-code".to_string()));
        assert_eq!(mapping.get("gemini"), Some(&"gemini-cli".to_string()));
        assert_eq!(mapping.get("qwen"), Some(&"qwen-code".to_string()));
    }

    #[test]
    fn test_auth_info() {
        let manager = AiToolsRegistryManager::new().expect("Should load registry");
        
        let claude_auth = manager.get_auth_info("claude").expect("Should find Claude auth");
        assert!(claude_auth.env_vars.contains(&"ANTHROPIC_API_KEY".to_string()));
        assert!(claude_auth.browser_auth);
        assert!(claude_auth.setup_url.contains("anthropic.com"));
    }

    #[test]
    fn test_sudo_requirements() {
        let manager = AiToolsRegistryManager::new().expect("Should load registry");
        
        // All NPM tools should require sudo in our setup
        assert!(manager.requires_sudo("claude"));
        assert!(manager.requires_sudo("gemini"));
        assert!(manager.requires_sudo("qwen"));
    }
}