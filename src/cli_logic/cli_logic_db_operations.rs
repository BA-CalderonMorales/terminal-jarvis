// CLI Logic Database Operations
//
// Handlers for database management commands:
// - Import TOML configurations
// - Show database status
// - Reset database
// - Credential management

use crate::db::{
    Credential, CredentialsRepository, DatabaseManager, TomlImporter, ToolsRepository,
};
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

    // Create importer and run tool configs
    println!(
        "{}",
        theme.secondary("Scanning config/tools/ for TOML files...")
    );
    let importer = TomlImporter::new(db.clone()).await?;
    let stats = importer.import_all().await?;

    // Display tool import results
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

    // Also import credentials if any exist
    println!();
    println!("{}", theme.secondary("Checking for saved credentials..."));
    let creds_repo = CredentialsRepository::new(db);
    let creds_stats = creds_repo.import_from_toml().await?;

    if creds_stats.imported > 0 {
        println!("  {} {}", theme.accent("[OK]"), creds_stats.summary());
    } else {
        println!(
            "  {}",
            theme.secondary("No saved credentials found to import")
        );
    }

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
            let prefs_repo = crate::db::PreferencesRepository::new(db.clone());

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

            // Show credentials summary
            let creds_repo = CredentialsRepository::new(db);
            let all_creds = creds_repo.get_all_as_map().await?;
            if !all_creds.is_empty() {
                println!();
                println!("  {}", theme.primary("Stored Credentials:"));
                for (tool_id, vars) in all_creds.iter() {
                    println!(
                        "    {} {} ({} keys)",
                        theme.accent("[+]"),
                        tool_id,
                        vars.len()
                    );
                }
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
            "Manage Credentials".to_string(),
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
            "Manage Credentials" => {
                handle_credentials_menu().await?;
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

/// Handle credentials management menu
pub async fn handle_credentials_menu() -> Result<()> {
    use crate::cli_logic::cli_logic_responsive_menu::create_themed_select;
    use inquire::Text;

    loop {
        let theme = theme_global_config::current_theme();

        print!("\x1b[2J\x1b[H"); // Clear screen

        println!("{}\n", theme.primary("Credential Management"));

        // Get current credentials
        let db = DatabaseManager::init().await?;
        let creds_repo = CredentialsRepository::new(db.clone());
        let tools_repo = ToolsRepository::new(db);
        let all_creds = creds_repo.get_all_as_map().await?;
        let all_tools = tools_repo.find_all().await?;

        // Show configured tools
        println!("  {}", theme.secondary("Configured tools:"));
        if all_creds.is_empty() {
            println!("    {}", theme.secondary("(none)"));
        } else {
            for (tool_id, vars) in &all_creds {
                let var_names: Vec<&str> = vars.keys().map(|s| s.as_str()).collect();
                println!(
                    "    {} {} - {}",
                    theme.accent("[+]"),
                    tool_id,
                    var_names.join(", ")
                );
            }
        }
        println!();

        // Show tools without credentials
        let unconfigured: Vec<_> = all_tools
            .iter()
            .filter(|t| !all_creds.contains_key(&t.id))
            .collect();

        if !unconfigured.is_empty() {
            println!("  {}", theme.secondary("Tools without credentials:"));
            for tool in unconfigured.iter().take(5) {
                println!("    {} {}", theme.secondary("[-]"), tool.display_name);
            }
            if unconfigured.len() > 5 {
                println!("    ... and {} more", unconfigured.len() - 5);
            }
            println!();
        }

        let options = vec![
            "Add/Update Credential".to_string(),
            "View Credentials".to_string(),
            "Remove Credential".to_string(),
            "Import from TOML".to_string(),
            "Back".to_string(),
        ];

        let selection = match create_themed_select(&theme, "Select an action:", options.clone())
            .with_page_size(10)
            .prompt()
        {
            Ok(s) => s,
            Err(_) => return Ok(()),
        };

        match selection.as_str() {
            "Add/Update Credential" => {
                // Select tool
                let mut tool_options: Vec<String> = all_tools
                    .iter()
                    .map(|t| format!("{} ({})", t.display_name, t.id))
                    .collect();
                tool_options.push("Cancel".to_string());

                let tool_selection =
                    match create_themed_select(&theme, "Select tool:", tool_options.clone())
                        .with_page_size(12)
                        .prompt()
                    {
                        Ok(s) => s,
                        Err(_) => continue,
                    };

                if tool_selection == "Cancel" {
                    continue;
                }

                // Extract tool id from selection
                let tool_id = tool_selection
                    .split('(')
                    .next_back()
                    .and_then(|s| s.strip_suffix(')'))
                    .unwrap_or(&tool_selection);

                // Get env var name
                let env_var = match Text::new(
                    "Environment variable name (e.g., ANTHROPIC_API_KEY):",
                )
                .prompt()
                {
                    Ok(v) if !v.is_empty() => v,
                    _ => continue,
                };

                // Get value (masked input)
                let value = match Text::new(&format!("Value for {}:", env_var)).prompt() {
                    Ok(v) if !v.is_empty() => v,
                    _ => continue,
                };

                // Save to database
                let cred = Credential::builder(tool_id, &env_var).value(&value).build();

                let db = DatabaseManager::init().await?;
                let creds_repo = CredentialsRepository::new(db);
                creds_repo.save(&cred).await?;

                println!("\n{}", theme.accent("Credential saved!"));
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            "View Credentials" => {
                println!();
                if all_creds.is_empty() {
                    println!("{}", theme.secondary("No credentials stored."));
                } else {
                    for (tool_id, vars) in &all_creds {
                        println!("  {} {}:", theme.primary("[Tool]"), tool_id);
                        for (env_var, value) in vars {
                            // Mask the value
                            let masked = if value.len() > 8 {
                                format!("{}...{}", &value[..4], &value[value.len() - 4..])
                            } else {
                                "****".to_string()
                            };
                            println!("    {} = {}", env_var, masked);
                        }
                    }
                }
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            "Remove Credential" => {
                if all_creds.is_empty() {
                    println!("\n{}", theme.secondary("No credentials to remove."));
                } else {
                    // Build list of tool/var pairs
                    let mut remove_options: Vec<String> = Vec::new();
                    for (tool_id, vars) in &all_creds {
                        for env_var in vars.keys() {
                            remove_options.push(format!("{} / {}", tool_id, env_var));
                        }
                    }
                    remove_options.push("Cancel".to_string());

                    let remove_selection = match create_themed_select(
                        &theme,
                        "Select credential to remove:",
                        remove_options,
                    )
                    .with_page_size(12)
                    .prompt()
                    {
                        Ok(s) => s,
                        Err(_) => continue,
                    };

                    if remove_selection != "Cancel" {
                        let parts: Vec<&str> = remove_selection.split(" / ").collect();
                        if parts.len() == 2 {
                            let db = DatabaseManager::init().await?;
                            let creds_repo = CredentialsRepository::new(db);
                            creds_repo.delete(parts[0], parts[1]).await?;
                            println!("\n{}", theme.accent("Credential removed!"));
                        }
                    }
                }
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            "Import from TOML" => {
                let db = DatabaseManager::init().await?;
                let creds_repo = CredentialsRepository::new(db);
                let stats = creds_repo.import_from_toml().await?;
                println!("\n{}", theme.primary(&stats.summary()));
                println!("\nPress Enter to continue...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            "Back" => return Ok(()),
            _ => {}
        }
    }
}
