use crate::cli_logic::cli_logic_autocomplete::{get_autocomplete, SlashCommandSuggester};
use crate::cli_logic::cli_logic_first_run::{is_first_run, run_first_time_wizard};
use crate::cli_logic::cli_logic_utilities::get_autocomplete_render_config;
use crate::cli_logic::cli_logic_welcome::display_welcome_screen;
use crate::db::core::connection::DatabaseManager;
use crate::installation_arguments::InstallationManager;
use crate::theme::{theme_global_config, theme_persistence, ThemeType};
use anyhow::Result;
use inquire::Text;

/// Handle the main interactive mode interface
pub async fn handle_interactive_mode() -> Result<()> {
    // Initialize database and load theme preference
    let db = DatabaseManager::init().await.ok();

    let saved_theme = if let Some(ref db) = db {
        theme_persistence::load_theme_preference(db.clone())
            .await
            .ok()
            .flatten()
    } else {
        None
    };

    // Initialize theme configuration with saved preference or default
    let theme_type = saved_theme.unwrap_or(ThemeType::TJarvis);
    let _ = theme_global_config::initialize_theme_config_with(theme_type);

    // Export saved credentials at session start so tools inherit API keys
    let _ = crate::auth_manager::AuthManager::export_saved_env_vars();

    // Run first-time wizard if this is a new installation
    if is_first_run() {
        run_first_time_wizard().await?;
    }

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
        // Get fresh theme on each iteration
        let theme = theme_global_config::current_theme();

        // Use inquire with autocomplete for slash command input
        let input = Text::new(">")
            .with_autocomplete(SlashCommandSuggester)
            .with_render_config(get_autocomplete_render_config())
            .with_help_message("Type / for commands, Tab to autocomplete")
            .prompt();

        let raw_input = match input {
            Ok(s) => s.trim().to_string(),
            Err(inquire::InquireError::OperationCanceled) => {
                // Ctrl+C pressed, show help
                println!("\n{}", theme.secondary("Type /exit to quit"));
                continue;
            }
            Err(inquire::InquireError::OperationInterrupted) => {
                // Ctrl+D or similar
                break;
            }
            Err(_) => continue,
        };

        // Handle empty input
        if raw_input.is_empty() {
            continue;
        }

        // Extract command from "command - description" format if selected from autocomplete
        let command = if raw_input.contains(" - ") {
            raw_input
                .split(" - ")
                .next()
                .unwrap_or(&raw_input)
                .to_string()
        } else {
            raw_input
        };

        // Resolve aliases to canonical commands
        let selection = {
            let ac = get_autocomplete();
            if let Ok(ac_guard) = ac.lock() {
                ac_guard
                    .resolve_command(&command)
                    .map(|s| s.to_string())
                    .unwrap_or(command.clone())
            } else {
                command.clone()
            }
        };

        if execute_command(&selection, &theme, npm_available).await? {
            break; // Exit signal received
        }
    }

    // Ensure terminal is reset when function exits
    print!("\x1b[0m");
    Ok(())
}

/// Execute a resolved command. Returns true if should exit.
async fn execute_command(
    selection: &str,
    theme: &crate::theme::Theme,
    npm_available: bool,
) -> Result<bool> {
    match selection {
        "/tools" => {
            print!("\x1b[2J\x1b[H");
            handle_ai_tools_menu().await?;
            refresh_screen(theme, npm_available).await?;
        }
        "/evals" => {
            print!("\x1b[2J\x1b[H");
            if let Err(e) = crate::cli_logic::cli_logic_evals_operations::show_evals_menu() {
                eprintln!("Error in Evals menu: {}", e);
            }
            refresh_screen(theme, npm_available).await?;
        }
        "/auth" => {
            print!("\x1b[2J\x1b[H");
            crate::cli_logic::handle_authentication_menu().await?;
            refresh_screen(theme, npm_available).await?;
        }
        "/links" => {
            print!("\x1b[2J\x1b[H");
            handle_important_links().await?;
            refresh_screen(theme, npm_available).await?;
        }
        "/settings" => {
            print!("\x1b[2J\x1b[H");
            handle_manage_tools_menu().await?;
            refresh_screen(theme, npm_available).await?;
        }
        "/db" => {
            print!("\x1b[2J\x1b[H");
            crate::cli_logic::handle_db_menu().await?;
            refresh_screen(theme, npm_available).await?;
        }
        "/theme" => {
            print!("\x1b[2J\x1b[H");
            handle_theme_selection().await?;
            refresh_screen(theme, npm_available).await?;
        }
        "/exit" | "/quit" => {
            print!("{}", theme.reset());
            print!("\x1b[0m");
            print!("\x1b[2J\x1b[H");
            println!("Goodbye!");
            return Ok(true);
        }
        "/help" => {
            display_available_commands(theme);
        }
        _ => {
            println!("\n{} Unknown command: {}", theme.accent("[!]"), selection);
            println!(
                "Type {} or press Tab for suggestions",
                theme.accent("/help")
            );
        }
    }
    Ok(false)
}

