// Repository Pattern Base
//
// Provides a common interface for all entity repositories.
// Encapsulates data access logic and provides type-safe operations.

use super::connection::DatabaseManager;
use super::schema::Table;
use anyhow::Result;
use std::sync::Arc;

/// Base trait for all repositories
///
/// Provides common CRUD operations that derived repositories can use.
/// Each entity repository implements this trait with its specific types.
#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    /// The entity type this repository manages
    type Entity: Send + Sync;

    /// The ID type for the entity
    type Id: Send + Sync;

    /// Get the table definition for this repository
    fn table(&self) -> &'static Table;

    /// Get the database manager
    fn db(&self) -> &Arc<DatabaseManager>;

    /// Find entity by ID
    async fn find_by_id(&self, id: &Self::Id) -> Result<Option<Self::Entity>>;

    /// Get all entities
    async fn find_all(&self) -> Result<Vec<Self::Entity>>;

    /// Save (insert or update) an entity
    async fn save(&self, entity: &Self::Entity) -> Result<()>;

    /// Delete entity by ID
    async fn delete(&self, id: &Self::Id) -> Result<bool>;

    /// Count all entities
    async fn count(&self) -> Result<i64>;
}

/// Helper struct for common repository operations
pub struct RepositoryHelper {
    db: Arc<DatabaseManager>,
}

impl RepositoryHelper {
    pub fn new(db: Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    /// Execute a count query
    pub async fn count(&self, table: &Table) -> Result<i64> {
        let sql = format!("SELECT COUNT(*) FROM {}", table.name);
        let mut rows = self.db.query(&sql, ()).await?;

        if let Some(row) = rows.next().await? {
            let count: i64 = row.get(0)?;
            Ok(count)
        } else {
            Ok(0)
        }
    }

    /// Check if a record exists
    pub async fn exists(&self, table: &Table, id_column: &str, id: &str) -> Result<bool> {
        let sql = format!(
            "SELECT 1 FROM {} WHERE {} = ? LIMIT 1",
            table.name, id_column
        );
        let mut rows = self.db.query(&sql, [id]).await?;
        Ok(rows.next().await?.is_some())
    }
}

// Note: async_trait is not a dependency yet, so we'll use a simpler pattern
// for now. The trait above documents the intended interface.

/// Simplified repository base that repositories can embed
pub struct BaseRepository {
    db: Arc<DatabaseManager>,
    table: &'static Table,
}

impl BaseRepository {
    pub fn new(db: Arc<DatabaseManager>, table: &'static Table) -> Self {
        Self { db, table }
    }

    pub fn db(&self) -> &Arc<DatabaseManager> {
        &self.db
    }

    pub fn table(&self) -> &'static Table {
        self.table
    }

    /// Execute a count query
    pub async fn count(&self) -> Result<i64> {
        let sql = format!("SELECT COUNT(*) FROM {}", self.table.name);
        let mut rows = self.db.query(&sql, ()).await?;

        if let Some(row) = rows.next().await? {
            let count: i64 = row.get(0)?;
            Ok(count)
        } else {
            Ok(0)
        }
    }

    /// Check if a record exists by primary key
    pub async fn exists(&self, id: &str) -> Result<bool> {
        let pk = self
            .table
            .primary_key()
            .ok_or_else(|| anyhow::anyhow!("Table {} has no primary key", self.table.name))?;

        let sql = format!(
            "SELECT 1 FROM {} WHERE {} = ? LIMIT 1",
            self.table.name, pk.name
        );
        let mut rows = self.db.query(&sql, [id]).await?;
        Ok(rows.next().await?.is_some())
    }

    /// Delete by primary key
    pub async fn delete_by_id(&self, id: &str) -> Result<bool> {
        let pk = self
            .table
            .primary_key()
            .ok_or_else(|| anyhow::anyhow!("Table {} has no primary key", self.table.name))?;

        let sql = format!("DELETE FROM {} WHERE {} = ?", self.table.name, pk.name);
        let affected = self.db.execute(&sql, [id]).await?;
        Ok(affected > 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::db::core::schema::PREFERENCES_TABLE;

    #[test]
    fn test_base_repository_table_access() {
        // We can't fully test without a DB, but we can verify the structure
        let table = &PREFERENCES_TABLE;
        assert_eq!(table.name, "preferences");
        assert!(table.primary_key().is_some());
    }
}
