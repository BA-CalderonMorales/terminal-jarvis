// CLI Logic Entry Point
// This module coordinates all CLI business logic operations

use crate::cli_logic::cli_logic_welcome::display_welcome_screen;
use crate::cli_logic::themed_components::{themed_confirm, themed_multiselect, themed_select_with};
use crate::installation_arguments::InstallationManager;
use crate::progress_utils::ProgressContext;
use crate::theme::theme_global_config;
use crate::tools::tools_display::ToolDisplayFormatter;
use crate::tools::ToolManager;
use anyhow::Result;

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

        // Clear screen completely and reset terminal state
        print!("\x1b[2J\x1b[H\x1b[?25h");
        std::io::Write::flush(&mut std::io::stdout()).unwrap_or_default();

        println!("{}\n", theme.primary("AI CLI Tools"));

        // Load tools without progress indicator (fixes rendering artifacts)
        let tools = ToolManager::get_available_tools_async().await;

        // Check for missing requirements and show advisory
        let tools_vec: Vec<_> = tools.iter().map(|(n, t)| (n.clone(), t.clone())).collect();
        let missing = ToolDisplayFormatter::get_missing_requirements(&tools_vec);
        if !missing.is_empty() {
            for (_, msg) in &missing {
                println!("{}", theme.accent(&format!("  ⚠ {}", msg)));
            }
            println!();
        }

        // Build tool list with requirement hints
        let mut options = Vec::new();
        let mut tool_mapping: Vec<Option<String>> = Vec::new();

        for (tool_name, tool_info) in tools.iter() {
            let formatted = ToolDisplayFormatter::format_menu_item(tool_name, tool_info);
            options.push(formatted);
            tool_mapping.push(Some(tool_name.clone()));
        }

        // Add back option
        options.push("Back to Main Menu".to_string());
        tool_mapping.push(None);

        let selection =
            match themed_select_with(&theme, "Select an AI tool to launch:", options.clone())
                .prompt()
            {
                Ok(selection) => selection,
                Err(_) => {
                    // User interrupted (Ctrl+C) - return to main menu
                    return Ok(());
                }
            };

        // Handle selection - extract tool name from formatted option
        if selection.contains("Back to Main Menu") {
            return Ok(());
        } else if let Some(index) = options.iter().position(|opt| opt == &selection) {
            if let Some(Some(tool_name)) = tool_mapping.get(index) {
                // Handle errors gracefully - don't crash the menu
                if let Err(e) = handle_tool_launch(tool_name).await {
                    eprintln!("\n{}", theme.accent(&format!("Error: {e}")));
                    println!("{}", theme.secondary("Press Enter to return to menu..."));
                    let _ = std::io::stdin().read_line(&mut String::new());
                }
            }
        }
    }
}

