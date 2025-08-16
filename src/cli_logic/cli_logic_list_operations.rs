use crate::installation_arguments::InstallationManager;
use crate::theme::theme_global_config;
use crate::tools::ToolManager;
use anyhow::Result;

/// Handle listing all available AI coding tools with their status
pub async fn handle_list_tools() -> Result<()> {
    println!("Available AI Coding Tools:\n");

    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();

    for (tool_name, tool_info) in tools.iter() {
        let install_info = install_commands.get(tool_name).unwrap();
        let status = if tool_info.is_installed {
            "Installed"
        } else {
            "Not installed"
        };

        println!(" {} - {}", tool_name, install_info.description);
        println!("  Status: {status}");
        println!("  Command: {}", tool_info.command);
        if install_info.requires_npm {
            println!("  Requires: NPM");
        }
        println!();
    }

    show_system_requirements_advisory();

    Ok(())
}

/// Display system requirements and installation advisory
fn show_system_requirements_advisory() {
    if !InstallationManager::check_npm_available() {
        let theme = theme_global_config::current_theme();
        println!(
            "{} {}",
            theme.secondary("⚠ ADVISORY:"),
            theme.primary("Node.js ecosystem not detected")
        );
        println!("  Most AI tools are distributed via NPM. Install from: https://nodejs.org/");
    }
}
