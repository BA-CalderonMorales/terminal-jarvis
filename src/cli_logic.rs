use crate::installation_arguments::InstallationManager;
use crate::services::{GitHubService, PackageService};
use crate::tools::ToolManager;
use anyhow::{anyhow, Result};
use inquire::{Confirm, MultiSelect, Select, Text};
use std::process::Command;

pub async fn handle_run_tool(tool: &str, args: &[String]) -> Result<()> {
    // Check if NPM is available first
    if !InstallationManager::check_npm_available() {
        println!("⚠️  Warning: NPM is not installed or not in PATH.");
        println!("   Most AI coding tools require NPM for installation.");
        println!("   Please install Node.js and NPM first: https://nodejs.org/");
        return Err(anyhow!("NPM is required but not available"));
    }

    // Check if tool is installed
    let cli_command = ToolManager::get_cli_command(tool);
    if !ToolManager::check_tool_installed(cli_command) {
        println!("❌ Tool '{tool}' is not installed.");
        let should_install = Confirm::new(&format!("📦 Install '{tool}' now?"))
            .with_default(true)
            .prompt()?;

        if should_install {
            handle_install_tool(tool).await?;
            println!("✅ Installation complete!\n");
        } else {
            return Err(anyhow!("Tool '{}' is required but not installed", tool));
        }
    }

    ToolManager::run_tool(tool, args).await
}

pub async fn handle_install_tool(tool: &str) -> Result<()> {
    let install_cmd = InstallationManager::get_install_command(tool)
        .ok_or_else(|| anyhow!("Tool '{}' not found in installation registry", tool))?;

    // Check NPM availability
    if install_cmd.requires_npm && !InstallationManager::check_npm_available() {
        println!("❌ NPM is not installed or not in PATH.");
        println!("   Please install Node.js and NPM first: https://nodejs.org/");
        return Err(anyhow!(
            "NPM is required to install {} but is not available",
            tool
        ));
    }

    println!(
        "📦 Installing {} using: {} {}",
        tool,
        install_cmd.command,
        install_cmd.args.join(" ")
    );

    let mut cmd = Command::new(install_cmd.command);
    cmd.args(&install_cmd.args);

    let status = cmd.status()?;

    if status.success() {
        println!("✅ {tool} installed successfully!");
        Ok(())
    } else {
        Err(anyhow!("Failed to install {}", tool))
    }
}

pub async fn handle_update_packages(package: Option<&str>) -> Result<()> {
    let package_service = PackageService::new()?;

    match package {
        Some(pkg) => {
            println!("Updating package: {pkg}");
            package_service.update_tool(pkg).await
        }
        None => {
            println!("Updating all packages...");
            let tools = InstallationManager::get_tool_names();
            for tool in tools {
                println!("Updating {tool}...");
                if let Err(e) = package_service.update_tool(tool).await {
                    eprintln!("Failed to update {tool}: {e}");
                }
            }
            Ok(())
        }
    }
}

pub async fn handle_list_tools() -> Result<()> {
    println!("🤖 Available AI Coding Tools:\n");

    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();

    for (tool_name, tool_info) in tools.iter() {
        let install_info = install_commands.get(tool_name).unwrap();
        let status = if tool_info.is_installed {
            "✅ Installed"
        } else {
            "📦 Not installed"
        };

        println!("  {} - {}", tool_name, install_info.description);
        println!("    Status: {status}");
        println!("    Command: {}", tool_info.command);
        if install_info.requires_npm {
            println!("    Requires: NPM");
        }
        println!();
    }

    if !InstallationManager::check_npm_available() {
        println!("⚠️  Warning: NPM is not available. Most tools require NPM for installation.");
        println!("   Install Node.js and NPM from: https://nodejs.org/");
    }

    Ok(())
}

pub async fn handle_tool_info(tool: &str) -> Result<()> {
    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();

    let tool_info = tools
        .get(tool)
        .ok_or_else(|| anyhow!("Tool '{}' not found", tool))?;
    let install_info = install_commands
        .get(tool)
        .ok_or_else(|| anyhow!("Installation info for '{}' not found", tool))?;

    println!("🔍 Tool Information: {tool}\n");
    println!("Description: {}", install_info.description);
    println!("Command: {}", tool_info.command);
    println!(
        "Status: {}",
        if tool_info.is_installed {
            "✅ Installed"
        } else {
            "📦 Not installed"
        }
    );
    println!(
        "Installation: {} {}",
        install_info.command,
        install_info.args.join(" ")
    );

    if install_info.requires_npm {
        let npm_status = if InstallationManager::check_npm_available() {
            "✅ Available"
        } else {
            "❌ Not available"
        };
        println!("NPM Required: Yes ({npm_status})");
    }

    Ok(())
}

pub async fn handle_templates_init() -> Result<()> {
    let github_service = GitHubService::new()?;

    println!("Initializing template repository...");
    println!("This requires gh CLI and will create a new GitHub repository for your templates.");

    github_service.init_template_repository().await
}

pub async fn handle_templates_create(name: &str) -> Result<()> {
    let github_service = GitHubService::new()?;

    println!("Creating template: {name}");
    github_service.create_template(name).await
}