/// Handle launching a specific tool - streamlined for fewer Enter presses
/// Per Issue #26: Skip args prompt for most cases, launch immediately
async fn handle_tool_launch(tool_name: &str) -> Result<()> {
    let theme = theme_global_config::current_theme();

    // Use async version that checks database first
    let tools = ToolManager::get_available_tools_async().await;
    let tool_info = match tools.get(tool_name) {
        Some(info) => info,
        None => {
            println!(
                "\n{}: Tool '{}' not found in available tools",
                theme.accent("Error"),
                tool_name
            );
            return Ok(());
        }
    };

    if !tool_info.is_installed {
        // Check if required package manager is available before prompting install
        if !tool_info.package_manager.is_available() {
            let pm_label = tool_info.package_manager.label();
            let install_hint = tool_info.package_manager.install_hint();

            println!(
                "\n{}",
                theme.accent(&format!(
                    "Cannot install '{}': {} is not available on your system.",
                    tool_name, pm_label
                ))
            );
            println!("{}", theme.secondary(install_hint));
            println!("\n{}", theme.secondary("Press Enter to continue..."));
            let _ = std::io::stdin().read_line(&mut String::new());
            return Ok(());
        }

        let should_install = match themed_confirm(&format!(
            "{} '{}' is not installed. Install it now?",
            theme.accent("Tool"),
            tool_name
        ))
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
            println!("\n{}", theme.accent(&format!("Installing {tool_name}...")));
            match handle_install_tool(tool_name).await {
                Ok(_) => {
                    println!("{}", theme.accent("Installation complete!\n"));
                }
                Err(e) => {
                    // Show error gracefully and return to menu instead of crashing
                    println!("\n{}", theme.accent(&format!("Installation failed: {e}")));
                    println!(
                        "{}",
                        theme.secondary("You can try again or check the requirements.")
                    );
                    println!("\n{}", theme.secondary("Press Enter to continue..."));
                    let _ = std::io::stdin().read_line(&mut String::new());
                    return Ok(());
                }
            }
        } else {
            return Ok(());
        }
    }

    // Issue #26 Fix: Skip args prompt - launch immediately with defaults
    // Most users just want to start the tool quickly without extra prompts
    // Args can be passed via CLI: `terminal-jarvis run claude -- <args>`
    let args: Vec<String> = vec![];

    // Always launch the tool and show post-tool menu regardless of success/failure
    let _result = launch_tool_with_progress(tool_name, &args).await;
    handle_post_tool_exit(tool_name, &args).await
}

/// Launch a tool with progress tracking
async fn launch_tool_with_progress(tool_name: &str, args: &[String]) -> Result<()> {
    let theme = theme_global_config::current_theme();

    let launch_progress = ProgressContext::new(&format!("Launching {tool_name}"));
    launch_progress.finish_success(&format!("Starting {tool_name}"));

    if !crate::cli_logic::cli_logic_headless::is_headless() {
        print!("\x1b[2K\r");
    }

    match ToolManager::run_tool(tool_name, args).await {
        Ok(_) => {
            println!(
                "\n{}",
                theme.accent(&format!("{tool_name} completed successfully!"))
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
                    theme.accent(&format!("Error running {tool_name}: {e}"))
                );
            }
        }
    }

    Ok(())
}

