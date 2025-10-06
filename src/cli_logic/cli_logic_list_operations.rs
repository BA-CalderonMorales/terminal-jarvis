//! Tool List Operations
//!
//! Handles listing all available AI coding tools with their installation status.
//! Uses the ToolListViewModel for MVVM-inspired presentation logic.

use crate::presentation::models::Tool;
use crate::tools::tools_config::get_tool_config_loader;
use crate::tools::tools_detection::get_available_tools;
use crate::presentation::view_models::ToolListViewModel;
use anyhow::Result;

/// Handle listing all available AI coding tools with their status
pub async fn handle_list_tools() -> Result<()> {
    // Get tools from the new model system
    let config_loader = get_tool_config_loader();
    let available_tools = get_available_tools();

    let mut tools = Vec::new();
    for (tool_name, tool_info) in available_tools {
        if let Some(tool_def) = config_loader.get_tool_definition(tool_name) {
            let tool =
                Tool::from_tool_definition(tool_name.to_string(), tool_def, tool_info.is_installed);
            tools.push(tool);
        }
    }

    // Use the view model for presentation
    let view_model = ToolListViewModel::new(tools);
    view_model.display_list();

    Ok(())
}
