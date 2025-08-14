use crate::config::ConfigManager;
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::{ProgressContext, ProgressUtils};
use crate::services::{GitHubService, PackageService};
use crate::theme_config;
use crate::tools::ToolManager;
use anyhow::{anyhow, Result};
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};
use inquire::{Confirm, MultiSelect, Select, Text};
use tokio::process::Command as AsyncCommand;

/// Create inquire RenderConfig based on current theme
fn get_themed_render_config() -> RenderConfig<'static> {
    let theme = theme_config::current_theme();

    // Map our theme to inquire colors based on theme name
    let (primary_color, accent_color, secondary_color) = match theme.name {
        "T.JARVIS" => (Color::DarkCyan, Color::LightCyan, Color::DarkBlue),
        "Classic" => (Color::White, Color::DarkCyan, Color::DarkGrey),
        "Matrix" => (Color::DarkGreen, Color::LightGreen, Color::Black),
        _ => (Color::DarkCyan, Color::LightCyan, Color::DarkGrey),
    };

    RenderConfig::default()
        .with_prompt_prefix(Styled::new("?").with_fg(accent_color))
        .with_answered_prompt_prefix(Styled::new("✓").with_fg(accent_color))
        .with_default_value(StyleSheet::new().with_fg(secondary_color))
        .with_help_message(StyleSheet::new().with_fg(secondary_color))
        .with_text_input(StyleSheet::new().with_fg(primary_color))
        .with_highlighted_option_prefix(Styled::new(">").with_fg(accent_color))
        .with_option(StyleSheet::new().with_fg(primary_color))
        .with_selected_option(Some(
            StyleSheet::new()
                .with_fg(accent_color)
                .with_attr(inquire::ui::Attributes::BOLD),
        ))
        .with_scroll_up_prefix(Styled::new("↑").with_fg(accent_color))
        .with_scroll_down_prefix(Styled::new("↓").with_fg(accent_color))
}

/// Apply themed render config to MultiSelect as well
fn apply_theme_to_multiselect<T: std::fmt::Display>(multiselect: MultiSelect<T>) -> MultiSelect<T> {
    multiselect.with_render_config(get_themed_render_config())
}

