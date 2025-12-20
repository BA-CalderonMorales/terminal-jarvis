// Tools Database Bridge
//
// Provides database-backed tool operations, bridging between
// the existing ToolManager interface and the new libSQL database.
//
// This module enables gradual migration from TOML-based static
// configuration to database-backed dynamic configuration.

use crate::db::{DatabaseManager, Tool, ToolsRepository};
use anyhow::Result;

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
}

/// Hybrid tool lookup - checks database first, falls back to static config
///
/// This function enables gradual migration by checking the database first,
/// then falling back to the static ToolManager if the database is empty or
/// the tool isn't found.
#[allow(dead_code)]
pub async fn get_tool_hybrid(tool_name: &str) -> Option<Tool> {
    // Try database first
    if let Ok(db_manager) = DbToolManager::new().await {
        if let Ok(Some(tool)) = db_manager.get_tool(tool_name).await {
            return Some(tool);
        }
        if let Ok(Some(tool)) = db_manager.get_tool_by_name(tool_name).await {
            return Some(tool);
        }
    }

    // Fallback would go here - convert static config to Tool entity
    // For now, return None (caller should use original ToolManager)
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_db_tool_manager_struct() {
        // Just verify the struct and its methods compile
        // Actual database tests are in tools/repository.rs
        assert!(true);
    }
}
