// Configuration Manager - File and cache management operations
//
// This module provides the ConfigManager struct for handling configuration
// file operations and version cache persistence with cleanup functionality.

use crate::config::config_version_cache::VersionCache;
use anyhow::Result;
use std::path::PathBuf;

/// Configuration manager for handling config files and caching
pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
            .join("terminal-jarvis");

        // Create config directory if it doesn't exist
        std::fs::create_dir_all(&config_dir)?;

        Ok(Self { config_dir })
    }

    /// Get the path to the version cache file
    pub fn get_version_cache_path(&self) -> PathBuf {
        self.config_dir.join("version_cache.toml")
    }

    /// Load version cache from disk
    pub fn load_version_cache(&self) -> Result<Option<VersionCache>> {
        let cache_path = self.get_version_cache_path();

        if !cache_path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&cache_path)?;
        let cache: VersionCache = toml::from_str(&content)?;

        if cache.is_expired() {
            // Clean up expired cache
            let _ = std::fs::remove_file(&cache_path);
            return Ok(None);
        }

        Ok(Some(cache))
    }

    /// Save version cache to disk
    pub fn save_version_cache(&self, cache: &VersionCache) -> Result<()> {
        let cache_path = self.get_version_cache_path();
        let content = toml::to_string_pretty(cache)?;
        std::fs::write(&cache_path, content)?;
        Ok(())
    }

    /// Clear version cache
    pub fn clear_version_cache(&self) -> Result<()> {
        let cache_path = self.get_version_cache_path();
        if cache_path.exists() {
            std::fs::remove_file(&cache_path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_manager_version_cache_file_operations() -> Result<()> {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new()?;
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };

        let cache = VersionCache::new("1.2.3 (@stable, beta)".to_string(), 3600);

        // Test save
        config_manager.save_version_cache(&cache)?;

        // Test load
        let loaded_cache = config_manager.load_version_cache()?.unwrap();
        assert_eq!(loaded_cache.version_info, "1.2.3 (@stable, beta)");
        assert_eq!(loaded_cache.ttl_seconds, 3600);

        // Test clear
        config_manager.clear_version_cache()?;
        let after_clear = config_manager.load_version_cache()?;
        assert!(after_clear.is_none());

        Ok(())
    }

    #[test]
    fn test_expired_cache_cleanup() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let config_manager = ConfigManager {
            config_dir: temp_dir.path().to_path_buf(),
        };

        // Create an expired cache
        let mut expired_cache = VersionCache::new("1.0.0".to_string(), 1);
        expired_cache.cached_at = 0; // Make it ancient

        config_manager.save_version_cache(&expired_cache)?;

        // When we try to load, it should return None and clean up the file
        let loaded = config_manager.load_version_cache()?;
        assert!(loaded.is_none());

        // The cache file should no longer exist
        assert!(!config_manager.get_version_cache_path().exists());

        Ok(())
    }
}
