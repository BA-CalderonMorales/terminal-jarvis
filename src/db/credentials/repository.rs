// Credentials Repository
//
// Data access layer for credential storage.
// Handles CRUD operations for API keys with optional encryption.

use super::entities::{Credential, ToolAuthStatus};
use crate::db::core::connection::DatabaseManager;
use crate::db::core::query_builder::QueryBuilder;
use crate::db::core::schema::CREDENTIALS_TABLE;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// Credentials repository for API key storage
pub struct CredentialsRepository {
    db: Arc<DatabaseManager>,
}

impl CredentialsRepository {
    /// Create a new credentials repository
    pub fn new(db: Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    // =========================================================================
    // CRUD Operations
    // =========================================================================

    /// Save or update a credential
    pub async fn save(&self, cred: &Credential) -> Result<()> {
        // Use upsert with tool_id + env_var as composite key
        let sql = "INSERT INTO credentials (tool_id, env_var, encrypted_value, updated_at) 
                   VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)
                   ON CONFLICT(tool_id, env_var) DO UPDATE SET
                   encrypted_value = excluded.encrypted_value,
                   updated_at = CURRENT_TIMESTAMP";

        self.db
            .execute(
                sql,
                libsql::params![
                    cred.tool_id.clone(),
                    cred.env_var.clone(),
                    cred.encrypted_value.clone(),
                ],
            )
            .await?;

        Ok(())
    }

    /// Get a specific credential
    pub async fn get(&self, tool_id: &str, env_var: &str) -> Result<Option<Credential>> {
        let sql = QueryBuilder::select(&CREDENTIALS_TABLE)
            .where_eq("tool_id")
            .where_eq("env_var")
            .build();

        let mut rows = self.db.query(&sql, [tool_id, env_var]).await?;

        if let Some(row) = rows.next().await? {
            Ok(Some(Credential {
                id: row.get(0)?,
                tool_id: row.get(1)?,
                env_var: row.get(2)?,
                encrypted_value: row.get(3)?,
                updated_at: None, // Skip parsing timestamp for now
            }))
        } else {
            Ok(None)
        }
    }

    /// Get all credentials for a tool
    pub async fn get_for_tool(&self, tool_id: &str) -> Result<Vec<Credential>> {
        let sql = QueryBuilder::select(&CREDENTIALS_TABLE)
            .where_eq("tool_id")
            .build();

        let mut rows = self.db.query(&sql, [tool_id]).await?;
        let mut creds = Vec::new();

        while let Some(row) = rows.next().await? {
            creds.push(Credential {
                id: row.get(0)?,
                tool_id: row.get(1)?,
                env_var: row.get(2)?,
                encrypted_value: row.get(3)?,
                updated_at: None,
            });
        }

        Ok(creds)
    }

    /// Get all credentials as a map (tool_id -> env_var -> value)
    pub async fn get_all_as_map(&self) -> Result<HashMap<String, HashMap<String, String>>> {
        let sql = QueryBuilder::select(&CREDENTIALS_TABLE).build();
        let mut rows = self.db.query(&sql, ()).await?;
        let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();

        while let Some(row) = rows.next().await? {
            let tool_id: String = row.get(1)?;
            let env_var: String = row.get(2)?;
            let value: Option<String> = row.get(3)?;

            if let Some(val) = value {
                result.entry(tool_id).or_default().insert(env_var, val);
            }
        }

        Ok(result)
    }

    /// Delete a specific credential
    pub async fn delete(&self, tool_id: &str, env_var: &str) -> Result<bool> {
        let sql = QueryBuilder::delete(&CREDENTIALS_TABLE)
            .where_eq("tool_id")
            .where_eq("env_var")
            .build();

        let affected = self
            .db
            .execute(&sql, libsql::params![tool_id, env_var])
            .await?;
        Ok(affected > 0)
    }

    /// Delete all credentials for a tool
    pub async fn delete_for_tool(&self, tool_id: &str) -> Result<u64> {
        let sql = QueryBuilder::delete(&CREDENTIALS_TABLE)
            .where_eq("tool_id")
            .build();

        let affected = self.db.execute(&sql, libsql::params![tool_id]).await?;
        Ok(affected)
    }

