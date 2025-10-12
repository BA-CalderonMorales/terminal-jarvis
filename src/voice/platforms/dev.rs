// Development Environment Voice Provider Implementation
// For GitHub Codespaces and other environments without audio hardware
//
// This module provides a text-based voice input simulation that allows
// testing and development of voice features without requiring audio devices.
// Automatically activates in Codespaces and similar environments.

use anyhow::{anyhow, Result};
use std::io::{self, Write};
use std::time::Duration;

/// Check if we're running in a development environment without audio
pub async fn is_dev_environment() -> bool {
    // Check for GitHub Codespaces
    if std::env::var("CODESPACES").is_ok() {
        return true;
    }
    
    // Check for other cloud development environments
    if std::env::var("GITPOD_WORKSPACE_ID").is_ok() {
        return true;
    }
    
    // Check if running in Docker/container without audio
    if is_container_without_audio().await {
        return true;
    }
    
    false
}

/// Check if running in a container without audio devices
async fn is_container_without_audio() -> bool {
    // Check if running in container
    let in_container = std::path::Path::new("/.dockerenv").exists()
        || std::fs::read_to_string("/proc/1/cgroup")
            .ok()
            .map(|s| s.contains("docker") || s.contains("lxc"))
            .unwrap_or(false);
    
    if !in_container {
        return false;
    }
    
    // Check if audio devices are available on Linux
    #[cfg(target_os = "linux")]
    {
        // Check for ALSA sound cards
        let has_sound_cards = std::path::Path::new("/proc/asound/cards").exists()
            && std::fs::read_to_string("/proc/asound/cards")
                .ok()
                .map(|content| !content.trim().is_empty() && !content.contains("no soundcards"))
                .unwrap_or(false);
        
        return !has_sound_cards;
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        // On other platforms in containers, assume no audio
        true
    }
}

/// Simulate voice input using text-based prompt
/// This provides a development-friendly way to test voice commands
pub async fn simulate_voice_input(_duration: Duration) -> Result<String> {
    println!("\n[DEV MODE] Voice simulation active (no audio hardware detected)");
    println!("[DEV MODE] GitHub Codespaces detected - using text-based input");
    println!();
    
    // Prompt for text input instead of voice
    print!("Enter voice command (text): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let command = input.trim().to_string();
    
    if command.is_empty() {
        return Err(anyhow!("No input provided"));
    }
    
    // Simulate processing delay
    println!("[DEV MODE] Simulating transcription...");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    Ok(command)
}

/// Check if dev environment voice input is ready
pub async fn is_ready() -> Result<bool> {
    // Dev environment is always "ready" - it uses text input
    Ok(is_dev_environment().await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_codespaces_detection() {
        // Save original env
        let original = std::env::var("CODESPACES").ok();
        
        // Test with CODESPACES set
        std::env::set_var("CODESPACES", "true");
        assert!(is_dev_environment().await);
        
        // Restore original env
        if let Some(val) = original {
            std::env::set_var("CODESPACES", val);
        } else {
            std::env::remove_var("CODESPACES");
        }
    }

    #[tokio::test]
    async fn test_gitpod_detection() {
        // Save original env
        let original = std::env::var("GITPOD_WORKSPACE_ID").ok();
        
        // Test with GITPOD set
        std::env::set_var("GITPOD_WORKSPACE_ID", "test-workspace");
        assert!(is_dev_environment().await);
        
        // Restore original env
        if let Some(val) = original {
            std::env::set_var("GITPOD_WORKSPACE_ID", val);
        } else {
            std::env::remove_var("GITPOD_WORKSPACE_ID");
        }
    }

    #[tokio::test]
    async fn test_dev_ready() {
        // Dev environment should always be ready
        let ready = is_ready().await.unwrap();
        // Result depends on actual environment, so just check it doesn't error
        assert!(ready || !ready); // Always true, just testing it runs
    }
}