pub async fn handle_run_tool(tool: &str, args: &[String]) -> Result<()> {
    // Check if NPM is available first
    if !InstallationManager::check_npm_available() {
        ProgressUtils::warning_message("Node.js runtime environment not detected");
        println!("  Most AI coding tools are distributed via the NPM ecosystem.");
        println!("  Please install Node.js to continue: https://nodejs.org/");
        return Err(anyhow!("Node.js runtime required"));
    }

    // Check if tool is installed with progress
    let check_progress = ProgressContext::new(&format!("Checking {tool} availability"));
    let cli_command = ToolManager::get_cli_command(tool);

    // Add a small delay to show the progress indicator
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    if !ToolManager::check_tool_installed(cli_command) {
        check_progress.finish_error(&format!("Tool '{tool}' is not installed"));

        let should_install = match Confirm::new(&format!("Install '{tool}' now?"))
            .with_render_config(get_themed_render_config())
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
            npm_check.finish_error("Node.js ecosystem not detected");
            println!("  Please install Node.js and NPM first: https://nodejs.org/");
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
                    "OpenCode requires shell environment refresh to update PATH",
                );
                ProgressUtils::info_message(
                    "Quick fix: Run 'source ~/.bashrc' or restart your terminal",
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
            let update_progress = ProgressContext::new(&format!("Updating {pkg}"));

            // Add a small delay to show the progress indicator
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            update_progress.update_message(&format!("Downloading latest version of {pkg}..."));

            let result = package_service.update_tool(pkg).await;

            match result {
                Ok(_) => {
                    update_progress.finish_success(&format!("{pkg} updated successfully"));
                    Ok(())
                }
                Err(e) => {
                    update_progress.finish_error(&format!("Failed to update {pkg}"));
                    Err(e)
                }
            }
        }
        None => {
            // This should not be called anymore from the updated menu, but keeping for compatibility
            let overall_progress = ProgressContext::new("Updating all packages");
            let tools = InstallationManager::get_tool_names();
            let mut had_errors = false;

            for (index, tool) in tools.iter().enumerate() {
                overall_progress.update_message(&format!(
                    "Updating {tool} ({}/{})...",
                    index + 1,
                    tools.len()
                ));

                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                if let Err(e) = package_service.update_tool(tool).await {
                    ProgressUtils::error_message(&format!("Failed to update {tool}: {e}"));
                    had_errors = true;
                } else {
                    ProgressUtils::success_message(&format!("{tool} updated successfully"));
                }
            }

            if had_errors {
                overall_progress.finish_error("Some packages failed to update");
            } else {
                overall_progress.finish_success("All packages updated successfully");
            }

            Ok(())
        }
    }
}

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

    if !InstallationManager::check_npm_available() {
        let theme = theme_config::current_theme();
        println!(
            "{} {}",
            theme.secondary("⚠ ADVISORY:"),
            theme.primary("Node.js ecosystem not detected")
        );
        println!("  Most AI tools are distributed via NPM. Install from: https://nodejs.org/");
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

    println!("Tool Information: {tool}\n");
    println!("Description: {}", install_info.description);
    println!("Command: {}", tool_info.command);
    println!(
        "Status: {}",
        if tool_info.is_installed {
            "Installed"
        } else {
            "Not installed"
        }
    );
    println!(
        "Installation: {} {}",
        install_info.command,
        install_info.args.join(" ")
    );

    if install_info.requires_npm {
        let npm_status = if InstallationManager::check_npm_available() {
            "Available"
        } else {
            "Not available"
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
        println!(" No templates found. Use 'terminal-jarvis templates create <name>' to create a template.");
    } else {
        for template in templates {
            println!(" - {template}");
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
    // Initialize theme configuration
    let _ = theme_config::initialize_theme_config();

    // Check NPM availability upfront
    let npm_available = InstallationManager::check_npm_available();

    loop {
        // Get fresh theme on each iteration to support theme switching
        let theme = theme_config::current_theme();
        // Clear screen first
        print!("\x1b[2J\x1b[H"); // Clear screen

        // Get terminal width for responsive design
        let term_width = if let Some((w, _)) = term_size::dimensions() {
            w
        } else {
            80 // fallback width
        };

        // Professional design with blue background in content areas - no initial line break
        // Use most of the terminal width for a more immersive experience
        let border_width = if term_width > 20 {
            term_width.saturating_sub(4) // Leave 2 chars padding on each side
        } else {
            term_width.saturating_sub(2) // Minimum border for very narrow terminals
        };
        let border_padding = format!("{}  {}", "\x1b[0m", "\x1b[0m"); // Explicit reset for padding spaces

        // Professional color scheme using T.JARVIS theme
        let top_border = format!(
            "{}╔{}╗{}",
            theme.colors.border,
            "═".repeat(border_width.saturating_sub(2)),
            theme.reset()
        );
        let bottom_border = format!(
            "{}╚{}╝{}",
            theme.colors.border,
            "═".repeat(border_width.saturating_sub(2)),
            theme.reset()
        );

        println!("{border_padding}{top_border}");

        // T.JARVIS ASCII art - professional and clean
        let logo_lines = vec![
      "$$$$$$$$\\           $$$$$\\  $$$$$$\\  $$$$$$$\\  $$\\    $$\\ $$$$$$\\  $$$$$$\\  ",
      "\\__$$  __|          \\__$$ |$$  __$$\\ $$  __$$\\ $$ |   $$ |\\_$$  _|$$  __$$\\ ",
      "   $$ |                $$ |$$ /  $$ |$$ |  $$ |$$ |   $$ |  $$ |  $$ /  \\__|",
      "   $$ |                $$ |$$$$$$$$ |$$$$$$$  |\\$$\\  $$  /  $$ |  \\$$$$$$\\  ",
      "   $$ |          $$\\   $$ |$$  __$$ |$$  __$$<  \\$$\\$$  /   $$ |   \\____$$\\ ",
      "   $$ |          $$ |  $$ |$$ |  $$ |$$ |  $$ |  \\$$$  /    $$ |  $$\\   $$ |",
      "   $$ |$$\\       \\$$$$$$  |$$ |  $$ |$$ |  $$ |   \\$  /   $$$$$$\\ \\$$$$$$  |",
      "   \\__|\\__|       \\______/ \\__|  \\__|\\__|  \\__|    \\_/    \\______| \\______/ ",
    ];

        let inner_width = border_width.saturating_sub(2); // Account for left and right border chars only

        // Helper function to print a line with proper border and background
        let print_border_line = |content: &str| {
            let full_line = theme.background_line_with_content(content, inner_width);
            println!(
                "{}{}║{}║{}",
                border_padding, theme.colors.border, full_line, theme.colors.border,
            );
        };

        // Add empty line before logo
        print_border_line("");

        for line in logo_lines {
            let line_chars: Vec<char> = line.chars().collect();
            let line_char_len = line_chars.len();

            if line_char_len <= inner_width {
                // Create content with logo color formatting
                let content = format!("{}{}", theme.logo_no_reset(""), line);
                print_border_line(&content);
            } else {
                let simple_line = "T.JARVIS";
                // Create content with logo color formatting
                let content = format!("{}{}", theme.logo_no_reset(""), simple_line);
                print_border_line(&content);
            }
        }

        // Add empty line after logo
        print_border_line("");

        // Add elegant separator line with theme colors
        let separator_content =
            theme.background_line_with_content(&"─".repeat(inner_width), inner_width);
        print_border_line(&separator_content);

        // Version and tagline in futuristic style - with NPM distribution tag if available
        let base_version = env!("CARGO_PKG_VERSION");

        // Initialize config manager for caching
        let config_manager = ConfigManager::new().unwrap_or_else(|_| {
            // Fallback if config manager fails - continue without caching
            ConfigManager::new().expect("Failed to create config manager")
        });

        // Show progress for NPM tag detection with caching
        let npm_progress = ProgressContext::new("Checking NPM distribution tags");
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
            // Use background_line_with_content for proper background fill
            let content = format!("{}{}", theme.secondary_no_reset(""), version_text);
            print_border_line(&content);
        }

        let tagline = "AI Coding Assistant Command Center";
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
            // Use background_line_with_content for proper background fill
            let content = format!("{}{}", theme.secondary_no_reset(""), tagline);
            print_border_line(&content);
        }

        // Add another elegant separator
        let separator_content2 =
            theme.background_line_with_content(&"─".repeat(inner_width), inner_width);
        print_border_line(&separator_content2);

        // Short hint about Important Links - shortened to fit border
        let links_hint = "See 'Important Links' menu";
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
            // Use background_line_with_content for proper background fill
            let content = format!("{}{}", theme.secondary_no_reset(""), links_hint);
            print_border_line(&content);
        }

        // Add empty line after links hint to match top padding
        print_border_line("");

        println!("{border_padding}{bottom_border}");
        println!();

        // Minimal setup warning
        if !npm_available {
            println!("Node.js required → https://nodejs.org/");
            println!();
        }

        // Main menu options - clean styling without redundant indicators
        let options = vec![
            "AI CLI Tools".to_string(),
            "Important Links".to_string(),
            "Settings".to_string(),
            "Exit".to_string(),
        ];

        let selection = match Select::new("Choose an option:", options.clone())
            .with_render_config(get_themed_render_config())
            .with_page_size(10)
            .prompt()
        {
            Ok(selection) => selection,
            Err(_) => {
                // User interrupted (Ctrl+C) - show clean exit message
                println!();
                println!("Goodbye!");
                return Ok(());
            }
        };

        // Handle selection
        match selection.as_str() {
            s if s.contains("AI CLI Tools") => {
                handle_ai_tools_menu().await?;
            }
            s if s.contains("Important Links") => {
                handle_important_links().await?;
            }
            s if s.contains("Settings") => {
                handle_manage_tools_menu().await?;
            }
            s if s.contains("Exit") => {
                print!("{}", theme.reset()); // Reset all formatting
                print!("\x1b[2J\x1b[H"); // Clear screen
                println!("Goodbye!");
                break;
            }
            _ => continue,
        }
    }
    // Ensure terminal is reset when function exits
    print!("\x1b[0m"); // Reset all formatting
    print!("\x1b[2J\x1b[H"); // Clear screen
    Ok(())
}

