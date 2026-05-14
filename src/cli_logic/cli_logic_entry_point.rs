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
    let Some(tool_info) = tools.get(tool_name) else {
        println!(
            "\n{}: Tool '{}' not found in available tools",
            theme.accent("Error"),
            tool_name
        );
        return Ok(());
    };

    if !tool_info.is_installed && !install_missing_tool_if_requested(tool_name, tool_info).await? {
        return Ok(());
    }

    // Issue #26 Fix: Skip args prompt - launch immediately with defaults
    // Most users just want to start the tool quickly without extra prompts
    // Args can be passed via CLI: `terminal-jarvis run claude -- <args>`
    let args: Vec<String> = vec![];

    // Always launch the tool and show post-tool menu regardless of success/failure
    let _result = launch_tool_with_progress(tool_name, &args).await;
    handle_post_tool_exit(tool_name, &args).await
}

async fn install_missing_tool_if_requested(
    tool_name: &str,
    tool_info: &crate::tools::ToolInfo,
) -> Result<bool> {
    let theme = theme_global_config::current_theme();
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
        wait_for_enter(&theme);
        return Ok(false);
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
            println!("\n{}", theme.accent("Installation cancelled"));
            return Ok(false);
        }
    };

    if !should_install {
        return Ok(false);
    }

    println!("\n{}", theme.accent(&format!("Installing {tool_name}...")));
    if let Err(e) = handle_install_tool(tool_name).await {
        println!("\n{}", theme.accent(&format!("Installation failed: {e}")));
        println!(
            "{}",
            theme.secondary("You can try again or check the requirements.")
        );
        wait_for_enter(&theme);
        return Ok(false);
    }

    println!("{}", theme.accent("Installation complete!\n"));
    Ok(true)
}

