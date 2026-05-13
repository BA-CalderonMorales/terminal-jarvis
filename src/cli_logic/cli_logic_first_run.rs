// CLI Logic First-Run Experience
// Handles first-run detection, tool discovery, and onboarding wizard

use crate::db::{DatabaseManager, PreferencesRepository, TomlImporter};
use crate::theme::theme_global_config;
use crate::tools::ToolManager;
use anyhow::Result;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Get the Terminal Jarvis config directory path
pub fn get_config_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".terminal-jarvis"))
}

/// Get the initialization marker file path
fn get_init_marker_path() -> Option<PathBuf> {
    get_config_dir().map(|d| d.join("initialized"))
}

/// Check if this is the first run (no initialization marker exists)
pub fn is_first_run() -> bool {
    get_init_marker_path().map(|p| !p.exists()).unwrap_or(true)
}

/// Mark the application as initialized
pub fn mark_initialized() -> Result<()> {
    if let Some(config_dir) = get_config_dir() {
        fs::create_dir_all(&config_dir)?;
        let marker_path = config_dir.join("initialized");
        fs::write(&marker_path, chrono::Utc::now().to_rfc3339())?;
    }
    Ok(())
}

/// Get detected tools information for display
pub struct ToolDetectionResult {
    pub installed: Vec<String>,
    pub available: Vec<String>,
}

/// Detect all tools and categorize them
pub fn detect_tools() -> ToolDetectionResult {
    let installed = ToolManager::get_installed_tools();
    let available = ToolManager::get_uninstalled_tools();

    ToolDetectionResult {
        installed,
        available,
    }
}

/// Display the first-run welcome wizard (3 screens max)
pub async fn run_first_time_wizard() -> Result<()> {
    let theme = theme_global_config::current_theme();

    // Screen 1: Welcome
    print!("\x1b[2J\x1b[H");
    println!();
    println!("{}", theme.primary("   Welcome to Terminal Jarvis!"));
    println!();
    println!(
        "   {}",
        theme.secondary("Your unified command center for AI coding tools.")
    );
    println!();
    println!("   Terminal Jarvis helps you:");
    println!(
        "   {} Manage multiple AI CLI tools from one place",
        theme.accent("[+]")
    );
    println!(
        "   {} Handle authentication seamlessly",
        theme.accent("[+]")
    );
    println!("   {} Keep your tools updated", theme.accent("[+]"));
    println!();

    print!("   Press Enter to continue...");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Screen 2: Tool Detection
    print!("\x1b[2J\x1b[H");
    let detection = detect_tools();
    display_detected_tools(&detection);

    // Initialize database and import tool configs in background
    println!();
    println!("   {}", theme.secondary("Setting up database..."));
    if let Err(e) = initialize_database().await {
        println!("   {} Database setup failed: {}", theme.accent("[!]"), e);
    } else {
        println!("   {} Configuration database ready", theme.accent("[OK]"));
    }

    print!("\n   Press Enter to continue...");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;

    // Mark as initialized before entering main interface
    if let Err(e) = mark_initialized() {
        eprintln!("Warning: Could not save initialization state: {e}");
    }

    Ok(())
}

/// Display detected tools in a clear format
pub fn display_detected_tools(detection: &ToolDetectionResult) {
    let theme = theme_global_config::current_theme();

    println!();
    println!("{}", theme.primary("   Tool Detection"));
    println!();

    if !detection.installed.is_empty() {
        println!(
            "   {} Installed tools:",
            theme.accent(&format!("[{}]", detection.installed.len()))
        );
        for tool in &detection.installed {
            println!(
                "      {} {}",
                theme.accent("[READY]"),
                theme.secondary(tool)
            );
        }
        println!();
    }

    if !detection.available.is_empty() {
        println!(
            "   {} Available to install:",
            theme.secondary(&format!("[{}]", detection.available.len()))
        );
        let display_count = detection.available.len().min(5);
        for tool in detection.available.iter().take(display_count) {
            println!("      {} {}", theme.secondary("[ - ]"), tool);
        }
        if detection.available.len() > 5 {
            println!(
                "      {} ...and {} more",
                theme.secondary("[ - ]"),
                detection.available.len() - 5
            );
        }
        println!();
        println!(
            "   {}",
            theme.secondary("Use /tools menu to install additional tools")
        );
    }

    if detection.installed.is_empty() {
        println!(
            "   {}",
            theme.accent("No AI tools detected. Use /tools to install your first one!")
        );
    }
}

/// Generate a compact tool status string for welcome screen
pub fn get_tool_status_line() -> String {
    let detection = detect_tools();

    if detection.installed.is_empty() {
        return String::from("No tools installed - type /tools to add some");
    }

    let tools_str = if detection.installed.len() <= 3 {
        detection.installed.join(", ")
    } else {
        format!(
            "{}, {} +{} more",
            detection.installed[0],
            detection.installed[1],
            detection.installed.len() - 2
        )
    };

    format!("Tools: {tools_str}")
}

/// Initialize the database during first run
async fn initialize_database() -> Result<()> {
    // Initialize database (creates schema)
    let db = DatabaseManager::init().await?;

    // Import TOML tool configurations
    let importer = TomlImporter::new(db.clone()).await?;
    let stats = importer.import_all().await?;

    // Store first-run timestamp in preferences
    let prefs_repo = PreferencesRepository::new(db);
    prefs_repo
        .set("first_run_completed", &chrono::Utc::now().to_rfc3339())
        .await?;
    prefs_repo
        .set("tools_imported", &stats.imported.to_string())
        .await?;

    Ok(())
}

