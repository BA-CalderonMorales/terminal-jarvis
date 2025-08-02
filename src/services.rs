use crate::config::Config;
use anyhow::{anyhow, Result};
use std::process::Command;
use tokio::process::Command as AsyncCommand;

/// Service for managing AI coding tool packages
pub struct PackageService {
    config: Config,
}

impl PackageService {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        Ok(Self { config })
    }

    /// Check if a tool is installed on the system
    pub async fn is_tool_installed(&self, tool: &str) -> Result<bool> {
        let output = AsyncCommand::new("which").arg(tool).output().await?;

        Ok(output.status.success())
    }

    /// Install a tool using the appropriate package manager
    pub async fn install_tool(&self, tool: &str) -> Result<()> {
        let tool_config = self
            .config
            .get_tool_config(tool)
            .ok_or_else(|| anyhow!("Tool {} not found in configuration", tool))?;

        if !tool_config.enabled {
            return Err(anyhow!("Tool {} is disabled in configuration", tool));
        }

        if let Some(install_cmd) = &tool_config.install_command {
            self.execute_command(install_cmd).await
        } else {
            // Fallback to default installation methods
            match tool {
                "claude-code" => self.install_npm_package("@anthropic-ai/claude-cli").await,
                "gemini-cli" => self.install_npm_package("@google/generative-ai-cli").await,
                "qwen-code" => self.install_cargo_package("qwen-code").await,
                "opencode" => self.install_npm_package("opencode").await,
                _ => Err(anyhow!("Unknown tool: {}", tool)),
            }
        }
    }

    /// Update a tool to the latest version
    pub async fn update_tool(&self, tool: &str) -> Result<()> {
        let tool_config = self
            .config
            .get_tool_config(tool)
            .ok_or_else(|| anyhow!("Tool {} not found in configuration", tool))?;

        if !tool_config.enabled {
            return Err(anyhow!("Tool {} is disabled in configuration", tool));
        }

        if let Some(update_cmd) = &tool_config.update_command {
            self.execute_command(update_cmd).await
        } else {
            // Fallback to default update methods
            match tool {
                "claude-code" | "gemini-cli" | "opencode" => self.update_npm_package(tool).await,
                "qwen-code" => self.update_cargo_package(tool).await,
                _ => Err(anyhow!("Unknown tool: {}", tool)),
            }
        }
    }

    /// Run a tool with the given arguments
    pub async fn run_tool(&self, tool: &str, args: &[String]) -> Result<()> {
        let mut cmd = AsyncCommand::new(tool);
        cmd.args(args);

        let status = cmd.status().await?;

        if !status.success() {
            return Err(anyhow!("Tool {} exited with status: {}", tool, status));
        }

        Ok(())
    }

    /// Get the version of an installed tool
    pub async fn get_tool_version(&self, tool: &str) -> Result<String> {
        let output = AsyncCommand::new(tool).arg("--version").output().await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to get version for {}", tool));
        }

        let version = String::from_utf8(output.stdout)?;
        Ok(version.trim().to_string())
    }

    /// Execute a shell command
    async fn execute_command(&self, command: &str) -> Result<()> {
        let mut parts = command.split_whitespace();
        let cmd = parts.next().ok_or_else(|| anyhow!("Empty command"))?;
        let args: Vec<&str> = parts.collect();

        let status = AsyncCommand::new(cmd).args(&args).status().await?;

        if !status.success() {
            return Err(anyhow!("Command failed: {}", command));
        }

        Ok(())
    }

    async fn install_npm_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("npm")
            .args(&["install", "-g", package])
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to install npm package: {}", package));
        }

        Ok(())
    }

    async fn install_cargo_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("cargo")
            .args(&["install", package])
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to install cargo package: {}", package));
        }

        Ok(())
    }

    async fn update_npm_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("npm")
            .args(&["update", "-g", package])
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to update npm package: {}", package));
        }

        Ok(())
    }

    async fn update_cargo_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("cargo")
            .args(&["install", "--force", package])
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to update cargo package: {}", package));
        }

        Ok(())
    }
}

/// Service for GitHub operations and template management
pub struct GitHubService {
    config: Config,
}

impl GitHubService {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        Ok(Self { config })
    }

    /// Initialize a template repository using GitHub CLI
    pub async fn init_template_repository(&self) -> Result<()> {
        // Check if gh CLI is installed
        let output = AsyncCommand::new("gh").arg("--version").output().await?;

        if !output.status.success() {
            return Err(anyhow!(
                "GitHub CLI (gh) is not installed. Please install it first."
            ));
        }

        // Create repository
        let status = AsyncCommand::new("gh")
            .args(&[
                "repo",
                "create",
                "jarvis-templates",
                "--private",
                "--confirm",
            ])
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to create template repository"));
        }

        println!("Template repository created successfully!");
        Ok(())
    }

    /// Create a new template
    pub async fn create_template(&self, name: &str) -> Result<()> {
        // This would implement template creation logic
        // For now, just a placeholder
        println!("Template '{}' would be created here", name);
        Ok(())
    }

    /// List available templates
    pub async fn list_templates(&self) -> Result<Vec<String>> {
        // This would implement template listing logic
        // For now, return empty list
        Ok(vec![])
    }

    /// Apply a template
    pub async fn apply_template(&self, name: &str) -> Result<()> {
        // This would implement template application logic
        // For now, just a placeholder
        println!("Template '{}' would be applied here", name);
        Ok(())
    }
}