    /// Clear all credentials
    pub async fn clear_all(&self) -> Result<u64> {
        let sql = "DELETE FROM credentials";
        let affected = self.db.execute(sql, ()).await?;
        Ok(affected)
    }

    // =========================================================================
    // Auth Status Helpers
    // =========================================================================

    /// Get authentication status for a tool
    pub async fn get_auth_status(
        &self,
        tool_id: &str,
        tool_name: &str,
        required_vars: Vec<String>,
    ) -> Result<ToolAuthStatus> {
        let creds = self.get_for_tool(tool_id).await?;
        let configured: Vec<String> = creds
            .into_iter()
            .filter(|c| c.has_value())
            .map(|c| c.env_var)
            .collect();

        Ok(ToolAuthStatus::new(tool_id, tool_name, required_vars).with_configured(configured))
    }

    /// Count credentials for a tool
    pub async fn count_for_tool(&self, tool_id: &str) -> Result<i64> {
        let sql = "SELECT COUNT(*) FROM credentials WHERE tool_id = ?1";
        let mut rows = self.db.query(sql, [tool_id]).await?;

        if let Some(row) = rows.next().await? {
            Ok(row.get(0)?)
        } else {
            Ok(0)
        }
    }

    /// Check if a tool has all required credentials configured
    pub async fn is_tool_configured(&self, tool_id: &str, required_vars: &[&str]) -> Result<bool> {
        for var in required_vars {
            if let Ok(Some(cred)) = self.get(tool_id, var).await {
                if !cred.has_value() {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        Ok(true)
    }

    // =========================================================================
    // Migration from TOML
    // =========================================================================

    /// Import credentials from the existing TOML-based store
    pub async fn import_from_toml(&self) -> Result<ImportCredentialsStats> {
        use crate::auth_manager::auth_credentials_store::CredentialsStore;

        let mut stats = ImportCredentialsStats::default();

        // Load existing credentials from TOML store
        let toml_creds = CredentialsStore::load()?;

        for (tool_id, vars) in toml_creds.tools {
            for (env_var, value) in vars {
                let cred = Credential::builder(&tool_id, &env_var)
                    .value(&value)
                    .build();

                match self.save(&cred).await {
                    Ok(_) => stats.imported += 1,
                    Err(e) => {
                        stats.errors += 1;
                        stats
                            .error_messages
                            .push(format!("{}/{}: {}", tool_id, env_var, e));
                    }
                }
            }
        }

        Ok(stats)
    }
}

/// Statistics from credential import
#[derive(Debug, Default)]
pub struct ImportCredentialsStats {
    pub imported: usize,
    pub errors: usize,
    pub error_messages: Vec<String>,
}

impl ImportCredentialsStats {
    pub fn summary(&self) -> String {
        if self.errors == 0 {
            format!("Imported {} credentials", self.imported)
        } else {
            format!(
                "Imported {} credentials ({} errors)",
                self.imported, self.errors
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create test credentials
    fn test_credentials() -> Vec<Credential> {
        vec![
            Credential::builder("claude", "ANTHROPIC_API_KEY")
                .value("sk-ant-test-key-12345")
                .build(),
            Credential::builder("openai", "OPENAI_API_KEY")
                .value("sk-test-openai-key-67890")
                .build(),
            Credential::builder("gemini", "GOOGLE_API_KEY")
                .value("AIza-test-google-key")
                .build(),
        ]
    }

    #[test]
    fn test_credential_creation() {
        let creds = test_credentials();
        assert_eq!(creds.len(), 3);
        assert!(creds[0].has_value());
        assert_eq!(creds[0].tool_id, "claude");
    }

    #[test]
    fn test_import_stats() {
        let mut stats = ImportCredentialsStats::default();
        stats.imported = 5;
        stats.errors = 0;

        assert_eq!(stats.summary(), "Imported 5 credentials");

        stats.errors = 2;
        assert!(stats.summary().contains("errors"));
    }

    #[tokio::test]
    async fn test_repository_operations() {
        // This test would require a test database
        // For now, just verify the types compile correctly
        let cred = Credential::builder("test-tool", "TEST_API_KEY")
            .value("test-value")
            .build();

        assert_eq!(cred.tool_id, "test-tool");
        assert_eq!(cred.env_var, "TEST_API_KEY");
        assert!(cred.has_value());
    }
}
