use crate::config::Config;
use crate::progress_utils::ProgressUtils;
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
    #[allow(dead_code)]
    pub async fn is_tool_installed(&self, tool: &str) -> Result<bool> {
        // First try 'which' command (Unix-like systems)
        let which_result = Command::new("which").arg(tool).output();

        if let Ok(output) = which_result {
            if output.status.success() {
                return Ok(true);
            }
        }

        // Also try running the tool with --version or --help to see if it exists
        let version_result = Command::new(tool).arg("--version").output();

        if let Ok(output) = version_result {
            if output.status.success() {
                return Ok(true);
            }
        }

        // Try --help as fallback
        let help_result = Command::new(tool).arg("--help").output();

        if let Ok(output) = help_result {
            if output.status.success() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Install a tool using the appropriate package manager
    #[allow(dead_code)]
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
            // Fallback to default installation methods for real AI coding tools
            match tool {
                "aider" => self.install_pip_package("aider-chat").await,
                "cursor-cli" => {
                    println!(
                        "Cursor CLI installation requires manual setup from https://cursor.sh/"
                    );
                    println!("Please download and install Cursor, then the CLI will be available.");
                    Ok(())
                }
                "codeium" => self.install_npm_package("@codeium/cli").await,
                "copilot-cli" => {
                    self.install_npm_package("@githubnext/github-copilot-cli")
                        .await
                }
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
            // Fallback to default update methods for real AI coding tools
            match tool {
                "aider" => self.update_pip_package("aider-chat").await,
                "cursor-cli" => {
                    // Cursor CLI updates are handled through the Cursor application
                    ProgressUtils::info_message(
                        "Cursor CLI updates are handled through the Cursor application.",
                    );
                    Ok(())
                }
                "codeium" | "copilot-cli" => self.update_npm_package(tool).await,
                _ => Err(anyhow!("Unknown tool: {}", tool)),
            }
        }
    }

    /// Run a tool with the given arguments
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

        let status = AsyncCommand::new(cmd)
            .args(&args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Command failed: {}", command));
        }

        Ok(())
    }

    #[allow(dead_code)]
    async fn install_npm_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("npm")
            .args(["install", "-g", package])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to install npm package: {}", package));
        }

        Ok(())
    }

    #[allow(dead_code)]
    async fn install_cargo_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("cargo")
            .args(["install", package])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to install cargo package: {}", package));
        }

        Ok(())
    }

    async fn update_npm_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("npm")
            .args(["update", "-g", package])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to update npm package: {}", package));
        }

        Ok(())
    }

    #[allow(dead_code)]
    async fn update_cargo_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("cargo")
            .args(["install", "--force", package])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to update cargo package: {}", package));
        }

        Ok(())
    }

    #[allow(dead_code)]
    async fn install_pip_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("pip")
            .args(["install", package])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to install pip package: {}", package));
        }

        Ok(())
    }

    async fn update_pip_package(&self, package: &str) -> Result<()> {
        let status = AsyncCommand::new("pip")
            .args(["install", "--upgrade", package])
            .status()
            .await?;

        if !status.success() {
            return Err(anyhow!("Failed to update pip package: {}", package));
        }

        Ok(())
    }
}

/// Service for managing GitHub operations and templates
pub struct GitHubService {
    #[allow(dead_code)]
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
            .args([
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
        println!("Template '{name}' would be created here");
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
        println!("Template '{name}' would be applied here");
        Ok(())
    }
}