async fn handle_ai_tools_menu() -> Result<()> {
    loop {
        // Get fresh theme on each iteration to support theme switching
        let theme = theme_config::current_theme();

        print!("\x1b[2J\x1b[H"); // Clear screen

        println!("{}\n", theme.primary("AI CLI Tools"));

        // Show loading indicator
        let loading_progress = ProgressContext::new("Loading AI tools status");
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

        let tools = ToolManager::get_available_tools();

        loading_progress.finish_success("AI tools status loaded");

        // Build clean tool list
        let mut options = Vec::new();
        let mut tool_mapping = Vec::new();

        for (tool_name, _tool_info) in tools.iter() {
            options.push(tool_name.to_string());
            tool_mapping.push(Some(*tool_name));
        }

        // Add back option
        options.push("Back to Main Menu".to_string());
        tool_mapping.push(None);

        let selection = match Select::new("Select an AI tool to launch:", options.clone())
            .with_render_config(get_themed_render_config())
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
        if selection.contains("Back to Main Menu") {
            return Ok(());
        } else if let Some(index) = options.iter().position(|opt| opt == &selection) {
            if let Some(Some(tool_name)) = tool_mapping.get(index) {
                let tools = ToolManager::get_available_tools();
                let tool_info = tools.get(tool_name).unwrap();

                if !tool_info.is_installed {
                    let should_install = match Confirm::new(&format!(
                        "{} '{}' is not installed. Install it now?",
                        theme.accent("Tool"),
                        tool_name
                    ))
                    .with_render_config(get_themed_render_config())
                    .with_default(true)
                    .prompt()
                    {
                        Ok(result) => result,
                        Err(_) => {
                            // User interrupted - go back to main menu
                            println!("\n{}", theme.accent("Installation cancelled"));
                            return Ok(());
                        }
                    };

                    if should_install {
                        println!(
                            "\n{}",
                            theme.accent(&format!("Installing {}...", tool_name))
                        );
                        handle_install_tool(tool_name).await?;
                        println!("{}", theme.accent("Installation complete!\n"));
                    } else {
                        return Ok(());
                    }
                }

                let args_input = match Text::new(&format!(
                    "{}Enter arguments for {tool_name} (or press Enter for default):",
                    theme.primary("")
                ))
                .with_default("")
                .prompt()
                {
                    Ok(input) => input,
                    Err(_) => {
                        // User interrupted - go back to main menu
                        println!("\n{}", theme.accent("Operation cancelled"));
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

                launch_progress
                    .update_message(&format!("Starting {tool_name} with args: {args:?}"));
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
                        println!(
                            "\n{}",
                            theme.accent(&format!("{} completed successfully!", tool_name))
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "\n{}",
                            theme.accent(&format!("Error running {}: {}", tool_name, e))
                        );
                    }
                }

                // Enhanced exit options for faster context switching
                let exit_options = vec![
                    "Back to Main Menu".to_string(),
                    "Switch to Another AI Tool".to_string(),
                    "Exit Terminal Jarvis".to_string(),
                ];

                let exit_choice = match Select::new("What would you like to do next?", exit_options)
                    .with_render_config(get_themed_render_config())
                    .with_page_size(5)
                    .prompt()
                {
                    Ok(choice) => choice,
                    Err(_) => {
                        // User interrupted - return to main menu by default
                        return Ok(());
                    }
                };

                match exit_choice.as_str() {
                    s if s.contains("Back to Main Menu") => {
                        // Return to main menu - break out of AI tools submenu
                        return Ok(());
                    }
                    s if s.contains("Switch to Another AI Tool") => {
                        // Stay in AI tools menu for context switching - continue the loop
                        continue;
                    }
                    s if s.contains("Exit Terminal Jarvis") => {
                        // Exit completely - break out of everything
                        println!("{}", theme.accent("Goodbye!"));
                        std::process::exit(0);
                    }
                    _ => {
                        // Default to returning to main menu
                        return Ok(());
                    }
                }
            } // Close the inner if let Some(Some(tool_name)) block
        } // Close the else if let Some(index) block
    } // End of loop
}

