use crate::config::ConfigManager;
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use crate::services::{GitHubService, PackageService};
use crate::tools::ToolManager;
use anyhow::{anyhow, Result};
use inquire::{Confirm, MultiSelect, Select, Text};
use tokio::process::Command as AsyncCommand;

pub async fn handle_run_tool(tool: &str, args: &[String]) -> Result<()> {
    // Check if NPM is available first
    if !InstallationManager::check_npm_available() {
        ProgressUtils::warning_message("NPM is not installed or not in PATH.");
        println!("   Most AI coding tools require NPM for installation.");
        println!("   Please install Node.js and NPM first: https://nodejs.org/");
        return Err(anyhow!("NPM is required but not available"));
    }

    // Check if tool is installed with progress
    let check_progress = ProgressContext::new(&format!("Checking {tool} availability"));
    let cli_command = ToolManager::get_cli_command(tool);

    // Add a small delay to show the progress indicator
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    if !ToolManager::check_tool_installed(cli_command) {
        check_progress.finish_error(&format!("Tool '{tool}' is not installed"));

        let should_install = match Confirm::new(&format!("📦 Install '{tool}' now?"))
            .with_default(true)
            .prompt()
        {
            Ok(result) => result,
            Err(_) => {
                // User interrupted - treat as "no"
                return Err(anyhow!("Installation cancelled"));
            }
        };

        if should_install {
            handle_install_tool(tool).await?;
            ProgressUtils::success_message("Installation complete!");
        } else {
            return Err(anyhow!("Tool '{}' is required but not installed", tool));
        }
    } else {
        check_progress.finish_success(&format!("{tool} is available"));
    }

    // Show startup progress for the tool
    let args_display = if args.is_empty() {
        "no arguments".to_string()
    } else {
        format!("arguments: {}", args.join(" "))
    };

    let startup_progress = ProgressContext::new(&format!("Launching {tool}"));
    startup_progress.update_message(&format!("Launching {tool} with {args_display}"));

    // Add a brief delay to show startup progress
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
    startup_progress.finish_success(&format!("Starting {tool}"));

    // Special handling for opencode - ensure clean terminal state
    if tool == "opencode" {
        use std::io::Write;
        // Force flush any remaining output and reset terminal
        print!("\x1b[2J\x1b[H\x1b[?25h"); // Clear screen, home cursor, show cursor
        std::io::stdout().flush().unwrap_or_default();
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }

    ToolManager::run_tool(tool, args).await
}

pub async fn handle_install_tool(tool: &str) -> Result<()> {
    let install_cmd = InstallationManager::get_install_command(tool)
        .ok_or_else(|| anyhow!("Tool '{}' not found in installation registry", tool))?;

    // Check NPM availability with progress
    if install_cmd.requires_npm {
        let npm_check = ProgressContext::new("Checking NPM availability");

        if !InstallationManager::check_npm_available() {
            npm_check.finish_error("NPM is not installed or not in PATH");
            println!("   Please install Node.js and NPM first: https://nodejs.org/");
            return Err(anyhow!(
                "NPM is required to install {} but is not available",
                tool
            ));
        }

        npm_check.finish_success("NPM is available");
    }

    // Create installation progress
    let progress = ProgressContext::new(&format!("Installing {tool}"));
    progress.update_message(&format!(
        "Installing {tool} using: {} {}",
        install_cmd.command,
        install_cmd.args.join(" ")
    ));

    // For NPM packages, simulate realistic installation progress
    if install_cmd.requires_npm {
        ProgressUtils::simulate_installation_progress(&progress.spinner, tool).await;
    }

    let mut cmd = AsyncCommand::new(install_cmd.command);
    cmd.args(&install_cmd.args);

    // Suppress output to avoid interfering with progress bar
    cmd.stdout(std::process::Stdio::null());
    cmd.stderr(std::process::Stdio::null());

    let status = cmd.status().await?;

    if status.success() {
        progress.finish_success(&format!("{tool} installed successfully!"));

        // Verify installation with progress - add delay for PATH updates
        let verify_progress = ProgressContext::new(&format!("Verifying {tool} installation"));

        // Wait a bit for PATH updates and system to recognize new binary
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        ProgressUtils::simulate_verification_progress(&verify_progress.spinner, tool).await;

        let cli_command = ToolManager::get_cli_command(tool);
        if ToolManager::check_tool_installed(cli_command) {
            verify_progress.finish_success(&format!("{tool} is ready to use"));
        } else {
            verify_progress.finish_error(&format!("{tool} installation could not be verified"));

            // For opencode, provide additional guidance
            if tool == "opencode" {
                ProgressUtils::warning_message(
                    "OpenCode may require a shell restart to be available in PATH.",
                );
                ProgressUtils::info_message(
                    "Try running: source ~/.bashrc or restart your terminal.",
                );
            }
        }

        Ok(())
    } else {
        progress.finish_error(&format!("Failed to install {tool}"));
        Err(anyhow!("Failed to install {}", tool))
    }
}

