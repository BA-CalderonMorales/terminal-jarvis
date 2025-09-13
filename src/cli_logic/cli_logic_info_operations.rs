use crate::installation_arguments::InstallationManager;
use crate::theme::theme_global_config;
use crate::tools::ToolManager;
use anyhow::{anyhow, Result};

/// Handle displaying detailed information about a specific tool
pub async fn handle_tool_info(tool: &str) -> Result<()> {
    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();

    let tool_info = tools
        .get(tool)
        .ok_or_else(|| anyhow!("Tool '{}' not found", tool))?;
    let install_info = install_commands
        .get(tool)
        .ok_or_else(|| anyhow!("Installation info for '{}' not found", tool))?;

    display_tool_info_formatted(tool, tool_info, install_info);

    Ok(())
}

/// Display tool information in a formatted, themed layout
fn display_tool_info_formatted(
    tool: &str,
    tool_info: &crate::tools::ToolInfo,
    install_info: &crate::ai_tools_registry::InstallCommand,
) {
    let theme = theme_global_config::current_theme();

    println!(
        "{}",
        theme.primary(&format!(
            "┌─ Tool Information: {} ─────────────────────────────────────┐",
            tool
        ))
    );
    println!(
        "{}",
        theme.primary("│                                                             │")
    );

    println!(
        "│ {:<59} │",
        theme.secondary(&format!("Description: {}", install_info.description))
    );
    println!(
        "│ {:<59} │",
        theme.secondary(&format!("Command: {}", tool_info.command))
    );

    let status_text = if tool_info.is_installed {
        theme.primary("Installed ✓")
    } else {
        theme.accent("Not installed ✗")
    };
    println!("│ Status: {:<50} │", status_text);

    println!(
        "│ {:<59} │",
        theme.secondary(&format!(
            "Installation: {} {}",
            install_info.command,
            install_info.args.join(" ")
        ))
    );

    if install_info.requires_npm {
        let npm_status = if InstallationManager::check_npm_available() {
            theme.primary("Available ✓")
        } else {
            theme.accent("Not available ✗")
        };
        println!("│ NPM Required: {:<46} │", npm_status);
    }

    println!(
        "{}",
        theme.primary("│                                                             │")
    );
    println!(
        "{}",
        theme.primary("└─────────────────────────────────────────────────────────────┘")
    );
}
