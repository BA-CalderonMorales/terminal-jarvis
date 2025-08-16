// CLI Logic Entry Point
// This module coordinates all CLI business logic operations

use crate::cli_logic::cli_logic_utilities::{apply_theme_to_multiselect, get_themed_render_config};
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::ProgressContext;
use crate::theme::theme_global_config;
use crate::tools::ToolManager;
use anyhow::Result;
use inquire::{Confirm, MultiSelect, Select, Text};

// Re-export all the handler functions
pub use crate::cli_logic::cli_logic_config_management::*;
pub use crate::cli_logic::cli_logic_info_operations::*;
pub use crate::cli_logic::cli_logic_interactive::*;
pub use crate::cli_logic::cli_logic_list_operations::*;
pub use crate::cli_logic::cli_logic_template_operations::*;
pub use crate::cli_logic::cli_logic_tool_execution::*;
pub use crate::cli_logic::cli_logic_update_operations::*;

/// Handle the AI tools submenu for tool selection and launching
pub async fn handle_ai_tools_menu() -> Result<()> {
    loop {
        // Get fresh theme on each iteration to support theme switching
        let theme = theme_global_config::current_theme();

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
                handle_tool_launch(tool_name).await?;
            }
        }
    }
}

/// Handle launching a specific tool with argument input
async fn handle_tool_launch(tool_name: &str) -> Result<()> {
    let theme = theme_global_config::current_theme();
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

    launch_tool_with_progress(tool_name, &args).await?;
    handle_post_tool_exit().await
}

/// Launch a tool with detailed progress tracking
async fn launch_tool_with_progress(tool_name: &str, args: &[String]) -> Result<()> {
    let theme = theme_global_config::current_theme();

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
    if tool_name == "opencode" {
        // For opencode, we need extra time and careful terminal state management
        // to prevent input focus issues on fresh installs
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    } else {
        // Clear any remaining progress indicators for other tools
        print!("\x1b[2K\r");
    }

    match ToolManager::run_tool(tool_name, args).await {
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

    Ok(())
}

/// Handle user choice after tool exit
async fn handle_post_tool_exit() -> Result<()> {
    let theme = theme_global_config::current_theme();

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
            Ok(())
        }
        s if s.contains("Switch to Another AI Tool") => {
            // Stay in AI tools menu for context switching - this will continue the loop in handle_ai_tools_menu
            Ok(())
        }
        s if s.contains("Exit Terminal Jarvis") => {
            // Exit completely - break out of everything
            println!("{}", theme.accent("Goodbye!"));
            std::process::exit(0);
        }
        _ => {
            // Default to returning to main menu
            Ok(())
        }
    }
}

/// Handle the important links menu
pub async fn handle_important_links() -> Result<()> {
    loop {
        let theme = theme_global_config::current_theme();

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

        display_link_information(&selection, &theme);

        if selection.contains("Back to Main Menu") {
            return Ok(());
        }

        println!("\n{}", theme.accent("Press Enter to continue..."));
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}

/// Display information for a selected link
fn display_link_information(selection: &str, theme: &crate::theme::Theme) {
    match selection {
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
            println!(
                "    https://github.com/BA-CalderonMorales/terminal-jarvis/blob/main/CHANGELOG.md"
            );
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
            println!("    https://github.com/BA-CalderonMorales/terminal-jarvis/tree/main/docs");
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
        _ => {}
    }
}

/// Handle the settings and tools management menu
pub async fn handle_manage_tools_menu() -> Result<()> {
    loop {
        let theme = theme_global_config::current_theme();

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

/// Handle the theme switching interface
async fn handle_theme_switch_menu() -> Result<()> {
    let current_theme = theme_global_config::current_theme();

    print!("\x1b[2J\x1b[H"); // Clear screen

    println!("{}", current_theme.primary("Theme Selection"));
    println!(
        "{}",
        current_theme.secondary(&format!("Current theme: {}", current_theme.name))
    );
    println!();

    let theme_options = vec![
        "T.JARVIS".to_string(),
        "Classic".to_string(),
        "Matrix".to_string(),
    ];

    let selected_theme = match Select::new("Choose a theme:", theme_options)
        .with_render_config(get_themed_render_config())
        .prompt()
    {
        Ok(theme) => theme,
        Err(_) => {
            // User interrupted - return to previous menu
            return Ok(());
        }
    };

    // Apply the selected theme
    let theme_type = match selected_theme.as_str() {
        "T.JARVIS" => crate::theme::ThemeType::TJarvis,
        "Classic" => crate::theme::ThemeType::Classic,
        "Matrix" => crate::theme::ThemeType::Matrix,
        _ => crate::theme::ThemeType::TJarvis,
    };

    theme_global_config::set_theme(theme_type);
    let new_theme = theme_global_config::current_theme();
    println!(
        "\n{}",
        new_theme.primary(&format!("Theme changed to: {}", selected_theme))
    );
    println!("The new theme will be applied immediately.");

    println!("\n{}", current_theme.accent("Press Enter to continue..."));
    let _ = std::io::stdin().read_line(&mut String::new());

    Ok(())
}

// Additional menu handlers will be moved to separate domain files as the refactoring continues
async fn handle_install_tools_menu() -> Result<()> {
    // Implementation moved to a focused module - placeholder for now
    use crate::progress_utils::ProgressUtils;

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
    // Implementation will be moved to update operations module - simplified for now
    use crate::progress_utils::ProgressUtils;

    let installed_tools: Vec<String> = ToolManager::get_installed_tools()
        .into_iter()
        .map(String::from)
        .collect();

    if installed_tools.is_empty() {
        ProgressUtils::info_message("No tools are installed yet!");
        println!("  Install tools first using the main menu.");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

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
        handle_update_packages(None).await?;
    } else {
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
