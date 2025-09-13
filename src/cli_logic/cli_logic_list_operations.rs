//! Tool List Operations
//! 
//! Handles listing all available AI coding tools with their installation status.
//! Uses the unified ToolDisplayFormatter for consistent formatting across all tools.

use crate::installation_arguments::InstallationManager;
use crate::tools::{ToolManager, tools_display::ToolDisplayFormatter};
use anyhow::Result;

/// Handle listing all available AI coding tools with their status
pub async fn handle_list_tools() -> Result<()> {
    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();

    // Create iterator that combines tool info with install commands
    let tools_iter = tools.iter().map(|(tool_name, tool_info)| {
        let install_info = install_commands.get(*tool_name).unwrap();
        (*tool_name, tool_info, install_info)
    });

    ToolDisplayFormatter::display_tool_list(tools_iter);
    ToolDisplayFormatter::show_system_requirements_advisory();

    Ok(())
}
