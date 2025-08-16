// NPM Operations Management - NPM distribution tag management and version caching
//
// This module handles NPM-specific operations including distribution tag
// information fetching, version caching with TTL support, and development
// version detection for Terminal Jarvis itself.

use crate::config::{ConfigManager, VersionCache};
use anyhow::Result;
use tokio::process::Command as AsyncCommand;

/// Manages NPM distribution tag operations and version caching
pub struct NpmOperationsManager;

impl NpmOperationsManager {
    pub fn new() -> Self {
        Self
    }

    /// Get NPM distribution tag information for Terminal Jarvis
    ///
    /// This function detects if Terminal Jarvis is installed via NPM and
    /// returns the distribution tag(s) for the current version.
    /// For development versions, it compares against published tags.
    pub async fn get_npm_dist_tag_info() -> Result<Option<String>> {
        // First, check if we're installed via NPM
        let npm_list_output = AsyncCommand::new("npm")
            .args(["list", "-g", "terminal-jarvis", "--depth=0"])
            .output()
            .await;

        if let Ok(output) = npm_list_output {
            if output.status.success() {
                // We're installed via NPM, get the actual installed version and tag
                let installed_output = String::from_utf8(output.stdout).unwrap_or_default();

                // Extract version number from output
                if let Some(at_pos) = installed_output.rfind('@') {
                    let current_version = installed_output[at_pos + 1..].trim();

                    // Get distribution tags for terminal-jarvis
                    let tags_output = AsyncCommand::new("npm")
                        .args(["dist-tag", "ls", "terminal-jarvis"])
                        .output()
                        .await;

                    if let Ok(tags_result) = tags_output {
                        if tags_result.status.success() {
                            let tags_str =
                                String::from_utf8(tags_result.stdout).unwrap_or_default();

                            return Ok(Self::extract_matching_tags(&tags_str, current_version));
                        }
                    }
                }
            }
        }

        // Not installed via NPM or couldn't determine tag, but for development purposes,
        // let's show if this is a development version by comparing with published tags
        let current_version = env!("CARGO_PKG_VERSION");

        let tags_output = AsyncCommand::new("npm")
            .args(["dist-tag", "ls", "terminal-jarvis"])
            .output()
            .await;

        if let Ok(output) = tags_output {
            if output.status.success() {
                let tags_str = String::from_utf8(output.stdout).unwrap_or_default();

                // Collect all tags that match our version
                let mut matching_tags = Vec::new();

                for line in tags_str.lines() {
                    if let Some((tag, version)) = line.split_once(':') {
                        let version = version.trim();
                        let tag = tag.trim();

                        if version == current_version {
                            matching_tags.push(tag);
                        }
                    }
                }

                // For development, show all matching tags with "-dev" suffix
                if !matching_tags.is_empty() {
                    // Sort tags for consistent display: stable, beta, latest, others
                    Self::sort_tags(&mut matching_tags);

                    let tags_string = matching_tags.join(", ");
                    return Ok(Some(format!("{tags_string}-dev")));
                }

                // Current version doesn't match any published version
                return Ok(Some("dev".to_string()));
            }
        }

        Ok(None)
    }

    /// Get NPM distribution tag information with caching
    /// Cache TTL: 1 hour by default
    pub async fn get_cached_npm_dist_tag_info(
        config_manager: &ConfigManager,
    ) -> Result<Option<String>> {
        // Default cache TTL: 1 hour (3600 seconds)
        const CACHE_TTL_SECONDS: u64 = 3600;

        // Try to load from cache first
        if let Ok(Some(cache)) = config_manager.load_version_cache() {
            if !cache.is_expired() {
                return Ok(Some(cache.version_info));
            }
        }

        // Cache miss or expired - fetch fresh version info
        let latest_version_info = Self::get_npm_dist_tag_info().await?;

        // If we got version info, save to cache
        if let Some(version_info) = &latest_version_info {
            let cache = VersionCache::new(version_info.clone(), CACHE_TTL_SECONDS);
            if let Err(e) = config_manager.save_version_cache(&cache) {
                // Log warning but don't fail - caching is best effort
                eprintln!("Warning: Failed to save version cache: {e}");
            }
        }

        Ok(latest_version_info)
    }

    /// Get cached version info with custom TTL
    pub async fn get_cached_npm_dist_tag_info_with_ttl(
        config_manager: &ConfigManager,
        ttl_seconds: u64,
    ) -> Result<Option<String>> {
        // Try to load from cache first
        if let Ok(Some(cache)) = config_manager.load_version_cache() {
            // Use cache if it's not expired and the TTL is at least what we want
            if !cache.is_expired() && cache.ttl_seconds >= ttl_seconds {
                return Ok(Some(cache.version_info));
            }
        }

        // Cache miss, expired, or different TTL - fetch fresh version info
        let latest_version_info = Self::get_npm_dist_tag_info().await?;

        // If we got version info, save to cache with specified TTL
        if let Some(version_info) = &latest_version_info {
            let cache = VersionCache::new(version_info.clone(), ttl_seconds);
            if let Err(e) = config_manager.save_version_cache(&cache) {
                eprintln!("Warning: Failed to save version cache: {e}");
            }
        }

        Ok(latest_version_info)
    }

    /// Extract matching tags from NPM dist-tag output for a given version
    fn extract_matching_tags(tags_str: &str, version: &str) -> Option<String> {
        // Collect all tags that match our version
        let mut matching_tags = Vec::new();

        for line in tags_str.lines() {
            if let Some((tag, tag_version)) = line.split_once(':') {
                let tag = tag.trim();
                let tag_version = tag_version.trim();

                if tag_version == version {
                    matching_tags.push(tag);
                }
            }
        }

        // Return all matching tags as a comma-separated string
        if !matching_tags.is_empty() {
            // Sort tags for consistent display: stable, beta, latest, others
            Self::sort_tags(&mut matching_tags);
            Some(matching_tags.join(", "))
        } else {
            None
        }
    }

    /// Sort tags in a consistent order: stable, beta, latest, others
    fn sort_tags(tags: &mut Vec<&str>) {
        tags.sort_by(|a, b| {
            let order_a = match *a {
                "stable" => 0,
                "beta" => 1,
                "latest" => 2,
                _ => 3,
            };
            let order_b = match *b {
                "stable" => 0,
                "beta" => 1,
                "latest" => 2,
                _ => 3,
            };
            order_a.cmp(&order_b)
        });
    }
}
