// Authentication Environment Detection - Browser prevention logic
//
// This module handles detection of environments where browser opening should be prevented,
// including CI environments, headless systems, and cloud development environments.

use std::env;

/// Environment detection utilities for preventing unwanted browser opening
pub struct EnvironmentDetector;

impl EnvironmentDetector {
    /// Check if we're running in an environment where browser opening should be prevented
    pub fn should_prevent_browser_opening() -> bool {
        // Prevent browser opening in CI environments
        if env::var("CI").is_ok() {
            return true;
        }

        // Prevent browser opening if no DISPLAY is set (headless environments)
        if env::var("DISPLAY").is_err() && env::var("WAYLAND_DISPLAY").is_err() {
            return true;
        }

        // Prevent browser opening in cloud development environments
        if env::var("CODESPACES").is_ok()
            || env::var("GITPOD_WORKSPACE_ID").is_ok()
            || env::var("CLOUD_SHELL").is_ok()
        {
            return true;
        }

        // Check for terminal-specific environments that can't handle browser opening
        if let Ok(term) = env::var("TERM") {
            if term == "dumb" || term.contains("screen") {
                return true;
            }
        }

        // Check if we're running in SSH session
        if env::var("SSH_CONNECTION").is_ok() || env::var("SSH_CLIENT").is_ok() {
            return true;
        }

        // Check if running in a container
        if std::path::Path::new("/.dockerenv").exists() {
            return true;
        }

        false
    }

    /// Check if we're in a CI environment specifically
    #[allow(dead_code)]
    pub fn is_ci_environment() -> bool {
        env::var("CI").is_ok()
            || env::var("GITHUB_ACTIONS").is_ok()
            || env::var("GITLAB_CI").is_ok()
            || env::var("TRAVIS").is_ok()
            || env::var("CIRCLECI").is_ok()
            || env::var("JENKINS_URL").is_ok()
    }

    /// Check if we're in a cloud development environment
    #[allow(dead_code)]
    pub fn is_cloud_environment() -> bool {
        env::var("CODESPACES").is_ok()
            || env::var("GITPOD_WORKSPACE_ID").is_ok()
            || env::var("CLOUD_SHELL").is_ok()
    }

    /// Check if we're in a container environment
    #[allow(dead_code)]
    pub fn is_container_environment() -> bool {
        std::path::Path::new("/.dockerenv").exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_detection_methods() {
        // Test CI detection
        let original_ci = env::var("CI").ok();
        env::set_var("CI", "true");
        assert!(EnvironmentDetector::is_ci_environment());
        assert!(EnvironmentDetector::should_prevent_browser_opening());

        // Restore original
        match original_ci {
            Some(val) => env::set_var("CI", val),
            None => env::remove_var("CI"),
        }

        // Test cloud environment detection
        let original_codespaces = env::var("CODESPACES").ok();
        env::set_var("CODESPACES", "true");
        assert!(EnvironmentDetector::is_cloud_environment());
        assert!(EnvironmentDetector::should_prevent_browser_opening());

        // Restore original
        match original_codespaces {
            Some(val) => env::set_var("CODESPACES", val),
            None => env::remove_var("CODESPACES"),
        }
    }
}
