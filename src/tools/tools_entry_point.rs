// Tools Entry Point Domain
// Main coordination for all tool management functionality

use anyhow::Result;
use std::collections::BTreeMap;

// Import all domain functions
pub use super::tools_command_mapping::{
    get_all_tools, get_cli_command, get_tool_info, ToolCommand,
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

    /// Get all available tools with their installation status
    pub fn get_available_tools() -> BTreeMap<&'static str, ToolInfo> {
        get_available_tools()
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
}
