use crate::cli_logic::cli_logic_welcome::display_welcome_screen;
use crate::installation_arguments::InstallationManager;
use crate::theme::theme_global_config;
use anyhow::Result;
use std::io::{self, Write};

/// Handle the main interactive mode interface
pub async fn handle_interactive_mode() -> Result<()> {
    // Initialize theme configuration
    let _ = theme_global_config::initialize_theme_config();

    // Export saved credentials at session start so tools inherit API keys
    let _ = crate::auth_manager::AuthManager::export_saved_env_vars();

    // Check NPM availability upfront
    let npm_available = InstallationManager::check_npm_available();

    // Get theme for welcome screen
    let theme = theme_global_config::current_theme();

    // Clear screen and show welcome screen ONCE at startup
    print!("\x1b[2J\x1b[H");
    display_welcome_screen();

    // Show additional interface info
    display_welcome_interface(&theme, npm_available).await?;

    loop {
        // Get fresh theme on each iteration to support theme switching
        let theme = theme_global_config::current_theme();

        // Simple command prompt
        print!("\n{} ", theme.primary(">"));
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let selection = input.trim().to_string();

        // Handle empty input
        if selection.is_empty() {
            continue;
        }

        // Handle slash commands
        match selection.as_str() {
            "/tools" => {
                print!("\x1b[2J\x1b[H");
                handle_ai_tools_menu().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                display_available_commands(&theme);
            }
            "/evals" => {
                print!("\x1b[2J\x1b[H");
                if let Err(e) = crate::cli_logic::cli_logic_evals_operations::show_evals_menu() {
                    eprintln!("Error in Evals menu: {}", e);
                }
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                display_available_commands(&theme);
            }
            "/auth" => {
                print!("\x1b[2J\x1b[H");
                crate::cli_logic::handle_authentication_menu().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                display_available_commands(&theme);
            }
            "/links" => {
                print!("\x1b[2J\x1b[H");
                handle_important_links().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                display_available_commands(&theme);
            }
            "/settings" => {
                print!("\x1b[2J\x1b[H");
                handle_manage_tools_menu().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                display_available_commands(&theme);
            }
            "/exit" | "/quit" => {
                print!("{}", theme.reset());
                print!("\x1b[2J\x1b[H");
                println!("Goodbye!");
                break;
            }
            "/help" => {
                display_available_commands(&theme);
            }
            _ => {
                println!("\n{} Unknown command: {}", theme.accent("[!]"), selection);
                println!("Type {} to see available commands", theme.accent("/help"));
            }
        }
    }
    // Ensure terminal is reset when function exits
    print!("\x1b[0m"); // Reset all formatting
    Ok(())
}

/// Display the welcome interface with T.JARVIS branding using responsive system
async fn display_welcome_interface(theme: &crate::theme::Theme, npm_available: bool) -> Result<()> {
    // Show warning only if npm is not available (tip is now part of ASCII art)
    if !npm_available {
        println!(
            "  {}",
            theme.secondary("[WARNING] Node.js required - see Important Links")
        );
        println!(); // Bottom spacing
    }

    Ok(())
}

/// Display available slash commands
fn display_available_commands(theme: &crate::theme::Theme) {
    println!();
    println!("{}", theme.accent("Available Commands:"));
    println!("  {}  - AI CLI Tools", theme.secondary("/tools"));
    println!("  {}  - Evals & Comparisons", theme.secondary("/evals"));
    println!("  {}   - Authentication", theme.secondary("/auth"));
    println!("  {}  - Important Links", theme.secondary("/links"));
    println!("  {}  - Settings", theme.secondary("/settings"));
    println!("  {}   - Show this help", theme.secondary("/help"));
    println!("  {}   - Exit Terminal Jarvis", theme.secondary("/exit"));
}

// Forward declarations for menu functions that will be in other modules
async fn handle_ai_tools_menu() -> Result<()> {
    // This will be implemented in a separate function
    crate::cli_logic::handle_ai_tools_menu().await
}

async fn handle_important_links() -> Result<()> {
    // This will be implemented in a separate function
    crate::cli_logic::handle_important_links().await
}

async fn handle_manage_tools_menu() -> Result<()> {
    // This will be implemented in a separate function
    crate::cli_logic::handle_manage_tools_menu().await
}
