// Tools Repository
//
// Data access layer for tool configurations.
// Uses QueryBuilder for type-safe SQL construction.

use super::entities::{Tool, ToolAuth, ToolInstall};
use crate::db::core::connection::DatabaseManager;
use crate::db::core::query_builder::QueryBuilder;
use crate::db::core::repository::BaseRepository;
use crate::db::core::schema::{TOOLS_TABLE, TOOL_AUTH_TABLE, TOOL_INSTALL_TABLE};
use anyhow::Result;
use std::sync::Arc;

/// Tools repository
pub struct ToolsRepository {
    base: BaseRepository,
}

impl ToolsRepository {
    /// Create a new tools repository
    pub fn new(db: Arc<DatabaseManager>) -> Self {
        Self {
            base: BaseRepository::new(db, &TOOLS_TABLE),
        }
    }

    // =========================================================================
    // Tool CRUD Operations
    // =========================================================================

    /// Get all enabled tools
    pub async fn find_all_enabled(&self) -> Result<Vec<Tool>> {
        let sql = QueryBuilder::select(&TOOLS_TABLE)
            .where_eq_literal("enabled", "1")
            .order_by("display_name", true)
            .build();

        self.query_tools(&sql, ()).await
    }

    /// Get all tools (including disabled)
    pub async fn find_all(&self) -> Result<Vec<Tool>> {
        let sql = QueryBuilder::select(&TOOLS_TABLE)
            .order_by("display_name", true)
            .build();

        self.query_tools(&sql, ()).await
    }

    /// Find tool by ID
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Tool>> {
        let sql = QueryBuilder::select(&TOOLS_TABLE).where_eq("id").build();

        let tools = self.query_tools(&sql, [id]).await?;
        Ok(tools.into_iter().next())
    }

    /// Find tool by CLI command
    pub async fn find_by_command(&self, command: &str) -> Result<Option<Tool>> {
        let sql = QueryBuilder::select(&TOOLS_TABLE)
            .where_eq("cli_command")
            .build();

        let tools = self.query_tools(&sql, [command]).await?;
        Ok(tools.into_iter().next())
    }

    /// Save (upsert) a tool
    pub async fn save(&self, tool: &Tool) -> Result<()> {
        let sql = QueryBuilder::upsert(&TOOLS_TABLE)
            .columns(&[
                "id",
                "display_name",
                "cli_command",
                "description",
                "homepage",
                "documentation",
                "requires_npm",
                "requires_sudo",
                "status",
                "enabled",
                "auto_update",
            ])
            .on_conflict_update(
                "id",
                &[
                    "display_name",
                    "cli_command",
                    "description",
                    "homepage",
                    "documentation",
                    "requires_npm",
                    "requires_sudo",
                    "status",
                    "enabled",
                    "auto_update",
                ],
            )
            .build();

        self.base
            .db()
            .execute(
                &sql,
                libsql::params![
                    tool.id.clone(),
                    tool.display_name.clone(),
                    tool.cli_command.clone(),
                    tool.description.clone(),
                    tool.homepage.clone(),
                    tool.documentation.clone(),
                    tool.requires_npm as i32,
                    tool.requires_sudo as i32,
                    tool.status.clone(),
                    tool.enabled as i32,
                    tool.auto_update as i32,
                ],
            )
            .await?;

        Ok(())
    }

    /// Delete a tool
    pub async fn delete(&self, id: &str) -> Result<bool> {
        self.base.delete_by_id(id).await
    }

    /// Count all tools
    pub async fn count(&self) -> Result<i64> {
        self.base.count().await
    }

    // =========================================================================
    // Tool Install Operations
    // =========================================================================