pub async fn handle_templates_list() -> Result<()> {
    let github_service = GitHubService::new()?;

    println!("Available templates:");
    let templates = github_service.list_templates().await?;

    if templates.is_empty() {
        println!("  No templates found. Use 'terminal-jarvis templates create <name>' to create a template.");
    } else {
        for template in templates {
            println!("  - {template}");
        }
    }

    Ok(())
}

pub async fn handle_templates_apply(name: &str) -> Result<()> {
    let github_service = GitHubService::new()?;

    println!("Applying template: {name}");
    github_service.apply_template(name).await
}

pub async fn handle_interactive_mode() -> Result<()> {
    // Check NPM availability upfront
    let npm_available = InstallationManager::check_npm_available();

    loop {
        // Clear screen and display sleek T.JARVIS interface
        print!("\x1b[2J\x1b[H"); // Clear screen

        // Get terminal width for responsive design
        let term_width = if let Some((w, _)) = term_size::dimensions() {
            w
        } else {
            80 // fallback width
        };

        // Create centered, minimalist header
        println!();
        let title_line = "══════════════════════════════════════════════════════════════════";
        let title_padding = " ".repeat((term_width.saturating_sub(title_line.len())) / 2);
        println!("{title_padding}{title_line}");
        println!();

        // Centered T.JARVIS logo with version
        let logo_padding = " ".repeat((term_width.saturating_sub(30)) / 2);
        println!("{logo_padding}████████╗       ██╗ █████╗ ██████╗ ██╗   ██╗██╗███████╗");
        println!("{logo_padding}╚══██╔══╝       ██║██╔══██╗██╔══██╗██║   ██║██║██╔════╝");
        println!("{logo_padding}   ██║          ██║███████║██████╔╝██║   ██║██║███████╗");
        println!("{logo_padding}   ██║     ██   ██║██╔══██║██╔══██╗╚██╗ ██╔╝██║╚════██║");
        println!("{logo_padding}   ██║  ██╗╚█████╔╝██║  ██║██║  ██║ ╚████╔╝ ██║███████║");
        println!("{logo_padding}   ╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝╚═╝  ╚═╝  ╚═══╝  ╚═╝╚══════╝");

        // Version and tagline centered
        let version_text = "v0.0.6";
        let version_padding = " ".repeat((term_width.saturating_sub(version_text.len())) / 2);
        println!();
        println!("{version_padding}{version_text}");

        let tagline = "🤖 Your AI Coding Assistant Command Center";
        let tagline_padding = " ".repeat((term_width.saturating_sub(tagline.len())) / 2);
        println!("{tagline_padding}{tagline}");
        println!();

        // Bottom border
        println!("{title_padding}{title_line}");
        println!();

        // Quick help tips (Gemini-inspired but Jarvis-themed)
        if !npm_available {
            println!(
                "⚠️  Setup Required: Install Node.js for full functionality → https://nodejs.org/"
            );
            println!();
        }

        println!("💡 Quick Start Tips:");
        println!("   • Use ↑↓ arrows to navigate, Space/Enter to select");
        println!("   • Install tools you need, T.JARVIS handles the rest");
        println!("   • Run tools directly or use interactive mode");
        println!("   • T.JARVIS remembers your preferences");
        println!();

        if !npm_available {
            println!("⚠️  Warning: NPM not detected. Install Node.js for full functionality.");
            println!("   Download from: https://nodejs.org/\n");
        }

        let tools = ToolManager::get_available_tools();
        let install_commands = InstallationManager::get_install_commands();

        // Build clean options list (only selectable items)
        let mut options = Vec::new();
        let mut tool_mapping = Vec::new();

        // Separate installed and uninstalled tools
        let mut installed_tools = Vec::new();
        let mut uninstalled_tools = Vec::new();

        for (tool_name, tool_info) in tools.iter() {
            let install_info = install_commands.get(tool_name).unwrap();

            if tool_info.is_installed {
                installed_tools.push((*tool_name, install_info.description));
            } else {
                uninstalled_tools.push((*tool_name, install_info.description));
            }
        }

        // Display installed tools section
        if !installed_tools.is_empty() {
            println!("┌─ Ready to Launch ─┐");
            for (tool_name, description) in &installed_tools {
                let display_text = format!("  ☐ 🚀 {tool_name} - {description}");
                options.push(display_text);
                tool_mapping.push(Some(*tool_name));
            }
            println!();
        }

        // Display uninstalled tools section
        if !uninstalled_tools.is_empty() {
            println!("┌─ Available to Install ─┐");
            for (tool_name, description) in &uninstalled_tools {
                let display_text = format!("  ☐ 📦 {tool_name} - {description}");
                options.push(display_text);
                tool_mapping.push(Some(*tool_name));
            }
            println!();
        }

        // Add management options
        options.push("☐ ⚙️  Manage Tools".to_string());
        tool_mapping.push(None);

        options.push("☐ 📋 List All Tools".to_string());
        tool_mapping.push(None);

        options.push("☐ 🚪 Exit".to_string());
        tool_mapping.push(None);

        let selection =
            Select::new("Use ↑↓ arrows and Enter to select:", options.clone()).prompt()?;

        // Handle selection
        if selection.contains("⚙️") {
            handle_manage_tools_menu().await?;
        } else if selection.contains("📋") {
            handle_list_tools().await?;
            println!("\nPress Enter to continue...");
            let _ = std::io::stdin().read_line(&mut String::new());
        } else if selection.contains("🚪") {
            println!("👋 Goodbye!");
            break;
        } else {
            // Find the corresponding tool
            if let Some(index) = options.iter().position(|opt| opt == &selection) {
                if let Some(Some(tool_name)) = tool_mapping.get(index) {
                    let tools = ToolManager::get_available_tools();
                    let tool_info = tools.get(tool_name).unwrap();

                    if !tool_info.is_installed {
                        // Tool not installed, ask if user wants to install
                        let should_install = Confirm::new(&format!(
                            "📦 '{tool_name}' is not installed. Install it now?"
                        ))
                        .with_default(true)
                        .prompt()?;

                        if should_install {
                            println!("\n📦 Installing {tool_name}...");
                            handle_install_tool(tool_name).await?;
                            println!("✅ Installation complete!\n");
                        } else {
                            continue;
                        }
                    }

                    // Ask for additional arguments
                    let args_input = Text::new(&format!(
                        "Enter arguments for {tool_name} (or press Enter for default):"
                    ))
                    .with_default("")
                    .prompt()?;

                    let args: Vec<String> = if args_input.trim().is_empty() {
                        vec![]
                    } else {
                        shell_words::split(&args_input).unwrap_or_else(|_| {
                            args_input
                                .split_whitespace()
                                .map(|s| s.to_string())
                                .collect()
                        })
                    };

                    println!("\n🚀 Launching {tool_name} with args: {args:?}\n");

                    // Launch the tool
                    match ToolManager::run_tool(tool_name, &args).await {
                        Ok(_) => {
                            println!("\n✅ {tool_name} completed successfully!");
                        }
                        Err(e) => {
                            eprintln!("\n❌ Error running {tool_name}: {e}");
                        }
                    }

                    println!("\nPress Enter to continue...");
                    let _ = std::io::stdin().read_line(&mut String::new());
                }
            }
        }
    }
    Ok(())
}

