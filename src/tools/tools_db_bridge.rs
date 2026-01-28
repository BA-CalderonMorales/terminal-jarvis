// Tools Database Bridge
//
// Provides database-backed tool operations, bridging between
// the existing ToolManager interface and the new libSQL database.
//
// This module enables gradual migration from TOML-based static
// configuration to database-backed dynamic configuration.
//
// Migration Strategy:
// 1. Check if database has tools (was import run?)
// 2. If yes, use database as primary source
// 3. If no, fall back to TOML config loader

use crate::db::{DatabaseManager, Tool, ToolsRepository};
use anyhow::Result;
use std::collections::BTreeMap;

use super::tools_command_mapping::get_command_mapping;
use super::tools_config::get_tool_config_loader;
use super::tools_detection::{check_tool_installed, infer_package_manager, ToolInfo};

/// Database-backed tool manager
///
/// Wraps database operations for tool management, providing
/// async methods that complement the existing synchronous ToolManager.
pub struct DbToolManager {
    #[allow(dead_code)]
    repository: ToolsRepository,
}

impl DbToolManager {
    /// Create a new database-backed tool manager
    pub async fn new() -> Result<Self> {
        let db = DatabaseManager::init().await?;
        let repository = ToolsRepository::new(db);
        Ok(Self { repository })
    }

    /// Get all tools from database
    pub async fn get_all_tools(&self) -> Result<Vec<Tool>> {
        self.repository.find_all().await
    }

    /// Get enabled tools only
    pub async fn get_enabled_tools(&self) -> Result<Vec<Tool>> {
        self.repository.find_all_enabled().await
    }

    /// Get a tool by ID
    pub async fn get_tool(&self, id: &str) -> Result<Option<Tool>> {
        self.repository.find_by_id(id).await
    }

    /// Get a tool by display name
    pub async fn get_tool_by_name(&self, display_name: &str) -> Result<Option<Tool>> {
        self.repository.find_by_display_name(display_name).await
    }

    /// Get a tool by CLI command
    #[allow(dead_code)]
    pub async fn get_tool_by_command(&self, command: &str) -> Result<Option<Tool>> {
        self.repository.find_by_command(command).await
    }

    /// Count total tools in database
    pub async fn count(&self) -> Result<i64> {
        self.repository.count().await
    }

    /// Check if database has tools (was import run?)
    pub async fn has_tools(&self) -> bool {
        self.count().await.unwrap_or(0) > 0
    }

    /// Enable a tool
    #[allow(dead_code)]
    pub async fn enable_tool(&self, id: &str) -> Result<()> {
        self.repository.set_enabled(id, true).await
    }

    /// Disable a tool
    #[allow(dead_code)]
    pub async fn disable_tool(&self, id: &str) -> Result<()> {
        self.repository.set_enabled(id, false).await
    }

    /// Get all available tools with installation status (async version)
    /// This is the database-backed equivalent of get_available_tools()
    pub async fn get_available_tools_async(&self) -> Result<BTreeMap<String, ToolInfo>> {
        let tools = self.repository.find_all_enabled().await?;
        let mut result = BTreeMap::new();

        for tool in tools {
            let is_installed = check_tool_installed(&tool.cli_command);
            let package_manager = infer_package_manager(&tool.id);
            result.insert(
                tool.id.clone(),
                ToolInfo {
                    // We need to leak the string to get &'static str
                    // This is acceptable since tools are long-lived
                    command: Box::leak(tool.cli_command.clone().into_boxed_str()),
                    is_installed,
                    package_manager,
                },
            );
        }

        Ok(result)
    }
}

/// Get available tools - database first, TOML fallback
///
/// This is the main entry point for hybrid tool retrieval.
/// It checks if the database has tools and uses them, otherwise
/// falls back to the TOML-based configuration.
pub async fn get_available_tools_hybrid() -> BTreeMap<String, ToolInfo> {
    // Try database first
    if let Ok(db_manager) = DbToolManager::new().await {
        if db_manager.has_tools().await {
            if let Ok(tools) = db_manager.get_available_tools_async().await {
                return tools;
            }
        }
    }

    // Fallback to TOML-based configuration
    get_available_tools_from_toml()
}

/// Get available tools from TOML configuration (synchronous fallback)
fn get_available_tools_from_toml() -> BTreeMap<String, ToolInfo> {
    let mut tools = BTreeMap::new();
    let mapping = get_command_mapping();
    let config_loader = get_tool_config_loader();

    // Get all tools from configuration
    let tool_names = config_loader.get_tool_names();

    for tool_name in tool_names {
        if let Some(cli_command) = mapping.get(tool_name.as_str()) {
            let is_installed = check_tool_installed(cli_command);
            let package_manager = infer_package_manager(&tool_name);
            tools.insert(
                tool_name.clone(),
                ToolInfo {
                    command: cli_command,
                    is_installed,
                    package_manager,
                },
            );
        }
    }

    tools
}

/// Hybrid tool lookup - checks database first, falls back to static config
///
/// This function enables gradual migration by checking the database first,
/// then falling back to the static ToolManager if the database is empty or
/// the tool isn't found.
pub async fn get_tool_hybrid(tool_name: &str) -> Option<Tool> {
    // Try database first
    if let Ok(db_manager) = DbToolManager::new().await {
        if db_manager.has_tools().await {
            if let Ok(Some(tool)) = db_manager.get_tool(tool_name).await {
                return Some(tool);
            }
            if let Ok(Some(tool)) = db_manager.get_tool_by_name(tool_name).await {
                return Some(tool);
            }
        }
    }

    // Fallback: convert TOML config to Tool entity
    let config_loader = get_tool_config_loader();
    if let Some(tool_def) = config_loader.get_tool_definition(tool_name) {
        return Some(
            Tool::new(tool_name, &tool_def.display_name, &tool_def.cli_command)
                .with_description(&tool_def.description),
        );
    }

    None
}

/// Get CLI command for a tool (hybrid lookup)
pub async fn get_cli_command_hybrid(tool_name: &str) -> Option<String> {
    if let Some(tool) = get_tool_hybrid(tool_name).await {
        return Some(tool.cli_command);
    }

    // Ultimate fallback to static mapping
    get_command_mapping().get(tool_name).map(|s| s.to_string())
}

/// Check if database has been initialized with tools
pub async fn is_db_initialized() -> bool {
    if let Ok(db_manager) = DbToolManager::new().await {
        return db_manager.has_tools().await;
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_db_tool_manager_struct() {
        // Just verify the struct and its methods compile
        // Actual database tests are in tools/repository.rs
        // This test exists to ensure the module compiles correctly
    }
}
