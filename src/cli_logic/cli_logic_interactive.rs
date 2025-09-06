use crate::cli_logic::cli_logic_infinite_menu::infinite_hybrid_menu_select_with_header;
use crate::cli_logic::cli_logic_intro_screen::display_intro_screen;
use crate::installation_arguments::InstallationManager;
use crate::theme::theme_global_config;
use anyhow::Result;

/// Handle the main interactive mode interface with intro screen
pub async fn handle_interactive_mode() -> Result<()> {
    // Initialize theme configuration
    let _ = theme_global_config::initialize_theme_config();

    // Show intro screen first
    loop {
        let should_continue = display_intro_screen().await?;

        if !should_continue {
            // User chose screensaver mode or exit - stay in intro loop or exit
            continue;
        }

        // User chose to continue - enter main CLI menu
        handle_main_menu().await?;

        // After exiting main menu, return to intro screen
    }
}

/// Handle the main CLI menu interface
async fn handle_main_menu() -> Result<()> {
    // Check NPM availability upfront
    let npm_available = InstallationManager::check_npm_available();

    loop {
        // Get fresh theme on each iteration to support theme switching
        let theme = theme_global_config::current_theme();
        // Clear screen first
        print!("\x1b[2J\x1b[H"); // Clear screen

        // Just show a simple header instead of the full ASCII interface
        display_simple_header(&theme, npm_available).await?;

        // Main menu options - clean styling with back option
        let options = vec![
            "AI CLI Tools".to_string(),
            "Important Links".to_string(),
            "Settings".to_string(),
            "Back to Intro".to_string(),
            "Exit".to_string(),
        ];

        // Use the new infinite hybrid menu system with header preserved for main menu
        let selection =
            match infinite_hybrid_menu_select_with_header("Choose an option:", options.clone())
                .await
            {
                Ok(selection) => selection,
                Err(e) => {
                    if e.to_string().contains("RESIZE_REDRAW_NEEDED")
                        || e.to_string().contains("SIZE_CHANGE_REDRAW")
                    {
                        // Resize or size change detected - continue the loop to redraw welcome interface
                        continue;
                    } else {
                        // User interrupted (Ctrl+C) - show clean exit message
                        println!();
                        println!("Goodbye!");
                        return Ok(());
                    }
                }
            };

        // Handle selection
        match selection.to_lowercase().as_str() {
            s if s.contains("ai cli tools") || s == "tools" => {
                handle_ai_tools_menu().await?;
            }
            s if s.contains("important links") || s == "links" => {
                handle_important_links().await?;
            }
            s if s.contains("settings") || s == "settings" => {
                handle_manage_tools_menu().await?;
            }
            s if s.contains("back to intro") || s == "back" => {
                // Return to intro screen
                return Ok(());
            }
            s if s.contains("exit") || s == "exit" => {
                print!("{}", theme.colors.reset); // Reset all formatting
                print!("\x1b[2J\x1b[H"); // Clear screen
                println!("Goodbye!");
                std::process::exit(0); // Exit completely
            }
            _ => continue,
        }
    }
}

/// Display a simple header instead of the full ASCII interface
async fn display_simple_header(theme: &crate::theme::Theme, npm_available: bool) -> Result<()> {
    println!();
    println!("{}Terminal Jarvis - AI Tools Command Center{}", 
             theme.colors.primary_text, theme.colors.reset);
    println!("{}Version: {}{}", 
             theme.colors.secondary_text, env!("CARGO_PKG_VERSION"), theme.colors.reset);
    
    if !npm_available {
        println!();
        println!("{}⚠ Node.js required → https://nodejs.org/{}", 
                 theme.colors.accent_text, theme.colors.reset);
    }
    
    println!();
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