/// Refresh screen after submenu returns
async fn refresh_screen(theme: &crate::theme::Theme, npm_available: bool) -> Result<()> {
    print!("\x1b[2J\x1b[H");
    display_welcome_screen();
    display_welcome_interface(theme, npm_available).await?;
    display_available_commands(theme);
    Ok(())
}

/// Display the welcome interface
async fn display_welcome_interface(theme: &crate::theme::Theme, npm_available: bool) -> Result<()> {
    if !npm_available {
        println!(
            "  {}",
            theme.secondary("[WARNING] Node.js required - see Important Links")
        );
        println!();
    }
    Ok(())
}

/// Display available slash commands
fn display_available_commands(theme: &crate::theme::Theme) {
    println!();
    println!("{}", theme.accent("Available Commands:"));
    println!(
        "  {} {} - AI CLI Tools",
        theme.secondary("/tools"),
        theme.secondary("(/t)")
    );
    println!(
        "  {} {} - Evals & Comparisons",
        theme.secondary("/evals"),
        theme.secondary("(/e)")
    );
    println!(
        "  {}  {} - Authentication",
        theme.secondary("/auth"),
        theme.secondary("(/a)")
    );
    println!(
        "  {} {} - Important Links",
        theme.secondary("/links"),
        theme.secondary("(/l)")
    );
    println!(
        "  {} {} - Settings",
        theme.secondary("/settings"),
        theme.secondary("(/s)")
    );
    println!("  {}    - Database Management", theme.secondary("/db"));
    println!("  {} - Change UI Theme", theme.secondary("/theme"));
    println!(
        "  {}  {} - Show this help",
        theme.secondary("/help"),
        theme.secondary("(/h)")
    );
    println!(
        "  {}  {} - Exit",
        theme.secondary("/exit"),
        theme.secondary("(/q)")
    );
    println!();
    println!(
        "{}",
        theme.secondary("Tip: Type / then Tab for autocomplete, arrows to navigate")
    );
}

/// Handle theme selection menu
async fn handle_theme_selection() -> Result<()> {
    use crate::db::core::connection::DatabaseManager;
    use crate::theme::{theme_global_config::ThemeConfig, theme_persistence};
    use inquire::Select;

    let theme = theme_global_config::current_theme();
    let current_type = theme_global_config::current_theme_type();

    println!();
    println!("{}", theme.accent("Theme Selection"));
    println!();

    // Build options with current indicator
    let themes = ThemeConfig::available_themes();
    let options: Vec<String> = themes
        .iter()
        .map(|(name, t)| {
            if *t == current_type {
                format!("{} [current]", name)
            } else {
                name.to_string()
            }
        })
        .collect();

    let selection = Select::new("Select a theme:", options.clone()).prompt();

    match selection {
        Ok(selected) => {
            // Find the selected theme
            let selected_name = selected.trim_end_matches(" [current]");
            if let Some((_, theme_type)) = themes.iter().find(|(name, _)| *name == selected_name) {
                // Update in-memory
                theme_global_config::set_theme(*theme_type);

                // Persist to database
                if let Ok(db) = DatabaseManager::init().await {
                    if let Err(e) = theme_persistence::save_theme_preference(db, *theme_type).await
                    {
                        eprintln!("Warning: Could not save theme preference: {}", e);
                    }
                }

                let new_theme = theme_global_config::current_theme();
                println!();
                println!(
                    "{} Theme changed to: {}",
                    new_theme.accent("[OK]"),
                    new_theme.primary(selected_name)
                );
            }
        }
        Err(_) => {
            println!("{}", theme.secondary("Theme selection cancelled"));
        }
    }

    Ok(())
}

// Forward declarations for menu functions
async fn handle_ai_tools_menu() -> Result<()> {
    crate::cli_logic::handle_ai_tools_menu().await
}

async fn handle_important_links() -> Result<()> {
    crate::cli_logic::handle_important_links().await
}

async fn handle_manage_tools_menu() -> Result<()> {
    crate::cli_logic::handle_manage_tools_menu().await
}
