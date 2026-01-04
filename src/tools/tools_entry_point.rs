// Tools Entry Point Domain
// Main coordination for all tool management functionality

use anyhow::Result;
use std::collections::BTreeMap;

// Import all domain functions
pub use super::tools_command_mapping::{
    get_all_tools, get_cli_command, get_tool_info, ToolCommand,
};
pub use super::tools_db_bridge::{
    get_available_tools_hybrid, get_cli_command_hybrid, get_tool_hybrid, is_db_initialized,
};
pub use super::tools_detection::{
    check_tool_installed, get_available_tools, get_installed_tools, get_uninstalled_tools, ToolInfo,
};
pub use super::tools_execution_engine::run_tool;

/// Main ToolManager struct providing the public API
pub struct ToolManager;

impl ToolManager {
    /// Get actual CLI command from display name
    pub fn get_cli_command(display_name: &str) -> &str {
        get_cli_command(display_name)
    }

    /// Get all available tools as ToolCommand structs
    #[allow(dead_code)]
    pub fn get_all_tools() -> Vec<ToolCommand> {
        get_all_tools()
    }

    /// Get all available tools with their installation status (sync version - uses TOML)
    pub fn get_available_tools() -> BTreeMap<&'static str, ToolInfo> {
        get_available_tools()
    }

    /// Get all available tools with installation status (async version - uses DB with TOML fallback)
    ///
    /// This is the preferred method for new code. It checks the database first,
    /// and falls back to TOML configuration if the database hasn't been initialized.
    pub async fn get_available_tools_async() -> BTreeMap<String, ToolInfo> {
        get_available_tools_hybrid().await
    }

    /// Check if the database has been initialized with tools
    pub async fn is_db_mode() -> bool {
        is_db_initialized().await
    }

    /// Check if a tool is installed by trying to run it
    pub fn check_tool_installed(tool: &str) -> bool {
        check_tool_installed(tool)
    }

    /// Run a tool with arguments - automatically handles session continuation for internal commands
    pub async fn run_tool(display_name: &str, args: &[String]) -> Result<()> {
        run_tool(display_name, args).await
    }

    /// Get list of installed tools (display names)
    pub fn get_installed_tools() -> Vec<&'static str> {
        get_installed_tools()
    }

    /// Get list of uninstalled tools (display names)
    pub fn get_uninstalled_tools() -> Vec<&'static str> {
        get_uninstalled_tools()
    }

    /// Get tool information by name
    #[allow(dead_code)]
    pub fn get_tool_info(tool_name: &str) -> Option<ToolCommand> {
        get_tool_info(tool_name)
    }

    /// Get CLI command for a tool (async version with DB support)
    pub async fn get_cli_command_async(tool_name: &str) -> Option<String> {
        get_cli_command_hybrid(tool_name).await
    }
}
