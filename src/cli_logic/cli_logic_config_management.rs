use crate::cli_logic::themed_components::themed_confirm;
use crate::config::{Config, ConfigManager};
use crate::progress_utils::ProgressUtils;
use crate::services::PackageService;
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

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
        let confirm = match themed_confirm("Are you sure you want to reset configuration to defaults? This will delete your current config file.")
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
    print_config_path_status();
    Ok(())
}

/// Handle the interactive /config slash command.
pub fn handle_config_command(args: &str) -> Result<()> {
    let args = args.trim();

    if args.is_empty() {
        print_config_path_status();
        return Ok(());
    }

    if args == "--reset" {
        crate::cli_logic::cli_logic_first_run::clear_custom_config_path()?;
        ProgressUtils::success_message("Configuration path override cleared");
        print_config_path_status();
        return Ok(());
    }

    let path = PathBuf::from(args);
    let canonical = validate_custom_config_path(&path)?;
    crate::cli_logic::cli_logic_first_run::save_custom_config_path(&canonical)?;
    ProgressUtils::success_message(&format!(
        "Configuration path set to {}",
        canonical.display()
    ));
    ProgressUtils::info_message("Future config loads will use this path");

    Ok(())
}

/// Handle the settings submenu for configuration path management.
pub async fn handle_config_path_menu() -> Result<()> {
    use crate::cli_logic::themed_components::{themed_select_with, themed_text};
    use crate::theme::theme_global_config;

    loop {
        let theme = theme_global_config::current_theme();
        print!("\x1b[2J\x1b[H");
        println!("{}\n", theme.accent("Configuration Path"));
        print_config_path_status();
        println!();

        let options = vec![
            "Set Custom Path".to_string(),
            "Reset to Default".to_string(),
            "Back to Settings".to_string(),
        ];

        let selection = match themed_select_with(&theme, "Choose an option:", options).prompt() {
            Ok(selection) => selection,
            Err(_) => return Ok(()),
        };

        match selection.as_str() {
            "Set Custom Path" => {
                let input = match themed_text("Config file path:").prompt() {
                    Ok(input) => input,
                    Err(_) => continue,
                };
                if let Err(e) = handle_config_command(&input) {
                    ProgressUtils::error_message(&e.to_string());
                }
                println!("\n{}", theme.accent("Press Enter to continue..."));
                let _ = std::io::stdin().read_line(&mut String::new());
            }
            "Reset to Default" => {
                handle_config_command("--reset")?;
                println!("\n{}", theme.accent("Press Enter to continue..."));
                let _ = std::io::stdin().read_line(&mut String::new());
            }
            _ => return Ok(()),
        }
    }
}

fn print_config_path_status() {
    let default_path =
        crate::config::default_config_path().unwrap_or_else(|| PathBuf::from("config.toml"));
    let (active_path, is_custom) = crate::config::active_config_path();
    let mode = if is_custom { "custom" } else { "default" };
    let exists = if active_path.exists() { "yes" } else { "no" };

    println!("Configuration path: {}", active_path.display());
    println!("Path type: {mode}");
    println!("File exists: {exists}");
    println!("Default path: {}", default_path.display());
}

fn validate_custom_config_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(anyhow!(
            "Configuration file does not exist: {}",
            path.display()
        ));
    }

    if !path.is_file() {
        return Err(anyhow!(
            "Configuration path must be a file: {}",
            path.display()
        ));
    }

    let canonical = path.canonicalize()?;
    let content = std::fs::read_to_string(&canonical)?;
    toml::from_str::<Config>(&content).map_err(|e| {
        anyhow!(
            "Configuration file is not valid Terminal Jarvis TOML: {}",
            e
        )
    })?;

    Ok(canonical)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    struct EnvVarGuard {
        originals: Vec<(&'static str, Option<OsString>)>,
    }

    impl EnvVarGuard {
        fn capture(keys: &[&'static str]) -> Self {
            Self {
                originals: keys
                    .iter()
                    .map(|key| (*key, std::env::var_os(key)))
                    .collect(),
            }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            for (key, value) in &self.originals {
                match value {
                    Some(val) => std::env::set_var(key, val),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    fn write_valid_config(path: &Path) {
        let config = Config::default();
        std::fs::write(path, toml::to_string_pretty(&config).unwrap()).unwrap();
    }

    #[test]
    fn test_validate_custom_config_path_rejects_missing_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let missing_path = temp_dir.path().join("missing.toml");

        let result = validate_custom_config_path(&missing_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_custom_config_path_rejects_invalid_toml() {
        let temp_dir = tempfile::tempdir().unwrap();
        let invalid_path = temp_dir.path().join("invalid.toml");
        std::fs::write(&invalid_path, "not = [valid").unwrap();

        let result = validate_custom_config_path(&invalid_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_config_command_persists_valid_path_and_rejects_invalid_path() {
        let _guard = crate::cli_logic::cli_logic_first_run::TEST_ENV_LOCK
            .lock()
            .unwrap();
        let _env = EnvVarGuard::capture(&["HOME"]);
        let temp_home = tempfile::tempdir().unwrap();
        std::env::set_var("HOME", temp_home.path());

        crate::cli_logic::cli_logic_first_run::clear_custom_config_path().unwrap();

        let valid_path = temp_home.path().join("valid.toml");
        write_valid_config(&valid_path);
        handle_config_command(valid_path.to_str().unwrap()).unwrap();

        let stored = crate::cli_logic::cli_logic_first_run::get_custom_config_path().unwrap();
        assert_eq!(stored, valid_path.canonicalize().unwrap());

        let missing_path = temp_home.path().join("missing.toml");
        assert!(handle_config_command(missing_path.to_str().unwrap()).is_err());
        assert_eq!(
            crate::cli_logic::cli_logic_first_run::get_custom_config_path(),
            Some(stored)
        );

        handle_config_command("--reset").unwrap();
        assert_eq!(
            crate::cli_logic::cli_logic_first_run::get_custom_config_path(),
            None
        );
    }
}