async fn handle_manage_tools_menu() -> Result<()> {
    let options = vec![
        "📦 Install Tools".to_string(),
        "🔄 Update Tools".to_string(),
        "ℹ️  Tool Information".to_string(),
        "🔙 Back to Main Menu".to_string(),
    ];

    let selection = Select::new("Management Options:", options).prompt()?;

    match selection.as_str() {
        s if s.starts_with("📦") => handle_install_tools_menu().await,
        s if s.starts_with("�") => handle_update_tools_menu().await,
        s if s.starts_with("ℹ️") => handle_tool_info_menu().await,
        _ => Ok(()), // Back to main menu
    }
}

async fn handle_install_tools_menu() -> Result<()> {
    if !InstallationManager::check_npm_available() {
        println!("❌ NPM is not available. Please install Node.js first.");
        println!("   Download from: https://nodejs.org/");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

    let uninstalled_tools = ToolManager::get_uninstalled_tools();

    if uninstalled_tools.is_empty() {
        println!("✅ All tools are already installed!");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

    let tools_to_install =
        MultiSelect::new("Select tools to install:", uninstalled_tools).prompt()?;

    println!();
    for tool in tools_to_install {
        if let Err(e) = handle_install_tool(tool).await {
            eprintln!("❌ Failed to install {tool}: {e}");
        }
    }

    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

async fn handle_update_tools_menu() -> Result<()> {
    let installed_tools: Vec<String> = ToolManager::get_installed_tools()
        .into_iter()
        .map(String::from)
        .collect();

    if installed_tools.is_empty() {
        println!("📦 No tools are installed yet!");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

    let mut options = installed_tools.clone();
    options.push("All Tools".to_string());

    let selection = Select::new("Select tools to update:", options).prompt()?;

    println!();
    if selection == "All Tools" {
        println!("🔄 Updating all installed tools...");
        handle_update_packages(None).await?;
    } else {
        println!("🔄 Updating {selection}...");
        handle_update_packages(Some(&selection)).await?;
    }

    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

async fn handle_tool_info_menu() -> Result<()> {
    let tool_names: Vec<String> = InstallationManager::get_tool_names()
        .into_iter()
        .map(String::from)
        .collect();
    let tool = Select::new("Select a tool for information:", tool_names).prompt()?;

    println!();
    handle_tool_info(&tool).await?;

    println!("\nPress Enter to continue...");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

#[allow(dead_code)]
fn extract_tool_name_from_selection<'a>(
    _selection: &str,
    tool_keys: &'a [Option<&str>],
    options: &[String],
) -> Option<&'a str> {
    if let Some(index) = options.iter().position(|opt| opt == _selection) {
        if let Some(Some(tool_name)) = tool_keys.get(index) {
            return Some(tool_name);
        }
    }
    None
}
