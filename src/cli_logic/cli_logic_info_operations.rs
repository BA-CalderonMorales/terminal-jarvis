//! Tool Information Operations
//!
//! Handles displaying detailed information about specific AI coding tools.
//! Uses the unified ToolDisplayFormatter for consistent formatting across all tools.

use crate::installation_arguments::InstallationManager;
use crate::tools::{
    tools_display::{ToolDisplayFormatter, ToolDisplayMode},
    ToolManager,
};
use anyhow::{anyhow, Result};

/// Handle displaying detailed information about a specific tool
pub async fn handle_tool_info(tool: &str) -> Result<()> {
    // Use async version that checks database first
    let tools = ToolManager::get_available_tools_async().await;
    let install_commands = InstallationManager::get_install_commands();

    let tool_info = tools
        .get(tool)
        .ok_or_else(|| anyhow!("Tool '{tool}' not found"))?;
    let install_info = install_commands
        .get(tool)
        .ok_or_else(|| anyhow!("Installation info for '{tool}' not found"))?;

    ToolDisplayFormatter::display_tool_info(
        tool,
        tool_info,
        install_info,
        ToolDisplayMode::Detailed,
    );

    Ok(())
}