    /// Get install info for a tool
    pub async fn get_install_info(&self, tool_id: &str) -> Result<Option<ToolInstall>> {
        let sql = QueryBuilder::select(&TOOL_INSTALL_TABLE)
            .where_eq("tool_id")
            .build();

        let mut rows = self.base.db().query(&sql, [tool_id]).await?;

        if let Some(row) = rows.next().await? {
            let args_json: Option<String> = row.get(2)?;
            let args: Vec<String> = args_json
                .and_then(|s: String| serde_json::from_str(&s).ok())
                .unwrap_or_default();

            Ok(Some(ToolInstall {
                tool_id: row.get(0)?,
                command: row.get(1)?,
                args,
                verify_command: row.get(3)?,
                post_install_message: row.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Save install info
    pub async fn save_install_info(&self, info: &ToolInstall) -> Result<()> {
        let sql = QueryBuilder::upsert(&TOOL_INSTALL_TABLE)
            .columns(&[
                "tool_id",
                "command",
                "args",
                "verify_command",
                "post_install_message",
            ])
            .on_conflict_update(
                "tool_id",
                &["command", "args", "verify_command", "post_install_message"],
            )
            .build();

        let args_json = serde_json::to_string(&info.args)?;

        self.base
            .db()
            .execute(
                &sql,
                libsql::params![
                    info.tool_id.clone(),
                    info.command.clone(),
                    args_json,
                    info.verify_command.clone(),
                    info.post_install_message.clone(),
                ],
            )
            .await?;

        Ok(())
    }

    // =========================================================================
    // Tool Auth Operations
    // =========================================================================

    /// Get auth info for a tool
    pub async fn get_auth_info(&self, tool_id: &str) -> Result<Option<ToolAuth>> {
        let sql = QueryBuilder::select(&TOOL_AUTH_TABLE)
            .where_eq("tool_id")
            .build();

        let mut rows = self.base.db().query(&sql, [tool_id]).await?;

        if let Some(row) = rows.next().await? {
            let env_vars_json: Option<String> = row.get(1)?;
            let env_vars: Vec<String> = env_vars_json
                .and_then(|s: String| serde_json::from_str(&s).ok())
                .unwrap_or_default();

            Ok(Some(ToolAuth {
                tool_id: row.get(0)?,
                env_vars,
                setup_url: row.get(2)?,
                browser_auth: row.get::<i32>(3)? != 0,
                auth_instructions: row.get(4)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Save auth info
    pub async fn save_auth_info(&self, info: &ToolAuth) -> Result<()> {
        let sql = QueryBuilder::upsert(&TOOL_AUTH_TABLE)
            .columns(&[
                "tool_id",
                "env_vars",
                "setup_url",
                "browser_auth",
                "auth_instructions",
            ])
            .on_conflict_update(
                "tool_id",
                &["env_vars", "setup_url", "browser_auth", "auth_instructions"],
            )
            .build();

        let env_vars_json = serde_json::to_string(&info.env_vars)?;

        self.base
            .db()
            .execute(
                &sql,
                libsql::params![
                    info.tool_id.clone(),
                    env_vars_json,
                    info.setup_url.clone(),
                    info.browser_auth as i32,
                    info.auth_instructions.clone(),
                ],
            )
            .await?;

        Ok(())
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Execute tool queries and map results
    async fn query_tools(
        &self,
        sql: &str,
        params: impl libsql::params::IntoParams,
    ) -> Result<Vec<Tool>> {
        let mut rows = self.base.db().query(sql, params).await?;
        let mut tools = Vec::new();

        while let Some(row) = rows.next().await? {
            tools.push(Tool {
                id: row.get(0)?,
                display_name: row.get(1)?,
                cli_command: row.get(2)?,
                description: row.get(3)?,
                homepage: row.get(4)?,
                documentation: row.get(5)?,
                requires_npm: row.get::<i32>(6)? != 0,
                requires_sudo: row.get::<i32>(7)? != 0,
                status: row.get(8)?,
                enabled: row.get::<i32>(9)? != 0,
                auto_update: row.get::<i32>(10)? != 0,
            });
        }

        Ok(tools)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_entity_creation() {
        let tool = Tool::new("test", "Test Tool", "test-cmd");
        assert_eq!(tool.id, "test");
        assert!(tool.enabled);
    }
}
