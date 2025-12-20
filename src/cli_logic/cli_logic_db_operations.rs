// CLI Logic Database Operations
//
// Handlers for database management commands:
// - Import TOML configurations
// - Show database status
// - Reset database

use crate::db::{DatabaseManager, TomlImporter};
use crate::theme::theme_global_config;
use anyhow::Result;
use std::io::{self, Write};

/// Handle database import command
pub async fn handle_db_import() -> Result<()> {
    let theme = theme_global_config::current_theme();

    println!();
    println!(
        "{}",
        theme.primary("Database Import - TOML to libSQL Migration")
    );
    println!();

    // Initialize database
    println!("{}", theme.secondary("Initializing database..."));
    let db = DatabaseManager::init().await?;

    // Create importer and run
    println!(
        "{}",
        theme.secondary("Scanning config/tools/ for TOML files...")
    );
    let importer = TomlImporter::new(db).await?;
    let stats = importer.import_all().await?;

    // Display results
    println!();
    for result in &stats.results {
        let status = if result.success {
            theme.accent("[OK]")
        } else {
            theme.secondary("[SKIP]")
        };
        println!("  {} {} - {}", status, result.tool_id, result.message);
    }

    println!();
    println!("{}", theme.primary(&stats.summary()));

    if stats.all_success() && stats.imported > 0 {
        println!();
        println!(
            "{}",
            theme.accent("Tools imported! Use 'terminal-jarvis db status' to verify.")
        );
    }

    Ok(())
}

/// Handle database status command
pub async fn handle_db_status() -> Result<()> {
    let theme = theme_global_config::current_theme();

    println!();
    println!("{}", theme.primary("Database Status"));
    println!();

    // Check if database exists
    if let Some(db_path) = DatabaseManager::get_db_path() {
        if db_path.exists() {
            let metadata = std::fs::metadata(&db_path)?;
            let size_kb = metadata.len() / 1024;

            println!("  {} {}", theme.secondary("Path:"), db_path.display());
            println!("  {} {} KB", theme.secondary("Size:"), size_kb);

            // Initialize and get stats
            let db = DatabaseManager::init().await?;
            let tools_repo = crate::db::ToolsRepository::new(db.clone());
            let prefs_repo = crate::db::PreferencesRepository::new(db);

            let tool_count = tools_repo.count().await?;
            let tools = tools_repo.find_all().await?;

            println!();
            println!("  {}", theme.primary("Tables:"));
            println!("    {} {} tools", theme.secondary("tools:"), tool_count);

            if !tools.is_empty() {
                println!();
                println!("  {}", theme.primary("Stored Tools:"));
                for tool in tools.iter().take(10) {
                    let status = if tool.enabled { "[+]" } else { "[-]" };
                    println!(
                        "    {} {} ({})",
                        theme.accent(status),
                        tool.display_name,
                        tool.cli_command
                    );
                }
                if tools.len() > 10 {
                    println!("    ... and {} more", tools.len() - 10);
                }
            }

            // Show preferences
            if let Ok(Some(last_tool)) = prefs_repo.get_last_used_tool().await {
                println!();
                println!("  {}", theme.primary("Preferences:"));
                println!("    {} {}", theme.secondary("Last used:"), last_tool);
            }
        } else {
            println!(
                "  {}",
                theme.secondary("Database not initialized. Run 'terminal-jarvis db import' first.")
            );
        }
    } else {
        println!("  {}", theme.accent("Could not determine database path."));
    }

    println!();
    Ok(())
}

/// Handle database reset command
pub async fn handle_db_reset(force: bool) -> Result<()> {
    let theme = theme_global_config::current_theme();

    if let Some(db_path) = DatabaseManager::get_db_path() {
        if !db_path.exists() {
            println!(
                "{}",
                theme.secondary("Database does not exist. Nothing to reset.")
            );
            return Ok(());
        }

        if !force {
            println!();
            println!(
                "{}",
                theme.accent("WARNING: This will delete all data in the database!")
            );
            println!("  Path: {}", db_path.display());
            println!();
            print!("Type 'yes' to confirm: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() != "yes" {
                println!("{}", theme.secondary("Reset cancelled."));
                return Ok(());
            }
        }

        // Delete the database file
        std::fs::remove_file(&db_path)?;
        println!("{}", theme.primary("Database reset successfully."));
        println!(
            "{}",
            theme.secondary("Run 'terminal-jarvis db import' to reimport tool configurations.")
        );
    } else {
        println!("{}", theme.accent("Could not determine database path."));
    }

    Ok(())
}

/// Handle interactive database menu (for /db command)
pub async fn handle_db_menu() -> Result<()> {
    use crate::cli_logic::cli_logic_responsive_menu::create_themed_select;

    loop {
        let theme = theme_global_config::current_theme();

        print!("\x1b[2J\x1b[H"); // Clear screen

        println!("{}\n", theme.primary("Database Management"));

        // Check database status
        let has_db = DatabaseManager::get_db_path()
            .map(|p| p.exists())
            .unwrap_or(false);

        let tool_count = if has_db {
            if let Ok(db) = DatabaseManager::init().await {
                let repo = crate::db::ToolsRepository::new(db);
                repo.count().await.unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        };

        println!(
            "  {} {}",
            theme.secondary("Status:"),
            if has_db {
                format!("{} tools in database", tool_count)
            } else {
                "Not initialized".to_string()
            }
        );
        println!();

        let options = vec![
            "Import TOML Configs".to_string(),
            "View Database Status".to_string(),
            "Reset Database".to_string(),
            "Back to Main Menu".to_string(),
        ];

        let selection = match create_themed_select(&theme, "Select an action:", options.clone())
            .with_page_size(10)
            .prompt()
        {
            Ok(s) => s,
            Err(_) => return Ok(()),
        };

        match selection.as_str() {
            "Import TOML Configs" => {
                handle_db_import().await?;
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            "View Database Status" => {
                handle_db_status().await?;
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            "Reset Database" => {
                handle_db_reset(false).await?;
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            "Back to Main Menu" => return Ok(()),
            _ => {}
        }
    }
}
