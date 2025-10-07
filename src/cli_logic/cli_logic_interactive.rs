use crate::cli_logic::cli_logic_responsive_menu::create_themed_select;
use crate::cli_logic::cli_logic_welcome::display_welcome_screen;
use crate::installation_arguments::InstallationManager;
use crate::theme::theme_global_config;
use anyhow::Result;

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

    // Add spacing before menu
    println!();

    loop {
        // Get fresh theme on each iteration to support theme switching
        let theme = theme_global_config::current_theme();

        let options = vec![
            "AI CLI Tools".to_string(),
            "Evals & Comparisons".to_string(),
            "Authentication".to_string(),
            "Important Links".to_string(),
            "Settings".to_string(),
            "Exit".to_string(),
        ];

        // Use inquire for inline menu rendering (no alternate screen)
        let selection =
            match create_themed_select(&theme, "Choose an option:", options.clone()).prompt() {
                Ok(selection) => selection,
                Err(_) => {
                    // User interrupted (Ctrl+C) - show clean exit message
                    println!("\nGoodbye!");
                    return Ok(());
                }
            };

        // Clear screen and redisplay welcome for next iteration
        print!("\x1b[2J\x1b[H");
        display_welcome_screen();
        display_welcome_interface(&theme, npm_available).await?;
        println!();

        // Handle selection
        match selection.as_str() {
            s if s.contains("AI CLI Tools") => {
                print!("\x1b[2J\x1b[H");
                handle_ai_tools_menu().await?;
                // Redisplay welcome after submenu
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                println!();
            }
            s if s.contains("Evals & Comparisons") => {
                print!("\x1b[2J\x1b[H");
                if let Err(e) = crate::cli_logic::cli_logic_evals_operations::show_evals_menu() {
                    eprintln!("Error in Evals menu: {}", e);
                }
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                println!();
            }
            s if s.contains("Evals & Comparisons") => {
                print!("\x1b[2J\x1b[H");
                if let Err(e) = crate::cli_logic::cli_logic_evals_operations::show_evals_menu() {
                    eprintln!("Error in Evals menu: {}", e);
                }
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                println!();
            }
            s if s.contains("Authentication") => {
                print!("\x1b[2J\x1b[H");
                crate::cli_logic::handle_authentication_menu().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                println!();
            }
            s if s.contains("Important Links") => {
                print!("\x1b[2J\x1b[H");
                handle_important_links().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                println!();
            }
            s if s.contains("Settings") => {
                print!("\x1b[2J\x1b[H");
                handle_manage_tools_menu().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_screen();
                display_welcome_interface(&theme, npm_available).await?;
                println!();
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
