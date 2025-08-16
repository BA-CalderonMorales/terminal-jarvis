use crate::installation_arguments::InstallationManager;
use crate::theme_config;
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
        let theme = theme_config::current_theme();
        println!(
            "{} {}",
            theme.secondary("⚠ ADVISORY:"),
            theme.primary("Node.js ecosystem not detected")
        );
        println!("  Most AI tools are distributed via NPM. Install from: https://nodejs.org/");
    }
}

/// List only installed tools
pub async fn list_installed_tools() -> Result<()> {
    let theme = theme_config::current_theme();
    println!("{}", theme.primary("Installed AI Coding Tools:\n"));

    let tools = ToolManager::get_available_tools();
    let mut installed_count = 0;

    for (tool_name, tool_info) in tools.iter() {
        if tool_info.is_installed {
            println!(" ✓ {} ({})", tool_name, tool_info.command);
            installed_count += 1;
        }
    }

    if installed_count == 0 {
        println!("{}", theme.secondary("  No AI coding tools are currently installed."));
        println!("  Use 'terminal-jarvis install <tool>' to install tools.");
    } else {
        println!("\n{} tools installed.", installed_count);
    }

    Ok(())
}

/// List only uninstalled tools  
pub async fn list_uninstalled_tools() -> Result<()> {
    let theme = theme_config::current_theme();
    println!("{}", theme.primary("Available AI Coding Tools (Not Installed):\n"));

    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();
    let mut uninstalled_count = 0;

    for (tool_name, tool_info) in tools.iter() {
        if !tool_info.is_installed {
            let install_info = install_commands.get(tool_name).unwrap();
            println!(" {} - {}", tool_name, install_info.description);
            uninstalled_count += 1;
        }
    }

    if uninstalled_count == 0 {
        println!("{}", theme.secondary("  All available AI coding tools are already installed!"));
    } else {
        println!("\n{} tools available for installation.", uninstalled_count);
        println!("Use 'terminal-jarvis install <tool>' to install any of these tools.");
    }

    Ok(())
}

/// Display tools in a formatted table-like structure
pub async fn list_tools_formatted() -> Result<()> {
    let theme = theme_config::current_theme();
    
    println!("{}", theme.primary("┌─────────────────────────────────────────────────────────────┐"));
    println!("{}", theme.primary("│                    AI Coding Tools Status                  │"));
    println!("{}", theme.primary("├─────────────────────────────────────────────────────────────┤"));

    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();

    for (tool_name, tool_info) in tools.iter() {
        let install_info = install_commands.get(tool_name).unwrap();
        let status_icon = if tool_info.is_installed { "✓" } else { "✗" };
        let status_color = if tool_info.is_installed {
            theme.primary(&format!("{} {}", status_icon, tool_name))
        } else {
            theme.secondary(&format!("{} {}", status_icon, tool_name))
        };
        
        println!("│ {} │", format!("{:<55}", status_color));
        println!("│   {} │", format!("{:<53}", install_info.description));
    }

    println!("{}", theme.primary("└─────────────────────────────────────────────────────────────┘"));

    show_system_requirements_advisory();
    
    Ok(())
}
