// Tools Command Mapping Domain
// Handles tool name resolution, command mapping, and installation commands

use std::collections::HashMap;

use super::tools_config::{get_tool_config_loader, AuthDefinition};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ToolCommand {
    pub command: &'static str,
    pub description: &'static str,
}

/// Installation command structure for compatibility with existing code
#[derive(Debug, Clone)]
pub struct InstallCommandInfo {
    pub command: String,
    pub args: Vec<String>,
    pub pipe_to: Option<String>, // For curl-based installations that pipe to bash
    pub description: String,
    pub requires_npm: bool,
    pub requires_sudo: bool,
}

/// Map display names to actual CLI command names
pub fn get_command_mapping() -> HashMap<&'static str, &'static str> {
    let mut mapping = HashMap::new();
    // Map display names to actual CLI commands (no API key enforcement)
    mapping.insert("claude", "claude"); // Assuming claude-code installs as 'claude'
    mapping.insert("gemini", "gemini"); // Assuming gemini-cli installs as 'gemini'
    mapping.insert("qwen", "qwen"); // Assuming qwen-code installs as 'qwen'
    mapping.insert("opencode", "opencode"); // OpenCode installs as 'opencode'
    mapping.insert("llxprt", "llxprt"); // LLxprt Code installs as 'llxprt'
    mapping.insert("codex", "codex"); // OpenAI Codex CLI installs as 'codex'
    mapping.insert("crush", "crush"); // Crush installs as 'crush'
    mapping.insert("goose", "goose"); // Block Goose CLI installs as 'goose'
    mapping.insert("amp", "amp"); // Sourcegraph Amp installs as 'amp'
    mapping.insert("aider", "aider"); // Aider installs as 'aider'
    mapping.insert("copilot", "copilot"); // GitHub Copilot CLI installs as 'copilot'
    mapping
}

/// Get actual CLI command from display name
pub fn get_cli_command(display_name: &str) -> &str {
    get_command_mapping()
        .get(display_name)
        .unwrap_or(&display_name)
}

/// Get installation command for a tool from configuration
pub fn get_install_command(tool_name: &str) -> Option<InstallCommandInfo> {
    let config_loader = get_tool_config_loader();

    if let Some(tool_def) = config_loader.get_tool_definition(tool_name) {
        if let Some(install_cmd) = config_loader.get_install_command(tool_name) {
            return Some(InstallCommandInfo {
                command: install_cmd.command.clone(),
                args: install_cmd.args.clone(),
                pipe_to: install_cmd.pipe_to.clone(),
                description: tool_def.description.clone(),
                requires_npm: tool_def.requires_npm,
                requires_sudo: tool_def.requires_sudo,
            });
        }
    }

    None
}

/// Get update command for a tool from configuration
#[allow(dead_code)] // Used by future update functionality
pub fn get_update_command(tool_name: &str) -> Option<InstallCommandInfo> {
    let config_loader = get_tool_config_loader();

    if let Some(tool_def) = config_loader.get_tool_definition(tool_name) {
        if let Some(update_cmd) = config_loader.get_update_command(tool_name) {
            return Some(InstallCommandInfo {
                command: update_cmd.command.clone(),
                args: update_cmd.args.clone(),
                pipe_to: update_cmd.pipe_to.clone(),
                description: tool_def.description.clone(),
                requires_npm: tool_def.requires_npm,
                requires_sudo: tool_def.requires_sudo,
            });
        }
    }

    None
}

/// Get authentication information for a tool
#[allow(dead_code)] // Used for auth management functionality
pub fn get_auth_info(tool_name: &str) -> Option<&AuthDefinition> {
    let config_loader = get_tool_config_loader();
    config_loader.get_auth_info(tool_name)
}

/// Get display name to config key mapping for compatibility
#[allow(dead_code)] // Used by services for configuration mapping
pub fn get_display_name_to_config_mapping() -> HashMap<String, String> {
    let config_loader = get_tool_config_loader();
    config_loader.get_display_name_to_config_mapping()
}

/// Check if a tool requires sudo for installation
#[allow(dead_code)] // Used for installation privilege management
pub fn requires_sudo(tool_name: &str) -> bool {
    let config_loader = get_tool_config_loader();
    config_loader.requires_sudo(tool_name)
}

/// Get all tools that require NPM
#[allow(dead_code)] // Used for NPM dependency validation
pub fn get_npm_tools() -> Vec<String> {
    let config_loader = get_tool_config_loader();
    config_loader.get_npm_tools()
}

/// Get all available tools as ToolCommand structs
#[allow(dead_code)]
pub fn get_all_tools() -> Vec<ToolCommand> {
    let config_loader = get_tool_config_loader();
    let tool_names = config_loader.get_tool_names();

    tool_names
        .iter()
        .filter_map(|name| {
            config_loader
                .get_tool_definition(name)
                .map(|tool_def| ToolCommand {
                    command: &tool_def.cli_command,
                    description: &tool_def.description,
                })
        })
        .collect()
}

/// Get tool information by name
#[allow(dead_code)]
pub fn get_tool_info(tool_name: &str) -> Option<ToolCommand> {
    let config_loader = get_tool_config_loader();
    config_loader
        .get_tool_definition(tool_name)
        .map(|tool_def| ToolCommand {
            command: &tool_def.cli_command,
            description: &tool_def.description,
        })
}