// --- Last-Used Tool Tracking ---

/// Get the path to the preferences file
fn get_preferences_path() -> Option<PathBuf> {
    get_config_dir().map(|d| d.join("preferences.json"))
}

fn read_file_preferences() -> serde_json::Map<String, serde_json::Value> {
    let Some(prefs_path) = get_preferences_path() else {
        return serde_json::Map::new();
    };

    let Ok(content) = fs::read_to_string(&prefs_path) else {
        return serde_json::Map::new();
    };

    serde_json::from_str::<serde_json::Value>(&content)
        .ok()
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default()
}

fn write_file_preferences(prefs: &serde_json::Map<String, serde_json::Value>) -> Result<()> {
    if let Some(prefs_path) = get_preferences_path() {
        if let Some(parent) = prefs_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            &prefs_path,
            serde_json::to_string_pretty(&serde_json::Value::Object(prefs.clone()))?,
        )?;
    }
    Ok(())
}

/// Save the last-used tool (hybrid: database + file fallback)
pub fn save_last_used_tool(tool: &str) -> Result<()> {
    // Try database first (async context required)
    // Fall back to file-based for sync context
    let mut prefs = read_file_preferences();
    prefs.insert(
        "last_used_tool".to_string(),
        serde_json::Value::String(tool.to_string()),
    );
    prefs.insert(
        "updated_at".to_string(),
        serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
    );
    write_file_preferences(&prefs)
}

/// Save the last-used tool to database (async version)
pub async fn save_last_used_tool_async(tool: &str) -> Result<()> {
    let db = DatabaseManager::init().await?;
    let prefs_repo = PreferencesRepository::new(db);
    prefs_repo.set_last_used_tool(tool).await?;

    // Also save to file for backwards compatibility
    let _ = save_last_used_tool(tool);
    Ok(())
}

/// Get the last-used tool (hybrid: database + file fallback)
pub fn get_last_used_tool() -> Option<String> {
    // Try file-based first (sync context)
    read_file_preferences()
        .get("last_used_tool")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Save the custom configuration file path in file-based preferences.
pub fn save_custom_config_path(path: &Path) -> Result<()> {
    let mut prefs = read_file_preferences();
    prefs.insert(
        "config_path".to_string(),
        serde_json::Value::String(path.to_string_lossy().to_string()),
    );
    prefs.insert(
        "updated_at".to_string(),
        serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
    );
    write_file_preferences(&prefs)
}

/// Get the custom configuration file path from file-based preferences.
pub fn get_custom_config_path() -> Option<PathBuf> {
    read_file_preferences()
        .get("config_path")
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(PathBuf::from)
}

/// Clear the custom configuration file path from file-based preferences.
pub fn clear_custom_config_path() -> Result<()> {
    let mut prefs = read_file_preferences();
    prefs.remove("config_path");
    prefs.insert(
        "updated_at".to_string(),
        serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
    );
    write_file_preferences(&prefs)
}

/// Get the last-used tool from database (async version)
pub async fn get_last_used_tool_async() -> Option<String> {
    let db = DatabaseManager::init().await.ok()?;
    let prefs_repo = PreferencesRepository::new(db);
    prefs_repo.get_last_used_tool().await.ok().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_detect_tools_returns_valid_result() {
        let result = detect_tools();
        // Result should have both vectors (may be empty)
        // Just verify detect_tools() runs without panic
        let _ = result.installed.len() + result.available.len();
    }

    #[test]
    fn test_get_tool_status_line_returns_string() {
        let status = get_tool_status_line();
        assert!(!status.is_empty());
    }

    #[test]
    fn test_config_dir_path() {
        // Should return Some path when home dir exists
        if let Some(config_dir) = get_config_dir() {
            assert!(config_dir.to_string_lossy().contains(".terminal-jarvis"));
        }
    }

    #[test]
    fn test_custom_config_path_preferences_persist_and_reset() {
        let _guard = ENV_LOCK.lock().unwrap();
        let temp_home = tempfile::tempdir().unwrap();
        let old_home = std::env::var_os("HOME");
        std::env::set_var("HOME", temp_home.path());

        let config_path = temp_home.path().join("custom-config.toml");
        let mut custom_config = Config::default();
        custom_config.api.base_url = "https://custom-config.example".to_string();
        fs::write(
            &config_path,
            toml::to_string_pretty(&custom_config).unwrap(),
        )
        .unwrap();

        save_last_used_tool("codex").unwrap();
        save_custom_config_path(&config_path).unwrap();

        assert_eq!(get_custom_config_path(), Some(config_path));
        assert_eq!(get_last_used_tool(), Some("codex".to_string()));
        assert_eq!(
            Config::load().unwrap().api.base_url,
            "https://custom-config.example"
        );

        clear_custom_config_path().unwrap();
        assert_eq!(get_custom_config_path(), None);
        assert_eq!(get_last_used_tool(), Some("codex".to_string()));

        if let Some(home) = old_home {
            std::env::set_var("HOME", home);
        } else {
            std::env::remove_var("HOME");
        }
    }
}
