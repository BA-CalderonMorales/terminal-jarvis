// Services Entry Point - Main service definitions and coordination
//
// This module exposes the service facade used by CLI domains.

use crate::config::{Config, ConfigManager};
use anyhow::Result;

use super::services_github_integration::GitHubIntegrationManager;
use super::services_npm_operations::NpmOperationsManager;

/// Service for Terminal Jarvis package metadata.
pub struct PackageService;

impl PackageService {
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
