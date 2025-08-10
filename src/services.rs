use crate::config::Config;
use crate::progress_utils::ProgressUtils;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::process::Command;
use tokio::process::Command as AsyncCommand;

/// Service for managing AI coding tool packages
pub struct PackageService {
    config: Config,
}

impl PackageService {
    pub fn new() -> Result<Self> {
        let mut config = Config::load()?;

        // Ensure all default tools are present
        config.ensure_default_tools();

        Ok(Self { config })
    }

    /// Map display names to configuration keys
    fn get_display_name_to_config_mapping() -> HashMap<&'static str, &'static str> {
        let mut mapping = HashMap::new();
        mapping.insert("claude", "claude-code");
        mapping.insert("gemini", "gemini-cli");
        mapping.insert("qwen", "qwen-code");
        mapping.insert("opencode", "opencode");
        mapping.insert("llxprt", "llxprt-code");
        mapping.insert("codex", "codex");
        mapping
    }

    /// Get the configuration key for a display name
    fn get_config_key_for_tool<'a>(&self, display_name: &'a str) -> &'a str {
        Self::get_display_name_to_config_mapping()
            .get(display_name)
            .unwrap_or(&display_name)
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
                "claude-code" => self.install_npm_package("@anthropic-ai/claude-code").await,
                "gemini-cli" => self.install_npm_package("@google/gemini-cli").await,
                "qwen-code" => self.install_npm_package("@qwen-code/qwen-code").await,
                "opencode" => self.install_npm_package("opencode-ai").await,
                "llxprt-code" => self.install_npm_package("@vybestack/llxprt-code").await,
                _ => Err(anyhow!("Unknown tool: {}", tool)),
            }
        }
    }

    /// Update a tool to the latest version
    pub async fn update_tool(&self, tool: &str) -> Result<()> {
        // Map display name to configuration key
        let config_key = self.get_config_key_for_tool(tool);

        let tool_config = self
            .config
            .get_tool_config(config_key)
            .ok_or_else(|| {
                // Provide helpful error message
                anyhow!(
                    "Tool '{}' not found in configuration. This might be due to an outdated config file. Try deleting ~/.config/terminal-jarvis/config.toml to reset to defaults.",
                    tool
                )
            })?;

        if !tool_config.enabled {
            return Err(anyhow!("Tool '{}' is disabled in configuration", tool));
        }

        if let Some(update_cmd) = &tool_config.update_command {
            self.execute_command(update_cmd).await
        } else {
            // Fallback to default update methods for real AI coding tools
            match config_key {
                "aider" => self.update_pip_package("aider-chat").await,
                "cursor-cli" => {
                    // Cursor CLI updates are handled through the Cursor application
                    ProgressUtils::info_message(
                        "Cursor CLI updates are handled through the Cursor application.",
                    );
                    Ok(())
                }
                "codeium" | "copilot-cli" => self.update_npm_package(config_key).await,
                "claude-code" => {
                    // Try different package names for Claude
                    match self.update_npm_package("@anthropic-ai/claude-code").await {
                        Ok(result) => Ok(result),
                        Err(_) => {
                            // Fallback to try other possible names
                            match self.update_npm_package("claude-cli").await {
                                Ok(result) => Ok(result),
                                Err(_) => self.update_npm_package("claude").await,
                            }
                        }
                    }
                }
                "gemini-cli" => {
                    // Try different package names for Gemini
                    match self.update_npm_package("@google/gemini-cli").await {
                        Ok(result) => Ok(result),
                        Err(_) => match self.update_npm_package("gemini-cli").await {
                            Ok(result) => Ok(result),
                            Err(_) => self.update_npm_package("@google/generative-ai-cli").await,
                        },
                    }
                }
                "qwen-code" => {
                    // Try different package names for Qwen
                    match self.update_npm_package("@qwen-code/qwen-code").await {
                        Ok(result) => Ok(result),
                        Err(_) => match self.update_npm_package("qwen-code").await {
                            Ok(result) => Ok(result),
                            Err(_) => self.update_npm_package("qwen").await,
                        },
                    }
                }
                "opencode" => {
                    // Try different package names for OpenCode
                    match self.update_npm_package("opencode-ai").await {
                        Ok(result) => Ok(result),
                        Err(_) => match self.update_npm_package("opencode").await {
                            Ok(result) => Ok(result),
                            Err(_) => self.update_npm_package("@opencode/cli").await,
                        },
                    }
                }
                "llxprt-code" => {
                    // Try @vybestack/llxprt-code package
                    match self.update_npm_package("@vybestack/llxprt-code").await {
                        Ok(result) => Ok(result),
                        Err(_) => self.update_npm_package("llxprt-code").await,
                    }
                }
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

    /// Get NPM distribution tag information for terminal-jarvis
    pub async fn get_npm_dist_tag_info() -> Result<Option<String>> {
        // First check if npm is available
        let npm_available = AsyncCommand::new("npm").args(["--version"]).output().await;

        if npm_available.is_err() {
            // NPM not available, return None
            return Ok(None);
        }

        // Check if we're running from an NPM installation by looking for global install
        let npm_check = AsyncCommand::new("npm")
            .args(["list", "-g", "terminal-jarvis", "--depth=0"])
            .output()
            .await;

        if let Ok(output) = npm_check {
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

                            // Collect all tags that match our version
                            let mut matching_tags = Vec::new();

                            for line in tags_str.lines() {
                                if let Some((tag, version)) = line.split_once(':') {
                                    let tag = tag.trim();
                                    let version = version.trim();

                                    if version == current_version {
                                        matching_tags.push(tag);
                                    }
                                }
                            }

                            // Return all matching tags as a comma-separated string
                            if !matching_tags.is_empty() {
                                // Sort tags for consistent display: stable, beta, latest, others
                                matching_tags.sort_by(|a, b| {
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

                                return Ok(Some(matching_tags.join(", ")));
                            }
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
                    matching_tags.sort_by(|a, b| {
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

                    let tags_string = matching_tags.join(", ");
                    return Ok(Some(format!("{tags_string}-dev")));
                }

                // Current version doesn't match any published version
                return Ok(Some("dev".to_string()));
            }
        }

        Ok(None)
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
                    "Package '{}' not found in npm registry. This might be a configuration error.",
                    package
                ));
            }
            Err(e) => {
                return Err(anyhow!(
                    "Failed to check npm package '{}': {}. Is npm installed and working?",
                    package,
                    e
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_name_to_config_mapping() {
        let mapping = PackageService::get_display_name_to_config_mapping();

        // Test that all expected mappings exist
        assert_eq!(mapping.get("claude"), Some(&"claude-code"));
        assert_eq!(mapping.get("gemini"), Some(&"gemini-cli"));
        assert_eq!(mapping.get("qwen"), Some(&"qwen-code"));
        assert_eq!(mapping.get("opencode"), Some(&"opencode"));
        assert_eq!(mapping.get("llxprt"), Some(&"llxprt-code"));
        assert_eq!(mapping.get("codex"), Some(&"codex"));
    }

    #[tokio::test]
    async fn test_config_key_resolution() -> Result<()> {
        let service = PackageService::new()?;

        // Test that display names are correctly mapped to config keys
        assert_eq!(service.get_config_key_for_tool("qwen"), "qwen-code");
        assert_eq!(service.get_config_key_for_tool("claude"), "claude-code");
        assert_eq!(service.get_config_key_for_tool("gemini"), "gemini-cli");
        assert_eq!(service.get_config_key_for_tool("opencode"), "opencode");
        assert_eq!(service.get_config_key_for_tool("llxprt"), "llxprt-code");
        assert_eq!(service.get_config_key_for_tool("codex"), "codex");

        // Test that unknown tools return themselves
        assert_eq!(
            service.get_config_key_for_tool("unknown-tool"),
            "unknown-tool"
        );

        Ok(())
    }
}
