// CLI Logic Entry Point
// This module coordinates all CLI business logic operations

use crate::cli_logic::cli_logic_responsive_menu::create_themed_select;
use crate::cli_logic::cli_logic_utilities::{apply_theme_to_multiselect, get_themed_render_config};
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::ProgressContext;
use crate::theme::theme_global_config;
use crate::tools::ToolManager;
use anyhow::Result;
use inquire::{Confirm, MultiSelect, Text};

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

        let selection =
            match create_themed_select(&theme, "Select an AI tool to launch:", options.clone())
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

    // Always launch the tool and show post-tool menu regardless of success/failure
    let _result = launch_tool_with_progress(tool_name, &args).await;
    handle_post_tool_exit(tool_name, &args).await
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
            // For aider specifically, be more graceful with error handling
            if tool_name == "aider" {
                println!(
                    "\n{}",
                    theme.accent("Aider session ended. Some compatibility issues may occur with uv-installed tools.")
                );
            } else {
                eprintln!(
                    "\n{}",
                    theme.accent(&format!("Error running {}: {}", tool_name, e))
                );
            }
        }
    }

    Ok(())
}

/// Handle user choice after tool exit
/// Adds an option to immediately reopen the last tool with the same arguments
async fn handle_post_tool_exit(last_tool: &str, last_args: &[String]) -> Result<()> {
    fn initial_case(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    loop {
        let theme = theme_global_config::current_theme();

        // Enhanced exit options for faster context switching (numbered)
        let tool_display = initial_case(last_tool);
        let exit_options = vec![
            format!("1. Reopen {}", tool_display),
            "2. Back to Main Menu".to_string(),
            "3. Switch to Another AI Tool".to_string(),
            "4. Exit Terminal Jarvis".to_string(),
        ];

        let exit_choice =
            match create_themed_select(&theme, "What would you like to do next?", exit_options)
                .with_page_size(6)
                .prompt()
            {
                Ok(choice) => choice,
                Err(_) => {
                    // User interrupted - return to main menu by default
                    return Ok(());
                }
            };

        match exit_choice.as_str() {
            s if s.contains("Reopen ") => {
                // Relaunch the same tool with the same args, then show this menu again
                let _ = launch_tool_with_progress(last_tool, last_args).await;
                continue;
            }
            s if s.contains("Back to Main Menu") => {
                // Return to main menu - break out of AI tools submenu
                return Ok(());
            }
            s if s.contains("Switch to Another AI Tool") => {
                // Stay in AI tools menu for context switching - this will continue the loop in handle_ai_tools_menu
                return Ok(());
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

        let selection =
            match create_themed_select(&theme, "Choose a resource to view:", options).prompt() {
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
            "Authentication".to_string(),
            "Switch Theme".to_string(),
            "Back to Main Menu".to_string(),
        ];

        let selection = match create_themed_select(&theme, "Choose an option:", options).prompt() {
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
            s if s.contains("Authentication") => {
                crate::cli_logic::handle_authentication_menu().await?;
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

    let theme = theme_global_config::current_theme();
    let selected_theme =
        match create_themed_select(&theme, "Choose a theme:", theme_options).prompt() {
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

    // Check installer prerequisites with progress (npm, uv, curl)
    let npm_check = ProgressContext::new("Checking NPM availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
    if !InstallationManager::check_npm_available() {
        npm_check.finish_error("Node.js ecosystem unavailable");
        ProgressUtils::error_message("Node.js runtime required. Install from: https://nodejs.org/");
        println!("  Download from: https://nodejs.org/");
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }
    npm_check.finish_success("NPM is available");

    let curl_check = ProgressContext::new("Checking curl availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    if InstallationManager::check_curl_available() {
        curl_check.finish_success("curl is available");
    } else {
        curl_check.finish_error("curl not found");
        ProgressUtils::info_message(
            "Some tools (e.g., Goose) use curl-based installers. Please install curl via your package manager.",
        );
        // Optional: Offer auto-install via apt-get if available
        let apt_exists = std::process::Command::new("which")
            .arg("apt-get")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if apt_exists {
            use inquire::Confirm;
            if Confirm::new("Install curl now using apt-get?")
                .with_default(false)
                .prompt()
                .unwrap_or(false)
            {
                let sudo_available = std::process::Command::new("which")
                    .arg("sudo")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false);
                use tokio::process::Command as AsyncCommand;
                let mut install = if sudo_available {
                    let mut c = AsyncCommand::new("sudo");
                    c.arg("apt-get");
                    c
                } else {
                    AsyncCommand::new("apt-get")
                };
                let _ = install
                    .args(["update"]) // update first (best effort)
                    .status()
                    .await;
                let mut install = if sudo_available {
                    let mut c = AsyncCommand::new("sudo");
                    c.arg("apt-get");
                    c
                } else {
                    AsyncCommand::new("apt-get")
                };
                let status = install
                    .args(["install", "-y", "curl"]) // non-interactive
                    .status()
                    .await;
                match status {
                    Ok(s) if s.success() => ProgressUtils::success_message("curl installed."),
                    _ => ProgressUtils::error_message("Failed to install curl with apt-get."),
                }
            }
        }
    }

    let uv_check = ProgressContext::new("Checking uv availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let uv_available = InstallationManager::check_uv_available();
    if uv_available {
        uv_check.finish_success("uv is available");
    } else {
        uv_check.finish_error("uv not found");
        ProgressUtils::info_message(
            "Some tools (e.g., Aider) use uv for installation. Install uv: https://docs.astral.sh/uv/getting-started/installation/",
        );
        // Offer to install uv automatically if curl is available
        if InstallationManager::check_curl_available() {
            use inquire::Confirm;
            if Confirm::new("Install uv now via official script?")
                .with_default(false)
                .prompt()
                .unwrap_or(false)
            {
                // Download and run uv installer: curl -LsSf https://astral.sh/uv/install.sh | sh
                use tokio::process::Command as AsyncCommand;
                let url = "https://astral.sh/uv/install.sh";
                let curl_output = AsyncCommand::new("curl")
                    .args(["-LsSf", url])
                    .output()
                    .await;
                match curl_output {
                    Ok(out) if out.status.success() => {
                        let mut sh = AsyncCommand::new("sh");
                        sh.stdin(std::process::Stdio::piped());
                        sh.stdout(std::process::Stdio::null());
                        sh.stderr(std::process::Stdio::null());
                        if let Ok(mut child) = sh.spawn() {
                            if let Some(stdin) = child.stdin.as_mut() {
                                use tokio::io::AsyncWriteExt;
                                let _ = stdin.write_all(&out.stdout).await;
                            }
                            let _ = child.wait().await;
                        }
                        // Re-check availability
                        if InstallationManager::check_uv_available() {
                            ProgressUtils::success_message("uv installed successfully. You may need to restart your shell for PATH updates.");
                        } else {
                            ProgressUtils::warning_message("uv installation finished but not detected on PATH yet. Try restarting your terminal.");
                        }
                    }
                    _ => {
                        ProgressUtils::error_message(
                            "Failed to download uv installer. See docs link above.",
                        );
                    }
                }
            }
        }
    }

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

    let theme = theme_global_config::current_theme();
    let selection = match create_themed_select(&theme, "Select tools to update:", options).prompt()
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
    loop {
        // Get fresh theme on each iteration
        let theme = theme_global_config::current_theme();

        print!("\x1b[2J\x1b[H"); // Clear screen
        println!("{}\n", theme.primary("Tool Information"));

        let tool_names: Vec<String> = InstallationManager::get_tool_names().into_iter().collect();

        // Add back option to the tool list
        let mut options = tool_names.clone();
        options.push("Back to Settings Menu".to_string());

        let selection = match create_themed_select(
            &theme,
            "Select a tool for information:",
            options,
        )
        .prompt()
        {
            Ok(selection) => selection,
            Err(_) => {
                // User interrupted - return to previous menu
                return Ok(());
            }
        };

        // Handle selection
        if selection.contains("Back to Settings Menu") {
            return Ok(());
        } else {
            println!();
            handle_tool_info(&selection).await?;

            println!("\n{}", theme.accent("Press Enter to continue..."));
            std::io::stdin().read_line(&mut String::new())?;
            // Loop continues, returning to Tool Information menu
        }
    }
}
