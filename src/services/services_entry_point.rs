// Services Entry Point - Main service class definitions and coordination
//
// This module provides the main PackageService and GitHubService classes,
// delegating domain-specific operations to specialized modules.

use crate::config::{Config, ConfigManager};
use anyhow::Result;

use super::services_github_integration::GitHubIntegrationManager;
use super::services_npm_operations::NpmOperationsManager;
use super::services_package_operations::PackageOperationsManager;
use super::services_tool_configuration::ToolConfigurationManager;

/// Service for managing AI coding tool packages
pub struct PackageService {
    config: Config,
    #[allow(dead_code)]
    tool_config_manager: ToolConfigurationManager,
    package_ops_manager: PackageOperationsManager,
    #[allow(dead_code)]
    npm_ops_manager: NpmOperationsManager,
}

impl PackageService {
    #[allow(dead_code)] // Framework code for future use
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        Ok(Self {
            config: config.clone(),
            tool_config_manager: ToolConfigurationManager::new(),
            package_ops_manager: PackageOperationsManager::new(),
            npm_ops_manager: NpmOperationsManager::new(),
        })
    }

    // Tool Configuration Operations
    #[allow(dead_code)]
    pub fn get_display_name_to_config_mapping(
    ) -> std::collections::HashMap<&'static str, &'static str> {
        ToolConfigurationManager::get_display_name_to_config_mapping()
    }

    #[allow(dead_code)]
    pub fn get_config_key_for_tool<'a>(&self, tool_display_name: &'a str) -> &'a str {
        self.tool_config_manager
            .get_config_key_for_tool(tool_display_name)
    }

    // Package Operations
    #[allow(dead_code)]
    pub async fn is_tool_installed(&self, tool_name: &str) -> Result<bool> {
        self.package_ops_manager.is_tool_installed(tool_name).await
    }

    #[allow(dead_code)]
    pub async fn install_tool(&self, tool_name: &str) -> Result<()> {
        self.package_ops_manager
            .install_tool(&self.config, tool_name)
            .await
    }

    #[allow(dead_code)] // Framework code for future use
    pub async fn update_tool(&self, tool_name: &str) -> Result<()> {
        self.package_ops_manager
            .update_tool(&self.config, tool_name)
            .await
    }

    // NPM Operations
    #[allow(dead_code)]
    pub async fn get_npm_dist_tag_info() -> Result<Option<String>> {
        NpmOperationsManager::get_npm_dist_tag_info().await
    }

    pub async fn get_cached_npm_dist_tag_info(
        config_manager: &ConfigManager,
    ) -> Result<Option<String>> {
        NpmOperationsManager::get_cached_npm_dist_tag_info(config_manager).await
    }

    pub async fn get_cached_npm_dist_tag_info_with_ttl(
        config_manager: &ConfigManager,
        ttl_seconds: u64,
    ) -> Result<Option<String>> {
        NpmOperationsManager::get_cached_npm_dist_tag_info_with_ttl(config_manager, ttl_seconds)
            .await
    }
}

/// Service for managing GitHub operations and templates
pub struct GitHubService {
    github_manager: GitHubIntegrationManager,
}

impl GitHubService {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        Ok(Self {
            github_manager: GitHubIntegrationManager::new(config),
        })
    }

    pub async fn init_template_repository(&self) -> Result<()> {
        self.github_manager.init_template_repository().await
    }

    pub async fn create_template(&self, name: &str) -> Result<()> {
        self.github_manager.create_template(name).await
    }

    pub async fn list_templates(&self) -> Result<Vec<String>> {
        self.github_manager.list_templates().await
    }

    pub async fn apply_template(&self, name: &str) -> Result<()> {
        self.github_manager.apply_template(name).await
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
        assert_eq!(mapping.get("crush"), Some(&"crush"));
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
        assert_eq!(service.get_config_key_for_tool("crush"), "crush");

        // Test that unknown tools return themselves
        assert_eq!(
            service.get_config_key_for_tool("unknown-tool"),
            "unknown-tool"
        );

        Ok(())
    }
}
