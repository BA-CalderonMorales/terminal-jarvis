// Tools Configuration Domain
// Handles loading tool configurations from modular config files and user preferences

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Complete tool definition loaded from config files
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
    pub status: String,
    pub install: InstallCommand,
    pub update: InstallCommand,
    pub auth: AuthDefinition,
    pub features: Option<ToolFeatures>,
}

/// Installation/update command definition
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstallCommand {
    pub command: String,
    pub args: Vec<String>,
    pub verify_command: Option<String>,
    pub post_install_message: Option<String>,
}

/// Authentication configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthDefinition {
    pub env_vars: Vec<String>,
    pub setup_url: String,
    pub browser_auth: bool,
    pub auth_instructions: Option<String>,
}

/// Tool feature capabilities
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolFeatures {
    pub supports_files: bool,
    pub supports_streaming: bool,
    pub supports_conversation: bool,
    pub max_context_tokens: Option<u64>,
    pub supported_languages: Vec<String>,
}

/// User preferences for individual tools
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolPreferences {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub auto_update: bool,
}

fn default_true() -> bool { true }

/// Configuration loader for tools
pub struct ToolConfigLoader {
    /// All discovered tool definitions
    tool_definitions: HashMap<String, ToolDefinition>,
    /// User preferences from terminal-jarvis.toml
    user_preferences: HashMap<String, ToolPreferences>,
}

impl ToolConfigLoader {
    /// Create new config loader and discover all tools
    pub fn new() -> Self {
        let mut loader = Self {
            tool_definitions: HashMap::new(),
            user_preferences: HashMap::new(),
        };

        // Auto-discover tools from config/tools/ directory
        if let Err(e) = loader.load_builtin_tools() {
            eprintln!("Warning: Failed to load tool configurations: {}", e);
        }

        // Load user preferences if available
        if let Err(e) = loader.load_user_preferences() {
            eprintln!("Warning: Failed to load user preferences: {}", e);
        }

        loader
    }

    /// Auto-discover and load tools from config/tools/ directory
    fn load_builtin_tools(&mut self) -> Result<()> {
        let config_dirs = vec![
            std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|p| p.join("../config/tools"))),
            Some(PathBuf::from("./config/tools")),
            Some(PathBuf::from("../config/tools")),
        ];

        for config_dir in config_dirs.into_iter().flatten() {
            if config_dir.exists() && config_dir.is_dir() {
                if let Ok(entries) = std::fs::read_dir(&config_dir) {
                    for entry in entries.flatten() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if file_name.ends_with(".toml") {
                                let tool_name = file_name.trim_end_matches(".toml");
                                if let Ok(tool_config) = self.load_tool_config(&entry.path()) {
                                    self.tool_definitions.insert(tool_name.to_string(), tool_config);
                                }
                            }
                        }
                    }
                }
                break; // Use the first config directory found
            }
        }

        Ok(())
    }

    /// Load individual tool configuration from TOML file
    fn load_tool_config(&self, path: &PathBuf) -> Result<ToolDefinition> {
        let content = std::fs::read_to_string(path)?;
        
        // Parse the tool TOML file
        #[derive(Deserialize)]
        struct ToolFile {
            tool: ToolDefinition,
        }
        
        let tool_file: ToolFile = toml::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse tool config {}: {}", path.display(), e))?;

        Ok(tool_file.tool)
    }

    /// Load user preferences from terminal-jarvis.toml files
    fn load_user_preferences(&mut self) -> Result<()> {
        let config_paths = vec![
            dirs::config_dir().map(|p| p.join("terminal-jarvis").join("config.toml")),
            Some(PathBuf::from("./terminal-jarvis.toml")),
        ];

        for path in config_paths.into_iter().flatten() {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(user_config) = toml::from_str::<UserConfigFile>(&content) {
                        if let Some(prefs) = user_config.preferences {
                            if let Some(tools) = prefs.tools {
                                self.user_preferences.extend(tools);
                            }
                        }
                        break; // Use the first config file found
                    }
                }
            }
        }

        Ok(())
    }

    /// Get tool definition by name
    pub fn get_tool_definition(&self, tool_name: &str) -> Option<&ToolDefinition> {
        self.tool_definitions.get(tool_name)
    }

    /// Get all tool names that have definitions
    pub fn get_tool_names(&self) -> Vec<String> {
        self.tool_definitions.keys().cloned().collect()
    }

    /// Check if tool is enabled by user preferences
    #[allow(dead_code)]  // Used for configuration management
    pub fn is_tool_enabled(&self, tool_name: &str) -> bool {
        if let Some(tool_def) = self.tool_definitions.get(tool_name) {
            if let Some(prefs) = self.user_preferences.get(&tool_def.config_key) {
                return prefs.enabled;
            }
        }
        true // Default to enabled
    }

    /// Get install command for tool
    pub fn get_install_command(&self, tool_name: &str) -> Option<&InstallCommand> {
        self.tool_definitions.get(tool_name).map(|t| &t.install)
    }

    /// Get update command for tool
    #[allow(dead_code)]  // Used for update functionality
    pub fn get_update_command(&self, tool_name: &str) -> Option<&InstallCommand> {
        self.tool_definitions.get(tool_name).map(|t| &t.update)
    }

    /// Get authentication info for tool
    #[allow(dead_code)]  // Used for auth management
    pub fn get_auth_info(&self, tool_name: &str) -> Option<&AuthDefinition> {
        self.tool_definitions.get(tool_name).map(|t| &t.auth)
    }

    /// Get display name to config key mapping (for compatibility)
    #[allow(dead_code)]  // Used for service mapping  
    pub fn get_display_name_to_config_mapping(&self) -> HashMap<String, String> {
        self.tool_definitions
            .iter()
            .map(|(_, tool_def)| (tool_def.display_name.clone(), tool_def.config_key.clone()))
            .collect()
    }

    /// Check if tool requires sudo
    #[allow(dead_code)]  // Used for installation privilege checking
    pub fn requires_sudo(&self, tool_name: &str) -> bool {
        self.tool_definitions
            .get(tool_name)
            .map(|t| t.requires_sudo)
            .unwrap_or(false)
    }

    /// Get tools that require NPM
    #[allow(dead_code)]  // Used for NPM dependency validation
    pub fn get_npm_tools(&self) -> Vec<String> {
        self.tool_definitions
            .iter()
            .filter(|(_, tool_def)| tool_def.requires_npm)
            .map(|(name, _)| name.clone())
            .collect()
    }
}

/// Simplified user config file structure for reading preferences
#[derive(Debug, Deserialize)]
struct UserConfigFile {
    preferences: Option<UserPreferencesFile>,
}

#[derive(Debug, Deserialize)]
struct UserPreferencesFile {
    tools: Option<HashMap<String, ToolPreferences>>,
}

/// Global tool config loader instance using safe singleton pattern
static TOOL_CONFIG_LOADER: std::sync::OnceLock<ToolConfigLoader> = std::sync::OnceLock::new();

/// Get global tool config loader (singleton pattern)
pub fn get_tool_config_loader() -> &'static ToolConfigLoader {
    TOOL_CONFIG_LOADER.get_or_init(|| ToolConfigLoader::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_config_loader_creation() {
        let loader = ToolConfigLoader::new();
        // Should not panic and should be able to get tool names
        let _tool_names = loader.get_tool_names();
    }

    #[test]
    fn test_global_config_loader() {
        let loader = get_tool_config_loader();
        let _tool_names = loader.get_tool_names();
    }
}