async fn handle_important_links() -> Result<()> {
    loop {
        let theme = theme_config::current_theme();

        print!("\x1b[2J\x1b[H"); // Clear screen

        println!("{}\n", theme.accent("Important Links & Resources"));

        let options = vec![
            "GitHub Repository".to_string(),
            "NPM Package".to_string(),
            "CHANGELOG.md".to_string(),
            "Cargo Package".to_string(),
            "Documentation".to_string(),
            "Homebrew Formula".to_string(),
            "Back to Main Menu".to_string(),
        ];

        let selection = match Select::new("Choose a resource to view:", options)
            .with_render_config(get_themed_render_config())
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
                println!("\n{}", theme.accent("Opening GitHub Repository..."));
                println!("    https://github.com/BA-CalderonMorales/terminal-jarvis");
                println!("  - View source code and contribute");
                println!("  - Report issues and feature requests");
                println!("  - Check latest releases and changelogs");
            }
            s if s.contains("NPM") => {
                println!("\n{}", theme.accent("NPM Package Information..."));
                println!("    https://www.npmjs.com/package/terminal-jarvis");
                println!("  - Install: npm install -g terminal-jarvis");
                println!("  - Run: npx terminal-jarvis");
                println!("  - Version history and statistics");
            }
            s if s.contains("CHANGELOG") => {
                println!("\n{}", theme.secondary("CHANGELOG.md - Version History"));
                println!("    https://github.com/BA-CalderonMorales/terminal-jarvis/blob/main/CHANGELOG.md");
                println!("  - Detailed release notes for each version");
                println!("  - Feature additions and bug fixes");
                println!("  - Breaking changes and migration guides");
            }
            s if s.contains("Cargo") => {
                println!("\n{}", theme.accent("Cargo Package Information"));
                println!("    https://crates.io/crates/terminal-jarvis");
                println!("  - Install: cargo install terminal-jarvis");
                println!("  - Rust ecosystem integration");
                println!("  - Development dependencies and features");
            }
            s if s.contains("Documentation") => {
                println!("\n{}", theme.accent("Documentation Hub"));
                println!(
                    "    https://github.com/BA-CalderonMorales/terminal-jarvis/tree/main/docs"
                );
                println!("  - Architecture overview and design decisions");
                println!("  - Installation guides for all platforms");
                println!("  - Testing procedures and contribution guidelines");
                println!("  - Limitations and troubleshooting");
            }
            s if s.contains("Homebrew") => {
                println!("\n{}", theme.accent("Homebrew Formula"));
                println!(
                    "    https://github.com/BA-CalderonMorales/terminal-jarvis/tree/main/homebrew"
                );
                println!("  - macOS/Linux package management");
                println!("  - Install: brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis");
                println!("  - System integration and automatic updates");
            }
            s if s.contains("Back to Main Menu") => {
                // Exit loop and return to main menu
                return Ok(());
            }
            _ => {
                // Unknown option - continue loop
                continue;
            }
        }

        println!("\n{}", theme.accent("Press Enter to continue..."));
        let _ = std::io::stdin().read_line(&mut String::new());

        // Continue loop to return to Important Links menu
    }
}

