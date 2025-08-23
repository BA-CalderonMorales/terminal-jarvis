// src/installation_arguments.rs
//
// Tool installation command management and validation
//
// This module provides a centralized system for managing installation commands
// across all supported AI coding tools, with NPM availability validation.

use std::collections::HashMap;

/// Installation command metadata for AI coding tools
///
/// Contains all information needed to install a specific tool, including
/// the command to run, arguments, user-facing description, and dependency requirements.
///
/// # Fields
///
/// * `command` - The primary command to execute (e.g., "npm", "cargo", "pip")
/// * `args` - Command-line arguments as a vector
/// * `description` - Human-readable description of the tool
/// * `requires_npm` - Whether the installation requires NPM to be available
#[derive(Debug, Clone)]
pub struct InstallCommand {
    pub command: &'static str,
    pub args: Vec<&'static str>,
    pub description: &'static str,
    pub requires_npm: bool,
}

/// Manages installation commands and dependency validation for AI coding tools
///
/// Provides a centralized interface for:
/// - Checking system dependencies (NPM availability)
/// - Retrieving installation commands for specific tools
/// - Validating installation prerequisites
///
/// All installation commands are statically defined to ensure consistency
/// and avoid runtime configuration errors.
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
    pub fn get_tool_names() -> Vec<&'static str> {
        Self::get_install_commands().keys().copied().collect()
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
        Self::get_install_commands().get(tool).cloned()
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
    /// This method rebuilds the HashMap on each call. For single-tool lookups,
    /// prefer [`get_install_command`](Self::get_install_command).
    pub fn get_install_commands() -> HashMap<&'static str, InstallCommand> {
        let mut commands = HashMap::new();

        commands.insert(
            "claude",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@anthropic-ai/claude-code"],
                description: "Anthropic's Claude for code assistance",
                requires_npm: true,
            },
        );

        commands.insert(
            "gemini",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@google/gemini-cli"],
                description: "Google's Gemini CLI tool",
                requires_npm: true,
            },
        );

        commands.insert(
            "qwen",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@qwen-code/qwen-code@latest"],
                description: "Qwen coding assistant",
                requires_npm: true,
            },
        );

        commands.insert(
            "opencode",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "opencode-ai@latest"],
                description: "OpenCode AI coding agent built for the terminal",
                requires_npm: true,
            },
        );

        commands.insert(
            "llxprt",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@vybestack/llxprt-code"],
                description:
                    "LLxprt Code - Multi-provider AI coding assistant with enhanced features",
                requires_npm: true,
            },
        );

        commands.insert(
            "codex",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@openai/codex"],
                description: "OpenAI Codex CLI - AI coding agent that runs locally",
                requires_npm: true,
            },
        );

        commands.insert(
            "crush",
            InstallCommand {
                command: "npm",
                args: vec!["install", "-g", "@charmland/crush"],
                description: "Charm's multi-model AI coding assistant with LSP support",
                requires_npm: true,
            },
        );

        commands
    }
}