fn wait_for_enter(theme: &crate::theme::Theme) {
    println!("\n{}", theme.secondary("Press Enter to continue..."));
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn command_exists(command: &str) -> bool {
    std::process::Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn apt_get_install_command() -> tokio::process::Command {
    use tokio::process::Command as AsyncCommand;

    if command_exists("sudo") {
        let mut command = AsyncCommand::new("sudo");
        command.arg("apt-get");
        command
    } else {
        AsyncCommand::new("apt-get")
    }
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

/// Handle user choice after tool exit
async fn handle_post_tool_exit(last_tool: &str, last_args: &[String]) -> Result<()> {
    loop {
        let theme = theme_global_config::current_theme();
        let tool_display = initial_case(last_tool);

        let exit_options = post_tool_exit_options(last_tool);

        let exit_choice = match themed_select_with(&theme, "What next?", exit_options).prompt() {
            Ok(choice) => choice,
            Err(_) => return Ok(()),
        };

        match exit_choice.as_str() {
            s if s.starts_with("Reopen ") => {
                let _ = launch_tool_with_progress(last_tool, last_args).await;
                continue;
            }
            s if s.starts_with("Update ") => {
                handle_post_tool_update(last_tool, &tool_display, last_args).await?;
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

fn post_tool_exit_options(last_tool: &str) -> Vec<String> {
    let tool_display = initial_case(last_tool);
    let mut exit_options = vec![format!("Reopen {tool_display}")];

    if InstallationManager::get_update_command(last_tool).is_some() {
        exit_options.push(format!("Update {tool_display}"));
    }

    exit_options.push("Back to Home".to_string());
    exit_options.push("Exit".to_string());

    exit_options
}

async fn handle_post_tool_update(
    last_tool: &str,
    tool_display: &str,
    last_args: &[String],
) -> Result<()> {
    let theme = theme_global_config::current_theme();
    println!();

    match handle_update_packages(Some(last_tool)).await {
        Ok(()) => {
            let reopen = themed_confirm(&format!("Reopen updated {tool_display}?"))
                .with_default(true)
                .prompt()
                .unwrap_or_default();

            if reopen {
                let _ = launch_tool_with_progress(last_tool, last_args).await;
            }
        }
        Err(err) => {
            println!(
                "{}",
                theme.secondary(&format!(
                    "Update failed for {tool_display}: {err}. Returning to menu."
                ))
            );
        }
    }

    Ok(())
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
            "Configuration Path".to_string(),
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
            s if s.contains("Configuration Path") => {
                handle_config_path_menu().await?;
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
async fn check_npm_for_install_menu() -> bool {
    use crate::progress_utils::ProgressUtils;

    let npm_check = ProgressContext::new("Checking NPM availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
    let npm_available = InstallationManager::check_npm_available();

    if npm_available {
        npm_check.finish_success("NPM is available");
        return true;
    }

    npm_check.finish_error("Node.js not available - npm tools cannot be installed");
    ProgressUtils::info_message(
        "Install Node.js from: https://nodejs.org/ to enable npm-based tools",
    );
    false
}

async fn ensure_curl_for_install_menu() {
    use crate::progress_utils::ProgressUtils;

    let curl_check = ProgressContext::new("Checking curl availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    if InstallationManager::check_curl_available() {
        curl_check.finish_success("curl is available");
        return;
    }

    curl_check.finish_error("curl not found");
    ProgressUtils::info_message(
        "Some tools (e.g., Goose) use curl-based installers. Please install curl via your package manager.",
    );

    if !command_exists("apt-get") {
        return;
    }

    let should_install = themed_confirm("Install curl now using apt-get?")
        .with_default(false)
        .prompt()
        .unwrap_or(false);
    if !should_install {
        return;
    }

    install_curl_with_apt_get().await;
}

async fn install_curl_with_apt_get() {
    use crate::progress_utils::ProgressUtils;

    let _ = apt_get_install_command().args(["update"]).status().await;
    let status = apt_get_install_command()
        .args(["install", "-y", "curl"])
        .status()
        .await;

    match status {
        Ok(status) if status.success() => ProgressUtils::success_message("curl installed."),
        _ => ProgressUtils::error_message("Failed to install curl with apt-get."),
    }
}

async fn ensure_uv_for_install_menu() {
    use crate::progress_utils::ProgressUtils;

    let uv_check = ProgressContext::new("Checking uv availability");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    if InstallationManager::check_uv_available() {
        uv_check.finish_success("uv is available");
        return;
    }

    uv_check.finish_error("uv not found");
    ProgressUtils::info_message(
        "Some tools (e.g., Aider) use uv for installation. Install uv: https://docs.astral.sh/uv/getting-started/installation/",
    );

    let should_install = InstallationManager::check_curl_available()
        && themed_confirm("Install uv now via official script?")
            .with_default(false)
            .prompt()
            .unwrap_or(false);
    if !should_install {
        return;
    }

    install_uv_with_official_script().await;
}

async fn install_uv_with_official_script() {
    use crate::progress_utils::ProgressUtils;
    use tokio::process::Command as AsyncCommand;

    let curl_output = AsyncCommand::new("curl")
        .args(["-LsSf", "https://astral.sh/uv/install.sh"])
        .output()
        .await;
    let Ok(output) = curl_output else {
        ProgressUtils::error_message("Failed to download uv installer. See docs link above.");
        return;
    };

    if !output.status.success() {
        ProgressUtils::error_message("Failed to download uv installer. See docs link above.");
        return;
    }

    let mut shell = AsyncCommand::new("sh");
    shell.stdin(std::process::Stdio::piped());
    shell.stdout(std::process::Stdio::null());
    shell.stderr(std::process::Stdio::null());
    if let Ok(mut child) = shell.spawn() {
        if let Some(stdin) = child.stdin.as_mut() {
            use tokio::io::AsyncWriteExt;
            let _ = stdin.write_all(&output.stdout).await;
        }
        let _ = child.wait().await;
    }

    if InstallationManager::check_uv_available() {
        ProgressUtils::success_message(
            "uv installed successfully. You may need to restart your shell for PATH updates.",
        );
    } else {
        ProgressUtils::warning_message(
            "uv installation finished but not detected on PATH yet. Try restarting your terminal.",
        );
    }
}

fn filter_installable_tools(tools: Vec<String>, npm_available: bool) -> Vec<String> {
    if npm_available {
        return tools;
    }

    tools
        .into_iter()
        .filter(|tool| {
            InstallationManager::get_install_command(tool)
                .map(|command| !command.requires_npm)
                .unwrap_or(false)
        })
        .collect()
}

async fn handle_install_tools_menu() -> Result<()> {
    // Implementation moved to a focused module - placeholder for now
    use crate::progress_utils::ProgressUtils;

    let npm_available = check_npm_for_install_menu().await;
    ensure_curl_for_install_menu().await;
    ensure_uv_for_install_menu().await;

    // Check which tools are uninstalled with progress
    let check_progress = ProgressContext::new("Scanning for uninstalled tools");
    tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;

    let uninstalled_tools =
        filter_installable_tools(ToolManager::get_uninstalled_tools(), npm_available);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_tool_exit_options_include_update_for_update_capable_tool() {
        let options = post_tool_exit_options("claude");

        assert_eq!(options[0], "Reopen Claude");
        assert_eq!(options[1], "Update Claude");
        assert!(options.contains(&"Back to Home".to_string()));
        assert!(options.contains(&"Exit".to_string()));
    }

    #[test]
    fn post_tool_exit_options_skip_update_when_tool_has_no_update_command() {
        let options = post_tool_exit_options("__missing_tool__");

        assert_eq!(
            options,
            vec!["Reopen __missing_tool__", "Back to Home", "Exit"]
        );
    }

    #[test]
    fn filter_installable_tools_keeps_all_tools_when_npm_is_available() {
        let tools = vec!["claude".to_string(), "goose".to_string()];

        assert_eq!(filter_installable_tools(tools.clone(), true), tools);
    }

    #[test]
    fn filter_installable_tools_hides_npm_tools_when_npm_is_unavailable() {
        let tools = vec![
            "gemini".to_string(),
            "goose".to_string(),
            "aider".to_string(),
        ];
        let filtered = filter_installable_tools(tools, false);

        assert!(!filtered.contains(&"gemini".to_string()));
        assert!(filtered.contains(&"goose".to_string()));
        assert!(filtered.contains(&"aider".to_string()));
    }
}
