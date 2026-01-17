// Package Operations Management - Core package installation and update logic
//
// This module handles the main package management operations including
// tool installation checking, installation execution, and update operations
// with fallback package name support.

use crate::config::Config;
use crate::progress_utils::ProgressUtils;
use anyhow::{anyhow, Result};
use tokio::process::Command as AsyncCommand;

/// Manages package installation and update operations
pub struct PackageOperationsManager;

impl PackageOperationsManager {
    #[allow(dead_code)] // Framework code for future use
    pub fn new() -> Self {
        Self
    }

    /// Check if a tool is installed by attempting to run it with --version
    #[allow(dead_code)]
    pub async fn is_tool_installed(&self, tool_name: &str) -> Result<bool> {
        let output = AsyncCommand::new(tool_name).arg("--version").output().await;

        match output {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }

    /// Install a tool using the configuration from the config file
    #[allow(dead_code)]
    pub async fn install_tool(&self, config: &Config, tool_name: &str) -> Result<()> {
        let tool_config = config
            .tools
            .get(tool_name)
            .ok_or_else(|| anyhow!("Tool '{tool_name}' not found in configuration"))?;

        if !tool_config.enabled {
            return Err(anyhow!("Tool '{tool_name}' is disabled"));
        }

        let install_command = tool_config
            .install_command
            .as_ref()
            .ok_or_else(|| anyhow!("No install command configured for tool '{tool_name}'"))?;

        println!("Installing {tool_name}...");

        let spinner = ProgressUtils::spinner("Installing");

        let result = self.execute_command(install_command).await;

        ProgressUtils::finish_with_success(
            &spinner,
            &format!("{tool_name} installed successfully"),
        );

        match result {
            Ok(_) => {
                println!("✓ {tool_name} installed successfully");
                Ok(())
            }
            Err(e) => {
                println!("✗ Failed to install {tool_name}: {e}");
                Err(e)
            }
        }
    }

    /// Update a tool with fallback package name support
    #[allow(dead_code)] // Framework code for future use
    pub async fn update_tool(&self, config: &Config, tool_name: &str) -> Result<()> {
        let tool_config = config
            .tools
            .get(tool_name)
            .ok_or_else(|| anyhow!("Tool '{tool_name}' not found in configuration"))?;

        if !tool_config.enabled {
            return Err(anyhow!("Tool '{tool_name}' is disabled"));
        }

        let update_command = tool_config
            .update_command
            .as_ref()
            .ok_or_else(|| anyhow!("No update command configured for tool '{tool_name}'"))?;

        // Avoid printing per-tool updating lines here; the caller is responsible for
        // concise, non-overlapping progress output when running concurrently.

        let spinner = ProgressUtils::spinner("Updating");

        let result = self
            .execute_command_with_fallback(update_command, tool_name)
            .await;

        if result.is_ok() {
            ProgressUtils::finish_with_success(
                &spinner,
                &format!("{tool_name} updated successfully"),
            );
        } else {
            spinner.finish_with_message("Update failed");
        }

        match result {
            Ok(_) => {
                println!("✓ {tool_name} updated successfully");
                Ok(())
            }
            Err(e) => {
                println!("✗ Failed to update {tool_name}: {e}");
                Err(e)
            }
        }
    }

    /// Execute a command with fallback package name support
    ///
    /// If the primary package name fails, tries with fallback names
    /// commonly used for tools that have multiple package names.
    #[allow(dead_code)] // Framework code for future use
    async fn execute_command_with_fallback(&self, command: &str, tool_name: &str) -> Result<()> {
        // Try the original command first
        if self.execute_command(command).await.is_ok() {
            return Ok(());
        }

        // If that fails, try with fallback package names
        let fallback_packages = self.get_fallback_package_names(tool_name);

        for fallback_package in fallback_packages {
            // Replace the original package name with the fallback in the command
            let fallback_command = command.replace(tool_name, &fallback_package);

            if self.execute_command(&fallback_command).await.is_ok() {
                return Ok(());
            }
        }

        // If all attempts fail, return the original error
        self.execute_command(command).await
    }

    /// Get fallback package names for tools that might have different package names
    #[allow(dead_code)] // Framework code for future use
    fn get_fallback_package_names(&self, tool_name: &str) -> Vec<String> {
        let mut fallbacks = Vec::new();

        // Common fallback patterns
        match tool_name {
            name if name.ends_with("-code") => {
                // Try without the -code suffix
                fallbacks.push(name.trim_end_matches("-code").to_string());
            }
            name if name.ends_with("-cli") => {
                // Try without the -cli suffix
                fallbacks.push(name.trim_end_matches("-cli").to_string());
            }
            _ => {
                // Try with common suffixes
                fallbacks.push(format!("{tool_name}-cli"));
                fallbacks.push(format!("{tool_name}-code"));
            }
        }

        fallbacks
    }

    /// Execute a shell command
    async fn execute_command(&self, command: &str) -> Result<()> {
        let mut parts = command.split_whitespace();
        let cmd = parts.next().ok_or_else(|| anyhow!("Empty command"))?;
        let args: Vec<&str> = parts.collect();

        let output = AsyncCommand::new(cmd).args(&args).output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            return Err(anyhow!(
                "Command '{}' failed. Error: {} {}",
                command,
                stderr.trim(),
                stdout.trim()
            ));
        }

        Ok(())
    }

    /// Install NPM package (for internal use)
    #[allow(dead_code)]
    async fn install_npm_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("npm")
            .args(["install", "-g", package])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to install npm package: {package}"));
        }

        Ok(())
    }

    /// Install Cargo package (for internal use)
    #[allow(dead_code)]
    async fn install_cargo_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("cargo")
            .args(["install", package])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to install cargo package: {package}"));
        }

        Ok(())
    }

    /// Update NPM package (for internal use)
    #[allow(dead_code)]
    async fn update_npm_package(&self, package: &str) -> Result<()> {
        // First try to check if the package exists
        let check_status = AsyncCommand::new("npm")
            .args(["view", package, "version"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await;

        match check_status {
            Ok(status) if !status.success() => {
                return Err(anyhow!(
                    "Package '{package}' not found in npm registry. This might be a configuration error."
                ));
            }
            Err(e) => {
                return Err(anyhow!(
                    "Failed to check npm package '{package}': {e}. Is npm installed and working?"
                ));
            }
            _ => {} // Package exists, continue with update
        }

        let output = AsyncCommand::new("npm")
            .args(["update", "-g", package])
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            return Err(anyhow!(
                "Failed to update npm package '{}'. Error: {} {}",
                package,
                stderr.trim(),
                stdout.trim()
            ));
        }

        Ok(())
    }

    /// Update Cargo package (for internal use)
    #[allow(dead_code)]
    async fn update_cargo_package(&self, package: &str) -> Result<()> {
        let output = AsyncCommand::new("cargo")
            .args(["install", "--force", package])
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            return Err(anyhow!(
                "Failed to update cargo package '{}'. Error: {} {}",
                package,
                stderr.trim(),
                stdout.trim()
            ));
        }

        Ok(())
    }

    /// Install Pip package (for internal use)
    #[allow(dead_code)]
    async fn install_pip_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("pip")
            .args(["install", package])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to install pip package: {package}"));
        }

        Ok(())
    }

    /// Update Pip package (for internal use)
    #[allow(dead_code)]
    async fn update_pip_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("pip")
            .args(["install", "--upgrade", package])
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to update pip package: {package}"));
        }

        Ok(())
    }
}
