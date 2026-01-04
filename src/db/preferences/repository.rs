// Preferences Repository
//
// Data access layer for user preferences.
// Uses QueryBuilder for type-safe SQL construction.

use super::keys::PreferenceKeys;
use crate::db::core::connection::DatabaseManager;
use crate::db::core::query_builder::QueryBuilder;
use crate::db::core::repository::BaseRepository;
use crate::db::core::schema::PREFERENCES_TABLE;
use anyhow::Result;
use std::sync::Arc;

/// Preferences repository
pub struct PreferencesRepository {
    base: BaseRepository,
}

impl PreferencesRepository {
    /// Create a new preferences repository
    pub fn new(db: Arc<DatabaseManager>) -> Self {
        Self {
            base: BaseRepository::new(db, &PREFERENCES_TABLE),
        }
    }

    /// Get a preference value
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let sql = QueryBuilder::select(&PREFERENCES_TABLE)
            .columns(&["value"])
            .where_eq("key")
            .build();

        let mut rows = self.base.db().query(&sql, [key]).await?;

        if let Some(row) = rows.next().await? {
            let value: Option<String> = row.get(0)?;
            Ok(value)
        } else {
            Ok(None)
        }
    }

    /// Set a preference value
    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let sql = QueryBuilder::upsert(&PREFERENCES_TABLE)
            .columns(&["key", "value", "updated_at"])
            .on_conflict_update("key", &["value", "updated_at"])
            .build();

        let now = chrono::Utc::now().to_rfc3339();
        self.base.db().execute(&sql, [key, value, &now]).await?;

        Ok(())
    }

    /// Delete a preference
    pub async fn delete(&self, key: &str) -> Result<bool> {
        let sql = QueryBuilder::delete(&PREFERENCES_TABLE)
            .where_eq("key")
            .build();

        let affected = self.base.db().execute(&sql, [key]).await?;
        Ok(affected > 0)
    }

    /// Get all preferences
    pub async fn get_all(&self) -> Result<Vec<(String, String)>> {
        let sql = QueryBuilder::select(&PREFERENCES_TABLE)
            .columns(&["key", "value"])
            .order_by("key", true)
            .build();

        let mut rows = self.base.db().query(&sql, ()).await?;
        let mut prefs = Vec::new();

        while let Some(row) = rows.next().await? {
            let key: String = row.get(0)?;
            let value: Option<String> = row.get(1)?;
            if let Some(v) = value {
                prefs.push((key, v));
            }
        }

        Ok(prefs)
    }

    // =========================================================================
    // Typed accessors for common preferences
    // =========================================================================

    /// Get last used tool
    pub async fn get_last_used_tool(&self) -> Result<Option<String>> {
        self.get(PreferenceKeys::LAST_USED_TOOL).await
    }

    /// Set last used tool
    pub async fn set_last_used_tool(&self, tool: &str) -> Result<()> {
        self.set(PreferenceKeys::LAST_USED_TOOL, tool).await
    }

    /// Get default tool
    pub async fn get_default_tool(&self) -> Result<Option<String>> {
        self.get(PreferenceKeys::DEFAULT_TOOL).await
    }

    /// Set default tool
    pub async fn set_default_tool(&self, tool: &str) -> Result<()> {
        self.set(PreferenceKeys::DEFAULT_TOOL, tool).await
    }

    /// Get current theme
    pub async fn get_theme(&self) -> Result<Option<String>> {
        self.get(PreferenceKeys::THEME).await
    }

    /// Set current theme
    pub async fn set_theme(&self, theme: &str) -> Result<()> {
        self.set(PreferenceKeys::THEME, theme).await
    }

    /// Check if first run is complete
    pub async fn is_initialized(&self) -> Result<bool> {
        let value = self.get(PreferenceKeys::INITIALIZED).await?;
        Ok(value.map(|v| v == "true").unwrap_or(false))
    }

    /// Mark as initialized
    pub async fn mark_initialized(&self) -> Result<()> {
        self.set(PreferenceKeys::INITIALIZED, "true").await
    }

    /// Check if first run wizard is complete
    pub async fn is_first_run_complete(&self) -> Result<bool> {
        let value = self.get(PreferenceKeys::FIRST_RUN_COMPLETE).await?;
        Ok(value.map(|v| v == "true").unwrap_or(false))
    }

    /// Mark first run wizard as complete
    pub async fn mark_first_run_complete(&self) -> Result<()> {
        self.set(PreferenceKeys::FIRST_RUN_COMPLETE, "true").await
    }

    // =========================================================================
    // Generic typed accessors
    // =========================================================================

    /// Get a boolean preference
    pub async fn get_bool(&self, key: &str) -> Result<Option<bool>> {
        let value = self.get(key).await?;
        Ok(value.map(|v| v == "true" || v == "1"))
    }

    /// Set a boolean preference
    pub async fn set_bool(&self, key: &str, value: bool) -> Result<()> {
        self.set(key, if value { "true" } else { "false" }).await
    }

    /// Get an integer preference
    pub async fn get_int(&self, key: &str) -> Result<Option<i64>> {
        let value = self.get(key).await?;
        Ok(value.and_then(|v| v.parse().ok()))
    }

    /// Set an integer preference
    pub async fn set_int(&self, key: &str, value: i64) -> Result<()> {
        self.set(key, &value.to_string()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_preference_keys_accessible() {
        // Verify keys are accessible through the module
        assert!(!PreferenceKeys::LAST_USED_TOOL.is_empty());
        assert!(!PreferenceKeys::DEFAULT_TOOL.is_empty());
    }
}
