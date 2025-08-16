use crate::cli_logic::cli_logic_utilities::get_themed_render_config;
use crate::config::{Config, ConfigManager};
use crate::progress_utils::ProgressUtils;
use crate::services::PackageService;
use anyhow::{anyhow, Result};
use inquire::Confirm;

/// Handle resetting configuration to defaults
pub async fn handle_config_reset() -> Result<()> {
    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("terminal-jarvis").join("config.toml"),
        None => {
            ProgressUtils::error_message("Could not determine config directory");
            return Err(anyhow!("Could not determine config directory"));
        }
    };

    if config_path.exists() {
        let confirm = match Confirm::new("Are you sure you want to reset configuration to defaults? This will delete your current config file.")
      .with_render_config(get_themed_render_config())
      .with_default(false)
      .prompt() {
        Ok(result) => result,
        Err(_) => {
          // User interrupted - cancel the operation
          ProgressUtils::info_message("Configuration reset cancelled");
          return Ok(());
        }
      };

        if confirm {
            std::fs::remove_file(&config_path)?;
            ProgressUtils::success_message("Configuration reset to defaults");
            ProgressUtils::info_message(
                "The config file has been deleted. Default settings will be used.",
            );
        } else {
            ProgressUtils::info_message("Configuration reset cancelled");
        }
    } else {
        ProgressUtils::info_message("No configuration file found. Using defaults already.");
    }

    Ok(())
}

/// Handle displaying current configuration
pub async fn handle_config_show() -> Result<()> {
    let config = Config::load()?;
    let config_str = toml::to_string_pretty(&config)?;

    println!("Current configuration:");
    println!("{config_str}");

    Ok(())
}

/// Handle displaying configuration file path
pub async fn handle_config_path() -> Result<()> {
    let config_path = match dirs::config_dir() {
        Some(dir) => dir.join("terminal-jarvis").join("config.toml"),
        None => {
            ProgressUtils::error_message("Could not determine config directory");
            return Err(anyhow!("Could not determine config directory"));
        }
    };

    println!("Configuration file path: {}", config_path.display());

    if config_path.exists() {
        ProgressUtils::success_message("Configuration file exists");
    } else {
        ProgressUtils::info_message("Configuration file does not exist (using defaults)");
    }

    Ok(())
}

/// Handle clearing version cache
pub async fn handle_cache_clear() -> Result<()> {
    let config_manager = ConfigManager::new()?;

    config_manager.clear_version_cache()?;
    ProgressUtils::success_message(" Version cache cleared");

    Ok(())
}

/// Handle displaying cache status
pub async fn handle_cache_status() -> Result<()> {
    let config_manager = ConfigManager::new()?;

    match config_manager.load_version_cache()? {
        Some(cache) => {
            display_cache_info(&cache);
        }
        None => {
            println!(" No version cache found");
        }
    }

    Ok(())
}

/// Handle refreshing version cache with custom TTL
pub async fn handle_cache_refresh(ttl: u64) -> Result<()> {
    let config_manager = ConfigManager::new()?;

    println!(" Refreshing version cache...");
    let latest_version_info =
        PackageService::get_cached_npm_dist_tag_info_with_ttl(&config_manager, ttl).await?;

    match latest_version_info {
        Some(version_info) => {
            ProgressUtils::success_message(&format!(
                " Cache refreshed with version info: {version_info} (TTL: {ttl}s)"
            ));
        }
        None => {
            ProgressUtils::warning_message(
                "Version caching unavailable - registry data incomplete",
            );
        }
    }

    Ok(())
}

/// Display cache information in a formatted way
fn display_cache_info(cache: &crate::config::VersionCache) {
    println!(" Version Cache Status:");
    println!(" Version Info: {}", cache.version_info);
    println!(" Cached at: {} (Unix timestamp)", cache.cached_at);
    println!(" TTL: {} seconds", cache.ttl_seconds);

    if cache.is_expired() {
        println!(" Status: Expired");
    } else {
        let remaining = cache.remaining_seconds();
        println!(" Status: Valid ({remaining} seconds remaining)");
    }
}

/// Display configuration management help
pub async fn display_config_help() -> Result<()> {
    let theme = crate::theme_config::current_theme();

    println!("{}", theme.primary("┌─ Configuration Management ─────────────────────────────────┐"));
    println!("{}", theme.primary("│                                                             │"));
    println!("│ {:<59} │", theme.accent("Config Commands:"));
    println!("│   {:<57} │", theme.secondary("show   - Display current configuration"));
    println!("│   {:<57} │", theme.secondary("path   - Show config file location"));
    println!("│   {:<57} │", theme.secondary("reset  - Reset to default configuration"));
    println!("{}", theme.primary("│                                                             │"));
    println!("│ {:<59} │", theme.accent("Cache Commands:"));
    println!("│   {:<57} │", theme.secondary("status  - Show version cache status"));
    println!("│   {:<57} │", theme.secondary("clear   - Clear version cache"));
    println!("│   {:<57} │", theme.secondary("refresh - Refresh cache with new TTL"));
    println!("{}", theme.primary("│                                                             │"));
    println!("{}", theme.primary("└─────────────────────────────────────────────────────────────┘"));

    Ok(())
}
