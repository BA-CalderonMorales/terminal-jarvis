// Version Cache Implementation - TTL-based caching for version information
//
// This module provides version caching functionality with time-to-live (TTL)
// support for efficient version information management.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCache {
    pub version_info: String,
    pub cached_at: u64,   // Unix timestamp
    pub ttl_seconds: u64, // Time to live in seconds
}

impl VersionCache {
    /// Create a new version cache entry with TTL
    pub fn new(version_info: String, ttl_seconds: u64) -> Self {
        let cached_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        Self {
            version_info,
            cached_at,
            ttl_seconds,
        }
    }

    /// Check if the cache entry has expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        now - self.cached_at > self.ttl_seconds
    }

    /// Get the remaining seconds before cache expiration
    pub fn remaining_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        (self.cached_at + self.ttl_seconds).saturating_sub(now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_cache_creation() {
        let cache = VersionCache::new("1.0.0 (@stable)".to_string(), 3600);
        assert_eq!(cache.version_info, "1.0.0 (@stable)");
        assert_eq!(cache.ttl_seconds, 3600);
        assert!(!cache.is_expired()); // Should not be expired immediately
    }

    #[test]
    fn test_version_cache_expiration() {
        let mut cache = VersionCache::new("1.0.0".to_string(), 1);
        cache.cached_at = 0; // Force expiration by setting very old timestamp
        assert!(cache.is_expired());
    }

    #[test]
    fn test_version_cache_remaining_seconds() {
        let cache = VersionCache::new("1.0.0".to_string(), 3600);
        let remaining = cache.remaining_seconds();
        // Should be close to 3600, allowing for a few seconds of execution time
        assert!(remaining > 3590 && remaining <= 3600);
    }
}
