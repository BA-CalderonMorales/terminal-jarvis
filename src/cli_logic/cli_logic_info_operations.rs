use crate::installation_arguments::InstallationManager;
use crate::theme_config;
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
    install_info: &crate::installation_arguments::InstallCommand,
) {
    let theme = theme_config::current_theme();

    println!("{}", theme.primary(&format!("┌─ Tool Information: {} ─────────────────────────────────────┐", tool)));
    println!("{}", theme.primary("│                                                             │"));
    
    println!("│ {:<59} │", theme.secondary(&format!("Description: {}", install_info.description)));
    println!("│ {:<59} │", theme.secondary(&format!("Command: {}", tool_info.command)));
    
    let status_text = if tool_info.is_installed {
        theme.primary("Installed ✓")
    } else {
        theme.accent("Not installed ✗")
    };
    println!("│ Status: {:<50} │", status_text);
    
    println!("│ {:<59} │", theme.secondary(&format!("Installation: {} {}", install_info.command, install_info.args.join(" "))));

    if install_info.requires_npm {
        let npm_status = if InstallationManager::check_npm_available() {
            theme.primary("Available ✓")
        } else {
            theme.accent("Not available ✗")
        };
        println!("│ NPM Required: {:<46} │", npm_status);
    }

    println!("{}", theme.primary("│                                                             │"));
    println!("{}", theme.primary("└─────────────────────────────────────────────────────────────┘"));
}

/// Display comprehensive system information
pub async fn display_system_info() -> Result<()> {
    let theme = theme_config::current_theme();

    println!("{}", theme.primary("┌─ System Information ───────────────────────────────────────┐"));
    println!("{}", theme.primary("│                                                             │"));

    // Terminal Jarvis version
    println!("│ {:<59} │", theme.secondary(&format!("Terminal Jarvis: v{}", env!("CARGO_PKG_VERSION"))));

    // Node.js/NPM status
    let npm_status = if InstallationManager::check_npm_available() {
        theme.primary("Available ✓")
    } else {
        theme.accent("Not available ✗")
    };
    println!("│ NPM Status: {:<46} │", npm_status);

    // Tool count summary
    let tools = ToolManager::get_available_tools();
    let installed_count = tools.values().filter(|t| t.is_installed).count();
    let total_count = tools.len();
    
    println!("│ {:<59} │", theme.secondary(&format!("Tools Available: {}", total_count)));
    println!("│ {:<59} │", theme.secondary(&format!("Tools Installed: {}", installed_count)));

    // Current theme
    let current_theme = theme_config::current_theme();
    println!("│ {:<59} │", theme.secondary(&format!("Active Theme: {}", current_theme.name)));

    println!("{}", theme.primary("│                                                             │"));
    println!("{}", theme.primary("└─────────────────────────────────────────────────────────────┘"));

    Ok(())
}

/// Display tool usage statistics and recommendations
pub async fn display_tool_recommendations() -> Result<()> {
    let theme = theme_config::current_theme();

    println!("{}", theme.primary("Tool Recommendations:"));
    println!();

    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();

    // Recommend popular/essential tools
    let recommended_tools = ["claude", "opencode", "llxprt"];
    
    for tool_name in &recommended_tools {
        if let (Some(tool_info), Some(install_info)) = (tools.get(*tool_name), install_commands.get(*tool_name)) {
            let status_icon = if tool_info.is_installed { "✓" } else { "○" };
            println!(" {} {} - {}", status_icon, tool_name, install_info.description);
        }
    }

    println!();
    println!("{}", theme.secondary("○ = Recommended for installation"));
    println!("{}", theme.secondary("✓ = Already installed"));

    Ok(())
}
