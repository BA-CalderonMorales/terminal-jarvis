// src/installation_arguments.rs
//
// Tool installation command management and validation
//
// This module provides a centralized system for managing installation commands
// across all supported AI coding tools, with NPM availability validation.
// 
// Configuration is now loaded from the modular config system for better maintainability
// and future database integration capabilities.

use crate::tools::tools_command_mapping::{get_install_command, get_update_command};
use crate::tools::tools_config::get_tool_config_loader;
use std::collections::HashMap;

/// Installation command structure for compatibility with existing code
#[derive(Debug, Clone)]
pub struct InstallCommand {
    pub command: String,
    pub args: Vec<String>,
    pub description: String,
    pub requires_npm: bool,
    #[allow(dead_code)]  // Used for installation privilege management
    pub requires_sudo: bool,
}

/// Manages installation commands and dependency validation for AI coding tools
///
/// Provides a centralized interface for:
/// - Checking system dependencies (NPM availability)
/// - Retrieving installation commands for specific tools
/// - Validating installation prerequisites
///
/// All installation commands are loaded from ai-tools-registry.toml to ensure consistency
/// and enable easy maintenance without code changes.
pub struct InstallationManager;

impl InstallationManager {
    /// Checks if NPM is available on the system
    ///
    /// Executes `npm --version` to verify NPM installation and accessibility.
    /// Used to validate prerequisites before attempting NPM-based tool installations.
    ///
    /// # Returns
    ///
    /// * `true` - NPM is installed and accessible
    /// * `false` - NPM is not available or execution failed
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if InstallationManager::check_npm_available() {
    ///     println!("NPM is available for tool installation");
    /// } else {
    ///     println!("NPM is required but not found");
    /// }
    /// ```
    pub fn check_npm_available() -> bool {
        std::process::Command::new("npm")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Returns a list of all supported tool names
    ///
    /// Provides the canonical list of tool names that can be used with
    /// [`get_install_command`](Self::get_install_command).
    ///
    /// # Returns
    ///
    /// A vector of tool name strings (e.g., ["claude", "gemini", "qwen"])
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let tools = InstallationManager::get_tool_names();
    /// for tool in tools {
    ///     println!("Supported tool: {}", tool);
    /// }
    /// ```
    pub fn get_tool_names() -> Vec<String> {
        let config_loader = get_tool_config_loader();
        config_loader.get_tool_names()
    }

    /// Retrieves installation command for a specific tool
    ///
    /// # Arguments
    ///
    /// * `tool` - The name of the tool (e.g., "claude", "gemini")
    ///
    /// # Returns
    ///
    /// * `Some(InstallCommand)` - Installation command metadata if tool is supported
    /// * `None` - If the tool name is not recognized
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Some(cmd) = InstallationManager::get_install_command("claude") {
    ///     println!("Install command: {} {}", cmd.command, cmd.args.join(" "));
    /// } else {
    ///     println!("Tool not found");
    /// }
    /// ```
    pub fn get_install_command(tool: &str) -> Option<InstallCommand> {
        get_install_command(tool).map(|cmd| InstallCommand {
            command: cmd.command,
            args: cmd.args,
            description: cmd.description,
            requires_npm: cmd.requires_npm,
            requires_sudo: cmd.requires_sudo,
        })
    }

    /// Retrieves update command for a specific tool
    ///
    /// # Arguments
    ///
    /// * `tool` - The name of the tool (e.g., "claude", "gemini")
    ///
    /// # Returns
    ///
    /// * `Some(InstallCommand)` - Update command metadata if tool is supported
    /// * `None` - If the tool name is not recognized
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// if let Some(cmd) = InstallationManager::get_update_command("claude") {
    ///     println!("Update command: {} {}", cmd.command, cmd.args.join(" "));
    /// } else {
    ///     println!("Tool not found");
    /// }
    /// ```
    #[allow(dead_code)]  // Used by update functionality
    pub fn get_update_command(tool: &str) -> Option<InstallCommand> {
        get_update_command(tool).map(|cmd| InstallCommand {
            command: cmd.command,
            args: cmd.args,
            description: cmd.description,
            requires_npm: cmd.requires_npm,
            requires_sudo: cmd.requires_sudo,
        })
    }

    /// Returns all available installation commands
    ///
    /// Provides the complete mapping of tool names to their installation commands.
    /// This is the authoritative source for all supported tools and their metadata.
    ///
    /// # Returns
    ///
    /// A HashMap mapping tool names to their InstallCommand structures
    ///
    /// # Note
    ///
    /// This method loads from the TOML registry on each call. For single-tool lookups,
    /// prefer [`get_install_command`](Self::get_install_command).
    pub fn get_install_commands() -> HashMap<String, InstallCommand> {
        let config_loader = get_tool_config_loader();
        let tool_names = config_loader.get_tool_names();
        let mut commands = HashMap::new();
        
        for tool in tool_names {
            if let Some(cmd) = get_install_command(&tool) {
                commands.insert(tool, InstallCommand {
                    command: cmd.command,
                    args: cmd.args,
                    description: cmd.description,
                    requires_npm: cmd.requires_npm,
                    requires_sudo: cmd.requires_sudo,
                });
            }
        }
        
        commands
    }
}