/// Capitalize the first letter of a string
fn initial_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Handle user choice after tool exit -- streamlined to 3 options
async fn handle_post_tool_exit(last_tool: &str, last_args: &[String]) -> Result<()> {
    loop {
        let theme = theme_global_config::current_theme();
        let tool_display = initial_case(last_tool);

        let exit_options = vec![
            format!("Reopen {tool_display}"),
            "Back to Home".to_string(),
            "Exit".to_string(),
        ];

        let exit_choice = match themed_select_with(&theme, "What next?", exit_options).prompt() {
            Ok(choice) => choice,
            Err(_) => return Ok(()),
        };

        match exit_choice.as_str() {
            s if s.starts_with("Reopen ") => {
                let _ = launch_tool_with_progress(last_tool, last_args).await;
                continue;
            }
            "Back to Home" => return Ok(()),
            "Exit" => {
                println!("{}", theme.accent("Goodbye!"));
                std::process::exit(0);
            }
            _ => return Ok(()),
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
            match themed_select_with(&theme, "Choose a resource to view:", options).prompt() {
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

        let selection = match themed_select_with(&theme, "Choose an option:", options).prompt() {
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
        "Default".to_string(),
        "Minimal".to_string(),
        "Terminal".to_string(),
    ];

    let theme = theme_global_config::current_theme();
    let selected_theme = match themed_select_with(&theme, "Choose a theme:", theme_options).prompt()
    {
        Ok(theme) => theme,
        Err(_) => {
            // User interrupted - return to previous menu
            return Ok(());
        }
    };

    // Apply the selected theme
    let theme_type = match selected_theme.as_str() {
        "Default" => crate::theme::ThemeType::TJarvis,
        "Minimal" => crate::theme::ThemeType::Classic,
        "Terminal" => crate::theme::ThemeType::Matrix,
        _ => crate::theme::ThemeType::TJarvis,
    };

    theme_global_config::set_theme(theme_type);
    let new_theme = theme_global_config::current_theme();

    // Clear screen and redraw with new theme to show immediate visual update
    print!("\x1b[2J\x1b[H");

    // Display welcome screen with new theme so ASCII art and welcome text update immediately
    display_welcome_screen();
    println!();

    println!(
        "{}",
        new_theme.primary(&format!("Theme changed to: {selected_theme}"))
    );
    println!(
        "{}",
        new_theme.secondary("The new theme has been applied immediately.")
    );
    println!(
        "{}",
        new_theme.secondary("All menu text will now use the new theme colors.")
    );

    println!("\n{}", new_theme.accent("Press Enter to continue..."));
    let _ = std::io::stdin().read_line(&mut String::new());

    Ok(())
}

// Additional menu handlers will be moved to separate domain files as the refactoring continues
async fn handle_install_tools_menu() -> Result<()> {
    // Implementation moved to a focused module - placeholder for now
    use crate::progress_utils::ProgressUtils;

    // Check installer prerequisites with progress (npm, uv, curl).
    // NOTE: A missing npm is a WARNING, not a hard stop. Curl/uv tools are
    // still installable without npm. We filter out npm tools below when npm
    // is unavailable so users can still install everything else.
    let npm_check = ProgressContext::new("Checking NPM availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
    let npm_available = InstallationManager::check_npm_available();
    if !npm_available {
        npm_check.finish_error("Node.js not available - npm tools cannot be installed");
        ProgressUtils::info_message(
            "Install Node.js from: https://nodejs.org/ to enable npm-based tools",
        );
    } else {
        npm_check.finish_success("NPM is available");
    }

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
        if apt_exists
            && themed_confirm("Install curl now using apt-get?")
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
        if InstallationManager::check_curl_available()
            && themed_confirm("Install uv now via official script?")
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

    // Check which tools are uninstalled with progress
    let check_progress = ProgressContext::new("Scanning for uninstalled tools");
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;

    let all_uninstalled = ToolManager::get_uninstalled_tools();

    // When npm is unavailable, exclude npm-dependent tools so users can still
    // install curl/uv tools. This is the key behaviour: prefer native downloads
    // (curl) and don't block the whole menu on a missing npm.
    let uninstalled_tools: Vec<String> = if npm_available {
        all_uninstalled
    } else {
        all_uninstalled
            .into_iter()
            .filter(|tool| {
                InstallationManager::get_install_command(tool)
                    .map(|cmd| !cmd.requires_npm)
                    .unwrap_or(false)
            })
            .collect()
    };

    if uninstalled_tools.is_empty() {
        if npm_available {
            check_progress.finish_success("All tools are already installed");
            ProgressUtils::success_message("All tools are already installed!");
        } else {
            check_progress.finish_success("All curl/uv installable tools are already installed");
            ProgressUtils::info_message(
                "Install Node.js to unlock additional npm-based tools: https://nodejs.org/",
            );
        }
        println!("Press Enter to continue...");
        std::io::stdin().read_line(&mut String::new())?;
        return Ok(());
    }

    let available_count = uninstalled_tools.len();
    check_progress.finish_success(&format!(
        "Found {available_count} tools available for installation{}",
        if !npm_available {
            " (npm tools hidden - Node.js not installed)"
        } else {
            ""
        }
    ));

    let tools_to_install =
        match themed_multiselect("Select tools to install:", uninstalled_tools).prompt() {
            Ok(tools) => tools,
            Err(_) => {
                // User interrupted - return to previous menu
                return Ok(());
            }
        };

    println!();
    for tool in &tools_to_install {
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

    let installed_tools: Vec<String> = ToolManager::get_installed_tools();

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
    let selection = match themed_select_with(&theme, "Select tools to update:", options).prompt() {
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

        let selection =
            match themed_select_with(&theme, "Select a tool for information:", options).prompt() {
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
