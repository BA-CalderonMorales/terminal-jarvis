use crate::cli_logic::cli_logic_responsive_display::ResponsiveDisplay;
use crate::cli_logic::cli_logic_responsive_menu::create_themed_select;
use crate::config::ConfigManager;
use crate::installation_arguments::InstallationManager;
use crate::services::PackageService;
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

    // Clear screen and show welcome interface ONCE at startup
    print!("\x1b[2J\x1b[H");
    display_welcome_interface(&theme, npm_available).await?;

    // Add spacing before menu
    println!();

    loop {
        // Get fresh theme on each iteration to support theme switching
        let theme = theme_global_config::current_theme();

        let options = vec![
            "AI CLI Tools".to_string(),
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
        display_welcome_interface(&theme, npm_available).await?;
        println!();

        // Handle selection
        match selection.as_str() {
            s if s.contains("AI CLI Tools") => {
                print!("\x1b[2J\x1b[H");
                handle_ai_tools_menu().await?;
                // Redisplay welcome after submenu
                print!("\x1b[2J\x1b[H");
                display_welcome_interface(&theme, npm_available).await?;
                println!();
            }
            s if s.contains("Authentication") => {
                print!("\x1b[2J\x1b[H");
                crate::cli_logic::handle_authentication_menu().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_interface(&theme, npm_available).await?;
                println!();
            }
            s if s.contains("Important Links") => {
                print!("\x1b[2J\x1b[H");
                handle_important_links().await?;
                print!("\x1b[2J\x1b[H");
                display_welcome_interface(&theme, npm_available).await?;
                println!();
            }
            s if s.contains("Settings") => {
                print!("\x1b[2J\x1b[H");
                handle_manage_tools_menu().await?;
                print!("\x1b[2J\x1b[H");
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
    // Always create fresh display to detect current terminal size
    let display = ResponsiveDisplay::new();

    println!(); // Top spacing
    display.print_logo(theme);
    println!(); // Spacing after logo
    display_version_info_responsive(&display, theme).await?;
    println!(); // Spacing after version
    display.print_separator(theme);

    // Minimal setup hint
    let hint_text = if !npm_available {
        "[WARNING] Node.js required | See 'Important Links'"
    } else {
        "See 'Important Links' menu for documentation"
    };
    display.print_centered_text(theme, hint_text);
    display.print_separator(theme);
    println!(); // Bottom spacing

    Ok(())
}

/// Display version and tagline information using responsive system
async fn display_version_info_responsive(
    display: &ResponsiveDisplay,
    theme: &crate::theme::Theme,
) -> Result<()> {
    // Version and tagline in professional style - with NPM distribution tag if available
    let base_version = env!("CARGO_PKG_VERSION");

    // Initialize config manager for caching
    let config_manager = ConfigManager::new()
        .unwrap_or_else(|_| ConfigManager::new().expect("Failed to create config manager"));

    // Quick NPM tag detection with caching (no progress bar to reduce visual noise)
    let npm_tag = PackageService::get_cached_npm_dist_tag_info(&config_manager)
        .await
        .unwrap_or(None);

    let version_text = if let Some(tag) = npm_tag {
        format!("v{} (@{})", base_version, tag)
    } else {
        format!("v{}", base_version)
    };

    display.print_centered_text(theme, &version_text);

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