async fn handle_manage_tools_menu() -> Result<()> {
    loop {
        let theme = theme_config::current_theme();

        print!("\x1b[2J\x1b[H"); // Clear screen

        println!("{}\n", theme.accent("Settings & Tools"));

        let options = vec![
            "Install Tools".to_string(),
            "Update Tools".to_string(),
            "List All Tools".to_string(),
            "Tool Information".to_string(),
            "Switch Theme".to_string(),
            "Back to Main Menu".to_string(),
        ];

        let selection = match Select::new("Choose an option:", options)
            .with_render_config(get_themed_render_config())
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
            s if s.contains("Install Tools") => {
                handle_install_tools_menu().await?;
            }
            s if s.contains("Update Tools") => {
                handle_update_tools_menu().await?;
            }
            s if s.contains("List All Tools") => {
                handle_list_tools().await?;
                println!("\n{}", theme.accent("Press Enter to continue..."));
                let _ = std::io::stdin().read_line(&mut String::new());
            }
            s if s.contains("Tool Information") => {
                handle_tool_info_menu().await?;
            }
            s if s.contains("Switch Theme") => {
                handle_theme_switch_menu().await?;
            }
            _ => {
                // Back to main menu
                return Ok(());
            }
        }
    }
}

async fn handle_install_tools_menu() -> Result<()> {
    // Check NPM availability with progress
    let npm_check = ProgressContext::new("Checking NPM availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    if !InstallationManager::check_npm_available() {
        npm_check.finish_error("Node.js ecosystem unavailable");
        ProgressUtils::error_message("Node.js runtime required. Install from: https://nodejs.org/");
        println!("  Download from: https://nodejs.org/");
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

    let tools_to_install = match apply_theme_to_multiselect(MultiSelect::new(
        "Select tools to install:",
        uninstalled_tools,
    ))
    .prompt()
    {
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
    let scan_progress = ProgressContext::new("Scanning for installed tools");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let installed_tools: Vec<String> = ToolManager::get_installed_tools()
        .into_iter()
        .map(String::from)
        .collect();

    if installed_tools.is_empty() {
        scan_progress.finish_error("No tools are installed yet");
        ProgressUtils::info_message("No tools are installed yet!");
        println!("  Install tools first using the main menu.");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

    scan_progress.finish_success(&format!("Found {} installed tools", installed_tools.len()));

    let mut options = installed_tools.clone();
    options.push("All Tools".to_string());

    let selection = match Select::new("Select tools to update:", options)
        .with_render_config(get_themed_render_config())
        .prompt()
    {
        Ok(selection) => selection,
        Err(_) => {
            // User interrupted - return to previous menu
            return Ok(());
        }
    };

    println!();
    if selection == "All Tools" {
        let update_progress = ProgressContext::new("Updating all installed tools concurrently");

        // Use concurrent updates for better performance
        let mut update_futures = Vec::new();

        for (index, tool) in installed_tools.iter().enumerate() {
            let tool_name = tool.clone();
            let total_tools = installed_tools.len();
            let current_index = index + 1;

            let future = async move {
                let individual_progress = ProgressContext::new(&format!(
                    "Updating {tool_name} ({current_index}/{total_tools})"
                ));
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                individual_progress
                    .update_message(&format!("Downloading latest version of {tool_name}..."));

                match handle_update_packages(Some(&tool_name)).await {
                    Ok(_) => {
                        individual_progress
                            .finish_success(&format!("{tool_name} updated successfully"));
                        Ok(tool_name)
                    }
                    Err(e) => {
                        individual_progress.finish_error(&format!("Failed to update {tool_name}"));
                        Err((tool_name, e))
                    }
                }
            };
            update_futures.push(future);
        }

        // Wait for all updates to complete
        let results = futures::future::join_all(update_futures).await;
        update_progress.finish_success("Concurrent updates completed");

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
        println!("\nUpdate Results Summary:");

        if !successful_tools.is_empty() {
            println!("Successfully updated {} tools:", successful_tools.len());
            for tool in &successful_tools {
                println!("  • {tool}");
            }
        }

        if !failed_tools.is_empty() {
            println!("\nFailed to update {} tools:", failed_tools.len());
            for (tool, error) in &failed_tools {
                println!("  • {tool}: {error}");
            }

            println!("\nTroubleshooting tips:");
            println!("  • Check your internet connection");
            println!(
                "  • Ensure you have the required package managers installed (npm, cargo, pip)"
            );
            println!("  • Some tools might need to be manually updated");
            println!("  • Try updating individual tools to see specific error messages");
        }

        if successful_tools.is_empty() && !failed_tools.is_empty() {
            println!("\nAll updates failed. Please check your environment setup.");
        } else if !successful_tools.is_empty() && !failed_tools.is_empty() {
            println!(
                "\nPartial success: {}/{} tools updated successfully",
                successful_tools.len(),
                installed_tools.len()
            );
        } else {
            println!("\n All tools updated successfully!");
        }
    } else {
        let update_progress = ProgressContext::new(&format!(" Updating {selection}"));
        match handle_update_packages(Some(&selection)).await {
            Ok(_) => {
                update_progress.finish_success(&format!("{selection} updated successfully"));
                println!("\n Update completed successfully!");
                println!(" {selection} is now up to date!");
            }
            Err(e) => {
                update_progress.finish_error(&format!("Failed to update {selection}"));
                println!("\n Update failed!");
                println!(" Error: {e}");
                println!("\nTroubleshooting tips:");
                println!("  • Check your internet connection");
                println!("  • Ensure the required package manager is installed");
                println!("  • Try running the update command manually");
                println!("  • Some tools may require manual updates");
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
    let tool = match Select::new("Select a tool for information:", tool_names)
        .with_render_config(get_themed_render_config())
        .prompt()
    {
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
      .with_render_config(get_themed_render_config())
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
    ProgressUtils::success_message(" Version cache cleared");

    Ok(())
}

pub async fn handle_cache_status() -> Result<()> {
    let config_manager = ConfigManager::new()?;

    match config_manager.load_version_cache()? {
        Some(cache) => {
            println!(" Version Cache Status:");
            println!(" Version Info: {}", cache.version_info);
            println!(" Cached at: {} (Unix timestamp)", cache.cached_at);
            println!(" TTL: {} seconds", cache.ttl_seconds);

            if cache.is_expired() {
                println!(" Status: Expired");
            } else {
                let remaining = cache.remaining_seconds();
                println!(" Status: Valid ({remaining} seconds remaining)");
            }
        }
        None => {
            println!(" No version cache found");
        }
    }

    Ok(())
}

pub async fn handle_cache_refresh(ttl: u64) -> Result<()> {
    let config_manager = ConfigManager::new()?;

    println!(" Refreshing version cache...");
    let latest_version_info =
        PackageService::get_cached_npm_dist_tag_info_with_ttl(&config_manager, ttl).await?;

    match latest_version_info {
        Some(version_info) => {
            ProgressUtils::success_message(&format!(
                " Cache refreshed with version info: {version_info} (TTL: {ttl}s)"
            ));
        }
        None => {
            ProgressUtils::warning_message(
                "Version caching unavailable - registry data incomplete",
            );
        }
    }

    Ok(())
}

async fn handle_theme_switch_menu() -> Result<()> {
    let current_theme = theme_config::current_theme();

    print!("\x1b[2J\x1b[H"); // Clear screen

    println!("{}\n", current_theme.accent("Theme Selection"));
    println!(
        "Current theme: {}\n",
        current_theme.primary(current_theme.name)
    );

    let theme_options = vec![
        "T.JARVIS (Professional Blue)".to_string(),
        "Classic (Terminal Default)".to_string(),
        "Matrix (Green on Black)".to_string(),
        "Back to Settings".to_string(),
    ];

    let selection = match Select::new("Choose a theme:", theme_options)
        .with_render_config(get_themed_render_config())
        .with_page_size(10)
        .prompt()
    {
        Ok(selection) => selection,
        Err(_) => return Ok(()),
    };

    match selection.as_str() {
        s if s.contains("T.JARVIS") => {
            theme_config::set_theme(crate::theme::ThemeType::TJarvis);
            println!("\n{}", current_theme.accent("Switched to T.JARVIS theme"));
        }
        s if s.contains("Classic") => {
            theme_config::set_theme(crate::theme::ThemeType::Classic);
            println!("\n{}", current_theme.accent("Switched to Classic theme"));
        }
        s if s.contains("Matrix") => {
            theme_config::set_theme(crate::theme::ThemeType::Matrix);
            println!("\n{}", current_theme.accent("Switched to Matrix theme"));
        }
        _ => return Ok(()),
    }

    println!(
        "{}",
        current_theme.secondary("Theme will take effect on next screen refresh.")
    );
    println!("\n{}", current_theme.accent("Press Enter to continue..."));
    let _ = std::io::stdin().read_line(&mut String::new());

    Ok(())
}