pub async fn handle_update_packages(package: Option<&str>) -> Result<()> {
    let package_service = PackageService::new()?;

    match package {
        Some(pkg) => {
            let update_progress = ProgressContext::new(&format!("⚡ Updating {pkg}"));

            // Add a small delay to show the progress indicator
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            update_progress.update_message(&format!("📦 Downloading latest version of {pkg}..."));

            let result = package_service.update_tool(pkg).await;

            match result {
                Ok(_) => {
                    update_progress.finish_success(&format!("✅ {pkg} updated successfully"));
                    Ok(())
                }
                Err(e) => {
                    update_progress.finish_error(&format!("❌ Failed to update {pkg}"));
                    Err(e)
                }
            }
        }
        None => {
            // This should not be called anymore from the updated menu, but keeping for compatibility
            let overall_progress = ProgressContext::new("⚡ Updating all packages");
            let tools = InstallationManager::get_tool_names();
            let mut had_errors = false;

            for (index, tool) in tools.iter().enumerate() {
                overall_progress.update_message(&format!(
                    "⚡ Updating {tool} ({}/{})...",
                    index + 1,
                    tools.len()
                ));

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                if let Err(e) = package_service.update_tool(tool).await {
                    ProgressUtils::error_message(&format!("❌ Failed to update {tool}: {e}"));
                    had_errors = true;
                } else {
                    ProgressUtils::success_message(&format!("✅ {tool} updated successfully"));
                }
            }

            if had_errors {
                overall_progress.finish_error("⚠️  Some packages failed to update");
            } else {
                overall_progress.finish_success("🎉 All packages updated successfully");
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

        // Version and tagline in futuristic style - with NPM distribution tag if available
        let base_version = env!("CARGO_PKG_VERSION");

        // Initialize config manager for caching
        let config_manager = ConfigManager::new().unwrap_or_else(|_| {
            // Fallback if config manager fails - continue without caching
            ConfigManager::new().expect("Failed to create config manager")
        });

        // Show progress for NPM tag detection with caching
        let npm_progress = ProgressContext::new("🔍 Checking NPM distribution tags");
        let npm_tag = PackageService::get_cached_npm_dist_tag_info(&config_manager)
            .await
            .unwrap_or(None);
        npm_progress.finish_success("NPM tag info loaded");

        let version_text = if let Some(tag) = npm_tag {
            format!("v{base_version} (@{tag})")
        } else {
            format!("v{base_version}")
        };

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
        // Calculate visual width accounting for emoji (which takes 2 columns)
        let tagline_visual_width = tagline
            .chars()
            .map(|c| {
                if c as u32 >= 0x1F300 {
                    2
                } else {
                    1
                } // Emoji range approximation
            })
            .sum::<usize>();

        if tagline_visual_width <= inner_width {
            let tagline_content_padding = (inner_width - tagline_visual_width) / 2;
            let tagline_left_padding = " ".repeat(tagline_content_padding);
            let tagline_right_padding =
                " ".repeat(inner_width - tagline_content_padding - tagline_visual_width);
            println!(
                "{border_padding}{dim_cyan}║ {tagline_left_padding}{neon_white}{tagline}{tagline_right_padding} ║{reset}"
            );
        }

        println!("{border_padding}{empty_border}");

        // Short hint about Important Links - shortened to fit border
        let links_hint = "📚 See 'Important Links' menu";
        // Calculate visual width accounting for emoji (which takes 2 columns)
        let links_visual_width = links_hint
            .chars()
            .map(|c| {
                if c as u32 >= 0x1F300 {
                    2
                } else {
                    1
                } // Emoji range approximation
            })
            .sum::<usize>();

        if links_visual_width <= inner_width {
            let links_content_padding = (inner_width - links_visual_width) / 2;
            let links_left_padding = " ".repeat(links_content_padding);
            let links_right_padding =
                " ".repeat(inner_width - links_content_padding - links_visual_width);
            println!(
                "{border_padding}{dim_cyan}║ {links_left_padding}{dim_cyan}{links_hint}{links_right_padding} ║{reset}"
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

        // Show progress while loading tool status
        let loading_progress = ProgressContext::new("Loading AI tools status");

        // Add a small delay to show the progress
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        loading_progress.finish_success("Interface initialized");

        // Main menu options - clean and organized
        let options = vec![
            "🤖 AI CLI Tools".to_string(),
            "📚 Important Links".to_string(),
            "⚙️  Settings".to_string(),
            "🚪 Exit".to_string(),
        ];

        let selection = match Select::new("Choose an option:", options.clone())
            .with_page_size(10)
            .prompt()
        {
            Ok(selection) => selection,
            Err(_) => {
                // User interrupted (Ctrl+C) - show clean exit message
                println!();
                println!("👋 Goodbye!");
                return Ok(());
            }
        };

        // Handle selection
        match selection.as_str() {
            s if s.starts_with("🤖") => {
                handle_ai_tools_menu().await?;
            }
            s if s.starts_with("📚") => {
                handle_important_links().await?;
            }
            s if s.starts_with("⚙️") => {
                handle_manage_tools_menu().await?;
            }
            s if s.starts_with("🚪") => {
                println!("{neon_blue}👋 Goodbye!{reset}");
                break;
            }
            _ => continue,
        }
    }
    Ok(())
}

async fn handle_ai_tools_menu() -> Result<()> {
    // Futuristic colors consistent with main interface
    let neon_cyan = "\x1b[96m";
    let neon_blue = "\x1b[94m";
    let neon_white = "\x1b[97m";
    let reset = "\x1b[0m";

    print!("\x1b[2J\x1b[H"); // Clear screen

    println!("{neon_cyan}🤖 AI CLI Tools{reset}\n");

    // Show loading indicator
    let loading_progress = ProgressContext::new("Loading AI tools status");
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    let tools = ToolManager::get_available_tools();
    let install_commands = InstallationManager::get_install_commands();

    loading_progress.finish_success("AI tools status loaded");

    // Build clean tool list with status indicators
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

    // Add back option
    options.push("🔙 Back to Main Menu".to_string());
    tool_mapping.push(None);

    let selection = match Select::new("Select an AI tool to launch:", options.clone())
        .with_page_size(15)
        .prompt()
    {
        Ok(selection) => selection,
        Err(_) => {
            // User interrupted (Ctrl+C) - return to main menu
            return Ok(());
        }
    };

    // Handle selection
    if selection.contains("🔙") {
        return Ok(());
    } else if let Some(index) = options.iter().position(|opt| opt == &selection) {
        if let Some(Some(tool_name)) = tool_mapping.get(index) {
            let tools = ToolManager::get_available_tools();
            let tool_info = tools.get(tool_name).unwrap();

            if !tool_info.is_installed {
                let should_install = match Confirm::new(&format!(
                    "{neon_blue}📦 '{tool_name}' is not installed. Install it now?{reset}"
                ))
                .with_default(true)
                .prompt()
                {
                    Ok(result) => result,
                    Err(_) => {
                        // User interrupted - go back to main menu
                        println!("\n{neon_cyan}🔙 Installation cancelled{reset}");
                        return Ok(());
                    }
                };

                if should_install {
                    println!("\n{neon_cyan}📦 Installing {tool_name}...{reset}");
                    handle_install_tool(tool_name).await?;
                    println!("{neon_cyan}✅ Installation complete!{reset}\n");
                } else {
                    return Ok(());
                }
            }

            let args_input = match Text::new(&format!(
                "{neon_white}Enter arguments for {tool_name} (or press Enter for default):{reset}"
            ))
            .with_default("")
            .prompt()
            {
                Ok(input) => input,
                Err(_) => {
                    // User interrupted - go back to main menu
                    println!("\n{neon_cyan}🔙 Operation cancelled{reset}");
                    return Ok(());
                }
            };

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

            // Show loading indicator before launching tool
            let launch_progress = ProgressContext::new(&format!("Launching {tool_name}"));

            // Show more detailed progress steps
            launch_progress.update_message(&format!("Preparing {tool_name} environment"));
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

            launch_progress.update_message(&format!("Initializing {tool_name}"));
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

            launch_progress.update_message(&format!("Starting {tool_name} with args: {args:?}"));
            tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

            // Finish progress right before starting the tool
            launch_progress.finish_success(&format!("{tool_name} ready - starting now"));

            // Special handling for opencode to ensure input focus works properly
            if *tool_name == "opencode" {
                // For opencode, we need extra time and careful terminal state management
                // to prevent input focus issues on fresh installs
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            } else {
                // Clear any remaining progress indicators for other tools
                print!("\x1b[2K\r");
            }

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

    Ok(())
}

async fn handle_important_links() -> Result<()> {
    // Futuristic colors consistent with main interface
    let neon_cyan = "\x1b[96m";
    let neon_blue = "\x1b[94m";
    let neon_green = "\x1b[92m";
    let neon_magenta = "\x1b[95m";
    let neon_yellow = "\x1b[93m";
    let reset = "\x1b[0m";

    print!("\x1b[2J\x1b[H"); // Clear screen

    println!("{neon_cyan}📚 Important Links & Resources{reset}\n");

    let options = vec![
        format!("{neon_blue}🐙 GitHub Repository{reset}"),
        format!("{neon_green}📦 NPM Package{reset}"),
        format!("{neon_magenta}📝 CHANGELOG.md{reset}"),
        format!("{neon_yellow}🔧 Cargo Package{reset}"),
        format!("{neon_cyan}📖 Documentation{reset}"),
        format!("{neon_blue}🚀 Homebrew Formula{reset}"),
        "🔙 Back to Main Menu".to_string(),
    ];

    let selection = match Select::new("Choose a resource to view:", options)
        .with_page_size(10)
        .prompt()
    {
        Ok(selection) => selection,
        Err(_) => {
            // User interrupted - return to main menu
            return Ok(());
        }
    };

    match selection.as_str() {
        s if s.contains("GitHub") => {
            println!("\n{neon_blue}🐙 Opening GitHub Repository...{reset}");
            println!("   📍 https://github.com/BA-CalderonMorales/terminal-jarvis");
            println!("   - View source code and contribute");
            println!("   - Report issues and feature requests");
            println!("   - Check latest releases and changelogs");
        }
        s if s.contains("NPM") => {
            println!("\n{neon_green}📦 NPM Package Information...{reset}");
            println!("   📍 https://www.npmjs.com/package/terminal-jarvis");
            println!("   - Install: npm install -g terminal-jarvis");
            println!("   - Run: npx terminal-jarvis");
            println!("   - Version history and statistics");
        }
        s if s.contains("CHANGELOG") => {
            println!("\n{neon_magenta}📝 CHANGELOG.md - Version History{reset}");
            println!("   📍 https://github.com/BA-CalderonMorales/terminal-jarvis/blob/main/CHANGELOG.md");
            println!("   - Detailed release notes for each version");
            println!("   - Feature additions and bug fixes");
            println!("   - Breaking changes and migration guides");
        }
        s if s.contains("Cargo") => {
            println!("\n{neon_yellow}🔧 Cargo Package Information{reset}");
            println!("   📍 https://crates.io/crates/terminal-jarvis");
            println!("   - Install: cargo install terminal-jarvis");
            println!("   - Rust ecosystem integration");
            println!("   - Development dependencies and features");
        }
        s if s.contains("Documentation") => {
            println!("\n{neon_cyan}📖 Documentation Hub{reset}");
            println!("   📍 https://github.com/BA-CalderonMorales/terminal-jarvis/tree/main/docs");
            println!("   - Architecture overview and design decisions");
            println!("   - Installation guides for all platforms");
            println!("   - Testing procedures and contribution guidelines");
            println!("   - Limitations and troubleshooting");
        }
        s if s.contains("Homebrew") => {
            println!("\n{neon_blue}🚀 Homebrew Formula{reset}");
            println!(
                "   📍 https://github.com/BA-CalderonMorales/terminal-jarvis/tree/main/homebrew"
            );
            println!("   - macOS/Linux package management");
            println!("   - Install: brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis");
            println!("   - System integration and automatic updates");
        }
        _ => {
            // Back to main menu
            return Ok(());
        }
    }

    println!("\n{neon_cyan}Press Enter to continue...{reset}");
    let _ = std::io::stdin().read_line(&mut String::new());

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

    let selection = match Select::new("Choose an option:", options)
        .with_page_size(10)
        .prompt()
    {
        Ok(selection) => selection,
        Err(_) => {
            // User interrupted - return to main menu
            return Ok(());
        }
    };

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
    // Check NPM availability with progress
    let npm_check = ProgressContext::new("Checking NPM availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    if !InstallationManager::check_npm_available() {
        npm_check.finish_error("NPM is not available");
        ProgressUtils::error_message("NPM is not available. Please install Node.js first.");
        println!("   Download from: https://nodejs.org/");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

    npm_check.finish_success("NPM is available");

    // Check which tools are uninstalled with progress
    let check_progress = ProgressContext::new("Scanning for uninstalled tools");
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;

    let uninstalled_tools = ToolManager::get_uninstalled_tools();

    if uninstalled_tools.is_empty() {
        check_progress.finish_success("All tools are already installed");
        ProgressUtils::success_message("All tools are already installed!");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

    check_progress.finish_success(&format!(
        "Found {} tools available for installation",
        uninstalled_tools.len()
    ));

    let tools_to_install =
        match MultiSelect::new("Select tools to install:", uninstalled_tools).prompt() {
            Ok(tools) => tools,
            Err(_) => {
                // User interrupted - return to previous menu
                return Ok(());
            }
        };

    println!();
    for tool in tools_to_install {
        if let Err(e) = handle_install_tool(tool).await {
            ProgressUtils::error_message(&format!("Failed to install {tool}: {e}"));
        }
    }

    println!("Press Enter to continue...");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

async fn handle_update_tools_menu() -> Result<()> {
    // Check for installed tools with progress
    let scan_progress = ProgressContext::new("🔍 Scanning for installed tools");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let installed_tools: Vec<String> = ToolManager::get_installed_tools()
        .into_iter()
        .map(String::from)
        .collect();

    if installed_tools.is_empty() {
        scan_progress.finish_error("❌ No tools are installed yet");
        ProgressUtils::info_message("No tools are installed yet!");
        println!("   Install tools first using the main menu.");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

    scan_progress.finish_success(&format!(
        "✅ Found {} installed tools",
        installed_tools.len()
    ));

    let mut options = installed_tools.clone();
    options.push("All Tools".to_string());

    let selection = match Select::new("Select tools to update:", options).prompt() {
        Ok(selection) => selection,
        Err(_) => {
            // User interrupted - return to previous menu
            return Ok(());
        }
    };

    println!();
    if selection == "All Tools" {
        let update_progress = ProgressContext::new("⚡ Updating all installed tools concurrently");

        // Use concurrent updates for better performance
        let mut update_futures = Vec::new();

        for (index, tool) in installed_tools.iter().enumerate() {
            let tool_name = tool.clone();
            let total_tools = installed_tools.len();
            let current_index = index + 1;

            let future = async move {
                let individual_progress = ProgressContext::new(&format!(
                    "⚡ Updating {tool_name} ({current_index}/{total_tools})"
                ));
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                individual_progress
                    .update_message(&format!("📦 Downloading latest version of {tool_name}..."));

                match handle_update_packages(Some(&tool_name)).await {
                    Ok(_) => {
                        individual_progress
                            .finish_success(&format!("✅ {tool_name} updated successfully"));
                        Ok(tool_name)
                    }
                    Err(e) => {
                        individual_progress
                            .finish_error(&format!("❌ Failed to update {tool_name}"));
                        Err((tool_name, e))
                    }
                }
            };
            update_futures.push(future);
        }

        // Wait for all updates to complete
        let results = futures::future::join_all(update_futures).await;
        update_progress.finish_success("🎯 Concurrent updates completed");

        // Analyze results and show detailed summary
        let mut successful_tools = Vec::new();
        let mut failed_tools = Vec::new();

        for result in results {
            match result {
                Ok(tool_name) => successful_tools.push(tool_name),
                Err((tool_name, error)) => failed_tools.push((tool_name, error)),
            }
        }

        // Display comprehensive results
        println!("\n🎯 Update Results Summary:");

        if !successful_tools.is_empty() {
            println!("✅ Successfully updated {} tools:", successful_tools.len());
            for tool in &successful_tools {
                println!("   ✓ {tool}");
            }
        }

        if !failed_tools.is_empty() {
            println!("\n❌ Failed to update {} tools:", failed_tools.len());
            for (tool, error) in &failed_tools {
                println!("   ✗ {tool}: {error}");
            }

            println!("\n💡 Troubleshooting tips:");
            println!("   • Check your internet connection");
            println!(
                "   • Ensure you have the required package managers installed (npm, cargo, pip)"
            );
            println!("   • Some tools might need to be manually updated");
            println!("   • Try updating individual tools to see specific error messages");
        }

        if successful_tools.is_empty() && !failed_tools.is_empty() {
            println!("\n⚠️  All updates failed. Please check your environment setup.");
        } else if !successful_tools.is_empty() && !failed_tools.is_empty() {
            println!(
                "\n🎯 Partial success: {}/{} tools updated successfully",
                successful_tools.len(),
                installed_tools.len()
            );
        } else {
            println!("\n🎉 All tools updated successfully!");
        }
    } else {
        let update_progress = ProgressContext::new(&format!("⚡ Updating {selection}"));
        match handle_update_packages(Some(&selection)).await {
            Ok(_) => {
                update_progress.finish_success(&format!("✅ {selection} updated successfully"));
                println!("\n🎉 Update completed successfully!");
                println!("✅ {selection} is now up to date!");
            }
            Err(e) => {
                update_progress.finish_error(&format!("❌ Failed to update {selection}"));
                println!("\n⚠️  Update failed!");
                println!("❌ Error: {e}");
                println!("\n💡 Troubleshooting tips:");
                println!("   • Check your internet connection");
                println!("   • Ensure the required package manager is installed");
                println!("   • Try running the update command manually");
                println!("   • Some tools may require manual updates");
            }
        }
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
    let tool = match Select::new("Select a tool for information:", tool_names).prompt() {
        Ok(selection) => selection,
        Err(_) => {
            // User interrupted - return to previous menu
            return Ok(());
        }
    };

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

// Configuration management handlers

pub async fn handle_config_reset() -> Result<()> {
    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("terminal-jarvis").join("config.toml"),
        None => {
            ProgressUtils::error_message("Could not determine config directory");
            return Err(anyhow!("Could not determine config directory"));
        }
    };

    if config_path.exists() {
        let confirm = match Confirm::new("Are you sure you want to reset configuration to defaults? This will delete your current config file.")
            .with_default(false)
            .prompt() {
                Ok(result) => result,
                Err(_) => {
                    // User interrupted - cancel the operation
                    ProgressUtils::info_message("Configuration reset cancelled");
                    return Ok(());
                }
            };

        if confirm {
            std::fs::remove_file(&config_path)?;
            ProgressUtils::success_message("Configuration reset to defaults");
            ProgressUtils::info_message(
                "The config file has been deleted. Default settings will be used.",
            );
        } else {
            ProgressUtils::info_message("Configuration reset cancelled");
        }
    } else {
        ProgressUtils::info_message("No configuration file found. Using defaults already.");
    }

    Ok(())
}

pub async fn handle_config_show() -> Result<()> {
    use crate::config::Config;

    let config = Config::load()?;
    let config_str = toml::to_string_pretty(&config)?;

    println!("Current configuration:");
    println!("{config_str}");

    Ok(())
}

pub async fn handle_config_path() -> Result<()> {
    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("terminal-jarvis").join("config.toml"),
        None => {
            ProgressUtils::error_message("Could not determine config directory");
            return Err(anyhow!("Could not determine config directory"));
        }
    };

    println!("Configuration file path: {}", config_path.display());

    if config_path.exists() {
        ProgressUtils::success_message("Configuration file exists");
    } else {
        ProgressUtils::info_message("Configuration file does not exist (using defaults)");
    }

    Ok(())
}

// Version cache management handlers

pub async fn handle_cache_clear() -> Result<()> {
    let config_manager = ConfigManager::new()?;

    config_manager.clear_version_cache()?;
    ProgressUtils::success_message("✅ Version cache cleared");

    Ok(())
}

pub async fn handle_cache_status() -> Result<()> {
    let config_manager = ConfigManager::new()?;

    match config_manager.load_version_cache()? {
        Some(cache) => {
            println!("📄 Version Cache Status:");
            println!("  Version Info: {}", cache.version_info);
            println!("  Cached at: {} (Unix timestamp)", cache.cached_at);
            println!("  TTL: {} seconds", cache.ttl_seconds);

            if cache.is_expired() {
                println!("  Status: ❌ Expired");
            } else {
                let remaining = cache.remaining_seconds();
                println!("  Status: ✅ Valid ({remaining} seconds remaining)");
            }
        }
        None => {
            println!("📄 No version cache found");
        }
    }

    Ok(())
}

pub async fn handle_cache_refresh(ttl: u64) -> Result<()> {
    let config_manager = ConfigManager::new()?;

    println!("🔄 Refreshing version cache...");
    let latest_version_info =
        PackageService::get_cached_npm_dist_tag_info_with_ttl(&config_manager, ttl).await?;

    match latest_version_info {
        Some(version_info) => {
            ProgressUtils::success_message(&format!(
                "✅ Cache refreshed with version info: {version_info} (TTL: {ttl}s)"
            ));
        }
        None => {
            ProgressUtils::warning_message("⚠️  No version information available to cache");
        }
    }

    Ok(())
}
