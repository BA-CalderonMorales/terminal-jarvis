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
        // Clear screen and display stunning T.JARVIS interface
        print!("\x1b[2J\x1b[H"); // Clear screen

        // Get terminal width for responsive design
        let term_width = if let Some((w, _)) = term_size::dimensions() {
            w
        } else {
            80 // fallback width
        };

        // Futuristic minimal design
        println!();
        let border_width = std::cmp::min(70, term_width.saturating_sub(4));
        let border_padding = if term_width > border_width {
            " ".repeat((term_width - border_width) / 2)
        } else {
            String::new()
        };

        // Futuristic neon colors - Electric cyan and bright white
        let neon_cyan = "\x1b[96m"; // Bright cyan
        let neon_white = "\x1b[97m"; // Bright white
        let neon_blue = "\x1b[94m"; // Bright blue
        let dim_cyan = "\x1b[36m"; // Regular cyan for borders
        let reset = "\x1b[0m";

        let top_border = format!(
            "{}╔{}╗{}",
            dim_cyan,
            "═".repeat(border_width.saturating_sub(2)),
            reset
        );
        let empty_border = format!(
            "{}║{}║{}",
            dim_cyan,
            " ".repeat(border_width.saturating_sub(2)),
            reset
        );
        let bottom_border = format!(
            "{}╚{}╝{}",
            dim_cyan,
            "═".repeat(border_width.saturating_sub(2)),
            reset
        );

        println!("{border_padding}{top_border}");
        println!("{border_padding}{empty_border}");

        // Futuristic T.JARVIS ASCII art
        let logo_lines = vec![
            "████████╗       ██╗ █████╗ ██████╗ ██╗   ██╗██╗███████╗",
            "╚══██╔══╝       ██║██╔══██╗██╔══██╗██║   ██║██║██╔════╝",
            "   ██║          ██║███████║██████╔╝██║   ██║██║███████╗",
            "   ██║     ██   ██║██╔══██║██╔══██╗╚██╗ ██╔╝██║╚════██║",
            "   ██║  ██╗╚█████╔╝██║  ██║██║  ██║ ╚████╔╝ ██║███████║",
            "   ╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝╚═╝  ╚═╝  ╚═══╝  ╚═╝╚══════╝",
        ];

        let inner_width = border_width.saturating_sub(4);

        for line in logo_lines {
            let line_chars: Vec<char> = line.chars().collect();
            let line_char_len = line_chars.len();

            if line_char_len <= inner_width {
                let content_padding = (inner_width - line_char_len) / 2;
                let left_padding = " ".repeat(content_padding);
                let right_padding = " ".repeat(inner_width - content_padding - line_char_len);
                println!(
                    "{border_padding}{dim_cyan}║ {left_padding}{neon_cyan}{line}{right_padding} ║{reset}"
                );
            } else {
                let simple_line = "T.JARVIS";
                let content_padding = (inner_width - simple_line.len()) / 2;
                let left_padding = " ".repeat(content_padding);
                let right_padding = " ".repeat(inner_width - content_padding - simple_line.len());
                println!(
                    "{border_padding}{dim_cyan}║ {left_padding}{neon_cyan}{simple_line}{right_padding} ║{reset}"
                );
            }
        }

        println!("{border_padding}{empty_border}");

        // Version and tagline in futuristic style
        let version_text = format!("v{}", env!("CARGO_PKG_VERSION"));
        if version_text.len() <= inner_width {
            let version_content_padding = (inner_width - version_text.len()) / 2;
            let version_left_padding = " ".repeat(version_content_padding);
            let version_right_padding =
                " ".repeat(inner_width - version_content_padding - version_text.len());
            println!(
                "{border_padding}{dim_cyan}║ {version_left_padding}{neon_blue}{version_text}{version_right_padding} ║{reset}"
            );
        }

        let tagline = "🤖 AI Coding Assistant Command Center";
        let tagline_display_len = tagline.chars().count() + 1;
        if tagline_display_len <= inner_width {
            let tagline_content_padding = (inner_width - tagline_display_len) / 2;
            let tagline_left_padding = " ".repeat(tagline_content_padding);
            let tagline_right_padding =
                " ".repeat(inner_width - tagline_content_padding - tagline_display_len);
            println!(
                "{border_padding}{dim_cyan}║ {tagline_left_padding}{neon_white}{tagline}{tagline_right_padding} ║{reset}"
            );
        }

        println!("{border_padding}{empty_border}");
        println!("{border_padding}{bottom_border}");
        println!();

        // Minimal setup warning
        if !npm_available {
            println!("{neon_blue}⚠️  Node.js required → https://nodejs.org/{reset}");
            println!();
        }

        let tools = ToolManager::get_available_tools();
        let install_commands = InstallationManager::get_install_commands();

        // Clean tool list - minimal and futuristic
        let mut options = Vec::new();
        let mut tool_mapping = Vec::new();

        for (tool_name, tool_info) in tools.iter() {
            let install_info = install_commands.get(tool_name).unwrap();
            let status_icon = if tool_info.is_installed {
                "🚀"
            } else {
                "📦"
            };
            let display_text = format!(
                "{} {} - {}",
                status_icon, tool_name, install_info.description
            );
            options.push(display_text);
            tool_mapping.push(Some(*tool_name));
        }

        // Clean separator and settings
        options.push("".to_string());
        tool_mapping.push(None);

        options.push("⚙️  Settings".to_string());
        tool_mapping.push(None);

        options.push("🚪 Exit".to_string());
        tool_mapping.push(None);

        let selection = Select::new("Select an AI tool to launch:", options.clone())
            .with_page_size(15)
            .prompt()?;

        // Handle selection
        if selection.contains("⚙️") {
            handle_manage_tools_menu().await?;
        } else if selection.contains("🚪") {
            println!("{neon_blue}👋 Goodbye!{reset}");
            break;
        } else if selection.is_empty() {
            continue;
        } else if let Some(index) = options.iter().position(|opt| opt == &selection) {
            if let Some(Some(tool_name)) = tool_mapping.get(index) {
                let tools = ToolManager::get_available_tools();
                let tool_info = tools.get(tool_name).unwrap();

                if !tool_info.is_installed {
                    let should_install = Confirm::new(&format!(
                        "{neon_blue}📦 '{tool_name}' is not installed. Install it now?{reset}"
                    ))
                    .with_default(true)
                    .prompt()?;

                    if should_install {
                        println!("\n{neon_cyan}📦 Installing {tool_name}...{reset}");
                        handle_install_tool(tool_name).await?;
                        println!("{neon_cyan}✅ Installation complete!{reset}\n");
                    } else {
                        continue;
                    }
                }

                let args_input = Text::new(&format!(
                    "{neon_white}Enter arguments for {tool_name} (or press Enter for default):{reset}"
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

                println!("\n{neon_cyan}🚀 Launching {tool_name} with args: {args:?}{reset}\n");

                match ToolManager::run_tool(tool_name, &args).await {
                    Ok(_) => {
                        println!("\n{neon_cyan}✅ {tool_name} completed successfully!{reset}");
                    }
                    Err(e) => {
                        eprintln!("\n{neon_cyan}❌ Error running {tool_name}: {e}{reset}");
                    }
                }

                println!("\n{neon_blue}Press Enter to continue...{reset}");
                let _ = std::io::stdin().read_line(&mut String::new());
            }
        }
    }
    Ok(())
}

async fn handle_manage_tools_menu() -> Result<()> {
    // Futuristic colors consistent with main interface
    let neon_cyan = "\x1b[96m";
    let reset = "\x1b[0m";

    print!("\x1b[2J\x1b[H"); // Clear screen

    println!("{neon_cyan}⚙️  Settings & Tools{reset}\n");

    let options = vec![
        "📦 Install Tools".to_string(),
        "🔄 Update Tools".to_string(),
        "📋 List All Tools".to_string(),
        "ℹ️  Tool Information".to_string(),
        "🔙 Back to Main Menu".to_string(),
    ];

    let selection = Select::new("Choose an option:", options)
        .with_page_size(10)
        .prompt()?;

    match selection.as_str() {
        s if s.starts_with("📦") => handle_install_tools_menu().await,
        s if s.starts_with("🔄") => handle_update_tools_menu().await,
        s if s.starts_with("📋") => {
            handle_list_tools().await?;
            println!("\n{neon_cyan}Press Enter to continue...{reset}");
            let _ = std::io::stdin().read_line(&mut String::new());
            Ok(())
        }
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
